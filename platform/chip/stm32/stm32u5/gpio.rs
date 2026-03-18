//! GPIO - General Purpose Input/Output
//! 通用输入输出
//!
//! # Overview / 概述
//! STM32U5 General Purpose Input/Output (GPIO) module provides flexible
//! control of device pins for various functions.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 13
//!
//! - Up to 9 GPIO ports (GPIOA-GPIOI)
//! - Up to 16 pins per port
//! - Pin modes: Input, Output, Alternate Function, Analog
//! - Output types: Push-pull, Open-drain
//! - Output speeds: Low, Medium, High, Very High
//! - Pull-up/Pull-down resistors
//! - Lock mechanism for pin configuration
//! - TrustZone security support
//! - External interrupt/event lines (EXTI)
//!
//! # Pin Multiplexing / 引脚复用
//! Each GPIO pin can be configured for various alternate functions:
//! - USART, SPI, I2C, CAN, USB
//! - Timers, ADC, DAC
//! - Clock outputs (MCO)
//! - And many more...
//!
//! # Reference / 参考
//! - RM0456 Chapter 13: General-purpose I/Os (GPIO)
//! - RM0456 Section 13.1: GPIO introduction
//! - RM0456 Section 13.2: GPIO main features
//! - RM0456 Section 13.3: GPIO functional description
//! - RM0456 Section 13.4: GPIO registers

//! GPIO port base addresses (AHB2 bus)
//! Reference: RM0456 Chapter 2, Table 1
pub const GPIOA_BASE: usize = 0x4202_0000;
pub const GPIOB_BASE: usize = 0x4202_0400;
pub const GPIOC_BASE: usize = 0x4202_0800;
pub const GPIOD_BASE: usize = 0x4202_0C00;
pub const GPIOE_BASE: usize = 0x4202_1000;
pub const GPIOF_BASE: usize = 0x4202_1400;
pub const GPIOG_BASE: usize = 0x4202_1800;
pub const GPIOH_BASE: usize = 0x4202_1C00;
pub const GPIOI_BASE: usize = 0x4202_2000;

//! GPIO Register Offsets
//! Reference: RM0456 Section 13.4: GPIO registers
pub mod reg {
    //! GPIO port mode register
    //! Reference: RM0456 Section 13.4.1
    pub const MODER: usize = 0x00;

    //! GPIO port output type register
    //! Reference: RM0456 Section 13.4.2
    pub const OTYPER: usize = 0x04;

    //! GPIO port output speed register
    //! Reference: RM0456 Section 13.4.3
    pub const OSPEEDR: usize = 0x08;

    //! GPIO port pull-up/pull-down register
    //! Reference: RM0456 Section 13.4.4
    pub const PUPDR: usize = 0x0C;

    //! GPIO port input data register
    //! Reference: RM0456 Section 13.4.5
    pub const IDR: usize = 0x10;

    //! GPIO port output data register
    //! Reference: RM0456 Section 13.4.6
    pub const ODR: usize = 0x14;

    //! GPIO port bit set/reset register
    //! Reference: RM0456 Section 13.4.7
    pub const BSRR: usize = 0x18;

    //! GPIO port configuration lock register
    //! Reference: RM0456 Section 13.4.8
    pub const LCKR: usize = 0x1C;

    //! GPIO alternate function low register
    //! Reference: RM0456 Section 13.4.9
    pub const AFRL: usize = 0x20;

    //! GPIO alternate function high register
    //! Reference: RM0456 Section 13.4.9
    pub const AFRH: usize = 0x24;

    //! GPIO port bit reset register
    //! Reference: RM0456 Section 13.4.10
    pub const BRR: usize = 0x28;

    //! GPIO port secure configuration register
    //! Reference: RM0456 Section 13.4.11
    pub const SECCFGR: usize = 0x30;
}

//! MODER Register Bit Definitions
//! Reference: RM0456 Section 13.4.1
pub mod moder_bits {
    pub const MODE_INPUT: u32 = 0b00;
    pub const MODE_OUTPUT: u32 = 0b01;
    pub const MODE_AF: u32 = 0b10;
    pub const MODE_ANALOG: u32 = 0b11;
}

