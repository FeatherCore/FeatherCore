# Rust 异步多任务与进程/线程模型

## 核心问题：Rust 异步多任务在哪个层级生效？

**简短回答**: Rust 的异步多任务在**线程内部**生效，是**用户态的协作式多任务**。

## 三层调度模型

```
┌─────────────────────────────────────────────────────────────┐
│ Level 3: 进程 (Process)                                     │
│ - 独立的地址空间                                             │
│ - 由操作系统内核调度                                         │
│ - 上下文切换开销大 (TLB 刷新、页表切换)                        │
│ - 隔离性好，一个进程崩溃不影响其他进程                       │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ fork/exec
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ Level 2: 线程 (Thread) - 内核级线程 (Kernel Thread)         │
│ - 共享进程地址空间                                           │
│ - 由操作系统内核调度                                         │
│ - 上下文切换开销中等                                         │
│ - 真正的并行执行 (多核)                                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ 创建线程
                            ▼
┌─────────────────────────────────────────────────────────────┐
│ Level 1: 异步任务 (Async Task) - Green Thread / Future      │
│ - 在线程内部运行                                             │
│ - 由用户态运行时调度 (Rust async runtime)                   │
│ - 上下文切换开销极小 (只是函数调用)                          │
│ - 协作式多任务 (需要 await 让出执行权)                        │
│ - 多个任务共享一个线程栈                                     │
└─────────────────────────────────────────────────────────────┘
```

## 详细对比

### 1. 进程 (Process)

**特点**:
- 独立的虚拟地址空间
- 拥有独立的文件描述符表、信号表等资源
- 内核调度的基本单位 (传统 Unix)
- 进程间通信 (IPC) 需要特殊机制 (管道、消息队列、共享内存)

**创建方式**:
```rust
// 系统调用层面
let pid = syscall::sys_fork();

if pid == 0 {
    // 子进程
    syscall::sys_execve("/bin/ls", args);
} else {
    // 父进程
    syscall::sys_waitpid(pid);
}
```

**上下文切换**:
```
进程 A → 进程 B
  ↓
保存进程 A 上下文:
  - 所有寄存器 (包括页表基址寄存器)
  - 页表
  - TLB 失效
  ↓
恢复进程 B 上下文:
  - 所有寄存器
  - 页表
  - 重新填充 TLB
  
开销：~1000-10000 CPU 周期
```

---

### 2. 线程 (Thread) - 内核级线程

**特点**:
- 共享进程的地址空间 (代码段、数据段、堆)
- 每个线程有独立的栈
- 内核调度的基本单位 (现代 OS)
- 真正的并行执行 (多核 CPU)
- 线程间通信简单 (共享内存)

**创建方式** (伪代码):
```rust
// FeatherCore kernel 中实现
pub fn create_thread(entry: fn(), stack_size: usize) -> ThreadId {
    // 1. 分配线程栈
    let stack = allocate_stack(stack_size);
    
    // 2. 创建线程控制块 (TCB)
    let tcb = ThreadControlBlock {
        stack_ptr: stack.top(),
        entry_point: entry,
        state: ThreadState::Ready,
        // ...
    };
    
    // 3. 添加到调度器
    SCHEDULER.lock().add_thread(tcb);
    
    // 4. 返回线程 ID
    tcb.id
}
```

**上下文切换**:
```
线程 A → 线程 B (同一进程内)
  ↓
保存线程 A 上下文:
  - 通用寄存器
  - 栈指针 SP
  - 程序计数器 PC
  - 状态寄存器
  ↓
恢复线程 B 上下文:
  - 通用寄存器
  - 栈指针 SP (切换到线程 B 的栈)
  - 程序计数器 PC
  - 状态寄存器
  
注意：页表不变，TLB 不失效
开销：~500-2000 CPU 周期
```

---

### 3. 异步任务 (Async Task / Future)

**特点**:
- 在线程内部运行 (用户态)
- 由异步运行时调度 (非内核)
- 协作式多任务 (需要 `await` 让出)
- 多个任务共享一个线程栈
- 上下文切换只是函数调用

**创建方式**:
```rust
// Rust async/await
async fn task1() {
    println!("Task 1");
    await;  // 让出执行权
    println!("Task 1 resumed");
}

async fn task2() {
    println!("Task 2");
}

// 在线程中运行
#[no_mangle]
pub fn thread_entry() {
    // 创建异步任务
    let future1 = task1();
    let future2 = task2();
    
    // 在线程中轮询执行
    pin_mut!(future1);
    pin_mut!(future2);
    
    // 简单的轮询调度
    loop {
        // 轮询 task1
        if let Poll::Ready(()) = future1.as_mut().poll(&mut cx) {
            // task1 完成
        }
        
        // 轮询 task2
        if let Poll::Ready(()) = future2.as_mut().poll(&mut cx) {
            // task2 完成
        }
    }
}
```

