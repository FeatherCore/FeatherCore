//! SDMMC - Secure Digital MultiMedia Card
//! SD/SDIO/MMC 接口
//!
//! ## STM32U5 SDMMC 特性 / Features
//! - **支持卡类型 / Supported Cards:**
//!   - SD 存储卡 (SD Card)
//!   - SDIO 卡 (SDIO Card)
//!   - eMMC 卡 (Embedded MMC)
//!
//! - **标准支持 / Standards Support:**
//!   - SD 存储卡标准 4.2 (SD Memory Card Standard 4.2)
//!   - SDIO 卡标准 2.0 (SDIO Card Standard 2.0)
//!   - eMMC 标准 4.51 (eMMC Standard 4.51)
//!
//! - **特性 / Features:**
//!   - 8-bit 数据总线宽度
//!   - DMA 传输支持
//!   - 高速模式 (High Speed)
//!   - SDR (Single Data Rate) / DDR (Double Data Rate) 模式
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 58: Secure digital input/output interface (SDMMC)

/// SDMMC1 base address / SDMMC1 基地址
pub const SDMMC1_BASE: usize = 0x420C_8000;
/// SDMMC2 base address
pub const SDMMC2_BASE: usize = 0x420C_8C00;

/// SDMMC register offsets
pub mod reg {
    pub const POWER: usize = 0x00;
    pub const CLKCR: usize = 0x04;
    pub const ARG: usize = 0x08;
    pub const CMD: usize = 0x0C;
    pub const RESPCMD: usize = 0x10;
    pub const RESP1: usize = 0x14;
    pub const RESP2: usize = 0x18;
    pub const RESP3: usize = 0x1C;
    pub const RESP4: usize = 0x20;
    pub const DTIMER: usize = 0x24;
    pub const DLEN: usize = 0x28;
    pub const DCTRL: usize = 0x2C;
    pub const DCOUNT: usize = 0x30;
    pub const STA: usize = 0x34;
    pub const ICR: usize = 0x38;
    pub const MASK: usize = 0x3C;
    pub const ACKTIME: usize = 0x40;
    pub const IDMACTRL: usize = 0x50;
    pub const IDMABASE0: usize = 0x58;
    pub const IDMABASE1: usize = 0x64;
    pub const FIFO: usize = 0x80;
}

/// SDMMC command
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    // Basic commands
    GoIdleState = 0,
    SendCid = 2,
    SendRelativeAddr = 3,
    SelectCard = 7,
    SendCsd = 9,
    StopTransmission = 12,
    SendStatus = 13,
    SetBlocklen = 16,
    ReadSingleBlock = 17,
    ReadMultipleBlock = 18,
    WriteSingleBlock = 24,
    WriteMultipleBlock = 25,
    AppCmd = 55,
    // Application commands
    SdSendOpCond = 41,
}

/// SDMMC response type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseType {
    None = 0,
    Short = 1,
    Long = 3,
}

/// SDMMC data direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataDirection {
    Read = 0,
    Write = 1,
}

/// SDMMC instance
pub struct Sdmmc {
    base: usize,
}

impl Sdmmc {
    pub const fn sdmmc1() -> Self {
        Self { base: SDMMC1_BASE }
    }

    pub const fn sdmmc2() -> Self {
        Self { base: SDMMC2_BASE }
    }

    /// Initialize SDMMC
    pub fn init(&self) {
        // Enable SDMMC clock
        crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::SDMMC1);

