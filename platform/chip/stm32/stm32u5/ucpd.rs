#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub const UCPD1_BASE: usize = 0x4000_DC00;
pub const UCPD2_BASE: usize = 0x4000_E000;

#[repr(C)]
pub struct UcpdRegs {
    pub cfg1: u32,
    pub cfg2: u32,
    pub cr: u32,
    pub imr: u32,
    pub sr: u32,
    pub icr: u32,
    pub tx_ordset: u32,
    pub tx_paysz: u32,
    pub txdr: u32,
    pub rx_ordset: u32,
    pub rx_paysz: u32,
    pub rxdr: u32,
    pub rx_ordext1: u32,
    pub rx_ordext2: u32,
}

pub struct Ucpd {
    base: usize,
}

#[derive(Clone, Copy)]
pub enum Instance {
    Ucpd1 = 1,
    Ucpd2 = 2,
}

#[derive(Clone, Copy)]
pub enum CcSel {
    Cc1 = 0,
    Cc2 = 1,
}

#[derive(Clone, Copy)]
pub enum CcRp {
    Default = 0,
    Power15A = 1,
    Power30A = 2,
    PowerUsb = 3,
}

#[derive(Clone, Copy)]
pub enum PsCcdet {
    Disabled = 0,
    VsrcDef = 1,
    Vsrc1500 = 2,
    Vsrc3000 = 3,
}

#[derive(Clone, Copy)]
pub enum Hbitclkdiv {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

#[derive(Clone, Copy)]
pub enum UcpdRole {
    Sink = 0,
    Source = 1,
    Drd = 2,
}

pub struct Config {
    pub cc_sel: CcSel,
    pub cc_rp: CcRp,
    pub cc_rd: bool,
    pub ps_ccdet: PsCcdet,
    pub hbitclkdiv: Hbitclkdiv,
    pub ucpd_role: UcpdRole,
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
