//! USART - Universal Synchronous/Asynchronous Receiver/Transmitter
//! 通用同步/异步收发器
//!
//! STM32U5 支持多个 USART/UART 接口，支持多种配置。

/// USART1 base address
pub const USART1_BASE: usize = 0x4001_3800;
/// USART2 base address
pub const USART2_BASE: usize = 0x4000_4400;
/// USART3 base address
pub const USART3_BASE: usize = 0x4000_4800;
/// UART4 base address
pub const UART4_BASE: usize = 0x4000_4C00;
/// UART5 base address
pub const UART5_BASE: usize = 0x4000_5000;
/// USART6 base address
pub const USART6_BASE: usize = 0x4001_6C00;
/// UART7 base address
pub const UART7_BASE: usize = 0x4000_7800;
/// UART8 base address
pub const UART8_BASE: usize = 0x4000_7C00;
/// UART9 base address
pub const UART9_BASE: usize = 0x4001_5800;

/// USART register offsets
pub mod reg {
    /// Control register 1
    pub const CR1: usize = 0x00;
    /// Control register 2
    pub const CR2: usize = 0x04;
    /// Control register 3
    pub const CR3: usize = 0x08;
    /// Baud rate register
    pub const BRR: usize = 0x0C;
    /// Guard time and prescaler register
    pub const GTPR: usize = 0x10;
    /// Receiver timeout register
    pub const RTOR: usize = 0x14;
    /// Request register
    pub const RQR: usize = 0x18;
    /// Interrupt and status register
    pub const ISR: usize = 0x1C;
    /// Interrupt flag clear register
    pub const ICR: usize = 0x20;
    /// Receive data register
    pub const RDR: usize = 0x24;
    /// Transmit data register
    pub const TDR: usize = 0x28;
    /// Prescaler register
    pub const PRESC: usize = 0x2C;
}

/// USART configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub flow_control: FlowControl,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baud_rate: 115200,
            data_bits: DataBits::Bits8,
            stop_bits: StopBits::Bits1,
            parity: Parity::None,
            flow_control: FlowControl::None,
        }
    }
}

/// Data bits
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataBits {
    Bits7 = 0b10,
    Bits8 = 0b00,
    Bits9 = 0b01,
}

/// Stop bits
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopBits {
    Bits0_5 = 0b01,
    Bits1 = 0b00,
    Bits1_5 = 0b11,
    Bits2 = 0b10,
}

/// Parity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parity {
    None = 0b00,
    Even = 0b10,
    Odd = 0b11,
}

/// Flow control
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowControl {
    None = 0b00,
    Rts = 0b01,
    Cts = 0b10,
    RtsCts = 0b11,
}

/// USART instance
pub struct Usart {
    base: usize,
}

impl Usart {
    /// Create USART1 instance
    pub const fn usart1() -> Self {
        Self { base: USART1_BASE }
    }

    /// Create USART2 instance
    pub const fn usart2() -> Self {
        Self { base: USART2_BASE }
    }

    /// Create USART3 instance
    pub const fn usart3() -> Self {
        Self { base: USART3_BASE }
    }

    /// Initialize USART with configuration
    pub fn init(&self, config: &Config, pclk_freq: u32) {
        unsafe {
            // Disable USART before configuration
            let cr1 = (self.base + reg::CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 0);

            // Configure baud rate
            // BRR = PCLK / baud_rate
            let brr = (self.base + reg::BRR) as *mut u32;
            let div = (pclk_freq + config.baud_rate / 2) / config.baud_rate;
            core::ptr::write_volatile(brr, div);

            // Configure CR2 (stop bits)
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let mut cr2_val = 0;
            cr2_val |= (config.stop_bits as u32) << 12;
            core::ptr::write_volatile(cr2, cr2_val);

            // Configure CR3 (flow control)
            let cr3 = (self.base + reg::CR3) as *mut u32;
            let mut cr3_val = 0;
            cr3_val |= (config.flow_control as u32) << 8;
            core::ptr::write_volatile(cr3, cr3_val);

            // Configure CR1 (data bits, parity, enable)
            let mut cr1_val = 0;
            cr1_val |= (config.data_bits as u32) << 28;
            cr1_val |= (config.parity as u32) << 9;
            cr1_val |= 1 << 3; // TE - Transmitter enable
            cr1_val |= 1 << 2; // RE - Receiver enable
            cr1_val |= 1 << 0; // UE - USART enable
            core::ptr::write_volatile(cr1, cr1_val);
        }
    }

    /// Send a byte
    pub fn send(&self, byte: u8) {
        unsafe {
            // Wait for TXE (transmit data register empty)
            let isr = (self.base + reg::ISR) as *mut u32;
            while (core::ptr::read_volatile(isr) & (1 << 7)) == 0 {}

            // Write data
            let tdr = (self.base + reg::TDR) as *mut u32;
            core::ptr::write_volatile(tdr, byte as u32);
        }
    }

    /// Receive a byte
    pub fn receive(&self) -> u8 {
        unsafe {
            // Wait for RXNE (read data register not empty)
            let isr = (self.base + reg::ISR) as *mut u32;
            while (core::ptr::read_volatile(isr) & (1 << 5)) == 0 {}

            // Read data
            let rdr = (self.base + reg::RDR) as *mut u32;
            core::ptr::read_volatile(rdr) as u8
        }
    }

    /// Check if data available to read
    pub fn is_rx_ready(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            (core::ptr::read_volatile(isr) & (1 << 5)) != 0
        }
    }

    /// Check if transmitter ready
    pub fn is_tx_ready(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            (core::ptr::read_volatile(isr) & (1 << 7)) != 0
        }
    }

    /// Send string
    pub fn send_str(&self, s: &str) {
        for byte in s.bytes() {
            self.send(byte);
        }
    }
}

/// Initialize USART1 for debug output (PA9 TX, PA10 RX)
pub fn init_usart1_debug(baud_rate: u32, pclk_freq: u32) {
    use super::gpio::{self, AlternateFunction, OutputType, Pull, Speed};

    // Enable USART1 clock
    crate::rcc::enable_apb2_clock(crate::rcc::apb2::USART1);

    // Configure PA9 (TX) and PA10 (RX) as alternate function
    let pa9 = gpio::pins::PA9;
    let pa10 = gpio::pins::PA10;

    pa9.init_alternate(
        AlternateFunction::AF7,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::None,
    );
    pa10.init_alternate(
        AlternateFunction::AF7,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::Up,
    );

    // Initialize USART1
    let usart = Usart::usart1();
    let config = Config {
        baud_rate,
        ..Default::default()
    };
    usart.init(&config, pclk_freq);
}

/// Global USART1 instance for debug output
static mut DEBUG_USART: Option<Usart> = None;

/// Initialize debug USART
pub fn init_debug_usart(baud_rate: u32, pclk_freq: u32) {
    init_usart1_debug(baud_rate, pclk_freq);
    unsafe {
        DEBUG_USART = Some(Usart::usart1());
    }
}

/// Print a character to debug USART
pub fn debug_putc(c: u8) {
    unsafe {
        if let Some(usart) = DEBUG_USART {
            usart.send(c);
        }
    }
}

/// Print a string to debug USART
pub fn debug_puts(s: &str) {
    for c in s.bytes() {
        debug_putc(c);
    }
}
