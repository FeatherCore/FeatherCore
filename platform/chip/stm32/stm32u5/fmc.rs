//! FMC - Flexible Memory Controller / 灵活存储控制器
//!
//! ## STM32U5 FMC 特性 / Features
//! - **支持存储器类型 / Supported Memory Types:**
//!   - NOR Flash (异步/同步 / Asynchronous/Synchronous)
//!   - PSRAM (CellularRAM, pseudo-SRAM)
//!   - SRAM (静态随机存储器)
//!   - NAND Flash (8-bit 数据宽度 / 8-bit data width)
//!   - SDRAM (同步动态随机存储器 / Synchronous DRAM)
//!
//! - **数据宽度 / Data Width:**
//!   - NOR/PSRAM/SRAM: **8-bit, 16-bit** (Note: STM32U5 FMC does NOT support 32-bit data width for external memories)
//!   - NAND: 8-bit only
//!   - SDRAM: 8-bit, 16-bit
//!
//! - **地址范围 / Address Ranges:**
//!   - Bank1 (NOR/PSRAM/SRAM): 0x6000_0000 - 0x6FFF_FFFF (256MB per sub-bank)
//!   - Bank2 (NAND): 0x8000_0000 - 0x8FFF_FFFF
//!   - Bank3 (NAND): 0x8000_0000 - 0x8FFF_FFFF (shared)
//!   - SDRAM Bank1: 0xC000_0000 - 0xCFFF_FFFF (256MB)
//!   - SDRAM Bank2: 0xD000_0000 - 0xDFFF_FFFF (256MB)
//!
//! - **时钟 / Clock:**
//!   - HCLK driven (up to 160 MHz on STM32U5)
//!   - Programmable clock divide ratio for synchronous accesses
//!
//! - **特性 / Key Features:**
//!   - 4 independent NOR/PSRAM banks with separate configuration
//!   - 2 NAND banks with ECC hardware (up to 8 bytes ECC per 512/256 bytes)
//!   - 2 SDRAM banks with auto-refresh management
//!   - Burst mode support for synchronous accesses
//!   - Write FIFO for SDRAM
//!   - Wait signal management (NWAIT)
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 17: Flexible memory controller (FMC)
//! - DS14395 Datasheet for specific device capabilities

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// FMC register base address / FMC 寄存器基地址
/// AHB3 bus, accessible at 0x420D_0000
pub const FMC_BASE: usize = 0x420D_0000;

// ============================================================================
// NOR/PSRAM/SRAM Bank Addresses / NOR/PSRAM/SRAM 存储区域地址
// ============================================================================

/// Bank 1 base address (NOR/PSRAM/SRAM) / Bank 1 基地址
/// Address range: 0x6000_0000 - 0x6FFF_FFFF (256MB)
pub const FMC_BANK1_BASE: usize = 0x6000_0000;

/// Bank 2 base address (NOR/PSRAM/SRAM) / Bank 2 基地址
/// Address range: 0x6400_0000 - 0x67FF_FFFF (64MB)
pub const FMC_BANK2_BASE: usize = 0x6400_0000;

/// Bank 3 base address (NOR/PSRAM/SRAM) / Bank 3 基地址
/// Address range: 0x6800_0000 - 0x6BFF_FFFF (64MB)
pub const FMC_BANK3_BASE: usize = 0x6800_0000;

/// Bank 4 base address (NOR/PSRAM/SRAM) / Bank 4 基地址
/// Address range: 0x6C00_0000 - 0x6FFF_FFFF (64MB)
pub const FMC_BANK4_BASE: usize = 0x6C00_0000;

// ============================================================================
// NAND Bank Addresses / NAND 存储区域地址
// ============================================================================

/// NAND Bank 2 base address / NAND Bank 2 基地址
/// Address range: 0x8000_0000 - 0x8FFF_FFFF (256MB)
pub const FMC_NAND_BANK2_BASE: usize = 0x8000_0000;

