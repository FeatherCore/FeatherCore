//! CRC - Cyclic Redundancy Check
//! 循环冗余校验计算单元
//!
//! # Overview / 概述
//! STM32U5 Cyclic Redundancy Check (CRC) provides hardware acceleration for
//! CRC calculations with support for multiple CRC standards.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 24: Cyclic redundancy check calculation unit (CRC)
//! 
//! ## Supported CRC Standards / 支持的 CRC 标准
//! - CRC-8
//! - CRC-16
//! - CRC-32
//! 
//! ## Programmable Features / 可编程特性
//! - Programmable polynomial
//! - Programmable initial value
//! - Input data reverse
//! - Output data reverse
//! 
//! ## Data Width / 数据宽度
//! - Byte operation (8-bit)
//! - Half-word operation (16-bit)
//! - Word operation (32-bit)
//! 
//! # Reference / 参考
//! - RM0456 Chapter 24: Cyclic redundancy check calculation unit (CRC)
//! - RM0456 Section 24.1: CRC introduction
//! - RM0456 Section 24.2: CRC main features
//! - RM0456 Section 24.3: CRC functional description
//! - RM0456 Section 24.4: CRC registers

/// CRC base address / CRC 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const CRC_BASE: usize = 0x4002_3000;

/// CRC register offsets / CRC 寄存器偏移
//! Reference: RM0456 Section 24.4: CRC register map
pub mod reg {
    /// CRC data register / CRC 数据寄存器
    //! Reference: RM0456 Section 24.4.1: CRC data register (CRC_DR)
    pub const DR: usize = 0x00;
    /// CRC independent data register / CRC 独立数据寄存器
    //! Reference: RM0456 Section 24.4.2: CRC independent data register (CRC_IDR)
    pub const IDR: usize = 0x04;
    /// CRC control register / CRC 控制寄存器
    //! Reference: RM0456 Section 24.4.3: CRC control register (CRC_CR)
    pub const CR: usize = 0x08;
    /// CRC initial value register
    //! Reference: RM0456 Section 24.4.4: CRC initial value register (CRC_INIT)
    pub const INIT: usize = 0x10;
    /// CRC polynomial register
    //! Reference: RM0456 Section 24.4.5: CRC polynomial register (CRC_POL)
    pub const POL: usize = 0x14;
}

/// CRC polynomial size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PolynomialSize {
    /// 32-bit polynomial
    Bits32 = 0b00,
    /// 16-bit polynomial
    Bits16 = 0b01,
    /// 8-bit polynomial
    Bits8 = 0b10,
    /// 7-bit polynomial
    Bits7 = 0b11,
}

/// CRC input data format
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputFormat {
    /// 32-bit words
    Words = 0b00,
    /// 16-bit half-words
    HalfWords = 0b01,
    /// 8-bit bytes
    Bytes = 0b10,
}

/// CRC configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Polynomial size
    pub poly_size: PolynomialSize,
    /// Polynomial value
    pub polynomial: u32,
    /// Initial value
    pub initial_value: u32,
    /// Input data reversal
    pub input_reversal: bool,
    /// Output data reversal
    pub output_reversal: bool,
    /// Input data format
    pub input_format: InputFormat,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poly_size: PolynomialSize::Bits32,
            polynomial: 0x04C11DB7, // CRC-32 polynomial
            initial_value: 0xFFFFFFFF,
            input_reversal: true,
            output_reversal: true,
            input_format: InputFormat::Bytes,
        }
    }
}

/// CRC instance
pub struct Crc;

impl Crc {
    /// Create CRC instance
    pub const fn new() -> Self {
        Self
    }

