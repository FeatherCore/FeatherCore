//! SYSCFG - System Configuration Controller
//! 系统配置控制器
//!
//! # Overview / 概述
//! STM32U5 System Configuration Controller (SYSCFG) manages various system-level
//! configuration options including memory remap, external interrupt selection, and
//! compensation cell control.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 15: System configuration controller (SYSCFG)
//!
//! ## Main Features / 主要特性
//! - Memory remap configuration
//! - External interrupt selection
//! - Compensation cell control
//! - I/O configuration
//! - Fast start (boot from flash) configuration
//! - DMA request routing
//!
//! # Reference / 参考
//! - RM0456 Chapter 15: System configuration controller (SYSCFG)
//! - RM0456 Section 15.1: SYSCFG introduction
//! - RM0456 Section 15.2: SYSCFG main features
//! - RM0456 Section 15.3: SYSCFG functional description
//! - RM0456 Section 15.4: SYSCFG registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// SYSCFG base address / SYSCFG 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const SYSCFG_BASE: usize = 0x4002_0000;

/// SYSCFG register offsets / SYSCFG 寄存器偏移
//! Reference: RM0456 Section 15.4: SYSCFG register map
pub mod reg {
    /// SYSCFG configuration register 1
    //! Reference: RM0456 Section 15.4.1: SYSCFG configuration register 1 (SYSCFG_CFGR1)
    pub const CFGR1: usize = 0x00;
    /// SYSCFG external interrupt configuration register 1
    //! Reference: RM0456 Section 15.4.2: SYSCFG external interrupt configuration register 1 (SYSCFG_EXTICR1)
    pub const EXTICR1: usize = 0x08;
    /// SYSCFG external interrupt configuration register 2
    //! Reference: RM0456 Section 15.4.3: SYSCFG external interrupt configuration register 2 (SYSCFG_EXTICR2)
    pub const EXTICR2: usize = 0x0C;
    /// SYSCFG external interrupt configuration register 3
    //! Reference: RM0456 Section 15.4.4: SYSCFG external interrupt configuration register 3 (SYSCFG_EXTICR3)
    pub const EXTICR3: usize = 0x10;
    /// SYSCFG external interrupt configuration register 4
    //! Reference: RM0456 Section 15.4.5: SYSCFG external interrupt configuration register 4 (SYSCFG_EXTICR4)
    pub const EXTICR4: usize = 0x14;
    /// SYSCFG configuration register 2
    //! Reference: RM0456 Section 15.4.6: SYSCFG configuration register 2 (SYSCFG_CFGR2)
    pub const CFGR2: usize = 0x18;
    /// SYSCFG SRAM2 write protection register
    //! Reference: RM0456 Section 15.4.7: SYSCFG SRAM2 write protection register (SYSCFG_SKR)
    pub const SKR: usize = 0x20;
    /// SYSCFG compensation cell control register
    //! Reference: RM0456 Section 15.4.8: SYSCFG compensation cell control register (SYSCFG_CCR)
    pub const CCR: usize = 0x20;
    /// SYSCFG peripheral mode configuration register
    //! Reference: RM0456 Section 15.4.9: SYSCFG peripheral mode configuration register (SYSCFG_PMCR)
    pub const PMCR: usize = 0x24;
    /// SYSCFG extended junction temperature register
    //! Reference: RM0456 Section 15.4.10: SYSCFG extended junction temperature register (SYSCFG_EXTJTR)
    pub const EXTJTR: usize = 0x30;

    /// SYSCFG VLAN register
    pub const VLANR: usize = 0x34;

    /// SYSCFG CPU2 peripheral AHB register
    pub const CPU2_AHBMCR: usize = 0x38;

    /// SYSCFG CPU2 peripheral APBMCR register
    pub const CPU2_APBMCR: usize = 0x3C;

    /// SYSCFG core debug register
    pub const COREDBG: usize = 0x40;

    /// SYSCFG C2 boot reset status register
    pub const C2BOOTR: usize = 0x80;

    /// SYSCFG C2 boot control register
    pub const C2BOOTCR: usize = 0x84;

    /// SYSCFG Interconnect matrix register
    pub const ICNR: usize = 0x88;

    /// SYSCFG Package pin count register
    pub const PKGR: usize = 0x8C;
}

/// CFGR1 Register Bit Definitions
/// Reference: RM0456 Section 15.4.1
pub mod cfgr1_bits {
    /// Boot pin configuration after reset
    pub const BOOT_ADD0: u32 = 0x3F << 0;
    /// Boot pin configuration after reset
    pub const BOOT_ADD1: u32 = 0x3F << 8;
    /// Memory remap selection
    pub const MEM_MODE: u32 = 0b111 << 24;
}

