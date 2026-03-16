//! FeatherCore Platform Adapter Layer
//! 
//! This module provides an adapter layer that selects and re-exports
//! the appropriate board and chip support packages based on compile-time features.
//! 
//! # Architecture
//! 
//! ```text
//! boot/kernel
//!     ↓ (depends on features)
//! common/platform (this crate)
//!     ↓ (re-exports)
//! platform/board, platform/chip
//!     ↓ (uses)
//! common/driver, common/sys
//! ```
//! 
//! # Usage
//! 
//! In boot/Cargo.toml or kernel/Cargo.toml:
//! ```toml
//! [dependencies]
//! feathercore-common = { path = "../common", features = ["platform"] }
//! feathercore-platform = { path = "../common/platform", features = ["stm32f429i-disc"] }
//! ```
//! 
//! In code:
//! ```rust
//! use feathercore_platform as platform;
//! 
//! fn main() {
//!     // Initialize platform (calls board and chip init)
//!     platform::init();
//!     
//!     // Access board information
//!     let board_name = platform::board_info();
//!     
//!     // Use chip-specific functions
//!     platform::chip_init();
//! }
//! ```
//! 
//! # Features
//! 
//! ## STM32 Boards
//! - `stm32f429i-disc` - STM32F429I Discovery (includes chip-stm32f4)
//! - `stm32h7s78-dk` - STM32H7S78 Discovery Kit (includes chip-stm32h7rs)
//! - `stm32n6570-dk` - STM32N6570 Discovery Kit (includes chip-stm32n6)
//! - `stm32u5a9j-dk` - STM32U5A9J Discovery Kit (includes chip-stm32u5)
//! - `stm32u5g9j-dk1` - STM32U5G9J Discovery Kit 1 (includes chip-stm32u5)
//! 
//! ## NXP Boards
//! - `mimxrt1170-evkb` - i.MX RT1170 EVKB (includes chip-imxrt1170)
//! - `frdm-rw612` - Freedom Kit RW612 (includes chip-rw612)
//! 
//! ## ESP Boards
//! - `esp32-c3-devkitc` - ESP32-C3 DevKit (includes chip-esp32c3)
//! - `esp32-c5-devkitc` - ESP32-C5 DevKit (includes chip-esp32c5)
//! - `esp32-c6-devkitm` - ESP32-C6 DevKit (includes chip-esp32c6)
//! 
//! ## Renesas Boards
//! - `ek-ra8p1` - EK-RA8P1 Evaluation Kit (includes chip-ra8)
//! 
//! ## Chip-only Features
//! - `chip-stm32f4`, `chip-stm32h7rs`, `chip-stm32n6`, `chip-stm32u5`
//! - `chip-imxrt1170`, `chip-rw612`
//! - `chip-esp32c3`, `chip-esp32c5`, `chip-esp32c6`
//! - `chip-ra8`

#![no_std]

// Re-export board modules
#[cfg(feature = "stm32f429i-disc")]
pub use feathercore_platform_board::stm32::stm32f429i_disc as board;

#[cfg(feature = "stm32h7s78-dk")]
pub use feathercore_platform_board::stm32::stm32h7s78_dk as board;

#[cfg(feature = "stm32n6570-dk")]
pub use feathercore_platform_board::stm32::stm32n6570_dk as board;

#[cfg(feature = "stm32u5a9j-dk")]
pub use feathercore_platform_board::stm32::stm32u5a9j_dk as board;

#[cfg(feature = "stm32u5g9j-dk1")]
pub use feathercore_platform_board::stm32::stm32u5g9j_dk1 as board;

#[cfg(feature = "mimxrt1170-evkb")]
pub use feathercore_platform_board::nxp::mimxrt1170_evkb as board;

#[cfg(feature = "frdm-rw612")]
pub use feathercore_platform_board::nxp::frdm_rw612 as board;

#[cfg(feature = "esp32-c3-devkitc")]
pub use feathercore_platform_board::esp::esp32_c3_devkitc as board;

#[cfg(feature = "esp32-c5-devkitc")]
pub use feathercore_platform_board::esp::esp32_c5_devkitc as board;

#[cfg(feature = "esp32-c6-devkitm")]
pub use feathercore_platform_board::esp::esp32_c6_devkitm as board;

