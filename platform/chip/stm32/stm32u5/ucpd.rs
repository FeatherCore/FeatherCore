//! UCPD - USB Power Delivery Controller
//! USB 电源传输控制器
//!
//! ## STM32U5 UCPD 特性 / Features
//! - **USB PD 协议 / USB PD Protocol:**
//!   - 支持 USB Power Delivery 规范
//!   - 源 (Source) 和受体 (Sink) 角色
//!   - 双角色设备 (DRD) 支持
//!
//! - **物理层 / Physical Layer:"
//!   - CC (Configuration Channel) 引脚管理
//!   - 可编程 Rp/Rd 电阻
//!   - BMC 编码/解码
//!
//! - **消息传输 / Message Transfer:"
//!   - 可编程消息有序集
//!   - 可配置有效载荷大小
//!   - DMA 支持
//!
//! - **事件处理 / Event Handling:"
//!   - 中断支持
//!   - 状态监测
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 42: USB Power Delivery controller

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// UCPD1 base address / UCPD1 基地址
/// AHB bus, accessible at 0x4000_DC00
pub const UCPD1_BASE: usize = 0x4000_DC00;
/// UCPD2 base address / UCPD2 基地址
pub const UCPD2_BASE: usize = 0x4000_E000;

// ============================================================================
// UCPD Register Map / UCPD 寄存器映射
// ============================================================================

/// UCPD Register Structure / UCPD 寄存器结构
#[repr(C)]
pub struct UcpdRegs {
    /// UCPD Configuration Register 1 / UCPD 配置寄存器 1
    pub cfg1: u32,
    /// UCPD Configuration Register 2 / UCPD 配置寄存器 2
    pub cfg2: u32,
    /// UCPD Control Register / UCPD 控制寄存器
    pub cr: u32,
    /// UCPD Interrupt Mask Register / UCPD 中断屏蔽寄存器
    pub imr: u32,
    /// UCPD Status Register / UCPD 状态寄存器
    pub sr: u32,
    /// UCPD Interrupt Clear Register / UCPD 中断清除寄存器
    pub icr: u32,
    /// UCPD TX Ordered Set Register / UCPD 发送有序集寄存器
    pub tx_ordset: u32,
    /// UCPD TX Payload Size Register / UCPD 发送有效载荷大小寄存器
    pub tx_paysz: u32,
    /// UCPD TX Data Register / UCPD 发送数据寄存器
    pub txdr: u32,
    /// UCPD RX Ordered Set Register / UCPD 接收有序集寄存器
    pub rx_ordset: u32,
    /// UCPD RX Payload Size Register / UCPD 接收有效载荷大小寄存器
    pub rx_paysz: u32,
    /// UCPD RX Data Register / UCPD 接收数据寄存器
    pub rxdr: u32,
    /// UCPD RX Ordered Set Extension 1 Register / UCPD 接收有序集扩展寄存器 1
    pub rx_ordext1: u32,
    /// UCPD RX Ordered Set Extension 2 Register / UCPD 接收有序集扩展寄存器 2
    pub rx_ordext2: u32,
}

/// UCPD instance / UCPD 实例
pub struct Ucpd {
    /// Base address / 基地址
    base: usize,
}

/// UCPD instance selection / UCPD 实例选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instance {
    /// UCPD1 instance / UCPD1 实例
    Ucpd1 = 1,
    /// UCPD2 instance / UCPD2 实例
    Ucpd2 = 2,
}

/// CC pin selection / CC 引脚选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CcSel {
    /// CC1 selected / 选择 CC1
    Cc1 = 0,
    /// CC2 selected / 选择 CC2
    Cc2 = 1,
}

/// CC Rp (Source) value / CC Rp (源端) 值
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CcRp {
    /// Default Rp (USB Default Power) / 默认 Rp (USB 默认功率)
    Default = 0,
    /// 1.5A current capability / 1.5A 电流能力
    Power15A = 1,
    /// 3.0A current capability / 3.0A 电流能力
    Power30A = 2,
    /// USB Power (reserved) / USB 功率 (保留)
    PowerUsb = 3,
}

/// CC Detection (Sink) mode / CC 检测 (受体) 模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PsCcdet {
    /// Disabled / 禁用
    Disabled = 0,
    /// Vsrc Default / Vsrc 默认
    VsrcDef = 1,
    /// Vsrc 1.5V / Vsrc 1.5V
    Vsrc1500 = 2,
    /// Vsrc 3.0V / Vsrc 3.0V
    Vsrc3000 = 3,
}

