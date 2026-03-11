//! I2C Driver Error Types
//! I2C 驱动错误类型
//!
//! This module defines the error types used by the I2C driver.
//! 该模块定义了 I2C 驱动使用的错误类型。

/// I2C driver error types
/// I2C 驱动错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cError {
    /// Invalid configuration / 无效配置
    InvalidConfig,
    /// Device not initialized / 设备未初始化
    NotInitialized,
    /// Device already initialized / 设备已初始化
    AlreadyInitialized,
    /// Bus busy / 总线忙
    Busy,
    /// Arbitration lost / 仲裁丢失
    ArbitrationLost,
    /// NACK received / 收到 NACK
    NackReceived,
    /// Timeout / 超时
    Timeout,
    /// Hardware error / 硬件错误
    HardwareError,
    /// Invalid speed / 无效速度
    InvalidSpeed,
    /// Invalid address / 无效地址
    InvalidAddress,
    /// Feature not supported / 功能不支持
    NotSupported,
    /// DMA error / DMA 错误
    DmaError,
}

impl core::fmt::Display for I2cError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            I2cError::InvalidConfig => write!(f, "Invalid configuration"),
            I2cError::NotInitialized => write!(f, "Device not initialized"),
            I2cError::AlreadyInitialized => write!(f, "Device already initialized"),
            I2cError::Busy => write!(f, "Bus busy"),
            I2cError::ArbitrationLost => write!(f, "Arbitration lost"),
            I2cError::NackReceived => write!(f, "NACK received"),
            I2cError::Timeout => write!(f, "Timeout"),
            I2cError::HardwareError => write!(f, "Hardware error"),
            I2cError::InvalidSpeed => write!(f, "Invalid speed"),
            I2cError::InvalidAddress => write!(f, "Invalid address"),
            I2cError::NotSupported => write!(f, "Feature not supported"),
            I2cError::DmaError => write!(f, "DMA error"),
        }
    }
}
