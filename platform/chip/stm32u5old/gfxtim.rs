//! GFXTIM - Graphics Timer
//! 图形定时器
//!
//! # Overview / 概述
//! STM32U5 Graphics Timer (GFXTIM) provides timing capabilities specifically
//! designed for graphics and display applications, supporting synchronization
//! with LCD-TFT display controllers.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 59: Graphics timer (GFXTIM)
//!
//! ## Main Features / 主要特性
//! - 16-bit auto-reload counter
//! - Programmable prescaler
//! - Multiple capture/compare channels
//! - Synchronization with LTDC
//! - Interrupt generation on various events
//! - PWM generation capabilities
//!
//! # Reference / 参考
//! - RM0456 Chapter 59: Graphics timer (GFXTIM)
//!   - Register map: RM0456, Section 59.4, pages 2421-2450
//!   - GFXTIM Control Register 1 (GFXTIM_CR1): RM0456, Section 59.4.1, page 2422
//!   - GFXTIM Status Register (GFXTIM_SR): RM0456, Section 59.4.5, page 2428

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// GFXTIM base address
pub const GFXTIM_BASE: usize = 0x5000_0000;

/// GFXTIM register offsets
pub mod reg {
    /// Control Register 1
    pub const CR1: usize = 0x00;
    /// Control Register 2
    pub const CR2: usize = 0x04;
    /// Slave Mode Control Register
    pub const SMCR: usize = 0x08;
    /// DMA/Interrupt Enable Register
    pub const DIER: usize = 0x0C;
    /// Status Register
    pub const SR: usize = 0x10;
    /// Event Generation Register
    pub const EGR: usize = 0x14;
    /// Capture/Compare Mode Register 1
    pub const CCMR1: usize = 0x18;
    /// Capture/Compare Mode Register 2
    pub const CCMR2: usize = 0x1C;
    /// Capture/Compare Enable Register
    pub const CCER: usize = 0x20;
    /// Counter
    pub const CNT: usize = 0x24;
    /// Prescaler
    pub const PSC: usize = 0x28;
    /// Auto-reload Register
    pub const ARR: usize = 0x2C;
    /// Repetition Counter Register
    pub const RCR: usize = 0x30;
    /// Capture/Compare Register 1
    pub const CCR1: usize = 0x34;
    /// Capture/Compare Register 2
    pub const CCR2: usize = 0x38;
    /// Capture/Compare Register 3
    pub const CCR3: usize = 0x3C;
    /// Capture/Compare Register 4
    pub const CCR4: usize = 0x40;
    /// Break and Dead-time Register
    pub const BDTR: usize = 0x44;
    /// DMA Control Register
    pub const DCR: usize = 0x48;
    /// DMA Address Register
    pub const DMAR: usize = 0x4C;
    /// Option Register
    pub const OR: usize = 0x50;
}

/// GFXTIM register bit definitions
pub mod bits {
    /// CR1 Register Bits
    pub mod cr1 {
        /// Counter Enable
        pub const CEN: u32 = 1 << 0;
        /// Update Disable
        pub const UDIS: u32 = 1 << 1;
        /// Update Request Source
        pub const URS: u32 = 1 << 2;
        /// One Pulse Mode
        pub const OPM: u32 = 1 << 3;
        /// Direction
        pub const DIR: u32 = 1 << 4;
        /// Center-aligned Mode
        pub const CMS: u32 = 0b11 << 5;
        /// Auto-reload Preload Enable
        pub const ARPE: u32 = 1 << 7;
        /// Clock Division
        pub const CKD: u32 = 0b11 << 8;
    }

    /// SR Register Bits
    pub mod sr {
        /// Update Interrupt Flag
        pub const UIF: u32 = 1 << 0;
        /// Capture/Compare 1 Interrupt Flag
        pub const CC1IF: u32 = 1 << 1;
        /// Capture/Compare 2 Interrupt Flag
        pub const CC2IF: u32 = 1 << 2;
        /// Capture/Compare 3 Interrupt Flag
        pub const CC3IF: u32 = 1 << 3;
        /// Capture/Compare 4 Interrupt Flag
        pub const CC4IF: u32 = 1 << 4;
        /// COM Interrupt Flag
        pub const COMIF: u32 = 1 << 5;
        /// Trigger Interrupt Flag
        pub const TIF: u32 = 1 << 6;
        /// Break Interrupt Flag
        pub const BIF: u32 = 1 << 7;
    }

