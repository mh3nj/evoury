//! "What is inside this archive?" without unpacking it.
//!
//! Two sources, best first:
//!
//! 1. **Kaelovun's sidecar** (`<archive>.kaelovun.json`). Instant, works for RAR,
//!    needs no tool, and carries file categories. It is trusted only if it still
//!    describes this exact archive (name and size match).
//! 2. **7-Zip** (`7z l -slt`), which lists ZIP, 7z, RAR and TAR. Listing only: nothing
//!    is ever extracted, and the archive path is passed after `--`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Entries returned to the UI at most. A 300,000-file archive should not freeze a webview.
pub const DEFAULT_MAX_ENTRIES: usize = 20_000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentEntry {
    /// Path inside the archive, always `/`-separated.
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    /// `transformable`, `preservable`, `container`, `license`... (sidecar only).
    #[serde(default)]
    pub category: Option<String>,
    /// CRC32 of the file's contents, when the manifest or 7-Zip reports one. Used to find
    /// the same file in several packages.
    #[serde(default)]
    pub crc: Option<u32>,
}

impl ContentEntry {
    /// The part of this entry that duplicate detection cares about (files only).
    pub fn as_package_file(&self) -> Option<atlas_core::PackageFile> {
        (!self.is_dir).then(|| atlas_core::PackageFile {
            path: self.path.clone(),
            size: self.size,
            crc: self.crc,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveContents {
    /// `"sidecar"` or `"7-zip"`.
    pub source: String,
    pub entries: Vec<ContentEntry>,
    pub total_files: usize,
    pub total_size: u64,
    /// True when `entries` was cut at the limit (the totals still cover everything).
    pub truncated: bool,
    /// Kaelovun's name for the package, when the sidecar has one.
    #[serde(default)]
    pub package_name: Option<String>,
}

/// Path of the sidecar belonging to `archive`.
pub fn sidecar_path(archive: &Path) -> PathBuf {
    let mut name = archive.file_name().unwrap_or_default().to_os_string();
    name.push(".kaelovun.json");
    archive.with_file_name(name)
}

/// List the contents of `archive`.
pub fn read_contents(archive: &Path, max_entries: usize) -> Result<ArchiveContents, String> {
    if !archive.is_file() {
        return Err(format!("Archive not found: {}", archive.display()));
    }
    if let Some(found) = from_sidecar(archive, max_entries) {
        return Ok(found);
    }
    from_seven_zip(archive, max_entries)
}

/// True when `archive` can be listed without any external tool (it has a valid manifest).
pub fn has_sidecar_listing(archive: &Path) -> bool {
    from_sidecar(archive, 1).is_some()
}

// ── sidecar ──────────────────────────────────────────────────

#[derive(Deserialize)]
struct Sidecar {
    #[serde(default)]
    archive: String,
    #[serde(default)]
    archive_bytes: u64,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    files: Vec<SidecarFile>,
}

#[derive(Deserialize)]
struct SidecarFile {
    path: String,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    crc32: Option<String>,
}

fn from_sidecar(archive: &Path, max_entries: usize) -> Option<ArchiveContents> {
    let raw = std::fs::read_to_string(sidecar_path(archive)).ok()?;
    let sidecar: Sidecar = serde_json::from_str(&raw).ok()?;

    // A sidecar that describes some other file (the archive was replaced or renamed
    // by hand) must not be believed.
    let actual_name = archive.file_name()?.to_string_lossy();
    let actual_size = std::fs::metadata(archive).ok()?.len();
    if sidecar.archive != actual_name || sidecar.archive_bytes != actual_size {
        return None;
    }
    // Without sizes (older sidecars) 7-Zip gives a better answer.
    if sidecar.files.is_empty() || sidecar.files.iter().any(|f| f.size.is_none()) {
        return None;
    }

    let mut entries: Vec<ContentEntry> = sidecar
        .files
        .into_iter()
        .map(|f| ContentEntry {
            path: f.path.replace('\\', "/"),
            size: f.size.unwrap_or(0),
            is_dir: false,
            category: f.category,
            crc: f
                .crc32
                .as_deref()
                .and_then(|c| u32::from_str_radix(c.trim(), 16).ok()),
        })
        .collect();
    entries.sort_by_key(|a| a.path.to_lowercase());
    Some(finish("sidecar", entries, max_entries, sidecar.name))
}

// ── 7-Zip ────────────────────────────────────────────────────

/// Where to look for 7-Zip: `EVOURY_7Z`, PATH, then the usual install folders.
pub fn find_seven_zip() -> Option<PathBuf> {
    if let Some(custom) = std::env::var_os("EVOURY_7Z") {
        let p = PathBuf::from(custom);
        if p.is_file() {
            return Some(p);
        }
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path) {
            for exe in ["7z", "7z.exe", "7zz", "7zz.exe"] {
                candidates.push(dir.join(exe));
            }
        }
    }
    for root in ["C:\\Program Files\\7-Zip", "C:\\Program Files (x86)\\7-Zip"] {
        candidates.push(Path::new(root).join("7z.exe"));
    }
    candidates.into_iter().find(|p| p.is_file())
}

fn from_seven_zip(archive: &Path, max_entries: usize) -> Result<ArchiveContents, String> {
    let exe = find_seven_zip().ok_or_else(|| {
        "Listing this archive needs 7-Zip (https://www.7-zip.org). Install it, or set the \
         EVOURY_7Z environment variable to the path of 7z.exe. Archives made with \
         Kaelovun's \"sidecar\" option can be listed without it."
            .to_string()
    })?;

    let mut command = Command::new(&exe);
    command
        .args(["l", "-slt", "-y", "-sccUTF-8"])
        .arg("--")
        .arg(archive)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW: no console flash
    }
    let output = command
        .output()
        .map_err(|e| format!("Could not run {}: {e}", exe.display()))?;

    let text = String::from_utf8_lossy(&output.stdout);
    // 0 = ok, 1 = warning (the listing is still complete).
    if !matches!(output.status.code(), Some(0) | Some(1)) {
        let blob = format!("{text}\n{}", String::from_utf8_lossy(&output.stderr)).to_lowercase();
        return Err(if blob.contains("password") || blob.contains("encrypted") {
            "This archive is password-protected, so its contents can't be listed.".to_string()
        } else {
            format!(
                "7-Zip could not read this archive (exit code {:?}).",
                output.status.code()
            )
        });
    }
    let entries = parse_slt(&text);
    if entries.is_empty() {
        return Err("7-Zip found nothing to list in this archive.".to_string());
    }
    Ok(finish("7-zip", entries, max_entries, None))
}

/// Parse `7z l -slt` output.
pub fn parse_slt(text: &str) -> Vec<ContentEntry> {
    let mut entries = Vec::new();
    let mut in_entries = false;
    let mut block: Vec<(String, String)> = Vec::new();

    let flush = |block: &mut Vec<(String, String)>, entries: &mut Vec<ContentEntry>| {
        if block.is_empty() {
            return;
        }
        let get = |key: &str| {
            block
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.as_str())
        };
        if let Some(path) = get("Path") {
            let attrs = get("Attributes").unwrap_or("");
            let is_dir = get("Folder") == Some("+") || attrs.starts_with('D');
            entries.push(ContentEntry {
                path: path.replace('\\', "/").trim_end_matches('/').to_string(),
                size: get("Size").and_then(|s| s.trim().parse().ok()).unwrap_or(0),
                is_dir,
                category: None,
                crc: get("CRC").and_then(|c| u32::from_str_radix(c.trim(), 16).ok()),
            });
        }
        block.clear();
    };

    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.starts_with("----------") {
            in_entries = true;
            block.clear();
            continue;
        }
        if !in_entries {
            continue;
        }
        if line.trim().is_empty() {
            flush(&mut block, &mut entries);
            continue;
        }
        if let Some((key, value)) = line.split_once(" = ") {
            block.push((key.trim().to_string(), value.to_string()));
        } else if let Some(key) = line.strip_suffix(" =") {
            block.push((key.trim().to_string(), String::new()));
        }
    }
    flush(&mut block, &mut entries);
    entries.sort_by_key(|a| a.path.to_lowercase());
    entries
}

