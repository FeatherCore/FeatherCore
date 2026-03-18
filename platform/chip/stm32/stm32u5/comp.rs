//! COMP - Comparator
//! 比较器
//!
//! ## STM32U5 COMP 特性 / Features
//! - **比较器数量 / Comparator Count:**
//!   - 最多 2 个独立比较器 (COMP1, COMP2)
//!
//! - **输入源 / Input Sources:**
//!   - IO 引脚输入
//!   - DAC 输出
//!   - 内部参考电压
//!
//! - **功能 / Features:**
//!   - 支持窗口模式 (Window mode)
//!   - 支持输出极性选择
//!   - 支持消隐功能 (Blanking function)
//!   - 可编程迟滞
//!   - 中断/事件生成
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 37: Comparator (COMP)
//! - RM0456 Section 37.1: COMP introduction
//! - RM0456 Section 37.2: COMP main features
//! - RM0456 Section 37.3: COMP functional description
//! - RM0456 Section 37.4: COMP registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// COMP1 base address / COMP1 基地址
pub const COMP1_BASE: usize = 0x4000_9200;
/// COMP2 base address / COMP2 基地址
pub const COMP2_BASE: usize = 0x4000_9204;

/// COMP register offsets / COMP 寄存器偏移
//! Reference: RM0456 Section 37.4: COMP registers
pub mod reg {
    /// Comparator Control and Status Register / 比较器控制与状态寄存器
    //! Reference: RM0456 Section 37.4.1: COMPx_CSR
    pub const CSR: usize = 0x00;
}

/// CSR Register Bit Definitions / CSR 寄存器位定义
//! Reference: RM0456 Section 37.4.1
pub mod csr_bits {
    /// Comparator enable / 比较器使能
    pub const EN: u32 = 1 << 0;
    /// Input plus selection / 正输入选择
    pub const INPSEL: u32 = 0b11 << 4;
    /// Input minus selection / 负输入选择
    pub const INMSEL: u32 = 0b111 << 8;
    /// Window mode enable / 窗口模式使能
    pub const WINMODE: u32 = 1 << 11;
    /// Output polarity / 输出极性
    pub const POLARITY: u32 = 1 << 15;
    /// Hysteresis / 迟滞
    pub const HYST: u32 = 0b11 << 16;
    /// Power mode / 电源模式
    pub const PWRMODE: u32 = 0b11 << 12;
    /// Blanking source / 消隐源
    pub const BLANKING: u32 = 0b111 << 18;
    /// Output / 输出
    pub const VALUE: u32 = 1 << 30;
    /// Lock / 锁定
    pub const LOCK: u32 = 1 << 31;
}

/// COMP input plus / COMP 正输入
//! Reference: RM0456 Section 37.3.2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputPlus {
    /// IO1 (PA1 for COMP1, PA3 for COMP2)
    Io1 = 0b00,
    /// IO2 (PA7 for COMP1, PB7 for COMP2)
    Io2 = 0b01,
    /// DAC1 output
    Dac1 = 0b10,
    /// DAC2 output
    Dac2 = 0b11,
}

/// COMP input minus / COMP 负输入
//! Reference: RM0456 Section 37.3.2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputMinus {
    /// 1/4 VREFINT
    VrefintDiv4 = 0b000,
    /// 1/2 VREFINT
    VrefintDiv2 = 0b001,
    /// 3/4 VREFINT
    VrefintDiv3_4 = 0b010,
    /// VREFINT
    Vrefint = 0b011,
    /// DAC1 CH1
    Dac1Ch1 = 0b100,
    /// DAC1 CH2
    Dac1Ch2 = 0b101,
    /// IO1 (PA0 for COMP1, PA2 for COMP2)
    Io1 = 0b110,
    /// IO2 (PB1 for COMP1, PB3 for COMP2)
    Io2 = 0b111,
}

/// COMP hysteresis / COMP 迟滞
//! Reference: RM0456 Section 37.3.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hysteresis {
    None = 0b00,
    Low = 0b01,
    Medium = 0b10,
    High = 0b11,
}

/// COMP power mode / COMP 电源模式
//! Reference: RM0456 Section 37.3.4
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// High speed / 高速
    HighSpeed = 0b00,
    /// Medium speed / 中速
    MediumSpeed = 0b01,
    /// Low power / 低功耗
    LowPower = 0b10,
    /// Ultra low power / 超低功耗
    UltraLowPower = 0b11,
}

/// COMP blanking source / COMP 消隐源
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlankingSource {
    NoBlanking = 0b000,
    Blanking1 = 0b001,
    Blanking2 = 0b010,
    Blanking3 = 0b011,
    Blanking4 = 0b100,
    Blanking5 = 0b101,
    Blanking6 = 0b110,
    Blanking7 = 0b111,
}

/// COMP configuration / COMP 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub input_plus: InputPlus,
    pub input_minus: InputMinus,
    pub hysteresis: Hysteresis,
    pub power_mode: PowerMode,
    pub output_polarity: bool, // true = inverted
    pub blanking_source: BlankingSource,
    pub window_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_plus: InputPlus::Io1,
            input_minus: InputMinus::Vrefint,
            hysteresis: Hysteresis::None,
            power_mode: PowerMode::HighSpeed,
            output_polarity: false,
            blanking_source: BlankingSource::NoBlanking,
            window_mode: false,
        }
    }
}

