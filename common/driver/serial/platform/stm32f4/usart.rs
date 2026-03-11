//! STM32F4 USART Driver
//! STM32F4 系列 USART 驱动
//!
//! This module provides the platform-specific implementation for STM32F4 series USART.
//! It implements the SerialDriver trait defined in the common layer.
//! 该模块为 STM32F4 系列 USART 提供平台特定实现。它实现了通用层定义的 SerialDriver 特征。

use crate::driver::serial::{SerialConfig, SerialDriver, SerialError};
use crate::device_tree::DeviceTree;

pub mod stm32f4xx {
    //! STM32F4xx USART Registers
    //! STM32F4xx USART 寄存器定义
    //!
    //! This module defines the USART register structures for STM32F4xx microcontrollers.
    //! 这些定义了 STM模块32F4xx 微控制器的 USART 寄存器结构。

    /// USART Register Map
    /// USART 寄存器映射
    #[repr(C)]
    pub struct UsartRegs {
        /// SR - Status Register / 状态寄存器
        pub sr: u32,
        /// DR - Data Register / 数据寄存器
        pub dr: u32,
        /// BRR - Baud Rate Register / 波特率寄存器
        pub brr: u32,
        /// CR1 - Control Register 1 / 控制寄存器 1
        pub cr1: u32,
        /// CR2 - Control Register 2 / 控制寄存器 2
        pub cr2: u32,
        /// CR3 - Control Register 3 / 控制寄存器 3
        pub cr3: u32,
        /// GTPR - Guard Time and Prescaler Register / 保护时间和预分频器寄存器
        pub gtpr: u32,
    }

    /// USART Status Register (SR) bit definitions
    /// USART 状态寄存器 (SR) 位定义
    pub const USART_SR_PE: u32 = 1 << 0;      // Parity error / 校验错误
    pub const USART_SR_FE: u32 = 1 << 1;      // Framing error / 帧错误
    pub const USART_SR_NE: u32 = 1 << 2;       // Noise error / 噪声错误
    pub const USART_SR_ORE: u32 = 1 << 3;     // Overrun error / 溢出错误
    pub const USART_SR_IDLE: u32 = 1 << 4;    // IDLE line detected / 检测到空闲行
    pub const USART_SR_RXNE: u32 = 1 << 5;    // Read data register not empty / 读取数据寄存器非空
    pub const USART_SR_TC: u32 = 1 << 6;      // Transmission complete / 传输完成
    pub const USART_SR_TXE: u32 = 1 << 7;     // Transmit data register empty / 发送数据寄存器空
    pub const USART_SR_LBD: u32 = 1 << 8;     // LIN break detection / LIN 断开检测
    pub const USART_SR_CTS: u32 = 1 << 9;     // CTS flag / CTS 标志

    /// USART Control Register 1 (CR1) bit definitions
    /// USART 控制寄存器 1 (CR1) 位定义
    pub const USART_CR1_SBK: u32 = 1 << 0;    // Send break / 发送断开
    pub const USART_CR1_RWU: u32 = 1 << 1;    // Receiver wakeup / 接收器唤醒
    pub const USART_CR1_RE: u32 = 1 << 2;     // Receiver enable / 接收器使能
    pub const USART_CR1_TE: u32 = 1 << 3;     // Transmitter enable / 发送器使能
    pub const USART_CR1_IDLEIE: u32 = 1 << 4; // IDLE interrupt enable / 空闲中断使能
    pub const USART_CR1_RXNEIE: u32 = 1 << 5; // RXNE interrupt enable / RXNE 中断使能
    pub const USART_CR1_TCIE: u32 = 1 << 6;   // TC interrupt enable / TC 中断使能
    pub const USART_CR1_TXEIE: u32 = 1 << 7;  // TXE interrupt enable / TXE 中断使能
    pub const USART_CR1_PEIE: u32 = 1 << 8;   // PE interrupt enable / PE 中断使能
    pub const USART_CR1_PS: u32 = 1 << 9;     // Parity selection / 校验选择
    pub const USART_CR1_PCE: u32 = 1 << 10;   // Parity control enable / 校验控制使能
    pub const USART_CR1_WAKE: u32 = 1 << 11;  // Wakeup method / 唤醒方法
    pub const USART_CR1_M: u32 = 1 << 12;     // Word length / 字长
    pub const USART_CR1_UE: u32 = 1 << 13;    // USART enable / USART 使能
    pub const USART_CR1_OVER8: u32 = 1 << 15; // Oversampling mode / 过采样模式
}

/// STM32F4 USART Driver Configuration
/// STM32F4 USART 驱动配置
#[derive(Debug, Clone, Copy)]
pub struct Stm32f4UsartConfig {
    /// USART base address / USART 基地址
    pub base_address: u32,
    /// USART interrupt number / USART 中断号
    pub irq: u32,
    /// USART clock frequency (Hz) / USART 时钟频率 (Hz)
    pub clock_frequency: u32,
}

