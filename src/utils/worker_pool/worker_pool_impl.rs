use lazy_static::lazy_static;
use log;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

struct Task {
    priority: usize,
    job: Box<dyn FnOnce() -> bool + Send + 'static>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order here so that a smaller number gives higher priority
        other.priority.cmp(&self.priority)
    }
}

struct WorkerPool {
    queue: Arc<(Mutex<BinaryHeap<Task>>, Condvar)>,
    workers: Vec<JoinHandle<()>>,
}

impl WorkerPool {
    pub fn new(name: String, num_threads: usize) -> Self {
        let queue = Arc::new((Mutex::new(BinaryHeap::new()), Condvar::new()));
        let mut workers = Vec::with_capacity(num_threads);

        for thread_num in 0..num_threads {
            let queue_clone = queue.clone();
            let name_clone = format!("{name}:{thread_num}");
            workers.push(thread::spawn(move || {
                WorkerPool::worker_thread(name_clone, queue_clone);
            }));
        }

        WorkerPool { queue, workers }
    }

    fn worker_thread(name: String, queue: Arc<(Mutex<BinaryHeap<Task>>, Condvar)>) {
        loop {
            let task = {
                let (lock, cvar) = &*queue;
                let mut guard = lock.lock().unwrap();
                while guard.is_empty() {
                    guard = cvar.wait(guard).unwrap();
                }
                guard.pop().unwrap()
            };

            if !(task.job)() {
                log::info!("Worker pool thread [{name}] shutting down...");
            };
        }
    }

    fn submit_task<F>(&self, priority: usize, f: F)
    where
        F: FnOnce() -> bool + Send + 'static,
    {
        let task = Task {
            priority,
            job: Box::new(f),
        };
        let (lock, cvar) = &*self.queue;
        let mut guard = lock.lock().unwrap();
        guard.push(task);
        cvar.notify_one();
    }

    fn submit_tasks<F>(&self, mut tasks: Vec<(usize, F)>)
        where
            F: FnOnce() -> bool + Send + 'static,
    {
        let (lock, cvar) = &*self.queue;
        let mut guard = lock.lock().unwrap();
        for (priority, f) in tasks.drain(..) {
            let task = Task {
                priority,
                job: Box::new(f),
            };
            guard.push(task);
        }

        cvar.notify_all();
    }
}

lazy_static! {
    static ref WORKER_POOLS: Mutex<HashMap<String, Arc<WorkerPool>>> = Mutex::new(HashMap::new());
}

pub fn create_worker_pool(pool_name: &str, pool_size: usize) {
    let mut pools = WORKER_POOLS.lock().unwrap();
    if !pools.contains_key(pool_name) {
        let pool = Arc::new(WorkerPool::new(pool_name.to_string(), pool_size));
        pools.insert(pool_name.to_string(), pool);
    }
}

pub fn shutdown_worker_pool(pool_name: &str) {
    let mut pools = WORKER_POOLS.lock().unwrap();

    if let Some(mut pool) = pools.get_mut(pool_name) {
        let count = pool.workers.len();
        for _ in 0..count {
            pool.submit_task(0, || {
                thread::sleep(Duration::from_millis(100));
                false
            });
        }
        thread::sleep(Duration::from_millis(150));
        // pool.wait_for_completion();
        pools.remove(pool_name);
        log::info!("Worker pool [{pool_name}] shut down.");
    }

}

pub fn submit_tasks_to_worker_pool(
    pool_name: &str,
    priority: usize,
    task_func: impl FnOnce() -> bool + Send + 'static) {
    let pools = WORKER_POOLS.lock().unwrap();
    if let Some(pool) = pools.get(pool_name) {
        pool.submit_task(priority, task_func);
    } else {
        // Optionally handle the error if the pool does not exist
        log::error!("Worker pool [{pool_name}] does not exist.");
    }
}


pub fn submit_task_to_worker_pool(
    pool_name: &str,
    tasks: Vec<(usize, impl FnOnce() -> bool + Send + 'static)>)
{
    let pools = WORKER_POOLS.lock().unwrap();
    if let Some(pool) = pools.get(pool_name) {
        pool.submit_tasks(tasks);
    } else {
        // Optionally handle the error if the pool does not exist
        log::error!("Worker pool [{pool_name}] does not exist.");
    }
}
