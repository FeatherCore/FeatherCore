//! ADC4 - Analog-to-Digital Converter 4
//! 模数转换器 4
//!
//! ## STM32U5 ADC4 特性 / Features
//! - **分辨率 / Resolution: 12-bit
//! - **通道数 / Channels: 最多 8 个外部通道
//! - **采样率 / Sampling Rate: 低功耗设计
//! - **工作模式 / Operating Modes:
//!   - 单通道模式 (Single channel mode)
//!   - 扫描模式 (Scan mode)
//!   - 间断模式 (Discontinuous mode)
//!
//! - **特性 / Features:
//!   - 可编程采样时间
//!   - 模拟看门狗 (Analog Watchdog)
//!   - 内部参考电压通道
//!   - 温度传感器通道
//!   - DMA 支持
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 34: Analog-to-digital converter (ADC4)
//! - RM0456 Section 34.1: ADC4 introduction
//! - RM0456 Section 34.2: ADC4 main features
//! - RM0456 Section 34.3: ADC4 functional description
//! - RM0456 Section 34.4: ADC4 registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// ADC4 base address (non-secure) / ADC4 基地址（非安全）
pub const ADC4_BASE: usize = 0x4202_8C00;

/// ADC4 register offsets / ADC4 寄存器偏移
//! Reference: RM0456 Section 34.4: ADC4 registers
pub mod reg {
    /// ADC4 Interrupt and Status Register / ADC4 中断和状态寄存器
    //! Reference: RM0456 Section 34.4.1: ADC4_ISR
    pub const ISR: usize = 0x00;

    /// ADC4 Interrupt Enable Register / ADC4 中断使能寄存器
    //! Reference: RM0456 Section 34.4.2: ADC4_IER
    pub const IER: usize = 0x04;

    /// ADC4 Control Register / ADC4 控制寄存器
    //! Reference: RM0456 Section 34.4.3: ADC4_CR
    pub const CR: usize = 0x08;

    /// ADC4 Configuration Register / ADC4 配置寄存器
    //! Reference: RM0456 Section 34.4.4: ADC4_CFGR
    pub const CFGR: usize = 0x0C;

    /// ADC4 Configuration Register 2 / ADC4 配置寄存器 2
    //! Reference: RM0456 Section 34.4.5: ADC4_CFGR2
    pub const CFGR2: usize = 0x10;

    /// ADC4 Sample Time Register 1 / ADC4 采样时间寄存器 1
    //! Reference: RM0456 Section 34.4.6: ADC4_SMPR1
    pub const SMPR1: usize = 0x14;

    /// ADC4 Watchdog Threshold Register 1 / ADC4 看门狗阈值寄存器 1
    pub const TR1: usize = 0x20;

    /// ADC4 Watchdog Threshold Register 2 / ADC4 看门狗阈值寄存器 2
    pub const TR2: usize = 0x24;

    /// ADC4 Watchdog Threshold Register 3 / ADC4 看门狗阈值寄存器 3
    pub const TR3: usize = 0x28;

    /// ADC4 Regular Sequence Register 1 / ADC4 规则序列寄存器 1
    //! Reference: RM0456 Section 34.4.7: ADC4_SQR1
    pub const SQR1: usize = 0x30;

    /// ADC4 Regular Sequence Register 2 / ADC4 规则序列寄存器 2
    pub const SQR2: usize = 0x34;

    /// ADC4 Regular Sequence Register 3 / ADC4 规则序列寄存器 3
    pub const SQR3: usize = 0x38;

    /// ADC4 Regular Sequence Register 4 / ADC4 规则序列寄存器 4
    pub const SQR4: usize = 0x3C;

    /// ADC4 Regular Data Register / ADC4 规则数据寄存器
    //! Reference: RM0456 Section 34.4.8: ADC4_DR
    pub const DR: usize = 0x40;

    /// ADC4 Offset Register 1 / ADC4 偏移寄存器 1
    pub const OFR1: usize = 0x60;

    /// ADC4 Analog Watchdog 2 Configuration Register / ADC4 模拟看门狗 2 配置寄存器
    pub const AWD2CR: usize = 0xA0;

    /// ADC4 Analog Watchdog 3 Configuration Register / ADC4 模拟看门狗 3 配置寄存器
    pub const AWD3CR: usize = 0xA4;

    /// ADC4 Differential Mode Selection Register / ADC4 差分模式选择寄存器
    pub const DIFSEL: usize = 0xB0;

    /// ADC4 Calibration Factors Register / ADC4 校准因子寄存器
    pub const CALFACT: usize = 0xB4;
}

