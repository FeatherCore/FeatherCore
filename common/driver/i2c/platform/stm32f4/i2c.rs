//! STM32F4 I2C Driver
//! STM32F4 系列 I2C 驱动
//!
//! This module provides the platform-specific implementation for STM32F4 series I2C.
//! It implements the I2cDriver trait defined in the common layer.
//! 该模块为 STM32F4 系列 I2C 提供平台特定实现。它实现了通用层定义的 I2cDriver 特征。

use crate::driver::i2c::{I2cConfig, I2cDriver, I2cError};
use crate::device_tree::DeviceTree;

pub mod stm32f4xx {
    //! STM32F4xx I2C Registers
    //! STM32F4xx I2C 寄存器定义
    //!
    //! This module defines the I2C register structures for STM32F4xx microcontrollers.
    //! 该模块定义了 STM32F4xx 微控制器的 I2C 寄存器结构。

    /// I2C Register Map
    /// I2C 寄存器映射
    #[repr(C)]
    pub struct I2cRegs {
        /// CR1 - Control Register 1 / 控制寄存器 1
        pub cr1: u32,
        /// CR2 - Control Register 2 / 控制寄存器 2
        pub cr2: u32,
        /// OAR1 - Own Address Register 1 / 自身地址寄存器 1
        pub oar1: u32,
        /// OAR2 - Own Address Register 2 / 自身地址寄存器 2
        pub oar2: u32,
        /// DR - Data Register / 数据寄存器
        pub dr: u32,
        /// SR1 - Status Register 1 / 状态寄存器 1
        pub sr1: u32,
        /// SR2 - Status Register 2 / 状态寄存器 2
        pub sr2: u32,
        /// CCR - Clock Control Register / 时钟控制寄存器
        pub ccr: u32,
        /// TRISE - TRISE Register / 上升时间寄存器
        pub trise: u32,
        /// FLTR - FLTR Register / 滤波器寄存器
        pub fltr: u32,
    }

    /// I2C Control Register 1 (CR1) bit definitions
    /// I2C 控制寄存器 1 (CR1) 位定义
    pub const I2C_CR1_PE: u32 = 1 << 0;       // Peripheral enable / 外设使能
    pub const I2C_CR1_SMBUS: u32 = 1 << 1;    // SMBus mode / SMBus 模式
    pub const I2C_CR1_SMBTYPE: u32 = 1 << 3;  // SMBus type / SMBus 类型
    pub const I2C_CR1_ENARP: u32 = 1 << 4;    // ARP enable / ARP 使能
    pub const I2C_CR1_ENPEC: u32 = 1 << 5;    // PEC enable / PEC 使能
    pub const I2C_CR1_ENGC: u32 = 1 << 6;     // General call enable / 广播呼叫使能
    pub const I2C_CR1_NOSTRETCH: u32 = 1 << 7; // Clock stretching disable / 时钟拉伸禁用
    pub const I2C_CR1_START: u32 = 1 << 8;    // Start generation / 开始生成
    pub const I2C_CR1_STOP: u32 = 1 << 9;     // Stop generation / 停止生成
    pub const I2C_CR1_ACK: u32 = 1 << 10;     // Acknowledge enable / 确认使能
    pub const I2C_CR1_POS: u32 = 1 << 11;     // Acknowledge/PEC Position / 确认/PEC 位置
    pub const I2C_CR1_PEC: u32 = 1 << 12;     // Packet error checking / 数据包错误检查
    pub const I2C_CR1_ALERT: u32 = 1 << 13;   // SMBus alert / SMBus 警报
    pub const I2C_CR1_SWRST: u32 = 1 << 15;   // Software reset / 软件重置

    /// I2C Control Register 2 (CR2) bit definitions
    /// I2C 控制寄存器 2 (CR2) 位定义
    pub const I2C_CR2_FREQ: u32 = 0x3F;       // Peripheral clock frequency / 外设时钟频率 (bits 5:0)
    pub const I2C_CR2_ITERREN: u32 = 1 << 8;  // Error interrupt enable / 错误中断使能
    pub const I2C_CR2_ITEVTEN: u32 = 1 << 9;  // Event interrupt enable / 事件中断使能
    pub const I2C_CR2_ITBUFEN: u32 = 1 << 10; // Buffer interrupt enable / 缓冲区中断使能
    pub const I2C_CR2_DMAEN: u32 = 1 << 11;   // DMA requests enable / DMA 请求使能
    pub const I2C_CR2_LAST: u32 = 1 << 12;    // DMA last transfer / DMA 最后传输

