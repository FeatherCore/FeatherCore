//! STM32U5 Series Microcontroller Support
//! STM32U5系列微控制器支持
//!
//! This module provides comprehensive driver support for the STM32U5 series
//! of microcontrollers (ARM Cortex-M33 with TrustZone).
//!
//! # Overview / 概述
//! The STM32U5 series is an ultra-low-power microcontroller family based on
//! the ARM Cortex-M33 processor, featuring:
//! - ARM Cortex-M33 core with TrustZone security
//! - Up to 160 MHz CPU frequency
//! - Ultra-low power consumption
//! - Hardware encryption accelerators
//! - TrustZone security
//!
//! # Reference Documents / 参考文档
//! - **RM0456**: STM32U5 Series Reference Manual
//!   - Chapter 1: Documentation conventions
//!   - Chapter 2: Memory and bus architecture
//!   - Chapter 3: System security
//!   - Chapter 4: Boot modes
//!   - Chapter 5: Global TrustZone controller (GTZC)
//!   - Chapter 6: RAM configuration controller (RAMCFG)
//!   - Chapter 7: Embedded flash memory (FLASH)
//!   - Chapter 8: Instruction cache (ICACHE)
//!   - Chapter 9: Data cache (DCACHE)
//!   - Chapter 10: Power control (PWR)
//!   - Chapter 11: Reset and clock control (RCC)
//!   - Chapter 12: Clock recovery system (CRS)
//!   - Chapter 13: General-purpose I/Os (GPIO)
//!   - Chapter 14: Low-power GPIO (LPGPIO)
//!   - Chapter 15: System configuration controller (SYSCFG)
//!   - Chapter 17: General purpose DMA (GPDMA)
//!   - Chapter 18: Low-power DMA (LPDMA)
//!   - Chapter 22: Nested vectored interrupt controller (NVIC)
//!   - Chapter 23: Extended interrupts and events (EXTI)
//!   - Chapter 24: Cyclic redundancy check (CRC)
//!   - Chapter 25: CORDIC coprocessor
//!   - Chapter 26: Filter math accelerator (FMAC)
//!   - Chapter 27: Flexible static memory controller (FSMC)
//!   - Chapter 28: Octo-SPI interface (OCTOSPI)
//!   - Chapter 31: SDMMC interface
//!   - Chapter 33: Analog-to-digital converter (ADC12)
//!   - Chapter 34: Analog-to-digital converter (ADC4)
//!   - Chapter 35: Digital-to-analog converter (DAC)
//!   - Chapter 36: Voltage reference buffer (VREFBUF)
//!   - Chapter 37: Comparator (COMP)
//!   - Chapter 38: Operational amplifier (OPAMP)
//!   - Chapter 39: Multi-function digital filter (MDF)
//!   - Chapter 40: Audio digital filter (ADF)
//!   - Chapter 41: Digital camera interface (DCMI)
//!   - Chapter 42: Parallel synchronous slave interface (PSSI)
//!   - Chapter 43: LCD-TFT display controller (LTDC)
//!   - Chapter 44: Chrom-ART accelerator (DMA2D) V1
//!   - Chapter 45: Chrom-ART accelerator (DMA2D) V2
//!   - Chapter 47: Touch sensing controller (TSC)
//!   - Chapter 48: True random number generator (RNG)
//!   - Chapter 49: AES hardware accelerator (AES)
//!   - Chapter 50: Secure AES coprocessor (SAES)
//!   - Chapter 51: Hash processor (HASH)
//!   - Chapter 52: On-the-fly decryption engine (OTFDEC)
//!   - Chapter 53: Public key accelerator (PKA)
//!   - Chapter 54: Advanced-control timers (TIM1/TIM8)
//!   - Chapter 55: General-purpose timers (TIM2/TIM3/TIM4/TIM5)
//!   - Chapter 56: General purpose timers (TIM15/TIM16/TIM17)
//!   - Chapter 57: Basic timers (TIM6/TIM7)
//!   - Chapter 58: Low-power timer (LPTIM)
//!   - Chapter 61: Independent watchdog (IWDG)
//!   - Chapter 62: System window watchdog (WWDG)
//!   - Chapter 63: Real-time clock (RTC)
//!   - Chapter 65: Inter-integrated circuit interface (I2C)
//!   - Chapter 66-67: USART/UART interfaces
//!   - Chapter 68: Serial peripheral interface (SPI)
//!   - Chapter 69: Serial audio interface (SAI)
//!   - Chapter 70: FDCAN controller
//!   - Chapter 71: USB device firmware upgrade (DFU)
//!   - Chapter 72: USB on-the-go full-speed (OTG_FS)
//!   - Chapter 73: USB on-the-go high-speed (OTG_HS)
//!   - Chapter 74: USB Type-C Power Delivery (UCPD)
//!
//! # Chip Features / 芯片特性
//! ## Core / 内核
//! - `rcc`: Reset and Clock Control - 复位和时钟控制
//! - `gpio`: General Purpose I/O - 通用输入输出
//! - `nvic`: Nested Vectored Interrupt Controller - 嵌套向量中断控制器
//! - `pwr`: Power Control - 电源控制
//! - `flash`: Flash Memory Interface - 闪存接口
//! - `dma`: Direct Memory Access - 直接内存访问
//! - `exti`: External Interrupt/Event Controller - 外部中断/事件控制器
//!
//! ## Communication / 通信接口
//! - `usart`: Universal Synchronous/Asynchronous Receiver/Transmitter
//!            通用同步/异步收发器
//! - `i2c`: Inter-Integrated Circuit - 集成电路总线
//! - `spi`: Serial Peripheral Interface - 串行外设接口
//! - `can`: Flexible Data-rate CAN - 灵活数据率CAN
//! - `usb`: USB On-The-Go Full/High Speed - USB全速/高速
//! - `i3c`: Improved Inter-Integrated Circuit - 改进型I2C
//!
//! ## Storage / 存储接口
//! - `sdmmc`: SD/SDIO/MMC Interface - SD/SDIO/MMC接口
//! - `octospi`: Octal Serial Peripheral Interface - 八通道SPI
//! - `fmc`: Flexible Memory Controller - 灵活存储控制器
//!
//! ## Analog / 模拟外设
//! - `adc`: Analog-to-Digital Converter - 模数转换器
//! - `dac`: Digital-to-Analog Converter - 数模转换器
//! - `opamp`: Operational Amplifier - 运算放大器
//! - `comp`: Comparator - 比较器
//! - `vrefbuf`: Voltage Reference Buffer - 电压参考缓冲器
//!
//! ## Timing / 定时器
//! - `timer`: General Purpose Timers - 通用定时器
//! - `lptim`: Low Power Timer - 低功耗定时器
//! - `rtc`: Real-Time Clock - 实时时钟
//! - `hrtim`: High Resolution Timer - 高分辨率定时器
//!
//! ## Security / 安全外设
//! - `aes`: Advanced Encryption Standard - 高级加密标准
//! - `hash`: Hash Processor - 哈希处理器
//! - `rng`: Random Number Generator - 随机数生成器
//! - `pka`: Public Key Accelerator - 公钥加速器
//! - `gtzc`: Global TrustZone Controller - 全局TrustZone控制器
//!
//! ## Watchdog / 看门狗
//! - `iwdg`: Independent Watchdog - 独立看门狗
//! - `wwdg`: Window Watchdog - 窗口看门狗
//!
//! ## Audio / 音频
//! - `sai`: Serial Audio Interface - 串行音频接口
//! - `mdf`: Multi-function Digital Filter - 多功能数字滤波器
//! - `adf`: Audio Digital Filter - 音频数字滤波器
//!
//! ## Touch / 触摸感应
//! - `tsc`: Touch Sensing Controller - 触摸感应控制器
//!
//! ## Display / 显示
//! - `ltdc`: LCD-TFT Display Controller - LCD-TFT显示控制器
//!
//! ## Camera / 摄像头
//! - `dcmi`: Digital Camera Interface - 数字摄像头接口
//! - `pssi`: Parallel Slave Interface - 并行从机接口
//!
//! ## Math Accelerators / 数学加速器
//! - `cordic`: Coordinate Rotation Digital Computer - 坐标旋转数字计算机
//! - `fmac`: Filter Math Accelerator - 滤波器数学加速器
//!
//! ## Power Management / 电源管理
//! - `smps`: Switched-Mode Power Supply - 开关模式电源
//!
//! ## Other / 其他
//! - `crc`: Cyclic Redundancy Check - 循环冗余校验
//! - `ramcfg`: RAM Configuration Controller - RAM配置控制器
//! - `ucpd`: USB Type-C Power Delivery - USB Type-C功率传输

