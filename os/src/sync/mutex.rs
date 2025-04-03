//! Mutex (spin-like and blocking(sleep))
//use alloc::collections::BTreeMap;
use super::UPSafeCell;
use crate::task::current_process;
use crate::task::TaskControlBlock;
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use crate::task::{current_task, wakeup_task};
use alloc::{collections::VecDeque, sync::Arc};
use crate::sync::deadlock::is_safe;

/// Mutex trait
pub trait Mutex: Sync + Send {
    /// Lock the mutex
    fn lock(&self);
    /// Unlock the mutex
    fn unlock(&self);
}

/// Spinlock Mutex struct
pub struct MutexSpin {
    locked: UPSafeCell<bool>,
}

impl MutexSpin {
    /// Create a new spinlock mutex
    pub fn new() -> Self {
        Self {
            locked: unsafe { UPSafeCell::new(false) },
        }
    }
}

impl Mutex for MutexSpin {
    /// Lock the spinlock mutex
    fn lock(&self) {
        trace!("kernel: MutexSpin::lock");
        loop {
            let mut locked = self.locked.exclusive_access();
            if *locked {
                drop(locked);
                suspend_current_and_run_next();
                continue;
            } else {
                *locked = true;
                return;
            }
        }
    }

    fn unlock(&self) {
        trace!("kernel: MutexSpin::unlock");
        let mut locked = self.locked.exclusive_access();
        *locked = false;
    }
}

/*
// 银行家算法实现
fn is_safe(available: &[usize], allocation: &BTreeMap<usize, usize>, need: &BTreeMap<usize, usize>) -> bool {
    let mut work = available.to_vec();
    let mut finish = BTreeMap::new();
        
    for tid in allocation.keys().chain(need.keys()) {
        finish.insert(tid, false);
    }
    
    loop {
        let mut found = false;
        for (tid, &need_val) in need {
            if !finish[tid] && need_val <= work[0] {
                work[0] += allocation.get(tid).unwrap_or(&0);
                finish.insert(tid, true);
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
    }
    
    finish.values().all(|&f| f)
    }
    
*/
    

/// Blocking Mutex struct
pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    locked: bool,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    /// Create a new blocking mutex
    pub fn new() -> Self {
        trace!("kernel: MutexBlocking::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }
}

impl Mutex for MutexBlocking {
    /// lock the blocking mutex
    fn lock(&self) {
        trace!("kernel: MutexBlocking::lock");
        let mut mutex_inner = self.inner.exclusive_access();
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());



            // Deadlock detection
            let process = current_process();
            let mut process_inner = process.inner_exclusive_access();
            if process_inner.deadlock_detect_enabled {
                let tid = current_task().unwrap().inner_exclusive_access().res.as_ref().unwrap().tid;
                            
                // 更新资源需求
                process_inner.mutex_need.insert(tid, 1);
                
                // 执行安全检查
                let available = process_inner.mutex_available[0]; // 取第一个可用资源
                if !is_safe(
                    available, //&process_inner.mutex_available,
                    &process_inner.mutex_allocation,
                    &process_inner.mutex_need
                ) {
                    //return -0xDEAD as isize;
                    panic!("Deadlock detected"); // 暂时用panic代替
                    // 这里需要修改函数签名才能返回错误码，暂时注释掉检测部分
                    // return -0xDEAD as isize;
                }
            }

            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            mutex_inner.locked = true;
        }
    }

    /// unlock the blocking mutex
    fn unlock(&self) {
        trace!("kernel: MutexBlocking::unlock");
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            wakeup_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}
