//! OPAMP - Operational Amplifier
//! 运算放大器
//!
//! ## STM32U5 OPAMP 特性 / Features
//! - **运算放大器数量 / Op-amp Count:** 最多 3 个独立运算放大器
//! - **工作模式 / Operating Modes:**
//!   - 跟随器模式 (Follower mode)
//!   - PGA 模式 (Programmable Gain Amplifier)
//!   - 外部增益模式 (External gain)
//!
//! - **特性 / Features:**
//!   - 可编程增益 (1-16)
//!   - 校准功能 (Calibration)
//!   - 低功耗模式
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 38: Operational amplifiers (OPAMP)
//! - RM0456 Section 38.1: OPAMP introduction
//! - RM0456 Section 38.2: OPAMP main features
//! - RM0456 Section 38.3: OPAMP functional description
//! - RM0456 Section 38.4: OPAMP registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// OPAMP1 base address / OPAMP1 基地址
pub const OPAMP1_BASE: usize = 0x4000_9030;
/// OPAMP2 base address
pub const OPAMP2_BASE: usize = 0x4000_9034;
/// OPAMP3 base address
pub const OPAMP3_BASE: usize = 0x4000_9038;

/// OPAMP register offsets / OPAMP 寄存器偏移
//! Reference: RM0456 Section 38.4: OPAMP registers
pub mod reg {
    /// OPAMP Control and Status Register / OPAMP 控制和状态寄存器
    //! Reference: RM0456 Section 38.4.1: OPAMPx_CSR
    pub const CSR: usize = 0x00;
    /// OPAMP Offset Trimming Register / OPAMP 偏移调整寄存器
    //! Reference: RM0456 Section 38.4.2: OPAMPx_OTR
    pub const OTR: usize = 0x04;
    /// OPAMP Low-Power Offset Trimming Register / OPAMP 低功耗偏移调整寄存器
    //! Reference: RM0456 Section 38.4.3: OPAMPx_LPOTR
    pub const LPOTR: usize = 0x08;
}

/// CSR Register Bit Definitions / CSR 寄存器位定义
//! Reference: RM0456 Section 38.4.1
pub mod csr_bits {
    /// OPAMP enable / OPAMP 使能
    pub const OPAMPEN: u32 = 1 << 0;
    /// Functional mode / 功能模式
    pub const VM_SEL: u32 = 0b11 << 1;
    /// PGA gain / PGA 增益
    pub const PGA_GAIN: u32 = 0b11 << 4;
    /// User trimming enable / 用户调整使能
    pub const USERTRIM: u32 = 1 << 6;
    /// Calibration mode / 校准模式
    pub const CALON: u32 = 1 << 7;
    /// Calibration output selection / 校准输出选择
    pub const CALSEL: u32 = 0b11 << 8;
    /// Power mode / 电源模式
    pub const OPAPOWER: u32 = 0b11 << 14;
    /// Output ready / 输出就绪
    pub const OPARANGE: u32 = 1 << 18;
    /// Calibration result / 校准结果
    pub const CALOUT: u32 = 0b11111 << 24;
    /// Calibration mode selection / 校准模式选择
    pub const CAL_MODE: u32 = 0b11 << 30;
}

/// OTR Register Bit Definitions / OTR 寄存器位定义
//! Reference: RM0456 Section 38.4.2
pub mod otr_bits {
    /// Offset trimming for PMOS / PMOS 偏移调整
    pub const TRIMOFFSETP: u32 = 0x1F << 0;
    /// Offset trimming for NMOS / NMOS 偏移调整
    pub const TRIMOFFSETN: u32 = 0x1F << 8;
}

/// OPAMP mode / OPAMP 模式
//! Reference: RM0456 Section 38.3.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Functional mode / 功能模式
    Functional = 0b00,
    /// Calibration mode / 校准模式
    Calibration = 0b01,
    /// Test mode / 测试模式
    Test = 0b10,
}

/// OPAMP functional mode / OPAMP 功能模式
//! Reference: RM0456 Section 38.3.2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionalMode {
    /// Standalone mode / 独立模式
    Standalone = 0b000,
    /// Follower mode / 跟随器模式
    Follower = 0b010,
    /// PGA mode / PGA 模式
    Pga = 0b100,
}

/// OPAMP PGA gain / OPAMP PGA 增益
//! Reference: RM0456 Section 38.3.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PgaGain {
    Gain2 = 0b00,
    Gain4 = 0b01,
    Gain8 = 0b10,
    Gain16 = 0b11,
}

/// OPAMP power mode / OPAMP 电源模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    Normal = 0b00,
    LowPower = 0b01,
    HighPerformance = 0b10,
}

/// OPAMP configuration / OPAMP 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub functional_mode: FunctionalMode,
    pub pga_gain: PgaGain,
    pub power_mode: PowerMode,
    pub user_trim: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            functional_mode: FunctionalMode::Follower,
            pga_gain: PgaGain::Gain2,
            power_mode: PowerMode::Normal,
            user_trim: false,
        }
    }
}

/// OPAMP instance / OPAMP 实例
pub struct Opamp {
    base: usize,
}

impl Opamp {
    /// Create OPAMP1 instance / 创建 OPAMP1 实例
    pub const fn opamp1() -> Self {
        Self { base: OPAMP1_BASE }
    }

    /// Create OPAMP2 instance / 创建 OPAMP2 实例
    pub const fn opamp2() -> Self {
        Self { base: OPAMP2_BASE }
    }