    /// DIER Register Bits
    pub mod dier {
        /// Update Interrupt Enable
        pub const UIE: u32 = 1 << 0;
        /// Capture/Compare 1 Interrupt Enable
        pub const CC1IE: u32 = 1 << 1;
        /// Capture/Compare 2 Interrupt Enable
        pub const CC2IE: u32 = 1 << 2;
        /// Capture/Compare 3 Interrupt Enable
        pub const CC3IE: u32 = 1 << 3;
        /// Capture/Compare 4 Interrupt Enable
        pub const CC4IE: u32 = 1 << 4;
        /// COM Interrupt Enable
        pub const COMIE: u32 = 1 << 5;
        /// Trigger Interrupt Enable
        pub const TIE: u32 = 1 << 6;
        /// Break Interrupt Enable
        pub const BIE: u32 = 1 << 7;
    }
}

/// GFXTIM instance
pub struct Gfxtim;

impl Gfxtim {
    /// Create GFXTIM instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable timer
    pub fn enable(&self) {
        unsafe {
            let cr1 = (GFXTIM_BASE + reg::CR1) as *mut u32;
            let mut val = read_volatile(cr1);
            val |= bits::cr1::CEN;
            write_volatile(cr1, val);
        }
    }

    /// Disable timer
    pub fn disable(&self) {
        unsafe {
            let cr1 = (GFXTIM_BASE + reg::CR1) as *mut u32;
            let mut val = read_volatile(cr1);
            val &= !bits::cr1::CEN;
            write_volatile(cr1, val);
        }
    }

    /// Set prescaler
    pub fn set_prescaler(&self, prescaler: u16) {
        unsafe {
            let psc = (GFXTIM_BASE + reg::PSC) as *mut u32;
            write_volatile(psc, prescaler as u32);
        }
    }

    /// Set auto-reload value
    pub fn set_auto_reload(&self, arr: u16) {
        unsafe {
            let arr_reg = (GFXTIM_BASE + reg::ARR) as *mut u32;
            write_volatile(arr_reg, arr as u32);
        }
    }

    /// Get counter value
    pub fn get_counter(&self) -> u16 {
        unsafe {
            let cnt = (GFXTIM_BASE + reg::CNT) as *const u32;
            read_volatile(cnt) as u16
        }
    }

    /// Set capture/compare 1 value
    pub fn set_ccr1(&self, value: u16) {
        unsafe {
            let ccr1 = (GFXTIM_BASE + reg::CCR1) as *mut u32;
            write_volatile(ccr1, value as u32);
        }
    }

    /// Enable update interrupt
    pub fn enable_update_interrupt(&self) {
        unsafe {
            let dier = (GFXTIM_BASE + reg::DIER) as *mut u32;
            let mut val = read_volatile(dier);
            val |= bits::dier::UIE;
            write_volatile(dier, val);
        }
    }

    /// Check update interrupt flag
    pub fn is_update_flag(&self) -> bool {
        unsafe {
            let sr = (GFXTIM_BASE + reg::SR) as *const u32;
            (read_volatile(sr) & bits::sr::UIF) != 0
        }
    }

    /// Clear update interrupt flag
    pub fn clear_update_flag(&self) {
        unsafe {
            let sr = (GFXTIM_BASE + reg::SR) as *mut u32;
            let mut val = read_volatile(sr);
            val &= !bits::sr::UIF;
            write_volatile(sr, val);
        }
    }

    /// Generate update event
    pub fn generate_update_event(&self) {
        unsafe {
            let egr = (GFXTIM_BASE + reg::EGR) as *mut u32;
            write_volatile(egr, 1 << 0);
        }
    }
}

impl Default for Gfxtim {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize GFXTIM with default configuration
pub fn init_gfxtim_default() -> Gfxtim {
    let gfxtim = Gfxtim::new();
    gfxtim
}

