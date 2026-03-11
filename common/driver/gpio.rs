//! GPIO Driver Common Layer (Core)
//! GPIO 驱动通用核心层
//!
//! This module provides the common GPIO driver interface that is independent
//! of specific hardware platforms. It defines the traits and structures
//! that all platform-specific GPIO drivers must implement.
//! 该模块提供与硬件平台无关的通用 GPIO 驱动接口。它定义了所有平台特定
//! GPIO 驱动必须实现的特征和结构。
//!
//! # Architecture / 架构
//! - **Common Core Layer**: Defines abstract interfaces (traits) for GPIO operations
//! - **Platform Adapter Layer**: Implements the interfaces for specific hardware (e.g., STM32)
//!
//! # Usage Example / 使用示例
//! ```ignore
//! use driver::gpio::{GpioDriver, GpioConfig, GpioPin, GpioMode, Result};
//!
//! // Initialize GPIO with platform-specific driver
//! let mut gpio = GpioDriver::new(stm32_gpio::CONFIG)?;
//! gpio.init()?;
//!
//! // Configure pin as output
//! let mut pin = gpio.get_pin(13)?;
//! pin.set_mode(GpioMode::Output)?;
//! 
//! // Set pin high
//! pin.set_high()?;
//!
//! // Read pin state
//! let state = pin.read()?;
//! ```

pub mod traits;
pub mod config;
pub mod error;

pub use traits::{GpioDriver, GpioPin};
pub use config::{GpioConfig, GpioMode, GpioSpeed, GpioPull};
pub use error::GpioError;

pub mod platform {
    //! Platform-specific GPIO driver implementations
    //! 平台特定的 GPIO 驱动实现

    #[cfg(feature = "stm32f4")]
    pub mod stm32f4 {
        //! STM32F4 Series GPIO Driver
        //! STM32F4 系列 GPIO 驱动

        pub mod gpio;
    }
}
