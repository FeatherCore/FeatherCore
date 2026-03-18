//! LPUART - Low Power Universal Asynchronous Receiver/Transmitter
//! 低功耗通用异步收发器
//!
//! # Overview / 概述
//! STM32U5 Low-Power Universal Asynchronous Receiver/Transmitter (LPUART) provides
//! flexible asynchronous communication with low power consumption capabilities.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 67: Low-power universal asynchronous receiver transmitter (LPUART)
//! 
//! ## LPUART Interfaces / LPUART 接口
//! - **LPUART1**
//! 
//! ## Transfer Speed / 传输速率
//! - Up to 10 Mbps (depending on clock source)
//! 
//! ## Advanced Features / 高级特性
//! Reference: RM0456 Section 67.2
//! - DMA support
//! - Hardware flow control (RTS/CTS)
//! - Low power modes (Stop 0, Stop 1, Stop 2, Standby)
//! - Wakeup from Stop modes
//! - Auto baud rate detection
//! - Multi-processor communication
//! - 8-bit oversampling mode
//! 
//! # Reference / 参考
//! - RM0456 Chapter 67: Low-power universal asynchronous receiver transmitter (LPUART)
//! - RM0456 Section 67.1: LPUART introduction
//! - RM0456 Section 67.2: LPUART main features
//! - RM0456 Section 67.3: LPUART functional description
//! - RM0456 Section 67.6: LPUART registers

/// LPUART1 base address / LPUART1 基地址
//! Reference: RM0456 Chapter 2, Table 1: Memory map and register boundary addresses
pub const LPUART1_BASE: usize = 0x4000_8000;

/// LPUART register offsets / LPUART 寄存器偏移
//! Reference: RM0456 Section 67.6: LPUART register map
pub mod reg {
    /// Control register 1 / 控制寄存器 1
    //! Reference: RM0456 Section 67.6.1: LPUART control register 1 (LPUART_CR1)
    pub const CR1: usize = 0x00;
    /// Control register 2 / 控制寄存器 2
    //! Reference: RM0456 Section 67.6.2: LPUART control register 2 (LPUART_CR2)
    pub const CR2: usize = 0x04;
    /// Control register 3 / 控制寄存器 3
    //! Reference: RM0456 Section 67.6.3: LPUART control register 3 (LPUART_CR3)
    pub const CR3: usize = 0x08;
    /// Baud rate register / 波特率寄存器
    //! Reference: RM0456 Section 67.6.4: LPUART baud rate register (LPUART_BRR)
    pub const BRR: usize = 0x0C;
    /// Request register / 请求寄存器
    pub const RQR: usize = 0x18;
    /// Interrupt and status register / 中断和状态寄存器
    pub const ISR: usize = 0x1C;
    /// Interrupt flag clear register / 中断标志清除寄存器
    pub const ICR: usize = 0x20;
    /// Receive data register / 接收数据寄存器
    pub const RDR: usize = 0x24;
    /// Transmit data register / 发送数据寄存器
    pub const TDR: usize = 0x28;
    /// Prescaler register / 预分频器寄存器
    pub const PRESC: usize = 0x2C;
}

/// LPUART configuration / LPUART 配置
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

/// Data bits / 数据位
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataBits {
    Bits7 = 0b10,
    Bits8 = 0b00,
    Bits9 = 0b01,
}

/// Stop bits / 停止位
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StopBits {
    Bits1 = 0b00,
    Bits2 = 0b10,
}

/// Parity / 校验位
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Parity {
    None = 0b00,
    Even = 0b10,
    Odd = 0b11,
}

/// Flow control / 流控制
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowControl {
    None = 0b00,
    Rts = 0b01,
    Cts = 0b10,
    RtsCts = 0b11,
}

/// LPUART instance / LPUART 实例
pub struct Lpuart {
    base: usize,
}

impl Lpuart {
    /// Create LPUART1 instance / 创建 LPUART1 实例
    pub const fn lpuart1() -> Self {
        Self { base: LPUART1_BASE }
    }

    /// Initialize LPUART with configuration / 使用配置初始化 LPUART
    pub fn init(&self, config: &Config, lpuart_clk: u32) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let cr2 = (self.base + reg::CR2) as *mut u32;
            let cr3 = (self.base + reg::CR3) as *mut u32;
            let brr = (self.base + reg::BRR) as *mut u32;

            core::ptr::write_volatile(cr1, 0);
            core::ptr::write_volatile(cr2, 0);
            core::ptr::write_volatile(cr3, 0);

            let brr_val = (lpuart_clk * 256 + config.baud_rate / 2) / config.baud_rate;
            core::ptr::write_volatile(brr, brr_val);

            let mut cr2_val = 0;
            cr2_val |= (config.stop_bits as u32) << 12;
            core::ptr::write_volatile(cr2, cr2_val);

