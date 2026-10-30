use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preview {
    pub path: PathBuf,
    /// Small grid thumbnail (`Name.thumb.avif` from Kaelovun), when one exists.
    #[serde(default)]
    pub thumb: Option<PathBuf>,
}