impl Stm32f4UsartConfig {
    /// Create a new configuration from device tree
    /// 从设备树创建新配置
    ///
    /// # Arguments / 参数
    /// * `dt` - Device tree / 设备树
    /// * `node_name` - USART node name in device tree / 设备树中的 USART 节点名称
    ///
    /// # Returns / 返回
    /// * `Result<Self, SerialError>` - Configuration or error / 配置或错误
    pub fn from_device_tree(dt: &DeviceTree, node_name: &str) -> Result<Self, SerialError> {
        let node = dt.get_node(node_name)
            .ok_or(SerialError::NotSupported)?;

        let reg = node.get_property("reg")
            .and_then(|p| match &p.value {
                crate::device_tree::PropertyValue::IntegerArray(arr) if arr.len() >= 2 => {
                    Some((arr[0], arr[1]))
                }
                _ => None,
            })
            .ok_or(SerialError::InvalidConfig)?;

        let irq = node.get_property("interrupts")
            .and_then(|p| match &p.value {
                crate::device_tree::PropertyValue::IntegerArray(arr) if !arr.is_empty() => {
                    Some(arr[0])
                }
                _ => None,
            })
            .unwrap_or(0);

        let clock_frequency = node.get_property("clock-frequency")
            .and_then(|p| match &p.value {
                crate::device_tree::PropertyValue::Integer(freq) => Some(*freq),
                _ => None,
            })
            .unwrap_or(45000000); // Default to 45MHz / 默认为 45MHz

        Ok(Stm32f4UsartConfig {
            base_address: reg.0,
            irq,
            clock_frequency,
        })
    }
}

/// STM32F4 USART Driver
/// STM32F4 USART 驱动
///
/// # Example / 示例
/// ```ignore
/// use driver::serial::platform::stm32f4::Stm32f4Usart;
///
/// let mut usart = Stm32f4Usart::new(config);
/// usart.init(&SerialConfig::default())?;
/// usart.write(b"Hello!")?;
/// ```
pub struct Stm32f4Usart {
    /// Configuration / 配置
    config: Stm32f4UsartConfig,
    /// Serial configuration / 串口配置
    serial_config: Option<SerialConfig>,
    /// Is initialized / 是否已初始化
    initialized: bool,
}

impl Stm32f4Usart {
    /// Create a new STM32F4 USART instance
    /// 创建新的 STM32F4 USART 实例
    ///
    /// # Arguments / 参数
    /// * `config` - Platform configuration / 平台配置
    ///
    /// # Returns / 返回
    /// * `Self` - New instance / 新实例
    pub fn new(config: Stm32f4UsartConfig) -> Self {
        Stm32f4Usart {
            config,
            serial_config: None,
            initialized: false,
        }
    }

    /// Get register base address / 获取寄存器基地址
    fn get_regs(&self) -> *const stm32f4xx::UsartRegs {
        self.config.base_address as *const stm32f4xx::UsartRegs
    }

    /// Calculate baud rate divisor / 计算波特率分频器
    ///
    /// # Arguments / 参数
    /// * `baud_rate` - Desired baud rate / 期望的波特率
    ///
    /// # Returns / 返回
    /// * `u16` - BRR value / BRR 值
    fn calc_baud_rate_div(&self, baud_rate: u32) -> u16 {
        // For STM32F4, oversampling by 16 (default)
        // Formula: BRR = USARTDIV = Clock / (16 * BaudRate)
        let usartdiv = self.config.clock_frequency / (16 * baud_rate);
        usartdiv as u16
    }

    /// Enable USART clock / 使能 USART 时钟
    ///
    /// This function enables the USART peripheral clock.
    /// 这个函数使能 USART 外设时钟。
    ///
    /// Note: In a real implementation, this would modify RCC registers.
    /// 注意：在实际实现中，这会修改 RCC 寄存器。
    fn enable_clock(&self) {
        unsafe {
            // RCC base address for STM32F4 is 0x40023800
            // Enable USART clock by setting the appropriate bit in APB2ENR
            let rcc = (0x40023800u32 + 0x44) as *mut u32;
            // USART1 is on APB2, bit 4
            *rcc |= 1 << 4;
        }
    }

    /// Configure GPIO pins for USART / 配置 USART 的 GPIO 引脚
    ///
    /// This function configures the TX and RX pins for the USART peripheral.
    /// 这个函数为 USART 外设配置 TX 和 RX 引脚。
    ///
    /// Note: In a real implementation, this would configure GPIO registers.
    /// 注意：在实际实现中，这会配置 GPIO 寄存器。
    fn configure_gpio(&self) {
        // Note: GPIO configuration would be done here
        // This typically involves:
        // 1. Enabling GPIO clock
        // 2. Configuring TX pin as alternate function push-pull
        // 3. Configuring RX pin as input floating
    }
}

