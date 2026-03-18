//! RCC - Reset and Clock Control
//! 复位和时钟控制
//!
//! # Overview / 概述
//! STM32U5 Reset and Clock Control (RCC) module manages the clock system
//! and reset functions of the chip.
//!
//! # Features / 功能特性
//! - Clock source switching and monitoring
//! - Peripheral clock gating
//! - Reset control (system reset, peripheral reset)
//! - Low-power mode management
//! - Clock Security System (CSS)
//!
//! # Clock Sources / 时钟源
//! Reference: RM0456 Chapter 11, Section 11.4
//!
//! | Clock | Frequency | Description |
//! |-------|-----------|-------------|
//! | MSIS | 100 kHz - 48 MHz | Multi-speed internal RC oscillator |
//! | MSIK | 100 kHz - 48 MHz | Multi-speed internal RC oscillator for kernel clocks |
//! | HSI16 | 16 MHz | High-speed internal 16 MHz RC oscillator |
//! | HSI48 | 48 MHz | High-speed internal 48 MHz RC oscillator |
//! | HSE | 4 - 48 MHz | High-speed external crystal or clock |
//! | LSI | 32 kHz / 250 Hz | Low-speed internal RC oscillator |
//! | LSE | 32.768 kHz | Low-speed external crystal or clock |
//! | SHSI | 48 MHz | Secure high-speed internal oscillator |
//! | PLL1 | Up to 160 MHz | System PLL |
//! | PLL2 | - | PLL2 for peripherals |
//! | PLL3 | - | PLL3 for peripherals |
//!
//! # Bus Clocks / 总线时钟
//! - **SYSCLK**: System clock, up to 160 MHz
//! - **HCLK**: AHB bus clock
//! - **PCLK1**: APB1 bus clock, up to 160 MHz
//! - **PCLK2**: APB2 bus clock, up to 160 MHz
//! - **PCLK3**: APB3 bus clock, up to 160 MHz
//!
//! # Reference / 参考
//! - RM0456 Chapter 11: Reset and clock control (RCC)
//! - RM0456 Section 11.1: RCC introduction
//! - RM0456 Section 11.2: RCC pins and internal signals
//! - RM0456 Section 11.3: RCC reset functional description
//! - RM0456 Section 11.4: RCC clock functional description

//! RCC base address (non-secure mode)
//! Reference: RM0456 Chapter 2, Table 1 or memory map
pub const RCC_BASE: usize = 0x4002_1000;

//! RCC secure base address (for TrustZone)
pub const RCC_BASE_SEC: usize = 0x5002_1000;

//! RCC Register Offsets
//! Reference: RM0456 Chapter 11, Section 11.10: RCC registers
pub mod reg {
    //! Clock Control Register
    //! Reference: RM0456 Section 11.10.1
    pub const CR: usize = 0x00;

    //! Internal Clock Sources Calibration Register 1
    //! Reference: RM0456 Section 11.10.2
    pub const ICSCR1: usize = 0x08;

    //! Internal Clock Sources Calibration Register 2
    pub const ICSCR2: usize = 0x0C;

    //! Internal Clock Sources Calibration Register 3
    pub const ICSCR3: usize = 0x10;

    //! Clock Recovery RC Register
    pub const CRRCR: usize = 0x14;

    //! Clock Configuration Register 1
    //! Reference: RM0456 Section 11.10.3
    pub const CFGR1: usize = 0x1C;

    //! Clock Configuration Register 2
    pub const CFGR2: usize = 0x20;

    //! Clock Configuration Register 3
    pub const CFGR3: usize = 0x24;

    //! PLL1 Configuration Register
    //! Reference: RM0456 Section 11.10.4
    pub const PLL1CFGR: usize = 0x28;

    //! PLL2 Configuration Register
    pub const PLL2CFGR: usize = 0x2C;

    //! PLL3 Configuration Register
    pub const PLL3CFGR: usize = 0x30;

    //! PLL1 Divider Configuration Register
    pub const PLL1DIVR: usize = 0x34;

    //! PLL1 Fractional Divider Register
    pub const PLL1FRACR: usize = 0x38;

    //! PLL2 Divider Configuration Register
    pub const PLL2DIVR: usize = 0x3C;