//! GPIO Pin Modes
//! Reference: RM0456 Section 13.3.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Input mode (reset value)
    Input = 0b00,
    /// General purpose output mode
    Output = 0b01,
    /// Alternate function mode
    Alternate = 0b10,
    /// Analog mode
    Analog = 0b11,
}

//! GPIO Output Types
//! Reference: RM0456 Section 13.3.2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputType {
    /// Push-pull output (reset value)
    PushPull = 0,
    /// Open-drain output
    OpenDrain = 1,
}

//! GPIO Output Speeds
//! Reference: RM0456 Section 13.3.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Speed {
    /// Low speed (0-780 kHz at 3.3V, 1-1.5 MHz at 1.8V)
    Low = 0b00,
    /// Medium speed (0-30 MHz at 3.3V, 0-20 MHz at 1.8V)
    Medium = 0b01,
    /// High speed (0-80 MHz at 3.3V, 0-40 MHz at 1.8V)
    High = 0b10,
    /// Very high speed (0-120 MHz at 3.3V, 0-60 MHz at 1.8V)
    VeryHigh = 0b11,
}

//! GPIO Pull-up/Pull-down Configuration
//! Reference: RM0456 Section 13.3.4
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pull {
    /// No pull-up or pull-down (reset value)
    None = 0b00,
    /// Pull-up resistor enabled
    Up = 0b01,
    /// Pull-down resistor enabled
    Down = 0b10,
    /// Reserved
    Reserved = 0b11,
}

//! GPIO Alternate Functions
//! Reference: RM0456 Section 13.3.5 and Table 21-23
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AlternateFunction {
    AF0 = 0,
    AF1 = 1,
    AF2 = 2,
    AF3 = 3,
    AF4 = 4,
    AF5 = 5,
    AF6 = 6,
    AF7 = 7,
    AF8 = 8,
    AF9 = 9,
    AF10 = 10,
    AF11 = 11,
    AF12 = 12,
    AF13 = 13,
    AF14 = 14,
    AF15 = 15,
}

//! GPIO Port Enumeration
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Port {
    PortA = 0,
    PortB = 1,
    PortC = 2,
    PortD = 3,
    PortE = 4,
    PortF = 5,
    PortG = 6,
    PortH = 7,
    PortI = 8,
}

//! GPIO Pin Structure
//! Reference: RM0456 Section 13.3
#[derive(Clone, Copy, Debug)]
pub struct Pin {
    port: Port,
    pin: u8,
}

impl Pin {
    /// Create a new GPIO pin
    ///
    /// # Arguments
    /// * `port` - GPIO port (A-I)
    /// * `pin` - Pin number (0-15)
    pub const fn new(port: Port, pin: u8) -> Self {
        assert!(pin < 16, "Pin number must be 0-15");
        Self { port, pin }
    }

    /// Get port base address
    fn port_base(&self) -> usize {
        match self.port {
            Port::PortA => GPIOA_BASE,
            Port::PortB => GPIOB_BASE,
            Port::PortC => GPIOC_BASE,
            Port::PortD => GPIOD_BASE,
            Port::PortE => GPIOE_BASE,
            Port::PortF => GPIOF_BASE,
            Port::PortG => GPIOG_BASE,
            Port::PortH => GPIOH_BASE,
            Port::PortI => GPIOI_BASE,
        }
    }

    /// Initialize pin as input
    /// Reference: RM0456 Section 13.3.1
    pub fn init_input(&self, pull: Pull) {
        self.set_mode(Mode::Input);
        self.set_pull(pull);
    }

    /// Initialize pin as output
    /// Reference: RM0456 Section 13.3.1
    pub fn init_output(&self, otype: OutputType, speed: Speed) {
        self.set_mode(Mode::Output);
        self.set_output_type(otype);
        self.set_speed(speed);
    }

