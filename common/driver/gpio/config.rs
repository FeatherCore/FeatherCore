//! GPIO Driver Configuration
//! GPIO 驱动配置
//!
//! This module defines the GPIO configuration structures that are used
//! to configure the GPIO pin behavior.
//! 该模块定义了用于配置 GPIO 引脚行为的 GPIO 配置结构。

/// GPIO mode / GPIO 模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioMode {
    /// Input mode / 输入模式
    Input,
    /// Output push-pull mode / 输出推挽模式
    Output,
    /// Output open-drain mode / 输出开漏模式
    OutputOpenDrain,
    /// Alternate function push-pull mode / 复用功能推挽模式
    Alternate,
    /// Alternate function open-drain mode / 复用功能开漏模式
    AlternateOpenDrain,
    /// Analog mode / 模拟模式
    Analog,
}

impl Default for GpioMode {
    fn default() -> Self {
        GpioMode::Input
    }
}

/// GPIO speed / GPIO 速度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioSpeed {
    /// Low speed (2 MHz) / 低速 (2 MHz)
    Low,
    /// Medium speed (25 MHz) / 中速 (25 MHz)
    Medium,
    /// High speed (50 MHz) / 高速 (50 MHz)
    High,
    /// Very high speed (100 MHz) / 超高速 (100 MHz)
    VeryHigh,
}

impl Default for GpioSpeed {
    fn default() -> Self {
        GpioSpeed::Low
    }
}

/// GPIO pull mode / GPIO 上拉/下拉模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioPull {
    /// No pull / 无上拉/下拉
    None,
    /// Pull up / 上拉
    PullUp,
    /// Pull down / 下拉
    PullDown,
}

impl Default for GpioPull {
    fn default() -> Self {
        GpioPull::None
    }
}

/// GPIO configuration structure
/// GPIO 配置结构
///
/// # Example / 示例
/// ```ignore
/// let config = GpioConfig {
///     mode: GpioMode::Output,
///     speed: GpioSpeed::High,
///     pull: GpioPull::None,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpioConfig {
    /// GPIO mode / GPIO 模式
    pub mode: GpioMode,
    /// GPIO speed / GPIO 速度
    pub speed: GpioSpeed,
    /// GPIO pull mode / GPIO 上拉/下拉模式
    pub pull: GpioPull,
}

impl Default for GpioConfig {
    fn default() -> Self {
        GpioConfig {
            mode: GpioMode::default(),
            speed: GpioSpeed::default(),
            pull: GpioPull::default(),
        }
    }
}
