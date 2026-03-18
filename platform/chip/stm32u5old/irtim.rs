//! IRTIM - Infrared Timer
//! 红外定时器
//!
//! # Overview / 概述
//! STM32U5 Infrared Timer (IRTIM) provides infrared modulation capabilities,
//! allowing the generation of infrared remote control signals with various
//! modulation schemes.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 60: Infrared timer (IRTIM)
//!
//! ## Main Features / 主要特性
//! - Infrared signal generation
//! - Support for multiple modulation schemes
//! - Carrier frequency generation
//! - Programmable duty cycle
//! - Integration with GPIO for IR output
//! - Low power modes support
//!
//! # Reference / 参考
//! - RM0456 Chapter 60: Infrared timer (IRTIM)
//!   - Register map: RM0456, Section 60.4, pages 2471-2484
//!   - IRTIM Control Register (IRTIM_CR): RM0456, Section 60.4.1, page 2472
//!   - IRTIM Status Register (IRTIM_SR): RM0456, Section 60.4.2, page 2474

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// IRTIM base address
pub const IRTIM_BASE: usize = 0x4001_5800;

/// IRTIM register offsets
pub mod reg {
    /// Control Register (IRTIM_CR)
    pub const CR: usize = 0x00;
    /// Status Register (IRTIM_SR)
    pub const SR: usize = 0x04;
    /// Interrupt Enable Register (IRTIM_IER)
    pub const IER: usize = 0x08;
    /// Interrupt Clear Register (IRTIM_ICR)
    pub const ICR: usize = 0x0C;
    /// Prescaler Register (IRTIM_PSC)
    pub const PSC: usize = 0x10;
    /// Auto-reload Register (IRTIM_ARR)
    pub const ARR: usize = 0x14;
    /// Compare Register 1 (IRTIM_CCR1)
    pub const CCR1: usize = 0x18;
    /// Compare Register 2 (IRTIM_CCR2)
    pub const CCR2: usize = 0x1C;
    /// Compare Register 3 (IRTIM_CCR3)
    pub const CCR3: usize = 0x20;
    /// Compare Register 4 (IRTIM_CCR4)
    pub const CCR4: usize = 0x24;
    /// Dead-time Register (IRTIM_DTR)
    pub const DTR: usize = 0x28;
    /// Repetition Counter Register (IRTIM_RCR)
    pub const RCR: usize = 0x2C;
    /// Break and Dead-time Register (IRTIM_BDTR)
    pub const BDTR: usize = 0x30;
    /// DMA Control Register (IRTIM_DCR)
    pub const DCR: usize = 0x34;
    /// DMA Address Register (IRTIM_DMAR)
    pub const DMAR: usize = 0x38;
    /// Option Register 1 (IRTIM_OR1)
    pub const OR1: usize = 0x3C;
    /// Option Register 2 (IRTIM_OR2)
    pub const OR2: usize = 0x40;
}

/// IRTIM register bit definitions
pub mod bits {
    /// CR Register Bits
    pub mod cr {
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
    }

    /// IER Register Bits
    pub mod ier {
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

    /// ICR Register Bits
    pub mod icr {
        /// Update Interrupt Clear
        pub const UIF: u32 = 1 << 0;
        /// Capture/Compare 1 Interrupt Clear
        pub const CC1IF: u32 = 1 << 1;
        /// Capture/Compare 2 Interrupt Clear
        pub const CC2IF: u32 = 1 << 2;
        /// Capture/Compare 3 Interrupt Clear
        pub const CC3IF: u32 = 1 << 3;
        /// Capture/Compare 4 Interrupt Clear
        pub const CC4IF: u32 = 1 << 4;
        /// COM Interrupt Clear
        pub const COMIF: u32 = 1 << 5;
        /// Trigger Interrupt Clear
        pub const TIF: u32 = 1 << 6;
        /// Break Interrupt Clear
        pub const BIF: u32 = 1 << 7;
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

    /// OR1 Register Bits
    pub mod or1 {
        /// IRTIM Output 1 Remap
        pub const IRTIM_OUT1_RMP: u32 = 0b11 << 0;
        /// IRTIM Output 2 Remap
        pub const IRTIM_OUT2_RMP: u32 = 0b11 << 2;
    }

