//! PKA - Public Key Accelerator
//! 公钥加速器
//!
//! # Overview / 概述
//! STM32U5 Public Key Accelerator (PKA) provides hardware acceleration for
//! asymmetric cryptographic operations including RSA and ECC.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 53: Public key accelerator (PKA)
//! 
//! ## Supported Algorithms / 支持的算法
//! - RSA (up to 4096-bit)
//! - ECC (Elliptic Curve Cryptography)
//! - ECDSA (Elliptic Curve Digital Signature Algorithm)
//! - ECDH (Elliptic Curve Diffie-Hellman)
//! - DH (Diffie-Hellman)
//! 
//! ## Advanced Features / 高级特性
//! - Hardware acceleration for large number operations
//! - Multiple elliptic curve support
//! - DMA support
//! 
//! # Reference / 参考
//! - RM0456 Chapter 53: Public key accelerator (PKA)
//! - RM0456 Section 53.1: PKA introduction
//! - RM0456 Section 53.2: PKA main features
//! - RM0456 Section 53.3: PKA functional description
//! - RM0456 Section 53.4: PKA registers

/// PKA base address / PKA 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const PKA_BASE: usize = 0x420C_0800;

/// PKA register offsets
//! Reference: RM0456 Section 53.4: PKA register map
pub mod reg {
    /// PKA control register
    //! Reference: RM0456 Section 53.4.1: PKA control register (PKA_CR)
    pub const CR: usize = 0x00;
    /// PKA status register
    //! Reference: RM0456 Section 53.4.2: PKA status register (PKA_SR)
    pub const SR: usize = 0x04;
    /// PKA clear flag register
    //! Reference: RM0456 Section 53.4.3: PKA clear flag register (PKA_CLRFR)
    pub const CLRFR: usize = 0x08;
    /// PKA RAM start address
    //! Reference: RM0456 Section 53.4.4: PKA RAM start address (PKA_RAM0)
    pub const RAM0: usize = 0x400;
}

/// PKA operations
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    /// RSA encryption/decryption
    RsaEncDec = 0x00,
    /// RSA signature/verification
    RsaSignVerify = 0x01,
    /// ECC sign
    EccSign = 0x10,
    /// ECC verification
    EccVerify = 0x11,
    /// ECC key generation
    EccKeyGen = 0x12,
    /// ECDH key exchange
    Ecdh = 0x13,
    /// Montgomery multiplication
    Montgomery = 0x20,
}

/// PKA error codes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PkaError {
    None,
    Timeout,
    IllegalParam,
    OutOfRange,
    DivByZero,
}

/// PKA instance
pub struct Pka;

impl Pka {
    /// Create PKA instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable PKA clock and reset
    pub fn enable(&self) {
        unsafe {
            let rcc_base = crate::rcc::RCC_BASE as *mut u32;
            let ahb2enr = rcc_base.add(0x4C / 4);
            *ahb2enr |= 1 << 8; // PKAEN

            // Reset PKA
            let rcc_base = crate::rcc::RCC_BASE as *mut u32;
            let ahb2rstr = rcc_base.add(0x2C / 4);
            *ahb2rstr |= 1 << 8;
            *ahb2rstr &= !(1 << 8);
        }
    }

    /// Start PKA operation
    pub fn start_operation(&self, operation: Operation) {
        unsafe {
            let cr = (PKA_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // START
            val |= (operation as u32) << 8; // PROC16
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Wait for operation to complete
    pub fn wait_completion(&self) -> Result<(), PkaError> {
        let mut timeout = 1000000;
        unsafe {
            let sr = (PKA_BASE + reg::SR) as *mut u32;
            while timeout > 0 {
                let val = core::ptr::read_volatile(sr);
                if val & (1 << 0) != 0 { // PROC_DONE
                    return Ok(());
                }
                if val & 0x1F != 0 { // Error flags
                    return Err(PkaError::IllegalParam);
                }
                timeout -= 1;
            }
            Err(PkaError::Timeout)
        }
    }

    /// Clear flags
    pub fn clear_flags(&self, flags: u32) {
        unsafe {
            let clrfr = (PKA_BASE + reg::CLRFR) as *mut u32;
            core::ptr::write_volatile(clrfr, flags);
        }
    }

    /// Read from PKA RAM
    pub fn read_ram(&self, offset: usize) -> u32 {
        unsafe {
            let addr = PKA_BASE + reg::RAM0 + offset * 4;
            let ptr = addr as *const u32;
            core::ptr::read_volatile(ptr)
        }
    }

    /// Write to PKA RAM
    pub fn write_ram(&self, offset: usize, value: u32) {
        unsafe {
            let addr = PKA_BASE + reg::RAM0 + offset * 4;
            let ptr = addr as *mut u32;
            core::ptr::write_volatile(ptr, value);
        }
    }
}

/// Initialize PKA
pub fn init() {
    let pka = Pka::new();
    pka.enable();
}
