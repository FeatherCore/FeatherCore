//! STM32H7S78-DK Board Support
//! 
//! This module provides initialization code for the
//! STM32H7S78 Discovery Kit from STMicroelectronics.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "STM32H7S78-DK";
    pub const BOARD_VENDOR: &str = "STMicroelectronics";
}
