# FeatherCore OS 架构关系说明

## 核心架构原则

```
┌─────────────────────────────────────────────────────────────┐
│                    编译时依赖关系                            │
└─────────────────────────────────────────────────────────────┘

                    ┌──────────────┐
                    │    common    │
                    │  (no_std)    │
                    └──────┬───────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 │
┌──────────────┐   ┌──────────────┐          │
│     boot     │   │   kernel     │          │
│  (no_std)    │   │  (no_std)    │          │
│              │   │              │          │
│  硬件初始化   │   │  操作系统    │          │
│  设备树解析   │   │  系统调用    │          │
│  加载内核    │   │  VFS         │          │
│              │   │  调度器      │          │
└──────────────┘   └──────┬───────┘          │
         │                │                  │
         │                │                  │
         │                ▼                  │
         │         ┌──────────────┐          │
         │         │    rootfs    │◄─────────┘
         │         │  (no_std)    │
         │         │              │
         │         │  Shell       │
         │         │  基本工具     │
         │         │              │
         │         └──────────────┘
         │
         │
         ▼
┌─────────────────────────────────────────┐
│              运行时流程                  │
└─────────────────────────────────────────┘

1. boot 初始化硬件
         │
         │ (跳转到)
         ▼
2. kernel 启动操作系统
         │
         │ (挂载)
         ▼
3. rootfs 作为根文件系统
         │
         │ (执行)
         ▼
4. /init 程序启动用户空间
```

## 依赖关系详解

### 1. common (公共库)
- **特性**: `no_std`
- **职责**: 提供公共功能、架构抽象、驱动框架、设备树解析
- **依赖**: 无内部依赖 (仅依赖 Rust 标准库的 core/alloc)
- **被依赖**: boot, kernel, rootfs

### 2. boot (Bootloader)
- **特性**: `no_std`
- **职责**: 
  - CPU 和硬件初始化
  - 设备树解析
  - 从存储加载内核到内存
  - 跳转到内核入口点
- **依赖**: `feathercore-common`
- **不依赖**: kernel, rootfs
- **关系**: 与 kernel 无编译时依赖，只有运行时跳转关系

### 3. kernel (内核)
- **特性**: `no_std`
- **职责**:
  - 任务调度
  - 内存管理
  - 中断处理
  - 虚拟文件系统 (VFS)
  - 系统调用接口
  - 设备驱动框架
- **依赖**: `feathercore-common`
- **不依赖**: boot, rootfs
- **关系**: 
  - 与 boot 无编译时依赖
  - 与 rootfs 无编译时依赖，只有运行时挂载关系

### 4. rootfs (根文件系统)
- **特性**: `no_std`
- **职责**:
  - 提供用户空间工具 (Shell, coreutils)
  - 系统配置文件
  - 初始化脚本 (/init)
- **依赖**: `feathercore-common`, `feathercore-kernel`
- **关系**: 
  - 依赖 kernel 提供的系统调用接口
  - 运行时被 kernel 挂载为根文件系统

## 编译时 vs 运行时关系

### 编译时依赖 (Cargo.toml)
```
common ← boot
common ← kernel
common, kernel ← rootfs
```

### 运行时流程
```
boot → (跳转到) → kernel → (挂载) → rootfs → (执行) → /init
```

## 关键设计决策

### 为什么 boot 不依赖 kernel？
1. **职责分离**: boot 只负责硬件初始化和加载内核
2. **独立性**: boot 可以独立编译和测试
3. **灵活性**: 可以支持不同的内核镜像格式
4. **大小优化**: boot 保持最小化，不包含内核代码

### 为什么 kernel 不依赖 rootfs？
1. **解耦**: 内核不关心具体的 rootfs 内容
2. **灵活性**: 可以挂载不同的 rootfs 镜像
3. **模块化**: 内核和 rootfs 可以独立开发和更新
4. **运行时绑定**: rootfs 在运行时通过 VFS 挂载，不是编译时绑定

### 为什么 rootfs 依赖 kernel？
1. **系统调用**: rootfs 工具需要使用 kernel 提供的 syscall 接口
2. **类型共享**: 共享文件描述符、错误码等类型定义
3. **接口一致性**: 确保用户空间工具和内核接口一致

## 内存布局

