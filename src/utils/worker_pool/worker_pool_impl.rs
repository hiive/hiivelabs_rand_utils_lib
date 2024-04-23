use crate::utils::worker_pool::worker_pool_message_impl::WorkerPoolMessage;
use lazy_static::lazy_static;
use log;
use std::any::Any;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

struct WorkerPool {
    queue: Arc<(Mutex<BinaryHeap<WorkerPoolMessage>>, Condvar)>,
    workers: Vec<(String, JoinHandle<()>)>,
    job_result_tx: Option<Sender<(Option<usize>, Option<usize>, Option<Box<dyn Any + Send>>)>>,
}

impl WorkerPool {
    pub fn new(
        name: String,
        num_threads: usize,
        job_result_tx: Option<Sender<(Option<usize>, Option<usize>, Option<Box<dyn Any + Send>>)>>,
    ) -> Self {
        let queue = Arc::new((Mutex::new(BinaryHeap::new()), Condvar::new()));
        let mut workers = Vec::with_capacity(num_threads);

        for thread_num in 0..num_threads {
            let queue_clone = queue.clone();
            let name_clone = format!("{name}:{thread_num}");
            let job_result_tx_clone = job_result_tx.clone();
            workers.push((
                name_clone.clone(),
                thread::spawn(move || {
                    WorkerPool::worker_thread(name_clone, queue_clone, job_result_tx_clone);
                }),
            ));
        }

        WorkerPool {
            queue,
            workers,
            job_result_tx,
        }
    }

    fn worker_thread(
        name: String,
        queue: Arc<(Mutex<BinaryHeap<WorkerPoolMessage>>, Condvar)>,
        job_result_tx: Option<Sender<(Option<usize>, Option<usize>, Option<Box<dyn Any + Send>>)>>,
    ) {
        loop {
            let task = {
                let (lock, cvar) = &*queue;
                let mut guard = lock.lock().unwrap();
                while guard.is_empty() {
                    guard = cvar.wait(guard).unwrap();
                }
                guard.pop().unwrap()
            };

            match task {
                WorkerPoolMessage::Shutdown => {
                    log::info!("Worker pool thread [{name}] shutting down...");
                    if let Some(job_result_tx) = &job_result_tx {
                        let ret_package = (None, None, None);
                        log::info!("Sending shutdown ack for worker pool thread : {name}");
                        if let Err(tx_result_err) = job_result_tx.send(ret_package) {
                            log::error!("Failed to return thread result: {tx_result_err}")
                        }
                    }
                    break;
                }
                WorkerPoolMessage::WorkerTask(task) => {
                    let task_result = (task.job)();
                    if let Some(job_result_tx) = &job_result_tx {
                        // we have some data to return.
                        if let Some(t_id) = task.task_id {
                            let ret_package = (Some(t_id), task.task_info, task_result);
                            log::info!("Sending job result: {t_id}: {:?}", task.task_info);
                            if let Err(tx_result_err) = job_result_tx.send(ret_package) {
                                log::error!("Failed to return thread result: {tx_result_err}")
                            }
                        }
                    }
                }
            }
        }
    }

    // fn submit_task<F>(&self, priority: usize, task_id: Option<String>, f: F)
    // where
    //     F: FnOnce() -> (bool, Option<Box<dyn Any + Send>>) + Send + 'static,
    // {
    //     let task = WorkerTask {
    //         priority,
    //         task_id,
    //         job: Box::new(f),
    //     };
    //     let (lock, cvar) = &*self.queue;
    //     let mut guard = lock.lock().unwrap();
    //     guard.push(task);
    //     cvar.notify_one();
    // }

    // fn submit_tasks<F>(&self, mut tasks: Vec<(usize, Option<String>, F)>)
    //     where
    //         F: FnOnce() -> (bool, Option<Box<dyn Any + Send>>) + Send + 'static,
    // {
    //     let (lock, cvar) = &*self.queue;
    //     let mut guard = lock.lock().unwrap();
    //     for (priority, task_id, f) in tasks.drain(..) {
    //         let task = WorkerTask {
    //             priority,
    //             task_id,
    //             job: Box::new(f),
    //         };
    //         guard.push(task);
    //     }
    //
    //     cvar.notify_all();
    // }

    fn submit_message(&self, message: WorkerPoolMessage) {
        let (lock, cvar) = &*self.queue;
        let mut guard = lock.lock().expect("Cannot get lock");
        guard.push(message);
        cvar.notify_one();
    }
}

lazy_static! {
    static ref WORKER_POOLS: Mutex<HashMap<String, Arc<Mutex<WorkerPool>>>> =
        Mutex::new(HashMap::new());
}

pub fn create_worker_pool(
    pool_name: &str,
    pool_size: usize,
    job_result_tx: Option<Sender<(Option<usize>, Option<usize>, Option<Box<dyn Any + Send>>)>>,
) {
    let mut pools = WORKER_POOLS.lock().unwrap();
    if !pools.contains_key(pool_name) {
        let pool = Arc::new(Mutex::new(WorkerPool::new(
            pool_name.to_string(),
            pool_size,
            job_result_tx,
        )));
        pools.insert(pool_name.to_string(), pool);
    }
}

pub fn shutdown_worker_pool(pool_name: &str) {
    let mut pools = WORKER_POOLS.lock().unwrap();
    if let Some(pool_arc) = pools.get_mut(pool_name) {
        let pool_lock_result = pool_arc.try_lock();
        match pool_lock_result {
            Ok(mut pool) => {
                let count = pool.workers.len();
                // send shutdown messages for each thread, with a slight delay to ensure that
                // each thread shuts down
                for _ in 0..count {
                    pool.submit_message(WorkerPoolMessage::Shutdown);
                    thread::sleep(Duration::from_millis(25));
                }

                // let's wait for the threads to complete
                for (thread_name, join_handle) in pool.workers.drain(..) {
                    match join_handle.join() {
                        Ok(_) => {
                            log::info!("Worker Pool thread [{thread_name}] shut down");
                        }
                        Err(err) => {
                            log::error!("Error shutting down Worker Pool thread: {err:?}");
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("Error locking Worker pool [{pool_name}] : {err:?}.");
            }
        }
    }
    // pool.wait_for_completion();
    pools.remove(pool_name);
    log::info!("Worker pool [{pool_name}] shut down.");
}

pub fn submit_message_to_worker_pool(pool_name: &str, message: WorkerPoolMessage) {
    let pools = WORKER_POOLS.lock().unwrap();
    if let Some(pool_arc) = pools.get(pool_name) {
        let pool_lock_result = pool_arc.try_lock();
        match pool_lock_result {
            Ok(pool) => {
                pool.submit_message(message);
            }
            Err(err) => {
                log::error!("Error locking Worker pool [{pool_name}] : {err:?}.");
            }
        }
    } else {
        // Optionally handle the error if the pool does not exist
        log::error!("Worker pool [{pool_name}] does not exist.");
    }
}
