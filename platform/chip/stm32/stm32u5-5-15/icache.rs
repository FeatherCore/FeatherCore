//! ICACHE - Instruction Cache
//! 指令缓存
//!
//! # Overview / 概述
//! STM32U5 Instruction Cache (ICACHE) improves execution performance by caching
//! instructions from external memory.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 8: Instruction cache (ICACHE)
//!
//! ## Cache Features / 缓存特性
//! - 8 KB instruction cache
//! - 4-way set associative
//! - 32-byte cache line
//!
//! ## Operation Modes / 工作模式
//! - Independent mode
//! - CPU cache mode
//!
//! # Reference / 参考
//! - RM0456 Chapter 8: Instruction cache (ICACHE)
//! - RM0456 Section 8.1: ICACHE introduction
//! - RM0456 Section 8.2: ICACHE main features
//! - RM0456 Section 8.3: ICACHE functional description
//! - RM0456 Section 8.4: ICACHE registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// ICACHE base address / ICACHE 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const ICACHE_BASE: usize = 0x4002_3400;

/// ICACHE register offsets / ICACHE 寄存器偏移
//! Reference: RM0456 Section 8.4: ICACHE register map
pub mod reg {
    /// ICACHE control register
    //! Reference: RM0456 Section 8.4.1: ICACHE control register (ICACHE_CR)
    pub const CR: usize = 0x00;
    /// ICACHE status register
    //! Reference: RM0456 Section 8.4.2: ICACHE status register (ICACHE_SR)
    pub const SR: usize = 0x04;
    /// ICACHE interrupt enable register
    //! Reference: RM0456 Section 8.4.3: ICACHE interrupt enable register (ICACHE_IER)
    pub const IER: usize = 0x08;
    /// ICACHE clear flag register
    //! Reference: RM0456 Section 8.4.4: ICACHE clear flag register (ICACHE_CCR)
    pub const CCR: usize = 0x0C;
    /// ICACHE monitor control register
    //! Reference: RM0456 Section 8.4.5: ICACHE monitor control register (ICACHE_MCR)
    pub const MCR: usize = 0x10;
    /// ICACHE monitor data register
    //! Reference: RM0456 Section 8.4.6: ICACHE monitor data register (ICACHE_MDR)
    pub const MDR: usize = 0x14;
}

/// ICACHE Control Register bits
/// Reference: RM0456 Section 8.4.1
pub mod cr_bits {
    /// ICACHE enable
    pub const EN: u32 = 1 << 0;
    /// ICACHE mode
    pub const MODE: u32 = 1 << 1;
}

/// ICACHE Status Register bits
/// Reference: RM0456 Section 8.4.2
pub mod sr_bits {
    /// ICACHE busy
    pub const BUSY: u32 = 1 << 0;
    /// ICACHE miss
    pub const MISS: u32 = 1 << 1;
    /// ICACHE hit
    pub const HIT: u32 = 1 << 2;
    /// End of operation flag
    pub const EOPF: u32 = 1 << 3;
}

/// ICACHE Interrupt Enable Register bits
/// Reference: RM0456 Section 8.4.3
pub mod ier_bits {
    /// End of operation interrupt enable
    pub const EOPIE: u32 = 1 << 0;
    /// Error interrupt enable
    pub const ERRIE: u32 = 1 << 1;
}

/// ICACHE Monitor Control Register bits
/// Reference: RM0456 Section 8.4.5
pub mod mcr_bits {
    /// Monitor enable
    pub const MON_EN: u32 = 1 << 0;
    /// Monitor reset
    pub const MON_RST: u32 = 1 << 1;
    /// Cache hit counter reset
    pub const HIT_RST: u32 = 1 << 2;
    /// Cache miss counter reset
    pub const MISS_RST: u32 = 1 << 3;
}

/// ICACHE operation mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IcacheMode {
    /// Independent mode
    Independent = 0,
    /// CPU cache mode
    CpuCache = 1,
}

/// ICACHE instance
pub struct Icache;

