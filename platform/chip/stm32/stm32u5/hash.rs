//! HASH - Hash Processor
//! 哈希处理器
//!
//! STM32U5 HASH 特性：
//! - SHA-1, SHA-224, SHA-256
//! - MD5
//! - HMAC 支持
//! - DMA 支持

/// HASH base address
pub const HASH_BASE: usize = 0x420C_0400;

/// HASH register offsets
pub mod reg {
    pub const CR: usize = 0x00;
    pub const DIN: usize = 0x04;
    pub const STR: usize = 0x08;
    pub const HR0: usize = 0x0C;
    pub const HR1: usize = 0x10;
    pub const HR2: usize = 0x14;
    pub const HR3: usize = 0x18;
    pub const HR4: usize = 0x1C;
    pub const HR5: usize = 0x20;
    pub const HR6: usize = 0x24;
    pub const HR7: usize = 0x28;
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
