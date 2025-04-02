//! Process management syscalls
use crate::task::TASK_MANAGER; // 添加在文件顶部
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        //*ts = TimeVal {
        //    sec: us / 1_000_000,
        //    usec: us % 1_000_000,
        //};
        (*ts).sec = us / 1_000_000;
        (*ts).usec = us % 1_000_000;
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    match trace_request {
        0 => { // 读取用户内存
            let addr = id as *const u8;
            unsafe { addr.read_volatile() as isize }
        },
        1 => { // 写入用户内存
            let addr = id as *mut u8;
            unsafe { addr.write_volatile(data as u8) };
            0
        },
        2 => TASK_MANAGER.get_syscall_count(id),
        _ => -1
    }
}
