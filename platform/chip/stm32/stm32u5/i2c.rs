//! I2C - Inter-Integrated Circuit
//! I2C 总线接口
//!
//! # Overview / 概述
//! STM32U5 Inter-Integrated Circuit (I2C) interfaces provide high-speed communication
//! with I2C-compatible devices, supporting multiple speed modes and advanced features.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 65: Inter-integrated circuit interface (I2C)
//! 
//! ## I2C Interfaces / I2C接口
//! - **I2C1, I2C2, I2C3, I2C4**
//! 
//! ## Transfer Speeds / 传输速率
//! - **Standard mode:** 100 kHz
//! - **Fast mode:** 400 kHz
//! - **Fast mode+:** 1 MHz
//! - **Ultra-fast mode:** 5 MHz
//! 
//! ## Advanced Features / 高级特性
//! - DMA support
//! - SMBus support
//! - PMBus support
//! - Clock stretching
//! - Multi-master/slave modes
//! - 7-bit and 10-bit addressing
//! - Automatic end mode
//! - PEC calculation
//! 
//! # Reference / 参考
//! - RM0456 Chapter 65: Inter-integrated circuit interface (I2C)
//! - RM0456 Section 65.1: I2C introduction
//! - RM0456 Section 65.2: I2C main features
//! - RM0456 Section 65.3: I2C functional description
//! - RM0456 Section 65.6: I2C registers

/// I2C1 base address / I2C1 基地址
//! Reference: RM0456 Chapter 2, Table 1: Memory map and register boundary addresses
pub const I2C1_BASE: usize = 0x4000_5400;
/// I2C2 base address / I2C2 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const I2C2_BASE: usize = 0x4000_5800;
/// I2C3 base address / I2C3 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const I2C3_BASE: usize = 0x4000_5C00;
/// I2C4 base address / I2C4 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const I2C4_BASE: usize = 0x4000_8400;

/// I2C register offsets
//! Reference: RM0456 Section 65.6: I2C register map
pub mod reg {
    /// Control register 1
    //! Reference: RM0456 Section 65.6.1: I2C control register 1 (I2C_CR1)
    pub const CR1: usize = 0x00;
    /// Control register 2
    //! Reference: RM0456 Section 65.6.2: I2C control register 2 (I2C_CR2)
    pub const CR2: usize = 0x04;
    /// Own address 1 register
    //! Reference: RM0456 Section 65.6.3: I2C own address 1 register (I2C_OAR1)
    pub const OAR1: usize = 0x08;
    /// Own address 2 register
    //! Reference: RM0456 Section 65.6.4: I2C own address 2 register (I2C_OAR2)
    pub const OAR2: usize = 0x0C;
    /// Timing register
    //! Reference: RM0456 Section 65.6.5: I2C timing register (I2C_TIMINGR)
    pub const TIMINGR: usize = 0x10;
    /// Timeout register
    //! Reference: RM0456 Section 65.6.6: I2C timeout register (I2C_TIMEOUTR)
    pub const TIMEOUTR: usize = 0x14;
    /// Interrupt and status register
    //! Reference: RM0456 Section 65.6.7: I2C interrupt and status register (I2C_ISR)
    pub const ISR: usize = 0x18;
    /// Interrupt clear register
    //! Reference: RM0456 Section 65.6.8: I2C interrupt clear register (I2C_ICR)
    pub const ICR: usize = 0x1C;
    /// PEC register
    //! Reference: RM0456 Section 65.6.9: I2C PEC register (I2C_PECR)
    pub const PECR: usize = 0x20;
    /// Receive data register
    //! Reference: RM0456 Section 65.6.10: I2C receive data register (I2C_RXDR)
    pub const RXDR: usize = 0x24;
    /// Transmit data register
    //! Reference: RM0456 Section 65.6.11: I2C transmit data register (I2C_TXDR)
    pub const TXDR: usize = 0x28;
    /// Autonomous mode control register
    //! Reference: RM0456 Section 65.6.12: I2C autonomous mode control register (I2C_AUTOCR)
    pub const AUTOCR: usize = 0x2C;
}

/// I2C speed modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2cSpeed {
    /// Standard mode (100 kHz)
    Standard = 100_000,
    /// Fast mode (400 kHz)
    Fast = 400_000,
    /// Fast mode plus (1 MHz)
    FastPlus = 1_000_000,
}

/// I2C configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub speed: I2cSpeed,
    pub own_address: u16,
    pub address_10bit: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed: I2cSpeed::Standard,
            own_address: 0,
            address_10bit: false,
        }
    }
}

/// I2C instance
pub struct I2c {
    base: usize,
}

impl I2c {
    /// Create I2C1 instance
    pub const fn i2c1() -> Self {
        Self { base: I2C1_BASE }
    }

    /// Create I2C2 instance
    pub const fn i2c2() -> Self {
        Self { base: I2C2_BASE }
    }

    /// Create I2C3 instance
    pub const fn i2c3() -> Self {
        Self { base: I2C3_BASE }
    }