**上下文切换**:
```
任务 A → 任务 B (同一线程内)
  ↓
保存任务 A 状态:
  - Future 内部状态 (状态机)
  - 局部变量 (保存在 Future 中)
  ↓
恢复任务 B 状态:
  - 从 Future 中读取状态
  - 继续执行 poll()
  
注意：不涉及寄存器保存/恢复，只是函数调用
开销：~10-50 CPU 周期
```

---

## 三层模型的关系图

```
┌─────────────────────────────────────────────────────────┐
│  进程 (Process) - PID=100                               │
│  地址空间：0x00000000 - 0xFFFFFFFF                       │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  线程 1 (Thread) - TID=1                         │   │
│  │  栈：0x10000 - 0x20000                           │   │
│  │                                                  │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │   │
│  │  │ Async 1  │  │ Async 2  │  │ Async 3  │      │   │
│  │  │ (Future) │  │ (Future) │  │ (Future) │      │   │
│  │  └──────────┘  └──────────┘  └──────────┘      │   │
│  │     用户态调度 (Rust async runtime)              │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  线程 2 (Thread) - TID=2                         │   │
│  │  栈：0x20000 - 0x30000                           │   │
│  │                                                  │   │
│  │  ┌──────────┐  ┌──────────┐                     │   │
│  │  │ Async 4  │  │ Async 5  │                     │   │
│  │  │ (Future) │  │ (Future) │                     │   │
│  │  └──────────┘  └──────────┘                     │   │
│  └─────────────────────────────────────────────────┘   │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  线程 3 (Thread) - TID=3                         │   │
│  │  (内核线程，无 async 任务)                         │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘

调度层次:
1. 内核调度线程 (Thread 1, 2, 3) - 抢占式
2. 用户态调度异步任务 (Async 1-5) - 协作式
```

---

## Rust Async 的工作原理

### 1. Future 状态机

```rust
// Rust 编译器将 async 函数转换为状态机
async fn example() {
    println!("Step 1");
    await;  // 暂停点
    println!("Step 2");
}

// 编译器生成的状态机 (简化版)
enum ExampleFuture {
    NotStarted,
    Suspended1,  // 在第一个 await 处暂停
    Completed,
}

impl Future for example {
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        match self.state {
            NotStarted => {
                println!("Step 1");
                self.state = Suspended1;
                Poll::Pending  // 返回 Pending，让出执行权
            }
            Suspended1 => {
                println!("Step 2");
                self.state = Completed;
                Poll::Ready(())  // 完成
            }
            Completed => Poll::Ready(()),
        }
    }
}
```

### 2. 轮询执行

```rust
// 异步运行时的工作
fn run_async_tasks() {
    let mut task1 = Box::pin(example());
    let mut task2 = Box::pin(other_task());
    
    // 创建 Waker
    let waker1 = create_waker(task1_id);
    let waker2 = create_waker(task2_id);
    
    // 创建 Context
    let mut cx1 = Context::from_waker(&waker1);
    let mut cx2 = Context::from_waker(&waker2);
    
    // 轮询
    loop {
        // 轮询 task1
        match task1.as_mut().poll(&mut cx1) {
            Poll::Ready(()) => println!("Task 1 done"),
            Poll::Pending => {
                // task1 暂停，执行其他任务
            }
        }
        
        // 轮询 task2
        match task2.as_mut().poll(&mut cx2) {
            Poll::Ready(()) => println!("Task 2 done"),
            Poll::Pending => {
                // task2 暂停
            }
        }
    }
}
```

---

## FeatherCore 的设计

### 当前架构分析

查看您的 `sched.rs`，当前的设计是：

```rust
Scheduler {
    tasks: Vec<Task>,      // 任务列表
    threads: Vec<Thread>,  // 线程列表
}

Thread {
    current_task: Option<usize>,  // 当前任务
    ready_queue: Vec<usize>,      // 就绪队列
}

Task {
    task_type: TaskType {
        Sync { entry_point },      // 同步任务
        Async { future },          // 异步任务
    },
    thread_id: usize,  // 所属线程
}
```

### 当前设计的问题

⚠️ **问题**: 当前的 `Thread` 只是逻辑上的"线程"，不是真正的内核线程！

**实际上**:
- 当前的 `Thread` 更像是"任务组"或"执行上下文"
- 没有独立的栈空间
- 没有真正的线程上下文切换

### 建议的三层架构