#![no_std]
#![allow(unused)]

// Public modules / 公共模块
pub mod rcc;
pub mod gpio;
pub mod usart;
pub mod i2c;
pub mod spi;
pub mod timer;
pub mod pwr;
pub mod flash;
pub mod gtzc;
pub mod ramcfg;
pub mod nvic;
pub mod gpdma;
pub mod adc;
pub mod dac;
pub mod rtc;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rng;
pub mod aes;
pub mod hash;
pub mod lpdma;
pub mod dma2d_v1;
pub mod dma2d_v2;
pub mod saes;
pub mod otfdec;
pub mod can;
pub mod usb;
pub mod usb_dfu;
pub mod usb_drd;
pub mod usb_otg_fs;
pub mod usb_otg_hs;
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

pub mod i3c_adapter;
pub mod syscfg;
pub mod lpgpio;
pub mod icache;
pub mod dcache;
pub mod crs;

/// Chip information / 芯片信息
pub mod info {
    /// Chip family name / 芯片系列名称
    pub const CHIP_FAMILY: &str = "STM32U5";

    /// Chip vendor / 芯片供应商
    pub const CHIP_VENDOR: &str = "STMicroelectronics";

    /// CPU core type / CPU核心类型
    pub const CPU_CORE: &str = "Cortex-M33";

