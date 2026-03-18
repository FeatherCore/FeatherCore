//! OCTOSPIM - OctoSPI Manager
//! OctoSPI管理器
//!
//! ## STM32U5 OCTOSPIM 特性 / Features
//! - **端口管理 / Port Management:**
//!   - OCTOSPI1 端口管理
//!   - OCTOSPI2 端口管理
//!
//! - **时钟管理 / Clock Management:**
//!   - 独立的时钟预分频器
//!   - 时钟门控控制
//!
//! - **特性 / Features:**
//!   - I/O Manager (IOM)
//!   - 简化OctoSPI配置
//!   - 延迟链管理
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 29: OctoSPI manager (OCTOSPIM)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// OCTOSPIM base address / OCTOSPIM 基地址
pub const OCTOSPIM_BASE: usize = 0x4202_1000;

/// OCTOSPIM register offsets / OCTOSPIM 寄存器偏移
//! Reference: RM0456 Section 29.4: OCTOSPIM register map
pub mod reg {
    /// OCTOSPIM control register
    //! Reference: RM0456 Section 29.4.1: OCTOSPIM control register (OCTOSPIM_CR)
    pub const CR: usize = 0x00;
    /// OCTOSPIM IO manager control register
    //! Reference: RM0456 Section 29.4.2: OCTOSPIM IO manager control register (OCTOSPIM_PCR)
    pub const PCR: usize = 0x04;
    /// OCTOSPIM IO manager status register
    //! Reference: RM0456 Section 29.4.3: OCTOSPIM IO manager status register (OCTOSPIM_PCR)
    pub const SR: usize = 0x08;
    /// OCTOSPIM flag clear register
    //! Reference: RM0456 Section 29.4.4: OCTOSPIM flag clear register (OCTOSPIM_FCR)
    pub const FCR: usize = 0x0C;
}

/// OCTOSPIM Control Register bits
/// Reference: RM0456 Section 29.4.1
pub mod cr_bits {
    /// OCTOSPIM enable
    pub const EN: u32 = 1 << 0;
    /// I/O manager enable
    pub const IOMEN: u32 = 1 << 1;
    /// Calibration start
    pub const CALSTART: u32 = 1 << 2;
    /// Calibration reset
    pub const CALRST: u32 = 1 << 3;
    /// DLL enable
    pub const DLLEN: u32 = 1 << 8;
    /// DLL reset
    pub const DLLRST: u32 = 1 << 9;
}

/// OCTOSPIM IO Manager Control Register bits
/// Reference: RM0456 Section 29.4.2
pub mod pcr_bits {
    /// OCTOSPI1 enable
    pub const OCTOSPI1EN: u32 = 1 << 0;
    /// OCTOSPI2 enable
    pub const OCTOSPI2EN: u32 = 1 << 1;
    /// OCTOSPI1 clock enable
    pub const OCTOSPI1CLKEN: u32 = 1 << 8;
    /// OCTOSPI2 clock enable
    pub const OCTOSPI2CLKEN: u32 = 1 << 9;
    /// OCTOSPI1 IOSPEEDN optimization
    pub const OCTOSPI1IOSPEEDN: u32 = 1 << 16;
    /// OCTOSPI2 IOSPEEDN optimization
    pub const OCTOSPI2IOSPEEDN: u32 = 1 << 17;
    /// OCTOSPI1 I/O delay
    pub const OCTOSPI1DLY: u32 = 0xF << 24;
    /// OCTOSPI2 I/O delay
    pub const OCTOSPI2DLY: u32 = 0xF << 28;
}

/// OCTOSPIM Status Register bits
/// Reference: RM0456 Section 29.4.3
pub mod sr_bits {
    /// OCTOSPI1 busy
    pub const OCTOSPI1BSY: u32 = 1 << 0;
    /// OCTOSPI2 busy
    pub const OCTOSPI2BSY: u32 = 1 << 1;
    /// Calibration busy
    pub const CALBSY: u32 = 1 << 8;
    /// Calibration done
    pub const CALDONE: u32 = 1 << 9;
    /// DLL ready
    pub const DLLRDY: u32 = 1 << 16;
}

/// OCTOSPIM instance
pub struct Octospim {
    base: usize,
}

impl Octospim {
    /// Create OCTOSPIM instance
    pub const fn new() -> Self {
        Self { base: OCTOSPIM_BASE }
    }

