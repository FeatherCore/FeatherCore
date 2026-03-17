//! LPTIM - Low Power Timer
//! 低功耗定时器
//!
//! # Overview / 概述
//! STM32U5 Low Power Timer (LPTIM) provides up to 6 independent low-power
//! timer modules that can operate in Stop mode.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 58: Low-power timer (LPTIM)
//!
//! ## Timer Count / 定时器数量
//! - **LPTIM1-LPTIM6**: 6 independent low-power timers
//!
//! ## Counter / 计数器
//! - 16-bit up/down counter
//! - 32-bit resolution with repetition register
//!
//! ## Clock Sources / 时钟源
//! Reference: RM0456 Section 58.3.2: Clock source selection
//! - **LSI**: Low Speed Internal (32 kHz)
//! - **LSE**: Low Speed External (32.768 kHz)
//! - **PCLK**: Peripheral Clock
//! - **HSI16**: High Speed Internal (16 MHz)
//!
//! ## Modes / 工作模式
//! - Encoder mode
//! - Timeout function
//! - PWM output
//! - One-shot mode
//! - Continuous mode
//!
//! ## Features / 特性
//! - Operates in Stop mode
//! - Low power consumption
//! - External input capture
//! - Internal trigger generation
//!
//! # Reference / 参考
//! - RM0456 Chapter 58: Low-power timer (LPTIM)
//! - RM0456 Section 58.1: LPTIM introduction
//! - RM0456 Section 58.2: LPTIM main features
//! - RM0456 Section 58.3: LPTIM functional description
//! - RM0456 Section 58.4: LPTIM registers

#![no_std]

/// LPTIM1 base address (APB1)
//! Reference: RM0456 Chapter 2, Table 1
pub const LPTIM1_BASE: usize = 0x4000_7C00;

/// LPTIM2 base address (APB1)
pub const LPTIM2_BASE: usize = 0x4000_9400;

/// LPTIM3 base address (APB1)
pub const LPTIM3_BASE: usize = 0x4000_9800;

/// LPTIM4 base address (APB1)
pub const LPTIM4_BASE: usize = 0x4000_9C00;

/// LPTIM5 base address (APB1)
pub const LPTIM5_BASE: usize = 0x4000_A000;

/// LPTIM6 base address (APB1)
pub const LPTIM6_BASE: usize = 0x4000_A400;

/// LPTIM register offsets
//! Reference: RM0456 Section 58.4: LPTIM registers
pub mod reg {
    /// LPTIM interrupt and status register
    //! Reference: RM0456 Section 58.4.1: LPTIM_ISR
    pub const ISR: usize = 0x00;

    /// LPTIM interrupt clear register
    //! Reference: RM0456 Section 58.4.2: LPTIM_ICR
    pub const ICR: usize = 0x04;

    /// LPTIM interrupt enable register
    //! Reference: RM0456 Section 58.4.3: LPTIM_IER
    pub const IER: usize = 0x08;

    /// LPTIM configuration register
    //! Reference: RM0456 Section 58.4.4: LPTIM_CFGR
    pub const CFGR: usize = 0x0C;

    /// LPTIM control register
    //! Reference: RM0456 Section 58.4.5: LPTIM_CR
    pub const CR: usize = 0x10;

    /// LPTIM compare register
    //! Reference: RM0456 Section 58.4.6: LPTIM_CMP
    pub const CMP: usize = 0x14;

    /// LPTIM auto-reload register
    //! Reference: RM0456 Section 58.4.7: LPTIM_ARR
    pub const ARR: usize = 0x18;

    /// LPTIM counter register
    //! Reference: RM0456 Section 58.4.8: LPTIM_CNT
    pub const CNT: usize = 0x1C;

    /// LPTIM option register
    //! Reference: RM0456 Section 58.4.9: LPTIM_OR
    pub const OR: usize = 0x20;

    /// LPTIM repetition counter register
    //! Reference: RM0456 Section 58.4.10: LPTIM_RCR
    pub const RCR: usize = 0x24;

