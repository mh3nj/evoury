pub mod canvas;
pub mod diff;
pub mod engine;
pub mod mode;

pub use canvas::CompareCanvas;
pub use diff::{DiffResult, DiffRegion};
pub use engine::CompareEngine;
pub use mode::CompareMode;