```
┌─────────────────────────────────────────┐
│  Flash Memory                           │
│  ┌─────────────────────────────────┐    │
│  │ boot (Bootloader)               │    │
│  │ - 硬件初始化代码                 │    │
│  │ - 设备树解析代码                 │    │
│  │ - 内核加载代码                   │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │ kernel (操作系统内核)            │    │
│  │ - 调度器                         │    │
│  │ - 内存管理                       │    │
│  │ - VFS                            │    │
│  │ - 系统调用                       │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │ rootfs (根文件系统镜像)          │    │
│  │ - /init                          │    │
│  │ - /bin/*                         │    │
│  │ - /etc/*                         │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│  RAM (运行时)                           │
│  ┌─────────────────────────────────┐    │
│  │ Boot 数据                        │    │
│  │ (boot 完成后可能被覆盖)            │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │ Kernel (运行中)                  │    │
│  │ - 内核栈                         │    │
│  │ - 堆内存                         │    │
│  │ - 任务栈                         │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │ Rootfs (已挂载)                  │    │
│  │ - VFS 挂载点                      │    │
│  │ - 打开的文件                     │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │ 用户空间进程栈                   │    │
│  │ - /init 进程                     │    │
│  │ - Shell 进程                     │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

## 启动流程详解

### 阶段 1: Boot
```
CPU Reset
    ↓
boot::_start() [汇编 - 设置栈]
    ↓
boot::boot_main()
    ├─ PlatformManager::init()
    ├─ 解析设备树
    ├─ 初始化时钟、内存、UART
    ├─ 从 Flash 加载 kernel 到 RAM
    └─ 跳转到 kernel::_start()
```

### 阶段 2: Kernel
```
kernel::_start()
    ↓
kernel::kernel_main()
    ├─ init_kernel()
    │  ├─ init_memory_from_device_tree()
    │  ├─ init_interrupt_controller()
    │  ├─ init_drivers_from_device_tree()
    │  └─ init_services()
    ├─ start_scheduler()
    ├─ 挂载 rootfs (VFS::mount)
    └─ 执行 /init 程序
```

### 阶段 3: Rootfs
```
/init 程序启动
    ├─ 挂载 proc, sysfs, devtmpfs
    ├─ 执行 /etc/rc.local
    └─ 启动 /bin/sh
    
Shell 等待用户输入
    ├─ 读取命令
    ├─ fork/exec 子进程
    └─ 等待命令完成
```

## Cargo.toml 配置要点

### common/Cargo.toml
```toml
[package]
name = "feathercore-common"
# no_std 在 src/lib.rs 中声明

[dependencies]
# 子库都是可选的
feathercore-arch-arm = { path = "./arch/arm", optional = true }
feathercore-driver = { path = "./driver", optional = true }
```

### boot/Cargo.toml
```toml
[package]
name = "feathercore-boot"
# no_std 在 src/main.rs 中声明

[dependencies]
feathercore-common = { path = "../common", features = ["driver", "devicetree"] }
# 不依赖 kernel 或 rootfs
```

### kernel/Cargo.toml
```toml
[package]
name = "feathercore-kernel"
# no_std 在 src/lib.rs 和 src/main.rs 中声明

[dependencies]
feathercore-common = { path = "../common", features = ["driver", "devicetree"] }
# 不依赖 boot 或 rootfs
```

### rootfs/Cargo.toml
```toml
[package]
name = "feathercore-rootfs"
# no_std 在所有工具源码中声明

[dependencies]
feathercore-common = { path = "../common" }
feathercore-kernel = { path = "../kernel" }
# 依赖 kernel 以使用系统调用接口
```

## 总结

FeatherCore OS 采用清晰的三层架构:

1. **Boot 层**: 纯硬件初始化，无 OS 概念
2. **Kernel 层**: 操作系统核心，提供抽象和服务
3. **Rootfs 层**: 用户空间工具，使用 kernel 提供的服务

三层之间通过明确的接口交互:
- Boot → Kernel: 运行时跳转 (无编译依赖)
- Kernel → Rootfs: 运行时挂载 (编译时 rootfs 依赖 kernel 接口)
- 所有层都依赖 common 提供的公共功能

这种设计保证了:
- ✅ 职责分离
- ✅ 编译独立性
- ✅ 运行时灵活性
- ✅ 代码可维护性
