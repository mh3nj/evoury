use std::sync::{Arc, Mutex};
use crate::job::{Job, JobPriority};
use crate::queue::JobQueue;
use crate::worker::WorkerPool;

pub struct Scheduler {
    queue: Arc<Mutex<JobQueue>>,
    pool: Arc<Mutex<WorkerPool>>,
    started: Arc<Mutex<bool>>,
}

impl Scheduler {
    pub fn new(worker_count: usize) -> Self {
        let queue = Arc::new(Mutex::new(JobQueue::new()));
        let pool = Arc::new(Mutex::new(WorkerPool::new(worker_count, queue.clone())));
        Self {
            queue,
            pool,
            started: Arc::new(Mutex::new(false)),
        }
    }

    pub fn submit(&self, job: Job) {
        self.queue.lock().unwrap().enqueue(job);
    }

    pub fn submit_with_priority(&self, name: &str, priority: JobPriority) {
        self.submit(Job::new(name, priority));
    }

    pub fn cancel(&self, job_id: uuid::Uuid) {
        self.queue.lock().unwrap().cancel(job_id);
    }

    pub fn cancel_all(&self) {
        self.queue.lock().unwrap().cancel_all();
    }

    pub fn pending_count(&self) -> usize {
        self.queue.lock().unwrap().pending_count()
    }

    pub fn running_count(&self) -> usize {
        self.queue.lock().unwrap().running_count()
    }

    pub fn active_workers(&self) -> usize {
        self.pool.lock().unwrap().active_count()
    }

    pub fn is_started(&self) -> bool {
        *self.started.lock().unwrap()
    }

    pub fn start(&self) {
        let mut started = self.started.lock().unwrap();
        if !*started {
            self.pool.lock().unwrap().start();
            *started = true;
        }
    }

    pub fn stop(&self) {
        let mut started = self.started.lock().unwrap();
        if *started {
            self.pool.lock().unwrap().stop();
            self.queue.lock().unwrap().cancel_all();
            *started = false;
        }
    }
}