    /// Maximum CPU frequency in Hz / 最大CPU频率(Hz)
    pub const CPU_FREQ_MAX: u32 = 160_000_000; // 160 MHz

    /// Maximum Flash size in bytes / 最大闪存大小(字节)
    pub const FLASH_SIZE_MAX: u32 = 4 * 1024 * 1024; // 4 MB

    /// Maximum SRAM size in bytes / 最大SRAM大小(字节)
    pub const SRAM_SIZE_MAX: u32 = 2 * 1024 * 1024; // 2 MB

    /// Package types / 封装类型
    pub mod packages {
        pub const LQFP64: &str = "LQFP64";
        pub const LQFP100: &str = "LQFP100";
        pub const LQFP144: &str = "LQFP144";
        pub const LQFP176: &str = "LQFP176";
        pub const UFBGA132: &str = "UFBGA132";
        pub const UFBGA169: &str = "UFBGA169";
        pub const WLCSP100: &str = "WLCSP100";
        pub const UFBGA225: &str = "UFBGA225";
    }
}

/// Memory map addresses / 内存映射地址
///
/// Reference: RM0456 Chapter 2: Memory and bus architecture
pub mod memory {
    /// Flash memory base address / 闪存基地址
    pub const FLASH_BASE: usize = 0x0800_0000;

    /// Flash size (varies by device) / 闪存大小(因设备而异)
    pub const FLASH_SIZE: usize = 2 * 1024 * 1024; // 2MB default

    /// SRAM1 base address / SRAM1基地址
    pub const SRAM1_BASE: usize = 0x2000_0000;

    /// SRAM1 size / SRAM1大小
    pub const SRAM1_SIZE: usize = 768 * 1024; // 768KB

    /// SRAM2 base address / SRAM2基地址
    pub const SRAM2_BASE: usize = 0x200C_0000;

    /// SRAM2 size / SRAM2大小
    pub const SRAM2_SIZE: usize = 64 * 1024; // 64KB

    /// SRAM3 base address / SRAM3基地址
    pub const SRAM3_BASE: usize = 0x200D_0000;

    /// SRAM3 size / SRAM3大小
    pub const SRAM3_SIZE: usize = 832 * 1024; // 832KB (on some variants)

    /// Backup SRAM base address / 备份SRAM基地址
    pub const BKPSRAM_BASE: usize = 0x4002_4000;

    /// Peripheral base address (non-secure) / 外设基地址(非安全)
    pub const PERIPH_BASE: usize = 0x4000_0000;

    /// Peripheral base address (non-secure alias) / 外设基地址(非安全别名)
    pub const PERIPH_BASE_NS: usize = 0x5000_0000;

    /// APB1 peripheral base / APB1外设基地址
    pub const APB1_PERIPH_BASE: usize = PERIPH_BASE + 0x0000;

    /// APB2 peripheral base / APB2外设基地址
    pub const APB2_PERIPH_BASE: usize = PERIPH_BASE + 0x1000;

