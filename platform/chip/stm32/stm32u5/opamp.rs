//! OPAMP - Operational Amplifier
//! 运算放大器
//!
//! STM32U5 OPAMP 特性：
//! - 最多 3 个独立运算放大器
//! - 支持多种工作模式
//! - 支持内部跟随器、PGA、外部增益
//! - 支持校准功能

/// OPAMP1 base address
pub const OPAMP1_BASE: usize = 0x4000_9030;
/// OPAMP2 base address
pub const OPAMP2_BASE: usize = 0x4000_9034;
/// OPAMP3 base address
pub const OPAMP3_BASE: usize = 0x4000_9038;

/// OPAMP register offsets
pub mod reg {
    pub const CSR: usize = 0x00;
    pub const OTR: usize = 0x04;
    pub const LPOTR: usize = 0x08;
}

/// OPAMP mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Functional mode
    Functional = 0b00,
    /// Calibration mode
    Calibration = 0b01,
    /// Test mode
    Test = 0b10,
}

/// OPAMP functional mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FunctionalMode {
    /// Standalone mode
    Standalone = 0b000,
    /// Follower mode
    Follower = 0b010,
    /// PGA mode
    Pga = 0b100,
}

/// OPAMP PGA gain
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PgaGain {
    Gain2 = 0b00,
    Gain4 = 0b01,
    Gain8 = 0b10,
    Gain16 = 0b11,
}

/// OPAMP instance
pub struct Opamp {
    base: usize,
}

impl Opamp {
    pub const fn opamp1() -> Self {
        Self { base: OPAMP1_BASE }
    }

    pub const fn opamp2() -> Self {
        Self { base: OPAMP2_BASE }
    }

    pub const fn opamp3() -> Self {
        Self { base: OPAMP3_BASE }
    }

    /// Initialize OPAMP in follower mode
    pub fn init_follower(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = 0;
            val |= (FunctionalMode::Follower as u32) << 1;
            val |= 1 << 0; // OPAMPEN
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Initialize OPAMP in PGA mode
    pub fn init_pga(&self, gain: PgaGain) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = 0;
            val |= (FunctionalMode::Pga as u32) << 1;
            val |= (gain as u32) << 4;
            val |= 1 << 0; // OPAMPEN
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Enable OPAMP
    pub fn enable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val |= 1 << 0;
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Disable OPAMP
    pub fn disable(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val &= !(1 << 0);
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Calibrate OPAMP
    pub fn calibrate(&self) {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            
            // Enter calibration mode
            let mut val = core::ptr::read_volatile(csr);
            val |= (Mode::Calibration as u32) << 30;
            core::ptr::write_volatile(csr, val);

            // Enable OPAMP for calibration
            let mut val = core::ptr::read_volatile(csr);
            val |= 1 << 0;
            core::ptr::write_volatile(csr, val);

            // Wait for calibration complete
            while (core::ptr::read_volatile(csr) & (1 << 14)) == 0 {}

            // Store calibration data
            let otr = (self.base + reg::OTR) as *mut u32;
            let trim_value = (core::ptr::read_volatile(csr) >> 24) & 0x1F;
            core::ptr::write_volatile(otr, trim_value);

            // Exit calibration mode
            let mut val = core::ptr::read_volatile(csr);
            val &= !(0b11 << 30);
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Check if OPAMP is ready
    pub fn is_ready(&self) -> bool {
        unsafe {
            let csr = (self.base + reg::CSR) as *mut u32;
            let val = core::ptr::read_volatile(csr);
            (val & (1 << 8)) != 0
        }
    }
}

/// Initialize OPAMP1 as voltage follower
pub fn init_opamp1_follower() {
    // Enable OPAMP clock
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::OPAMP);
    
    let opamp = Opamp::opamp1();
    opamp.init_follower();
}

/// Initialize OPAMP2 as PGA with gain 4
pub fn init_opamp2_pga() {
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::OPAMP);
    
    let opamp = Opamp::opamp2();
    opamp.init_pga(PgaGain::Gain4);
}
