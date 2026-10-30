use atlas_core::{
    archive::{parse_archive_name, parse_preview_name, stable_asset_id},
    scan::{FolderState, ScanFingerprint, ScanMode, ScanProgress, ScanStats},
    ArchiveType, Asset, AssetState, Preview,
};
use chrono::{DateTime, Utc};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use walkdir::WalkDir;

/// A folder whose modification time is this recent is not trusted on the next scan:
/// a second change within the same timestamp tick would be invisible.
const RACY_WINDOW_NS: i64 = 2_000_000_000;

/// What a previous scan of one library folder found. Handing it back to
/// [`Scanner::scan_root`] in an incremental mode lets unchanged folders be reused
/// without being listed again.
#[derive(Debug, Default, Clone)]
pub struct Snapshot {
    pub assets: Vec<Asset>,
    pub folders: Vec<FolderState>,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub assets: Vec<Asset>,
    /// Folder states to remember for the next incremental scan.
    pub folders: Vec<FolderState>,
    pub stats: ScanStats,
    /// Folders that were listed on disk.
    pub read_folders: u64,
    /// Folders whose packages were reused from the previous scan.
    pub reused_folders: u64,
}

pub struct Scanner {
    scan_id: Uuid,
    mode: ScanMode,
    event_sender: Option<Sender<atlas_events::AtlasEvent>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            scan_id: Uuid::new_v4(),
            mode: ScanMode::FullScan,
            event_sender: None,
        }
    }

    pub fn with_mode(mut self, mode: ScanMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_event_sender(mut self, sender: Sender<atlas_events::AtlasEvent>) -> Self {
        self.event_sender = Some(sender);
        self
    }

    /// Full scan of `root` for the first (nil-id) library. Kept for callers that do not
    /// keep a snapshot; the fingerprints argument is ignored.
    pub fn scan(
        &self,
        root: impl AsRef<Path>,
        _fingerprints: &[ScanFingerprint],
    ) -> (Vec<Asset>, ScanStats) {
        let result = self.scan_root(root.as_ref(), Uuid::nil(), None);
        (result.assets, result.stats)
    }

    /// Scan the library folder `root`.
    ///
    /// In `IncrementalScan`, `StartupScan` and `BackgroundScan` modes, folders whose
    /// modification time matches `previous` are not listed again: their packages are taken
    /// from the snapshot. Every other mode (full, verification, repair, folder) reads
    /// everything from disk.
    pub fn scan_root(&self, root: &Path, root_id: Uuid, previous: Option<&Snapshot>) -> ScanResult {
        let start = Instant::now();
        self.emit(atlas_events::AtlasEvent::ScanStarted {
            scan_id: self.scan_id,
            mode: format!("{:?}", self.mode),
            root: root.to_path_buf(),
        });

        let reuse = previous.filter(|_| {
            matches!(
                self.mode,
                ScanMode::IncrementalScan | ScanMode::StartupScan | ScanMode::BackgroundScan
            )
        });
        let known: HashMap<&Path, &FolderState> = reuse
            .map(|p| p.folders.iter().map(|f| (f.path.as_path(), f)).collect())
            .unwrap_or_default();
        let mut by_dir: HashMap<&Path, Vec<&Asset>> = HashMap::new();
        if let Some(p) = reuse {
            for asset in &p.assets {
                if let Some(parent) = asset.archive_path.parent() {
                    by_dir.entry(parent).or_default().push(asset);
                }
            }
        }

        let now_ns = system_time_ns(SystemTime::now());
        let mut assets: Vec<Asset> = Vec::new();
        let mut folders: Vec<FolderState> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        let (mut read_folders, mut reused_folders) = (0u64, 0u64);
        let mut previews_seen = 0u64;

        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let modified_ns = fs::metadata(&dir)
                .and_then(|m| m.modified())
                .map(system_time_ns)
                .unwrap_or(-1);

            let reusable = known
                .get(dir.as_path())
                .filter(|state| modified_ns >= 0 && state.modified_ns == modified_ns);

            let subdirs = if let Some(state) = reusable {
                reused_folders += 1;
                for asset in by_dir.get(dir.as_path()).into_iter().flatten() {
                    previews_seen += u64::from(asset.has_preview());
                    let mut asset = (*asset).clone();
                    asset.root_id = root_id;
                    assets.push(asset);
                }
                folders.push((*state).clone());
                state.subdirs.clone()
            } else {
                read_folders += 1;
                let listing = self.read_folder(root, root_id, &dir, &mut errors);
                previews_seen += listing.previews_seen;
                assets.extend(listing.assets);
                // Remember the folder, unless it changed so recently that the next scan
                // could not tell a further change apart.
                let racy = modified_ns < 0 || now_ns - modified_ns < RACY_WINDOW_NS;
                folders.push(FolderState {
                    path: dir.clone(),
                    modified_ns: if racy { -1 } else { modified_ns },
                    subdirs: listing.subdirs.clone(),
                });
                listing.subdirs
            };

            for name in subdirs.iter().rev() {
                stack.push(dir.join(name));
            }

            let visited = read_folders + reused_folders;
            if visited % 25 == 0 {
                self.emit(atlas_events::AtlasEvent::ScanProgress(ScanProgress {
                    scan_id: self.scan_id,
                    mode: self.mode,
                    total_files: 0,
                    scanned_files: visited,
                    archives_found: assets.len() as u64,
                    previews_found: previews_seen,
                    assets_created: 0,
                    errors: errors.clone(),
                    current_file: Some(dir.to_string_lossy().to_string()),
                }));
            }
        }

        assets.sort_by(|a, b| a.archive_path.cmp(&b.archive_path));

        let stats = ScanStats {
            total_archives: assets.len() as u64,
            total_previews: previews_seen,
            paired: assets.iter().filter(|a| a.has_preview()).count() as u64,
            missing_previews: assets.iter().filter(|a| !a.has_preview()).count() as u64,
            errors: errors.len() as u64,
            duration_ms: start.elapsed().as_millis() as u64,
        };

        self.emit(atlas_events::AtlasEvent::ScanFinished {
            scan_id: self.scan_id,
        });

        ScanResult {
            assets,
            folders,
            stats,
            read_folders,
            reused_folders,
        }
    }

    /// List one folder and turn what is in it into assets.
    ///
    /// Packages are identified by (folder, name). Keying by name alone made two
    /// `Logo.rar` files in different folders collapse into one asset and pair each of
    /// them with whichever `Logo.avif` was seen first. Because pairing never crosses a
    /// folder boundary, a folder can be processed (or reused) on its own.
    fn read_folder(
        &self,
        root: &Path,
        root_id: Uuid,
        dir: &Path,
        errors: &mut Vec<String>,
    ) -> Listing {
        let mut subdirs: Vec<String> = Vec::new();
        let mut archives: HashMap<String, Candidate> = HashMap::new();
        let mut previews: HashMap<String, (usize, PathBuf)> = HashMap::new();
        let mut thumbs: HashMap<String, (usize, PathBuf)> = HashMap::new();

        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => {
                errors.push(format!("Failed to read {}: {}", dir.display(), e));
                return Listing::default();
            }
        };

        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            if file_type.is_dir() {
                if !is_ignored_dir_name(&name) {
                    subdirs.push(name);
                }
                continue;
            }
            if !file_type.is_file() {
                continue; // symlinks and special files
            }
            let path = entry.path();

            // Unfinished files (`Name.kaelovun-part.7z`), macOS `._` forks and
            // non-primary volumes of split archives are skipped by these parsers.
            if let Some(parsed) = parse_archive_name(&path) {
                let modified = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .unwrap_or(UNIX_EPOCH);
                let candidate = Candidate {
                    stem: parsed.stem,
                    kind: parsed.kind,
                    path,
                    modified,
                };
                let key = candidate.stem.to_lowercase();
                match archives.get(&key) {
                    Some(existing) if !candidate.beats(existing) => {}
                    _ => {
                        archives.insert(key, candidate);
                    }
                }
            } else if let Some((stem, is_thumb, rank)) = parse_preview_name(&path) {
                let key = stem.to_lowercase();
                let map = if is_thumb { &mut thumbs } else { &mut previews };
                match map.get(&key) {
                    Some((best, _)) if *best <= rank => {}
                    _ => {
                        map.insert(key, (rank, path));
                    }
                }
            }
        }

        subdirs.sort();
        let mut keys: Vec<&String> = archives.keys().collect();
        keys.sort();

        let mut assets = Vec::new();
        for key in keys {
            let candidate = &archives[key];
            let metadata = match fs::metadata(&candidate.path) {
                Ok(m) => m,
                Err(e) => {
                    errors.push(format!(
                        "Failed to read metadata for {}: {}",
                        candidate.path.display(),
                        e
                    ));
                    continue;
                }
            };
            let modified: DateTime<Utc> = match metadata.modified() {
                Ok(t) => t.into(),
                Err(_) => Utc::now(),
            };
            let thumb = thumbs.get(key).map(|(_, p)| p.clone());
            let preview = match (previews.get(key), &thumb) {
                (Some((_, path)), _) => Some(Preview {
                    path: path.clone(),
                    thumb: thumb.clone(),
                }),
                // Only a thumbnail exists: still better than an empty card.
                (None, Some(t)) => Some(Preview {
                    path: t.clone(),
                    thumb: Some(t.clone()),
                }),
                (None, None) => None,
            };

            assets.push(Asset {
                id: stable_asset_id(root_id, root, dir, &candidate.stem),
                name: candidate.stem.clone(),
                archive_type: candidate.kind,
                archive_path: candidate.path.clone(),
                preview,
                file_size: metadata.len(),
                modified_at: modified,
                collection_id: None,
                category_id: None,
                state: AssetState::Ready,
                root_id,
            });
        }

        Listing {
            assets,
            subdirs,
            previews_seen: previews.len() as u64,
        }
    }

    pub fn build_fingerprints(&self, root: impl AsRef<Path>) -> Vec<ScanFingerprint> {
        let mut fingerprints = Vec::new();
        let mut folder_map: HashMap<PathBuf, (u64, u64, u64)> = HashMap::new();

        for entry in WalkDir::new(root.as_ref())
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            let parent = path.parent().unwrap_or(root.as_ref()).to_path_buf();
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let modified = match meta.modified() {
                Ok(t) => t
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                Err(_) => 0,
            };
            let entry = folder_map.entry(parent).or_insert((0, 0, 0));
            entry.0 += 1;
            entry.1 += meta.len();
            entry.2 = std::cmp::max(entry.2, modified);
        }

        for (folder, (file_count, total_size, last_modified)) in folder_map {
            fingerprints.push(ScanFingerprint {
                folder,
                file_count,
                total_size,
                last_modified,
            });
        }

        fingerprints
    }

    fn emit(&self, event: atlas_events::AtlasEvent) {
        if let Some(ref sender) = self.event_sender {
            let _ = sender.send(event);
        }
    }
}

