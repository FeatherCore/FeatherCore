//! NXP FRDM-RW612 Board Support
//! 
//! This module provides initialization code for the
//! NXP Freedom Kit for RW612.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "FRDM-RW612";
    pub const BOARD_VENDOR: &str = "NXP";
}