    /// I2C Status Register 1 (SR1) bit definitions
    /// I2C 状态寄存器 1 (SR1) 位定义
    pub const I2C_SR1_SB: u32 = 1 << 0;       // Start bit / 开始位
    pub const I2C_SR1_ADDR: u32 = 1 << 1;     // Address sent / 地址已发送
    pub const I2C_SR1_BTF: u32 = 1 << 2;      // Byte transfer finished / 字节传输完成
    pub const I2C_SR1_ADD10: u32 = 1 << 3;    // 10-bit header sent / 10位头部已发送
    pub const I2C_SR1_STOPF: u32 = 1 << 4;    // Stop detection / 停止检测
    pub const I2C_SR1_RXNE: u32 = 1 << 6;     // Data register not empty / 数据寄存器非空
    pub const I2C_SR1_TXE: u32 = 1 << 7;      // Data register empty / 数据寄存器空
    pub const I2C_SR1_BERR: u32 = 1 << 8;      // Bus error / 总线错误
    pub const I2C_SR1_ARLO: u32 = 1 << 9;     // Arbitration lost / 仲裁丢失
    pub const I2C_SR1_AF: u32 = 1 << 10;      // Acknowledge failure / 确认失败
    pub const I2C_SR1_OVR: u32 = 1 << 11;      // Overrun / 溢出
    pub const I2C_SR1_PECERR: u32 = 1 << 12;   // PEC Error in reception / 接收 PEC 错误
    pub const I2C_SR1_TIMEOUT: u32 = 1 << 14;  // Timeout / 超时
    pub const I2C_SR1_SMBALERT: u32 = 1 << 15; // SMBus alert / SMBus 警报

    /// I2C Status Register 2 (SR2) bit definitions
    /// I2C 状态寄存器 2 (SR2) 位定义
    pub const I2C_SR2_MSL: u32 = 1 << 0;       // Master/slave / 主/从
    pub const I2C_SR2_BUSY: u32 = 1 << 1;     // Bus busy / 总线忙
    pub const I2C_SR2_TRA: u32 = 1 << 2;      // Transmitter/receiver / 发送器/接收器
    pub const I2C_SR2_GENCALL: u32 = 1 << 4;   // General call address (Slave mode) / 广播呼叫地址 (从模式)
    pub const I2C_SR2_SMBDEFAULT: u32 = 1 << 5; // SMBus device default address / SMBus 设备默认地址
    pub const I2C_SR2_SMBHOST: u32 = 1 << 6;  // SMBus host header (Slave mode) / SMBus 主机头部 (从模式)
    pub const I2C_SR2_DUALF: u32 = 1 << 7;    // Dual flag (Slave mode) / 双标志 (从模式)
    pub const I2C_SR2_PEC: u32 = 0xFF << 8;   // Packet error checking register / 数据包错误检查寄存器 (bits 15:8)

    /// I2C Clock Control Register (CCR) bit definitions
    /// I2C 时钟控制寄存器 (CCR) 位定义
    pub const I2C_CCR_CCR: u32 = 0x0FFF;      // Clock control register / 时钟控制寄存器 (bits 11:0)
    pub const I2C_CCR_DUTY: u32 = 1 << 14;     // Fast mode duty cycle / 快速模式占空比
    pub const I2C_CCR_FS: u32 = 1 << 15;      // I2C master mode selection / I2C 主机模式选择
}

/// STM32F4 I2C Driver Configuration
/// STM32F4 I2C 驱动配置
#[derive(Debug, Clone, Copy)]
pub struct Stm32f4I2cConfig {
    /// I2C base address / I2C 基地址
    pub base_address: u32,
    /// I2C interrupt number / I2C 中断号
    pub irq: u32,
    /// I2C clock frequency (Hz) / I2C 时钟频率 (Hz)
    pub clock_frequency: u32,
}

impl Stm32f4I2cConfig {
    /// Create a new configuration from device tree
    /// 从设备树创建新配置
    ///
    /// # Arguments / 参数
    /// * `dt` - Device tree / 设备树
    /// * `node_name` - I2C node name in device tree / 设备树中的 I2C 节点名称
    ///
    /// # Returns / 返回
    /// * `Result<Self, I2cError>` - Configuration or error / 配置或错误
    pub fn from_device_tree(dt: &DeviceTree, node_name: &str) -> Result<Self, I2cError> {
        let node = dt.get_node(node_name)
            .ok_or(I2cError::NotSupported)?;

        let reg = node.get_property("reg")
            .and_then(|p| match &p.value {
                crate::device_tree::PropertyValue::IntegerArray(arr) if arr.len() >= 2 => {
                    Some((arr[0], arr[1]))
                }
                _ => None,
            })
            .ok_or(I2cError::InvalidConfig)?;

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

        Ok(Stm32f4I2cConfig {
            base_address: reg.0,
            irq,
            clock_frequency,
        })
    }
}