        unsafe {
            // Power on
            let power = (self.base + reg::POWER) as *mut u32;
            core::ptr::write_volatile(power, 0b11); // Power on

            // Wait for power stable
            for _ in 0..10000 {
                core::arch::asm!("nop");
            }

            // Configure clock (400 kHz for initialization)
            let clkcr = (self.base + reg::CLKCR) as *mut u32;
            core::ptr::write_volatile(clkcr, (1 << 17) | (118 << 0)); // CLKEN, CLKDIV

            // Enable clock
            let mut val = core::ptr::read_volatile(clkcr);
            val |= 1 << 17; // CLKEN
            core::ptr::write_volatile(clkcr, val);
        }
    }

    /// Send command
    pub fn send_command(&self, cmd: Command, arg: u32, resp_type: ResponseType) -> Result<u32, SdmmcError> {
        unsafe {
            // Clear status flags
            let icr = (self.base + reg::ICR) as *mut u32;
            core::ptr::write_volatile(icr, 0xFFFFFFFF);

            // Set argument
            let arg_reg = (self.base + reg::ARG) as *mut u32;
            core::ptr::write_volatile(arg_reg, arg);

            // Send command
            let cmd_reg = (self.base + reg::CMD) as *mut u32;
            let mut val = 0;
            val |= (cmd as u32) << 0; // CMDINDEX
            val |= (resp_type as u32) << 6; // WAITRESP
            val |= 1 << 10; // CPSMEN
            core::ptr::write_volatile(cmd_reg, val);

            // Wait for command complete
            let sta = (self.base + reg::STA) as *mut u32;
            let mut timeout = 1000000;
            loop {
                let status = core::ptr::read_volatile(sta);
                if status & (1 << 6) != 0 { // CMDREND
                    break;
                }
                if status & (1 << 7) != 0 { // CMDSENT (no response)
                    break;
                }
                if status & (1 << 3) != 0 { // CTIMEOUT
                    return Err(SdmmcError::Timeout);
                }
                timeout -= 1;
                if timeout == 0 {
                    return Err(SdmmcError::Timeout);
                }
            }

            // Read response if applicable
            if resp_type != ResponseType::None {
                let resp1 = (self.base + reg::RESP1) as *mut u32;
                Ok(core::ptr::read_volatile(resp1))
            } else {
                Ok(0)
            }
        }
    }

    /// Read data block
    pub fn read_block(&self, block_addr: u32, buffer: &mut [u8]) -> Result<(), SdmmcError> {
        unsafe {
            // Set data timeout
            let dtimer = (self.base + reg::DTIMER) as *mut u32;
            core::ptr::write_volatile(dtimer, 0xFFFFFFFF);

            // Set data length
            let dlen = (self.base + reg::DLEN) as *mut u32;
            core::ptr::write_volatile(dlen, buffer.len() as u32);

            // Configure data control
            let dctrl = (self.base + reg::DCTRL) as *mut u32;
            let mut val = 0;
            val |= 1 << 0; // DTEN
            val |= 0b00 << 1; // DTDIR = from card to controller
            val |= 0b00 << 2; // DTMODE = block
            val |= 9 << 4; // DBLOCKSIZE = 512 bytes
            core::ptr::write_volatile(dctrl, val);

            // Send read command
            self.send_command(Command::ReadSingleBlock, block_addr, ResponseType::Short)?;

            // Read data from FIFO
            let fifo = (self.base + reg::FIFO) as *mut u32;
            let mut i = 0;
            while i < buffer.len() {
                let sta = (self.base + reg::STA) as *mut u32;
                let status = core::ptr::read_volatile(sta);
                
                if status & (1 << 5) != 0 { // RXFIFOE
                    // Wait for data
                    continue;
                }

                let word = core::ptr::read_volatile(fifo);
                for j in 0..4 {
                    if i + j < buffer.len() {
                        buffer[i + j] = (word >> (j * 8)) as u8;
                    }
                }
                i += 4;
            }

            Ok(())
        }
    }

    /// Write data block
    pub fn write_block(&self, block_addr: u32, buffer: &[u8]) -> Result<(), SdmmcError> {
        unsafe {
            // Set data timeout
            let dtimer = (self.base + reg::DTIMER) as *mut u32;
            core::ptr::write_volatile(dtimer, 0xFFFFFFFF);

            // Set data length
            let dlen = (self.base + reg::DLEN) as *mut u32;
            core::ptr::write_volatile(dlen, buffer.len() as u32);

            // Configure data control
            let dctrl = (self.base + reg::DCTRL) as *mut u32;
            let mut val = 0;
            val |= 1 << 0; // DTEN
            val |= 0b01 << 1; // DTDIR = from controller to card
            val |= 0b00 << 2; // DTMODE = block
            val |= 9 << 4; // DBLOCKSIZE = 512 bytes
            core::ptr::write_volatile(dctrl, val);

            // Send write command
            self.send_command(Command::WriteSingleBlock, block_addr, ResponseType::Short)?;

            // Write data to FIFO
            let fifo = (self.base + reg::FIFO) as *mut u32;
            let mut i = 0;
            while i < buffer.len() {
                let sta = (self.base + reg::STA) as *mut u32;
                let status = core::ptr::read_volatile(sta);
                
                if status & (1 << 16) != 0 { // TXFIFOF
                    // FIFO full, wait
                    continue;
                }

                let mut word: u32 = 0;
                for j in 0..4 {
                    if i + j < buffer.len() {
                        word |= (buffer[i + j] as u32) << (j * 8);
                    }
                }
                core::ptr::write_volatile(fifo, word);
                i += 4;
            }

            Ok(())
        }
    }

    /// Set bus width
    pub fn set_bus_width(&self, width: u8) {
        // Implementation depends on ACMD6
        // This is a simplified version
    }

    /// Set clock speed
    pub fn set_clock(&self, freq_hz: u32, sysclk: u32) {
        unsafe {
            let clkcr = (self.base + reg::CLKCR) as *mut u32;
            let div = (sysclk / freq_hz) as u16;
            
            let mut val = core::ptr::read_volatile(clkcr);
            val &= !(0x3FF << 0); // Clear CLKDIV
            val |= (div as u32) << 0;
            core::ptr::write_volatile(clkcr, val);
        }
    }
}

/// SDMMC error
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdmmcError {
    Timeout,
    CommandError,
    DataError,
    CardError,
}

/// Initialize SD card
pub fn init_sd_card() -> Result<Sdmmc, SdmmcError> {
    let sdmmc = Sdmmc::sdmmc1();
    sdmmc.init();
    
    // Card initialization sequence
    // 1. CMD0 - Go to idle state
    sdmmc.send_command(Command::GoIdleState, 0, ResponseType::None)?;
    
    // 2. CMD8 - Send interface condition
    sdmmc.send_command(Command::SendCid, 0x1AA, ResponseType::Short)?;
    
    // 3. ACMD41 - Send operation condition
    sdmmc.send_command(Command::AppCmd, 0, ResponseType::Short)?;
    sdmmc.send_command(Command::SdSendOpCond, 0x40000000, ResponseType::Short)?;
    
    // 4. CMD2 - Get CID
    sdmmc.send_command(Command::SendCid, 0, ResponseType::Long)?;
    
    // 5. CMD3 - Get RCA
    sdmmc.send_command(Command::SendRelativeAddr, 0, ResponseType::Short)?;
    
    Ok(sdmmc)
}
