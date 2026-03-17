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
//!
//! # Reference / 参考
//! - RM0456 Chapter 51: Hash processor (HASH)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// HASH base address / HASH 基地址
/// Reference: RM0456 Chapter 2, Table 1
pub const HASH_BASE: usize = 0x4002_1400;

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
