//! TSC - Touch Sensing Controller
//! 触摸感应控制器
//!
//! ## STM32U5 TSC 特性 / Features
//! - **通道数量 / Channels:** 最多 24 个触摸感应通道 (8 groups x 4 channels)
//! - **感应类型 / Sensing Types:** 电容感应 (Capacitive sensing)
//! - **特性 / Features:**
//!   - 可编程充放电时间 (Programmable charge/discharge time)
//!   - 采样电容支持 (Sampling capacitor)
//!   - DMA 传输支持
//!   - 自动校准
//!   - 可编程最大计数值 (255, 511, 1023, 2047, 4095, 8191, 16383)
//!   - 扩频支持 (Spread spectrum)
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 47: Touch sensing controller (TSC)
//!   - Register map: RM0456, Section 47.7, pages 2027-2044
//!   - TSC Control Register (TSC_CR): RM0456, Section 47.7.1, page 2028
//!   - TSC Interrupt Enable Register (TSC_IER): RM0456, Section 47.7.2, page 2029
//!   - TSC I/O Channel Control Register (TSC_IOCCR): RM0456, Section 47.7.8, page 2035
//!   - TSC I/O Group Counter Register (TSC_IOGxCR): RM0456, Section 47.7.10, page 2037

/// TSC base address / TSC 基地址
pub const TSC_BASE: usize = 0x4002_4000;

/// TSC register offsets
pub mod reg {
    /// TSC Control Register (TSC_CR)
    /// RM0456, Section 47.7.1, page 2028
    pub const CR: usize = 0x00;
    /// TSC Interrupt Enable Register (TSC_IER)
    /// RM0456, Section 47.7.2, page 2029
    pub const IER: usize = 0x04;
    /// TSC Interrupt Clear Register (TSC_ICR)
    /// RM0456, Section 47.7.3, page 2030
    pub const ICR: usize = 0x08;
    /// TSC Interrupt Status Register (TSC_ISR)
    /// RM0456, Section 47.7.4, page 2031
    pub const ISR: usize = 0x0C;
    /// TSC I/O Hysteresis Control Register (TSC_IOHCR)
    /// RM0456, Section 47.7.5, page 2032
    pub const IOHCR: usize = 0x10;
    /// TSC I/O Analog Switch Control Register (TSC_IOASCR)
    /// RM0456, Section 47.7.6, page 2033
    pub const IOASCR: usize = 0x18;
    /// TSC I/O Sampling Capacitor Control Register (TSC_IOSCR)
    /// RM0456, Section 47.7.7, page 2034
    pub const IOSCR: usize = 0x20;
    /// TSC I/O Channel Control Register (TSC_IOCCR)
    /// RM0456, Section 47.7.8, page 2035
    pub const IOCCR: usize = 0x28;
    /// TSC I/O Group Control and Status Register (TSC_IOGCSR)
    /// RM0456, Section 47.7.9, page 2036
    pub const IOGCSR: usize = 0x30;
    /// TSC I/O Group 1 Counter Register (TSC_IOG1CR)
    /// RM0456, Section 47.7.10, page 2037
    pub const IOG1CR: usize = 0x34;
    /// TSC I/O Group 2 Counter Register (TSC_IOG2CR)
    pub const IOG2CR: usize = 0x38;
    /// TSC I/O Group 3 Counter Register (TSC_IOG3CR)
    pub const IOG3CR: usize = 0x3C;
    /// TSC I/O Group 4 Counter Register (TSC_IOG4CR)
    pub const IOG4CR: usize = 0x40;
    /// TSC I/O Group 5 Counter Register (TSC_IOG5CR)
    pub const IOG5CR: usize = 0x44;
    /// TSC I/O Group 6 Counter Register (TSC_IOG6CR)
    pub const IOG6CR: usize = 0x48;
    /// TSC I/O Group 7 Counter Register (TSC_IOG7CR)
    pub const IOG7CR: usize = 0x4C;
    /// TSC I/O Group 8 Counter Register (TSC_IOG8CR)
    pub const IOG8CR: usize = 0x50;
}

