use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Result};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::searcher::{Search, SearchResultItem, SearchResultList};
use crate::WiloListItem;

#[derive(Default)]
pub struct ApplicationEntry {
    pub info: Vec<WiloListItem>,
}

impl ApplicationEntry {
    pub fn new(path: &str) -> Result<ApplicationEntry> {
        parse_application_dir(path)
    }

    pub fn join(mut self, path: &str) -> Result<ApplicationEntry> {
        let new_info = parse_application_dir(path)?;
        self.info.extend(new_info.info);
        Ok(self)
    }
}

impl Search for ApplicationEntry {
    fn search(&self, pattern: &str) -> Result<Vec<WiloListItem>> {
        let list = self
            .info
            .clone()
            .into_par_iter()
            .map(|item| (item.name.trim().to_lowercase(), item))
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

fn parse_application_file(path: &str) -> Result<WiloListItem> {
    let mut info = WiloListItem::default();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .flatten()
        .map(|line| line.trim().to_string())
        .for_each(|line| {
            if info.exec.is_empty() && line.starts_with("Exec=") {
                let exec = line.strip_prefix("Exec=").unwrap();
                info.exec = exec.to_string();
            }
            if info.name.is_empty() && line.starts_with("Name=") {
                let name = line.strip_prefix("Name=").unwrap();
                info.name = name.to_string();
            }
        });
    if info.name.is_empty() || info.exec.is_empty() {
        return Err(anyhow!("invalid .desktop: {}", path));
    }
    Ok(info)
}

fn parse_application_dir(path: &str) -> Result<ApplicationEntry> {
    let info = read_dir(path)?
        .par_bridge()
        .flatten()
        .filter_map(|entry| {
            let entry = entry.path();
            if entry.is_file()
                && entry.extension().is_some()
                && entry.extension().unwrap() == "desktop"
            {
                if let Ok(r) = parse_application_file(entry.to_str().unwrap()) {
                    Some(r)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(ApplicationEntry { info })
}
