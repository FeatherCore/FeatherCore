//! STM32U5 Chip Support
//! 
//! This module provides comprehensive drivers and initialization for
//! STM32U5 series microcontrollers (Cortex-M33 with TrustZone).
//!
//! # Features
//! - ARM Cortex-M33 core with TrustZone
//! - Up to 160 MHz CPU frequency
//! - Ultra-low power consumption
//! - Hardware encryption accelerator
//! - TrustZone security
//!
//! # Reference Documents
//! - RM0456: STM32U5 Series Reference Manual
//! - DS14395: STM32U5 Datasheet
//!
//! # Module Overview
//! ## Core
//! - `rcc`: Reset and Clock Control
//! - `gpio`: General Purpose I/O
//! - `nvic`: Nested Vectored Interrupt Controller
//! - `pwr`: Power Control
//! - `flash`: Flash Memory Interface
//! - `dma`: Direct Memory Access
//!
//! ## Communication
//! - `usart`: Universal Synchronous/Asynchronous Receiver/Transmitter
//! - `i2c`: Inter-Integrated Circuit
//! - `spi`: Serial Peripheral Interface
//! - `can`: Flexible Data-rate CAN
//! - `usb`: USB On-The-Go Full Speed
//!
//! ## Storage
//! - `sdmmc`: SD/SDIO/MMC Interface
//! - `octospi`: Octal Serial Peripheral Interface
//! - `fmc`: Flexible Memory Controller
//!
//! ## Analog
//! - `adc`: Analog-to-Digital Converter
//! - `dac`: Digital-to-Analog Converter
//! - `opamp`: Operational Amplifier
//! - `comp`: Comparator
//! - `vrefbuf`: Voltage Reference Buffer
//!
//! ## Timing
//! - `timer`: General Purpose Timers
//! - `lptim`: Low Power Timers
//! - `rtc`: Real-Time Clock
//!
//! ## Security
//! - `aes`: Advanced Encryption Standard
//! - `hash`: Hash Processor (SHA, MD5)
//! - `rng`: Random Number Generator
//! - `crc`: Cyclic Redundancy Check
//!
//! ## Watchdog
//! - `iwdg`: Independent Watchdog
//! - `wwdg`: Window Watchdog
//!
//! ## Audio
//! - `sai`: Serial Audio Interface
//!
//! ## Touch
//! - `tsc`: Touch Sensing Controller
//!
//! ## Display
//! - `ltdc`: LCD-TFT Display Controller
//!
//! ## Camera
//! - `dcmi`: Digital Camera Interface
//!
//! ## Math Accelerators
//! - `cordic`: Coordinate Rotation Digital Computer
//! - `fmac`: Filter Math Accelerator
//! - `hrtim`: High Resolution Timer
//!
//! ## Digital Filters
//! - `mdf`: Multi-function Digital Filter
//! - `adf`: Audio Digital Filter
//!
//! ## Interface
//! - `pssi`: Parallel Slave Interface
//! - `ucpd`: USB Type-C Power Delivery
//! - `i3c`: Improved Inter-Integrated Circuit
//! - `exti`: External Interrupt/Event Controller

//! ## Power
//! - `smps`: Switched-Mode Power Supply

//! ## Security
//! - `pka`: Public Key Accelerator

#![no_std]

pub mod rcc;
pub mod gpio;
pub mod usart;
pub mod i2c;
pub mod spi;
pub mod timer;
pub mod pwr;
pub mod flash;
pub mod nvic;
pub mod dma;
pub mod adc;
pub mod dac;
pub mod rtc;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rng;
pub mod aes;
pub mod hash;
pub mod can;
pub mod usb;
pub mod sdmmc;
pub mod octospi;
pub mod fmc;
pub mod sai;
pub mod tsc;
pub mod opamp;
pub mod comp;
pub mod vrefbuf;
pub mod ltdc;
pub mod dcmi;
pub mod cordic;
pub mod lptim;
pub mod fmac;
pub mod hrtim;
pub mod mdf;
pub mod adf;
pub mod pssi;
pub mod ucpd;
pub mod i3c;
pub mod exti;
pub mod smps;
pub mod pka;

/// Chip information
pub mod info {
    pub const CHIP_FAMILY: &str = "STM32U5";
    pub const CHIP_VENDOR: &str = "STMicroelectronics";
    pub const CPU_CORE: &str = "Cortex-M33";
    pub const CPU_FREQ_MAX: u32 = 160_000_000; // 160 MHz
    pub const FLASH_SIZE_MAX: u32 = 4 * 1024 * 1024; // 4 MB
    pub const SRAM_SIZE_MAX: u32 = 2 * 1024 * 1024; // 2 MB
}

/// Memory map addresses
pub mod memory {
    // Flash memory
    pub const FLASH_BASE: usize = 0x0800_0000;
    pub const FLASH_SIZE: usize = 2 * 1024 * 1024; // 2MB max
    
    // SRAM
    pub const SRAM1_BASE: usize = 0x2000_0000;
    pub const SRAM1_SIZE: usize = 768 * 1024; // 768KB SRAM1
    pub const SRAM2_BASE: usize = 0x200C_0000;
    pub const SRAM2_SIZE: usize = 64 * 1024;  // 64KB SRAM2
    pub const SRAM3_BASE: usize = 0x200D_0000;
    pub const SRAM3_SIZE: usize = 832 * 1024; // 832KB SRAM3 (on some variants)
    
    // Peripheral base
    pub const PERIPH_BASE: usize = 0x4000_0000;
    pub const PERIPH_BASE_NS: usize = 0x5000_0000; // Non-secure alias
    
