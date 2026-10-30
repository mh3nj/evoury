//! Files that appear in more than one package, and packages that mostly contain the
//! same files (the same pack saved twice, a font bundled into twelve mockups).
//!
//! A file is identified by (size, CRC32). That is not cryptographic, so these results are
//! *candidates* shown to a person, never something to delete automatically.

use atlas_core::PackageFile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PackageContent {
    pub id: Uuid,
    pub files: Vec<PackageFile>,
}

#[derive(Debug, Clone, Copy)]
pub struct SharedOptions {
    /// Ignore files smaller than this (tiny READMEs and licenses are in everything).
    pub min_file_size: u64,
    /// A file in more packages than this is boilerplate, not duplication.
    pub max_packages_per_file: usize,
    /// A pair is "similar" when this share of the smaller package is common.
    pub min_ratio: f32,
    pub min_shared_files: usize,
}

impl Default for SharedOptions {
    fn default() -> Self {
        Self {
            min_file_size: 1024,
            max_packages_per_file: 40,
            min_ratio: 0.5,
            min_shared_files: 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedFile {
    pub size: u64,
    pub crc: u32,
    /// A few of the names this file has (it may be called differently in each package).
    pub names: Vec<String>,
    pub assets: Vec<Uuid>,
}

impl SharedFile {
    pub fn wasted_bytes(&self) -> u64 {
        self.size * (self.assets.len() as u64).saturating_sub(1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimilarPair {
    pub a: Uuid,
    pub b: Uuid,
    pub shared_files: usize,
    pub shared_bytes: u64,
    /// `shared_bytes` as a share of the smaller package, 0..=1.
    pub ratio: f32,
    /// Same files in both (typically the same pack in two formats or two folders).
    pub identical: bool,
}

type Key = (u64, u32);

fn keyed(files: &[PackageFile], min_size: u64) -> HashMap<Key, &PackageFile> {
    let mut map = HashMap::new();
    for f in files {
        if let (Some(crc), true) = (f.crc, f.size >= min_size) {
            map.entry((f.size, crc)).or_insert(f);
        }
    }
    map
}

/// Files that occur in at least two packages, biggest waste first.
pub fn find_shared_files(packages: &[PackageContent], opts: &SharedOptions) -> Vec<SharedFile> {
    let mut index: HashMap<Key, (Vec<Uuid>, Vec<String>)> = HashMap::new();
    for pkg in packages {
        for (key, file) in keyed(&pkg.files, opts.min_file_size) {
            let entry = index.entry(key).or_default();
            entry.0.push(pkg.id);
            let name = file
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&file.path)
                .to_string();
            if entry.1.len() < 5 && !entry.1.contains(&name) {
                entry.1.push(name);
            }
        }
    }
    let mut out: Vec<SharedFile> = index
        .into_iter()
        .filter(|(_, (ids, _))| ids.len() >= 2 && ids.len() <= opts.max_packages_per_file)
        .map(|((size, crc), (mut assets, names))| {
            assets.sort();
            SharedFile {
                size,
                crc,
                names,
                assets,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.wasted_bytes()
            .cmp(&a.wasted_bytes())
            .then(a.size.cmp(&b.size))
            .then(a.crc.cmp(&b.crc))
    });
    out
}

/// Pairs of packages that mostly contain the same files, most similar first.
pub fn find_similar(packages: &[PackageContent], opts: &SharedOptions) -> Vec<SimilarPair> {
    let maps: Vec<HashMap<Key, &PackageFile>> = packages
        .iter()
        .map(|p| keyed(&p.files, opts.min_file_size))
        .collect();
    let totals: Vec<u64> = maps
        .iter()
        .map(|m| m.keys().map(|(s, _)| *s).sum())
        .collect();

    let mut index: HashMap<Key, Vec<usize>> = HashMap::new();
    for (i, map) in maps.iter().enumerate() {
        for key in map.keys() {
            index.entry(*key).or_default().push(i);
        }
    }

    let mut pairs: HashMap<(usize, usize), (usize, u64)> = HashMap::new();
    for (key, owners) in &index {
        if owners.len() < 2 || owners.len() > opts.max_packages_per_file {
            continue;
        }
        for (x, &i) in owners.iter().enumerate() {
            for &j in &owners[x + 1..] {
                let entry = pairs.entry((i.min(j), i.max(j))).or_default();
                entry.0 += 1;
                entry.1 += key.0;
            }
        }
    }

    let mut out: Vec<SimilarPair> = pairs
        .into_iter()
        .filter_map(|((i, j), (files, bytes))| {
            let smaller = totals[i].min(totals[j]);
            if smaller == 0 || files < opts.min_shared_files {
                return None;
            }
            let ratio = (bytes as f64 / smaller as f64).min(1.0) as f32;
            (ratio >= opts.min_ratio).then(|| SimilarPair {
                a: packages[i].id,
                b: packages[j].id,
                shared_files: files,
                shared_bytes: bytes,
                ratio,
                identical: files == maps[i].len() && files == maps[j].len(),
            })
        })
        .collect();
    out.sort_by(|x, y| {
        y.ratio
            .partial_cmp(&x.ratio)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(y.shared_bytes.cmp(&x.shared_bytes))
            .then(x.a.cmp(&y.a))
            .then(x.b.cmp(&y.b))
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(path: &str, size: u64, crc: Option<u32>) -> PackageFile {
        PackageFile {
            path: path.into(),
            size,
            crc,
        }
    }
    fn pkg(n: u128, files: Vec<PackageFile>) -> PackageContent {
        PackageContent {
            id: Uuid::from_u128(n),
            files,
        }
    }
    fn id(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn a_font_bundled_in_several_packages_is_found_once_with_all_owners() {
        let font = |dir: &str| f(&format!("{dir}/Inter.ttf"), 300_000, Some(0xAAAA));
        let packages = vec![
            pkg(1, vec![font("fonts"), f("a.psd", 9000, Some(1))]),
            pkg(2, vec![font("assets"), f("b.psd", 9000, Some(2))]),
            pkg(3, vec![f("Inter Regular.ttf", 300_000, Some(0xAAAA))]),
            pkg(4, vec![f("c.psd", 5000, Some(3))]),
        ];
        let shared = find_shared_files(&packages, &SharedOptions::default());
        assert_eq!(shared.len(), 1);
        assert_eq!(shared[0].assets, vec![id(1), id(2), id(3)]);
        assert_eq!(shared[0].wasted_bytes(), 600_000);
        assert!(shared[0].names.contains(&"Inter.ttf".to_string()));
        assert!(shared[0].names.contains(&"Inter Regular.ttf".to_string()));
    }

    #[test]
    fn boilerplate_tiny_files_and_unknown_crcs_are_ignored() {
        let mut packages = Vec::new();
        for n in 0..50u128 {
            packages.push(pkg(
                n,
                vec![
                    f("LICENSE.txt", 4000, Some(7)), // in all 50: boilerplate
                    f("tiny.txt", 10, Some(8)),      // below the size floor
                    f("nocrc.bin", 99_999, None),    // cannot be compared
                ],
            ));
        }
        assert!(find_shared_files(&packages, &SharedOptions::default()).is_empty());
    }

    #[test]
    fn same_size_different_content_is_not_shared() {
        let packages = vec![
            pkg(1, vec![f("a.bin", 5000, Some(1))]),
            pkg(2, vec![f("a.bin", 5000, Some(2))]),
        ];
        assert!(find_shared_files(&packages, &SharedOptions::default()).is_empty());
    }

    #[test]
    fn a_pack_saved_twice_is_an_identical_pair() {
        let files = || {
            vec![
                f("a.psd", 9000, Some(1)),
                f("b.png", 7000, Some(2)),
                f("c.ai", 5000, Some(3)),
            ]
        };
        let packages = vec![
            pkg(1, files()),
            pkg(2, files()),
            pkg(3, vec![f("z.psd", 8000, Some(9))]),
        ];
        let pairs = find_similar(&packages, &SharedOptions::default());
        assert_eq!(pairs.len(), 1);
        assert_eq!((pairs[0].a, pairs[0].b), (id(1), id(2)));
        assert!(pairs[0].identical);
        assert_eq!((pairs[0].shared_files, pairs[0].shared_bytes), (3, 21_000));
        assert!((pairs[0].ratio - 1.0).abs() < 1e-6);
    }

    #[test]
    fn a_small_pack_inside_a_big_one_is_similar_but_not_identical() {
        let big = pkg(
            1,
            vec![
                f("a", 10_000, Some(1)),
                f("b", 10_000, Some(2)),
                f("c", 10_000, Some(3)),
                f("d", 10_000, Some(4)),
            ],
        );
        let small = pkg(
            2,
            vec![
                f("a", 10_000, Some(1)),
                f("b", 10_000, Some(2)),
                f("x", 2_000, Some(99)),
            ],
        );
        let pairs = find_similar(&[big, small], &SharedOptions::default());
        assert_eq!(pairs.len(), 1);
        assert!(!pairs[0].identical);
        assert!(
            pairs[0].ratio > 0.9 && pairs[0].ratio < 1.0,
            "20000 of 22000 = {}",
            pairs[0].ratio
        );
    }

    #[test]
    fn weak_overlap_and_single_shared_files_are_not_reported() {
        let a = pkg(1, vec![f("a", 10_000, Some(1)), f("b", 90_000, Some(2))]);
        let b = pkg(2, vec![f("a", 10_000, Some(1)), f("c", 90_000, Some(3))]);
        // 10% shared
        assert!(find_similar(&[a.clone(), b.clone()], &SharedOptions::default()).is_empty());
        // a single shared file never counts, however large the share
        let c = pkg(3, vec![f("a", 10_000, Some(1))]);
        assert!(find_similar(&[a, c], &SharedOptions::default()).is_empty());
    }

    #[test]
    fn most_similar_first() {
        let base = || {
            vec![
                f("a", 10_000, Some(1)),
                f("b", 10_000, Some(2)),
                f("c", 10_000, Some(3)),
            ]
        };
        let near = {
            let mut v = base();
            v.push(f("n", 1_500, Some(50)));
            v
        };
        let packages = vec![pkg(1, base()), pkg(2, base()), pkg(3, near)];
        let pairs = find_similar(&packages, &SharedOptions::default());
        assert_eq!((pairs[0].a, pairs[0].b), (id(1), id(2)));
        assert!(pairs.len() >= 2);
        assert!(pairs[0].ratio >= pairs[1].ratio);
    }
}