    /// Initialize CRC unit
    pub fn init(&self, config: &Config) {
        // Enable CRC clock
        crate::rcc::enable_ahb1_clock(crate::rcc::ahb1::CRC);

        unsafe {
            // Reset CRC unit
            let cr = (CRC_BASE + reg::CR) as *mut u32;
            core::ptr::write_volatile(cr, 1 << 0); // RESET

            // Set polynomial
            let pol = (CRC_BASE + reg::POL) as *mut u32;
            core::ptr::write_volatile(pol, config.polynomial);

            // Set initial value
            let init = (CRC_BASE + reg::INIT) as *mut u32;
            core::ptr::write_volatile(init, config.initial_value);

            // Configure control register
            let mut cr_val = 0;
            cr_val |= (config.poly_size as u32) << 3;
            cr_val |= (config.input_format as u32) << 5;
            if config.input_reversal {
                cr_val |= 1 << 5;
            }
            if config.output_reversal {
                cr_val |= 1 << 7;
            }
            core::ptr::write_volatile(cr, cr_val);
        }
    }

    /// Reset CRC calculation
    pub fn reset(&self) {
        unsafe {
            let cr = (CRC_BASE + reg::CR) as *mut u32;
            core::ptr::write_volatile(cr, 1 << 0); // RESET
        }
    }

    /// Write 32-bit data
    pub fn write_u32(&self, data: u32) {
        unsafe {
            let dr = (CRC_BASE + reg::DR) as *mut u32;
            core::ptr::write_volatile(dr, data);
        }
    }

    /// Write 16-bit data
    pub fn write_u16(&self, data: u16) {
        unsafe {
            let dr = (CRC_BASE + reg::DR) as *mut u16;
            core::ptr::write_volatile(dr, data);
        }
    }

    /// Write 8-bit data
    pub fn write_u8(&self, data: u8) {
        unsafe {
            let dr = (CRC_BASE + reg::DR) as *mut u8;
            core::ptr::write_volatile(dr, data);
        }
    }

    /// Read CRC result
    pub fn read(&self) -> u32 {
        unsafe {
            let dr = (CRC_BASE + reg::DR) as *mut u32;
            core::ptr::read_volatile(dr)
        }
    }

    /// Calculate CRC-32 of a byte slice
    pub fn calculate_crc32(&self, data: &[u8]) -> u32 {
        let config = Config {
            poly_size: PolynomialSize::Bits32,
            polynomial: 0x04C11DB7,
            initial_value: 0xFFFFFFFF,
            input_reversal: true,
            output_reversal: true,
            input_format: InputFormat::Bytes,
        };

        self.init(&config);

        for byte in data {
            self.write_u8(*byte);
        }

        self.read() ^ 0xFFFFFFFF
    }

    /// Calculate CRC-16 of a byte slice
    pub fn calculate_crc16(&self, data: &[u8]) -> u16 {
        let config = Config {
            poly_size: PolynomialSize::Bits16,
            polynomial: 0x8005,
            initial_value: 0xFFFF,
            input_reversal: true,
            output_reversal: true,
            input_format: InputFormat::Bytes,
        };

        self.init(&config);

        for byte in data {
            self.write_u8(*byte);
        }

        (self.read() & 0xFFFF) as u16
    }

    /// Calculate CRC-8 of a byte slice
    pub fn calculate_crc8(&self, data: &[u8]) -> u8 {
        let config = Config {
            poly_size: PolynomialSize::Bits8,
            polynomial: 0x07,
            initial_value: 0x00,
            input_reversal: false,
            output_reversal: false,
            input_format: InputFormat::Bytes,
        };

        self.init(&config);

        for byte in data {
            self.write_u8(*byte);
        }

        (self.read() & 0xFF) as u8
    }
}

/// Calculate CRC-32 of data
pub fn crc32(data: &[u8]) -> u32 {
    let crc = Crc::new();
    crc.calculate_crc32(data)
}

/// Calculate CRC-16 of data
pub fn crc16(data: &[u8]) -> u16 {
    let crc = Crc::new();
    crc.calculate_crc16(data)
}

/// Calculate CRC-8 of data
pub fn crc8(data: &[u8]) -> u8 {
    let crc = Crc::new();
    crc.calculate_crc8(data)
}
