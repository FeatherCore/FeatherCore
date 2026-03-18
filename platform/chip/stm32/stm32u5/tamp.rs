//! TAMP - Tamper
//! 篡改检测
//!
//! # Overview / 概述
//! STM32U5 Tamper (TAMP) controller provides tamper detection, backup registers,
//! and monotonic counters for security applications.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 64: Tamper and backup registers (TAMP)
//! 
//! ## Tamper Detection / 篡改检测
//! - 8 tamper inputs with configurable sensitivity
//! - Internal tamper events
//! - Tamper interrupt and reset generation
//! 
//! ## Backup Registers / 备份寄存器
//! - 32 backup registers (32-bit each)
//! - Retained in VBAT mode
//! 
//! ## Monotonic Counters / 单调计数器
//! - 2 monotonic counters
//! - Increment-only operation
//! 
//! ## Advanced Features / 高级特性
//! - Active tamper protection
//! - Internal/external tamper events
//! - Tamper timestamping
//! - Erase backup registers on tamper
//! 
//! # Reference / 参考
//! - RM0456 Chapter 64: Tamper and backup registers (TAMP)
//! - RM0456 Section 64.1: TAMP introduction
//! - RM0456 Section 64.2: TAMP main features
//! - RM0456 Section 64.3: TAMP functional description
//! - RM0456 Section 64.6: TAMP registers

/// TAMP base address / TAMP 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const TAMP_BASE: usize = 0x4200_0400;

/// TAMP register offsets / TAMP 寄存器偏移
//! Reference: RM0456 Section 64.6: TAMP register map
pub mod reg {
    /// TAMP control register 1
    //! Reference: RM0456 Section 64.6.1: TAMP control register 1 (TAMP_CR1)
    pub const CR1: usize = 0x00;
    /// TAMP control register 2
    //! Reference: RM0456 Section 64.6.2: TAMP control register 2 (TAMP_CR2)
    pub const CR2: usize = 0x04;
    /// TAMP filter control register
    //! Reference: RM0456 Section 64.6.3: TAMP filter control register (TAMP_FLTCR)
    pub const FLTCR: usize = 0x08;
    /// TAMP active tamper control register
    //! Reference: RM0456 Section 64.6.4: TAMP active tamper control register (TAMP_ATCR1)
    pub const ATCR1: usize = 0x0C;
    /// TAMP active tamper seed register
    pub const ATSEEDR: usize = 0x10;
    /// TAMP active tamper seed register MSB
    pub const ATOR: usize = 0x14;
    /// TAMP status register
    //! Reference: RM0456 Section 64.6.5: TAMP status register (TAMP_SR)
    pub const SR: usize = 0x20;
    /// TAMP masked interrupt status register
    pub const MISR: usize = 0x24;
    /// TAMP status clear register
    //! Reference: RM0456 Section 64.6.6: TAMP status clear register (TAMP_SCR)
    pub const SCR: usize = 0x2C;
    /// TAMP monotonic counter register 1
    //! Reference: RM0456 Section 64.6.7: TAMP monotonic counter register 1 (TAMP_COUNTR1)
    pub const COUNTR1: usize = 0x30;
    /// TAMP monotonic counter register 2
    pub const COUNTR2: usize = 0x34;
    /// TAMP backup register x
    pub const BKP0R: usize = 0x100;
    pub const BKP1R: usize = 0x104;
    pub const BKP2R: usize = 0x108;
    pub const BKP3R: usize = 0x10C;
    pub const BKP4R: usize = 0x110;
    pub const BKP5R: usize = 0x114;
    pub const BKP6R: usize = 0x118;
    pub const BKP7R: usize = 0x11C;
    pub const BKP8R: usize = 0x120;
    pub const BKP9R: usize = 0x124;
    pub const BKP10R: usize = 0x128;
    pub const BKP11R: usize = 0x12C;
    pub const BKP12R: usize = 0x130;
    pub const BKP13R: usize = 0x134;
    pub const BKP14R: usize = 0x138;
    pub const BKP15R: usize = 0x13C;
    pub const BKP16R: usize = 0x140;
    pub const BKP17R: usize = 0x144;
    pub const BKP18R: usize = 0x148;
    pub const BKP19R: usize = 0x14C;
    pub const BKP20R: usize = 0x150;
    pub const BKP21R: usize = 0x154;
    pub const BKP22R: usize = 0x158;
    pub const BKP23R: usize = 0x15C;
    pub const BKP24R: usize = 0x160;
    pub const BKP25R: usize = 0x164;
    pub const BKP26R: usize = 0x168;
    pub const BKP27R: usize = 0x16C;
    pub const BKP28R: usize = 0x170;
    pub const BKP29R: usize = 0x174;
    pub const BKP30R: usize = 0x178;
    pub const BKP31R: usize = 0x17C;
}