    /// AHB1 peripheral base / AHB1外设基地址
    pub const AHB1_PERIPH_BASE: usize = PERIPH_BASE + 0x2000;

    /// AHB2 peripheral base / AHB2外设基地址
    pub const AHB2_PERIPH_BASE: usize = PERIPH_BASE + 0x4000;

    /// AHB3 peripheral base (external memory) / AHB3外设基地址(外部存储器)
    pub const AHB3_PERIPH_BASE: usize = PERIPH_BASE + 0x6000;

    // Common peripheral addresses / 常用外设地址

    /// RCC base address / RCC基地址
    /// Reference: RM0456 Chapter 11, Section 11.1
    pub const RCC_BASE: usize = 0x4002_1000;

    /// PWR base address / PWR基地址
    /// Reference: RM0456 Chapter 10, Section 10.1
    pub const PWR_BASE: usize = 0x4002_0000;

    /// Flash controller base address / 闪存控制器基地址
    /// Reference: RM0456 Chapter 7, Section 7.1
    pub const FLASH_BASE_ADDR: usize = 0x4002_2000;

    /// GPIO base addresses / GPIO基地址
    pub mod gpio {
        use super::AHB2_PERIPH_BASE;
        pub const GPIOA_BASE: usize = AHB2_PERIPH_BASE + 0x0000;
        pub const GPIOB_BASE: usize = AHB2_PERIPH_BASE + 0x0400;
        pub const GPIOC_BASE: usize = AHB2_PERIPH_BASE + 0x0800;
        pub const GPIOD_BASE: usize = AHB2_PERIPH_BASE + 0x0C00;
        pub const GPIOE_BASE: usize = AHB2_PERIPH_BASE + 0x1000;
        pub const GPIOF_BASE: usize = AHB2_PERIPH_BASE + 0x1400;
        pub const GPIOG_BASE: usize = AHB2_PERIPH_BASE + 0x1800;
        pub const GPIOH_BASE: usize = AHB2_PERIPH_BASE + 0x1C00;
        pub const GPIOI_BASE: usize = AHB2_PERIPH_BASE + 0x2000;
    }
}

/// Initialize chip hardware / 初始化芯片硬件
///
/// This function performs basic chip initialization:
/// 1. Configure power supply (PWR)
/// 2. Enable HSI16 oscillator (RCC)
/// 3. Configure flash latency (FLASH)
/// 4. Set up vector table (NVIC)
/// 5. Enable GPIO clocks (RCC)
///
/// Reference: RM0456 for initialization sequence
pub fn init() {
    // Initialize power controller
    // Reference: RM0456 Chapter 10: Power control
    pwr::init();

    // Initialize RCC (Reset and Clock Control)
    // Reference: RM0456 Chapter 11: Reset and clock control
    rcc::init();

    // Initialize flash controller
    // Reference: RM0456 Chapter 7: Embedded flash memory
    flash::init();

    // Initialize NVIC
    // Reference: RM0456 Chapter 22: Nested vectored interrupt controller
    nvic::init();

    // Initialize GPIO ports
    // Reference: RM0456 Chapter 13: General-purpose I/Os
    gpio::init();
}

/// Initialize chip with PLL for maximum frequency / 使用PLL初始化芯片以获得最大频率
///
/// This function configures the system to run at maximum frequency (160 MHz)
/// using PLL1 with HSI16 as input.
///
/// Clock configuration:
/// - PLL1 input: HSI16 (16 MHz)
/// - PLL1 divider (M): 4
/// - PLL1 multiplier (N): 40
/// - PLL1 output: 160 MHz
/// - System clock: PLL1_R
///
/// Reference: RM0456 Chapter 11, Section 11.4
pub fn init_with_pll() {
    // Basic initialization first
    init();

    // Configure PLL for 160 MHz system clock
    rcc::configure_pll_160mhz();
}

/// System reset / 系统复位
///
/// Initiates a system reset through the ARM Cortex-M33 AIRCR register.
///
/// Reference: RM0456 Chapter 11, Section 11.3.2
pub fn system_reset() -> ! {
    unsafe {
        // Write to AIRCR register to request reset
        // Bits[7:0] = 0xFA (key)
        // Bit[2] = SYSRESETREQ
        let aircr = 0xE000_ED0C as *mut u32;
        core::ptr::write_volatile(aircr, 0x05FA_0004);
    }
    loop {}
}