    /// Initialize I2C
    pub fn init(&self, config: &Config, pclk_freq: u32) {
        unsafe {
            // Disable I2C before configuration
            let cr1 = (self.base + reg::CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 0);

            // Calculate timing values
            // For standard mode 100 kHz with PCLK = 160 MHz
            let timing = calculate_timing(pclk_freq, config.speed);

            let timingr = (self.base + reg::TIMINGR) as *mut u32;
            core::ptr::write_volatile(timingr, timing);

            // Configure own address
            if config.own_address != 0 {
                let oar1 = (self.base + reg::OAR1) as *mut u32;
                if config.address_10bit {
                    core::ptr::write_volatile(oar1, (1 << 15) | (1 << 10) | (config.own_address as u32));
                } else {
                    core::ptr::write_volatile(oar1, (1 << 15) | ((config.own_address as u32) << 1));
                }
            }

            // Enable I2C
            let cr1 = (self.base + reg::CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 1 << 0); // PE
        }
    }

    /// Generate START condition and send address
    pub fn start(&self, address: u8, read: bool) {
        unsafe {
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let mut val = 0;
            val |= (address as u32) << 1;
            if read {
                val |= 1 << 10; // RD_WRN = 1 (read)
            }
            val |= 1 << 13; // START
            val |= 1 << 25; // AUTOEND
            core::ptr::write_volatile(cr2, val);
        }
    }

    /// Generate STOP condition
    pub fn stop(&self) {
        unsafe {
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let mut val = core::ptr::read_volatile(cr2);
            val |= 1 << 14; // STOP
            core::ptr::write_volatile(cr2, val);
        }
    }

    /// Write a byte
    pub fn write_byte(&self, byte: u8) {
        unsafe {
            // Wait for TXIS (transmit interrupt status)
            let isr = (self.base + reg::ISR) as *mut u32;
            while (core::ptr::read_volatile(isr) & (1 << 1)) == 0 {}

            let txdr = (self.base + reg::TXDR) as *mut u32;
            core::ptr::write_volatile(txdr, byte as u32);
        }
    }

    /// Read a byte
    pub fn read_byte(&self) -> u8 {
        unsafe {
            // Wait for RXNE (receive data register not empty)
            let isr = (self.base + reg::ISR) as *mut u32;
            while (core::ptr::read_volatile(isr) & (1 << 2)) == 0 {}

            let rxdr = (self.base + reg::RXDR) as *mut u32;
            core::ptr::read_volatile(rxdr) as u8
        }
    }

    /// Check if transfer complete
    pub fn is_complete(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            (core::ptr::read_volatile(isr) & (1 << 6)) != 0 // TC
        }
    }

    /// Write data to slave
    pub fn write(&self, address: u8, data: &[u8]) {
        self.start(address, false);
        for byte in data {
            self.write_byte(*byte);
        }
        while !self.is_complete() {}
    }

    /// Read data from slave
    pub fn read(&self, address: u8, buffer: &mut [u8]) {
        unsafe {
            // Configure transfer size
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let mut val = core::ptr::read_volatile(cr2);
            val &= !(0xFF << 16);
            val |= (buffer.len() as u32) << 16;
            core::ptr::write_volatile(cr2, val);
        }

        self.start(address, true);
        for byte in buffer.iter_mut() {
            *byte = self.read_byte();
        }
    }

    /// Write then read (combined transaction)
    pub fn write_read(&self, address: u8, write_data: &[u8], read_buffer: &mut [u8]) {
        self.write(address, write_data);
        self.read(address, read_buffer);
    }
}

/// Calculate timing register value
fn calculate_timing(pclk_freq: u32, speed: I2cSpeed) -> u32 {
    match speed {
        I2cSpeed::Standard => {
            // For 100 kHz with 160 MHz PCLK
            // PRESC = 15, SCLDEL = 4, SDADEL = 2, SCLH = 39, SCLL = 49
            (15 << 28) | (4 << 20) | (2 << 16) | (39 << 8) | 49
        }
        I2cSpeed::Fast => {
            // For 400 kHz with 160 MHz PCLK
            // PRESC = 7, SCLDEL = 3, SDADEL = 0, SCLH = 12, SCLL = 39
            (7 << 28) | (3 << 20) | (0 << 16) | (12 << 8) | 39
        }
        I2cSpeed::FastPlus => {
            // For 1 MHz with 160 MHz PCLK
            // PRESC = 3, SCLDEL = 2, SDADEL = 0, SCLH = 7, SCLL = 38
            (3 << 28) | (2 << 20) | (0 << 16) | (7 << 8) | 38
        }
    }
}

/// Initialize I2C1 with default configuration
pub fn init_i2c1_default(pclk_freq: u32) {
    use super::gpio::{AlternateFunction, OutputType, Pull, Speed};

    // Enable I2C1 clock
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::I2C1);

    // Configure PB8 (SCL) and PB9 (SDA)
    let pb8 = super::gpio::pins::PB8;
    let pb9 = super::gpio::pins::PB9;

    pb8.init_alternate(
        AlternateFunction::AF4,
        OutputType::OpenDrain,
        Speed::High,
        Pull::Up,
    );
    pb9.init_alternate(
        AlternateFunction::AF4,
        OutputType::OpenDrain,
        Speed::High,
        Pull::Up,
    );

    // Initialize I2C1
    let i2c = I2c::i2c1();
    let config = Config::default();
    i2c.init(&config, pclk_freq);
}