    /// Initialize pin as alternate function
    /// Reference: RM0456 Section 13.3.1 and 13.3.5
    pub fn init_alternate(&self, af: AlternateFunction, otype: OutputType, speed: Speed, pull: Pull) {
        self.set_mode(Mode::Alternate);
        self.set_alternate_function(af);
        self.set_output_type(otype);
        self.set_speed(speed);
        self.set_pull(pull);
    }

    /// Initialize pin as analog mode
    /// Reference: RM0456 Section 13.3.1
    pub fn init_analog(&self) {
        self.set_mode(Mode::Analog);
    }

    /// Set pin mode
    /// Reference: RM0456 Section 13.4.1
    pub fn set_mode(&self, mode: Mode) {
        unsafe {
            let moder = (self.port_base() + reg::MODER) as *mut u32;
            let mut val = core::ptr::read_volatile(moder);
            let pos = (self.pin as u32) * 2;
            val &= !(0b11 << pos);
            val |= (mode as u32) << pos;
            core::ptr::write_volatile(moder, val);
        }
    }

    /// Set output type
    /// Reference: RM0456 Section 13.4.2
    pub fn set_output_type(&self, otype: OutputType) {
        unsafe {
            let otyper = (self.port_base() + reg::OTYPER) as *mut u32;
            let mut val = core::ptr::read_volatile(otyper);
            if otype == OutputType::OpenDrain {
                val |= 1 << self.pin;
            } else {
                val &= !(1 << self.pin);
            }
            core::ptr::write_volatile(otyper, val);
        }
    }

    /// Set output speed
    /// Reference: RM0456 Section 13.4.3
    pub fn set_speed(&self, speed: Speed) {
        unsafe {
            let ospeedr = (self.port_base() + reg::OSPEEDR) as *mut u32;
            let mut val = core::ptr::read_volatile(ospeedr);
            let pos = (self.pin as u32) * 2;
            val &= !(0b11 << pos);
            val |= (speed as u32) << pos;
            core::ptr::write_volatile(ospeedr, val);
        }
    }

    /// Set pull-up/pull-down
    /// Reference: RM0456 Section 13.4.4
    pub fn set_pull(&self, pull: Pull) {
        unsafe {
            let pupdr = (self.port_base() + reg::PUPDR) as *mut u32;
            let mut val = core::ptr::read_volatile(pupdr);
            let pos = (self.pin as u32) * 2;
            val &= !(0b11 << pos);
            val |= (pull as u32) << pos;
            core::ptr::write_volatile(pupdr, val);
        }
    }

    /// Set alternate function
    /// Reference: RM0456 Section 13.4.9
    pub fn set_alternate_function(&self, af: AlternateFunction) {
        unsafe {
            let af = af as u32;
            if self.pin < 8 {
                let afrl = (self.port_base() + reg::AFRL) as *mut u32;
                let mut val = core::ptr::read_volatile(afrl);
                let pos = (self.pin as u32) * 4;
                val &= !(0b1111 << pos);
                val |= af << pos;
                core::ptr::write_volatile(afrl, val);
            } else {
                let afrh = (self.port_base() + reg::AFRH) as *mut u32;
                let mut val = core::ptr::read_volatile(afrh);
                let pos = ((self.pin - 8) as u32) * 4;
                val &= !(0b1111 << pos);
                val |= af << pos;
                core::ptr::write_volatile(afrh, val);
            }
        }
    }

    /// Read pin input state
    /// Reference: RM0456 Section 13.4.5
    pub fn read(&self) -> bool {
        unsafe {
            let idr = (self.port_base() + reg::IDR) as *mut u32;
            let val = core::ptr::read_volatile(idr);
            (val & (1 << self.pin)) != 0
        }
    }

    /// Write pin output state
    /// Reference: RM0456 Section 13.4.7
    pub fn write(&self, high: bool) {
        unsafe {
            let bsrr = (self.port_base() + reg::BSRR) as *mut u32;
            if high {
                core::ptr::write_volatile(bsrr, 1 << self.pin);
            } else {
                core::ptr::write_volatile(bsrr, 1 << (self.pin + 16));
            }
        }
    }