    //! PLL2 Fractional Divider Register
    pub const PLL2FRACR: usize = 0x40;

    //! PLL3 Divider Configuration Register
    pub const PLL3DIVR: usize = 0x44;

    //! PLL3 Fractional Divider Register
    pub const PLL3FRACR: usize = 0x48;

    //! Clock Interrupt Enable Register
    pub const CIER: usize = 0x50;

    //! Clock Interrupt Flag Register
    pub const CIFR: usize = 0x54;

    //! Clock Interrupt Clear Register
    pub const CICR: usize = 0x58;

    //! AHB1 Peripheral Reset Register
    pub const AHB1RSTR: usize = 0x60;

    //! AHB2 Peripheral Reset Register 1
    pub const AHB2RSTR1: usize = 0x64;

    //! AHB2 Peripheral Reset Register 2
    pub const AHB2RSTR2: usize = 0x68;

    //! AHB3 Peripheral Reset Register
    pub const AHB3RSTR: usize = 0x6C;

    //! APB1 Peripheral Reset Register 1
    pub const APB1RSTR1: usize = 0x74;

    //! APB1 Peripheral Reset Register 2
    pub const APB1RSTR2: usize = 0x78;

    //! APB2 Peripheral Reset Register
    pub const APB2RSTR: usize = 0x7C;

    //! APB3 Peripheral Reset Register
    pub const APB3RSTR: usize = 0x80;

    //! AHB1 Peripheral Clock Enable Register
    //! Reference: RM0456 Section 11.10.5
    pub const AHB1ENR: usize = 0x88;

    //! AHB2 Peripheral Clock Enable Register 1
    pub const AHB2ENR1: usize = 0x8C;

    //! AHB2 Peripheral Clock Enable Register 2
    pub const AHB2ENR2: usize = 0x90;

    //! AHB3 Peripheral Clock Enable Register
    pub const AHB3ENR: usize = 0x94;

    //! APB1 Peripheral Clock Enable Register 1
    pub const APB1ENR1: usize = 0x9C;

    //! APB1 Peripheral Clock Enable Register 2
    pub const APB1ENR2: usize = 0xA0;

    //! APB2 Peripheral Clock Enable Register
    pub const APB2ENR: usize = 0xA4;

    //! APB3 Peripheral Clock Enable Register
    pub const APB3ENR: usize = 0xA8;

    //! AHB1 Peripheral Sleep/Stop Mode Clock Enable Register
    pub const AHB1SMENR: usize = 0xB0;

    //! AHB2 Peripheral Sleep/Stop Mode Clock Enable Register 1
    pub const AHB2SMENR1: usize = 0xB4;

    //! AHB2 Peripheral Sleep/Stop Mode Clock Enable Register 2
    pub const AHB2SMENR2: usize = 0xB8;

    //! AHB3 Peripheral Sleep/Stop Mode Clock Enable Register
    pub const AHB3SMENR: usize = 0xBC;

    //! APB1 Peripheral Sleep/Stop Mode Clock Enable Register 1
    pub const APB1SMENR1: usize = 0xC4;

    //! APB1 Peripheral Sleep/Stop Mode Clock Enable Register 2
    pub const APB1SMENR2: usize = 0xC8;

    //! APB2 Peripheral Sleep/Stop Mode Clock Enable Register
    pub const APB2SMENR: usize = 0xCC;

    //! APB3 Peripheral Sleep/Stop Mode Clock Enable Register
    pub const APB3SMENR: usize = 0xD0;

    //! SRD Autonomous Mode Register
    pub const SRDAMR: usize = 0xD8;

    //! Peripheral Independent Clock Configuration Register 1
    //! Reference: RM0456 Section 11.10.6
    pub const CCIPR1: usize = 0xE0;

    //! Peripheral Independent Clock Configuration Register 2
    pub const CCIPR2: usize = 0xE4;

    //! Peripheral Independent Clock Configuration Register 3
    pub const CCIPR3: usize = 0xE8;

    //! Backup Domain Control Register
    //! Reference: RM0456 Section 11.10.7
    pub const BDCR: usize = 0xF0;

    //! Control/Status Register
    pub const CSR: usize = 0xF4;

    //! RCC Security Configuration Register
    pub const SECCFGR: usize = 0x110;

