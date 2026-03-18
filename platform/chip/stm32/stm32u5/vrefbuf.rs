//! VREFBUF - Voltage Reference Buffer
//! 电压参考缓冲器
//!
//! ## STM32U5 VREFBUF 特性 / Features
//! - **电压输出 / Voltage Outputs:
//!   - 1.5V
//!   - 1.8V
//!   - 2.048V
//!   - 2.5V
//!
//! - **特性 / Features:
//!   - 内部电压参考缓冲器
//!   - 支持外部电容连接
//!   - 可配置高阻抗模式
//!   - 电压就绪标志
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 36: Voltage reference buffer (VREFBUF)
//! - RM0456 Section 36.1: VREFBUF introduction
//! - RM0456 Section 36.2: VREFBUF main features
//! - RM0456 Section 36.3: VREFBUF functional description
//! - RM0456 Section 36.4: VREFBUF registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// VREFBUF base address / VREFBUF 基地址
pub const VREFBUF_BASE: usize = 0x4000_7030;

/// VREFBUF register offsets / VREFBUF 寄存器偏移
//! Reference: RM0456 Section 36.4: VREFBUF registers
pub mod reg {
    /// VREFBUF Control and Status Register / VREFBUF 控制和状态寄存器
    //! Reference: RM0456 Section 36.4.1: VREFBUF_CSR
    pub const CSR: usize = 0x00;
    /// VREFBUF Calibration Control Register / VREFBUF 校准控制寄存器
    //! Reference: RM0456 Section 36.4.2: VREFBUF_CCR
    pub const CCR: usize = 0x04;
}

/// CSR Register Bit Definitions / CSR 寄存器位定义
//! Reference: RM0456 Section 36.4.1
pub mod csr_bits {
    /// VREFBUF enable / VREFBUF 使能
    pub const ENVR: u32 = 1 << 0;
    /// High impedance mode / 高阻抗模式
    pub const HIZ: u32 = 1 << 1;
    /// Voltage reference ready / 电压参考就绪
    pub const VRR: u32 = 1 << 3;
    /// Voltage scale selection / 电压比例选择
    pub const VRS: u32 = 0b111 << 4;
}

/// CCR Register Bit Definitions / CCR 寄存器位定义
//! Reference: RM0456 Section 36.4.2
pub mod ccr_bits {
    /// Trimming code / 调整码
    pub const TRIM: u32 = 0x3F << 0;
}

/// VREFBUF voltage scale / VREFBUF 电压比例
//! Reference: RM0456 Section 36.3.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoltageScale {
    /// 1.5V
    V1_5 = 0b000,
    /// 1.8V
    V1_8 = 0b001,
    /// 2.048V
    V2_048 = 0b010,
    /// 2.5V
    V2_5 = 0b011,
}

impl VoltageScale {
    /// Get voltage in millivolts / 获取电压（毫伏）
    pub fn to_mv(&self) -> u32 {
        match self {
            VoltageScale::V1_5 => 1500,
            VoltageScale::V1_8 => 1800,
            VoltageScale::V2_048 => 2048,
            VoltageScale::V2_5 => 2500,
        }
    }
}

/// VREFBUF instance / VREFBUF 实例
pub struct VrefBuf;

impl VrefBuf {
    /// Create new VREFBUF instance / 创建新的 VREFBUF 实例
    pub const fn new() -> Self {
        Self
    }

    /// Initialize VREFBUF / 初始化 VREFBUF
    //! Reference: RM0456 Section 36.3.1
    pub fn init(&self, scale: VoltageScale) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            
            // Configure voltage scale / 配置电压比例
            let mut val = read_volatile(csr);
            val &= !csr_bits::VRS;
            val |= (scale as u32) << 4;
            write_volatile(csr, val);

            // Enable VREFBUF / 使能 VREFBUF
            let mut val = read_volatile(csr);
            val |= csr_bits::ENVR;
            write_volatile(csr, val);

            // Wait for VRR (voltage ready) / 等待 VRR（电压就绪）
            while (read_volatile(csr) & csr_bits::VRR) == 0 {}
        }
    }

    /// Enable high impedance mode / 使能高阻抗模式
    //! Reference: RM0456 Section 36.3.2
    pub fn enable_high_z(&self) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val |= csr_bits::HIZ;
            write_volatile(csr, val);
        }
    }

    /// Disable high impedance mode / 禁用高阻抗模式
    pub fn disable_high_z(&self) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::HIZ;
            write_volatile(csr, val);
        }
    }

    /// Check if voltage is ready / 检查电压是否就绪
    //! Reference: RM0456 Section 36.4.1, bit VRR
    pub fn is_ready(&self) -> bool {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let val = read_volatile(csr);
            (val & csr_bits::VRR) != 0
        }
    }

    /// Disable VREFBUF / 禁用 VREFBUF
    pub fn disable(&self) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::ENVR;
            write_volatile(csr, val);
        }
    }

    /// Set trimming value / 设置调整值
    //! Reference: RM0456 Section 36.3.3
    pub fn set_trimming(&self, trim: u8) {
        unsafe {
            let ccr = (VREFBUF_BASE + reg::CCR) as *mut u32;
            let mut val = read_volatile(ccr);
            val &= !ccr_bits::TRIM;
            val |= (trim & 0x3F) as u32;
            write_volatile(ccr, val);
        }
    }

    /// Get trimming value / 获取调整值
    pub fn get_trimming(&self) -> u8 {
        unsafe {
            let ccr = (VREFBUF_BASE + reg::CCR) as *const u32;
            (read_volatile(ccr) & ccr_bits::TRIM) as u8
        }
    }
}

/// Initialize VREFBUF with 2.5V output / 用 2.5V 输出初始化 VREFBUF
pub fn init_vrefbuf_2v5() {
    let vrefbuf = VrefBuf::new();
    vrefbuf.init(VoltageScale::V2_5);
}

/// Initialize VREFBUF with 1.8V output / 用 1.8V 输出初始化 VREFBUF
pub fn init_vrefbuf_1v8() {
    let vrefbuf = VrefBuf::new();
    vrefbuf.init(VoltageScale::V1_8);
}

/// Initialize VREFBUF with 2.048V output / 用 2.048V 输出初始化 VREFBUF
pub fn init_vrefbuf_2v048() {
    let vrefbuf = VrefBuf::new();
    vrefbuf.init(VoltageScale::V2_048);
}

/// Initialize VREFBUF with 1.5V output / 用 1.5V 输出初始化 VREFBUF
pub fn init_vrefbuf_1v5() {
    let vrefbuf = VrefBuf::new();
    vrefbuf.init(VoltageScale::V1_5);
}

/// Get current VREFBUF voltage in millivolts / 获取当前 VREFBUF 电压（毫伏）
pub fn get_vref_mv() -> u32 {
    unsafe {
        let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
        let val = read_volatile(csr);
        let scale = (val >> 4) & 0b111;
        match scale {
            0b000 => 1500,
            0b001 => 1800,
            0b010 => 2048,
            0b011 => 2500,
            _ => 2500,
        }
    }
}
