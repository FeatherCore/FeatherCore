//! STM32F4 GPIO Driver
//! STM32F4 系列 GPIO 驱动
//!
//! This module provides the platform-specific implementation for STM32F4 series GPIO.
//! It implements the GpioDriver and GpioPin traits defined in the common layer.
//! 该模块为 STM32F4 系列 GPIO 提供平台特定实现。它实现了通用层定义的 GpioDriver 和 GpioPin 特征。

use crate::driver::gpio::{GpioConfig, GpioDriver, GpioError, GpioMode, GpioPin, GpioPull, GpioSpeed};
use crate::device_tree::DeviceTree;

pub mod stm32f4xx {
    //! STM32F4xx GPIO Registers
    //! STM32F4xx GPIO 寄存器定义
    //!
    //! This module defines the GPIO register structures for STM32F4xx microcontrollers.
    //! 该模块定义了 STM32F4xx 微控制器的 GPIO 寄存器结构。

    /// GPIO Register Map
    /// GPIO 寄存器映射
    #[repr(C)]
    pub struct GpioRegs {
        /// MODER - Mode Register / 模式寄存器
        pub moder: u32,
        /// OTYPER - Output Type Register / 输出类型寄存器
        pub otyper: u32,
        /// OSPEEDR - Output Speed Register / 输出速度寄存器
        pub ospeedr: u32,
        /// PUPDR - Pull-Up/Pull-Down Register / 上拉/下拉寄存器
        pub pupdr: u32,
        /// IDR - Input Data Register / 输入数据寄存器
        pub idr: u32,
        /// ODR - Output Data Register / 输出数据寄存器
        pub odr: u32,
        /// BSRR - Bit Set/Reset Register / 位设置/重置寄存器
        pub bsrr: u32,
        /// LCKR - Configuration Lock Register / 配置锁定寄存器
        pub lckr: u32,
        /// AFR[0] - Alternate Function Low Register / 复用功能低寄存器
        pub afrl: u32,
        /// AFR[1] - Alternate Function High Register / 复用功能高寄存器
        pub afrh: u32,
    }

    /// GPIO port base addresses
    /// GPIO 端口基地址
    pub const GPIOA_BASE: u32 = 0x40020000;
    pub const GPIOB_BASE: u32 = 0x40020400;
    pub const GPIOC_BASE: u32 = 0x40020800;
    pub const GPIOD_BASE: u32 = 0x40020C00;
    pub const GPIOE_BASE: u32 = 0x40021000;
    pub const GPIOF_BASE: u32 = 0x40021400;
    pub const GPIOG_BASE: u32 = 0x40021800;
    pub const GPIOH_BASE: u32 = 0x40021C00;
    pub const GPIOI_BASE: u32 = 0x40022000;

    /// MODER mode values / MODER 模式值
    pub const MODER_INPUT: u32 = 0b00;
    pub const MODER_OUTPUT: u32 = 0b01;
    pub const MODER_ALTERNATE: u32 = 0b10;
    pub const MODER_ANALOG: u32 = 0b11;

    /// OTYPER type values / OTYPER 类型值
    pub const OTYPER_PP: u32 = 0b0;
    pub const OTYPER_OD: u32 = 0b1;

    /// OSPEEDR speed values / OSPEEDR 速度值
    pub const OSPEEDR_LOW: u32 = 0b00;
    pub const OSPEEDR_MEDIUM: u32 = 0b01;
    pub const OSPEEDR_HIGH: u32 = 0b10;
    pub const OSPEEDR_VERY_HIGH: u32 = 0b11;

    /// PUPDR pull values / PUPDR 上拉/下拉值
    pub const PUPDR_NONE: u32 = 0b00;
    pub const PUPDR_PULLUP: u32 = 0b01;
    pub const PUPDR_PULLDOWN: u32 = 0b10;

    /// BSRR bit positions / BSRR 位位置
    pub const BSRR_SET_POS: u32 = 0;
    pub const BSRR_RESET_POS: u32 = 16;
}

