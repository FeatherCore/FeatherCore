//! RAMCFG - RAM Configuration Controller / RAM 配置控制器
//!
//! ## STM32U5 RAMCFG 特性 / Features
//! - **SRAM 配置 / SRAM Configuration:**
//!   - SRAM1, SRAM2, SRAM3, SRAM4 独立配置
//!   - 每个 SRAM 区域可独立设置安全属性
//!   - 写保护、读保护配置
//! - **特色功能 / Key Features:**
//!   - 256 字节或 1KB 粒度保护区域
//!   - 安全/非安全属性配置
//!   - 锁定功能
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 6: RAM configuration controller (RAMCFG)
//! - STM32U5 Reference Manual for specific RAM regions

#![no_std]

use core::ptr::{read_volatile, write_volatile};

// ============================================================================
// RAMCFG Base Address / RAMCFG 基地址
// ============================================================================

/// RAMCFG base address / RAMCFG 基地址
pub const RAMCFG_BASE: usize = 0x4002_4000;

// ============================================================================
// Register Offsets / 寄存器偏移
// ============================================================================

/// RAMCFG register offsets / RAMCFG 寄存器偏移
/// Reference: RM0456 Chapter 6.3 / 参考: RM0456 第6.3节
pub mod reg {
    /// RAMCFG configuration register 1 / RAMCFG 配置寄存器 1
    pub const CR1: usize = 0x00;
    /// RAMCFG protection register 1 / RAMCFG 保护寄存器 1
    pub const PR1: usize = 0x04;
    /// RAMCFG configuration register 2 / RAMCFG 配置寄存器 2
    pub const CR2: usize = 0x08;
    /// RAMCFG protection register 2 / RAMCFG 保护寄存器 2
    pub const PR2: usize = 0x0C;
    /// RAMCFG configuration register 3 / RAMCFG 配置寄存器 3
    pub const CR3: usize = 0x10;
    /// RAMCFG protection register 3 / RAMCFG 保护寄存器 3
    pub const PR3: usize = 0x14;
    /// RAMCFG configuration register 4 / RAMCFG 配置寄存器 4
    pub const CR4: usize = 0x18;
    /// RAMCFG protection register 4 / RAMCFG 保护寄存器 4
    pub const PR4: usize = 0x1C;
    /// RAMCFG ECC configuration register / RAMCFG ECC 配置寄存器
    pub const ECCR: usize = 0x20;
    /// RAMCFG SRAM1 write lock register / RAMCFG SRAM1 写锁定寄存器
    pub const SRAM1_WLLR: usize = 0x30;
    /// RAMCFG SRAM2 write lock register / RAMCFG SRAM2 写锁定寄存器
    pub const SRAM2_WLLR: usize = 0x34;
    /// RAMCFG SRAM3 write lock register / RAMCFG SRAM3 写锁定寄存器
    pub const SRAM3_WLLR: usize = 0x38;
    /// RAMCFG SRAM4 write lock register / RAMCFG SRAM4 写锁定寄存器
    pub const SRAM4_WLLR: usize = 0x3C;
    /// RAMCFG peripheral configuration register / RAMCFG 外设配置寄存器
    pub const PCR: usize = 0x40;
}

// ============================================================================
// Register Bit Definitions / 寄存器位定义
// ============================================================================

/// RAMCFG Control Register bits / RAMCFG 控制寄存器位
/// Reference: RM0456 Chapter 6.3.1 / 参考: RM0456 第6.3.1节
pub mod cr_bits {
    pub const VPC: u32 = 1 << 0;              /// VPC / VPC
    pub const VCS: u32 = 1 << 1;              /// VCS / VCS
    pub const WRAP: u32 = 1 << 2;              /// WRAP / WRAP
    pub const ECCEN: u32 = 1 << 3;              /// ECC enable / ECC 使能
    pub const ECCLOCK: u32 = 1 << 4;            /// ECC lock / ECC 锁定
    pub const BKALL: u32 = 1 << 8;             /// Bank all / Bank 全部
    pub const BKSEL_SHIFT: u32 = 12;           /// Bank select shift / Bank 选择位移
    pub const BKSEL_MASK: u32 = 0xF << 12;     /// Bank select mask / Bank 选择掩码
    pub const SEC: u32 = 1 << 20;              /// Secure / 安全
    pub const SECLOCK: u32 = 1 << 21;          /// Secure lock / 安全锁定
    pub const NSLK: u32 = 1 << 22;             /// Non-secure lock / 非安全锁定
    pub const PRIV: u32 = 1 << 24;              /// Privileged / 特权
    pub const PRIVLOCK: u32 = 1 << 25;         /// Privileged lock / 特权锁定
}

