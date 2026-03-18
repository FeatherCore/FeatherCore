//! RTC - Real-Time Clock
//! 实时时钟
//!
//! # Overview / 概述
//! STM32U5 Real-Time Clock (RTC) provides calendar functions, alarms, wakeup timer,
//! and backup registers that maintain data during power loss.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 63: Real-time clock (RTC)
//! 
//! ## Calendar / 日历功能
//! - Seconds, minutes, hours (12/24-hour format)
//! - Date, month, year (0-99, 2000-2099)
//! - Weekday
//! - Sub-second precision
//! 
//! ## Alarms / 闹钟功能
//! - **Alarm A** and **Alarm B** with flexible match options
//! - Wakeup timer with configurable clock source
//! 
//! ## Other Features / 其他特性
//! - Timestamp functionality
//! - Tamper detection
//! - 32 backup registers (32-bit each)
//! - Backup domain retention
//! 
//! # Reference / 参考
//! - RM0456 Chapter 63: Real-time clock (RTC)
//! - RM0456 Section 63.1: RTC introduction
//! - RM0456 Section 63.2: RTC main features
//! - RM0456 Section 63.3: RTC functional description
//! - RM0456 Section 63.6: RTC registers

/// RTC base address (backup domain)
//! Reference: RM0456 Chapter 2, Table 1: Memory map and register boundary addresses
pub const RTC_BASE: usize = 0x4200_0000;

/// RTC register offsets
//! Reference: RM0456 Section 63.6: RTC registers
pub mod reg {
    /// RTC time register
    //! Reference: RM0456 Section 63.6.2: RTC time register (RTC_TR)
    pub const TR: usize = 0x00;
    /// RTC date register
    //! Reference: RM0456 Section 63.6.3: RTC date register (RTC_DR)
    pub const DR: usize = 0x04;
    /// RTC sub second register
    //! Reference: RM0456 Section 63.6.4: RTC subsecond register (RTC_SSR)
    pub const SSR: usize = 0x08;
    /// RTC initialization control and status register
    //! Reference: RM0456 Section 63.6.5: RTC initialization control and status register (RTC_ICSR)
    pub const ICSR: usize = 0x0C;
    /// RTC prescaler register
    //! Reference: RM0456 Section 63.6.6: RTC prescaler register (RTC_PRER)
    pub const PRER: usize = 0x10;
    /// RTC wakeup timer register
    //! Reference: RM0456 Section 63.6.7: RTC wake-up timer register (RTC_WUTR)
    pub const WUTR: usize = 0x14;
    /// RTC control register
    //! Reference: RM0456 Section 63.6.8: RTC control register (RTC_CR)
    pub const CR: usize = 0x18;
    /// RTC write protection register
    //! Reference: RM0456 Section 63.6.9: RTC write protection register (RTC_WPR)
    pub const WPR: usize = 0x24;
    /// RTC calibration register
    //! Reference: RM0456 Section 63.6.10: RTC calibration register (RTC_CALR)
    pub const CALR: usize = 0x28;
    /// RTC shift control register
    //! Reference: RM0456 Section 63.6.11: RTC shift control register (RTC_SHIFTR)
    pub const SHIFTR: usize = 0x2C;
    /// RTC time stamp time register
    //! Reference: RM0456 Section 63.6.12: RTC time stamp time register (RTC_TSTR)
    pub const TSTR: usize = 0x30;
    /// RTC time stamp date register
    pub const TSDR: usize = 0x34;
    /// RTC time stamp sub second register
    pub const TSSSR: usize = 0x38;
    /// RTC alarm A register
    pub const ALRMAR: usize = 0x40;
    /// RTC alarm A sub second register
    pub const ALRMASSR: usize = 0x44;
    /// RTC alarm B register
    pub const ALRMBR: usize = 0x48;
    /// RTC alarm B sub second register
    pub const ALRMBSSR: usize = 0x4C;
    /// RTC status register
    pub const SR: usize = 0x50;
    /// RTC masked interrupt status register
    pub const MISR: usize = 0x54;
    /// RTC status clear register
    pub const SCR: usize = 0x5C;
    /// RTC alarm A binary register
    pub const ALRABINR: usize = 0x70;
    /// RTC alarm B binary register
    pub const ALRBBINR: usize = 0x74;
}

/// Backup register base address
pub const BACKUP_REG_BASE: usize = 0x4200_0080;

