//! GFXMMU - Graphics Memory Management Unit
//! 图形内存管理单元
//!
//! ## STM32U5 GFXMMU 特性 / Features
//! - **LUT (Look-Up Table):**
//!   - 256-entry palette for color lookup
//!   - 24-bit RGB format per entry
//!
//! - **缓冲区管理 / Buffer Management:**
//!   - 多缓冲区支持
//!   - 内存映射模式
//!   - FCM (Flash Control Memory) 保护
//!
//! - **特性 / Features:**
//!   - LCD/TFT 显示控制器接口
//!   - DSI 接口
//!   - JPEG 解码器接口
//!   - DMA2D 接口
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 29: Graphics memory management unit (GFXMMU)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// GFXMMU base address / GFXMMU 基地址
pub const GFXMMU_BASE: usize = 0x5100_0000;

/// GFXMMU register offsets / GFXMMU 寄存器偏移
//! Reference: RM0456 Section 29.4: GFXMMU register map
pub mod reg {
    /// GFXMMU control register
    //! Reference: RM0456 Section 29.4.1: GFXMMU control register (GFXMMU_CR)
    pub const CR: usize = 0x00;
    /// GFXMMU status register
    //! Reference: RM0456 Section 29.4.2: GFXMMU status register (GFXMMU_SR)
    pub const SR: usize = 0x04;
    /// GFXMMU interrupt enable register
    //! Reference: RM0456 Section 29.4.3: GFXMMU interrupt enable register (GFXMMU_IER)
    pub const IER: usize = 0x08;
    /// GFXMMU clear flag register
    //! Reference: RM0456 Section 29.4.4: GFXMMU clear flag register (GFXMMU_CFR)
    pub const CFR: usize = 0x0C;
    /// GFXMMU LUT control register
    //! Reference: RM0456 Section 29.4.5: GFXMMU LUT control register (GFXMMU_LUTCR)
    pub const LUTCR: usize = 0x400;
    /// GFXMMU LUT address register
    //! Reference: RM0456 Section 29.4.6: GFXMMU LUT address register (GFXMMU_LUTAR)
    pub const LUTAR: usize = 0x404;
    /// GFXMMU LUT data register
    //! Reference: RM0456 Section 29.4.7: GFXMMU LUT data register (GFXMMU_LUTDR)
    pub const LUTDR: usize = 0x408;
    /// GFXMMU buffer control register
    //! Reference: RM0456 Section 29.4.8: GFXMMU buffer control register (GFXMMU_BCR)
    pub const BCR: usize = 0x800;
    /// GFXMMU buffer 0 address register
    //! Reference: RM0456 Section 29.4.9: GFXMMU buffer 0 address register (GFXMMU_B0AR)
    pub const B0AR: usize = 0x804;
    /// GFXMMU buffer 1 address register
    //! Reference: RM0456 Section 29.4.10: GFXMMU buffer 1 address register (GFXMMU_B1AR)
    pub const B1AR: usize = 0x808;
    /// GFXMMU buffer 2 address register
    //! Reference: RM0456 Section 29.4.11: GFXMMU buffer 2 address register (GFXMMU_B2AR)
    pub const B2AR: usize = 0x80C;
    /// GFXMMU buffer 3 address register
    //! Reference: RM0456 Section 29.4.12: GFXMMU buffer 3 address register (GFXMMU_B3AR)
    pub const B3AR: usize = 0x810;
    /// GFXMMU buffer OVR address register
    //! Reference: RM0456 Section 29.4.13: GFXMMU buffer OVR address register (GFXMMU_BOVR)
    pub const BOVR: usize = 0x814;
    /// GFXMMU buffer watermark register
    //! Reference: RM0456 Section 29.4.14: GFXMMU buffer watermark register (GFXMMU_BWTR)
    pub const BWTR: usize = 0x818;
    /// GFXMMU FCM control register
    //! Reference: RM0456 Section 29.4.15: GFXMMU FCM control register (GFXMMU_FCR)
    pub const FCR: usize = 0x1000;
    /// GFXMMU FCM source address register
    //! Reference: RM0456 Section 29.4.16: GFXMMU FCM source address register (GFXMMU_FSAR)
    pub const FSAR: usize = 0x1004;
    /// GFXMMU FCM destination address register
    //! Reference: RM0456 Section 29.4.17: GFXMMU FCM destination address register (GFXMMU_FDAR)
    pub const FDAR: usize = 0x1008;
    /// GFXMMU FCM size register
    //! Reference: RM0456 Section 29.4.18: GFXMMU FCM size register (GFXMMU_FSR)
    pub const FSR: usize = 0x100C;
}