    /// Toggle pin output
    /// Reference: RM0456 Section 13.4.6
    pub fn toggle(&self) {
        unsafe {
            let odr = (self.port_base() + reg::ODR) as *mut u32;
            let val = core::ptr::read_volatile(odr);
            let new_val = val ^ (1 << self.pin);
            core::ptr::write_volatile(odr, new_val);
        }
    }

    /// Set pin high
    pub fn set_high(&self) {
        self.write(true);
    }

    /// Set pin low
    pub fn set_low(&self) {
        self.write(false);
    }

    /// Check if pin is high
    pub fn is_high(&self) -> bool {
        self.read()
    }

    /// Check if pin is low
    pub fn is_low(&self) -> bool {
        !self.read()
    }
}

/// Initialize GPIO ports and clocks
/// Reference: RM0456 Section 13.2 and Chapter 11
pub fn init() {
    // Enable GPIO clocks in RCC
    // Reference: RM0456 Section 11.10.5: RCC_AHB2ENR
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOA);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOB);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOC);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOD);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOE);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOF);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOG);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOH);
    crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::GPIOI);
}

/// Predefined pins for common use
pub mod pins {
    use super::{Pin, Port};

    /// GPIOA pins (PA0-PA15)
    pub const PA0: Pin = Pin::new(Port::PortA, 0);
    pub const PA1: Pin = Pin::new(Port::PortA, 1);
    pub const PA2: Pin = Pin::new(Port::PortA, 2);
    pub const PA3: Pin = Pin::new(Port::PortA, 3);
    pub const PA4: Pin = Pin::new(Port::PortA, 4);
    pub const PA5: Pin = Pin::new(Port::PortA, 5);
    pub const PA6: Pin = Pin::new(Port::PortA, 6);
    pub const PA7: Pin = Pin::new(Port::PortA, 7);
    pub const PA8: Pin = Pin::new(Port::PortA, 8);
    pub const PA9: Pin = Pin::new(Port::PortA, 9);
    pub const PA10: Pin = Pin::new(Port::PortA, 10);
    pub const PA11: Pin = Pin::new(Port::PortA, 11);
    pub const PA12: Pin = Pin::new(Port::PortA, 12);
    pub const PA13: Pin = Pin::new(Port::PortA, 13);
    pub const PA14: Pin = Pin::new(Port::PortA, 14);
    pub const PA15: Pin = Pin::new(Port::PortA, 15);

    /// GPIOB pins (PB0-PB15)
    pub const PB0: Pin = Pin::new(Port::PortB, 0);
    pub const PB1: Pin = Pin::new(Port::PortB, 1);
    pub const PB2: Pin = Pin::new(Port::PortB, 2);
    pub const PB3: Pin = Pin::new(Port::PortB, 3);
    pub const PB4: Pin = Pin::new(Port::PortB, 4);
    pub const PB5: Pin = Pin::new(Port::PortB, 5);
    pub const PB6: Pin = Pin::new(Port::PortB, 6);
    pub const PB7: Pin = Pin::new(Port::PortB, 7);
    pub const PB8: Pin = Pin::new(Port::PortB, 8);
    pub const PB9: Pin = Pin::new(Port::PortB, 9);
    pub const PB10: Pin = Pin::new(Port::PortB, 10);
    pub const PB11: Pin = Pin::new(Port::PortB, 11);
    pub const PB12: Pin = Pin::new(Port::PortB, 12);
    pub const PB13: Pin = Pin::new(Port::PortB, 13);
    pub const PB14: Pin = Pin::new(Port::PortB, 14);
    pub const PB15: Pin = Pin::new(Port::PortB, 15);

