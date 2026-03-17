//! TSC - Touch Sensing Controller
//! 触摸感应控制器
//!
//! ## STM32U5 TSC 特性 / Features
//! - **通道数量 / Channels:** 最多 24 个触摸感应通道
//! - **感应类型 / Sensing Types:** 电容感应 (Capacitive sensing)
//! - **特性 / Features:**
//!   - 可编程充放电时间 (Programmable charge/discharge time)
//!   - 采样电容支持 (Sampling capacitor)
//!   - DMA 传输支持
//!   - 自动校准
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 59: Touch sensing controller (TSC)

/// TSC base address / TSC 基地址
pub const TSC_BASE: usize = 0x4002_4000;

/// TSC register offsets
pub mod reg {
    pub const CR: usize = 0x00;
    pub const IER: usize = 0x04;
    pub const ICR: usize = 0x08;
    pub const ISR: usize = 0x0C;
    pub const IOHCR: usize = 0x10;
    pub const IOASCR: usize = 0x18;
    pub const IOSCR: usize = 0x20;
    pub const IOCCR: usize = 0x28;
    pub const IOGCSR: usize = 0x30;
    pub const IOG1CR: usize = 0x34;
    pub const IOG2CR: usize = 0x38;
    pub const IOG3CR: usize = 0x3C;
    pub const IOG4CR: usize = 0x40;
    pub const IOG5CR: usize = 0x44;
    pub const IOG6CR: usize = 0x48;
    pub const IOG7CR: usize = 0x4C;
    pub const IOG8CR: usize = 0x50;
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
