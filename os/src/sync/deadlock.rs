// src/sync/deadlock.rs
#![allow(unused)]

extern crate alloc;
use alloc::collections::BTreeMap;

// 死锁检测模块 → 移除非模块级注释

/// 死锁检测模块
/// 
/// 实现基于资源分配图的死锁检测算法
/// 包含银行家算法等安全状态检查机制

/// 系统资源状态快照
#[derive(Debug)]
pub struct SystemState {
    /// 可用资源计数
    pub available: usize,
    /// 资源分配表
    pub allocation: BTreeMap<usize, usize>,
    /// 资源需求表
    pub need: BTreeMap<usize, usize>,
}

impl SystemState {
    /// 创建新状态快照
    pub fn new(
        available: usize,
        allocation: BTreeMap<usize, usize>,
        need: BTreeMap<usize, usize>
    ) -> Self {
        Self { available, allocation, need }
    }
}

/// 银行家算法安全状态检查
/// 
/// 根据Dijkstra的银行家算法判断系统是否处于安全状态
/// 
/// # 参数
/// - `_available`: 当前可用资源总数  
/// - `_allocation`: 线程已分配资源映射 (tid ➔ count)  
/// - `_need`: 线程资源需求映射 (tid ➔ count)  
/// 
/// # 返回值
/// 返回 `true` 表示系统处于安全状态
pub fn is_safe(
    available: usize,   // 移除参数名前下划线
    allocation: &BTreeMap<usize, usize>,
    need: &BTreeMap<usize, usize>   // 参数名与函数体内使用保持一致
) -> bool {
    // TODO: 实现算法逻辑
    let mut work = available;
    let mut finish = BTreeMap::new();

    // 初始化完成状态
    for tid in allocation.keys().chain(need.keys()) {
        finish.insert(*tid, false);
    }

    loop {
        let mut found = false;
        for (tid, need_val) in need {
            if !finish[tid] && *need_val <= work {
                // 分配资源
                work += allocation.get(tid).unwrap_or(&0);
                finish.insert(*tid, true);
                found = true;
            }
        }
        if !found {
            break;
        }
    }

    finish.values().all(|&v| v)
}