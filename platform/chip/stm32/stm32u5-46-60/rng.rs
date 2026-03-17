//! RNG - Random Number Generator
//! 随机数生成器
//!
//! # Overview / 概述
//! STM32U5 Random Number Generator (RNG) provides hardware-based random number
//! generation compliant with NIST SP 800-90B standard.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 48: Random number generator (RNG)
//! 
//! ## Random Number Generation / 随机数生成
//! - Hardware random number generator
//! - Compliant with NIST SP 800-90B standard
//! - 32-bit random number output
//! 
//! ## Advanced Features / 高级特性
//! - DMA support
//! - Interrupt support
//! - Continuous/single-shot mode
//! 
//! # Reference / 参考
//! - RM0456 Chapter 48: Random number generator (RNG)
//! - RM0456 Section 48.1: RNG introduction
//! - RM0456 Section 48.2: RNG main features
//! - RM0456 Section 48.3: RNG functional description
//! - RM0456 Section 48.4: RNG registers

/// RNG base address / RNG 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const RNG_BASE: usize = 0x4204_0000;

/// RNG register offsets / RNG 寄存器偏移
//! Reference: RM0456 Section 48.4: RNG register map
pub mod reg {
    /// Control register / 控制寄存器
    //! Reference: RM0456 Section 48.4.1: RNG control register (RNG_CR)
    pub const CR: usize = 0x00;
    /// Status register / 状态寄存器
    //! Reference: RM0456 Section 48.4.2: RNG status register (RNG_SR)
    pub const SR: usize = 0x04;
    /// Data register / 数据寄存器
    //! Reference: RM0456 Section 48.4.3: RNG data register (RNG_DR)
    pub const DR: usize = 0x08;
    /// Health test control register / 健康测试控制寄存器
    //! Reference: RM0456 Section 48.4.4: RNG health test configuration register (RNG_HTCR)
    pub const HTCR: usize = 0x0C;
}

/// RNG configuration / RNG 配置结构体
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Enable RNG / 使能 RNG
    pub enable: bool,
    /// Enable interrupt / 使能中断
    pub interrupt_enable: bool,
    /// RNG clock divider / RNG 时钟分频
    pub clock_divider: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enable: true,
            interrupt_enable: false,
            clock_divider: 0,
        }
    }
}

/// RNG status / RNG 状态
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RngStatus {
    /// RNG ready / RNG 就绪
    Ready,
    /// RNG busy / RNG 忙碌
    Busy,
    /// Seed error / 种子错误
    SeedError,
    /// Clock error / 时钟错误
    ClockError,
    /// Health test error / 健康测试错误
    HealthTestError,
}

/// RNG instance / RNG 实例
pub struct Rng {
    base: usize,
}

impl Rng {
    /// Create new RNG instance / 创建新的 RNG 实例
    pub const fn new() -> Self {
        Self { base: RNG_BASE }
    }

    /// Enable RNG clock in RCC / 在 RCC 中使能 RNG 时钟
    fn enable_clock(&self) {
        unsafe {
            let rcc_base = crate::rcc::RCC_BASE as *mut u32;
            let ahb2enr = rcc_base.add(0x4C / 4);
            *ahb2enr |= 1 << 5; // RNGEN
        }
    }

    /// Initialize RNG with configuration / 使用配置初始化 RNG
    pub fn init(&self, config: &Config) {
        self.enable_clock();

        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = 0u32;

            if config.enable {
                val |= 1 << 2; // RNGEN
            }
            if config.interrupt_enable {
                val |= 1 << 3; // IE
            }
            val |= (config.clock_divider as u32) << 8; // CLKDIV

            core::ptr::write_volatile(cr, val);
        }
    }

    /// Enable RNG / 使能 RNG
    pub fn enable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 2; // RNGEN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable RNG / 禁用 RNG
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 2); // RNGEN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Get RNG status / 获取 RNG 状态
    pub fn get_status(&self) -> RngStatus {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);

            if val & (1 << 0) == 0 {
                return RngStatus::Busy;
            }

            if val & (1 << 2) != 0 {
                return RngStatus::ClockError;
            }
            if val & (1 << 1) != 0 {
                return RngStatus::SeedError;
            }

            RngStatus::Ready
        }
    }

    /// Check if RNG data is ready / 检查 RNG 数据是否就绪
    pub fn is_ready(&self) -> bool {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            (core::ptr::read_volatile(sr) & (1 << 0)) != 0
        }
    }

    /// Read a 32-bit random number / 读取 32 位随机数
    pub fn read(&self) -> u32 {
        unsafe {
            // Wait for random data to be ready / 等待随机数据就绪
            while !self.is_ready() {}

            let dr = (self.base + reg::DR) as *const u32;
            core::ptr::read_volatile(dr)
        }
    }

    /// Read multiple random numbers / 读取多个随机数
    pub fn read_multiple(&self, buffer: &mut [u32]) {
        for word in buffer.iter_mut() {
            *word = self.read();
        }
    }

    /// Generate random number in range / 生成指定范围内的随机数
    pub fn read_range(&self, min: u32, max: u32) -> u32 {
        assert!(max > min, "max must be greater than min");

        let range = max - min + 1;
        let random = self.read();

        // Use rejection sampling for uniform distribution / 使用拒绝采样确保均匀分布
        let max_valid = (u32::MAX / range) * range;
        let mut result = random;
        while result >= max_valid {
            result = self.read();
        }

        min + (result % range)
    }
}

/// Initialize RNG with default configuration / 使用默认配置初始化 RNG
pub fn init_rng_default() -> Rng {
    let rng = Rng::new();
    let config = Config::default();
    rng.init(&config);
    rng
}

/// Generate a random u32 / 生成随机 u32
pub fn random_u32() -> u32 {
    let rng = Rng::new();
    rng.init(&Config::default());
    rng.read()
}

/// Generate a random u64 / 生成随机 u64
pub fn random_u64() -> u64 {
    let rng = Rng::new();
    rng.init(&Config::default());

    let low = rng.read() as u64;
    let high = rng.read() as u64;

    (high << 32) | low
}

/// Generate random bytes / 生成随机字节
pub fn random_bytes(buffer: &mut [u8]) {
    let rng = Rng::new();
    rng.init(&Config::default());

    let mut i = 0;
    while i < buffer.len() {
        let random = rng.read();
        let bytes = random.to_le_bytes();
        for j in 0..4 {
            if i + j < buffer.len() {
                buffer[i + j] = bytes[j];
            }
        }
        i += 4;
    }
}
