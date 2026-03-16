//! GPIO - General Purpose Input/Output
//! 通用输入输出
//!
//! STM32U5 GPIO 模块支持最多 9 个 GPIO 端口 (A-I)，每个端口最多 16 个引脚。
//! 支持多种模式：输入、输出、复用功能、模拟。

/// GPIO port base addresses
pub const GPIOA_BASE: usize = 0x4202_0000;
pub const GPIOB_BASE: usize = 0x4202_0400;
pub const GPIOC_BASE: usize = 0x4202_0800;
pub const GPIOD_BASE: usize = 0x4202_0C00;
pub const GPIOE_BASE: usize = 0x4202_1000;
pub const GPIOF_BASE: usize = 0x4202_1400;
pub const GPIOG_BASE: usize = 0x4202_1800;
pub const GPIOH_BASE: usize = 0x4202_1C00;
pub const GPIOI_BASE: usize = 0x4202_2000;

/// GPIO register offsets
pub mod reg {
    /// GPIO port mode register
    pub const MODER: usize = 0x00;
    /// GPIO port output type register
    pub const OTYPER: usize = 0x04;
    /// GPIO port output speed register
    pub const OSPEEDR: usize = 0x08;
    /// GPIO port pull-up/pull-down register
    pub const PUPDR: usize = 0x0C;
    /// GPIO port input data register
    pub const IDR: usize = 0x10;
    /// GPIO port output data register
    pub const ODR: usize = 0x14;
    /// GPIO port bit set/reset register
    pub const BSRR: usize = 0x18;
    /// GPIO port configuration lock register
    pub const LCKR: usize = 0x1C;
    /// GPIO alternate function low register
    pub const AFRL: usize = 0x20;
    /// GPIO alternate function high register
    pub const AFRH: usize = 0x24;
    /// GPIO port bit reset register
    pub const BRR: usize = 0x28;
    /// GPIO port secure configuration register (TrustZone)
    pub const SECCFGR: usize = 0x30;
}

/// GPIO modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Input mode
    Input = 0b00,
    /// General purpose output mode
    Output = 0b01,
    /// Alternate function mode
    Alternate = 0b10,
    /// Analog mode
    Analog = 0b11,
}

/// GPIO output types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputType {
    /// Push-pull output
    PushPull = 0,
    /// Open-drain output
    OpenDrain = 1,
}

/// GPIO output speeds
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Speed {
    /// Low speed
    Low = 0b00,
    /// Medium speed
    Medium = 0b01,
    /// High speed
    High = 0b10,
    /// Very high speed
    VeryHigh = 0b11,
}

/// GPIO pull-up/pull-down configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pull {
    /// No pull-up or pull-down
    None = 0b00,
    /// Pull-up
    Up = 0b01,
    /// Pull-down
    Down = 0b10,
}

/// GPIO alternate functions
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

/// GPIO port enumeration
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Port {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
}

/// GPIO pin structure
#[derive(Clone, Copy, Debug)]
pub struct Pin {
    port: Port,
    pin: u8,
}

impl Pin {
    /// Create a new GPIO pin
    pub const fn new(port: Port, pin: u8) -> Self {
        assert!(pin < 16, "Pin number must be 0-15");
        Self { port, pin }
    }

    /// Get port base address
    fn port_base(&self) -> usize {
        match self.port {
            Port::A => GPIOA_BASE,
            Port::B => GPIOB_BASE,
            Port::C => GPIOC_BASE,
            Port::D => GPIOD_BASE,
            Port::E => GPIOE_BASE,
            Port::F => GPIOF_BASE,
            Port::G => GPIOG_BASE,
            Port::H => GPIOH_BASE,
            Port::I => GPIOI_BASE,
        }
    }

    /// Initialize pin as input
    pub fn init_input(&self, pull: Pull) {
        self.set_mode(Mode::Input);
        self.set_pull(pull);
    }

    /// Initialize pin as output
    pub fn init_output(&self, otype: OutputType, speed: Speed) {
        self.set_mode(Mode::Output);
        self.set_output_type(otype);
        self.set_speed(speed);
    }

    /// Initialize pin as alternate function
    pub fn init_alternate(&self, af: AlternateFunction, otype: OutputType, speed: Speed, pull: Pull) {
        self.set_mode(Mode::Alternate);
        self.set_alternate_function(af);
        self.set_output_type(otype);
        self.set_speed(speed);
        self.set_pull(pull);
    }

    /// Initialize pin as analog
    pub fn init_analog(&self) {
        self.set_mode(Mode::Analog);
    }

    /// Set pin mode
    pub fn set_mode(&self, mode: Mode) {
        unsafe {
            let moder = (self.port_base() + reg::MODER) as *mut u32;
            let mut val = core::ptr::read_volatile(moder);
            let pos = self.pin * 2;
            val &= !(0b11 << pos);
            val |= (mode as u32) << pos;
            core::ptr::write_volatile(moder, val);
        }
    }

    /// Set output type
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
    pub fn set_speed(&self, speed: Speed) {
        unsafe {
            let ospeedr = (self.port_base() + reg::OSPEEDR) as *mut u32;
            let mut val = core::ptr::read_volatile(ospeedr);
            let pos = self.pin * 2;
            val &= !(0b11 << pos);
            val |= (speed as u32) << pos;
            core::ptr::write_volatile(ospeedr, val);
        }
    }