    /// Create OPAMP3 instance / 创建 OPAMP3 实例
    pub const fn opamp3() -> Self {
        Self { base: OPAMP3_BASE }
    }

    /// Initialize OPAMP / 初始化 OPAMP
    //! Reference: RM0456 Section 38.3.1
    pub fn init(&self, config: &Config) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = 0;
            
            // Configure functional mode / 配置功能模式
            val |= (config.functional_mode as u32) << 1;
            
            // Configure PGA gain if PGA mode / 如果是 PGA 模式，配置增益
            if config.functional_mode == FunctionalMode::Pga {
                val |= (config.pga_gain as u32) << 4;
            }
            
            // Configure power mode / 配置电源模式
            val |= (config.power_mode as u32) << 14;
            
            // Configure user trim / 配置用户调整
            if config.user_trim {
                val |= csr_bits::USERTRIM;
            }
            
            write_volatile(csr, val);
        }
    }

    /// Initialize OPAMP in follower mode / 在跟随器模式下初始化 OPAMP
    pub fn init_follower(&self) {
        let config = Config {
            functional_mode: FunctionalMode::Follower,
            ..Default::default()
        };
        self.init(&config);
        self.enable();
    }

    /// Initialize OPAMP in PGA mode / 在 PGA 模式下初始化 OPAMP
    pub fn init_pga(&self, gain: PgaGain) {
        let config = Config {
            functional_mode: FunctionalMode::Pga,
            pga_gain: gain,
            ..Default::default()
        };
        self.init(&config);
        self.enable();
    }

    /// Enable OPAMP / 使能 OPAMP
    pub fn enable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val |= csr_bits::OPAMPEN;
            write_volatile(csr, val);
        }
    }

    /// Disable OPAMP / 禁用 OPAMP
    pub fn disable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::OPAMPEN;
            write_volatile(csr, val);
        }
    }

    /// Set PGA gain / 设置 PGA 增益
    pub fn set_pga_gain(&self, gain: PgaGain) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::PGA_GAIN;
            val |= (gain as u32) << 4;
            write_volatile(csr, val);
        }
    }

    /// Set functional mode / 设置功能模式
    pub fn set_functional_mode(&self, mode: FunctionalMode) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::VM_SEL;
            val |= (mode as u32) << 1;
            write_volatile(csr, val);
        }
    }

    /// Set power mode / 设置电源模式
    pub fn set_power_mode(&self, mode: PowerMode) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::OPAPOWER;
            val |= (mode as u32) << 14;
            write_volatile(csr, val);
        }
    }

    /// Check if OPAMP is ready / 检查 OPAMP 是否就绪
    pub fn is_ready(&self) -> bool {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let val = read_volatile(csr);
            (val & csr_bits::OPARANGE) != 0
        }
    }

    /// Calibrate OPAMP / 校准 OPAMP
    //! Reference: RM0456 Section 38.3.4
    pub fn calibrate(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            
            // Enter calibration mode / 进入校准模式
            let mut val = read_volatile(csr);
            val |= (Mode::Calibration as u32) << 30;
            write_volatile(csr, val);

            // Enable OPAMP for calibration / 为校准使能 OPAMP
            let mut val = read_volatile(csr);
            val |= csr_bits::OPAMPEN;
            write_volatile(csr, val);

            // Wait for calibration complete / 等待校准完成
            while (read_volatile(csr) & (1 << 14)) == 0 {}

            // Store calibration data / 存储校准数据
            let otr = (self.base + reg::OTR) as *mut u32;
            let trim_value = (read_volatile(csr) >> 24) & 0x1F;
            write_volatile(otr, trim_value);

            // Exit calibration mode / 退出校准模式
            let mut val = read_volatile(csr);
            val &= !csr_bits::CAL_MODE;
            write_volatile(csr, val);
        }
    }

    /// Set offset trimming / 设置偏移调整
    pub fn set_offset_trimming(&self, trim_p: u8, trim_n: u8) {
        unsafe {
            let otr = (self.base + reg::OTR) as *mut u32;
            let mut val = read_volatile(otr);
            val &= !(otr_bits::TRIMOFFSETP | otr_bits::TRIMOFFSETN);
            val |= (trim_p & 0x1F) as u32;
            val |= ((trim_n & 0x1F) as u32) << 8;
            write_volatile(otr, val);
        }
    }

    /// Enable user trimming / 使能用户调整
    pub fn enable_user_trim(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val |= csr_bits::USERTRIM;
            write_volatile(csr, val);
        }
    }

    /// Disable user trimming / 禁用用户调整
    pub fn disable_user_trim(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = read_volatile(csr);
            val &= !csr_bits::USERTRIM;
            write_volatile(csr, val);
        }
    }
}

/// Initialize OPAMP1 as voltage follower / 初始化 OPAMP1 为电压跟随器
pub fn init_opamp1_follower() {
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::OPAMP);
    
    let opamp = Opamp::opamp1();
    opamp.init_follower();
}

/// Initialize OPAMP2 as PGA with gain 4 / 初始化 OPAMP2 为增益 4 的 PGA
pub fn init_opamp2_pga() {
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::OPAMP);
    
    let opamp = Opamp::opamp2();
    opamp.init_pga(PgaGain::Gain4);
}

/// Initialize and calibrate OPAMP3 / 初始化并校准 OPAMP3
pub fn init_opamp3_calibrated() {
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::OPAMP);
    
    let opamp = Opamp::opamp3();
    opamp.init_follower();
    opamp.calibrate();
}