/// CFGR2 Register Bit Definitions
/// Reference: RM0456 Section 15.4.6
pub mod cfgr2_bits {
    /// SRAM2 page write protection
    pub const PWR_LOCK: u32 = 1 << 8;
    /// SRAM2 page write protection key
    pub const PWR_LOCK_KEY: u32 = 1 << 9;
    /// ECC prefix
    pub const ECC_PREFIX: u32 = 1 << 16;
}

/// CCR (Compensation Cell Register) Bit Definitions
/// Reference: RM0456 Section 15.4.8
pub mod ccr_bits {
    /// Compensation cell enable
    pub const COMP_EN: u32 = 1 << 0;
    /// Compensation cell ready
    pub const COMP_RDY: u32 = 1 << 8;
}

/// PMCR Register Bit Definitions
/// Reference: RM0456 Section 15.4.9
pub mod pmcr_bits {
    /// Ethernet PHY interface selection
    pub const ETH_SEL: u32 = 0b111 << 0;
    /// CAN1 RX/TX selection
    pub const CAN1_SEL: u32 = 0b11 << 4;
    /// CAN2 RX/TX selection
    pub const CAN2_SEL: u32 = 0b11 << 6;
    /// I2C3 SCL/SDA selection
    pub const I2C3_SEL: u32 = 0b11 << 8;
    /// SPI3 selection
    pub const SPI3_SEL: u32 = 0b11 << 10;
    /// UART5 or USART6 selection
    pub const UART5_SEL: u32 = 1 << 12;
    /// UART4 or USART6 selection
    pub const UART4_SEL: u32 = 1 << 13;
    /// I2C4 selection
    pub const I2C4_SEL: u32 = 1 << 14;
    /// I2C2 selection
    pub const I2C2_SEL: u32 = 1 << 15;
    /// DSIHOST selection
    pub const DSI_SEL: u32 = 1 << 16;
    /// LCD/TFT selection
    pub const LCD_SEL: u32 = 0b111 << 17;
    /// USB OTG FS selection
    pub const OTG_FS_SEL: u32 = 1 << 20;
    /// Trace I/O selection
    pub const IO_SEL: u32 = 0b11 << 21;
}

/// Memory remap configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemRemap {
    /// Flash main memory at address 0x00000000
    Flash = 0,
    /// System flash at address 0x00000000
    SystemFlash = 1,
    /// Boot from OTP at 0x00000000
    OTP = 2,
    /// SRAM1 at address 0x00000000
    SRAM1 = 3,
    /// Reserved
    Reserved = 4,
}

/// Fast Start boot address after reset
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BootAddress {
    /// Flash main memory
    FlashMain = 0,
    /// System flash
    SystemFlash = 1,
    /// Boot from OTP
    OTP = 2,
    /// SRAM1
    SRAM1 = 3,
    /// SRAM2
    SRAM2 = 4,
    /// Reserved
    Reserved = 5,
    /// Backup SRAM
    BKPSRAM = 6,
    /// Reserved
    Reserved2 = 7,
}

/// Ethernet PHY interface type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EthernetPhy {
    /// MII (Media Independent Interface)
    MII = 0b000,
    /// RMII (Reduced MII)
    RMII = 0b001,
    /// Reserved
    Reserved = 0b010,
}

