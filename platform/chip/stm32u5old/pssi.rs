//! PSSI - Parallel Synchronous Slave Interface
//! 并行同步从接口
//!
//! ## STM32U5 PSSI 特性 / Features
//! - **数据宽度 / Data Width:**
//!   - 8-bit 数据传输
//!   - 16-bit 数据传输
//!
//! - **接口特性 / Interface Features:**
//!   - 并行同步从接口
//!   - DMA 支持
//!   - 可编程控制信号
//!   - 准备好 (Ready) 信号支持
//!
//! - **传输模式 / Transfer Modes:**
//!   - 连续模式
//!   - 非连续模式
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 42: Parallel synchronous slave interface (PSSI)
//! - RM0456 Section 42.1: PSSI introduction
//! - RM0456 Section 42.2: PSSI main features
//! - RM0456 Section 42.3: PSSI functional description
//! - RM0456 Section 42.4: PSSI registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// PSSI base address / PSSI 基地址
pub const PSSI_BASE: usize = 0x4004_0400;

/// PSSI register offsets / PSSI 寄存器偏移
//! Reference: RM0456 Section 42.4: PSSI registers
pub mod reg {
    /// PSSI Control Register / PSSI 控制寄存器
    //! Reference: RM0456 Section 42.4.1: PSSI_CR
    pub const CR: usize = 0x00;
    /// PSSI Status Register / PSSI 状态寄存器
    //! Reference: RM0456 Section 42.4.2: PSSI_SR
    pub const SR: usize = 0x04;
    /// PSSI Raw Interrupt Status Register / PSSI 原始中断状态寄存器
    //! Reference: RM0456 Section 42.4.3: PSSI_RIS
    pub const RIS: usize = 0x08;
    /// PSSI Interrupt Enable Register / PSSI 中断使能寄存器
    //! Reference: RM0456 Section 42.4.4: PSSI_IER
    pub const IER: usize = 0x0C;
    /// PSSI Masked Interrupt Status Register / PSSI 屏蔽中断状态寄存器
    //! Reference: RM0456 Section 42.4.5: PSSI_MIS
    pub const MIS: usize = 0x10;
    /// PSSI Interrupt Clear Register / PSSI 中断清除寄存器
    //! Reference: RM0456 Section 42.4.6: PSSI_ICR
    pub const ICR: usize = 0x14;
    /// PSSI Data Register / PSSI 数据寄存器
    //! Reference: RM0456 Section 42.4.7: PSSI_DR
    pub const DR: usize = 0x18;
}

/// CR Register Bit Definitions / CR 寄存器位定义
//! Reference: RM0456 Section 42.4.1
pub mod cr_bits {
    /// Data width / 数据宽度
    pub const DDW: u32 = 1 << 0;
    /// Bus width / 总线宽度
    pub const DBW: u32 = 1 << 4;
    /// Control signal / 控制信号
    pub const CKPOL: u32 = 0b11 << 6;
    /// Ready polarity / 准备好极性
    pub const RDEPOL: u32 = 1 << 8;
    /// Output enable / 输出使能
    pub const OUTEN: u32 = 1 << 16;
    /// DMA enable / DMA 使能
    pub const DMAEN: u32 = 1 << 17;
    /// Continuous mode / 连续模式
    pub const DC: u32 = 1 << 18;
    /// PSSI enable / PSSI 使能
    pub const PSSIEN: u32 = 1 << 14;
    /// Transfer start / 传输开始
    pub const START: u32 = 1 << 15;
}

/// SR Register Bit Definitions / SR 寄存器位定义
//! Reference: RM0456 Section 42.4.2
pub mod sr_bits {
    /// RBNE / 接收缓冲区非空
    pub const RBNE: u32 = 1 << 0;
    /// TBE / 发送缓冲区空
    pub const TBE: u32 = 1 << 1;
    /// BUSY / 忙
    pub const BUSY: u32 = 1 << 2;
    /// OVR / 溢出
    pub const OVR: u32 = 1 << 3;
}

