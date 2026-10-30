//! Archives that are byte-for-byte identical.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ArchiveRef {
    pub id: Uuid,
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactGroup {
    pub hash: String,
    pub size: u64,
    pub assets: Vec<Uuid>,
}

impl ExactGroup {
    /// Space that would be freed by keeping only one copy.
    pub fn wasted_bytes(&self) -> u64 {
        self.size * (self.assets.len() as u64).saturating_sub(1)
    }
}

/// Group byte-identical archives.
///
/// Work is staged so that most files are never read in full: equal size, then equal
/// `quick` signature, then equal `full` hash. The two closures do the hashing (and can
/// cache results); returning `None` drops that file from consideration (unreadable).
pub fn find_exact(
    archives: &[ArchiveRef],
    quick: &mut dyn FnMut(&ArchiveRef) -> Option<String>,
    full: &mut dyn FnMut(&ArchiveRef) -> Option<String>,
) -> Vec<ExactGroup> {
    let mut by_size: HashMap<u64, Vec<&ArchiveRef>> = HashMap::new();
    for a in archives.iter().filter(|a| a.size > 0) {
        by_size.entry(a.size).or_default().push(a);
    }

    let mut groups = Vec::new();
    for (size, same_size) in by_size.into_iter().filter(|(_, v)| v.len() > 1) {
        let mut by_quick: HashMap<String, Vec<&ArchiveRef>> = HashMap::new();
        for a in same_size {
            if let Some(sig) = quick(a) {
                by_quick.entry(sig).or_default().push(a);
            }
        }
        for candidates in by_quick.into_values().filter(|v| v.len() > 1) {
            let mut by_full: HashMap<String, Vec<Uuid>> = HashMap::new();
            for a in candidates {
                if let Some(hash) = full(a) {
                    by_full.entry(hash).or_default().push(a.id);
                }
            }
            for (hash, mut ids) in by_full.into_iter().filter(|(_, v)| v.len() > 1) {
                ids.sort();
                groups.push(ExactGroup {
                    hash,
                    size,
                    assets: ids,
                });
            }
        }
    }
    groups.sort_by(|a, b| {
        b.wasted_bytes()
            .cmp(&a.wasted_bytes())
            .then(a.hash.cmp(&b.hash))
    });
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn a(n: u128, size: u64) -> ArchiveRef {
        ArchiveRef {
            id: Uuid::from_u128(n),
            path: PathBuf::from(format!("/{n}")),
            size,
        }
    }

    #[test]
    fn groups_identical_archives_and_only_hashes_what_could_match() {
        let archives = vec![
            a(1, 100),
            a(2, 100),
            a(3, 100),
            a(4, 200),
            a(5, 300),
            a(6, 0),
            a(7, 0),
        ];
        let quick_calls = Cell::new(0);
        let full_calls = Cell::new(0);
        // 1 and 2 are identical, 3 only looks similar in the samples
        let groups = find_exact(
            &archives,
            &mut |x| {
                quick_calls.set(quick_calls.get() + 1);
                Some("same-samples".to_string()).filter(|_| x.size == 100)
            },
            &mut |x| {
                full_calls.set(full_calls.get() + 1);
                Some(if x.id.as_u128() == 3 { "other" } else { "h" }.to_string())
            },
        );
        assert_eq!(groups.len(), 1);
        assert_eq!(
            groups[0].assets,
            vec![Uuid::from_u128(1), Uuid::from_u128(2)]
        );
        assert_eq!(groups[0].wasted_bytes(), 100);
        assert_eq!(
            quick_calls.get(),
            3,
            "files with a unique size must not be hashed"
        );
        assert_eq!(full_calls.get(), 3);
    }

    #[test]
    fn unreadable_files_are_skipped_not_fatal() {
        let archives = vec![a(1, 10), a(2, 10)];
        let groups = find_exact(
            &archives,
            &mut |x| (x.id.as_u128() == 1).then(|| "q".into()),
            &mut |_| Some("h".into()),
        );
        assert!(groups.is_empty());
    }

    #[test]
    fn biggest_waste_first() {
        let archives = vec![a(1, 10), a(2, 10), a(3, 1000), a(4, 1000)];
        let groups = find_exact(&archives, &mut |x| Some(format!("q{}", x.size)), &mut |x| {
            Some(format!("h{}", x.size))
        });
        assert_eq!(
            groups.iter().map(|g| g.size).collect::<Vec<_>>(),
            vec![1000, 10]
        );
    }

    #[test]
    fn three_copies() {
        let archives = vec![a(1, 50), a(2, 50), a(3, 50)];
        let groups = find_exact(&archives, &mut |_| Some("q".into()), &mut |_| {
            Some("h".into())
        });
        assert_eq!(groups[0].assets.len(), 3);
        assert_eq!(groups[0].wasted_bytes(), 100);
    }
}
