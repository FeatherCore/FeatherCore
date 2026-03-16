#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub const MDF1_BASE: usize = 0x4003_5000;

pub const MDF_FLT0_BASE: usize = MDF1_BASE + 0x000;
pub const MDF_FLT1_BASE: usize = MDF1_BASE + 0x100;
pub const MDF_FLT2_BASE: usize = MDF1_BASE + 0x200;
pub const MDF_FLT3_BASE: usize = MDF1_BASE + 0x300;
pub const MDF_FLT4_BASE: usize = MDF1_BASE + 0x400;
pub const MDF_FLT5_BASE: usize = MDF1_BASE + 0x500;

#[repr(C)]
pub struct MdfFilterRegs {
    pub dfltcr: u32,
    pub dfltcicr: u32,
    pub dfltmsicr: u32,
    pub dfltmdr: u32,
    pub dfltdur: u32,
    pub dfltier: u32,
    pub dfltisr: u32,
    pub dfltsifr: u32,
    pub dfltifr: u32,
    pub reserved1: u32,
    pub dfltincr: u32,
    pub reserved2: u32,
    pub dfltrsfcr: u32,
    pub reserved3: [u32; 2],
    pub dfltoutr: u32,
    pub dfltrdhr: u32,
    pub reserved4: [u32; 2],
    pub dfltoutdr: u32,
    pub reserved5: [u32; 3],
    pub dfltrgcr: u32,
    pub reserved6: [u32; 3],
    pub dfltgsr: u32,
    pub reserved7: [u32; 11],
    pub dfltoldcr: u32,
    pub dfltoldval: u32,
    pub dfltoldmin: u32,
    pub dfltoldmax: u32,
}

#[repr(C)]
pub struct MdfCommonRegs {
    pub gcr: u32,
    pub ckgcr: u32,
    pub sparecr: u32,
    pub spareval: u32,
    pub idcr: u32,
}

pub struct Mdf;

#[derive(Clone, Copy)]
pub enum Filter {
    F0 = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
}

#[derive(Clone, Copy)]
pub enum SincOrder {
    FastSinc = 0,
    Sinc1 = 1,
    Sinc2 = 2,
    Sinc3 = 3,
    Sinc4 = 4,
    Sinc5 = 5,
}

#[derive(Clone, Copy)]
pub enum CkoPrescaler {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
    Div16 = 4,
    Div32 = 5,
    Div64 = 6,
    Div128 = 7,
}

#[derive(Clone, Copy)]
pub enum CkoSrc {
    Cko0 = 0,
    Cko1 = 1,
    Cko2 = 2,
    Cko3 = 3,
    Cko4 = 4,
    Cko5 = 5,
}

#[derive(Clone, Copy)]
pub enum Trgsrc {
    Trg0 = 0,
    Trg1 = 1,
    Trg2 = 2,
    Trg3 = 3,
    Trg4 = 4,
    Trg5 = 5,
    Trg6 = 6,
    Trg7 = 7,
}

#[derive(Clone, Copy)]
pub enum DmaMode {
    Disabled = 0,
    OneShot = 1,
    Circular = 2,
}

pub struct FilterConfig {
    pub sinc_order: SincOrder,
    pub sinc_osr: u16,
    pub hpf_cutoff: u8,
    pub cko_prescaler: CkoPrescaler,
    pub cko_src: CkoSrc,
    pub dmaden: bool,
    pub dma_mode: DmaMode,
    pub old_enable: bool,
    pub old_threshold: u16,
}

impl Default for FilterConfig {
    fn default() -> Self {
        FilterConfig {
            sinc_order: SincOrder::Sinc3,
            sinc_osr: 64,
            hpf_cutoff: 0,
            cko_prescaler: CkoPrescaler::Div1,
            cko_src: CkoSrc::Cko0,
            dmaden: false,
            dma_mode: DmaMode::Disabled,
            old_enable: false,
            old_threshold: 0,
        }
    }
}

pub struct ClockConfig {
    pub ckgden: bool,
    pub ckgmod: bool,
    pub ckgen: u8,
    pub ckgosr: u8,
}

impl Default for ClockConfig {
    fn default() -> Self {
        ClockConfig {
            ckgden: false,
            ckgmod: false,
            ckgen: 0,
            ckgosr: 0,
        }
    }
}

impl Mdf {
    pub fn new() -> Self {
        Mdf
    }

    fn filter_regs(&self, filter: Filter) -> &mut MdfFilterRegs {
        let base = match filter {
            Filter::F0 => MDF_FLT0_BASE,
            Filter::F1 => MDF_FLT1_BASE,
            Filter::F2 => MDF_FLT2_BASE,
            Filter::F3 => MDF_FLT3_BASE,
            Filter::F4 => MDF_FLT4_BASE,
            Filter::F5 => MDF_FLT5_BASE,
        };
        unsafe { &mut *(base as *mut MdfFilterRegs) }
    }