/// RIS/IER/MIS/ICR Register Bit Definitions / RIS/IER/MIS/ICR 寄存器位定义
//! Reference: RM0456 Section 42.4.3 to 42.4.6
pub mod int_bits {
    /// RBNE interrupt / RBNE 中断
    pub const RBNE: u32 = 1 << 0;
    /// TBE interrupt / TBE 中断
    pub const TBE: u32 = 1 << 1;
    /// BUSY interrupt / BUSY 中断
    pub const BUSY: u32 = 1 << 2;
    /// OVR interrupt / OVR 中断
    pub const OVR: u32 = 1 << 3;
}

/// Data width / 数据宽度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataWidth {
    /// 8-bit data width / 8位数据宽度
    Bits8 = 0,
    /// 16-bit data width / 16位数据宽度
    Bits16 = 1,
}

/// Bus width / 总线宽度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BusWidth {
    /// 8-bit bus width / 8位总线宽度
    Bits8 = 0,
    /// 16-bit bus width / 16位总线宽度
    Bits16 = 1,
}

/// Control signal / 控制信号
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlSignal {
    /// Both PCLK and PDEN / PCLK 和 PDEN 都使用
    Both = 0,
    /// PCLK only / 仅使用 PCLK
    PclkOnly = 1,
    /// PDEN only / 仅使用 PDEN
    PdenOnly = 2,
}

/// Ready polarity / 准备好极性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadyPolarity {
    /// Active low / 低电平有效
    ActiveLow = 0,
    /// Active high / 高电平有效
    ActiveHigh = 1,
}

/// PSSI configuration / PSSI 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Data width / 数据宽度
    pub data_width: DataWidth,
    /// Bus width / 总线宽度
    pub bus_width: BusWidth,
    /// Control signal / 控制信号
    pub control_signal: ControlSignal,
    /// Ready polarity / 准备好极性
    pub ready_polarity: ReadyPolarity,
    /// Output enable / 输出使能
    pub outen_enable: bool,
    /// DMA enable / DMA 使能
    pub dma_enable: bool,
    /// Continuous mode / 连续模式
    pub continuous: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            data_width: DataWidth::Bits8,
            bus_width: BusWidth::Bits8,
            control_signal: ControlSignal::Both,
            ready_polarity: ReadyPolarity::ActiveHigh,
            outen_enable: false,
            dma_enable: false,
            continuous: true,
        }
    }
}

/// PSSI instance / PSSI 实例
pub struct Pssi;

impl Pssi {
    /// Create new PSSI instance / 创建新的 PSSI 实例
    pub const fn new() -> Self {
        Pssi
    }

