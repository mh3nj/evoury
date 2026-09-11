use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use atlas_core::Asset;
use uuid::Uuid;

pub enum SearchResult {
    Chunk(Vec<Uuid>),
    Done,
}

pub struct ProgressiveSearch {
    cancel_token: Option<Sender<()>>,
}

impl ProgressiveSearch {
    pub fn new() -> Self {
        Self { cancel_token: None }
    }

    pub fn search(
        &mut self,
        assets: Vec<Asset>,
        query: String,
        batch_size: usize,
        _timeout_ms: u64,
    ) -> Receiver<SearchResult> {
        let (tx, rx) = mpsc::channel();
        let (cancel_tx, cancel_rx) = mpsc::channel();
        self.cancel_token = Some(cancel_tx);

        let query_lower = query.to_lowercase();

        thread::spawn(move || {
            let mut batch = Vec::new();

            for asset in &assets {
                if cancel_rx.try_recv().is_ok() {
                    let _ = tx.send(SearchResult::Done);
                    return;
                }

                let name_lower = asset.name.to_lowercase();
                if name_lower.contains(&query_lower) {
                    batch.push(asset.id);
                }

                if batch.len() >= batch_size {
                    let _ = tx.send(SearchResult::Chunk(batch.clone()));
                    batch.clear();
                }
            }

            if !batch.is_empty() {
                let _ = tx.send(SearchResult::Chunk(batch));
            }
            let _ = tx.send(SearchResult::Done);
        });

        rx
    }

    pub fn cancel(&mut self) {
        if let Some(tx) = self.cancel_token.take() {
            let _ = tx.send(());
        }
    }
}
