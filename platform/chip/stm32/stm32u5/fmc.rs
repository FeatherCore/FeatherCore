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
/// Reference: RM0456 Chapter 17.6 / 参考: RM0456 第17.6节
pub mod reg {
    // NOR/PSRAM Control and Timing Registers / NOR/PSRAM 控制和时序寄存器
    // Reference: RM0456 Chapter 17.6.1 / 参考: RM0456 第17.6.1节
    pub const BCR1: usize = 0x00;
    pub const BTR1: usize = 0x04;
    pub const BCR2: usize = 0x08;
    pub const BTR2: usize = 0x0C;
    pub const BCR3: usize = 0x10;
    pub const BTR3: usize = 0x14;
    pub const BCR4: usize = 0x18;
    pub const BTR4: usize = 0x1C;
    
    // NOR/PSRAM Extended Timing Registers / NOR/PSRAM 扩展时序寄存器
    pub const BWTR1: usize = 0x104;
    pub const BWTR2: usize = 0x10C;
    pub const BWTR3: usize = 0x114;
    pub const BWTR4: usize = 0x11C;
    
    // NAND/PC Card Control and Status Registers / NAND/PC Card 控制和状态寄存器
    // Reference: RM0456 Chapter 17.6.3 / 参考: RM0456 第17.6.3节
    pub const PCR2: usize = 0x60;
    pub const SR2: usize = 0x64;
    pub const PMEM2: usize = 0x68;
    pub const PATT2: usize = 0x6C;
    pub const ECCR2: usize = 0x74;
    
    pub const PCR3: usize = 0x80;
    pub const SR3: usize = 0x84;
    pub const PMEM3: usize = 0x88;
    pub const PATT3: usize = 0x8C;
    pub const ECCR3: usize = 0x94;
    
    pub const PCR4: usize = 0xA0;
    pub const SR4: usize = 0xA4;
    pub const PMEM4: usize = 0xA8;
    pub const PATT4: usize = 0xAC;
    pub const PIO4: usize = 0xB0;
    
    // SDRAM Control and Timing Registers / SDRAM 控制和时序寄存器
    // Reference: RM0456 Chapter 17.6.4 / 参考: RM0456 第17.6.4节
    pub const SDCR1: usize = 0x140;
    pub const SDCR2: usize = 0x144;
    pub const SDTR1: usize = 0x148;
    pub const SDTR2: usize = 0x14C;
    pub const SDCMR: usize = 0x150;
    pub const SDRTR: usize = 0x154;
    pub const SDSR: usize = 0x158;
}

// ============================================================================
// Register Bit Definitions / 寄存器位定义
// ============================================================================

/// NOR/PSRAM Bank Control Register bits / NOR/PSRAM Bank 控制寄存器位
/// Reference: RM0456 Chapter 17.6.1 / 参考: RM0456 第17.6.1节
pub mod bcr_bits {
    pub const MBKEN: u32 = 1 << 0;           /// Memory bank enable / 存储器 Bank 使能
    pub const MUXEN: u32 = 1 << 1;           /// Address/Data multiplexing enable / 地址/数据复用使能
    pub const MTYP_SHIFT: u32 = 2;            /// Memory type shift / 存储器类型位移
    pub const MTYP_MASK: u32 = 0x3 << 2;     /// Memory type mask / 存储器类型掩码
    pub const MTYP_SRAM: u32 = 0x0 << 2;     /// SRAM / SRAM
    pub const MTYP_PSRAM: u32 = 0x1 << 2;    /// PSRAM (CellularRAM) / PSRAM
    pub const MTYP_NOR: u32 = 0x2 << 2;       /// NOR Flash / NOR Flash
    pub const MWID_SHIFT: u32 = 4;           /// Memory data width shift / 存储器数据宽度位移
    pub const MWID_MASK: u32 = 0x3 << 4;     /// Memory data width mask / 存储器数据宽度掩码
    pub const MWID_8BIT: u32 = 0x0 << 4;     /// 8-bit / 8 位
    pub const MWID_16BIT: u32 = 0x1 << 4;    /// 16-bit / 16 位
    pub const FACCEN: u32 = 1 << 6;          /// Flash access enable / Flash 访问使能
    pub const BURSTEN: u32 = 1 << 8;         /// Burst enable / 突发使能
    pub const WAITPOL: u32 = 1 << 9;         /// Wait signal polarity / 等待信号极性
    pub const WAITCFG: u32 = 1 << 11;        /// Wait timing configuration / 等待时序配置
    pub const WREN: u32 = 1 << 12;           /// Write enable / 写使能
    pub const WAITEN: u32 = 1 << 13;         /// Wait enable / 等待使能
    pub const EXTMOD: u32 = 1 << 14;          /// Extended mode enable / 扩展模式使能
    pub const ASYNCWAIT: u32 = 1 << 15;       /// Asynchronous wait / 异步等待
    pub const CBURSTRW: u32 = 1 << 19;       /// Write burst enable / 写突发使能
    pub const CCLKEN: u32 = 1 << 20;          /// Continuous clock enable / 连续时钟使能
    pub const WFDIS: u32 = 1 << 21;           /// Write FIFO disable / 写 FIFO 禁用
    pub const BANK4_1: u32 = 1 << 22;         /// NAND Flash bank 1 / NAND Flash Bank 1
    pub const BANK4_2: u32 = 1 << 23;         /// NAND Flash bank 2 / NAND Flash Bank 2
}