#[derive(Default)]
struct Listing {
    assets: Vec<Asset>,
    subdirs: Vec<String>,
    previews_seen: u64,
}

/// One archive found during a scan.
struct Candidate {
    stem: String,
    kind: ArchiveType,
    path: PathBuf,
    modified: SystemTime,
}

impl Candidate {
    /// When the same package exists in several formats (`Pack.rar` and `Pack.7z`,
    /// for example after converting with "keep original"), pick one deterministically:
    /// the newest file, then the better format, then the path.
    fn beats(&self, other: &Candidate) -> bool {
        (
            self.modified,
            self.kind.rank(),
            std::cmp::Reverse(&self.path),
        ) > (
            other.modified,
            other.kind.rank(),
            std::cmp::Reverse(&other.path),
        )
    }
}

/// Modification time as nanoseconds since the Unix epoch (negative before it).
pub(crate) fn system_time_ns(t: SystemTime) -> i64 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as i64,
        Err(e) => -(e.duration().as_nanos() as i64),
    }
}

/// Folders that never contain library content.
pub(crate) fn is_ignored_dir_name(name: &str) -> bool {
    name == "$RECYCLE.BIN"
        || name == "System Volume Information"
        || name == "__MACOSX"
        || name.starts_with(".Trash")
        || name == ".git"
}

