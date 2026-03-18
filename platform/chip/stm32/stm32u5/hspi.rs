//! HSPI - HyperBus Serial Peripheral Interface
//! HyperBus串行外设接口
//!
//! ## STM32U5 HSPI 特性 / Features
//! - **数据线模式 / Data Line Modes:**
//!   - HyperBus (8数据线)
//!   - Octal SPI (8数据线)
//!   - Quad SPI (4数据线)
//!   - Dual SPI (2数据线)
//!   - Standard SPI (1数据线)
//!
//! - **支持的设备 / Supported Devices:**
//!   - HyperFlash
//!   - HyperRAM
//!   - Octal NOR Flash
//!   - Octal PSRAM
//!
//! - **特性 / Features:**
//!   - 内存映射模式 (Memory-mapped mode)
//!   - XIP (Execute In Place) 支持
//!   - 最高 200 MB/s 吞吐量
//!   - DMA 支持
//!   - 延迟锁相环 (DLL)
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 30: HyperBus interface (HSPI)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// HSPI1 base address / HSPI1 基地址
pub const HSPI1_BASE: usize = 0x4202_0000;

/// HSPI register offsets / HSPI 寄存器偏移
//! Reference: RM0456 Section 30.4: HSPI register map
pub mod reg {
    /// HSPI control register
    //! Reference: RM0456 Section 30.4.1: HSPI control register (HSPI_CR)
    pub const CR: usize = 0x00;
    /// HSPI device configuration register 1
    //! Reference: RM0456 Section 30.4.2: HSPI device configuration register 1 (HSPI_DCR1)
    pub const DCR1: usize = 0x08;
    /// HSPI device configuration register 2
    //! Reference: RM0456 Section 30.4.3: HSPI device configuration register 2 (HSPI_DCR2)
    pub const DCR2: usize = 0x0C;
    /// HSPI device configuration register 3
    //! Reference: RM0456 Section 30.4.4: HSPI device configuration register 3 (HSPI_DCR3)
    pub const DCR3: usize = 0x10;
    /// HSPI device configuration register 4
    //! Reference: RM0456 Section 30.4.5: HSPI device configuration register 4 (HSPI_DCR4)
    pub const DCR4: usize = 0x14;
    /// HSPI status register
    //! Reference: RM0456 Section 30.4.6: HSPI status register (HSPI_SR)
    pub const SR: usize = 0x20;
    /// HSPI flag clear register
    //! Reference: RM0456 Section 30.4.7: HSPI flag clear register (HSPI_FCR)
    pub const FCR: usize = 0x24;
    /// HSPI data length register
    //! Reference: RM0456 Section 30.4.8: HSPI data length register (HSPI_DLR)
    pub const DLR: usize = 0x40;
    /// HSPI address register
    //! Reference: RM0456 Section 30.4.9: HSPI address register (HSPI_AR)
    pub const AR: usize = 0x48;
    /// HSPI data register
    //! Reference: RM0456 Section 30.4.10: HSPI data register (HSPI_DR)
    pub const DR: usize = 0x50;
    /// HSPI polling match mask register
    //! Reference: RM0456 Section 30.4.11: HSPI polling match mask register (HSPI_PSMKR)
    pub const PSMKR: usize = 0x80;
    /// HSPI polling match address register
    //! Reference: RM0456 Section 30.4.12: HSPI polling match address register (HSPI_PSMAR)
    pub const PSMAR: usize = 0x88;
    /// HSPI polling interval register
    //! Reference: RM0456 Section 30.4.13: HSPI polling interval register (HSPI_PIR)
    pub const PIR: usize = 0x90;
    /// HSPI communication configuration register
    //! Reference: RM0456 Section 30.4.14: HSPI communication configuration register (HSPI_CCR)
    pub const CCR: usize = 0x100;
    /// HSPI timing configuration register
    //! Reference: RM0456 Section 30.4.15: HSPI timing configuration register (HSPI_TCR)
    pub const TCR: usize = 0x108;
    /// HSPI instruction register
    //! Reference: RM0456 Section 30.4.16: HSPI instruction register (HSPI_IR)
    pub const IR: usize = 0x110;
    /// HSPI alternate bytes register
    //! Reference: RM0456 Section 30.4.17: HSPI alternate bytes register (HSPI_ABR)
    pub const ABR: usize = 0x118;
    /// HSPI low-power timeout register
    //! Reference: RM0456 Section 30.4.18: HSPI low-power timeout register (HSPI_LPTR)
    pub const LPTR: usize = 0x120;
    /// HSPI write communication configuration register
    //! Reference: RM0456 Section 30.4.19: HSPI write communication configuration register (HSPI_WCCR)
    pub const WCCR: usize = 0x140;
    /// HSPI write timing configuration register
    //! Reference: RM0456 Section 30.4.20: HSPI write timing configuration register (HSPI_WTCR)
    pub const WTCR: usize = 0x148;
    /// HSPI write instruction register
    //! Reference: RM0456 Section 30.4.21: HSPI write instruction register (HSPI_WIR)
    pub const WIR: usize = 0x150;
    /// HSPI write alternate bytes register
    //! Reference: RM0456 Section 30.4.22: HSPI write alternate bytes register (HSPI_WABR)
    pub const WABR: usize = 0x158;
    /// HSPI HyperBus latency configuration register
    //! Reference: RM0456 Section 30.4.23: HSPI HyperBus latency configuration register (HSPI_HLCR)
    pub const HLCR: usize = 0x200;
}

