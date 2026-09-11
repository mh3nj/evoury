use std::path::PathBuf;
use crate::report::{CrashReport, DiagnosticBundle, SystemInfo};

pub struct DiagnosticCollector {
    pub app_version: String,
    pub app_data_path: PathBuf,
    pub crash_reports: Vec<CrashReport>,
}

impl DiagnosticCollector {
    pub fn new(app_version: &str, app_data_path: PathBuf) -> Self {
        Self {
            app_version: app_version.to_string(),
            app_data_path,
            crash_reports: Vec::new(),
        }
    }

    pub fn add_crash_report(&mut self, report: CrashReport) {
        self.crash_reports.push(report);
    }

    pub fn collect_system_info(&self) -> SystemInfo {
        let free = free_memory();
        let total = total_memory();

        SystemInfo {
            os: std::env::consts::OS.to_string(),
            os_version: std::env::consts::ARCH.to_string(),
            cpu_count: std::thread::available_parallelism()
                .map(|n| n.get()).unwrap_or(1),
            total_memory_mb: total,
            free_memory_mb: free,
            app_data_path: self.app_data_path.to_string_lossy().to_string(),
            disk_free_mb: 0,
        }
    }

    pub fn build_bundle(&self, config_snapshot: &str) -> DiagnosticBundle {
        let system_info = self.collect_system_info();
        DiagnosticBundle::new(
            self.crash_reports.clone(),
            system_info,
            config_snapshot,
        )
    }

    pub fn export_to_json(&self, config_snapshot: &str) -> Result<String, String> {
        let bundle = self.build_bundle(config_snapshot);
        serde_json::to_string_pretty(&bundle).map_err(|e| e.to_string())
    }
}

fn total_memory() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(info) = std::fs::read_to_string("/proc/meminfo") {
            for line in info.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = val.parse::<u64>() {
                            return kb / 1024;
                        }
                    }
                }
            }
        }
    }
    0
}

fn free_memory() -> u64 {
    0
}