impl Icache {
    /// Create ICACHE instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable ICACHE
    pub fn enable(&self) {
        unsafe {
            let cr = (ICACHE_BASE + reg::CR) as *mut u32;
            write_volatile(cr, cr_bits::EN);
        }
    }

    /// Disable ICACHE
    pub fn disable(&self) {
        unsafe {
            let cr = (ICACHE_BASE + reg::CR) as *mut u32;
            write_volatile(cr, 0);
        }
    }

    /// Set ICACHE mode
    pub fn set_mode(&self, mode: IcacheMode) {
        unsafe {
            let cr = (ICACHE_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, (val & !cr_bits::MODE) | ((mode as u32) << 1));
        }
    }

    /// Get ICACHE mode
    pub fn get_mode(&self) -> IcacheMode {
        unsafe {
            let cr = (ICACHE_BASE + reg::CR) as *const u32;
            if (read_volatile(cr) & cr_bits::MODE) != 0 {
                IcacheMode::CpuCache
            } else {
                IcacheMode::Independent
            }
        }
    }

    /// Invalidate all cache
    pub fn invalidate_all(&self) {
        unsafe {
            let ccr = (ICACHE_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 1);
        }
    }

    /// Get status register
    pub fn status(&self) -> u32 {
        unsafe {
            let sr = (ICACHE_BASE + reg::SR) as *const u32;
            read_volatile(sr)
        }
    }

    /// Check if ICACHE is busy
    pub fn is_busy(&self) -> bool {
        (self.status() & sr_bits::BUSY) != 0
    }

    /// Check if last operation was a miss
    pub fn is_miss(&self) -> bool {
        (self.status() & sr_bits::MISS) != 0
    }

    /// Check if last operation was a hit
    pub fn is_hit(&self) -> bool {
        (self.status() & sr_bits::HIT) != 0
    }

    /// Get end of operation flag
    pub fn is_eop(&self) -> bool {
        (self.status() & sr_bits::EOPF) != 0
    }

    /// Clear end of operation flag
    pub fn clear_eop_flag(&self) {
        unsafe {
            let ccr = (ICACHE_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 1 << 1);
        }
    }

    /// Enable end of operation interrupt
    pub fn enable_eop_interrupt(&self) {
        unsafe {
            let ier = (ICACHE_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::EOPIE);
        }
    }

    /// Enable error interrupt
    pub fn enable_error_interrupt(&self) {
        unsafe {
            let ier = (ICACHE_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::ERRIE);
        }
    }

    /// Disable interrupts
    pub fn disable_interrupts(&self) {
        unsafe {
            let ier = (ICACHE_BASE + reg::IER) as *mut u32;
            write_volatile(ier, 0);
        }
    }

    /// Enable cache hit/miss monitor
    pub fn enable_monitor(&self) {
        unsafe {
            let mcr = (ICACHE_BASE + reg::MCR) as *mut u32;
            write_volatile(mcr, mcr_bits::MON_EN);
        }
    }

    /// Disable cache hit/miss monitor
    pub fn disable_monitor(&self) {
        unsafe {
            let mcr = (ICACHE_BASE + reg::MCR) as *mut u32;
            write_volatile(mcr, 0);
        }
    }

    /// Reset monitor counters
    pub fn reset_monitor(&self) {
        unsafe {
            let mcr = (ICACHE_BASE + reg::MCR) as *mut u32;
            write_volatile(mcr, mcr_bits::MON_RST);
        }
    }

    /// Get cache hit count
    pub fn get_hit_count(&self) -> u32 {
        unsafe {
            let mdr = (ICACHE_BASE + reg::MDR) as *const u32;
            read_volatile(mdr) & 0xFFFF
        }
    }

    /// Get cache miss count
    pub fn get_miss_count(&self) -> u32 {
        unsafe {
            let mdr = (ICACHE_BASE + reg::MDR) as *const u32;
            (read_volatile(mdr) >> 16) & 0xFFFF
        }
    }
}

impl Default for Icache {
    fn default() -> Self {
        Self::new()
    }
}