    /// Enable OCTOSPIM
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            write_volatile(cr, cr_bits::EN);
        }
    }

    /// Disable OCTOSPIM
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            write_volatile(cr, 0);
        }
    }

    /// Enable I/O Manager
    pub fn enable_io_manager(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::IOMEN;
            write_volatile(cr, val);
        }
    }

    /// Disable I/O Manager
    pub fn disable_io_manager(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::IOMEN;
            write_volatile(cr, val);
        }
    }

    /// Enable OCTOSPI1
    pub fn enable_octospi1(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val |= pcr_bits::OCTOSPI1EN;
            write_volatile(pcr, val);
        }
    }

    /// Disable OCTOSPI1
    pub fn disable_octospi1(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI1EN;
            write_volatile(pcr, val);
        }
    }

    /// Enable OCTOSPI2
    pub fn enable_octospi2(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val |= pcr_bits::OCTOSPI2EN;
            write_volatile(pcr, val);
        }
    }

    /// Disable OCTOSPI2
    pub fn disable_octospi2(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI2EN;
            write_volatile(pcr, val);
        }
    }

    /// Enable OCTOSPI1 clock
    pub fn enable_octospi1_clock(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val |= pcr_bits::OCTOSPI1CLKEN;
            write_volatile(pcr, val);
        }
    }

    /// Disable OCTOSPI1 clock
    pub fn disable_octospi1_clock(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI1CLKEN;
            write_volatile(pcr, val);
        }
    }

    /// Enable OCTOSPI2 clock
    pub fn enable_octospi2_clock(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val |= pcr_bits::OCTOSPI2CLKEN;
            write_volatile(pcr, val);
        }
    }

    /// Disable OCTOSPI2 clock
    pub fn disable_octospi2_clock(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI2CLKEN;
            write_volatile(pcr, val);
        }
    }

    /// Set OCTOSPI1 I/O delay
    pub fn set_octospi1_delay(&self, delay: u8) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI1DLY;
            val |= ((delay & 0xF) as u32) << 24;
            write_volatile(pcr, val);
        }
    }

    /// Set OCTOSPI2 I/O delay
    pub fn set_octospi2_delay(&self, delay: u8) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI2DLY;
            val |= ((delay & 0xF) as u32) << 28;
            write_volatile(pcr, val);
        }
    }

    /// Enable OCTOSPI1 I/O speed optimization
    pub fn enable_octospi1_speed_opt(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val |= pcr_bits::OCTOSPI1IOSPEEDN;
            write_volatile(pcr, val);
        }
    }

    /// Disable OCTOSPI1 I/O speed optimization
    pub fn disable_octospi1_speed_opt(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI1IOSPEEDN;
            write_volatile(pcr, val);
        }
    }

    /// Enable OCTOSPI2 I/O speed optimization
    pub fn enable_octospi2_speed_opt(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val |= pcr_bits::OCTOSPI2IOSPEEDN;
            write_volatile(pcr, val);
        }
    }

    /// Disable OCTOSPI2 I/O speed optimization
    pub fn disable_octospi2_speed_opt(&self) {
        unsafe {
            let pcr = (self.base + reg::PCR) as *mut u32;
            let mut val = read_volatile(pcr);
            val &= !pcr_bits::OCTOSPI2IOSPEEDN;
            write_volatile(pcr, val);
        }
    }

    /// Enable DLL
    pub fn enable_dll(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::DLLEN;
            write_volatile(cr, val);
        }
    }

    /// Disable DLL
    pub fn disable_dll(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::DLLEN;
            write_volatile(cr, val);
        }
    }

    /// Reset DLL
    pub fn reset_dll(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::DLLRST;
            write_volatile(cr, val);
        }
    }

    /// Start calibration
    pub fn start_calibration(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::CALSTART;
            write_volatile(cr, val);
        }
    }

    /// Reset calibration
    pub fn reset_calibration(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::CALRST;
            write_volatile(cr, val);
        }
    }

    /// Get status register
    pub fn status(&self) -> u32 {
        unsafe {
            read_volatile((self.base + reg::SR) as *const u32)
        }
    }

    /// Check if OCTOSPI1 is busy
    pub fn is_octospi1_busy(&self) -> bool {
        (self.status() & sr_bits::OCTOSPI1BSY) != 0
    }

    /// Check if OCTOSPI2 is busy
    pub fn is_octospi2_busy(&self) -> bool {
        (self.status() & sr_bits::OCTOSPI2BSY) != 0
    }

    /// Check if calibration is busy
    pub fn is_calibration_busy(&self) -> bool {
        (self.status() & sr_bits::CALBSY) != 0
    }

    /// Check if calibration is done
    pub fn is_calibration_done(&self) -> bool {
        (self.status() & sr_bits::CALDONE) != 0
    }

    /// Check if DLL is ready
    pub fn is_dll_ready(&self) -> bool {
        (self.status() & sr_bits::DLLRDY) != 0
    }

    /// Clear all flags
    pub fn clear_flags(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, 0xFF);
        }
    }
}