    //! RCC Privilege Configuration Register
    pub const PRIVCFGR: usize = 0x114;
}

//! CR Register Bit Definitions
//! Reference: RM0456 Section 11.10.1: RCC clock control register (RCC_CR)
pub mod cr_bits {
    pub const MSISON: u32 = 1 << 0;
    pub const MSIKERON: u32 = 1 << 1;
    pub const MSISRDY: u32 = 1 << 2;
    pub const MSIPLLEN: u32 = 1 << 3;
    pub const MSIKON: u32 = 1 << 4;
    pub const MSIKRDY: u32 = 1 << 5;
    pub const HSION: u32 = 1 << 8;
    pub const HSIKERON: u32 = 1 << 9;
    pub const HSIRDY: u32 = 1 << 10;
    pub const HSI48ON: u32 = 1 << 12;
    pub const HSI48RDY: u32 = 1 << 13;
    pub const SHSION: u32 = 1 << 14;
    pub const SHSIRDY: u32 = 1 << 15;
    pub const HSEON: u32 = 1 << 16;
    pub const HSERDY: u32 = 1 << 17;
    pub const HSEBYP: u32 = 1 << 18;
    pub const CSSON: u32 = 1 << 19;
    pub const PLL1ON: u32 = 1 << 24;
    pub const PLL1RDY: u32 = 1 << 25;
    pub const PLL2ON: u32 = 1 << 26;
    pub const PLL2RDY: u32 = 1 << 27;
    pub const PLL3ON: u32 = 1 << 28;
    pub const PLL3RDY: u32 = 1 << 29;
}

//! CFGR1 Register Bit Definitions
//! Reference: RM0456 Section 11.10.3: RCC clock configuration register (RCC_CFGR)
pub mod cfgr1_bits {
    pub const SW: u32 = 0b11 << 0;
    pub const SWS: u32 = 0b11 << 2;
    pub const STOPWUCK: u32 = 1 << 4;
    pub const STOPKERWUCK: u32 = 1 << 5;
    pub const MCOSEL: u32 = 0b1111 << 24;
    pub const MCOPRE: u32 = 0b111 << 28;

    pub const SW_MSI: u32 = 0b00 << 0;
    pub const SW_HSI16: u32 = 0b01 << 0;
    pub const SW_HSE: u32 = 0b10 << 0;
    pub const SW_PLL1: u32 = 0b11 << 0;
}

//! PLL1CFGR Register Bit Definitions
//! Reference: RM0456 Section 11.10.4
pub mod pll1cfgr_bits {
    pub const PLL1SRC: u32 = 0b11 << 0;
    pub const PLL1RGE: u32 = 0b11 << 2;
    pub const PLL1FRACEN: u32 = 1 << 4;
    pub const PLL1M: u32 = 0b1111 << 8;
    pub const PLL1MBOOST: u32 = 0b1111 << 12;
    pub const PLL1PEN: u32 = 1 << 16;
    pub const PLL1QEN: u32 = 1 << 17;
    pub const PLL1REN: u32 = 1 << 18;

    pub const PLL1SRC_NONE: u32 = 0b00 << 0;
    pub const PLL1SRC_HSI16: u32 = 0b01 << 0;
    pub const PLL1SRC_HSE: u32 = 0b10 << 0;
    pub const PLL1SRC_MSI: u32 = 0b11 << 0;
}

//! Clock sources enumeration
//! Reference: RM0456 Section 11.4
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockSource {
    /// Multi-Speed Internal (MSI)
    MSI = 0,
    /// High-Speed Internal 16 MHz
    HSI16 = 1,
    /// High-Speed External
    HSE = 2,
    /// PLL1
    PLL1 = 3,
}

//! System clock frequencies tracking
//! These are updated when clock configuration changes
static mut SYSCLK_FREQ: u32 = 4_000_000; // Default MSI after reset / 复位后默认MSI
static mut HCLK_FREQ: u32 = 4_000_000;
static mut PCLK1_FREQ: u32 = 4_000_000;
static mut PCLK2_FREQ: u32 = 4_000_000;
static mut PCLK3_FREQ: u32 = 4_000_000;

