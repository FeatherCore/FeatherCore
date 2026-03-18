//! TIMER - General Purpose Timers
//! 通用定时器
//!
//! # Overview / 概述
//! STM32U5 series features multiple timer types for various timing and
//! control applications.
//!
//! # Timer Types / 定时器类型
//! Reference: RM0456 Chapter 54-57
//!
//! ## Advanced-control Timers / 高级控制定时器
//! - **TIM1, TIM8**
//!   - 4 capture/compare channels
//!   - Complementary outputs
//!   - Dead-time insertion
//!   - PWM emergency stop
//!   - Encoder interface
//!   - Trigger input/output
//!
//! ## General Purpose Timers / 通用定时器
//! - **TIM2, TIM3, TIM4, TIM5**
//!   - 4 capture/compare channels
//!   - Input capture
//!   - Output compare
//!   - PWM output
//!   - Encoder interface
//!
//! - **TIM15, TIM16, TIM17**
//!   - 2 capture/compare channels
//!   - Simpler functionality
//!
//! ## Basic Timers / 基本定时器
//! - **TIM6, TIM7**
//!   - Simple timing functions
//!   - DAC trigger
//!
//! ## Low Power Timers / 低功耗定时器
//! - **LPTIM1-5** (see lptim.rs)
//!
//! # Reference / 参考
//! - RM0456 Chapter 54: Advanced-control timers (TIM1/TIM8)
//! - RM0456 Chapter 55: General-purpose timers (TIM2/TIM3/TIM4/TIM5)
//! - RM0456 Chapter 56: General-purpose timers (TIM15/TIM16/TIM17)
//! - RM0456 Chapter 57: Basic timers (TIM6/TIM7)

#![no_std]

/// TIM2 base address (APB1)
//! Reference: RM0456 Chapter 2, Table 1
pub const TIM2_BASE: usize = 0x4000_0000;

/// TIM3 base address (APB1)
pub const TIM3_BASE: usize = 0x4000_0400;

/// TIM4 base address (APB1)
pub const TIM4_BASE: usize = 0x4000_0800;

/// TIM5 base address (APB1)
pub const TIM5_BASE: usize = 0x4000_0C00;

/// TIM6 base address (APB1 - basic timer)
pub const TIM6_BASE: usize = 0x4000_1000;

/// TIM7 base address (APB1 - basic timer)
pub const TIM7_BASE: usize = 0x4000_1400;

/// TIM1 base address (APB2 - advanced timer)
pub const TIM1_BASE: usize = 0x4001_2C00;

/// TIM8 base address (APB2 - advanced timer)
pub const TIM8_BASE: usize = 0x4001_3400;

/// TIM15 base address (APB2 - general purpose timer)
pub const TIM15_BASE: usize = 0x4001_4000;

/// TIM16 base address (APB2 - general purpose timer)
pub const TIM16_BASE: usize = 0x4001_4400;

/// TIM17 base address (APB2 - general purpose timer)
pub const TIM17_BASE: usize = 0x4001_4800;

/// Timer register offsets
//! Reference: RM0456 Section 54.4 (TIM1), Section 55.4 (TIM2), Section 56.4 (TIM15), Section 57.4 (TIM6)
pub mod reg {
    /// Control register 1
    //! Reference: RM0456 Section 54.4.1: TIMx_CR1
    pub const CR1: usize = 0x00;

    /// Control register 2
    //! Reference: RM0456 Section 54.4.2: TIMx_CR2
    pub const CR2: usize = 0x04;

    /// Slave mode control register
    //! Reference: RM0456 Section 54.4.3: TIMx_SMCR
    pub const SMCR: usize = 0x08;

    /// DMA/interrupt enable register
    //! Reference: RM0456 Section 54.4.4: TIMx_DIER
    pub const DIER: usize = 0x0C;

    /// Status register
    //! Reference: RM0456 Section 54.4.5: TIMx_SR
    pub const SR: usize = 0x10;

    /// Event generation register
    //! Reference: RM0456 Section 54.4.6: TIMx_EGR
    pub const EGR: usize = 0x14;

    /// Capture/compare mode register 1
    //! Reference: RM0456 Section 54.4.7: TIMx_CCMR1
    pub const CCMR1: usize = 0x18;

