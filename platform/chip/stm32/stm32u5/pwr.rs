//! PWR - Power Control
//! 电源控制
//!
//! # Overview / 概述
//! STM32U5 Power Control (PWR) module manages the power supply,
//! voltage scaling, and low-power modes of the chip.
//!
//! # Features / 功能特性
//! - Voltage scaling (Range 1: 160 MHz, Range 2: 110 MHz, Range 3: 55 MHz)
//! - Multiple low-power modes (Run, Sleep, Stop0/1/2/3, Standby, Shutdown)
//! - Power supply monitoring (PVD, VBAT)
//! - Backup domain control
//! - Wakeup sources management
//! - USB Type-C Power Delivery support
//!
//! # Power Modes / 电源模式
//! Reference: RM0456 Chapter 10, Section 10.7
//!
//! | Mode | Description | Wake-up |
//! |------|-------------|---------|
//! | Run | Full performance, all clocks running | - |
//! | Sleep | CPU off, peripherals running | Any interrupt |
//! | Stop0 | 160 MHz capable, all SRAM retained | Any EXTI |
//! | Stop1 | Partial retention, faster wake | Any EXTI |
//! | Stop2 | Minimal retention | Any EXTI |
//! | Stop3 | Only BKPRAM retained | Any EXTI |
//! | Standby | Only RTC/LSE/BKPRAM | WKUP pins, RTC, reset |
//! | Shutdown | Only RTC/LPUART | WKUP pins, RTC |
//!
//! # Reference / 参考
//! - RM0456 Chapter 10: Power control (PWR)
//! - RM0456 Section 10.1: PWR introduction
//! - RM0456 Section 10.2: PWR main features
//! - RM0456 Section 10.3: PWR regulation
//! - RM0456 Section 10.4: Power supply supervisor
//! - RM0456 Section 10.5: Backup domain
//! - RM0456 Section 10.6: Low-power modes
//! - RM0456 Section 10.7: Power mode transitions
//! - RM0456 Section 10.8: PWR registers

//! PWR base address (non-secure mode)
//! Reference: RM0456 Chapter 2, Table 1 or memory map
pub const PWR_BASE: usize = 0x4002_0000;

//! PWR secure base address (for TrustZone)
pub const PWR_BASE_SEC: usize = 0x5002_0000;

//! PWR Register Offsets
//! Reference: RM0456 Chapter 10, Section 10.8: PWR registers
pub mod reg {
    //! Power Control Register 1
    //! Reference: RM0456 Section 10.8.1
    pub const CR1: usize = 0x00;

    //! Power Control Register 2
    pub const CR2: usize = 0x04;

    //! Power Control Register 3
    pub const CR3: usize = 0x08;

    //! Power Voltage Scaling Register
    //! Reference: RM0456 Section 10.8.4
    pub const VOSR: usize = 0x0C;

    //! Power Supply Voltage Monitoring Control Register
    pub const SVMCR: usize = 0x10;

    //! Power Wakeup Control Register 1
    pub const WUCR1: usize = 0x14;

    //! Power Wakeup Control Register 2
    pub const WUCR2: usize = 0x18;

    //! Power Wakeup Control Register 3
    pub const WUCR3: usize = 0x1C;

    //! Power Backup Domain Control Register 1
    pub const BDCR1: usize = 0x20;

    //! Power Backup Domain Control Register 2
    pub const BDCR2: usize = 0x24;

    //! Power Disable Backup Domain Register
    pub const DBPR: usize = 0x28;

    //! Power USB Type-C and Power Delivery Register
    pub const UCPDR: usize = 0x2C;

    //! Power Security Configuration Register
    pub const SECCFGR: usize = 0x30;

    //! Power Privilege Configuration Register
    pub const PRIVCFGR: usize = 0x34;

    //! Power Status Register
    pub const SR: usize = 0x38;

    //! Power Supply Voltage Monitoring Status Register
    pub const SVMSR: usize = 0x3C;

    //! Power Backup Domain Status Register
    pub const BDSR: usize = 0x40;