/// NAND Bank 3 base address / NAND Bank 3 基地址
/// Address range: 0x8000_0000 - 0x8FFF_FFFF (shared with Bank 2)
pub const FMC_NAND_BANK3_BASE: usize = 0x8000_0000;

// ============================================================================
// SDRAM Bank Addresses / SDRAM 存储区域地址
// ============================================================================

/// SDRAM Bank 1 base address / SDRAM Bank 1 基地址
/// Address range: 0xC000_0000 - 0xCFFF_FFFF (256MB)
pub const FMC_SDRAM_BANK1_BASE: usize = 0xC000_0000;

/// SDRAM Bank 2 base address / SDRAM Bank 2 基地址
/// Address range: 0xD000_0000 - 0xDFFF_FFFF (256MB)
pub const FMC_SDRAM_BANK2_BASE: usize = 0xD000_0000;

// ============================================================================
// Register Offsets / 寄存器偏移
// ============================================================================

/// FMC register offsets / FMC 寄存器偏移
pub mod reg {
    // NOR/PSRAM Control and Timing Registers / NOR/PSRAM 控制和时序寄存器
    pub const BCR1: usize = 0x00;   // Bank 1 control register
    pub const BTR1: usize = 0x04;   // Bank 1 timing register
    pub const BCR2: usize = 0x08;   // Bank 2 control register
    pub const BTR2: usize = 0x0C;   // Bank 2 timing register
    pub const BCR3: usize = 0x10;   // Bank 3 control register
    pub const BTR3: usize = 0x14;   // Bank 3 timing register
    pub const BCR4: usize = 0x18;   // Bank 4 control register
    pub const BTR4: usize = 0x1C;   // Bank 4 timing register
    
    // NAND/PC Card Control and Status Registers / NAND/PC Card 控制和状态寄存器
    pub const PCR2: usize = 0x60;   // Bank 2 control register
    pub const SR2: usize = 0x64;    // Bank 2 status register
    pub const PMEM2: usize = 0x68;  // Bank 2 memory timing register
    pub const PATT2: usize = 0x6C;  // Bank 2 attribute timing register
    pub const ECCR2: usize = 0x74;  // Bank 2 ECC result register
    
    pub const PCR3: usize = 0x80;   // Bank 3 control register
    pub const SR3: usize = 0x84;    // Bank 3 status register
    pub const PMEM3: usize = 0x88;  // Bank 3 memory timing register
    pub const PATT3: usize = 0x8C;  // Bank 3 attribute timing register
    pub const ECCR3: usize = 0x94;  // Bank 3 ECC result register
    
    pub const PCR4: usize = 0xA0;   // Bank 4 control register (PC Card)
    pub const SR4: usize = 0xA4;    // Bank 4 status register
    pub const PMEM4: usize = 0xA8;  // Bank 4 memory timing register
    pub const PATT4: usize = 0xAC;  // Bank 4 attribute timing register
    pub const PIO4: usize = 0xB0;   // Bank 4 I/O timing register
    
    // SDRAM Control and Timing Registers / SDRAM 控制和时序寄存器
    pub const SDCR1: usize = 0x140; // SDRAM Bank 1 control register
    pub const SDCR2: usize = 0x144; // SDRAM Bank 2 control register
    pub const SDTR1: usize = 0x148; // SDRAM Bank 1 timing register
    pub const SDTR2: usize = 0x14C; // SDRAM Bank 2 timing register
    pub const SDCMR: usize = 0x150; // SDRAM command mode register
    pub const SDRTR: usize = 0x154; // SDRAM refresh timer register
    pub const SDSR: usize = 0x158;  // SDRAM status register
    pub const SDCR3: usize = 0x15C; // SDRAM Bank 3 control register (if available)
    pub const SDTR3: usize = 0x160; // SDRAM Bank 3 timing register (if available)
}

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// FMC Bank selection / FMC Bank 选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bank {
    /// Bank 1: NOR/PSRAM/SRAM (0x6000_0000)
    Bank1 = 0,
    /// Bank 2: NOR/PSRAM/SRAM (0x6400_0000)
    Bank2 = 1,
    /// Bank 3: NOR/PSRAM/SRAM (0x6800_0000)
    Bank3 = 2,
    /// Bank 4: NOR/PSRAM/SRAM/PC Card (0x6C00_0000)
    Bank4 = 3,
}

