//! PWR - Power Control
//! 电源控制
//!
//! STM32U5 电源控制模块支持多种低功耗模式：
//! - Sleep 模式
//! - Stop 模式 (0-3)
//! - Standby 模式
//! - Shutdown 模式

/// PWR 基地址
pub const PWR_BASE: usize = 0x4002_0000;

/// PWR 寄存器偏移
pub mod reg {
    /// Power control register 1
    pub const CR1: usize = 0x00;
    /// Power control register 2
    pub const CR2: usize = 0x04;
    /// Power control register 3
    pub const CR3: usize = 0x08;
    /// Power voltage detector register
    pub const PVDR: usize = 0x0C;
    /// Power control status register 1
    pub const SR1: usize = 0x10;
    /// Power control status register 2
    pub const SR2: usize = 0x14;
    /// Power status clear register
    pub const SCR: usize = 0x18;
    /// Power control register 4
    pub const CR4: usize = 0x1C;
    /// Power security configuration register
    pub const SECCFGR: usize = 0x20;
    /// Power privilege configuration register
    pub const PRIVCFGR: usize = 0x24;
    /// Power control status register 3
    pub const SR3: usize = 0x28;
}

/// Voltage scaling modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoltageScale {
    /// Range 0: up to 160 MHz
    Range0 = 0b00,
    /// Range 1: up to 110 MHz
    Range1 = 0b01,
    /// Range 2: up to 55 MHz
    Range2 = 0b10,
    /// Range 3: up to 25 MHz
    Range3 = 0b11,
}

/// Low power modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LowPowerMode {
    /// Sleep mode
    Sleep,
    /// Stop 0 mode
    Stop0,
    /// Stop 1 mode
    Stop1,
    /// Stop 2 mode
    Stop2,
    /// Stop 3 mode
    Stop3,
    /// Standby mode
    Standby,
    /// Shutdown mode
    Shutdown,
}

/// Initialize power controller
pub fn init() {
    unsafe {
        // Enable PWR clock
        crate::rcc::enable_apb1_clock(crate::rcc::apb1::PWR);

        // Configure voltage scaling to Range 0 (highest performance)
        set_voltage_scale(VoltageScale::Range0);

        // Wait for voltage scaling to be ready
        while !is_voltage_scaling_ready() {}
    }
}

/// Set voltage scaling
pub fn set_voltage_scale(scale: VoltageScale) {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !(0b11 << 9); // Clear VOS bits
        val |= (scale as u32) << 9;
        core::ptr::write_volatile(cr1, val);
    }
}

/// Get current voltage scaling
pub fn get_voltage_scale() -> VoltageScale {
    unsafe {
        let sr2 = (PWR_BASE + reg::SR2) as *mut u32;
        let val = core::ptr::read_volatile(sr2);
        match (val >> 14) & 0b11 {
            0b00 => VoltageScale::Range0,
            0b01 => VoltageScale::Range1,
            0b10 => VoltageScale::Range2,
            _ => VoltageScale::Range3,
        }
    }
}

/// Check if voltage scaling is ready
pub fn is_voltage_scaling_ready() -> bool {
    unsafe {
        let sr2 = (PWR_BASE + reg::SR2) as *mut u32;
        let val = core::ptr::read_volatile(sr2);
        (val & (1 << 13)) != 0 // VOSF flag
    }
}

/// Enable backup domain write access
pub fn enable_backup_access() {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val |= 1 << 8; // DBP bit
        core::ptr::write_volatile(cr1, val);
    }
}

/// Disable backup domain write access
pub fn disable_backup_access() {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !(1 << 8); // DBP bit
        core::ptr::write_volatile(cr1, val);
    }
}

/// Enter Sleep mode
/// 
/// CPU clock is stopped, all peripherals continue to operate
pub fn enter_sleep_mode() {
    unsafe {
        // Clear SLEEPDEEP bit
        let scr = 0xE000_ED10 as *mut u32;
        let mut val = core::ptr::read_volatile(scr);
        val &= !(1 << 2);
        core::ptr::write_volatile(scr, val);

        // Wait For Interrupt
        core::arch::asm!("wfi");
    }
}

/// Enter Stop mode
/// 
/// All clocks in the VCORE domain are stopped
pub fn enter_stop_mode(mode: LowPowerMode) {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);

        // Configure stop mode
        match mode {
            LowPowerMode::Stop0 => {
                val &= !(0b111 << 5); // LPMS = 000
            }
            LowPowerMode::Stop1 => {
                val &= !(0b111 << 5);
                val |= 0b001 << 5; // LPMS = 001
            }
            LowPowerMode::Stop2 => {
                val &= !(0b111 << 5);
                val |= 0b010 << 5; // LPMS = 010
            }
            LowPowerMode::Stop3 => {
                val &= !(0b111 << 5);
                val |= 0b011 << 5; // LPMS = 011
            }
            _ => {}
        }
        core::ptr::write_volatile(cr1, val);

        // Set SLEEPDEEP bit
        let scr = 0xE000_ED10 as *mut u32;
        let mut val = core::ptr::read_volatile(scr);
        val |= 1 << 2;
        core::ptr::write_volatile(scr, val);

        // Wait For Interrupt
        core::arch::asm!("wfi");
    }
}

/// Enter Standby mode
/// 
/// VCORE domain is powered off
pub fn enter_standby_mode() {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !(0b111 << 5);
        val |= 0b100 << 5; // LPMS = 100
        core::ptr::write_volatile(cr1, val);

        // Set SLEEPDEEP bit
        let scr = 0xE000_ED10 as *mut u32;
        let mut val = core::ptr::read_volatile(scr);
        val |= 1 << 2;
        core::ptr::write_volatile(scr, val);

        // Wait For Interrupt
        core::arch::asm!("wfi");
    }
}

/// Check if device was in Standby mode
pub fn was_in_standby() -> bool {
    unsafe {
        let sr1 = (PWR_BASE + reg::SR1) as *mut u32;
        let val = core::ptr::read_volatile(sr1);
        (val & (1 << 0)) != 0 // SBF flag
    }
}

/// Clear standby flag
pub fn clear_standby_flag() {
    unsafe {
        let scr = (PWR_BASE + reg::SCR) as *mut u32;
        core::ptr::write_volatile(scr, 1 << 0); // CSBF bit
    }
}

/// Configure wakeup pin
pub fn enable_wakeup_pin(pin: u8, rising: bool) {
    unsafe {
        // Enable wakeup pin in CR4
        let cr4 = (PWR_BASE + reg::CR4) as *mut u32;
        let mut val = core::ptr::read_volatile(cr4);
        val |= 1 << (pin - 1); // EWUPx bit
        core::ptr::write_volatile(cr4, val);

        // Configure edge detection in CR3
        let cr3 = (PWR_BASE + reg::CR3) as *mut u32;
        let mut val = core::ptr::read_volatile(cr3);
        if rising {
            val |= 1 << (pin + 1); // WPx rising
        } else {
            val &= !(1 << (pin + 1));
        }
        core::ptr::write_volatile(cr3, val);
    }
}