/// External interrupt port
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtiPort {
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

/// Memory remap configuration
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemRemap {
    /// Flash main memory at address 0x00000000
    Flash = 0,
    /// System flash at address 0x00000000
    SystemFlash = 1,
    /// SRAM1 at address 0x00000000
    SRAM1 = 3,
    /// Reserved
    Reserved = 4,
}

/// SYSCFG instance
pub struct Syscfg;

impl Syscfg {
    /// Create SYSCFG instance
    pub const fn new() -> Self {
        Self
    }

    /// Configure memory remap
    pub fn set_mem_remap(&self, remap: MemRemap) {
        unsafe {
            let cfgr1 = (SYSCFG_BASE + reg::CFGR1) as *mut u32;
            let val = read_volatile(cfgr1);
            write_volatile(cfgr1, (val & !cfgr1_bits::MEM_MODE) | ((remap as u32) << 24));
        }
    }

    /// Get memory remap configuration
    pub fn get_mem_remap(&self) -> MemRemap {
        unsafe {
            let cfgr1 = (SYSCFG_BASE + reg::CFGR1) as *const u32;
            let val = (read_volatile(cfgr1) >> 24) & 0x7;
            match val {
                0 => MemRemap::Flash,
                1 => MemRemap::SystemFlash,
                2 => MemRemap::OTP,
                3 => MemRemap::SRAM1,
                _ => MemRemap::Reserved,
            }
        }
    }

    /// Set fast boot address 0
    pub fn set_boot_address0(&self, addr: BootAddress) {
        unsafe {
            let cfgr1 = (SYSCFG_BASE + reg::CFGR1) as *mut u32;
            let val = read_volatile(cfgr1);
            write_volatile(cfgr1, (val & !cfgr1_bits::BOOT_ADD0) | ((addr as u32) << 0));
        }
    }

    /// Set fast boot address 1
    pub fn set_boot_address1(&self, addr: BootAddress) {
        unsafe {
            let cfgr1 = (SYSCFG_BASE + reg::CFGR1) as *mut u32;
            let val = read_volatile(cfgr1);
            write_volatile(cfgr1, (val & !cfgr1_bits::BOOT_ADD1) | ((addr as u32) << 8));
        }
    }

    /// Enable compensation cell
    pub fn enable_comp_cell(&self) {
        unsafe {
            let ccr = (SYSCFG_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, ccr_bits::COMP_EN);
        }
    }

    /// Disable compensation cell
    pub fn disable_comp_cell(&self) {
        unsafe {
            let ccr = (SYSCFG_BASE + reg::CCR) as *mut u32;
            write_volatile(ccr, 0);
        }
    }

    /// Check if compensation cell is ready
    pub fn is_comp_cell_ready(&self) -> bool {
        unsafe {
            let ccr = (SYSCFG_BASE + reg::CCR) as *const u32;
            (read_volatile(ccr) & ccr_bits::COMP_RDY) != 0
        }
    }

    /// Configure Ethernet PHY interface
    pub fn set_ethernet_phy(&self, phy: EthernetPhy) {
        unsafe {
            let pmcr = (SYSCFG_BASE + reg::PMCR) as *mut u32;
            let val = read_volatile(pmcr);
            write_volatile(pmcr, (val & !pmcr_bits::ETH_SEL) | ((phy as u32) << 0));
        }
    }

    /// Configure CAN1 RX/TX pins
    pub fn set_can1_pins(&self, sel: u8) {
        unsafe {
            let pmcr = (SYSCFG_BASE + reg::PMCR) as *mut u32;
            let val = read_volatile(pmcr);
            write_volatile(pmcr, (val & !pmcr_bits::CAN1_SEL) | ((sel & 0x3) << 4));
        }
    }

    /// Configure I2C3 pins
    pub fn set_i2c3_pins(&self, sel: u8) {
        unsafe {
            let pmcr = (SYSCFG_BASE + reg::PMCR) as *mut u32;
            let val = read_volatile(pmcr);
            write_volatile(pmcr, (val & !pmcr_bits::I2C3_SEL) | ((sel & 0x3) << 8));
        }
    }

    /// Configure external interrupt for a pin
    pub fn configure_exti(&self, line: u8, port: ExtiPort) {
        if line > 15 {
            return;
        }

        unsafe {
            let exticr_offset = (line / 4) * 4;
            let exticr = (SYSCFG_BASE + reg::EXTICR1 + exticr_offset) as *mut u32;
            let val = read_volatile(exticr);
            let shift = (line % 4) * 4;
            write_volatile(exticr, (val & !(0xF << shift)) | ((port as u32) << shift));
        }
    }

    /// Enable SRAM2 page write protection
    pub fn enable_sram2_write_protection(&self) {
        unsafe {
            let cfgr2 = (SYSCFG_BASE + reg::CFGR2) as *mut u32;
            write_volatile(cfgr2, cfgr2_bits::PWR_LOCK_KEY);
            write_volatile(cfgr2, cfgr2_bits::PWR_LOCK);
        }
    }

    /// Disable SRAM2 page write protection
    pub fn disable_sram2_write_protection(&self) {
        unsafe {
            let cfgr2 = (SYSCFG_BASE + reg::CFGR2) as *mut u32;
            write_volatile(cfgr2, cfgr2_bits::PWR_LOCK_KEY);
            write_volatile(cfgr2, 0);
        }
    }

    /// Get C2 boot reset status
    pub fn get_c2_boot_status(&self) -> u32 {
        unsafe {
            read_volatile((SYSCFG_BASE + reg::C2BOOTR) as *const u32)
        }
    }

    /// Set C2 boot address
    pub fn set_c2_boot_address(&self, addr: u32) {
        unsafe {
            let c2bootcr = (SYSCFG_BASE + reg::C2BOOTCR) as *mut u32;
            write_volatile(c2bootcr, addr & 0xFFFF_FFC0);
        }
    }

    /// Get interconnect matrix configuration
    pub fn get_interconnect_config(&self) -> u32 {
        unsafe {
            read_volatile((SYSCFG_BASE + reg::ICNR) as *const u32)
        }
    }

    /// Get package pin count
    pub fn get_package_pins(&self) -> u32 {
        unsafe {
            read_volatile((SYSCFG_BASE + reg::PKGR) as *const u32) & 0xFF
        }
    }
}

impl Default for Syscfg {
    fn default() -> Self {
        Self::new()
    }
}
