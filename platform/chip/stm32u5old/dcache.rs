//! DCACHE - Data Cache
//! 数据缓存
//!
//! # Overview / 概述
//! STM32U5 Data Cache (DCACHE) improves data access performance by caching
//! data from external memory.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 9: Data cache (DCACHE)
//!
//! ## Cache Features / 缓存特性
//! - 4 KB data cache
//! - 4-way set associative
//! - 32-byte cache line
//!
//! ## Operation Modes / 工作模式
//! - Write-through mode
//! - Write-back mode
//!
//! # Reference / 参考
//! - RM0456 Chapter 9: Data cache (DCACHE)
//! - RM0456 Section 9.1: DCACHE introduction
//! - RM0456 Section 9.2: DCACHE main features
//! - RM0456 Section 9.3: DCACHE functional description
//! - RM0456 Section 9.4: DCACHE registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// DCACHE base address / DCACHE 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const DCACHE_BASE: usize = 0x4002_3500;

/// DCACHE register offsets / DCACHE 寄存器偏移
//! Reference: RM0456 Section 9.4: DCACHE register map
pub mod reg {
    /// DCACHE control register
    //! Reference: RM0456 Section 9.4.1: DCACHE control register (DCACHE_CR)
    pub const CR: usize = 0x00;
    /// DCACHE status register
    //! Reference: RM0456 Section 9.4.2: DCACHE status register (DCACHE_SR)
    pub const SR: usize = 0x04;
    /// DCACHE interrupt enable register
    //! Reference: RM0456 Section 9.4.3: DCACHE interrupt enable register (DCACHE_IER)
    pub const IER: usize = 0x08;
    /// DCACHE clear flag register
    //! Reference: RM0456 Section 9.4.4: DCACHE clear flag register (DCACHE_CCR)
    pub const CCR: usize = 0x0C;
    /// DCACHE monitor control register
    //! Reference: RM0456 Section 9.4.5: DCACHE monitor control register (DCACHE_MCR)
    pub const MCR: usize = 0x10;
    /// DCACHE monitor data register
    //! Reference: RM0456 Section 9.4.6: DCACHE monitor data register (DCACHE_MDR)
    pub const MDR: usize = 0x14;
}

/// DCACHE Control Register bits
/// Reference: RM0456 Section 9.4.1
pub mod cr_bits {
    /// DCACHE enable
    pub const EN: u32 = 1 << 0;
    /// Write mode (0: write-through, 1: write-back)
    pub const WRITE_MODE: u32 = 1 << 1;
    /// Force write-through
    pub const FORCE_WT: u32 = 1 << 2;
}

/// DCACHE Status Register bits
/// Reference: RM0456 Section 9.4.2
pub mod sr_bits {
    /// DCACHE busy
    pub const BUSY: u32 = 1 << 0;
    /// DCACHE miss
    pub const MISS: u32 = 1 << 1;
    /// DCACHE hit
    pub const HIT: u32 = 1 << 2;
    /// End of operation flag
    pub const EOPF: u32 = 1 << 3;
    /// Write buffer busy
    pub const WBUSY: u32 = 1 << 4;
}

/// DCACHE Interrupt Enable Register bits
/// Reference: RM0456 Section 9.4.3
pub mod ier_bits {
    /// End of operation interrupt enable
    pub const EOPIE: u32 = 1 << 0;
    /// Error interrupt enable
    pub const ERRIE: u32 = 1 << 1;
}

/// DCACHE Monitor Control Register bits
/// Reference: RM0456 Section 9.4.5
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

/// DCACHE write mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteMode {
    /// Write-through mode
    WriteThrough = 0,
    /// Write-back mode
    WriteBack = 1,
}

/// DCACHE instance
pub struct Dcache;

impl Dcache {
    /// Create DCACHE instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable DCACHE
    pub fn enable(&self) {
        unsafe {
            let cr = (DCACHE_BASE + reg::CR) as *mut u32;
            write_volatile(cr, cr_bits::EN);
        }
    }

