// src/syscall/error.rs
#![allow(missing_docs)] // 如果暂时不想写文档可以临时允许

//! 系统调用错误码定义
//! 
//! 遵循Linux标准错误码约定

/// 系统调用错误类型
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallError {
    /// 死锁检测失败 (错误码: -0xDEAD)
    EDEADLK = -0xDEAD,
    /// 无效参数 (错误码: -2)
    EINVAL  = -2,
    /// 资源暂时不可用/需要重试 (错误码: -3)
    EAGAIN  = -3,
    /// 非法内存地址 (错误码: -4)
    EFAULT  = -4,
    /// 内存不足 (错误码: -5)
    ENOMEM  = -5,
    /// 无效文件描述符 (错误码: -6)
    EBADF   = -6,
}