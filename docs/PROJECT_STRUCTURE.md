# FeatherCore 项目架构说明

## 项目结构总览

```
FeatherCore/
├── boot/                      # 二进制目标 (Bootloader)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # 入口点
│       ├── mod.rs             # 根模块
│       ├── arch/              # 架构相关模块
│       ├── drivers/           # Boot 阶段驱动
│       └── loader/            # 内核加载器
│
├── kernel/                    # 二进制目标 (操作系统内核)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs            # 入口点
│       ├── lib.rs             # 内核库
│       └── ...
│
├── common/                    # 聚合库 (Workspace)
│   ├── Cargo.toml             # 定义 workspace
│   ├── arch/                  # 架构子库
│   │   ├── arm/               # ARM 架构实现
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── cpu.rs
│   │   │       ├── mmu.rs
│   │   │       └── ...
│   │   └── riscv/             # RISC-V 架构实现
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs
│   │           └── ...
│   ├── driver/                # 驱动子库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── gpio/
│   │       ├── spi/
│   │       ├── i2c/
│   │       └── ...
│   ├── mmu/                   # MMU 抽象子库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── no_mmu.rs
│   │       └── arm_mmu.rs
│   └── platform/              # 平台支持子库
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── stm32f4/
│           ├── stm32h7/
│           └── ...
│
└── rootfs/                    # 多二进制目标 (用户空间工具)
    ├── Cargo.toml
    └── src/
        ├── shell/
        └── coreutils/
```

## 依赖关系

```
boot (二进制)
  └─→ common (聚合库)
      ├─→ common::arch (ARM 或 RISC-V)
      ├─→ common::driver
      ├─→ common::mmu
      └─→ common::platform

kernel (二进制)
  └─→ common (聚合库)
      ├─→ common::arch
      ├─→ common::driver
      ├─→ common::mmu
      └─→ common::platform

rootfs (多二进制)
  └─→ common (聚合库)
  └─→ kernel (库)
```

## 设计原则

### 1. boot 是二进制目标

- **源码**: `boot/src/main.rs` 是入口点
- **模块组织**: `boot/src/` 下可以有多个模块
- **依赖**: 只依赖 `common` 聚合库
- **产物**: 可执行的 ELF 二进制文件

示例 boot 模块结构:
```
boot/src/
├── main.rs              # #![no_std] #![no_main] fn _start()
├── lib.rs               # 可选：共享代码
├── mod.rs               # 根模块声明
├── arch/                # 架构相关
│   ├── mod.rs
│   ├── arm.rs
│   └── riscv.rs
├── drivers/             # Boot 阶段驱动
│   ├── mod.rs
│   ├── uart.rs
│   └── flash.rs
└── loader/              # 内核加载
    ├── mod.rs
    └── elf.rs
```

### 2. common 是纯聚合库

**关键点**:
- ❌ **common 本身不包含功能代码**
- ✅ **common 只是 re-export 子库**
- ✅ **每个子库是独立的 Cargo 项目**
- ✅ **子库有自己的模块组织**

```rust
// common/src/lib.rs - 只是 re-export
#![no_std]

// 根据 feature 选择架构
#[cfg(feature = "arm")]
pub use feathercore_arch_arm as arch;

#[cfg(feature = "riscv")]
pub use feathercore_arch_riscv as arch;

// 导出驱动库
#[cfg(feature = "driver")]
pub use feathercore_driver as driver;

// 导出 MMU 库
#[cfg(feature = "with_mmu")]
pub use feathercore_mmu as mmu;

// 导出平台库
#[cfg(feature = "platform")]
pub use feathercore_platform as platform;
```

### 3. 子库是独立的 Cargo 项目

每个子库都有自己的 `Cargo.toml` 和模块结构：

```
common/arch/arm/
├── Cargo.toml           # 独立的 Cargo 项目
└── src/
    ├── lib.rs           # 库的根
    ├── mod.rs           # 模块声明
    ├── cpu/             # CPU 相关模块
    │   ├── mod.rs
    │   ├── context.rs
    │   └── exception.rs
    ├── mmu/             # MMU 模块
    │   ├── mod.rs
    │   ├── page_table.rs
    │   └── tlb.rs
    └── cache/           # 缓存模块
        ├── mod.rs
        └── operations.rs
```