/// H-bit clock divider / H 位时钟分频器
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hbitclkdiv {
    /// Clock divide by 1 / 时钟 1 分频
    Div1 = 0,
    /// Clock divide by 2 / 时钟 2 分频
    Div2 = 1,
    /// Clock divide by 4 / 时钟 4 分频
    Div4 = 2,
    /// Clock divide by 8 / 时钟 8 分频
    Div8 = 3,
}

/// UCPD role / UCPD 角色
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UcpdRole {
    /// Sink (Power Consumer) / 受体 (功率消耗者)
    Sink = 0,
    /// Source (Power Provider) / 源端 (功率提供者)
    Source = 1,
    /// Dual Role Device / 双角色设备
    Drd = 2,
}

/// UCPD configuration / UCPD 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// CC pin selection / CC 引脚选择
    pub cc_sel: CcSel,
    /// CC Rp value / CC Rp 值
    pub cc_rp: CcRp,
    /// CC Rd enable / CC Rd 使能
    pub cc_rd: bool,
    /// Power detection mode / 功率检测模式
    pub ps_ccdet: PsCcdet,
    /// H-bit clock divider / H 位时钟分频器
    pub hbitclkdiv: Hbitclkdiv,
    /// UCPD role / UCPD 角色
    pub ucpd_role: UcpdRole,
    /// FRS RX enable / 快速角色交换接收使能
    pub frs_rx_en: bool,
    pub frs_tx_en: bool,
    pub rx_dma_en: bool,
    pub tx_dma_en: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cc_sel: CcSel::Cc1,
            cc_rp: CcRp::Default,
            cc_rd: true,
            ps_ccdet: PsCcdet::VsrcDef,
            hbitclkdiv: Hbitclkdiv::Div1,
            ucpd_role: UcpdRole::Sink,
            frs_rx_en: false,
            frs_tx_en: false,
            rx_dma_en: false,
            tx_dma_en: false,
        }
    }
}

pub struct PowerContract {
    pub max_voltage_mv: u16,
    pub max_current_ma: u16,
    pub max_power_mw: u16,
}

impl Default for PowerContract {
    fn default() -> Self {
        PowerContract {
            max_voltage_mv: 5000,
            max_current_ma: 500,
            max_power_mw: 2500,
        }
    }
}

impl Ucpd {
    pub fn new(instance: Instance) -> Self {
        let base = match instance {
            Instance::Ucpd1 => UCPD1_BASE,
            Instance::Ucpd2 => UCPD2_BASE,
        };
        Ucpd { base }
    }

