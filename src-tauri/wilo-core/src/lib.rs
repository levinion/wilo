mod app;
mod searcher;
mod window;

use searcher::Search;

use anyhow::Result;
use app::ApplicationEntry;
use window::WindowEntry;

#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WiloListItem {
    pub name: String,
    pub exec: String,
    pub mode: u32,
}

#[derive(Default)]
pub enum WiloMode {
    #[default]
    ApplicationMode = 0,
    WindowMode = 1,
}

pub fn search(pat: &str) -> Result<Vec<WiloListItem>> {
    let pat = pat.trim().to_lowercase();
    if pat.starts_with("w ") {
        let pat = pat.strip_prefix("w ").unwrap();
        search_window(pat)
    } else {
        search_application(&pat)
    }
}

fn search_application(pat: &str) -> Result<Vec<WiloListItem>> {
    let home = std::env!("HOME");
    let local_dir = format!("{}/{}", home, ".local/share/applications");
    let info = ApplicationEntry::new("/usr/share/applications")?.join(&local_dir)?;
    info.search(pat)
}

fn search_window(pat: &str) -> Result<Vec<WiloListItem>> {
    let info = WindowEntry::new()?;
    info.search(pat)
}

pub fn execute(mode: u32, exec: &str) -> Result<()> {
    if mode == WiloMode::ApplicationMode as u32 {
        let args = exec.split_whitespace().collect::<Vec<_>>();
        std::process::Command::new("fish")
            .arg("-c")
            .args(args)
            .spawn()?;
        Ok(())
    } else if mode == WiloMode::WindowMode as u32 {
        let entry = WindowEntry::new()?;
        entry.active_window(exec.parse::<u32>()?)?;
        Ok(())
    } else {
        unreachable!();
    }
}
