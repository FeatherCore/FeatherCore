# FeatherCore Common 库架构规范（修正版）

## 核心原则

### 1. common 是纯聚合库

```
common/
├── Cargo.toml
└── src/
    └── lib.rs          # 只做 re-export，无其他文件
```

### 2. 每个子库的 src 下只有 lib.rs

```
driver/
├── Cargo.toml
├── src/
│   └── lib.rs          # ✅ 只有 lib.rs
├── gpio/               # ✅ 模块目录在 src 同级
│   ├── mod.rs
│   ├── config.rs
│   ├── error.rs
│   └── traits.rs
├── spi/
│   ├── mod.rs
│   └── ...
└── ...
```

### 3. 模块文件在 src 同级目录

**错误示例** ❌:
```
driver/src/
├── lib.rs
├── gpio.rs      # ❌ 不应该在 src 下
├── spi.rs       # ❌ 不应该在 src 下
└── uart.rs      # ❌ 不应该在 src 下
```

**正确示例** ✅:
```
driver/
├── src/
│   └── lib.rs           # ✅ 只有 lib.rs
├── gpio/                # ✅ 模块目录在 src 同级
│   ├── mod.rs
│   ├── config.rs
│   └── error.rs
└── spi/
    └── mod.rs
```

## 完整目录结构

```
common/
├── Cargo.toml                  # 聚合库定义
├── src/
│   └── lib.rs                  # 只 re-export 子库
├── arch/
│   ├── arm/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs          # 只有 lib.rs
│   └── riscv/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── mmu/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
│   ├── types.rs                # 模块文件
│   ├── no_mmu.rs               # 模块文件
│   └── arm_mmu.rs              # 模块文件
├── driver/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs              # 只声明模块
│   ├── gpio/                   # 模块目录
│   │   ├── mod.rs              # 模块声明和导出
│   │   ├── config.rs           # 子模块：配置
│   │   ├── error.rs            # 子模块：错误
│   │   └── traits.rs           # 子模块：特征
│   ├── spi/
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   └── ...
│   ├── i2c/
│   ├── serial/
│   ├── i2s/
│   └── timer/
└── platform/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## 文件内容规范

### common/src/lib.rs

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

### driver/src/lib.rs

```rust
//! FeatherCore Driver Library

#![no_std]

// 声明模块（实现在 src 同级目录）
pub mod gpio;
pub mod i2c;
pub mod serial;
pub mod spi;
pub mod i2s;
pub mod timer;

// 导出公共 API
pub use gpio::{GpioConfig, GpioDriver, GpioError};
pub use i2c::{I2cConfig, I2cDriver, I2cError};
// ...
```

### driver/gpio/mod.rs

```rust
//! GPIO Driver

mod config;
mod error;
mod traits;

pub use config::{GpioConfig, GpioMode, GpioPull, GpioSpeed};
pub use error::GpioError;
pub use traits::GpioDriver;
```

### driver/gpio/config.rs

```rust
//! GPIO configuration types

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GpioMode {
    Input,
    Output,
    #[default]
    AltFunction,
    Analog,
}

// ... 其他配置类型
```

### driver/gpio/error.rs

```rust
//! GPIO error types

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioError {
    InvalidPin,
    InvalidMode,
    NotInitialized,
}

// ... 错误实现
```

## 模块组织规则

### 规则 1: src 下只有 lib.rs

✅ **正确**:
```
driver/src/lib.rs
```

❌ **错误**:
```
driver/src/lib.rs
driver/src/gpio.rs      # 错误！
driver/src/spi.rs       # 错误！
```

### 规则 2: 模块在 src 同级目录

✅ **正确**:
```
driver/
├── src/lib.rs
├── gpio/mod.rs
└── spi/mod.rs
```

❌ **错误**:
```
driver/
├── src/lib.rs
├── src/gpio/mod.rs     # 错误！
└── src/spi/mod.rs      # 错误！
```

### 规则 3: lib.rs 只声明和导出

✅ **正确** (driver/src/lib.rs):
```rust
#![no_std]

// 声明模块
pub mod gpio;
pub mod spi;

// 导出 API
pub use gpio::GPIO;
pub use spi::SPI;
```

❌ **错误**:
```rust
#![no_std]

// 不应该在 lib.rs 中定义结构体
pub struct GPIO {      // 错误！
    pin: u8,
}
```

## 子模块组织

### 扁平结构（简单模块）

```
mmu/
├── src/
│   └── lib.rs
├── types.rs          # 类型定义
├── no_mmu.rs         # no_mmu 实现
└── arm_mmu.rs        # arm_mmu 实现
```

**mmu/src/lib.rs**:
```rust
#![no_std]

mod types;

#[cfg(feature = "no_mmu")]
mod no_mmu;

#[cfg(feature = "with_mmu")]
mod arm_mmu;

pub use types::*;
pub use no_mmu::NoMmu;
pub use arm_mmu::ArmMmu;
```

### 分层结构（复杂模块）

```
driver/gpio/
├── mod.rs            # 模块声明和导出
├── config.rs         # 配置子模块
├── error.rs          # 错误子模块
└── traits.rs         # 特征子模块
```

**driver/gpio/mod.rs**:
```rust
mod config;
mod error;
mod traits;

pub use config::GpioConfig;
pub use error::GpioError;
pub use traits::GpioDriver;
```

## Cargo.toml 配置

### common/Cargo.toml

```toml
[package]
name = "feathercore-common"
version = "0.1.0"
edition = "2021"

[dependencies]
feathercore-arch-arm = { path = "./arch/arm", optional = true }
feathercore-mmu = { path = "./mmu", optional = true }
feathercore-driver = { path = "./driver", optional = true }
feathercore-platform = { path = "./platform", optional = true }

[features]
arm = ["feathercore-arch-arm"]
mmu = ["feathercore-mmu"]
driver = ["feathercore-driver"]
```

### driver/Cargo.toml

```toml
[package]
name = "feathercore-driver"
version = "0.1.0"
edition = "2021"

[lib]
name = "feathercore_driver"
path = "src/lib.rs"

[features]
default = []
gpio = []
spi = []
```

## 使用示例

### 在 boot 中使用

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

use feathercore_common::driver::gpio::{GPIO, GpioMode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut led = GPIO::new(13);
    led.set_mode(GpioMode::Output);
    led.write(true);
}
```

### 在 kernel 中使用

```rust
// kernel/src/driver/uart.rs
#![no_std]

use feathercore_common::driver::serial::{SerialDriver, SerialConfig};

pub struct MyUART {
    base: usize,
}

impl SerialDriver for MyUART {
    fn init(&mut self, config: SerialConfig) -> Result<(), SerialError> {
        // 实现
    }
}
```

## 总结

### ✅ 关键规则

1. **src 下只有 lib.rs** - 所有子库都遵循
2. **模块在 src 同级目录** - 不在 src 子目录
3. **lib.rs 只声明和导出** - 不包含实现
4. **common 只聚合** - 无功能代码

### ✅ 优势

- **清晰的模块边界**: 每个模块在独立目录
- **易于导航**: 目录结构一目了然
- **便于维护**: 模块文件不会拥挤在 src 下
- **符合 Rust 惯例**: 标准的库组织方式

这就是 FeatherCore Common 库的正确架构！