    /// Capture/compare mode register 2
    //! Reference: RM0456 Section 54.4.8: TIMx_CCMR2
    pub const CCMR2: usize = 0x1C;

    /// Capture/compare enable register
    //! Reference: RM0456 Section 54.4.9: TIMx_CCER
    pub const CCER: usize = 0x20;

    /// Counter
    //! Reference: RM0456 Section 54.4.10: TIMx_CNT
    pub const CNT: usize = 0x24;

    /// Prescaler
    //! Reference: RM0456 Section 54.4.11: TIMx_PSC
    pub const PSC: usize = 0x28;

    /// Auto-reload register
    //! Reference: RM0456 Section 54.4.12: TIMx_ARR
    pub const ARR: usize = 0x2C;

    /// Repetition counter register
    //! Reference: RM0456 Section 54.4.13: TIMx_RCR
    pub const RCR: usize = 0x30;

    /// Capture/compare register 1
    //! Reference: RM0456 Section 54.4.14: TIMx_CCR1
    pub const CCR1: usize = 0x34;

    /// Capture/compare register 2
    //! Reference: RM0456 Section 54.4.15: TIMx_CCR2
    pub const CCR2: usize = 0x38;

    /// Capture/compare register 3
    //! Reference: RM0456 Section 54.4.16: TIMx_CCR3
    pub const CCR3: usize = 0x3C;

    /// Capture/compare register 4
    //! Reference: RM0456 Section 54.4.17: TIMx_CCR4
    pub const CCR4: usize = 0x40;

    /// Break and dead-time register (advanced timers only)
    //! Reference: RM0456 Section 54.4.18: TIMx_BDTR
    pub const BDTR: usize = 0x44;

    /// DMA control register
    //! Reference: RM0456 Section 54.4.19: TIMx_DCR
    pub const DCR: usize = 0x48;

    /// DMA address for full transfer
    //! Reference: RM0456 Section 54.4.20: TIMx_DMAR
    pub const DMAR: usize = 0x4C;

    /// Option register
    //! Reference: RM0456 Section 54.4.21: TIMx_OR
    pub const OR: usize = 0x50;
}

/// CR1 Register Bit Definitions
//! Reference: RM0456 Section 54.4.1
pub mod cr1_bits {
    /// Counter enable
    pub const CEN: u32 = 1 << 0;
    /// Update disable
    pub const UDIS: u32 = 1 << 1;
    /// Update request source
    pub const URS: u32 = 1 << 2;
    /// One pulse mode
    pub const OPM: u32 = 1 << 3;
    /// Direction
    pub const DIR: u32 = 1 << 4;
    /// Center-aligned mode
    pub const CMS: u32 = 0b11 << 5;
    /// Auto-reload preload enable
    pub const ARPE: u32 = 1 << 7;
    /// Clock division
    pub const CKD: u32 = 0b11 << 8;
}

/// DIER Register Bit Definitions
//! Reference: RM0456 Section 54.4.4
pub mod dier_bits {
    /// Update interrupt enable
    pub const UIE: u32 = 1 << 0;
    /// Capture/compare 1 interrupt enable
    pub const CC1IE: u32 = 1 << 1;
    /// Capture/compare 2 interrupt enable
    pub const CC2IE: u32 = 1 << 2;
    /// Capture/compare 3 interrupt enable
    pub const CC3IE: u32 = 1 << 3;
    /// Capture/compare 4 interrupt enable
    pub const CC4IE: u32 = 1 << 4;
    /// COM interrupt enable
    pub const COMINIE: u32 = 1 << 5;
    /// Trigger interrupt enable
    pub const TIE: u32 = 1 << 6;
    /// Break interrupt enable
    pub const BIE: u32 = 1 << 7;
    /// Update DMA request enable
    pub const UDE: u32 = 1 << 8;
    /// Capture/compare 1 DMA request enable
    pub const CC1DE: u32 = 1 << 9;
    /// Capture/compare 2 DMA request enable
    pub const CC2DE: u32 = 1 << 10;
    /// Capture/compare 3 DMA request enable
    pub const CC3DE: u32 = 1 << 11;
    /// Capture/compare 4 DMA request enable
    pub const CC4DE: u32 = 1 << 12;
    /// COM DMA request enable
    pub const COMDDE: u32 = 1 << 13;
    /// Trigger DMA request enable
    pub const TDE: u32 = 1 << 14;
}

