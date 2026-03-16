#![no_std]

use core::ptr::{read_volatile, write_volatile};

pub const HRTIM_BASE: usize = 0x4003_6C00;

pub const HRTIM_MASTER_BASE: usize = HRTIM_BASE + 0x000;
pub const HRTIM_TIMA_BASE: usize = HRTIM_BASE + 0x080;
pub const HRTIM_TIMB_BASE: usize = HRTIM_BASE + 0x100;
pub const HRTIM_TIMC_BASE: usize = HRTIM_BASE + 0x180;
pub const HRTIM_TIMD_BASE: usize = HRTIM_BASE + 0x200;
pub const HRTIM_TIME_BASE: usize = HRTIM_BASE + 0x280;
pub const HRTIM_COMMON_BASE: usize = HRTIM_BASE + 0x380;

#[repr(C)]
pub struct HrtimTimerRegs {
    pub cr: u32,
    pub isr: u32,
    pub icr: u32,
    pub dier: u32,
    pub cnt: u32,
    pub per: u32,
    pub rep: u32,
    pub cmp1: u32,
    pub cmp1c: u32,
    pub cmp2: u32,
    pub cmp3: u32,
    pub cmp4: u32,
    pub cpt1: u32,
    pub cpt2: u32,
    pub dtr: u32,
    pub set1r: u32,
    pub rst1r: u32,
    pub set2r: u32,
    pub rst2r: u32,
    pub eefr1: u32,
    pub eefr2: u32,
    pub rstr: u32,
    pub chpr: u32,
    pub cpt1cr: u32,
    pub cpt2cr: u32,
    pub outr: u32,
    pub fltr: u32,
}

#[repr(C)]
pub struct HrtimMasterRegs {
    pub cr: u32,
    pub isr: u32,
    pub icr: u32,
    pub dier: u32,
    pub cnt: u32,
    pub per: u32,
    pub rep: u32,
    pub cmp1: u32,
    pub cmp2: u32,
    pub cmp3: u32,
    pub cmp4: u32,
}

#[repr(C)]
pub struct HrtimCommonRegs {
    pub cr1: u32,
    pub cr2: u32,
    pub isr: u32,
    pub icr: u32,
    pub ier: u32,
    pub oenr: u32,
    pub odisr: u32,
    pub odsr: u32,
    pub bmcr: u32,
    pub bmtrgr: u32,
    pub bmcmpcr: u32,
    pub bmper: u32,
    pub eecr1: u32,
    pub eecr2: u32,
    pub eecr3: u32,
    pub adc1r: u32,
    pub adc2r: u32,
    pub adc3r: u32,
    pub adc4r: u32,
    pub dllcr: u32,
    pub fltinr1: u32,
    pub fltinr2: u32,
    pub bdmupr: u32,
    pub bdtupr: u32,
    pub bdcmpcr: u32,
    pub bdmadr: u32,
}

#[derive(Clone, Copy)]
pub enum Timer {
    Master = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
}

#[derive(Clone, Copy)]
pub enum Prescaler {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
}

#[derive(Clone, Copy)]
pub enum Event {
    Src1 = 0,
    Src2 = 1,
    Src3 = 2,
    Src4 = 3,
}

pub struct Hrtim;

pub struct TimerConfig {
    pub prescaler: Prescaler,
    pub period: u16,
    pub repetition: u8,
    pub continuous: bool,
}

impl Default for TimerConfig {
    fn default() -> Self {
        TimerConfig {
            prescaler: Prescaler::Div1,
            period: 0xFFFF,
            repetition: 0,
            continuous: true,
        }
    }
}

pub struct PwmConfig {
    pub cmp1: u16,
    pub cmp2: u16,
    pub cmp3: u16,
    pub cmp4: u16,
    pub output1_polarity: bool,
    pub output2_polarity: bool,
    pub output1_idle_state: bool,
    pub output2_idle_state: bool,
    pub fault_enabled: bool,
    pub deadtime_rising: u16,
    pub deadtime_falling: u16,
}

impl Default for PwmConfig {
    fn default() -> Self {
        PwmConfig {
            cmp1: 0,
            cmp2: 0,
            cmp3: 0,
            cmp4: 0,
            output1_polarity: false,
            output2_polarity: false,
            output1_idle_state: false,
            output2_idle_state: false,
            fault_enabled: true,
            deadtime_rising: 0,
            deadtime_falling: 0,
        }
    }
}

impl Hrtim {
    pub fn new() -> Self {
        Hrtim
    }

    fn master_regs(&self) -> &mut HrtimMasterRegs {
        unsafe { &mut *(HRTIM_MASTER_BASE as *mut HrtimMasterRegs) }
    }

    fn timer_regs(&self, timer: Timer) -> &mut HrtimTimerRegs {
        let base = match timer {
            Timer::A => HRTIM_TIMA_BASE,
            Timer::B => HRTIM_TIMB_BASE,
            Timer::C => HRTIM_TIMC_BASE,
            Timer::D => HRTIM_TIMD_BASE,
            Timer::E => HRTIM_TIME_BASE,
            _ => HRTIM_TIMA_BASE,
        };
        unsafe { &mut *(base as *mut HrtimTimerRegs) }
    }

