//! Serial Driver Configuration
//! 串口驱动配置
//!
//! This module defines the serial configuration structures that are used
//! to configure the serial port behavior.
//! 该模块定义了用于配置串口行为的串口配置结构。

/// Serial data bits / 串口数据位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBits {
    /// 5 data bits / 5 数据位
    DataBits5 = 5,
    /// 6 data bits / 6 数据位
    DataBits6 = 6,
    /// 7 data bits / 7 数据位
    DataBits7 = 7,
    /// 8 data bits / 8 数据位
    DataBits8 = 8,
}

impl Default for DataBits {
    fn default() -> Self {
        DataBits::DataBits8
    }
}

/// Serial stop bits / 串口停止位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    /// 1 stop bit / 1 停止位
    One = 1,
    /// 2 stop bits / 2 停止位
    Two = 2,
}

impl Default for StopBits {
    fn default() -> Self {
        StopBits::One
    }
}

/// Serial parity / 串口校验位
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// No parity / 无校验
    None,
    /// Even parity / 偶校验
    Even,
    /// Odd parity / 奇校验
    Odd,
}

impl Default for Parity {
    fn default() -> Self {
        Parity::None
    }
}

/// Serial flow control / 串口流控制
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowControl {
    /// No flow control / 无流控制
    None,
    /// RTS/CTS flow control / RTS/CTS 流控制
    RtsCts,
    /// XON/XOFF flow control / XON/XOFF 流控制
    XonXoff,
}

impl Default for FlowControl {
    fn default() -> Self {
        FlowControl::None
    }
}

/// Serial configuration structure
/// 串口配置结构
///
/// # Example / 示例
/// ```ignore
/// let config = SerialConfig {
///     baud_rate: 115200,
///     data_bits: DataBits::DataBits8,
///     stop_bits: StopBits::One,
///     parity: Parity::None,
///     flow_control: FlowControl::None,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerialConfig {
    /// Baud rate / 波特率
    pub baud_rate: u32,
    /// Data bits / 数据位
    pub data_bits: DataBits,
    /// Stop bits / 停止位
    pub stop_bits: StopBits,
    /// Parity / 校验位
    pub parity: Parity,
    /// Flow control / 流控制
    pub flow_control: FlowControl,
}

impl Default for SerialConfig {
    fn default() -> Self {
        SerialConfig {
            baud_rate: 115200,
            data_bits: DataBits::default(),
            stop_bits: StopBits::default(),
            parity: Parity::default(),
            flow_control: FlowControl::default(),
        }
    }
}