/// RAMCFG Protection Register bits / RAMCFG 保护寄存器位
/// Reference: RM0456 Chapter 6.3.2 / 参考: RM0456 第6.3.2节
pub mod pr_bits {
    pub const PRG_SHIFT: u32 = 0;              /// Protection group shift / 保护组位移
    pub const PRG_MASK: u32 = 0xFF << 0;       /// Protection group mask / 保护组掩码
    pub const PRGLOCK: u32 = 1 << 16;          /// Protection lock / 保护锁定
    pub const PRGW: u32 = 1 << 24;              /// Protection write / 保护写
    pub const PRGR: u32 = 1 << 25;             /// Protection read / 保护读
    pub const PRGNS: u32 = 1 << 26;             /// Protection non-secure / 保护非安全
    pub const PRGPRIV: u32 = 1 << 27;          /// Protection privileged / 保护特权
    pub const BLKALL: u32 = 1 << 28;            /// Block all / 块全部
    pub const BLKSEL_SHIFT: u32 = 30;          /// Block select shift / 块选择位移
    pub const BLKSEL_MASK: u32 = 0x3 << 30;     /// Block select mask / 块选择掩码
}

/// RAMCFG ECC Register bits / RAMCFG ECC 寄存器位
/// Reference: RM0456 Chapter 6.3.3 / 参考: RM0456 第6.3.3节
pub mod eccr_bits {
    pub const ECCEN: u32 = 1 << 0;              /// ECC enable / ECC 使能
    pub const ECCSEL: u32 = 1 << 1;             /// ECC select / ECC 选择
    pub const ECCDIS: u32 = 1 << 2;             /// ECC disable / ECC 禁用
    pub const ECCLOCK: u32 = 1 << 4;            /// ECC lock / ECC 锁定
    pub const SECEE: u32 = 1 << 8;              /// Secure error enable / 安全错误使能
    pub const SECDED: u32 = 1 << 9;             /// SECDED status / SECDED 状态
    pub const NSECEE: u32 = 1 << 12;            /// Non-secure error enable / 非安全错误使能
    pub const NSECDED: u32 = 1 << 13;           /// NSECDED status / NSECDED 状态
    pub const ECCADDIE: u32 = 1 << 16;          /// ECC address interrupt enable / ECC 地址中断使能
    pub const ECCADD: u32 = 1 << 17;           /// ECC address status / ECC 地址状态
    pub const ECCADDIP: u32 = 1 << 18;         /// ECC address interrupt pending / ECC 地址中断待处理
}

/// RAMCFG Write Lock Register bits / RAMCFG 写锁定寄存器位
pub mod wllr_bits {
    pub const WLL_SHIFT: u32 = 0;               /// Write lock level shift / 写锁定级别位移
    pub const WLL_MASK: u32 = 0xFF << 0;        /// Write lock level mask / 写锁定级别掩码
    pub const WLLLOCK: u32 = 1 << 16;           /// Write lock lock / 写锁定锁定
}

