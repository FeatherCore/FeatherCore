//! ESP32-C5-DevKitC Board Support
//! 
//! This module provides initialization code for the
//! Espressif ESP32-C5 Development Kit.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "ESP32-C5-DEVKITC";
    pub const BOARD_VENDOR: &str = "Espressif";
}
