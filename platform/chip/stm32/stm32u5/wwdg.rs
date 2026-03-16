//! WWDG - Window Watchdog
//! 窗口看门狗
//!
//! WWDG 特性：
//! - 7-bit 递减计数器
//! - 可编程的窗口值
//! - 必须在窗口内喂狗，否则产生复位
//! - 早期唤醒中断 (EWI)
//! - 由 APB 时钟驱动

/// WWDG base address
pub const WWDG_BASE: usize = 0x4000_2C00;

/// WWDG register offsets
pub mod reg {
    /// WWDG control register
    pub const CR: usize = 0x00;
    /// WWDG configuration register
    pub const CFR: usize = 0x04;
    /// WWDG status register
    pub const SR: usize = 0x08;
}

/// WWDG prescaler
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prescaler {
    /// Divide by 1
    Div1 = 0b00,
    /// Divide by 2
    Div2 = 0b01,
    /// Divide by 4
    Div4 = 0b10,
    /// Divide by 8
    Div8 = 0b11,
}

/// WWDG instance
pub struct Wwdg;

impl Wwdg {
    /// Create WWDG instance
    pub const fn new() -> Self {
        Self
    }

    /// Initialize WWDG
    ///
    /// # Arguments
    /// * `prescaler` - Clock prescaler
    /// * `window` - Window value (7-bit)
    /// * `counter` - Initial counter value (7-bit, must be > 0x3F)
    pub fn init(&self, prescaler: Prescaler, window: u8, counter: u8) {
        // Enable WWDG clock
        crate::rcc::enable_apb1_clock(crate::rcc::apb1::WWDG);

        unsafe {
            // Configure prescaler and window
            let cfr = (WWDG_BASE + reg::CFR) as *mut u32;
            let mut val = 0;
            val |= (prescaler as u32) << 7;
            val |= (window as u32) & 0x7F;
            core::ptr::write_volatile(cfr, val);

            // Set counter and enable watchdog
            let cr = (WWDG_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= (counter as u32) & 0x7F;
            val |= 1 << 7; // WDGA - Watchdog activation
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Reload the watchdog counter
    pub fn reload(&self, counter: u8) {
        unsafe {
            let cr = (WWDG_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(0x7F << 0); // Clear counter
            val |= (counter as u32) & 0x7F;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable early wakeup interrupt
    pub fn enable_ewi(&self) {
        unsafe {
            let cfr = (WWDG_BASE + reg::CFR) as *mut u32;
            let mut val = core::ptr::read_volatile(cfr);
            val |= 1 << 9; // EWI
            core::ptr::write_volatile(cfr, val);
        }
    }

    /// Check if early wakeup interrupt occurred
    pub fn is_ewi(&self) -> bool {
        unsafe {
            let sr = (WWDG_BASE + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & 1) != 0
        }
    }

    /// Clear early wakeup interrupt flag
    pub fn clear_ewi(&self) {
        unsafe {
            let sr = (WWDG_BASE + reg::SR) as *mut u32;
            core::ptr::write_volatile(sr, 0);
        }
    }

    /// Get current counter value
    pub fn get_counter(&self) -> u8 {
        unsafe {
            let cr = (WWDG_BASE + reg::CR) as *mut u32;
            let val = core::ptr::read_volatile(cr);
            (val & 0x7F) as u8
        }
    }
}

/// Initialize WWDG with default configuration
///
/// Default: 48 MHz PCLK1, prescaler /8, ~50ms timeout
pub fn init_wwdg_default() {
    let wwdg = Wwdg::new();
    wwdg.init(Prescaler::Div8, 0x60, 0x7F);
}
