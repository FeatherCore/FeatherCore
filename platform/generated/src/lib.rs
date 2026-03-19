//! FeatherCore Generated Library
//! FeatherCore 生成代码库
//!
//! This library contains generated code from device tree and other build-time sources.
//! 该库包含从设备树和其他构建时来源生成的代码。
//!
//! # Usage / 使用方法
//! ```ignore
//! use feathercore_common::generated;
//!
//! // Get chip configuration / 获取芯片配置
//! let chip_name = generated::chip::CHIP_NAME;
//!
//! // Get board configuration / 获取开发板配置
//! let board_name = generated::board::BOARD_NAME;
//!
//! // Get device tree info / 获取设备树信息
//! let dt = generated::device_tree::DeviceTree::new();
//! let compat = dt.compatible();
//! ```

#![no_std]

/// Chip configuration constants (from board config)
/// 芯片配置常量（来自板级配置）
pub mod chip {
    //! Chip Configuration / 芯片配置

    /// Chip name (e.g., "stm32f429vgt6")
    pub const CHIP_NAME: Option<&'static str> = option_env!("FEATHERCORE_CHIP_NAME");

    /// Chip vendor (e.g., "st")
    pub const CHIP_VENDOR: Option<&'static str> = option_env!("FEATHERCORE_CHIP_VENDOR");

    /// Chip family (e.g., "stm32f4")
    pub const CHIP_FAMILY: Option<&'static str> = option_env!("FEATHERCORE_CHIP_FAMILY");

    /// Flash size in bytes
    pub const FLASH_SIZE: u32 = 0;

    /// RAM size in bytes
    pub const RAM_SIZE: u32 = 0;

    /// CPU core count
    pub const CORE_COUNT: u32 = 1;

    /// CPU frequency in Hz
    pub const CPU_FREQ_HZ: u32 = 0;
}

/// Board configuration constants (from board config)
/// 开发板配置常量（来自板级配置）
pub mod board {
    //! Board Configuration / 开发板配置

    /// Board name (e.g., "stm32f429i-disc")
    pub const BOARD_NAME: Option<&'static str> = option_env!("FEATHERCORE_BOARD_NAME");

    /// Board full name
    pub const BOARD_FULL_NAME: Option<&'static str> = option_env!("FEATHERCORE_BOARD_FULL_NAME");

    /// Board vendor (e.g., "st")
    pub const BOARD_VENDOR: Option<&'static str> = option_env!("FEATHERCORE_BOARD_VENDOR");

    /// Debugger present
    pub const HAS_DEBUGGER: bool = true;
}

/// Device tree generated structure
/// 设备树生成的结构
pub mod device_tree {
    //! Device Tree Structure / 设备树结构

    /// Memory region structure
    #[derive(Debug, Clone, Copy)]
    pub struct MemoryRegion {
        pub base_address: u32,
        pub size: u32,
        pub name: &'static str,
    }

    /// Get all memory regions
    pub fn memory_regions() -> &'static [MemoryRegion] {
        &[]
    }

    /// Device tree root node
    pub struct DeviceTree {
        _private: (),
    }

    impl DeviceTree {
        pub const fn new() -> Self {
            Self { _private: () }
        }

        pub fn compatible(&self) -> Option<&'static str> {
            option_env!("FEATHERCORE_DEVICE_COMPATIBLE")
        }

        pub fn model(&self) -> Option<&'static str> {
            option_env!("FEATHERCORE_DEVICE_MODEL")
        }

        pub fn address_cells(&self) -> u32 {
            2
        }

        pub fn size_cells(&self) -> u32 {
            1
        }

        pub fn has_interrupt_controller(&self) -> bool {
            false
        }

        pub fn has_clock_controller(&self) -> bool {
            false
        }
    }

    impl Default for DeviceTree {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// GPIO pin configuration
pub mod gpio {
    //! GPIO Configuration / GPIO 配置

    #[derive(Debug, Clone, Copy)]
    pub struct GpioPin {
        pub port: u8,
        pub pin: u8,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LedConfig {
        pub pin: GpioPin,
        pub active_high: bool,
    }

    pub fn led_configs() -> &'static [LedConfig] {
        &[]
    }
}

/// Serial (UART) configuration
pub mod serial {
    //! Serial Configuration / 串口配置

    #[derive(Debug, Clone, Copy)]
    pub struct UartConfig {
        pub base_address: u32,
        pub irq: u32,
        pub clock_frequency: u32,
        pub baud_rate: u32,
    }

    pub fn uart_configs() -> &'static [UartConfig] {
        &[]
    }
}

/// I2C configuration
pub mod i2c {
    //! I2C Configuration / I2C 配置

    #[derive(Debug, Clone, Copy)]
    pub struct I2cConfig {
        pub base_address: u32,
        pub irq: u32,
        pub clock_frequency: u32,
    }

    pub fn i2c_configs() -> &'static [I2cConfig] {
        &[]
    }
}

/// SPI configuration
pub mod spi {
    //! SPI Configuration / SPI 配置

    #[derive(Debug, Clone, Copy)]
    pub struct SpiConfig {
        pub base_address: u32,
        pub irq: u32,
        pub clock_frequency: u32,
    }

    pub fn spi_configs() -> &'static [SpiConfig] {
        &[]
    }
}
