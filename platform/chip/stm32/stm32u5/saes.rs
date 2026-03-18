//! SAES - Secure AES Coprocessor
//! 安全 AES 协处理器
//!
//! # Overview / 概述
//! STM32U5 Secure AES (SAES) provides hardware acceleration for AES encryption/decryption
//! with enhanced security features for TrustZone applications.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 50: Secure AES coprocessor (SAES)
//!
//! ## AES Features / AES 特性
//! - AES-128/192/256 encryption and decryption
//! - ECB, CBC, CTR, GCM, CCM modes
//! - DMA support
//! - Key protection in secure memory
//!
//! ## Security Features / 安全特性
//! - Secure key storage
//! - Hardware key derivation
//! - Side-channel attack protection
//! - TrustZone support
//!
//! # Reference / 参考
//! - RM0456 Chapter 50: Secure AES coprocessor (SAES)
//!   - Register map: RM0456, Section 50.4, pages 2085-2107
//!   - SAES Control Register (SAES_CR): RM0456, Section 50.4.1, page 2086
//!   - SAES Status Register (SAES_SR): RM0456, Section 50.4.2, page 2088
//!   - SAES Key Registers (SAES_KEYRx): RM0456, Section 50.4.5, page 2091
//!   - SAES Initialization Vector Registers (SAES_IVRx): RM0456, Section 50.4.6, page 2092

use core::ptr::{read_volatile, write_volatile};

/// SAES base address / SAES 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const SAES_BASE: usize = 0x4002_8000;

/// SAES register offsets / SAES 寄存器偏移
//! Reference: RM0456 Section 50.4: SAES register map
pub mod reg {
    /// SAES control register
    //! Reference: RM0456 Section 50.4.1: SAES control register (SAES_CR)
    pub const CR: usize = 0x00;
    /// SAES status register
    //! Reference: RM0456 Section 50.4.2: SAES status register (SAES_SR)
    pub const SR: usize = 0x04;
    /// SAES data input register
    //! Reference: RM0456 Section 50.4.3: SAES data input register (SAES_DINR)
    pub const DINR: usize = 0x08;
    /// SAES data output register
    //! Reference: RM0456 Section 50.4.4: SAES data output register (SAES_DOUTR)
    pub const DOUTR: usize = 0x0C;
    /// SAES key register 0
    //! Reference: RM0456 Section 50.4.5: SAES key register (SAES_KEYR0)
    pub const KEYR0: usize = 0x10;
    /// SAES key register 1
    pub const KEYR1: usize = 0x14;
    /// SAES key register 2
    pub const KEYR2: usize = 0x18;
    /// SAES key register 3
    pub const KEYR3: usize = 0x1C;
    /// SAES initialization vector register 0
    pub const IVR0: usize = 0x20;
    /// SAES initialization vector register 1
    pub const IVR1: usize = 0x24;
    /// SAES initialization vector register 2
    pub const IVR2: usize = 0x28;
    /// SAES initialization vector register 3
    pub const IVR3: usize = 0x2C;
    /// SAES key extension register 0
    pub const KEYEXT0: usize = 0x30;
    /// SAES key extension register 1
    pub const KEYEXT1: usize = 0x34;
    /// SAES key extension register 2
    pub const KEYEXT2: usize = 0x38;
    /// SAES key extension register 3
    pub const KEYEXT3: usize = 0x3C;
    /// SAES key extension register 4
    pub const KEYEXT4: usize = 0x40;
    /// SAES key extension register 5
    pub const KEYEXT5: usize = 0x44;
    /// SAES key extension register 6
    pub const KEYEXT6: usize = 0x48;
    /// SAES key extension register 7
    pub const KEYEXT7: usize = 0x4C;
}

/// SAES register bit definitions
pub mod bits {
    /// SAES Control Register (SAES_CR) bits
    pub mod cr {
        /// SAES Enable (EN)
        pub const EN: u32 = 1 << 0;
        /// Algorithm Mode (ALGOMODE)
        pub const ALGOMODE_MASK: u32 = 0b111 << 1;
        pub const ALGOMODE_ECB: u32 = 0b000 << 1;
        pub const ALGOMODE_CBC: u32 = 0b001 << 1;
        pub const ALGOMODE_CTR: u32 = 0b010 << 1;
        pub const ALGOMODE_GCM: u32 = 0b011 << 1;
        pub const ALGOMODE_CCM: u32 = 0b100 << 1;
        /// Key Size (KEYSIZE)
        pub const KEYSIZE_MASK: u32 = 0b11 << 4;
        pub const KEYSIZE_128: u32 = 0b00 << 4;
        pub const KEYSIZE_192: u32 = 0b01 << 4;
        pub const KEYSIZE_256: u32 = 0b10 << 4;
        /// Chaining Mode (CHMOD)
        pub const CHMOD_MASK: u32 = 0b111 << 6;
        pub const CHMOD_ECB: u32 = 0b000 << 6;
        pub const CHMOD_CBC: u32 = 0b001 << 6;
        pub const CHMOD_CTR: u32 = 0b010 << 6;
        pub const CHMOD_GCM: u32 = 0b011 << 6;
        pub const CHMOD_CCM: u32 = 0b100 << 6;
        /// DMA Enable (DMAEN)
        pub const DMAEN: u32 = 1 << 11;
        /// Data Type (DATATYPE)
        pub const DATATYPE_MASK: u32 = 0b11 << 12;
        pub const DATATYPE_WORD: u32 = 0b00 << 12;
        pub const DATATYPE_HALFWORD: u32 = 0b01 << 12;
        pub const DATATYPE_BYTE: u32 = 0b10 << 12;
        pub const DATATYPE_BIT: u32 = 0b11 << 12;
        /// Key Selection (KEYSEL)
        pub const KEYSEL_MASK: u32 = 0b111 << 16;
        pub const KEYSEL_SW: u32 = 0b000 << 16;
        pub const KEYSEL_HUK: u32 = 0b001 << 16;
        pub const KEYSEL_BHK: u32 = 0b010 << 16;
        pub const KEYSEL_KL: u32 = 0b011 << 16;
        /// Key Derivation Enable (KDEN)
        pub const KDEN: u32 = 1 << 23;
        /// Key Derivation Mode (KDMOD)
        pub const KDMOD_MASK: u32 = 0b11 << 24;
    }

