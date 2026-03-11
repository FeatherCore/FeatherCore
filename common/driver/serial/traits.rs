//! Serial Driver Traits
//! 串口驱动特征定义
//!
//! This module defines the traits that all serial drivers must implement.
//! It provides a common interface for serial operations independent of
//! the underlying hardware platform.
//! 该模块定义了所有串口驱动必须实现的特征。它提供了与底层硬件平台无关的
//! 通用串口操作接口。

use crate::driver::serial::{SerialConfig, SerialError};

/// Serial driver trait that must be implemented by all platform-specific drivers
/// 串口驱动特征，所有平台特定的驱动都必须实现
pub trait SerialDriver {
    /// Initialize the serial driver with the given configuration
    /// 使用给定配置初始化串口驱动
    ///
    /// # Arguments / 参数
    /// * `config` - Serial configuration / 串口配置
    ///
    /// # Returns / 返回
    /// * `Result<(), SerialError>` - Initialization result / 初始化结果
    ///
    /// # Example / 示例
    /// ```ignore
    /// fn init(&mut self, config: &SerialConfig) -> Result<(), SerialError> {
    ///     // Platform-specific initialization
    ///     Ok(())
    /// }
    /// ```
    fn init(&mut self, config: &SerialConfig) -> Result<(), SerialError>;

    /// Write data to the serial port
    /// 向串口写入数据
    ///
    /// # Arguments / 参数
    /// * `data` - Data to write / 要写入的数据
    ///
    /// # Returns / 返回
    /// * `Result<usize, SerialError>` - Number of bytes written / 写入的字节数
    ///
    /// # Example / 示例
    /// ```ignore
    /// fn write(&mut self, data: &[u8]) -> Result<usize, SerialError> {
    ///     // Platform-specific write implementation
    ///     Ok(data.len())
    /// }
    /// ```
    fn write(&mut self, data: &[u8]) -> Result<usize, SerialError>;

    /// Read data from the serial port
    /// 从串口读取数据
    ///
    /// # Arguments / 参数
    /// * `buffer` - Buffer to store read data / 存储读取数据的缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<usize, SerialError>` - Number of bytes read / 读取的字节数
    ///
    /// # Example / 示例
    /// ```ignore
    /// fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SerialError> {
    ///     // Platform-specific read implementation
    ///     Ok(0)
    /// }
    /// ```
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SerialError>;

    /// Check if there is data available to read
    /// 检查是否有数据可读
    ///
    /// # Returns / 返回
    /// * `Result<bool, SerialError>` - True if data is available / 如果有数据可读返回 true
    fn is_data_available(&mut self) -> Result<bool, SerialError>;

    /// Flush the transmit buffer
    /// 刷新发送缓冲区
    ///
    /// # Returns / 返回
    /// * `Result<(), SerialError>` - Flush result / 刷新结果
    fn flush(&mut self) -> Result<(), SerialError>;

    /// Enable the serial port
    /// 使能串口
    ///
    /// # Returns / 返回
    /// * `Result<(), SerialError>` - Enable result / 使能结果
    fn enable(&mut self) -> Result<(), SerialError>;

    /// Disable the serial port
    /// 禁用串口
    ///
    /// # Returns / 返回
    /// * `Result<(), SerialError>` - Disable result / 禁用结果
    fn disable(&mut self) -> Result<(), SerialError>;
}