pub(crate) fn is_ignored_dir(entry: &walkdir::DirEntry) -> bool {
    entry.depth() != 0
        && entry.file_type().is_dir()
        && is_ignored_dir_name(&entry.file_name().to_string_lossy())
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::time::{Duration, SystemTime};

    struct TempDir(PathBuf);

    impl TempDir {
        fn new(label: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "atlas-scanner-{}-{}-{:?}",
                label,
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }
        fn touch(&self, rel: &str) -> PathBuf {
            let p = self.0.join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, rel.as_bytes()).unwrap();
            p
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn scan(root: &Path) -> Vec<Asset> {
        Scanner::new().scan(root, &[]).0
    }

    fn by_name<'a>(assets: &'a [Asset], name: &str) -> Vec<&'a Asset> {
        assets.iter().filter(|a| a.name == name).collect()
    }

    #[test]
    fn same_name_in_different_folders_stays_two_assets_with_their_own_previews() {
        let t = TempDir::new("samename");
        t.touch("fonts/Logo.rar");
        t.touch("fonts/Logo.avif");
        t.touch("fonts/Logo.thumb.avif");
        t.touch("mockups/Logo.7z");
        t.touch("mockups/Logo.png");
        let assets = scan(&t.0);
        let logos = by_name(&assets, "Logo");
        assert_eq!(logos.len(), 2, "one of the two Logo packages vanished");
        let fonts = logos
            .iter()
            .find(|a| a.archive_path.ends_with("Logo.rar"))
            .unwrap();
        let mockups = logos
            .iter()
            .find(|a| a.archive_path.ends_with("Logo.7z"))
            .unwrap();
        let fp = fonts.preview.as_ref().unwrap();
        assert!(
            fp.path.ends_with("fonts/Logo.avif")
                && fp.thumb.as_ref().unwrap().ends_with("Logo.thumb.avif")
        );
        let mp = mockups.preview.as_ref().unwrap();
        assert!(mp.path.ends_with("mockups/Logo.png") && mp.thumb.is_none());
        assert_ne!(fonts.id, mockups.id);
    }

    #[test]
    fn unfinished_kaelovun_files_are_not_indexed() {
        let t = TempDir::new("partial");
        t.touch("Pack.kaelovun-part.7z");
        t.touch("Pack.kaelovun-part.avif");
        t.touch("Other.7z");
        let assets = scan(&t.0);
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Other");
    }

    #[test]
    fn kaelovun_output_with_a_numbered_name_pairs_correctly() {
        let t = TempDir::new("numbered");
        for f in [
            "Pack.7z",
            "Pack.avif",
            "Pack (2).7z",
            "Pack (2).avif",
            "Pack (2).thumb.avif",
        ] {
            t.touch(f);
        }
        let assets = scan(&t.0);
        assert_eq!(assets.len(), 2);
        let two = by_name(&assets, "Pack (2)")[0];
        assert!(two
            .preview
            .as_ref()
            .unwrap()
            .path
            .ends_with("Pack (2).avif"));
        assert!(by_name(&assets, "Pack")[0]
            .preview
            .as_ref()
            .unwrap()
            .thumb
            .is_none());
    }

    #[test]
    fn split_archives_are_one_asset() {
        let t = TempDir::new("volumes");
        for f in [
            "v.part1.rar",
            "v.part2.rar",
            "v.part3.rar",
            "s.7z.001",
            "s.7z.002",
        ] {
            t.touch(f);
        }
        let mut names: Vec<_> = scan(&t.0).into_iter().map(|a| a.name).collect();
        names.sort();
        assert_eq!(names, vec!["s", "v"]);
    }

    #[test]
    fn the_same_package_in_two_formats_is_one_asset_and_the_newest_wins() {
        let t = TempDir::new("formats");
        let old = t.touch("p.rar");
        let new = t.touch("p.7z");
        let now = SystemTime::now();
        File::options()
            .write(true)
            .open(old)
            .unwrap()
            .set_modified(now - Duration::from_secs(3600))
            .unwrap();
        File::options()
            .write(true)
            .open(new)
            .unwrap()
            .set_modified(now)
            .unwrap();
        let assets = scan(&t.0);
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].archive_type, ArchiveType::SevenZip);
    }

    #[test]
    fn ids_survive_rescans_and_format_conversion() {
        let t = TempDir::new("ids");
        let rar = t.touch("sets/p.rar");
        let first = scan(&t.0)[0].id;
        assert_eq!(first, scan(&t.0)[0].id, "id changed between two scans");
        fs::remove_file(rar).unwrap();
        t.touch("sets/p.7z");
        assert_eq!(
            first,
            scan(&t.0)[0].id,
            "converting rar -> 7z changed the asset id"
        );
    }

    #[test]
    fn ids_do_not_depend_on_where_the_library_lives() {
        let a = TempDir::new("movea");
        let b = TempDir::new("moveb");
        a.touch("x/y.zip");
        b.touch("x/y.zip");
        assert_eq!(scan(&a.0)[0].id, scan(&b.0)[0].id);
    }

    #[test]
    fn junk_folders_and_resource_forks_are_ignored() {
        let t = TempDir::new("junk");
        t.touch("$RECYCLE.BIN/old.7z");
        t.touch("__MACOSX/._real.7z");
        t.touch("real/._real.7z");
        t.touch("real/real.7z");
        let assets = scan(&t.0);
        assert_eq!(assets.len(), 1);
        assert!(assets[0].archive_path.ends_with("real/real.7z"));
    }

    #[test]
    fn thumbnail_only_still_gives_a_card_an_image() {
        let t = TempDir::new("thumbonly");
        t.touch("a.zip");
        t.touch("a.thumb.avif");
        let p = scan(&t.0)[0].preview.clone().unwrap();
        assert!(p.path.ends_with("a.thumb.avif"));
    }

    #[test]
    fn preview_format_preference_is_deterministic() {
        let t = TempDir::new("prefer");
        t.touch("a.zip");
        t.touch("a.png");
        t.touch("a.avif");
        t.touch("a.jpg");
        assert!(scan(&t.0)[0]
            .preview
            .as_ref()
            .unwrap()
            .path
            .ends_with("a.avif"));
    }

    #[test]
    fn previews_without_an_archive_are_not_assets() {
        let t = TempDir::new("orphan");
        t.touch("lonely.avif");
        let (assets, stats) = Scanner::new().scan(&t.0, &[]);
        assert!(assets.is_empty());
        assert_eq!(stats.total_previews, 1);
    }

    // ── incremental scanning ─────────────────────────────────

    #[cfg(unix)]
    fn backdate_dirs(root: &Path) {
        let old = SystemTime::now() - std::time::Duration::from_secs(3600);
        for entry in WalkDir::new(root).into_iter().flatten() {
            if entry.file_type().is_dir() {
                fs::File::open(entry.path())
                    .unwrap()
                    .set_modified(old)
                    .unwrap();
            }
        }
    }

    fn incremental(root: &Path, previous: &ScanResult) -> ScanResult {
        let snapshot = Snapshot {
            assets: previous.assets.clone(),
            folders: previous.folders.clone(),
        };
        Scanner::new()
            .with_mode(ScanMode::IncrementalScan)
            .scan_root(root, Uuid::nil(), Some(&snapshot))
    }

    fn full(root: &Path) -> ScanResult {
        Scanner::new().scan_root(root, Uuid::nil(), None)
    }

    fn names(result: &ScanResult) -> Vec<String> {
        let mut n: Vec<_> = result.assets.iter().map(|a| a.name.clone()).collect();
        n.sort();
        n
    }

    #[test]
    fn a_folder_changed_a_moment_ago_is_never_reused() {
        let t = TempDir::new("racy");
        t.touch("a/one.7z");
        let first = full(&t.0);
        let second = incremental(&t.0, &first);
        assert_eq!(
            second.reused_folders, 0,
            "a just-written folder must be listed again"
        );
        assert_eq!(names(&second), names(&first));
    }

    #[cfg(unix)]
    #[test]
    fn unchanged_folders_are_reused_without_being_listed() {
        let t = TempDir::new("reuse");
        t.touch("a/one.7z");
        t.touch("a/one.avif");
        t.touch("b/deep/two.zip");
        t.touch("top.rar");
        backdate_dirs(&t.0);
        let first = full(&t.0);
        let second = incremental(&t.0, &first);
        assert_eq!((second.read_folders, second.reused_folders), (0, 4));
        assert_eq!(names(&second), vec!["one", "top", "two"]);
        let ids = |r: &ScanResult| {
            let mut v: Vec<_> = r.assets.iter().map(|a| a.id).collect();
            v.sort();
            v
        };
        assert_eq!(ids(&first), ids(&second));
        assert!(second
            .assets
            .iter()
            .find(|a| a.name == "one")
            .unwrap()
            .has_preview());
        assert_eq!(second.stats.paired, 1);
    }

    #[cfg(unix)]
    #[test]
    fn only_the_changed_folder_is_read_again() {
        let t = TempDir::new("changed");
        t.touch("a/one.7z");
        t.touch("b/two.7z");
        backdate_dirs(&t.0);
        let first = full(&t.0);
        t.touch("b/three.7z");
        let second = incremental(&t.0, &first);
        assert_eq!(names(&second), vec!["one", "three", "two"]);
        assert_eq!(second.read_folders, 1, "only b/ should have been listed");
        assert_eq!(second.reused_folders, 2);
    }

    #[cfg(unix)]
    #[test]
    fn deleted_files_and_folders_disappear() {
        let t = TempDir::new("deleted");
        t.touch("a/one.7z");
        t.touch("b/two.7z");
        t.touch("b/sub/three.7z");
        backdate_dirs(&t.0);
        let first = full(&t.0);
        fs::remove_dir_all(t.0.join("b/sub")).unwrap();
        fs::remove_file(t.0.join("a/one.7z")).unwrap();
        let second = incremental(&t.0, &first);
        assert_eq!(names(&second), vec!["two"]);
    }

    #[cfg(unix)]
    #[test]
    fn full_scans_ignore_the_snapshot() {
        let t = TempDir::new("fullmode");
        t.touch("a/one.7z");
        backdate_dirs(&t.0);
        let first = full(&t.0);
        let snapshot = Snapshot {
            assets: first.assets.clone(),
            folders: first.folders.clone(),
        };
        let again = Scanner::new().scan_root(&t.0, Uuid::nil(), Some(&snapshot));
        assert_eq!(again.reused_folders, 0);
        let verify = Scanner::new()
            .with_mode(ScanMode::VerificationScan)
            .scan_root(&t.0, Uuid::nil(), Some(&snapshot));
        assert_eq!(verify.reused_folders, 0);
    }

    #[test]
    fn the_same_folder_in_two_libraries_gets_different_ids() {
        let t = TempDir::new("roots");
        t.touch("x/y.7z");
        let a = Scanner::new().scan_root(&t.0, Uuid::nil(), None).assets[0].id;
        let b = Scanner::new()
            .scan_root(&t.0, Uuid::from_u128(5), None)
            .assets[0]
            .id;
        assert_ne!(a, b);
        let r = Scanner::new().scan_root(&t.0, Uuid::from_u128(5), None);
        assert_eq!(r.assets[0].root_id, Uuid::from_u128(5));
    }

    #[test]
    fn a_missing_root_is_an_error_not_a_panic() {
        let result = full(Path::new("/definitely/not/here"));
        assert!(result.assets.is_empty());
        assert_eq!(result.stats.errors, 1);
    }
}
