//! ADC - Analog-to-Digital Converter
//! 模数转换器
//!
//! # Overview / 概述
//! STM32U5 series features up to 3 ADCs: ADC1, ADC2 (synchronous operation),
//! and ADC4 (low-power).
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 33 (ADC12) and Chapter 34 (ADC4)
//!
//! ## ADC Modules / ADC模块
//! - **ADC1/ADC2**: Main ADCs supporting 12-bit resolution, up to 4 Msps (synchronous mode)
//! - **ADC4**: Low-power ADC supporting 12-bit resolution
//!
//! ## Conversion Modes / 转换模式
//! Reference: RM0456 Section 33.3
//! - Single conversion mode
//! - Continuous conversion mode
//! - Scan mode (multiple channels)
//! - Discontinuous mode
//! - Injected conversion
//!
//! ## Trigger Sources / 触发源
//! Reference: RM0456 Section 33.3.14
//! - Software trigger
//! - External triggers (Timer, EXTI)
//!
//! ## Features / 特性
//! - Configurable sample time
//! - Analog watchdog (AWD)
//! - Internal reference voltage channel
//! - Temperature sensor channel
//! - VBAT channel
//!
//! # ADC Channels / ADC通道
//! | Channel | Pin | Description |
//! |---------|-----|-------------|
//! | 0 | PA0 | ADC1_IN0 |
//! | 1 | PA1 | ADC1_IN1 |
//! | ... | ... | ... |
//! | 16 | - | Internal temperature sensor |
//! | 17 | - | Internal voltage reference (VREFINT) |
//! | 18 | - | VBAT channel |
//!
//! # Reference / 参考
//! - RM0456 Chapter 33: Analog-to-digital converter (ADC12)
//! - RM0456 Section 33.1: ADC introduction
//! - RM0456 Section 33.2: ADC main features
//! - RM0456 Section 33.3: ADC functional description
//! - RM0456 Section 33.4: ADC registers
//! - RM0456 Chapter 34: Analog-to-digital converter (ADC4)

#![no_std]

/// ADC1 base address (non-secure)
//! Reference: RM0456 Chapter 2, Table 1
pub const ADC1_BASE: usize = 0x4202_8000;

/// ADC2 base address (non-secure)
pub const ADC2_BASE: usize = 0x4202_8100;

/// ADC4 base address (non-secure)
pub const ADC4_BASE: usize = 0x4202_8C00;

/// ADC common registers base address
//! Reference: RM0456 Section 33.4.15
pub const ADC1_COMMON_BASE: usize = 0x4202_8300;

/// ADC register offsets
//! Reference: RM0456 Section 33.4: ADC registers
pub mod reg {
    /// ADC Interrupt and Status Register
    //! Reference: RM0456 Section 33.4.1
    pub const ISR: usize = 0x00;

    /// ADC Interrupt Enable Register
    //! Reference: RM0456 Section 33.4.2
    pub const IER: usize = 0x04;

    /// ADC Control Register
    //! Reference: RM0456 Section 33.4.3
    pub const CR: usize = 0x08;

    /// ADC Configuration Register
    //! Reference: RM0456 Section 33.4.4
    pub const CFGR: usize = 0x0C;

    /// ADC Configuration Register 2
    //! Reference: RM0456 Section 33.4.5
    pub const CFGR2: usize = 0x10;

    /// ADC Sample Time Register 1
    //! Reference: RM0456 Section 33.4.6
    pub const SMPR1: usize = 0x14;

    /// ADC Sample Time Register 2
    pub const SMPR2: usize = 0x18;

    /// ADC Watchdog Threshold Register 1
    pub const TR1: usize = 0x20;

    /// ADC Watchdog Threshold Register 2
    pub const TR2: usize = 0x24;

    /// ADC Watchdog Threshold Register 3
    pub const TR3: usize = 0x28;

    /// ADC Regular Sequence Register 1
    //! Reference: RM0456 Section 33.4.7
    pub const SQR1: usize = 0x30;

    /// ADC Regular Sequence Register 2
    pub const SQR2: usize = 0x34;

    /// ADC Regular Sequence Register 3
    pub const SQR3: usize = 0x38;

    /// ADC Regular Sequence Register 4
    pub const SQR4: usize = 0x3C;

    /// ADC Regular Data Register
    //! Reference: RM0456 Section 33.4.8
    pub const DR: usize = 0x40;

    /// ADC Injected Sequence Register
    pub const JSQR: usize = 0x4C;

