//! ADC - Analog-to-Digital Converter
//! 模数转换器
//!
//! STM32U5 具有两个 ADC 模块：
//! - ADC1/ADC2: 主 ADC，支持 12-bit 分辨率，最高 4 Msps
//! - ADC4: 低功耗 ADC，支持 12-bit 分辨率
//!
//! 支持多种转换模式：单次、连续、扫描、间断模式

/// ADC1 base address
pub const ADC1_BASE: usize = 0x4202_8000;
/// ADC2 base address
pub const ADC2_BASE: usize = 0x4202_8100;
/// ADC4 base address
pub const ADC4_BASE: usize = 0x4202_8C00;
/// ADC common registers base address
pub const ADC1_COMMON_BASE: usize = 0x4202_8300;

/// ADC register offsets
pub mod reg {
    /// ADC interrupt and status register
    pub const ISR: usize = 0x00;
    /// ADC interrupt enable register
    pub const IER: usize = 0x04;
    /// ADC control register
    pub const CR: usize = 0x08;
    /// ADC configuration register
    pub const CFGR: usize = 0x0C;
    /// ADC configuration register 2
    pub const CFGR2: usize = 0x10;
    /// ADC sample time register 1
    pub const SMPR1: usize = 0x14;
    /// ADC sample time register 2
    pub const SMPR2: usize = 0x18;
    /// ADC watchdog threshold register 1
    pub const TR1: usize = 0x20;
    /// ADC watchdog threshold register 2
    pub const TR2: usize = 0x24;
    /// ADC watchdog threshold register 3
    pub const TR3: usize = 0x28;
    /// ADC regular sequence register 1
    pub const SQR1: usize = 0x30;
    /// ADC regular sequence register 2
    pub const SQR2: usize = 0x34;
    /// ADC regular sequence register 3
    pub const SQR3: usize = 0x38;
    /// ADC regular sequence register 4
    pub const SQR4: usize = 0x3C;
    /// ADC regular data register
    pub const DR: usize = 0x40;
    /// ADC injected sequence register
    pub const JSQR: usize = 0x4C;
    /// ADC offset register 1
    pub const OFR1: usize = 0x60;
    /// ADC offset register 2
    pub const OFR2: usize = 0x64;
    /// ADC offset register 3
    pub const OFR3: usize = 0x68;
    /// ADC offset register 4
    pub const OFR4: usize = 0x6C;
    /// ADC injected data register 1
    pub const JDR1: usize = 0x80;
    /// ADC injected data register 2
    pub const JDR2: usize = 0x84;
    /// ADC injected data register 3
    pub const JDR3: usize = 0x88;
    /// ADC injected data register 4
    pub const JDR4: usize = 0x8C;
    /// ADC analog watchdog 2 configuration register
    pub const AWD2CR: usize = 0xA0;
    /// ADC analog watchdog 3 configuration register
    pub const AWD3CR: usize = 0xA4;
    /// ADC differential mode selection register
    pub const DIFSEL: usize = 0xB0;
    /// ADC calibration factors register
    pub const CALFACT: usize = 0xB4;
}

/// ADC common register offsets
pub mod common_reg {
    /// ADC common status register
    pub const CSR: usize = 0x00;
    /// ADC common control register
    pub const CCR: usize = 0x08;
    /// ADC common data register for dual mode
    pub const CDR: usize = 0x0C;
    /// ADC hardware configuration register
    pub const HWCFGR0: usize = 0x10;
    /// ADC hardware configuration register 1
    pub const HWCFGR1: usize = 0x14;
    /// ADC hardware configuration register 2
    pub const HWCFGR2: usize = 0x18;
    /// ADC version register
    pub const VERR: usize = 0x3F4;
    /// ADC identification register
    pub const IPDR: usize = 0x3F8;
    /// ADC size ID register
    pub const SIDR: usize = 0x3FC;
}

/// ADC resolution
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Resolution {
    /// 12-bit resolution
    Bits12 = 0b00,
    /// 10-bit resolution
    Bits10 = 0b01,
    /// 8-bit resolution
    Bits8 = 0b10,
    /// 6-bit resolution
    Bits6 = 0b11,
}

/// ADC sample time cycles
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

/// ADC clock prescaler
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockPrescaler {
    /// Synchronous clock mode, ADC clock = AHB/1
    Div1 = 0b00000,
    /// Synchronous clock mode, ADC clock = AHB/2
    Div2 = 0b00001,
    /// Synchronous clock mode, ADC clock = AHB/4
    Div4 = 0b00010,
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

/// ADC configuration
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
    /// Overrun mode (0=overwrite, 1=preserve)
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

/// ADC channel
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
    /// ADC1_IN8 - PA8
    Channel8 = 8,
    /// ADC1_IN9 - PA9
    Channel9 = 9,
    /// ADC1_IN10 - PA10
    Channel10 = 10,
    /// ADC1_IN11 - PA11
    Channel11 = 11,
    /// ADC1_IN12 - PA12
    Channel12 = 12,
    /// ADC1_IN13 - PA13
    Channel13 = 13,
    /// ADC1_IN14 - PA14
    Channel14 = 14,
    /// ADC1_IN15 - PA15
    Channel15 = 15,
    /// ADC1_IN16 - Internal temperature sensor
    TemperatureSensor = 16,
    /// ADC1_IN17 - Internal voltage reference
    Vrefint = 17,
    /// ADC1_IN18 - VBAT channel
    Vbat = 18,
}

