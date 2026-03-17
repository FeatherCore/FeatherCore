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

/// COMP1 base address / COMP1 基地址
pub const COMP1_BASE: usize = 0x4000_9200;
/// COMP2 base address / COMP2 基地址
pub const COMP2_BASE: usize = 0x4000_9204;

/// COMP register offsets / COMP 寄存器偏移
pub mod reg {
    /// Comparator Control and Status Register / 比较器控制与状态寄存器
    pub const CSR: usize = 0x00;
}

/// COMP input plus
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

/// COMP input minus
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

/// COMP hysteresis
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hysteresis {
    None = 0b00,
    Low = 0b01,
    Medium = 0b10,
    High = 0b11,
}

/// COMP power mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// High speed
    HighSpeed = 0b00,
    /// Medium speed
    MediumSpeed = 0b01,
    /// Low power
    LowPower = 0b10,
    /// Ultra low power
    UltraLowPower = 0b11,
}

/// COMP configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub input_plus: InputPlus,
    pub input_minus: InputMinus,
    pub hysteresis: Hysteresis,
    pub power_mode: PowerMode,
    pub output_polarity: bool, // true = inverted
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_plus: InputPlus::Io1,
            input_minus: InputMinus::Vrefint,
            hysteresis: Hysteresis::None,
            power_mode: PowerMode::HighSpeed,
            output_polarity: false,
        }
    }
}

/// COMP instance
pub struct Comp {
    base: usize,
}

impl Comp {
    pub const fn comp1() -> Self {
        Self { base: COMP1_BASE }
    }

    pub const fn comp2() -> Self {
        Self { base: COMP2_BASE }
    }

    /// Initialize comparator
    pub fn init(&self, config: &Config) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = 0;
            val |= (config.input_plus as u32) << 4;
            val |= (config.input_minus as u32) << 8;
            val |= (config.hysteresis as u32) << 16;
            val |= (config.power_mode as u32) << 12;
            if config.output_polarity {
                val |= 1 << 15;
            }
            val |= 1 << 0; // COMPEN
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Enable comparator
    pub fn enable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val |= 1 << 0;
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Disable comparator
    pub fn disable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val &= !(1 << 0);
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Read output state
    pub fn read_output(&self) -> bool {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let val = core::ptr::read_volatile(csr);
            (val & (1 << 30)) != 0
        }
    }

    /// Lock comparator configuration
    pub fn lock(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val |= 1 << 31;
            core::ptr::write_volatile(csr, val);
        }
    }
}

/// Initialize COMP1 with default configuration
pub fn init_comp1_default() {
    // Enable COMP clock
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::COMP);
    
    let comp = Comp::comp1();
    let config = Config::default();
    comp.init(&config);
}

/// Initialize COMP2 for voltage monitoring
pub fn init_comp2_voltage_monitor(threshold: InputMinus) {
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::COMP);
    
    let comp = Comp::comp2();
    let config = Config {
        input_plus: InputPlus::Io1,
        input_minus: threshold,
        hysteresis: Hysteresis::Low,
        power_mode: PowerMode::LowPower,
        output_polarity: false,
    };
    comp.init(&config);
}
