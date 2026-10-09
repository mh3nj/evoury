use crate::state::IndexProgress;
use atlas_core::Asset;
use rayon::prelude::*;

pub struct Indexer;

impl Indexer {
    pub fn run<F>(assets: Vec<Asset>, mut callback: F)
    where
        F: FnMut(IndexProgress),
    {
        let total = assets.len();

        assets.par_iter().for_each(|_| {});

        for completed in 1..=total {
            callback(IndexProgress { total, completed });
        }
    }
}