/// NOR/PSRAM Bank Timing Register bits / NOR/PSRAM Bank 时序寄存器位
/// Reference: RM0456 Chapter 17.6.1 / 参考: RM0456 第17.6.1节
pub mod btr_bits {
    pub const ADDSET_SHIFT: u32 = 0;         /// Address setup shift / 地址建立位移
    pub const ADDSET_MASK: u32 = 0xF << 0;    /// Address setup mask / 地址建立掩码
    pub const ADDHLD_SHIFT: u32 = 4;          /// Address hold shift / 地址保持位移
    pub const ADDHLD_MASK: u32 = 0xF << 4;   /// Address hold mask / 地址保持掩码
    pub const DATAST_SHIFT: u32 = 8;          /// Data setup shift / 数据建立位移
    pub const DATAST_MASK: u32 = 0xFF << 8;   /// Data setup mask / 数据建立掩码
    pub const BUSTURN_SHIFT: u32 = 16;        /// Bus turnaround shift / 总线周转位移
    pub const BUSTURN_MASK: u32 = 0xF << 16;  /// Bus turnaround mask / 总线周转掩码
    pub const CLKDIV_SHIFT: u32 = 20;         /// Clock divide shift / 时钟分频位移
    pub const CLKDIV_MASK: u32 = 0xF << 20;   /// Clock divide mask / 时钟分频掩码
    pub const DATLAT_SHIFT: u32 = 24;         /// Data latency shift / 数据延迟位移
    pub const DATLAT_MASK: u32 = 0xF << 24;   /// Data latency mask / 数据延迟掩码
    pub const ACCMOD_SHIFT: u32 = 28;          /// Access mode shift / 访问模式位移
    pub const ACCMOD_MASK: u32 = 0x3 << 28;    /// Access mode mask / 访问模式掩码
    pub const ACCMOD_A: u32 = 0x0 << 28;       /// Mode A / 模式 A
    pub const ACCMOD_B: u32 = 0x1 << 28;       /// Mode B / 模式 B
    pub const ACCMOD_C: u32 = 0x2 << 28;       /// Mode C / 模式 C
    pub const ACCMOD_D: u32 = 0x3 << 28;       /// Mode D / 模式 D
}

/// NAND Control Register bits / NAND 控制寄存器位
/// Reference: RM0456 Chapter 17.6.3 / 参考: RM0456 第17.6.3节
pub mod pcr_bits {
    pub const PWAITEN: u32 = 1 << 1;           /// Wait enable / 等待使能
    pub const PTYP: u32 = 1 << 3;             /// NAND Flash type / NAND Flash 类型
    pub const PWID_SHIFT: u32 = 4;            /// NAND Flash data width shift / NAND Flash 数据宽度位移
    pub const PWID_MASK: u32 = 0x3 << 4;      /// NAND Flash data width mask / NAND Flash 数据宽度掩码
    pub const PWID_8BIT: u32 = 0x0 << 4;      /// 8-bit / 8 位
    pub const PWID_16BIT: u32 = 0x1 << 4;     /// 16-bit / 16 位
    pub const ECCEN: u32 = 1 << 6;            /// ECC enable / ECC 使能
    pub const ECCPS_SHIFT: u32 = 17;           /// ECC page size shift / ECC 页面大小位移
    pub const ECCPS_MASK: u32 = 0x7 << 17;     /// ECC page size mask / ECC 页面大小掩码
    pub const ECCPS_256: u32 = 0x0 << 17;      /// 256 bytes / 256 字节
    pub const ECCPS_512: u32 = 0x1 << 17;      /// 512 bytes / 512 字节
    pub const ECCPS_1024: u32 = 0x2 << 17;     /// 1024 bytes / 1024 字节
    pub const ECCPS_2048: u32 = 0x3 << 17;     /// 2048 bytes / 2048 字节
    pub const ECCPS_4096: u32 = 0x4 << 17;     /// 4096 bytes / 4096 字节
    pub const ECCPS_8192: u32 = 0x5 << 17;     /// 8192 bytes / 8192 字节
}

/// NAND Status Register bits / NAND 状态寄存器位
/// Reference: RM0456 Chapter 17.6.3 / 参考: RM0456 第17.6.3节
pub mod sr_bits {
    pub const IRS: u32 = 1 << 0;              /// Invalid command / 无效命令
    pub const ILS: u32 = 1 << 1;              /// Invalid command latch / 无效命令锁存
    pub const IFS: u32 = 1 << 2;              /// Invalid flash access / 无效 Flash 访问
    pub const ILFS: u32 = 1 << 3;              /// Invalid flash location / 无效 Flash 位置
    pub const PEF: u32 = 1 << 5;              /// Programming error / 编程错误
    pub const PTER: u32 = 1 << 5;             /// Programming / 编程
    pub const RB0: u32 = 1 << 6;              /// NAND Flash R/B0 / NAND Flash R/B0
    pub const RB1: u32 = 1 << 7;              /// NAND Flash R/B1 / NAND Flash R/B1
}