/// TSC register bit definitions
pub mod bits {
    /// TSC Control Register (TSC_CR) bits
    pub mod cr {
        /// TSC Enable (TSCE)
        pub const TSCE: u32 = 1 << 0;
        /// Start acquisition (START)
        pub const START: u32 = 1 << 1;
        /// Acquisition mode (AM)
        pub const AM: u32 = 1 << 2;
        /// Synchronization pin polarity (SYNCPOL)
        pub const SYNCPOL: u32 = 1 << 3;
        /// Spread spectrum deviation (SSD)
        pub const SSD: u32 = 0b1111111 << 17;
        /// Spread spectrum prescaler (SSPSC)
        pub const SSPSC: u32 = 1 << 16;
        /// Pulse generator prescaler (PGPSC)
        pub const PGPSC: u32 = 0b111 << 12;
        /// Max count value (MCV)
        pub const MCV: u32 = 0b111 << 5;
        /// Charge transfer pulse high (CTPH)
        pub const CTPH: u32 = 0b1111 << 28;
        /// Charge transfer pulse low (CTPL)
        pub const CTPL: u32 = 0b1111 << 24;
    }

    /// TSC Interrupt Enable Register (TSC_IER) bits
    pub mod ier {
        /// End of acquisition interrupt enable (EOAIE)
        pub const EOAIE: u32 = 1 << 0;
        /// Max count error interrupt enable (MCEIE)
        pub const MCEIE: u32 = 1 << 1;
    }

    /// TSC Interrupt Clear Register (TSC_ICR) bits
    pub mod icr {
        /// End of acquisition interrupt clear (EOAIC)
        pub const EOAIC: u32 = 1 << 0;
        /// Max count error interrupt clear (MCEIC)
        pub const MCEIC: u32 = 1 << 1;
    }

    /// TSC Interrupt Status Register (TSC_ISR) bits
    pub mod isr {
        /// End of acquisition flag (EOAF)
        pub const EOAF: u32 = 1 << 0;
        /// Max count error flag (MCEF)
        pub const MCEF: u32 = 1 << 1;
    }

    /// TSC I/O Group Control and Status Register (TSC_IOGCSR) bits
    pub mod iogcsr {
        /// Analog I/O group enable (G1E-G8E)
        pub const G1E: u32 = 1 << 0;
        pub const G2E: u32 = 1 << 1;
        pub const G3E: u32 = 1 << 2;
        pub const G4E: u32 = 1 << 3;
        pub const G5E: u32 = 1 << 4;
        pub const G6E: u32 = 1 << 5;
        pub const G7E: u32 = 1 << 6;
        pub const G8E: u32 = 1 << 7;
        /// Analog I/O group status (G1S-G8S)
        pub const G1S: u32 = 1 << 16;
        pub const G2S: u32 = 1 << 17;
        pub const G3S: u32 = 1 << 18;
        pub const G4S: u32 = 1 << 19;
        pub const G5S: u32 = 1 << 20;
        pub const G6S: u32 = 1 << 21;
        pub const G7S: u32 = 1 << 22;
        pub const G8S: u32 = 1 << 23;
    }
}

/// TSC group
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Group {
    Group1 = 0,
    Group2 = 1,
    Group3 = 2,
    Group4 = 3,
    Group5 = 4,
    Group6 = 5,
    Group7 = 6,
    Group8 = 7,
}

/// TSC channel within a group
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
    Channel1 = 0,
    Channel2 = 1,
    Channel3 = 2,
    Channel4 = 3,
}

/// TSC configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Charge transfer pulse high
    pub ctpulse_high: u8,
    /// Charge transfer pulse low
    pub ctpulse_low: u8,
    /// Spread spectrum deviation
    pub ss_dev: u8,
    /// Spread spectrum prescaler
    pub ss_presc: bool,
    /// Pulse generator prescaler
    pub pg_presc: u8,
    /// Max count value
    pub max_count: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ctpulse_high: 1,
            ctpulse_low: 1,
            ss_dev: 0,
            ss_presc: false,
            pg_presc: 0,
            max_count: 0b00, // 255
        }
    }
}

