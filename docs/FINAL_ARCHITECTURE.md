# FeatherCore Common 库架构（最终版）

## 核心架构

```
common/
├── Cargo.toml                  # 聚合库定义
├── src/
│   └── lib.rs                  # 只 re-export 子库
│
├── arch/                       # 架构支持库 (单一子库)
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs              # 根据 feature 选择架构模块
│   ├── arm/                    # ARM 架构模块
│   │   ├── mod.rs
│   │   ├── cpu.rs
│   │   ├── cache.rs
│   │   └── ...
│   └── riscv/                  # RISC-V 架构模块
│       ├── mod.rs
│       └── ...
│
├── sys/                        # 系统功能库 (单一子库)
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── mmu/                    # MMU 模块 (属于 sys)
│   │   ├── mod.rs
│   │   ├── types.rs
│   │   ├── no_mmu.rs
│   │   └── arm_mmu.rs
│   ├── cpu/                    # CPU 模块
│   ├── interrupt/              # 中断模块
│   ├── memory/                 # 内存模块
│   └── clock/                  # 时钟模块
│
├── driver/                     # 驱动库 (单一子库)
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── gpio/
│   ├── spi/
│   ├── i2c/
│   ├── serial/
│   ├── i2s/
│   └── timer/
│
├── platform/                   # 平台支持库 (单一子库)
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── stm32f4/               # STM32F4 平台模块
│   ├── stm32h7/               # STM32H7 平台模块
│   └── ...
│
└── generated/                  # 生成代码库 (单一子库)
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    └── devicetree/            # 设备树生成模块
```

## 关键设计原则

### 1. common 是纯聚合库

**common/src/lib.rs**:
```rust
//! FeatherCore Common Library
//! 
//! 这是聚合库，只 re-export 子库，本身无功能代码。

#![no_std]

// 导出架构库
#[cfg(feature = "arch")]
pub use feathercore_arch as arch;

// 导出系统库
#[cfg(feature = "sys")]
pub use feathercore_sys as sys;

// 导出驱动库
#[cfg(feature = "driver")]
pub use feathercore_driver as driver;

// 导出平台库
#[cfg(feature = "platform")]
pub use feathercore_platform as platform;

// 导出生成代码库
#[cfg(feature = "generated")]
pub use feathercore_generated as generated;
```

### 2. arch 是单一子库，不同架构是模块

**arch/Cargo.toml**:
```toml
[package]
name = "feathercore-arch"
version = "0.1.0"
edition = "2021"

[features]
# 架构选择
arm = []
riscv = []

# ARM 变体
armv7-m = ["arm"]
armv7-em = ["arm"]
armv8-a = ["arm"]

# RISC-V 变体
riscv32imac = ["riscv"]
riscv64gc = ["riscv"]
```

**arch/src/lib.rs**:
```rust
//! FeatherCore Architecture Library

#![no_std]

// 根据 feature 选择架构模块
#[cfg(feature = "arm")]
pub mod arm;

#[cfg(feature = "riscv")]
pub mod riscv;

// 导出当前架构
#[cfg(feature = "arm")]
pub use arm::*;

#[cfg(feature = "riscv")]
pub use riscv::*;
```

**arch/arm/mod.rs**:
```rust
//! ARM Architecture Module

mod cpu;
mod cache;
mod mmu;

pub use cpu::CpuContext;
pub use cache::CacheOps;
pub use mmu::PageTable;
```

### 3. sys 包含所有系统级功能（包括 MMU）

**sys/Cargo.toml**:
```toml
[package]
name = "feathercore-sys"
version = "0.1.0"
edition = "2021"

[features]
# MMU 支持
no_mmu = []
with_mmu = []
```

**sys/src/lib.rs**:
```rust
//! FeatherCore System Library

#![no_std]

pub mod mmu;
pub mod cpu;
pub mod interrupt;
pub mod memory;
pub mod clock;

// 导出常用类型
pub use mmu::{MemFlags, PhysAddr, VirtAddr};
pub use cpu::CpuId;
pub use interrupt::InterruptNumber;
```

**sys/mmu/mod.rs**:
```rust
//! MMU Module - Memory Management Unit

mod types;

#[cfg(feature = "no_mmu")]
mod no_mmu;

#[cfg(feature = "with_mmu")]
#[cfg(target_arch = "arm")]
mod arm_mmu;

pub use types::*;

#[cfg(feature = "no_mmu")]
pub use no_mmu::NoMmu;

#[cfg(feature = "with_mmu")]
#[cfg(target_arch = "arm")]
pub use arm_mmu::ArmMmu;
```

### 4. driver 包含所有设备驱动

