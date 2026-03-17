//! DAC - Digital-to-Analog Converter
//! 数模转换器
//!
//! # Overview / 概述
//! STM32U5 Digital-to-Analog Converter (DAC) provides two 12-bit output channels
//! for generating analog voltages from digital values.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 35: Digital-to-analog converter (DAC)
//!
//! ## DAC Channels / DAC通道
//! - **DAC1_OUT1**: PA4 (12-bit resolution)
//! - **DAC1_OUT2**: PA5 (12-bit resolution)
//!
//! ## Trigger Sources / 触发源
//! Reference: RM0456 Section 35.3.4
//! - Software trigger
//! - Timer triggers (TIM2, TIM4, TIM6, TIM7, TIM8, TIM15)
//! - EXTI trigger (line 9)
//! - HRTIM triggers
//! - LPTIM triggers (LPTIM1, LPTIM2, LPTIM3)
//!
//! ## Features / 特性
//! - Single or dual channel mode
//! - 12-bit resolution (can be configured to 8-bit for faster updates)
//! - Output buffer with programmable gain
//! - Waveform generation (triangular, noise)
//! - DMA support for continuous conversions
//! - Sample and hold functionality
//!
//! # Output Voltage Calculation / 输出电压计算
//! Reference: RM0456 Section 35.3.2
//! 
//! Vout = (D / 4095) * Vref
//! 
//! Where:
//! - D = Digital value (0-4095)
//! - Vref = Reference voltage (typically 3.3V)
//!
//! # Reference / 参考
//! - RM0456 Chapter 35: Digital-to-analog converter (DAC)
//! - RM0456 Section 35.1: DAC introduction
//! - RM0456 Section 35.2: DAC main features
//! - RM0456 Section 35.3: DAC functional description
//! - RM0456 Section 35.4: DAC registers

#![no_std]

/// DAC base address (non-secure)
//! Reference: RM0456 Chapter 2, Table 1
pub const DAC_BASE: usize = 0x4000_7400;

/// DAC register offsets
//! Reference: RM0456 Section 35.4: DAC registers
pub mod reg {
    /// DAC control register
    //! Reference: RM0456 Section 35.4.1: DAC_CR
    pub const CR: usize = 0x00;

    /// DAC software trigger register
    //! Reference: RM0456 Section 35.4.2: DAC_SWTRGR
    pub const SWTRGR: usize = 0x04;

    /// DAC channel 1 12-bit right-aligned data holding register
    //! Reference: RM0456 Section 35.4.3: DAC_DHR12R1
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
    //! Reference: RM0456 Section 35.4.4: DAC_DOR1
    pub const DOR1: usize = 0x2C;

    /// DAC channel 2 data output register
    pub const DOR2: usize = 0x30;

    /// DAC status register
    //! Reference: RM0456 Section 35.4.5: DAC_SR
    pub const SR: usize = 0x34;

    /// DAC calibration mode register
    //! Reference: RM0456 Section 35.4.6: DAC_CCR
    pub const CCR: usize = 0x38;

    /// DAC mode register
    //! Reference: RM0456 Section 35.4.7: DAC_MCR
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

/// CR Register Bit Definitions
//! Reference: RM0456 Section 35.4.1
pub mod cr_bits {
    /// Channel 1 enable
    pub const EN1: u32 = 1 << 0;
    /// Channel 1 buffer disable
    pub const BOFF1: u32 = 1 << 1;
    /// Channel 1 trigger enable
    pub const TEN1: u32 = 1 << 2;
    /// Channel 1 trigger selection
    pub const TSEL1: u32 = 0b1111 << 3;
    /// Channel 1 waveform generation enable
    pub const WAVE1: u32 = 0b11 << 7;
    /// Channel 1 noise wave generation
    pub const MAMP1: u32 = 0b1111 << 9;
    /// Channel 1 DMA enable
    pub const DMAEN1: u32 = 1 << 12;
    /// Channel 1 DMA underrun interrupt enable
    pub const DMAUDRIE1: u32 = 1 << 13;
    /// Channel 1 ready flag
    pub const DAC1RDY: u32 = 1 << 14;

    /// Channel 2 enable
    pub const EN2: u32 = 1 << 16;
    /// Channel 2 buffer disable
    pub const BOFF2: u32 = 1 << 17;
    /// Channel 2 trigger enable
    pub const TEN2: u32 = 1 << 18;
    /// Channel 2 trigger selection
    pub const TSEL2: u32 = 0b1111 << 19;
    /// Channel 2 waveform generation enable
    pub const WAVE2: u32 = 0b11 << 23;
    /// Channel 2 noise wave generation
    pub const MAMP2: u32 = 0b1111 << 25;
    /// Channel 2 DMA enable
    pub const DMAEN2: u32 = 1 << 28;
    /// Channel 2 DMA underrun interrupt enable
    pub const DMAUDRIE2: u32 = 1 << 29;
    /// Channel 2 ready flag
    pub const DAC2RDY: u32 = 1 << 30;
}

/// SR Register Bit Definitions
//! Reference: RM0456 Section 35.4.5
pub mod sr_bits {
    /// DAC channel 1 DMA underrun flag
    pub const DMAUDR1: u32 = 1 << 13;
    /// DAC channel 1 ready flag
    pub const DAC1RDY: u32 = 1 << 14;
    /// DAC channel 2 DMA underrun flag
    pub const DMAUDR2: u32 = 1 << 29;
    /// DAC channel 2 ready flag
    pub const DAC2RDY: u32 = 1 << 30;
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
//! Reference: RM0456 Section 35.3.4, Table 428
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trigger {
    /// Software trigger
    Software = 0b0000,
    /// Timer 8 TRGO
    Tim8 = 0b0001,
    /// Timer 7 TRGO
    Tim7 = 0b0010,
    /// Timer 15 TRGO
    Tim15 = 0b0011,
    /// Timer 2 TRGO
    Tim2 = 0b0100,
    /// Timer 4 TRGO
    Tim4 = 0b0101,
    /// External line 9
    Exti9 = 0b0110,
    /// Timer 6 TRGO
    Tim6 = 0b0111,
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
    /// LPTIM4 OUT
    Lptim4 = 0b1110,
    /// LPTIM5 OUT
    Lptim5 = 0b1111,
}

/// DAC waveform generation mode
//! Reference: RM0456 Section 35.3.6
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Waveform {
    /// No waveform generation
    None = 0b00,
    /// Noise wave
    Noise = 0b01,
    /// Triangle wave
    Triangle = 0b10,
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
    /// Waveform generation
    pub waveform: Waveform,
    /// Noise amplitude (0-15 for noise waveform)
    pub noise_amplitude: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_enable: true,
            trigger: Trigger::Software,
            trigger_enable: false,
            dma_enable: false,
            waveform: Waveform::None,
            noise_amplitude: 0,
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
    //! Reference: RM0456 Section 35.3.1
    pub fn init_channel(&self, channel: Channel, config: &Config) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);

