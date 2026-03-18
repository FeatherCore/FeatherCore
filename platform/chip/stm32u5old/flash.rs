//! FLASH - Flash Memory Interface
//! 嵌入式闪存存储器接口
//!
//! # Overview / 概述
//! STM32U5 Embedded Flash Memory provides non-volatile storage for code and data.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 7
//!
//! ## Storage Capacity / 存储容量
//! - Up to 4 MB Flash memory (dual bank architecture)
//! - Up to 128 KB SRAM with ECC
//! - 1 KB or 2 KB page size
//!
//! ## Architecture / 架构
//! - Dual Bank architecture supporting RWW (Read While Write)
//! - Main storage area and option bytes area
//! - Secure/Non-secure region programming
//!
//! ## Security Features / 安全特性
//! - ECC (Error Correction Code) support
//! - TrustZone security extension (TZEN)
//! - Secure access protection / Secure storage area
//! - PCROP (Read/Write Protection)
//! - RDP (Read Protection) levels
//!
//! ## Cache Features / 缓存特性
//! - Instruction cache (I-Cache)
//! - Data cache (D-Cache)
//! - Prefetch buffer
//!
//! ## Operations / 操作
//! - Page erase
//! - Mass erase / Bank erase
//! - Double word programming
//! - Option bytes programming
//! - Secure/Non-secure region programming
//!
//! # Latency (Wait States) / 延迟 (等待状态)
//! Reference: RM0456 Section 7.4.1
//!
//! | LATENCY | SYSCLK Range |
//! |---------|---------------|
//! | 0 (Ws0) | 0 < SYSCLK <= 32 MHz |
//! | 1 (Ws1) | 32 < SYSCLK <= 64 MHz |
//! | 2 (Ws2) | 64 < SYSCLK <= 96 MHz |
//! | 3 (Ws3) | 96 < SYSCLK <= 128 MHz |
//! | 4 (Ws4) | 128 < SYSCLK <= 160 MHz |
//! | 5 (Ws5) | > 160 MHz |
//!
//! # Reference / 参考
//! - RM0456 Chapter 7: Embedded flash memory (FLASH)
//! - RM0456 Section 7.1: FLASH introduction
//! - RM0456 Section 7.2: FLASH main features
//! - RM0456 Section 7.3: FLASH functional description
//! - RM0456 Section 7.4: FLASH registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// Flash interface base address (non-secure)
//! Reference: RM0456 Chapter 2, Table 1
pub const FLASH_BASE: usize = 0x4002_2000;

/// Flash Option Bytes base address
//! Reference: RM0456 Chapter 2, Table 1
pub const FLASH_OPT_BASE: usize = 0x4002_3000;

/// Flash memory base address (code storage)
pub const FLASH_MEM_BASE: usize = 0x0800_0000;

/// Flash Register Offsets
//! Reference: RM0456 Section 7.5: FLASH registers
pub mod reg {
    /// Flash Access Control Register
    //! Reference: RM0456 Section 7.5.1
    pub const ACR: usize = 0x00;

    /// Flash Key Register
    //! Reference: RM0456 Section 7.5.2
    pub const KEYR: usize = 0x08;

    /// Flash Option Key Register
    //! Reference: RM0456 Section 7.5.3
    pub const OPTKEYR: usize = 0x0C;

    /// Flash Status Register
    //! Reference: RM0456 Section 7.5.4
    pub const SR: usize = 0x10;

    /// Flash Control Register
    //! Reference: RM0456 Section 7.5.5
    pub const CR: usize = 0x14;

    /// Flash ECC Register
    //! Reference: RM0456 Section 7.5.6
    pub const ECCR: usize = 0x18;

    /// Flash Option Register
    //! Reference: RM0456 Section 7.5.7
    pub const OPTR: usize = 0x20;

    /// Flash Non-Secure Status Register
    pub const NSSR: usize = 0x24;

    /// Flash Secure Status Register
    pub const SECSR: usize = 0x28;

    /// Flash Secure Control Register
    pub const SECCR: usize = 0x2C;

    /// Flash Option Status Register
    pub const OPTSR: usize = 0x34;