/// SR Register Bit Definitions
//! Reference: RM0456 Section 54.4.5
pub mod sr_bits {
    /// Update interrupt flag
    pub const UIF: u32 = 1 << 0;
    /// Capture/compare 1 interrupt flag
    pub const CC1IF: u32 = 1 << 1;
    /// Capture/compare 2 interrupt flag
    pub const CC2IF: u32 = 1 << 2;
    /// Capture/compare 3 interrupt flag
    pub const CC3IF: u32 = 1 << 3;
    /// Capture/compare 4 interrupt flag
    pub const CC4IF: u32 = 1 << 4;
    /// COM interrupt flag
    pub const COMIF: u32 = 1 << 5;
    /// Trigger interrupt flag
    pub const TIF: u32 = 1 << 6;
    /// Break interrupt flag
    pub const BIF: u32 = 1 << 7;
    /// Overflow flag
    pub const OF: u32 = 1 << 8;
}

/// Timer configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Prescaler value (PSC)
    pub prescaler: u16,
    /// Auto-reload value (ARR)
    pub auto_reload: u32,
    /// One-pulse mode
    pub one_pulse: bool,
    /// Auto-reload preload enable
    pub auto_reload_preload: bool,
    /// Update disable
    pub update_disable: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prescaler: 0,
            auto_reload: 0xFFFF,
            one_pulse: false,
            auto_reload_preload: true,
            update_disable: false,
        }
    }
}

/// Timer instance
pub struct Timer {
    base: usize,
}

impl Timer {
    /// Create TIM2 instance
    pub const fn tim2() -> Self {
        Self { base: TIM2_BASE }
    }

    /// Create TIM3 instance
    pub const fn tim3() -> Self {
        Self { base: TIM3_BASE }
    }

    /// Create TIM4 instance
    pub const fn tim4() -> Self {
        Self { base: TIM4_BASE }
    }

    /// Create TIM5 instance
    pub const fn tim5() -> Self {
        Self { base: TIM5_BASE }
    }

    /// Create TIM6 instance (basic timer)
    pub const fn tim6() -> Self {
        Self { base: TIM6_BASE }
    }

    /// Create TIM7 instance (basic timer)
    pub const fn tim7() -> Self {
        Self { base: TIM7_BASE }
    }

    /// Create TIM1 instance (advanced timer)
    pub const fn tim1() -> Self {
        Self { base: TIM1_BASE }
    }

    /// Create TIM8 instance (advanced timer)
    pub const fn tim8() -> Self {
        Self { base: TIM8_BASE }
    }

    /// Create TIM15 instance
    pub const fn tim15() -> Self {
        Self { base: TIM15_BASE }
    }

    /// Create TIM16 instance
    pub const fn tim16() -> Self {
        Self { base: TIM16_BASE }
    }

    /// Create TIM17 instance
    pub const fn tim17() -> Self {
        Self { base: TIM17_BASE }
    }

    /// Initialize timer
    //! Reference: RM0456 Section 54.3.1: Basic timer configuration
    pub fn init(&self, config: &Config) {
        unsafe {
            // Disable timer before configuration
            // Reference: RM0456 Section 54.4.1, bit CEN
            let cr1 = (self.base + reg::CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 0);

            // Set prescaler
            // Reference: RM0456 Section 54.4.11: TIMx_PSC
            let psc = (self.base + reg::PSC) as *mut u32;
            core::ptr::write_volatile(psc, config.prescaler as u32);

            // Set auto-reload
            // Reference: RM0456 Section 54.4.12: TIMx_ARR
            let arr = (self.base + reg::ARR) as *mut u32;
            core::ptr::write_volatile(arr, config.auto_reload);

            // Configure CR1
            // Reference: RM0456 Section 54.4.1: TIMx_CR1
            let mut cr1_val = 0;
            if config.one_pulse {
                cr1_val |= cr1_bits::OPM;
            }
            if config.auto_reload_preload {
                cr1_val |= cr1_bits::ARPE;
            }
            if config.update_disable {
                cr1_val |= cr1_bits::UDIS;
            }
            core::ptr::write_volatile(cr1, cr1_val);

            // Generate update event to reload registers
            // Reference: RM0456 Section 54.4.6: TIMx_EGR
            let egr = (self.base + reg::EGR) as *mut u32;
            core::ptr::write_volatile(egr, 1 << 0); // UG

            // Clear update flag
            let sr = (self.base + reg::SR) as *mut u32;
            core::ptr::write_volatile(sr, 0);
        }
    }