/// SDRAM Control Register bits / SDRAM 控制寄存器位
/// Reference: RM0456 Chapter 17.6.4 / 参考: RM0456 第17.6.4节
pub mod sdcr_bits {
    pub const NC_SHIFT: u32 = 0;               /// Number of column address bits shift / 列地址位数位移
    pub const NC_MASK: u32 = 0x3 << 0;        /// Number of column address bits mask / 列地址位数掩码
    pub const NC_8BIT: u32 = 0x0 << 0;         /// 8-bit column / 8位列
    pub const NC_9BIT: u32 = 0x1 << 0;         /// 9-bit column / 9位列
    pub const NC_10BIT: u32 = 0x2 << 0;        /// 10-bit column / 10位列
    pub const NC_11BIT: u32 = 0x3 << 0;        /// 11-bit column / 11位列
    pub const NR_SHIFT: u32 = 2;              /// Number of row address bits shift / 行地址位数位移
    pub const NR_MASK: u32 = 0x3 << 2;         /// Number of row address bits mask / 行地址位数掩码
    pub const NR_11BIT: u32 = 0x0 << 2;        /// 11-bit row / 11位行
    pub const NR_12BIT: u32 = 0x1 << 2;        /// 12-bit row / 12位行
    pub const NR_13BIT: u32 = 0x2 << 2;        /// 13-bit row / 13位行
    pub const MWID_SHIFT: u32 = 4;            /// Memory data width shift / 存储器数据宽度位移
    pub const MWID_MASK: u32 = 0x3 << 4;       /// Memory data width mask / 存储器数据宽度掩码
    pub const MWID_8BIT: u32 = 0x0 << 4;       /// 8-bit / 8 位
    pub const MWID_16BIT: u32 = 0x1 << 4;      /// 16-bit / 16 位
    pub const NB: u32 = 1 << 6;                /// Number of internal banks / 内部 Bank 数量
    pub const CAS_SHIFT: u32 = 7;              /// CAS latency shift / CAS 延迟位移
    pub const CAS_MASK: u32 = 0x3 << 7;        /// CAS latency mask / CAS 延迟掩码
    pub const CAS_1: u32 = 0x1 << 7;           /// 1 cycle / 1 周期
    pub const CAS_2: u32 = 0x2 << 7;           /// 2 cycles / 2 周期
    pub const CAS_3: u32 = 0x3 << 7;           /// 3 cycles / 3 周期
    pub const WP: u32 = 1 << 9;                /// Write protection / 写保护
    pub const SDCLK_SHIFT: u32 = 10;           /// SDRAM clock shift / SDRAM 时钟位移
    pub const SDCLK_MASK: u32 = 0x3 << 10;     /// SDRAM clock mask / SDRAM 时钟掩码
    pub const SDCLK_DISABLE: u32 = 0x0 << 10;  /// SDRAM clock disabled / SDRAM 时钟禁用
    pub const SDCLK_2: u32 = 0x1 << 10;        /// SDRAM clock = 2*HCLK / SDRAM 时钟 = 2*HCLK
    pub const SDCLK_3: u32 = 0x2 << 10;        /// SDRAM clock = 3*HCLK / SDRAM 时钟 = 3*HCLK
    pub const RBURST: u32 = 1 << 12;           /// Read burst / 读突发
    pub const RPIPE_SHIFT: u32 = 13;           /// Read pipe shift / 读流水线位移
    pub const RPIPE_MASK: u32 = 0x3 << 13;     /// Read pipe mask / 读流水线掩码
    pub const RPIPE_NO: u32 = 0x0 << 13;       /// No pipeline / 无流水线
    pub const RPIPE_1: u32 = 0x1 << 13;        /// 1 cycle / 1 周期
    pub const RPIPE_2: u32 = 0x2 << 13;        /// 2 cycles / 2 周期
}

/// SDRAM Timing Register bits / SDRAM 时序寄存器位
/// Reference: RM0456 Chapter 17.6.4 / 参考: RM0456 第17.6.4节
pub mod sdtr_bits {
    pub const TMRD_SHIFT: u32 = 0;             /// Load mode register to active shift / 加载模式寄存器到激活位移
    pub const TMRD_MASK: u32 = 0xF << 0;       /// Load mode register to active mask / 加载模式寄存器到激活掩码
    pub const TXSR_SHIFT: u32 = 4;             /// Exit self-refresh to active shift / 退出自刷新到激活位移
    pub const TXSR_MASK: u32 = 0xF << 4;       /// Exit self-refresh to active mask / 退出自刷新到激活掩码
    pub const TRAS_SHIFT: u32 = 8;             /// Self-refresh time shift / 自刷新时间位移
    pub const TRAS_MASK: u32 = 0xF << 8;       /// Self-refresh time mask / 自刷新时间掩码
    pub const TRC_SHIFT: u32 = 12;             /// Row cycle delay shift / 行周期延迟位移
    pub const TRC_MASK: u32 = 0xF << 12;        /// Row cycle delay mask / 行周期延迟掩码
    pub const TWR_SHIFT: u32 = 16;             /// Write recovery time shift / 写恢复时间位移
    pub const TWR_MASK: u32 = 0xF << 16;        /// Write recovery time mask / 写恢复时间掩码
    pub const TRP_SHIFT: u32 = 20;             /// Row precharge delay shift / 行预充电延迟位移
    pub const TRP_MASK: u32 = 0xF << 20;        /// Row precharge delay mask / 行预充电延迟掩码
    pub const TRCD_SHIFT: u32 = 24;            /// Row to column delay shift / 行到列延迟位移
    pub const TRCD_MASK: u32 = 0xF << 24;      /// Row to column delay mask / 行到列延迟掩码
}