/// STM32F4 I2C Driver
/// STM32F4 I2C 驱动
///
/// # Example / 示例
/// ```ignore
/// use driver::i2c::platform::stm32f4::Stm32f4I2c;
///
/// let mut i2c = Stm32f4I2c::new(config);
/// i2c.init(&I2cConfig::default())?;
/// i2c.write(0x50, &[0x00, 0x01])?;
/// ```
pub struct Stm32f4I2c {
    /// Configuration / 配置
    config: Stm32f4I2cConfig,
    /// I2C configuration / I2C 配置
    i2c_config: Option<I2cConfig>,
    /// Is initialized / 是否已初始化
    initialized: bool,
}

impl Stm32f4I2c {
    /// Create a new STM32F4 I2C instance
    /// 创建新的 STM32F4 I2C 实例
    ///
    /// # Arguments / 参数
    /// * `config` - Platform configuration / 平台配置
    ///
    /// # Returns / 返回
    /// * `Self` - New instance / 新实例
    pub fn new(config: Stm32f4I2cConfig) -> Self {
        Stm32f4I2c {
            config,
            i2c_config: None,
            initialized: false,
        }
    }

    /// Get register base address / 获取寄存器基地址
    fn get_regs(&self) -> *const stm32f4xx::I2cRegs {
        self.config.base_address as *const stm32f4xx::I2cRegs
    }

    /// Calculate CCR value for I2C speed
    /// 计算 I2C 速度的 CCR 值
    ///
    /// # Arguments / 参数
    /// * `speed` - Desired I2C speed / 期望的 I2C 速度
    ///
    /// # Returns / 返回
    /// * `u32` - CCR value / CCR 值
    fn calc_ccr(&self, speed: crate::driver::i2c::I2cSpeed) -> u32 {
        let freq = self.config.clock_frequency / 1000000; // MHz
        let speed_hz = speed as u32;

        if speed_hz == 100000 { // Standard mode / 标准模式
            // CCR = (f_PCLK1) / (2 * f_SCL)
            (self.config.clock_frequency / (2 * speed_hz)) as u32
        } else { // Fast mode / 快速模式
            // Duty cycle 16/9
            let ccr = (self.config.clock_frequency * 9) / (25 * speed_hz);
            (ccr & 0x0FFF) | stm32f4xx::I2C_CCR_FS | stm32f4xx::I2C_CCR_DUTY
        }
    }

    /// Calculate TRISE value
    /// 计算 TRISE 值
    ///
    /// # Arguments / 参数
    /// * `speed` - I2C speed / I2C 速度
    ///
    /// # Returns / 返回
    /// * `u32` - TRISE value / TRISE 值
    fn calc_trise(&self, speed: crate::driver::i2c::I2cSpeed) -> u32 {
        let freq = self.config.clock_frequency / 1000000; // MHz
        
        if speed as u32 == 100000 { // Standard mode / 标准模式
            (freq + 1) as u32
        } else { // Fast mode / 快速模式
            (freq * 300 / 1000) + 1
        }
    }

    /// Enable I2C clock / 使能 I2C 时钟
    ///
    /// This function enables the I2C peripheral clock.
    /// 这个函数使能 I2C 外设时钟。
    ///
    /// Note: In a real implementation, this would modify RCC registers.
    /// 注意：在实际实现中，这会修改 RCC 寄存器。
    fn enable_clock(&self) {
        unsafe {
            // RCC base address for STM32F4 is 0x40023800
            // Enable I2C1 clock by setting the appropriate bit in APB1ENR
            let rcc = (0x40023800u32 + 0x40) as *mut u32;
            // I2C1 is on APB1, bit 21
            *rcc |= 1 << 21;
        }
    }

    /// Configure GPIO pins for I2C / 配置 I2C 的 GPIO 引脚
    ///
    /// This function configures the SCL and SDA pins for the I2C peripheral.
    /// 这个函数为 I2C 外设配置 SCL 和 SDA 引脚。
    ///
    /// Note: In a real implementation, this would configure GPIO registers.
    /// 注意：在实际实现中，这会配置 GPIO 寄存器。
    fn configure_gpio(&self) {
        // Note: GPIO configuration would be done here
        // This typically involves:
        // 1. Enabling GPIO clock
        // 2. Configuring SCL pin as alternate function open-drain
        // 3. Configuring SDA pin as alternate function open-drain
    }