/// RTC time structure
#[derive(Clone, Copy, Debug, Default)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub sub_seconds: u16,
    pub pm: bool, // true = PM, false = AM (12-hour mode)
}

/// RTC date structure
#[derive(Clone, Copy, Debug, Default)]
pub struct Date {
    pub year: u8,    // 0-99 (2000-2099)
    pub month: u8,   // 1-12
    pub day: u8,     // 1-31
    pub weekday: u8, // 1-7 (Monday=1)
}

/// RTC datetime structure
#[derive(Clone, Copy, Debug, Default)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

/// Alarm ID
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Alarm {
    AlarmA = 0,
    AlarmB = 1,
}

/// Alarm match mask
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlarmMask {
    /// Match seconds only
    Seconds = 0b0000,
    /// Match seconds and minutes
    Minutes = 0b1000,
    /// Match seconds, minutes, and hours
    Hours = 0b1100,
    /// Match seconds, minutes, hours, and day/date
    Day = 0b1110,
    /// Match all fields
    All = 0b1111,
}

/// RTC wakeup clock source
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WakeupClock {
    /// RTC/16 clock
    Div16 = 0b000,
    /// RTC/8 clock
    Div8 = 0b001,
    /// RTC/4 clock
    Div4 = 0b010,
    /// RTC/2 clock
    Div2 = 0b011,
    /// ck_spre (1 Hz)
    CkSpre = 0b100,
    /// ck_spre with additional counter
    CkSpreExtended = 0b110,
}

/// RTC initialization result
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RtcError {
    InitFailed,
    WriteProtected,
    InvalidTime,
    InvalidDate,
}

/// RTC instance
pub struct Rtc;

impl Rtc {
    /// Create RTC instance
    pub const fn new() -> Self {
        Self
    }

    /// Disable write protection
    fn disable_write_protection(&self) {
        unsafe {
            let wpr = (RTC_BASE + reg::WPR) as *mut u32;
            core::ptr::write_volatile(wpr, 0xCA);
            core::ptr::write_volatile(wpr, 0x53);
        }
    }

    /// Enable write protection
    fn enable_write_protection(&self) {
        unsafe {
            let wpr = (RTC_BASE + reg::WPR) as *mut u32;
            core::ptr::write_volatile(wpr, 0xFF);
        }
    }

    /// Enter initialization mode
    fn enter_init_mode(&self) -> Result<(), RtcError> {
        unsafe {
            let icsr = (RTC_BASE + reg::ICSR) as *mut u32;

            // Check if already in init mode
            if (core::ptr::read_volatile(icsr) & (1 << 6)) != 0 {
                return Ok(());
            }

            // Request init mode
            let mut val = core::ptr::read_volatile(icsr);
            val |= 1 << 7; // INIT
            core::ptr::write_volatile(icsr, val);

            // Wait for init mode
            let mut timeout = 100000;
            while (core::ptr::read_volatile(icsr) & (1 << 6)) == 0 {
                timeout -= 1;
                if timeout == 0 {
                    return Err(RtcError::InitFailed);
                }
            }

            Ok(())
        }
    }

    /// Exit initialization mode
    fn exit_init_mode(&self) {
        unsafe {
            let icsr = (RTC_BASE + reg::ICSR) as *mut u32;
            let mut val = core::ptr::read_volatile(icsr);
            val &= !(1 << 7); // Clear INIT
            core::ptr::write_volatile(icsr, val);
        }
    }

    /// Wait for synchronization
    fn wait_for_sync(&self) {
        unsafe {
            let icsr = (RTC_BASE + reg::ICSR) as *mut u32;

            // Clear RSF
            let mut val = core::ptr::read_volatile(icsr);
            val &= !(1 << 5);
            core::ptr::write_volatile(icsr, val);

            // Wait for RSF
            while (core::ptr::read_volatile(icsr) & (1 << 5)) == 0 {}
        }
    }

