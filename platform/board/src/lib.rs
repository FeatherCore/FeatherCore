//! FeatherCore Board Support Packages
//! 
//! This crate provides board-specific initialization code.
//! Each board is a module within this crate.
//! 
//! # Structure
//! 
//! ```text
//! board/
//! ├── stm32/              # STM32 boards module
//! │   ├── stm32f429i-disc/   # STM32F429I-DISCO board
//! │   ├── stm32h7s78-dk/     # STM32H7S78-DK board
//! │   └── ...
//! ├── nxp/                # NXP boards module
//! ├── esp/                # ESP boards module
//! └── renesas/            # Renesas boards module
//! ```
//! 
//! # Usage
//! 
//! Enable the board you need via Cargo features:
//! ```toml
//! [dependencies]
//! feathercore-platform-board = { path = "path/to/platform/board", features = ["stm32f429i-disc"] }
//! ```
//! 
//! Then use in your code:
//! ```rust
//! use feathercore_platform_board::stm32::stm32f429i_disc;
//! 
//! fn main() {
//!     stm32f429i_disc::init();
//! }
//! ```

#![no_std]

// STM32 boards
pub mod stm32 {
    //! STM32 board modules
    
    #[cfg(feature = "stm32f429i-disc")]
    pub mod stm32f429i_disc;
    
    #[cfg(feature = "stm32h7s78-dk")]
    pub mod stm32h7s78_dk;
    
    #[cfg(feature = "stm32n6570-dk")]
    pub mod stm32n6570_dk;
    
    #[cfg(feature = "stm32u5a9j-dk")]
    pub mod stm32u5a9j_dk;
    
    #[cfg(feature = "stm32u5g9j-dk1")]
    pub mod stm32u5g9j_dk1;
}

// NXP boards
pub mod nxp {
    //! NXP board modules
    
    #[cfg(feature = "mimxrt1170-evkb")]
    pub mod mimxrt1170_evkb;
    
    #[cfg(feature = "frdm-rw612")]
    pub mod frdm_rw612;
}

// ESP boards
pub mod esp {
    //! ESP board modules
    
    #[cfg(feature = "esp32-c3-devkitc")]
    pub mod esp32_c3_devkitc;
    
    #[cfg(feature = "esp32-c5-devkitc")]
    pub mod esp32_c5_devkitc;
    
    #[cfg(feature = "esp32-c6-devkitm")]
    pub mod esp32_c6_devkitm;
}

// Renesas boards
pub mod renesas {
    //! Renesas board modules
    
    #[cfg(feature = "ek-ra8p1")]
    pub mod ek_ra8p1;
}
