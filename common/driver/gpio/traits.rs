//! GPIO driver traits
//! GPIO 驱动特征

use super::{GpioError, GpioMode};

/// GPIO driver trait / GPIO 驱动特征
pub trait GpioDriver {
    fn init(&mut self) -> Result<(), GpioError>;
    fn set_mode(&mut self, pin: u8, mode: GpioMode) -> Result<(), GpioError>;
    fn write(&mut self, pin: u8, value: bool) -> Result<(), GpioError>;
    fn read(&self, pin: u8) -> Result<bool, GpioError>;
}
