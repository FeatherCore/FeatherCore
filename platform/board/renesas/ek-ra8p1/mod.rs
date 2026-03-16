//! Renesas EK-RA8P1 Board Support
//! 
//! This module provides initialization code for the
//! Renesas EK-RA8P1 Evaluation Kit.

#![no_std]

/// Initialize board hardware
pub fn init() {
    // TODO: Initialize board hardware
}

/// Board information
pub mod info {
    pub const BOARD_NAME: &str = "EK-RA8P1";
    pub const BOARD_VENDOR: &str = "Renesas";
}