/// HSPI Control Register bits
/// Reference: RM0456 Section 30.4.1
pub mod cr_bits {
    /// HSPI enable
    pub const EN: u32 = 1 << 0;
    /// HyperBus enable
    pub const HBEN: u32 = 1 << 1;
    /// DMA enable
    pub const DMAEN: u32 = 1 << 2;
    /// Timeout enable
    pub const TOEN: u32 = 1 << 3;
    /// Memory-mapped mode enable
    pub const MMEN: u32 = 1 << 4;
    /// Alternate bytes bytes order
    pub const ABORT: u32 = 1 << 5;
}

/// HSPI Status Register bits
/// Reference: RM0456 Section 30.4.6
pub mod sr_bits {
    /// Transfer in progress
    pub const TIP: u32 = 1 << 0;
    /// HyperBus busy
    pub const HBBY: u32 = 1 << 1;
    /// Status flag
    pub const STATUS: u32 = 1 << 2;
    /// Timeout flag
    pub const TOF: u32 = 1 << 3;
    /// Transfer error flag
    pub const TEF: u32 = 1 << 4;
    /// Transfer complete flag
    pub const TCF: u32 = 1 << 5;
    /// Match flag
    pub const MF: u32 = 1 << 6;
}

/// HSPI Device Configuration Register 1 bits
/// Reference: RM0456 Section 30.4.2
pub mod dcr1_bits {
    /// Device type
    pub const DTYP: u32 = 0b1111 << 0;
    /// Memory size
    pub const MSIZE: u32 = 0b11111 << 16;
    /// Clock mode
    pub const CKMODE: u32 = 0b11 << 24;
}

/// HSPI Device Configuration Register 2 bits
/// Reference: RM0456 Section 30.4.3
pub mod dcr2_bits {
    /// Clock prescaler
    pub const PRESCALER: u32 = 0xFF << 0;
    /// Transfer counter
    pub const TC: u32 = 0xFFFF << 16;
}

/// HSPI Communication Configuration Register bits
/// Reference: RM0456 Section 30.4.14
pub mod ccr_bits {
    /// Instruction
    pub const INST: u32 = 0xFF << 0;
    /// Instruction mode
    pub const IMODE: u32 = 0b111 << 8;
    /// Address mode
    pub const ADMODE: u32 = 0b111 << 10;
    /// Address size
    pub const ADSIZE: u32 = 0b11 << 12;
    /// Alternate bytes mode
    pub const ABMODE: u32 = 0b111 << 14;
    /// Alternate bytes size
    pub const ABSIZE: u32 = 0b11 << 16;
    /// Data mode
    pub const DMODE: u32 = 0b111 << 18;
    /// Number of dummy cycles
    pub const DCYC: u32 = 0b11111 << 20;
    /// Functional mode
    pub const FMODE: u32 = 0b11 << 26;
    /// Busy clear
    pub const BUSYCLR: u32 = 1 << 28;
    /// DDR mode
    pub const DDR: u32 = 1 << 29;
    /// Double data rate command
    pub const DDRCMD: u32 = 1 << 30;
}

/// HSPI HyperBus Latency Configuration Register bits
/// Reference: RM0456 Section 30.4.23
pub mod hlcr_bits {
    /// Latency mode
    pub const LM: u32 = 1 << 0;
    /// Latency count
    pub const LC: u32 = 0b1111111 << 1;
    /// Maximum latency
    pub const MAXL: u32 = 0b1111 << 8;
}

