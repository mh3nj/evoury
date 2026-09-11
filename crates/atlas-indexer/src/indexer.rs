use rayon::prelude::*;
use atlas_core::Asset;
use crate::state::IndexProgress;

pub struct Indexer;

impl Indexer {
    pub fn run<F>(assets: Vec<Asset>, mut callback: F)
    where
        F: FnMut(IndexProgress),
    {
        let total = assets.len();
        let mut completed = 0;

        assets.par_iter().for_each(|_| {});

        for _ in 0..total {
            completed += 1;
            callback(IndexProgress { total, completed });
        }
    }
}