/// Tamper input selection
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TamperInput {
    Tamper1 = 0,
    Tamper2 = 1,
    Tamper3 = 2,
    Tamper4 = 3,
    Tamper5 = 4,
    Tamper6 = 5,
    Tamper7 = 6,
    Tamper8 = 7,
}

/// Tamper trigger
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TamperTrigger {
    RisingEdge = 0,
    FallingEdge = 1,
    LowLevel = 2,
    HighLevel = 3,
}

/// TAMP instance
pub struct Tamp;

impl Tamp {
    /// Create TAMP instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable TAMP clock
    pub fn enable_clock(&self) {
        crate::rcc::enable_apb1_clock(crate::rcc::apb1::TAMP);
    }

    /// Disable TAMP clock
    pub fn disable_clock(&self) {
        crate::rcc::disable_apb1_clock(crate::rcc::apb1::TAMP);
    }

    /// Enable tamper detection
    pub fn enable_tamper(&self, input: TamperInput, trigger: TamperTrigger) {
        unsafe {
            let cr1 = (TAMP_BASE + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            
            let input_bit = input as u8;
            val |= 1 << input_bit;
            
            core::ptr::write_volatile(cr1, val);

            let cr2 = (TAMP_BASE + reg::CR2) as *mut u32;
            let mut val2 = core::ptr::read_volatile(cr2);
            
            let trigger_val = (trigger as u32) << (input_bit * 2);
            val2 &= !(0b11 << (input_bit * 2));
            val2 |= trigger_val;
            
            core::ptr::write_volatile(cr2, val2);
        }
    }

    /// Disable tamper detection
    pub fn disable_tamper(&self, input: TamperInput) {
        unsafe {
            let cr1 = (TAMP_BASE + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val &= !(1 << (input as u8));
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Check if tamper event occurred
    pub fn is_tamper_detected(&self, input: TamperInput) -> bool {
        unsafe {
            let sr = (TAMP_BASE + reg::SR) as *const u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << (input as u8))) != 0
        }
    }

    /// Clear tamper event flag
    pub fn clear_tamper_flag(&self, input: TamperInput) {
        unsafe {
            let scr = (TAMP_BASE + reg::SCR) as *mut u32;
            core::ptr::write_volatile(scr, 1 << (input as u8));
        }
    }

    /// Read backup register
    pub fn read_backup(&self, index: u8) -> u32 {
        assert!(index < 32, "Backup register index must be 0-31");
        unsafe {
            let reg = (TAMP_BASE + reg::BKP0R + (index as usize * 4)) as *const u32;
            core::ptr::read_volatile(reg)
        }
    }

    /// Write backup register
    pub fn write_backup(&self, index: u8, value: u32) {
        assert!(index < 32, "Backup register index must be 0-31");
        unsafe {
            let reg = (TAMP_BASE + reg::BKP0R + (index as usize * 4)) as *mut u32;
            core::ptr::write_volatile(reg, value);
        }
    }

    /// Read monotonic counter 1
    pub fn read_counter1(&self) -> u32 {
        unsafe {
            let reg = (TAMP_BASE + reg::COUNTR1) as *const u32;
            core::ptr::read_volatile(reg)
        }
    }

    /// Read monotonic counter 2
    pub fn read_counter2(&self) -> u32 {
        unsafe {
            let reg = (TAMP_BASE + reg::COUNTR2) as *const u32;
            core::ptr::read_volatile(reg)
        }
    }

    /// Increment monotonic counter 1
    pub fn increment_counter1(&self) {
        unsafe {
            let cr1 = (TAMP_BASE + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= 1 << 15;
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Increment monotonic counter 2
    pub fn increment_counter2(&self) {
        unsafe {
            let cr1 = (TAMP_BASE + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= 1 << 16;
            core::ptr::write_volatile(cr1, val);
        }
    }
}

/// Initialize TAMP with default configuration
pub fn init_tamp_default() {
    let tamp = Tamp::new();
    tamp.enable_clock();
}
