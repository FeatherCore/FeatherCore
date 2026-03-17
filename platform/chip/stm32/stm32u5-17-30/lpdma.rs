//! LPDMA - Low-Power Direct Memory Access
//! 低功耗直接内存访问控制器
//!
//! # Overview / 概述
//! The Low-Power DMA (LPDMA) is a dedicated DMA controller optimized for
//! low-power modes, allowing data transfers while the system is in sleep mode.
//!
//! # Features / 功能特性
//! - 4 channels for concurrent transfers
//! - Memory-to-memory, peripheral-to-memory, memory-to-peripheral transfers
//! - Linked-list support
//! - Circular mode support
//! - Low-power operation (works in Sleep mode)
//!
//! # Reference / 参考
//! - RM0456 Chapter 19: Low-power DMA (LPDMA)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// LPDMA1 base address / LPDMA1 基地址
/// Reference: RM0456 Chapter 2, Table 1
pub const LPDMA1_BASE: usize = 0x4002_7000;

/// LPDMA Channel
pub struct LpdmaChannel {
    channel: u8,
}

impl LpdmaChannel {
    pub const fn new(channel: u8) -> Self {
        assert!(channel < 4, "LPDMA channel must be 0-3");
        Self { channel }
    }

    fn ch_base(&self) -> usize {
        LPDMA1_BASE + 0x50 + (self.channel as usize * 0x40)
    }

    pub fn enable(&self) {
        unsafe {
            let cr = (self.ch_base() + 0x0C) as *mut u32;
            let mut val = read_volatile(cr);
            val |= 1 << 0;
            write_volatile(cr, val);
        }
    }

    pub fn disable(&self) {
        unsafe {
            let cr = (self.ch_base() + 0x0C) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !(1 << 0);
            write_volatile(cr, val);
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe {
            let cr = (self.ch_base() + 0x0C) as *const u32;
            (read_volatile(cr) & 1) != 0
        }
    }

    pub fn clear_flags(&self) {
        unsafe {
            let fcr = (self.ch_base() + 0x04) as *mut u32;
            write_volatile(fcr, 0x7F);
        }
    }

    pub fn is_transfer_complete(&self) -> bool {
        unsafe {
            let sr = (self.ch_base() + 0x08) as *const u32;
            (read_volatile(sr) & 1) != 0
        }
    }
}

/// Initialize LPDMA
pub fn init() {
    unsafe {
        let rcc_en = (0x4002_1014 as *mut u32);
        let val = read_volatile(rcc_en);
        write_volatile(rcc_en, val | (1 << 29));
    }
}
