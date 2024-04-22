use std::cmp::Ordering;
use crate::utils::worker_pool::task_impl::Task;

// pub struct WorkerTask {
//     pub task_id: Option<String>,
//     pub task_priority: usize,
//     pub task_func: Box<dyn FnOnce() -> (bool, Option<Box<dyn Any + Send>>) + Send + 'static>
// }

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum WorkerPoolMessage {
    Shutdown,
    WorkerTask(Task),
}