    /// Flash Option Clear Register
    pub const OPTCLRR: usize = 0x38;

    /// Flash Boot Address Register 0
    pub const BOOT_ADD0: usize = 0x40;

    /// Flash Boot Address Register 1
    pub const BOOT_ADD1: usize = 0x44;

    /// Flash Secure Boot Address Register 0
    pub const SECBOOT_ADD0: usize = 0x48;

    /// Flash Secure Boot Address Register 1
    pub const SECBOOT_ADD1: usize = 0x4C;

    /// Flash Option Control Register
    //! Reference: RM0456 Section 7.5.8
    pub const OPTCR: usize = 0x50;

    /// Flash Option Control Register 1
    pub const OPTCR1: usize = 0x54;

    /// Flash ECC Lock Register
    pub const ECCLR: usize = 0x58;

    /// Flash Address ECC Register
    pub const ADDECC_REG: usize = 0x5C;

    /// Flash Current Bank Register
    pub const CURRENT_BANK: usize = 0x60;
}

// ============================================================================
// Register Bit Definitions / 寄存器位定义
// ============================================================================

/// ACR (Access Control Register) bits
//! Reference: RM0456 Section 7.5.1
pub mod acr_bits {
    /// Latency (wait states) / 延迟 (等待状态)
    pub const LATENCY_SHIFT: u32 = 0;
    pub const LATENCY_MASK: u32 = 0xF;

    /// Prefetch enable / 预取使能
    pub const PRFTEN: u32 = 1 << 8;

    /// Instruction cache enable / 指令缓存使能
    pub const ICEN: u32 = 1 << 9;

    /// Data cache enable / 数据缓存使能
    pub const DCREN: u32 = 1 << 10;
    pub const DCEN: u32 = 1 << 10;

    /// Instruction cache reset / 指令缓存复位
    pub const ICRST: u32 = 1 << 11;

    /// Data cache reset / 数据缓存复位
    pub const DCRST: u32 = 1 << 12;

    /// Flash empty flag / Flash空标志
    pub const EMPTY: u32 = 1 << 16;

    /// Wait 16 cycles / 等待16周期
    pub const WAIT16: u32 = 1 << 20;
}

/// SR (Status Register) bits
//! Reference: RM0456 Section 7.5.4
pub mod sr_bits {
    /// Bank 1 busy / Bank 1忙
    pub const BSY1: u32 = 1 << 0;

    /// Bank 2 busy / Bank 2忙
    pub const BSY2: u32 = 1 << 1;

    /// Write verification error / 写验证错误
    pub const WR_VRFY: u32 = 1 << 3;

    /// Programming sequence error / 编程序列错误
    pub const PGS: u32 = 1 << 4;

    /// Programming parallelism error / 编程并行错误
    pub const PGP: u32 = 1 << 5;

    /// Programming alignment error / 编程对齐错误
    pub const PGA: u32 = 1 << 6;

    /// Write protection error / 写保护错误
    pub const WRP: u32 = 1 << 7;

    /// Operation error / 操作错误
    pub const OPERR: u32 = 1 << 8;

    /// Read error / 读错误
    pub const RDERR: u32 = 1 << 9;

    /// ECC fail error / ECC失败错误
    pub const ECCFE: u32 = 1 << 12;

    /// ECC single bit error detected / 检测到ECC单比特错误
    pub const ECCRD: u32 = 1 << 13;

    /// ECC single bit error corrected / ECC单比特错误已纠正
    pub const SNECC: u32 = 1 << 14;

    /// ECC double bit error / ECC双比特错误
    pub const DBECC: u32 = 1 << 15;

    /// Configuration busy / 配置忙
    pub const CFGBSY: u32 = 1 << 16;
}

/// CR (Control Register) bits
//! Reference: RM0456 Section 7.5.5
pub mod cr_bits {
    /// Program / 编程
    pub const PG: u32 = 1 << 0;

    /// Sector Erase / 扇区擦除
    pub const SER: u32 = 1 << 1;

    /// Bank Erase / Bank擦除
    pub const BER: u32 = 1 << 2;

