//! TIM6/TIM7 - Basic Timers
//! 基本定时器 (TIM6/TIM7)
//!
//! # Overview / 概述
//! STM32U5 basic timers TIM6 and TIM7 provide simple timing capabilities
//! with 16-bit auto-reload counters, primarily used for timebase generation
//! and DAC triggering.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 57: Basic timers (TIM6/TIM7)
//!
//! ## Main Features / 主要特性
//! - 16-bit auto-reload up-counter
//! - Programmable prescaler
//! - Update interrupt/DMA generation
//! - DAC trigger output
//! - No input/output channels
//!
//! # Reference / 参考
//! - RM0456 Chapter 57: Basic timers (TIM6/TIM7)
//!   - Register map: RM0456, Section 57.4, pages 2325-2340
//!   - TIMx Control Register 1 (TIMx_CR1): RM0456, Section 57.4.1, page 2326
//!   - TIMx Status Register (TIMx_SR): RM0456, Section 57.4.3, page 2328

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// TIM6 base address (APB1)
pub const TIM6_BASE: usize = 0x4000_1000;

/// TIM7 base address (APB1)
pub const TIM7_BASE: usize = 0x4000_1400;

/// Timer register offsets (common for TIM6/7)
pub mod reg {
    /// Control Register 1
    pub const CR1: usize = 0x00;
    /// Control Register 2
    pub const CR2: usize = 0x04;
    /// DMA/Interrupt Enable Register
    pub const DIER: usize = 0x0C;
    /// Status Register
    pub const SR: usize = 0x10;
    /// Event Generation Register
    pub const EGR: usize = 0x14;
    /// Counter
    pub const CNT: usize = 0x24;
    /// Prescaler
    pub const PSC: usize = 0x28;
    /// Auto-reload Register
    pub const ARR: usize = 0x2C;
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
        /// Auto-reload Preload Enable
        pub const ARPE: u32 = 1 << 7;
    }

    /// CR2 Register Bits
    pub mod cr2 {
        /// Master Mode Selection
        pub const MMS: u32 = 0b111 << 4;
    }

    /// SR Register Bits
    pub mod sr {
        /// Update Interrupt Flag
        pub const UIF: u32 = 1 << 0;
    }

    /// DIER Register Bits
    pub mod dier {
        /// Update Interrupt Enable
        pub const UIE: u32 = 1 << 0;
        /// Update DMA Request Enable
        pub const UDE: u32 = 1 << 8;
    }
}

/// Basic Timer instance
pub struct BasicTimer {
    base: usize,
}

impl BasicTimer {
    /// Create TIM6 instance
    pub const fn tim6() -> Self {
        Self { base: TIM6_BASE }
    }

    /// Create TIM7 instance
    pub const fn tim7() -> Self {
        Self { base: TIM7_BASE }
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

    /// Enable auto-reload preload
    pub fn enable_auto_reload_preload(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = read_volatile(cr1);
            val |= bits::cr1::ARPE;
            write_volatile(cr1, val);
        }
    }

    /// Disable auto-reload preload
    pub fn disable_auto_reload_preload(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = read_volatile(cr1);
            val &= !bits::cr1::ARPE;
            write_volatile(cr1, val);
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

    /// Enable update DMA
    pub fn enable_update_dma(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = read_volatile(dier);
            val |= bits::dier::UDE;
            write_volatile(dier, val);
        }
    }

    /// Disable update DMA
    pub fn disable_update_dma(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = read_volatile(dier);
            val &= !bits::dier::UDE;
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

    /// Configure master mode for DAC trigger
    pub fn set_master_mode(&self, mode: u8) {
        unsafe {
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let mut val = read_volatile(cr2);
            val &= !bits::cr2::MMS;
            val |= ((mode as u32) & 0b111) << 4;
            write_volatile(cr2, val);
        }
    }

    /// Initialize timer for delay
    pub fn init_delay(&self, prescaler: u16, auto_reload: u16) {
        self.set_prescaler(prescaler);
        self.set_auto_reload(auto_reload);
        self.enable_auto_reload_preload();
        self.generate_update_event();
        self.clear_update_flag();
    }

    /// Delay in milliseconds (blocking)
    pub fn delay_ms(&self, ms: u32, timer_freq_hz: u32) {
        let ticks = (timer_freq_hz / 1000) * ms;
        let prescaler = (ticks / 0xFFFF) as u16 + 1;
        let auto_reload = (ticks / prescaler as u32) as u16;

        self.init_delay(prescaler - 1, auto_reload - 1);
        self.enable();

        while !self.is_update_flag() {}
        self.clear_update_flag();
        self.disable();
    }
}

impl Default for BasicTimer {
    fn default() -> Self {
        Self::tim6()
    }
}

/// Initialize TIM6 as delay timer
pub fn init_tim6_delay() -> BasicTimer {
    let timer = BasicTimer::tim6();
    timer
}

/// Simple delay using TIM6
pub fn delay_ms_tim6(ms: u32) {
    let timer = BasicTimer::tim6();
    timer.delay_ms(ms, 16000000);
}