    //! Power Wakeup Status Register
    pub const WUSR: usize = 0x44;

    //! Power Wakeup Status Clear Register
    pub const WUSCR: usize = 0x48;

    //! Power Application Pull-up/Pull-down Configuration Register
    pub const APCR: usize = 0x4C;

    //! Power Port A Pull Control Register
    pub const PUCR_A: usize = 0x50;
    pub const PDCR_A: usize = 0x54;
    pub const PUCR_B: usize = 0x58;
    pub const PDCR_B: usize = 0x5C;
    pub const PUCR_C: usize = 0x60;
    pub const PDCR_C: usize = 0x64;
    pub const PUCR_D: usize = 0x68;
    pub const PDCR_D: usize = 0x6C;
    pub const PUCR_E: usize = 0x70;
    pub const PDCR_E: usize = 0x74;
    pub const PUCR_F: usize = 0x78;
    pub const PDCR_F: usize = 0x7C;
    pub const PUCR_G: usize = 0x80;
    pub const PDCR_G: usize = 0x84;
    pub const PUCR_H: usize = 0x88;
    pub const PDCR_H: usize = 0x8C;
    pub const PUCR_I: usize = 0x90;
    pub const PDCR_I: usize = 0x94;
    pub const PUCR_J: usize = 0x98;
    pub const PDCR_J: usize = 0x9C;

    //! Power Control Register 4
    pub const CR4: usize = 0xA8;

    //! Power Control Register 5
    pub const CR5: usize = 0xAC;
}

//! CR1 Register Bit Definitions
//! Reference: RM0456 Section 10.8.1
pub mod cr1_bits {
    pub const LPMS: u32 = 0b111 << 0;
    pub const VBE: u32 = 1 << 8;
    pub const VBR: u32 = 1 << 9;
    pub const DBP: u32 = 1 << 8;
    pub const LPmf: u32 = 1 << 11;
    pub const APC: u32 = 1 << 15;
    pub const RUN_SS: u32 = 1 << 16;
    pub const RUN_R1: u32 = 1 << 17;
    pub const R1RSEL: u32 = 1 << 18;
}

//! CR3 Register Bit Definitions
//! Reference: RM0456 Section 10.8.3
pub mod cr3_bits {
    pub const EIWUL: u32 = 1 << 0;
    pub const REGSEL: u32 = 1 << 1;
    pub const FSTEN: u32 = 1 << 2;
    pub const UCPD_DBDIS: u32 = 1 << 6;
    pub const UCPD_STDBY: u32 = 1 << 7;
    pub const EWUP1: u32 = 1 << 8;
    pub const EWUP2: u32 = 1 << 9;
    pub const EWUP3: u32 = 1 << 10;
    pub const EWUP4: u32 = 1 << 11;
    pub const EWUP5: u32 = 1 << 12;
    pub const EWUP6: u32 = 1 << 13;
    pub const EWUP7: u32 = 1 << 14;
    pub const EWUP8: u32 = 1 << 15;
}

//! VOSR Register Bit Definitions
//! Reference: RM0456 Section 10.8.4
pub mod vosr_bits {
    pub const VOS: u32 = 0b11 << 16;
    pub const VOSRDY: u32 = 1 << 15;
    pub const VOSY: u32 = 1 << 14;
    pub const COMP1_VREFOUTEN: u32 = 1 << 31;
}

//! SVMCR Register Bit Definitions
//! Reference: RM0456 Section 10.8.5
pub mod svmcr_bits {
    pub const PVDEN: u32 = 1 << 0;
    pub const PVDLS: u32 = 0b111 << 1;
    pub const PVDRST: u32 = 1 << 4;
    pub const VREFEN: u32 = 1 << 8;
    pub const VREFRDY: u32 = 1 << 9;
    pub const TSEN: u32 = 1 << 12;
    pub const TSRDY: u32 = 1 << 13;
}