```rust
// 1. 进程 (Process)
pub struct Process {
    pid: Pid,
    address_space: AddressSpace,  // 页表
    threads: Vec<ThreadId>,        // 线程列表
    resources: Resources,          // 文件描述符等
}

// 2. 线程 (Thread) - 内核级
pub struct Thread {
    tid: Tid,
    process_id: Pid,
    stack: Stack,                  // 独立栈空间
    context: Context,              // 寄存器上下文
    state: ThreadState,            // Ready/Running/Blocked
    async_runtime: Option<AsyncRuntime>,  // 可选的异步运行时
}

// 3. 异步任务 (Async Task) - 用户态
pub struct AsyncTask {
    id: TaskId,
    thread_id: Tid,                // 所属线程
    future: Pin<Box<dyn Future>>,  // Future 状态机
    state: TaskState,
}

// 异步运行时 (在线程内部)
pub struct AsyncRuntime {
    tasks: Vec<AsyncTask>,
    waker_map: HashMap<TaskId, Waker>,
}
```

---

## 实现建议

### 阶段 1: 先实现内核线程

```rust
// kernel/src/thread.rs
pub struct Thread {
    tid: usize,
    stack: [u8; 8192],  // 8KB 栈
    context: ThreadContext,
    state: ThreadState,
}

pub struct ThreadContext {
    // ARM Cortex-M 示例
    r4: usize,
    r5: usize,
    r6: usize,
    r7: usize,
    r8: usize,
    r9: usize,
    r10: usize,
    r11: usize,
    sp: usize,   // 栈指针
    lr: usize,   // 链接寄存器
    pc: usize,   // 程序计数器
    xpsr: usize, // 状态寄存器
}

// 线程切换 (汇编实现)
#[naked]
unsafe extern "C" fn switch_context(
    old_ctx: *mut ThreadContext,
    new_ctx: *const ThreadContext,
) {
    // 保存旧上下文
    // 恢复新上下文
    // 返回到新线程
}
```

### 阶段 2: 在线程中运行异步任务

```rust
// kernel/src/async_runtime.rs
pub struct AsyncRuntime {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
    wakers: Vec<Waker>,
}

impl AsyncRuntime {
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.tasks.push(Box::pin(future));
    }
    
    pub fn poll_all(&mut self) {
        for i in 0..self.tasks.len() {
            let waker = self.wakers[i].clone();
            let mut cx = Context::from_waker(&waker);
            
            // 轮询任务
            match self.tasks[i].as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // 任务完成，移除
                }
                Poll::Pending => {
                    // 继续等待
                }
            }
        }
    }
}

// 在线程入口函数中
#[no_mangle]
pub fn thread_entry() {
    // 创建异步运行时
    let mut runtime = AsyncRuntime::new();
    
    // 创建异步任务
    runtime.spawn(async {
        println!("Async task 1");
    });
    
    runtime.spawn(async {
        println!("Async task 2");
    });
    
    // 运行
    loop {
        runtime.poll_all();
        
        // 如果没有就绪任务，可以:
        // 1. 休眠 (WFI 指令)
        // 2. 让出线程时间片
    }
}
```

---

## 总结

### Rust 异步多任务的层级

| 特性 | 进程 | 线程 (内核) | 异步任务 (用户态) |
|------|------|-------------|------------------|
| **调度者** | 操作系统内核 | 操作系统内核 | 用户态运行时 |
| **并行性** | 是 (多核) | 是 (多核) | 否 (单线程内协作) |
| **上下文切换开销** | 大 (~10000 周期) | 中 (~1000 周期) | 小 (~50 周期) |
| **隔离性** | 完全隔离 | 共享地址空间 | 共享线程栈 |
| **创建开销** | 大 | 中 | 小 |
| **数量限制** | 数百 | 数千 | 数十万 |
| **通信方式** | IPC | 共享内存 | 共享变量 |

### FeatherCore 的实现路径

```
现在:
  └─ Scheduler (逻辑调度)
      ├─ "Thread" (不是真正的线程)
      └─ Task (Sync/Async)

短期 (1-2 个月):
  └─ Scheduler
      ├─ Thread (真正的内核线程，有栈和上下文)
      │   └─ 运行 Sync 任务
      └─ Process (进程，有地址空间)
          └─ 包含多个 Thread

中期 (3-6 个月):
  └─ Scheduler
      ├─ Process
      │   └─ Thread (内核线程)
      │       └─ AsyncRuntime (用户态)
      │           └─ AsyncTask (Future)
      └─ 支持真正的并行执行
```

### 关键设计决策

✅ **线程是内核调度的基本单位**  
✅ **异步任务在线程内部运行**  
✅ **一个线程可以运行多个异步任务**  
✅ **异步任务是协作式调度 (需要 await)**  
✅ **线程是抢占式调度 (定时器中断)**  

这就是 Rust 异步多任务与进程/线程的完整关系！
