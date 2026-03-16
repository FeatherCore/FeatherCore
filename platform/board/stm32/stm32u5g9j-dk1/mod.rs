//! STM32U5G9J-DK1 Board Support
//! 
//! This module provides initialization code for the
//! STM32U5G9J Discovery Kit 1 from STMicroelectronics.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "STM32U5G9J-DK1";
    pub const BOARD_VENDOR: &str = "STMicroelectronics";
}