/// RAMCFG Peripheral Configuration Register bits / RAMCFG 外设配置寄存器位
pub mod pcr_bits {
    pub const PERIPH_SPI1_SEC: u32 = 1 << 0;
    pub const PERIPH_SPI2_SEC: u32 = 1 << 1;
    pub const PERIPH_SPI3_SEC: u32 = 1 << 2;
    pub const PERIPH_I2C1_SEC: u32 = 1 << 4;
    pub const PERIPH_I2C2_SEC: u32 = 1 << 5;
    pub const PERIPH_I2C3_SEC: u32 = 1 << 6;
    pub const PERIPH_USART1_SEC: u32 = 1 << 8;
    pub const PERIPH_USART2_SEC: u32 = 1 << 9;
    pub const PERIPH_USART3_SEC: u32 = 1 << 10;
    pub const PERIPH_LPUART1_SEC: u32 = 1 << 12;
    pub const PERIPH_ADC1_SEC: u32 = 1 << 16;
    pub const PERIPH_DAC1_SEC: u32 = 1 << 18;
    pub const PERIPH_RNG_SEC: u32 = 1 << 20;
    pub const PERIPH_CRYP_SEC: u32 = 1 << 24;
    pub const PERIPH_HASH_SEC: u32 = 1 << 25;
    pub const PCR_LOCK: u32 = 1 << 31;
}

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// RAMCFG SRAM Bank selection / RAMCFG SRAM Bank 选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RamBank {
    /// SRAM1 / SRAM1
    SRAM1 = 0,
    /// SRAM2 / SRAM2
    SRAM2 = 1,
    /// SRAM3 / SRAM3
    SRAM3 = 2,
    /// SRAM4 / SRAM4
    SRAM4 = 3,
}

/// RAMCFG Protection Block / RAMCFG 保护块
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProtectionBlock {
    /// Block 0 / 块 0
    Block0 = 0,
    /// Block 1 / 块 1
    Block1 = 1,
    /// Block 2 / 块 2
    Block2 = 2,
    /// Block 3 / 块 3
    Block3 = 3,
    /// All blocks / 所有块
    All = 4,
}

/// RAMCFG Security Attribution / RAMCFG 安全属性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecurityAttribution {
    /// Secure / 安全
    Secure = 0,
    /// Non-secure / 非安全
    NonSecure = 1,
}

/// RAMCFG Privilege Attribution / RAMCFG 特权属性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrivilegeAttribution {
    /// Non-privileged / 非特权
    NonPrivileged = 0,
    /// Privileged / 特权
    Privileged = 1,
}

// ============================================================================
// Configuration Structures / 配置结构体
// ============================================================================

/// RAMCFG SRAM Configuration / RAMCFG SRAM 配置
#[derive(Clone, Copy, Debug)]
pub struct RamConfig {
    pub secure: bool,
    pub secure_lock: bool,
    pub nonsecure_lock: bool,
    pub privileged: bool,
    pub privileged_lock: bool,
    pub ecc_enable: bool,
    pub ecc_lock: bool,
    pub write_protection: bool,
    pub read_protection: bool,
}

impl Default for RamConfig {
    fn default() -> Self {
        Self {
            secure: false,
            secure_lock: false,
            nonsecure_lock: false,
            privileged: false,
            privileged_lock: false,
            ecc_enable: false,
            ecc_lock: false,
            write_protection: false,
            read_protection: false,
        }
    }
}

/// RAMCFG Region Protection / RAMCFG 区域保护
#[derive(Clone, Copy, Debug)]
pub struct RegionProtection {
    pub protection_group: u8,
    pub write_protect: bool,
    pub read_protect: bool,
    pub nonsecure_protect: bool,
    pub privileged_protect: bool,
    pub lock: bool,
}

impl Default for RegionProtection {
    fn default() -> Self {
        Self {
            protection_group: 0,
            write_protect: false,
            read_protect: false,
            nonsecure_protect: false,
            privileged_protect: false,
            lock: false,
        }
    }
}

// ============================================================================
// RAMCFG Driver / RAMCFG 驱动
// ============================================================================

/// RAMCFG Driver / RAMCFG 驱动
pub struct Ramcfg;

impl Ramcfg {
    pub const fn new() -> Self {
        Self
    }

    fn reg(&self, offset: usize) -> *mut u32 {
        (RAMCFG_BASE + offset) as *mut u32
    }

    // ============================================================================
    // SRAM Configuration Functions / SRAM 配置功能
    // ============================================================================

