//! RCC - Reset and Clock Control
//! 复位和时钟控制
//!
//! STM32U5 RCC 模块负责管理芯片的时钟系统和复位功能。
//! 支持多种时钟源:HSI16、HSE、MSI、LSI、LSE、HSI48、PLL
//!
//! ## 时钟源 / Clock Sources
//! - **HSI16**: 16 MHz 高速内部振荡器 (High Speed Internal)
//! - **HSE**: 高速外部振荡器 (High Speed External), 4-48 MHz
//! - **MSI**: 多速内部振荡器 (Multi-Speed Internal), 100 kHz-48 MHz
//! - **LSI**: 32 kHz 低速内部振荡器 (Low Speed Internal)
//! - **LSE**: 32.768 kHz 低速外部振荡器 (Low Speed External)
//! - **HSI48**: 48 MHz 高速内部振荡器,用于 USB
//! - **PLL**: 锁相环,支持 160 MHz 输出
//!
//! ## 总线时钟 / Bus Clocks
//! - **SYSCLK**: 系统时钟,最高 160 MHz
//! - **HCLK**: AHB 总线时钟
//! - **PCLK1**: APB1 总线时钟,最高 160 MHz
//! - **PCLK2**: APB2 总线时钟,最高 160 MHz
//! - **PCLK3**: APB3 总线时钟,最高 160 MHz

/// RCC base address / RCC 基地址
pub const RCC_BASE: usize = 0x4002_1000;

/// RCC register offsets / RCC 寄存器偏移
pub mod reg {
    /// Clock control register / 时钟控制寄存器
    pub const CR: usize = 0x00;
    /// Internal clock sources calibration register / 内部时钟源校准寄存器
    pub const ICSCR: usize = 0x04;
    /// Clock configuration register / 时钟配置寄存器
    pub const CFGR: usize = 0x08;
    /// PLL configuration register / PLL 配置寄存器
    pub const PLLCFGR: usize = 0x0C;
    /// PLL divider configuration register / PLL 分频配置寄存器
    pub const PLLDIVR: usize = 0x10;
    /// PLL fractional divider register / PLL 分数分频寄存器
    pub const PLLFRACR: usize = 0x14;
    /// Clock interrupt enable register / 时钟中断使能寄存器
    pub const CIER: usize = 0x18;
    /// Clock interrupt flag register / 时钟中断标志寄存器
    pub const CIFR: usize = 0x1C;
    /// Clock interrupt clear register / 时钟中断清除寄存器
    pub const CICR: usize = 0x20;
    /// AHB1 peripheral reset register / AHB1 外设复位寄存器
    pub const AHB1RSTR: usize = 0x28;
    /// AHB2 peripheral reset register / AHB2 外设复位寄存器
    pub const AHB2RSTR: usize = 0x2C;
    /// AHB3 peripheral reset register / AHB3 外设复位寄存器
    pub const AHB3RSTR: usize = 0x30;
    /// APB1 peripheral reset register 1 / APB1 外设复位寄存器 1
    pub const APB1RSTR1: usize = 0x34;
    /// APB1 peripheral reset register 2 / APB1 外设复位寄存器 2
    pub const APB1RSTR2: usize = 0x38;
    /// APB2 peripheral reset register / APB2 外设复位寄存器
    pub const APB2RSTR: usize = 0x3C;
    /// APB3 peripheral reset register / APB3 外设复位寄存器
    pub const APB3RSTR: usize = 0x40;
    /// AHB1 peripheral clock enable register / AHB1 外设时钟使能寄存器
    pub const AHB1ENR: usize = 0x48;
    /// AHB2 peripheral clock enable register / AHB2 外设时钟使能寄存器
    pub const AHB2ENR: usize = 0x4C;
    /// AHB3 peripheral clock enable register / AHB3 外设时钟使能寄存器
    pub const AHB3ENR: usize = 0x50;
    /// APB1 peripheral clock enable register 1 / APB1 外设时钟使能寄存器 1
    pub const APB1ENR1: usize = 0x54;
    /// APB1 peripheral clock enable register 2 / APB1 外设时钟使能寄存器 2
    pub const APB1ENR2: usize = 0x58;
    /// APB2 peripheral clock enable register / APB2 外设时钟使能寄存器
    pub const APB2ENR: usize = 0x5C;
    /// APB3 peripheral clock enable register / APB3 外设时钟使能寄存器
    pub const APB3ENR: usize = 0x60;
    /// AHB1 peripheral clocks enable in Sleep and Stop modes register
    /// AHB1 外设睡眠和停止模式时钟使能寄存器
    pub const AHB1SMENR: usize = 0x68;
    /// AHB2 peripheral clocks enable in Sleep and Stop modes register
    /// AHB2 外设睡眠和停止模式时钟使能寄存器
    pub const AHB2SMENR: usize = 0x6C;
    /// AHB3 peripheral clocks enable in Sleep and Stop modes register
    /// AHB3 外设睡眠和停止模式时钟使能寄存器
    pub const AHB3SMENR: usize = 0x70;
    /// APB1 peripheral clocks enable in Sleep and Stop modes register 1
    /// APB1 外设睡眠和停止模式时钟使能寄存器 1
    pub const APB1SMENR1: usize = 0x74;
    /// APB1 peripheral clocks enable in Sleep and Stop modes register 2
    /// APB1 外设睡眠和停止模式时钟使能寄存器 2
    pub const APB1SMENR2: usize = 0x78;
    /// APB2 peripheral clocks enable in Sleep and Stop modes register
    /// APB2 外设睡眠和停止模式时钟使能寄存器
    pub const APB2SMENR: usize = 0x7C;
    /// APB3 peripheral clocks enable in Sleep and Stop modes register
    /// APB3 外设睡眠和停止模式时钟使能寄存器
    pub const APB3SMENR: usize = 0x80;
    /// Peripherals independent clock configuration register 1 / 外设独立时钟配置寄存器 1
    pub const CCIPR1: usize = 0x88;
    /// Peripherals independent clock configuration register 2 / 外设独立时钟配置寄存器 2
    pub const CCIPR2: usize = 0x8C;
    /// Peripherals independent clock configuration register 3 / 外设独立时钟配置寄存器 3
    pub const CCIPR3: usize = 0x90;
    /// Backup domain control register / 备份域控制寄存器
    pub const BDCR: usize = 0x94;
    /// Control/status register / 控制/状态寄存器
    pub const CSR: usize = 0x98;
    /// AHB3 peripheral clock enable register 2 / AHB3 外设时钟使能寄存器 2
    pub const AHB3ENR2: usize = 0x9C;
}

