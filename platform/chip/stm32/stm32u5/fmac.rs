#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub const FMAC_BASE: usize = 0x4004_1400;

#[repr(C)]
pub struct FmacRegs {
    pub x1y1: u32,
    pub x2y2: u32,
    pub x3y3: u32,
    pub x4y4: u32,
    pub x5y5: u32,
    pub x6y6: u32,
    pub x7y7: u32,
    pub x8y8: u32,
    pub x9y9: u32,
    pub x10y10: u32,
    pub x11y11: u32,
    pub x12y12: u32,
    pub x13y13: u32,
    pub x14y14: u32,
    pub x15y15: u32,
    pub x16y16: u32,
    pub cr: u32,
    pub sr: u32,
    pub wdata: u32,
    pub rdata: u32,
}

pub struct Fmac;

#[derive(Clone, Copy)]
pub enum Function {
    ConvFir = 0,
    ConvIirDirect1 = 1,
    ConvIirDirect2 = 2,
    ConvIirDirect1Fast = 3,
}

#[derive(Clone, Copy)]
pub enum P {
    P1 = 0,
    P2 = 1,
    P4 = 2,
    P8 = 3,
}

#[derive(Clone, Copy)]
pub enum Q {
    Q1 = 0,
    Q2 = 1,
    Q4 = 2,
    Q8 = 3,
}

#[derive(Clone, Copy)]
pub enum R {
    R1 = 0,
    R2 = 1,
    R4 = 2,
    R8 = 3,
}

pub struct Config {
    pub func: Function,
    pub p: P,
    pub q: Q,
    pub r: R,
    pub x1_buf_base: u8,
    pub x1_buf_size: u8,
    pub y_buf_base: u8,
    pub y_buf_size: u8,
    pub x2_buf_base: u8,
    pub x2_buf_size: u8,
    pub clip: bool,
    pub sat: bool,
    pub din_dma: bool,
    pub dout_dma: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            func: Function::ConvFir,
            p: P::P1,
            q: Q::Q1,
            r: R::R1,
            x1_buf_base: 0,
            x1_buf_size: 16,
            y_buf_base: 16,
            y_buf_size: 16,
            x2_buf_base: 32,
            x2_buf_size: 16,
            clip: false,
            sat: false,
            din_dma: false,
            dout_dma: false,
        }
    }
}

impl Fmac {
    pub fn new() -> Self {
        Fmac
    }

    fn regs(&self) -> &mut FmacRegs {
        unsafe { &mut *(FMAC_BASE as *mut FmacRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr |= 1 << 16;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr &= !(1 << 16);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1rstr = rcc_base.add(0x90 / 4);
            *ahb1rstr |= 1 << 16;
            *ahb1rstr &= !(1 << 16);
        }
    }

    pub fn configure(&self, config: &Config) {
        let cr = (config.func as u32) << 0
            | (config.p as u32) << 4
            | (config.q as u32) << 6
            | (config.r as u32) << 8
            | (config.x1_buf_base as u32) << 16
            | (config.x1_buf_size as u32) << 24
            | (config.clip as u32) << 12
            | (config.sat as u32) << 13
            | (config.din_dma as u32) << 14
            | (config.dout_dma as u32) << 15;
        unsafe { write_volatile(&mut self.regs().cr, cr) };

        let x2y2 = (config.x2_buf_base as u32) << 16 | (config.x2_buf_size as u32) << 24;
        unsafe { write_volatile(&mut self.regs().x2y2, x2y2) };

        let x3y3 = (config.y_buf_base as u32) << 16 | (config.y_buf_size as u32) << 24;
        unsafe { write_volatile(&mut self.regs().x3y3, x3y3) };
    }

    pub fn enable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 31));
        }
    }

    pub fn disable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 31));
        }
    }

    pub fn is_ready(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x01) != 0 }
    }

    pub fn write_data(&self, data: i16) {
        while !self.is_ready() {}
        unsafe { write_volatile(&mut self.regs().wdata, data as u32) };
    }

    pub fn read_data(&self) -> i16 {
        while !self.is_ready() {}
        unsafe { read_volatile(&self.regs().rdata) as i16 }
    }

    pub fn write_coeff(&self, index: usize, coeff: i16) {
        let addr = FMAC_BASE + 0x100 + index * 4;
        unsafe { write_volatile(addr as *mut u32, coeff as u32) };
    }

    pub fn read_coeff(&self, index: usize) -> i16 {
        let addr = FMAC_BASE + 0x100 + index * 4;
        unsafe { read_volatile(addr as *const u32) as i16 }
    }

    pub fn clear_x1_buffer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 30));
            while (read_volatile(&self.regs().cr) & (1 << 30)) != 0 {}
        }
    }

    pub fn clear_x2_buffer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 29));
            while (read_volatile(&self.regs().cr) & (1 << 29)) != 0 {}
        }
    }

    pub fn clear_y_buffer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 28));
            while (read_volatile(&self.regs().cr) & (1 << 28)) != 0 {}
        }
    }

    pub fn reset_filter(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 27));
            while (read_volatile(&self.regs().cr) & (1 << 27)) != 0 {}
        }
    }

    pub fn get_x1_full_wm(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 4) & 0x03) as u8 }
    }

    pub fn get_y_empty_wm(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 6) & 0x03) as u8 }
    }

    pub fn is_x1_full(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 8) & 0x01 != 0 }
    }

    pub fn is_x2_full(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 9) & 0x01 != 0 }
    }

    pub fn is_y_empty(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 10) & 0x01 != 0 }
    }

    pub fn is_saturation(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 11) & 0x01 != 0 }
    }

    pub fn clear_saturation_flag(&self) {
        unsafe { write_volatile(&mut self.regs().sr, 1 << 11) };
    }

    pub fn get_unread_samples(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 16) & 0xFF) as u8 }
    }

    pub fn load_coefficients(&self, coeffs: &[i16]) {
        for (i, &coeff) in coeffs.iter().enumerate() {
            self.write_coeff(i, coeff);
        }
    }

    pub fn process_sample(&self, sample: i16) -> i16 {
        self.write_data(sample);
        self.read_data()
    }

    pub fn process_buffer(&self, input: &[i16], output: &mut [i16]) {
        for (i, &sample) in input.iter().enumerate() {
            if i < output.len() {
                output[i] = self.process_sample(sample);
            }
        }
    }
}
