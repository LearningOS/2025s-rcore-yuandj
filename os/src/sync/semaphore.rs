//! Semaphore

use crate::sync::UPSafeCell;
use crate::task::current_process;
use crate::task::{block_current_and_run_next, current_task, wakeup_task, TaskControlBlock};
use alloc::{collections::VecDeque, sync::Arc};
use crate::sync::deadlock::is_safe;

/// semaphore structure
pub struct Semaphore {
    /// semaphore inner
    pub inner: UPSafeCell<SemaphoreInner>,
}

pub struct SemaphoreInner {
    pub count: isize,
    pub wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl Semaphore {
    /// Create a new semaphore
    pub fn new(res_count: usize) -> Self {
        trace!("kernel: Semaphore::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(SemaphoreInner {
                    count: res_count as isize,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }

    /// up operation of semaphore
    pub fn up(&self) {
        trace!("kernel: Semaphore::up");
        let mut inner = self.inner.exclusive_access();
        inner.count += 1;
        if inner.count <= 0 {
            if let Some(task) = inner.wait_queue.pop_front() {
                wakeup_task(task);
            }
        }
    }

    /// down operation of semaphore
    pub fn down(&self) {
        trace!("kernel: Semaphore::down");
        let mut inner = self.inner.exclusive_access();
        inner.count -= 1;
        if inner.count < 0 {
            // Deadlock detection
            let process = current_process();
            let mut process_inner = process.inner_exclusive_access();
            if process_inner.deadlock_detect_enabled {
                let tid = current_task().unwrap().inner_exclusive_access().res.as_ref().unwrap().tid;
                //let sem_id = /* 需要获取当前semaphore的ID */;
                //let sem_id = process_inner.semaphore_list.iter().position(|s| Arc::ptr_eq(s.as_ref().unwrap(), self)).unwrap();
                //let sem_id = process_inner.semaphore_list.iter().position(|s| {
                //    if let Some(sem) = s { Arc::ptr_eq(sem, self) } else { false }
                //}).unwrap();                
                //let sem = self as *const Semaphore as usize; // 临时方案获取唯一标识
                //let sem_id = process_inner.semaphore_list.iter().position(|s| {
                //    s.as_ref().map_or(false, |s| Arc::ptr_eq(s, &self))
                //}).unwrap();
                //let sem_id = process_inner.semaphore_list.iter()
                //    //.position(|s| s.as_ref().map_or(false, |s| Arc::ptr_eq(s, self)))
                //    .position(|s| s.as_ref().map_or(false, |s| Arc::ptr_eq(s, &self.into())))
                //    .unwrap();
                let sem_id = process_inner.semaphore_list.iter().position(|s| {
                    //s.as_ref().map_or(false, |s| Arc::ptr_eq(s, &(*self).into()))
                    //s.as_ref().map_or(false, |ss| Arc::ptr_eq(ss, self)) // 直接比较Arc指针
                    s.as_ref().map_or(false, |ss| Arc::as_ptr(ss) == self as *const _)
                }).unwrap();

                // 更新分配和需求
                let available = process_inner.semaphore_list[sem_id].as_ref().unwrap().inner.exclusive_access().count as usize;
                *process_inner.semaphore_allocation.entry(tid).or_insert(0) += 1;
                //*process_inner.semaphore_need.entry(tid).or_insert(0) = 
                    //process_inner.semaphore_resources[sem_id].initial - process_inner.semaphore_allocation[tid];
                    //process_inner.semaphore_list[sem_id].as_ref().unwrap().inner.exclusive_access().count as usize;
                *process_inner.semaphore_need.entry(tid).or_insert(0) = available;

                if !is_safe(
                    //&process_inner.semaphore_available,
                    //&process_inner.semaphore_list[sem_id].as_ref().unwrap().inner.exclusive_access().count as usize // 需要类型转换
                    available, //process_inner.semaphore_list[sem_id].as_ref().unwrap().inner.exclusive_access().count as usize,
                    &process_inner.semaphore_allocation,
                    //&process_inner.semaphore_need,
                    //&process_inner.semaphore_available
                    &process_inner.semaphore_need
                ) {
                    //return -0xDEAD as isize;
                    panic!("Deadlock detected");
                }
            }            
            
            inner.wait_queue.push_back(current_task().unwrap());
            drop(inner);
            block_current_and_run_next();
        }
    }
}