/// CR Register Bit Definitions / CR 寄存器位定义
//! Reference: RM0456 Section 34.4.3
pub mod cr_bits {
    /// ADC enable / ADC 使能
    pub const ADEN: u32 = 1 << 0;
    /// ADC disable / ADC 禁用
    pub const ADDIS: u32 = 1 << 1;
    /// ADC start conversion / ADC 开始转换
    pub const ADSTART: u32 = 1 << 2;
    /// ADC stop conversion / ADC 停止转换
    pub const ADSTP: u32 = 1 << 4;
    /// ADC voltage regulator enable / ADC 电压调节器使能
    pub const ADVREGEN: u32 = 1 << 28;
    /// ADC calibration / ADC 校准
    pub const ADCAL: u32 = 1 << 31;
}

/// CFGR Register Bit Definitions / CFGR 寄存器位定义
//! Reference: RM0456 Section 34.4.4
pub mod cfgr_bits {
    /// Resolution / 分辨率
    pub const RES: u32 = 0b11 << 3;
    /// Continuous conversion / 连续转换
    pub const CONT: u32 = 1 << 13;
    /// Overrun mode / 覆盖模式
    pub const OVRMOD: u32 = 1 << 12;
    /// DMA enable / DMA 使能
    pub const DMAEN: u32 = 1 << 0;
    /// Scan mode / 扫描模式
    pub const SCAN: u32 = 1 << 5;
    /// Discontinuous mode / 间断模式
    pub const DISCEN: u32 = 1 << 1;
}

/// ISR Register Bit Definitions / ISR 寄存器位定义
//! Reference: RM0456 Section 34.4.1
pub mod isr_bits {
    /// ADC ready / ADC 就绪
    pub const ADRDY: u32 = 1 << 0;
    /// End of conversion / 转换结束
    pub const EOC: u32 = 1 << 2;
    /// End of sequence / 序列结束
    pub const EOS: u32 = 1 << 3;
    /// Analog watchdog 1 / 模拟看门狗 1
    pub const AWD1: u32 = 1 << 7;
}

/// ADC4 Resolution / ADC4 分辨率
//! Reference: RM0456 Section 34.3.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Resolution {
    /// 12-bit resolution (default)
    Bits12 = 0b00,
    /// 10-bit resolution
    Bits10 = 0b01,
    /// 8-bit resolution
    Bits8 = 0b10,
    /// 6-bit resolution
    Bits6 = 0b11,
}

/// ADC4 Sample Time Cycles / ADC4 采样时间周期
//! Reference: RM0456 Section 34.3.5
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SampleTime {
    /// 2.5 cycles
    Cycles2_5 = 0b000,
    /// 6.5 cycles
    Cycles6_5 = 0b001,
    /// 12.5 cycles
    Cycles12_5 = 0b010,
    /// 24.5 cycles
    Cycles24_5 = 0b011,
    /// 47.5 cycles
    Cycles47_5 = 0b100,
    /// 92.5 cycles
    Cycles92_5 = 0b101,
    /// 247.5 cycles
    Cycles247_5 = 0b110,
    /// 640.5 cycles
    Cycles640_5 = 0b111,
}

/// ADC4 Channel / ADC4 通道
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Channel {
    /// ADC4_IN0 - PB14
    Channel0 = 0,
    /// ADC4_IN1 - PB15
    Channel1 = 1,
    /// ADC4_IN2 - PC0
    Channel2 = 2,
    /// ADC4_IN3 - PC1
    Channel3 = 3,
    /// ADC4_IN4 - PC2
    Channel4 = 4,
    /// ADC4_IN5 - PC3
    Channel5 = 5,
    /// ADC4_IN6 - PC4
    Channel6 = 6,
    /// ADC4_IN7 - PC5
    Channel7 = 7,
    /// Internal temperature sensor
    TemperatureSensor = 8,
    /// Internal voltage reference (VREFINT)
    Vrefint = 9,
}

/// ADC4 Configuration / ADC4 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// ADC resolution
    pub resolution: Resolution,
    /// Continuous conversion mode
    pub continuous: bool,
    /// Scan mode
    pub scan_mode: bool,
    /// DMA enable
    pub dma_enable: bool,
    /// Overrun mode (false=overwrite, true=preserve)
    pub overrun_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resolution: Resolution::Bits12,
            continuous: false,
            scan_mode: false,
            dma_enable: false,
            overrun_mode: false,
        }
    }
}

/// ADC4 Instance / ADC4 实例
pub struct Adc4;

