use crate::{sync::UPSafeCell, config::MAX_APP_NUM, task::{TaskStatus, TaskContext, __switch}};

use super::TaskControlBlock;
/// The task manager, where all the tasks are managed.
///
/// Functions implemented on `TaskManager` deals with all task state transitions
/// and task context switching. For convenience, you can find wrappers around it
/// in the module level.
///
/// Most of `TaskManager` are hidden behind the field `inner`, to defer
/// borrowing checks to runtime. You can see examples on how to use `inner` in
/// existing functions on `TaskManager`.
pub struct TaskManager {
  /// total number of tasks
  pub num_app: usize,
  /// use inner value to get mutable access
  pub inner: UPSafeCell<TaskManagerInner>,
}

/// The task manager inner in 'UPSafeCell'
pub struct TaskManagerInner {
  /// task list
  pub tasks: [TaskControlBlock; MAX_APP_NUM],
  /// id of current `Running` task
  pub current_task: usize,
}


impl TaskManager {
  /// Run the first task in task list.
  ///
  /// Generally, the first task in task list is an idle task (we call it zero process later).
  /// But in ch3, we load apps statically, so the first task is a real app.
  pub fn run_first_task(&self) -> ! {
      let mut inner = self.inner.exclusive_access();
      let task0 = &mut inner.tasks[0];
      task0.task_status = TaskStatus::Running;
      let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
      drop(inner);
      let mut _unused = TaskContext::zero_init();
      // before this, we should drop local variables that must be dropped manually
      unsafe {
          __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
      }
      panic!("unreachable in run_first_task!");
  }

  /// Change the status of current `Running` task into `Ready`.
  pub fn mark_current_suspended(&self) {
      let mut inner = self.inner.exclusive_access();
      let current = inner.current_task;
      inner.tasks[current].task_status = TaskStatus::Ready;
  }

  /// Change the status of current `Running` task into `Exited`.
  pub fn mark_current_exited(&self) {
      let mut inner = self.inner.exclusive_access();
      let current = inner.current_task;
      inner.tasks[current].task_status = TaskStatus::Exited;
  }

  /// Find next task to run and return task id.
  ///
  /// In this case, we only return the first `Ready` task in task list.
  pub fn find_next_task(&self) -> Option<usize> {
      let inner = self.inner.exclusive_access();
      let current = inner.current_task;
      (current + 1..current + self.num_app + 1)
          .map(|id| id % self.num_app)
          .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
  }

  /// Switch current `Running` task to the task we have found,
  /// or there is no `Ready` task and we can exit with all applications completed
  pub fn run_next_task(&self) {
      if let Some(next) = self.find_next_task() {
          let mut inner = self.inner.exclusive_access();
          let current = inner.current_task;
          inner.tasks[next].task_status = TaskStatus::Running;
        //   inner.tasks[next].task_time += 1;
          inner.current_task = next;
          let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
          let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
          drop(inner);
          // before this, we should drop local variables that must be dropped manually
          unsafe {
              __switch(current_task_cx_ptr, next_task_cx_ptr);
          }
          // go back to user mode
      } else {
          panic!("All applications completed!");
      }
  }

  // LAB1: Try to implement your function to update or get task info!
}
