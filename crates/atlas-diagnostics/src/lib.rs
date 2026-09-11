pub mod collector;
pub mod report;

pub use collector::DiagnosticCollector;
pub use report::{CrashReport, DiagnosticBundle, SystemInfo};
