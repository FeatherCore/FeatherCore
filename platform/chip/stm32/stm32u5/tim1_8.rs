//! TIM1/TIM8 - Advanced-control Timers
//! 高级控制定时器 (TIM1/TIM8)
//!
//! # Overview / 概述
//! STM32U5 advanced-control timers TIM1 and TIM8 provide sophisticated
//! timing capabilities with complementary outputs, dead-time insertion,
//! and emergency stop functions.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 54: Advanced-control timers (TIM1 and TIM8)
//!
//! ## Main Features / 主要特性
//! - 16-bit up/down auto-reload counter
//! - Up to 4 capture/compare channels
//! - Complementary outputs with dead-time insertion
//! - Emergency stop input (break)
//! - Programmable dead-time generator
//! - Repetition counter
//! - DMA support
//! - Break input for emergency stop
//!
//! # Reference / 参考
//! - RM0456 Chapter 54: Advanced-control timers (TIM1 and TIM8)
//!   - Register map: RM0456, Section 54.4, pages 2147-2232
//!   - TIMx Control Register 1 (TIMx_CR1): RM0456, Section 54.4.1, page 2148
//!   - TIMx Status Register (TIMx_SR): RM0456, Section 54.4.5, page 2156
//!   - TIMx Capture/Compare Registers (TIMx_CCRx): RM0456, Section 54.4.14, page 2168

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// TIM1 base address (APB2)
pub const TIM1_BASE: usize = 0x4001_0000;

/// TIM8 base address (APB2)
pub const TIM8_BASE: usize = 0x4001_3400;

