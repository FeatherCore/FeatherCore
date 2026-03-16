//! I3C - Improved Inter-Integrated Circuit
//! 改进型集成电路总线
//!
//! STM32U5 I3C 特性:
//! - I3C 主设备模式
//! - 支持 I2C 兼容模式
//! - 支持高速模式 (High Speed): 8 Mbps
//! - 支持带内中断 (In-Band Interrupt)
//! - 支持主设备请求 (Master Request)
//! - DMA 支持

/// I3C1 base address
pub const I3C1_BASE: usize = 0x4000_6000;

/// I3C register offsets
pub mod reg {
    /// Control register
    pub const CR: usize = 0x00;
    /// Status register
    pub const SR: usize = 0x04;
    /// Interrupt enable register
    pub const IER: usize = 0x08;
    /// Interrupt status register
    pub const ISR: usize = 0x0C;
    /// Interrupt clear register
    pub const ICR: usize = 0x10;
    /// Timing register 1
    pub const TIMINGR1: usize = 0x14;
    /// Timing register 2
    pub const TIMINGR2: usize = 0x18;
    /// Timing register 3
    pub const TIMINGR3: usize = 0x1C;
    /// Address and command register
    pub const_ADDR_CMD: usize = 0x20;
    /// Data receive register
    pub const RXDR: usize = 0x24;
    /// Data transmit register
    pub const TXDR: usize = 0x28;
    /// Control clear register
    pub const CRCLR: usize = 0x2C;
    /// Timing register 4
    pub const TIMINGR4: usize = 0x30;
    /// Timing register 5
    pub const TIMINGR5: usize = 0x34;
}

/// I3C speed modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpeedMode {
    /// Standard mode (I2C compatible, 100 kHz)
    Standard = 0,
    /// Fast mode (I2C compatible, 400 kHz)
    Fast = 1,
    /// Fast mode plus (I2C compatible, 1 MHz)
    FastPlus = 2,
    /// High Speed mode (I3C, 8 Mbps)
    HighSpeed = 3,
}

/// I3C command type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandType {
    /// Address + Write
    Write = 0,
    /// Address + Read
    Read = 1,
    /// I3C CCC (Common Command Code)
    CCC = 2,
}

/// I3C CCC commands
pub mod ccc {
    /// Broadcast ENTDAA (Enter DAA - Dynamic Address Assignment)
    pub const ENTDAA: u8 = 0x00;
    /// Broadcast SETDASA (Set Dynamic Address from Static)
    pub const SETDASA: u8 = 0x01;
    /// Broadcast SETNEWDA (Set New Dynamic Address)
    pub const SETNEWDA: u8 = 0x08;
    /// Broadcast GETPID (Get Provisional ID)
    pub const GETPID: u8 = 0x0D;
    /// Broadcast GETBCR (Get Bus Characteristic)
    pub const GETBCR: u8 = 0x0E;
    /// Broadcast GETDC (Get Device Characteristic)
    pub const GETDC: u8 = 0x0F;
    /// Broadcast GETMWL (Get Max Write Length)
    pub const GETMWL: u8 = 0x10;
    /// Broadcast GETMRL (Get Max Read Length)
    pub const GETMRL: u8 = 0x11;
    /// Broadcast SETXTIME (Set Exchange Timing)
    pub const SETXTIME: u8 = 0x28;
    /// Broadcast GETXTIME (Get Exchange Timing)
    pub const GETXTIME: u8 = 0x29;
    /// Broadcast SETAASA (Set All Devices to Address)
    pub const SETAASA: u8 = 0x2D;
    /// Broadcast SETBUSCON (Set Bus Configuration)
    pub const SETBUSCON: u8 = 0x2E;
}

/// I3C instance
pub struct I3c {
    base: usize,
}

impl I3c {
    /// Create I3C1 instance
    pub const fn i3c1() -> Self {
        Self { base: I3C1_BASE }
    }

    /// Initialize I3C with timing configuration
    pub fn init(&self, speed: SpeedMode, pclk_freq: u32) {
        unsafe {
            // Enable I3C1 clock
            let rcc_base = crate::rcc::RCC_BASE as *mut u32;
            let apb1enr2 = rcc_base.add(0x34 / 4);
            *apb1enr2 |= 1 << 2; // I3C1EN

            // Disable I3C
            let cr = (self.base + reg::CR) as *mut u32;
            core::ptr::write_volatile(cr, 0);

            // Configure timing based on speed mode
            let (presc, scl_low, scl_high, sda_dly) = match speed {
                SpeedMode::Standard => (15, 50, 50, 20),
                SpeedMode::Fast => (7, 25, 25, 10),
                SpeedMode::FastPlus => (3, 12, 12, 5),
                SpeedMode::HighSpeed => (0, 4, 4, 2),
            };

            // Timing register 1
            let timingr1 = (self.base + reg::TIMINGR1) as *mut u32;
            let mut val = 0;
            val |= (presc as u32) << 28; // PRESC[3:0]
            val |= (scl_low as u32) << 20; // SCLLI[7:0]
            val |= (sda_dly as u32) << 12; // SDADLY[3:0]
            val |= (scl_high as u32) << 4; // SCLHI[7:0]
            core::ptr::write_volatile(timingr1, val);

            // Enable I3C
            let cr = (self.base + reg::CR) as *mut u32;
            core::ptr::write_volatile(cr, 1 << 0); // ENABLE
        }
    }