            let mut cr3_val = 0;
            cr3_val |= (config.flow_control as u32) << 8;
            core::ptr::write_volatile(cr3, cr3_val);

            let mut cr1_val = 0;
            cr1_val |= (config.data_bits as u32) << 28;
            cr1_val |= (config.parity as u32) << 9;
            cr1_val |= 1 << 3;
            cr1_val |= 1 << 2;
            cr1_val |= 1 << 0;
            core::ptr::write_volatile(cr1, cr1_val);
        }
    }

    /// Send a byte / 发送一个字节
    pub fn send(&self, byte: u8) {
        unsafe {
            let isr = (self.base + reg::ISR) as *const u32;
            while (core::ptr::read_volatile(isr) & (1 << 7)) == 0 {}

            let tdr = (self.base + reg::TDR) as *mut u32;
            core::ptr::write_volatile(tdr, byte as u32);
        }
    }

    /// Receive a byte / 接收一个字节
    pub fn receive(&self) -> u8 {
        unsafe {
            let isr = (self.base + reg::ISR) as *const u32;
            while (core::ptr::read_volatile(isr) & (1 << 5)) == 0 {}

            let rdr = (self.base + reg::RDR) as *const u32;
            core::ptr::read_volatile(rdr) as u8
        }
    }

    /// Check if data available to read / 检查是否有数据可读
    pub fn is_rx_ready(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *const u32;
            (core::ptr::read_volatile(isr) & (1 << 5)) != 0
        }
    }

    /// Check if transmitter ready / 检查发送器是否准备好
    pub fn is_tx_ready(&self) -> bool {
        unsafe {
            let isr = (self.base + reg::ISR) as *const u32;
            (core::ptr::read_volatile(isr) & (1 << 7)) != 0
        }
    }

    /// Send string / 发送字符串
    pub fn send_str(&self, s: &str) {
        for byte in s.bytes() {
            self.send(byte);
        }
    }

    /// Enable wakeup from stop mode / 使能从停止模式唤醒
    pub fn enable_wakeup(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= 1 << 2;
            core::ptr::write_volatile(cr1, val);

            let cr3 = (self.base + reg::CR3) as *mut u32;
            let mut val3 = core::ptr::read_volatile(cr3);
            val3 |= 1 << 21;
            core::ptr::write_volatile(cr3, val3);
        }
    }

    /// Disable wakeup from stop mode / 禁用从停止模式唤醒
    pub fn disable_wakeup(&self) {
        unsafe {
            let cr1 = (self.base + reg::CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val &= !(1 << 2);
            core::ptr::write_volatile(cr1, val);

            let cr3 = (self.base + reg::CR3) as *mut u32;
            let mut val3 = core::ptr::read_volatile(cr3);
            val3 &= !(1 << 21);
            core::ptr::write_volatile(cr3, val3);
        }
    }
}

/// Initialize LPUART1 for debug output (PA2 TX, PA3 RX)
/// 初始化 LPUART1 用于调试输出 (PA2 TX, PA3 RX)
pub fn init_lpuart1_debug(baud_rate: u32, lpuart_clk: u32) {
    use super::gpio::{AlternateFunction, OutputType, Pull, Speed};

    crate::rcc::enable_apb1_clock(crate::rcc::apb1::LPUART1);

    let pa2 = super::gpio::pins::PA2;
    let pa3 = super::gpio::pins::PA3;

    pa2.init_alternate(
        AlternateFunction::AF8,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::None,
    );
    pa3.init_alternate(
        AlternateFunction::AF8,
        OutputType::PushPull,
        Speed::VeryHigh,
        Pull::Up,
    );

    let lpuart = Lpuart::lpuart1();
    let config = Config {
        baud_rate,
        ..Default::default()
    };
    lpuart.init(&config, lpuart_clk);
}

/// Global LPUART1 instance for debug output / 用于调试输出的全局 LPUART1 实例
static mut DEBUG_LPUART: Option<Lpuart> = None;

/// Initialize debug LPUART / 初始化调试 LPUART
pub fn init_debug_lpuart(baud_rate: u32, lpuart_clk: u32) {
    init_lpuart1_debug(baud_rate, lpuart_clk);
    unsafe {
        DEBUG_LPUART = Some(Lpuart::lpuart1());
    }
}

/// Print a character to debug LPUART / 向调试 LPUART 打印一个字符
pub fn debug_putc(c: u8) {
    unsafe {
        if let Some(lpuart) = DEBUG_LPUART {
            lpuart.send(c);
        }
    }
}

/// Print a string to debug LPUART / 向调试 LPUART 打印字符串
pub fn debug_puts(s: &str) {
    for c in s.bytes() {
        debug_putc(c);
    }
}