    /// ADC Offset Register 1
    pub const OFR1: usize = 0x60;

    /// ADC Offset Register 2
    pub const OFR2: usize = 0x64;

    /// ADC Offset Register 3
    pub const OFR3: usize = 0x68;

    /// ADC Offset Register 4
    pub const OFR4: usize = 0x6C;

    /// ADC Injected Data Register 1
    pub const JDR1: usize = 0x80;

    /// ADC Injected Data Register 2
    pub const JDR2: usize = 0x84;

    /// ADC Injected Data Register 3
    pub const JDR3: usize = 0x88;

    /// ADC Injected Data Register 4
    pub const JDR4: usize = 0x8C;

    /// ADC Analog Watchdog 2 Configuration Register
    pub const AWD2CR: usize = 0xA0;

    /// ADC Analog Watchdog 3 Configuration Register
    pub const AWD3CR: usize = 0xA4;

    /// ADC Differential Mode Selection Register
    pub const DIFSEL: usize = 0xB0;

    /// ADC Calibration Factors Register
    pub const CALFACT: usize = 0xB4;
}

/// ADC Common register offsets
//! Reference: RM0456 Section 33.4.15
pub mod common_reg {
    /// ADC Common Status Register
    pub const CSR: usize = 0x00;

    /// ADC Common Control Register
    //! Reference: RM0456 Section 33.4.16
    pub const CCR: usize = 0x08;

    /// ADC Common Data Register for dual mode
    pub const CDR: usize = 0x0C;

    /// ADC Hardware Configuration Register 0
    pub const HWCFGR0: usize = 0x10;

    /// ADC Hardware Configuration Register 1
    pub const HWCFGR1: usize = 0x14;

    /// ADC Hardware Configuration Register 2
    pub const HWCFGR2: usize = 0x18;

    /// ADC Version Register
    pub const VERR: usize = 0x3F4;

    /// ADC Identification Register
    pub const IIDR: usize = 0x3F8;

    /// ADC Size ID Register
    pub const SIDR: usize = 0x3FC;
}

/// CR Register Bit Definitions
//! Reference: RM0456 Section 33.4.3
pub mod cr_bits {
    /// ADC enable / ADC使能
    pub const ADEN: u32 = 1 << 0;

    /// ADC disable / ADC禁用
    pub const ADDIS: u32 = 1 << 1;

    /// ADC start conversion / ADC开始转换
    pub const ADSTART: u32 = 1 << 2;

    /// ADC stop conversion / ADC停止转换
    pub const ADSTP: u32 = 1 << 4;

    /// ADC voltage regulator enable / ADC电压调节器使能
    pub const ADVREGEN: u32 = 1 << 28;

    /// ADC calibration / ADC校准
    pub const ADCAL: u32 = 1 << 31;

    /// ADC linearity calibration / ADC线性校准
    pub const ADCALLIN: u32 = 1 << 30;
}

/// CFGR Register Bit Definitions
//! Reference: RM0456 Section 33.4.4
pub mod cfgr_bits {
    /// Resolution / 分辨率
    pub const RES: u32 = 0b11 << 3;

    /// Continuous conversion / 连续转换
    pub const CONT: u32 = 1 << 13;

    /// Overrun mode / 覆盖模式
    pub const OVRMOD: u32 = 1 << 12;

    /// DMA enable / DMA使能
    pub const DMAEN: u32 = 1 << 0;

    /// Scan mode / 扫描模式
    pub const SCAN: u32 = 1 << 5;

    /// Discontinuous mode / 间断模式
    pub const DISCEN: u32 = 1 << 1;
}

/// CCR Register Bit Definitions
//! Reference: RM0456 Section 33.4.16
pub mod ccr_bits {
    /// ADC prescaler / ADC预分频器
    pub const PRESC: u32 = 0b1111 << 18;

    /// VREFINT enable / VREFINT使能
    pub const VREFEN: u32 = 1 << 22;

    /// Temperature sensor enable / 温度传感器使能
    pub const VSENSEEN: u32 = 1 << 23;

    /// VBAT enable / VBAT使能
    pub const VBATEN: u32 = 1 << 24;
}

/// ISR Register Bit Definitions
//! Reference: RM0456 Section 33.4.1
pub mod isr_bits {
    /// ADC ready / ADC就绪
    pub const ADRDY: u32 = 1 << 0;

    /// End of conversion / 转换结束
    pub const EOC: u32 = 1 << 2;

    /// End of sequence / 序列结束
    pub const EOS: u32 = 1 << 3;

