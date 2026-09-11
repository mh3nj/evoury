use serde::{Deserialize, Serialize};
use crate::issue::HealthIssue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub scanned: usize,
    pub issues: Vec<HealthIssue>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthReport {
    pub fn healthy(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.severity, crate::IssueSeverity::Error | crate::IssueSeverity::Critical)).count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.severity, crate::IssueSeverity::Warning)).count()
    }

    pub fn info_count(&self) -> usize {
        self.issues.iter().filter(|i| matches!(i.severity, crate::IssueSeverity::Info)).count()
    }
}