    fn common_regs(&self) -> &mut HrtimCommonRegs {
        unsafe { &mut *(HRTIM_COMMON_BASE as *mut HrtimCommonRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr |= 1 << 29;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr &= !(1 << 29);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2rstr = rcc_base.add(0x94 / 4);
            *ahb2rstr |= 1 << 29;
            *ahb2rstr &= !(1 << 29);
        }
    }

    pub fn calibrate_dll(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().dllcr, 0x0000_0001);
            while (read_volatile(&self.common_regs().isr) & 0x01) == 0 {}
            write_volatile(&mut self.common_regs().icr, 0x01);
        }
    }

    pub fn configure_master(&self, config: &TimerConfig) {
        unsafe {
            let cr = (config.prescaler as u32) << 3
                | (if config.continuous { 0 } else { 1 }) << 2;
            write_volatile(&mut self.master_regs().cr, cr);
            write_volatile(&mut self.master_regs().per, config.period as u32);
            write_volatile(&mut self.master_regs().rep, config.repetition as u32);
        }
    }

    pub fn configure_timer(&self, timer: Timer, config: &TimerConfig) {
        if timer == Timer::Master {
            self.configure_master(config);
            return;
        }
        unsafe {
            let cr = (config.prescaler as u32) << 3
                | (if config.continuous { 0 } else { 1 }) << 2;
            write_volatile(&mut self.timer_regs(timer).cr, cr);
            write_volatile(&mut self.timer_regs(timer).per, config.period as u32);
            write_volatile(&mut self.timer_regs(timer).rep, config.repetition as u32);
        }
    }

    pub fn configure_pwm(&self, timer: Timer, config: &PwmConfig) {
        if timer == Timer::Master {
            return;
        }
        unsafe {
            write_volatile(&mut self.timer_regs(timer).cmp1, config.cmp1 as u32);
            write_volatile(&mut self.timer_regs(timer).cmp2, config.cmp2 as u32);
            write_volatile(&mut self.timer_regs(timer).cmp3, config.cmp3 as u32);
            write_volatile(&mut self.timer_regs(timer).cmp4, config.cmp4 as u32);

            let outr = (config.output1_polarity as u32) << 1
                | (config.output2_polarity as u32) << 3
                | (config.output1_idle_state as u32) << 8
                | (config.output2_idle_state as u32) << 9;
            write_volatile(&mut self.timer_regs(timer).outr, outr);

            let dtr = (config.deadtime_rising as u32) << 16
                | (config.deadtime_falling as u32);
            write_volatile(&mut self.timer_regs(timer).dtr, dtr);
        }
    }

    pub fn start_timer(&self, timer: Timer) {
        let bit = match timer {
            Timer::Master => 0,
            Timer::A => 1,
            Timer::B => 2,
            Timer::C => 3,
            Timer::D => 4,
            Timer::E => 5,
        };
        unsafe {
            let cr1 = read_volatile(&self.common_regs().cr1);
            write_volatile(&mut self.common_regs().cr1, cr1 | (1 << bit));
        }
    }

    pub fn stop_timer(&self, timer: Timer) {
        let bit = match timer {
            Timer::Master => 0,
            Timer::A => 1,
            Timer::B => 2,
            Timer::C => 3,
            Timer::D => 4,
            Timer::E => 5,
        };
        unsafe {
            let cr1 = read_volatile(&self.common_regs().cr1);
            write_volatile(&mut self.common_regs().cr1, cr1 & !(1 << bit));
        }
    }

    pub fn enable_outputs(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().oenr, 0x3F);
        }
    }

    pub fn disable_outputs(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().odisr, 0x3F);
        }
    }

    pub fn are_outputs_enabled(&self) -> bool {
        unsafe { (read_volatile(&self.common_regs().odsr) & 0x3F) == 0x3F }
    }

    pub fn set_compare(&self, timer: Timer, cmp: u8, value: u16) {
        if timer == Timer::Master {
            match cmp {
                1 => unsafe { write_volatile(&mut self.master_regs().cmp1, value as u32) },
                2 => unsafe { write_volatile(&mut self.master_regs().cmp2, value as u32) },
                3 => unsafe { write_volatile(&mut self.master_regs().cmp3, value as u32) },
                4 => unsafe { write_volatile(&mut self.master_regs().cmp4, value as u32) },
                _ => {}
            }
        } else {
            match cmp {
                1 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp1, value as u32) },
                2 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp2, value as u32) },
                3 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp3, value as u32) },
                4 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp4, value as u32) },
                _ => {}
            }
        }
    }

    pub fn get_counter(&self, timer: Timer) -> u16 {
        if timer == Timer::Master {
            unsafe { read_volatile(&self.master_regs().cnt) as u16 }
        } else {
            unsafe { read_volatile(&self.timer_regs(timer).cnt) as u16 }
        }
    }

    pub fn set_period(&self, timer: Timer, period: u16) {
        if timer == Timer::Master {
            unsafe { write_volatile(&mut self.master_regs().per, period as u32) }
        } else {
            unsafe { write_volatile(&mut self.timer_regs(timer).per, period as u32) }
        }
    }

    pub fn enable_interrupt(&self, timer: Timer, source: u8) {
        let dier_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe {
                let dier = read_volatile(&self.master_regs().dier);
                write_volatile(&mut self.master_regs().dier, dier | dier_bit);
            }
        } else {
            unsafe {
                let dier = read_volatile(&self.timer_regs(timer).dier);
                write_volatile(&mut self.timer_regs(timer).dier, dier | dier_bit);
            }
        }
    }

    pub fn disable_interrupt(&self, timer: Timer, source: u8) {
        let dier_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe {
                let dier = read_volatile(&self.master_regs().dier);
                write_volatile(&mut self.master_regs().dier, dier & !dier_bit);
            }
        } else {
            unsafe {
                let dier = read_volatile(&self.timer_regs(timer).dier);
                write_volatile(&mut self.timer_regs(timer).dier, dier & !dier_bit);
            }
        }
    }

    pub fn clear_interrupt(&self, timer: Timer, source: u8) {
        let icr_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe { write_volatile(&mut self.master_regs().icr, icr_bit) }
        } else {
            unsafe { write_volatile(&mut self.timer_regs(timer).icr, icr_bit) }
        }
    }

    pub fn is_interrupt_active(&self, timer: Timer, source: u8) -> bool {
        let isr_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe { (read_volatile(&self.master_regs().isr) & isr_bit) != 0 }
        } else {
            unsafe { (read_volatile(&self.timer_regs(timer).isr) & isr_bit) != 0 }
        }
    }

    pub fn configure_adc_trigger(&self, adc_num: u8, event: u8, timer: Timer, compare: u8) {
        let timer_bits = match timer {
            Timer::Master => 0,
            Timer::A => 1,
            Timer::B => 2,
            Timer::C => 3,
            Timer::D => 4,
            Timer::E => 5,
        };
        let value = (timer_bits << 4) | (compare as u32);

        unsafe {
            match adc_num {
                1 => {
                    let adc1r = read_volatile(&self.common_regs().adc1r);
                    write_volatile(&mut self.common_regs().adc1r, adc1r | (1 << event));
                }
                2 => {
                    let adc2r = read_volatile(&self.common_regs().adc2r);
                    write_volatile(&mut self.common_regs().adc2r, adc2r | (1 << event));
                }
                3 => {
                    let adc3r = read_volatile(&self.common_regs().adc3r);
                    write_volatile(&mut self.common_regs().adc3r, adc3r | (1 << event));
                }
                4 => {
                    let adc4r = read_volatile(&self.common_regs().adc4r);
                    write_volatile(&mut self.common_regs().adc4r, adc4r | (1 << event));
                }
                _ => {}
            }
        }
    }

    pub fn enable_fault(&self, fault: u8) {
        unsafe {
            let fltinr1 = read_volatile(&self.common_regs().fltinr1);
            write_volatile(&mut self.common_regs().fltinr1, fltinr1 | (1 << (fault * 8)));
        }
    }

    pub fn disable_fault(&self, fault: u8) {
        unsafe {
            let fltinr1 = read_volatile(&self.common_regs().fltinr1);
            write_volatile(&mut self.common_regs().fltinr1, fltinr1 & !(1 << (fault * 8)));
        }
    }

    pub fn enter_burst_mode(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().bmtrgr, 0x01);
        }
    }

    pub fn exit_burst_mode(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().bmtrgr, 0x02);
        }
    }

    pub fn configure_burst_mode(&self, prescaler: u8, period: u16) {
        unsafe {
            let bmcr = (prescaler as u32) << 16 | (period as u32);
            write_volatile(&mut self.common_regs().bmcr, bmcr);
        }
    }

    pub fn set_duty_cycle(&self, timer: Timer, duty_percent: f32) {
        if timer == Timer::Master {
            return;
        }
        let period = unsafe { read_volatile(&self.timer_regs(timer).per) as u16 };
        let compare = (period as f32 * duty_percent / 100.0) as u16;
        unsafe { write_volatile(&mut self.timer_regs(timer).cmp1, compare as u32) };
    }

    pub fn update_frequency(&self, timer: Timer, frequency_hz: u32, sysclk_mhz: u32) {
        let prescaler = unsafe { (read_volatile(&self.timer_regs(timer).cr) >> 3) & 0x03 };
        let prescale_factor = match prescaler {
            0 => 1,
            1 => 2,
            2 => 4,
            _ => 1,
        };
        let period = (sysclk_mhz * 1_000_000) / (frequency_hz * prescale_factor as u32) - 1;
        self.set_period(timer, period as u16);
    }
}
