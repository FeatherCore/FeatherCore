//! GPIO Driver Traits
//! GPIO 驱动特征定义
//!
//! This module defines the traits that all GPIO drivers must implement.
//! It provides a common interface for GPIO operations independent of
//! the underlying hardware platform.
//! 该模块定义了所有 GPIO 驱动必须实现的特征。它提供了与底层硬件平台无关的
//! 通用 GPIO 操作接口。

use crate::driver::gpio::{GpioConfig, GpioError, GpioMode, GpioPin};

/// GPIO driver trait that must be implemented by all platform-specific drivers
/// GPIO 驱动特征，所有平台特定的驱动都必须实现
pub trait GpioDriver {
    /// Initialize the GPIO driver
    /// 初始化 GPIO 驱动
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Initialization result / 初始化结果
    fn init(&mut self) -> Result<(), GpioError>;

    /// Get a GPIO pin by pin number
    /// 通过引脚编号获取 GPIO 引脚
    ///
    /// # Arguments / 参数
    /// * `pin_number` - Pin number / 引脚编号
    ///
    /// # Returns / 返回
    /// * `Result<Box<dyn GpioPin>, GpioError>` - GPIO pin or error / GPIO 引脚或错误
    fn get_pin(&mut self, pin_number: u16) -> Result<Box<dyn GpioPin>, GpioError>;

    /// Get a GPIO pin by port and pin number
    /// 通过端口和引脚编号获取 GPIO 引脚
    ///
    /// # Arguments / 参数
    /// * `port` - Port name (e.g., "A", "B", "C") / 端口名称（如 "A"、"B"、"C"）
    /// * `pin` - Pin number within the port / 端口内的引脚编号
    ///
    /// # Returns / 返回
    /// * `Result<Box<dyn GpioPin>, GpioError>` - GPIO pin or error / GPIO 引脚或错误
    fn get_pin_by_port(&mut self, port: &str, pin: u8) -> Result<Box<dyn GpioPin>, GpioError>;

    /// Enable the GPIO peripheral
    /// 使能 GPIO 外设
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Enable result / 使能结果
    fn enable(&mut self) -> Result<(), GpioError>;

    /// Disable the GPIO peripheral
    /// 禁用 GPIO 外设
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Disable result / 禁用结果
    fn disable(&mut self) -> Result<(), GpioError>;
}

/// GPIO pin trait that must be implemented by all platform-specific pin implementations
/// GPIO 引脚特征，所有平台特定的引脚实现都必须实现
pub trait GpioPin {
    /// Set the pin mode
    /// 设置引脚模式
    ///
    /// # Arguments / 参数
    /// * `mode` - GPIO mode / GPIO 模式
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Set result / 设置结果
    fn set_mode(&mut self, mode: GpioMode) -> Result<(), GpioError>;

    /// Set the pin speed
    /// 设置引脚速度
    ///
    /// # Arguments / 参数
    /// * `speed` - GPIO speed / GPIO 速度
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Set result / 设置结果
    fn set_speed(&mut self, speed: crate::driver::gpio::GpioSpeed) -> Result<(), GpioError>;

    /// Set the pin pull mode
    /// 设置引脚上拉/下拉模式
    ///
    /// # Arguments / 参数
    /// * `pull` - GPIO pull mode / GPIO 上拉/下拉模式
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Set result / 设置结果
    fn set_pull(&mut self, pull: crate::driver::gpio::GpioPull) -> Result<(), GpioError>;

    /// Set the pin to high
    /// 设置引脚为高电平
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Set result / 设置结果
    fn set_high(&mut self) -> Result<(), GpioError>;

    /// Set the pin to low
    /// 设置引脚为低电平
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Set result / 设置结果
    fn set_low(&mut self) -> Result<(), GpioError>;

    /// Toggle the pin state
    /// 切换引脚状态
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Toggle result / 切换结果
    fn toggle(&mut self) -> Result<(), GpioError>;

    /// Read the pin state
    /// 读取引脚状态
    ///
    /// # Returns / 返回
    /// * `Result<bool, GpioError>` - Pin state (true for high, false for low) / 引脚状态（高电平为 true，低电平为 false）
    fn read(&self) -> Result<bool, GpioError>;

    /// Get the pin number
    /// 获取引脚编号
    ///
    /// # Returns / 返回
    /// * `u16` - Pin number / 引脚编号
    fn pin_number(&self) -> u16;

    /// Get the port name
    /// 获取端口名称
    ///
    /// # Returns / 返回
    /// * `&str` - Port name / 端口名称
    fn port(&self) -> &str;
}
