# FeatherCore OS 设计文档

## 1. 概述

FeatherCore 是一个使用 Rust 编写的类 Unix 操作系统，参考了 Linux 的设计理念和架构。

### 1.1 设计目标

- **类 Unix API**: 提供熟悉的 Unix 系统调用接口
- **内存安全**: 完全使用 Rust，保证内存安全
- **模块化设计**: 清晰的模块划分，易于扩展
- **多架构支持**: 支持 ARM Cortex-M、RISC-V 等嵌入式架构

## 2. 系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                      应用层 (Application Layer)              │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │  Shell  │  │   ls    │  │  cat    │  │   ...   │        │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ 系统调用 (Syscalls)
┌─────────────────────────────────────────────────────────────┐
│                      内核层 (Kernel Layer)                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │     VFS      │  │   Process    │  │    Memory    │      │
│  │  (文件系统)  │  │   (进程管理)  │  │   (内存管理)  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Scheduler  │  │   Drivers    │  │    IPC       │      │
│  │   (调度器)   │  │   (驱动框架)  │  │ (进程通信)   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ 硬件抽象层 (HAL)
┌─────────────────────────────────────────────────────────────┐
│                   引导层 (Boot Layer)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Hardware   │  │  Device Tree │  │   Kernel     │      │
│  │    Init      │  │   (设备树)   │  │    Load      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      硬件层 (Hardware)                       │
│         ARM Cortex-M | RISC-V | x86_64 (未来)                │
└─────────────────────────────────────────────────────────────┘
```

## 3. 目录结构

```
FeatherCore/
├── boot/                       # Bootloader
│   ├── src/
│   │   └── main.rs            # 引导程序入口
│   ├── link.x.in              # 链接脚本模板
│   └── Cargo.toml
│
├── kernel/                     # 内核
│   ├── src/
│   │   ├── main.rs            # 内核入口
│   │   ├── lib.rs             # 内核库
│   │   ├── sched.rs           # 调度器
│   │   ├── task.rs            # 任务管理
│   │   ├── mm.rs              # 内存管理
│   │   ├── irq.rs             # 中断处理
│   │   ├── sync.rs            # 同步原语
│   │   ├── time.rs            # 时间管理
│   │   ├── log.rs             # 日志系统
│   │   ├── future.rs          # 异步运行时
│   │   ├── syscall/           # 系统调用接口
│   │   │   └── mod.rs
│   │   ├── vfs/               # 虚拟文件系统
│   │   │   └── mod.rs
│   │   └── driver/            # 驱动框架
│   │       └── mod.rs
│   ├── link.x.in              # 链接脚本模板
│   └── Cargo.toml
│
├── rootfs/                     # 根文件系统
│   ├── bin/                   # 基本工具
│   ├── sbin/                  # 系统工具
│   ├── etc/                   # 配置文件
│   ├── dev/                   # 设备文件
│   ├── proc/                  # 进程信息
│   ├── sys/                   # 系统信息
│   ├── home/                  # 用户目录
│   ├── var/                   # 可变数据
│   ├── tmp/                   # 临时文件
│   ├── init                   # 初始化脚本
│   ├── Cargo.toml             # rootfs 工具配置
│   └── src/
│       ├── shell/             # Shell 实现
│       └── coreutils/         # 基本工具集
│
├── common/                     # 公共库
│   ├── src/
│   │   ├── lib.rs
│   │   ├── platform.rs        # 平台管理
│   │   └── generated/         # 生成的代码
│   │       └── devicetree.rs
│   └── arch/                  # 架构相关代码
│       ├── arm/
│       └── riscv/
│
├── build_tool/                 # 构建工具
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       ├── linker.rs
│       └── device_tree.rs
│
└── build.sh                    # 构建脚本
```

## 4. 核心模块说明

### 4.1 Boot (引导程序)

**职责**:
- 硬件初始化 (时钟、内存、GPIO)
- 设备树解析
- 加载内核到内存
- 跳转到内核入口

**关键流程**:
```
1. CPU 复位 -> _start()
2. 设置栈指针
3. 初始化平台 (PlatformManager::init())
4. 解析设备树
5. 初始化关键硬件 (UART、时钟)
6. 从存储加载内核
7. 设置内核参数
8. 跳转到内核入口
```

### 4.2 Kernel (内核)

#### 4.2.1 调度器 (sched.rs)

**特性**:
- 基于优先级的抢占式调度
- 支持线程和任务两级抽象
- 异步任务支持 (Future-based)
- 轮转调度 (Round-Robin)

**调度策略**:
```
1. 首先轮询当前线程中所有就绪的异步任务
2. 在同一线程内切换任务 (无需线程上下文切换)
3. 如果当前线程没有就绪任务，查找其他线程
4. 线程间切换需要保存/恢复上下文
```

#### 4.2.2 任务管理 (task.rs)

**任务类型**:
- **同步任务**: 具有入口点的传统任务
- **异步任务**: 基于 Future 的异步任务

**任务状态**:
- Ready: 就绪状态
- Running: 运行状态
- Blocked: 阻塞状态
- Suspended: 挂起状态
- Terminated: 终止状态

#### 4.2.3 内存管理 (mm.rs)

**功能**:
- 堆内存分配 (GlobalAllocator)
- 内存块管理 (BlockHeader)
- 内存回收和合并

**实现**:
- 使用链表管理空闲内存块
- 支持内存块分割和合并
- First-fit 分配策略

#### 4.2.4 虚拟文件系统 (vfs/mod.rs)

**设计**:
- Unix-like 文件抽象
- 统一的文件操作接口 (FileOps trait)
- 文件描述符表管理

**支持的操作**:
- open/close
- read/write
- stat
- seek

**文件类型**:
- 普通文件 (S_IFREG)
- 目录 (S_IFDIR)
- 字符设备 (S_IFCHR)
- 块设备 (S_IFBLK)
- 符号链接 (S_IFLNK)
- 套接字 (S_IFSOCK)

#### 4.2.5 系统调用 (syscall/mod.rs)

**系统调用号**:
```rust
// 进程控制
SYS_EXIT = 1
SYS_FORK = 2
SYS_READ = 3
SYS_WRITE = 4
SYS_OPEN = 5
SYS_CLOSE = 6
SYS_GETPID = 20