    /// Initialize RTC with LSE (32.768 kHz)
    pub fn init_lse(&self) -> Result<(), RtcError> {
        // Enable backup domain access
        crate::pwr::enable_backup_access();

        // Enable LSE
        unsafe {
            let bdcr = (crate::rcc::RCC_BASE + crate::rcc::reg::BDCR) as *mut u32;

            // Enable LSE
            let mut val = core::ptr::read_volatile(bdcr);
            val |= 1 << 0; // LSEON
            core::ptr::write_volatile(bdcr, val);

            // Wait for LSE ready
            while (core::ptr::read_volatile(bdcr) & (1 << 1)) == 0 {}

            // Select LSE as RTC clock
            let mut val = core::ptr::read_volatile(bdcr);
            val &= !(0b11 << 8); // Clear RTCSEL
            val |= 0b01 << 8;    // RTCSEL = LSE
            core::ptr::write_volatile(bdcr, val);

            // Enable RTC clock
            let mut val = core::ptr::read_volatile(bdcr);
            val |= 1 << 15; // RTCEN
            core::ptr::write_volatile(bdcr, val);
        }

        self.disable_write_protection();

        // Enter init mode
        self.enter_init_mode()?;

        unsafe {
            // Configure prescaler for 1 Hz
            // PREDIV_A = 127, PREDIV_S = 255
            // 32768 / (128 * 256) = 1 Hz
            let prer = (RTC_BASE + reg::PRER) as *mut u32;
            core::ptr::write_volatile(prer, (127 << 16) | 255);

            // Set 24-hour format
            let cr = (RTC_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 6); // FMT = 0 (24-hour)
            core::ptr::write_volatile(cr, val);
        }

        // Exit init mode
        self.exit_init_mode();

        self.enable_write_protection();

        // Wait for synchronization
        self.wait_for_sync();

        Ok(())
    }

    /// Set time
    pub fn set_time(&self, time: &Time) -> Result<(), RtcError> {
        self.disable_write_protection();
        self.enter_init_mode()?;

        unsafe {
            let tr = (RTC_BASE + reg::TR) as *mut u32;

            let mut val = 0;
            val |= ((time.seconds % 10) as u32) << 0;   // SU
            val |= ((time.seconds / 10) as u32) << 4;   // ST
            val |= ((time.minutes % 10) as u32) << 8;   // MNU
            val |= ((time.minutes / 10) as u32) << 12;  // MNT
            val |= ((time.hours % 10) as u32) << 16;    // HU
            val |= ((time.hours / 10) as u32) << 20;    // HT
            if time.pm {
                val |= 1 << 22; // PM
            }

            core::ptr::write_volatile(tr, val);
        }

        self.exit_init_mode();
        self.enable_write_protection();

        Ok(())
    }

    /// Get time
    pub fn get_time(&self) -> Time {
        unsafe {
            let tr = (RTC_BASE + reg::TR) as *mut u32;
            let ssr = (RTC_BASE + reg::SSR) as *mut u32;

            let tr_val = core::ptr::read_volatile(tr);
            let ssr_val = core::ptr::read_volatile(ssr);

            Time {
                seconds: ((tr_val >> 4) & 0x7) as u8 * 10 + ((tr_val >> 0) & 0xF) as u8,
                minutes: ((tr_val >> 12) & 0x7) as u8 * 10 + ((tr_val >> 8) & 0xF) as u8,
                hours: ((tr_val >> 20) & 0x3) as u8 * 10 + ((tr_val >> 16) & 0xF) as u8,
                sub_seconds: ssr_val as u16,
                pm: ((tr_val >> 22) & 1) != 0,
            }
        }
    }

    /// Set date
    pub fn set_date(&self, date: &Date) -> Result<(), RtcError> {
        self.disable_write_protection();
        self.enter_init_mode()?;

        unsafe {
            let dr = (RTC_BASE + reg::DR) as *mut u32;

            let mut val = 0;
            val |= ((date.day % 10) as u32) << 0;      // DU
            val |= ((date.day / 10) as u32) << 4;      // DT
            val |= ((date.month % 10) as u32) << 8;    // MU
            val |= ((date.month / 10) as u32) << 12;   // MT
            val |= ((date.weekday) as u32) << 13;      // WDU
            val |= ((date.year % 10) as u32) << 16;    // YU
            val |= ((date.year / 10) as u32) << 20;    // YT

            core::ptr::write_volatile(dr, val);
        }

        self.exit_init_mode();
        self.enable_write_protection();

        Ok(())
    }

