//! DAC - Digital-to-Analog Converter
//! 数模转换器
//!
//! STM32U5 具有两个 12-bit DAC 通道：
//! - DAC1_OUT1: PA4
//! - DAC1_OUT2: PA5
//!
//! 支持多种触发源和波形生成功能

/// DAC base address
pub const DAC_BASE: usize = 0x4000_7400;

/// DAC register offsets
pub mod reg {
    /// DAC control register
    pub const CR: usize = 0x00;
    /// DAC software trigger register
    pub const SWTRGR: usize = 0x04;
    /// DAC channel 1 12-bit right-aligned data holding register
    pub const DHR12R1: usize = 0x08;
    /// DAC channel 1 12-bit left-aligned data holding register
    pub const DHR12L1: usize = 0x0C;
    /// DAC channel 1 8-bit right-aligned data holding register
    pub const DHR8R1: usize = 0x10;
    /// DAC channel 2 12-bit right-aligned data holding register
    pub const DHR12R2: usize = 0x14;
    /// DAC channel 2 12-bit left-aligned data holding register
    pub const DHR12L2: usize = 0x18;
    /// DAC channel 2 8-bit right-aligned data holding register
    pub const DHR8R2: usize = 0x1C;
    /// Dual DAC 12-bit right-aligned data holding register
    pub const DHR12RD: usize = 0x20;
    /// Dual DAC 12-bit left-aligned data holding register
    pub const DHR12LD: usize = 0x24;
    /// Dual DAC 8-bit right-aligned data holding register
    pub const DHR8RD: usize = 0x28;
    /// DAC channel 1 data output register
    pub const DOR1: usize = 0x2C;
    /// DAC channel 2 data output register
    pub const DOR2: usize = 0x30;
    /// DAC status register
    pub const SR: usize = 0x34;
    /// DAC calibration mode register
    pub const CCR: usize = 0x38;
    /// DAC mode register
    pub const MCR: usize = 0x3C;
    /// DAC channel 1 sample and hold sample time register
    pub const SHSR1: usize = 0x40;
    /// DAC channel 2 sample and hold sample time register
    pub const SHSR2: usize = 0x44;
    /// DAC sample and hold time register
    pub const SHHR: usize = 0x48;
    /// DAC sample and hold refresh time register
    pub const SHRR: usize = 0x4C;
}

/// DAC channel
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
    /// DAC channel 1 (PA4)
    Channel1 = 0,
    /// DAC channel 2 (PA5)
    Channel2 = 1,
}

/// DAC trigger source
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trigger {
    /// Software trigger
    Software = 0b000,
    /// Timer 8 TRGO
    Tim8 = 0b001,
    /// Timer 7 TRGO
    Tim7 = 0b010,
    /// Timer 15 TRGO
    Tim15 = 0b011,
    /// Timer 2 TRGO
    Tim2 = 0b100,
    /// Timer 4 TRGO
    Tim4 = 0b101,
    /// External line 9
    Exti9 = 0b110,
    /// Timer 6 TRGO
    Tim6 = 0b111,
    /// Timer 3 TRGO
    Tim3 = 0b1000,
    /// HRTIM RST TRG1
    HrtimRst1 = 0b1001,
    /// HRTIM RST TRG2
    HrtimRst2 = 0b1010,
    /// LPTIM1 OUT
    Lptim1 = 0b1011,
    /// LPTIM2 OUT
    Lptim2 = 0b1100,
    /// LPTIM3 OUT
    Lptim3 = 0b1101,
}

/// DAC configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Enable buffer
    pub buffer_enable: bool,
    /// Trigger source
    pub trigger: Trigger,
    /// Trigger enable
    pub trigger_enable: bool,
    /// DMA enable
    pub dma_enable: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_enable: true,
            trigger: Trigger::Software,
            trigger_enable: false,
            dma_enable: false,
        }
    }
}

/// DAC instance
pub struct Dac {
    base: usize,
}

impl Dac {
    /// Create DAC instance
    pub const fn new() -> Self {
        Self { base: DAC_BASE }
    }