    /// Wait for flag in SR1
    /// 等待 SR1 中的标志
    ///
    /// # Arguments / 参数
    /// * `flag` - Flag to wait for / 要等待的标志
    /// * `timeout` - Timeout in microseconds / 超时时间（微秒）
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Success or error / 成功或错误
    fn wait_flag(&self, flag: u32, timeout: u32) -> Result<(), I2cError> {
        let regs = self.get_regs();
        let start = core::time::Instant::now();

        unsafe {
            while ((*regs).sr1 & flag) == 0 {
                if start.elapsed().as_micros() > timeout as u128 {
                    return Err(I2cError::Timeout);
                }
            }
        }

        Ok(())
    }

    /// Send start condition / 发送开始条件
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Success or error / 成功或错误
    fn send_start(&self) -> Result<(), I2cError> {
        let regs = self.get_regs();

        // Generate start condition / 生成开始条件
        unsafe {
            (*regs).cr1 |= stm32f4xx::I2C_CR1_START;
        }

        // Wait for SB flag / 等待 SB 标志
        self.wait_flag(stm32f4xx::I2C_SR1_SB, 1000)?;

        Ok(())
    }

    /// Send address / 发送地址
    ///
    /// # Arguments / 参数
    /// * `addr` - Device address / 设备地址
    /// * `read` - True for read, false for write / 读取为 true，写入为 false
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Success or error / 成功或错误
    fn send_address(&self, addr: u16, read: bool) -> Result<(), I2cError> {
        let regs = self.get_regs();
        let address = if read {
            (addr << 1) | 1
        } else {
            addr << 1
        };

        // Send address / 发送地址
        unsafe {
            (*regs).dr = address as u32;
        }

        // Wait for ADDR flag / 等待 ADDR 标志
        self.wait_flag(stm32f4xx::I2C_SR1_ADDR, 1000)?;

        // Clear ADDR flag by reading SR1 and SR2 / 通过读取 SR1 和 SR2 清除 ADDR 标志
        unsafe {
            let _ = (*regs).sr1;
            let _ = (*regs).sr2;
        }

        Ok(())
    }

    /// Send stop condition / 发送停止条件
    ///
    /// # Returns / 返回
    /// * `Result<(), I2cError>` - Success or error / 成功或错误
    fn send_stop(&self) -> Result<(), I2cError> {
        let regs = self.get_regs();

        // Generate stop condition / 生成停止条件
        unsafe {
            (*regs).cr1 |= stm32f4xx::I2C_CR1_STOP;
        }

        Ok(())
    }
}

impl I2cDriver for Stm32f4I2c {
    fn init(&mut self, config: &I2cConfig) -> Result<(), I2cError> {
        if self.initialized {
            return Err(I2cError::AlreadyInitialized);
        }

        let regs = self.get_regs();

        // Enable clock / 使能时钟
        self.enable_clock();

        // Configure GPIO / 配置 GPIO
        self.configure_gpio();

        // Disable I2C before configuration / 在配置前禁用 I2C
        unsafe {
            (*regs).cr1 &= !stm32f4xx::I2C_CR1_PE;
        }

        // Configure CR2 (peripheral clock frequency) / 配置 CR2（外设时钟频率）
        let freq = self.config.clock_frequency / 1000000; // MHz
        if freq > 50 {
            return Err(I2cError::InvalidConfig);
        }

        unsafe {
            (*regs).cr2 &= !stm32f4xx::I2C_CR2_FREQ;
            (*regs).cr2 |= freq as u32;
        }

        // Configure CCR (clock control) / 配置 CCR（时钟控制）
        let ccr = self.calc_ccr(config.speed);
        unsafe {
            (*regs).ccr = ccr;
        }

        // Configure TRISE (rise time) / 配置 TRISE（上升时间）
        let trise = self.calc_trise(config.speed);
        unsafe {
            (*regs).trise = trise;
        }

        // Configure address mode / 配置地址模式
        unsafe {
            match config.address_mode {
                crate::driver::i2c::I2cAddressMode::SevenBit => {
                    (*regs).oar1 = 0x0000;
                }
                crate::driver::i2c::I2cAddressMode::TenBit => {
                    (*regs).oar1 = 1 << 15; // 10-bit address mode / 10位地址模式
                }
            }
        }

        // Enable ACK / 使能 ACK
        unsafe {
            (*regs).cr1 |= stm32f4xx::I2C_CR1_ACK;
        }

        // Enable I2C / 使能 I2C
        unsafe {
            (*regs).cr1 |= stm32f4xx::I2C_CR1_PE;
        }

        self.i2c_config = Some(*config);
        self.initialized = true;

        Ok(())
    }

