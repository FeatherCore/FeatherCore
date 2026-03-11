//! Serial Driver Common Layer (Core)
//! 串口驱动通用核心层
//!
//! This module provides the common serial driver of specific is independent
//! hardware platforms. It defines the interface that traits and structures
//! that all platform-specific serial drivers must implement.
//! 该模块提供与硬件平台无关的通用串口驱动接口。它定义了所有平台特定
//! 串口驱动必须实现的特征和结构。
//!
//! # Architecture / 架构
//! - **Common Core Layer**: Defines abstract interfaces (traits) for serial operations
//! - **Platform Adapter Layer**: Implements the interfaces for specific hardware (e.g., STM32)
//!
//! # Usage Example / 使用示例
//! ```ignore
//! use driver::serial::{Serial, SerialConfig, Result};
//!
//! // Initialize serial with platform-specific driver
//! let mut serial = Serial::new(stm32_usart::CONFIG)?;
//!
//! // Write data
//! serial.write(b"Hello, World!")?;
//!
//! // Read data
//! let mut buf = [0u8; 64];
//! let len = serial.read(&mut buf)?;
//! ```

pub mod traits;
pub mod config;
pub mod error;

pub use traits::SerialDriver;
pub use config::SerialConfig;
pub use error::SerialError;

pub mod platform {
    //! Platform-specific serial driver implementations
    //! 平台特定的串口驱动实现

    #[cfg(feature = "stm32f4")]
    pub mod stm32f4 {
        //! STM32F4 Series Serial Driver
        //! STM32F4 系列串口驱动

        pub mod usart;
    }
}
