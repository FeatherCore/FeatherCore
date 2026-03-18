//! LPGPIO - Low-Power General Purpose I/O
//! 低功耗通用输入输出
//!
//! # Overview / 概述
//! STM32U5 Low-Power GPIO (LPGPIO) provides GPIO functionality in low-power modes
//! allowing I/O operations while the main system is in low-power state.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 14: Low-power GPIO (LPGPIO)
//!
//! ## Main Features / 主要特性
//! - 8 LPGPIO pins (LPGPIO0-LPGPIO7)
//! - Independent control in low-power modes
//! - Interrupt capability
//! - Wakeup from low-power modes
//!
//! # Reference / 参考
//! - RM0456 Chapter 14: Low-power GPIO (LPGPIO)
//! - RM0456 Section 14.1: LPGPIO introduction
//! - RM0456 Section 14.2: LPGPIO main features
//! - RM0456 Section 14.3: LPGPIO functional description
//! - RM0456 Section 14.4: LPGPIO registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// LPGPIO base address / LPGPIO 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const LPGPIO_BASE: usize = 0x4002_4000;

/// LPGPIO register offsets / LPGPIO 寄存器偏移
//! Reference: RM0456 Section 14.4: LPGPIO register map
pub mod reg {
    /// LPGPIO mode register
    //! Reference: RM0456 Section 14.4.1: LPGPIO mode register (LPGPIO_MODER)
    pub const MODER: usize = 0x00;
    /// LPGPIO output type register
    //! Reference: RM0456 Section 14.4.2: LPGPIO output type register (LPGPIO_OTYPER)
    pub const OTYPER: usize = 0x04;
    /// LPGPIO output speed register
    //! Reference: RM0456 Section 14.4.3: LPGPIO output speed register (LPGPIO_OSPEEDR)
    pub const OSPEEDR: usize = 0x08;
    /// LPGPIO pull-up/pull-down register
    //! Reference: RM0456 Section 14.4.4: LPGPIO pull-up/pull-down register (LPGPIO_PUPDR)
    pub const PUPDR: usize = 0x0C;
    /// LPGPIO input data register
    //! Reference: RM0456 Section 14.4.5: LPGPIO input data register (LPGPIO_IDR)
    pub const IDR: usize = 0x10;
    /// LPGPIO output data register
    //! Reference: RM0456 Section 14.4.6: LPGPIO output data register (LPGPIO_ODR)
    pub const ODR: usize = 0x14;
    /// LPGPIO bit set/reset register
    //! Reference: RM0456 Section 14.4.7: LPGPIO bit set/reset register (LPGPIO_BSRR)
    pub const BSRR: usize = 0x18;
    /// LPGPIO configuration register
    //! Reference: RM0456 Section 14.4.8: LPGPIO configuration register (LPGPIO_LCKR)
    pub const LCKR: usize = 0x1C;
    /// LPGPIO alternate function register
    //! Reference: RM0456 Section 14.4.9: LPGPIO alternate function register (LPGPIO_AFRL)
    pub const AFRL: usize = 0x20;
    /// LPGPIO alternate function high register
    //! Reference: RM0456 Section 14.4.10: LPGPIO alternate function high register (LPGPIO_AFRH)
    pub const AFRH: usize = 0x24;
    /// LPGPIO wakeup interrupt enable register
    //! Reference: RM0456 Section 14.4.11: LPGPIO wakeup interrupt enable register (LPGPIO_WAKEUPENR)
    pub const WAKEUPENR: usize = 0x30;
    /// LPGPIO event control register
    //! Reference: RM0456 Section 14.4.12: LPGPIO event control register (LPGPIO_EVCR)
    pub const EVCR: usize = 0x40;
    /// LPGPIO edge event register
    //! Reference: RM0456 Section 14.4.13: LPGPIO edge event register (LPGPIO_EVR)
    pub const EVR: usize = 0x44;
}

/// LPGPIO Mode Register bits
pub mod moder_bits {
    pub const MODE_INPUT: u32 = 0b00;
    pub const MODE_OUTPUT: u32 = 0b01;
    pub const MODE_AF: u32 = 0b10;
    pub const MODE_ANALOG: u32 = 0b11;
}

