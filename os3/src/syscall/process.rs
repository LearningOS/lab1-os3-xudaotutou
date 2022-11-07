//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{
    exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER,
};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    println!("invoke");
    let inner = TASK_MANAGER.inner.exclusive_access();
    let cur_task_block = inner.tasks[inner.current_task];
    let status = cur_task_block.task_status;
    println!(">");
    let time = get_time_us() - cur_task_block.start_time;
    let syscall_times: [u32; MAX_SYSCALL_NUM] = cur_task_block.syscall_times.clone();

    unsafe {*ti = TaskInfo {
        status,
        syscall_times,
        time,
    };}
    println!("<");
    drop(inner);
    0
}