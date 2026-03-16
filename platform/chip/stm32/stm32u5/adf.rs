#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub const ADF1_BASE: usize = 0x4004_2000;

#[repr(C)]
pub struct AdfRegs {
    pub gcr: u32,
    pub ckgcr: u32,
    pub sparecr: u32,
    pub spareval: u32,
    pub idcr: u32,
    pub reserved1: [u32; 3],
    pub dfltcr: u32,
    pub dfltcicr: u32,
    pub dfltmsicr: u32,
    pub dfltmdr: u32,
    pub dfltdur: u32,
    pub dfltier: u32,
    pub dfltisr: u32,
    pub dfltsifr: u32,
    pub dfltifr: u32,
    pub reserved2: u32,
    pub dfltincr: u32,
    pub reserved3: u32,
    pub dfltrsfcr: u32,
    pub reserved4: [u32; 2],
    pub dfltoutr: u32,
    pub dfltrdhr: u32,
    pub reserved5: [u32; 2],
    pub dfltoutdr: u32,
    pub reserved6: [u32; 3],
    pub dfltrgcr: u32,
    pub reserved7: [u32; 3],
    pub dfltgsr: u32,
    pub reserved8: [u32; 11],
    pub dfltoldcr: u32,
    pub dfltoldval: u32,
    pub dfltoldmin: u32,
    pub dfltoldmax: u32,
}

pub struct Adf;

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

#[derive(Clone, Copy)]
pub enum Procdly {
    Delay0 = 0,
    Delay1 = 1,
    Delay2 = 2,
    Delay3 = 3,
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
    pub procdly: Procdly,
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
            procdly: Procdly::Delay0,
        }
    }
}

pub struct ClockConfig {
    pub ckgden: bool,
    pub ckgmod: bool,
    pub ckgen: u8,
    pub ckgosr: u8,
    pub ckgfreq: u8,
}

impl Default for ClockConfig {
    fn default() -> Self {
        ClockConfig {
            ckgden: false,
            ckgmod: false,
            ckgen: 0,
            ckgosr: 0,
            ckgfreq: 0,
        }
    }
}

pub struct AudioConfig {
    pub sample_rate: u32,
    pub bit_depth: u8,
    pub channels: u8,
    pub stereo_mode: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            sample_rate: 48000,
            bit_depth: 16,
            channels: 2,
            stereo_mode: true,
        }
    }
}

impl Adf {
    pub fn new() -> Self {
        Adf
    }