/// Get current system clock frequency in Hz / 获取当前系统时钟频率(Hz)
///
/// Returns the current SYSCLK frequency.
///
/// Reference: RM0456 Chapter 11, Section 11.4
pub fn get_sysclk_freq() -> u32 {
    rcc::get_sysclk_freq()
}

/// Get AHB bus frequency in Hz / 获取AHB总线频率(Hz)
///
/// Returns the current HCLK frequency.
///
/// Reference: RM0456 Chapter 11, Section 11.4
pub fn get_hclk_freq() -> u32 {
    rcc::get_hclk_freq()
}

/// Get APB1 bus frequency in Hz / 获取APB1总线频率(Hz)
///
/// Returns the current PCLK1 frequency.
///
/// Reference: RM0456 Chapter 11, Section 11.4
pub fn get_pclk1_freq() -> u32 {
    rcc::get_pclk1_freq()
}

/// Get APB2 bus frequency in Hz / 获取APB2总线频率(Hz)
///
/// Returns the current PCLK2 frequency.
///
/// Reference: RM0456 Chapter 11, Section 11.4
pub fn get_pclk2_freq() -> u32 {
    rcc::get_pclk2_freq()
}

/// Get APB3 bus frequency in Hz / 获取APB3总线频率(Hz)
///
/// Returns the current PCLK3 frequency.
///
/// Reference: RM0456 Chapter 11, Section 11.4
pub fn get_pclk3_freq() -> u32 {
    rcc::get_pclk3_freq()
}

/// Delay for specified milliseconds (blocking) / 延时指定毫秒(阻塞)
///
/// Uses a busy-wait loop based on the system clock.
///
/// Reference: RM0456 Chapter 55: General-purpose timers
pub fn delay_ms(ms: u32) {
    timer::delay_ms(ms);
}

/// Delay for specified microseconds (blocking) / 延时指定微秒(阻塞)
///
/// Uses a busy-wait loop based on the system clock.
///
/// Reference: RM0456 Chapter 55: General-purpose timers
pub fn delay_us(us: u32) {
    timer::delay_us(us);
}

/// Enable global interrupts / 全局中断使能
///
/// Enables interrupts by clearing the PRIMASK register.
///
/// Reference: ARM Cortex-M33 Programming Manual
pub fn enable_interrupts() {
    nvic::enable_interrupts();
}

/// Disable global interrupts / 全局中断禁用
///
/// Disables interrupts by setting the PRIMASK register.
///
/// Reference: ARM Cortex-M33 Programming Manual
pub fn disable_interrupts() {
    nvic::disable_interrupts();
}

/// Power management functions / 电源管理功能
///
/// Low-power mode control based on PWR peripheral.
///
/// Reference: RM0456 Chapter 10: Power control
pub mod power {
    pub use super::pwr::*;

    /// Enter Sleep mode / 进入睡眠模式
    ///
    /// In Sleep mode, the CPU clock is stopped but all peripherals continue
    /// to operate. The device can wake up from Sleep mode by any peripheral
    /// with an interrupt/event or by an external reset.
    ///
    /// Reference: RM0456 Chapter 10, Section 10.7.5
    pub fn sleep() {
        pwr::enter_sleep_mode();
    }

    /// Enter Stop mode / 进入停止模式
    ///
    /// In Stop mode, all clocks in the VCORE domain are stopped, and the
    /// PLL, HSI16, and HSE oscillators are disabled. Some peripherals can
    /// continue to operate depending on configuration.
    ///
    /// Reference: RM0456 Chapter 10, Section 10.7.6-10.7.9
    pub fn stop(mode: pwr::LowPowerMode) {
        pwr::enter_stop_mode(mode);
    }

    /// Enter Standby mode / 进入待机模式
    ///
    /// In Standby mode, the VCORE domain is powered off. Only the backup
    /// domain (RTC, backup registers, LSE, LSI) remains powered.
    ///
    /// Reference: RM0456 Chapter 10, Section 10.7.10
    pub fn standby() {
        pwr::enter_standby_mode();
    }

    /// Enter Shutdown mode / 进入关机模式
    ///
    /// Shutdown mode offers the lowest power consumption. Only the backup
    /// domain remains powered (RTC and LPUART if configured).
    ///
    /// Reference: RM0456 Chapter 10, Section 10.7.11
    pub fn shutdown() {
        pwr::enter_shutdown_mode();
    }
}

