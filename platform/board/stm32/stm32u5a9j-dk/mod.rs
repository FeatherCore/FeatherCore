//! STM32U5A9J-DK Board Support
//! 
//! This module provides initialization code for the
//! STM32U5A9J Discovery Kit from STMicroelectronics.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "STM32U5A9J-DK";
    pub const BOARD_VENDOR: &str = "STMicroelectronics";
}
