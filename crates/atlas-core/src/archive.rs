//! Archive file naming rules shared by the scanner, the watcher and the UI.
//!
//! Kaelovun writes, for every package:
//!
//! ```text
//! Name.7z | Name.rar | Name.zip     the archive
//! Name.avif                         full-size preview   (optional)
//! Name.thumb.avif                   grid thumbnail      (optional)
//! Name.<archive>.kaelovun.json      sidecar manifest    (optional)
//! ```
//!
//! While an archive is still being written it carries the marker
//! `.kaelovun-part` (for example `Name.kaelovun-part.7z`). Those files are
//! unfinished and must never be indexed.

use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// Marker Kaelovun puts in the name of a file that is still being written.
pub const PARTIAL_MARKER: &str = ".kaelovun-part";

/// Fixed namespace for asset ids, so the same package gets the same id on every scan.
const ID_NAMESPACE: Uuid = Uuid::from_u128(0x6576_6f75_7279_4b61_656c_6f76_756e_0001);

/// One file inside a package, as far as duplicate detection cares.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageFile {
    /// Path inside the archive, `/`-separated.
    pub path: String,
    pub size: u64,
    /// CRC32 of the contents when the archive (or Kaelovun's manifest) reports one.
    pub crc: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchiveType {
    Zip,
    Rar,
    SevenZip,
    Tar,
    Unknown,
}

impl ArchiveType {
    /// Which format a path is, judged from its name (`Unknown` if it is not an archive).
    pub fn from_path(path: &Path) -> Self {
        parse_archive_name(path)
            .map(|n| n.kind)
            .unwrap_or(ArchiveType::Unknown)
    }

    /// Tie-break when the same package exists in several formats: higher wins.
    pub fn rank(self) -> u8 {
        match self {
            ArchiveType::SevenZip => 4,
            ArchiveType::Rar => 3,
            ArchiveType::Zip => 2,
            ArchiveType::Tar => 1,
            ArchiveType::Unknown => 0,
        }
    }
}

/// The package name and format of an archive file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveName {
    /// Name without the archive extension (`logo` for `logo.7z`, `logo.part1.rar`, `logo.tar.gz`).
    pub stem: String,
    pub kind: ArchiveType,
}

fn file_name(path: &Path) -> Option<String> {
    path.file_name().map(|n| n.to_string_lossy().into_owned())
}

/// `name` without `suffix` (ASCII, case-insensitive), or `None`.
fn strip_suffix_ci<'a>(name: &'a str, suffix: &str) -> Option<&'a str> {
    let (n, s) = (name.as_bytes(), suffix.as_bytes());
    if n.len() > s.len() && n[n.len() - s.len()..].eq_ignore_ascii_case(s) {
        // The matched bytes are ASCII, so the cut is always on a char boundary.
        Some(&name[..name.len() - s.len()])
    } else {
        None
    }
}

/// `("logo", 2)` for `logo.part2` (the `.rar` already removed).
fn split_part_suffix(stem: &str) -> Option<(&str, u32)> {
    let idx = stem.to_ascii_lowercase().rfind(".part")?;
    let digits = &stem[idx + 5..];
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    Some((&stem[..idx], digits.parse().ok()?))
}

/// True for a file that is still being written (or its temporary copy).
pub fn is_partial(path: &Path) -> bool {
    file_name(path)
        .map(|n| n.to_ascii_lowercase().contains(PARTIAL_MARKER))
        .unwrap_or(false)
}

