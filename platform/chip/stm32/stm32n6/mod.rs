//! STM32N6 Chip Support
//! 
//! This module provides drivers and initialization for
//! STM32N6 series microcontrollers.

#![no_std]

/// Initialize chip hardware
pub fn init() {
    // TODO: Initialize chip hardware
}

/// Chip information
pub mod info {
    pub const CHIP_FAMILY: &str = "STM32N6";
    pub const CHIP_VENDOR: &str = "STMicroelectronics";
    pub const CPU_CORE: &str = "Cortex-M33";
}
