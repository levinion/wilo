use anyhow::Result;

use crate::WiloListItem;

#[derive(Default)]
pub struct SearchResultList<T> {
    pub list: Vec<SearchResultItem<T>>,
}

impl<T> SearchResultList<T> {
    pub fn sort(mut self) -> Vec<T> {
        self.list
            .sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        self.list.into_iter().map(|item| item.item).collect()
    }
}

pub struct SearchResultItem<T> {
    pub priority: u32,
    pub item: T,
}

pub trait Search {
    fn search(&self, pattern: &str) -> Result<Vec<WiloListItem>>;
}