/// Timer register offsets (common for TIM1/8)
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
    /// Capture/Compare Mode Register 3
    pub const CCMR3: usize = 0x54;
    /// Capture/Compare Register 5
    pub const CCR5: usize = 0x58;
    /// Capture/Compare Register 6
    pub const CCR6: usize = 0x5C;
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
        /// UIF status bit remapping
        pub const UIFREMAP: u32 = 1 << 11;
    }

    /// CR2 Register Bits
    pub mod cr2 {
        /// Capture/compare preloaded control
        pub const CCPC: u32 = 1 << 0;
        /// Capture/compare control update selection
        pub const CCUS: u32 = 1 << 2;
        /// Master mode selection
        pub const MMS: u32 = 0b111 << 4;
        /// TI1 selection
        pub const TI1S: u32 = 1 << 7;
        /// Output Idle State 1
        pub const OIS1: u32 = 1 << 8;
        /// Output Idle State 1N
        pub const OIS1N: u32 = 1 << 9;
        /// Output Idle State 2
        pub const OIS2: u32 = 1 << 10;
        /// Output Idle State 2N
        pub const OIS2N: u32 = 1 << 11;
        /// Output Idle State 3
        pub const OIS3: u32 = 1 << 12;
        /// Output Idle State 3N
        pub const OIS3N: u32 = 1 << 13;
        /// Output Idle State 4
        pub const OIS4: u32 = 1 << 14;
        /// Output Idle State 5
        pub const OIS5: u32 = 1 << 16;
        /// Output Idle State 6
        pub const OIS6: u32 = 1 << 18;
    }

    /// SMCR Register Bits
    pub mod smcr {
        /// Slave mode selection
        pub const SMS: u32 = 0b111 << 0;
        /// Trigger selection
        pub const TS: u32 = 0b111 << 4;
        /// Master/Slave mode
        pub const MSM: u32 = 1 << 7;
        /// External trigger filter
        pub const ETF: u32 = 0b1111 << 8;
        /// External trigger prescaler
        pub const ETPS: u32 = 0b11 << 12;
        /// External clock enable
        pub const ECE: u32 = 1 << 14;
        /// External trigger polarity
        pub const ETP: u32 = 1 << 15;
        /// Slave mode selection bit 3
        pub const SMS_3: u32 = 1 << 16;
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
        /// Update DMA Request Enable
        pub const UDE: u32 = 1 << 8;
        /// Capture/Compare 1 DMA Request Enable
        pub const CC1DE: u32 = 1 << 9;
        /// Capture/Compare 2 DMA Request Enable
        pub const CC2DE: u32 = 1 << 10;
        /// Capture/Compare 3 DMA Request Enable
        pub const CC3DE: u32 = 1 << 11;
        /// Capture/Compare 4 DMA Request Enable
        pub const CC4DE: u32 = 1 << 12;
        /// COM DMA Request Enable
        pub const COMDE: u32 = 1 << 13;
        /// Trigger DMA Request Enable
        pub const TDE: u32 = 1 << 14;
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
        /// Capture/Compare 1 Overcapture Flag
        pub const CC1OF: u32 = 1 << 9;
        /// Capture/Compare 2 Overcapture Flag
        pub const CC2OF: u32 = 1 << 10;
        /// Capture/Compare 3 Overcapture Flag
        pub const CC3OF: u32 = 1 << 11;
        /// Capture/Compare 4 Overcapture Flag
        pub const CC4OF: u32 = 1 << 12;
        /// Capture/Compare 5 Interrupt Flag
        pub const CC5IF: u32 = 1 << 16;
        /// Capture/Compare 6 Interrupt Flag
        pub const CC6IF: u32 = 1 << 17;
    }

    /// EGR Register Bits
    pub mod egr {
        /// Update Generation
        pub const UG: u32 = 1 << 0;
        /// Capture/Compare 1 Generation
        pub const CC1G: u32 = 1 << 1;
        /// Capture/Compare 2 Generation
        pub const CC2G: u32 = 1 << 2;
        /// Capture/Compare 3 Generation
        pub const CC3G: u32 = 1 << 3;
        /// Capture/Compare 4 Generation
        pub const CC4G: u32 = 1 << 4;
        /// Capture/Compare Control Update Generation
        pub const COMG: u32 = 1 << 5;
        /// Trigger Generation
        pub const TG: u32 = 1 << 6;
        /// Break Generation
        pub const BG: u32 = 1 << 7;
        /// Capture/Compare 5 Generation
        pub const CC5G: u32 = 1 << 16;
        /// Capture/Compare 6 Generation
        pub const CC6G: u32 = 1 << 17;
    }

    /// CCMR1 Register Bits (Output mode)
    pub mod ccmr1_output {
        /// Capture/Compare 1 Selection
        pub const CC1S: u32 = 0b11 << 0;
        /// Output Compare 1 Fast Enable
        pub const OC1FE: u32 = 1 << 2;
        /// Output Compare 1 Preload Enable
        pub const OC1PE: u32 = 1 << 3;
        /// Output Compare 1 Mode
        pub const OC1M: u32 = 0b111 << 4;
        /// Output Compare 1 Clear Enable
        pub const OC1CE: u32 = 1 << 7;
        /// Capture/Compare 2 Selection
        pub const CC2S: u32 = 0b11 << 8;
        /// Output Compare 2 Fast Enable
        pub const OC2FE: u32 = 1 << 10;
        /// Output Compare 2 Preload Enable
        pub const OC2PE: u32 = 1 << 11;
        /// Output Compare 2 Mode
        pub const OC2M: u32 = 0b111 << 12;
        /// Output Compare 2 Clear Enable
        pub const OC2CE: u32 = 1 << 15;
        /// Output Compare 1 Mode bit 3
        pub const OC1M_3: u32 = 1 << 16;
        /// Output Compare 2 Mode bit 3
        pub const OC2M_3: u32 = 1 << 24;
    }

    /// CCMR1 Register Bits (Input mode)
    pub mod ccmr1_input {
        /// Capture/Compare 1 Selection
        pub const CC1S: u32 = 0b11 << 0;
        /// Input Capture 1 Prescaler
        pub const IC1PSC: u32 = 0b11 << 2;
        /// Input Capture 1 Filter
        pub const IC1F: u32 = 0b1111 << 4;
        /// Capture/Compare 2 Selection
        pub const CC2S: u32 = 0b11 << 8;
        /// Input Capture 2 Prescaler
        pub const IC2PSC: u32 = 0b11 << 10;
        /// Input Capture 2 Filter
        pub const IC2F: u32 = 0b1111 << 12;
    }

    /// CCMR2 Register Bits (Output mode)
    pub mod ccmr2_output {
        /// Capture/Compare 3 Selection
        pub const CC3S: u32 = 0b11 << 0;
        /// Output Compare 3 Fast Enable
        pub const OC3FE: u32 = 1 << 2;
        /// Output Compare 3 Preload Enable
        pub const OC3PE: u32 = 1 << 3;
        /// Output Compare 3 Mode
        pub const OC3M: u32 = 0b111 << 4;
        /// Output Compare 3 Clear Enable
        pub const OC3CE: u32 = 1 << 7;
        /// Capture/Compare 4 Selection
        pub const CC4S: u32 = 0b11 << 8;
        /// Output Compare 4 Fast Enable
        pub const OC4FE: u32 = 1 << 10;
        /// Output Compare 4 Preload Enable
        pub const OC4PE: u32 = 1 << 11;
        /// Output Compare 4 Mode
        pub const OC4M: u32 = 0b111 << 12;
        /// Output Compare 4 Clear Enable
        pub const OC4CE: u32 = 1 << 15;
        /// Output Compare 3 Mode bit 3
        pub const OC3M_3: u32 = 1 << 16;
        /// Output Compare 4 Mode bit 3
        pub const OC4M_3: u32 = 1 << 24;
    }

    /// CCMR2 Register Bits (Input mode)
    pub mod ccmr2_input {
        /// Capture/Compare 3 Selection
        pub const CC3S: u32 = 0b11 << 0;
        /// Input Capture 3 Prescaler
        pub const IC3PSC: u32 = 0b11 << 2;
        /// Input Capture 3 Filter
        pub const IC3F: u32 = 0b1111 << 4;
        /// Capture/Compare 4 Selection
        pub const CC4S: u32 = 0b11 << 8;
        /// Input Capture 4 Prescaler
        pub const IC4PSC: u32 = 0b11 << 10;
        /// Input Capture 4 Filter
        pub const IC4F: u32 = 0b1111 << 12;
    }

    /// CCER Register Bits
    pub mod ccer {
        /// Capture/Compare 1 Output Enable
        pub const CC1E: u32 = 1 << 0;
        /// Capture/Compare 1 Output Polarity
        pub const CC1P: u32 = 1 << 1;
        /// Capture/Compare 1 Complementary Output Enable
        pub const CC1NE: u32 = 1 << 2;
        /// Capture/Compare 1 Complementary Output Polarity
        pub const CC1NP: u32 = 1 << 3;
        /// Capture/Compare 2 Output Enable
        pub const CC2E: u32 = 1 << 4;
        /// Capture/Compare 2 Output Polarity
        pub const CC2P: u32 = 1 << 5;
        /// Capture/Compare 2 Complementary Output Enable
        pub const CC2NE: u32 = 1 << 6;
        /// Capture/Compare 2 Complementary Output Polarity
        pub const CC2NP: u32 = 1 << 7;
        /// Capture/Compare 3 Output Enable
        pub const CC3E: u32 = 1 << 8;
        /// Capture/Compare 3 Output Polarity
        pub const CC3P: u32 = 1 << 9;
        /// Capture/Compare 3 Complementary Output Enable
        pub const CC3NE: u32 = 1 << 10;
        /// Capture/Compare 3 Complementary Output Polarity
        pub const CC3NP: u32 = 1 << 11;
        /// Capture/Compare 4 Output Enable
        pub const CC4E: u32 = 1 << 12;
        /// Capture/Compare 4 Output Polarity
        pub const CC4P: u32 = 1 << 13;
        /// Capture/Compare 5 Output Enable
        pub const CC5E: u32 = 1 << 16;
        /// Capture/Compare 5 Output Polarity
        pub const CC5P: u32 = 1 << 17;
        /// Capture/Compare 6 Output Enable
        pub const CC6E: u32 = 1 << 20;
        /// Capture/Compare 6 Output Polarity
        pub const CC6P: u32 = 1 << 21;
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
        /// Break Filter
        pub const BKF: u32 = 0b1111 << 16;
        /// Break 2 Filter
        pub const BK2F: u32 = 0b1111 << 20;
        /// Break 2 Enable
        pub const BK2E: u32 = 1 << 24;
        /// Break 2 Polarity
        pub const BK2P: u32 = 1 << 25;
    }

    /// DCR Register Bits
    pub mod dcr {
        /// DMA Base Address
        pub const DBA: u32 = 0b11111 << 0;
        /// DMA Burst Length
        pub const DBL: u32 = 0b11111 << 8;
    }

    /// CCMR3 Register Bits
    pub mod ccmr3_output {
        /// Output Compare 5 Mode
        pub const OC5M: u32 = 0b111 << 4;
        /// Output Compare 5 Preload Enable
        pub const OC5PE: u32 = 1 << 3;
        /// Output Compare 5 Fast Enable
        pub const OC5FE: u32 = 1 << 2;
        /// Output Compare 5 Mode bit 3
        pub const OC5M_3: u32 = 1 << 16;
        /// Output Compare 6 Mode
        pub const OC6M: u32 = 0b111 << 20;
        /// Output Compare 6 Preload Enable
        pub const OC6PE: u32 = 1 << 19;
        /// Output Compare 6 Fast Enable
        pub const OC6FE: u32 = 1 << 18;
        /// Output Compare 6 Mode bit 3
        pub const OC6M_3: u32 = 1 << 24;
    }
}