/// Parse an archive file name.
///
/// Returns `None` for anything that is not the *primary* file of a finished archive:
/// non-archives, files still being written, and every volume of a split archive
/// except the first (`x.part2.rar`, `x.7z.002`, ...).
pub fn parse_archive_name(path: &Path) -> Option<ArchiveName> {
    let name = file_name(path)?;
    if name.starts_with("._") || is_partial(path) {
        return None;
    }

    for suffix in [
        ".tar.gz", ".tar.bz2", ".tar.xz", ".tgz", ".tbz2", ".txz", ".tar",
    ] {
        if let Some(stem) = strip_suffix_ci(&name, suffix) {
            return non_empty(stem, ArchiveType::Tar);
        }
    }

    // Numbered volumes: name.7z.001, name.zip.001, name.rar.001
    if let Some(rest) = name.rfind('.').map(|i| (&name[..i], &name[i + 1..])) {
        let (base, num) = rest;
        if num.len() == 3 && num.bytes().all(|b| b.is_ascii_digit()) {
            for (ext, kind) in [
                (".7z", ArchiveType::SevenZip),
                (".zip", ArchiveType::Zip),
                (".rar", ArchiveType::Rar),
            ] {
                if let Some(stem) = strip_suffix_ci(base, ext) {
                    return if num == "001" {
                        non_empty(stem, kind)
                    } else {
                        None
                    };
                }
            }
            return None;
        }
    }

    if let Some(stem) = strip_suffix_ci(&name, ".7z") {
        return non_empty(stem, ArchiveType::SevenZip);
    }
    if let Some(stem) = strip_suffix_ci(&name, ".zip") {
        return non_empty(stem, ArchiveType::Zip);
    }
    if let Some(stem) = strip_suffix_ci(&name, ".rar") {
        return match split_part_suffix(stem) {
            Some((base, 1)) => non_empty(base, ArchiveType::Rar),
            Some(_) => None,
            None => non_empty(stem, ArchiveType::Rar),
        };
    }
    None
}

fn non_empty(stem: &str, kind: ArchiveType) -> Option<ArchiveName> {
    if stem.is_empty() {
        None
    } else {
        Some(ArchiveName {
            stem: stem.to_string(),
            kind,
        })
    }
}

/// Image extensions that can be a preview, best first.
pub const PREVIEW_EXTENSIONS: [&str; 5] = ["avif", "webp", "jpg", "jpeg", "png"];

/// `(stem, is_thumbnail)` for a preview image (`Name.avif` -> `("Name", false)`,
/// `Name.thumb.avif` -> `("Name", true)`), or `None` if it isn't one.
pub fn parse_preview_name(path: &Path) -> Option<(String, bool, usize)> {
    let name = file_name(path)?;
    if name.starts_with("._") || is_partial(path) {
        return None;
    }
    let dot = name.rfind('.')?;
    let ext = name[dot + 1..].to_ascii_lowercase();
    let rank = PREVIEW_EXTENSIONS.iter().position(|e| *e == ext)?;
    let stem = &name[..dot];
    match strip_suffix_ci(stem, ".thumb") {
        Some(base) if !base.is_empty() => Some((base.to_string(), true, rank)),
        _ => Some((stem.to_string(), false, rank)),
    }
}

/// True for any file the library cares about (archives and preview images).
pub fn is_library_file(path: &Path) -> bool {
    parse_archive_name(path).is_some() || parse_preview_name(path).is_some()
}