/// ADC instance
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
    pub fn init(&self, config: &Config) {
        unsafe {
            // Enable voltage regulator
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 28; // ADVREGEN
            core::ptr::write_volatile(cr, val);

            // Wait for regulator startup
            for _ in 0..1000 {
                core::arch::asm!("nop");
            }

            // Configure common clock prescaler
            let ccr = (ADC1_COMMON_BASE + common_reg::CCR) as *mut u32;
            let mut ccr_val = core::ptr::read_volatile(ccr);
            ccr_val &= !(0b11111 << 18); // Clear PRESC
            ccr_val |= (config.clock_prescaler as u32) << 18;
            core::ptr::write_volatile(ccr, ccr_val);

            // Configure ADC
            let cfgr = (self.base + reg::CFGR) as *mut u32;
            let mut cfgr_val = 0;
            cfgr_val |= (config.resolution as u32) << 3;
            if config.continuous {
                cfgr_val |= 1 << 13; // CONT
            }
            if config.dma_enable {
                cfgr_val |= 1 << 0; // DMAEN
            }
            if !config.overrun_mode {
                cfgr_val |= 1 << 12; // OVRMOD = 1 (overwrite)
            }
            core::ptr::write_volatile(cfgr, cfgr_val);

            // Calibrate ADC
            self.calibrate();

            // Enable ADC
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // ADEN
            core::ptr::write_volatile(cr, val);

            // Wait for ADC ready
            while !self.is_ready() {}
        }
    }

    /// Calibrate ADC
    pub fn calibrate(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;

            // Start calibration
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 31; // ADCAL
            core::ptr::write_volatile(cr, val);

            // Wait for calibration complete
            while (core::ptr::read_volatile(cr) & (1 << 31)) != 0 {}
        }
    }

    /// Check if ADC is ready
    pub fn is_ready(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & (1 << 0)) != 0 // ADRDY
        }
    }

    /// Set sample time for a channel
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
    pub fn configure_sequence(&self, channel: Channel) {
        unsafe {
            let sqr1 = (self.base + reg::SQR1) as *mut u32;
            let mut val = core::ptr::read_volatile(sqr1);
            val &= !(0x1F << 6); // Clear SQ1
            val |= (channel as u32) << 6; // Set SQ1
            val &= !(0xF << 0); // Clear L (sequence length)
            val |= 0 << 0; // L = 0 (1 conversion)
            core::ptr::write_volatile(sqr1, val);
        }
    }

    /// Start conversion
    pub fn start_conversion(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 2; // ADSTART
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Stop conversion
    pub fn stop_conversion(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 4; // ADSTP
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Check if conversion is complete
    pub fn is_conversion_complete(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & (1 << 2)) != 0 // EOC
        }
    }

    /// Read conversion result
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
    pub fn read_temperature(&self) -> i32 {
        // Enable temperature sensor
        unsafe {
            let ccr = (ADC1_COMMON_BASE + common_reg::CCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ccr);
            val |= 1 << 23; // VSENSEEN
            core::ptr::write_volatile(ccr, val);
        }

        // Set long sample time for temperature sensor
        self.set_sample_time(Channel::TemperatureSensor, SampleTime::Cycles640_5);

        // Read raw value
        let raw = self.convert(Channel::TemperatureSensor);

        // Convert to temperature
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
    pub fn read_vrefint(&self) -> u32 {
        // Enable VREFINT
        unsafe {
            let ccr = (ADC1_COMMON_BASE + common_reg::CCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ccr);
            val |= 1 << 22; // VREFEN
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
    // Enable ADC1 clock
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::ADC12);

    let adc = Adc::adc1();
    let config = Config::default();
    adc.init(&config);
}

/// GPIO to ADC channel mapping helper
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
        (0, 8) => Some(Channel::Channel8),  // PA8
        (0, 9) => Some(Channel::Channel9),  // PA9
        (0, 10) => Some(Channel::Channel10), // PA10
        (0, 11) => Some(Channel::Channel11), // PA11
        (0, 12) => Some(Channel::Channel12), // PA12
        (0, 13) => Some(Channel::Channel13), // PA13
        (0, 14) => Some(Channel::Channel14), // PA14
        (0, 15) => Some(Channel::Channel15), // PA15
        _ => None,
    }
}