            let ch = channel as u8;

            // Enable channel
            // Reference: RM0456 Section 35.4.1, bit ENx
            val |= 1 << (ch * 16);

            // Configure buffer
            // Reference: RM0456 Section 35.4.1, bit BOFFx
            if !config.buffer_enable {
                val |= 1 << (ch * 16 + 1);
            }

            // Configure trigger
            // Reference: RM0456 Section 35.4.1, bits TENx and TSELx[3:0]
            if config.trigger_enable {
                val |= 1 << (ch * 16 + 2);
                val &= !(0b1111 << (ch * 16 + 3));
                val |= (config.trigger as u32) << (ch * 16 + 3);
            }

            // Configure waveform generation
            // Reference: RM0456 Section 35.4.1, bits WAVE[1:0] and MAMP[3:0]
            val &= !(0b11 << (ch * 16 + 7));
            val |= (config.waveform as u32) << (ch * 16 + 7);
            val &= !(0b1111 << (ch * 16 + 9));
            val |= (config.noise_amplitude as u32) << (ch * 16 + 9);

            // Configure DMA
            // Reference: RM0456 Section 35.4.1, bit DMAENx
            if config.dma_enable {
                val |= 1 << (ch * 16 + 12);
            }

            core::ptr::write_volatile(cr, val);

            // Wait for channel to be ready
            while !self.is_channel_ready(channel) {}
        }
    }

    /// Check if channel is ready
    //! Reference: RM0456 Section 35.4.5, bit DACxRDY
    pub fn is_channel_ready(&self, channel: Channel) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << (channel as u8 * 16 + 14))) != 0
        }
    }

    /// Set output value (12-bit right-aligned)
    //! Reference: RM0456 Section 35.3.2
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
    //! Reference: RM0456 Section 35.3.2
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
    //! Reference: RM0456 Section 35.3.4
    pub fn software_trigger(&self, channel: Channel) {
        unsafe {
            let swtrgr = (self.base + reg::SWTRGR) as *mut u32;
            core::ptr::write_volatile(swtrgr, 1 << (channel as u8));
        }
    }

    /// Get current output value
    //! Reference: RM0456 Section 35.4.4
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

    /// Set waveform generation
    //! Reference: RM0456 Section 35.3.6
    pub fn set_waveform(&self, channel: Channel, waveform: Waveform, amplitude: u8) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(0b11 << (channel as u8 * 16 + 7));
            val |= (waveform as u32) << (channel as u8 * 16 + 7);
            val &= !(0b1111 << (channel as u8 * 16 + 9));
            val |= (amplitude as u32) << (channel as u8 * 16 + 9);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Clear DMA underrun flag
    //! Reference: RM0456 Section 35.4.5
    pub fn clear_dma_underrun(&self, channel: Channel) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let mut val = core::ptr::read_volatile(sr);
            val &= !(1 << (channel as u8 * 16 + 13));
            core::ptr::write_volatile(sr, val);
        }
    }

    /// Check if DMA underrun occurred
    pub fn has_dma_underrun(&self, channel: Channel) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << (channel as u8 * 16 + 13))) != 0
        }
    }
}

/// Initialize DAC with default configuration
pub fn init_dac_default() {
    // Enable DAC clock
    // Reference: RM0456 Section 11.10.5: RCC_APB1ENR1
    crate::rcc::enable_apb1_clock(crate::rcc::apb1_1::DAC1);

    // Configure PA4 as analog for DAC1_OUT1
    // Reference: RM0456 Chapter 13: GPIO
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
                unsafe { core::arch::asm!("nop"); };
            }
        }
    }
}

/// Generate triangular wave on DAC channel
pub fn generate_triangle_wave(channel: Channel, frequency_hz: u32, amplitude: u16) {
    let dac = Dac::new();
    let config = Config {
        waveform: Waveform::Triangle,
        ..Default::default()
    };
    dac.init_channel(channel, &config);

    // Set amplitude
    dac.set_waveform(channel, Waveform::Triangle, (amplitude / 256) as u8);

    // Start generation
    dac.software_trigger(channel);
}

/// Generate noise wave on DAC channel
pub fn generate_noise_wave(channel: Channel, amplitude: u8) {
    let dac = Dac::new();
    let config = Config {
        waveform: Waveform::Noise,
        noise_amplitude: amplitude,
        ..Default::default()
    };
    dac.init_channel(channel, &config);

    // Start generation
    dac.software_trigger(channel);
}
