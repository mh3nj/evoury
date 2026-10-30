//! The persistent library index.
//!
//! Evoury used to keep its library only in memory and scan everything on every launch.
//! This module stores library folders ("roots"), the assets found in them and what each
//! folder looked like at the last scan, so the app can show the library immediately and
//! only look at what changed.
//!
//! All functions take a plain `&Connection`; callers decide about locking.

use atlas_core::{ArchiveType, Asset, AssetState, FolderState, PackageFile, Preview};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// A library folder.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryRoot {
    pub id: Uuid,
    pub path: PathBuf,
    pub name: String,
    pub added_at: String,
    pub last_scanned_at: Option<String>,
}

fn row_root(row: &rusqlite::Row) -> SqlResult<LibraryRoot> {
    let id: String = row.get(0)?;
    let path: String = row.get(1)?;
    Ok(LibraryRoot {
        id: Uuid::parse_str(&id).unwrap_or_default(),
        path: PathBuf::from(path),
        name: row.get(2)?,
        added_at: row.get(3)?,
        last_scanned_at: row.get(4)?,
    })
}

// ── roots ────────────────────────────────────────────────────

pub fn list_roots(conn: &Connection) -> SqlResult<Vec<LibraryRoot>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, name, added_at, last_scanned_at FROM library_roots ORDER BY added_at, rowid",
    )?;
    let rows = stmt.query_map([], row_root)?;
    rows.collect()
}

pub fn get_root(conn: &Connection, id: Uuid) -> SqlResult<Option<LibraryRoot>> {
    conn.query_row(
        "SELECT id, path, name, added_at, last_scanned_at FROM library_roots WHERE id = ?1",
        params![id.to_string()],
        row_root,
    )
    .optional()
}

fn normalize(path: &Path) -> String {
    let text = path.to_string_lossy();
    let trimmed = text.trim_end_matches(['/', '\\']);
    // Keep "/" and "C:\" intact
    if trimmed.is_empty() || trimmed.ends_with(':') {
        text.into_owned()
    } else {
        trimmed.to_string()
    }
}

/// Add a library folder (or return the one that already has this path).
///
/// The first library gets `Uuid::nil()` as its id, which keeps its asset ids identical to
/// what they were before multiple libraries existed. Later ones get random ids.
pub fn add_root(conn: &Connection, path: &Path, name: Option<&str>) -> SqlResult<LibraryRoot> {
    let path_text = normalize(path);
    if let Some(existing) = conn
        .query_row(
            "SELECT id, path, name, added_at, last_scanned_at FROM library_roots WHERE path = ?1",
            params![path_text],
            row_root,
        )
        .optional()?
    {
        return Ok(existing);
    }
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM library_roots", [], |r| r.get(0))?;
    let id = if count == 0 {
        Uuid::nil()
    } else {
        Uuid::new_v4()
    };
    let name = name
        .map(str::to_string)
        .filter(|n| !n.trim().is_empty())
        .or_else(|| {
            Path::new(&path_text)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| path_text.clone());
    let added_at = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO library_roots (id, path, name, added_at) VALUES (?1, ?2, ?3, ?4)",
        params![id.to_string(), path_text, name, added_at],
    )?;
    Ok(LibraryRoot {
        id,
        path: PathBuf::from(path_text),
        name,
        added_at,
        last_scanned_at: None,
    })
}

pub fn rename_root(conn: &Connection, id: Uuid, name: &str) -> SqlResult<bool> {
    Ok(conn.execute(
        "UPDATE library_roots SET name = ?2 WHERE id = ?1",
        params![id.to_string(), name],
    )? > 0)
}