    /// Configure SRAM1 / 配置 SRAM1
    /// 
    /// # Arguments
    /// * `config` - SRAM configuration / SRAM 配置
    pub fn config_sram1(&self, config: &RamConfig) {
        unsafe {
            let cr = self.reg(reg::CR1);
            let mut val = 0u32;
            
            if config.secure {
                val |= cr_bits::SEC;
            }
            if config.secure_lock {
                val |= cr_bits::SECLOCK;
            }
            if config.nonsecure_lock {
                val |= cr_bits::NSLK;
            }
            if config.privileged {
                val |= cr_bits::PRIV;
            }
            if config.privileged_lock {
                val |= cr_bits::PRIVLOCK;
            }
            if config.ecc_enable {
                val |= cr_bits::ECCEN;
            }
            if config.ecc_lock {
                val |= cr_bits::ECCLOCK;
            }
            
            write_volatile(cr, val);
        }
    }

    /// Configure SRAM2 / 配置 SRAM2
    pub fn config_sram2(&self, config: &RamConfig) {
        unsafe {
            let cr = self.reg(reg::CR2);
            let mut val = 0u32;
            
            if config.secure {
                val |= cr_bits::SEC;
            }
            if config.secure_lock {
                val |= cr_bits::SECLOCK;
            }
            if config.nonsecure_lock {
                val |= cr_bits::NSLK;
            }
            if config.privileged {
                val |= cr_bits::PRIV;
            }
            if config.privileged_lock {
                val |= cr_bits::PRIVLOCK;
            }
            if config.ecc_enable {
                val |= cr_bits::ECCEN;
            }
            if config.ecc_lock {
                val |= cr_bits::ECCLOCK;
            }
            
            write_volatile(cr, val);
        }
    }

    /// Configure SRAM3 / 配置 SRAM3
    pub fn config_sram3(&self, config: &RamConfig) {
        unsafe {
            let cr = self.reg(reg::CR3);
            let mut val = 0u32;
            
            if config.secure {
                val |= cr_bits::SEC;
            }
            if config.secure_lock {
                val |= cr_bits::SECLOCK;
            }
            if config.nonsecure_lock {
                val |= cr_bits::NSLK;
            }
            if config.privileged {
                val |= cr_bits::PRIV;
            }
            if config.privileged_lock {
                val |= cr_bits::PRIVLOCK;
            }
            if config.ecc_enable {
                val |= cr_bits::ECCEN;
            }
            if config.ecc_lock {
                val |= cr_bits::ECCLOCK;
            }
            
            write_volatile(cr, val);
        }
    }

    /// Configure SRAM4 / 配置 SRAM4
    pub fn config_sram4(&self, config: &RamConfig) {
        unsafe {
            let cr = self.reg(reg::CR4);
            let mut val = 0u32;
            
            if config.secure {
                val |= cr_bits::SEC;
            }
            if config.secure_lock {
                val |= cr_bits::SECLOCK;
            }
            if config.nonsecure_lock {
                val |= cr_bits::NSLK;
            }
            if config.privileged {
                val |= cr_bits::PRIV;
            }
            if config.privileged_lock {
                val |= cr_bits::PRIVLOCK;
            }
            if config.ecc_enable {
                val |= cr_bits::ECCEN;
            }
            if config.ecc_lock {
                val |= cr_bits::ECCLOCK;
            }
            
            write_volatile(cr, val);
        }
    }

    // ============================================================================
    // Protection Functions / 保护功能
    // ============================================================================

    /// Set SRAM1 region protection / 设置 SRAM1 区域保护
    /// 
    /// # Arguments
    /// * `block` - Protection block / 保护块
    /// * `protection` - Region protection / 区域保护
    pub fn set_sram1_protection(&self, block: ProtectionBlock, protection: &RegionProtection) {
        unsafe {
            let pr = self.reg(reg::PR1);
            let mut val = protection.protection_group as u32;
            
            if protection.write_protect {
                val |= pr_bits::PRGW;
            }
            if protection.read_protect {
                val |= pr_bits::PRGR;
            }
            if protection.nonsecure_protect {
                val |= pr_bits::PRGNS;
            }
            if protection.privileged_protect {
                val |= pr_bits::PRGPRIV;
            }
            if protection.lock {
                val |= pr_bits::PRGLOCK;
            }
            
            write_volatile(pr, val);
        }
    }