    /// Program size shift / 编程大小位移
    pub const PSIZE_SHIFT: u32 = 4;
    pub const PSIZE_MASK: u32 = 0x3 << 4;

    /// 8-bit programming / 8位编程
    pub const PSIZE_X8: u32 = 0x0 << 4;

    /// 16-bit programming / 16位编程
    pub const PSIZE_X16: u32 = 0x1 << 4;

    /// 32-bit programming / 32位编程
    pub const PSIZE_X32: u32 = 0x2 << 4;

    /// 64-bit programming / 64位编程
    pub const PSIZE_X64: u32 = 0x3 << 4;

    /// Start operation / 启动操作
    pub const START: u32 = 1 << 7;

    /// Sector number shift / 扇区编号位移
    pub const SNB_SHIFT: u32 = 8;
    pub const SNB_MASK: u32 = 0x1F << 8;

    /// Bank selection / Bank选择
    pub const BKER: u32 = 1 << 13;

    /// Mass Erase / 批量擦除
    pub const MER: u32 = 1 << 15;

    /// Start D1 domain erase / 启动D1域擦除
    pub const STRT_D1: u32 = 1 << 16;

    /// Start D2 domain erase / 启动D2域擦除
    pub const STRT_D2: u32 = 1 << 17;

    /// End of operation interrupt enable / 操作完成中断使能
    pub const EOPIE: u32 = 1 << 24;

    /// Error interrupt enable / 错误中断使能
    pub const ERRIE: u32 = 1 << 25;

    /// Lock / 锁定
    pub const LOCK: u32 = 1 << 31;
}

/// ECCR (ECC Register) bits
//! Reference: RM0456 Section 7.5.6
pub mod eccr_bits {
    /// ECC enable / ECC使能
    pub const ECCEN: u32 = 1 << 0;

    /// ECC disable / ECC禁用
    pub const ECCDI: u32 = 1 << 1;

    /// ECC correction enable / ECC纠错使能
    pub const ECC_COR: u32 = 1 << 2;

    /// ECC lock / ECC锁定
    pub const ECC_LOCK: u32 = 1 << 3;

    /// Trigger / 触发
    pub const TRIGGER: u32 = 1 << 4;
}

/// SECCR (Secure Control Register) bits
//! Reference: RM0456 Section 7.5.7
pub mod seccr_bits {
    /// Program / 编程
    pub const PG: u32 = 1 << 0;

    /// Sector Erase / 扇区擦除
    pub const SER: u32 = 1 << 1;

    /// Bank Erase / Bank擦除
    pub const BER: u32 = 1 << 2;

    /// Program size / 编程大小
    pub const PSIZE_SHIFT: u32 = 4;
    pub const PSIZE_MASK: u32 = 0x3 << 4;

    /// Start / 启动
    pub const START: u32 = 1 << 7;

    /// Sector number / 扇区编号
    pub const SNB_SHIFT: u32 = 8;
    pub const SNB_MASK: u32 = 0x1F << 8;

    /// Bank selection / Bank选择
    pub const BKER: u32 = 1 << 13;

    /// Lock / 锁定
    pub const LOCK: u32 = 1 << 31;
}

/// OPTR (Option Register) bits
//! Reference: RM0456 Section 7.5.8
pub mod optr_bits {
    /// Read protection shift / 读保护位移
    pub const RDP_SHIFT: u32 = 0;
    pub const RDP_MASK: u32 = 0xFF;

    /// Level 0 - No protection / 级别0 - 无保护
    pub const RDP_LEVEL0: u32 = 0xAA;

    /// Level 1 - Chip protection / 级别1 - 芯片保护
    pub const RDP_LEVEL1: u32 = 0x00;

    /// Level 2 - Read protection / 级别2 - 读保护
    pub const RDP_LEVEL2: u32 = 0xCC;

    /// BOR level / BOR级别
    pub const BOR_LEV: u32 = 0xF << 8;

    /// NRST mode / NRST模式
    pub const NRST_MODE: u32 = 1 << 12;

    /// nRST in STOP mode / STOP模式复位
    pub const nRST_STOP: u32 = 1 << 20;