    /// End of injected sequence / 注入序列结束
    pub const JEOS: u32 = 1 << 5;

    /// Analog watchdog 1 / 模拟看门狗1
    pub const AWD1: u32 = 1 << 7;

    /// Injected data overrun / 注入数据覆盖
    pub const JQOVF: u32 = 1 << 10;
}

/// ADC Resolution
//! Reference: RM0456 Section 33.3.3
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

/// ADC Sample Time Cycles
//! Reference: RM0456 Section 33.3.5
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

/// ADC Clock Prescaler
//! Reference: RM0456 Section 33.3.2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockPrescaler {
    /// Synchronous clock mode, ADC clock = AHB/1
    Div1 = 0b00000,
    /// Synchronous clock mode, ADC clock = AHB/2
    Div2 = 0b00001,
    /// Synchronous clock mode, ADC clock = AHB/4
    Div4 = 0b00010,
    /// Synchronous clock mode, ADC clock = AHB/6
    Div6 = 0b00011,
    /// Synchronous clock mode, ADC clock = AHB/8
    Div8 = 0b00100,
    /// Synchronous clock mode, ADC clock = AHB/10
    Div10 = 0b00101,
    /// Synchronous clock mode, ADC clock = AHB/12
    Div12 = 0b00110,
    /// Synchronous clock mode, ADC clock = AHB/16
    Div16 = 0b00111,
    /// Synchronous clock mode, ADC clock = AHB/32
    Div32 = 0b01000,
    /// Synchronous clock mode, ADC clock = AHB/64
    Div64 = 0b01001,
    /// Synchronous clock mode, ADC clock = AHB/128
    Div128 = 0b01010,
    /// Synchronous clock mode, ADC clock = AHB/256
    Div256 = 0b01011,
    /// Asynchronous clock mode, ADC clock = PLL"P"/1
    AsyncDiv1 = 0b10000,
    /// Asynchronous clock mode, ADC clock = PLL"P"/2
    AsyncDiv2 = 0b10001,
    /// Asynchronous clock mode, ADC clock = PLL"P"/4
    AsyncDiv4 = 0b10010,
    /// Asynchronous clock mode, ADC clock = PLL"P"/6
    AsyncDiv6 = 0b10011,
    /// Asynchronous clock mode, ADC clock = PLL"P"/8
    AsyncDiv8 = 0b10100,
    /// Asynchronous clock mode, ADC clock = PLL"P"/10
    AsyncDiv10 = 0b10101,
    /// Asynchronous clock mode, ADC clock = PLL"P"/12
    AsyncDiv12 = 0b10110,
    /// Asynchronous clock mode, ADC clock = PLL"P"/16
    AsyncDiv16 = 0b10111,
    /// Asynchronous clock mode, ADC clock = PLL"P"/32
    AsyncDiv32 = 0b11000,
    /// Asynchronous clock mode, ADC clock = PLL"P"/64
    AsyncDiv64 = 0b11001,
    /// Asynchronous clock mode, ADC clock = PLL"P"/128
    AsyncDiv128 = 0b11010,
    /// Asynchronous clock mode, ADC clock = PLL"P"/256
    AsyncDiv256 = 0b11011,
}

/// ADC Configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// ADC resolution
    pub resolution: Resolution,
    /// Clock prescaler
    pub clock_prescaler: ClockPrescaler,
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
            clock_prescaler: ClockPrescaler::Div4,
            continuous: false,
            scan_mode: false,
            dma_enable: false,
            overrun_mode: false,
        }
    }
}

/// ADC Channel
//! Reference: RM0456 Section 33.3.8
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Channel {
    /// ADC1_IN0 - PA0
    Channel0 = 0,
    /// ADC1_IN1 - PA1
    Channel1 = 1,
    /// ADC1_IN2 - PA2
    Channel2 = 2,
    /// ADC1_IN3 - PA3
    Channel3 = 3,
    /// ADC1_IN4 - PA4
    Channel4 = 4,
    /// ADC1_IN5 - PA5
    Channel5 = 5,
    /// ADC1_IN6 - PA6
    Channel6 = 6,
    /// ADC1_IN7 - PA7
    Channel7 = 7,
    /// ADC1_IN8 - PC0
    Channel8 = 8,
    /// ADC1_IN9 - PC1
    Channel9 = 9,
    /// ADC1_IN10 - PC2
    Channel10 = 10,
    /// ADC1_IN11 - PC3
    Channel11 = 11,
    /// ADC1_IN12 - PB0
    Channel12 = 12,
    /// ADC1_IN13 - PB1
    Channel13 = 13,
    /// ADC1_IN14 - PB2
    Channel14 = 14,
    /// ADC1_IN15 - PB4 (or PA15)
    Channel15 = 15,
    /// Internal temperature sensor
    TemperatureSensor = 16,
    /// Internal voltage reference (VREFINT)
    Vrefint = 17,
    /// VBAT channel
    Vbat = 18,
}