/// SDRAM Command Register bits / SDRAM 命令寄存器位
/// Reference: RM0456 Chapter 17.6.4 / 参考: RM0456 第17.6.4节
pub mod sdcmr_bits {
    pub const CMD_SHIFT: u32 = 0;              /// Command shift / 命令位移
    pub const CMD_MASK: u32 = 0x7 << 0;        /// Command mask / 命令掩码
    pub const CMD_NORMAL: u32 = 0x0 << 0;      /// Normal mode / 正常模式
    pub const CMD_CLK_CONFIG: u32 = 0x1 << 0;  /// Clock configuration enable / 时钟配置使能
    pub const CMD_PRECHARGE: u32 = 0x2 << 0;   /// Precharge all banks / 预充电所有 Bank
    pub const CMD_AUTOREFRESH: u32 = 0x3 << 0; /// Auto-refresh / 自动刷新
    pub const CMD_LOAD_MODE: u32 = 0x4 << 0;   /// Load mode register / 加载模式寄存器
    pub const CMD_SELFREFRESH: u32 = 0x5 << 0; /// Self-refresh / 自刷新
    pub const CMD_POWERDOWN: u32 = 0x6 << 0;   /// Power-down / 掉电
    pub const CTB1: u32 = 1 << 3;              /// Command target bank 1 / 命令目标 Bank 1
    pub const CTB2: u32 = 1 << 4;              /// Command target bank 2 / 命令目标 Bank 2
    pub const MODE_SHIFT: u32 = 5;             /// Mode shift / 模式位移
    pub const MODE_MASK: u32 = 0x3 << 5;        /// Mode mask / 模式掩码
    pub const MODE_NORMAL: u32 = 0x0 << 5;      /// Normal mode / 正常模式
    pub const MODE_READ: u32 = 0x1 << 5;       /// Read mode / 读模式
    pub const MODE_WRITE: u32 = 0x2 << 5;       /// Write mode / 写模式
}

/// SDRAM Refresh Timer Register bits / SDRAM 刷新定时器寄存器位
pub mod sdrtr_bits {
    pub const COUNT_SHIFT: u32 = 1;            /// Refresh timer shift / 刷新定时器位移
    pub const COUNT_MASK: u32 = 0x1FFF << 1;   /// Refresh timer mask / 刷新定时器掩码
    pub const REIE: u32 = 1 << 14;             /// Refresh error interrupt enable / 刷新错误中断使能
}

/// SDRAM Status Register bits / SDRAM 状态寄存器位
pub mod sdsr_bits {
    pub const RE: u32 = 1 << 0;                /// Refresh error / 刷新错误
    pub const MODES1_SHIFT: u32 = 1;           /// Mode 1 status shift / 模式 1 状态位移
    pub const MODES1_MASK: u32 = 0x3 << 1;     /// Mode 1 status mask / 模式 1 状态掩码
    pub const MODES2_SHIFT: u32 = 3;           /// Mode 2 status shift / 模式 2 状态位移
    pub const MODES2_MASK: u32 = 0x3 << 3;     /// Mode 2 status mask / 模式 2 状态掩码
    pub const BUSY: u32 = 1 << 5;              /// Busy flag / 忙标志
}

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// FMC Bank selection / FMC Bank 选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bank {
    Bank1 = 0,
    Bank2 = 1,
    Bank3 = 2,
    Bank4 = 3,
}

/// Memory type for NOR/PSRAM banks / NOR/PSRAM 存储器类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemoryType {
    Sram = 0b00,
    Psram = 0b01,
    Nor = 0b10,
}

/// Data bus width / 数据总线宽度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataWidth {
    Bits8 = 0b00,
    Bits16 = 0b01,
}

/// Burst access mode / 突发访问模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BurstMode {
    Disabled = 0,
    Enabled = 1,
}

/// Wait signal polarity / 等待信号极性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaitSignalPolarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}

/// Wait signal timing / 等待信号时序
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaitConfig {
    BeforeWaitState = 0,
    DuringWaitState = 1,
}

/// Write burst mode / 写突发模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WriteBurst {
    Single = 0,
    Burst = 1,
}

/// Extended mode enable / 扩展模式使能
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtendedMode {
    Disabled = 0,
    Enabled = 1,
}

// ============================================================================
// Configuration Structures / 配置结构体
// ============================================================================

/// NOR/PSRAM/SRAM configuration / NOR/PSRAM/SRAM 配置
#[derive(Clone, Copy, Debug)]
pub struct NorConfig {
    pub memory_type: MemoryType,
    pub data_width: DataWidth,
    pub burst_mode: BurstMode,
    pub wait_signal_polarity: WaitSignalPolarity,
    pub wait_config: WaitConfig,
    pub write_enable: bool,
    pub wait_enable: bool,
    pub ext_mode: ExtendedMode,
    pub async_wait: bool,
    pub write_burst: WriteBurst,
    pub continuous_clock: bool,
}

