//! LPTIM - Low Power Timer
//! 低功耗定时器
//!
//! STM32U5 LPTIM 特性：
//! - 6 个独立低功耗定时器 (LPTIM1-6)
//! - 支持 16-bit 计数器
//! - 支持多种时钟源 (LSI, LSE, PCLK, HSI)
//! - 支持编码器模式
//! - 支持超时功能
//! - 在 Stop 模式下继续运行

/// LPTIM1 base address
pub const LPTIM1_BASE: usize = 0x4000_7C00;
/// LPTIM2 base address
pub const LPTIM2_BASE: usize = 0x4000_9400;
/// LPTIM3 base address
pub const LPTIM3_BASE: usize = 0x4000_9800;
/// LPTIM4 base address
pub const LPTIM4_BASE: usize = 0x4000_9C00;
/// LPTIM5 base address
pub const LPTIM5_BASE: usize = 0x4000_A000;
/// LPTIM6 base address
pub const LPTIM6_BASE: usize = 0x4000_A400;

/// LPTIM register offsets
pub mod reg {
    pub const ISR: usize = 0x00;
    pub const ICR: usize = 0x04;
    pub const IER: usize = 0x08;
    pub const CFGR: usize = 0x0C;
    pub const CR: usize = 0x10;
    pub const CMP: usize = 0x14;
    pub const ARR: usize = 0x18;
    pub const CNT: usize = 0x1C;
    pub const OR: usize = 0x20;
    pub const RCR: usize = 0x24;
    pub const CCMR1: usize = 0x28;
    pub const CCR1: usize = 0x2C;
}

/// LPTIM clock source
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockSource {
    /// APB clock
    Apb = 0b00,
    /// LSI clock
    Lsi = 0b01,
    /// HSI16 clock
    Hsi16 = 0b10,
    /// LSE clock
    Lse = 0b11,
}

/// LPTIM prescaler
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prescaler {
    /// Divide by 1
    Div1 = 0b000,
    /// Divide by 2
    Div2 = 0b001,
    /// Divide by 4
    Div4 = 0b010,
    /// Divide by 8
    Div8 = 0b011,
    /// Divide by 16
    Div16 = 0b100,
    /// Divide by 32
    Div32 = 0b101,
    /// Divide by 64
    Div64 = 0b110,
    /// Divide by 128
    Div128 = 0b111,
}

/// LPTIM filter
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    /// Any external input change is considered valid
    None = 0b00,
    /// External input change must be stable for 2 clock periods
    Clocks2 = 0b01,
    /// External input change must be stable for 4 clock periods
    Clocks4 = 0b10,
    /// External input change must be stable for 8 clock periods
    Clocks8 = 0b11,
}

/// LPTIM configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub clock_source: ClockSource,
    pub prescaler: Prescaler,
    pub filter: Filter,
    pub waveform_polarity: bool, // true = inverted
    pub preload_enable: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock_source: ClockSource::Lse,
            prescaler: Prescaler::Div1,
            filter: Filter::None,
            waveform_polarity: false,
            preload_enable: true,
        }
    }
}

/// LPTIM instance
pub struct Lptim {
    base: usize,
}

impl Lptim {
    pub const fn lptim1() -> Self {
        Self { base: LPTIM1_BASE }
    }

    pub const fn lptim2() -> Self {
        Self { base: LPTIM2_BASE }
    }

    pub const fn lptim3() -> Self {
        Self { base: LPTIM3_BASE }
    }

    pub const fn lptim4() -> Self {
        Self { base: LPTIM4_BASE }
    }

    pub const fn lptim5() -> Self {
        Self { base: LPTIM5_BASE }
    }

    pub const fn lptim6() -> Self {
        Self { base: LPTIM6_BASE }
    }

    /// Initialize LPTIM
    pub fn init(&self, config: &Config) {
        unsafe {
            // Configure
            let cfgr = (self.base + reg::CFGR) as *mut u32;
            let mut val = 0;
            val |= (config.clock_source as u32) << 0;
            val |= (config.prescaler as u32) << 3;
            val |= (config.filter as u32) << 6;
            if config.waveform_polarity {
                val |= 1 << 17;
            }
            if config.preload_enable {
                val |= 1 << 22;
            }
            core::ptr::write_volatile(cfgr, val);
        }
    }

    /// Enable LPTIM
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // ENABLE
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable LPTIM
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 0);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Start continuous mode
    pub fn start_continuous(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 2; // CNTSTRT
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Start single shot mode
    pub fn start_single(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 1; // SNGSTRT
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Set auto-reload value
    pub fn set_auto_reload(&self, value: u16) {
        unsafe {
            let arr = (self.base + reg::ARR) as *mut u32;
            core::ptr::write_volatile(arr, value as u32);
        }
    }

    /// Set compare value
    pub fn set_compare(&self, value: u16) {
        unsafe {
            let cmp = (self.base + reg::CMP) as *mut u32;
            core::ptr::write_volatile(cmp, value as u32);
        }
    }

    /// Get counter value
    pub fn get_counter(&self) -> u16 {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            core::ptr::read_volatile(cnt) as u16
        }
    }

    /// Check if counter matches compare value
    pub fn is_match(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & (1 << 0)) != 0 // CMPM
        }
    }

    /// Clear match flag
    pub fn clear_match(&self) {
        unsafe {
            let icr = (self.base + reg::ICR) as *mut u32;
            core::ptr::write_volatile(icr, 1 << 0);
        }
    }

    /// Enable interrupt
    pub fn enable_interrupt(&self, match_int: bool, arr_int: bool) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            let mut val = 0;
            if match_int {
                val |= 1 << 0;
            }
            if arr_int {
                val |= 1 << 1;
            }
            core::ptr::write_volatile(ier, val);
        }
    }
}

/// Initialize LPTIM1 for low power delay (using LSE 32.768 kHz)
pub fn init_lptim1_delay() {
    // Enable LPTIM1 clock
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::LPTIM1);

    let lptim = Lptim::lptim1();
    let config = Config {
        clock_source: ClockSource::Lse,
        prescaler: Prescaler::Div32, // 1 kHz
        ..Default::default()
    };

    lptim.init(&config);
    lptim.enable();
}

/// Delay using LPTIM1 (milliseconds)
pub fn delay_ms_lptim1(ms: u16) {
    let lptim = Lptim::lptim1();

    lptim.set_auto_reload(ms);
    lptim.start_single();

    // Wait for match
    while !lptim.is_match() {}
    lptim.clear_match();
}

/// Initialize LPTIM2 for PWM output
pub fn init_lptim2_pwm(frequency_hz: u32, duty_cycle_percent: u8) {
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::LPTIM2);

    let lptim = Lptim::lptim2();
    let config = Config {
        clock_source: ClockSource::Hsi16,
        prescaler: Prescaler::Div4,
        ..Default::default()
    };

    lptim.init(&config);

    // Calculate period and duty
    let period = (16000000 / 4 / frequency_hz) as u16;
    let duty = (period as u32 * duty_cycle_percent as u32 / 100) as u16;

    lptim.set_auto_reload(period);
    lptim.set_compare(duty);

    lptim.enable();
    lptim.start_continuous();
}