/// ADC Instance
pub struct Adc {
    base: usize,
}

impl Adc {
    /// Create ADC1 instance
    pub const fn adc1() -> Self {
        Self { base: ADC1_BASE }
    }

    /// Create ADC2 instance
    pub const fn adc2() -> Self {
        Self { base: ADC2_BASE }
    }

    /// Create ADC4 instance
    pub const fn adc4() -> Self {
        Self { base: ADC4_BASE }
    }

    /// Initialize ADC
    //! Reference: RM0456 Section 33.3.1
    pub fn init(&self, config: &Config) {
        unsafe {
            // Enable voltage regulator
            // Reference: RM0456 Section 33.3.1
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::ADVREGEN;
            core::ptr::write_volatile(cr, val);

            // Wait for regulator startup (typical 10us)
            for _ in 0..1000 {
                core::arch::asm!("nop");
            }

            // Configure common clock prescaler
            // Reference: RM0456 Section 33.3.2
            let ccr = (ADC1_COMMON_BASE + common_reg::CCR) as *mut u32;
            let mut ccr_val = core::ptr::read_volatile(ccr);
            ccr_val &= !ccr_bits::PRESC;
            ccr_val |= (config.clock_prescaler as u32) << 18;
            core::ptr::write_volatile(ccr, ccr_val);

            // Configure ADC
            // Reference: RM0456 Section 33.4.4
            let cfgr = (self.base + reg::CFGR) as *mut u32;
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
            core::ptr::write_volatile(cfgr, cfgr_val);

            // Calibrate ADC
            // Reference: RM0456 Section 33.3.12
            self.calibrate();

            // Enable ADC
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::ADEN;
            core::ptr::write_volatile(cr, val);

            // Wait for ADC ready
            while !self.is_ready() {}
        }
    }

