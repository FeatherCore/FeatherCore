//! OctoSPI - Octal Serial Peripheral Interface
//! 八线串行外设接口
//!
//! ## STM32U5 OctoSPI 特性 / Features
//! - **数据线模式 / Data Line Modes:**
//!   - 单线 SPI (Single SPI)
//!   - 双线 SPI (Dual SPI)
//!   - 四线 SPI (Quad SPI)
//!   - 八线 SPI (Octal SPI)
//!
//! - **支持的设备 / Supported Devices:**
//!   - NOR Flash
//!   - PSRAM
//!   - HyperBus 设备
//!
//! - **特性 / Features:**
//!   - 内存映射模式 (Memory-mapped mode)
//!   - XIP (Execute In Place) 支持
//!   - 最高 160 MB/s 吞吐量
//!   - DMA 支持
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 28: OctoSPI interface (OCTOSPI)

/// OCTOSPI1 base address / OCTOSPI1 基地址
pub const OCTOSPI1_BASE: usize = 0x420C_0000;
/// OctoSPI2 base address
pub const OCTOSPI2_BASE: usize = 0x420C_0400;

/// OctoSPI register offsets
pub mod reg {
    pub const CR: usize = 0x00;
    pub const DCR1: usize = 0x08;
    pub const DCR2: usize = 0x0C;
    pub const DCR3: usize = 0x10;
    pub const DCR4: usize = 0x14;
    pub const SR: usize = 0x20;
    pub const FCR: usize = 0x24;
    pub const DLR: usize = 0x40;
    pub const AR: usize = 0x48;
    pub const DR: usize = 0x50;
    pub const PSMKR: usize = 0x80;
    pub const PSMAR: usize = 0x88;
    pub const PIR: usize = 0x90;
    pub const CCR: usize = 0x100;
    pub const TCR: usize = 0x108;
    pub const IR: usize = 0x110;
    pub const ABR: usize = 0x118;
    pub const LPTR: usize = 0x120;
    pub const WCCR: usize = 0x140;
    pub const WTCR: usize = 0x148;
    pub const WIR: usize = 0x150;
    pub const WABR: usize = 0x158;
    pub const HLCR: usize = 0x200;
}

/// OctoSPI mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Single = 0,
    Dual = 1,
    Quad = 2,
    Octo = 3,
}

/// OctoSPI instruction mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionMode {
    None = 0,
    Single = 1,
    Dual = 2,
    Quad = 3,
    Octo = 4,
}

/// OctoSPI address size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressSize {
    Bits8 = 0,
    Bits16 = 1,
    Bits24 = 2,
    Bits32 = 3,
}

/// OctoSPI instance
pub struct Octospi {
    base: usize,
}

impl Octospi {
    pub const fn octospi1() -> Self {
        Self { base: OCTOSPI1_BASE }
    }

    pub const fn octospi2() -> Self {
        Self { base: OCTOSPI2_BASE }
    }

    /// Initialize OctoSPI
    pub fn init(&self) {
        // Enable OctoSPI clock
        crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::OCTOSPI1);