/// Remove a library folder and everything indexed for it (never touches files on disk).
pub fn remove_root(conn: &Connection, id: Uuid) -> SqlResult<bool> {
    let tx = conn.unchecked_transaction()?;
    let id_text = id.to_string();
    // The duplicate caches are keyed by asset id and have no foreign key.
    tx.execute(
        "DELETE FROM archive_hashes WHERE asset_id IN (SELECT id FROM library_assets WHERE root_id = ?1)",
        params![id_text],
    )?;
    tx.execute(
        "DELETE FROM package_files WHERE asset_id IN (SELECT id FROM library_assets WHERE root_id = ?1)",
        params![id_text],
    )?;
    let removed = tx.execute("DELETE FROM library_roots WHERE id = ?1", params![id_text])?;
    tx.commit()?;
    Ok(removed > 0)
}

// ── assets and folder states ─────────────────────────────────

fn type_to_text(t: ArchiveType) -> &'static str {
    match t {
        ArchiveType::Zip => "Zip",
        ArchiveType::Rar => "Rar",
        ArchiveType::SevenZip => "SevenZip",
        ArchiveType::Tar => "Tar",
        ArchiveType::Unknown => "Unknown",
    }
}

fn text_to_type(s: &str) -> ArchiveType {
    match s {
        "Zip" => ArchiveType::Zip,
        "Rar" => ArchiveType::Rar,
        "SevenZip" => ArchiveType::SevenZip,
        "Tar" => ArchiveType::Tar,
        _ => ArchiveType::Unknown,
    }
}

fn state_to_text(s: AssetState) -> &'static str {
    match s {
        AssetState::Discovered => "Discovered",
        AssetState::Paired => "Paired",
        AssetState::Validated => "Validated",
        AssetState::Indexed => "Indexed",
        AssetState::Ready => "Ready",
        AssetState::Broken => "Broken",
    }
}

fn text_to_state(s: &str) -> AssetState {
    match s {
        "Discovered" => AssetState::Discovered,
        "Paired" => AssetState::Paired,
        "Validated" => AssetState::Validated,
        "Indexed" => AssetState::Indexed,
        "Broken" => AssetState::Broken,
        _ => AssetState::Ready,
    }
}

fn opt_uuid(text: Option<String>) -> Option<Uuid> {
    text.and_then(|t| Uuid::parse_str(&t).ok())
}

const ASSET_COLUMNS: &str =
    "id, root_id, name, archive_type, archive_path, preview_path, thumb_path, \
     file_size, modified_at, collection_id, category_id, state";

fn row_asset(row: &rusqlite::Row) -> SqlResult<Asset> {
    let id: String = row.get(0)?;
    let root_id: String = row.get(1)?;
    let preview_path: Option<String> = row.get(5)?;
    let thumb_path: Option<String> = row.get(6)?;
    let modified: String = row.get(8)?;
    let preview = preview_path.map(|p| Preview {
        path: PathBuf::from(p),
        thumb: thumb_path.map(PathBuf::from),
    });
    let archive_type: String = row.get(3)?;
    let archive_path: String = row.get(4)?;
    let state: String = row.get(11)?;
    Ok(Asset {
        id: Uuid::parse_str(&id).unwrap_or_default(),
        root_id: Uuid::parse_str(&root_id).unwrap_or_default(),
        name: row.get(2)?,
        archive_type: text_to_type(&archive_type),
        archive_path: PathBuf::from(archive_path),
        preview,
        file_size: row.get::<_, i64>(7)?.max(0) as u64,
        modified_at: DateTime::parse_from_rfc3339(&modified)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        collection_id: opt_uuid(row.get(9)?),
        category_id: opt_uuid(row.get(10)?),
        state: text_to_state(&state),
    })
}

/// Every indexed asset, across all library folders.
pub fn load_assets(conn: &Connection) -> SqlResult<Vec<Asset>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {ASSET_COLUMNS} FROM library_assets ORDER BY archive_path"
    ))?;
    let rows = stmt.query_map([], row_asset)?;
    rows.collect()
}

pub fn load_root_assets(conn: &Connection, root_id: Uuid) -> SqlResult<Vec<Asset>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {ASSET_COLUMNS} FROM library_assets WHERE root_id = ?1 ORDER BY archive_path"
    ))?;
    let rows = stmt.query_map(params![root_id.to_string()], row_asset)?;
    rows.collect()
}