    /// Set SRAM2 region protection / 设置 SRAM2 区域保护
    pub fn set_sram2_protection(&self, block: ProtectionBlock, protection: &RegionProtection) {
        unsafe {
            let pr = self.reg(reg::PR2);
            let mut val = protection.protection_group as u32;
            
            if protection.write_protect {
                val |= pr_bits::PRGW;
            }
            if protection.read_protect {
                val |= pr_bits::PRGR;
            }
            if protection.nonsecure_protect {
                val |= pr_bits::PRGNS;
            }
            if protection.privileged_protect {
                val |= pr_bits::PRGPRIV;
            }
            if protection.lock {
                val |= pr_bits::PRGLOCK;
            }
            
            write_volatile(pr, val);
        }
    }

    /// Set SRAM3 region protection / 设置 SRAM3 区域保护
    pub fn set_sram3_protection(&self, block: ProtectionBlock, protection: &RegionProtection) {
        unsafe {
            let pr = self.reg(reg::PR3);
            let mut val = protection.protection_group as u32;
            
            if protection.write_protect {
                val |= pr_bits::PRGW;
            }
            if protection.read_protect {
                val |= pr_bits::PRGR;
            }
            if protection.nonsecure_protect {
                val |= pr_bits::PRGNS;
            }
            if protection.privileged_protect {
                val |= pr_bits::PRGPRIV;
            }
            if protection.lock {
                val |= pr_bits::PRGLOCK;
            }
            
            write_volatile(pr, val);
        }
    }

    /// Set SRAM4 region protection / 设置 SRAM4 区域保护
    pub fn set_sram4_protection(&self, block: ProtectionBlock, protection: &RegionProtection) {
        unsafe {
            let pr = self.reg(reg::PR4);
            let mut val = protection.protection_group as u32;
            
            if protection.write_protect {
                val |= pr_bits::PRGW;
            }
            if protection.read_protect {
                val |= pr_bits::PRGR;
            }
            if protection.nonsecure_protect {
                val |= pr_bits::PRGNS;
            }
            if protection.privileged_protect {
                val |= pr_bits::PRGPRIV;
            }
            if protection.lock {
                val |= pr_bits::PRGLOCK;
            }
            
            write_volatile(pr, val);
        }
    }

    // ============================================================================
    // ECC Functions / ECC 功能
    // ============================================================================

    /// Enable ECC for SRAM / 使能 SRAM ECC
    pub fn enable_ecc(&self) {
        unsafe {
            let eccr = self.reg(reg::ECCR);
            write_volatile(eccr, eccr_bits::ECCEN);
        }
    }

    /// Disable ECC for SRAM / 禁用 SRAM ECC
    pub fn disable_ecc(&self) {
        unsafe {
            let eccr = self.reg(reg::ECCR);
            write_volatile(eccr, eccr_bits::ECCDIS);
        }
    }

    /// Get ECC status / 获取 ECC 状态
    pub fn get_ecc_status(&self) -> u32 {
        unsafe {
            read_volatile(self.reg(reg::ECCR))
        }
    }

    /// Check if SECDED error occurred / 检查是否发生 SECDED 错误
    pub fn has_sec_error(&self) -> bool {
        (self.get_ecc_status() & eccr_bits::SECDED) != 0
    }

    /// Check if non-secure SECDED error occurred / 检查是否发生非安全 SECDED 错误
    pub fn has_nonsec_error(&self) -> bool {
        (self.get_ecc_status() & eccr_bits::NSECDED) != 0
    }

    // ============================================================================
    // Write Lock Functions / 写锁定功能
    // ============================================================================

    /// Set SRAM1 write lock level / 设置 SRAM1 写锁定级别
    pub fn set_sram1_write_lock(&self, level: u8) {
        unsafe {
            let wllr = self.reg(reg::SRAM1_WLLR);
            write_volatile(wllr, (level as u32 & 0xFF) | wllr_bits::WLLLOCK);
        }
    }