/// Debug utilities / 调试工具
///
/// Debug UART functionality for printing messages.
///
/// Reference: RM0456 Chapter 66: USART
pub mod debug {
    use super::usart;

    /// Initialize debug UART (USART1, 115200 baud) / 初始化调试UART
    pub fn init() {
        usart::init_debug_usart(115200, super::get_pclk2_freq());
    }

    /// Print string to debug UART / 向调试UART打印字符串
    pub fn print(s: &str) {
        usart::debug_puts(s);
    }

    /// Print line to debug UART / 向调试UART打印行
    pub fn println(s: &str) {
        usart::debug_puts(s);
        usart::debug_puts("\r\n");
    }

    /// Print hexadecimal value / 打印十六进制值
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

/// Cryptography utilities / 加密工具
///
/// Hardware-accelerated cryptographic functions.
///
/// Reference: RM0456 Chapters 48-53
pub mod crypto {
    pub use super::aes;
    pub use super::hash;
    pub use super::rng;
    pub use super::pka;
    pub use super::crc;
}

/// Communication interfaces / 通信接口
///
/// Serial communication peripherals.
///
/// Reference: RM0456 Chapters 65-73
pub mod comm {
    pub use super::usart;
    pub use super::i2c;
    pub use super::spi;
    pub use super::can;
    pub use super::usb;
    pub use super::i3c;
}

/// Storage interfaces / 存储接口
///
/// External memory and storage controllers.
///
/// Reference: RM0456 Chapters 27-32
pub mod storage {
    pub use super::sdmmc;
    pub use super::octospi;
    pub use super::fmc;
    pub use super::flash;
}

/// Analog peripherals / 模拟外设
///
/// ADC, DAC, OPAMP, Comparator, VREF.
///
/// Reference: RM0456 Chapters 33-38
pub mod analog {
    pub use super::adc;
    pub use super::dac;
    pub use super::opamp;
    pub use super::comp;
    pub use super::vrefbuf;
}

/// Timing peripherals / 定时器外设
///
/// Timer and RTC peripherals.
///
/// Reference: RM0456 Chapters 54-63
pub mod timing {
    pub use super::timer;
    pub use super::lptim;
    pub use super::rtc;
    pub use super::hrtim;
}

/// Watchdog peripherals / 看门狗外设
///
/// Independent and Window Watchdogs.
///
/// Reference: RM0456 Chapters 61-62
pub mod watchdog {
    pub use super::iwdg;
    pub use super::wwdg;
}

/// Audio peripherals / 音频外设
///
/// Serial Audio Interface and digital filters.
///
/// Reference: RM0456 Chapters 39-40, 69
pub mod audio {
    pub use super::sai;
    pub use super::mdf;
    pub use super::adf;
}

/// Touch sensing / 触摸感应
///
/// Touch sensing controller.
///
/// Reference: RM0456 Chapter 47
pub mod touch {
    pub use super::tsc;
}

/// Display peripherals / 显示外设
///
/// LCD-TFT display controller.
///
/// Reference: RM0456 Chapter 43
pub mod display {
    pub use super::ltdc;
}

/// Camera peripherals / 摄像头外设
///
/// Digital camera interface and PSSI.
///
/// Reference: RM0456 Chapters 41-42
pub mod camera {
    pub use super::dcmi;
    pub use super::pssi;
}

/// Math accelerators / 数学加速器
///
/// CORDIC and FMAC peripherals.
///
/// Reference: RM0456 Chapters 25-26
pub mod math {
    pub use super::cordic;
    pub use super::fmac;
}

/// Digital filters / 数字滤波器
///
/// MDF and ADF peripherals.
///
/// Reference: RM0456 Chapters 39-40
pub mod filters {
    pub use super::mdf;
    pub use super::adf;
}

/// Interface peripherals / 接口外设
///
/// USB Type-C PD and other interfaces.
///
/// Reference: RM0456 Chapters 74
pub mod interface {
    pub use super::pssi;
    pub use super::ucpd;
}

/// High resolution timing / 高分辨率定时
///
/// High Resolution Timer.
///
/// Reference: RM0456 Chapter 37
pub mod hrtiming {
    pub use super::hrtim;
}

/// Re-export commonly used types / 导出常用类型
pub use gpio::{Pin, Port, pins};
pub use timer::delay_ms as delay;