/// GFXMMU Control Register bits
/// Reference: RM0456 Section 29.4.1
pub mod cr_bits {
    /// GFXMMU enable
    pub const EN: u32 = 1 << 0;
    /// LUT enable
    pub const LUTE: u32 = 1 << 1;
    /// LCD timing enable
    pub const LCDTIMEN: u32 = 1 << 2;
    /// LCD VSYNC
    pub const LCDVS: u32 = 1 << 3;
}

/// GFXMMU Status Register bits
/// Reference: RM0456 Section 29.4.2
pub mod sr_bits {
    /// Buffer busy
    pub const BBUSY: u32 = 1 << 0;
    /// FCM busy
    pub const FCBUSY: u32 = 1 << 1;
    /// LUT busy
    pub const LUTBUSY: u32 = 1 << 2;
    /// Buffer 0 fill status
    pub const BF0: u32 = 1 << 8;
    /// Buffer 1 fill status
    pub const BF1: u32 = 1 << 9;
    /// Buffer 2 fill status
    pub const BF2: u32 = 1 << 10;
    /// Buffer 3 fill status
    pub const BF3: u32 = 1 << 11;
}

/// GFXMMU Interrupt Enable Register bits
/// Reference: RM0456 Section 29.4.3
pub mod ier_bits {
    /// Buffer interrupt enable
    pub const BIE: u32 = 1 << 0;
    /// FCM transfer error interrupt enable
    pub const FTEIE: u32 = 1 << 1;
    /// FCM transfer complete interrupt enable
    pub const FTCIE: u32 = 1 << 2;
    /// LUT transfer interrupt enable
    pub const LUTIE: u32 = 1 << 3;
}

/// GFXMMU LUT Control Register bits
/// Reference: RM0456 Section 29.4.5
pub mod lutcr_bits {
    /// LUT start
    pub const LUTSTART: u32 = 1 << 0;
    /// LUT configuration lock
    pub const LUTCONFIG_LOCK: u32 = 1 << 1;
}

/// GFXMMU Buffer Control Register bits
/// Reference: RM0456 Section 29.4.8
pub mod bcr_bits {
    /// Buffer 0 enable
    pub const B0E: u32 = 1 << 0;
    /// Buffer 1 enable
    pub const B1E: u32 = 1 << 1;
    /// Buffer 2 enable
    pub const B2E: u32 = 1 << 2;
    /// Buffer 3 enable
    pub const B3E: u32 = 1 << 3;
    /// Buffer 0 write permission
    pub const B0WP: u32 = 1 << 8;
    /// Buffer 1 write permission
    pub const B1WP: u32 = 1 << 9;
    /// Buffer 2 write permission
    pub const B2WP: u32 = 1 << 10;
    /// Buffer 3 write permission
    pub const B3WP: u32 = 1 << 11;
    /// Overlay buffer enable
    pub const BOVRE: u32 = 1 << 12;
    /// Overlay write permission
    pub const BOVRWP: u32 = 1 << 13;
    /// Buffer 0 read permission
    pub const B0RP: u32 = 1 << 16;
    /// Buffer 1 read permission
    pub const B1RP: u32 = 1 << 17;
    /// Buffer 2 read permission
    pub const B2RP: u32 = 1 << 18;
    /// Buffer 3 read permission
    pub const B3RP: u32 = 1 << 19;
    /// Overlay read permission
    pub const BOVRRP: u32 = 1 << 20;
}

/// GFXMMU FCM Control Register bits
/// Reference: RM0456 Section 29.4.15
pub mod fcr_bits {
    /// FCM enable
    pub const FCMEN: u32 = 1 << 0;
    /// FCM start
    pub const FCMSTART: u32 = 1 << 1;
    /// FCM suspend
    pub const FCMSUSPEND: u32 = 1 << 2;
    /// FCM clear
    pub const FCMCLEAR: u32 = 1 << 3;
}

