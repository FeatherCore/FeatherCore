//! GPIO Driver Error Types
//! GPIO 驱动错误类型
//!
//! This module defines the error types used by the GPIO driver.
//! 该模块定义了 GPIO 驱动使用的错误类型。

/// GPIO driver error types
/// GPIO 驱动错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioError {
    /// Invalid configuration / 无效配置
    InvalidConfig,
    /// Device not initialized / 设备未初始化
    NotInitialized,
    /// Device already initialized / 设备已初始化
    AlreadyInitialized,
    /// Invalid pin number / 无效引脚编号
    InvalidPin,
    /// Invalid port name / 无效端口名称
    InvalidPort,
    /// Invalid mode / 无效模式
    InvalidMode,
    /// Invalid speed / 无效速度
    InvalidSpeed,
    /// Invalid pull mode / 无效上拉/下拉模式
    InvalidPull,
    /// Operation not supported / 操作不支持
    NotSupported,
    /// Hardware error / 硬件错误
    HardwareError,
}

impl core::fmt::Display for GpioError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GpioError::InvalidConfig => write!(f, "Invalid configuration"),
            GpioError::NotInitialized => write!(f, "Device not initialized"),
            GpioError::AlreadyInitialized => write!(f, "Device already initialized"),
            GpioError::InvalidPin => write!(f, "Invalid pin number"),
            GpioError::InvalidPort => write!(f, "Invalid port name"),
            GpioError::InvalidMode => write!(f, "Invalid mode"),
            GpioError::InvalidSpeed => write!(f, "Invalid speed"),
            GpioError::InvalidPull => write!(f, "Invalid pull mode"),
            GpioError::NotSupported => write!(f, "Operation not supported"),
            GpioError::HardwareError => write!(f, "Hardware error"),
        }
    }
}