impl Adc4 {
    /// Create ADC4 instance / 创建 ADC4 实例
    pub const fn new() -> Self {
        Self
    }

    /// Initialize ADC4 / 初始化 ADC4
    //! Reference: RM0456 Section 34.3.1
    pub fn init(&self, config: &Config) {
        unsafe {
            let cr = (ADC4_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::ADVREGEN;
            write_volatile(cr, val);

            for _ in 0..1000 {
                core::arch::asm!("nop");
            }

            let cfgr = (ADC4_BASE + reg::CFGR) as *mut u32;
            let mut cfgr_val = 0;
            cfgr_val |= (config.resolution as u32) << 3;
            if config.continuous {
                cfgr_val |= cfgr_bits::CONT;
            }
            if config.dma_enable {
                cfgr_val |= cfgr_bits::DMAEN;
            }
            if config.overrun_mode {
                cfgr_val |= cfgr_bits::OVRMOD;
            }
            if config.scan_mode {
                cfgr_val |= cfgr_bits::SCAN;
            }
            write_volatile(cfgr, cfgr_val);

            self.calibrate();

            let mut val = read_volatile(cr);
            val |= cr_bits::ADEN;
            write_volatile(cr, val);

            while !self.is_ready() {}
        }
    }

    /// Calibrate ADC4 / 校准 ADC4
    //! Reference: RM0456 Section 34.3.12
    pub fn calibrate(&self) {
        unsafe {
            let cr = (ADC4_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::ADCAL;
            write_volatile(cr, val);
            while (read_volatile(cr) & cr_bits::ADCAL) != 0 {}
        }
    }

    /// Check if ADC is ready / 检查 ADC 是否就绪
    //! Reference: RM0456 Section 34.4.1
    pub fn is_ready(&self) -> bool {
        unsafe {
            let isr = (ADC4_BASE + reg::ISR) as *mut u32;
            let val = read_volatile(isr);
            (val & isr_bits::ADRDY) != 0
        }
    }

    /// Set sample time for a channel / 为通道设置采样时间
    pub fn set_sample_time(&self, channel: Channel, sample_time: SampleTime) {
        unsafe {
            let smpr1 = (ADC4_BASE + reg::SMPR1) as *mut u32;
            let mut val = read_volatile(smpr1);
            let pos = (channel as u8) * 3;
            val &= !(0b111 << pos);
            val |= (sample_time as u32) << pos;
            write_volatile(smpr1, val);
        }
    }

    /// Configure regular sequence (single channel) / 配置规则序列（单通道）
    pub fn configure_sequence(&self, channel: Channel) {
        unsafe {
            let sqr1 = (ADC4_BASE + reg::SQR1) as *mut u32;
            let mut val = read_volatile(sqr1);
            val &= !(0x1F << 6);
            val |= (channel as u32) << 6;
            val &= !(0xF << 0);
            val |= 0 << 0;
            write_volatile(sqr1, val);
        }
    }

    /// Start conversion / 开始转换
    pub fn start_conversion(&self) {
        unsafe {
            let cr = (ADC4_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::ADSTART;
            write_volatile(cr, val);
        }
    }

    /// Check if conversion is complete / 检查转换是否完成
    pub fn is_conversion_complete(&self) -> bool {
        unsafe {
            let isr = (ADC4_BASE + reg::ISR) as *mut u32;
            let val = read_volatile(isr);
            (val & isr_bits::EOC) != 0
        }
    }

    /// Read conversion result / 读取转换结果
    pub fn read(&self) -> u16 {
        unsafe {
            let dr = (ADC4_BASE + reg::DR) as *mut u32;
            read_volatile(dr) as u16
        }
    }

    /// Single conversion (blocking) / 单次转换（阻塞）
    pub fn convert(&self, channel: Channel) -> u16 {
        self.configure_sequence(channel);
        self.start_conversion();
        while !self.is_conversion_complete() {}
        self.read()
    }

    /// Enable ADC4 / 使能 ADC4
    pub fn enable(&self) {
        unsafe {
            let cr = (ADC4_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::ADEN;
            write_volatile(cr, val);
        }
    }

    /// Disable ADC4 / 禁用 ADC4
    pub fn disable(&self) {
        unsafe {
            let cr = (ADC4_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::ADEN;
            write_volatile(cr, val);
        }
    }
}

/// Initialize ADC4 with default configuration / 使用默认配置初始化 ADC4
pub fn init_adc4_default() {
    let adc = Adc4::new();
    let config = Config::default();
    adc.init(&config);
}
