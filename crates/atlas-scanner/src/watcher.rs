use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use crossbeam_channel::Sender;
use atlas_events::AtlasEvent;

struct WatchEntry {
    modified: u64,
    size: u64,
}

pub struct FilesystemWatcher {
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl FilesystemWatcher {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn start<P: AsRef<Path> + Send + 'static>(
        &mut self,
        path: P,
        sender: Sender<AtlasEvent>,
        poll_interval: Duration,
    ) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let root = path.as_ref().to_path_buf();

        self.handle = Some(thread::spawn(move || {
            let mut known: HashMap<PathBuf, WatchEntry> = HashMap::new();

            if let Ok(entries) = scan_directory(&root) {
                known = entries;
            }

            while running.load(Ordering::SeqCst) {
                thread::sleep(poll_interval);

                let current = match scan_directory(&root) {
                    Ok(c) => c,
                    Err(_) => continue,
                };

                for (path, entry) in &current {
                    match known.get(path) {
                        None => {
                            let _ = sender.send(AtlasEvent::WatcherEvent {
                                kind: "created".into(),
                                path: path.clone(),
                            });
                        }
                        Some(existing) if existing.modified != entry.modified || existing.size != entry.size => {
                            let _ = sender.send(AtlasEvent::WatcherEvent {
                                kind: "modified".into(),
                                path: path.clone(),
                            });
                        }
                        _ => {}
                    }
                }

                for path in known.keys() {
                    if !current.contains_key(path) {
                        let _ = sender.send(AtlasEvent::WatcherEvent {
                            kind: "removed".into(),
                            path: path.clone(),
                        });
                    }
                }

                known = current;
            }
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

fn scan_directory(root: &Path) -> std::io::Result<HashMap<PathBuf, WatchEntry>> {
    let mut map = HashMap::new();
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();
        let ext = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();
        match ext.as_str() {
            "zip" | "rar" | "7z" | "tar" | "avif" | "png" | "jpg" | "jpeg" | "webp" => {}
            _ => continue,
        }
        if let Ok(meta) = entry.metadata() {
            let modified = match meta.modified() {
                Ok(t) => t
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                Err(_) => 0,
            };
            map.insert(
                path,
                WatchEntry {
                    modified,
                    size: meta.len(),
                },
            );
        }
    }
    Ok(map)
}
