//! I2S Driver
//! I2S 驱动

mod config;
mod error;
mod traits;

pub use config::I2sConfig;
pub use error::I2sError;
pub use traits::I2sDriver;
