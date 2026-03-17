//! EXTI - External Interrupt/Event Controller
//! 外部中断/事件控制器
//!
//! # Overview / 概述
//! STM32U5 Extended Interrupt and Event Controller (EXTI) manages external interrupts
//! and events with up to 22 configurable interrupt/event lines.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 23: Extended interrupt and event controller (EXTI)
//! 
//! ## Interrupt Lines / 中断线
//! - Up to 22 external interrupt/event lines
//! 
//! ## Trigger Modes / 触发方式
//! - Rising edge trigger
//! - Falling edge trigger
//! - Both edges trigger
//! 
//! ## Advanced Features / 高级特性
//! - Software trigger support
//! - Independent configuration per line
//! - Masking capability
//! 
//! # Reference / 参考
//! - RM0456 Chapter 23: Extended interrupt and event controller (EXTI)
//! - RM0456 Section 23.1: EXTI introduction
//! - RM0456 Section 23.2: EXTI main features
//! - RM0456 Section 23.3: EXTI functional description
//! - RM0456 Section 23.4: EXTI registers

/// EXTI base address / EXTI 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const EXTI_BASE: usize = 0x4002_1800;

/// EXTI register offsets
//! Reference: RM0456 Section 23.4: EXTI register map
pub mod reg {
    /// Rising trigger selection register
    //! Reference: RM0456 Section 23.4.1: Rising trigger selection register (EXTI_RTSR1)
    pub const RTSR1: usize = 0x00;
    /// Falling trigger selection register
    //! Reference: RM0456 Section 23.4.2: Falling trigger selection register (EXTI_FTSR1)
    pub const FTSR1: usize = 0x04;
    /// Software interrupt event register
    //! Reference: RM0456 Section 23.4.3: Software interrupt event register (EXTI_SWIER1)
    pub const SWIER1: usize = 0x08;
    /// Rising edge pending register
    //! Reference: RM0456 Section 23.4.4: Rising edge pending register (EXTI_RPR1)
    pub const RPR1: usize = 0x0C;
    /// Falling edge pending register
    //! Reference: RM0456 Section 12.4.5: Falling edge pending register (EXTI_FPR1)
    pub const FPR1: usize = 0x10;
    /// Security rising edge pending register
    //! Reference: RM0456 Section 12.4.6: Security rising edge pending register (EXTI_SR1PR)
    pub const SR1PR: usize = 0x14;
    /// External interrupt configuration register 1
    //! Reference: RM0456 Section 12.4.7: External interrupt configuration register 1 (EXTI_EXTICR1)
    pub const EXTICR1: usize = 0x60;
    /// External interrupt configuration register 2
    //! Reference: RM0456 Section 12.4.7: External interrupt configuration register 2 (EXTI_EXTICR2)
    pub const EXTICR2: usize = 0x64;
    /// External interrupt configuration register 3
    //! Reference: RM0456 Section 12.4.7: External interrupt configuration register 3 (EXTI_EXTICR3)
    pub const EXTICR3: usize = 0x68;
    /// External interrupt configuration register 4
    //! Reference: RM0456 Section 12.4.7: External interrupt configuration register 4 (EXTI_EXTICR4)
    pub const EXTICR4: usize = 0x6C;
    /// Interrupt mask register 1
    //! Reference: RM0456 Section 12.4.8: Interrupt mask register (EXTI_IMR1)
    pub const IMR1: usize = 0x80;
    /// Event mask register 1
    //! Reference: RM0456 Section 12.4.9: Event mask register (EXTI_EMR1)
    pub const EMR1: usize = 0x84;
    /// Interrupt mask register 2
    pub const IMR2: usize = 0x90;
    /// Event mask register 2
    pub const EMR2: usize = 0x94;
    /// Interrupt mask register 3
    pub const IMR3: usize = 0xA0;
    /// Event mask register 3
    pub const EMR3: usize = 0xA4;
}

/// Trigger mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriggerMode {
    /// Rising edge trigger / 上升沿触发
    Rising = 0b01,
    /// Falling edge trigger / 下降沿触发
    Falling = 0b10,
    /// Both rising and falling edge trigger / 双沿触发
    Both = 0b11,
}

/// EXTI line
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Line {
    /// EXTI line 0
    Line0 = 0,
    /// EXTI line 1
    Line1 = 1,
    /// EXTI line 2
    Line2 = 2,
    /// EXTI line 3
    Line3 = 3,
    /// EXTI line 4
    Line4 = 4,
    /// EXTI line 5
    Line5 = 5,
    /// EXTI line 6
    Line6 = 6,
    /// EXTI line 7
    Line7 = 7,
    /// EXTI line 8
    Line8 = 8,
    /// EXTI line 9
    Line9 = 9,
    /// EXTI line 10
    Line10 = 10,
    /// EXTI line 11
    Line11 = 11,
    /// EXTI line 12
    Line12 = 12,
    /// EXTI line 13
    Line13 = 13,
    /// EXTI line 14
    Line14 = 14,
    /// EXTI line 15
    Line15 = 15,
}