/// HSPI mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Indirect write mode
    IndirectWrite = 0,
    /// Indirect read mode
    IndirectRead = 1,
    /// Status polling mode
    StatusPoll = 2,
    /// Memory-mapped mode
    MemoryMapped = 3,
}

/// HSPI data line mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataLines {
    /// Single data line (1-1-1)
    Single = 0,
    /// Dual data lines (1-1-2)
    Dual = 1,
    /// Quad data lines (1-1-4)
    Quad = 2,
    /// Octal data lines (1-1-8)
    Octal = 3,
}

/// HSPI clock mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClockMode {
    /// Clock mode 0: CLK low when idle
    Mode0 = 0,
    /// Clock mode 3: CLK high when idle
    Mode3 = 1,
}

/// HSPI device type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceType {
    /// HyperBus
    HyperBus = 0,
    /// Micron MEM
    MicronMEM = 1,
    /// Macronix OCTAL
    MacronixOCTAL = 2,
    /// Cypress OCTAL
    CypressOCTAL = 3,
    /// Winbond OCTAL
    WinbondOCTAL = 4,
    /// ISSI OCTAL
    ISSIOCTAL = 5,
    /// GigaDevice OCTAL
    GigaDeviceOCTAL = 6,
    /// Reserved
    Reserved = 7,
}

/// HSPI instance
pub struct Hspi {
    base: usize,
}

impl Hspi {
    /// Create HSPI1 instance
    pub const fn hspi1() -> Self {
        Self { base: HSPI1_BASE }
    }

    /// Initialize HSPI
    pub fn init(&self) {
        unsafe {
            // Configure device
            let dcr1 = (self.base + reg::DCR1) as *mut u32;
            let mut val = 0;
            val |= (DeviceType::HyperBus as u32 & 0xF) << 0; // DTYP = HyperBus
            val |= (23 << 16); // MSIZE = 23 (64MB)
            val |= (ClockMode::Mode0 as u32) << 24; // CKMODE = 0
            write_volatile(dcr1, val);

            // Configure clock prescaler
            let dcr2 = (self.base + reg::DCR2) as *mut u32;
            write_volatile(dcr2, 0); // PRESCALER = 0 (div 1)

            // Configure HyperBus latency
            let hlcr = (self.base + reg::HLCR) as *mut u32;
            let mut val = 0;
            val |= 6 << 1; // LC = 6 (48 cycles)
            val |= 6 << 8; // MAXL = 6
            write_volatile(hlcr, val);

            // Enable HSPI
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::EN;
            write_volatile(cr, val);
        }
    }