impl Default for NorConfig {
    fn default() -> Self {
        Self {
            memory_type: MemoryType::Sram,
            data_width: DataWidth::Bits16,
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
#[derive(Clone, Copy, Debug)]
pub struct Timing {
    pub address_setup: u8,
    pub address_hold: u8,
    pub data_setup: u8,
    pub bus_turn: u8,
    pub clk_div: u8,
    pub data_latency: u8,
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
            access_mode: 0,
        }
    }
}

/// Access mode types / 访问模式类型
pub mod access_mode {
    pub const MODE_A: u8 = 0;
    pub const MODE_B: u8 = 1;
    pub const MODE_C: u8 = 2;
    pub const MODE_D: u8 = 3;
}

/// FMC instance / FMC 实例
pub struct Fmc;

impl Fmc {
    pub const fn new() -> Self {
        Self
    }

    fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb3enr = rcc_base.add(0xE4 / 4);
            *ahb3enr |= 1 << 0;
        }
    }

    pub fn init_nor(&self, bank: Bank, config: &NorConfig, timing: &Timing) {
        self.enable_clock();

        unsafe {
            let bcr = (FMC_BASE + reg::BCR1 + (bank as usize) * 8) as *mut u32;
            let mut val = 0;
            val |= (config.memory_type as u32) << 2;
            val |= (config.data_width as u32) << 4;
            val |= (config.burst_mode as u32) << 8;
            val |= (config.wait_signal_polarity as u32) << 9;
            val |= (config.wait_config as u32) << 11;
            val |= (config.write_enable as u32) << 12;
            val |= (config.wait_enable as u32) << 13;
            val |= (config.ext_mode as u32) << 14;
            val |= (config.async_wait as u32) << 15;
            val |= (config.write_burst as u32) << 19;
            val |= (config.continuous_clock as u32) << 20;
            val |= 1 << 0;
            write_volatile(bcr, val);

            let btr = (FMC_BASE + reg::BTR1 + (bank as usize) * 8) as *mut u32;
            let mut val = 0;
            val |= (timing.address_setup as u32) << 0;
            val |= (timing.address_hold as u32) << 4;
            val |= (timing.data_setup as u32) << 8;
            val |= (timing.bus_turn as u32) << 16;
            val |= (timing.clk_div as u32) << 20;
            val |= (timing.data_latency as u32) << 24;
            val |= (timing.access_mode as u32) << 28;
            write_volatile(btr, val);
        }
    }

    pub fn read8(&self, bank: Bank, addr: u32) -> u8 {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u8;
            read_volatile(ptr)
        }
    }

    pub fn read16(&self, bank: Bank, addr: u32) -> u16 {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u16;
            read_volatile(ptr)
        }
    }

    pub fn read32(&self, bank: Bank, addr: u32) -> u32 {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u32;
            read_volatile(ptr)
        }
    }

    pub fn write8(&self, bank: Bank, addr: u32, data: u8) {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u8;
            write_volatile(ptr, data);
        }
    }

    pub fn write16(&self, bank: Bank, addr: u32, data: u16) {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u16;
            write_volatile(ptr, data);
        }
    }

    pub fn write32(&self, bank: Bank, addr: u32, data: u32) {
        unsafe {
            let base = FMC_BANK1_BASE + (bank as usize) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u32;
            write_volatile(ptr, data);
        }
    }

    pub fn get_bank_base(&self, bank: Bank) -> usize {
        match bank {
            Bank::Bank1 => FMC_BANK1_BASE,
            Bank::Bank2 => FMC_BANK2_BASE,
            Bank::Bank3 => FMC_BANK3_BASE,
            Bank::Bank4 => FMC_BANK4_BASE,
        }
    }

    pub fn disable_bank(&self, bank: Bank) {
        unsafe {
            let bcr = (FMC_BASE + reg::BCR1 + (bank as usize) * 8) as *mut u32;
            let val = read_volatile(bcr);
            write_volatile(bcr, val & !0x01);
        }
    }
}

// ============================================================================
// SDRAM Support / SDRAM 支持
// ============================================================================

/// SDRAM configuration / SDRAM 配置
#[derive(Clone, Copy, Debug)]
pub struct SdramConfig {
    pub column_bits: u8,
    pub row_bits: u8,
    pub data_width: DataWidth,
    pub internal_bank: u8,
    pub cas_latency: u8,
    pub write_protection: bool,
    pub read_burst: bool,
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
#[derive(Clone, Copy, Debug)]
pub struct SdramTiming {
    pub load_to_active_delay: u8,
    pub exit_self_refresh_delay: u8,
    pub self_refresh_time: u8,
    pub row_cycle_delay: u8,
    pub write_recovery_time: u8,
    pub rp_delay: u8,
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
    Normal = 0b000,
    ClockConfig = 0b001,
    PrechargeAll = 0b010,
    AutoRefresh = 0b011,
    LoadMode = 0b100,
    SelfRefresh = 0b101,
    PowerDown = 0b110,
}

/// SDRAM bank selection / SDRAM Bank 选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SdramBank {
    Bank1 = 1,
    Bank2 = 2,
    Both = 3,
}