    /// Start timer
    //! Reference: RM0456 Section 54.3.1: Start the timer
    pub fn start(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= cr1_bits::CEN;
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Stop timer
    pub fn stop(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val &= !cr1_bits::CEN;
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Reset counter
    pub fn reset(&self) {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            core::ptr::write_volatile(cnt, 0);
        }
    }

    /// Get current counter value
    //! Reference: RM0456 Section 54.4.10: TIMx_CNT
    pub fn get_count(&self) -> u32 {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            core::ptr::read_volatile(cnt)
        }
    }

    /// Set counter value
    pub fn set_count(&self, value: u32) {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            core::ptr::write_volatile(cnt, value);
        }
    }

    /// Check if update interrupt flag is set
    //! Reference: RM0456 Section 54.4.5: TIMx_SR
    pub fn is_update_interrupt(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & sr_bits::UIF) != 0
        }
    }

    /// Clear update interrupt flag
    //! Reference: RM0456 Section 54.4.5: TIMx_SR
    pub fn clear_update_interrupt(&self) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let mut val = core::ptr::read_volatile(sr);
            val &= !sr_bits::UIF;
            core::ptr::write_volatile(sr, val);
        }
    }

    /// Enable update interrupt
    //! Reference: RM0456 Section 54.4.4: TIMx_DIER
    pub fn enable_interrupt(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = core::ptr::read_volatile(dier);
            val |= dier_bits::UIE;
            core::ptr::write_volatile(dier, val);
        }
    }

    /// Disable update interrupt
    pub fn disable_interrupt(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = core::ptr::read_volatile(dier);
            val &= !dier_bits::UIE;
            core::ptr::write_volatile(dier, val);
        }
    }

    /// Delay using timer (blocking)
    pub fn delay_ms(&self, ms: u32, timer_freq_hz: u32) {
        let ticks = (timer_freq_hz / 1000) * ms;
        let config = Config {
            prescaler: 0,
            auto_reload: ticks,
            one_pulse: true,
            ..Default::default()
        };
        self.init(&config);
        self.start();
        
        // Wait for timer to complete
        while !self.is_update_interrupt() {}
        self.clear_update_interrupt();
    }

    /// Set compare value for channel 1
    //! Reference: RM0456 Section 54.4.14: TIMx_CCR1
    pub fn set_compare1(&self, value: u32) {
        unsafe {
            let ccr1 = (self.base + reg::CCR1) as *mut u32;
            core::ptr::write_volatile(ccr1, value);
        }
    }

    /// Set compare value for channel 2
    //! Reference: RM0456 Section 54.4.15: TIMx_CCR2
    pub fn set_compare2(&self, value: u32) {
        unsafe {
            let ccr2 = (self.base + reg::CCR2) as *mut u32;
            core::ptr::write_volatile(ccr2, value);
        }
    }

    /// Set compare value for channel 3
    //! Reference: RM0456 Section 54.4.16: TIMx_CCR3
    pub fn set_compare3(&self, value: u32) {
        unsafe {
            let ccr3 = (self.base + reg::CCR3) as *mut u32;
            core::ptr::write_volatile(ccr3, value);
        }
    }

    /// Set compare value for channel 4
    //! Reference: RM0456 Section 54.4.17: TIMx_CCR4
    pub fn set_compare4(&self, value: u32) {
        unsafe {
            let ccr4 = (self.base + reg::CCR4) as *mut u32;
            core::ptr::write_volatile(ccr4, value);
        }
    }

    /// Get capture value for channel 1
    pub fn get_capture1(&self) -> u32 {
        unsafe {
            let ccr1 = (self.base + reg::CCR1) as *mut u32;
            core::ptr::read_volatile(ccr1)
        }
    }

    /// Get capture value for channel 2
    pub fn get_capture2(&self) -> u32 {
        unsafe {
            let ccr2 = (self.base + reg::CCR2) as *mut u32;
            core::ptr::read_volatile(ccr2)
        }
    }
}