// 内存管理
SYS_MMAP = 90
SYS_MUNMAP = 91
SYS_BRK = 93

// 文件系统
SYS_STAT = 4
SYS_LSEEK = 8
SYS_GETDENTS = 89
```

**调用约定**:
```
参数通过寄存器传递:
- arg0, arg1, arg2, arg3, arg4, arg5
返回值:
- ret0, ret1
```

#### 4.2.6 驱动框架 (driver/mod.rs)

**设备类型**:
- Block: 块设备
- Character: 字符设备
- Network: 网络设备

**设备操作接口**:
```rust
trait DeviceOps {
    fn open(&self) -> Result<(), ()>;
    fn close(&self) -> Result<(), ()>;
    fn read(&self, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&self, buf: &[u8]) -> Result<usize, ()>;
    fn ioctl(&self, cmd: u32, arg: usize) -> Result<usize, ()>;
}
```

### 4.3 Rootfs (根文件系统)

**目录结构**:
```
rootfs/
├── bin/          # 基本命令 (sh, ls, cat, etc.)
├── sbin/         # 系统管理命令
├── etc/          # 配置文件
│   ├── passwd    # 用户信息
│   ├── group     # 组信息
│   ├── profile   # 系统环境变量
│   └── rc.local  # 启动脚本
├── dev/          # 设备文件 (/dev/null, /dev/zero, etc.)
├── proc/         # 进程信息 (伪文件系统)
├── sys/          # 系统信息 (伪文件系统)
├── home/         # 用户主目录
├── var/          # 可变数据 (日志、缓存)
├── tmp/          # 临时文件
└── init          # 初始化进程
```

**基本工具集**:
- **sh**: Shell 解释器
- **ls**: 列出目录内容
- **cat**: 连接和打印文件
- **echo**: 显示文本
- **cd**: 切换目录
- **pwd**: 显示当前目录
- **mkdir**: 创建目录
- **rm**: 删除文件
- **cp**: 复制文件
- **mv**: 移动文件
- **ps**: 显示进程状态
- **mount**: 挂载文件系统

## 5. 构建流程

### 5.1 构建顺序

```
1. build_tool (std)          ──解析──>  platform/
2. build_tool (std)          ──生成──>  common/src/generated/
3. build_tool (std)          ──生成──>  boot/link.x
4. build_tool (std)          ──生成──>  kernel/link.x
5. boot (no_std, target)     ──编译──>  boot.elf
6. kernel (no_std, target)   ──编译──>  kernel.elf
7. rootfs (no_std, target)   ──编译──>  rootfs utilities
```

### 5.2 使用构建脚本

```bash
# 完整构建
./build.sh build-all stm32f429i-disc thumbv7em-none-eabihf

# 单独构建组件
./build.sh build-tool
./build.sh generate stm32f429i-disc
./build.sh build-boot stm32f429i-disc thumbv7em-none-eabihf
./build.sh build-kernel stm32f429i-disc thumbv7em-none-eabihf

# 清理
./build.sh clean
```

## 6. 启动流程

### 6.1 Boot 阶段

```
1. CPU 复位
   ↓
2. 执行 _start() (汇编)
   ↓
3. 设置栈指针 (SP)
   ↓
4. 调用 boot_main()
   ↓
5. PlatformManager::init()
   ↓
6. 解析设备树
   ↓
7. 初始化硬件 (时钟、内存、UART)
   ↓
8. 从 Flash 加载内核到 RAM
   ↓
9. 设置内核启动参数
   ↓
10. 跳转到 kernel_main()
```

### 6.2 Kernel 阶段

```
1. kernel_main()
   ↓
2. init_kernel()
   ├─ init_memory_from_device_tree()
   ├─ init_interrupt_controller()
   ├─ init_drivers_from_device_tree()
   └─ init_services()
   ↓
3. start_scheduler()
   ├─ 创建空闲任务
   ├─ 创建主线程
   └─ 开始调度
   ↓
4. 执行第一个用户空间任务
   ↓
