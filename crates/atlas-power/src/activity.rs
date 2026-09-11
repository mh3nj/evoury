use std::time::{Duration, Instant};

pub struct ActivityTracker {
    last_activity: Instant,
    idle_threshold: Duration,
    sleep_threshold: Duration,
}

impl ActivityTracker {
    pub fn new() -> Self {
        Self {
            last_activity: Instant::now(),
            idle_threshold: Duration::from_secs(300),
            sleep_threshold: Duration::from_secs(1800),
        }
    }

    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }

    pub fn idle_for(&self) -> Duration {
        Instant::now().duration_since(self.last_activity)
    }

    pub fn is_active(&self) -> bool {
        self.idle_for() < self.idle_threshold
    }

    pub fn is_idle(&self) -> bool {
        let idle = self.idle_for();
        idle >= self.idle_threshold && idle < self.sleep_threshold
    }

    pub fn is_sleeping(&self) -> bool {
        self.idle_for() >= self.sleep_threshold
    }

    pub fn set_idle_threshold(&mut self, secs: u64) {
        self.idle_threshold = Duration::from_secs(secs);
    }

    pub fn set_sleep_threshold(&mut self, secs: u64) {
        self.sleep_threshold = Duration::from_secs(secs);
    }
}
