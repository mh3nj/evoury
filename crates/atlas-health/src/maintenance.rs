use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaintenanceTask {
    VacuumOrphanedMetadata,
    RebuildMissingPreviews,
    ReindexAll,
    CleanupOldVersions { keep: usize },
    PruneActivityLog { older_than_days: i64 },
    CompactCache,
    VerifyFileIntegrity,
    RepairBrokenReferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceReport {
    pub task: MaintenanceTask,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub items_processed: usize,
    pub items_fixed: usize,
    pub errors: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub auto_run: bool,
    pub interval_hours: u32,
    pub tasks: Vec<MaintenanceTask>,
}

impl Default for MaintenanceSchedule {
    fn default() -> Self {
        Self {
            auto_run: true,
            interval_hours: 24,
            tasks: vec![
                MaintenanceTask::VacuumOrphanedMetadata,
                MaintenanceTask::PruneActivityLog { older_than_days: 90 },
                MaintenanceTask::CompactCache,
            ],
        }
    }
}

pub struct MaintenanceEngine {
    reports: Vec<MaintenanceReport>,
    schedule: MaintenanceSchedule,
}

impl MaintenanceEngine {
    pub fn new() -> Self {
        Self { reports: Vec::new(), schedule: MaintenanceSchedule::default() }
    }

    pub fn run_task(&mut self, task: &MaintenanceTask) -> MaintenanceReport {
        let started = chrono::Utc::now();
        let (processed, fixed, errors): (usize, usize, Vec<String>) = match task {
            MaintenanceTask::VacuumOrphanedMetadata => (0, 0, vec![]),
            MaintenanceTask::RebuildMissingPreviews => (0, 0, vec![]),
            MaintenanceTask::ReindexAll => (0, 0, vec![]),
            MaintenanceTask::CleanupOldVersions { keep: _ } => (0, 0, vec![]),
            MaintenanceTask::PruneActivityLog { older_than_days: _ } => (0, 0, vec![]),
            MaintenanceTask::CompactCache => (0, 0, vec![]),
            MaintenanceTask::VerifyFileIntegrity => (0, 0, vec![]),
            MaintenanceTask::RepairBrokenReferences => (0, 0, vec![]),
        };
        let success = errors.is_empty();
        let report = MaintenanceReport {
            task: task.clone(),
            started_at: started,
            completed_at: chrono::Utc::now(),
            items_processed: processed,
            items_fixed: fixed,
            errors,
            success,
        };
        self.reports.push(report.clone());
        report
    }

    pub fn run_all(&mut self) -> Vec<MaintenanceReport> {
        let tasks = self.schedule.tasks.clone();
        tasks.iter().map(|t| self.run_task(t)).collect()
    }

    pub fn schedule(&self) -> &MaintenanceSchedule { &self.schedule }
    pub fn schedule_mut(&mut self) -> &mut MaintenanceSchedule { &mut self.schedule }
    pub fn reports(&self) -> &Vec<MaintenanceReport> { &self.reports }
    pub fn last_report(&self) -> Option<&MaintenanceReport> { self.reports.last() }
    pub fn reports_since(&self, since: &chrono::DateTime<chrono::Utc>) -> Vec<&MaintenanceReport> {
        self.reports.iter().filter(|r| r.completed_at > *since).collect()
    }
}
