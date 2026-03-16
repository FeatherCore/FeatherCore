//! ESP32-C6-DevKitM Board Support
//! 
//! This module provides initialization code for the
//! Espressif ESP32-C6 Development Kit.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "ESP32-C6-DEVKITM";
    pub const BOARD_VENDOR: &str = "Espressif";
}