pub fn load_folders(conn: &Connection, root_id: Uuid) -> SqlResult<Vec<FolderState>> {
    let mut stmt =
        conn.prepare("SELECT path, modified_ns, subdirs FROM library_folders WHERE root_id = ?1")?;
    let rows = stmt.query_map(params![root_id.to_string()], |row| {
        let path: String = row.get(0)?;
        let subdirs: String = row.get(2)?;
        Ok(FolderState {
            path: PathBuf::from(path),
            modified_ns: row.get(1)?,
            subdirs: serde_json::from_str(&subdirs).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

/// Store the result of a scan of one library folder, atomically.
///
/// `assets` are upserted. With `prune`, assets of this library that are not in `assets`
/// are deleted (their files are gone) together with their duplicate-detection caches.
/// `folders` replace the remembered folder states.
pub fn save_scan(
    conn: &Connection,
    root_id: Uuid,
    assets: &[Asset],
    folders: &[FolderState],
    prune: bool,
) -> SqlResult<()> {
    save_inner(conn, root_id, assets, Some(folders), prune)
}

/// Like [`save_scan`] for results that are not a complete picture of the library folder
/// (the remembered folder states are left as they were).
pub fn save_assets(
    conn: &Connection,
    root_id: Uuid,
    assets: &[Asset],
    prune: bool,
) -> SqlResult<()> {
    save_inner(conn, root_id, assets, None, prune)
}

fn save_inner(
    conn: &Connection,
    root_id: Uuid,
    assets: &[Asset],
    folders: Option<&[FolderState]>,
    prune: bool,
) -> SqlResult<()> {
    let tx = conn.unchecked_transaction()?;
    let root_text = root_id.to_string();

    {
        let mut upsert = tx.prepare(
            "INSERT INTO library_assets (id, root_id, name, archive_type, archive_path, preview_path,
                 thumb_path, file_size, modified_at, collection_id, category_id, state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
             ON CONFLICT(id) DO UPDATE SET
                 root_id = excluded.root_id, name = excluded.name,
                 archive_type = excluded.archive_type, archive_path = excluded.archive_path,
                 preview_path = excluded.preview_path, thumb_path = excluded.thumb_path,
                 file_size = excluded.file_size, modified_at = excluded.modified_at,
                 collection_id = excluded.collection_id, category_id = excluded.category_id,
                 state = excluded.state",
        )?;
        for a in assets {
            upsert.execute(params![
                a.id.to_string(),
                root_text,
                a.name,
                type_to_text(a.archive_type),
                a.archive_path.to_string_lossy(),
                a.preview
                    .as_ref()
                    .map(|p| p.path.to_string_lossy().into_owned()),
                a.preview
                    .as_ref()
                    .and_then(|p| p.thumb.as_ref())
                    .map(|t| t.to_string_lossy().into_owned()),
                a.file_size as i64,
                a.modified_at.to_rfc3339(),
                a.collection_id.map(|u| u.to_string()),
                a.category_id.map(|u| u.to_string()),
                state_to_text(a.state),
            ])?;
        }
    }

    if prune {
        tx.execute_batch(
            "CREATE TEMP TABLE IF NOT EXISTS _keep (id TEXT PRIMARY KEY); DELETE FROM _keep;",
        )?;
        {
            let mut keep = tx.prepare("INSERT OR IGNORE INTO _keep (id) VALUES (?1)")?;
            for a in assets {
                keep.execute(params![a.id.to_string()])?;
            }
        }
        for table in ["archive_hashes", "package_files"] {
            tx.execute(
                &format!(
                    "DELETE FROM {table} WHERE asset_id IN
                       (SELECT id FROM library_assets WHERE root_id = ?1 AND id NOT IN (SELECT id FROM _keep))"
                ),
                params![root_text],
            )?;
        }
        tx.execute(
            "DELETE FROM library_assets WHERE root_id = ?1 AND id NOT IN (SELECT id FROM _keep)",
            params![root_text],
        )?;
        tx.execute_batch("DROP TABLE _keep;")?;
    }

    if let Some(folders) = folders {
        tx.execute(
            "DELETE FROM library_folders WHERE root_id = ?1",
            params![root_text],
        )?;
        let mut insert = tx.prepare(
            "INSERT INTO library_folders (root_id, path, modified_ns, subdirs) VALUES (?1, ?2, ?3, ?4)",
        )?;
        for f in folders {
            insert.execute(params![
                root_text,
                f.path.to_string_lossy(),
                f.modified_ns,
                serde_json::to_string(&f.subdirs).unwrap_or_else(|_| "[]".into()),
            ])?;
        }
    }
    tx.execute(
        "UPDATE library_roots SET last_scanned_at = ?2 WHERE id = ?1",
        params![root_text, Utc::now().to_rfc3339()],
    )?;
    tx.commit()
}

/// Change which collection / category an asset is in.
pub fn set_assignment(
    conn: &Connection,
    asset_id: Uuid,
    collection_id: Option<Uuid>,
    category_id: Option<Uuid>,
) -> SqlResult<()> {
    conn.execute(
        "UPDATE library_assets SET collection_id = ?2, category_id = ?3 WHERE id = ?1",
        params![
            asset_id.to_string(),
            collection_id.map(|u| u.to_string()),
            category_id.map(|u| u.to_string())
        ],
    )?;
    Ok(())
}

// ── duplicate-detection caches ───────────────────────────────

/// What was learned about an archive file's bytes, valid while size and mtime match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveHash {
    pub size: u64,
    pub modified_ns: i64,
    pub quick: Option<String>,
    pub full: Option<String>,
}

pub fn get_archive_hash(conn: &Connection, asset_id: Uuid) -> SqlResult<Option<ArchiveHash>> {
    conn.query_row(
        "SELECT size, modified_ns, quick, full FROM archive_hashes WHERE asset_id = ?1",
        params![asset_id.to_string()],
        |row| {
            Ok(ArchiveHash {
                size: row.get::<_, i64>(0)?.max(0) as u64,
                modified_ns: row.get(1)?,
                quick: row.get(2)?,
                full: row.get(3)?,
            })
        },
    )
    .optional()
}

pub fn put_archive_hash(conn: &Connection, asset_id: Uuid, hash: &ArchiveHash) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO archive_hashes (asset_id, size, modified_ns, quick, full)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(asset_id) DO UPDATE SET size = excluded.size,
             modified_ns = excluded.modified_ns, quick = excluded.quick, full = excluded.full",
        params![
            asset_id.to_string(),
            hash.size as i64,
            hash.modified_ns,
            hash.quick,
            hash.full
        ],
    )?;
    Ok(())
}

/// `(archive_size, archive_modified_ns)` the cached file list was made from, if any.
pub fn package_files_version(conn: &Connection, asset_id: Uuid) -> SqlResult<Option<(u64, i64)>> {
    conn.query_row(
        "SELECT archive_size, archive_modified_ns FROM package_files WHERE asset_id = ?1 LIMIT 1",
        params![asset_id.to_string()],
        |row| Ok((row.get::<_, i64>(0)?.max(0) as u64, row.get(1)?)),
    )
    .optional()
}

/// Replace the cached file list of an asset. An empty list is stored as a single marker
/// row (empty path) so "analysed, nothing inside" is not mistaken for "not analysed".
pub fn put_package_files(
    conn: &Connection,
    asset_id: Uuid,
    archive_size: u64,
    archive_modified_ns: i64,
    files: &[PackageFile],
) -> SqlResult<()> {
    let tx = conn.unchecked_transaction()?;
    let id = asset_id.to_string();
    tx.execute("DELETE FROM package_files WHERE asset_id = ?1", params![id])?;
    {
        let mut insert = tx.prepare(
            "INSERT OR REPLACE INTO package_files
               (asset_id, archive_size, archive_modified_ns, path, size, crc)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        if files.is_empty() {
            insert.execute(params![
                id,
                archive_size as i64,
                archive_modified_ns,
                "",
                0i64,
                None::<i64>
            ])?;
        }
        for f in files {
            insert.execute(params![
                id,
                archive_size as i64,
                archive_modified_ns,
                f.path,
                f.size as i64,
                f.crc.map(|c| c as i64)
            ])?;
        }
    }
    tx.commit()
}

/// The cached file lists of all current assets (stale ones, whose archive changed size, are skipped).
pub fn load_package_files(conn: &Connection) -> SqlResult<Vec<(Uuid, Vec<PackageFile>)>> {
    let mut stmt = conn.prepare(
        "SELECT p.asset_id, p.path, p.size, p.crc
           FROM package_files p JOIN library_assets a ON a.id = p.asset_id
          WHERE p.archive_size = a.file_size AND p.path <> ''
          ORDER BY p.asset_id",
    )?;
    let mut out: Vec<(Uuid, Vec<PackageFile>)> = Vec::new();
    let rows = stmt.query_map([], |row| {
        let id: String = row.get(0)?;
        Ok((
            Uuid::parse_str(&id).unwrap_or_default(),
            PackageFile {
                path: row.get(1)?,
                size: row.get::<_, i64>(2)?.max(0) as u64,
                crc: row.get::<_, Option<i64>>(3)?.map(|c| c as u32),
            },
        ))
    })?;
    for row in rows {
        let (id, file) = row?;
        match out.last_mut() {
            Some((last, files)) if *last == id => files.push(file),
            _ => out.push((id, vec![file])),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{connection::open_in_memory, MigrationRunner};

    fn db() -> Connection {
        let conn = open_in_memory().unwrap();
        MigrationRunner::run(&conn).unwrap();
        conn
    }

    fn asset(root: Uuid, n: u128, path: &str) -> Asset {
        Asset {
            id: Uuid::from_u128(n),
            name: format!("a{n}"),
            archive_type: ArchiveType::SevenZip,
            archive_path: PathBuf::from(path),
            preview: None,
            file_size: 100 + n as u64,
            modified_at: Utc::now(),
            collection_id: None,
            category_id: None,
            state: AssetState::Ready,
            root_id: root,
        }
    }

    #[test]
    fn the_first_root_is_nil_and_later_ones_are_not() {
        let conn = db();
        let a = add_root(&conn, Path::new("/lib/one"), None).unwrap();
        let b = add_root(&conn, Path::new("/lib/two/"), Some("Second")).unwrap();
        assert!(a.id.is_nil() && !b.id.is_nil());
        assert_eq!((a.name.as_str(), b.name.as_str()), ("one", "Second"));
        assert_eq!(b.path, PathBuf::from("/lib/two")); // trailing slash dropped
        assert_eq!(list_roots(&conn).unwrap().len(), 2);
    }

    #[test]
    fn adding_the_same_folder_twice_returns_the_existing_root() {
        let conn = db();
        let a = add_root(&conn, Path::new("/lib/one"), None).unwrap();
        let again = add_root(&conn, Path::new("/LIB/one/"), None).unwrap();
        assert_eq!(a.id, again.id);
        assert_eq!(list_roots(&conn).unwrap().len(), 1);
    }

    #[test]
    fn assets_round_trip_with_previews_and_assignments() {
        let conn = db();
        let root = add_root(&conn, Path::new("/lib"), None).unwrap();
        let mut a = asset(root.id, 1, "/lib/a.7z");
        a.preview = Some(Preview {
            path: PathBuf::from("/lib/a.avif"),
            thumb: Some(PathBuf::from("/lib/a.thumb.avif")),
        });
        a.collection_id = Some(Uuid::from_u128(77));
        a.state = AssetState::Broken;
        let b = asset(root.id, 2, "/lib/b.rar");
        save_scan(&conn, root.id, &[a.clone(), b], &[], true).unwrap();
        let loaded = load_assets(&conn).unwrap();
        assert_eq!(loaded.len(), 2);
        let got = loaded.iter().find(|x| x.id == a.id).unwrap();
        assert_eq!(
            got.preview.as_ref().unwrap().thumb,
            a.preview.as_ref().unwrap().thumb
        );
        assert_eq!(
            (got.collection_id, got.state),
            (Some(Uuid::from_u128(77)), AssetState::Broken)
        );
        assert_eq!(got.file_size, a.file_size);
        assert_eq!(got.modified_at.timestamp(), a.modified_at.timestamp());
        assert!(loaded
            .iter()
            .find(|x| x.id == Uuid::from_u128(2))
            .unwrap()
            .preview
            .is_none());
    }

    #[test]
    fn saving_again_updates_in_place_and_prune_removes_the_rest() {
        let conn = db();
        let root = add_root(&conn, Path::new("/lib"), None).unwrap();
        save_scan(
            &conn,
            root.id,
            &[
                asset(root.id, 1, "/lib/a.7z"),
                asset(root.id, 2, "/lib/b.7z"),
            ],
            &[],
            true,
        )
        .unwrap();
        save_scan(
            &conn,
            root.id,
            &[asset(root.id, 1, "/lib/moved/a.7z")],
            &[],
            true,
        )
        .unwrap();
        let loaded = load_assets(&conn).unwrap();
        assert_eq!(loaded.len(), 1);
        assert!(loaded[0].archive_path.ends_with("moved/a.7z"));
    }

    #[test]
    fn partial_saves_do_not_prune_and_other_roots_are_untouched() {
        let conn = db();
        let r1 = add_root(&conn, Path::new("/one"), None).unwrap();
        let r2 = add_root(&conn, Path::new("/two"), None).unwrap();
        save_scan(
            &conn,
            r1.id,
            &[asset(r1.id, 1, "/one/a.7z"), asset(r1.id, 2, "/one/b.7z")],
            &[],
            true,
        )
        .unwrap();
        save_scan(&conn, r2.id, &[asset(r2.id, 3, "/two/c.7z")], &[], true).unwrap();
        save_scan(&conn, r1.id, &[asset(r1.id, 1, "/one/a.7z")], &[], false).unwrap();
        assert_eq!(load_root_assets(&conn, r1.id).unwrap().len(), 2);
        save_scan(&conn, r1.id, &[asset(r1.id, 1, "/one/a.7z")], &[], true).unwrap();
        assert_eq!(load_root_assets(&conn, r1.id).unwrap().len(), 1);
        assert_eq!(load_root_assets(&conn, r2.id).unwrap().len(), 1);
    }

    #[test]
    fn save_assets_keeps_the_remembered_folders() {
        let conn = db();
        let root = add_root(&conn, Path::new("/lib"), None).unwrap();
        let f = FolderState {
            path: PathBuf::from("/lib"),
            modified_ns: 7,
            subdirs: vec![],
        };
        save_scan(&conn, root.id, &[], &[f.clone()], true).unwrap();
        save_assets(&conn, root.id, &[asset(root.id, 1, "/lib/a.7z")], false).unwrap();
        assert_eq!(load_folders(&conn, root.id).unwrap(), vec![f]);
        assert_eq!(load_assets(&conn).unwrap().len(), 1);
    }

    #[test]
    fn folder_states_round_trip_and_are_replaced() {
        let conn = db();
        let root = add_root(&conn, Path::new("/lib"), None).unwrap();
        let state = |p: &str, ns: i64| FolderState {
            path: PathBuf::from(p),
            modified_ns: ns,
            subdirs: vec!["x y".into(), "فارسی".into()],
        };
        save_scan(
            &conn,
            root.id,
            &[],
            &[state("/lib", 5), state("/lib/a", -1)],
            true,
        )
        .unwrap();
        let mut got = load_folders(&conn, root.id).unwrap();
        got.sort_by(|a, b| a.path.cmp(&b.path));
        assert_eq!(got, vec![state("/lib", 5), state("/lib/a", -1)]);
        save_scan(&conn, root.id, &[], &[state("/lib", 9)], true).unwrap();
        assert_eq!(
            load_folders(&conn, root.id).unwrap(),
            vec![state("/lib", 9)]
        );
    }

    #[test]
    fn removing_a_root_removes_its_assets_folders_and_caches() {
        let conn = db();
        let r1 = add_root(&conn, Path::new("/one"), None).unwrap();
        let r2 = add_root(&conn, Path::new("/two"), None).unwrap();
        let a = asset(r1.id, 1, "/one/a.7z");
        let b = asset(r2.id, 2, "/two/b.7z");
        save_scan(&conn, r1.id, &[a.clone()], &[], true).unwrap();
        save_scan(&conn, r2.id, &[b.clone()], &[], true).unwrap();
        put_package_files(
            &conn,
            a.id,
            a.file_size,
            1,
            &[PackageFile {
                path: "x".into(),
                size: 1,
                crc: Some(1),
            }],
        )
        .unwrap();
        put_archive_hash(
            &conn,
            a.id,
            &ArchiveHash {
                size: 1,
                modified_ns: 1,
                quick: Some("q".into()),
                full: None,
            },
        )
        .unwrap();
        assert!(remove_root(&conn, r1.id).unwrap());
        assert_eq!(load_assets(&conn).unwrap().len(), 1);
        assert!(get_archive_hash(&conn, a.id).unwrap().is_none());
        assert!(package_files_version(&conn, a.id).unwrap().is_none());
        assert_eq!(list_roots(&conn).unwrap().len(), 1);
        assert!(!remove_root(&conn, r1.id).unwrap());
    }

    #[test]
    fn caches_follow_the_asset() {
        let conn = db();
        let root = add_root(&conn, Path::new("/lib"), None).unwrap();
        let a = asset(root.id, 1, "/lib/a.7z");
        save_scan(&conn, root.id, &[a.clone()], &[], true).unwrap();
        let files = vec![
            PackageFile {
                path: "f/a.ttf".into(),
                size: 10,
                crc: Some(0xDEADBEEF),
            },
            PackageFile {
                path: "b.txt".into(),
                size: 2,
                crc: None,
            },
        ];
        put_package_files(&conn, a.id, a.file_size, 42, &files).unwrap();
        assert_eq!(
            package_files_version(&conn, a.id).unwrap(),
            Some((a.file_size, 42))
        );
        let loaded = load_package_files(&conn).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].1.len(), 2);
        assert!(loaded[0].1.iter().any(|f| f.crc == Some(0xDEADBEEF)));
        // the archive changed size: its cached list is no longer trusted
        let mut changed = a.clone();
        changed.file_size += 1;
        save_scan(&conn, root.id, &[changed], &[], true).unwrap();
        assert!(load_package_files(&conn).unwrap().is_empty());
        // pruning an asset drops its caches
        save_scan(&conn, root.id, &[], &[], true).unwrap();
        assert!(package_files_version(&conn, a.id).unwrap().is_none());
    }

    #[test]
    fn an_empty_archive_is_remembered_as_analysed() {
        let conn = db();
        let id = Uuid::from_u128(9);
        put_package_files(&conn, id, 5, 6, &[]).unwrap();
        assert_eq!(package_files_version(&conn, id).unwrap(), Some((5, 6)));
    }

    #[test]
    fn migration_is_idempotent() {
        let conn = db();
        assert!(MigrationRunner::run(&conn).unwrap().is_empty());
        assert!(MigrationRunner::pending(&conn).unwrap().is_empty());
    }
}