    fn common_regs(&self) -> &mut MdfCommonRegs {
        unsafe { &mut *(MDF1_BASE as *mut MdfCommonRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr |= 1 << 25;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr &= !(1 << 25);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1rstr = rcc_base.add(0x90 / 4);
            *ahb1rstr |= 1 << 25;
            *ahb1rstr &= !(1 << 25);
        }
    }

    pub fn configure_clock(&self, config: &ClockConfig) {
        let ckgcr = (config.ckgden as u32) << 31
            | (config.ckgmod as u32) << 30
            | (config.ckgen as u32) << 16
            | (config.ckgosr as u32) << 0;
        unsafe { write_volatile(&mut self.common_regs().ckgcr, ckgcr) };
    }

    pub fn configure_filter(&self, filter: Filter, config: &FilterConfig) {
        unsafe {
            let dfltcr = (config.sinc_order as u32) << 13
                | (config.sinc_osr as u32) << 0;
            write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr);

            let dfltcicr = (config.hpf_cutoff as u32) << 0;
            write_volatile(&mut self.filter_regs(filter).dfltcicr, dfltcicr);

            let dfltmdr = (config.cko_prescaler as u32) << 4
                | (config.cko_src as u32) << 0;
            write_volatile(&mut self.filter_regs(filter).dfltmdr, dfltmdr);

            let dmaden_bit = if config.dmaden { 1u32 << 14 } else { 0 };
            let dma_mode_bits = (config.dma_mode as u32) << 12;
            write_volatile(&mut self.filter_regs(filter).dfltcr,
                read_volatile(&self.filter_regs(filter).dfltcr) | dmaden_bit | dma_mode_bits);

            if config.old_enable {
                let dfltoldcr = (1u32 << 31) | (config.old_threshold as u32);
                write_volatile(&mut self.filter_regs(filter).dfltoldcr, dfltoldcr);
            }
        }
    }

    pub fn enable_filter(&self, filter: Filter) {
        unsafe {
            let dfltcr = read_volatile(&self.filter_regs(filter).dfltcr);
            write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr | (1 << 31));
        }
    }

    pub fn disable_filter(&self, filter: Filter) {
        unsafe {
            let dfltcr = read_volatile(&self.filter_regs(filter).dfltcr);
            write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr & !(1 << 31));
        }
    }

    pub fn is_filter_enabled(&self, filter: Filter) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltcr) & (1 << 31)) != 0
        }
    }

    pub fn get_data(&self, filter: Filter) -> i32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoutdr) as i32 }
    }

    pub fn is_data_available(&self, filter: Filter) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltisr) & 0x01) != 0
        }
    }

    pub fn clear_data_ready(&self, filter: Filter) {
        unsafe {
            write_volatile(&mut self.filter_regs(filter).dfltifr, 0x01);
        }
    }

    pub fn enable_interrupt(&self, filter: Filter, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.filter_regs(filter).dfltier);
            write_volatile(&mut self.filter_regs(filter).dfltier, dfltier | (1 << source));
        }
    }

    pub fn disable_interrupt(&self, filter: Filter, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.filter_regs(filter).dfltier);
            write_volatile(&mut self.filter_regs(filter).dfltier, dfltier & !(1 << source));
        }
    }

    pub fn is_interrupt_active(&self, filter: Filter, source: u8) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltisr) & (1 << source)) != 0
        }
    }

    pub fn clear_interrupt(&self, filter: Filter, source: u8) {
        unsafe {
            write_volatile(&mut self.filter_regs(filter).dfltifr, 1 << source);
        }
    }

    pub fn get_old_value(&self, filter: Filter) -> i16 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoldval) as i16 }
    }

    pub fn get_old_min(&self, filter: Filter) -> i16 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoldmin) as i16 }
    }

    pub fn get_old_max(&self, filter: Filter) -> i16 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoldmax) as i16 }
    }

    pub fn reset_old_counters(&self, filter: Filter) {
        unsafe {
            let dfltoldcr = read_volatile(&self.filter_regs(filter).dfltoldcr);
            write_volatile(&mut self.filter_regs(filter).dfltoldcr, dfltoldcr | (1 << 30));
        }
    }

    pub fn configure_trigger(&self, filter: Filter, trgsrc: Trgsrc, trgsens: bool) {
        unsafe {
            let dfltrgcr = (trgsrc as u32) << 4 | (trgsens as u32) << 1 | 1;
            write_volatile(&mut self.filter_regs(filter).dfltrgcr, dfltrgcr);
        }
    }

    pub fn software_trigger(&self, filter: Filter) {
        unsafe {
            let dfltrgcr = read_volatile(&self.filter_regs(filter).dfltrgcr);
            write_volatile(&mut self.filter_regs(filter).dfltrgcr, dfltrgcr | (1 << 2));
        }
    }

    pub fn is_trigger_active(&self, filter: Filter) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltgsr) & 0x01) != 0
        }
    }

    pub fn get_raw_counter(&self, filter: Filter) -> u32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltincr) }
    }

    pub fn set_acquisition_mode(&self, filter: Filter, continuous: bool) {
        unsafe {
            let dfltcr = read_volatile(&self.filter_regs(filter).dfltcr);
            if continuous {
                write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr | (1 << 30));
            } else {
                write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr & !(1 << 30));
            }
        }
    }

    pub fn set_scale_factor(&self, filter: Filter, scale: u8) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.filter_regs(filter).dfltrsfcr);
            write_volatile(&mut self.filter_regs(filter).dfltrsfcr,
                (dfltrsfcr & !0xFF) | (scale as u32));
        }
    }

    pub fn enable_scale_offset(&self, filter: Filter, enable: bool) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.filter_regs(filter).dfltrsfcr);
            if enable {
                write_volatile(&mut self.filter_regs(filter).dfltrsfcr, dfltrsfcr | (1 << 31));
            } else {
                write_volatile(&mut self.filter_regs(filter).dfltrsfcr, dfltrsfcr & !(1 << 31));
            }
        }
    }

    pub fn set_offset(&self, filter: Filter, offset: i16) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.filter_regs(filter).dfltrsfcr);
            write_volatile(&mut self.filter_regs(filter).dfltrsfcr,
                (dfltrsfcr & !0xFFFF_0000) | ((offset as u32) << 16));
        }
    }

    pub fn get_data_right_aligned(&self, filter: Filter) -> i32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoutr) as i32 }
    }

    pub fn get_data_raw(&self, filter: Filter) -> i32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltrdhr) as i32 }
    }
}
