//! SAI - Serial Audio Interface
//! 串行音频接口
//!
//! STM32U5 SAI 特性：
//! - 两个独立的音频子模块（A 和 B）
//! - 支持 I2S、LSB/MSB 对齐、PCM/DSP、TDM
//! - 支持 SPDIF 输出
//! - 支持 AC'97
//! - 8 字 FIFO
//! - 支持 DMA

/// SAI1 base address
pub const SAI1_BASE: usize = 0x4001_5400;
/// SAI2 base address
pub const SAI2_BASE: usize = 0x4001_5800;

/// SAI block registers (per block A/B)
pub mod block_reg {
    pub const SAI_CR1: usize = 0x00;
    pub const SAI_CR2: usize = 0x04;
    pub const SAI_FRCR: usize = 0x08;
    pub const SAI_SLOTR: usize = 0x0C;
    pub const SAI_IMR: usize = 0x10;
    pub const SAI_SR: usize = 0x14;
    pub const SAI_CLRFR: usize = 0x18;
    pub const SAI_DR: usize = 0x1C;
}

/// SAI block offset
pub const SAI_BLOCK_A_OFFSET: usize = 0x00;
pub const SAI_BLOCK_B_OFFSET: usize = 0x20;

/// SAI mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SaiMode {
    /// Master transmitter
    MasterTx = 0b00,
    /// Master receiver
    MasterRx = 0b01,
    /// Slave transmitter
    SlaveTx = 0b10,
    /// Slave receiver
    SlaveRx = 0b11,
}

/// SAI protocol
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Protocol {
    /// Free protocol
    Free = 0b00,
    /// SPDIF
    Spdif = 0b01,
    /// AC'97
    Ac97 = 0b10,
}

/// SAI data size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataSize {
    Bits8 = 0b010,
    Bits10 = 0b011,
    Bits16 = 0b100,
    Bits20 = 0b101,
    Bits24 = 0b110,
    Bits32 = 0b111,
}

/// SAI configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub mode: SaiMode,
    pub protocol: Protocol,
    pub data_size: DataSize,
    pub sample_rate: u32,
    pub mono: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: SaiMode::MasterTx,
            protocol: Protocol::Free,
            data_size: DataSize::Bits16,
            sample_rate: 48000,
            mono: false,
        }
    }
}

/// SAI block
pub struct SaiBlock {
    base: usize,
    block_offset: usize,
}

impl SaiBlock {
    fn block_base(&self) -> usize {
        self.base + self.block_offset
    }

    /// Initialize SAI block
    pub fn init(&self, config: &Config, mclk_freq: u32) {
        unsafe {
            // Disable block
            let cr1 = (self.block_base() + block_reg::SAI_CR1) as *mut u32;
            core::ptr::write_volatile(cr1, 0);

            // Configure CR1
            let mut val = 0;
            val |= (config.mode as u32) << 0;
            val |= (config.protocol as u32) << 2;
            val |= (config.data_size as u32) << 5;
            if config.mono {
                val |= 1 << 12;
            }
            core::ptr::write_volatile(cr1, val);

            // Configure frame (I2S standard: 32-bit frame for 16-bit data)
            let frcr = (self.block_base() + block_reg::SAI_FRCR) as *mut u32;
            let mut val = 0;
            val |= 31 << 0;  // FRL = 32 - 1
            val |= 15 << 8;  // FSALL = 16 - 1
            val |= 1 << 16;  // FSOFF
            val |= 1 << 17;  // FSPOL
            core::ptr::write_volatile(frcr, val);

            // Configure slot
            let slotr = (self.block_base() + block_reg::SAI_SLOTR) as *mut u32;
            let mut val = 0;
            val |= 0b11 << 6;  // SLOTSZ = 16-bit
            val |= 1 << 0;     // NBSLOT = 1 slot
            val |= 0b0001 << 16; // SLOTEN = slot 0 enabled
            core::ptr::write_volatile(slotr, val);

            // Enable block
            let cr1 = (self.block_base() + block_reg::SAI_CR1) as *mut u32;
            let mut val = core::ptr::read_volatile(cr1);
            val |= 1 << 16; // SAIEN
            core::ptr::write_volatile(cr1, val);
        }
    }

    /// Write data
    pub fn write(&self, data: u32) {
        unsafe {
            let dr = (self.block_base() + block_reg::SAI_DR) as *mut u32;
            core::ptr::write_volatile(dr, data);
        }
    }

    /// Read data
    pub fn read(&self) -> u32 {
        unsafe {
            let dr = (self.block_base() + block_reg::SAI_DR) as *mut u32;
            core::ptr::read_volatile(dr)
        }
    }

    /// Check if FIFO is empty
    pub fn is_fifo_empty(&self) -> bool {
        unsafe {
            let sr = (self.block_base() + block_reg::SAI_SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 4)) != 0
        }
    }

    /// Check if FIFO is full
    pub fn is_fifo_full(&self) -> bool {
        unsafe {
            let sr = (self.block_base() + block_reg::SAI_SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 5)) != 0
        }
    }
}

/// SAI instance
pub struct Sai {
    base: usize,
}

impl Sai {
    pub const fn sai1() -> Self {
        Self { base: SAI1_BASE }
    }

    pub const fn sai2() -> Self {
        Self { base: SAI2_BASE }
    }

    /// Get block A
    pub fn block_a(&self) -> SaiBlock {
        SaiBlock {
            base: self.base,
            block_offset: SAI_BLOCK_A_OFFSET,
        }
    }

    /// Get block B
    pub fn block_b(&self) -> SaiBlock {
        SaiBlock {
            base: self.base,
            block_offset: SAI_BLOCK_B_OFFSET,
        }
    }

    /// Initialize SAI
    pub fn init(&self) {
        // Enable SAI clock
        crate::rcc::enable_apb2_clock(crate::rcc::apb2::SAI1);
    }
}

/// Initialize SAI for I2S audio output
pub fn init_sai_i2s_output() {
    let sai = Sai::sai1();
    sai.init();

    // Configure block A as master transmitter
    let block_a = sai.block_a();
    let config = Config {
        mode: SaiMode::MasterTx,
        protocol: Protocol::Free,
        data_size: DataSize::Bits16,
        sample_rate: 48000,
        mono: false,
    };
    block_a.init(&config, 12288000);
}
