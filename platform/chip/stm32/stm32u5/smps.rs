//! SMPS - Switched-Mode Power Supply Controller
//! 开关电源控制器
//!
//! STM32U5 SMPS 特性:
//! - 内部 SMPS 降压转换器
//! - 可编程输出电压 (1.0V - 1.8V)
//! - 支持 bypass 模式
//! - 低功耗模式支持

/// SMPS base address
pub const SMPS_BASE: usize = 0x4201_E000;

/// SMPS register offsets
pub mod reg {
    /// SMPS control register
    pub const CR: usize = 0x00;
    /// SMPS configuration register
    pub const CFGR: usize = 0x04;
    /// SMPS status register
    pub const SR: usize = 0x08;
    /// SMPS interrupt enable register
    pub const IER: usize = 0x0C;
    /// SMPS interrupt status register
    pub const ISR: usize = 0x10;
    /// SMPS power mode register
    pub const PMR: usize = 0x14;
}

/// SMPS output voltage (in mV)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputVoltage {
    /// 1.0V
    V1000 = 0x00,
    /// 1.1V
    V1100 = 0x01,
    /// 1.2V
    V1200 = 0x02,
    /// 1.3V
    V1300 = 0x03,
    /// 1.35V
    V1350 = 0x04,
    /// 1.4V
    V1400 = 0x05,
    /// 1.5V
    V1500 = 06,
    /// 1.6V
    V1600 = 0x07,
    /// 1.7V
    V1700 = 0x08,
    /// 1.8V
    V1800 = 0x09,
}

/// SMPS switching frequency
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchingFrequency {
    /// 1.5 MHz
    Freq1_5MHz = 0b00,
    /// 2.2 MHz
    Freq2_2MHz = 0b01,
    /// 4 MHz
    Freq4MHz = 0b10,
    /// 8 MHz
    Freq8MHz = 0b11,
}

/// SMPS mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SmpsMode {
    /// Disabled
    Disabled = 0b00,
    /// Bypass mode (LDO)
    Bypass = 0b01,
    /// Low Power mode
    LowPower = 0b10,
    /// Normal mode (SMPS)
    Normal = 0b11,
}

/// SMPS instance
pub struct Smps;

impl Smps {
    /// Create SMPS instance
    pub const fn new() -> Self {
        Self
    }

    /// Initialize SMPS
    pub fn init(&self) {
        unsafe {
            let cr = (SMPS_BASE + reg::CR) as *mut u32;
            core::ptr::write_volatile(cr, 0);
        }
    }

    /// Configure SMPS output voltage
    pub fn set_voltage(&self, voltage: OutputVoltage) {
        unsafe {
            let cfgr = (SMPS_BASE + reg::CFGR) as *mut u32;
            let mut val = core::ptr::read_volatile(cfgr);
            val &= !(0x0F << 16); // Clear VOUT bits
            val |= (voltage as u32) << 16;
            core::ptr::write_volatile(cfgr, val);
        }
    }

    /// Configure switching frequency
    pub fn set_frequency(&self, freq: SwitchingFrequency) {
        unsafe {
            let cfgr = (SMPS_BASE + reg::CFGR) as *mut u32;
            let mut val = core::ptr::read_volatile(cfgr);
            val &= !(0b11 << 8); // Clear FREQ bits
            val |= (freq as u32) << 8;
            core::ptr::write_volatile(cfgr, val);
        }
    }

    /// Set SMPS mode
    pub fn set_mode(&self, mode: SmpsMode) {
        unsafe {
            let cr = (SMPS_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(0b11 << 8); // Clear MODE bits
            val |= (mode as u32) << 8;
            core::ptr::write_volatile(cr, val);

            // Enable SMPS if not disabled
            if mode != SmpsMode::Disabled {
                val |= 1 << 0; // EN
            } else {
                val &= !(1 << 0);
            }
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable SMPS
    pub fn enable(&self) {
        unsafe {
            let cr = (SMPS_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable SMPS
    pub fn disable(&self) {
        unsafe {
            let cr = (SMPS_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 0);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Get status
    pub fn is_ready(&self) -> bool {
        unsafe {
            let sr = (SMPS_BASE + reg::SR) as *mut u32;
            (core::ptr::read_volatile(sr) & (1 << 8)) != 0 // PWR_RDY
        }
    }

    /// Enable interrupts
    pub fn enable_interrupt(&self, interrupt: u32) {
        unsafe {
            let ier = (SMPS_BASE + reg::IER) as *mut u32;
            let val = core::ptr::read_volatile(ier);
            core::ptr::write_volatile(ier, val | interrupt);
        }
    }

    /// Get interrupt status
    pub fn get_isr(&self) -> u32 {
        unsafe {
            let isr = (SMPS_BASE + reg::ISR) as *mut u32;
            core::ptr::read_volatile(isr)
        }
    }
}

/// Initialize SMPS with default configuration
pub fn init_smps_default() {
    let smps = Smps::new();
    smps.init();
    smps.set_voltage(OutputVoltage::V1300);
    smps.set_frequency(SwitchingFrequency::Freq2_2MHz);
    smps.set_mode(SmpsMode::Normal);
}