        unsafe {
            // Configure device
            let dcr1 = (self.base + reg::DCR1) as *mut u32;
            let mut val = 0;
            val |= (25 << 16); // MTYP = 25 (Macronix Flash)
            val |= (2 << 8);   // FSIZE = 2 (4MB)
            val |= (3 << 0);   // CKMODE = 3
            core::ptr::write_volatile(dcr1, val);

            // Configure clock
            let dcr2 = (self.base + reg::DCR2) as *mut u32;
            core::ptr::write_volatile(dcr2, 1 << 0); // PRESCALER = 1

            // Configure timing
            let dcr3 = (self.base + reg::DCR3) as *mut u32;
            core::ptr::write_volatile(dcr3, (1 << 16) | (1 << 8) | 1);

            // Enable OctoSPI
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // EN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Configure for memory-mapped mode
    pub fn enable_memory_mapped(&self) {
        unsafe {
            // Configure read operation
            let ccr = (self.base + reg::CCR) as *mut u32;
            let mut val = 0;
            val |= (0xEB << 0);  // INSTRUCTION = Fast Read Octo
            val |= (4 << 8);     // IMODE = Octo
            val |= (4 << 10);    // ADMODE = Octo
            val |= (2 << 12);    // ADSIZE = 24-bit
            val |= (4 << 14);    // ABMODE = Octo
            val |= (1 << 16);    // ABSIZE = 8-bit
            val |= (4 << 18);    // DMODE = Octo
            val |= (8 << 20);    // DCYC = 8 dummy cycles
            val |= (1 << 22);    // FMODE = Memory-mapped
            core::ptr::write_volatile(ccr, val);

            // Enable memory-mapped mode
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 2; // FMODE = 1
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Read data in indirect mode
    pub fn read(&self, addr: u32, buffer: &mut [u8]) {
        unsafe {
            // Set address
            let ar = (self.base + reg::AR) as *mut u32;
            core::ptr::write_volatile(ar, addr);

            // Set data length
            let dlr = (self.base + reg::DLR) as *mut u32;
            core::ptr::write_volatile(dlr, (buffer.len() - 1) as u32);

            // Configure for read
            let ccr = (self.base + reg::CCR) as *mut u32;
            let mut val = 0;
            val |= (0x03 << 0);  // INSTRUCTION = Read
            val |= (1 << 8);     // IMODE = Single
            val |= (1 << 10);    // ADMODE = Single
            val |= (2 << 12);    // ADSIZE = 24-bit
            val |= (1 << 18);    // DMODE = Single
            val |= (0 << 22);    // FMODE = Indirect read
            core::ptr::write_volatile(ccr, val);

            // Start transfer
            let cr = (self.base + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // EN
            core::ptr::write_volatile(cr, val);

            // Read data
            let dr = (self.base + reg::DR) as *mut u8;
            for i in 0..buffer.len() {
                // Wait for data
                loop {
                    let sr = (self.base + reg::SR) as *mut u32;
                    let status = core::ptr::read_volatile(sr);
                    if status & (1 << 1) != 0 { // FTF
                        break;
                    }
                }
                buffer[i] = core::ptr::read_volatile(dr);
            }
        }
    }

    /// Write data in indirect mode
    pub fn write(&self, addr: u32, buffer: &[u8]) {
        unsafe {
            // Set address
            let ar = (self.base + reg::AR) as *mut u32;
            core::ptr::write_volatile(ar, addr);

            // Set data length
            let dlr = (self.base + reg::DLR) as *mut u32;
            core::ptr::write_volatile(dlr, (buffer.len() - 1) as u32);

            // Configure for write
            let ccr = (self.base + reg::CCR) as *mut u32;
            let mut val = 0;
            val |= (0x02 << 0);  // INSTRUCTION = Page Program
            val |= (1 << 8);     // IMODE = Single
            val |= (1 << 10);    // ADMODE = Single
            val |= (2 << 12);    // ADSIZE = 24-bit
            val |= (1 << 18);    // DMODE = Single
            val |= (1 << 26);    // FMODE = Indirect write
            core::ptr::write_volatile(ccr, val);

            // Write data
            let dr = (self.base + reg::DR) as *mut u8;
            for i in 0..buffer.len() {
                // Wait for FIFO not full
                loop {
                    let sr = (self.base + reg::SR) as *mut u32;
                    let status = core::ptr::read_volatile(sr);
                    if status & (1 << 2) == 0 { // !FTF
                        break;
                    }
                }
                core::ptr::write_volatile(dr, buffer[i]);
            }
        }
    }

    /// Send command (for flash operations)
    pub fn send_command(&self, cmd: u8) {
        unsafe {
            let ccr = (self.base + reg::CCR) as *mut u32;
            let mut val = 0;
            val |= (cmd as u32) << 0; // INSTRUCTION
            val |= (1 << 8);          // IMODE = Single
            val |= (3 << 26);         // FMODE = Automatic polling
            core::ptr::write_volatile(ccr, val);
        }
    }

    /// Wait for busy flag
    pub fn wait_busy(&self) {
        unsafe {
            loop {
                let sr = (self.base + reg::SR) as *mut u32;
                let status = core::ptr::read_volatile(sr);
                if status & (1 << 5) == 0 { // !BUSY
                    break;
                }
            }
        }
    }
}

/// Initialize OctoSPI for NOR Flash
pub fn init_octospi_flash() {
    let octospi = Octospi::octospi1();
    octospi.init();
    
    // Reset flash
    octospi.send_command(0x66); // Enable Reset
    octospi.send_command(0x99); // Reset
    
    // Wait for reset complete
    octospi.wait_busy();
}

/// Enable XIP (Execute In Place)
pub fn enable_xip() {
    let octospi = Octospi::octospi1();
    octospi.enable_memory_mapped();
}