    /// nRST in STANDBY mode / STANDBY模式复位
    pub const nRST_STANDBY: u32 = 1 << 21;

    /// IWDG software control / IWDG软件控制
    pub const IWDG_SW: u32 = 1 << 22;

    /// WWDG software control / WWDG软件控制
    pub const WWDG_SW: u32 = 1 << 24;

    /// TrustZone enable / TrustZone使能
    pub const TZEN: u32 = 1 << 28;

    /// Debug software enable / 调试软件使能
    pub const DBG_SWEN: u32 = 1 << 29;

    /// Debug general / 调试通用
    pub const DBG_GEN: u32 = 1 << 30;
}

/// OPTSR (Option Status Register) bits
//! Reference: RM0456 Section 7.5.9
pub mod optsr_bits {
    /// Option byte busy / 选项字节忙
    pub const OPTBUSY: u32 = 1 << 0;

    /// Option byte error / 选项字节错误
    pub const OPTVERR: u32 = 1 << 1;

    /// BOR value / BOR值
    pub const BORV_SHIFT: u32 = 2;
    pub const BORV_MASK: u32 = 0x3 << 2;

    /// TrustZone ready / TrustZone就绪
    pub const TZENRDY: u32 = 1 << 4;
}

/// OPTCR (Option Control Register) bits
//! Reference: RM0456 Section 7.5.10
pub mod optcr_bits {
    /// Option lock / 选项锁定
    pub const OPTLOCK: u32 = 1 << 0;

    /// OBL launch / OBL启动
    pub const OBL_LAUNCH: u32 = 1 << 1;

    /// Force option load / 强制选项加载
    pub const FORCE_OPTLOAD: u32 = 1 << 14;
}

/// SECSR (Secure Status Register) bits
//! Reference: RM0456 Section 7.5.7
pub mod secsr_bits {
    /// Security error / 安全错误
    pub const SEC_ERROR: u32 = 1 << 0;

    /// Security hit / 安全命中
    pub const SEC_HIT: u32 = 1 << 1;
}

/// Flash Keys
//! Reference: RM0456 Section 7.3.8
pub const FLASH_KEY1: u32 = 0x4567_0123;
pub const FLASH_KEY2: u32 = 0xCDEF_89AB;
pub const FLASH_OPTKEY1: u32 = 0x0819_2A3B;
pub const FLASH_OPTKEY2: u32 = 0x4C5D_6E7F;

/// Flash Latency (Wait States)
//! Reference: RM0456 Section 7.4.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Latency {
    /// 0 wait states (0 < SYSCLK <= 32 MHz)
    Ws0 = 0,
    /// 1 wait state (32 < SYSCLK <= 64 MHz)
    Ws1 = 1,
    /// 2 wait states (64 < SYSCLK <= 96 MHz)
    Ws2 = 2,
    /// 3 wait states (96 < SYSCLK <= 128 MHz)
    Ws3 = 3,
    /// 4 wait states (128 < SYSCLK <= 160 MHz)
    Ws4 = 4,
    /// 5 wait states (for higher frequencies)
    Ws5 = 5,
}

/// Initialize Flash controller
//! Reference: RM0456 Chapter 7
pub fn init() {
    // Default initialization - flash is ready after reset
    // Wait for any ongoing operations to complete
    wait_not_busy();
}

/// Set flash latency (wait states)
//! Reference: RM0456 Section 7.4.1
pub fn set_latency(latency: Latency) {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val &= !0xF;
        val |= latency as u32;
        core::ptr::write_volatile(acr, val);

        while get_latency() != latency {}
    }
}

/// Get current flash latency
//! Reference: RM0456 Section 7.5.1
pub fn get_latency() -> Latency {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let val = core::ptr::read_volatile(acr) & 0xF;
        match val {
            0 => Latency::Ws0,
            1 => Latency::Ws1,
            2 => Latency::Ws2,
            3 => Latency::Ws3,
            4 => Latency::Ws4,
            _ => Latency::Ws5,
        }
    }
}

