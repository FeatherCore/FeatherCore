//! JPEG - JPEG Codec
//! JPEG 编解码器
//!
//! # Overview / 概述
//! STM32U5 JPEG codec provides hardware acceleration for JPEG image compression
//! and decompression, supporting various JPEG formats and features.
//!
//! # Features / 功能特性
//! Reference: RM0456 Chapter 46: JPEG codec (JPEG)
//!
//! ## Main Features / 主要特性
//! - JPEG baseline compression and decompression
//! - Support for YCbCr 4:2:0, 4:2:2, 4:4:4 formats
//! - Support for grayscale format
//! - Up to 8192 x 8192 resolution
//! - DMA support
//! - Error concealment
//!
//! # Reference / 参考
//! - RM0456 Chapter 46: JPEG codec (JPEG)
//!   - Register map: RM0456, Section 46.7, pages 1931-1970
//!   - JPEG Control Register (JPEG_CR): RM0456, Section 46.7.1, page 1932
//!   - JPEG Status Register (JPEG_SR): RM0456, Section 46.7.2, page 1935
//!   - JPEG Configuration Register (JPEG_CONFR): RM0456, Section 46.7.3, page 1937

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// JPEG base address / JPEG 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const JPEG_BASE: usize = 0x5200_4000;

/// JPEG register offsets
pub mod reg {
    /// JPEG Control Register (JPEG_CR)
    /// RM0456, Section 46.7.1, page 1932
    pub const CR: usize = 0x00;
    /// JPEG Status Register (JPEG_SR)
    /// RM0456, Section 46.7.2, page 1935
    pub const SR: usize = 0x04;
    /// JPEG Configuration Register 1 (JPEG_CONFR1)
    /// RM0456, Section 46.7.3, page 1937
    pub const CONFR1: usize = 0x08;
    /// JPEG Configuration Register 2 (JPEG_CONFR2)
    pub const CONFR2: usize = 0x0C;
    /// JPEG Configuration Register 3 (JPEG_CONFR3)
    pub const CONFR3: usize = 0x10;
    /// JPEG Configuration Register 4 (JPEG_CONFR4)
    pub const CONFR4: usize = 0x14;
    /// JPEG Data Input Register (JPEG_DIR)
    pub const DIR: usize = 0x20;
    /// JPEG Data Output Register (JPEG_DOR)
    pub const DOR: usize = 0x24;
}

/// JPEG register bit definitions
pub mod bits {
    /// JPEG Control Register (JPEG_CR) bits
    pub mod cr {
        /// JPEG Enable (JPEGEN)
        pub const JPEGEN: u32 = 1 << 0;
        /// Start (START)
        pub const START: u32 = 1 << 1;
        /// Mode Select (MODSEL)
        pub const MODSEL: u32 = 1 << 3;
        /// Interrupt Enable (IE)
        pub const IE: u32 = 1 << 4;
        /// DMA Input Enable (DMAINEN)
        pub const DMAINEN: u32 = 1 << 11;
        /// DMA Output Enable (DMAOUTEN)
        pub const DMAOUTEN: u32 = 1 << 12;
        /// FIFO Threshold (FTH)
        pub const FTH_MASK: u32 = 0b111 << 6;
    }

    /// JPEG Status Register (JPEG_SR) bits
    pub mod sr {
        /// Ready for Data Input (RDYIN)
        pub const RDYIN: u32 = 1 << 0;
        /// Ready for Data Output (RDYOUT)
        pub const RDYOUT: u32 = 1 << 1;
        /// End of Conversion (EOC)
        pub const EOC: u32 = 1 << 2;
        /// Header Parsed (HDP)
        pub const HDP: u32 = 1 << 3;
        /// FIFO Empty (FEF)
        pub const FEF: u32 = 1 << 4;
        /// FIFO Full (FFF)
        pub const FFF: u32 = 1 << 5;
    }
}

/// JPEG color format
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorFormat {
    /// Grayscale
    Grayscale = 0,
    /// YCbCr 4:2:0
    YCbCr420 = 1,
    /// YCbCr 4:2:2
    YCbCr422 = 2,
    /// YCbCr 4:4:4
    YCbCr444 = 3,
}

/// JPEG operation mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperationMode {
    /// Compression mode
    Compress = 0,
    /// Decompression mode
    Decompress = 1,
}

/// JPEG configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Operation mode
    pub mode: OperationMode,
    /// Color format
    pub color_format: ColorFormat,
    /// Image width
    pub width: u16,
    /// Image height
    pub height: u16,
    /// Quality (0-100)
    pub quality: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: OperationMode::Compress,
            color_format: ColorFormat::YCbCr420,
            width: 640,
            height: 480,
            quality: 75,
        }
    }
}

/// JPEG instance
pub struct Jpeg;

impl Jpeg {
    /// Create JPEG instance
    pub const fn new() -> Self {
        Self
    }

    /// Enable JPEG
    pub fn enable(&self) {
        unsafe {
            let cr = (JPEG_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= bits::cr::JPEGEN;
            write_volatile(cr, val);
        }
    }

    /// Disable JPEG
    pub fn disable(&self) {
        unsafe {
            let cr = (JPEG_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !bits::cr::JPEGEN;
            write_volatile(cr, val);
        }
    }

    /// Initialize JPEG with configuration
    pub fn init(&self, config: &Config) {
        unsafe {
            let cr = (JPEG_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= bits::cr::JPEGEN;
            if config.mode == OperationMode::Decompress {
                val |= bits::cr::MODSEL;
            }
            write_volatile(cr, val);
        }
    }

    /// Start JPEG operation
    pub fn start(&self) {
        unsafe {
            let cr = (JPEG_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= bits::cr::START;
            write_volatile(cr, val);
        }
    }

    /// Check if ready for input
    pub fn is_ready_input(&self) -> bool {
        unsafe {
            let sr = (JPEG_BASE + reg::SR) as *mut u32;
            (read_volatile(sr) & bits::sr::RDYIN) != 0
        }
    }

    /// Check if ready for output
    pub fn is_ready_output(&self) -> bool {
        unsafe {
            let sr = (JPEG_BASE + reg::SR) as *mut u32;
            (read_volatile(sr) & bits::sr::RDYOUT) != 0
        }
    }

    /// Check if operation complete
    pub fn is_complete(&self) -> bool {
        unsafe {
            let sr = (JPEG_BASE + reg::SR) as *mut u32;
            (read_volatile(sr) & bits::sr::EOC) != 0
        }
    }

    /// Write data
    pub fn write_data(&self, data: u32) {
        unsafe {
            let dir = (JPEG_BASE + reg::DIR) as *mut u32;
            write_volatile(dir, data);
        }
    }

    /// Read data
    pub fn read_data(&self) -> u32 {
        unsafe {
            let dor = (JPEG_BASE + reg::DOR) as *const u32;
            read_volatile(dor)
        }
    }

    /// Enable DMA
    pub fn enable_dma(&self, input: bool, output: bool) {
        unsafe {
            let cr = (JPEG_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            if input {
                val |= bits::cr::DMAINEN;
            }
            if output {
                val |= bits::cr::DMAOUTEN;
            }
            write_volatile(cr, val);
        }
    }

    /// Disable DMA
    pub fn disable_dma(&self) {
        unsafe {
            let cr = (JPEG_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !(bits::cr::DMAINEN | bits::cr::DMAOUTEN);
            write_volatile(cr, val);
        }
    }
}

impl Default for Jpeg {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize JPEG with default configuration
pub fn init_jpeg_default() -> Jpeg {
    let jpeg = Jpeg::new();
    let config = Config::default();
    jpeg.init(&config);
    jpeg
}

