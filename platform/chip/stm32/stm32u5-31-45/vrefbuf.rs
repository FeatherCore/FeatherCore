//! VREFBUF - Voltage Reference Buffer
//! 电压参考缓冲器
//!
//! STM32U5 VREFBUF 特性：
//! - 提供稳定的电压参考
//! - 支持 1.5V, 1.8V, 2.048V, 2.5V 输出
//! - 支持外部电容
//! - 可作为 ADC/DAC 参考电压

/// VREFBUF base address
pub const VREFBUF_BASE: usize = 0x4000_7030;

/// VREFBUF register offsets
pub mod reg {
    pub const CSR: usize = 0x00;
    pub const CCR: usize = 0x04;
}

/// VREFBUF voltage scale
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoltageScale {
    /// 1.5V
    V1_5 = 0b000,
    /// 1.8V
    V1_8 = 0b001,
    /// 2.048V
    V2_048 = 0b010,
    /// 2.5V
    V2_5 = 0b011,
}

impl VoltageScale {
    /// Get voltage in millivolts
    pub fn to_mv(&self) -> u32 {
        match self {
            VoltageScale::V1_5 => 1500,
            VoltageScale::V1_8 => 1800,
            VoltageScale::V2_048 => 2048,
            VoltageScale::V2_5 => 2500,
        }
    }
}

/// VREFBUF instance
pub struct VrefBuf;

impl VrefBuf {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize VREFBUF
    pub fn init(&self, scale: VoltageScale) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            
            // Configure voltage scale
            let mut val = core::ptr::read_volatile(csr);
            val &= !(0b111 << 0);
            val |= (scale as u32) << 0;
            core::ptr::write_volatile(csr, val);

            // Enable VREFBUF
            let mut val = core::ptr::read_volatile(csr);
            val |= 1 << 0;
            core::ptr::write_volatile(csr, val);

            // Wait for VRR (voltage ready)
            while (core::ptr::read_volatile(csr) & (1 << 3)) == 0 {}
        }
    }

    /// Enable high impedance mode
    pub fn enable_high_z(&self) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val |= 1 << 1;
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Disable high impedance mode
    pub fn disable_high_z(&self) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val &= !(1 << 1);
            core::ptr::write_volatile(csr, val);
        }
    }

    /// Check if voltage is ready
    pub fn is_ready(&self) -> bool {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let val = core::ptr::read_volatile(csr);
            (val & (1 << 3)) != 0
        }
    }

    /// Disable VREFBUF
    pub fn disable(&self) {
        unsafe {
            let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
            let mut val = core::ptr::read_volatile(csr);
            val &= !(1 << 0);
            core::ptr::write_volatile(csr, val);
        }
    }
}

/// Initialize VREFBUF with 2.5V output
pub fn init_vrefbuf_2v5() {
    let vrefbuf = VrefBuf::new();
    vrefbuf.init(VoltageScale::V2_5);
}

/// Initialize VREFBUF with 1.8V output
pub fn init_vrefbuf_1v8() {
    let vrefbuf = VrefBuf::new();
    vrefbuf.init(VoltageScale::V1_8);
}

/// Get current VREFBUF voltage in millivolts
pub fn get_vref_mv() -> u32 {
    unsafe {
        let csr = (VREFBUF_BASE + reg::CSR) as *mut u32;
        let val = core::ptr::read_volatile(csr);
        let scale = val & 0b111;
        match scale {
            0b000 => 1500,
            0b001 => 1800,
            0b010 => 2048,
            0b011 => 2500,
            _ => 2500,
        }
    }
}
