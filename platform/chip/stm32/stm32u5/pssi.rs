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
//! - RM0456 Reference Manual, Chapter 51: Parallel synchronous slave interface (PSSI)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// PSSI base address / PSSI 基地址
/// AHB1 bus, accessible at 0x4004_0400
pub const PSSI_BASE: usize = 0x4004_0400;

// ============================================================================
// PSSI Register Map / PSSI 寄存器映射
// ============================================================================

/// PSSI Register Structure / PSSI 寄存器结构
#[repr(C)]
pub struct PssiRegs {
    /// PSSI Control Register / PSSI 控制寄存器
    pub cr: u32,
    /// PSSI Status Register / PSSI 状态寄存器
    pub sr: u32,
    /// PSSI Raw Interrupt Status Register / PSSI 原始中断状态寄存器
    pub risr: u32,
    /// PSSI Interrupt Enable Register / PSSI 中断使能寄存器
    pub ier: u32,
    /// PSSI Masked Interrupt Status Register / PSSI 屏蔽中断状态寄存器
    pub misr: u32,
    /// PSSI Interrupt Clear Register / PSSI 中断清除寄存器
    pub icr: u32,
    /// PSSI Data Register / PSSI 数据寄存器
    pub dr: u32,
}

/// PSSI instance / PSSI 实例
pub struct Pssi;

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

/// PSSI transfer descriptor / PSSI 传输描述符
#[derive(Clone, Copy, Debug)]
pub struct PssiTransfer {
    /// Data pointer / 数据指针
    pub data_ptr: *const u8,
    /// Transfer length / 传输长度
    pub length: usize,
}

impl Pssi {
    pub fn new() -> Self {
        Pssi
    }

    fn regs(&self) -> &mut PssiRegs {
        unsafe { &mut *(PSSI_BASE as *mut PssiRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr |= 1 << 12;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr &= !(1 << 12);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2rstr = rcc_base.add(0x94 / 4);
            *ahb2rstr |= 1 << 12;
            *ahb2rstr &= !(1 << 12);
        }
    }

    pub fn configure(&self, config: &Config) {
        let cr = (config.data_width as u32) << 0
            | (config.bus_width as u32) << 4
            | (config.control_signal as u32) << 6
            | (config.ready_polarity as u32) << 8
            | (config.outen_enable as u32) << 16
            | (config.dma_enable as u32) << 17
            | (config.continuous as u32) << 18;
        unsafe { write_volatile(&mut self.regs().cr, cr) };
    }

    pub fn enable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 14));
        }
    }

    pub fn disable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 14));
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { (read_volatile(&self.regs().cr) & (1 << 14)) != 0 }
    }

    pub fn start_transfer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 15));
        }
    }

    pub fn stop_transfer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 15));
        }
    }

    pub fn is_transfer_active(&self) -> bool {
        unsafe { (read_volatile(&self.regs().cr) & (1 << 15)) != 0 }
    }

    pub fn write_data(&self, data: u16) {
        unsafe { write_volatile(&mut self.regs().dr, data as u32) };
    }

    pub fn read_data(&self) -> u16 {
        unsafe { read_volatile(&self.regs().dr) as u16 }
    }

    pub fn is_rx_not_empty(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x01) != 0 }
    }

    pub fn is_tx_empty(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x02) != 0 }
    }

    pub fn is_busy(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x04) != 0 }
    }

    pub fn is_overrun(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x08) != 0 }
    }

    pub fn clear_overrun(&self) {
        unsafe { write_volatile(&mut self.regs().icr, 0x08) };
    }

    pub fn enable_interrupt(&self, source: u8) {
        unsafe {
            let ier = read_volatile(&self.regs().ier);
            write_volatile(&mut self.regs().ier, ier | (1 << source));
        }
    }

    pub fn disable_interrupt(&self, source: u8) {
        unsafe {
            let ier = read_volatile(&self.regs().ier);
            write_volatile(&mut self.regs().ier, ier & !(1 << source));
        }
    }

    pub fn is_interrupt_active(&self, source: u8) -> bool {
        unsafe { (read_volatile(&self.regs().misr) & (1 << source)) != 0 }
    }

    pub fn clear_interrupt(&self, source: u8) {
        unsafe { write_volatile(&mut self.regs().icr, 1 << source) };
    }

    pub fn wait_for_tx_empty(&self) {
        while !self.is_tx_empty() {}
    }

    pub fn wait_for_rx_not_empty(&self) {
        while !self.is_rx_not_empty() {}
    }

    pub fn send_byte(&self, data: u8) {
        self.wait_for_tx_empty();
        self.write_data(data as u16);
    }

    pub fn send_halfword(&self, data: u16) {
        self.wait_for_tx_empty();
        self.write_data(data);
    }

    pub fn receive_byte(&self) -> u8 {
        self.wait_for_rx_not_empty();
        self.read_data() as u8
    }

    pub fn receive_halfword(&self) -> u16 {
        self.wait_for_rx_not_empty();
        self.read_data()
    }

    pub fn send_buffer(&self, buffer: &[u8]) {
        for &byte in buffer {
            self.send_byte(byte);
        }
    }

    pub fn receive_buffer(&self, buffer: &mut [u8]) {
        for byte in buffer.iter_mut() {
            *byte = self.receive_byte();
        }
    }

    pub fn send_buffer_16(&self, buffer: &[u16]) {
        for &halfword in buffer {
            self.send_halfword(halfword);
        }
    }

    pub fn receive_buffer_16(&self, buffer: &mut [u16]) {
        for halfword in buffer.iter_mut() {
            *halfword = self.receive_halfword();
        }
    }
}