/// A stable id for the package called `stem` that lives in `dir` under `root`.
///
/// `root_id` tells library folders apart so that two libraries holding `Fonts/Logo` do not
/// collide. The first library uses `Uuid::nil()`, which adds nothing to the key, so its
/// ids are the same as before multiple libraries existed.
///
/// It depends on *where the package is and what it is called*, not on the archive
/// format or a random number, so:
/// * the same package keeps its id across rescans and restarts (tags, ratings and
///   favourites stay attached), and
/// * converting `Name.rar` to `Name.7z` does not turn it into a different asset.
pub fn stable_asset_id(root_id: Uuid, root: &Path, dir: &Path, stem: &str) -> Uuid {
    let rel = dir.strip_prefix(root).unwrap_or(dir);
    let rel = rel.to_string_lossy().replace('\\', "/");
    let mut key = format!("{}/{}", rel.trim_matches('/'), stem).to_lowercase();
    if !root_id.is_nil() {
        key = format!("{root_id}/{key}");
    }
    Uuid::new_v5(&ID_NAMESPACE, key.trim_start_matches('/').as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn parse(name: &str) -> Option<(String, ArchiveType)> {
        parse_archive_name(&PathBuf::from(name)).map(|n| (n.stem, n.kind))
    }

    #[test]
    fn plain_archives() {
        assert_eq!(
            parse("Logo Pack.7z"),
            Some(("Logo Pack".into(), ArchiveType::SevenZip))
        );
        assert_eq!(parse("a.RAR"), Some(("a".into(), ArchiveType::Rar)));
        assert_eq!(parse("a.b.zip"), Some(("a.b".into(), ArchiveType::Zip)));
        assert_eq!(parse("a.tar.gz"), Some(("a".into(), ArchiveType::Tar)));
        assert_eq!(parse("a.TGZ"), Some(("a".into(), ArchiveType::Tar)));
    }

    #[test]
    fn non_archives_and_empty_stems() {
        assert_eq!(parse("a.psd"), None);
        assert_eq!(parse(".7z"), None);
        assert_eq!(parse("notes"), None);
        assert_eq!(parse("a.gz"), None); // a lone .gz is a compressed file, not a package
    }

    #[test]
    fn unfinished_files_are_never_archives() {
        assert_eq!(parse("Pack.kaelovun-part.7z"), None);
        assert_eq!(parse("Pack.KAELOVUN-PART.rar"), None);
        assert!(is_partial(Path::new("x/Pack.kaelovun-part.avif")));
        assert!(!is_partial(Path::new("x/Pack.avif")));
    }

    #[test]
    fn only_the_first_volume_counts() {
        assert_eq!(parse("v.part1.rar"), Some(("v".into(), ArchiveType::Rar)));
        assert_eq!(parse("v.part01.rar"), Some(("v".into(), ArchiveType::Rar)));
        assert_eq!(parse("v.part2.rar"), None);
        assert_eq!(parse("v.7z.001"), Some(("v".into(), ArchiveType::SevenZip)));
        assert_eq!(parse("v.7z.002"), None);
        assert_eq!(parse("v.zip.001"), Some(("v".into(), ArchiveType::Zip)));
        assert_eq!(
            parse("v.part.rar"),
            Some(("v.part".into(), ArchiveType::Rar))
        );
    }

    #[test]
    fn macos_resource_forks_are_ignored() {
        assert_eq!(parse("._Pack.7z"), None);
    }

    #[test]
    fn non_ascii_names_do_not_panic() {
        assert_eq!(
            parse("پرونده.7z"),
            Some(("پرونده".into(), ArchiveType::SevenZip))
        );
        assert_eq!(parse("İİİ.RAR"), Some(("İİİ".into(), ArchiveType::Rar)));
        assert_eq!(parse("日本語"), None);
    }

    #[test]
    fn previews_and_thumbnails() {
        let p = |n: &str| parse_preview_name(&PathBuf::from(n)).map(|(s, t, _)| (s, t));
        assert_eq!(p("Pack.avif"), Some(("Pack".into(), false)));
        assert_eq!(p("Pack.thumb.avif"), Some(("Pack".into(), true)));
        assert_eq!(p("Pack (2).PNG"), Some(("Pack (2)".into(), false)));
        assert_eq!(p("Pack.kaelovun-part.avif"), None);
        assert_eq!(p("Pack.7z"), None);
        assert_eq!(p(".thumb.avif"), Some((".thumb".into(), false)));
    }

    #[test]
    fn avif_is_preferred_over_png() {
        let rank = |n: &str| parse_preview_name(&PathBuf::from(n)).unwrap().2;
        assert!(rank("a.avif") < rank("a.webp"));
        assert!(rank("a.webp") < rank("a.png"));
    }

    #[test]
    fn ids_are_stable_and_ignore_the_archive_format() {
        let root = Path::new("/lib");
        let a = stable_asset_id(Uuid::nil(), root, Path::new("/lib/Fonts"), "Logo");
        assert_eq!(
            a,
            stable_asset_id(Uuid::nil(), root, Path::new("/lib/Fonts"), "Logo")
        );
        assert_eq!(
            a,
            stable_asset_id(Uuid::nil(), root, Path::new("/lib/fonts"), "LOGO")
        ); // case-insensitive
        assert_ne!(
            a,
            stable_asset_id(Uuid::nil(), root, Path::new("/lib/Mockups"), "Logo")
        );
        assert_ne!(
            a,
            stable_asset_id(Uuid::nil(), root, Path::new("/lib/Fonts"), "Logo 2")
        );
        // the same relative path in a second library is a different asset
        let second = Uuid::from_u128(7);
        assert_ne!(
            a,
            stable_asset_id(second, root, Path::new("/lib/Fonts"), "Logo")
        );
        assert_eq!(
            stable_asset_id(second, root, Path::new("/lib/Fonts"), "Logo"),
            stable_asset_id(
                second,
                Path::new("/moved"),
                Path::new("/moved/Fonts"),
                "Logo"
            )
        );
        // moving the whole library folder keeps every id
        assert_eq!(
            a,
            stable_asset_id(
                Uuid::nil(),
                Path::new("/other"),
                Path::new("/other/Fonts"),
                "Logo"
            )
        );
    }
}
