use std::sync::{Arc, Mutex};
use crate::task::{BackgroundTask, TaskStatus, TaskEntry};
use crate::worker::Worker;
use uuid::Uuid;
use chrono::Utc;

pub struct WorkerPool {
    workers: Vec<Box<dyn Worker>>,
    queue: Arc<Mutex<Vec<TaskEntry>>>,
    max_concurrent: usize,
}

impl WorkerPool {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            workers: Vec::new(),
            queue: Arc::new(Mutex::new(Vec::new())),
            max_concurrent,
        }
    }

    pub fn register(&mut self, worker: Box<dyn Worker>) {
        self.workers.push(worker);
    }

    pub fn enqueue(&self, task: BackgroundTask) -> Uuid {
        let id = Uuid::new_v4();
        let entry = TaskEntry {
            id,
            priority: task.priority(),
            status: TaskStatus::Pending,
            task,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            progress: 0.0,
        };
        let mut queue = self.queue.lock().unwrap();
        queue.push(entry);
        queue.sort_by_key(|e| e.priority as u8);
        id
    }

    pub fn dequeue(&self) -> Option<TaskEntry> {
        let mut queue = self.queue.lock().unwrap();
        let pos = queue.iter().position(|e| matches!(e.status, TaskStatus::Pending))?;
        Some(queue.remove(pos))
    }

    pub fn pending_count(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.iter().filter(|e| matches!(e.status, TaskStatus::Pending)).count()
    }

    pub fn running_count(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.iter().filter(|e| matches!(e.status, TaskStatus::Running)).count()
    }

    pub fn status(&self, id: &Uuid) -> Option<TaskStatus> {
        let queue = self.queue.lock().unwrap();
        queue.iter().find(|e| e.id == *id).map(|e| e.status.clone())
    }

    pub fn can_accept_more(&self) -> bool {
        self.running_count() < self.max_concurrent
    }

    pub fn dispatch(&self, task: &BackgroundTask) -> Result<(), String> {
        for worker in &self.workers {
            if worker.can_handle(task) {
                return worker.execute(task);
            }
        }
        Err(format!("No worker found for task: {:?}", task))
    }
}
