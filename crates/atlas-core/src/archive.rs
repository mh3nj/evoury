use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArchiveType {
    Zip,
    Rar,
    SevenZip,
    Tar,
    Unknown,
}