#[cfg(feature = "ek-ra8p1")]
pub use feathercore_platform_board::renesas::ek_ra8p1 as board;

// Re-export chip modules
#[cfg(feature = "stm32f429i-disc")]
pub use feathercore_platform_chip::stm32::stm32f4 as chip;

#[cfg(feature = "stm32h7s78-dk")]
pub use feathercore_platform_chip::stm32::stm32h7rs as chip;

#[cfg(feature = "stm32n6570-dk")]
pub use feathercore_platform_chip::stm32::stm32n6 as chip;

#[cfg(feature = "stm32u5a9j-dk")]
pub use feathercore_platform_chip::stm32::stm32u5 as chip;

#[cfg(feature = "stm32u5g9j-dk1")]
pub use feathercore_platform_chip::stm32::stm32u5 as chip;

#[cfg(feature = "mimxrt1170-evkb")]
pub use feathercore_platform_chip::nxp::imxrt1170 as chip;

#[cfg(feature = "frdm-rw612")]
pub use feathercore_platform_chip::nxp::rw612 as chip;

#[cfg(feature = "esp32-c3-devkitc")]
pub use feathercore_platform_chip::esp::esp32c3 as chip;

#[cfg(feature = "esp32-c5-devkitc")]
pub use feathercore_platform_chip::esp::esp32c5 as chip;

#[cfg(feature = "esp32-c6-devkitm")]
pub use feathercore_platform_chip::esp::esp32c6 as chip;

#[cfg(feature = "ek-ra8p1")]
pub use feathercore_platform_chip::renesas::ra8 as chip;

// Chip-only features
#[cfg(all(feature = "chip-stm32f4", not(feature = "stm32f429i-disc")))]
pub use feathercore_platform_chip::stm32::stm32f4 as chip;

#[cfg(all(feature = "chip-stm32h7rs", not(feature = "stm32h7s78-dk")))]
pub use feathercore_platform_chip::stm32::stm32h7rs as chip;

#[cfg(all(feature = "chip-stm32n6", not(feature = "stm32n6570-dk")))]
pub use feathercore_platform_chip::stm32::stm32n6 as chip;

#[cfg(all(feature = "chip-stm32u5", not(any(feature = "stm32u5a9j-dk", feature = "stm32u5g9j-dk1"))))]
pub use feathercore_platform_chip::stm32::stm32u5 as chip;

#[cfg(all(feature = "chip-imxrt1170", not(feature = "mimxrt1170-evkb")))]
pub use feathercore_platform_chip::nxp::imxrt1170 as chip;

#[cfg(all(feature = "chip-rw612", not(feature = "frdm-rw612")))]
pub use feathercore_platform_chip::nxp::rw612 as chip;

#[cfg(all(feature = "chip-esp32c3", not(feature = "esp32-c3-devkitc")))]
pub use feathercore_platform_chip::esp::esp32c3 as chip;

#[cfg(all(feature = "chip-esp32c5", not(feature = "esp32-c5-devkitc")))]
pub use feathercore_platform_chip::esp::esp32c5 as chip;

#[cfg(all(feature = "chip-esp32c6", not(feature = "esp32-c6-devkitm")))]
pub use feathercore_platform_chip::esp::esp32c6 as chip;

#[cfg(all(feature = "chip-ra8", not(feature = "ek-ra8p1")))]
pub use feathercore_platform_chip::renesas::ra8 as chip;

/// Initialize the platform
/// 
/// This function calls the board's init function, which in turn
/// calls the chip's init function.
#[inline]
pub fn init() {
    #[cfg(feature = "board")]
    board::init();
}

/// Initialize chip hardware
#[inline]
pub fn chip_init() {
    #[cfg(feature = "chip")]
    chip::init();
}

/// Get board information
#[inline]
pub fn board_info() -> BoardInfo {
    BoardInfo {
        #[cfg(feature = "board")]
        name: board::info::BOARD_NAME,
        
        #[cfg(not(feature = "board"))]
        name: "Unknown",
        
        #[cfg(feature = "board")]
        vendor: board::info::BOARD_VENDOR,
        
        #[cfg(not(feature = "board"))]
        vendor: "Unknown",
    }
}

/// Board information structure
pub struct BoardInfo {
    pub name: &'static str,
    pub vendor: &'static str,
}