/// Memory type for NOR/PSRAM banks / NOR/PSRAM 存储器类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemoryType {
    /// SRAM (Static RAM)
    /// 静态随机存储器
    Sram = 0b00,
    /// PSRAM (Pseudo-SRAM / CellularRAM)
    /// 伪静态随机存储器
    Psram = 0b01,
    /// NOR Flash
    /// NOR 闪存
    Nor = 0b10,
}

/// Data bus width / 数据总线宽度
/// Note: STM32U5 FMC supports 8-bit and 16-bit only for external memories
/// 注意：STM32U5 FMC 对外部存储器仅支持 8 位和 16 位数据宽度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataWidth {
    /// 8-bit data bus / 8 位数据总线
    Bits8 = 0b00,
    /// 16-bit data bus / 16 位数据总线
    /// This is the maximum width supported by STM32U5 FMC
    /// 这是 STM32U5 FMC 支持的最大宽度
    Bits16 = 0b01,
}

/// Burst access mode / 突发访问模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BurstMode {
    /// Burst mode disabled / 突发模式禁用
    Disabled = 0,
    /// Burst mode enabled (for synchronous accesses) / 突发模式启用（用于同步访问）
    Enabled = 1,
}

/// Wait signal polarity / 等待信号极性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaitSignalPolarity {
    /// NWAIT active low / NWAIT 低电平有效
    ActiveLow = 0,
    /// NWAIT active high / NWAIT 高电平有效
    ActiveHigh = 1,
}

/// Wait signal timing / 等待信号时序
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaitConfig {
    /// NWAIT signal active before wait state / NWAIT 信号在等待状态之前有效
    BeforeWaitState = 0,
    /// NWAIT signal active during wait state / NWAIT 信号在等待状态期间有效
    DuringWaitState = 1,
}

/// Write burst mode / 写突发模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteBurst {
    /// Write operations always performed in single burst
    /// 写操作始终以单次突发执行
    Single = 0,
    /// Write operations performed in burst mode (same as read)
    /// 写操作以突发模式执行（与读相同）
    Burst = 1,
}

/// Extended mode enable / 扩展模式使能
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtendedMode {
    /// Extended mode disabled (BTR used for both read and write)
    /// 扩展模式禁用（读写使用相同时序）
    Disabled = 0,
    /// Extended mode enabled (BWTR used for write timing)
    /// 扩展模式启用（写操作使用 BWTR 时序）
    Enabled = 1,
}

// ============================================================================
// Configuration Structures / 配置结构体
// ============================================================================

/// NOR/PSRAM/SRAM configuration / NOR/PSRAM/SRAM 配置
#[derive(Clone, Copy, Debug)]
pub struct NorConfig {
    /// Memory type / 存储器类型
    pub memory_type: MemoryType,
    /// Data bus width / 数据总线宽度 (8-bit or 16-bit, 32-bit NOT supported)
    /// 数据总线宽度（8 位或 16 位，不支持 32 位）
    pub data_width: DataWidth,
    /// Burst mode for synchronous accesses / 同步访问突发模式
    pub burst_mode: BurstMode,
    /// Wait signal polarity / 等待信号极性
    pub wait_signal_polarity: WaitSignalPolarity,
    /// Wait signal configuration / 等待信号配置
    pub wait_config: WaitConfig,
    /// Write enable / 写使能
    pub write_enable: bool,
    /// Wait enable / 等待使能
    pub wait_enable: bool,
    /// Extended mode (separate write timing) / 扩展模式（独立写时序）
    pub ext_mode: ExtendedMode,
    /// Asynchronous wait / 异步等待
    pub async_wait: bool,
    /// Write burst mode / 写突发模式
    pub write_burst: WriteBurst,
    /// Continuous clock (for synchronous burst) / 连续时钟（用于同步突发）
    pub continuous_clock: bool,
}

