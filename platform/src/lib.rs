//! FeatherCore Platform Library
//! 
//! This is a meta-crate that re-exports board and chip support packages.
//! It has no actual functionality, just provides convenient access to platform-specific modules.
//! 
//! # Structure
//! 
//! ```text
//! platform/
//! ├── board/          # Board support packages (sub-crate)
//! │   ├── stm32/      # STM32 boards module
//! │   └── nxp/        # NXP boards module
//! └── chip/           # Chip support packages (sub-crate)
//!     ├── stm32/      # STM32 chips module
//!     └── nxp/        # NXP chips module
//! ```
//! 
//! # Usage
//! 
//! Add dependency in your Cargo.toml:
//! ```toml
//! [dependencies]
//! feathercore-platform = { path = "path/to/platform", features = ["board-stm32f429i-disc", "chip-stm32f4"] }
//! ```
//! 
//! Then use in your code:
//! ```rust
//! use feathercore_platform::board::stm32::stm32f429i_disc;
//! use feathercore_platform::chip::stm32::stm32f4;
//! 
//! fn main() {
//!     // Initialize chip
//!     stm32f4::init();
//!     
//!     // Initialize board
//!     stm32f429i_disc::init();
//! }
//! ```

#![no_std]

// Re-export board crate
pub use feathercore_platform_board as board;

// Re-export chip crate
pub use feathercore_platform_chip as chip;
