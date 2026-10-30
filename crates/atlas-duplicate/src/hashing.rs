//! Hashing archive files for exact-duplicate detection.
//!
//! Reading every archive in a large library end to end is slow, so detection is staged:
//! files with different sizes cannot match, files with different *quick* signatures
//! (size plus three 1 MiB samples) cannot match, and only the survivors are hashed in
//! full.

use blake2::{Blake2b512, Blake2s256, Digest};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

const SAMPLE: u64 = 1 << 20;

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut out, b| {
            let _ = write!(out, "{b:02x}");
            out
        })
}

/// Size plus the BLAKE2 hash of the first, middle and last MiB. Cheap, and two files
/// that differ anywhere in those samples (or in size) are certainly different.
pub fn quick_signature(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let size = file.metadata()?.len();
    let mut hasher = Blake2s256::new();
    hasher.update(size.to_le_bytes());

    let mut buf = vec![0u8; SAMPLE as usize];
    let offsets: Vec<u64> = if size <= SAMPLE * 3 {
        vec![0]
    } else {
        vec![0, size / 2 - SAMPLE / 2, size - SAMPLE]
    };
    for offset in offsets {
        file.seek(SeekFrom::Start(offset))?;
        let mut left = if size <= SAMPLE * 3 { size } else { SAMPLE };
        while left > 0 {
            let want = left.min(buf.len() as u64) as usize;
            let n = file.read(&mut buf[..want])?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
            left -= n as u64;
        }
    }
    Ok(format!("{size}-{}", hex(&hasher.finalize())))
}

/// BLAKE2b-512 of the whole file. `cancel` is polled between chunks.
pub fn full_hash(path: &Path, cancel: &dyn Fn() -> bool) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Blake2b512::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        if cancel() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled"));
        }
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(label: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("atlas-dup-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn identical_files_match_and_different_ones_do_not() {
        let d = temp("hash");
        std::fs::write(d.join("a"), b"hello world").unwrap();
        std::fs::write(d.join("b"), b"hello world").unwrap();
        std::fs::write(d.join("c"), b"hello there").unwrap();
        let never = || false;
        assert_eq!(
            quick_signature(&d.join("a")).unwrap(),
            quick_signature(&d.join("b")).unwrap()
        );
        assert_ne!(
            quick_signature(&d.join("a")).unwrap(),
            quick_signature(&d.join("c")).unwrap()
        );
        assert_eq!(
            full_hash(&d.join("a"), &never).unwrap(),
            full_hash(&d.join("b"), &never).unwrap()
        );
        assert_ne!(
            full_hash(&d.join("a"), &never).unwrap(),
            full_hash(&d.join("c"), &never).unwrap()
        );
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn large_files_differing_only_between_the_samples_are_caught_by_the_full_hash() {
        let d = temp("big");
        let size = 5 * 1024 * 1024usize;
        let a = vec![7u8; size];
        let mut b = a.clone();
        b[size / 2 + 700_000] ^= 0xFF; // outside the three sampled MiBs
        std::fs::write(d.join("a"), &a).unwrap();
        std::fs::write(d.join("b"), &b).unwrap();
        let never = || false;
        assert_eq!(
            quick_signature(&d.join("a")).unwrap(),
            quick_signature(&d.join("b")).unwrap()
        );
        assert_ne!(
            full_hash(&d.join("a"), &never).unwrap(),
            full_hash(&d.join("b"), &never).unwrap()
        );
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn size_is_part_of_the_signature() {
        let d = temp("size");
        std::fs::write(d.join("a"), vec![1u8; 10]).unwrap();
        std::fs::write(d.join("b"), vec![1u8; 11]).unwrap();
        assert_ne!(
            quick_signature(&d.join("a")).unwrap(),
            quick_signature(&d.join("b")).unwrap()
        );
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn cancel_stops_the_full_hash() {
        let d = temp("cancel");
        std::fs::write(d.join("a"), b"x").unwrap();
        let err = full_hash(&d.join("a"), &|| true).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::Interrupted);
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn missing_file_is_an_error() {
        assert!(quick_signature(Path::new("/no/such/file")).is_err());
    }
}
