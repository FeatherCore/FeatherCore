#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub const PSSI_BASE: usize = 0x4004_0400;

#[repr(C)]
pub struct PssiRegs {
    pub cr: u32,
    pub sr: u32,
    pub risr: u32,
    pub ier: u32,
    pub misr: u32,
    pub icr: u32,
    pub dr: u32,
}

pub struct Pssi;

#[derive(Clone, Copy)]
pub enum DataWidth {
    Bits8 = 0,
    Bits16 = 1,
}

#[derive(Clone, Copy)]
pub enum BusWidth {
    Bits8 = 0,
    Bits16 = 1,
}

#[derive(Clone, Copy)]
pub enum ControlSignal {
    Both = 0,
    PclkOnly = 1,
    PdenOnly = 2,
}

#[derive(Clone, Copy)]
pub enum ReadyPolarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}

pub struct Config {
    pub data_width: DataWidth,
    pub bus_width: BusWidth,
    pub control_signal: ControlSignal,
    pub ready_polarity: ReadyPolarity,
    pub outen_enable: bool,
    pub dma_enable: bool,
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

pub struct PssiTransfer {
    pub data_ptr: *const u8,
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
