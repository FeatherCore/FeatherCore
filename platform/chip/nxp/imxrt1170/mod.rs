//! NXP i.MX RT1170 Chip Support
//! 
//! This module provides drivers and initialization for
//! NXP i.MX RT1170 crossover processor.

#![no_std]

/// Initialize chip hardware
pub fn init() {
    // TODO: Initialize chip hardware
}

/// Chip information
pub mod info {
    pub const CHIP_FAMILY: &str = "IMXRT1170";
    pub const CHIP_VENDOR: &str = "NXP";
    pub const CPU_CORE: &str = "Cortex-M7 + Cortex-M4";
}