/// TSC instance
pub struct Tsc;

impl Tsc {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize TSC
    pub fn init(&self, config: &Config) {
        // Enable TSC clock
        crate::rcc::enable_ahb1_clock(crate::rcc::ahb1::TSC);

        unsafe {
            // Configure TSC
            let cr = (TSC_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= (config.ctpulse_high as u32) << 28;
            val |= (config.ctpulse_low as u32) << 24;
            val |= (config.ss_dev as u32) << 17;
            val |= (config.ss_presc as u32) << 16;
            val |= (config.pg_presc as u32) << 12;
            val |= (config.max_count as u32) << 5;
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable channel
    pub fn enable_channel(&self, group: Group, channel: Channel) {
        unsafe {
            let ioascr = (TSC_BASE + reg::IOASCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ioascr);
            val |= 1 << ((group as u8) * 4 + (channel as u8));
            core::ptr::write_volatile(ioascr, val);
        }
    }

    /// Disable channel
    pub fn disable_channel(&self, group: Group, channel: Channel) {
        unsafe {
            let ioascr = (TSC_BASE + reg::IOASCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ioascr);
            val &= !(1 << ((group as u8) * 4 + (channel as u8)));
            core::ptr::write_volatile(ioascr, val);
        }
    }

    /// Set channel as sampling capacitor
    pub fn set_sampling_cap(&self, group: Group, channel: Channel) {
        unsafe {
            let ioscr = (TSC_BASE + reg::IOSCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ioscr);
            val |= 1 << ((group as u8) * 4 + (channel as u8));
            core::ptr::write_volatile(ioscr, val);
        }
    }

    /// Set channel as output
    pub fn set_channel_output(&self, group: Group, channel: Channel) {
        unsafe {
            let ioccr = (TSC_BASE + reg::IOCCR) as *mut u32;
            let mut val = core::ptr::read_volatile(ioccr);
            val |= 1 << ((group as u8) * 4 + (channel as u8));
            core::ptr::write_volatile(ioccr, val);
        }
    }

    /// Start acquisition
    pub fn start(&self) {
        unsafe {
            let cr = (TSC_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 1; // START
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Check if acquisition is complete
    pub fn is_complete(&self) -> bool {
        unsafe {
            let isr = (TSC_BASE + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (val & 1) != 0 // EOAOF
        }
    }

    /// Read group counter value
    pub fn read_group(&self, group: Group) -> u16 {
        unsafe {
            let iogcr = (TSC_BASE + reg::IOG1CR + (group as usize) * 4) as *mut u32;
            (core::ptr::read_volatile(iogcr) & 0x3FFF) as u16
        }
    }

    /// Enable TSC
    pub fn enable(&self) {
        unsafe {
            let cr = (TSC_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // TSCE
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable TSC
    pub fn disable(&self) {
        unsafe {
            let cr = (TSC_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 0);
            core::ptr::write_volatile(cr, val);
        }
    }
}

/// Initialize TSC with default configuration
pub fn init_tsc_default() {
    let tsc = Tsc::new();
    let config = Config::default();
    tsc.init(&config);
    tsc.enable();
}

/// Simple touch detection example
pub fn read_touch_sensor(group: Group, channel: Channel) -> u16 {
    let tsc = Tsc::new();

    // Configure channel
    tsc.enable_channel(group, channel);
    tsc.set_channel_output(group, channel);

    // Set sampling capacitor (usually channel 0 of each group)
    tsc.set_sampling_cap(group, Channel::Channel0);

    // Start acquisition
    tsc.start();

    // Wait for completion
    while !tsc.is_complete() {}

    // Read result
    tsc.read_group(group)
}