impl SerialDriver for Stm32f4Usart {
    fn init(&mut self, config: &SerialConfig) -> Result<(), SerialError> {
        if self.initialized {
            return Err(SerialError::AlreadyInitialized);
        }

        // Validate baud rate / 验证波特率
        if config.baud_rate == 0 || config.baud_rate > 115200 {
            return Err(SerialError::InvalidBaudRate);
        }

        let regs = self.get_regs();

        // Enable clock / 使能时钟
        self.enable_clock();

        // Configure GPIO / 配置 GPIO
        self.configure_gpio();

        // Disable USART before configuration / 在配置前禁用 USART
        unsafe {
            (*regs).cr1 &= !stm32f4xx::USART_CR1_UE;
        }

        // Configure word length (M bit) / 配置字长 (M 位)
        // 0: 8 data bits, 1: 9 data bits
        unsafe {
            match config.data_bits {
                crate::driver::serial::DataBits::DataBits8 => {
                    (*regs).cr1 &= !stm32f4xx::USART_CR1_M;
                }
                crate::driver::serial::DataBits::DataBits9 => {
                    (*regs).cr1 |= stm32f4xx::USART_CR1_M;
                }
                _ => return Err(SerialError::InvalidConfig),
            }
        }

        // Configure parity / 配置校验
        unsafe {
            match config.parity {
                crate::driver::serial::Parity::None => {
                    (*regs).cr1 &= !stm32f4xx::USART_CR1_PCE;
                }
                crate::driver::serial::Parity::Even => {
                    (*regs).cr1 |= stm32f4xx::USART_CR1_PCE;
                    (*regs).cr1 &= !stm32f4xx::USART_CR1_PS;
                }
                crate::driver::serial::Parity::Odd => {
                    (*regs).cr1 |= stm32f4xx::USART_CR1_PCE;
                    (*regs).cr1 |= stm32f4xx::USART_CR1_PS;
                }
            }
        }

        // Configure stop bits / 配置停止位
        // CR2: bits 13:12 = 00: 1 stop bit, 10: 2 stop bits
        unsafe {
            match config.stop_bits {
                crate::driver::serial::StopBits::One => {
                    (*regs).cr2 &= !(0x3 << 12);
                }
                crate::driver::serial::StopBits::Two => {
                    (*regs).cr2 &= !(0x3 << 12);
                    (*regs).cr2 |= (0x2 << 12);
                }
            }
        }

        // Configure baud rate / 配置波特率
        let brr = self.calc_baud_rate_div(config.baud_rate);
        unsafe {
            (*regs).brr = brr as u32;
        }

        // Enable transmitter and receiver / 使能发送器和接收器
        unsafe {
            (*regs).cr1 |= stm32f4xx::USART_CR1_TE | stm32f4xx::USART_CR1_RE;
        }

        // Enable USART / 使能 USART
        unsafe {
            (*regs).cr1 |= stm32f4xx::USART_CR1_UE;
        }

        self.serial_config = Some(*config);
        self.initialized = true;

        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, SerialError> {
        if !self.initialized {
            return Err(SerialError::NotInitialized);
        }

        let regs = self.get_regs();
        let mut written = 0;

        for &byte in data {
            // Wait for TXE (transmit data register empty) / 等待 TXE (发送数据寄存器空)
            unsafe {
                while ((*regs).sr & stm32f4xx::USART_SR_TXE) == 0 {}
                (*regs).dr = byte as u32;
            }
            written += 1;
        }

        // Wait for TC (transmission complete) / 等待 TC (传输完成)
        unsafe {
            while ((*regs).sr & stm32f4xx::USART_SR_TC) == 0 {}
        }

        Ok(written)
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, SerialError> {
        if !self.initialized {
            return Err(SerialError::NotInitialized);
        }

        let regs = self.get_regs();
        let mut read = 0;

        for byte in buffer.iter_mut() {
            // Check if data is available / 检查是否有数据
            unsafe {
                if (*regs).sr & stm32f4xx::USART_SR_RXNE != 0 {
                    *byte = (*regs).dr as u8;
                    read += 1;
                } else {
                    break;
                }
            }
        }

        Ok(read)
    }

    fn is_data_available(&mut self) -> Result<bool, SerialError> {
        if !self.initialized {
            return Err(SerialError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            Ok(((*regs).sr & stm32f4xx::USART_SR_RXNE) != 0)
        }
    }

    fn flush(&mut self) -> Result<(), SerialError> {
        if !self.initialized {
            return Err(SerialError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            while ((*regs).sr & stm32f4xx::USART_SR_TC) == 0 {}
        }
        Ok(())
    }

    fn enable(&mut self) -> Result<(), SerialError> {
        if !self.initialized {
            return Err(SerialError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            (*regs).cr1 |= stm32f4xx::USART_CR1_UE;
        }
        Ok(())
    }

    fn disable(&mut self) -> Result<(), SerialError> {
        if !self.initialized {
            return Err(SerialError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            (*regs).cr1 &= !stm32f4xx::USART_CR1_UE;
        }
        Ok(())
    }
}