## Cargo.toml 配置

### boot/Cargo.toml (二进制)

```toml
[package]
name = "feathercore-boot"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "feathercore-boot"
path = "src/main.rs"

[dependencies]
# 只依赖聚合库
feathercore-common = { path = "../common" }

[features]
# 通过 common 的 features 选择平台和架构
default = ["stm32f4"]
stm32f4 = ["feathercore-common/stm32f4"]
```

### common/Cargo.toml (聚合库)

```toml
[package]
name = "feathercore-common"
version = "0.1.0"
edition = "2021"

# 注意：没有 [lib] 段，因为这是 workspace
# 或者有一个空的 lib.rs 只做 re-export

[dependencies]
# 子库作为依赖
feathercore-arch-arm = { path = "./arch/arm", optional = true }
feathercore-arch-riscv = { path = "./arch/riscv", optional = true }
feathercore-driver = { path = "./driver", optional = true }
feathercore-mmu = { path = "./mmu", optional = true }
feathercore-platform = { path = "./platform", optional = true }

[features]
# 架构选择
arm = ["feathercore-arch-arm"]
riscv = ["feathercore-arch-riscv"]

# 平台选择
stm32f4 = ["arm", "feathercore-platform/stm32f4"]
stm32h7 = ["arm", "feathercore-platform/stm32h7"]

# MMU 选择
no_mmu = ["feathercore-mmu/no_mmu"]
with_mmu = ["feathercore-mmu/with_mmu"]
```

### common/arch/arm/Cargo.toml (子库)

```toml
[package]
name = "feathercore-arch-arm"
version = "0.1.0"
edition = "2021"

[lib]
name = "feathercore_arch_arm"
path = "src/lib.rs"

[features]
# ARM 变体
armv7-m = []
armv7-em = []
armv7-a = []
armv8-a = []
```

## 使用示例

### boot 中使用 common

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

// 通过 common 使用子库
use feathercore_common::arch::cpu;
use feathercore_common::driver::uart;
use feathercore_common::platform::stm32f4;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化 CPU
    cpu::init();
    
    // 初始化串口
    let uart = uart::UART::new(stm32f4::USART1_BASE);
    uart.write(b"Boot starting...\n");
    
    // 加载内核
    // ...
}
```

### common 子库的模块组织

```rust
// common/arch/arm/src/lib.rs
#![no_std]

pub mod cpu;
pub mod mmu;
pub mod cache;
pub mod interrupt;

// 导出公共 API
pub use cpu::CpuContext;
pub use mmu::PageTable;
```

```rust
// common/arch/arm/src/cpu/mod.rs
pub mod context;
pub mod exception;

pub use context::CpuContext;
pub use exception::ExceptionFrame;
```

## 编译流程

### 编译 boot

```bash
cd boot

# 为 STM32F4 编译
cargo build --release \
  --features stm32f4 \
  --target thumbv7em-none-eabihf

# 产物
target/thumbv7em-none-eabihf/release/feathercore-boot
```

### 编译过程

1. Cargo 解析 `boot/Cargo.toml`
2. 发现依赖 `feathercore-common`
3. 解析 `common/Cargo.toml`
4. 根据 feature `stm32f4` 激活:
   - `feathercore-arch-arm` (带 armv7-m feature)
   - `feathercore-driver`
   - `feathercore-platform/stm32f4`
5. 编译所有子库
6. 编译 boot 二进制

## 关键优势

### 1. 模块化

- 每个子库独立开发、测试
- 可以单独发布子库到 crates.io
- 清晰的职责分离

### 2. 灵活性

- 通过 features 选择需要的组件
- 可以轻松添加新架构、新平台
- 不会引入不需要的代码

### 3. 可维护性

- common 本身很简单，只是聚合
- 功能代码都在子库中
- 每个子库有清晰的模块结构

### 4. 编译优化

- 不需要的子库不会编译
- LTO 可以跨子库优化
- 编译缓存更有效

## 总结

✅ **boot**: 二进制目标，有自己的模块组织  
✅ **common**: 纯聚合库，re-export 子库  
✅ **子库**: 独立的 Cargo 项目，有自己的模块  
✅ **依赖**: boot → common → 子库  

这种设计保证了清晰的架构和良好的可维护性！