/// Advanced Timer instance
pub struct AdvancedTimer {
    base: usize,
}

impl AdvancedTimer {
    /// Create TIM1 instance
    pub const fn tim1() -> Self {
        Self { base: TIM1_BASE }
    }

    /// Create TIM8 instance
    pub const fn tim8() -> Self {
        Self { base: TIM8_BASE }
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

    /// Set repetition counter
    pub fn set_repetition_counter(&self, rcr: u8) {
        unsafe {
            let rcr_reg = (self.base + reg::RCR) as *mut u32;
            write_volatile(rcr_reg, rcr as u32);
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

    /// Set capture/compare 3 value
    pub fn set_ccr3(&self, value: u16) {
        unsafe {
            let ccr3 = (self.base + reg::CCR3) as *mut u32;
            write_volatile(ccr3, value as u32);
        }
    }

    /// Set capture/compare 4 value
    pub fn set_ccr4(&self, value: u16) {
        unsafe {
            let ccr4 = (self.base + reg::CCR4) as *mut u32;
            write_volatile(ccr4, value as u32);
        }
    }

    /// Set capture/compare 5 value
    pub fn set_ccr5(&self, value: u16) {
        unsafe {
            let ccr5 = (self.base + reg::CCR5) as *mut u32;
            write_volatile(ccr5, value as u32);
        }
    }

    /// Set capture/compare 6 value
    pub fn set_ccr6(&self, value: u16) {
        unsafe {
            let ccr6 = (self.base + reg::CCR6) as *mut u32;
            write_volatile(ccr6, value as u32);
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

    /// Set dead-time
    pub fn set_dead_time(&self, dtg: u8) {
        unsafe {
            let bdtr = (self.base + reg::BDTR) as *mut u32;
            let mut val = read_volatile(bdtr);
            val &= !bits::bdtr::DTG;
            val |= dtg as u32;
            write_volatile(bdtr, val);
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
            write_volatile(egr, bits::egr::UG);
        }
    }

    /// Generate COM event
    pub fn generate_com_event(&self) {
        unsafe {
            let egr = (self.base + reg::EGR) as *mut u32;
            write_volatile(egr, bits::egr::COMG);
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
}

impl Default for AdvancedTimer {
    fn default() -> Self {
        Self::tim1()
    }
}

/// Initialize TIM1 with default configuration
pub fn init_tim1_default() -> AdvancedTimer {
    let tim = AdvancedTimer::tim1();
    tim
}

/// Initialize TIM8 with default configuration
pub fn init_tim8_default() -> AdvancedTimer {
    let tim = AdvancedTimer::tim8();
    tim
}