/// Configure flash for given system clock frequency
//! Reference: RM0456 Section 7.4.1
pub fn configure_for_sysclk(freq_hz: u32) {
    let latency = if freq_hz <= 32_000_000 {
        Latency::Ws0
    } else if freq_hz <= 64_000_000 {
        Latency::Ws1
    } else if freq_hz <= 96_000_000 {
        Latency::Ws2
    } else if freq_hz <= 128_000_000 {
        Latency::Ws3
    } else {
        Latency::Ws4
    };
    set_latency(latency);
}

/// Enable instruction cache
//! Reference: RM0456 Section 7.4.2
pub fn enable_icache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val |= acr_bits::ICEN;
        core::ptr::write_volatile(acr, val);
    }
}

/// Disable instruction cache
pub fn disable_icache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val &= !acr_bits::ICEN;
        core::ptr::write_volatile(acr, val);
    }
}

/// Enable data cache
//! Reference: RM0456 Section 7.4.3
pub fn enable_dcache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val |= acr_bits::DCEN;
        core::ptr::write_volatile(acr, val);
    }
}

/// Disable data cache
pub fn disable_dcache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val &= !acr_bits::DCEN;
        core::ptr::write_volatile(acr, val);
    }
}

/// Unlock flash for write/erase operations
//! Reference: RM0456 Section 7.3.8
pub fn unlock() -> Result<(), FlashError> {
    unsafe {
        let cr = (FLASH_BASE + reg::CR) as *mut u32;

        if (core::ptr::read_volatile(cr) & cr_bits::LOCK) == 0 {
            return Ok(());
        }

        let keyr = (FLASH_BASE + reg::KEYR) as *mut u32;
        core::ptr::write_volatile(keyr, FLASH_KEY1);
        core::ptr::write_volatile(keyr, FLASH_KEY2);

        if (core::ptr::read_volatile(cr) & cr_bits::LOCK) != 0 {
            return Err(FlashError::UnlockFailed);
        }

        Ok(())
    }
}

