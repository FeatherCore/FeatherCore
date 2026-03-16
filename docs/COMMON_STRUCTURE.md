# FeatherCore Common 库架构规范

## 核心原则

### 1. common 是纯聚合库

```
common/
├── Cargo.toml
└── src/
    └── lib.rs          # 只做 re-export，无其他文件
```

**common/src/lib.rs**:
```rust
//! FeatherCore Common Library
//! 
//! 这是聚合库，只 re-export 子库，本身无功能代码。

#![no_std]

#[cfg(feature = "arm")]
pub use feathercore_arch_arm as arch;

#[cfg(feature = "mmu")]
pub use feathercore_mmu as mmu;

#[cfg(feature = "driver")]
pub use feathercore_driver as driver;

#[cfg(feature = "platform")]
pub use feathercore_platform as platform;
```

### 2. 子库的 src 目录只有 lib.rs

**错误示例** ❌:
```
driver/src/
├── lib.rs      # ❌ 还有其他 .rs 文件
├── gpio.rs
├── spi.rs
└── uart.rs
```

**正确示例** ✅:
```
driver/src/
└── lib.rs      # ✅ 只有 lib.rs
```

### 3. 模块在 src 同级目录下

**错误示例** ❌:
```
driver/src/
├── lib.rs
├── gpio/       # ❌ 模块目录在 src 下
│   └── mod.rs
└── spi/
    └── mod.rs
```

**正确示例** ✅:
```
driver/
├── src/
│   └── lib.rs          # ✅ 只有 lib.rs
├── gpio/               # ✅ 模块目录在 src 同级
│   ├── mod.rs
│   └── ...
└── spi/
    ├── mod.rs
    └── ...
```

## 正确的目录结构

### common (聚合库)

```
common/
├── Cargo.toml              # workspace 定义
├── src/
│   └── lib.rs              # 只 re-export，无模块
├── arch/
│   ├── arm/                # 子库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs      # 只有 lib.rs
│   └── riscv/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── mmu/                    # 子库
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── driver/                 # 子库
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── platform/               # 子库
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### driver (子库示例)

```
driver/
├── Cargo.toml
├── src/
│   └── lib.rs              # 只声明模块，无实现
├── gpio/                   # 模块目录
│   ├── mod.rs              # 模块实现
│   ├── config.rs           # 子模块
│   ├── error.rs
│   └── traits.rs
├── spi/
│   ├── mod.rs
│   ├── config.rs
│   ├── error.rs
│   └── traits.rs
└── uart/
    ├── mod.rs
    └── ...
```

**driver/src/lib.rs**:
```rust
//! FeatherCore Driver Library

#![no_std]

// 声明模块 (模块实现在 src 同级目录)
mod gpio;
mod spi;
mod uart;

// 导出公共 API
pub use gpio::GPIO;
pub use spi::SPI;
pub use uart::UART;
```

**driver/gpio/mod.rs**:
```rust
//! GPIO 驱动模块

mod config;
mod error;
mod traits;

pub use config::Config;
pub use error::Error;
pub use traits::GPIOPin;

pub struct GPIO {
    // ...
}
```

### mmu (子库示例)

```
mmu/
├── Cargo.toml
├── src/
│   └── lib.rs              # 只声明模块
├── types.rs                # 类型定义模块
├── no_mmu.rs               # no_mmu 实现模块
└── arm_mmu.rs              # arm_mmu 实现模块
```

**mmu/src/lib.rs**:
```rust
//! FeatherCore MMU Library

#![no_std]

// 声明模块
mod types;

#[cfg(feature = "no_mmu")]
mod no_mmu;

#[cfg(feature = "with_mmu")]
#[cfg(target_arch = "arm")]
mod arm_mmu;

// 导出公共 API
pub use types::*;

#[cfg(feature = "no_mmu")]
pub use no_mmu::NoMmu;

#[cfg(feature = "with_mmu")]
#[cfg(target_arch = "arm")]
pub use arm_mmu::ArmMmu;
```

## 模块组织规则

### 规则 1: src 下只有 lib.rs

每个子库的 `src/` 目录下**只能有** `lib.rs` 文件。

```rust
// ✅ 正确
common/driver/src/lib.rs

// ❌ 错误
common/driver/src/driver.rs
```

### 规则 2: 模块在 src 同级目录

所有模块都在 `src/` 的**同级目录**下创建。

```rust
// ✅ 正确
common/driver/gpio/mod.rs
common/driver/spi/mod.rs

// ❌ 错误
common/driver/src/gpio/mod.rs
common/driver/src/spi/mod.rs
```

### 规则 3: lib.rs 只声明和导出

`lib.rs` 只负责：
1. 声明模块 (`mod xxx;`)
2. 导出公共 API (`pub use xxx;`)

```rust
// ✅ 正确：只声明和导出
mod gpio;
mod spi;

pub use gpio::GPIO;
pub use spi::SPI;

// ❌ 错误：包含实现
pub struct GPIO {  // 不应该在 lib.rs 中定义
    pin: u8,
}
```

## Cargo.toml 配置

### common/Cargo.toml (聚合库)

```toml
[package]
name = "feathercore-common"
version = "0.1.0"
edition = "2021"

# 注意：没有 [lib] 段，或者只有最简单的配置
# 因为这是聚合库

[dependencies]
# 子库作为依赖
feathercore-arch-arm = { path = "./arch/arm", optional = true }
feathercore-mmu = { path = "./mmu", optional = true }
feathercore-driver = { path = "./driver", optional = true }
feathercore-platform = { path = "./platform", optional = true }

[features]
# 通过 features 选择子库
arm = ["feathercore-arch-arm"]
mmu = ["feathercore-mmu"]
driver = ["feathercore-driver"]
platform = ["feathercore-platform"]
```

### driver/Cargo.toml (子库)

```toml
[package]
name = "feathercore-driver"
version = "0.1.0"
edition = "2021"

[lib]
name = "feathercore_driver"
path = "src/lib.rs"

[features]
# 子库自己的 features
default = []
gpio = []
spi = []
uart = []
```

## 使用示例

### 在 boot 中使用

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

// 通过 common 使用子库
use feathercore_common::driver::uart::UART;
use feathercore_common::mmu::NoMmu;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart = UART::new(0x40011000);
    uart.write(b"Boot starting...\n");
}
```

### 模块实现

```rust
// driver/gpio/mod.rs
mod config;
mod error;
mod traits;

use config::Config;
use error::Error;
use traits::GPIOPin;

pub struct GPIO {
    base: usize,
}

impl GPIO {
    pub fn new(base: usize) -> Self {
        GPIO { base }
    }
    
    pub fn set_high(&self, pin: u8) {
        // 实现
    }
}
```

## 总结

### ✅ 正确结构

```
common/
├── src/lib.rs          # 只 re-export
├── driver/
│   ├── src/lib.rs      # 只声明模块
│   ├── gpio/           # 模块实现
│   └── spi/
└── mmu/
    ├── src/lib.rs
    ├── types.rs
    └── no_mmu.rs
```

### ❌ 错误结构

```
common/
├── src/
│   ├── lib.rs
│   └── driver.rs       # ❌ 不应该有
├── driver/
│   └── src/
│       ├── lib.rs
│       ├── gpio.rs     # ❌ 应该在 src 外
│       └── spi.rs      # ❌ 应该在 src 外
```

### 关键规则

1. **src 下只有 lib.rs**
2. **模块在 src 同级目录**
3. **lib.rs 只声明和导出**
4. **common 只聚合，无功能**

遵循这些规则保证了清晰的模块化和可维护性！