/// Port selection for EXTI
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Port {
    /// Port A
    PA = 0,
    /// Port B
    PB = 1,
    /// Port C
    PC = 2,
    /// Port D
    PD = 3,
    /// Port E
    PE = 4,
    /// Port F
    PF = 5,
    /// Port G
    PG = 6,
    /// Port H
    PH = 7,
}

/// EXTI instance
pub struct Exti;

impl Exti {
    /// Create EXTI instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable interrupt on line
    pub fn enable_interrupt(&self, line: Line) {
        unsafe {
            let imr = (EXTI_BASE + reg::IMR1) as *mut u32;
            let mut val = core::ptr::read_volatile(imr);
            val |= 1 << (line as u32);
            core::ptr::write_volatile(imr, val);
        }
    }

    /// Disable interrupt on line
    pub fn disable_interrupt(&self, line: Line) {
        unsafe {
            let imr = (EXTI_BASE + reg::IMR1) as *mut u32;
            let mut val = core::ptr::read_volatile(imr);
            val &= !(1 << (line as u32));
            core::ptr::write_volatile(imr, val);
        }
    }

    /// Enable event on line
    pub fn enable_event(&self, line: Line) {
        unsafe {
            let emr = (EXTI_BASE + reg::EMR1) as *mut u32;
            let mut val = core::ptr::read_volatile(emr);
            val |= 1 << (line as u32);
            core::ptr::write_volatile(emr, val);
        }
    }

    /// Disable event on line
    pub fn disable_event(&self, line: Line) {
        unsafe {
            let emr = (EXTI_BASE + reg::EMR1) as *mut u32;
            let mut val = core::ptr::read_volatile(emr);
            val &= !(1 << (line as u32));
            core::ptr::write_volatile(emr, val);
        }
    }

    /// Set trigger mode
    pub fn set_trigger(&self, line: Line, mode: TriggerMode) {
        unsafe {
            match mode {
                TriggerMode::Rising | TriggerMode::Both => {
                    let rtsr = (EXTI_BASE + reg::RTSR1) as *mut u32;
                    let mut val = core::ptr::read_volatile(rtsr);
                    val |= 1 << (line as u32);
                    core::ptr::write_volatile(rtsr, val);
                }
                _ => {}
            }

            match mode {
                TriggerMode::Falling | TriggerMode::Both => {
                    let ftsr = (EXTI_BASE + reg::FTSR1) as *mut u32;
                    let mut val = core::ptr::read_volatile(ftsr);
                    val |= 1 << (line as u32);
                    core::ptr::write_volatile(ftsr, val);
                }
                _ => {}
            }
        }
    }

    /// Configure GPIO source for line
    pub fn configure_gpio(&self, line: Line, port: Port) {
        let line_num = line as usize;
        let exticr_offset = (line_num / 4) * 4;
        let shift = (line_num % 4) * 4;

        unsafe {
            let exticr = (EXTI_BASE + reg::EXTICR1 + exticr_offset) as *mut u32;
            let mut val = core::ptr::read_volatile(exticr);
            val &= !(0xF << shift);
            val |= (port as u32) << shift;
            core::ptr::write_volatile(exticr, val);
        }
    }

    /// Generate software interrupt
    pub fn software_interrupt(&self, line: Line) {
        unsafe {
            let swier = (EXTI_BASE + reg::SWIER1) as *mut u32;
            let mut val = core::ptr::read_volatile(swier);
            val |= 1 << (line as u32);
            core::ptr::write_volatile(swier, val);
        }
    }

    /// Get pending interrupt status
    pub fn get_pending(&self, line: Line) -> bool {
        unsafe {
            let rpr = (EXTI_BASE + reg::RPR1) as *mut u32;
            let val = core::ptr::read_volatile(rpr);
            (val & (1 << (line as u32))) != 0
        }
    }

    /// Clear pending interrupt
    pub fn clear_pending(&self, line: Line) {
        unsafe {
            let rpr = (EXTI_BASE + reg::RPR1) as *mut u32;
            core::ptr::write_volatile(rpr, 1 << (line as u32));
        }
    }
}

/// Initialize EXTI
pub fn init() {
    let exti = Exti::new();
}
