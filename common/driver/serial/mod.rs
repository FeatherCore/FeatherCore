//! Serial Driver
//! 串口驱动

mod config;
mod error;
mod traits;

pub use config::SerialConfig;
pub use error::SerialError;
pub use traits::SerialDriver;
