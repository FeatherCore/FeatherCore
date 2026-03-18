//! TIM15/TIM16/TIM17 - General Purpose Timers
//! 通用定时器 (TIM15/TIM16/TIM17)
//!
//! # Overview / 概述
//! STM32U5 general-purpose timers TIM15, TIM16, and TIM17 provide
//! 2-channel timing and control capabilities with various features.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 56: General-purpose timers (TIM15/TIM16/TIM17)
//!
//! ## Main Features / 主要特性
//! - 16-bit up/down auto-reload counter
//! - 2 capture/compare channels
//! - Programmable prescaler
//! - Interrupt/DMA generation on update
//! - Break input for emergency stop
//! - Complementary outputs with dead-time insertion
//!
//! # Reference / 参考
//! - RM0456 Chapter 56: General-purpose timers (TIM15/TIM16/TIM17)
//!   - Register map: RM0456, Section 56.4, pages 2283-2316
//!   - TIMx Control Register 1 (TIMx_CR1): RM0456, Section 56.4.1, page 2284
//!   - TIMx Status Register (TIMx_SR): RM0456, Section 56.4.5, page 2290
//!   - TIMx Capture/Compare Registers (TIMx_CCRx): RM0456, Section 56.4.14, page 2300

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// TIM15 base address (APB2)
pub const TIM15_BASE: usize = 0x4001_4000;

/// TIM16 base address (APB2)
pub const TIM16_BASE: usize = 0x4001_4400;

/// TIM17 base address (APB2)
pub const TIM17_BASE: usize = 0x4001_4800;

/// Timer register offsets (common for TIM15/16/17)
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
    /// Break and Dead-time Register
    pub const BDTR: usize = 0x44;
    /// DMA Control Register
    pub const DCR: usize = 0x48;
    /// DMA Address Register
    pub const DMAR: usize = 0x4C;
    /// Option Register
    pub const OR: usize = 0x50;
}

/// Timer register bit definitions
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
        /// COM Interrupt Enable
        pub const COMIE: u32 = 1 << 5;
        /// Trigger Interrupt Enable
        pub const TIE: u32 = 1 << 6;
        /// Break Interrupt Enable
        pub const BIE: u32 = 1 << 7;
    }

    /// BDTR Register Bits
    pub mod bdtr {
        /// Dead-time Generator Enable
        pub const DTG: u32 = 0xFF << 0;
        /// Lock Configuration
        pub const LOCK: u32 = 0b11 << 8;
        /// Off-state Selection for Idle mode
        pub const OSSI: u32 = 1 << 10;
        /// Off-state Selection for Run mode
        pub const OSSR: u32 = 1 << 11;
        /// Break Enable
        pub const BKE: u32 = 1 << 12;
        /// Break Polarity
        pub const BKP: u32 = 1 << 13;
        /// Automatic Output Enable
        pub const AOE: u32 = 1 << 14;
        /// Main Output Enable
        pub const MOE: u32 = 1 << 15;
    }
}

/// Timer instance
pub struct Timer {
    base: usize,
}

impl Timer {
    /// Create TIM15 instance
    pub const fn tim15() -> Self {
        Self { base: TIM15_BASE }
    }

    /// Create TIM16 instance
    pub const fn tim16() -> Self {
        Self { base: TIM16_BASE }
    }

    /// Create TIM17 instance
    pub const fn tim17() -> Self {
        Self { base: TIM17_BASE }
    }

    /// Enable timer
    pub fn enable(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = read_volatile(cr1);
            val |= bits::cr1::CEN;
            write_volatile(cr1, val);
        }
    }

    /// Disable timer
    pub fn disable(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = read_volatile(cr1);
            val &= !bits::cr1::CEN;
            write_volatile(cr1, val);
        }
    }

    /// Set prescaler
    pub fn set_prescaler(&self, prescaler: u16) {
        unsafe {
            let psc = (self.base + reg::PSC) as *mut u32;
            write_volatile(psc, prescaler as u32);
        }
    }

    /// Set auto-reload value
    pub fn set_auto_reload(&self, arr: u16) {
        unsafe {
            let arr_reg = (self.base + reg::ARR) as *mut u32;
            write_volatile(arr_reg, arr as u32);
        }
    }

    /// Get counter value
    pub fn get_counter(&self) -> u16 {
        unsafe {
            let cnt = (self.base + reg::CNT) as *const u32;
            read_volatile(cnt) as u16
        }
    }

    /// Set counter value
    pub fn set_counter(&self, value: u16) {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            write_volatile(cnt, value as u32);
        }
    }

    /// Set capture/compare 1 value
    pub fn set_ccr1(&self, value: u16) {
        unsafe {
            let ccr1 = (self.base + reg::CCR1) as *mut u32;
            write_volatile(ccr1, value as u32);
        }
    }

    /// Set capture/compare 2 value
    pub fn set_ccr2(&self, value: u16) {
        unsafe {
            let ccr2 = (self.base + reg::CCR2) as *mut u32;
            write_volatile(ccr2, value as u32);
        }
    }

    /// Enable update interrupt
    pub fn enable_update_interrupt(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = read_volatile(dier);
            val |= bits::dier::UIE;
            write_volatile(dier, val);
        }
    }

    /// Disable update interrupt
    pub fn disable_update_interrupt(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = read_volatile(dier);
            val &= !bits::dier::UIE;
            write_volatile(dier, val);
        }
    }

    /// Check update interrupt flag
    pub fn is_update_flag(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *const u32;
            (read_volatile(sr) & bits::sr::UIF) != 0
        }
    }

    /// Clear update interrupt flag
    pub fn clear_update_flag(&self) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let mut val = read_volatile(sr);
            val &= !bits::sr::UIF;
            write_volatile(sr, val);
        }
    }

    /// Generate update event
    pub fn generate_update_event(&self) {
        unsafe {
            let egr = (self.base + reg::EGR) as *mut u32;
            write_volatile(egr, 1 << 0);
        }
    }

    /// Enable main output (MOE)
    pub fn enable_main_output(&self) {
        unsafe {
            let bdtr = (self.base + reg::BDTR) as *mut u32;
            let mut val = read_volatile(bdtr);
            val |= bits::bdtr::MOE;
            write_volatile(bdtr, val);
        }
    }

    /// Disable main output (MOE)
    pub fn disable_main_output(&self) {
        unsafe {
            let bdtr = (self.base + reg::BDTR) as *mut u32;
            let mut val = read_volatile(bdtr);
            val &= !bits::bdtr::MOE;
            write_volatile(bdtr, val);
        }
    }
}