/// Lock flash
//! Reference: RM0456 Section 7.3.8
pub fn lock() {
    unsafe {
        let cr = (FLASH_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= cr_bits::LOCK;
        core::ptr::write_volatile(cr, val);
    }
}

/// Wait for flash not busy
//! Reference: RM0456 Section 7.5.4
pub fn wait_not_busy() {
    unsafe {
        let sr = (FLASH_BASE + reg::SR) as *mut u32;
        while (core::ptr::read_volatile(sr) & (sr_bits::BSY1 | sr_bits::BSY2)) != 0 {}
    }
}

/// Clear error flags
//! Reference: RM0456 Section 7.5.4
pub fn clear_errors() {
    unsafe {
        let sr = (FLASH_BASE + reg::SR) as *mut u32;
        let val = core::ptr::read_volatile(sr);
        core::ptr::write_volatile(sr, val & 0x0000_FFFF);
    }
}

/// Erase a sector (page)
//! Reference: RM0456 Section 7.3.6
//!
//! # Arguments
//! * `sector` - Sector number to erase
pub fn erase_sector(sector: u32) -> Result<(), FlashError> {
    unlock()?;

    unsafe {
        wait_not_busy();
        clear_errors();

        let cr = (FLASH_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val &= !cr_bits::SNB_MASK;
        val |= sector << cr_bits::SNB_SHIFT;
        val |= cr_bits::SER;
        core::ptr::write_volatile(cr, val);

        val |= cr_bits::START;
        core::ptr::write_volatile(cr, val);

        wait_not_busy();

        let mut val = core::ptr::read_volatile(cr);
        val &= !cr_bits::SER;
        core::ptr::write_volatile(cr, val);
    }

    lock();
    Ok(())
}

/// Program a 64-bit double word
//! Reference: RM0456 Section 7.3.5
//!
//! # Arguments
//! * `address` - Flash address to write (must be 8-byte aligned)
//! * `data` - 64-bit data to write
pub fn program_double_word(address: usize, data: u64) -> Result<(), FlashError> {
    if address % 8 != 0 {
        return Err(FlashError::AlignmentError);
    }

    unlock()?;

    unsafe {
        wait_not_busy();
        clear_errors();

        let cr = (FLASH_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= cr_bits::PG;
        val &= !cr_bits::PSIZE_MASK;
        val |= cr_bits::PSIZE_X64;
        core::ptr::write_volatile(cr, val);

        let ptr = address as *mut u32;
        core::ptr::write_volatile(ptr, data as u32);
        core::ptr::write_volatile(ptr.add(1), (data >> 32) as u32);

        wait_not_busy();

        let mut val = core::ptr::read_volatile(cr);
        val &= !cr_bits::PG;
        core::ptr::write_volatile(cr, val);
    }

    lock();
    Ok(())
}

/// Program data to flash
//!
//! # Arguments
//! * `address` - Starting flash address (must be 8-byte aligned)
//! * `data` - Data to write
pub fn program(address: usize, data: &[u8]) -> Result<(), FlashError> {
    if address % 8 != 0 || data.len() % 8 != 0 {
        return Err(FlashError::AlignmentError);
    }

    for (i, chunk) in data.chunks(8).enumerate() {
        let mut word: u64 = 0;
        for (j, &byte) in chunk.iter().enumerate() {
            word |= (byte as u64) << (j * 8);
        }
        program_double_word(address + i * 8, word)?;
    }

    Ok(())
}

/// Flash errors
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlashError {
    /// Unlock failed
    UnlockFailed,
    /// Alignment error
    AlignmentError,
    /// Programming error
    ProgrammingError,
    /// Erase error
    EraseError,
    /// Write protection error
    WriteProtectionError,
}

/// Get flash size in KB
//! Reference: RM0456 Section 7.1
pub fn get_flash_size_kb() -> u32 {
    unsafe {
        let flash_size_reg = 0x1FFF_75E0 as *const u16;
        core::ptr::read_volatile(flash_size_reg) as u32
    }
}

/// Get unique device ID (96-bit)
//! Reference: RM0456 Section 7.1
pub fn get_unique_id() -> [u32; 3] {
    unsafe {
        let uid_base = 0x1FFF_7580 as *const u32;
        [
            read_volatile(uid_base),
            read_volatile(uid_base.add(1)),
            read_volatile(uid_base.add(2)),
        ]
    }
}

// ============================================================================
// Option Bytes Functions / 选项字节功能
// ============================================================================

/// Unlock option bytes for write
//! Reference: RM0456 Section 7.3.8
pub fn unlock_option_bytes() -> Result<(), FlashError> {
    unsafe {
        let optkeyr = (FLASH_BASE + reg::OPTKEYR) as *mut u32;
        write_volatile(optkeyr, FLASH_OPTKEY1);
        write_volatile(optkeyr, FLASH_OPTKEY2);
        Ok(())
    }
}

/// Lock option bytes
pub fn lock_option_bytes() {
    unsafe {
        let optcr = (FLASH_BASE + reg::OPTCR) as *mut u32;
        write_volatile(optcr, optcr_bits::OPTLOCK);
    }
}

/// Get option byte status
//! Reference: RM0456 Section 7.5.8
pub fn get_option_bytes() -> u32 {
    unsafe {
        read_volatile((FLASH_BASE + reg::OPTR) as *const u32)
    }
}

/// Check if option bytes are locked
pub fn is_option_bytes_locked() -> bool {
    unsafe {
        (read_volatile((FLASH_BASE + reg::OPTCR) as *const u32) & optcr_bits::OPTLOCK) != 0
    }
}

/// Launch option bytes reload
//! Reference: RM0456 Section 7.5.10
pub fn launch_option_bytes() {
    unsafe {
        let optcr = (FLASH_BASE + reg::OPTCR) as *mut u32;
        write_volatile(optcr, optcr_bits::OBL_LAUNCH);
        while is_option_bytes_busy() {}
    }
}

/// Check if option bytes operation is busy
pub fn is_option_bytes_busy() -> bool {
    unsafe {
        (read_volatile((FLASH_BASE + reg::OPTSR) as *const u32) & optsr_bits::OPTBUSY) != 0
    }
}

/// Set read protection level
//! Reference: RM0456 Section 7.3.9
pub fn set_read_protection(level: u8) -> Result<(), FlashError> {
    if level > 2 {
        return Err(FlashError::AlignmentError);
    }

    unlock()?;
    unlock_option_bytes()?;

    unsafe {
        let optr = (FLASH_BASE + reg::OPTR) as *mut u32;
        let val = read_volatile(optr);
        let new_val = match level {
            0 => (val & !optr_bits::RDP_MASK) | optr_bits::RDP_LEVEL0,
            1 => (val & !optr_bits::RDP_MASK) | optr_bits::RDP_LEVEL1,
            2 => (val & !optr_bits::RDP_MASK) | optr_bits::RDP_LEVEL2,
            _ => val,
        };
        write_volatile(optr, new_val);
        launch_option_bytes();
    }

    Ok(())
}

/// Get read protection level
pub fn get_read_protection() -> u8 {
    let optr = get_option_bytes();
    match optr & optr_bits::RDP_MASK {
        optr_bits::RDP_LEVEL0 => 0,
        optr_bits::RDP_LEVEL1 => 1,
        optr_bits::RDP_LEVEL2 => 2,
        _ => 1,
    }
}

/// Enable TrustZone
//! Reference: RM0456 Section 7.3.10
pub fn enable_trustzone() -> Result<(), FlashError> {
    unlock()?;
    unlock_option_bytes()?;

    unsafe {
        let optr = (FLASH_BASE + reg::OPTR) as *mut u32;
        let val = read_volatile(optr);
        write_volatile(optr, val | optr_bits::TZEN);
        launch_option_bytes();
    }

    Ok(())
}

/// Check if TrustZone is enabled
pub fn is_trustzone_enabled() -> bool {
    (get_option_bytes() & optr_bits::TZEN) != 0
}

// ============================================================================
// Security Functions / 安全功能
// ============================================================================

/// Erase bank
//! Reference: RM0456 Section 7.3.7
pub fn erase_bank(bank: u8) -> Result<(), FlashError> {
    if bank > 2 {
        return Err(FlashError::AlignmentError);
    }

    unlock()?;

    unsafe {
        wait_not_busy();
        clear_errors();

        let cr = (FLASH_BASE + reg::CR) as *mut u32;
        let mut val = read_volatile(cr);

        if bank == 1 {
            val |= cr_bits::BER;
            val |= cr_bits::STRT_D1;
        } else {
            val |= cr_bits::BER;
            val |= cr_bits::STRT_D2;
        }

        write_volatile(cr, val);
        wait_not_busy();
    }

    lock();
    Ok(())
}

/// Mass erase (both banks)
pub fn mass_erase() -> Result<(), FlashError> {
    unlock()?;

    unsafe {
        wait_not_busy();
        clear_errors();

        let cr = (FLASH_BASE + reg::CR) as *mut u32;
        let mut val = read_volatile(cr);
        val |= cr_bits::MER;
        val |= cr_bits::STRT_D1;
        val |= cr_bits::STRT_D2;
        write_volatile(cr, val);

        wait_not_busy();
    }

    lock();
    Ok(())
}

// ============================================================================
// ECC Functions / ECC功能
// ============================================================================

/// Enable ECC
//! Reference: RM0456 Section 7.3.11
pub fn enable_ecc() {
    unsafe {
        let eccr = (FLASH_BASE + reg::ECCR) as *mut u32;
        write_volatile(eccr, eccr_bits::ECCEN);
    }
}

/// Disable ECC
pub fn disable_ecc() {
    unsafe {
        let eccr = (FLASH_BASE + reg::ECCR) as *mut u32;
        write_volatile(eccr, eccr_bits::ECCDI);
    }
}

/// Enable ECC correction
pub fn enable_ecc_correction() {
    unsafe {
        let eccr = (FLASH_BASE + reg::ECCR) as *mut u32;
        write_volatile(eccr, eccr_bits::ECC_COR);
    }
}

/// Get ECC status
//! Reference: RM0456 Section 7.5.6
pub fn get_ecc_status() -> u32 {
    unsafe {
        read_volatile((FLASH_BASE + reg::ECCR) as *const u32)
    }
}

/// Check for ECC single bit error
pub fn has_ecc_single_error() -> bool {
    (get_ecc_status() & sr_bits::SNECC) != 0
}

/// Check for ECC double bit error
pub fn has_ecc_double_error() -> bool {
    (get_ecc_status() & sr_bits::DBECC) != 0
}

/// Check for ECC failure
pub fn has_ecc_failure() -> bool {
    (get_ecc_status() & sr_bits::ECCFE) != 0
}

/// Get ECC error address
pub fn get_ecc_error_address() -> u32 {
    unsafe {
        read_volatile((FLASH_BASE + reg::ADDECC_REG) as *const u32)
    }
}

/// Clear ECC error flags
pub fn clear_ecc_errors() {
    unsafe {
        let sr = (FLASH_BASE + reg::SR) as *mut u32;
        let val = read_volatile(sr);
        write_volatile(sr, val & (sr_bits::SNECC | sr_bits::DBECC | sr_bits::ECCFE));
    }
}

// ============================================================================
// Cache Functions / 缓存功能
// ============================================================================

/// Reset instruction cache
//! Reference: RM0456 Section 7.4.2
pub fn reset_icache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let val = read_volatile(acr);
        write_volatile(acr, val | acr_bits::ICRST);
        write_volatile(acr, val & !acr_bits::ICRST);
    }
}

/// Reset data cache
//! Reference: RM0456 Section 7.4.3
pub fn reset_dcache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let val = read_volatile(acr);
        write_volatile(acr, val | acr_bits::DCRST);
        write_volatile(acr, val & !acr_bits::DCRST);
    }
}

