//! SPI Driver
//! SPI 驱动

mod config;
mod error;
mod traits;

pub use config::SpiConfig;
pub use error::SpiError;
pub use traits::SpiDriver;