/// COMP instance / COMP 实例
pub struct Comp {
    base: usize,
}

impl Comp {
    /// Create COMP1 instance / 创建 COMP1 实例
    pub const fn comp1() -> Self {
        Self { base: COMP1_BASE }
    }

    /// Create COMP2 instance / 创建 COMP2 实例
    pub const fn comp2() -> Self {
        Self { base: COMP2_BASE }
    }

    /// Initialize comparator / 初始化比较器
    //! Reference: RM0456 Section 37.3.1
    pub fn init(&self, config: &Config) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = 0;
            val |= (config.input_plus as u32) << 4;
            val |= (config.input_minus as u32) << 8;
            val |= (config.hysteresis as u32) << 16;
            val |= (config.power_mode as u32) << 12;
            val |= (config.blanking_source as u32) << 18;
            if config.output_polarity {
                val |= csr_bits::POLARITY;
            }
            if config.window_mode {
                val |= csr_bits::WINMODE;
            }
            val |= csr_bits::EN; // COMPEN
            write_volatile(csr, val);
        }
    }

    /// Enable comparator / 使能比较器
    pub fn enable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val |= csr_bits::EN;
            write_volatile(csr, val);
        }
    }

    /// Disable comparator / 禁用比较器
    pub fn disable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::EN;
            write_volatile(csr, val);
        }
    }

    /// Read output state / 读取输出状态
    //! Reference: RM0456 Section 37.4.1, bit VALUE
    pub fn read_output(&self) -> bool {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let val = read_volatile(csr);
            (val & csr_bits::VALUE) != 0
        }
    }

    /// Lock comparator configuration / 锁定比较器配置
    //! Reference: RM0456 Section 37.3.5
    pub fn lock(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val |= csr_bits::LOCK;
            write_volatile(csr, val);
        }
    }

    /// Set input plus / 设置正输入
    pub fn set_input_plus(&self, input: InputPlus) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::INPSEL;
            val |= (input as u32) << 4;
            write_volatile(csr, val);
        }
    }

    /// Set input minus / 设置负输入
    pub fn set_input_minus(&self, input: InputMinus) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::INMSEL;
            val |= (input as u32) << 8;
            write_volatile(csr, val);
        }
    }

    /// Set hysteresis / 设置迟滞
    pub fn set_hysteresis(&self, hyst: Hysteresis) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::HYST;
            val |= (hyst as u32) << 16;
            write_volatile(csr, val);
        }
    }

    /// Set power mode / 设置电源模式
    pub fn set_power_mode(&self, mode: PowerMode) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::PWRMODE;
            val |= (mode as u32) << 12;
            write_volatile(csr, val);
        }
    }

    /// Set blanking source / 设置消隐源
    pub fn set_blanking_source(&self, source: BlankingSource) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::BLANKING;
            val |= (source as u32) << 18;
            write_volatile(csr, val);
        }
    }

    /// Enable window mode / 使能窗口模式
    pub fn enable_window_mode(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val |= csr_bits::WINMODE;
            write_volatile(csr, val);
        }
    }

    /// Disable window mode / 禁用窗口模式
    pub fn disable_window_mode(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::WINMODE;
            write_volatile(csr, val);
        }
    }

    /// Set output polarity / 设置输出极性
    pub fn set_output_polarity(&self, inverted: bool) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            if inverted {
                val |= csr_bits::POLARITY;
            } else {
                val &= !csr_bits::POLARITY;
            }
            write_volatile(csr, val);
        }
    }
}

/// Initialize COMP1 with default configuration / 使用默认配置初始化 COMP1
pub fn init_comp1_default() {
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::COMP);
    
    let comp = Comp::comp1();
    let config = Config::default();
    comp.init(&config);
}

/// Initialize COMP2 for voltage monitoring / 为电压监控初始化 COMP2
pub fn init_comp2_voltage_monitor(threshold: InputMinus) {
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::COMP);
    
    let comp = Comp::comp2();
    let config = Config {
        input_plus: InputPlus::Io1,
        input_minus: threshold,
        hysteresis: Hysteresis::Low,
        power_mode: PowerMode::LowPower,
        output_polarity: false,
        blanking_source: BlankingSource::NoBlanking,
        window_mode: false,
    };
    comp.init(&config);
}

/// Initialize window mode with COMP1 and COMP2 / 使用 COMP1 和 COMP2 初始化窗口模式
pub fn init_window_mode(low_threshold: InputMinus, high_threshold: InputMinus) {
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::COMP);
    
    let comp1 = Comp::comp1();
    let comp2 = Comp::comp2();
    
    let config1 = Config {
        input_plus: InputPlus::Io1,
        input_minus: low_threshold,
        hysteresis: Hysteresis::None,
        power_mode: PowerMode::HighSpeed,
        output_polarity: false,
        blanking_source: BlankingSource::NoBlanking,
        window_mode: false,
    };
    comp1.init(&config1);
    
    let config2 = Config {
        input_plus: InputPlus::Io1,
        input_minus: high_threshold,
        hysteresis: Hysteresis::None,
        power_mode: PowerMode::HighSpeed,
        output_polarity: true,
        blanking_source: BlankingSource::NoBlanking,
        window_mode: true,
    };
    comp2.init(&config2);
}