    fn regs(&self) -> &mut UcpdRegs {
        unsafe { &mut *(self.base as *mut UcpdRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let apb1enr2 = rcc_base.add(0xE0 / 4);
            let bit = if self.base == UCPD1_BASE { 8 } else { 9 };
            *apb1enr2 |= 1 << bit;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let apb1enr2 = rcc_base.add(0xE0 / 4);
            let bit = if self.base == UCPD1_BASE { 8 } else { 9 };
            *apb1enr2 &= !(1 << bit);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let apb1rstr2 = rcc_base.add(0x98 / 4);
            let bit = if self.base == UCPD1_BASE { 8 } else { 9 };
            *apb1rstr2 |= 1 << bit;
            *apb1rstr2 &= !(1 << bit);
        }
    }

    pub fn configure(&self, config: &Config) {
        let cfg1 = (config.cc_sel as u32) << 17
            | (config.cc_rp as u32) << 19
            | (config.cc_rd as u32) << 18
            | (config.ps_ccdet as u32) << 21
            | (config.hbitclkdiv as u32) << 0
            | (config.frs_rx_en as u32) << 16
            | (config.frs_tx_en as u32) << 15
            | (config.rx_dma_en as u32) << 14
            | (config.tx_dma_en as u32) << 13;
        unsafe { write_volatile(&mut self.regs().cfg1, cfg1) };

        let cfg2 = (config.ucpd_role as u32) << 0;
        unsafe { write_volatile(&mut self.regs().cfg2, cfg2) };
    }

    pub fn enable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 0));
        }
    }

    pub fn disable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 0));
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { (read_volatile(&self.regs().cr) & 0x01) != 0 }
    }

    pub fn start_rx(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 1));
        }
    }

    pub fn stop_rx(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 1));
        }
    }

    pub fn start_tx(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 2));
        }
    }

    pub fn stop_tx(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 2));
        }
    }

    pub fn send_hard_reset(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 3));
        }
    }

    pub fn send_cable_reset(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 4));
        }
    }

    pub fn send_frs(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 5));
        }
    }

    pub fn set_tx_message(&self, header: u16, data: &[u8]) {
        unsafe {
            write_volatile(&mut self.regs().tx_paysz, data.len() as u32);
            write_volatile(&mut self.regs().tx_ordset, header as u32);
            for &byte in data {
                write_volatile(&mut self.regs().txdr, byte as u32);
            }
        }
    }

    pub fn get_rx_message_size(&self) -> u16 {
        unsafe { read_volatile(&self.regs().rx_paysz) as u16 }
    }

    pub fn get_rx_header(&self) -> u16 {
        unsafe { read_volatile(&self.regs().rx_ordset) as u16 }
    }

    pub fn read_rx_data(&self, buffer: &mut [u8]) -> usize {
        let size = self.get_rx_message_size() as usize;
        let to_read = size.min(buffer.len());
        for i in 0..to_read {
            buffer[i] = unsafe { read_volatile(&self.regs().rxdr) as u8 };
        }
        to_read
    }

    pub fn is_rx_data_ready(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x01) != 0 }
    }

    pub fn is_tx_complete(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x02) != 0 }
    }

    pub fn is_rx_overflow(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x04) != 0 }
    }

    pub fn is_rx_hard_reset(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x08) != 0 }
    }

    pub fn is_rx_msg_end(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x10) != 0 }
    }

    pub fn is_rx_rst(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x20) != 0 }
    }

    pub fn is_tx_msg_disc(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x40) != 0 }
    }

    pub fn is_tx_msg_sent(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x80) != 0 }
    }

    pub fn is_tx_undflw(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x100) != 0 }
    }

    pub fn is_rx_ordset(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x200) != 0 }
    }

    pub fn is_rx_idr(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x400) != 0 }
    }

    pub fn is_txis(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x800) != 0 }
    }

    pub fn is_txfne(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x1000) != 0 }
    }

    pub fn is_cc1_vstate(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 16) & 0x03) as u8 }
    }

    pub fn is_cc2_vstate(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 18) & 0x03) as u8 }
    }

    pub fn enable_interrupt(&self, source: u8) {
        unsafe {
            let imr = read_volatile(&self.regs().imr);
            write_volatile(&mut self.regs().imr, imr | (1 << source));
        }
    }

    pub fn disable_interrupt(&self, source: u8) {
        unsafe {
            let imr = read_volatile(&self.regs().imr);
            write_volatile(&mut self.regs().imr, imr & !(1 << source));
        }
    }

    pub fn clear_interrupt(&self, source: u8) {
        unsafe { write_volatile(&mut self.regs().icr, 1 << source) };
    }

    pub fn detect_cc_status(&self) -> (bool, bool) {
        let cc1 = self.is_cc1_vstate();
        let cc2 = self.is_cc2_vstate();
        (cc1 != 0, cc2 != 0)
    }

    pub fn is_cable_connected(&self) -> bool {
        let (cc1, cc2) = self.detect_cc_status();
        cc1 || cc2
    }

    pub fn get_cc_orientation(&self) -> Option<CcSel> {
        let (cc1, cc2) = self.detect_cc_status();
        if cc1 && !cc2 {
            Some(CcSel::Cc1)
        } else if !cc1 && cc2 {
            Some(CcSel::Cc2)
        } else {
            None
        }
    }

    pub fn request_power(&self, contract: &PowerContract) {
        let header = ((contract.max_voltage_mv / 50) as u16) << 10
            | ((contract.max_current_ma / 10) as u16) << 0;
        self.set_tx_message(header, &[]);
    }

    pub fn send_source_capabilities(&self, contracts: &[PowerContract]) {
        let mut data = [0u8; 28];
        let num_sources = contracts.len().min(7);
        
        for (i, contract) in contracts.iter().enumerate().take(num_sources) {
            let pdo = ((contract.max_voltage_mv / 50) as u32) << 10
                | ((contract.max_current_ma / 10) as u32) << 0;
            let offset = i * 4;
            data[offset] = (pdo & 0xFF) as u8;
            data[offset + 1] = ((pdo >> 8) & 0xFF) as u8;
            data[offset + 2] = ((pdo >> 16) & 0xFF) as u8;
            data[offset + 3] = ((pdo >> 24) & 0xFF) as u8;
        }
        
        let header = (num_sources as u16) << 12 | 0x01;
        self.set_tx_message(header, &data[..num_sources * 4]);
    }

    pub fn handle_power_delivery(&self) {
        if self.is_rx_data_ready() {
            let header = self.get_rx_header();
            let msg_type = header & 0x1F;
            
            match msg_type {
                0x01 => {}
                0x02 => {}
                0x03 => {}
                0x04 => {}
                0x05 => {}
                0x06 => {}
                0x07 => {}
                _ => {}
            }
            
            self.clear_interrupt(0);
        }
    }
}
