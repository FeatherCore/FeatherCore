//! I2C Driver Common Layer (Core)
//! I2C 驱动通用核心层
//!
//! This module provides the common I2C driver interface that is independent
//! of specific hardware platforms. It defines the traits and structures
//! that all platform-specific I2C drivers must implement.
//! 该模块提供与硬件平台无关的通用 I2C 驱动接口。它定义了所有平台特定
//! I2C 驱动必须实现的特征和结构。
//!
//! # Architecture / 架构
//! - **Common Core Layer**: Defines abstract interfaces (traits) for I2C operations
//! - **Platform Adapter Layer**: Implements the interfaces for specific hardware (e.g., STM32)
//!
//! # Usage Example / 使用示例
//! ```ignore
//! use driver::i2c::{I2cDriver, I2cConfig, Result};
//!
//! // Initialize I2C with platform-specific driver
//! let mut i2c = I2cDriver::new(stm32_i2c::CONFIG)?;
//! i2c.init(&I2cConfig::default())?;
//!
//! // Write data to a device
//! i2c.write(0x50, &[0x00, 0x01, 0x02])?;
//!
//! // Read data from a device
//! let mut buf = [0u8; 4];
//! i2c.read(0x50, &mut buf)?;
//! ```

pub mod traits;
pub mod config;
pub mod error;

pub use traits::I2cDriver;
pub use config::I2cConfig;
pub use error::I2cError;

pub mod platform {
    //! Platform-specific I2C driver implementations
    //! 平台特定的 I2C 驱动实现

    #[cfg(feature = "stm32f4")]
    pub mod stm32f4 {
        //! STM32F4 Series I2C Driver
        //! STM32F4 系列 I2C 驱动

        pub mod i2c;
    }
}
