//! The catalog: library folders, the persistent index, scanning and duplicate analysis.
//!
//! Everything the app needs to know about "what is in my libraries" lives here, behind one
//! type, so the Tauri commands stay thin and all of it can be tested without a window.
//!
//! * **Several library folders.** Each has a stable id; the first one has the nil id so its
//!   asset ids are unchanged from before multiple libraries existed.
//! * **Persistent index.** Assets and what each folder looked like at the last scan are
//!   kept in SQLite. On launch [`Catalog::load_from_index`] fills the in-memory library
//!   instantly; a background incremental scan then re-lists only folders that changed.
//! * **Safe by default.** A library folder that cannot be read (unplugged drive, network
//!   share down) is reported and left exactly as it is; it never empties the library.
//! * **Duplicates.** Byte-identical archives (staged hashing, cached), files shared between
//!   packages, and packages that mostly contain the same files.

use atlas_core::scan::ScanMode;
use atlas_core::{Asset, PackageFile};
use atlas_db::library_index::{self as index, ArchiveHash, LibraryRoot};
use atlas_duplicate::{
    find_exact, find_shared_files, find_similar, full_hash, quick_signature, ArchiveRef,
    ExactGroup, PackageContent, SharedFile, SharedOptions, SimilarPair,
};
use atlas_events::AtlasEvent;
use atlas_library::Library;
use atlas_scanner::contents::{find_seven_zip, has_sidecar_listing, read_contents};
use atlas_scanner::{Scanner, Snapshot};
use crossbeam_channel::Sender;
use rusqlite::Connection;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, String>;

fn db_err(e: rusqlite::Error) -> String {
    format!("Library index error: {e}")
}