//! SR Register Bit Definitions
//! Reference: RM0456 Section 10.8.9
pub mod sr_bits {
    pub const PVDO: u32 = 1 << 0;
    pub const VOSF: u32 = 1 << 1;
    pub const REGS: u32 = 1 << 4;
    pub const FST_RDY: u32 = 1 << 5;
    pub const STOPF: u32 = 1 << 6;
    pub const SBF: u32 = 1 << 7;
    pub const WUFI: u32 = 1 << 8;
    pub const WKUPF1: u32 = 1 << 16;
    pub const WKUPF2: u32 = 1 << 17;
    pub const WKUPF3: u32 = 1 << 18;
    pub const WKUPF4: u32 = 1 << 19;
    pub const WKUPF5: u32 = 1 << 20;
    pub const WKUPF6: u32 = 1 << 21;
    pub const WKUPF7: u32 = 1 << 22;
    pub const WKUPF8: u32 = 1 << 23;
}

//! Low Power Mode Selection
//! Reference: RM0456 Section 10.7
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LowPowerMode {
    /// Run mode (normal operation)
    Run = 0b000,
    /// Sleep mode
    Sleep = 0b000,
    /// Stop 0 mode (highest performance in stop)
    Stop0 = 0b000,
    /// Stop 1 mode (partial retention)
    Stop1 = 0b001,
    /// Stop 2 mode (minimal retention)
    Stop2 = 0b010,
    /// Stop 3 mode (BKPRAM retained)
    Stop3 = 0b011,
    /// Standby mode
    Standby = 0b100,
    /// Shutdown mode
    Shutdown = 0b110,
}

//! Voltage Scaling Range
//! Reference: RM0456 Section 10.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoltageScaling {
    /// Range 1: up to 160 MHz
    Range1 = 0b00,
    /// Range 2: up to 110 MHz
    Range2 = 0b01,
    /// Range 3: up to 55 MHz
    Range3 = 0b10,
}

//! Initialize PWR peripheral
//! Reference: RM0456 Chapter 10
pub fn init() {
    unsafe {
        // Enable PWR clock
        // Reference: RM0456 Section 11.10.5: APB1ENR1
        let rcc_apb1enr = 0x4002_109C as *mut u32;
        let val = core::ptr::read_volatile(rcc_apb1enr);
        core::ptr::write_volatile(rcc_apb1enr, val | (1 << 28));

        // Enable backup domain access
        // Reference: RM0456 Section 10.8.1, bit DBP
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val |= cr1_bits::DBP;
        core::ptr::write_volatile(cr1, val);

        // Wait for backup domain to be ready
        while (core::ptr::read_volatile(cr1) & cr1_bits::DBP) == 0 {}
    }
}

//! Set voltage scaling
//! Reference: RM0456 Section 10.3.2
pub fn set_voltage_scaling(range: VoltageScaling) {
    unsafe {
        // Configure VOS in VOSR register
        // Reference: RM0456 Section 10.8.4
        let vosr = (PWR_BASE + reg::VOSR) as *mut u32;
        let mut val = core::ptr::read_volatile(vosr);
        val &= !vosr_bits::VOS;
        val |= (range as u32) << 16;
        core::ptr::write_volatile(vosr, val);

        // Wait for VOS ready
        while (core::ptr::read_volatile(vosr) & vosr_bits::VOSRDY) == 0 {}
    }
}

//! Configure voltage scaling for maximum performance (160 MHz)
//! Reference: RM0456 Section 10.3
pub fn configure_for_160mhz() {
    set_voltage_scaling(VoltageScaling::Range1);
}

//! Enter Sleep mode
//! Reference: RM0456 Section 10.7.5
//!
//! In Sleep mode, the CPU clock is stopped but all peripherals
//! continue to operate. The device can wake up by any interrupt.
pub fn enter_sleep_mode() {
    unsafe {
        // Set LPMS to Sleep
        // Reference: RM0456 Section 10.8.1
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !cr1_bits::LPMS;
        core::ptr::write_volatile(cr1, val);

        // Wait for interrupt (WFI)
        // Reference: ARM Cortex-M33 Programming Manual
        asm!("wfi");
    }
}

