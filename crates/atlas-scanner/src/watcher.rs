use atlas_events::AtlasEvent;
use crossbeam_channel::Sender;
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
        self.start_many(vec![path.as_ref().to_path_buf()], sender, poll_interval);
    }

    /// Watch several library folders with one thread.
    ///
    /// A folder that cannot be read (unplugged drive, share down) is skipped for that
    /// round and its previous state kept, so going offline does not report every file as
    /// removed and coming back does not report every file as created.
    pub fn start_many(
        &mut self,
        roots: Vec<PathBuf>,
        sender: Sender<AtlasEvent>,
        poll_interval: Duration,
    ) {
        if self.running.load(Ordering::SeqCst) || roots.is_empty() {
            return;
        }
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();

        self.handle = Some(thread::spawn(move || {
            let mut known: Vec<HashMap<PathBuf, WatchEntry>> = roots
                .iter()
                .map(|root| scan_directory(root).unwrap_or_default())
                .collect();

            while running.load(Ordering::SeqCst) {
                sleep_while_running(&running, poll_interval);
                if !running.load(Ordering::SeqCst) {
                    break;
                }

                for (root, previous) in roots.iter().zip(known.iter_mut()) {
                    let current = match scan_directory(root) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };
                    report_changes(previous, &current, &sender);
                    *previous = current;
                }
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

/// Sleep for `total`, but wake early when the watcher is stopped, so stopping (or switching
/// to another set of library folders) never waits out a whole poll interval.
fn sleep_while_running(running: &AtomicBool, total: Duration) {
    let step = Duration::from_millis(100);
    let mut slept = Duration::ZERO;
    while slept < total && running.load(Ordering::SeqCst) {
        let nap = step.min(total - slept);
        thread::sleep(nap);
        slept += nap;
    }
}

/// Send an event for every file that appeared, changed or disappeared between two snapshots.
fn report_changes(
    known: &HashMap<PathBuf, WatchEntry>,
    current: &HashMap<PathBuf, WatchEntry>,
    sender: &Sender<AtlasEvent>,
) {
    for (path, entry) in current {
        match known.get(path) {
            None => {
                let _ = sender.send(AtlasEvent::WatcherEvent {
                    kind: "created".into(),
                    path: path.clone(),
                });
            }
            Some(existing)
                if existing.modified != entry.modified || existing.size != entry.size =>
            {
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
}

fn scan_directory(root: &Path) -> std::io::Result<HashMap<PathBuf, WatchEntry>> {
    // An unplugged drive or a share that is down looks like an empty folder to the walker.
    // Say so explicitly, so the caller keeps its previous state instead of reporting every
    // file as removed.
    if !root.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "library folder is not available",
        ));
    }
    let mut map = HashMap::new();
    for entry in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !crate::scanner::is_ignored_dir(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();
        // Same rules as the scanner: archives and previews only, and never a file
        // that is still being written (Kaelovun's `*.kaelovun-part.*`), which would
        // otherwise fire an event on every size change while it is being built.
        if !atlas_core::archive::is_library_file(&path) {
            continue;
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

impl Default for FilesystemWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changes_in_several_folders_are_reported_and_an_offline_folder_is_not_a_mass_removal() {
        let base = std::env::temp_dir().join(format!("atlas-watch-many-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let (a, b) = (base.join("a"), base.join("b"));
        std::fs::create_dir_all(&a).unwrap();
        std::fs::create_dir_all(&b).unwrap();
        std::fs::write(b.join("old.7z"), b"x").unwrap();

        let (tx, rx) = crossbeam_channel::unbounded();
        let mut watcher = FilesystemWatcher::new();
        watcher.start_many(vec![a.clone(), b.clone()], tx, Duration::from_millis(40));
        std::thread::sleep(Duration::from_millis(150));
        std::fs::write(a.join("new.7z"), b"x").unwrap();
        std::fs::remove_dir_all(&b).unwrap(); // the second folder goes offline
        std::thread::sleep(Duration::from_millis(300));
        watcher.stop();

        let events: Vec<_> = rx.try_iter().collect();
        let _ = std::fs::remove_dir_all(&base);
        assert!(events.iter().any(|e| matches!(e,
            AtlasEvent::WatcherEvent { kind, path } if kind == "created" && path.ends_with("new.7z"))));
        assert!(
            !events.iter().any(|e| matches!(e,
            AtlasEvent::WatcherEvent { kind, .. } if kind == "removed")),
            "an offline library must not be reported as deleted"
        );
    }

    #[test]
    fn partial_files_and_non_library_files_do_not_fire_events() {
        let dir = std::env::temp_dir().join(format!("atlas-watch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("$RECYCLE.BIN")).unwrap();
        for f in [
            "A.7z",
            "A.avif",
            "A.thumb.avif",
            "B.part1.rar",
            "B.part2.rar",
            "C.kaelovun-part.7z",
            "notes.txt",
            "$RECYCLE.BIN/D.7z",
        ] {
            std::fs::write(dir.join(f), b"x").unwrap();
        }
        let mut seen: Vec<String> = scan_directory(&dir)
            .unwrap()
            .keys()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        seen.sort();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(seen, vec!["A.7z", "A.avif", "A.thumb.avif", "B.part1.rar"]);
    }
}
