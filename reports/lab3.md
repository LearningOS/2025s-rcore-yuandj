# 第三章实验报告

## 实验总结
本章实现了分时多任务系统，主要工作包括：
1. 多道程序加载与初始化
2. 任务切换上下文保存与恢复
3. 系统调用 `sys_trace` 实现（支持内存读写、调用统计）
4. 时钟中断与抢占式调度
5. 通过时间片轮转算法验证多任务并发执行

## 简答作业

#### **问题1：正确进入U态后的程序特征验证**
**测试步骤与结果：**
1. **测试用例**：
   - `ch2b_bad_address`：尝试访问非法地址 `0x0`
   - `ch2b_bad_instructions`：执行非法指令（如 `sret`）
   - `ch2b_bad_register`：访问 S 态寄存器（如 `sstatus`）

2. **观察到的错误行为**：
   ```bash
   [kernel] PageFault in application, bad addr = 0x0, kernel killed it.
   [kernel] IllegalInstruction in application, kernel killed it.
   [kernel] IllegalInstruction in application, kernel killed it.
   ```
   - **非法地址访问**：触发 `PageFault`，内核终止进程。
   - **非法指令/寄存器访问**：触发 `IllegalInstruction`，内核终止进程。

3. **SBI 版本信息**：
   - **RustSBI 版本**：0.3.0-alpha.2
   - **实现版本**：RustSBI-QEMU 0.2.0-alpha.2

#### **问题2：深入理解 trap.S**
**代码片段分析：**
```asm
# trap.S

__alltraps:
    csrrw sp, sscratch, sp  # L13
    # ... 保存寄存器到内核栈 ...

__restore:
    mv sp, a0                # L40
    # ... 恢复寄存器 ...
    ld t0, 32*8(sp)          # L43
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0         # L44
    csrw sepc, t1
    csrw sscratch, t2
    # ... 恢复通用寄存器 ...
    csrrw sp, sscratch, sp   # L60
    sret
```

**逐问题解析：**

1. **L40：`mv sp, a0`**
   - **sp 的值**：此时 `sp` 指向内核栈中的 `TrapContext` 结构（即保存的用户态上下文）。
   - **两种使用情景**：
     1. **首次进入用户态**：从内核初始化时直接跳转到 `__restore`。
     2. **从内核返回用户态**：在中断处理完成后恢复用户上下文。

2. **L43-L48：特殊寄存器恢复**
   ```asm
   ld t0, 32*8(sp)   # sstatus
   ld t1, 33*8(sp)   # sepc
   ld t2, 2*8(sp)    # sscratch
   csrw sstatus, t0
   csrw sepc, t1
   csrw sscratch, t2
   ```
   - **意义**：
     - `sstatus`：恢复用户态特权级和中断使能状态。
     - `sepc`：设置返回用户态后执行的指令地址。
     - `sscratch`：保存内核栈指针，用于快速切换。

3. **L50-L56：跳过 x2 和 x4**
   ```asm
   ld x1, 1*8(sp)
   ld x3, 3*8(sp)
   .set n, 5
   .rept 27
      LOAD_GP %n
      .set n, n+1
   .endr
   ```
   - **原因**：
     - **x2 (sp)**：已在 `L60` 通过 `csrrw` 恢复，避免重复操作。
     - **x4 (tp)**：通常用于线程局部存储，用户程序未使用，无需恢复。

4. **L60：`csrrw sp, sscratch, sp`**
   - **操作后寄存器值**：
     - `sp`：指向用户栈（从 `sscratch` 恢复）。
     - `sscratch`：指向内核栈（为下次 Trap 保存内核栈指针）。

5. **状态切换指令**
   - **进入用户态**：通过 `sret` 指令完成，该指令会：
     1. 将 `sstatus.SPP` 位恢复为 U 态。
     2. 跳转到 `sepc` 指向的地址（用户程序的下一条指令）。

6. **L13：`csrrw sp, sscratch, sp`**
   - **操作后寄存器值**：
     - `sp`：指向内核栈（切换为内核态栈）。
     - `sscratch`：保存用户栈指针（为后续恢复做准备）。

7. **U 态进入 S 态**
   - **触发方式**：用户程序通过 `ecall` 指令主动触发异常，硬件自动：
     1. 保存 `sepc` 为 `ecall` 下一条指令地址。
     2. 切换特权级到 S 态，跳转到 `stvec` 指向的 `__alltraps`。



## 荣誉准则
**我郑重声明，本实验代码和报告均为本人独立完成，未参考任何未授权资料。**
本实验，借助 DEEPSEEK 指导辅助完成，包括：阅读文档，调试错误，增添注释，总结报告。

## 实验心得（可选）
调试过程中，任务切换时的上下文保存最易出错，需仔细检查寄存器恢复顺序...