/// STM32F4 GPIO Driver Configuration
/// STM32F4 GPIO 驱动配置
#[derive(Debug, Clone, Copy)]
pub struct Stm32f4GpioConfig {
    /// Base addresses for each GPIO port / 每个 GPIO 端口的基地址
    pub port_bases: [u32; 9], // GPIOA-I
}

impl Stm32f4GpioConfig {
    /// Create a new configuration
    /// 创建新配置
    ///
    /// # Returns / 返回
    /// * `Self` - New configuration / 新配置
    pub fn new() -> Self {
        Stm32f4GpioConfig {
            port_bases: [
                stm32f4xx::GPIOA_BASE,
                stm32f4xx::GPIOB_BASE,
                stm32f4xx::GPIOC_BASE,
                stm32f4xx::GPIOD_BASE,
                stm32f4xx::GPIOE_BASE,
                stm32f4xx::GPIOF_BASE,
                stm32f4xx::GPIOG_BASE,
                stm32f4xx::GPIOH_BASE,
                stm32f4xx::GPIOI_BASE,
            ],
        }
    }

    /// Create a new configuration from device tree
    /// 从设备树创建新配置
    ///
    /// # Arguments / 参数
    /// * `dt` - Device tree / 设备树
    ///
    /// # Returns / 返回
    /// * `Result<Self, GpioError>` - Configuration or error / 配置或错误
    pub fn from_device_tree(_dt: &DeviceTree) -> Result<Self, GpioError> {
        Ok(Self::new())
    }
}

/// STM32F4 GPIO Pin
/// STM32F4 GPIO 引脚
pub struct Stm32f4GpioPin {
    /// Port index (0=A, 1=B, ..., 8=I) / 端口索引（0=A, 1=B, ..., 8=I）
    port_idx: usize,
    /// Pin number (0-15) / 引脚编号（0-15）
    pin: u8,
    /// GPIO register base address / GPIO 寄存器基地址
    base_address: u32,
    /// Pin configuration / 引脚配置
    config: GpioConfig,
}

impl Stm32f4GpioPin {
    /// Create a new GPIO pin
    /// 创建新的 GPIO 引脚
    ///
    /// # Arguments / 参数
    /// * `port_idx` - Port index / 端口索引
    /// * `pin` - Pin number / 引脚编号
    /// * `base_address` - GPIO register base address / GPIO 寄存器基地址
    ///
    /// # Returns / 返回
    /// * `Self` - New pin / 新引脚
    pub fn new(port_idx: usize, pin: u8, base_address: u32) -> Self {
        Stm32f4GpioPin {
            port_idx,
            pin,
            base_address,
            config: GpioConfig::default(),
        }
    }

    /// Get GPIO registers / 获取 GPIO 寄存器
    fn get_regs(&self) -> *const stm32f4xx::GpioRegs {
        self.base_address as *const stm32f4xx::GpioRegs
    }

    /// Get port name from index / 从索引获取端口名称
    fn port_name(&self) -> &str {
        match self.port_idx {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            7 => "H",
            8 => "I",
            _ => "A",
        }
    }

    /// Calculate pin number from port and pin / 从端口和引脚计算引脚编号
    fn calculate_pin_number(port_idx: usize, pin: u8) -> u16 {
        (port_idx as u16) * 16 + pin as u16
    }
}

impl GpioPin for Stm32f4GpioPin {
    fn set_mode(&mut self, mode: GpioMode) -> Result<(), GpioError> {
        let regs = self.get_regs();
        let pin_pos = (self.pin as u32) * 2;

        unsafe {
            // Clear current mode / 清除当前模式
            (*regs).moder &= !(0b11 << pin_pos);

            // Set new mode / 设置新模式
            match mode {
                GpioMode::Input => {
                    (*regs).moder |= (stm32f4xx::MODER_INPUT << pin_pos);
                    (*regs).otyper &= !(1 << self.pin); // PP
                }
                GpioMode::Output => {
                    (*regs).moder |= (stm32f4xx::MODER_OUTPUT << pin_pos);
                    (*regs).otyper &= !(1 << self.pin); // PP
                }
                GpioMode::OutputOpenDrain => {
                    (*regs).moder |= (stm32f4xx::MODER_OUTPUT << pin_pos);
                    (*regs).otyper |= (1 << self.pin); // OD
                }
                GpioMode::Alternate => {
                    (*regs).moder |= (stm32f4xx::MODER_ALTERNATE << pin_pos);
                    (*regs).otyper &= !(1 << self.pin); // PP
                }
                GpioMode::AlternateOpenDrain => {
                    (*regs).moder |= (stm32f4xx::MODER_ALTERNATE << pin_pos);
                    (*regs).otyper |= (1 << self.pin); // OD
                }
                GpioMode::Analog => {
                    (*regs).moder |= (stm32f4xx::MODER_ANALOG << pin_pos);
                }
            }
        }

        self.config.mode = mode;
        Ok(())
    }