/// LPGPIO Output Type Register bits
pub mod otyper_bits {
    pub const PUSHPULL: u32 = 0;
    pub const OPENDRAIN: u32 = 1;
}

/// LPGPIO Pull-up/Pull-down Register bits
pub mod pupdr_bits {
    pub const NONE: u32 = 0b00;
    pub const UP: u32 = 0b01;
    pub const DOWN: u32 = 0b10;
}

/// LPGPIO Wakeup Enable Register bits
pub mod wakeupenr_bits {
    pub const WAKEUP0: u32 = 1 << 0;
    pub const WAKEUP1: u32 = 1 << 1;
    pub const WAKEUP2: u32 = 1 << 2;
    pub const WAKEUP3: u32 = 1 << 3;
    pub const WAKEUP4: u32 = 1 << 4;
    pub const WAKEUP5: u32 = 1 << 5;
    pub const WAKEUP6: u32 = 1 << 6;
    pub const WAKEUP7: u32 = 1 << 7;
}

/// LPGPIO Event Control Register bits
pub mod evcr_bits {
    pub const EVT_EN: u32 = 1 << 0;
    pub const EVT_EDGE: u32 = 1 << 1;
}

/// LPGPIO Edge Event Register bits
pub mod evr_bits {
    pub const EVT0: u32 = 1 << 0;
    pub const EVT1: u32 = 1 << 1;
    pub const EVT2: u32 = 1 << 2;
    pub const EVT3: u32 = 1 << 3;
    pub const EVT4: u32 = 1 << 4;
    pub const EVT5: u32 = 1 << 5;
    pub const EVT6: u32 = 1 << 6;
    pub const EVT7: u32 = 1 << 7;
}

/// LPGPIO pin mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PinMode {
    /// Input mode
    Input = 0,
    /// General purpose output mode
    Output = 1,
    /// Alternate function mode
    Alternate = 2,
    /// Analog mode
    Analog = 3,
}

/// LPGPIO instance
pub struct Lpgpio;

impl Lpgpio {
    /// Create LPGPIO instance
    pub const fn new() -> Self {
        Self
    }

    /// Set pin mode
    pub fn set_mode(&self, pin: u8, mode: PinMode) {
        if pin > 7 {
            return;
        }
        unsafe {
            let moder = (LPGPIO_BASE + reg::MODER) as *mut u32;
            let val = read_volatile(moder);
            let val = val & !(0x3 << (pin * 2));
            write_volatile(moder, val | ((mode as u32) << (pin * 2)));
        }
    }

    /// Set output type
    pub fn set_output_type(&self, pin: u8, open_drain: bool) {
        if pin > 7 {
            return;
        }
        unsafe {
            let otyper = (LPGPIO_BASE + reg::OTYPER) as *mut u32;
            let val = read_volatile(otyper);
            if open_drain {
                write_volatile(otyper, val | (1 << pin));
            } else {
                write_volatile(otyper, val & !(1 << pin));
            }
        }
    }

    /// Set output speed
    pub fn set_speed(&self, pin: u8, speed: u8) {
        if pin > 7 || speed > 3 {
            return;
        }
        unsafe {
            let ospeedr = (LPGPIO_BASE + reg::OSPEEDR) as *mut u32;
            let val = read_volatile(ospeedr);
            write_volatile(ospeedr, val & !(0x3 << (pin * 2)) | ((speed as u32) << (pin * 2)));
        }
    }

    /// Set pull-up/pull-down
    pub fn set_pull(&self, pin: u8, pull: u8) {
        if pin > 7 || pull > 2 {
            return;
        }
        unsafe {
            let pupdr = (LPGPIO_BASE + reg::PUPDR) as *mut u32;
            let val = read_volatile(pupdr);
            write_volatile(pupdr, val & !(0x3 << (pin * 2)) | ((pull as u32) << (pin * 2)));
        }
    }

    /// Set alternate function
    pub fn set_alternate_function(&self, pin: u8, af: u8) {
        if pin > 7 || af > 15 {
            return;
        }
        unsafe {
            if pin < 4 {
                let afrl = (LPGPIO_BASE + reg::AFRL) as *mut u32;
                let val = read_volatile(afrl);
                write_volatile(afrl, val & !(0xF << (pin * 4)) | ((af as u32) << (pin * 4)));
            } else {
                let afrh = (LPGPIO_BASE + reg::AFRH) as *mut u32;
                let val = read_volatile(afrh);
                write_volatile(afrh, val & !(0xF << ((pin - 4) * 4)) | ((af as u32) << ((pin - 4) * 4)));
            }
        }
    }

