//! OTFDEC - On-the-fly Decryption Engine
//! 在线解密引擎
//!
//! # Overview / 概述
//! STM32U5 On-the-fly Decryption Engine (OTFDEC) provides hardware decryption
//! of external memory data while reading, enabling secure execution from external memory.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 52: On-the-fly decryption engine (OTFDEC)
//!
//! ## Main Features / 主要特性
//! - AES-128 decryption
//! - ECB and CTR modes
//! - Up to 4 regions
//! - Non-secure and secure regions
//! - DMA support
//!
//! # Reference / 参考
//! - RM0456 Chapter 52: On-the-fly decryption engine (OTFDEC)
//! - RM0456 Section 52.1: OTFDEC introduction
//! - RM0456 Section 52.2: OTFDEC main features
//! - RM0456 Section 52.3: OTFDEC functional description
//! - RM0456 Section 52.4: OTFDEC registers

use core::ptr::{read_volatile, write_volatile};

/// OTFDEC base address / OTFDEC 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const OTFDEC_BASE: usize = 0x4002_5000;

/// OTFDEC register offsets / OTFDEC 寄存器偏移
//! Reference: RM0456 Section 52.4: OTFDEC register map
pub mod reg {
    /// OTFDEC configuration register 1
    //! Reference: RM0456 Section 52.4.1: OTFDEC configuration register 1 (OTFDEC_CR1)
    pub const CR1: usize = 0x00;
    /// OTFDEC configuration register 2
    //! Reference: RM0456 Section 52.4.2: OTFDEC configuration register 2 (OTFDEC_CR2)
    pub const CR2: usize = 0x04;
    /// OTFDEC configuration register 3
    //! Reference: RM0456 Section 52.4.3: OTFDEC configuration register 3 (OTFDEC_CR3)
    pub const CR3: usize = 0x08;
    /// OTFDEC configuration register 4
    //! Reference: RM0456 Section 52.4.4: OTFDEC configuration register 4 (OTFDEC_CR4)
    pub const CR4: usize = 0x0C;
    /// OTFDEC region 1 start address register
    //! Reference: RM0456 Section 52.4.5: OTFDEC region 1 start address register (OTFDEC_R1STARTLR)
    pub const R1STARTLR: usize = 0x10;
    /// OTFDEC region 1 end address register
    //! Reference: RM0456 Section 52.4.6: OTFDEC region 1 end address register (OTFDEC_R1ENDXR)
    pub const R1ENDXR: usize = 0x14;
    /// OTFDEC region 2 start address register
    pub const R2STARTXR: usize = 0x18;
    /// OTFDEC region 2 end address register
    pub const R2ENDXR: usize = 0x1C;
    /// OTFDEC region 3 start address register
    pub const R3STARTXR: usize = 0x20;
    /// OTFDEC region 3 end address register
    pub const R3ENDXR: usize = 0x24;
    /// OTFDEC region 4 start address register
    pub const R4STARTXR: usize = 0x28;
    /// OTFDEC region 4 end address register
    pub const R4ENDXR: usize = 0x2C;
    /// OTFDEC key register 0
    //! Reference: RM0456 Section 52.4.13: OTFDEC key register (OTFDEC_KEYR)
    pub const KEYR: usize = 0x40;
    /// OTFDEC initialization vector register
    pub const IVR: usize = 0x44;
    /// OTFDEC status register
    //! Reference: RM0456 Section 52.4.14: OTFDEC status register (OTFDEC_SR)
    pub const SR: usize = 0x50;
    /// OTFDEC interrupt enable register
    //! Reference: RM0456 Section 52.4.15: OTFDEC interrupt enable register (OTFDEC_IER)
    pub const IER: usize = 0x54;
}

/// OTFDEC instance
pub struct Otfdec;

impl Otfdec {
    /// Create OTFDEC instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable OTFDEC
    pub fn enable(&self) {
        unsafe {
            let cr1 = (OTFDEC_BASE + reg::CR1) as *mut u32;
            write_volatile(cr1, 1 << 0);
        }
    }

    /// Disable OTFDEC
    pub fn disable(&self) {
        unsafe {
            let cr1 = (OTFDEC_BASE + reg::CR1) as *mut u32;
            write_volatile(cr1, 0);
        }
    }

    /// Enable region 1 decryption
    pub fn enable_region1(&self) {
        unsafe {
            let cr1 = (OTFDEC_BASE + reg::CR1) as *mut u32;
            let val = read_volatile(cr1);
            write_volatile(cr1, val | (1 << 8));
        }
    }

    /// Disable region 1 decryption
    pub fn disable_region1(&self) {
        unsafe {
            let cr1 = (OTFDEC_BASE + reg::CR1) as *mut u32;
            let val = read_volatile(cr1);
            write_volatile(cr1, val & !(1 << 8));
        }
    }

    /// Configure region 1
    pub fn config_region1(&self, start_addr: u32, end_addr: u32) {
        unsafe {
            let r1startlr = (OTFDEC_BASE + reg::R1STARTLR) as *mut u32;
            write_volatile(r1startlr, start_addr);
            let r1endxr = (OTFDEC_BASE + reg::R1ENDXR) as *mut u32;
            write_volatile(r1endxr, end_addr);
        }
    }

    /// Write key
    pub fn write_key(&self, key: u32) {
        unsafe {
            let keyr = (OTFDEC_BASE + reg::KEYR) as *mut u32;
            write_volatile(keyr, key);
        }
    }

    /// Write initialization vector
    pub fn write_iv(&self, iv: u32) {
        unsafe {
            let ivr = (OTFDEC_BASE + reg::IVR) as *mut u32;
            write_volatile(ivr, iv);
        }
    }

    /// Get status
    pub fn status(&self) -> u32 {
        unsafe {
            let sr = (OTFDEC_BASE + reg::SR) as *const u32;
            read_volatile(sr)
        }
    }
}

impl Default for Otfdec {
    fn default() -> Self {
        Self::new()
    }
}