    fn set_speed(&mut self, speed: GpioSpeed) -> Result<(), GpioError> {
        let regs = self.get_regs();
        let pin_pos = (self.pin as u32) * 2;

        unsafe {
            // Clear current speed / 清除当前速度
            (*regs).ospeedr &= !(0b11 << pin_pos);

            // Set new speed / 设置新速度
            match speed {
                GpioSpeed::Low => {
                    (*regs).ospeedr |= (stm32f4xx::OSPEEDR_LOW << pin_pos);
                }
                GpioSpeed::Medium => {
                    (*regs).ospeedr |= (stm32f4xx::OSPEEDR_MEDIUM << pin_pos);
                }
                GpioSpeed::High => {
                    (*regs).ospeedr |= (stm32f4xx::OSPEEDR_HIGH << pin_pos);
                }
                GpioSpeed::VeryHigh => {
                    (*regs).ospeedr |= (stm32f4xx::OSPEEDR_VERY_HIGH << pin_pos);
                }
            }
        }

        self.config.speed = speed;
        Ok(())
    }

    fn set_pull(&mut self, pull: GpioPull) -> Result<(), GpioError> {
        let regs = self.get_regs();
        let pin_pos = (self.pin as u32) * 2;

        unsafe {
            // Clear current pull / 清除当前上拉/下拉
            (*regs).pupdr &= !(0b11 << pin_pos);

            // Set new pull / 设置新上拉/下拉
            match pull {
                GpioPull::None => {
                    (*regs).pupdr |= (stm32f4xx::PUPDR_NONE << pin_pos);
                }
                GpioPull::PullUp => {
                    (*regs).pupdr |= (stm32f4xx::PUPDR_PULLUP << pin_pos);
                }
                GpioPull::PullDown => {
                    (*regs).pupdr |= (stm32f4xx::PUPDR_PULLDOWN << pin_pos);
                }
            }
        }

        self.config.pull = pull;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), GpioError> {
        let regs = self.get_regs();

        unsafe {
            (*regs).bsrr = 1 << self.pin;
        }

        Ok(())
    }

    fn set_low(&mut self) -> Result<(), GpioError> {
        let regs = self.get_regs();

        unsafe {
            (*regs).bsrr = 1 << (self.pin + 16);
        }

        Ok(())
    }

    fn toggle(&mut self) -> Result<(), GpioError> {
        let regs = self.get_regs();

        unsafe {
            let current_state = (*regs).odr & (1 << self.pin);
            if current_state != 0 {
                (*regs).bsrr = 1 << (self.pin + 16); // Reset
            } else {
                (*regs).bsrr = 1 << self.pin; // Set
            }
        }

        Ok(())
    }

    fn read(&self) -> Result<bool, GpioError> {
        let regs = self.get_regs();

        unsafe {
            let state = (*regs).idr & (1 << self.pin);
            Ok(state != 0)
        }
    }

    fn pin_number(&self) -> u16 {
        Stm32f4GpioPin::calculate_pin_number(self.port_idx, self.pin)
    }

    fn port(&self) -> &str {
        self.port_name()
    }
}

/// STM32F4 GPIO Driver
/// STM32F4 GPIO 驱动
///
/// # Example / 示例
/// ```ignore
/// use driver::gpio::platform::stm32f4::Stm32f4Gpio;
///
/// let mut gpio = Stm32f4Gpio::new(Stm32f4GpioConfig::new());
/// gpio.init()?;
///
/// let mut pin = gpio.get_pin_by_port("G", 13)?;
/// pin.set_mode(GpioMode::Output)?;
/// pin.set_high()?;
/// ```
pub struct Stm32f4Gpio {
    /// Configuration / 配置
    config: Stm32f4GpioConfig,
    /// Is initialized / 是否已初始化
    initialized: bool,
}

