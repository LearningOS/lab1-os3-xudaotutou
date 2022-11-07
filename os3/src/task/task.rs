//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // pub task_time: usize,
    pub start_time:usize,
    pub syscall_times: [u32; MAX_SYSCALL_NUM]
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