//! Enter Stop mode
//! Reference: RM0456 Section 10.7.6-10.7.9
//!
//! In Stop mode, all clocks in the VCORE domain are stopped.
//! The device can wake up by any EXTI line.
pub fn enter_stop_mode(mode: LowPowerMode) {
    unsafe {
        // Configure LPMS
        // Reference: RM0456 Section 10.8.1
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !cr1_bits::LPMS;
        val |= (mode as u32) << 0;
        core::ptr::write_volatile(cr1, val);

        // Wait for interrupt (WFI)
        // Reference: ARM Cortex-M33 Programming Manual
        asm!("wfi");

        // Clear STOPF flag after wakeup
        // Reference: RM0456 Section 10.8.9
        let sr = (PWR_BASE + reg::SR) as *mut u32;
        let val = core::ptr::read_volatile(sr);
        core::ptr::write_volatile(sr, val & !sr_bits::STOPF);
    }
}

//! Enter Standby mode
//! Reference: RM0456 Section 10.7.10
//!
//! In Standby mode, the VCORE domain is powered off.
//! Only the backup domain (RTC, backup registers, LSE, LSI) remains powered.
pub fn enter_standby_mode() {
    unsafe {
        // Set LPMS to Standby
        // Reference: RM0456 Section 10.8.1
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !cr1_bits::LPMS;
        val |= (LowPowerMode::Standby as u32) << 0;
        core::ptr::write_volatile(cr1, val);

        // Wait for event (WFE)
        asm!("wfe");
        asm!("wfi");
    }
}

//! Enter Shutdown mode
//! Reference: RM0456 Section 10.7.11
//!
//! Shutdown mode offers the lowest power consumption.
//! Only the backup domain remains powered (RTC and LPUART if configured).
pub fn enter_shutdown_mode() {
    unsafe {
        // Set LPMS to Shutdown
        // Reference: RM0456 Section 10.8.1
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !cr1_bits::LPMS;
        val |= (LowPowerMode::Shutdown as u32) << 0;
        core::ptr::write_volatile(cr1, val);

        // Wait for event (WFE)
        asm!("wfe");
        asm!("wfi");
    }
}

//! Enable wakeup pin
//! Reference: RM0456 Section 10.6.2
//!
//! # Arguments
//! * `pin` - Wakeup pin number (1-8)
pub fn enable_wakeup_pin(pin: u8) {
    if pin < 1 || pin > 8 {
        return;
    }

    unsafe {
        let cr3 = (PWR_BASE + reg::CR3) as *mut u32;
        let mut val = core::ptr::read_volatile(cr3);
        val |= 1 << (8 + pin - 1);
        core::ptr::write_volatile(cr3, val);
    }
}

//! Disable wakeup pin
//! Reference: RM0456 Section 10.6.2
//!
//! # Arguments
//! * `pin` - Wakeup pin number (1-8)
pub fn disable_wakeup_pin(pin: u8) {
    if pin < 1 || pin > 8 {
        return;
    }

    unsafe {
        let cr3 = (PWR_BASE + reg::CR3) as *mut u32;
        let mut val = core::ptr::read_volatile(cr3);
        val &= !(1 << (8 + pin - 1));
        core::ptr::write_volatile(cr3, val);
    }
}

//! Enable backup domain write access
//! Reference: RM0456 Section 10.5.3
pub fn enable_backup_domain_write_access() {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val |= cr1_bits::DBP;
        core::ptr::write_volatile(cr1, val);

        while (core::ptr::read_volatile(cr1) & cr1_bits::DBP) == 0 {}
    }
}

//! Disable backup domain write access
//! Reference: RM0456 Section 10.5.3
pub fn disable_backup_domain_write_access() {
    unsafe {
        let cr1 = (PWR_BASE + reg::CR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cr1);
        val &= !cr1_bits::DBP;
        core::ptr::write_volatile(cr1, val);
    }
}

