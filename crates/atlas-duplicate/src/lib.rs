pub mod detector;
pub mod exact;
pub mod group;
pub mod hashing;
pub mod mode;
pub mod shared;
pub mod signature;

pub use detector::DuplicateDetector;
pub use group::DuplicateGroup;
pub use mode::DetectionMode;
pub use signature::FileSignature;

pub use exact::{find_exact, ArchiveRef, ExactGroup};
pub use hashing::{full_hash, quick_signature};
pub use shared::{
    find_shared_files, find_similar, PackageContent, SharedFile, SharedOptions, SimilarPair,
};
