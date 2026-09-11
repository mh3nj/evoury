use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::job::{Job, JobStatus};

struct JobWrapper(Job);

impl Eq for JobWrapper {}

impl PartialEq for JobWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.priority == other.0.priority && self.0.id == other.0.id
    }
}

impl PartialOrd for JobWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JobWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.priority.cmp(&self.0.priority)
            .then_with(|| self.0.created_at.cmp(&other.0.created_at))
    }
}

pub struct JobQueue {
    pending: BinaryHeap<JobWrapper>,
    running: HashMap<uuid::Uuid, Job>,
    history: Vec<Job>,
    max_history: usize,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            pending: BinaryHeap::new(),
            running: HashMap::new(),
            history: Vec::new(),
            max_history: 100,
        }
    }

    pub fn enqueue(&mut self, job: Job) {
        self.pending.push(JobWrapper(job));
    }

    pub fn dequeue(&mut self) -> Option<Job> {
        self.pending.pop().map(|w| {
            let mut job = w.0;
            job.status = JobStatus::Running;
            job.started_at = Some(chrono::Utc::now());
            self.running.insert(job.id, job.clone());
            job
        })
    }

    pub fn complete(&mut self, job_id: uuid::Uuid) {
        if let Some(mut job) = self.running.remove(&job_id) {
            job.status = JobStatus::Completed;
            job.completed_at = Some(chrono::Utc::now());
            self.history.push(job);
            self.trim_history();
        }
    }

    pub fn fail(&mut self, job_id: uuid::Uuid, error: String) {
        if let Some(mut job) = self.running.remove(&job_id) {
            job.status = JobStatus::Failed(error);
            job.completed_at = Some(chrono::Utc::now());
            self.history.push(job);
            self.trim_history();
        }
    }

    pub fn cancel(&mut self, job_id: uuid::Uuid) {
        self.pending = self.pending.drain().filter(|w| w.0.id != job_id).collect();
        if let Some(mut job) = self.running.remove(&job_id) {
            job.status = JobStatus::Cancelled;
            job.completed_at = Some(chrono::Utc::now());
            self.history.push(job);
            self.trim_history();
        }
    }

    pub fn cancel_all(&mut self) {
        for job in self.pending.drain() {
            let mut j = job.0;
            j.status = JobStatus::Cancelled;
            j.completed_at = Some(chrono::Utc::now());
            self.history.push(j);
        }
        let ids: Vec<uuid::Uuid> = self.running.keys().copied().collect();
        for id in ids {
            self.cancel(id);
        }
        self.trim_history();
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn running_count(&self) -> usize {
        self.running.len()
    }

    pub fn history(&self) -> &[Job] {
        &self.history
    }

    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    fn trim_history(&mut self) {
        while self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::JobPriority;

    #[test]
    fn test_priority_ordering() {
        let mut q = JobQueue::new();
        q.enqueue(Job::new("low", JobPriority::Low));
        q.enqueue(Job::new("high", JobPriority::High));
        q.enqueue(Job::new("normal", JobPriority::Normal));
        q.enqueue(Job::new("immediate", JobPriority::Immediate));

        assert_eq!(q.dequeue().unwrap().name, "immediate");
        assert_eq!(q.dequeue().unwrap().name, "high");
        assert_eq!(q.dequeue().unwrap().name, "normal");
        assert_eq!(q.dequeue().unwrap().name, "low");
    }

    #[test]
    fn test_complete_flow() {
        let mut q = JobQueue::new();
        let job = Job::new("test", JobPriority::Normal);
        let id = job.id;
        q.enqueue(job);
        let dequeued = q.dequeue().unwrap();
        assert_eq!(dequeued.status, JobStatus::Running);
        q.complete(id);
        assert_eq!(q.history().last().unwrap().status, JobStatus::Completed);
    }
}
