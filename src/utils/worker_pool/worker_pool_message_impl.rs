use crate::utils::worker_pool::task_impl::Task;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum WorkerPoolMessage {
    Shutdown,
    WorkerTask(Task),
}