    // APB1 peripherals
    pub const APB1_PERIPH_BASE: usize = PERIPH_BASE + 0x0000;
    // APB2 peripherals  
    pub const APB2_PERIPH_BASE: usize = PERIPH_BASE + 0x1000;
    // AHB1 peripherals
    pub const AHB1_PERIPH_BASE: usize = PERIPH_BASE + 0x2000;
    // AHB2 peripherals
    pub const AHB2_PERIPH_BASE: usize = PERIPH_BASE + 0x4000;
    // AHB3 peripherals (external memory)
    pub const AHB3_PERIPH_BASE: usize = PERIPH_BASE + 0x6000;
}

/// Initialize chip hardware
/// 
/// This function performs basic chip initialization:
/// 1. Configure power supply
/// 2. Enable HSI oscillator
/// 3. Configure flash latency
/// 4. Set up vector table
/// 5. Enable GPIO clocks
pub fn init() {
    // Initialize power controller
    pwr::init();
    
    // Initialize RCC (Reset and Clock Control)
    rcc::init();
    
    // Initialize flash controller
    flash::init();
    
    // Initialize NVIC
    nvic::init();
    
    // Initialize GPIO
    gpio::init();
}

/// Initialize chip with full clock configuration
/// 
/// This function configures the system to run at maximum frequency (160 MHz)
/// using the PLL with HSI16 as input.
pub fn init_with_pll() {
    // Basic initialization first
    init();
    
    // Configure PLL for 160 MHz system clock
    rcc::configure_pll_160mhz();
}

/// System reset
pub fn system_reset() -> ! {
    unsafe {
        // Write to AIRCR register to request reset
        let aircr = 0xE000_ED0C as *mut u32;
        core::ptr::write_volatile(aircr, 0x05FA_0004);
    }
    loop {}
}

/// Get current system clock frequency in Hz
pub fn get_sysclk_freq() -> u32 {
    rcc::get_sysclk_freq()
}

/// Get AHB bus frequency in Hz
pub fn get_hclk_freq() -> u32 {
    rcc::get_hclk_freq()
}

/// Get APB1 bus frequency in Hz
pub fn get_pclk1_freq() -> u32 {
    rcc::get_pclk1_freq()
}

/// Get APB2 bus frequency in Hz
pub fn get_pclk2_freq() -> u32 {
    rcc::get_pclk2_freq()
}

/// Delay for specified milliseconds (blocking)
pub fn delay_ms(ms: u32) {
    timer::delay_ms(ms);
}

/// Delay for specified microseconds (blocking)
pub fn delay_us(us: u32) {
    timer::delay_us(us);
}

/// Enable global interrupts
pub fn enable_interrupts() {
    nvic::enable_interrupts();
}

/// Disable global interrupts
pub fn disable_interrupts() {
    nvic::disable_interrupts();
}

/// Enter low power mode
pub mod power {
    pub use super::pwr::*;
    
    /// Enter Sleep mode
    pub fn sleep() {
        pwr::enter_sleep_mode();
    }
    
    /// Enter Stop mode
    pub fn stop(mode: pwr::LowPowerMode) {
        pwr::enter_stop_mode(mode);
    }
    
    /// Enter Standby mode
    pub fn standby() {
        pwr::enter_standby_mode();
    }
}

/// Debug utilities
pub mod debug {
    use super::usart;
    
    /// Initialize debug UART (USART1, 115200 baud)
    pub fn init() {
        usart::init_debug_usart(115200, super::get_pclk2_freq());
    }
    
    /// Print string to debug UART
    pub fn print(s: &str) {
        usart::debug_puts(s);
    }
    
    /// Print line to debug UART
    pub fn println(s: &str) {
        usart::debug_puts(s);
        usart::debug_puts("\r\n");
    }
    
    /// Print hexadecimal value
    pub fn print_hex(value: u32) {
        usart::debug_puts("0x");
        for i in (0..8).rev() {
            let nibble = (value >> (i * 4)) & 0xF;
            let c = if nibble < 10 {
                b'0' + nibble as u8
            } else {
                b'A' + (nibble - 10) as u8
            };
            usart::debug_putc(c);
        }
    }
}

/// Cryptography utilities
pub mod crypto {
    pub use super::aes;
    pub use super::hash;
    pub use super::rng;
    pub use super::crc;
}

/// Communication interfaces
pub mod comm {
    pub use super::usart;
    pub use super::i2c;
    pub use super::spi;
    pub use super::can;
    pub use super::usb;
}

/// Storage interfaces
pub mod storage {
    pub use super::sdmmc;
    pub use super::octospi;
    pub use super::fmc;
    pub use super::flash;
}

/// Analog peripherals
pub mod analog {
    pub use super::adc;
    pub use super::dac;
    pub use super::opamp;
    pub use super::comp;
    pub use super::vrefbuf;
}

/// Timing peripherals
pub mod timing {
    pub use super::timer;
    pub use super::lptim;
    pub use super::rtc;
}

/// Watchdog peripherals
pub mod watchdog {
    pub use super::iwdg;
    pub use super::wwdg;
}

/// Audio peripherals
pub mod audio {
    pub use super::sai;
}

/// Touch sensing
pub mod touch {
    pub use super::tsc;
}

/// Display peripherals
pub mod display {
    pub use super::ltdc;
}

/// Camera peripherals
pub mod camera {
    pub use super::dcmi;
}

/// Math accelerators
pub mod math {
    pub use super::cordic;
    pub use super::fmac;
}

/// Digital filters
pub mod filters {
    pub use super::mdf;
    pub use super::adf;
}

/// Interface peripherals
pub mod interface {
    pub use super::pssi;
    pub use super::ucpd;
}

/// High resolution timing
pub mod hrtiming {
    pub use super::hrtim;
}

/// Re-export commonly used types
pub use gpio::{Pin, Port, pins};
pub use timer::delay_ms as delay;