    /// GPIOC pins (PC0-PC15)
    pub const PC0: Pin = Pin::new(Port::PortC, 0);
    pub const PC1: Pin = Pin::new(Port::PortC, 1);
    pub const PC2: Pin = Pin::new(Port::PortC, 2);
    pub const PC3: Pin = Pin::new(Port::PortC, 3);
    pub const PC4: Pin = Pin::new(Port::PortC, 4);
    pub const PC5: Pin = Pin::new(Port::PortC, 5);
    pub const PC6: Pin = Pin::new(Port::PortC, 6);
    pub const PC7: Pin = Pin::new(Port::PortC, 7);
    pub const PC8: Pin = Pin::new(Port::PortC, 8);
    pub const PC9: Pin = Pin::new(Port::PortC, 9);
    pub const PC10: Pin = Pin::new(Port::PortC, 10);
    pub const PC11: Pin = Pin::new(Port::PortC, 11);
    pub const PC12: Pin = Pin::new(Port::PortC, 12);
    pub const PC13: Pin = Pin::new(Port::PortC, 13);
    pub const PC14: Pin = Pin::new(Port::PortC, 14);
    pub const PC15: Pin = Pin::new(Port::PortC, 15);

    /// GPIOD pins (PD0-PD15)
    pub const PD0: Pin = Pin::new(Port::PortD, 0);
    pub const PD1: Pin = Pin::new(Port::PortD, 1);
    pub const PD2: Pin = Pin::new(Port::PortD, 2);
    pub const PD3: Pin = Pin::new(Port::PortD, 3);
    pub const PD4: Pin = Pin::new(Port::PortD, 4);
    pub const PD5: Pin = Pin::new(Port::PortD, 5);
    pub const PD6: Pin = Pin::new(Port::PortD, 6);
    pub const PD7: Pin = Pin::new(Port::PortD, 7);
    pub const PD8: Pin = Pin::new(Port::PortD, 8);
    pub const PD9: Pin = Pin::new(Port::PortD, 9);
    pub const PD10: Pin = Pin::new(Port::PortD, 10);
    pub const PD11: Pin = Pin::new(Port::PortD, 11);
    pub const PD12: Pin = Pin::new(Port::PortD, 12);
    pub const PD13: Pin = Pin::new(Port::PortD, 13);
    pub const PD14: Pin = Pin::new(Port::PortD, 14);
    pub const PD15: Pin = Pin::new(Port::PortD, 15);

    /// GPIOE pins (PE0-PE15)
    pub const PE0: Pin = Pin::new(Port::PortE, 0);
    pub const PE1: Pin = Pin::new(Port::PortE, 1);
    pub const PE2: Pin = Pin::new(Port::PortE, 2);
    pub const PE3: Pin = Pin::new(Port::PortE, 3);
    pub const PE4: Pin = Pin::new(Port::PortE, 4);
    pub const PE5: Pin = Pin::new(Port::PortE, 5);
    pub const PE6: Pin = Pin::new(Port::PortE, 6);
    pub const PE7: Pin = Pin::new(Port::PortE, 7);
    pub const PE8: Pin = Pin::new(Port::PortE, 8);
    pub const PE9: Pin = Pin::new(Port::PortE, 9);
    pub const PE10: Pin = Pin::new(Port::PortE, 10);
    pub const PE11: Pin = Pin::new(Port::PortE, 11);
    pub const PE12: Pin = Pin::new(Port::PortE, 12);
    pub const PE13: Pin = Pin::new(Port::PortE, 13);
    pub const PE14: Pin = Pin::new(Port::PortE, 14);
    pub const PE15: Pin = Pin::new(Port::PortE, 15);

    /// GPIOF pins (PF0-PF15)
    pub const PF0: Pin = Pin::new(Port::PortF, 0);
    pub const PF1: Pin = Pin::new(Port::PortF, 1);
    pub const PF2: Pin = Pin::new(Port::PortF, 2);
    pub const PF3: Pin = Pin::new(Port::PortF, 3);
    pub const PF4: Pin = Pin::new(Port::PortF, 4);
    pub const PF5: Pin = Pin::new(Port::PortF, 5);
    pub const PF6: Pin = Pin::new(Port::PortF, 6);
    pub const PF7: Pin = Pin::new(Port::PortF, 7);
    pub const PF8: Pin = Pin::new(Port::PortF, 8);
    pub const PF9: Pin = Pin::new(Port::PortF, 9);
    pub const PF10: Pin = Pin::new(Port::PortF, 10);
    pub const PF11: Pin = Pin::new(Port::PortF, 11);
    pub const PF12: Pin = Pin::new(Port::PortF, 12);
    pub const PF13: Pin = Pin::new(Port::PortF, 13);
    pub const PF14: Pin = Pin::new(Port::PortF, 14);
    pub const PF15: Pin = Pin::new(Port::PortF, 15);

