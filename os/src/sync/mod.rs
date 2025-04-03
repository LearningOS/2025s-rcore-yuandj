//! Synchronization and interior mutability primitives

// 在 src/sync/mod.rs 开头添加模块说明
//! 同步原语模块
//! 
//! 包含以下组件：
//! - Mutex: 互斥锁实现
//! - Condvar: 条件变量实现
//! - Semaphore: 信号量实现
//! - deadlock: 死锁检测机制

mod condvar;
mod mutex;
mod semaphore;
mod up;

// 为子模块添加文档
/// 死锁预防与检测模块
/// 
/// 实现基于银行家算法的资源分配策略
pub mod deadlock;
pub use condvar::Condvar;
pub use mutex::{Mutex, MutexBlocking, MutexSpin};
pub use semaphore::Semaphore;
pub use up::UPSafeCell;

extern crate alloc;
pub use alloc::collections::BTreeMap;
