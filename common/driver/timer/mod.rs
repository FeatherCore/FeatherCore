//! Timer Driver
//! 定时器驱动

mod config;
mod error;
mod traits;

pub use config::TimerConfig;
pub use error::TimerError;
pub use traits::TimerDriver;
