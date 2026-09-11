use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::{ActivityTracker, PowerMode};

pub struct SleepManager {
    tracker: Arc<Mutex<ActivityTracker>>,
    on_mode_change: Option<Box<dyn Fn(PowerMode) + Send + Sync>>,
}

impl SleepManager {
    pub fn new() -> Self {
        Self {
            tracker: Arc::new(Mutex::new(ActivityTracker::new())),
            on_mode_change: None,
        }
    }

    pub fn touch(&self) {
        self.tracker.lock().unwrap().touch();
    }

    pub fn mode(&self) -> PowerMode {
        let tracker = self.tracker.lock().unwrap();
        if tracker.is_active() {
            PowerMode::Active
        } else if tracker.is_idle() {
            PowerMode::Idle
        } else {
            PowerMode::Sleeping
        }
    }

    pub fn on_mode_change<F>(&mut self, callback: F)
    where
        F: Fn(PowerMode) + Send + Sync + 'static,
    {
        self.on_mode_change = Some(Box::new(callback));
    }

    pub fn start_monitoring(&self, interval_secs: u64) {
        let tracker = self.tracker.clone();
        let mut last_mode = self.mode();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(interval_secs));
                let current = {
                    let t = tracker.lock().unwrap();
                    if t.is_active() {
                        PowerMode::Active
                    } else if t.is_idle() {
                        PowerMode::Idle
                    } else {
                        PowerMode::Sleeping
                    }
                };

                if current != last_mode {
                    last_mode = current;
                }
            }
        });
    }
}
