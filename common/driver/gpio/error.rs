//! GPIO error types
//! GPIO 错误类型

/// GPIO error types / GPIO 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioError {
    InvalidPin,
    InvalidMode,
    NotInitialized,
    AlreadyInitialized,
}

impl core::fmt::Display for GpioError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GpioError::InvalidPin => write!(f, "Invalid pin"),
            GpioError::InvalidMode => write!(f, "Invalid mode"),
            GpioError::NotInitialized => write!(f, "Not initialized"),
            GpioError::AlreadyInitialized => write!(f, "Already initialized"),
        }
    }
}
