//! I2C Driver Configuration
//! I2C 驱动配置
//!
//! This module defines the I2C configuration structures that are used
//! to configure the I2C bus behavior.
//! 该模块定义了用于配置 I2C 总线行为的 I2C 配置结构。

/// I2C speed mode / I2C 速度模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cSpeed {
    /// Standard mode (100 kHz) / 标准模式 (100 kHz)
    Standard100KHz = 100000,
    /// Fast mode (400 kHz) / 快速模式 (400 kHz)
    Fast400KHz = 400000,
    /// Fast mode plus (1 MHz) / 快速模式+ (1 MHz)
    FastPlus1MHz = 1000000,
}

impl Default for I2cSpeed {
    fn default() -> Self {
        I2cSpeed::Standard100KHz
    }
}

/// I2C address mode / I2C 地址模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cAddressMode {
    /// 7-bit addressing / 7位地址
    SevenBit,
    /// 10-bit addressing / 10位地址
}

impl Default for I2cAddressMode {
    fn default() -> Self {
        I2cAddressMode::SevenBit
    }
}

/// I2C configuration structure
/// I2C 配置结构
///
/// # Example / 示例
/// ```ignore
/// let config = I2cConfig {
///     speed: I2cSpeed::Fast400KHz,
///     address_mode: I2cAddressMode::SevenBit,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2cConfig {
    /// I2C speed / I2C 速度
    pub speed: I2cSpeed,
    /// Address mode / 地址模式
    pub address_mode: I2cAddressMode,
}

impl Default for I2cConfig {
    fn default() -> Self {
        I2cConfig {
            speed: I2cSpeed::default(),
            address_mode: I2cAddressMode::default(),
        }
    }
}