impl Fmc {
    pub fn init_sdram(&self, bank: SdramBank, config: &SdramConfig, timing: &SdramTiming) {
        self.enable_clock();
        
        unsafe {
            let sdcr = match bank {
                SdramBank::Bank1 => (FMC_BASE + reg::SDCR1) as *mut u32,
                SdramBank::Bank2 => (FMC_BASE + reg::SDCR2) as *mut u32,
                _ => (FMC_BASE + reg::SDCR1) as *mut u32,
            };
            
            let mut val = 0;
            val |= ((config.column_bits - 8) as u32) << 0;
            val |= ((config.row_bits - 11) as u32) << 2;
            val |= (config.data_width as u32) << 4;
            val |= ((config.internal_bank / 2 - 1) as u32) << 6;
            val |= ((config.cas_latency - 1) as u32) << 7;
            val |= (config.write_protection as u32) << 9;
            val |= (config.read_burst as u32) << 12;
            val |= (config.read_pipe as u32) << 13;
            
            write_volatile(sdcr, val);

            let sdtr = match bank {
                SdramBank::Bank1 => (FMC_BASE + reg::SDTR1) as *mut u32,
                SdramBank::Bank2 => (FMC_BASE + reg::SDTR2) as *mut u32,
                _ => (FMC_BASE + reg::SDTR1) as *mut u32,
            };
            
            let mut val = 0;
            val |= (timing.load_to_active_delay as u32) << 0;
            val |= (timing.exit_self_refresh_delay as u32) << 4;
            val |= (timing.self_refresh_time as u32) << 8;
            val |= (timing.row_cycle_delay as u32) << 12;
            val |= (timing.write_recovery_time as u32) << 16;
            val |= (timing.rp_delay as u32) << 20;
            val |= (timing.rc_delay as u32) << 24;
            write_volatile(sdtr, val);
        }
    }

