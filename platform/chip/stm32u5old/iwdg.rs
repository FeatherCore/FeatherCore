//! IWDG - Independent Watchdog
//! 独立看门狗
//!
//! # Overview / 概述
//! STM32U5 Independent Watchdog (IWDG) provides system protection by generating
//! a reset when the counter reaches zero, even in Stop and Standby modes.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 61: Independent watchdog (IWDG)
//! 
//! ## Counter / 计数器
//! - Independent 12-bit downcounter
//! - Independent 32 kHz Low-Speed Internal oscillator (LSI)
//! 
//! ## Operation / 工作方式
//! - Reset generated when counter decrements from 0xFFF to 0x000
//! - Continues operating in Stop and Standby modes
//! - Refresh operation reloads the counter
//! 
//! ## Advanced Features / 高级特性
//! - Independent from system clock
//! - Non-maskable
//! - Programmable timeout period
//! 
//! # Reference / 参考
//! - RM0456 Chapter 61: Independent watchdog (IWDG)
//! - RM0456 Section 61.1: IWDG introduction
//! - RM0456 Section 61.2: IWDG main features
//! - RM0456 Section 61.3: IWDG functional description
//! - RM0456 Section 61.4: IWDG registers

/// IWDG base address / IWDG 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const IWDG_BASE: usize = 0x4000_3000;

/// IWDG register offsets
//! Reference: RM0456 Section 61.4: IWDG register map
pub mod reg {
    /// IWDG key register
    //! Reference: RM0456 Section 61.4.1: IWDG key register (IWDG_KR)
    pub const KR: usize = 0x00;
    /// IWDG prescaler register
    //! Reference: RM0456 Section 61.4.2: IWDG prescaler register (IWDG_PR)
    pub const PR: usize = 0x04;
    /// IWDG reload register
    //! Reference: RM0456 Section 61.4.3: IWDG reload register (IWDG_RLR)
    pub const RLR: usize = 0x08;
    /// IWDG status register
    //! Reference: RM0456 Section 61.4.4: IWDG status register (IWDG_SR)
    pub const SR: usize = 0x0C;
    /// IWDG window register
    //! Reference: RM0456 Section 61.4.5: IWDG window register (IWDG_WINR)
    pub const WINR: usize = 0x10;
    /// IWDG early wakeup interrupt register
    //! Reference: RM0456 Section 61.4.6: IWDG early wakeup interrupt register (IWDG_EWCR)
    pub const EWCR: usize = 0x14;
}

/// IWDG keys
pub const KEY_RELOAD: u16 = 0xAAAA;
pub const KEY_ENABLE: u16 = 0xCCCC;
pub const KEY_WRITE_ACCESS: u16 = 0x5555;

/// IWDG prescaler
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prescaler {
    /// Divide by 4
    Div4 = 0,
    /// Divide by 8
    Div8 = 1,
    /// Divide by 16
    Div16 = 2,
    /// Divide by 32
    Div32 = 3,
    /// Divide by 64
    Div64 = 4,
    /// Divide by 128
    Div128 = 5,
    /// Divide by 256
    Div256 = 6,
}

/// IWDG instance
pub struct Iwdg;

impl Iwdg {
    /// Create IWDG instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable write access to prescaler and reload registers
    fn enable_write_access(&self) {
        unsafe {
            let kr = (IWDG_BASE + reg::KR) as *mut u16;
            core::ptr::write_volatile(kr, KEY_WRITE_ACCESS);
        }
    }

    /// Reload the watchdog counter
    pub fn reload(&self) {
        unsafe {
            let kr = (IWDG_BASE + reg::KR) as *mut u16;
            core::ptr::write_volatile(kr, KEY_RELOAD);
        }
    }

    /// Initialize IWDG with timeout
    ///
    /// # Arguments
    /// * `prescaler` - Clock prescaler
    /// * `reload` - Reload value (0-4095)
    ///
    /// Timeout = (Reload + 1) * Prescaler / LSI_FREQ
    /// LSI_FREQ = 32 kHz
    pub fn init(&self, prescaler: Prescaler, reload: u16) {
        // Enable write access
        self.enable_write_access();

        unsafe {
            // Set prescaler
            let pr = (IWDG_BASE + reg::PR) as *mut u16;
            core::ptr::write_volatile(pr, prescaler as u16);

            // Set reload value
            let rlr = (IWDG_BASE + reg::RLR) as *mut u16;
            core::ptr::write_volatile(rlr, reload);
        }

        // Wait for registers to be updated
        while self.is_busy() {}

        // Reload counter to start watchdog
        self.reload();

        // Start watchdog (if not already started)
        unsafe {
            let kr = (IWDG_BASE + reg::KR) as *mut u16;
            core::ptr::write_volatile(kr, KEY_ENABLE);
        }
    }

    /// Check if IWDG is busy (registers being updated)
    pub fn is_busy(&self) -> bool {
        unsafe {
            let sr = (IWDG_BASE + reg::SR) as *mut u16;
            let val = core::ptr::read_volatile(sr);
            (val & 0b111) != 0
        }
    }

    /// Set window value (window watchdog mode)
    pub fn set_window(&self, window: u16) {
        self.enable_write_access();

        unsafe {
            let winr = (IWDG_BASE + reg::WINR) as *mut u16;
            core::ptr::write_volatile(winr, window);
        }
    }

    /// Calculate prescaler and reload for desired timeout
    ///
    /// # Arguments
    /// * `timeout_ms` - Desired timeout in milliseconds
    ///
    /// # Returns
    /// (Prescaler, Reload) tuple
    pub fn calculate_timeout(timeout_ms: u32) -> (Prescaler, u16) {
        // LSI frequency = 32 kHz
        let lsi_freq = 32000u32;

        // Try different prescalers
        for (presc_val, presc) in [
            (4, Prescaler::Div4),
            (8, Prescaler::Div8),
            (16, Prescaler::Div16),
            (32, Prescaler::Div32),
            (64, Prescaler::Div64),
            (128, Prescaler::Div128),
            (256, Prescaler::Div256),
        ] {
            let reload = (timeout_ms * lsi_freq / presc_val / 1000) as u16;
            if reload <= 4095 {
                return (presc, reload);
            }
        }

        // Maximum timeout
        (Prescaler::Div256, 4095)
    }
}

/// Initialize IWDG with timeout in milliseconds
pub fn init_iwdg_timeout(timeout_ms: u32) {
    let (prescaler, reload) = Iwdg::calculate_timeout(timeout_ms);
    let iwdg = Iwdg::new();
    iwdg.init(prescaler, reload);
}

/// Feed the watchdog (must be called periodically)
pub fn feed() {
    let iwdg = Iwdg::new();
    iwdg.reload();
}

/// Initialize IWDG with 1 second timeout (default)
pub fn init_iwdg_default() {
    init_iwdg_timeout(1000);
}
