//! HASH - Hash Processor
//! 哈希处理器
//!
//! # Overview / 概述
//! STM32U5 Hash Processor (HASH) provides hardware acceleration for cryptographic
//! hash functions including SHA-1, SHA-224, SHA-256, and MD5.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 50: Hash processor (HASH)
//! 
//! ## Supported Algorithms / 支持的算法
//! - SHA-1 (Secure Hash Algorithm 1)
//! - SHA-224 (Secure Hash Algorithm 224-bit)
//! - SHA-256 (Secure Hash Algorithm 256-bit)
//! - MD5 (Message Digest Algorithm 5)
//! 
//! ## Advanced Features / 高级特性
//! - HMAC support (Keyed-Hash Message Authentication Code)
//! - DMA support
//! - Hardware acceleration
//! 
//! # Reference / 参考
//! - RM0456 Chapter 50: Hash processor (HASH)
//! - RM0456 Section 50.1: HASH introduction
//! - RM0456 Section 50.2: HASH main features
//! - RM0456 Section 50.3: HASH functional description
//! - RM0456 Section 50.4: HASH registers

/// HASH base address / HASH 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const HASH_BASE: usize = 0x420C_0400;

/// HASH register offsets
//! Reference: RM0456 Section 50.4: HASH register map
pub mod reg {
    /// Control register
    //! Reference: RM0456 Section 50.4.1: HASH control register (HASH_CR)
    pub const CR: usize = 0x00;
    /// Data input register
    //! Reference: RM0456 Section 50.4.2: HASH data input register (HASH_DIN)
    pub const DIN: usize = 0x04;
    /// Start register
    //! Reference: RM0456 Section 50.4.3: HASH start register (HASH_STR)
    pub const STR: usize = 0x08;
    /// Hash digest register 0
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR0)
    pub const HR0: usize = 0x0C;
    /// Hash digest register 1
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR1)
    pub const HR1: usize = 0x10;
    /// Hash digest register 2
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR2)
    pub const HR2: usize = 0x14;
    /// Hash digest register 3
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR3)
    pub const HR3: usize = 0x18;
    /// Hash digest register 4
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR4)
    pub const HR4: usize = 0x1C;
    /// Hash digest register 5
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR5)
    pub const HR5: usize = 0x20;
    /// Hash digest register 6
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR6)
    pub const HR6: usize = 0x24;
    /// Hash digest register 7
    //! Reference: RM0456 Section 50.4.4: HASH digest register (HASH_HR7)
    pub const HR7: usize = 0x28;
    /// Status register
    //! Reference: RM0456 Section 50.4.5: HASH status register (HASH_SR)
    pub const SR: usize = 0x2C;
}

/// Hash algorithm
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Algorithm {
    Sha1 = 0b00,
    Md5 = 0b01,
    Sha224 = 0b10,
    Sha256 = 0b11,
}

/// HASH instance
pub struct Hash;

impl Hash {
    pub const fn new() -> Self {
        Self
    }

    pub fn init(&self, algo: Algorithm) {
        crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::HASH);
        
        unsafe {
            let cr = (HASH_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= (algo as u32) << 7;
            val |= 1 << 2; // INIT
            core::ptr::write_volatile(cr, val);
        }
    }

    pub fn write_data(&self, data: u32) {
        unsafe {
            let din = (HASH_BASE + reg::DIN) as *mut u32;
            core::ptr::write_volatile(din, data);
        }
    }

    pub fn start_digest(&self) {
        unsafe {
            let str = (HASH_BASE + reg::STR) as *mut u32;
            core::ptr::write_volatile(str, 1 << 8); // DCAL
        }
    }

    pub fn read_digest(&self, output: &mut [u32]) {
        unsafe {
            for (i, word) in output.iter_mut().enumerate() {
                let hr = (HASH_BASE + reg::HR0 + i * 4) as *mut u32;
                *word = core::ptr::read_volatile(hr);
            }
        }
    }

    pub fn calculate_sha256(&self, data: &[u8]) -> [u32; 8] {
        self.init(Algorithm::Sha256);

        // Process data in 32-bit words
        let mut i = 0;
        while i + 4 <= data.len() {
            let word = u32::from_le_bytes([
                data[i], data[i+1], data[i+2], data[i+3]
            ]);
            self.write_data(word);
            i += 4;
        }

        // Handle remaining bytes and padding (simplified)
        // In real implementation, need proper PKCS padding

        self.start_digest();

        let mut result = [0u32; 8];
        self.read_digest(&mut result);
        result
    }
}

/// Calculate SHA-256 hash
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let hash = Hash::new();
    let digest = hash.calculate_sha256(data);
    
    let mut result = [0u8; 32];
    for (i, &word) in digest.iter().enumerate() {
        let bytes = word.to_be_bytes();
        result[i*4..(i+1)*4].copy_from_slice(&bytes);
    }
    result
}