    /// Calibrate ADC
    //! Reference: RM0456 Section 33.3.12
    pub fn calibrate(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;

            // Start calibration
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::ADCAL;
            core::ptr::write_volatile(cr, val);

            // Wait for calibration complete
            while (core::ptr::read_volatile(cr) & cr_bits::ADCAL) != 0 {}
        }
    }

    /// Check if ADC is ready
    //! Reference: RM0456 Section 33.4.1
    pub fn is_ready(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & isr_bits::ADRDY) != 0
        }
    }

    /// Set sample time for a channel
    //! Reference: RM0456 Section 33.3.5
    pub fn set_sample_time(&self, channel: Channel, sample_time: SampleTime) {
        unsafe {
            let ch_num = channel as u8;
            if ch_num < 10 {
                let smpr1 = (self.base + reg::SMPR1) as *mut u32;
                let mut val = core::ptr::read_volatile(smpr1);
                let pos = ch_num * 3;
                val &= !(0b111 << pos);
                val |= (sample_time as u32) << pos;
                core::ptr::write_volatile(smpr1, val);
            } else {
                let smpr2 = (self.base + reg::SMPR2) as *mut u32;
                let mut val = core::ptr::read_volatile(smpr2);
                let pos = (ch_num - 10) * 3;
                val &= !(0b111 << pos);
                val |= (sample_time as u32) << pos;
                core::ptr::write_volatile(smpr2, val);
            }
        }
    }

    /// Configure regular sequence (single channel)
    //! Reference: RM0456 Section 33.3.6
    pub fn configure_sequence(&self, channel: Channel) {
        unsafe {
            let sqr1 = (self.base + reg::SQR1) as *mut u32;
            let mut val = core::ptr::read_volatile(sqr1);
            val &= !(0x1F << 6);
            val |= (channel as u32) << 6;
            val &= !(0xF << 0);
            val |= 0 << 0;
            core::ptr::write_volatile(sqr1, val);
        }
    }

    /// Start conversion
    //! Reference: RM0456 Section 33.3.3
    pub fn start_conversion(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::ADSTART;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Stop conversion
    pub fn stop_conversion(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::ADSTP;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Check if conversion is complete
    //! Reference: RM0456 Section 33.4.1
    pub fn is_conversion_complete(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & isr_bits::EOC) != 0
        }
    }

    /// Read conversion result
    //! Reference: RM0456 Section 33.4.8
    pub fn read(&self) -> u16 {
        unsafe {
            let dr = (self.base + reg::DR) as *mut u32;
            core::ptr::read_volatile(dr) as u16
        }
    }

    /// Single conversion (blocking)
    pub fn convert(&self, channel: Channel) -> u16 {
        self.configure_sequence(channel);
        self.start_conversion();
        while !self.is_conversion_complete() {}
        self.read()
    }

    /// Read temperature sensor (in millidegrees Celsius)
    //! Reference: RM0456 Section 33.3.10
    pub fn read_temperature(&self) -> i32 {
        unsafe {
            // Enable temperature sensor
            let ccr = (ADC1_COMMON_BASE + common_reg::CCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ccr);
            val |= ccr_bits::VSENSEEN;
            core::ptr::write_volatile(ccr, val);
        }

        // Set long sample time for temperature sensor
        self.set_sample_time(Channel::TemperatureSensor, SampleTime::Cycles640_5);

        // Read raw value
        let raw = self.convert(Channel::TemperatureSensor);

        // TS_CAL1 at 30°C, TS_CAL2 at 130°C (stored in ROM)
        let ts_cal1 = unsafe { core::ptr::read_volatile(0x0BFA_0708 as *const u16) };
        let ts_cal2 = unsafe { core::ptr::read_volatile(0x0BFA_070A as *const u16) };

        // Temperature formula: T = (130 - 30) * (raw - TS_CAL1) / (TS_CAL2 - TS_CAL1) + 30
        let temp = ((130 - 30) * (raw as i32 - ts_cal1 as i32) * 1000)
            / (ts_cal2 as i32 - ts_cal1 as i32)
            + 30000;

        temp
    }

    /// Read internal voltage reference (in millivolts)
    //! Reference: RM0456 Section 33.3.11
    pub fn read_vrefint(&self) -> u32 {
        unsafe {
            // Enable VREFINT
            let ccr = (ADC1_COMMON_BASE + common_reg::CCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ccr);
            val |= ccr_bits::VREFEN;
            core::ptr::write_volatile(ccr, val);
        }

        // Set long sample time
        self.set_sample_time(Channel::Vrefint, SampleTime::Cycles640_5);

        // Read raw value
        let raw = self.convert(Channel::Vrefint);

        // VREFINT_CAL is the ADC raw value at 3.0V (stored in ROM)
        let vrefint_cal = unsafe { core::ptr::read_volatile(0x0BFA_0700 as *const u16) };

        // Calculate VDD: VDD = 3.0V * VREFINT_CAL / raw
        let vdd = (3000u32 * vrefint_cal as u32) / raw as u32;

        vdd
    }
}

/// Initialize ADC1 with default configuration
pub fn init_adc1_default() {
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2_2::ADC1);

    let adc = Adc::adc1();
    let config = Config::default();
    adc.init(&config);
}

/// Initialize ADC4 with default configuration
pub fn init_adc4_default() {
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2_2::ADC4);

    let adc = Adc::adc4();
    let config = Config::default();
    adc.init(&config);
}

/// GPIO to ADC channel mapping helper
//! Reference: RM0456 Chapter 13: GPIO
pub fn gpio_to_channel(port: u8, pin: u8) -> Option<Channel> {
    match (port, pin) {
        (0, 0) => Some(Channel::Channel0),  // PA0
        (0, 1) => Some(Channel::Channel1),  // PA1
        (0, 2) => Some(Channel::Channel2),  // PA2
        (0, 3) => Some(Channel::Channel3),  // PA3
        (0, 4) => Some(Channel::Channel4),  // PA4
        (0, 5) => Some(Channel::Channel5),  // PA5
        (0, 6) => Some(Channel::Channel6),  // PA6
        (0, 7) => Some(Channel::Channel7),  // PA7
        (2, 0) => Some(Channel::Channel8),  // PC0
        (2, 1) => Some(Channel::Channel9),  // PC1
        (2, 2) => Some(Channel::Channel10), // PC2
        (2, 3) => Some(Channel::Channel11), // PC3
        (1, 0) => Some(Channel::Channel12), // PB0
        (1, 1) => Some(Channel::Channel13), // PB1
        (1, 2) => Some(Channel::Channel14), // PB2
        (1, 4) => Some(Channel::Channel15), // PB4
        _ => None,
    }
}
