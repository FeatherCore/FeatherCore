//! TIMER - General Purpose Timers
//! 通用定时器
//!
//! STM32U5 支持多个定时器：
//! - 高级定时器：TIM1, TIM8
//! - 通用定时器：TIM2, TIM3, TIM4, TIM5, TIM15, TIM16, TIM17
//! - 基本定时器：TIM6, TIM7
//! - 低功耗定时器：LPTIM1, LPTIM2, LPTIM3, LPTIM4, LPTIM5, LPTIM6

/// TIM2 base address
pub const TIM2_BASE: usize = 0x4000_0000;
/// TIM3 base address
pub const TIM3_BASE: usize = 0x4000_0400;
/// TIM4 base address
pub const TIM4_BASE: usize = 0x4000_0800;
/// TIM5 base address
pub const TIM5_BASE: usize = 0x4000_0C00;
/// TIM6 base address
pub const TIM6_BASE: usize = 0x4000_1000;
/// TIM7 base address
pub const TIM7_BASE: usize = 0x4000_1400;
/// TIM1 base address
pub const TIM1_BASE: usize = 0x4001_2C00;
/// TIM8 base address
pub const TIM8_BASE: usize = 0x4001_3400;
/// TIM15 base address
pub const TIM15_BASE: usize = 0x4001_4000;
/// TIM16 base address
pub const TIM16_BASE: usize = 0x4001_4400;
/// TIM17 base address
pub const TIM17_BASE: usize = 0x4001_4800;

/// Timer register offsets
pub mod reg {
    /// Control register 1
    pub const CR1: usize = 0x00;
    /// Control register 2
    pub const CR2: usize = 0x04;
    /// Slave mode control register
    pub const SMCR: usize = 0x08;
    /// DMA/interrupt enable register
    pub const DIER: usize = 0x0C;
    /// Status register
    pub const SR: usize = 0x10;
    /// Event generation register
    pub const EGR: usize = 0x14;
    /// Capture/compare mode register 1
    pub const CCMR1: usize = 0x18;
    /// Capture/compare mode register 2
    pub const CCMR2: usize = 0x1C;
    /// Capture/compare enable register
    pub const CCER: usize = 0x20;
    /// Counter
    pub const CNT: usize = 0x24;
    /// Prescaler
    pub const PSC: usize = 0x28;
    /// Auto-reload register
    pub const ARR: usize = 0x2C;
    /// Repetition counter register
    pub const RCR: usize = 0x30;
    /// Capture/compare register 1
    pub const CCR1: usize = 0x34;
    /// Capture/compare register 2
    pub const CCR2: usize = 0x38;
    /// Capture/compare register 3
    pub const CCR3: usize = 0x3C;
    /// Capture/compare register 4
    pub const CCR4: usize = 0x40;
    /// Break and dead-time register
    pub const BDTR: usize = 0x44;
    /// DMA control register
    pub const DCR: usize = 0x48;
    /// DMA address for full transfer
    pub const DMAR: usize = 0x4C;
    /// Option register
    pub const OR: usize = 0x50;
    /// Capture/compare mode register 3
    pub const CCMR3: usize = 0x54;
    /// Capture/compare register 5
    pub const CCR5: usize = 0x58;
    /// Capture/compare register 6
    pub const CCR6: usize = 0x5C;
    /// Alternate function option register 1
    pub const AF1: usize = 0x60;
    /// Alternate function option register 2
    pub const AF2: usize = 0x64;
}

/// Timer configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub prescaler: u16,
    pub auto_reload: u32,
    pub one_pulse: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prescaler: 0,
            auto_reload: 0xFFFF,
            one_pulse: false,
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

    /// Create TIM6 instance (basic timer)
    pub const fn tim6() -> Self {
        Self { base: TIM6_BASE }
    }

    /// Create TIM7 instance (basic timer)
    pub const fn tim7() -> Self {
        Self { base: TIM7_BASE }
    }

    /// Initialize timer
    pub fn init(&self, config: &Config) {
        unsafe {
            // Disable timer before configuration
            let cr1 = (self.base + reg::CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 0);

            // Set prescaler
            let psc = (self.base + reg::PSC) as *mut u32;
            core::ptr::write_volatile(psc, config.prescaler as u32);

            // Set auto-reload
            let arr = (self.base + reg::ARR) as *mut u32;
            core::ptr::write_volatile(arr, config.auto_reload);

            // Generate update event to reload registers
            let egr = (self.base + reg::EGR) as *mut u32;
            core::ptr::write_volatile(egr, 1 << 0); // UG

            // Configure CR1
            let mut cr1_val = 0;
            if config.one_pulse {
                cr1_val |= 1 << 3; // OPM
            }
            core::ptr::write_volatile(cr1, cr1_val);
        }
    }

    /// Start timer
    pub fn start(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= 1 << 0; // CEN
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Stop timer
    pub fn stop(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val &= !(1 << 0); // CEN
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
    pub fn get_count(&self) -> u32 {
        unsafe {
            let cnt = (self.base + reg::CNT) as *mut u32;
            core::ptr::read_volatile(cnt)
        }
    }

    /// Check if update interrupt flag is set
    pub fn is_update_interrupt(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 0)) != 0 // UIF
        }
    }

    /// Clear update interrupt flag
    pub fn clear_update_interrupt(&self) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let mut val = core::ptr::read_volatile(sr);
            val &= !(1 << 0); // Clear UIF
            core::ptr::write_volatile(sr, val);
        }
    }

    /// Enable update interrupt
    pub fn enable_interrupt(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = core::ptr::read_volatile(dier);
            val |= 1 << 0; // UIE
            core::ptr::write_volatile(dier, val);
        }
    }

    /// Disable update interrupt
    pub fn disable_interrupt(&self) {
        unsafe {
            let dier = (self.base + reg::DIER) as *mut u32;
            let mut val = core::ptr::read_volatile(dier);
            val &= !(1 << 0); // UIE
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
        };
        self.init(&config);
        self.start();
        
        // Wait for timer to complete
        while !self.is_update_interrupt() {}
        self.clear_update_interrupt();
    }
}

/// Initialize TIM6 as a basic delay timer
pub fn init_tim6_delay(pclk_freq: u32) -> Timer {
    // Enable TIM6 clock
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::TIM6);

    let timer = Timer::tim6();
    let config = Config {
        prescaler: (pclk_freq / 1000000 - 1) as u16, // 1 MHz timer clock
        auto_reload: 0xFFFF,
        one_pulse: false,
    };
    timer.init(&config);
    timer
}

/// Simple blocking delay in microseconds
pub fn delay_us(us: u32) {
    // Simple software delay
    for _ in 0..(us * 16) {
        unsafe { core::arch::asm!("nop") };
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