impl Default for NorConfig {
    fn default() -> Self {
        Self {
            memory_type: MemoryType::Sram,
            data_width: DataWidth::Bits16,  // Default to 16-bit / 默认为 16 位
            burst_mode: BurstMode::Disabled,
            wait_signal_polarity: WaitSignalPolarity::ActiveLow,
            wait_config: WaitConfig::DuringWaitState,
            write_enable: true,
            wait_enable: false,
            ext_mode: ExtendedMode::Disabled,
            async_wait: false,
            write_burst: WriteBurst::Single,
            continuous_clock: false,
        }
    }
}

/// NOR/PSRAM timing configuration / NOR/PSRAM 时序配置
/// All timing values are in HCLK cycles / 所有时序值以 HCLK 周期为单位
#[derive(Clone, Copy, Debug)]
pub struct Timing {
    /// Address setup phase duration (0-15 HCLK cycles)
    /// 地址建立阶段持续时间（0-15 个 HCLK 周期）
    pub address_setup: u8,
    /// Address hold phase duration (1-15 HCLK cycles for async multiplexed)
    /// 地址保持阶段持续时间（异步复用模式为 1-15 个 HCLK 周期）
    pub address_hold: u8,
    /// Data setup phase duration (1-255 HCLK cycles)
    /// 数据建立阶段持续时间（1-255 个 HCLK 周期）
    pub data_setup: u8,
    /// Bus turnaround phase duration (0-15 HCLK cycles)
    /// 总线周转阶段持续时间（0-15 个 HCLK 周期）
    pub bus_turn: u8,
    /// Clock division ratio (2-16, for synchronous accesses)
    /// 时钟分频比（2-16，用于同步访问）
    pub clk_div: u8,
    /// Data latency (0-15, for synchronous NOR Flash)
    /// 数据延迟（0-15，用于同步 NOR Flash）
    pub data_latency: u8,
    /// Access mode (A, B, C, or D)
    /// 访问模式（A、B、C 或 D）
    pub access_mode: u8,
}

impl Default for Timing {
    fn default() -> Self {
        Self {
            address_setup: 0,
            address_hold: 1,
            data_setup: 1,
            bus_turn: 0,
            clk_div: 2,
            data_latency: 0,
            access_mode: 0,  // Mode A (default for SRAM)
        }
    }
}

/// Access mode types / 访问模式类型
pub mod access_mode {
    /// Mode A: Default mode for SRAM / SRAM 默认模式
    pub const MODE_A: u8 = 0;
    /// Mode B: Extended mode / 扩展模式
    pub const MODE_B: u8 = 1;
    /// Mode C: NOR Flash with OE toggling / 带 OE 切换的 NOR Flash
    pub const MODE_C: u8 = 2;
    /// Mode D: Extended mode with OE toggling / 带 OE 切换的扩展模式
    pub const MODE_D: u8 = 3;
}

/// FMC instance / FMC 实例
pub struct Fmc;

impl Fmc {
    /// Create new FMC instance / 创建 FMC 实例
    pub const fn new() -> Self {
        Self
    }

