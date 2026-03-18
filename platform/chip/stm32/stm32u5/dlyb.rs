//! DLYB - Delay Block
//! 延迟块
//!
//! ## STM32U5 DLYB 特性 / Features
//! - **延迟单元数量 / Delay Cells: 最多 32 个可编程延迟单元
//! - **延迟线长度 / Delay Line Length: 1-31 个延迟单元
//! - **输出选择 / Output Selection: 可选择延迟或旁路
//! - **采样点调整 / Sampling Point Adjustment: 用于数据采样优化
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 32: Delay block (DLYB)
//! - RM0456 Section 32.1: DLYB introduction
//! - RM0456 Section 32.2: DLYB main features
//! - RM0456 Section 32.3: DLYB functional description
//! - RM0456 Section 32.4: DLYB registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// DLYB1 base address / DLYB1 基地址
pub const DLYB1_BASE: usize = 0x4201_6400;
/// DLYB2 base address / DLYB2 基地址
pub const DLYB2_BASE: usize = 0x4201_6800;

/// DLYB register offsets / DLYB 寄存器偏移
//! Reference: RM0456 Section 32.4: DLYB registers
pub mod reg {
    /// DLYB control register / DLYB 控制寄存器
    //! Reference: RM0456 Section 32.4.1: DLYB_CR
    pub const CR: usize = 0x00;
    /// DLYB configuration register / DLYB 配置寄存器
    //! Reference: RM0456 Section 32.4.2: DLYB_CFGR
    pub const CFGR: usize = 0x04;
}

/// CR Register Bit Definitions / CR 寄存器位定义
//! Reference: RM0456 Section 32.4.1
pub mod cr_bits {
    /// Delay line enable / 延迟线使能
    pub const DEN: u32 = 1 << 0;
    /// Delay line bypass / 延迟线旁路
    pub const SEN: u32 = 1 << 1;
}

/// DLYB instance / DLYB 实例
pub struct Dlyb {
    base: usize,
}

impl Dlyb {
    /// Create DLYB1 instance / 创建 DLYB1 实例
    pub const fn dlyb1() -> Self {
        Self { base: DLYB1_BASE }
    }

    /// Create DLYB2 instance / 创建 DLYB2 实例
    pub const fn dlyb2() -> Self {
        Self { base: DLYB2_BASE }
    }

    /// Enable delay line / 使能延迟线
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::DEN;
            write_volatile(cr, val);
        }
    }

    /// Disable delay line / 禁用延迟线
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::DEN;
            write_volatile(cr, val);
        }
    }

    /// Enable bypass mode / 使能旁路模式
    pub fn enable_bypass(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::SEN;
            write_volatile(cr, val);
        }
    }

    /// Disable bypass mode / 禁用旁路模式
    pub fn disable_bypass(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::SEN;
            write_volatile(cr, val);
        }
    }

    /// Configure delay line length / 配置延迟线长度
    //! Reference: RM0456 Section 32.3.1
    /// # Arguments
    /// * `length` - Delay units (1-31) / 延迟单元数 (1-31)
    pub fn set_delay_length(&self, length: u8) {
        unsafe {
            let cfgr = (self.base + reg::CFGR) as *mut u32;
            let mut val = read_volatile(cfgr);
            val &= !(0x1F << 0);
            val |= (length & 0x1F) as u32;
            write_volatile(cfgr, val);
        }
    }

    /// Get current delay length / 获取当前延迟长度
    pub fn get_delay_length(&self) -> u8 {
        unsafe {
            let cfgr = (self.base + reg::CFGR) as *const u32;
            (read_volatile(cfgr) & 0x1F) as u8
        }
    }

    /// Calibrate delay line / 校准延迟线
    //! Reference: RM0456 Section 32.3.2
    pub fn calibrate(&self) -> Result<u8, DlybError> {
        unsafe {
            let cfgr = (self.base + reg::CFGR) as *mut u32;
            
            for length in 1..=31 {
                self.set_delay_length(length);
                
                let mut val = read_volatile(cfgr);
                val |= 1 << 8;
                write_volatile(cfgr, val);
                
                for _ in 0..10000 {
                    core::arch::asm!("nop");
                }
                
                val = read_volatile(cfgr);
                if val & (1 << 9) != 0 {
                    return Ok(length);
                }
            }
            
            Err(DlybError::CalibrationFailed)
        }
    }

    /// Initialize DLYB with optimal delay / 使用最佳延迟初始化 DLYB
    pub fn init_optimal(&self) -> Result<(), DlybError> {
        let optimal_length = self.calibrate()?;
        self.set_delay_length(optimal_length);
        self.enable();
        Ok(())
    }

    /// Check if delay line is locked / 检查延迟线是否锁定
    pub fn is_locked(&self) -> bool {
        unsafe {
            let cfgr = (self.base + reg::CFGR) as *const u32;
            (read_volatile(cfgr) & (1 << 9)) != 0
        }
    }
}

/// DLYB error / DLYB 错误
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DlybError {
    CalibrationFailed,
}

/// Initialize DLYB1 / 初始化 DLYB1
pub fn init_dlyb1() {
    let dlyb = Dlyb::dlyb1();
    dlyb.set_delay_length(16);
    dlyb.enable();
}

/// Initialize DLYB2 / 初始化 DLYB2
pub fn init_dlyb2() {
    let dlyb = Dlyb::dlyb2();
    dlyb.set_delay_length(16);
    dlyb.enable();
}
