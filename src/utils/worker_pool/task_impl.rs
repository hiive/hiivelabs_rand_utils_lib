use std::any::Any;
use std::cmp::Ordering;

pub struct Task {
    pub priority: usize,
    pub task_id: Option<usize>,
    pub task_info: Option<usize>,
    pub job: Box<dyn FnOnce() -> Option<Box<dyn Any + Send>> + Send + 'static>,
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