/// Clock sources / 时钟源
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockSource {
    /// High Speed Internal 16 MHz / 高速内部 16 MHz
    HSI16 = 0,
    /// High Speed External / 高速外部
    HSE = 1,
    /// Multi-Speed Internal (100 kHz to 48 MHz) / 多速内部 (100 kHz 到 48 MHz)
    MSI = 2,
    /// PLL clock / PLL 时钟
    PLL = 3,
}

/// System clock frequencies / 系统时钟频率
static mut SYSCLK_FREQ: u32 = 16_000_000; // Default HSI16 / 默认 HSI16
static mut HCLK_FREQ: u32 = 16_000_000;
static mut PCLK1_FREQ: u32 = 16_000_000;
static mut PCLK2_FREQ: u32 = 16_000_000;
static mut PCLK3_FREQ: u32 = 16_000_000;

/// Initialize RCC with default HSI16 clock / 使用默认 HSI16 时钟初始化 RCC
pub fn init() {
    unsafe {
        // Enable HSI16 oscillator / 使能 HSI16 振荡器
        let cr = (RCC_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= 1 << 0; // HSION
        core::ptr::write_volatile(cr, val);
        
        // Wait for HSI16 ready / 等待 HSI16 就绪
        while (core::ptr::read_volatile(cr) & (1 << 1)) == 0 {}
        
        // Select HSI16 as system clock / 选择 HSI16 作为系统时钟
        let cfgr = (RCC_BASE + reg::CFGR) as *mut u32;
        let mut val = core::ptr::read_volatile(cfgr);
        val &= !(0b11 << 0); // Clear SW bits / 清除 SW 位
        val |= 0b01 << 0;    // SW = 01 (HSI16)
        core::ptr::write_volatile(cfgr, val);
        
        // Wait for HSI16 to be used as system clock / 等待 HSI16 作为系统时钟
        while (core::ptr::read_volatile(cfgr) & (0b11 << 2)) != (0b01 << 2) {}
        
        // Update frequencies / 更新频率
        SYSCLK_FREQ = 16_000_000;
        HCLK_FREQ = 16_000_000;
        PCLK1_FREQ = 16_000_000;
        PCLK2_FREQ = 16_000_000;
        PCLK3_FREQ = 16_000_000;
    }
}

/// Configure PLL for 160 MHz system clock / 配置 PLL 为 160 MHz 系统时钟
/// 
/// PLL configuration: HSI16 / 4 * 40 / 1 = 160 MHz
/// PLL 配置:HSI16 / 4 * 40 / 1 = 160 MHz
pub fn configure_pll_160mhz() {
    unsafe {
        // Disable PLL before configuration / 配置前禁用 PLL
        let cr = (RCC_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val &= !(1 << 24); // PLL1ON = 0
        core::ptr::write_volatile(cr, val);
        
        // Wait for PLL to be disabled / 等待 PLL 禁用
        while (core::ptr::read_volatile(cr) & (1 << 25)) != 0 {}
        
        // Configure PLL: PLLSRC = HSI16 (01), PLLM = 4 (011), PLLN = 40
        // 配置 PLL:PLLSRC = HSI16 (01), PLLM = 4 (011), PLLN = 40
        let pllcfgr = (RCC_BASE + reg::PLLCFGR) as *mut u32;
        let mut val = 0;
        val |= 0b01 << 0;      // PLLSRC = HSI16
        val |= 0b011 << 4;     // PLLM = 4 (divide by 4) / 分频 4
        val |= 40 << 8;        // PLLN = 40 (multiply by 40) / 倍频 40
        val |= 0 << 24;        // PLLR = 0 (divide by 1) / 分频 1
        val |= 1 << 28;        // PLLREN = 1 (enable PLLR output) / 使能 PLLR 输出
        core::ptr::write_volatile(pllcfgr, val);
        
        // Enable PLL / 使能 PLL
        let cr = (RCC_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= 1 << 24; // PLL1ON = 1
        core::ptr::write_volatile(cr, val);
        
        // Wait for PLL ready / 等待 PLL 就绪
        while (core::ptr::read_volatile(cr) & (1 << 25)) == 0 {}
        
        // Configure flash latency for 160 MHz (4 wait states)
        // 为 160 MHz 配置 Flash 延迟 (4 个等待状态)
        let flash_acr = 0x4002_2000 as *mut u32;
        let mut val = core::ptr::read_volatile(flash_acr);
        val &= !(0xF << 0);
        val |= 4 << 0; // LATENCY = 4
        core::ptr::write_volatile(flash_acr, val);
        
        // Wait for flash latency to be set / 等待 Flash 延迟设置
        while (core::ptr::read_volatile(flash_acr) & 0xF) != 4 {}
        
        // Select PLL as system clock / 选择 PLL 作为系统时钟
        let cfgr = (RCC_BASE + reg::CFGR) as *mut u32;
        let mut val = core::ptr::read_volatile(cfgr);
        val &= !(0b11 << 0); // Clear SW bits / 清除 SW 位
        val |= 0b11 << 0;    // SW = 11 (PLL)
        core::ptr::write_volatile(cfgr, val);
        
        // Wait for PLL to be used as system clock / 等待 PLL 作为系统时钟
        while (core::ptr::read_volatile(cfgr) & (0b11 << 2)) != (0b11 << 2) {}
        
        // Update frequencies / 更新频率
        // PLL output: 16 MHz / 4 * 40 = 160 MHz
        SYSCLK_FREQ = 160_000_000;
        // HCLK = SYSCLK / 1 (HPRE = 0)
        HCLK_FREQ = 160_000_000;
        // PCLK1 = HCLK / 1 (PPRE1 = 0)
        PCLK1_FREQ = 160_000_000;
        // PCLK2 = HCLK / 1 (PPRE2 = 0)
        PCLK2_FREQ = 160_000_000;
        // PCLK3 = HCLK / 1 (PPRE3 = 0)
        PCLK3_FREQ = 160_000_000;
    }
}

/// Get current system clock frequency / 获取当前系统时钟频率
pub fn get_sysclk_freq() -> u32 {
    unsafe { SYSCLK_FREQ }
}

/// Get AHB bus frequency / 获取 AHB 总线频率
pub fn get_hclk_freq() -> u32 {
    unsafe { HCLK_FREQ }
}

/// Get APB1 bus frequency / 获取 APB1 总线频率
pub fn get_pclk1_freq() -> u32 {
    unsafe { PCLK1_FREQ }
}

/// Get APB2 bus frequency / 获取 APB2 总线频率
pub fn get_pclk2_freq() -> u32 {
    unsafe { PCLK2_FREQ }
}

/// Get APB3 bus frequency / 获取 APB3 总线频率
pub fn get_pclk3_freq() -> u32 {
    unsafe { PCLK3_FREQ }
}

/// Enable AHB1 peripheral clock / 使能 AHB1 外设时钟
pub fn enable_ahb1_clock(periph: u32) {
    unsafe {
        let ahb1enr = (RCC_BASE + reg::AHB1ENR) as *mut u32;
        let val = core::ptr::read_volatile(ahb1enr);
        core::ptr::write_volatile(ahb1enr, val | periph);
    }
}

/// Enable AHB2 peripheral clock / 使能 AHB2 外设时钟
pub fn enable_ahb2_clock(periph: u32) {
    unsafe {
        let ahb2enr = (RCC_BASE + reg::AHB2ENR) as *mut u32;
        let val = core::ptr::read_volatile(ahb2enr);
        core::ptr::write_volatile(ahb2enr, val | periph);
    }
}

/// Enable AHB3 peripheral clock / 使能 AHB3 外设时钟
pub fn enable_ahb3_clock(periph: u32) {
    unsafe {
        let ahb3enr = (RCC_BASE + reg::AHB3ENR) as *mut u32;
        let val = core::ptr::read_volatile(ahb3enr);
        core::ptr::write_volatile(ahb3enr, val | periph);
    }
}

/// Enable APB1 peripheral clock / 使能 APB1 外设时钟
pub fn enable_apb1_clock(periph: u32) {
    unsafe {
        let apb1enr1 = (RCC_BASE + reg::APB1ENR1) as *mut u32;
        let val = core::ptr::read_volatile(apb1enr1);
        core::ptr::write_volatile(apb1enr1, val | periph);
    }
}

/// Enable APB2 peripheral clock / 使能 APB2 外设时钟
pub fn enable_apb2_clock(periph: u32) {
    unsafe {
        let apb2enr = (RCC_BASE + reg::APB2ENR) as *mut u32;
        let val = core::ptr::read_volatile(apb2enr);
        core::ptr::write_volatile(apb2enr, val | periph);
    }
}

/// Enable APB3 peripheral clock / 使能 APB3 外设时钟
pub fn enable_apb3_clock(periph: u32) {
    unsafe {
        let apb3enr = (RCC_BASE + reg::APB3ENR) as *mut u32;
        let val = core::ptr::read_volatile(apb3enr);
        core::ptr::write_volatile(apb3enr, val | periph);
    }
}

/// AHB1 peripheral clock bits / AHB1 外设时钟位
pub mod ahb1 {
    /// DMA1 / 直接内存访问控制器 1
    pub const DMA1: u32 = 1 << 0;
    /// DMA2 / 直接内存访问控制器 2
    pub const DMA2: u32 = 1 << 1;
    /// DMAMUX1 / DMA 复用器 1
    pub const DMAMUX1: u32 = 1 << 2;
    /// DMAMUX2 / DMA 复用器 2
    pub const DMAMUX2: u32 = 1 << 3;
    /// Flash memory interface / Flash 存储器接口
    pub const FLASH: u32 = 1 << 8;
    /// CRC calculation unit / CRC 计算单元
    pub const CRC: u32 = 1 << 12;
    /// Touch sensing controller / 触摸感应控制器
    pub const TSC: u32 = 1 << 16;
    /// RAM configuration controller / RAM 配置控制器
    pub const RAMCFG: u32 = 1 << 17;
    /// MDMA controller / MDMA 控制器
    pub const MDMA: u32 = 1 << 20;
}

/// AHB2 peripheral clock bits / AHB2 外设时钟位
pub mod ahb2 {
    /// GPIO port A / GPIO 端口 A
    pub const GPIOA: u32 = 1 << 0;
    /// GPIO port B / GPIO 端口 B
    pub const GPIOB: u32 = 1 << 1;
    /// GPIO port C / GPIO 端口 C
    pub const GPIOC: u32 = 1 << 2;
    /// GPIO port D / GPIO 端口 D
    pub const GPIOD: u32 = 1 << 3;
    /// GPIO port E / GPIO 端口 E
    pub const GPIOE: u32 = 1 << 4;
    /// GPIO port F / GPIO 端口 F
    pub const GPIOF: u32 = 1 << 5;
    /// GPIO port G / GPIO 端口 G
    pub const GPIOG: u32 = 1 << 6;
    /// GPIO port H / GPIO 端口 H
    pub const GPIOH: u32 = 1 << 7;
    /// GPIO port I / GPIO 端口 I
    pub const GPIOI: u32 = 1 << 8;
    /// ADC1 and ADC2 / ADC1 和 ADC2
    pub const ADC12: u32 = 1 << 10;
    /// ADC3 / 模数转换器 3
    pub const ADC3: u32 = 1 << 11;
    /// DAC1 / 数模转换器 1
    pub const DAC1: u32 = 1 << 12;
    /// FDCAN1 / 灵活数据率 CAN 1
    pub const FDCAN1: u32 = 1 << 14;
    /// FDCAN2 / 灵活数据率 CAN 2
    pub const FDCAN2: u32 = 1 << 15;
    /// USB OTG FS / USB 全速 OTG
    pub const USB_OTG_FS: u32 = 1 << 21;
    /// AES / 高级加密标准
    pub const AES: u32 = 1 << 4;
    /// HASH / 哈希处理器
    pub const HASH: u32 = 1 << 5;
    /// RNG / 随机数生成器
    pub const RNG: u32 = 1 << 6;
    /// PKA / 公钥加速器
    pub const PKA: u32 = 1 << 8;
}

/// AHB3 peripheral clock bits / AHB3 外设时钟位
pub mod ahb3 {
    /// Flexible memory controller / 灵活存储控制器
    pub const FMC: u32 = 1 << 0;
    /// OctoSPI1 / 八通道 SPI 1
    pub const OCTOSPI1: u32 = 1 << 1;
    /// OctoSPI2 / 八通道 SPI 2
    pub const OCTOSPI2: u32 = 1 << 2;
    /// SDMMC1 / SD/SDIO/MMC 1
    pub const SDMMC1: u32 = 1 << 4;
    /// SDMMC2 / SD/SDIO/MMC 2
    pub const SDMMC2: u32 = 1 << 5;
    /// LCD-TFT display controller / LCD-TFT 显示控制器
    pub const LTDC: u32 = 1 << 8;
    /// Digital camera interface / 数字摄像头接口
    pub const DCMI: u32 = 1 << 10;
    /// PSSI / 并行 slave 接口
    pub const PSSI: u32 = 1 << 12;
}

/// APB1 peripheral clock bits / APB1 外设时钟位
pub mod apb1 {
    /// Timer 2 / 定时器 2
    pub const TIM2: u32 = 1 << 0;
    /// Timer 3 / 定时器 3
    pub const TIM3: u32 = 1 << 1;
    /// Timer 4 / 定时器 4
    pub const TIM4: u32 = 1 << 2;
    /// Timer 5 / 定时器 5
    pub const TIM5: u32 = 1 << 3;
    /// Timer 6 / 定时器 6
    pub const TIM6: u32 = 1 << 4;
    /// Timer 7 / 定时器 7
    pub const TIM7: u32 = 1 << 5;
    /// Window watchdog / 窗口看门狗
    pub const WWDG: u32 = 1 << 11;
    /// SPI 2 / 串行外设接口 2
    pub const SPI2: u32 = 1 << 14;
    /// SPI 3 / 串行外设接口 3
    pub const SPI3: u32 = 1 << 15;
    /// USART 2 / 通用同步/异步收发器 2
    pub const USART2: u32 = 1 << 17;
    /// USART 3 / 通用同步/异步收发器 3
    pub const USART3: u32 = 1 << 18;
    /// UART 4 / 通用异步收发器 4
    pub const UART4: u32 = 1 << 19;
    /// UART 5 / 通用异步收发器 5
    pub const UART5: u32 = 1 << 20;
    /// I2C 1 / 集成电路总线 1
    pub const I2C1: u32 = 1 << 21;
    /// I2C 2 / 集成电路总线 2
    pub const I2C2: u32 = 1 << 22;
    /// I2C 3 / 集成电路总线 3
    pub const I2C3: u32 = 1 << 23;
    /// CRS / 时钟恢复系统
    pub const CRS: u32 = 1 << 24;
    /// Power control / 电源控制
    pub const PWR: u32 = 1 << 28;
    /// DAC1 / 数模转换器 1
    pub const DAC1: u32 = 1 << 29;
    /// OPAMP / 运算放大器
    pub const OPAMP: u32 = 1 << 30;
    /// Low power timer 1 / 低功耗定时器 1
    pub const LPTIM1: u32 = 1 << 31;
}

/// APB1 peripheral 2 clock bits / APB1 外设 2 时钟位
pub mod apb1_2 {
    /// Low power timer 2 / 低功耗定时器 2
    pub const LPTIM2: u32 = 1 << 5;
    /// Low power timer 3 / 低功耗定时器 3
    pub const LPTIM3: u32 = 1 << 6;
    /// I2C 4 / 集成电路总线 4
    pub const I2C4: u32 = 1 << 7;
    /// LPUART 1 / 低功耗 UART 1
    pub const LPUART1: u32 = 1 << 8;
}

/// APB2 peripheral clock bits / APB2 外设时钟位
pub mod apb2 {
    /// Timer 1 / 定时器 1
    pub const TIM1: u32 = 1 << 11;
    /// Timer 8 / 定时器 8
    pub const TIM8: u32 = 1 << 13;
    /// SPI 1 / 串行外设接口 1
    pub const SPI1: u32 = 1 << 12;
    /// Timer 15 / 定时器 15
    pub const TIM15: u32 = 1 << 16;
    /// Timer 16 / 定时器 16
    pub const TIM16: u32 = 1 << 17;
    /// Timer 17 / 定时器 17
    pub const TIM17: u32 = 1 << 18;
    /// USART 1 / 通用同步/异步收发器 1
    pub const USART1: u32 = 1 << 14;
    /// USART 6 / 通用同步/异步收发器 6
    pub const USART6: u32 = 1 << 5;
    /// UART 7 / 通用异步收发器 7
    pub const UART7: u32 = 1 << 6;
    /// UART 8 / 通用异步收发器 8
    pub const UART8: u32 = 1 << 7;
    /// UART 9 / 通用异步收发器 9
    pub const UART9: u32 = 1 << 6;
    /// USART 10 / 通用同步/异步收发器 10
    pub const USART10: u32 = 1 << 7;
    /// SAI 1 / 串行音频接口 1
    pub const SAI1: u32 = 1 << 21;
    /// SAI 2 / 串行音频接口 2
    pub const SAI2: u32 = 1 << 22;
    /// HRTIM / 高分辨率定时器
    pub const HRTIM: u32 = 1 << 26;
}

/// APB3 peripheral clock bits / APB3 外设时钟位
pub mod apb3 {
    /// RTC / 实时时钟
    pub const RTC: u32 = 1 << 0;
    /// VREFBUF / 电压参考缓冲器
    pub const VREFBUF: u32 = 1 << 1;
    /// COMP / 比较器
    pub const COMP: u32 = 1 << 2;
    /// UCPD 1 / USB Type-C PD 1
    pub const UCPD1: u32 = 1 << 8;
    /// UCPD 2 / USB Type-C PD 2
    pub const UCPD2: u32 = 1 << 9;
    /// MDF / 多功能数字滤波器
    pub const MDF: u32 = 1 << 12;
    /// ADF / 音频数字滤波器
    pub const ADF: u32 = 1 << 13;
}
