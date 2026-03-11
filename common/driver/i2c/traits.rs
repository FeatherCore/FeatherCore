//! I2C Driver Traits
//! I2C 驱动特征定义
//!
//! This module defines the traits that all I2C drivers must implement.
//! It provides a common interface for I2C operations independent of
//! the underlying hardware platform.
//! 该模块定义了所有 I2C 驱动必须实现的特征。它提供了与底层硬件平台无关的
//! 通用 I2C 操作接口。

use crate::driver::i2c::{I2cConfig, I2cError};

/// I2C driver trait that must be implemented by all platform-specific drivers
/// I2C 驱动特征，所有平台特定的驱动都必须实现
pub trait I2cDriver {
    /// Initialize the I2C driver with the given configuration
    /// 使用给定配置初始化 I2C 驱动
    ///
    /// # Arguments / 参数
    /// * `config` - I2C configuration / I2C 配置
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Initialization result / 初始化结果
    ///
    /// # Example / 示例
    /// ```ignore
    /// fn init(&mut self, config: &I2cConfig) -> Result<(), I2cError> {
    ///     // Platform-specific initialization
    ///     Ok(())
    /// }
    /// ```
    fn init(&mut self, config: &I2cConfig) -> Result<(), I2cError>;

    /// Write data to an I2C device
    /// 向 I2C 设备写入数据
    ///
    /// # Arguments / 参数
    /// * `addr` - Device address (7-bit or 10-bit) / 设备地址 (7位或10位)
    /// * `data` - Data to write / 要写入的数据
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Write result / 写入结果
    fn write(&mut self, addr: u16, data: &[u8]) -> Result<(), I2cError>;

    /// Read data from an I2C device
    /// 从 I2C 设备读取数据
    ///
    /// # Arguments / 参数
    /// * `addr` - Device address (7-bit or 10-bit) / 设备地址 (7位或10位)
    /// * `buffer` - Buffer to store read data / 存储读取数据的缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, I2cError>` - Number of bytes read / 读取的字节数
    fn read(&mut self, addr: u16, buffer: &mut [u8]) -> Result<usize, I2cError>;

    /// Write and then read data (combined format)
    /// 先写入后读取数据 (组合格式)
    ///
    /// This is commonly used for I2C devices that require a register address
    /// to be specified before reading data.
    /// 这常用于需要在读取数据前指定寄存器地址的 I2C 设备。
    ///
    /// # Arguments / 参数
    /// * `addr` - Device address / 设备地址
    /// * `write_data` - Data to write (typically register address) / 要写入的数据 (通常是寄存器地址)
    /// * `read_buffer` - Buffer to store read data / 存储读取数据的缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, I2cError>` - Number of bytes read / 读取的字节数
    fn write_read(&mut self, addr: u16, write_data: &[u8], read_buffer: &mut [u8]) -> Result<usize, I2cError>;

    /// Check if the I2C bus is busy
    /// 检查 I2C 总线是否忙
    ///
    /// # Returns / 返回
    /// * `Result<bool, I2cError>` - True if bus is busy / 如果总线忙返回 true
    fn is_bus_busy(&mut self) -> Result<bool, I2cError>;

    /// Scan the I2C bus for devices
    /// 扫描 I2C 总线上的设备
    ///
    /// # Returns / 返回
    /// * `Result<Vec<u8>, I2cError>` - List of found device addresses / 找到的设备地址列表
    fn scan(&mut self) -> Result<Vec<u8>, I2cError>;

    /// Enable the I2C peripheral
    /// 使能 I2C 外设
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Enable result / 使能结果
    fn enable(&mut self) -> Result<(), I2cError>;

    /// Disable the I2C peripheral
    /// 禁用 I2C 外设
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Disable result / 禁用结果
    fn disable(&mut self) -> Result<(), I2cError>;
}