    /// OR2 Register Bits
    pub mod or2 {
        /// IRTIM Input 1 Remap
        pub const IRTIM_IN1_RMP: u32 = 0b11 << 0;
        /// IRTIM Input 2 Remap
        pub const IRTIM_IN2_RMP: u32 = 0b11 << 2;
    }
}

/// Infrared modulation mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModulationMode {
    /// No modulation (direct output)
    Direct = 0,
    /// PWM modulation
    Pwm = 1,
    /// Manchester encoding
    Manchester = 2,
    /// RC5 encoding
    Rc5 = 3,
}

/// IRTIM instance
pub struct Irtim;

impl Irtim {
    /// Create IRTIM instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable IRTIM
    pub fn enable(&self) {
        unsafe {
            let cr = (IRTIM_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= bits::cr::CEN;
            write_volatile(cr, val);
        }
    }

    /// Disable IRTIM
    pub fn disable(&self) {
        unsafe {
            let cr = (IRTIM_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !bits::cr::CEN;
            write_volatile(cr, val);
        }
    }

    /// Set prescaler
    pub fn set_prescaler(&self, prescaler: u16) {
        unsafe {
            let psc = (IRTIM_BASE + reg::PSC) as *mut u32;
            write_volatile(psc, prescaler as u32);
        }
    }

    /// Set auto-reload value (carrier frequency period)
    pub fn set_auto_reload(&self, arr: u16) {
        unsafe {
            let arr_reg = (IRTIM_BASE + reg::ARR) as *mut u32;
            write_volatile(arr_reg, arr as u32);
        }
    }

    /// Set compare register 1 (duty cycle for PWM)
    pub fn set_ccr1(&self, value: u16) {
        unsafe {
            let ccr1 = (IRTIM_BASE + reg::CCR1) as *mut u32;
            write_volatile(ccr1, value as u32);
        }
    }

    /// Set compare register 2
    pub fn set_ccr2(&self, value: u16) {
        unsafe {
            let ccr2 = (IRTIM_BASE + reg::CCR2) as *mut u32;
            write_volatile(ccr2, value as u32);
        }
    }

    /// Set compare register 3
    pub fn set_ccr3(&self, value: u16) {
        unsafe {
            let ccr3 = (IRTIM_BASE + reg::CCR3) as *mut u32;
            write_volatile(ccr3, value as u32);
        }
    }

    /// Set compare register 4
    pub fn set_ccr4(&self, value: u16) {
        unsafe {
            let ccr4 = (IRTIM_BASE + reg::CCR4) as *mut u32;
            write_volatile(ccr4, value as u32);
        }
    }

    /// Get counter value
    pub fn get_counter(&self) -> u16 {
        unsafe {
            let cnt = (IRTIM_BASE + reg::CR) as *const u32;
            (read_volatile(cnt) >> 16) as u16
        }
    }

    /// Enable update interrupt
    pub fn enable_update_interrupt(&self) {
        unsafe {
            let ier = (IRTIM_BASE + reg::IER) as *mut u32;
            let mut val = read_volatile(ier);
            val |= bits::ier::UIE;
            write_volatile(ier, val);
        }
    }

    /// Disable update interrupt
    pub fn disable_update_interrupt(&self) {
        unsafe {
            let ier = (IRTIM_BASE + reg::IER) as *mut u32;
            let mut val = read_volatile(ier);
            val &= !bits::ier::UIE;
            write_volatile(ier, val);
        }
    }

    /// Check update interrupt flag
    pub fn is_update_flag(&self) -> bool {
        unsafe {
            let sr = (IRTIM_BASE + reg::SR) as *const u32;
            (read_volatile(sr) & bits::sr::UIF) != 0
        }
    }

    /// Clear update interrupt flag
    pub fn clear_update_flag(&self) {
        unsafe {
            let icr = (IRTIM_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, bits::icr::UIF);
        }
    }

    /// Generate update event
    pub fn generate_update_event(&self) {
        unsafe {
            let cr = (IRTIM_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= 1 << 1;
            write_volatile(cr, val);
        }
    }

    /// Enable auto-reload preload
    pub fn enable_auto_reload_preload(&self) {
        unsafe {
            let cr = (IRTIM_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= bits::cr::ARPE;
            write_volatile(cr, val);
        }
    }

    /// Disable auto-reload preload
    pub fn disable_auto_reload_preload(&self) {
        unsafe {
            let cr = (IRTIM_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !bits::cr::ARPE;
            write_volatile(cr, val);
        }
    }

    /// Configure for infrared transmission with PWM modulation
    /// 
    /// # Arguments
    /// * `carrier_freq_hz` - Carrier frequency in Hz (typically 38kHz)
    /// * `duty_cycle_percent` - Duty cycle percentage (0-100)
    /// * `timer_freq_hz` - Timer clock frequency in Hz
    pub fn configure_pwm_ir(&self, carrier_freq_hz: u32, duty_cycle_percent: u8, timer_freq_hz: u32) {
        let period = timer_freq_hz / carrier_freq_hz;
        let prescaler = (period / 0xFFFF) as u16 + 1;
        let arr = (period / prescaler as u32) as u16;
        let ccr = (arr as u32 * duty_cycle_percent as u32 / 100) as u16;

        self.set_prescaler(prescaler - 1);
        self.set_auto_reload(arr - 1);
        self.set_ccr1(ccr);
        self.enable_auto_reload_preload();
        self.generate_update_event();
        self.clear_update_flag();
    }

    /// Start infrared transmission
    pub fn start_transmission(&self) {
        self.enable();
    }

    /// Stop infrared transmission
    pub fn stop_transmission(&self) {
        self.disable();
    }
}

impl Default for Irtim {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize IRTIM with default configuration for 38kHz IR
pub fn init_irtim_default() -> Irtim {
    let irtim = Irtim::new();
    irtim.configure_pwm_ir(38000, 50, 16000000);
    irtim
}

/// Simple IR transmission using 38kHz carrier
pub fn send_ir_pulse(duration_us: u32) {
    let irtim = Irtim::new();
    irtim.start_transmission();
    
    let start = cortex_m::peripheral::DWT::get_cycle_count();
    let cycles = duration_us * 16;
    
    while cortex_m::peripheral::DWT::get_cycle_count().wrapping_sub(start) < cycles {}
    
    irtim.stop_transmission();
}
