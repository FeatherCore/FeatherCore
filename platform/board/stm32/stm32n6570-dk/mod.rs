//! STM32N6570-DK Board Support
//! 
//! This module provides initialization code for the
//! STM32N6570 Discovery Kit from STMicroelectronics.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "STM32N6570-DK";
    pub const BOARD_VENDOR: &str = "STMicroelectronics";
}
