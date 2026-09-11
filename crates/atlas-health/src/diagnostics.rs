use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub worker_count: usize,
    pub active_workers: usize,
    pub pending_jobs: usize,
    pub running_jobs: usize,
    pub registered_commands: usize,
    pub asset_count: usize,
    pub cache_entries: usize,
    pub power_mode: String,
    pub uptime_seconds: u64,
}

pub struct DiagnosticsCollector;

impl DiagnosticsCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(
        &self,
        worker_count: usize,
        active_workers: usize,
        pending_jobs: usize,
        running_jobs: usize,
        registered_commands: usize,
        asset_count: usize,
        cache_entries: usize,
        power_mode: &str,
        uptime_seconds: u64,
    ) -> DiagnosticsSnapshot {
        DiagnosticsSnapshot {
            timestamp: chrono::Utc::now(),
            worker_count,
            active_workers,
            pending_jobs,
            running_jobs,
            registered_commands,
            asset_count,
            cache_entries,
            power_mode: power_mode.to_string(),
            uptime_seconds,
        }
    }
}