    /// Enable HyperBus mode
    pub fn enable_hyperbus(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::HBEN;
            write_volatile(cr, val);
        }
    }

    /// Disable HyperBus mode
    pub fn disable_hyperbus(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::HBEN;
            write_volatile(cr, val);
        }
    }

    /// Configure memory-mapped mode for read
    pub fn configure_memory_mapped_read(&self, instruction: u8, latency: u8) {
        unsafe {
            // Configure timing
            let tcr = (self.base + reg::TCR) as *mut u32;
            let mut val = 0;
            val |= (latency & 0x3F) << 0; // DCYC
            write_volatile(tcr, val);

            // Configure read communication
            let ccr = (self.base + reg::CCR) as *mut u32;
            let mut val = 0;
            val |= (instruction as u32) << 0; // INSTRUCTION
            val |= (DataLines::Octal as u32 & 0x7) << 8; // IMODE = Octal
            val |= (DataLines::Octal as u32 & 0x7) << 10; // ADMODE = Octal
            val |= (2 << 12); // ADSIZE = 32-bit
            val |= (DataLines::Octal as u32 & 0x7) << 14; // ABMODE = Octal
            val |= (DataLines::Octal as u32 & 0x7) << 18; // DMODE = Octal
            val |= (1 << 26); // FMODE = Memory-mapped
            write_volatile(ccr, val);
        }
    }

    /// Configure memory-mapped mode for write
    pub fn configure_memory_mapped_write(&self, instruction: u8, latency: u8) {
        unsafe {
            // Configure write timing
            let wtcr = (self.base + reg::WTCR) as *mut u32;
            let mut val = 0;
            val |= (latency & 0x3F) << 0; // DCYC
            write_volatile(wtcr, val);

            // Configure write communication
            let wccr = (self.base + reg::WCCR) as *mut u32;
            let mut val = 0;
            val |= (instruction as u32) << 0; // INSTRUCTION
            val |= (DataLines::Octal as u32 & 0x7) << 8; // IMODE = Octal
            val |= (DataLines::Octal as u32 & 0x7) << 10; // ADMODE = Octal
            val |= (2 << 12); // ADSIZE = 32-bit
            val |= (DataLines::Octal as u32 & 0x7) << 14; // ABMODE = Octal
            val |= (DataLines::Octal as u32 & 0x7) << 18; // DMODE = Octal
            write_volatile(wccr, val);
        }
    }

    /// Enable memory-mapped mode
    pub fn enable_memory_mapped(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::MMEN;
            write_volatile(cr, val);
        }
    }

    /// Disable memory-mapped mode
    pub fn disable_memory_mapped(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::MMEN;
            write_volatile(cr, val);
        }
    }

    /// Enable DMA
    pub fn enable_dma(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::DMAEN;
            write_volatile(cr, val);
        }
    }

    /// Disable DMA
    pub fn disable_dma(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::DMAEN;
            write_volatile(cr, val);
        }
    }

    /// Get status register
    pub fn status(&self) -> u32 {
        unsafe {
            read_volatile((self.base + reg::SR) as *const u32)
        }
    }

    /// Check if transfer is in progress
    pub fn is_busy(&self) -> bool {
        (self.status() & sr_bits::TIP) != 0
    }

    /// Check if HyperBus is busy
    pub fn is_hb_busy(&self) -> bool {
        (self.status() & sr_bits::HBBY) != 0
    }

    /// Check transfer complete flag
    pub fn is_transfer_complete(&self) -> bool {
        (self.status() & sr_bits::TCF) != 0
    }

    /// Check transfer error flag
    pub fn has_transfer_error(&self) -> bool {
        (self.status() & sr_bits::TEF) != 0
    }

    /// Clear transfer complete flag
    pub fn clear_tcf(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, sr_bits::TCF);
        }
    }

    /// Clear transfer error flag
    pub fn clear_tef(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, sr_bits::TEF);
        }
    }

    /// Clear all flags
    pub fn clear_flags(&self) {
        unsafe {
            let fcr = (self.base + reg::FCR) as *mut u32;
            write_volatile(fcr, sr_bits::TCF | sr_bits::TEF | sr_bits::TOF | sr_bits::MF);
        }
    }

    /// Set data length
    pub fn set_data_length(&self, length: u32) {
        unsafe {
            let dlr = (self.base + reg::DLR) as *mut u32;
            write_volatile(dlr, length);
        }
    }

    /// Set address
    pub fn set_address(&self, address: u32) {
        unsafe {
            let ar = (self.base + reg::AR) as *mut u32;
            write_volatile(ar, address);
        }
    }

    /// Write data
    pub fn write_data(&self, data: u32) {
        unsafe {
            let dr = (self.base + reg::DR) as *mut u32;
            write_volatile(dr, data);
        }
    }

    /// Read data
    pub fn read_data(&self) -> u32 {
        unsafe {
            read_volatile((self.base + reg::DR) as *const u32)
        }
    }

    /// Set clock prescaler
    pub fn set_prescaler(&self, prescaler: u8) {
        unsafe {
            let dcr2 = (self.base + reg::DCR2) as *mut u32;
            write_volatile(dcr2, (prescaler & 0xFF) as u32);
        }
    }

    /// Configure polling match
    pub fn configure_polling(&self, mask: u32, match_val: u32) {
        unsafe {
            let psmkr = (self.base + reg::PSMKR) as *mut u32;
            write_volatile(psmkr, mask);

            let psmar = (self.base + reg::PSMAR) as *mut u32;
            write_volatile(psmar, match_val);
        }
    }

    /// Set polling interval
    pub fn set_polling_interval(&self, interval: u16) {
        unsafe {
            let pir = (self.base + reg::PIR) as *mut u32;
            write_volatile(pir, interval as u32);
        }
    }

    /// Enable timeout
    pub fn enable_timeout(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::TOEN;
            write_volatile(cr, val);
        }
    }

    /// Set low-power timeout
    pub fn set_low_power_timeout(&self, timeout: u16) {
        unsafe {
            let lptr = (self.base + reg::LPTR) as *mut u32;
            write_volatile(lptr, timeout as u32);
        }
    }

    /// Disable HSPI
    pub fn disable(&self) {
        unsafe {
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::EN;
            write_volatile(cr, val);
        }
    }
}
