//! ESP32-C3 Chip Support
//! 
//! This module provides drivers and initialization for
//! Espressif ESP32-C3 Wi-Fi MCU.

#![no_std]

/// Initialize chip hardware
pub fn init() {
    // TODO: Initialize chip hardware
}

/// Chip information
pub mod info {
    pub const CHIP_FAMILY: &str = "ESP32C3";
    pub const CHIP_VENDOR: &str = "Espressif";
    pub const CPU_CORE: &str = "RISC-V 32-bit";
}