/// GFXMMU instance
pub struct Gfxmmu {
    base: usize,
}

impl Gfxmmu {
    /// Create GFXMMU instance
    pub const fn new() -> Self {
        Self { base: GFXMMU_BASE }
    }

    /// Enable GFXMMU
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            write_volatile(cr, cr_bits::EN);
        }
    }

    /// Disable GFXMMU
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            write_volatile(cr, 0);
        }
    }

    /// Enable LUT
    pub fn enable_lut(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::LUTE;
            write_volatile(cr, val);
        }
    }

    /// Disable LUT
    pub fn disable_lut(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::LUTE;
            write_volatile(cr, val);
        }
    }

    /// Enable LCD timing
    pub fn enable_lcd_timing(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::LCDTIMEN;
            write_volatile(cr, val);
        }
    }

    /// Disable LCD timing
    pub fn disable_lcd_timing(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::LCDTIMEN;
            write_volatile(cr, val);
        }
    }

    /// Get status register
    pub fn status(&self) -> u32 {
        unsafe {
            read_volatile((self.base + reg::SR) as *const u32)
        }
    }

    /// Check if buffer is busy
    pub fn is_buffer_busy(&self) -> bool {
        (self.status() & sr_bits::BBUSY) != 0
    }

    /// Check if FCM is busy
    pub fn is_fcm_busy(&self) -> bool {
        (self.status() & sr_bits::FCBUSY) != 0
    }

    /// Check if LUT is busy
    pub fn is_lut_busy(&self) -> bool {
        (self.status() & sr_bits::LUTBUSY) != 0
    }

    /// Enable buffer interrupt
    pub fn enable_buffer_interrupt(&self) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::BIE);
        }
    }

    /// Enable FCM transfer complete interrupt
    pub fn enable_fcm_complete_interrupt(&self) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::FTCIE);
        }
    }

    /// Enable FCM transfer error interrupt
    pub fn enable_fcm_error_interrupt(&self) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::FTEIE);
        }
    }

    /// Enable LUT transfer interrupt
    pub fn enable_lut_interrupt(&self) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            write_volatile(ier, ier_bits::LUTIE);
        }
    }

    /// Disable all interrupts
    pub fn disable_interrupts(&self) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            write_volatile(ier, 0);
        }
    }

    /// Clear buffer interrupt flag
    pub fn clear_buffer_flag(&self) {
        unsafe {
            let cfr = (self.base + reg::CFR) as *mut u32;
            write_volatile(cfr, 1);
        }
    }

    /// Clear FCM complete flag
    pub fn clear_fcm_complete_flag(&self) {
        unsafe {
            let cfr = (self.base + reg::CFR) as *mut u32;
            write_volatile(cfr, 2);
        }
    }

    /// Clear FCM error flag
    pub fn clear_fcm_error_flag(&self) {
        unsafe {
            let cfr = (self.base + reg::CFR) as *mut u32;
            write_volatile(cfr, 4);
        }
    }

    /// Clear LUT flag
    pub fn clear_lut_flag(&self) {
        unsafe {
            let cfr = (self.base + reg::CFR) as *mut u32;
            write_volatile(cfr, 8);
        }
    }

    /// Clear all flags
    pub fn clear_all_flags(&self) {
        unsafe {
            let cfr = (self.base + reg::CFR) as *mut u32;
            write_volatile(cfr, 0xF);
        }
    }

    /// Enable buffer 0
    pub fn enable_buffer0(&self) {
        unsafe {
            let bcr = (self.base + reg::BCR) as *mut u32;
            let mut val = read_volatile(bcr);
            val |= bcr_bits::B0E;
            write_volatile(bcr, val);
        }
    }

    /// Enable buffer 1
    pub fn enable_buffer1(&self) {
        unsafe {
            let bcr = (self.base + reg::BCR) as *mut u32;
            let mut val = read_volatile(bcr);
            val |= bcr_bits::B1E;
            write_volatile(bcr, val);
        }
    }

    /// Enable buffer 2
    pub fn enable_buffer2(&self) {
        unsafe {
            let bcr = (self.base + reg::BCR) as *mut u32;
            let mut val = read_volatile(bcr);
            val |= bcr_bits::B2E;
            write_volatile(bcr, val);
        }
    }

    /// Enable buffer 3
    pub fn enable_buffer3(&self) {
        unsafe {
            let bcr = (self.base + reg::BCR) as *mut u32;
            let mut val = read_volatile(bcr);
            val |= bcr_bits::B3E;
            write_volatile(bcr, val);
        }
    }

    /// Set buffer 0 address
    pub fn set_buffer0_address(&self, addr: u32) {
        unsafe {
            let b0ar = (self.base + reg::B0AR) as *mut u32;
            write_volatile(b0ar, addr);
        }
    }

    /// Set buffer 1 address
    pub fn set_buffer1_address(&self, addr: u32) {
        unsafe {
            let b1ar = (self.base + reg::B1AR) as *mut u32;
            write_volatile(b1ar, addr);
        }
    }

    /// Set buffer 2 address
    pub fn set_buffer2_address(&self, addr: u32) {
        unsafe {
            let b2ar = (self.base + reg::B2AR) as *mut u32;
            write_volatile(b2ar, addr);
        }
    }

    /// Set buffer 3 address
    pub fn set_buffer3_address(&self, addr: u32) {
        unsafe {
            let b3ar = (self.base + reg::B3AR) as *mut u32;
            write_volatile(b3ar, addr);
        }
    }

    /// Set overlay buffer address
    pub fn set_overlay_address(&self, addr: u32) {
        unsafe {
            let bovr = (self.base + reg::BOVR) as *mut u32;
            write_volatile(bovr, addr);
        }
    }

    /// Enable overlay buffer
    pub fn enable_overlay(&self) {
        unsafe {
            let bcr = (self.base + reg::BCR) as *mut u32;
            let mut val = read_volatile(bcr);
            val |= bcr_bits::BOVRE;
            write_volatile(bcr, val);
        }
    }

    /// Disable overlay buffer
    pub fn disable_overlay(&self) {
        unsafe {
            let bcr = (self.base + reg::BCR) as *mut u32;
            let mut val = read_volatile(bcr);
            val &= !bcr_bits::BOVRE;
            write_volatile(bcr, val);
        }
    }

    /// Set watermark for buffer interrupt
    pub fn set_watermark(&self, watermark: u32) {
        unsafe {
            let bwtr = (self.base + reg::BWTR) as *mut u32;
            write_volatile(bwtr, watermark);
        }
    }

    /// Configure LUT entry
    pub fn configure_lut_entry(&self, index: u8, red: u8, green: u8, blue: u8) {
        unsafe {
            let lutcr = (self.base + reg::LUTCR) as *mut u32;
            write_volatile(lutcr, lutcr_bits::LUTSTART | (index as u32));

            let lutdr = (self.base + reg::LUTDR) as *mut u32;
            let value = ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32);
            write_volatile(lutdr, value);
        }
    }

    /// Start LUT transfer
    pub fn start_lut(&self) {
        unsafe {
            let lutcr = (self.base + reg::LUTCR) as *mut u32;
            write_volatile(lutcr, lutcr_bits::LUTSTART);
        }
    }

    /// Enable FCM
    pub fn enable_fcm(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, fcr_bits::FCMEN);
        }
    }

    /// Disable FCM
    pub fn disable_fcm(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, 0);
        }
    }

    /// Set FCM source address
    pub fn set_fcm_source(&self, addr: u32) {
        unsafe {
            let fsar = (self.base + reg::FSAR) as *mut u32;
            write_volatile(fsar, addr);
        }
    }

    /// Set FCM destination address
    pub fn set_fcm_destination(&self, addr: u32) {
        unsafe {
            let fdar = (self.base + reg::FDAR) as *mut u32;
            write_volatile(fdar, addr);
        }
    }

    /// Set FCM transfer size
    pub fn set_fcm_size(&self, size: u32) {
        unsafe {
            let fsr = (self.base + reg::FSR) as *mut u32;
            write_volatile(fsr, size);
        }
    }

    /// Start FCM transfer
    pub fn start_fcm(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, fcr_bits::FCMEN | fcr_bits::FCMSTART);
        }
    }

    /// Suspend FCM transfer
    pub fn suspend_fcm(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, fcr_bits::FCMEN | fcr_bits::FCMSUSPEND);
        }
    }

    /// Clear FCM
    pub fn clear_fcm(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, fcr_bits::FCMCLEAR);
        }
    }
}
