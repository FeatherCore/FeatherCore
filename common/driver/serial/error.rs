//! Serial Driver Error Types
//! 串口驱动错误类型
//!
//! This module defines the error types used by the serial driver.
//! 该模块定义了串口驱动使用的错误类型。

/// Serial driver error types
/// 串口驱动错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerialError {
    /// Invalid configuration / 无效配置
    InvalidConfig,
    /// Device not initialized / 设备未初始化
    NotInitialized,
    /// Device already initialized / 设备已初始化
    AlreadyInitialized,
    /// Write error / 写入错误
    WriteError,
    /// Read error / 读取错误
    ReadError,
    /// Buffer overflow / 缓冲区溢出
    BufferOverflow,
    /// Timeout / 超时
    Timeout,
    /// Hardware error / 硬件错误
    HardwareError,
    /// Invalid baud rate / 无效波特率
    InvalidBaudRate,
    /// Feature not supported / 功能不支持
    NotSupported,
    /// Device busy / 设备忙
    Busy,
    /// DMA error / DMA 错误
    DmaError,
}

impl core::fmt::Display for SerialError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerialError::InvalidConfig => write!(f, "Invalid configuration"),
            SerialError::NotInitialized => write!(f, "Device not initialized"),
            SerialError::AlreadyInitialized => write!(f, "Device already initialized"),
            SerialError::WriteError => write!(f, "Write error"),
            SerialError::ReadError => write!(f, "Read error"),
            SerialError::BufferOverflow => write!(f, "Buffer overflow"),
            SerialError::Timeout => write!(f, "Timeout"),
            SerialError::HardwareError => write!(f, "Hardware error"),
            SerialError::InvalidBaudRate => write!(f, "Invalid baud rate"),
            SerialError::NotSupported => write!(f, "Feature not supported"),
            SerialError::Busy => write!(f, "Device busy"),
            SerialError::DmaError => write!(f, "DMA error"),
        }
    }
}