    /// SAES Status Register (SAES_SR) bits
    pub mod sr {
        /// Busy Flag (BUSY)
        pub const BUSY: u32 = 1 << 0;
        /// Write FIFO Empty (WRERR)
        pub const WRERR: u32 = 1 << 1;
        /// Read FIFO Empty (RDERR)
        pub const RDERR: u32 = 1 << 2;
        /// Computation Complete (CCF)
        pub const CCF: u32 = 1 << 3;
        /// Key Derivation Complete (KDCF)
        pub const KDCF: u32 = 1 << 4;
    }
}

/// SAES operation mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Encryption mode
    Encrypt = 0,
    /// Decryption mode
    Decrypt = 1,
}

/// SAES key size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeySize {
    /// 128-bit key
    Key128 = 0,
    /// 192-bit key
    Key192 = 1,
    /// 256-bit key
    Key256 = 2,
}

/// SAES cryptographic mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CryptoMode {
    /// Electronic Codebook (ECB)
    ECB = 0,
    /// Cipher Block Chaining (CBC)
    CBC = 1,
    /// Counter (CTR)
    CTR = 2,
    /// Galois/Counter Mode (GCM)
    GCM = 3,
}

/// SAES instance
pub struct Saes;

impl Saes {
    /// Create SAES instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable SAES
    pub fn enable(&self) {
        unsafe {
            let cr = (SAES_BASE + reg::CR) as *mut u32;
            write_volatile(cr, 1 << 0);
        }
    }

    /// Disable SAES
    pub fn disable(&self) {
        unsafe {
            let cr = (SAES_BASE + reg::CR) as *mut u32;
            write_volatile(cr, 0);
        }
    }

    /// Set operation mode
    pub fn set_mode(&self, mode: Mode) {
        unsafe {
            let cr = (SAES_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, (val & !(1 << 1)) | ((mode as u32) << 1));
        }
    }

    /// Set key size
    pub fn set_key_size(&self, key_size: KeySize) {
        unsafe {
            let cr = (SAES_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, (val & !(0x3 << 3)) | ((key_size as u32 & 0x3) << 3));
        }
    }

    /// Set cryptographic mode
    pub fn set_crypto_mode(&self, crypto_mode: CryptoMode) {
        unsafe {
            let cr = (SAES_BASE + reg::CR) as *mut u32;
            let val = read_volatile(cr);
            write_volatile(cr, (val & !(0xF << 5)) | ((crypto_mode as u32 & 0xF) << 5));
        }
    }

    /// Write key
    pub fn write_key(&self, key: &[u32; 8]) {
        unsafe {
            for (i, &key_word) in key.iter().enumerate() {
                let keyr = (SAES_BASE + reg::KEYR0 + i * 4) as *mut u32;
                write_volatile(keyr, key_word);
            }
        }
    }

    /// Write initialization vector
    pub fn write_iv(&self, iv: &[u32; 4]) {
        unsafe {
            for (i, &iv_word) in iv.iter().enumerate() {
                let ivr = (SAES_BASE + reg::IVR0 + i * 4) as *mut u32;
                write_volatile(ivr, iv_word);
            }
        }
    }

    /// Write data input
    pub fn write_data(&self, data: u32) {
        unsafe {
            let dinr = (SAES_BASE + reg::DINR) as *mut u32;
            write_volatile(dinr, data);
        }
    }

    /// Read data output
    pub fn read_data(&self) -> u32 {
        unsafe {
            let doutr = (SAES_BASE + reg::DOUTR) as *const u32;
            read_volatile(doutr)
        }
    }

    /// Get status
    pub fn status(&self) -> u32 {
        unsafe {
            let sr = (SAES_BASE + reg::SR) as *const u32;
            read_volatile(sr)
        }
    }

    /// Check if busy
    pub fn is_busy(&self) -> bool {
        (self.status() & 0x1) != 0
    }
}

impl Default for Saes {
    fn default() -> Self {
        Self::new()
    }
}