impl Stm32f4Gpio {
    /// Create a new STM32F4 GPIO instance
    /// 创建新的 STM32F4 GPIO 实例
    ///
    /// # Arguments / 参数
    /// * `config` - Platform configuration / 平台配置
    ///
    /// # Returns / 返回
    /// * `Self` - New instance / 新实例
    pub fn new(config: Stm32f4GpioConfig) -> Self {
        Stm32f4Gpio {
            config,
            initialized: false,
        }
    }

    /// Enable GPIO clock / 使能 GPIO 时钟
    ///
    /// # Arguments / 参数
    /// * `port_idx` - Port index / 端口索引
    ///
    /// # Returns / 返回
    /// * `Result<(), GpioError>` - Success or error / 成功或错误
    fn enable_port_clock(&self, port_idx: usize) -> Result<(), GpioError> {
        if port_idx >= 9 {
            return Err(GpioError::InvalidPort);
        }

        unsafe {
            // RCC base address for STM32F4 is 0x40023800
            // Enable GPIO clock by setting the appropriate bit in AHB1ENR
            let rcc = (0x40023800u32 + 0x30) as *mut u32;
            *rcc |= 1 << port_idx;
        }

        Ok(())
    }

    /// Convert port name to index / 将端口名称转换为索引
    ///
    /// # Arguments / 参数
    /// * `port` - Port name / 端口名称
    ///
    /// # Returns / 返回
    /// * `Result<usize, GpioError>` - Port index or error / 端口索引或错误
    fn port_to_index(&self, port: &str) -> Result<usize, GpioError> {
        match port.to_uppercase().as_str() {
            "A" => Ok(0),
            "B" => Ok(1),
            "C" => Ok(2),
            "D" => Ok(3),
            "E" => Ok(4),
            "F" => Ok(5),
            "G" => Ok(6),
            "H" => Ok(7),
            "I" => Ok(8),
            _ => Err(GpioError::InvalidPort),
        }
    }
}

impl GpioDriver for Stm32f4Gpio {
    fn init(&mut self) -> Result<(), GpioError> {
        if self.initialized {
            return Err(GpioError::AlreadyInitialized);
        }

        // Enable clocks for all GPIO ports / 使能所有 GPIO 端口的时钟
        for port_idx in 0..9 {
            self.enable_port_clock(port_idx)?;
        }

        self.initialized = true;
        Ok(())
    }

    fn get_pin(&mut self, pin_number: u16) -> Result<Box<dyn GpioPin>, GpioError> {
        if !self.initialized {
            return Err(GpioError::NotInitialized);
        }

        if pin_number >= 144 { // 9 ports * 16 pins = 144
            return Err(GpioError::InvalidPin);
        }

        let port_idx = (pin_number / 16) as usize;
        let pin = (pin_number % 16) as u8;

        self.enable_port_clock(port_idx)?;

        Ok(Box::new(Stm32f4GpioPin::new(
            port_idx,
            pin,
            self.config.port_bases[port_idx],
        )))
    }

    fn get_pin_by_port(&mut self, port: &str, pin: u8) -> Result<Box<dyn GpioPin>, GpioError> {
        if !self.initialized {
            return Err(GpioError::NotInitialized);
        }

        if pin > 15 {
            return Err(GpioError::InvalidPin);
        }

        let port_idx = self.port_to_index(port)?;

        self.enable_port_clock(port_idx)?;

        Ok(Box::new(Stm32f4GpioPin::new(
            port_idx,
            pin,
            self.config.port_bases[port_idx],
        )))
    }

    fn enable(&mut self) -> Result<(), GpioError> {
        if !self.initialized {
            return self.init();
        }
        Ok(())
    }

    fn disable(&mut self) -> Result<(), GpioError> {
        if !self.initialized {
            return Err(GpioError::NotInitialized);
        }

        self.initialized = false;
        Ok(())
    }
}
