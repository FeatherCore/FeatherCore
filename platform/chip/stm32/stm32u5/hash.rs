//! HASH - Hash Processor
//! 哈希处理器
//!
//! # Overview / 概述
//! The hash processor provides hardware acceleration for SHA-1, SHA-224, SHA-256,
//! and MD5 hash algorithms.
//!
//! # Features / 功能特性
//! - SHA-1, SHA-224, SHA-256 algorithms
//! - MD5 algorithm
//! - DMA support for efficient data processing
//! - Automatic padding
//! - HMAC support
//! - Multi-buffer support
//!
//! # Reference / 参考
//! - RM0456 Chapter 51: Hash processor (HASH)
//!   - Register map: RM0456, Section 51.7, pages 2145-2169
//!   - HASH Control Register (HASH_CR): RM0456, Section 51.7.1, page 2146
//!   - HASH Status Register (HASH_SR): RM0456, Section 51.7.2, page 2148
//!   - HASH Data Input Register (HASH_DIN): RM0456, Section 51.7.3, page 2149
//!   - HASH Digest Register (HASH_HRx): RM0456, Section 51.7.5, page 2151

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// HASH base address / HASH 基地址
/// Reference: RM0456 Chapter 2, Table 1
pub const HASH_BASE: usize = 0x4002_1400;

/// HASH register offsets
pub mod reg {
    /// HASH Control Register (HASH_CR)
    /// RM0456, Section 51.7.1, page 2146
    pub const CR: usize = 0x00;
    /// HASH Status Register (HASH_SR)
    /// RM0456, Section 51.7.2, page 2148
    pub const SR: usize = 0x04;
    /// HASH Data Input Register (HASH_DIN)
    /// RM0456, Section 51.7.3, page 2149
    pub const DIN: usize = 0x08;
    /// HASH Start Register (HASH_STR)
    /// RM0456, Section 51.7.4, page 2150
    pub const STR: usize = 0x0C;
    /// HASH Digest Register 0 (HASH_HR0)
    /// RM0456, Section 51.7.5, page 2151
    pub const HR0: usize = 0x310;
    /// HASH Digest Register 1 (HASH_HR1)
    pub const HR1: usize = 0x314;
    /// HASH Digest Register 2 (HASH_HR2)
    pub const HR2: usize = 0x318;
    /// HASH Digest Register 3 (HASH_HR3)
    pub const HR3: usize = 0x31C;
    /// HASH Digest Register 4 (HASH_HR4)
    pub const HR4: usize = 0x320;
    /// HASH Digest Register 5 (HASH_HR5)
    pub const HR5: usize = 0x324;
    /// HASH Digest Register 6 (HASH_HR6)
    pub const HR6: usize = 0x328;
    /// HASH Digest Register 7 (HASH_HR7)
    pub const HR7: usize = 0x32C;
    /// HASH Context Swap Registers (HASH_CSRx)
    /// RM0456, Section 51.7.11, page 2163
    pub const CSR0: usize = 0x0F8;
}

/// HASH register bit definitions
pub mod bits {
    /// HASH Control Register (HASH_CR) bits
    pub mod cr {
        /// Algorithm Selection (ALGO)
        pub const ALGO_MASK: u32 = 0b111 << 0;
        pub const ALGO_SHA1: u32 = 0b000 << 0;
        pub const ALGO_MD5: u32 = 0b001 << 0;
        pub const ALGO_SHA224: u32 = 0b010 << 0;
        pub const ALGO_SHA256: u32 = 0b011 << 0;
        /// Data Type (DATATYPE)
        pub const DATATYPE_MASK: u32 = 0b11 << 4;
        pub const DATATYPE_WORD: u32 = 0b00 << 4;
        pub const DATATYPE_HALFWORD: u32 = 0b01 << 4;
        pub const DATATYPE_BYTE: u32 = 0b10 << 4;
        pub const DATATYPE_BIT: u32 = 0b11 << 4;
        /// Mode Selection (MODE)
        pub const MODE: u32 = 1 << 6;
        /// DMA Enable (DMAE)
        pub const DMAE: u32 = 1 << 3;
        /// Initialize message digest (INIT)
        pub const INIT: u32 = 1 << 2;
    }

    /// HASH Status Register (HASH_SR) bits
    pub mod sr {
        /// Input FIFO Not Full (DINIS)
        pub const DINIS: u32 = 1 << 0;
        /// Digest Calculation Complete (DCIS)
        pub const DCIS: u32 = 1 << 1;
        /// DMA Input FIFO Service Request (DMAS)
        pub const DMAS: u32 = 1 << 2;
        /// Busy Flag (BUSY)
        pub const BUSY: u32 = 1 << 3;
    }
}

/// HASH control register
#[derive(Clone, Copy, Debug)]
pub struct HashControl {
    pub digest_selected: bool,
    pub mode: HashMode,
    pub datatype: HashDatatype,
    pub dma_enable: bool,
    pub last_word_valid: bool,
    pub nb_bits_valid: u8,
    pub init: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum HashMode {
    SHA1,
    MD5,
    SHA224,
    SHA256,
}

#[derive(Clone, Copy, Debug)]
pub enum HashDatatype {
    Word,
    HalfWord,
    Byte,
    Bit,
}

impl Default for HashControl {
    fn default() -> Self {
        Self {
            digest_selected: false,
            mode: HashMode::SHA256,
            datatype: HashDatatype::Word,
            dma_enable: false,
            last_word_valid: true,
            nb_bits_valid: 0,
            init: false,
        }
    }
}

/// HASH digest output
#[derive(Clone, Copy, Debug)]
pub struct Digest {
    pub data: [u32; 8],
}

impl HashControl {
    pub fn configure(&self) {
        unsafe {
            let cr = HASH_BASE as *mut u32;
            let mut val: u32 = 0;
            
            match self.mode {
                HashMode::SHA1 => val |= 0 << 3,
                HashMode::MD5 => val |= 1 << 3,
                HashMode::SHA224 => val |= 2 << 3,
                HashMode::SHA256 => val |= 3 << 3,
            }
            
            match self.datatype {
                HashDatatype::Word => {},
                HashDatatype::HalfWord => val |= 1 << 1,
                HashDatatype::Byte => val |= 2 << 1,
                HashDatatype::Bit => val |= 3 << 1,
            }
            
            if self.dma_enable {
                val |= 1 << 8;
            }
            
            if self.init {
                val |= 1 << 2;
            }
            
            write_volatile(cr, val);
        }
    }
}

/// Initialize HASH peripheral
pub fn init() {
    unsafe {
        let cr = HASH_BASE as *mut u32;
        write_volatile(cr, 1 << 2);
    }
}

/// Compute hash of data
pub fn compute_hash(data: &[u32]) -> Digest {
    let mut ctrl = HashControl::default();
    ctrl.init = true;
    ctrl.configure();
    
    unsafe {
        let din = (HASH_BASE + 0x400) as *mut u32;
        for (i, &word) in data.iter().enumerate() {
            if i < 16 {
                write_volatile(din.add(i), word);
            }
        }
        
        let cr = HASH_BASE as *mut u32;
        write_volatile(cr, read_volatile(cr) | 1 << 0);
        
        while read_volatile(cr) & (1 << 1) == 0 {}
        
        let digest_ptr = (HASH_BASE + 0x400) as *const u32;
        let mut digest = Digest { data: [0u32; 8] };
        for i in 0..8 {
            digest.data[i] = read_volatile(digest_ptr.add(i));
        }
        digest
    }
}
