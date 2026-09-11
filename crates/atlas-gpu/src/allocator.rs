use crate::tracker::GpuTracker;
use crate::texture::TextureHandle;

pub struct GpuAllocator;

impl GpuAllocator {
    pub fn allocate(tracker: &mut GpuTracker, handle: TextureHandle) -> bool {
        tracker.track(handle);
        if tracker.over_budget() {
            let evicted = tracker.evict_oldest(5);
            if evicted == 0 {
                return false;
            }
        }
        true
    }

    pub fn deallocate(tracker: &mut GpuTracker, id: &uuid::Uuid) {
        tracker.untrack(id);
    }
}