/// Reset both caches
pub fn reset_caches() {
    reset_icache();
    reset_dcache();
}

/// Enable prefetch
//! Reference: RM0456 Section 7.4.1
pub fn enable_prefetch() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let val = read_volatile(acr);
        write_volatile(acr, val | acr_bits::PRFTEN);
    }
}

/// Disable prefetch
pub fn disable_prefetch() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let val = read_volatile(acr);
        write_volatile(acr, val & !acr_bits::PRFTEN);
    }
}

// ============================================================================
// Status Functions / 状态功能
// ============================================================================

/// Check if flash is busy
pub fn is_busy() -> bool {
    unsafe {
        (read_volatile((FLASH_BASE + reg::SR) as *const u32) & (sr_bits::BSY1 | sr_bits::BSY2)) != 0
    }
}

/// Check if Bank 1 is busy
pub fn is_bank1_busy() -> bool {
    unsafe {
        (read_volatile((FLASH_BASE + reg::SR) as *const u32) & sr_bits::BSY1) != 0
    }
}

/// Check if Bank 2 is busy
pub fn is_bank2_busy() -> bool {
    unsafe {
        (read_volatile((FLASH_BASE + reg::SR) as *const u32) & sr_bits::BSY2) != 0
    }
}

/// Get flash status
pub fn get_status() -> u32 {
    unsafe {
        read_volatile((FLASH_BASE + reg::SR) as *const u32)
    }
}

/// Check for any error
pub fn has_error() -> bool {
    let sr = get_status();
    (sr & (sr_bits::WRP | sr_bits::OPERR | sr_bits::PGS | sr_bits::PGP | sr_bits::PGA)) != 0
}

/// Clear all status flags
pub fn clear_status() {
    unsafe {
        let sr = (FLASH_BASE + reg::SR) as *mut u32;
        write_volatile(sr, 0x03FF_FFFF);
    }
}

// ============================================================================
// Flash Information / Flash信息
// ============================================================================

/// Get flash page size in bytes
pub fn get_page_size() -> usize {
    2048 // 2KB for STM32U5
}

/// Get number of pages per bank
pub fn get_pages_per_bank() -> u32 {
    let size_kb = get_flash_size_kb();
    size_kb / 2
}

/// Check if flash is empty (virgin)
pub fn is_empty() -> bool {
    unsafe {
        (read_volatile((FLASH_BASE + reg::ACR) as *const u32) & acr_bits::EMPTY) != 0
    }
}

/// Check dual bank mode support
pub fn is_dual_bank() -> bool {
    true // STM32U5 supports dual bank
}