/// A library folder with a little live information for the UI.
#[derive(Debug, Clone, Serialize)]
pub struct RootInfo {
    pub id: Uuid,
    pub path: String,
    pub name: String,
    pub added_at: String,
    pub last_scanned_at: Option<String>,
    pub asset_count: usize,
    /// False when the folder cannot be reached right now (unplugged drive, share down).
    pub available: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ScanSummary {
    pub added: usize,
    pub updated: usize,
    pub removed: usize,
    /// Assets in the whole library afterwards.
    pub total: usize,
    pub read_folders: u64,
    pub reused_folders: u64,
    pub duration_ms: u64,
    /// Problems that did not stop the scan (unreachable library folders, unreadable folders).
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AnalyzeProgress {
    /// Archives whose contents were read in this call.
    pub analyzed: usize,
    /// Archives that could not be read (encrypted, damaged); not retried until they change.
    pub failed: usize,
    /// Archives that need 7-Zip, which is not installed, to be read.
    pub needs_tool: usize,
    /// Archives still waiting to be analysed.
    pub remaining: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DuplicateReport {
    pub exact: Vec<ExactGroup>,
    pub shared_files: Vec<SharedFile>,
    pub similar: Vec<SimilarPair>,
    /// Archives whose contents have been analysed / archives in the library.
    pub analyzed: usize,
    pub total: usize,
}

pub struct Catalog {
    library: Arc<Library>,
    db: Arc<Mutex<Connection>>,
    events: Option<Sender<AtlasEvent>>,
}

fn ns(asset: &Asset) -> i64 {
    asset.modified_at.timestamp_nanos_opt().unwrap_or(0)
}

/// Compare two paths the way a file system does, ignoring case and slash direction.
fn key(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches('/')
        .to_lowercase()
}

impl Catalog {
    pub fn new(
        library: Arc<Library>,
        db: Arc<Mutex<Connection>>,
        events: Option<Sender<AtlasEvent>>,
    ) -> Self {
        Self {
            library,
            db,
            events,
        }
    }

    fn db(&self) -> Result<MutexGuard<'_, Connection>> {
        self.db
            .lock()
            .map_err(|_| "Library index is unavailable".to_string())
    }

    pub fn library(&self) -> &Arc<Library> {
        &self.library
    }

    // ── startup ──────────────────────────────────────────────

    /// Fill the in-memory library from the index. Instant, no disk scan; returns the count.
    pub fn load_from_index(&self) -> Result<usize> {
        let assets = index::load_assets(&*self.db()?).map_err(db_err)?;
        let count = assets.len();
        for asset in assets {
            self.library.insert(asset);
        }
        Ok(count)
    }

    // ── library folders ──────────────────────────────────────

    pub fn roots(&self) -> Result<Vec<RootInfo>> {
        let roots = index::list_roots(&*self.db()?).map_err(db_err)?;
        let all = self.library.all();
        Ok(roots
            .into_iter()
            .map(|r| RootInfo {
                asset_count: all.iter().filter(|a| a.root_id == r.id).count(),
                available: r.path.is_dir(),
                id: r.id,
                path: r.path.to_string_lossy().into_owned(),
                name: r.name,
                added_at: r.added_at,
                last_scanned_at: r.last_scanned_at,
            })
            .collect())
    }

    pub fn root(&self, id: Uuid) -> Result<LibraryRoot> {
        index::get_root(&*self.db()?, id)
            .map_err(db_err)?
            .ok_or_else(|| "That library folder is not in Evoury.".to_string())
    }

    /// Add a library folder (returns the existing one if the path is already added).
    ///
    /// A folder inside another library folder, or one that contains another, is refused:
    /// its packages would be indexed twice under different ids.
    pub fn add_root(&self, path: &Path, name: Option<&str>) -> Result<LibraryRoot> {
        if !path.is_dir() {
            return Err(format!("{} is not a folder.", path.display()));
        }
        let new = key(path);
        let conn = self.db()?;
        for existing in index::list_roots(&conn).map_err(db_err)? {
            let old = key(&existing.path);
            if old == new {
                return Ok(existing);
            }
            if new.starts_with(&format!("{old}/")) {
                return Err(format!(
                    "{} is already part of the library \"{}\".",
                    path.display(),
                    existing.name
                ));
            }
            if old.starts_with(&format!("{new}/")) {
                return Err(format!(
                    "{} contains the library \"{}\". Remove that one first.",
                    path.display(),
                    existing.name
                ));
            }
        }
        index::add_root(&conn, path, name).map_err(db_err)
    }

    pub fn rename_root(&self, id: Uuid, name: &str) -> Result<()> {
        let name = name.trim();
        if name.is_empty() {
            return Err("A library needs a name.".into());
        }
        index::rename_root(&*self.db()?, id, name).map_err(db_err)?;
        Ok(())
    }

    /// Remove a library folder from Evoury. Files on disk are never touched.
    /// Returns how many assets left the library.
    pub fn remove_root(&self, id: Uuid) -> Result<usize> {
        index::remove_root(&*self.db()?, id).map_err(db_err)?;
        Ok(self.library.remove_root(id))
    }

    // ── scanning ─────────────────────────────────────────────

    /// Scan one library folder and update the library and the index.
    ///
    /// Incremental modes re-list only folders that changed since the last scan. A library
    /// folder that is unreachable is an error and changes nothing.
    pub fn scan_root(&self, id: Uuid, mode: ScanMode) -> Result<ScanSummary> {
        let root = self.root(id)?;
        if !root.path.is_dir() {
            return Err(format!(
                "\"{}\" is not available right now ({}). Its assets are kept.",
                root.name,
                root.path.display()
            ));
        }

        let incremental = matches!(
            mode,
            ScanMode::IncrementalScan | ScanMode::StartupScan | ScanMode::BackgroundScan
        );
        let snapshot = if incremental {
            let conn = self.db()?;
            Some(Snapshot {
                assets: index::load_root_assets(&conn, id).map_err(db_err)?,
                folders: index::load_folders(&conn, id).map_err(db_err)?,
            })
        } else {
            None
        };

        let mut scanner = Scanner::new().with_mode(mode);
        if let Some(sender) = &self.events {
            scanner = scanner.with_event_sender(sender.clone());
        }
        let result = scanner.scan_root(&root.path, id, snapshot.as_ref());

        // A scan that found nothing and hit errors saw an unreadable folder, not an empty one.
        let unreadable = result.assets.is_empty() && result.stats.errors > 0;
        let partial = matches!(mode, ScanMode::FolderScan | ScanMode::FileScan);
        let prune = !unreadable && !partial;

        let mut summary = ScanSummary {
            read_folders: result.read_folders,
            reused_folders: result.reused_folders,
            duration_ms: result.stats.duration_ms,
            ..Default::default()
        };
        if unreadable {
            summary.errors.push(format!(
                "\"{}\" could not be read; its assets were left as they were.",
                root.name
            ));
        } else if result.stats.errors > 0 {
            summary.errors.push(format!(
                "\"{}\": {} item(s) could not be read.",
                root.name, result.stats.errors
            ));
        }

        let report = self.library.sync(result.assets, id, prune);
        summary.added = report.added;
        summary.updated = report.updated;
        summary.removed = report.removed;
        summary.total = self.library.len();

        // Save what the library now holds for this folder: that includes any collection or
        // category an asset was given, which a fresh scan result does not know about.
        let current = self.library.by_root(id);
        let conn = self.db()?;
        if prune && !partial {
            index::save_scan(&conn, id, &current, &result.folders, true).map_err(db_err)?;
        } else {
            index::save_assets(&conn, id, &current, false).map_err(db_err)?;
        }
        Ok(summary)
    }

    /// Scan every library folder. One that cannot be reached is reported and skipped.
    pub fn scan_all(&self, mode: ScanMode) -> Result<ScanSummary> {
        let roots = index::list_roots(&*self.db()?).map_err(db_err)?;
        let mut total = ScanSummary::default();
        for root in roots {
            match self.scan_root(root.id, mode) {
                Ok(s) => {
                    total.added += s.added;
                    total.updated += s.updated;
                    total.removed += s.removed;
                    total.read_folders += s.read_folders;
                    total.reused_folders += s.reused_folders;
                    total.duration_ms += s.duration_ms;
                    total.errors.extend(s.errors);
                }
                Err(e) => total.errors.push(e),
            }
        }
        total.total = self.library.len();
        Ok(total)
    }

    /// Add `path` as a library folder if it is not one yet, then scan it.
    pub fn add_and_scan(&self, path: &Path, mode: ScanMode) -> Result<(LibraryRoot, ScanSummary)> {
        let root = self.add_root(path, None)?;
        let summary = self.scan_root(root.id, mode)?;
        Ok((root, summary))
    }

    /// Put an asset in a collection / category (kept across rescans and restarts).
    pub fn set_assignment(
        &self,
        asset_id: Uuid,
        collection_id: Option<Uuid>,
        category_id: Option<Uuid>,
    ) -> Result<()> {
        let mut asset = self
            .library
            .get(&asset_id)
            .ok_or_else(|| "That asset is no longer in the library.".to_string())?;
        asset.collection_id = collection_id;
        asset.category_id = category_id;
        self.library.insert(asset);
        index::set_assignment(&*self.db()?, asset_id, collection_id, category_id).map_err(db_err)
    }

    // ── duplicates ───────────────────────────────────────────

    fn hash_for(&self, asset: &Asset, full: bool, cancel: &dyn Fn() -> bool) -> Option<String> {
        let modified_ns = ns(asset);
        let cached = {
            let conn = self.db().ok()?;
            index::get_archive_hash(&conn, asset.id)
                .ok()
                .flatten()
                .filter(|h| h.size == asset.file_size && h.modified_ns == modified_ns)
        };
        if let Some(h) = &cached {
            if let Some(value) = if full { &h.full } else { &h.quick } {
                return Some(value.clone());
            }
        }
        let value = if full {
            full_hash(&asset.archive_path, cancel).ok()?
        } else {
            quick_signature(&asset.archive_path).ok()?
        };
        let mut row = cached.unwrap_or(ArchiveHash {
            size: asset.file_size,
            modified_ns,
            quick: None,
            full: None,
        });
        if full {
            row.full = Some(value.clone());
        } else {
            row.quick = Some(value.clone());
        }
        if let Ok(conn) = self.db() {
            let _ = index::put_archive_hash(&conn, asset.id, &row);
        }
        Some(value)
    }

    /// Byte-identical archives. Hashes are cached, so the second run is nearly instant.
    pub fn find_exact_duplicates(&self, cancel: &dyn Fn() -> bool) -> Result<Vec<ExactGroup>> {
        let assets = self.library.all();
        let by_id: HashMap<Uuid, &Asset> = assets.iter().map(|a| (a.id, a)).collect();
        let refs: Vec<ArchiveRef> = assets
            .iter()
            .filter(|a| a.archive_path.is_file())
            .map(|a| ArchiveRef {
                id: a.id,
                path: a.archive_path.clone(),
                size: a.file_size,
            })
            .collect();
        let groups = find_exact(
            &refs,
            &mut |r| self.hash_for(by_id[&r.id], false, cancel),
            &mut |r| self.hash_for(by_id[&r.id], true, cancel),
        );
        if cancel() {
            return Err("Cancelled.".into());
        }
        Ok(groups)
    }

    /// Read the contents of up to `limit` archives that have not been analysed (or changed
    /// since). Reads Kaelovun's manifest when there is one, otherwise lists with 7-Zip.
    /// Call repeatedly until `remaining` stops shrinking.
    pub fn analyze_contents(
        &self,
        limit: usize,
        cancel: &dyn Fn() -> bool,
    ) -> Result<AnalyzeProgress> {
        let have_tool = find_seven_zip().is_some();
        let mut pending: Vec<Asset> = Vec::new();
        {
            let conn = self.db()?;
            for asset in self.library.all() {
                let version = index::package_files_version(&conn, asset.id).map_err(db_err)?;
                if version != Some((asset.file_size, ns(&asset))) && asset.archive_path.is_file() {
                    pending.push(asset);
                }
            }
        }
        pending.sort_by(|a, b| a.archive_path.cmp(&b.archive_path));

        let mut progress = AnalyzeProgress::default();
        let mut handled = 0usize;
        for asset in &pending {
            if cancel() || handled >= limit {
                break;
            }
            if !have_tool && !has_sidecar_listing(&asset.archive_path) {
                progress.needs_tool += 1;
                continue;
            }
            handled += 1;
            let files: Vec<PackageFile> = match read_contents(&asset.archive_path, 200_000) {
                Ok(c) => {
                    progress.analyzed += 1;
                    c.entries
                        .iter()
                        .filter_map(|e| e.as_package_file())
                        .collect()
                }
                Err(_) => {
                    // Encrypted or damaged: remember that, so it is not retried forever.
                    progress.failed += 1;
                    Vec::new()
                }
            };
            let conn = self.db()?;
            index::put_package_files(&conn, asset.id, asset.file_size, ns(asset), &files)
                .map_err(db_err)?;
        }
        // Everything not handled is still pending, including archives that need the tool.
        progress.remaining = pending.len() - handled;
        Ok(progress)
    }

    /// Everything duplicate detection knows: byte-identical archives, files shared between
    /// packages, and near-identical packages. The last two cover only analysed archives.
    pub fn duplicate_report(
        &self,
        opts: &SharedOptions,
        cancel: &dyn Fn() -> bool,
    ) -> Result<DuplicateReport> {
        let exact = self.find_exact_duplicates(cancel)?;
        let packages: Vec<PackageContent> = index::load_package_files(&*self.db()?)
            .map_err(db_err)?
            .into_iter()
            .map(|(id, files)| PackageContent { id, files })
            .collect();
        Ok(DuplicateReport {
            exact,
            shared_files: find_shared_files(&packages, opts),
            similar: find_similar(&packages, opts),
            analyzed: packages.len(),
            total: self.library.len(),
        })
    }
}

#[cfg(test)]
mod tests;