    /// Disable DCACHE
    pub fn disable(&self) {
        unsafe {
            let cr = (DCACHE_BASE + reg::CR) as *mut u32;
            write_volatile(cr, 0);
        }
    }

    /// Set write mode
    pub fn set_write_mode(&self, mode: WriteMode) {
        unsafe {
            let cr = (DCACHE_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, (val & !cr_bits::WRITE_MODE) | ((mode as u32) << 1));
        }
    }

    /// Get write mode
    pub fn get_write_mode(&self) -> WriteMode {
        unsafe {
            let cr = (DCACHE_BASE + reg::CR) as *const u32;
            if (read_volatile(cr) & cr_bits::WRITE_MODE) != 0 {
                WriteMode::WriteBack
            } else {
                WriteMode::WriteThrough
            }
        }
    }

    /// Force write-through mode
    pub fn force_write_through(&self) {
        unsafe {
            let cr = (DCACHE_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, val | cr_bits::FORCE_WT);
        }
    }

    /// Disable force write-through
    pub fn disable_force_write_through(&self) {
        unsafe {
            let cr = (DCACHE_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, val & !cr_bits::FORCE_WT);
        }
    }

    /// Invalidate all cache
    pub fn invalidate_all(&self) {
        unsafe {
            let ccr = (DCACHE_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 1);
        }
    }

    /// Clean all cache (write-back data to memory)
    pub fn clean_all(&self) {
        unsafe {
            let ccr = (DCACHE_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 2);
        }
    }

    /// Clean and invalidate all cache
    pub fn clean_invalidate_all(&self) {
        unsafe {
            let ccr = (DCACHE_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 3);
        }
    }

    /// Get status register
    pub fn status(&self) -> u32 {
        unsafe {
            let sr = (DCACHE_BASE + reg::SR) as *const u32;
            read_volatile(sr)
        }
    }

    /// Check if DCACHE is busy
    pub fn is_busy(&self) -> bool {
        (self.status() & sr_bits::BUSY) != 0
    }

    /// Check if write buffer is busy
    pub fn is_write_buffer_busy(&self) -> bool {
        (self.status() & sr_bits::WBUSY) != 0
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
            let ccr = (DCACHE_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 1 << 1);
        }
    }

    /// Enable end of operation interrupt
    pub fn enable_eop_interrupt(&self) {
        unsafe {
            let ier = (DCACHE_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::EOPIE);
        }
    }

    /// Enable error interrupt
    pub fn enable_error_interrupt(&self) {
        unsafe {
            let ier = (DCACHE_BASE + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::ERRIE);
        }
    }

    /// Disable interrupts
    pub fn disable_interrupts(&self) {
        unsafe {
            let ier = (DCACHE_BASE + reg::IER) as *mut u32;
            write_volatile(ier, 0);
        }
    }

    /// Enable cache hit/miss monitor
    pub fn enable_monitor(&self) {
        unsafe {
            let mcr = (DCACHE_BASE + reg::MCR) as *mut u32;
            write_volatile(mcr, mcr_bits::MON_EN);
        }
    }

    /// Disable cache hit/miss monitor
    pub fn disable_monitor(&self) {
        unsafe {
            let mcr = (DCACHE_BASE + reg::MCR) as *mut u32;
            write_volatile(mcr, 0);
        }
    }

    /// Reset monitor counters
    pub fn reset_monitor(&self) {
        unsafe {
            let mcr = (DCACHE_BASE + reg::MCR) as *mut u32;
            write_volatile(mcr, mcr_bits::MON_RST);
        }
    }

    /// Get cache hit count
    pub fn get_hit_count(&self) -> u32 {
        unsafe {
            let mdr = (DCACHE_BASE + reg::MDR) as *const u32;
            read_volatile(mdr) & 0xFFFF
        }
    }

    /// Get cache miss count
    pub fn get_miss_count(&self) -> u32 {
        unsafe {
            let mdr = (DCACHE_BASE + reg::MDR) as *const u32;
            (read_volatile(mdr) >> 16) & 0xFFFF
        }
    }
}

impl Default for Dcache {
    fn default() -> Self {
        Self::new()
    }
}