fn finish(
    source: &str,
    mut entries: Vec<ContentEntry>,
    max_entries: usize,
    package_name: Option<String>,
) -> ArchiveContents {
    let total_files = entries.iter().filter(|e| !e.is_dir).count();
    let total_size = entries.iter().filter(|e| !e.is_dir).map(|e| e.size).sum();
    let truncated = entries.len() > max_entries;
    entries.truncate(max_entries);
    ArchiveContents {
        source: source.to_string(),
        entries,
        total_files,
        total_size,
        truncated,
        package_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLT: &str = "7-Zip 23.01 (x64)\n\nListing archive: x.7z\n\n--\nPath = x.7z\nType = 7z\n\n----------\nPath = dir\nFolder = +\nSize = 0\nAttributes = D\n\nPath = dir\\my file.psd\nFolder = -\nSize = 4096\nPacked Size = 10\nAttributes = A\nCRC = 1A2B3C4D\n\nPath = a.txt\nFolder = -\nSize = 5\n";

    fn temp(label: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("atlas-contents-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn parses_slt_listing() {
        let e = parse_slt(SLT);
        assert_eq!(e.len(), 3);
        assert_eq!(e[0].path, "a.txt");
        let psd = e.iter().find(|x| x.path == "dir/my file.psd").unwrap();
        assert_eq!((psd.size, psd.is_dir), (4096, false));
        assert_eq!(psd.crc, Some(0x1A2B3C4D));
        assert_eq!(e[0].crc, None); // a.txt has no CRC line
        assert!(e
            .iter()
            .find(|x| x.path == "dir")
            .unwrap()
            .as_package_file()
            .is_none());
        assert!(e.iter().find(|x| x.path == "dir").unwrap().is_dir);
    }

    #[test]
    fn totals_ignore_folders_and_truncation_keeps_the_totals() {
        let c = finish("7-zip", parse_slt(SLT), 1, None);
        assert_eq!(
            (c.total_files, c.total_size, c.truncated, c.entries.len()),
            (2, 4101, true, 1)
        );
    }

    #[test]
    fn sidecar_is_used_when_it_matches_the_archive() {
        let dir = temp("ok");
        let archive = dir.join("Pack.rar");
        std::fs::write(&archive, b"12345").unwrap();
        std::fs::write(
            sidecar_path(&archive),
            r#"{"schema":1,"archive":"Pack.rar","archive_bytes":5,"name":"Pack","files":[
                {"path":"b/two.png","size":20,"category":"preservable","crc32":"0000BEEF"},
                {"path":"a/one.psd","size":10,"category":"transformable"}]}"#,
        )
        .unwrap();
        let c = read_contents(&archive, 100).unwrap();
        assert_eq!(c.source, "sidecar");
        assert_eq!(c.package_name.as_deref(), Some("Pack"));
        assert_eq!(c.entries[0].path, "a/one.psd");
        assert_eq!(c.entries[0].category.as_deref(), Some("transformable"));
        assert_eq!(c.entries[1].crc, Some(0xBEEF));
        assert_eq!(c.entries[0].crc, None);
        assert_eq!((c.total_files, c.total_size), (2, 30));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_stale_sidecar_is_not_believed() {
        let dir = temp("stale");
        let archive = dir.join("Pack.7z");
        std::fs::write(&archive, b"123456789").unwrap(); // 9 bytes, sidecar says 5
        std::fs::write(
            sidecar_path(&archive),
            r#"{"archive":"Pack.7z","archive_bytes":5,"files":[{"path":"a","size":1}]}"#,
        )
        .unwrap();
        assert!(from_sidecar(&archive, 10).is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_sidecar_for_another_archive_is_not_believed() {
        let dir = temp("other");
        let archive = dir.join("New.7z");
        std::fs::write(&archive, b"12345").unwrap();
        std::fs::write(
            sidecar_path(&archive),
            r#"{"archive":"Old.7z","archive_bytes":5,"files":[{"path":"a","size":1}]}"#,
        )
        .unwrap();
        assert!(from_sidecar(&archive, 10).is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn sidecars_without_sizes_fall_back_to_the_real_tool() {
        let dir = temp("nosize");
        let archive = dir.join("Old.zip");
        std::fs::write(&archive, b"12345").unwrap();
        std::fs::write(
            sidecar_path(&archive),
            r#"{"archive":"Old.zip","archive_bytes":5,"files":[{"path":"a"}]}"#,
        )
        .unwrap();
        assert!(from_sidecar(&archive, 10).is_none());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn missing_archive_is_a_clear_error() {
        assert!(read_contents(Path::new("/definitely/not/here.7z"), 10)
            .unwrap_err()
            .contains("not found"));
    }

    #[test]
    fn lists_a_real_archive_with_seven_zip_when_installed() {
        let Some(exe) = find_seven_zip() else { return };
        let dir = temp("real");
        std::fs::create_dir_all(dir.join("src/sub dir")).unwrap();
        std::fs::write(dir.join("src/a b.txt"), "hello").unwrap();
        std::fs::write(dir.join("src/sub dir/\u{67e5}.txt"), "x").unwrap();
        let archive = dir.join("t.7z");
        let ok = Command::new(exe)
            .current_dir(dir.join("src"))
            .args(["a", "-y"])
            .arg(&archive)
            .arg("*")
            .stdout(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        assert!(ok);
        let c = read_contents(&archive, 100).unwrap();
        assert_eq!(c.source, "7-zip");
        assert_eq!(c.total_files, 2);
        assert!(c.entries.iter().any(|e| e.path == "a b.txt" && e.size == 5));
        assert!(c
            .entries
            .iter()
            .any(|e| e.path.ends_with("sub dir/\u{67e5}.txt")));
        let _ = std::fs::remove_dir_all(dir);
    }
}