    /// Set pull-up/pull-down
    pub fn set_pull(&self, pull: Pull) {
        unsafe {
            let pupdr = (self.port_base() + reg::PUPDR) as *mut u32;
            let mut val = core::ptr::read_volatile(pupdr);
            let pos = self.pin * 2;
            val &= !(0b11 << pos);
            val |= (pull as u32) << pos;
            core::ptr::write_volatile(pupdr, val);
        }
    }

    /// Set alternate function
    pub fn set_alternate_function(&self, af: AlternateFunction) {
        unsafe {
            let af = af as u32;
            if self.pin < 8 {
                let afrl = (self.port_base() + reg::AFRL) as *mut u32;
                let mut val = core::ptr::read_volatile(afrl);
                let pos = self.pin * 4;
                val &= !(0b1111 << pos);
                val |= af << pos;
                core::ptr::write_volatile(afrl, val);
            } else {
                let afrh = (self.port_base() + reg::AFRH) as *mut u32;
                let mut val = core::ptr::read_volatile(afrh);
                let pos = (self.pin - 8) * 4;
                val &= !(0b1111 << pos);
                val |= af << pos;
                core::ptr::write_volatile(afrh, val);
            }
        }
    }

    /// Read pin input state
    pub fn read(&self) -> bool {
        unsafe {
            let idr = (self.port_base() + reg::IDR) as *mut u32;
            let val = core::ptr::read_volatile(idr);
            (val & (1 << self.pin)) != 0
        }
    }

    /// Write pin output state
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

/// Initialize GPIO clock
pub fn init() {
    // Enable GPIO clocks in RCC
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

    // GPIOA pins
    pub const PA0: Pin = Pin::new(Port::A, 0);
    pub const PA1: Pin = Pin::new(Port::A, 1);
    pub const PA2: Pin = Pin::new(Port::A, 2);
    pub const PA3: Pin = Pin::new(Port::A, 3);
    pub const PA4: Pin = Pin::new(Port::A, 4);
    pub const PA5: Pin = Pin::new(Port::A, 5);
    pub const PA6: Pin = Pin::new(Port::A, 6);
    pub const PA7: Pin = Pin::new(Port::A, 7);
    pub const PA8: Pin = Pin::new(Port::A, 8);
    pub const PA9: Pin = Pin::new(Port::A, 9);
    pub const PA10: Pin = Pin::new(Port::A, 10);
    pub const PA11: Pin = Pin::new(Port::A, 11);
    pub const PA12: Pin = Pin::new(Port::A, 12);
    pub const PA13: Pin = Pin::new(Port::A, 13);
    pub const PA14: Pin = Pin::new(Port::A, 14);
    pub const PA15: Pin = Pin::new(Port::A, 15);

    // GPIOB pins
    pub const PB0: Pin = Pin::new(Port::B, 0);
    pub const PB1: Pin = Pin::new(Port::B, 1);
    pub const PB2: Pin = Pin::new(Port::B, 2);
    pub const PB3: Pin = Pin::new(Port::B, 3);
    pub const PB4: Pin = Pin::new(Port::B, 4);
    pub const PB5: Pin = Pin::new(Port::B, 5);
    pub const PB6: Pin = Pin::new(Port::B, 6);
    pub const PB7: Pin = Pin::new(Port::B, 7);
    pub const PB8: Pin = Pin::new(Port::B, 8);
    pub const PB9: Pin = Pin::new(Port::B, 9);
    pub const PB10: Pin = Pin::new(Port::B, 10);
    pub const PB11: Pin = Pin::new(Port::B, 11);
    pub const PB12: Pin = Pin::new(Port::B, 12);
    pub const PB13: Pin = Pin::new(Port::B, 13);
    pub const PB14: Pin = Pin::new(Port::B, 14);
    pub const PB15: Pin = Pin::new(Port::B, 15);

    // GPIOC pins
    pub const PC0: Pin = Pin::new(Port::C, 0);
    pub const PC1: Pin = Pin::new(Port::C, 1);
    pub const PC2: Pin = Pin::new(Port::C, 2);
    pub const PC3: Pin = Pin::new(Port::C, 3);
    pub const PC4: Pin = Pin::new(Port::C, 4);
    pub const PC5: Pin = Pin::new(Port::C, 5);
    pub const PC6: Pin = Pin::new(Port::C, 6);
    pub const PC7: Pin = Pin::new(Port::C, 7);
    pub const PC8: Pin = Pin::new(Port::C, 8);
    pub const PC9: Pin = Pin::new(Port::C, 9);
    pub const PC10: Pin = Pin::new(Port::C, 10);
    pub const PC11: Pin = Pin::new(Port::C, 11);
    pub const PC12: Pin = Pin::new(Port::C, 12);
    pub const PC13: Pin = Pin::new(Port::C, 13);
    pub const PC14: Pin = Pin::new(Port::C, 14);
    pub const PC15: Pin = Pin::new(Port::C, 15);
}