//! Enable backup regulator
//! Reference: RM0456 Section 10.5.1
pub fn enable_backup_regulator() {
    unsafe {
        let bdcr1 = (PWR_BASE + reg::BDCR1) as *mut u32;
        let mut val = core::ptr::read_volatile(bdcr1);
        val |= 1 << 0;  // BREN
        core::ptr::write_volatile(bdcr1, val);
    }
}

//! Disable backup regulator
pub fn disable_backup_regulator() {
    unsafe {
        let bdcr1 = (PWR_BASE + reg::BDCR1) as *mut u32;
        let mut val = core::ptr::read_volatile(bdcr1);
        val &= !(1 << 0);
        core::ptr::write_volatile(bdcr1, val);
    }
}

//! Check if PVD output is active
//! Reference: RM0456 Section 10.8.9
pub fn is_pvd_output_active() -> bool {
    unsafe {
        let sr = (PWR_BASE + reg::SR) as *mut u32;
        (core::ptr::read_volatile(sr) & sr_bits::PVDO) != 0
    }
}

//! Check if wakeup flag is set
//! Reference: RM0456 Section 10.8.9
pub fn is_wakeup_flag_set(pin: u8) -> bool {
    if pin < 1 || pin > 8 {
        return false;
    }

    unsafe {
        let wusr = (PWR_BASE + reg::WUSR) as *mut u32;
        (core::ptr::read_volatile(wusr) & (1 << (pin - 1 + 16))) != 0
    }
}

//! Clear wakeup flag
//! Reference: RM0456 Section 10.8.11
pub fn clear_wakeup_flag(pin: u8) {
    if pin < 1 || pin > 8 {
        return;
    }

    unsafe {
        let wuscr = (PWR_BASE + reg::WUSCR) as *mut u32;
        let mut val = core::ptr::read_volatile(wuscr);
        val |= 1 << (pin - 1);
        core::ptr::write_volatile(wuscr, val);
    }
}

//! Enable PVD (Programmable Voltage Detector)
//! Reference: RM0456 Section 10.4
pub fn enable_pvd() {
    unsafe {
        let svmcr = (PWR_BASE + reg::SVMCR) as *mut u32;
        let mut val = core::ptr::read_volatile(svmcr);
        val |= svmcr_bits::PVDEN;
        core::ptr::write_volatile(svmcr, val);
    }
}

//! Disable PVD
pub fn disable_pvd() {
    unsafe {
        let svmcr = (PWR_BASE + reg::SVMCR) as *mut u32;
        let mut val = core::ptr::read_volatile(svmcr);
        val &= !svmcr_bits::PVDEN;
        core::ptr::write_volatile(svmcr, val);
    }
}

//! Set PVD level
//! Reference: RM0456 Section 10.8.5
//!
//! # Arguments
//! * `level` - PVD level (0-7), corresponding to different voltage thresholds
pub fn set_pvd_level(level: u8) {
    if level > 7 {
        return;
    }

    unsafe {
        let svmcr = (PWR_BASE + reg::SVMCR) as *mut u32;
        let mut val = core::ptr::read_volatile(svmcr);
        val &= !svmcr_bits::PVDLS;
        val |= (level & 0x7) << 1;
        core::ptr::write_volatile(svmcr, val);
    }
}

//! Enable USB Type-C Power Delivery
//! Reference: RM0456 Section 10.8.6
pub fn enable_ucpd() {
    unsafe {
        let cr3 = (PWR_BASE + reg::CR3) as *mut u32;
        let mut val = core::ptr::read_volatile(cr3);
        val &= !cr3_bits::UCPD_DBDIS;
        core::ptr::write_volatile(cr3, val);
    }
}

//! Disable USB Type-C Power Delivery
pub fn disable_ucpd() {
    unsafe {
        let cr3 = (PWR_BASE + reg::CR3) as *mut u32;
        let mut val = core::ptr::read_volatile(cr3);
        val |= cr3_bits::UCPD_DBDIS;
        core::ptr::write_volatile(cr3, val);
    }
}