    fn write(&mut self, addr: u16, data: &[u8]) -> Result<(), I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        let regs = self.get_regs();

        // Send start condition / 发送开始条件
        self.send_start()?;

        // Send address (write) / 发送地址（写入）
        self.send_address(addr, false)?;

        // Send data / 发送数据
        for &byte in data {
            // Wait for TXE / 等待 TXE
            self.wait_flag(stm32f4xx::I2C_SR1_TXE, 1000)?;

            // Send byte / 发送字节
            unsafe {
                (*regs).dr = byte as u32;
            }
        }

        // Wait for BTF (byte transfer finished) / 等待 BTF（字节传输完成）
        self.wait_flag(stm32f4xx::I2C_SR1_BTF, 1000)?;

        // Send stop condition / 发送停止条件
        self.send_stop()?;

        Ok(())
    }

    fn read(&mut self, addr: u16, buffer: &mut [u8]) -> Result<usize, I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        if buffer.is_empty() {
            return Ok(0);
        }

        let regs = self.get_regs();

        // Send start condition / 发送开始条件
        self.send_start()?;

        // Send address (read) / 发送地址（读取）
        self.send_address(addr, true)?;

        // Handle single byte read / 处理单字节读取
        if buffer.len() == 1 {
            // Disable ACK before reading / 读取前禁用 ACK
            unsafe {
                (*regs).cr1 &= !stm32f4xx::I2C_CR1_ACK;
            }

            // Send stop condition / 发送停止条件
            self.send_stop()?;

            // Wait for RXNE / 等待 RXNE
            self.wait_flag(stm32f4xx::I2C_SR1_RXNE, 1000)?;

            // Read data / 读取数据
            unsafe {
                buffer[0] = (*regs).dr as u8;
            }
        } else {
            // For multiple bytes / 对于多个字节
            for i in 0..buffer.len() {
                // Wait for RXNE / 等待 RXNE
                self.wait_flag(stm32f4xx::I2C_SR1_RXNE, 1000)?;

                // Read data / 读取数据
                unsafe {
                    buffer[i] = (*regs).dr as u8;
                }

                // Last byte handling / 最后字节处理
                if i == buffer.len() - 2 {
                    // Disable ACK before last byte / 最后字节前禁用 ACK
                    unsafe {
                        (*regs).cr1 &= !stm32f4xx::I2C_CR1_ACK;
                    }

                    // Send stop condition / 发送停止条件
                    self.send_stop()?;
                }
            }
        }

        // Re-enable ACK for next operations / 为下一次操作重新使能 ACK
        unsafe {
            (*regs).cr1 |= stm32f4xx::I2C_CR1_ACK;
        }

        Ok(buffer.len())
    }

    fn write_read(&mut self, addr: u16, write_data: &[u8], read_buffer: &mut [u8]) -> Result<usize, I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        // Write phase / 写入阶段
        self.write(addr, write_data)?;

        // Read phase / 读取阶段
        self.read(addr, read_buffer)
    }

    fn is_bus_busy(&mut self) -> Result<bool, I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            Ok(((*regs).sr2 & stm32f4xx::I2C_SR2_BUSY) != 0)
        }
    }

    fn scan(&mut self) -> Result<Vec<u8>, I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        let mut found_addresses = Vec::new();

        // Scan 7-bit addresses (0x00 to 0x7F)
        // 扫描 7位地址（0x00 到 0x7F）
        for addr in 1..128 {
            if let Ok(_) = self.write(addr, &[]) {
                found_addresses.push(addr as u8);
            }
        }

        Ok(found_addresses)
    }

    fn enable(&mut self) -> Result<(), I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            (*regs).cr1 |= stm32f4xx::I2C_CR1_PE;
        }
        Ok(())
    }

    fn disable(&mut self) -> Result<(), I2cError> {
        if !self.initialized {
            return Err(I2cError::NotInitialized);
        }

        let regs = self.get_regs();
        unsafe {
            (*regs).cr1 &= !stm32f4xx::I2C_CR1_PE;
        }
        Ok(())
    }
}