5. 执行 /init 脚本
   ↓
6. 启动 Shell
```

### 6.3 用户空间初始化

```
1. 执行 /init
   ├─ 挂载 proc, sysfs, devtmpfs
   ├─ 设置 hostname
   ├─ 执行 /etc/rc.local
   └─ 启动 /bin/sh
   ↓
2. Shell 读取 /etc/profile
   ↓
3. 显示提示符，等待用户输入
```

## 7. 系统调用接口

### 7.1 进程管理

```rust
// 退出进程
sys_exit(exit_code: i32) -> !

// 获取进程 ID
sys_getpid() -> usize

// 创建子进程 (未来实现)
sys_fork() -> usize

// 执行新程序 (未来实现)
sys_execve(path: &str, argv: &[&str], envp: &[&str]) -> usize

// 等待子进程 (未来实现)
sys_waitpid(pid: usize, status: &mut i32) -> usize
```

### 7.2 文件操作

```rust
// 打开文件
sys_open(pathname: &str, flags: u32, mode: u32) -> usize

// 关闭文件描述符
sys_close(fd: usize) -> usize

// 读取文件
sys_read(fd: usize, buf: &mut [u8]) -> usize

// 写入文件
sys_write(fd: usize, buf: &[u8]) -> usize

// 获取文件状态
sys_stat(path: &str) -> Stat

// 文件定位
sys_lseek(fd: usize, offset: i64, whence: i32) -> u64
```

### 7.3 内存管理

```rust
// 内存映射 (未来实现)
sys_mmap(addr: usize, length: usize, prot: u32, flags: u32, fd: usize, offset: u64) -> usize

// 取消内存映射 (未来实现)
sys_munmap(addr: usize, length: usize) -> usize

// 设置内存保护 (未来实现)
sys_mprotect(addr: usize, length: usize, prot: u32) -> usize

// 修改数据段大小 (未来实现)
sys_brk(addr: usize) -> usize
```

## 8. 设备树格式

### 8.1 基本结构

```toml
# STM32F4 设备树示例

/ {
    compatible = "st,stm32f405", "st,stm32f4";
    
    # 时钟定义
    clocks {
        hsi: hsi@0 {
            compatible = "st,stm32f4-hsi";
            clock-frequency = <16000000>;
        };
    };
    
    # GPIO 端口
    gpioa: gpio@40020000 {
        compatible = "st,stm32f4-gpio";
        reg = <0x40020000 0x400>;
        interrupts = <0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15>;
    };
    
    # 串口
    usart1: serial@40011000 {
        compatible = "st,stm32f4-usart";
        reg = <0x40011000 0x400>;
        interrupts = <37>;
        clocks = <&rcc 0>;
    };
}
```

### 8.2 配置层次

```
chip 配置 (通用)          board 配置 (定制)
┌─────────────────┐       ┌─────────────────┐
│ [chip]          │       │ chip = "stm32f4"│
│ name = "stm32f4"│       │                 │
│ vendor = "st"   │       │ [board]         │
│                 │       │ name = "xxx"    │
│ [cpu]           │       │                 │
│ core = "cortex-m4"      │ [supported]     │
│ frequency = 180MHz      │ gpio = true     │
│                 │       │ uart = true     │
│ [memory]        │       │                 │
│ flash = 2MB     │       │ [pinout]        │
│ sram = 192KB    │       │ led1 = "PG13"   │
│                 │       │ uart_tx = "PA9" │
│ [bootloader]    │       │                 │
│ size = 64KB     │       └─────────────────┘
└─────────────────┘              ▲
        │                       │
        │  继承 / Inherit       │
        │  覆盖 / Override     │
        └───────────────────────┘
```

## 9. 开发指南

### 9.1 添加新驱动

1. 在 `kernel/src/driver/` 创建新驱动文件
2. 实现 `DeviceOps` trait
3. 在设备树中添加设备节点
4. 在初始化代码中注册驱动

### 9.2 添加新系统调用

1. 在 `kernel/src/syscall/mod.rs` 添加系统调用号
2. 实现系统调用处理函数
3. 在 `handle_syscall()` 中注册处理程序
4. 更新用户空间库以支持新调用

### 9.3 添加新开发板支持

1. 在 `platform/board/` 创建板级配置
2. 在 `platform/chip/` 创建芯片配置 (如需要)
3. 创建设备树文件 (.dts 或 .toml)
4. 在 `boot/Cargo.toml` 和 `kernel/Cargo.toml` 添加 feature
5. 更新构建工具以支持新板

## 10. 参考资料

- **U-Boot**: Bootloader 参考实现
- **Linux Kernel**: 内核设计参考
- **BusyBox**: rootfs 工具参考
- **Rust Embedded**: 嵌入式 Rust 开发指南

## 11. 未来计划

- [ ] 完善系统调用实现
- [ ] 实现完整的 VFS (支持多种文件系统)
- [ ] 添加网络栈支持
- [ ] 实现多进程支持
- [ ] 添加 POSIX 线程 (pthreads) 支持
- [ ] 支持 x86_64 架构
- [ ] 实现图形界面支持