    /// LPTIM capture/compare mode register 1
    //! Reference: RM0456 Section 58.4.11: LPTIM_CCMR1
    pub const CCMR1: usize = 0x28;

    /// LPTIM capture/compare register 1
    //! Reference: RM0456 Section 58.4.12: LPTIM_CCR1
    pub const CCR1: usize = 0x2C;
}

/// CFGR Register Bit Definitions
//! Reference: RM0456 Section 58.4.4: LPTIM_CFGR
pub mod cfgr_bits {
    /// Clock source selection
    pub const CKSEL: u32 = 0b11 << 0;
    /// Clock prescaler
    pub const PRESC: u32 = 0b111 << 3;
    /// Input filter
    pub const TRGFLT: u32 = 0b11 << 6;
    /// Waveform generation polarity
    pub const WAVEPOL: u32 = 1 << 17;
    /// Preload enable
    pub const PRELOAD: u32 = 1 << 22;
}

/// CR Register Bit Definitions
//! Reference: RM0456 Section 58.4.5: LPTIM_CR
pub mod cr_bits {
    /// LPTIM enable
    pub const ENABLE: u32 = 1 << 0;
    /// Single shot start
    pub const SNGSTRT: u32 = 1 << 1;
    /// Continuous start
    pub const CNTSTRT: u32 = 1 << 2;
}

/// ISR Register Bit Definitions
//! Reference: RM0456 Section 58.4.1: LPTIM_ISR
pub mod isr_bits {
    /// Compare match
    pub const CMPM: u32 = 1 << 0;
    /// Autoreload match
    pub const ARRM: u32 = 1 << 1;
    /// External trigger edge
    pub const EXTTRIG: u32 = 1 << 2;
    /// Compare register update
    pub const CMPOK: u32 = 1 << 3;
    /// Autoreload register update
    pub const ARROK: u32 = 1 << 4;
    /// Counter direction change
    pub const UP: u32 = 1 << 5;
    /// Counter overflow
    pub const DOWN: u32 = 1 << 6;
}