    /// Enable FMC clock / 使能 FMC 时钟
    fn enable_clock(&self) {
        // Enable AHB3 clock for FMC
        // 使能 FMC 的 AHB3 时钟
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb3enr = rcc_base.add(0xE4 / 4);
            *ahb3enr |= 1 << 0;  // FMCEN bit
        }
    }

    /// Initialize FMC for NOR/PSRAM/SRAM / 初始化 NOR/PSRAM/SRAM
    /// 
    /// # Arguments
    /// * `bank` - FMC bank to configure (Bank1-4)
    /// * `config` - Memory configuration
    /// * `timing` - Timing configuration
    pub fn init_nor(&self, bank: Bank, config: &NorConfig, timing: &Timing) {
        self.enable_clock();

        unsafe {
            // Configure BCR (Bank Control Register)
            let bcr = (FMC_BASE + reg::BCR1 + (bank as usize) * 8) as *mut u32;
            let mut val = 0;
            
            // MTYP[1:0]: Memory type
            val |= (config.memory_type as u32) << 2;
            
            // MWID[1:0]: Memory data bus width (8 or 16-bit, NOT 32-bit)
            // MWID[1:0]：存储器数据总线宽度（8 或 16 位，不是 32 位）
            val |= (config.data_width as u32) << 4;
            
            // BURSTEN: Burst enable for synchronous accesses
            val |= (config.burst_mode as u32) << 8;
            
            // WAITPOL: Wait signal polarity
            val |= (config.wait_signal_polarity as u32) << 9;
            
            // WAITCFG: Wait timing configuration
            val |= (config.wait_config as u32) << 11;
            
            // WREN: Write enable
            val |= (config.write_enable as u32) << 12;
            
            // WAITEN: Wait enable
            val |= (config.wait_enable as u32) << 13;
            
            // EXTMOD: Extended mode enable
            val |= (config.ext_mode as u32) << 14;
            
            // ASYNCWAIT: Asynchronous wait
            val |= (config.async_wait as u32) << 15;
            
            // CBURSTRW: Write burst enable
            val |= (config.write_burst as u32) << 19;
            
            // CCLKEN: Continuous clock enable
            val |= (config.continuous_clock as u32) << 20;
            
            // MBKEN: Memory bank enable (must be set last)
            val |= 1 << 0;
            
            write_volatile(bcr, val);

            // Configure BTR (Bank Timing Register)
            let btr = (FMC_BASE + reg::BTR1 + (bank as usize) * 8) as *mut u32;
            let mut val = 0;
            val |= (timing.address_setup as u32) << 0;   // ADDSET[3:0]
            val |= (timing.address_hold as u32) << 4;    // ADDHLD[3:0]
            val |= (timing.data_setup as u32) << 8;      // DATAST[7:0]
            val |= (timing.bus_turn as u32) << 16;       // BUSTURN[3:0]
            val |= (timing.clk_div as u32) << 20;        // CLKDIV[3:0]
            val |= (timing.data_latency as u32) << 24;   // DATLAT[3:0]
            val |= (timing.access_mode as u32) << 28;    // ACCMOD[1:0]
            write_volatile(btr, val);
        }
    }

    /// Read 8-bit data from NOR/PSRAM/SRAM / 从 NOR/PSRAM/SRAM 读取 8 位数据
    pub fn read8(&self, bank: Bank, addr: u32) -> u8 {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u8;
            read_volatile(ptr)
        }
    }

    /// Read 16-bit data from NOR/PSRAM/SRAM / 从 NOR/PSRAM/SRAM 读取 16 位数据
    pub fn read16(&self, bank: Bank, addr: u32) -> u16 {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u16;
            read_volatile(ptr)
        }
    }

    /// Read 32-bit data from NOR/PSRAM/SRAM / 从 NOR/PSRAM/SRAM 读取 32 位数据
    /// Note: This performs two 16-bit reads internally on 16-bit bus
    /// 注意：在 16 位总线上这会内部执行两次 16 位读取
    pub fn read32(&self, bank: Bank, addr: u32) -> u32 {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u32;
            read_volatile(ptr)
        }
    }

    /// Write 8-bit data to NOR/PSRAM/SRAM / 向 NOR/PSRAM/SRAM 写入 8 位数据
    pub fn write8(&self, bank: Bank, addr: u32, data: u8) {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u8;
            write_volatile(ptr, data);
        }
    }

    /// Write 16-bit data to NOR/PSRAM/SRAM / 向 NOR/PSRAM/SRAM 写入 16 位数据
    pub fn write16(&self, bank: Bank, addr: u32, data: u16) {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u16;
            write_volatile(ptr, data);
        }
    }

    /// Write 32-bit data to NOR/PSRAM/SRAM / 向 NOR/PSRAM/SRAM 写入 32 位数据
    /// Note: This performs two 16-bit writes internally on 16-bit bus
    /// 注意：在 16 位总线上这会内部执行两次 16 位写入
    pub fn write32(&self, bank: Bank, addr: u32, data: u32) {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u32;
            write_volatile(ptr, data);
        }
    }

    /// Get bank base address / 获取 Bank 基地址
    pub fn get_bank_base(&self, bank: Bank) -> usize {
        match bank {
            Bank::Bank1 => FMC_BANK1_BASE,
            Bank::Bank2 => FMC_BANK2_BASE,
            Bank::Bank3 => FMC_BANK3_BASE,
            Bank::Bank4 => FMC_BANK4_BASE,
        }
    }

    /// Disable memory bank / 禁用存储器 Bank
    pub fn disable_bank(&self, bank: Bank) {
        unsafe {
            let bcr = (FMC_BASE + reg::BCR1 + (bank as usize) * 8) as *mut u32;
            let val = read_volatile(bcr);
            write_volatile(bcr, val & !0x01);  // Clear MBKEN
        }
    }
}