    /// Write output data
    pub fn write_pin(&self, pin: u8, high: bool) {
        if pin > 7 {
            return;
        }
        unsafe {
            let odr = (LPGPIO_BASE + reg::ODR) as *mut u32;
            let val = read_volatile(odr);
            let val = if high { val | (1 << pin) } else { val & !(1 << pin) };
            write_volatile(odr, val);
        }
    }

    /// Set pin high using BSRR
    pub fn set_high(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let bsrr = (LPGPIO_BASE + reg::BSRR) as *mut u32;
            write_volatile(bsrr, 1 << pin);
        }
    }

    /// Reset pin low using BSRR
    pub fn set_low(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let bsrr = (LPGPIO_BASE + reg::BSRR) as *mut u32;
            write_volatile(bsrr, 1 << (pin + 16));
        }
    }

    /// Toggle pin
    pub fn toggle(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let odr = (LPGPIO_BASE + reg::ODR) as *mut u32;
            let val = read_volatile(odr);
            write_volatile(odr, val ^ (1 << pin));
        }
    }

    /// Read input data
    pub fn read_pin(&self, pin: u8) -> bool {
        if pin > 7 {
            return false;
        }
        unsafe {
            let idr = (LPGPIO_BASE + reg::IDR) as *const u32;
            read_volatile(idr) & (1 << pin) != 0
        }
    }

    /// Read output data
    pub fn read_output(&self, pin: u8) -> bool {
        if pin > 7 {
            return false;
        }
        unsafe {
            let odr = (LPGPIO_BASE + reg::ODR) as *const u32;
            read_volatile(odr) & (1 << pin) != 0
        }
    }

    /// Enable wakeup interrupt for pin
    pub fn enable_wakeup(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let wakeupenr = (LPGPIO_BASE + reg::WAKEUPENR) as *mut u32;
            let val = read_volatile(wakeupenr);
            write_volatile(wakeupenr, val | (1 << pin));
        }
    }

    /// Disable wakeup interrupt for pin
    pub fn disable_wakeup(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let wakeupenr = (LPGPIO_BASE + reg::WAKEUPENR) as *mut u32;
            let val = read_volatile(wakeupenr);
            write_volatile(wakeupenr, val & !(1 << pin));
        }
    }

    /// Check if pin wakeup is enabled
    pub fn is_wakeup_enabled(&self, pin: u8) -> bool {
        if pin > 7 {
            return false;
        }
        unsafe {
            let wakeupenr = (LPGPIO_BASE + reg::WAKEUPENR) as *const u32;
            read_volatile(wakeupenr) & (1 << pin) != 0
        }
    }

    /// Enable event generation for pin
    pub fn enable_event(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let evcr = (LPGPIO_BASE + reg::EVCR) as *mut u32;
            write_volatile(evcr, evcr_bits::EVT_EN | (pin as u32));
        }
    }

    /// Disable event generation
    pub fn disable_event(&self) {
        unsafe {
            let evcr = (LPGPIO_BASE + reg::EVCR) as *mut u32;
            write_volatile(evcr, 0);
        }
    }

    /// Get edge event register
    pub fn get_edge_events(&self) -> u8 {
        unsafe {
            let evr = (LPGPIO_BASE + reg::EVR) as *const u32;
            (read_volatile(evr) & 0xFF) as u8
        }
    }

    /// Lock pin configuration
    pub fn lock_pin(&self, pin: u8) {
        if pin > 7 {
            return;
        }
        unsafe {
            let lckr = (LPGPIO_BASE + reg::LCKR) as *mut u32;
            write_volatile(lckr, (1 << pin) | 0x10000);
        }
    }

    /// Check if configuration is locked
    pub fn is_locked(&self) -> bool {
        unsafe {
            let lckr = (LPGPIO_BASE + reg::LCKR) as *const u32;
            (read_volatile(lckr) & 0x10000) != 0
        }
    }
}

impl Default for Lpgpio {
    fn default() -> Self {
        Self::new()
    }
}