    /// Enable PSSI clock / 使能 PSSI 时钟
    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr |= 1 << 12;
        }
    }

    /// Disable PSSI clock / 禁用 PSSI 时钟
    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr &= !(1 << 12);
        }
    }

    /// Configure PSSI / 配置 PSSI
    pub fn configure(&self, config: &Config) {
        unsafe {
            let cr = (PSSI_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= (config.data_width as u32) << 0;
            val |= (config.bus_width as u32) << 4;
            val |= (config.control_signal as u32) << 6;
            val |= (config.ready_polarity as u32) << 8;
            val |= (config.outen_enable as u32) << 16;
            val |= (config.dma_enable as u32) << 17;
            val |= (config.continuous as u32) << 18;
            write_volatile(cr, val);
        }
    }

    /// Enable PSSI / 使能 PSSI
    pub fn enable(&self) {
        unsafe {
            let cr = (PSSI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::PSSIEN;
            write_volatile(cr, val);
        }
    }

    /// Disable PSSI / 禁用 PSSI
    pub fn disable(&self) {
        unsafe {
            let cr = (PSSI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::PSSIEN;
            write_volatile(cr, val);
        }
    }

    /// Start transfer / 开始传输
    pub fn start_transfer(&self) {
        unsafe {
            let cr = (PSSI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::START;
            write_volatile(cr, val);
        }
    }

    /// Stop transfer / 停止传输
    pub fn stop_transfer(&self) {
        unsafe {
            let cr = (PSSI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::START;
            write_volatile(cr, val);
        }
    }

    /// Write data / 写入数据
    pub fn write_data(&self, data: u16) {
        unsafe {
            let dr = (PSSI_BASE + reg::DR) as *mut u32;
            write_volatile(dr, data as u32);
        }
    }

    /// Read data / 读取数据
    pub fn read_data(&self) -> u16 {
        unsafe {
            let dr = (PSSI_BASE + reg::DR) as *const u32;
            read_volatile(dr) as u16
        }
    }

    /// Check if receive buffer not empty / 检查接收缓冲区是否非空
    pub fn is_rx_not_empty(&self) -> bool {
        unsafe {
            let sr = (PSSI_BASE + reg::SR) as *const u32;
            (read_volatile(sr) & sr_bits::RBNE) != 0
        }
    }

    /// Check if transmit buffer empty / 检查发送缓冲区是否空
    pub fn is_tx_empty(&self) -> bool {
        unsafe {
            let sr = (PSSI_BASE + reg::SR) as *const u32;
            (read_volatile(sr) & sr_bits::TBE) != 0
        }
    }

    /// Check if busy / 检查是否忙
    pub fn is_busy(&self) -> bool {
        unsafe {
            let sr = (PSSI_BASE + reg::SR) as *const u32;
            (read_volatile(sr) & sr_bits::BUSY) != 0
        }
    }

    /// Check if overrun / 检查是否溢出
    pub fn is_overrun(&self) -> bool {
        unsafe {
            let sr = (PSSI_BASE + reg::SR) as *const u32;
            (read_volatile(sr) & sr_bits::OVR) != 0
        }
    }

    /// Clear overrun flag / 清除溢出标志
    pub fn clear_overrun(&self) {
        unsafe {
            let icr = (PSSI_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, int_bits::OVR);
        }
    }

    /// Enable interrupt / 使能中断
    pub fn enable_interrupt(&self, source: u8) {
        unsafe {
            let ier = (PSSI_BASE + reg::IER) as *mut u32;
            let mut val = read_volatile(ier);
            val |= 1 << source;
            write_volatile(ier, val);
        }
    }

    /// Disable interrupt / 禁用中断
    pub fn disable_interrupt(&self, source: u8) {
        unsafe {
            let ier = (PSSI_BASE + reg::IER) as *mut u32;
            let mut val = read_volatile(ier);
            val &= !(1 << source);
            write_volatile(ier, val);
        }
    }

    /// Wait for TX empty / 等待 TX 空
    pub fn wait_for_tx_empty(&self) {
        while !self.is_tx_empty() {}
    }

    /// Wait for RX not empty / 等待 RX 非空
    pub fn wait_for_rx_not_empty(&self) {
        while !self.is_rx_not_empty() {}
    }

    /// Send byte / 发送字节
    pub fn send_byte(&self, data: u8) {
        self.wait_for_tx_empty();
        self.write_data(data as u16);
    }

    /// Receive byte / 接收字节
    pub fn receive_byte(&self) -> u8 {
        self.wait_for_rx_not_empty();
        self.read_data() as u8
    }

    /// Send halfword / 发送半字
    pub fn send_halfword(&self, data: u16) {
        self.wait_for_tx_empty();
        self.write_data(data);
    }

    /// Receive halfword / 接收半字
    pub fn receive_halfword(&self) -> u16 {
        self.wait_for_rx_not_empty();
        self.read_data()
    }

    /// Send buffer / 发送缓冲区
    pub fn send_buffer(&self, buffer: &[u8]) {
        for &byte in buffer {
            self.send_byte(byte);
        }
    }

    /// Receive buffer / 接收缓冲区
    pub fn receive_buffer(&self, buffer: &mut [u8]) {
        for byte in buffer.iter_mut() {
            *byte = self.receive_byte();
        }
    }
}
