//! CRS - Clock Recovery System
//! 时钟恢复系统
//!
//! # Overview / 概述
//! STM32U5 Clock Recovery System (CRS) provides automatic trimming of the internal
//! RC oscillator using an external synchronization signal.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 12: Clock recovery system (CRS)
//!
//! ## Synchronization Sources / 同步源
//! - USB SOF (Start of Frame)
//! - External signal on CRS_SYNC pin
//!
//! ## Trimming Capabilities / 校准能力
//! - Automatic HSI48 trimming
//! - Frequency error counter
//! - Programmable reload value
//!
//! # Reference / 参考
//! - RM0456 Chapter 12: Clock recovery system (CRS)
//! - RM0456 Section 12.1: CRS introduction
//! - RM0456 Section 12.2: CRS main features
//! - RM0456 Section 12.3: CRS functional description
//! - RM0456 Section 12.4: CRS registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// CRS base address / CRS 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const CRS_BASE: usize = 0x4002_1000;

/// CRS register offsets / CRS 寄存器偏移
//! Reference: RM0456 Section 12.4: CRS register map
pub mod reg {
    /// CRS control register
    //! Reference: RM0456 Section 12.4.1: CRS control register (CRS_CR)
    pub const CR: usize = 0x00;
    /// CRS configuration register
    //! Reference: RM0456 Section 12.4.2: CRS configuration register (CRS_CFGR)
    pub const CFGR: usize = 0x04;
    /// CRS interrupt enable register
    //! Reference: RM0456 Section 12.4.3: CRS interrupt enable register (CRS_IER)
    pub const IER: usize = 0x08;
    /// CRS interrupt flag clear register
    //! Reference: RM0456 Section 12.4.4: CRS interrupt flag clear register (CRS_ICR)
    pub const ICR: usize = 0x0C;
    /// CRS status register
    //! Reference: RM0456 Section 12.4.5: CRS status register (CRS_ISR)
    pub const ISR: usize = 0x10;
    /// CRS reload value register
    //! Reference: RM0456 Section 12.4.6: CRS reload value register (CRS_ReloadValueR)
    pub const RELOAD: usize = 0x14;
    /// CRS HSI48 calibration value register
    //! Reference: RM0456 Section 12.4.7: CRS HSI48 calibration value register (CRS_HSI48CalibrationR)
    pub const HSI48CAL: usize = 0x18;
}

/// CRS Control Register bits
/// Reference: RM0456 Section 12.4.1
pub mod cr_bits {
    /// CRS enable
    pub const EN: u32 = 1 << 0;
    /// Automatic trimming enable
    pub const AUTOTRIMEN: u32 = 1 << 1;
    /// Frequency error counter enable
    pub const CEN: u32 = 1 << 2;
    /// HSI48 oscillator trim update by software
    pub const HSI48TRIM: u32 = 1 << 3;
    /// HSI48 oscillator smooth trimming
    pub const SMOOTHTING: u32 = 1 << 4;
    /// HSI48 calibration on reset
    pub const HSI48CAL: u32 = 1 << 5;
}

/// CRS Configuration Register bits
/// Reference: RM0456 Section 12.4.2
pub mod cfgr_bits {
    /// Synchronization source selection
    pub const SYNCSRC: u32 = 0b11 << 0;
    /// Synchronization polarity
    pub const SYNCPOL: u32 = 1 << 2;
    /// Sync event selection
    pub const SYNCEVENT: u32 = 1 << 3;
    /// Frequency error counter division ratio
    pub const FEDIV: u32 = 0b111 << 5;
    /// Reload value
    pub const RELOAD: u32 = 0xFFFF << 16;
}

/// CRS Interrupt Enable Register bits
/// Reference: RM0456 Section 12.4.3
pub mod ier_bits {
    /// Expected sync interrupt enable
    pub const ESYNCIE: u32 = 1 << 0;
    /// Sync error interrupt enable
    pub const SYNCERRIE: u32 = 1 << 1;
    /// Sync OK interrupt enable
    pub const SYNCOKIE: u32 = 1 << 2;
    /// Frequency error limit interrupt enable
    pub const FEDIE: u32 = 1 << 3;
    /// Trim error interrupt enable
    pub const TRIMERRORIE: u32 = 1 << 4;
}

/// CRS Status/Flag Register bits
/// Reference: RM0456 Section 12.4.5
pub mod isr_bits {
    /// Expected sync flag
    pub const ESYNCF: u32 = 1 << 0;
    /// Sync error flag
    pub const SYNCERRF: u32 = 1 << 1;
    /// Sync OK flag
    pub const SYNCOKF: u32 = 1 << 2;
    /// Frequency error data valid
    pub const FEDAV: u32 = 1 << 3;
    /// Frequency error direction
    pub const FEDDIR: u32 = 1 << 4;
    /// Frequency error counter value
    pub const FED: u32 = 0x3FF << 16;
}

/// CRS synchronization source
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncSource {
    /// USB SOF (Start of Frame)
    USB = 0,
    /// External CRS_SYNC pin
    External = 1,
    /// Reserved
    Reserved = 2,
}

/// CRS sync polarity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncPolarity {
    /// Rising edge
    Rising = 0,
    /// Falling edge
    Falling = 1,
}

/// CRS instance
pub struct Crs;

impl Crs {
    /// Create CRS instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable CRS
    pub fn enable(&self) {
        unsafe {
            let cr = (CRS_BASE + reg::CR) as *mut u32;
            write_volatile(cr, cr_bits::EN);
        }
    }

