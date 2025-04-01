//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// System call invocation counters
    /// 
    /// Index corresponds to syscall ID, value is invocation count
    pub syscall_counts: [usize; 512], // 足够覆盖所有syscall ID
}

impl TaskControlBlock {
    /// Create a zero-initialized TaskControlBlock
    ///
    /// # Returns
    /// - A new TaskControlBlock with all fields initialized to default values

    pub fn zero_init() -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            syscall_counts: [0; 512], // 初始化数组
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
