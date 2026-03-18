//! DBG - Debug
//! 调试
//!
//! # Overview / 概述
//! STM32U5 Debug (DBG) module provides debug and trace capabilities,
//! including clock control for debug in low power modes.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 75: Debug support (DBG)
//! 
//! ## Debug Features / 调试特性
//! - Debug MCU configuration
//! - Clock gating control in low power modes
//! - APB1, APB2, AHB peripherals debug control
//! 
//! ## Low Power Debug / 低功耗调试
//! - Debug in Stop mode
//! - Debug in Standby mode
//! - Peripheral clock gating during debug
//! 
//! # Reference / 参考
//! - RM0456 Chapter 75: Debug support (DBG)
//! - RM0456 Section 75.1: DBG introduction
//! - RM0456 Section 75.2: DBG main features
//! - RM0456 Section 75.3: DBG functional description
//! - RM0456 Section 75.5: DBG registers

/// DBG base address / DBG 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const DBG_BASE: usize = 0xE004_2000;

/// DBG register offsets / DBG 寄存器偏移
//! Reference: RM0456 Section 75.5: DBG register map
pub mod reg {
    /// DBGMCU IDCODE register / DBGMCU IDCODE 寄存器
    //! Reference: RM0456 Section 75.5.1: DBGMCU_IDCODE
    pub const IDCODE: usize = 0x00;
    /// DBGMCU configuration register / DBGMCU 配置寄存器
    //! Reference: RM0456 Section 75.5.2: DBGMCU_CR
    pub const CR: usize = 0x04;
    /// DBGMCU APB1 freeze register 1 / DBGMCU APB1 冻结寄存器 1
    //! Reference: RM0456 Section 75.5.3: DBGMCU_APB1FZR1
    pub const APB1FZR1: usize = 0x08;
    /// DBGMCU APB1 freeze register 2 / DBGMCU APB1 冻结寄存器 2
    //! Reference: RM0456 Section 75.5.4: DBGMCU_APB1FZR2
    pub const APB1FZR2: usize = 0x0C;
    /// DBGMCU APB2 freeze register / DBGMCU APB2 冻结寄存器
    //! Reference: RM0456 Section 75.5.5: DBGMCU_APB2FZR
    pub const APB2FZR: usize = 0x10;
    /// DBGMCU AHB1 freeze register / DBGMCU AHB1 冻结寄存器
    pub const AHB1FZR: usize = 0x14;
    /// DBGMCU AHB3 freeze register / DBGMCU AHB3 冻结寄存器
    pub const AHB3FZR: usize = 0x18;
    /// DBGMCU AHB2 freeze register / DBGMCU AHB2 冻结寄存器
    pub const AHB2FZR: usize = 0x20;
}

/// DBG instance / DBG 实例
pub struct Dbg;

impl Dbg {
    /// Create DBG instance / 创建 DBG 实例
    pub const fn new() -> Self {
        Self
    }

    /// Get device ID / 获取设备 ID
    pub fn get_device_id(&self) -> u32 {
        unsafe {
            let idcode = (DBG_BASE + reg::IDCODE) as *const u32;
            core::ptr::read_volatile(idcode)
        }
    }

    /// Get revision ID / 获取修订版本 ID
    pub fn get_revision_id(&self) -> u16 {
        unsafe {
            let idcode = (DBG_BASE + reg::IDCODE) as *const u32;
            ((core::ptr::read_volatile(idcode) >> 16) & 0xFFFF) as u16
        }
    }

    /// Get device ID code / 获取设备 ID 代码
    pub fn get_dev_id(&self) -> u16 {
        unsafe {
            let idcode = (DBG_BASE + reg::IDCODE) as *const u32;
            (core::ptr::read_volatile(idcode) & 0x0FFF) as u16
        }
    }

    /// Enable debug in Stop mode / 在停止模式下使能调试
    pub fn enable_stop_mode_debug(&self) {
        unsafe {
            let cr = (DBG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 1;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable debug in Stop mode / 在停止模式下禁用调试
    pub fn disable_stop_mode_debug(&self) {
        unsafe {
            let cr = (DBG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 1);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable debug in Standby mode / 在待机模式下使能调试
    pub fn enable_standby_mode_debug(&self) {
        unsafe {
            let cr = (DBG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 2;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable debug in Standby mode / 在待机模式下禁用调试
    pub fn disable_standby_mode_debug(&self) {
        unsafe {
            let cr = (DBG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 2);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable trace pin assignment / 使能跟踪引脚分配
    pub fn enable_trace(&self) {
        unsafe {
            let cr = (DBG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 5;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable trace pin assignment / 禁用跟踪引脚分配
    pub fn disable_trace(&self) {
        unsafe {
            let cr = (DBG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 5);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Freeze APB1 peripheral 1 during debug / 在调试期间冻结 APB1 外设 1
    pub fn freeze_apb1_periph1(&self, bit: u8) {
        unsafe {
            let apb1fzr1 = (DBG_BASE + reg::APB1FZR1) as *mut u32;
            let mut val = core::ptr::read_volatile(apb1fzr1);
            val |= 1 << bit;
            core::ptr::write_volatile(apb1fzr1, val);
        }
    }

    /// Unfreeze APB1 peripheral 1 during debug / 在调试期间解冻 APB1 外设 1
    pub fn unfreeze_apb1_periph1(&self, bit: u8) {
        unsafe {
            let apb1fzr1 = (DBG_BASE + reg::APB1FZR1) as *mut u32;
            let mut val = core::ptr::read_volatile(apb1fzr1);
            val &= !(1 << bit);
            core::ptr::write_volatile(apb1fzr1, val);
        }
    }

    /// Freeze APB1 peripheral 2 during debug / 在调试期间冻结 APB1 外设 2
    pub fn freeze_apb1_periph2(&self, bit: u8) {
        unsafe {
            let apb1fzr2 = (DBG_BASE + reg::APB1FZR2) as *mut u32;
            let mut val = core::ptr::read_volatile(apb1fzr2);
            val |= 1 << bit;
            core::ptr::write_volatile(apb1fzr2, val);
        }
    }

    /// Unfreeze APB1 peripheral 2 during debug / 在调试期间解冻 APB1 外设 2
    pub fn unfreeze_apb1_periph2(&self, bit: u8) {
        unsafe {
            let apb1fzr2 = (DBG_BASE + reg::APB1FZR2) as *mut u32;
            let mut val = core::ptr::read_volatile(apb1fzr2);
            val &= !(1 << bit);
            core::ptr::write_volatile(apb1fzr2, val);
        }
    }

    /// Freeze APB2 peripheral during debug / 在调试期间冻结 APB2 外设
    pub fn freeze_apb2_periph(&self, bit: u8) {
        unsafe {
            let apb2fzr = (DBG_BASE + reg::APB2FZR) as *mut u32;
            let mut val = core::ptr::read_volatile(apb2fzr);
            val |= 1 << bit;
            core::ptr::write_volatile(apb2fzr, val);
        }
    }

    /// Unfreeze APB2 peripheral during debug / 在调试期间解冻 APB2 外设
    pub fn unfreeze_apb2_periph(&self, bit: u8) {
        unsafe {
            let apb2fzr = (DBG_BASE + reg::APB2FZR) as *mut u32;
            let mut val = core::ptr::read_volatile(apb2fzr);
            val &= !(1 << bit);
            core::ptr::write_volatile(apb2fzr, val);
        }
    }
}

/// Initialize DBG with default configuration / 使用默认配置初始化 DBG
pub fn init_dbg_default() {
    let dbg = Dbg::new();
    dbg.enable_stop_mode_debug();
}