//! Initialize RCC with default MSI clock
//! Reference: RM0456 Chapter 11
//!
//! After reset, the system clock source is MSI (configured at 4 MHz).
//! This function initializes the RCC peripheral and enables the HSI16
//! oscillator as a preparation for higher frequency operation.
pub fn init() {
    unsafe {
        // Enable HSI16 oscillator
        // Reference: RM0456 Section 11.10.1, bit HSION
        let cr = (RCC_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= cr_bits::HSION;
        core::ptr::write_volatile(cr, val);

        // Wait for HSI16 ready
        // Reference: RM0456 Section 11.10.1, bit HSIRDY
        while (core::ptr::read_volatile(cr) & cr_bits::HSIRDY) == 0 {}

        // Select HSI16 as system clock (optional, for higher performance)
        // Reference: RM0456 Section 11.10.3, bits SW[1:0]
        let cfgr1 = (RCC_BASE + reg::CFGR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cfgr1);
        val &= !cfgr1_bits::SW;
        val |= cfgr1_bits::SW_HSI16;
        core::ptr::write_volatile(cfgr1, val);

        // Wait for HSI16 to be used as system clock
        // Reference: RM0456 Section 11.10.3, bits SWS[1:0]
        while (core::ptr::read_volatile(cfgr1) & cfgr1_bits::SWS) != cfgr1_bits::SWS {}

        // Update frequency tracking
        SYSCLK_FREQ = 16_000_000;
        HCLK_FREQ = 16_000_000;
        PCLK1_FREQ = 16_000_000;
        PCLK2_FREQ = 16_000_000;
        PCLK3_FREQ = 16_000_000;
    }
}

//! Configure PLL1 for 160 MHz system clock
//! Reference: RM0456 Section 11.4.3: PLL configuration
//!
//! PLL1 configuration example:
//! - Input clock: HSI16 (16 MHz)
//! - PLL1M divider: 4 (16 MHz / 4 = 4 MHz)
//! - PLL1N multiplier: 40 (4 MHz * 40 = 160 MHz VCO)
//! - PLL1R divider: 1 (160 MHz / 1 = 160 MHz SYSCLK)
//!
//! This achieves maximum performance for STM32U5 series.
pub fn configure_pll_160mhz() {
    unsafe {
        // Disable PLL1 before configuration
        // Reference: RM0456 Section 11.10.1, bit PLL1ON
        let cr = (RCC_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val &= !cr_bits::PLL1ON;
        core::ptr::write_volatile(cr, val);

        // Wait for PLL1 to be disabled
        while (core::ptr::read_volatile(cr) & cr_bits::PLL1RDY) != 0 {}

        // Configure PLL1
        // Reference: RM0456 Section 11.10.4
        let pll1cfgr = (RCC_BASE + reg::PLL1CFGR) as *mut u32;
        let mut val = 0;
        val |= pll1cfgr_bits::PLL1SRC_HSI16;  // PLL1SRC = HSI16
        val |= 0b0011 << 8;  // PLL1M = 4 (divide by 4)
        val |= pll1cfgr_bits::PLL1REN;  // Enable PLL1R output
        core::ptr::write_volatile(pll1cfgr, val);

        // Configure PLL1 dividers
        // Reference: RM0456 Section 11.10.4
        let pll1divr = (RCC_BASE + reg::PLL1DIVR) as *mut u32;
        let mut val = 0;
        val |= 40 << 0;  // PLL1N = 40 (multiply by 40)
        val |= 1 << 9;   // PLL1P = 1 (divide by 1)
        val |= 1 << 16;  // PLL1Q = 1 (divide by 1)
        val |= 1 << 24;  // PLL1R = 1 (divide by 1)
        core::ptr::write_volatile(pll1divr, val);

        // Enable PLL1
        // Reference: RM0456 Section 11.10.1, bit PLL1ON
        let cr = (RCC_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= cr_bits::PLL1ON;
        core::ptr::write_volatile(cr, val);

        // Wait for PLL1 ready
        while (core::ptr::read_volatile(cr) & cr_bits::PLL1RDY) == 0 {}

        // Configure Flash latency for 160 MHz
        // Reference: RM0456 Section 7.4.1: FLASH_ACR
        let flash_acr = 0x4002_2000 as *mut u32;
        let mut val = core::ptr::read_volatile(flash_acr);
        val &= !0xF;
        val |= 4;  // LATENCY = 4 wait states for 160 MHz
        core::ptr::write_volatile(flash_acr, val);

        // Wait for flash latency to be applied
        while (core::ptr::read_volatile(flash_acr) & 0xF) != 4 {}

        // Select PLL1 as system clock
        // Reference: RM0456 Section 11.10.3, bits SW[1:0]
        let cfgr1 = (RCC_BASE + reg::CFGR1) as *mut u32;
        let mut val = core::ptr::read_volatile(cfgr1);
        val &= !cfgr1_bits::SW;
        val |= cfgr1_bits::SW_PLL1;
        core::ptr::write_volatile(cfgr1, val);

        // Wait for PLL1 to be used as system clock
        while (core::ptr::read_volatile(cfgr1) & cfgr1_bits::SWS) != cfgr1_bits::SWS {}

        // Update frequency tracking
        SYSCLK_FREQ = 160_000_000;
        HCLK_FREQ = 160_000_000;
        PCLK1_FREQ = 160_000_000;
        PCLK2_FREQ = 160_000_000;
        PCLK3_FREQ = 160_000_000;
    }
}

//! Get current system clock frequency in Hz
//! Reference: RM0456 Section 11.4
pub fn get_sysclk_freq() -> u32 {
    unsafe { SYSCLK_FREQ }
}

//! Get AHB bus frequency in Hz
pub fn get_hclk_freq() -> u32 {
    unsafe { HCLK_FREQ }
}

//! Get APB1 bus frequency in Hz
pub fn get_pclk1_freq() -> u32 {
    unsafe { PCLK1_FREQ }
}

//! Get APB2 bus frequency in Hz
pub fn get_pclk2_freq() -> u32 {
    unsafe { PCLK2_FREQ }
}

//! Get APB3 bus frequency in Hz
pub fn get_pclk3_freq() -> u32 {
    unsafe { PCLK3_FREQ }
}

//! Enable AHB1 peripheral clock
//! Reference: RM0456 Section 11.10.5
pub fn enable_ahb1_clock(periph: u32) {
    unsafe {
        let ahb1enr = (RCC_BASE + reg::AHB1ENR) as *mut u32;
        let val = core::ptr::read_volatile(ahb1enr);
        core::ptr::write_volatile(ahb1enr, val | periph);
    }
}

//! Enable AHB2 peripheral clock
pub fn enable_ahb2_clock(periph: u32) {
    unsafe {
        let ahb2enr1 = (RCC_BASE + reg::AHB2ENR1) as *mut u32;
        let val = core::ptr::read_volatile(ahb2enr1);
        core::ptr::write_volatile(ahb2enr1, val | periph);
    }
}

//! Enable AHB3 peripheral clock
pub fn enable_ahb3_clock(periph: u32) {
    unsafe {
        let ahb3enr = (RCC_BASE + reg::AHB3ENR) as *mut u32;
        let val = core::ptr::read_volatile(ahb3enr);
        core::ptr::write_volatile(ahb3enr, val | periph);
    }
}

//! Enable APB1 peripheral clock
pub fn enable_apb1_clock(periph: u32) {
    unsafe {
        let apb1enr1 = (RCC_BASE + reg::APB1ENR1) as *mut u32;
        let val = core::ptr::read_volatile(apb1enr1);
        core::ptr::write_volatile(apb1enr1, val | periph);
    }
}

//! Enable APB2 peripheral clock
pub fn enable_apb2_clock(periph: u32) {
    unsafe {
        let apb2enr = (RCC_BASE + reg::APB2ENR) as *mut u32;
        let val = core::ptr::read_volatile(apb2enr);
        core::ptr::write_volatile(apb2enr, val | periph);
    }
}

//! Enable APB3 peripheral clock
pub fn enable_apb3_clock(periph: u32) {
    unsafe {
        let apb3enr = (RCC_BASE + reg::APB3ENR) as *mut u32;
        let val = core::ptr::read_volatile(apb3enr);
        core::ptr::write_volatile(apb3enr, val | periph);
    }
}

//! AHB1 Peripheral Clock Enable Bits
//! Reference: RM0456 Section 11.10.5: RCC AHB1 peripheral clock enable register (RCC_AHB1ENR)
pub mod ahb1 {
    /// GPDMA1 clock enable
    pub const GPDMA1: u32 = 1 << 0;
    /// CORDIC clock enable
    pub const CORDIC: u32 = 1 << 1;
    /// FMAC clock enable
    pub const FMAC: u32 = 1 << 2;
    /// MDF1 clock enable
    pub const MDF1: u32 = 1 << 3;
    /// Flash interface clock enable
    pub const FLASH: u32 = 1 << 8;
    /// CRC clock enable
    pub const CRC: u32 = 1 << 12;
    /// TSC clock enable
    pub const TSC: u32 = 1 << 16;
    /// RAMCFG clock enable
    pub const RAMCFG: u32 = 1 << 17;
    /// DMA2D clock enable
    pub const DMA2D: u32 = 1 << 18;
    /// GTZC1 clock enable
    pub const GTZC1: u32 = 1 << 24;
}

//! AHB2 Peripheral Clock Enable Bits (Part 1)
//! Reference: RM0456 Section 11.10.5: RCC AHB2 peripheral clock enable register 1 (RCC_AHB2ENR1)
pub mod ahb2 {
    /// GPIO port A clock enable
    pub const GPIOA: u32 = 1 << 0;
    /// GPIO port B clock enable
    pub const GPIOB: u32 = 1 << 1;
    /// GPIO port C clock enable
    pub const GPIOC: u32 = 1 << 2;
    /// GPIO port D clock enable
    pub const GPIOD: u32 = 1 << 3;
    /// GPIO port E clock enable
    pub const GPIOE: u32 = 1 << 4;
    /// GPIO port F clock enable
    pub const GPIOF: u32 = 1 << 5;
    /// GPIO port G clock enable
    pub const GPIOG: u32 = 1 << 6;
    /// GPIO port H clock enable
    pub const GPIOH: u32 = 1 << 7;
    /// GPIO port I clock enable
    pub const GPIOI: u32 = 1 << 8;
}

//! AHB2 Peripheral Clock Enable Bits (Part 2)
//! Reference: RM0456 Section 11.10.5: RCC AHB2 peripheral clock enable register 2 (RCC_AHB2ENR2)
pub mod ahb2_2 {
    /// ADC1 clock enable
    pub const ADC1: u32 = 1 << 0;
    /// ADC2 clock enable
    pub const ADC2: u32 = 1 << 1;
    /// ADC4 clock enable
    pub const ADC4: u32 = 1 << 2;
    /// DAC1 clock enable
    pub const DAC1: u32 = 1 << 3;
    /// AES clock enable
    pub const AES: u32 = 1 << 4;
    /// HASH clock enable
    pub const HASH: u32 = 1 << 5;
    /// RNG clock enable
    pub const RNG: u32 = 1 << 6;
    /// SAES clock enable
    pub const SAES: u32 = 1 << 7;
    /// PKA clock enable
    pub const PKA: u32 = 1 << 8;
    /// OTFDEC1 clock enable
    pub const OTFDEC1: u32 = 1 << 9;
    /// OTFDEC2 clock enable
    pub const OTFDEC2: u32 = 1 << 10;
    /// FDCAN1 clock enable
    pub const FDCAN1: u32 = 1 << 14;
    /// FDCAN2 clock enable
    pub const FDCAN2: u32 = 1 << 15;
    /// USB FS clock enable
    pub const USB_FS: u32 = 1 << 21;
}

//! AHB3 Peripheral Clock Enable Bits
//! Reference: RM0456 Section 11.10.5
pub mod ahb3 {
    /// FMC clock enable
    pub const FMC: u32 = 1 << 0;
    /// OCTOSPI1 clock enable
    pub const OCTOSPI1: u32 = 1 << 1;
    /// OCTOSPI2 clock enable
    pub const OCTOSPI2: u32 = 1 << 2;
    /// SDMMC1 clock enable
    pub const SDMMC1: u32 = 1 << 4;
    /// SDMMC2 clock enable
    pub const SDMMC2: u32 = 1 << 5;
    /// LTDC clock enable
    pub const LTDC: u32 = 1 << 8;
    /// DCMI clock enable
    pub const DCMI: u32 = 1 << 10;
    /// PSSI clock enable
    pub const PSSI: u32 = 1 << 12;
}

//! APB1 Peripheral Clock Enable Bits (Part 1)
//! Reference: RM0456 Section 11.10.5
pub mod apb1_1 {
    /// TIM2 clock enable
    pub const TIM2: u32 = 1 << 0;
    /// TIM3 clock enable
    pub const TIM3: u32 = 1 << 1;
    /// TIM4 clock enable
    pub const TIM4: u32 = 1 << 2;
    /// TIM5 clock enable
    pub const TIM5: u32 = 1 << 3;
    /// TIM6 clock enable
    pub const TIM6: u32 = 1 << 4;
    /// TIM7 clock enable
    pub const TIM7: u32 = 1 << 5;
    /// WWDG clock enable
    pub const WWDG: u32 = 1 << 11;
    /// SPI2 clock enable
    pub const SPI2: u32 = 1 << 14;
    /// SPI3 clock enable
    pub const SPI3: u32 = 1 << 15;
    /// USART2 clock enable
    pub const USART2: u32 = 1 << 17;
    /// USART3 clock enable
    pub const USART3: u32 = 1 << 18;
    /// UART4 clock enable
    pub const UART4: u32 = 1 << 19;
    /// UART5 clock enable
    pub const UART5: u32 = 1 << 20;
    /// I2C1 clock enable
    pub const I2C1: u32 = 1 << 21;
    /// I2C2 clock enable
    pub const I2C2: u32 = 1 << 22;
    /// I2C3 clock enable
    pub const I2C3: u32 = 1 << 23;
    /// CRS clock enable
    pub const CRS: u32 = 1 << 24;
    /// PWR clock enable
    pub const PWR: u32 = 1 << 28;
    /// DAC1 clock enable
    pub const DAC1: u32 = 1 << 29;
    /// OPAMP clock enable
    pub const OPAMP: u32 = 1 << 30;
    /// LPTIM1 clock enable
    pub const LPTIM1: u32 = 1 << 31;
}

//! APB1 Peripheral Clock Enable Bits (Part 2)
//! Reference: RM0456 Section 11.10.5
pub mod apb1_2 {
    /// LPTIM2 clock enable
    pub const LPTIM2: u32 = 1 << 5;
    /// LPTIM3 clock enable
    pub const LPTIM3: u32 = 1 << 6;
    /// I2C4 clock enable
    pub const I2C4: u32 = 1 << 7;
    /// LPUART1 clock enable
    pub const LPUART1: u32 = 1 << 8;
}

//! APB2 Peripheral Clock Enable Bits
//! Reference: RM0456 Section 11.10.5
pub mod apb2 {
    /// TIM1 clock enable
    pub const TIM1: u32 = 1 << 11;
    /// SPI1 clock enable
    pub const SPI1: u32 = 1 << 12;
    /// TIM8 clock enable
    pub const TIM8: u32 = 1 << 13;
    /// USART1 clock enable
    pub const USART1: u32 = 1 << 14;
    /// TIM15 clock enable
    pub const TIM15: u32 = 1 << 16;
    /// TIM16 clock enable
    pub const TIM16: u32 = 1 << 17;
    /// TIM17 clock enable
    pub const TIM17: u32 = 1 << 18;
    /// SAI1 clock enable
    pub const SAI1: u32 = 1 << 21;
    /// SAI2 clock enable
    pub const SAI2: u32 = 1 << 22;
    /// HRTIM clock enable
    pub const HRTIM: u32 = 1 << 26;
}

//! APB3 Peripheral Clock Enable Bits
//! Reference: RM0456 Section 11.10.5
pub mod apb3 {
    /// RTC clock enable
    pub const RTC: u32 = 1 << 0;
    /// VREFBUF clock enable
    pub const VREFBUF: u32 = 1 << 1;
    /// COMP clock enable
    pub const COMP: u32 = 1 << 2;
    /// UCPD1 clock enable
    pub const UCPD1: u32 = 1 << 8;
    /// UCPD2 clock enable
    pub const UCPD2: u32 = 1 << 9;
    /// MDF clock enable
    pub const MDF: u32 = 1 << 12;
    /// ADF clock enable
    pub const ADF: u32 = 1 << 13;
}