    /// Disable CRS
    pub fn disable(&self) {
        unsafe {
            let cr = (CRS_BASE + reg::CR) as *mut u32;
            write_volatile(cr, 0);
        }
    }

    /// Enable automatic trimming
    pub fn enable_autotrim(&self) {
        unsafe {
            let cr = (CRS_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, val | cr_bits::AUTOTRIMEN);
        }
    }

    /// Disable automatic trimming
    pub fn disable_autotrim(&self) {
        unsafe {
            let cr = (CRS_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, val & !cr_bits::AUTOTRIMEN);
        }
    }

    /// Enable frequency error counter
    pub fn enable_freq_error_counter(&self) {
        unsafe {
            let cr = (CRS_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, val | cr_bits::CEN);
        }
    }

    /// Disable frequency error counter
    pub fn disable_freq_error_counter(&self) {
        unsafe {
            let cr = (CRS_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, val & !cr_bits::CEN);
        }
    }

    /// Set sync source
    pub fn set_sync_source(&self, source: SyncSource) {
        unsafe {
            let cfgr = (CRS_BASE + reg::CFGR) as *mut u32;
            let val = read_volatile(cfgr);
            write_volatile(cfgr, (val & !cfgr_bits::SYNCSRC) | ((source as u32) << 0));
        }
    }

    /// Set sync polarity
    pub fn set_sync_polarity(&self, polarity: SyncPolarity) {
        unsafe {
            let cfgr = (CRS_BASE + reg::CFGR) as *mut u32;
            let val = read_volatile(cfgr);
            write_volatile(cfgr, (val & !cfgr_bits::SYNCPOL) | ((polarity as u32) << 2));
        }
    }

    /// Set frequency error counter division ratio
    pub fn set_freq_error_div(&self, div: u8) {
        unsafe {
            let cfgr = (CRS_BASE + reg::CFGR) as *mut u32;
            let val = read_volatile(cfgr);
            write_volatile(cfgr, (val & !cfgr_bits::FEDIV) | (((div & 0x7) as u32) << 5));
        }
    }

    /// Set reload value for auto-trim
    pub fn set_reload_value(&self, reload: u16) {
        unsafe {
            let reload_reg = (CRS_BASE + reg::RELOAD) as *mut u32;
            write_volatile(reload_reg, reload as u32);
        }
    }

    /// Set HSI48 calibration value
    pub fn set_hsi48_calibration(&self, cal: u8) {
        unsafe {
            let cal_reg = (CRS_BASE + reg::HSI48CAL) as *mut u32;
            write_volatile(cal_reg, cal as u32);
        }
    }

    /// Get status register
    pub fn status(&self) -> u32 {
        unsafe {
            let isr = (CRS_BASE + reg::ISR) as *const u32;
            read_volatile(isr)
        }
    }

    /// Check if expected sync flag is set
    pub fn is_expected_sync(&self) -> bool {
        (self.status() & isr_bits::ESYNCF) != 0
    }

    /// Check if sync error occurred
    pub fn has_sync_error(&self) -> bool {
        (self.status() & isr_bits::SYNCERRF) != 0
    }

    /// Check if sync OK flag is set
    pub fn is_sync_ok(&self) -> bool {
        (self.status() & isr_bits::SYNCOKF) != 0
    }

    /// Check if frequency error data is available
    pub fn is_freq_error_valid(&self) -> bool {
        (self.status() & isr_bits::FEDAV) != 0
    }

    /// Get frequency error direction (true = too fast, false = too slow)
    pub fn get_freq_error_dir(&self) -> bool {
        (self.status() & isr_bits::FEDDIR) != 0
    }

    /// Get frequency error counter value
    pub fn get_freq_error_count(&self) -> u16 {
        ((self.status() >> 16) & 0x3FF) as u16
    }

    /// Clear expected sync flag
    pub fn clear_expected_sync_flag(&self) {
        unsafe {
            let icr = (CRS_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, isr_bits::ESYNCF);
        }
    }

    /// Clear sync error flag
    pub fn clear_sync_error_flag(&self) {
        unsafe {
            let icr = (CRS_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, isr_bits::SYNCERRF);
        }
    }

    /// Clear sync OK flag
    pub fn clear_sync_ok_flag(&self) {
        unsafe {
            let icr = (CRS_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, isr_bits::SYNCOKF);
        }
    }

    /// Clear all interrupt flags
    pub fn clear_all_flags(&self) {
        unsafe {
            let icr = (CRS_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, 0x1F);
        }
    }

    /// Enable expected sync interrupt
    pub fn enable_expected_sync_interrupt(&self) {
        unsafe {
            let ier = (CRS_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::ESYNCIE);
        }
    }

    /// Enable sync error interrupt
    pub fn enable_sync_error_interrupt(&self) {
        unsafe {
            let ier = (CRS_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::SYNCERRIE);
        }
    }

    /// Enable sync OK interrupt
    pub fn enable_sync_ok_interrupt(&self) {
        unsafe {
            let ier = (CRS_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::SYNCOKIE);
        }
    }

    /// Enable frequency error interrupt
    pub fn enable_freq_error_interrupt(&self) {
        unsafe {
            let ier = (CRS_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::FEDIE);
        }
    }

    /// Disable all interrupts
    pub fn disable_interrupts(&self) {
        unsafe {
            let ier = (CRS_BASE + reg::IER) as *mut u32;
            write_volatile(ier, 0);
        }
    }
}

impl Default for Crs {
    fn default() -> Self {
        Self::new()
    }
}