    fn regs(&self) -> &mut AdfRegs {
        unsafe { &mut *(ADF1_BASE as *mut AdfRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr |= 1 << 24;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr &= !(1 << 24);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1rstr = rcc_base.add(0x90 / 4);
            *ahb1rstr |= 1 << 24;
            *ahb1rstr &= !(1 << 24);
        }
    }

    pub fn configure_clock(&self, config: &ClockConfig) {
        let ckgcr = (config.ckgden as u32) << 31
            | (config.ckgmod as u32) << 30
            | (config.ckgen as u32) << 16
            | (config.ckgosr as u32) << 8
            | (config.ckgfreq as u32) << 0;
        unsafe { write_volatile(&mut self.regs().ckgcr, ckgcr) };
    }

    pub fn configure_filter(&self, config: &FilterConfig) {
        unsafe {
            let dfltcr = (config.sinc_order as u32) << 13
                | (config.sinc_osr as u32) << 0
                | (config.procdly as u32) << 4;
            write_volatile(&mut self.regs().dfltcr, dfltcr);

            let dfltcicr = (config.hpf_cutoff as u32) << 0;
            write_volatile(&mut self.regs().dfltcicr, dfltcicr);

            let dfltmdr = (config.cko_prescaler as u32) << 4
                | (config.cko_src as u32) << 0;
            write_volatile(&mut self.regs().dfltmdr, dfltmdr);

            let dmaden_bit = if config.dmaden { 1u32 << 14 } else { 0 };
            let dma_mode_bits = (config.dma_mode as u32) << 12;
            write_volatile(&mut self.regs().dfltcr,
                read_volatile(&self.regs().dfltcr) | dmaden_bit | dma_mode_bits);

            if config.old_enable {
                let dfltoldcr = (1u32 << 31) | (config.old_threshold as u32);
                write_volatile(&mut self.regs().dfltoldcr, dfltoldcr);
            }
        }
    }

    pub fn enable(&self) {
        unsafe {
            let dfltcr = read_volatile(&self.regs().dfltcr);
            write_volatile(&mut self.regs().dfltcr, dfltcr | (1 << 31));
        }
    }

    pub fn disable(&self) {
        unsafe {
            let dfltcr = read_volatile(&self.regs().dfltcr);
            write_volatile(&mut self.regs().dfltcr, dfltcr & !(1 << 31));
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltcr) & (1 << 31)) != 0
        }
    }

    pub fn get_data(&self) -> i32 {
        unsafe { read_volatile(&self.regs().dfltoutdr) as i32 }
    }

    pub fn is_data_available(&self) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltisr) & 0x01) != 0
        }
    }

    pub fn clear_data_ready(&self) {
        unsafe {
            write_volatile(&mut self.regs().dfltifr, 0x01);
        }
    }

    pub fn enable_interrupt(&self, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.regs().dfltier);
            write_volatile(&mut self.regs().dfltier, dfltier | (1 << source));
        }
    }

    pub fn disable_interrupt(&self, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.regs().dfltier);
            write_volatile(&mut self.regs().dfltier, dfltier & !(1 << source));
        }
    }

    pub fn is_interrupt_active(&self, source: u8) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltisr) & (1 << source)) != 0
        }
    }

    pub fn clear_interrupt(&self, source: u8) {
        unsafe {
            write_volatile(&mut self.regs().dfltifr, 1 << source);
        }
    }

    pub fn get_old_value(&self) -> i16 {
        unsafe { read_volatile(&self.regs().dfltoldval) as i16 }
    }

    pub fn get_old_min(&self) -> i16 {
        unsafe { read_volatile(&self.regs().dfltoldmin) as i16 }
    }

    pub fn get_old_max(&self) -> i16 {
        unsafe { read_volatile(&self.regs().dfltoldmax) as i16 }
    }

    pub fn reset_old_counters(&self) {
        unsafe {
            let dfltoldcr = read_volatile(&self.regs().dfltoldcr);
            write_volatile(&mut self.regs().dfltoldcr, dfltoldcr | (1 << 30));
        }
    }

    pub fn configure_trigger(&self, trgsrc: Trgsrc, trgsens: bool) {
        unsafe {
            let dfltrgcr = (trgsrc as u32) << 4 | (trgsens as u32) << 1 | 1;
            write_volatile(&mut self.regs().dfltrgcr, dfltrgcr);
        }
    }

    pub fn software_trigger(&self) {
        unsafe {
            let dfltrgcr = read_volatile(&self.regs().dfltrgcr);
            write_volatile(&mut self.regs().dfltrgcr, dfltrgcr | (1 << 2));
        }
    }

    pub fn is_trigger_active(&self) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltgsr) & 0x01) != 0
        }
    }

    pub fn get_raw_counter(&self) -> u32 {
        unsafe { read_volatile(&self.regs().dfltincr) }
    }

    pub fn set_acquisition_mode(&self, continuous: bool) {
        unsafe {
            let dfltcr = read_volatile(&self.regs().dfltcr);
            if continuous {
                write_volatile(&mut self.regs().dfltcr, dfltcr | (1 << 30));
            } else {
                write_volatile(&mut self.regs().dfltcr, dfltcr & !(1 << 30));
            }
        }
    }

    pub fn set_scale_factor(&self, scale: u8) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.regs().dfltrsfcr);
            write_volatile(&mut self.regs().dfltrsfcr,
                (dfltrsfcr & !0xFF) | (scale as u32));
        }
    }

    pub fn enable_scale_offset(&self, enable: bool) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.regs().dfltrsfcr);
            if enable {
                write_volatile(&mut self.regs().dfltrsfcr, dfltrsfcr | (1 << 31));
            } else {
                write_volatile(&mut self.regs().dfltrsfcr, dfltrsfcr & !(1 << 31));
            }
        }
    }

    pub fn set_offset(&self, offset: i16) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.regs().dfltrsfcr);
            write_volatile(&mut self.regs().dfltrsfcr,
                (dfltrsfcr & !0xFFFF_0000) | ((offset as u32) << 16));
        }
    }

    pub fn get_data_right_aligned(&self) -> i32 {
        unsafe { read_volatile(&self.regs().dfltoutr) as i32 }
    }

    pub fn get_data_raw(&self) -> i32 {
        unsafe { read_volatile(&self.regs().dfltrdhr) as i32 }
    }

    pub fn configure_audio(&self, audio_config: &AudioConfig) {
        let osr = match audio_config.sample_rate {
            8000 => 768,
            16000 => 384,
            32000 => 192,
            44100 => 136,
            48000 => 128,
            96000 => 64,
            _ => 128,
        };

        let sinc_order = if audio_config.sample_rate >= 96000 {
            SincOrder::Sinc2
        } else {
            SincOrder::Sinc3
        };

        let filter_config = FilterConfig {
            sinc_order,
            sinc_osr: osr,
            hpf_cutoff: 1,
            cko_prescaler: CkoPrescaler::Div1,
            cko_src: CkoSrc::Cko0,
            dmaden: true,
            dma_mode: DmaMode::Circular,
            old_enable: false,
            old_threshold: 0,
            procdly: Procdly::Delay0,
        };

        self.configure_filter(&filter_config);
    }

    pub fn set_gain(&self, gain_db: i8) {
        let scale = if gain_db >= 0 {
            (1u8 << (gain_db as u8 / 6)).min(255)
        } else {
            0
        };
        self.set_scale_factor(scale);
    }

    pub fn calibrate_offset(&self) -> i16 {
        unsafe {
            let dfltoldcr = read_volatile(&self.regs().dfltoldcr);
            write_volatile(&mut self.regs().dfltoldcr, dfltoldcr | (1 << 30));
            while (read_volatile(&self.regs().dfltoldcr) & (1 << 30)) != 0 {}
            read_volatile(&self.regs().dfltoldval) as i16
        }
    }

    pub fn wait_for_data(&self) -> i32 {
        while !self.is_data_available() {}
        let data = self.get_data();
        self.clear_data_ready();
        data
    }

    pub fn read_samples(&self, buffer: &mut [i32]) -> usize {
        let mut count = 0;
        for sample in buffer.iter_mut() {
            if self.is_data_available() {
                *sample = self.get_data();
                self.clear_data_ready();
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
