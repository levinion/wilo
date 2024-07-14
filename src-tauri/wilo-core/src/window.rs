use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use xcb::{
    x::{self, Window},
    Xid, XidNew,
};

use crate::{
    searcher::{Search, SearchResultItem, SearchResultList},
    WiloListItem, WiloMode,
};

pub struct WindowEntry {
    conn: xcb::Connection,
}

impl WindowEntry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Result<Self> {
        let conn = xcb::Connection::connect(None)?.0;
        Ok(Self { conn })
    }

    fn get_atom(&self, name: &[u8]) -> x::Atom {
        let cookie = self.conn.send_request(&x::InternAtom {
            only_if_exists: false,
            name,
        });
        let reply = self.conn.wait_for_reply(cookie).unwrap();
        reply.atom()
    }

    pub fn window_titles(&self) -> Vec<(u32, String)> {
        let client_list = self.get_atom(b"_NET_CLIENT_LIST");
        let string = self.get_atom(b"UTF8_STRING");
        let window_name = self.get_atom(b"_NET_WM_NAME");

        self.conn
            .get_setup()
            .roots()
            .map(|screen| screen.root())
            .map(|window| {
                self.conn.send_request(&xcb::x::GetProperty {
                    delete: false,
                    window,
                    property: client_list,
                    r#type: xcb::x::ATOM_WINDOW,
                    long_offset: 0,
                    long_length: 1024,
                })
            })
            .filter_map(|cookie| self.conn.wait_for_reply(cookie).ok())
            .flat_map(|reply| reply.value().to_vec())
            .map(|window| {
                let cookie = self.conn.send_request(&xcb::x::GetProperty {
                    delete: false,
                    window,
                    property: window_name,
                    r#type: string,
                    long_offset: 0,
                    long_length: 1024,
                });
                let reply = self.conn.wait_for_reply(cookie);
                (window.resource_id(), reply)
            })
            .filter(|(_id, reply)| reply.is_ok())
            .map(|(id, reply)| {
                let reply = reply.unwrap();
                (id, String::from_utf8(reply.value().to_vec()).unwrap())
            })
            .collect()
    }

    pub fn active_window(&self, wid: u32) -> Result<()> {
        let window = unsafe { Window::new(wid) };
        let cookie = self.conn.send_request_checked(&x::MapWindow { window });
        self.conn.check_request(cookie)?;
        Ok(())
    }
}

impl Search for WindowEntry {
    fn search(&self, pattern: &str) -> Result<Vec<crate::WiloListItem>> {
        let titles = WindowEntry::new()?.window_titles();
        let list = titles
            .into_par_iter()
            .map(|(id, title)| {
                (
                    title.trim().to_lowercase(),
                    WiloListItem {
                        name: title,
                        exec: id.to_string(),
                        mode: WiloMode::WindowMode as u32,
                    },
                )
            })
            .map(|(key, item)| {
                if key.starts_with(pattern) {
                    (100, item)
                } else if key.contains(pattern) {
                    (50, item)
                } else {
                    (0, item)
                }
            })
            .map(|(priority, item)| SearchResultItem { priority, item })
            .collect::<Vec<_>>();
        let list = SearchResultList { list };
        Ok(list.sort())
    }
}

mod test {
    #[allow(unused)]
    use crate::window::WindowEntry;

    #[test]
    fn test_window_titles() {
        let info = WindowEntry::new().unwrap();
        let titles = info.window_titles();
        assert!(!titles.is_empty());
    }
}