// ============================================================================
// SDRAM Support / SDRAM 支持
// ============================================================================

/// SDRAM configuration / SDRAM 配置
#[derive(Clone, Copy, Debug)]
pub struct SdramConfig {
    /// Number of column address bits (8-11) / 列地址位数
    pub column_bits: u8,
    /// Number of row address bits (11-13) / 行地址位数
    pub row_bits: u8,
    /// Data bus width / 数据总线宽度 (8 or 16-bit)
    pub data_width: DataWidth,
    /// Number of internal banks (2 or 4) / 内部 Bank 数量
    pub internal_bank: u8,
    /// CAS latency (1-3) / CAS 延迟
    pub cas_latency: u8,
    /// Write protection / 写保护
    pub write_protection: bool,
    /// Read burst mode / 读突发模式
    pub read_burst: bool,
    /// Read pipe delay (0-2) / 读流水线延迟
    pub read_pipe: u8,
}

impl Default for SdramConfig {
    fn default() -> Self {
        Self {
            column_bits: 8,
            row_bits: 12,
            data_width: DataWidth::Bits16,
            internal_bank: 4,
            cas_latency: 2,
            write_protection: false,
            read_burst: true,
            read_pipe: 0,
        }
    }
}

/// SDRAM timing parameters / SDRAM 时序参数
/// All values in clock cycles / 所有值以时钟周期为单位
#[derive(Clone, Copy, Debug)]
pub struct SdramTiming {
    /// Load mode register to active delay (TMRD) / 加载模式寄存器到激活延迟
    pub load_to_active_delay: u8,
    /// Exit self-refresh to active delay (TXSR) / 退出自刷新到激活延迟
    pub exit_self_refresh_delay: u8,
    /// Self-refresh time (TRAS) / 自刷新时间
    pub self_refresh_time: u8,
    /// Row cycle delay (TRC) / 行周期延迟
    pub row_cycle_delay: u8,
    /// Write recovery time (TWR) / 写恢复时间
    pub write_recovery_time: u8,
    /// Row precharge delay (TRP) / 行预充电延迟
    pub rp_delay: u8,
    /// Row to column delay (TRCD) / 行到列延迟
    pub rc_delay: u8,
}

