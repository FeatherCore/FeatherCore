//! SPI - Serial Peripheral Interface
//! 串行外设接口
//!
//! STM32U5 支持多个 SPI/I2S 接口，支持全双工同步通信。

/// SPI1 base address
pub const SPI1_BASE: usize = 0x4001_3000;
/// SPI2 base address
pub const SPI2_BASE: usize = 0x4000_3800;
/// SPI3 base address
pub const SPI3_BASE: usize = 0x4000_3C00;

/// SPI register offsets
pub mod reg {
    /// Control register 1
    pub const CR1: usize = 0x00;
    /// Control register 2
    pub const CR2: usize = 0x04;
    /// Status register
    pub const SR: usize = 0x08;
    /// Data register
    pub const DR: usize = 0x0C;
    /// CRC polynomial register
    pub const CRCPR: usize = 0x10;
    /// RX CRC register
    pub const RXCRCR: usize = 0x14;
    /// TX CRC register
    pub const TXCRCR: usize = 0x18;
    /// Configuration register
    pub const I2SCFGR: usize = 0x1C;
    /// Prescaler register
    pub const I2SPR: usize = 0x20;
    /// Transfer size register
    pub const TSIZE: usize = 0x24;
    /// Interrupt enable register
    pub const IER: usize = 0x2C;
}

/// SPI mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Mode 0: CPOL=0, CPHA=0
    Mode0 = 0,
    /// Mode 1: CPOL=0, CPHA=1
    Mode1 = 1,
    /// Mode 2: CPOL=1, CPHA=0
    Mode2 = 2,
    /// Mode 3: CPOL=1, CPHA=1
    Mode3 = 3,
}

/// SPI data size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataSize {
    /// 4 bits
    Bits4 = 0b0011,
    /// 5 bits
    Bits5 = 0b0100,
    /// 6 bits
    Bits6 = 0b0101,
    /// 7 bits
    Bits7 = 0b0110,
    /// 8 bits
    Bits8 = 0b0111,
    /// 9 bits
    Bits9 = 0b1000,
    /// 10 bits
    Bits10 = 0b1001,
    /// 11 bits
    Bits11 = 0b1010,
    /// 12 bits
    Bits12 = 0b1011,
    /// 13 bits
    Bits13 = 0b1100,
    /// 14 bits
    Bits14 = 0b1101,
    /// 15 bits
    Bits15 = 0b1110,
    /// 16 bits
    Bits16 = 0b1111,
}

/// SPI bit order
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BitOrder {
    /// MSB first
    MsbFirst = 0,
    /// LSB first
    LsbFirst = 1,
}

/// SPI configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub mode: Mode,
    pub data_size: DataSize,
    pub bit_order: BitOrder,
    pub baud_rate_prescaler: u8,
    pub master: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Mode0,
            data_size: DataSize::Bits8,
            bit_order: BitOrder::MsbFirst,
            baud_rate_prescaler: 0b001, // fPCLK/4
            master: true,
        }
    }
}

/// SPI instance
pub struct Spi {
    base: usize,
}

impl Spi {
    /// Create SPI1 instance
    pub const fn spi1() -> Self {
        Self { base: SPI1_BASE }
    }

    /// Create SPI2 instance
    pub const fn spi2() -> Self {
        Self { base: SPI2_BASE }
    }

    /// Create SPI3 instance
    pub const fn spi3() -> Self {
        Self { base: SPI3_BASE }
    }

    /// Initialize SPI
    pub fn init(&self, config: &Config) {
        unsafe {
            // Disable SPI before configuration
            let cr1 = (self.base + reg::CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 0);

            // Configure CR1
            let mut cr1_val = 0;
            cr1_val |= (config.mode as u32 & 0b01) << 0; // CPHA
            cr1_val |= ((config.mode as u32 >> 1) & 0b01) << 1; // CPOL
            cr1_val |= (config.master as u32) << 2; // MSTR
            cr1_val |= (config.baud_rate_prescaler as u32 & 0b111) << 3; // BR
            cr1_val |= (config.bit_order as u32) << 7; // LSBFIRST
            cr1_val |= 1 << 6; // SPE - SPI enable
            core::ptr::write_volatile(cr1, cr1_val);

            // Configure CR2
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let mut cr2_val = 0;
            cr2_val |= (config.data_size as u32) << 0; // DS
            cr2_val |= 1 << 12; // SSOE - SS output enable
            core::ptr::write_volatile(cr2, cr2_val);
        }
    }

    /// Enable SPI
    pub fn enable(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= 1 << 6; // SPE
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Disable SPI
    pub fn disable(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val &= !(1 << 6); // SPE
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Check if transmit buffer is empty
    pub fn is_tx_empty(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 1)) != 0 // TXE
        }
    }

    /// Check if receive buffer is not empty
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 0)) != 0 // RXNE
        }
    }

    /// Check if SPI is busy
    pub fn is_busy(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 7)) != 0 // BSY
        }
    }

    /// Write a byte
    pub fn write(&self, byte: u8) {
        unsafe {
            // Wait for TXE
            while !self.is_tx_empty() {}

            let dr = (self.base + reg::DR) as *mut u32;
            core::ptr::write_volatile(dr, byte as u32);
        }
    }

    /// Read a byte
    pub fn read(&self) -> u8 {
        unsafe {
            // Wait for RXNE
            while !self.is_rx_not_empty() {}

            let dr = (self.base + reg::DR) as *mut u32;
            core::ptr::read_volatile(dr) as u8
        }
    }

    /// Transfer a byte (write and read simultaneously)
    pub fn transfer(&self, byte: u8) -> u8 {
        self.write(byte);
        self.read()
    }

    /// Transfer multiple bytes
    pub fn transfer_slice(&self, write_data: &[u8], read_buffer: &mut [u8]) {
        for (i, &byte) in write_data.iter().enumerate() {
            let received = self.transfer(byte);
            if i < read_buffer.len() {
                read_buffer[i] = received;
            }
        }
    }

    /// Send data
    pub fn send(&self, data: &[u8]) {
        for &byte in data {
            self.write(byte);
        }
        // Wait for completion
        while self.is_busy() {}
    }

    /// Receive data
    pub fn receive(&self, buffer: &mut [u8]) {
        for byte in buffer.iter_mut() {
            *byte = self.transfer(0xFF);
        }
    }
}

/// Initialize SPI1 with default configuration
pub fn init_spi1_default() {
    use super::gpio::{AlternateFunction, OutputType, Pull, Speed};

    // Enable SPI1 clock
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::SPI1);

    // Configure PA5 (SCK), PA6 (MISO), PA7 (MOSI)
    let pa5 = super::gpio::pins::PA5;
    let pa6 = super::gpio::pins::PA6;
    let pa7 = super::gpio::pins::PA7;

    pa5.init_alternate(
        AlternateFunction::AF5,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::None,
    );
    pa6.init_alternate(
        AlternateFunction::AF5,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::None,
    );
    pa7.init_alternate(
        AlternateFunction::AF5,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::None,
    );

    // Initialize SPI1
    let spi = Spi::spi1();
    let config = Config::default();
    spi.init(&config);
}
