use std::path::Path;
use crate::issue::{HealthIssue, IssueSeverity, IssueType};
use crate::report::HealthReport;

pub struct HealthChecker;

impl HealthChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, assets: &[(String, Option<String>)]) -> HealthReport {
        let mut issues = Vec::new();
        let scanned = assets.len();

        for (path, preview_path) in assets {
            let p = Path::new(path);
            if !p.exists() {
                issues.push(HealthIssue {
                    asset_id: None,
                    issue_type: IssueType::MissingFile,
                    severity: IssueSeverity::Error,
                    message: format!("File not found: {}", path),
                    repair_hint: Some("Remove orphaned asset or restore file".into()),
                });
                continue;
            }

            match preview_path {
                Some(prev) if !Path::new(prev).exists() => {
                    issues.push(HealthIssue {
                        asset_id: None,
                        issue_type: IssueType::MissingPreview,
                        severity: IssueSeverity::Warning,
                        message: format!("Preview missing for: {}", path),
                        repair_hint: Some("Add an AVIF/PNG/JPG with matching filename".into()),
                    });
                }
                None => {
                    issues.push(HealthIssue {
                        asset_id: None,
                        issue_type: IssueType::UnpairedAsset,
                        severity: IssueSeverity::Info,
                        message: format!("No preview paired with: {}", path),
                        repair_hint: None,
                    });
                }
                _ => {}
            }
        }

        HealthReport {
            scanned,
            issues,
            timestamp: chrono::Utc::now(),
        }
    }
}
