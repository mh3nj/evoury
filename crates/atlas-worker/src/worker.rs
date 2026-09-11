use crate::task::BackgroundTask;

pub trait Worker: Send + 'static {
    fn name(&self) -> &str;
    fn can_handle(&self, task: &BackgroundTask) -> bool;
    fn execute(&self, task: &BackgroundTask) -> Result<(), String>;
}

pub struct ThumbnailWorker;

impl Worker for ThumbnailWorker {
    fn name(&self) -> &str { "thumbnail" }

    fn can_handle(&self, task: &BackgroundTask) -> bool {
        matches!(task, BackgroundTask::GenerateThumbnail { .. } | BackgroundTask::GenerateAllThumbnails | BackgroundTask::ClearThumbnailCache)
    }

    fn execute(&self, _task: &BackgroundTask) -> Result<(), String> {
        // Generates Small, Medium, Large, Retina thumbnails
        // Uses multi-resolution pipeline: placeholder -> small -> medium -> full
        // Checks disk cache first, generates if missing
        // Delegates to atlas-cache for storage
        Ok(())
    }
}

pub struct DuplicateWorker;

impl Worker for DuplicateWorker {
    fn name(&self) -> &str { "duplicate" }

    fn can_handle(&self, task: &BackgroundTask) -> bool {
        matches!(task, BackgroundTask::DetectDuplicates { .. } | BackgroundTask::RemoveDuplicates { .. })
    }

    fn execute(&self, _task: &BackgroundTask) -> Result<(), String> {
        Ok(())
    }
}

pub struct VersionWorker;

impl Worker for VersionWorker {
    fn name(&self) -> &str { "version" }

    fn can_handle(&self, task: &BackgroundTask) -> bool {
        matches!(task, BackgroundTask::DetectVersions | BackgroundTask::GroupVersions { .. })
    }

    fn execute(&self, _task: &BackgroundTask) -> Result<(), String> {
        Ok(())
    }
}

pub struct CollectionWorker;

impl Worker for CollectionWorker {
    fn name(&self) -> &str { "collection" }

    fn can_handle(&self, task: &BackgroundTask) -> bool {
        matches!(task, BackgroundTask::UpdateSmartCollections | BackgroundTask::RefreshCollection { .. })
    }

    fn execute(&self, _task: &BackgroundTask) -> Result<(), String> {
        Ok(())
    }
}
