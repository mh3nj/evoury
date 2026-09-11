use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub app_version: String,
    pub os: String,
    pub error_message: String,
    pub error_kind: String,
    pub stack_trace: Option<String>,
    pub asset_count: Option<usize>,
    pub library_path: Option<String>,
    pub db_integrity: Option<bool>,
}

impl CrashReport {
    pub fn new(
        app_version: &str,
        error_message: &str,
        error_kind: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            app_version: app_version.to_string(),
            os: std::env::consts::OS.to_string(),
            error_message: error_message.to_string(),
            error_kind: error_kind.to_string(),
            stack_trace: None,
            asset_count: None,
            library_path: None,
            db_integrity: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticBundle {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub crash_reports: Vec<CrashReport>,
    pub system_info: SystemInfo,
    pub config_snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub os_version: String,
    pub cpu_count: usize,
    pub total_memory_mb: u64,
    pub free_memory_mb: u64,
    pub app_data_path: String,
    pub disk_free_mb: u64,
}

impl DiagnosticBundle {
    pub fn new(
        crash_reports: Vec<CrashReport>,
        system_info: SystemInfo,
        config_snapshot: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            crash_reports,
            system_info,
            config_snapshot: config_snapshot.to_string(),
        }
    }
}