/// Initialize TIM6 as a basic delay timer
//! Reference: RM0456 Chapter 57: Basic timers
pub fn init_tim6_delay(pclk_freq: u32) -> Timer {
    // Enable TIM6 clock
    // Reference: RM0456 Section 11.10.5: RCC_APB1ENR1
    crate::rcc::enable_apb1_clock(crate::rcc::apb1_1::TIM6);

    let timer = Timer::tim6();
    let config = Config {
        prescaler: (pclk_freq / 1000000 - 1) as u16, // 1 MHz timer clock
        auto_reload: 0xFFFF,
        one_pulse: false,
        ..Default::default()
    };
    timer.init(&config);
    timer
}

/// Simple blocking delay in microseconds
pub fn delay_us(us: u32) {
    // Simple software delay
    for _ in 0..(us * 16) {
        unsafe { core::arch::asm!("nop"); };
    }
}

/// Simple blocking delay in milliseconds
pub fn delay_ms(ms: u32) {
    delay_us(ms * 1000);
}

/// Delay timer instance
static mut DELAY_TIMER: Option<Timer> = None;

/// Initialize delay timer with TIM6
pub fn init_delay_timer(pclk_freq: u32) {
    let timer = init_tim6_delay(pclk_freq);
    unsafe {
        DELAY_TIMER = Some(timer);
    }
}

/// Delay in milliseconds using TIM6
pub fn delay_ms_tim6(ms: u32) {
    unsafe {
        if let Some(timer) = DELAY_TIMER {
            timer.delay_ms(ms, 1000000); // Assuming 1 MHz timer clock
        }
    }
}

/// Configure timer for PWM output
//!
/// # Arguments
/// * `timer` - Timer instance
/// * `channel` - Capture/compare channel (1-4)
/// * `freq_hz` - PWM frequency in Hz
/// * `duty_cycle` - Duty cycle (0-100)
/// * `pclk_freq` - APB clock frequency in Hz
pub fn configure_pwm(
    timer: &Timer,
    channel: u8,
    freq_hz: u32,
    duty_cycle: u8,
    pclk_freq: u32
) {
    if channel < 1 || channel > 4 {
        return;
    }

    let prescaler = (pclk_freq / (freq_hz * 1000)) as u16;
    let auto_reload = 999;
    let compare_value = (auto_reload * duty_cycle as u32) / 100;

    let config = Config {
        prescaler,
        auto_reload,
        one_pulse: false,
        ..Default::default()
    };

    timer.init(&config);

    // Configure capture/compare mode
    unsafe {
        let ccmr = match channel {
            1 => (timer.base + reg::CCMR1) as *mut u32,
            2 => (timer.base + reg::CCMR1) as *mut u32,
            3 => (timer.base + reg::CCMR2) as *mut u32,
            4 => (timer.base + reg::CCMR2) as *mut u32,
            _ => return,
        };

        let shift = if channel % 2 == 1 {
            0
        } else {
            8
        };

        // Configure PWM mode 1
        let mut val = core::ptr::read_volatile(ccmr);
        val &= !(0b1111 << shift);
        val |= 0b110 << shift; // PWM mode 1
        val |= 1 << (shift + 7); // Preload enable
        core::ptr::write_volatile(ccmr, val);

        // Enable output
        let ccer = (timer.base + reg::CCER) as *mut u32;
        let mut val = core::ptr::read_volatile(ccer);
        val |= 1 << ((channel - 1) * 4); // CCxE
        core::ptr::write_volatile(ccer, val);

        // Set compare value
        match channel {
            1 => timer.set_compare1(compare_value),
            2 => timer.set_compare2(compare_value),
            3 => timer.set_compare3(compare_value),
            4 => timer.set_compare4(compare_value),
            _ => {}
        }
    }
}

/// Start PWM output
pub fn start_pwm(timer: &Timer) {
    timer.start();
}

/// Stop PWM output
pub fn stop_pwm(timer: &Timer) {
    timer.stop();
}
