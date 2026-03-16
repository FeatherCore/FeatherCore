//! FLASH - Flash Memory Interface
//! Flash 存储器接口
//!
//! STM32U5 Flash 特性：
//! - 最高 4MB Flash 容量
//! - 双 Bank 架构（支持 RWW - Read While Write）
//! - 支持 ECC
//! - 支持 TrustZone 安全扩展

/// Flash interface base address
pub const FLASH_BASE: usize = 0x4002_2000;

/// Flash register offsets
pub mod reg {
    /// Flash access control register
    pub const ACR: usize = 0x00;
    /// Flash key register
    pub const KEYR: usize = 0x08;
    /// Flash option key register
    pub const OPTKEYR: usize = 0x0C;
    /// Flash status register
    pub const SR: usize = 0x10;
    /// Flash control register
    pub const CR: usize = 0x14;
    /// Flash ECC register
    pub const ECC: usize = 0x18;
    /// Flash option control register
    pub const OPTR: usize = 0x20;
    /// Flash non-secure status register
    pub const NSSR: usize = 0x24;
    /// Flash secure status register
    pub const SECSR: usize = 0x28;
    /// Flash secure control register
    pub const SECCR: usize = 0x2C;
}

/// Flash keys
pub const FLASH_KEY1: u32 = 0x4567_0123;
pub const FLASH_KEY2: u32 = 0xCDEF_89AB;
pub const FLASH_OPTKEY1: u32 = 0x0819_2A3B;
pub const FLASH_OPTKEY2: u32 = 0x4C5D_6E7F;

/// Flash latency (wait states)
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
pub fn init() {
    // Default initialization - flash is ready after reset
    // Wait for any ongoing operations to complete
    wait_not_busy();
}

/// Set flash latency
pub fn set_latency(latency: Latency) {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val &= !(0xF << 0); // Clear LATENCY bits
        val |= (latency as u32) << 0;
        core::ptr::write_volatile(acr, val);

        // Wait for latency to be applied
        while get_latency() != latency {}
    }
}

/// Get current flash latency
pub fn get_latency() -> Latency {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let val = core::ptr::read_volatile(acr);
        match val & 0xF {
            0 => Latency::Ws0,
            1 => Latency::Ws1,
            2 => Latency::Ws2,
            3 => Latency::Ws3,
            4 => Latency::Ws4,
            _ => Latency::Ws5,
        }
    }
}

/// Enable instruction cache
pub fn enable_icache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val |= 1 << 9; // ICEN
        core::ptr::write_volatile(acr, val);
    }
}

/// Disable instruction cache
pub fn disable_icache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val &= !(1 << 9); // ICEN
        core::ptr::write_volatile(acr, val);
    }
}

/// Enable data cache
pub fn enable_dcache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val |= 1 << 10; // DCEN
        core::ptr::write_volatile(acr, val);
    }
}

/// Disable data cache
pub fn disable_dcache() {
    unsafe {
        let acr = (FLASH_BASE + reg::ACR) as *mut u32;
        let mut val = core::ptr::read_volatile(acr);
        val &= !(1 << 10); // DCEN
        core::ptr::write_volatile(acr, val);
    }
}

/// Unlock flash for write/erase operations
pub fn unlock() -> Result<(), FlashError> {
    unsafe {
        let cr = (FLASH_BASE + reg::CR) as *mut u32;

        // Check if already unlocked
        if (core::ptr::read_volatile(cr) & (1 << 0)) == 0 {
            return Ok(());
        }

        // Write key sequence
        let keyr = (FLASH_BASE + reg::KEYR) as *mut u32;
        core::ptr::write_volatile(keyr, FLASH_KEY1);
        core::ptr::write_volatile(keyr, FLASH_KEY2);

        // Verify unlock
        if (core::ptr::read_volatile(cr) & (1 << 0)) != 0 {
            return Err(FlashError::UnlockFailed);
        }

        Ok(())
    }
}

/// Lock flash
pub fn lock() {
    unsafe {
        let cr = (FLASH_BASE + reg::CR) as *mut u32;
        let mut val = core::ptr::read_volatile(cr);
        val |= 1 << 0; // LOCK
        core::ptr::write_volatile(cr, val);
    }
}

/// Wait for flash not busy
pub fn wait_not_busy() {
    unsafe {
        let sr = (FLASH_BASE + reg::SR) as *mut u32;
        // Wait for BSY bit to clear
        while (core::ptr::read_volatile(sr) & (1 << 16)) != 0 {}
    }
}

/// Check and clear error flags
pub fn clear_errors() {
    unsafe {
        let sr = (FLASH_BASE + reg::SR) as *mut u32;
        let val = core::ptr::read_volatile(sr);
        // Clear all error flags
        core::ptr::write_volatile(sr, val & 0x0000_FFFF);
    }
}

/// Erase a sector (page)
///
/// # Arguments
/// * `sector` - Sector number to erase
pub fn erase_sector(sector: u32) -> Result<(), FlashError> {
    unlock()?;

    unsafe {
        wait_not_busy();
        clear_errors();

        let cr = (FLASH_BASE + reg::CR) as *mut u32;

        // Configure sector erase
        let mut val = core::ptr::read_volatile(cr);
        val &= !(0xFF << 8); // Clear SNB bits
        val |= sector << 8;  // Set sector number
        val |= 1 << 1;       // SER (Sector Erase)
        core::ptr::write_volatile(cr, val);

        // Start erase
        val |= 1 << 7; // STRT
        core::ptr::write_volatile(cr, val);

        // Wait for completion
        wait_not_busy();

        // Clear SER bit
        let mut val = core::ptr::read_volatile(cr);
        val &= !(1 << 1);
        core::ptr::write_volatile(cr, val);
    }

    lock();
    Ok(())
}

/// Program a 64-bit double word
///
/// # Arguments
/// * `address` - Flash address to write (must be 8-byte aligned)
/// * `data` - 64-bit data to write
pub fn program_double_word(address: usize, data: u64) -> Result<(), FlashError> {
    if address % 8 != 0 {
        return Err(FlashError::AlignmentError);
    }

    unlock()?;

    unsafe {
        wait_not_busy();
        clear_errors();

        let cr = (FLASH_BASE + reg::CR) as *mut u32;

        // Enable programming
        let mut val = core::ptr::read_volatile(cr);
        val |= 1 << 0; // PG
        core::ptr::write_volatile(cr, val);

        // Write data (two 32-bit writes)
        let ptr = address as *mut u32;
        core::ptr::write_volatile(ptr, data as u32);
        core::ptr::write_volatile(ptr.add(1), (data >> 32) as u32);

        // Wait for completion
        wait_not_busy();

        // Clear PG bit
        let mut val = core::ptr::read_volatile(cr);
        val &= !(1 << 0);
        core::ptr::write_volatile(cr, val);
    }

    lock();
    Ok(())
}

/// Program data to flash
///
/// # Arguments
/// * `address` - Starting flash address (must be 8-byte aligned)
/// * `data` - Data to write
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
pub fn get_flash_size_kb() -> u32 {
    // Read from device info
    unsafe {
        let flash_size_reg = 0x1FFF_75E0 as *const u16;
        core::ptr::read_volatile(flash_size_reg) as u32
    }
}

/// Get unique device ID
pub fn get_unique_id() -> [u32; 3] {
    unsafe {
        let uid_base = 0x1FFF_7580 as *const u32;
        [
            core::ptr::read_volatile(uid_base),
            core::ptr::read_volatile(uid_base.add(1)),
            core::ptr::read_volatile(uid_base.add(2)),
        ]
    }
}