**driver/src/lib.rs**:
```rust
//! FeatherCore Driver Library

#![no_std]

pub mod gpio;
pub mod spi;
pub mod i2c;
pub mod serial;
pub mod i2s;
pub mod timer;

// 导出常用类型
pub use gpio::{GpioDriver, GpioConfig};
pub use spi::{SpiDriver, SpiConfig};
pub use serial::{SerialDriver, SerialConfig};
```

### 5. platform 包含平台特定实现

**platform/src/lib.rs**:
```rust
//! FeatherCore Platform Library

#![no_std]

// 根据 feature 选择平台
#[cfg(feature = "stm32f4")]
pub mod stm32f4;

#[cfg(feature = "stm32h7")]
pub mod stm32h7;

#[cfg(feature = "raspi3")]
pub mod raspi3;

// 导出当前平台
#[cfg(feature = "stm32f4")]
pub use stm32f4::*;
```

## 完整目录结构

```
common/
├── Cargo.toml
├── src/
│   └── lib.rs
│
├── arch/                       # 架构库
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── arm/                    # ARM 模块
│   │   ├── mod.rs
│   │   ├── cpu.rs
│   │   ├── cache.rs
│   │   ├── mmu.rs
│   │   └── interrupt.rs
│   └── riscv/                  # RISC-V 模块
│       ├── mod.rs
│       ├── cpu.rs
│       └── ...
│
├── sys/                        # 系统库
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── mmu/                    # MMU 模块 ⭐
│   │   ├── mod.rs
│   │   ├── types.rs
│   │   ├── no_mmu.rs
│   │   └── arm_mmu.rs
│   ├── cpu/
│   │   ├── mod.rs
│   │   └── ...
│   ├── interrupt/
│   ├── memory/
│   └── clock/
│
├── driver/                     # 驱动库
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── gpio/
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   ├── error.rs
│   │   └── traits.rs
│   ├── spi/
│   ├── i2c/
│   ├── serial/
│   ├── i2s/
│   └── timer/
│
├── platform/                   # 平台库
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs
│   ├── stm32f4/
│   │   ├── mod.rs
│   │   ├── clock.rs
│   │   ├── interrupt.rs
│   │   └── memory.rs
│   ├── stm32h7/
│   └── ...
│
└── generated/                  # 生成代码库
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    └── devicetree/
        ├── mod.rs
        └── ...
```

## 使用示例

### 在 boot 中使用

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

use feathercore_common::arch::arm::cpu;
use feathercore_common::sys::mmu;
use feathercore_common::driver::uart;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化 CPU (使用 arch 库)
    cpu::init();
    
    // 初始化 MMU (使用 sys 库)
    #[cfg(feature = "with_mmu")]
    mmu::init();
    
    // 初始化串口 (使用 driver 库)
    let uart = uart::UART::new(0x40011000);
    uart.write(b"Boot starting...\n");
}
```

### Cargo.toml 配置

```toml
# boot/Cargo.toml
[dependencies]
feathercore-common = { 
    path = "../common",
    features = [
        "arch",           # 启用架构库
        "sys",            # 启用系统库
        "driver",         # 启用驱动库
        "arm",            # 选择 ARM 架构
        "armv7-m",        # 选择 ARMv7-M (Cortex-M4)
        "no_mmu",         # 不带 MMU
        "stm32f4",        # STM32F4 平台
    ]
}
```

## Cargo Features 层次

```
common (聚合层)
  ├─ arch (架构库)
  │   ├─ arm (ARM 模块)
  │   └─ riscv (RISC-V 模块)
  │
  ├─ sys (系统库)
  │   ├─ mmu (MMU 模块)
  │   ├─ cpu (CPU 模块)
  │   ├─ interrupt (中断模块)
  │   ├─ memory (内存模块)
  │   └─ clock (时钟模块)
  │
  ├─ driver (驱动库)
  │   ├─ gpio
  │   ├─ spi
  │   └─ ...
  │
  └─ platform (平台库)
      ├─ stm32f4
      └─ stm32h7
```

## 总结

### ✅ 架构优势

1. **清晰的层次**:
   - common (聚合) → arch/sys/driver/platform (子库) → 模块

2. **模块化**:
   - arch: 单一库，不同架构是模块
   - sys: 包含所有系统级功能 (MMU, CPU, Interrupt)
   - driver: 包含所有设备驱动
   - platform: 包含所有平台支持

3. **灵活性**:
   - 通过 features 选择架构
   - 通过 features 选择平台
   - 通过 features 配置 MMU

4. **可维护性**:
   - 每个子库独立
   - 模块在 src 同级目录
   - lib.rs 只声明和导出

### ✅ 关键修正

- ✅ MMU 属于 sys 子库
- ✅ arch 是单一子库，不同架构是模块
- ✅ 所有子库的 src 下只有 lib.rs
- ✅ 模块在 src 同级目录

这就是 FeatherCore Common 的最终正确架构！