    /// Set SRAM2 write lock level / 设置 SRAM2 写锁定级别
    pub fn set_sram2_write_lock(&self, level: u8) {
        unsafe {
            let wllr = self.reg(reg::SRAM2_WLLR);
            write_volatile(wllr, (level as u32 & 0xFF) | wllr_bits::WLLLOCK);
        }
    }

    /// Set SRAM3 write lock level / 设置 SRAM3 写锁定级别
    pub fn set_sram3_write_lock(&self, level: u8) {
        unsafe {
            let wllr = self.reg(reg::SRAM3_WLLR);
            write_volatile(wllr, (level as u32 & 0xFF) | wllr_bits::WLLLOCK);
        }
    }

    /// Set SRAM4 write lock level / 设置 SRAM4 写锁定级别
    pub fn set_sram4_write_lock(&self, level: u8) {
        unsafe {
            let wllr = self.reg(reg::SRAM4_WLLR);
            write_volatile(wllr, (level as u32 & 0xFF) | wllr_bits::WLLLOCK);
        }
    }

    // ============================================================================
    // Status Functions / 状态功能
    // ============================================================================

    /// Get SRAM1 configuration status / 获取 SRAM1 配置状态
    pub fn get_sram1_status(&self) -> u32 {
        unsafe {
            read_volatile(self.reg(reg::CR1))
        }
    }

    /// Get SRAM2 configuration status / 获取 SRAM2 配置状态
    pub fn get_sram2_status(&self) -> u32 {
        unsafe {
            read_volatile(self.reg(reg::CR2))
        }
    }

    /// Get SRAM3 configuration status / 获取 SRAM3 配置状态
    pub fn get_sram3_status(&self) -> u32 {
        unsafe {
            read_volatile(self.reg(reg::CR3))
        }
    }

    /// Get SRAM4 configuration status / 获取 SRAM4 配置状态
    pub fn get_sram4_status(&self) -> u32 {
        unsafe {
            read_volatile(self.reg(reg::CR4))
        }
    }

    /// Check if SRAM1 is secure / 检查 SRAM1 是否安全
    pub fn is_sram1_secure(&self) -> bool {
        (self.get_sram1_status() & cr_bits::SEC) != 0
    }

    /// Check if SRAM2 is secure / 检查 SRAM2 是否安全
    pub fn is_sram2_secure(&self) -> bool {
        (self.get_sram2_status() & cr_bits::SEC) != 0
    }

    /// Check if SRAM3 is secure / 检查 SRAM3 是否安全
    pub fn is_sram3_secure(&self) -> bool {
        (self.get_sram3_status() & cr_bits::SEC) != 0
    }

    /// Check if SRAM4 is secure / 检查 SRAM4 是否安全
    pub fn is_sram4_secure(&self) -> bool {
        (self.get_sram4_status() & cr_bits::SEC) != 0
    }
}

// ============================================================================
// Convenience Functions / 便捷函数
// ============================================================================

/// Create default RAM configuration / 创建默认 RAM 配置
pub fn config_ram_default() -> RamConfig {
    RamConfig::default()
}

/// Create secure RAM configuration / 创建安全 RAM 配置
pub fn config_ram_secure() -> RamConfig {
    RamConfig {
        secure: true,
        secure_lock: true,
        nonsecure_lock: false,
        privileged: false,
        privileged_lock: false,
        ecc_enable: false,
        ecc_lock: false,
        write_protection: false,
        read_protection: false,
    }
}

/// Create non-secure RAM configuration / 创建非安全 RAM 配置
pub fn config_ram_nonsecure() -> RamConfig {
    RamConfig {
        secure: false,
        secure_lock: true,
        nonsecure_lock: true,
        privileged: false,
        privileged_lock: false,
        ecc_enable: false,
        ecc_lock: false,
        write_protection: false,
        read_protection: false,
    }
}

/// Create protected RAM configuration / 创建受保护 RAM 配置
pub fn config_ram_protected() -> RamConfig {
    RamConfig {
        secure: false,
        secure_lock: true,
        nonsecure_lock: true,
        privileged: true,
        privileged_lock: true,
        ecc_enable: false,
        ecc_lock: false,
        write_protection: true,
        read_protection: true,
    }
}
