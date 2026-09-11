use std::collections::VecDeque;
use atlas_core::Asset;

#[derive(Debug)]
pub struct LazyPage {
    pub index: usize,
    pub assets: Vec<Asset>,
    pub loaded: bool,
}

pub struct PageBuffer {
    pages: VecDeque<LazyPage>,
    page_size: usize,
    max_buffered_pages: usize,
}

impl PageBuffer {
    pub fn new(page_size: usize, max_buffered_pages: usize) -> Self {
        Self {
            pages: VecDeque::new(),
            page_size,
            max_buffered_pages,
        }
    }

    pub fn page_count(&self, total_assets: usize) -> usize {
        (total_assets + self.page_size - 1) / self.page_size
    }

    pub fn insert_page(&mut self, page: LazyPage) {
        if self.pages.len() >= self.max_buffered_pages {
            self.pages.pop_front();
        }
        self.pages.push_back(page);
    }

    pub fn get_page(&self, index: usize) -> Option<&LazyPage> {
        self.pages.iter().find(|p| p.index == index)
    }

    pub fn get_page_mut(&mut self, index: usize) -> Option<&mut LazyPage> {
        self.pages.iter_mut().find(|p| p.index == index)
    }

    pub fn loaded_pages(&self) -> usize {
        self.pages.len()
    }

    pub fn clear(&mut self) {
        self.pages.clear();
    }

    pub fn range(&self, start: usize, end: usize) -> Vec<&Asset> {
        let mut result = Vec::new();
        for page in &self.pages {
            if page.index >= start && page.index <= end {
                result.extend(&page.assets);
            }
        }
        result
    }
}

impl LazyPage {
    pub fn new(index: usize, assets: Vec<Asset>) -> Self {
        let loaded = !assets.is_empty();
        Self { index, assets, loaded }
    }

    pub fn empty(index: usize) -> Self {
        Self { index, assets: Vec::new(), loaded: false }
    }
}