    /// Get date
    pub fn get_date(&self) -> Date {
        unsafe {
            let dr = (RTC_BASE + reg::DR) as *mut u32;
            let dr_val = core::ptr::read_volatile(dr);

            Date {
                day: ((dr_val >> 4) & 0x3) as u8 * 10 + ((dr_val >> 0) & 0xF) as u8,
                month: ((dr_val >> 12) & 0x1) as u8 * 10 + ((dr_val >> 8) & 0xF) as u8,
                year: ((dr_val >> 20) & 0xF) as u8 * 10 + ((dr_val >> 16) & 0xF) as u8,
                weekday: ((dr_val >> 13) & 0x7) as u8,
            }
        }
    }

    /// Set alarm
    pub fn set_alarm(&self, alarm: Alarm, time: &Time, mask: AlarmMask) -> Result<(), RtcError> {
        self.disable_write_protection();

        unsafe {
            let cr = (RTC_BASE + reg::CR) as *mut u32;
            let (alrmr, alrmassr) = match alarm {
                Alarm::AlarmA => ((RTC_BASE + reg::ALRMAR) as *mut u32, (RTC_BASE + reg::ALRMASSR) as *mut u32),
                Alarm::AlarmB => ((RTC_BASE + reg::ALRMBR) as *mut u32, (RTC_BASE + reg::ALRMBSSR) as *mut u32),
            };

            // Disable alarm
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << (alarm as u8 * 8)); // Clear ALRxE
            core::ptr::write_volatile(cr, val);

            // Wait for alarm to be disabled
            while (core::ptr::read_volatile(cr) & (1 << (alarm as u8 * 8 + 8))) != 0 {}

            // Configure alarm
            let mut val = 0;
            val |= ((time.seconds % 10) as u32) << 0;
            val |= ((time.seconds / 10) as u32) << 4;
            val |= ((time.minutes % 10) as u32) << 8;
            val |= ((time.minutes / 10) as u32) << 12;
            val |= ((time.hours % 10) as u32) << 16;
            val |= ((time.hours / 10) as u32) << 20;
            if time.pm {
                val |= 1 << 22;
            }
            val |= (mask as u32) << 24; // MSK
            core::ptr::write_volatile(alrmr, val);

            // Configure sub seconds
            core::ptr::write_volatile(alrmassr, 0);

            // Enable alarm
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << (alarm as u8 * 8); // ALRxE
            core::ptr::write_volatile(cr, val);
        }

        self.enable_write_protection();

        Ok(())
    }

    /// Enable wakeup timer
    pub fn enable_wakeup(&self, clock: WakeupClock, value: u16) {
        self.disable_write_protection();

        unsafe {
            let cr = (RTC_BASE + reg::CR) as *mut u32;
            let wutr = (RTC_BASE + reg::WUTR) as *mut u32;

            // Disable wakeup timer
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 10); // WUTE
            core::ptr::write_volatile(cr, val);

            // Wait for WUTWF
            while (core::ptr::read_volatile(cr) & (1 << 2)) == 0 {}

            // Set wakeup value
            core::ptr::write_volatile(wutr, value as u32);

            // Configure clock source
            let mut val = core::ptr::read_volatile(cr);
            val &= !(0b111 << 0); // Clear WUCKSEL
            val |= (clock as u32) << 0;
            core::ptr::write_volatile(cr, val);

            // Enable wakeup timer
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 10; // WUTE
            core::ptr::write_volatile(cr, val);
        }

        self.enable_write_protection();
    }

    /// Disable wakeup timer
    pub fn disable_wakeup(&self) {
        self.disable_write_protection();

        unsafe {
            let cr = (RTC_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 10); // WUTE
            core::ptr::write_volatile(cr, val);
        }

        self.enable_write_protection();
    }

    /// Read backup register
    pub fn read_backup(&self, index: u8) -> u32 {
        assert!(index < 32, "Backup register index must be 0-31");
        unsafe {
            let reg = (BACKUP_REG_BASE + (index as usize * 4)) as *mut u32;
            core::ptr::read_volatile(reg)
        }
    }

    /// Write backup register
    pub fn write_backup(&self, index: u8, value: u32) {
        assert!(index < 32, "Backup register index must be 0-31");
        unsafe {
            let reg = (BACKUP_REG_BASE + (index as usize * 4)) as *mut u32;
            core::ptr::write_volatile(reg, value);
        }
    }
}

/// Initialize RTC with default configuration
pub fn init_rtc_default() -> Result<(), RtcError> {
    let rtc = Rtc::new();
    rtc.init_lse()
}
