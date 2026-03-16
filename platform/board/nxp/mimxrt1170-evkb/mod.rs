//! NXP i.MX RT1170 EVKB Board Support
//! 
//! This module provides initialization code for the
//! NXP i.MX RT1170 Evaluation Kit.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "MIMXRT1170-EVKB";
    pub const BOARD_VENDOR: &str = "NXP";
}