/// LPTIM clock source
//! Reference: RM0456 Section 58.3.2: Clock source selection
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
//! Reference: RM0456 Section 58.3.2: Clock prescaler
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
//! Reference: RM0456 Section 58.3.3: Input filtering
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
    /// Create LPTIM1 instance
    pub const fn lptim1() -> Self {
        Self { base: LPTIM1_BASE }
    }

    /// Create LPTIM2 instance
    pub const fn lptim2() -> Self {
        Self { base: LPTIM2_BASE }
    }

    /// Create LPTIM3 instance
    pub const fn lptim3() -> Self {
        Self { base: LPTIM3_BASE }
    }

    /// Create LPTIM4 instance
    pub const fn lptim4() -> Self {
        Self { base: LPTIM4_BASE }
    }

    /// Create LPTIM5 instance
    pub const fn lptim5() -> Self {
        Self { base: LPTIM5_BASE }
    }

    /// Create LPTIM6 instance
    pub const fn lptim6() -> Self {
        Self { base: LPTIM6_BASE }
    }

    /// Initialize LPTIM
    //! Reference: RM0456 Section 58.3.1: LPTIM initialization
    pub fn init(&self, config: &Config) {
        unsafe {
            // Configure
            // Reference: RM0456 Section 58.4.4: LPTIM_CFGR
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
    //! Reference: RM0456 Section 58.4.5: LPTIM_CR
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::ENABLE;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable LPTIM
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !cr_bits::ENABLE;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Start continuous mode
    //! Reference: RM0456 Section 58.3.6: Continuous mode
    pub fn start_continuous(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::CNTSTRT;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Start single shot mode
    //! Reference: RM0456 Section 58.3.5: One-shot mode
    pub fn start_single(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= cr_bits::SNGSTRT;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Set auto-reload value
    //! Reference: RM0456 Section 58.4.7: LPTIM_ARR
    pub fn set_auto_reload(&self, value: u16) {
        unsafe {
            let arr = (self.base + reg::ARR) as *mut u32;
            core::ptr::write_volatile(arr, value as u32);
        }
    }

    /// Set compare value
    //! Reference: RM0456 Section 58.4.6: LPTIM_CMP
    pub fn set_compare(&self, value: u16) {
        unsafe {
            let cmp = (self.base + reg::CMP) as *mut u32;
            core::ptr::write_volatile(cmp, value as u32);
        }
    }

    /// Set repetition counter
    //! Reference: RM0456 Section 58.4.10: LPTIM_RCR
    pub fn set_repetition(&self, value: u8) {
        unsafe {
            let rcr = (self.base + reg::RCR) as *mut u32;
            core::ptr::write_volatile(rcr, value as u32);
        }
    }

    /// Get counter value
    //! Reference: RM0456 Section 58.4.8: LPTIM_CNT
    pub fn get_counter(&self) -> u16 {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            core::ptr::read_volatile(cnt) as u16
        }
    }

    /// Check if counter matches compare value
    //! Reference: RM0456 Section 58.4.1: LPTIM_ISR
    pub fn is_match(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & isr_bits::CMPM) != 0
        }
    }

    /// Check if autoreload match
    pub fn is_autoreload_match(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & isr_bits::ARRM) != 0
        }
    }

    /// Clear match flag
    //! Reference: RM0456 Section 58.4.2: LPTIM_ICR
    pub fn clear_match(&self) {
        unsafe {
            let icr = (self.base + reg::ICR) as *mut u32;
            core::ptr::write_volatile(icr, isr_bits::CMPM);
        }
    }

    /// Clear autoreload flag
    pub fn clear_autoreload(&self) {
        unsafe {
            let icr = (self.base + reg::ICR) as *mut u32;
            core::ptr::write_volatile(icr, isr_bits::ARRM);
        }
    }

    /// Enable interrupt
    //! Reference: RM0456 Section 58.4.3: LPTIM_IER
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

    /// Disable interrupt
    pub fn disable_interrupt(&self) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            core::ptr::write_volatile(ier, 0);
        }
    }

    /// Configure PWM mode
    //! Reference: RM0456 Section 58.3.7: PWM mode
    pub fn configure_pwm(&self, period: u16, duty: u16) {
        self.set_auto_reload(period);
        self.set_compare(duty);
    }
}

/// Initialize LPTIM1 for low power delay (using LSE 32.768 kHz)
//! Reference: RM0456 Section 58.3.4: Timeout function
pub fn init_lptim1_delay() {
    // Enable LPTIM1 clock
    // Reference: RM0456 Section 11.10.5: RCC_APB1ENR2
    crate::rcc::enable_apb1_clock(crate::rcc::apb1_2::LPTIM1);

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
//! Reference: RM0456 Section 58.3.7: PWM mode
pub fn init_lptim2_pwm(frequency_hz: u32, duty_cycle_percent: u8) {
    // Enable LPTIM2 clock
    // Reference: RM0456 Section 11.10.5: RCC_APB1ENR2
    crate::rcc::enable_apb1_clock(crate::rcc::apb1_2::LPTIM2);

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

    lptim.configure_pwm(period, duty);

    lptim.enable();
    lptim.start_continuous();
}

/// Initialize LPTIM3 for encoder mode
//! Reference: RM0456 Section 58.3.8: Encoder mode
pub fn init_lptim3_encoder() {
    // Enable LPTIM3 clock
    // Reference: RM0456 Section 11.10.5: RCC_APB1ENR2
    crate::rcc::enable_apb1_clock(crate::rcc::apb1_2::LPTIM3);

    let lptim = Lptim::lptim3();
    let config = Config {
        clock_source: ClockSource::Apb,
        prescaler: Prescaler::Div1,
        filter: Filter::Clocks4,
        ..Default::default()
    };

    lptim.init(&config);
    lptim.enable();
}

/// Get encoder count from LPTIM3
pub fn get_encoder_count() -> u16 {
    let lptim = Lptim::lptim3();
    lptim.get_counter()
}