impl Default for SdramTiming {
    fn default() -> Self {
        Self {
            load_to_active_delay: 2,
            exit_self_refresh_delay: 7,
            self_refresh_time: 4,
            row_cycle_delay: 7,
            write_recovery_time: 2,
            rp_delay: 2,
            rc_delay: 2,
        }
    }
}

/// SDRAM commands / SDRAM 命令
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdramCommand {
    /// Normal mode / 正常模式
    Normal = 0b000,
    /// Clock configuration enable / 时钟配置使能
    ClockConfig = 0b001,
    /// Precharge all banks / 预充电所有 Bank
    PrechargeAll = 0b010,
    /// Auto-refresh / 自动刷新
    AutoRefresh = 0b011,
    /// Load mode register / 加载模式寄存器
    LoadMode = 0b100,
    /// Self-refresh / 自刷新
    SelfRefresh = 0b101,
    /// Power-down / 掉电
    PowerDown = 0b110,
}

/// SDRAM bank selection / SDRAM Bank 选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdramBank {
    /// SDRAM Bank 1 / SDRAM Bank 1
    Bank1 = 1,
    /// SDRAM Bank 2 / SDRAM Bank 2
    Bank2 = 2,
    /// Both banks / 两个 Bank
    Both = 3,
}

impl Fmc {
    /// Initialize SDRAM / 初始化 SDRAM
    /// 
    /// # Arguments
    /// * `bank` - SDRAM bank (1 or 2)
    /// * `config` - SDRAM configuration
    /// * `timing` - SDRAM timing parameters
    pub fn init_sdram(&self, bank: SdramBank, config: &SdramConfig, timing: &SdramTiming) {
        self.enable_clock();
        
        unsafe {
            // Configure SDCR (SDRAM Control Register)
            let sdcr = match bank {
                SdramBank::Bank1 => (FMC_BASE + reg::SDCR1) as *mut u32,
                SdramBank::Bank2 => (FMC_BASE + reg::SDCR2) as *mut u32,
                _ => (FMC_BASE + reg::SDCR1) as *mut u32,
            };
            
            let mut val = 0;
            // NC[1:0]: Number of column address bits (0=8, 1=9, 2=10, 3=11)
            val |= ((config.column_bits - 8) as u32) << 0;
            // NR[1:0]: Number of row address bits (0=11, 1=12, 2=13)
            val |= ((config.row_bits - 11) as u32) << 2;
            // MWID[1:0]: Memory data bus width (8 or 16-bit)
            val |= (config.data_width as u32) << 4;
            // NB: Number of internal banks (0=2 banks, 1=4 banks)
            val |= ((config.internal_bank / 2 - 1) as u32) << 6;
            // CAS[1:0]: CAS latency (0=1, 1=2, 2=3)
            val |= ((config.cas_latency - 1) as u32) << 7;
            // WP: Write protection
            val |= (config.write_protection as u32) << 9;
            // RBURST: Read burst
            val |= (config.read_burst as u32) << 12;
            // RPIPE[1:0]: Read pipe
            val |= (config.read_pipe as u32) << 13;
            
            write_volatile(sdcr, val);

            // Configure SDTR (SDRAM Timing Register)
            let sdtr = match bank {
                SdramBank::Bank1 => (FMC_BASE + reg::SDTR1) as *mut u32,
                SdramBank::Bank2 => (FMC_BASE + reg::SDTR2) as *mut u32,
                _ => (FMC_BASE + reg::SDTR1) as *mut u32,
            };
            
            let mut val = 0;
            val |= (timing.load_to_active_delay as u32) << 0;      // TMRD[3:0]
            val |= (timing.exit_self_refresh_delay as u32) << 4;   // TXSR[3:0]
            val |= (timing.self_refresh_time as u32) << 8;         // TRAS[3:0]
            val |= (timing.row_cycle_delay as u32) << 12;          // TRC[3:0]
            val |= (timing.write_recovery_time as u32) << 16;      // TWR[3:0]
            val |= (timing.rp