    /// GPIOG pins (PG0-PG15)
    pub const PG0: Pin = Pin::new(Port::PortG, 0);
    pub const PG1: Pin = Pin::new(Port::PortG, 1);
    pub const PG2: Pin = Pin::new(Port::PortG, 2);
    pub const PG3: Pin = Pin::new(Port::PortG, 3);
    pub const PG4: Pin = Pin::new(Port::PortG, 4);
    pub const PG5: Pin = Pin::new(Port::PortG, 5);
    pub const PG6: Pin = Pin::new(Port::PortG, 6);
    pub const PG7: Pin = Pin::new(Port::PortG, 7);
    pub const PG8: Pin = Pin::new(Port::PortG, 8);
    pub const PG9: Pin = Pin::new(Port::PortG, 9);
    pub const PG10: Pin = Pin::new(Port::PortG, 10);
    pub const PG11: Pin = Pin::new(Port::PortG, 11);
    pub const PG12: Pin = Pin::new(Port::PortG, 12);
    pub const PG13: Pin = Pin::new(Port::PortG, 13);
    pub const PG14: Pin = Pin::new(Port::PortG, 14);
    pub const PG15: Pin = Pin::new(Port::PortG, 15);

    /// GPIOH pins (PH0-PH15)
    pub const PH0: Pin = Pin::new(Port::PortH, 0);
    pub const PH1: Pin = Pin::new(Port::PortH, 1);
    pub const PH2: Pin = Pin::new(Port::PortH, 2);
    pub const PH3: Pin = Pin::new(Port::PortH, 3);
    pub const PH4: Pin = Pin::new(Port::PortH, 4);
    pub const PH5: Pin = Pin::new(Port::PortH, 5);
    pub const PH6: Pin = Pin::new(Port::PortH, 6);
    pub const PH7: Pin = Pin::new(Port::PortH, 7);
    pub const PH8: Pin = Pin::new(Port::PortH, 8);
    pub const PH9: Pin = Pin::new(Port::PortH, 9);
    pub const PH10: Pin = Pin::new(Port::PortH, 10);
    pub const PH11: Pin = Pin::new(Port::PortH, 11);
    pub const PH12: Pin = Pin::new(Port::PortH, 12);
    pub const PH13: Pin = Pin::new(Port::PortH, 13);
    pub const PH14: Pin = Pin::new(Port::PortH, 14);
    pub const PH15: Pin = Pin::new(Port::PortH, 15);

    /// GPIOI pins (PI0-PI15)
    pub const PI0: Pin = Pin::new(Port::PortI, 0);
    pub const PI1: Pin = Pin::new(Port::PortI, 1);
    pub const PI2: Pin = Pin::new(Port::PortI, 2);
    pub const PI3: Pin = Pin::new(Port::PortI, 3);
    pub const PI4: Pin = Pin::new(Port::PortI, 4);
    pub const PI5: Pin = Pin::new(Port::PortI, 5);
    pub const PI6: Pin = Pin::new(Port::PortI, 6);
    pub const PI7: Pin = Pin::new(Port::PortI, 7);
    pub const PI8: Pin = Pin::new(Port::PortI, 8);
    pub const PI9: Pin = Pin::new(Port::PortI, 9);
    pub const PI10: Pin = Pin::new(Port::PortI, 10);
    pub const PI11: Pin = Pin::new(Port::PortI, 11);
    pub const PI12: Pin = Pin::new(Port::PortI, 12);
    pub const PI13: Pin = Pin::new(Port::PortI, 13);
    pub const PI14: Pin = Pin::new(Port::PortI, 14);
    pub const PI15: Pin = Pin::new(Port::PortI, 15);
}