    pub fn send_sdram_command(&self, cmd: SdramBank, command: SdramCommand, bank: SdramBank) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            let mut val = command as u32;
            val |= ((bank as u32) & 0x03) << 3;
            write_volatile(sdcmr, val);
        }
    }

    pub fn set_sdram_refresh_rate(&self, rate: u16) {
        unsafe {
            let sdtr = (FMC_BASE + reg::SDRTR) as *mut u32;
            write_volatile(sdtr, (rate as u32) << 1);
        }
    }

    pub fn get_sdram_status(&self) -> u32 {
        unsafe {
            let sdsr = (FMC_BASE + reg::SDSR) as *mut u32;
            read_volatile(sdsr)
        }
    }

    pub fn is_sdram_busy(&self) -> bool {
        (self.get_sdram_status() & 0x01) != 0
    }

    pub fn read_sdram(&self, bank: SdramBank, addr: u32) -> u16 {
        unsafe {
            let base = match bank {
                SdramBank::Bank1 => FMC_SDRAM_BANK1_BASE,
                SdramBank::Bank2 => FMC_SDRAM_BANK2_BASE,
                _ => FMC_SDRAM_BANK1_BASE,
            };
            let ptr = (base + addr as usize) as *mut u16;
            read_volatile(ptr)
        }
    }

    pub fn write_sdram(&self, bank: SdramBank, addr: u32, data: u16) {
        unsafe {
            let base = match bank {
                SdramBank::Bank1 => FMC_SDRAM_BANK1_BASE,
                SdramBank::Bank2 => FMC_SDRAM_BANK2_BASE,
                _ => FMC_SDRAM_BANK1_BASE,
            };
            let ptr = (base + addr as usize) as *mut u16;
            write_volatile(ptr, data);
        }
    }

    pub fn write_sdram32(&self, bank: SdramBank, addr: u32, data: u32) {
        unsafe {
            let base = match bank {
                SdramBank::Bank1 => FMC_SDRAM_BANK1_BASE,
                SdramBank::Bank2 => FMC_SDRAM_BANK2_BASE,
                _ => FMC_SDRAM_BANK1_BASE,
            };
            let ptr = (base + addr as usize) as *mut u32;
            write_volatile(ptr, data);
        }
    }

    pub fn read_sdram32(&self, bank: SdramBank, addr: u32) -> u32 {
        unsafe {
            let base = match bank {
                SdramBank::Bank1 => FMC_SDRAM_BANK1_BASE,
                SdramBank::Bank2 => FMC_SDRAM_BANK2_BASE,
                _ => FMC_SDRAM_BANK1_BASE,
            };
            let ptr = (base + addr as usize) as *mut u32;
            read_volatile(ptr)
        }
    }

    pub fn sdram_clock_enable(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_CLK_CONFIG | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_precharge_all(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_PRECHARGE | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_auto_refresh(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_AUTOREFRESH | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_load_mode_register(&self, mode: u32) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            let val = sdcmr_bits::CMD_LOAD_MODE | sdcmr_bits::CTB1 | ((mode & 0xFFFF) << 5);
            write_volatile(sdcmr, val);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_enter_self_refresh(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_SELFREFRESH | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_exit_self_refresh(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_NORMAL | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_power_down(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_POWERDOWN | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_power_up(&self) {
        unsafe {
            let sdcmr = (FMC_BASE + reg::SDCMR) as *mut u32;
            write_volatile(sdcmr, sdcmr_bits::CMD_NORMAL | sdcmr_bits::CTB1);
            while (self.get_sdram_status() & sdsr_bits::BUSY) != 0 {}
        }
    }

    pub fn sdram_init_sequence(&self, config: &SdramConfig, timing: &SdramTiming) {
        self.enable_clock();
        
        unsafe {
            let sdcr = (FMC_BASE + reg::SDCR1) as *mut u32;
            let mut val = 0u32;
            val |= ((config.column_bits - 8) as u32) << 0;
            val |= ((config.row_bits - 11) as u32) << 2;
            val |= (config.data_width as u32) << 4;
            val |= if config.internal_bank == 4 { sdcr_bits::NB } else { 0 };
            val |= ((config.cas_latency - 1) as u32) << 7;
            val |= if config.write_protection { sdcr_bits::WP } else { 0 };
            val |= if config.read_burst { sdcr_bits::RBURST } else { 0 };
            val |= (config.read_pipe as u32) << 13;
            write_volatile(sdcr, val);

            let sdtr = (FMC_BASE + reg::SDTR1) as *mut u32;
            let mut val = 0u32;
            val |= ((timing.load_to_active_delay - 1) as u32) << 0;
            val |= ((timing.exit_self_refresh_delay - 1) as u32) << 4;
            val |= ((timing.self_refresh_time - 1) as u32) << 8;
            val |= ((timing.row_cycle_delay - 1) as u32) << 12;
            val |= ((timing.write_recovery_time - 1) as u32) << 16;
            val |= ((timing.rp_delay - 1) as u32) << 20;
            val |= ((timing.rc_delay - 1) as u32) << 24;
            write_volatile(sdtr, val);
        }

        self.sdram_clock_enable();
        self.sdram_precharge_all();
        
        for _ in 0..8 {
            self.sdram_auto_refresh();
        }
        
        let mode: u32 = 0x0230;
        self.sdram_load_mode_register(mode);
    }
}

// ============================================================================
// NAND Flash Support / NAND Flash 支持
// ============================================================================

/// NAND Bank selection / NAND Bank 选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NandBank {
    Bank2 = 2,
    Bank3 = 3,
}

/// NAND memory type / NAND 存储器类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NandMemoryType {
    Nand8 = 0,
    Nand16 = 1,
}

/// NAND ECC size / NAND ECC 大小
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NandEccSize {
    ECC4 = 0,
    ECC8 = 1,
}

/// NAND configuration / NAND 配置
#[derive(Clone, Copy, Debug)]
pub struct NandConfig {
    pub memory_type: NandMemoryType,
    pub ecc_size: NandEccSize,
    pub ecc_enable: bool,
    pub page_size: u8,
    pub wait_feature: bool,
    pub ce_control: bool,
}

impl Default for NandConfig {
    fn default() -> Self {
        Self {
            memory_type: NandMemoryType::Nand8,
            ecc_size: NandEccSize::ECC4,
            ecc_enable: true,
            page_size: 4,
            wait_feature: true,
            ce_control: true,
        }
    }
}

/// NAND timing configuration / NAND 时序配置
#[derive(Clone, Copy, Debug)]
pub struct NandTiming {
    pub mem_setup: u8,
    pub mem_wait: u8,
    pub mem_hold: u8,
    pub mem_hiz: u8,
    pub common_setup: u8,
    pub common_wait: u8,
    pub common_hold: u8,
    pub common_hiz: u8,
}

impl Default for NandTiming {
    fn default() -> Self {
        Self {
            mem_setup: 1,
            mem_wait: 3,
            mem_hold: 1,
            mem_hiz: 1,
            common_setup: 1,
            common_wait: 3,
            common_hold: 1,
            common_hiz: 1,
        }
    }
}

impl Fmc {
    pub fn init_nand(&self, bank: NandBank, config: &NandConfig, timing: &NandTiming) {
        self.enable_clock();
        
        let bank_idx = bank as usize;
        
        unsafe {
            let pcr = (FMC_BASE + reg::PCR2 + (bank_idx - 2) * 0x20) as *mut u32;
            let mut val = 0;
            val |= (config.wait_feature as u32) << 1;
            val |= 1 << 3;
            val |= (config.memory_type as u32) << 4;
            val |= (config.ecc_enable as u32) << 6;
            val |= (config.page_size as u32) << 17;
            write_volatile(pcr, val);
            
            let pmem = (FMC_BASE + reg::PMEM2 + (bank_idx - 2) * 0x20) as *mut u32;
            let mut val = 0;
            val |= (timing.mem_setup as u32) << 0;
            val |= (timing.mem_wait as u32) << 8;
            val |= (timing.mem_hold as u32) << 16;
            val |= (timing.mem_hiz as u32) << 24;
            write_volatile(pmem, val);
            
            let patt = (FMC_BASE + reg::PATT2 + (bank_idx - 2) * 0x20) as *mut u32;
            let mut val = 0;
            val |= (timing.common_setup as u32) << 0;
            val |= (timing.common_wait as u32) << 8;
            val |= (timing.common_hold as u32) << 16;
            val |= (timing.common_hiz as u32) << 24;
            write_volatile(patt, val);
        }
    }
    
    pub fn enable_nand_bank(&self, bank: NandBank) {
        let bank_idx = bank as usize;
        unsafe {
            let pcr = (FMC_BASE + reg::PCR2 + (bank_idx - 2) * 0x20) as *mut u32;
            let val = read_volatile(pcr);
            write_volatile(pcr, val | (1 << 0));
        }
    }
    
    pub fn disable_nand_bank(&self, bank: NandBank) {
        let bank_idx = bank as usize;
        unsafe {
            let pcr = (FMC_BASE + reg::PCR2 + (bank_idx - 2) * 0x20) as *mut u32;
            let val = read_volatile(pcr);
            write_volatile(pcr, val & !(1 << 0));
        }
    }
    
    pub fn get_nand_status(&self, bank: NandBank) -> u32 {
        let bank_idx = bank as usize;
        unsafe {
            let sr = (FMC_BASE + reg::SR2 + (bank_idx - 2) * 0x20) as *mut u32;
            read_volatile(sr)
        }
    }
    
    pub fn is_nand_ready(&self, bank: NandBank) -> bool {
        (self.get_nand_status(bank) & (1 << 6)) != 0
    }
    
    pub fn get_ecc(&self, bank: NandBank) -> u32 {
        let bank_idx = bank as usize;
        unsafe {
            let eccr = (FMC_BASE + reg::ECCR2 + (bank_idx - 2) * 0x20) as *const u32;
            read_volatile(eccr)
        }
    }
    
    pub fn read_nand(&self, bank: NandBank, addr: u32) -> u8 {
        unsafe {
            let base = FMC_NAND_BANK2_BASE + (bank as usize - 2) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u8;
            read_volatile(ptr)
        }
    }
    
    pub fn write_nand(&self, bank: NandBank, addr: u32, data: u8) {
        unsafe {
            let base = FMC_NAND_BANK2_BASE + (bank as usize - 2) * 0x0400_0000;
            let ptr = (base + addr as usize) as *mut u8;
            write_volatile(ptr, data);
        }
    }
}

// ============================================================================
// Convenience Functions / 便捷函数
// ============================================================================

pub fn config_nor_default() -> NorConfig {
    NorConfig::default()
}

pub fn config_sdram_default() -> SdramConfig {
    SdramConfig::default()
}

pub fn config_nand_default() -> NandConfig {
    NandConfig::default()
}

pub fn config_nor_16bit_sram() -> NorConfig {
    NorConfig {
        memory_type: MemoryType::Sram,
        data_width: DataWidth::Bits16,
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

pub fn config_nor_16bit_psram() -> NorConfig {
    NorConfig {
        memory_type: MemoryType::Psram,
        data_width: DataWidth::Bits16,
        burst_mode: BurstMode::Enabled,
        wait_signal_polarity: WaitSignalPolarity::ActiveLow,
        wait_config: WaitConfig::DuringWaitState,
        write_enable: true,
        wait_enable: true,
        ext_mode: ExtendedMode::Disabled,
        async_wait: false,
        write_burst: WriteBurst::Burst,
        continuous_clock: true,
    }
}

pub fn config_nor_sync_burst() -> NorConfig {
    NorConfig {
        memory_type: MemoryType::Nor,
        data_width: DataWidth::Bits16,
        burst_mode: BurstMode::Enabled,
        wait_signal_polarity: WaitSignalPolarity::ActiveLow,
        wait_config: WaitConfig::DuringWaitState,
        write_enable: true,
        wait_enable: true,
        ext_mode: ExtendedMode::Disabled,
        async_wait: false,
        write_burst: WriteBurst::Burst,
        continuous_clock: true,
    }
}

pub fn config_sdram_16bit_256mbit() -> SdramConfig {
    SdramConfig {
        column_bits: 9,
        row_bits: 12,
        data_width: DataWidth::Bits16,
        internal_bank: 4,
        cas_latency: 2,
        write_protection: false,
        read_burst: true,
        read_pipe: 0,
    }
}

pub fn config_sdram_timing_100mhz() -> SdramTiming {
    SdramTiming {
        load_to_active_delay: 2,
        exit_self_refresh_delay: 7,
        self_refresh_time: 4,
        row_cycle_delay: 7,
        write_recovery_time: 2,
        rp_delay: 2,
        rc_delay: 2,
    }
}

pub fn config_nand_8bit_512() -> NandConfig {
    NandConfig {
        memory_type: NandMemoryType::Nand8,
        ecc_size: NandEccSize::ECC4,
        ecc_enable: true,
        page_size: 4,
        wait_feature: true,
        ce_control: true,
    }
}

pub fn config_nand_timing_default() -> NandTiming {
    NandTiming {
        mem_setup: 1,
        mem_wait: 5,
        mem_hold: 1,
        mem_hiz: 1,
        common_setup: 1,
        common_wait: 5,
        common_hold: 1,
        common_hiz: 1,
    }
}

pub fn timing_100mhz_nor() -> Timing {
    Timing {
        address_setup: 1,
        address_hold: 1,
        data_setup: 2,
        bus_turn: 1,
        clk_div: 2,
        data_latency: 2,
        access_mode: 0,
    }
}

pub fn timing_60mhz_nor() -> Timing {
    Timing {
        address_setup: 1,
        address_hold: 1,
        data_setup: 3,
        bus_turn: 1,
        clk_div: 2,
        data_latency: 1,
        access_mode: 0,
    }
}
