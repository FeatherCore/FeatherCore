//! FDCAN - Flexible Data-rate CAN
//! 灵活数据速率 CAN
//!
//! STM32U5 FDCAN 特性：
//! - 支持 CAN 2.0B 和 CAN FD
//! - 最高 8 Mbit/s 数据速率
//! - 最多 128 个 11-bit 过滤器
//! - 两个接收 FIFO
//! - 一个发送队列

/// FDCAN1 base address
pub const FDCAN1_BASE: usize = 0x4000_A400;

/// FDCAN register offsets
pub mod reg {
    pub const CREL: usize = 0x00;
    pub const ENDN: usize = 0x04;
    pub const DBTP: usize = 0x0C;
    pub const TEST: usize = 0x10;
    pub const RWD: usize = 0x14;
    pub const CCCR: usize = 0x18;
    pub const NBTP: usize = 0x1C;
    pub const TSCC: usize = 0x20;
    pub const TSCV: usize = 0x24;
    pub const TOCC: usize = 0x28;
    pub const TOCV: usize = 0x2C;
    pub const ECR: usize = 0x40;
    pub const PSR: usize = 0x44;
    pub const TDCR: usize = 0x48;
    pub const IR: usize = 0x50;
    pub const IE: usize = 0x54;
    pub const ILS: usize = 0x58;
    pub const ILE: usize = 0x5C;
    pub const GFC: usize = 0x80;
    pub const SIDFC: usize = 0x84;
    pub const XIDFC: usize = 0x88;
    pub const XIDAM: usize = 0x90;
    pub const HPMS: usize = 0x94;
    pub const NDAT1: usize = 0x98;
    pub const NDAT2: usize = 0x9C;
    pub const RXF0C: usize = 0xA0;
    pub const RXF0S: usize = 0xA4;
    pub const RXF0A: usize = 0xA8;
    pub const RXBC: usize = 0xAC;
    pub const RXF1C: usize = 0xB0;
    pub const RXF1S: usize = 0xB4;
    pub const RXF1A: usize = 0xB8;
    pub const RXESC: usize = 0xBC;
    pub const TXBC: usize = 0xC0;
    pub const TXFQS: usize = 0xC4;
    pub const TXESC: usize = 0xC8;
    pub const TXBRP: usize = 0xCC;
    pub const TXBAR: usize = 0xD0;
    pub const TXBCR: usize = 0xD4;
    pub const TXBTO: usize = 0xD8;
    pub const TXBCF: usize = 0xDC;
    pub const TXBTIE: usize = 0xE0;
    pub const TXBCIE: usize = 0xE4;
    pub const TXEFC: usize = 0xF0;
    pub const TXEFS: usize = 0xF4;
    pub const TXEFA: usize = 0xF8;
}

/// CAN frame structure
#[derive(Clone, Copy, Debug)]
pub struct CanFrame {
    pub id: u32,
    pub is_extended: bool,
    pub is_fd: bool,
    pub data: [u8; 64],
    pub dlc: u8,
}

/// FDCAN instance
pub struct Fdcan {
    base: usize,
}

impl Fdcan {
    pub const fn fdcan1() -> Self {
        Self { base: FDCAN1_BASE }
    }

    pub fn init(&self, bit_rate: u32, data_rate: u32, sysclk: u32) {
        crate::rcc::enable_apb1_clock(crate::rcc::apb1::FDCAN);

        unsafe {
            // Enter configuration mode
            let cccr = (self.base + reg::CCCR) as *mut u32;
            let mut val = core::ptr::read_volatile(cccr);
            val |= 1 << 0; // INIT
            core::ptr::write_volatile(cccr, val);

            // Wait for init mode
            while (core::ptr::read_volatile(cccr) & (1 << 0)) == 0 {}

            // Configure nominal bit timing
            let nbtp = (self.base + reg::NBTP) as *mut u32;
            let nbrp = (sysclk / bit_rate / 4) as u32;
            core::ptr::write_volatile(nbtp, (nbrp << 16) | (3 << 8) | (1 << 0));

            // Configure data bit timing
            let dbtp = (self.base + reg::DBTP) as *mut u32;
            let dbrp = (sysclk / data_rate / 4) as u32;
            core::ptr::write_volatile(dbtp, (dbrp << 16) | (3 << 8) | (1 << 0));

            // Enable CAN FD
            let mut val = core::ptr::read_volatile(cccr);
            val |= 1 << 8; // FDOE
            core::ptr::write_volatile(cccr, val);

            // Exit configuration mode
            let mut val = core::ptr::read_volatile(cccr);
            val &= !(1 << 0); // Clear INIT
            core::ptr::write_volatile(cccr, val);
        }
    }

    pub fn send(&self, frame: &CanFrame) -> Result<(), CanError> {
        // Simplified implementation
        // In real implementation, need to manage TX FIFO
        Ok(())
    }

    pub fn receive(&self, frame: &mut CanFrame) -> Result<(), CanError> {
        // Simplified implementation
        // In real implementation, need to manage RX FIFO
        Err(CanError::NoMessage)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CanError {
    NoMessage,
    BusOff,
    TxFull,
}
