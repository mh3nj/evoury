use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use crate::queue::JobQueue;

pub struct Worker;

impl Worker {
    pub fn new(_id: usize) -> Self {
        Self
    }
}

pub struct WorkerPool {
    handles: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    active: Arc<AtomicUsize>,
    running: Arc<Mutex<bool>>,
    queue: Arc<Mutex<JobQueue>>,
    capacity: usize,
}

impl WorkerPool {
    pub fn new(size: usize, queue: Arc<Mutex<JobQueue>>) -> Self {
        Self {
            handles: Arc::new(Mutex::new(Vec::with_capacity(size))),
            active: Arc::new(AtomicUsize::new(0)),
            running: Arc::new(Mutex::new(false)),
            queue,
            capacity: size,
        }
    }

    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            return;
        }
        *running = true;
        drop(running);

        for _ in 0..self.capacity {
            let queue = self.queue.clone();
            let active = self.active.clone();
            let running = self.running.clone();
            let handles = self.handles.clone();

            let handle = thread::spawn(move || {
                loop {
                    {
                        let is_running = running.lock().unwrap();
                        if !*is_running {
                            break;
                        }
                    }

                    let job = {
                        let mut q = queue.lock().unwrap();
                        q.dequeue()
                    };

                    if let Some(job) = job {
                        active.fetch_add(1, Ordering::SeqCst);
                        thread::sleep(Duration::from_millis(10));
                        let mut q = queue.lock().unwrap();
                        q.complete(job.id);
                        active.fetch_sub(1, Ordering::SeqCst);
                    } else {
                        thread::sleep(Duration::from_millis(50));
                    }
                }
            });

            handles.lock().unwrap().push(handle);
        }
    }

    pub fn stop(&self) {
        {
            let mut running = self.running.lock().unwrap();
            *running = false;
        }

        let handles = self.handles.lock().unwrap().drain(..).collect::<Vec<_>>();
        for h in handles {
            let _ = h.join();
        }
    }

    pub fn active_count(&self) -> usize {
        self.active.load(Ordering::SeqCst)
    }

    pub fn worker_count(&self) -> usize {
        self.capacity
    }
}
