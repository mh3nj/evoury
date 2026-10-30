use super::Migration;

/// The persistent library index: library folders, the assets found in them, what each
/// folder looked like at the last scan (for incremental scans), and the caches that
/// duplicate detection builds.
pub const MIGRATION: Migration = Migration {
    version: 2,
    name: "library_index",
    sql: r#"
CREATE TABLE IF NOT EXISTS library_roots (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE COLLATE NOCASE,
    name TEXT NOT NULL,
    added_at TEXT NOT NULL,
    last_scanned_at TEXT
);

CREATE TABLE IF NOT EXISTS library_assets (
    id TEXT PRIMARY KEY,
    root_id TEXT NOT NULL,
    name TEXT NOT NULL,
    archive_type TEXT NOT NULL,
    archive_path TEXT NOT NULL,
    preview_path TEXT,
    thumb_path TEXT,
    file_size INTEGER NOT NULL,
    modified_at TEXT NOT NULL,
    collection_id TEXT,
    category_id TEXT,
    state TEXT NOT NULL,
    FOREIGN KEY (root_id) REFERENCES library_roots(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_library_assets_root ON library_assets(root_id);

CREATE TABLE IF NOT EXISTS library_folders (
    root_id TEXT NOT NULL,
    path TEXT NOT NULL,
    modified_ns INTEGER NOT NULL,
    subdirs TEXT NOT NULL,
    PRIMARY KEY (root_id, path),
    FOREIGN KEY (root_id) REFERENCES library_roots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS archive_hashes (
    asset_id TEXT PRIMARY KEY,
    size INTEGER NOT NULL,
    modified_ns INTEGER NOT NULL,
    quick TEXT,
    full TEXT
);

CREATE TABLE IF NOT EXISTS package_files (
    asset_id TEXT NOT NULL,
    archive_size INTEGER NOT NULL,
    archive_modified_ns INTEGER NOT NULL,
    path TEXT NOT NULL,
    size INTEGER NOT NULL,
    crc INTEGER,
    PRIMARY KEY (asset_id, path)
);
CREATE INDEX IF NOT EXISTS idx_package_files_content ON package_files(size, crc);
"#,
    rollback: Some(
        r#"
DROP TABLE IF EXISTS package_files;
DROP TABLE IF EXISTS archive_hashes;
DROP TABLE IF EXISTS library_folders;
DROP TABLE IF EXISTS library_assets;
DROP TABLE IF EXISTS library_roots;
"#,
    ),
};