    /// Initialize DAC channel
    pub fn init_channel(&self, channel: Channel, config: &Config) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);

            let ch = channel as u8;

            // Enable channel
            val |= 1 << (ch * 16); // ENx

            // Configure buffer
            if !config.buffer_enable {
                val |= 1 << (ch * 16 + 1); // BOFFx
            }

            // Configure trigger
            if config.trigger_enable {
                val |= 1 << (ch * 16 + 2); // TENx
                val &= !(0b1111 << (ch * 16 + 3)); // Clear TSELx
                val |= (config.trigger as u32) << (ch * 16 + 3); // Set TSELx
            }

            // Configure DMA
            if config.dma_enable {
                val |= 1 << (ch * 16 + 12); // DMAENx
            }

            core::ptr::write_volatile(cr, val);

            // Wait for channel to be ready
            while !self.is_channel_ready(channel) {}
        }
    }

    /// Check if channel is ready
    pub fn is_channel_ready(&self, channel: Channel) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << (channel as u8 * 16 + 14))) != 0 // DACxRDY
        }
    }

    /// Set output value (12-bit right-aligned)
    pub fn set_value_12bit(&self, channel: Channel, value: u16) {
        unsafe {
            let reg = match channel {
                Channel::Channel1 => (self.base + reg::DHR12R1) as *mut u32,
                Channel::Channel2 => (self.base + reg::DHR12R2) as *mut u32,
            };
            core::ptr::write_volatile(reg, (value & 0xFFF) as u32);
        }
    }

    /// Set output value (8-bit right-aligned)
    pub fn set_value_8bit(&self, channel: Channel, value: u8) {
        unsafe {
            let reg = match channel {
                Channel::Channel1 => (self.base + reg::DHR8R1) as *mut u32,
                Channel::Channel2 => (self.base + reg::DHR8R2) as *mut u32,
            };
            core::ptr::write_volatile(reg, value as u32);
        }
    }

    /// Set output voltage
    ///
    /// # Arguments
    /// * `channel` - DAC channel
    /// * `voltage_mv` - Output voltage in millivolts (0-3300 for 3.3V VREF)
    /// * `vref_mv` - Reference voltage in millivolts
    pub fn set_voltage(&self, channel: Channel, voltage_mv: u32, vref_mv: u32) {
        let value = ((voltage_mv * 4095) / vref_mv) as u16;
        self.set_value_12bit(channel, value);
    }

    /// Trigger software conversion
    pub fn software_trigger(&self, channel: Channel) {
        unsafe {
            let swtrgr = (self.base + reg::SWTRGR) as *mut u32;
            core::ptr::write_volatile(swtrgr, 1 << (channel as u8));
        }
    }

    /// Get current output value
    pub fn get_output(&self, channel: Channel) -> u16 {
        unsafe {
            let reg = match channel {
                Channel::Channel1 => (self.base + reg::DOR1) as *mut u32,
                Channel::Channel2 => (self.base + reg::DOR2) as *mut u32,
            };
            core::ptr::read_volatile(reg) as u16
        }
    }

    /// Enable channel
    pub fn enable_channel(&self, channel: Channel) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << ((channel as u8) * 16);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable channel
    pub fn disable_channel(&self, channel: Channel) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << ((channel as u8) * 16));
            core::ptr::write_volatile(cr, val);
        }
    }
}

/// Initialize DAC with default configuration
pub fn init_dac_default() {
    // Enable DAC clock
    crate::rcc::enable_apb1_clock(crate::rcc::apb1::DAC);

    // Configure PA4 as analog for DAC1_OUT1
    use super::gpio;
    let pa4 = gpio::pins::PA4;
    pa4.init_analog();

    let dac = Dac::new();
    let config = Config::default();
    dac.init_channel(Channel::Channel1, &config);
}

/// Generate simple sine wave on DAC channel
pub fn generate_sine_wave(channel: Channel, frequency_hz: u32, sample_rate_hz: u32) {
    use micromath::F32Ext;

    let dac = Dac::new();
    let samples = sample_rate_hz / frequency_hz;

    loop {
        for i in 0..samples {
            let angle = 2.0 * core::f32::consts::PI * (i as f32) / (samples as f32);
            let sine = libm::sinf(angle);
            let value = ((sine + 1.0) * 2047.5) as u16; // Convert to 0-4095 range
            dac.set_value_12bit(channel, value);
            dac.software_trigger(channel);

            // Simple delay
            for _ in 0..100 {
                unsafe { core::arch::asm!("nop") };
            }
        }
    }
}
