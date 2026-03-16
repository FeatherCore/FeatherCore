//! I2C Driver
//! I2C 驱动

mod config;
mod error;
mod traits;

pub use config::I2cConfig;
pub use error::I2cError;
pub use traits::I2cDriver;
