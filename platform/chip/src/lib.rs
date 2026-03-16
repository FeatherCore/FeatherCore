//! FeatherCore Chip Support Packages
//! 
//! This crate provides chip-specific drivers and initialization code.
//! Each chip is a module within this crate.
//! 
//! # Structure
//! 
//! ```text
//! chip/
//! ├── stm32/              # STM32 chips module
//! │   ├── stm32f4/        # STM32F4 series
//! │   ├── stm32h7rs/      # STM32H7RS series
//! │   └── ...
//! ├── nxp/                # NXP chips module
//! ├── esp/                # ESP chips module
//! └── renesas/            # Renesas chips module
//! ```
//! 
//! # Usage
//! 
//! Enable the chip you need via Cargo features:
//! ```toml
//! [dependencies]
//! feathercore-platform-chip = { path = "path/to/platform/chip", features = ["chip-stm32f4"] }
//! ```
//! 
//! Then use in your code:
//! ```rust
//! use feathercore_platform_chip::stm32::stm32f4;
//! 
//! fn init_chip() {
//!     stm32f4::init();
//! }
//! ```

#![no_std]

// STM32 chips
pub mod stm32 {
    //! STM32 chip modules
    
    #[cfg(feature = "chip-stm32f4")]
    pub mod stm32f4;
    
    #[cfg(feature = "chip-stm32h7rs")]
    pub mod stm32h7rs;
    
    #[cfg(feature = "chip-stm32n6")]
    pub mod stm32n6;
    
    #[cfg(feature = "chip-stm32u5")]
    pub mod stm32u5;
}

// NXP chips
pub mod nxp {
    //! NXP chip modules
    
    #[cfg(feature = "chip-imxrt1170")]
    pub mod imxrt1170;
    
    #[cfg(feature = "chip-rw612")]
    pub mod rw612;
}

// ESP chips
pub mod esp {
    //! ESP chip modules
    
    #[cfg(feature = "chip-esp32c3")]
    pub mod esp32c3;
    
    #[cfg(feature = "chip-esp32c5")]
    pub mod esp32c5;
    
    #[cfg(feature = "chip-esp32c6")]
    pub mod esp32c6;
}

// Renesas chips
pub mod renesas {
    //! Renesas chip modules
    
    #[cfg(feature = "chip-ra8")]
    pub mod ra8;
}