    /// Send CCC (Common Command Code)
    pub fn send_ccc(&self, ccc_cmd: u8, data: Option<&[u8]>) {
        unsafe {
            // Clear status
            let sr = (self.base + reg::SR) as *mut u32;
            let _ = core::ptr::read_volatile(sr);

            // Configure command
            let addr_cmd = (self.base + reg::ADDR_CMD) as *mut u32;
            let mut val = 0;
            val |= (ccc_cmd as u32) << 16; // CMD[7:0]
            val |= 1 << 8; // CCC (Common Command Code)
            if data.is_some() {
                val |= 0 << 14; // Write transfer
            } else {
                val |= 1 << 14; // No data
            }
            core::ptr::write_volatile(addr_cmd, val);

            // Write data if provided
            if let Some(data) = data {
                let txdr = (self.base + reg::TXDR) as *mut u32;
                for &byte in data {
                    while (core::ptr::read_volatile(sr) & (1 << 1)) == 0 {} // TXFNE
                    core::ptr::write_volatile(txdr, byte as u32);
                }
            }

            // Wait for completion
            while (core::ptr::read_volatile(sr) & (1 << 5)) == 0 {} // TXC
        }
    }

    /// Send data to I3C/I2C device
    pub fn write(&self, address: u8, data: &[u8]) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;

            // Configure address and command
            let addr_cmd = (self.base + reg::ADDR_CMD) as *mut u32;
            let mut val = 0;
            val |= ((address as u32) << 1) & 0xFE; // Address[6:0]
            val |= 0 << 14; // Write
            val |= (data.len() as u32) << 16; // DBLB
            core::ptr::write_volatile(addr_cmd, val);

            // Write data
            let txdr = (self.base + reg::TXDR) as *mut u32;
            for &byte in data {
                while (core::ptr::read_volatile(sr) & (1 << 1)) == 0 {} // TXFNE
                core::ptr::write_volatile(txdr, byte as u32);
            }

            // Wait for completion
            while (core::ptr::read_volatile(sr) & (1 << 5)) == 0 {} // TXC
        }
    }

    /// Read data from I3C/I2C device
    pub fn read(&self, address: u8, buffer: &mut [u8]) {
        unsafe {
            let sr = (self.base + reg::SR) as *mut u32;
            let rxdr = (self.base + reg::RXDR) as *mut u32;

            // Configure address and command
            let addr_cmd = (self.base + reg::ADDR_CMD) as *mut u32;
            let mut val = 0;
            val |= ((address as u32) << 1) & 0xFE; // Address[6:0]
            val |= 1 << 14; // Read
            val |= (buffer.len() as u32) << 16; // DBLB
            core::ptr::write_volatile(addr_cmd, val);

            // Read data
            for byte in buffer.iter_mut() {
                while (core::ptr::read_volatile(sr) & (1 << 0)) == 0 {} // RXFNE
                *byte = core::ptr::read_volatile(rxdr) as u8;
            }

            // Wait for completion
            while (core::ptr::read_volatile(sr) & (1 << 4)) == 0 {} // RXC
        }
    }

    /// Enable interrupt
    pub fn enable_interrupt(&self, interrupt: u32) {
        unsafe {
            let ier = (self.base + reg::IER) as *mut u32;
            let val = core::ptr::read_volatile(ier);
            core::ptr::write_volatile(ier, val | interrupt);
        }
    }

    /// Get interrupt status
    pub fn get_isr(&self) -> u32 {
        unsafe {
            let isr = (self.base + reg::ISR) as *mut u32;
            core::ptr::read_volatile(isr)
        }
    }

    /// Clear interrupt
    pub fn clear_interrupt(&self, flags: u32) {
        unsafe {
            let icr = (self.base + reg::ICR) as *mut u32;
            core::ptr::write_volatile(icr, flags);
        }
    }
}

/// Initialize I3C1 with default configuration
pub fn init_i3c1_default(pclk_freq: u32) {
    let i3c = I3c::i3c1();
    i3c.init(SpeedMode::HighSpeed, pclk_freq);
}
