//! USB OTG - USB On-The-Go Controller
//! USB 全速/高速接口控制器
//!
//! ## STM32U5 USB 特性 / Features
//! - **USB 控制器 / USB Controllers:**
//!   - USB OTG FS (Full Speed): 12 Mbps, 内置全速 PHY
//!   - USB OTG HS (High Speed): 480 Mbps, 内置高速 PHY (仅特定型号: STM32U59x/5Ax/5Fx/5Gx)
//!
//! - **工作模式 / Operating Modes:**
//!   - Device 模式 (外设模式)
//!   - Host 模式 (主机模式)
//!   - OTG (On-The-Go) 模式 - 双角色设备 (DRD)
//!
//! - **端点 / Endpoints:**
//!   - FS: 8 个双向端点
//!   - HS: 更多端点支持
//!
//! - **特性 / Features:**
//!   - 专用 FIFO (FS: 1.25KB)
//!   - DMA 支持
//!   - SOF 输出
//!   - 电源管理
//!   - 电池充电检测 (BCD)
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 72: USB on-the-go full-speed (OTG_FS)
//! - RM0456 Reference Manual, Chapter 73: USB on-the-go high-speed (OTG_HS)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

// ============================================================================
// USB OTG Base Addresses / USB OTG 基地址
// ============================================================================

/// USB OTG FS base address / USB OTG FS 基地址
/// AHB2 bus, accessible at 0x4204_0000
pub const USB_OTG_FS_BASE: usize = 0x4204_0000;

/// USB OTG HS base address / USB OTG HS 基地址 (仅特定型号)
/// AHB2 bus, accessible at 0x4204_4000
pub const USB_OTG_HS_BASE: usize = 0x4204_4000;

// ============================================================================
// USB OTG Global Core Registers / USB OTG 全局核心寄存器
// ============================================================================

/// USB OTG Global Core Registers / USB OTG 全局核心寄存器偏移
/// Reference: RM0456 Chapter 72.12 / 参考: RM0456 第72.12节
pub mod global_reg {
    /// OTG Control and Status Register / OTG 控制与状态寄存器
    /// Address offset: 0x000
    /// Reference: RM0456 Chapter 72.12.1 / 参考: RM0456 第72.12.1节
    pub const GOTGCTL: usize = 0x000;

    /// OTG Interrupt Register / OTG 中断寄存器
    /// Address offset: 0x004
    /// Reference: RM0456 Chapter 72.12.2 / 参考: RM0456 第72.12.2节
    pub const GOTGINT: usize = 0x004;

    /// OTG AHB Configuration Register / OTG AHB 配置寄存器
    /// Address offset: 0x008
    /// Reference: RM0456 Chapter 72.12.4 / 参考: RM0456 第72.12.4节
    pub const GAHBCFG: usize = 0x008;

    /// OTG USB Configuration Register / OTG USB 配置寄存器
    /// Address offset: 0x00C
    /// Reference: RM0456 Chapter 72.12.6 / 参考: RM0456 第72.12.6节
    pub const GUSBCFG: usize = 0x00C;

    /// OTG Reset Register / OTG 复位寄存器
    /// Address offset: 0x010
    /// Reference: RM0456 Chapter 72.12.8 / 参考: RM0456 第72.12.8节
    pub const GRSTCTL: usize = 0x010;

    /// OTG Core Interrupt Register / OTG 核心中断寄存器
    /// Address offset: 0x014
    pub const GINTSTS: usize = 0x014;

    /// OTG Interrupt Mask Register / OTG 中断屏蔽寄存器
    /// Address offset: 0x018
    pub const GINTMSK: usize = 0x018;

    /// OTG Receive Status Debug Read Register / OTG 接收状态调试读取寄存器
    /// Address offset: 0x01C
    pub const GRXSTSR: usize = 0x01C;

    /// OTG Receive Status Read/Pop Register / OTG 接收状态读取/弹出寄存器
    /// Address offset: 0x020
    pub const GRXSTSP: usize = 0x020;

    /// OTG Receive FIFO Size Register / OTG 接收 FIFO 大小寄存器
    /// Address offset: 0x024
    pub const GRXFSIZ: usize = 0x024;

    /// OTG Host Non-Periodic Transmit FIFO Size Register / OTG 主机非周期发送 FIFO 大小寄存器
    /// Address offset: 0x028
    pub const HNPTXFSIZ: usize = 0x028;

    /// OTG Endpoint 0 Transmit FIFO Size Register / OTG 端点 0 发送 FIFO 大小寄存器
    /// Address offset: 0x028
    pub const DIEPTXF0: usize = 0x028;

    /// OTG Non-Periodic Transmit FIFO/Queue Status Register / OTG 非周期发送 FIFO/队列状态寄存器
    /// Address offset: 0x02C
    pub const HNPTXSTS: usize = 0x02C;

    /// OTG General Core Configuration Register / OTG 通用核心配置寄存器
    /// Address offset: 0x038
    pub const GCCFG: usize = 0x038;

    /// OTG Core ID Register / OTG 核心 ID 寄存器
    /// Address offset: 0x03C
    pub const CID: usize = 0x03C;

    /// OTG Hardware Configuration Register 1 / OTG 硬件配置寄存器 1
    /// Address offset: 0x044
    pub const GHWCFG1: usize = 0x044;

    /// OTG Hardware Configuration Register 2 / OTG 硬件配置寄存器 2
    /// Address offset: 0x048
    pub const GHWCFG2: usize = 0x048;

    /// OTG Hardware Configuration Register 3 / OTG 硬件配置寄存器 3
    /// Address offset: 0x04C
    pub const GHWCFG3: usize = 0x04C;

    /// OTG Hardware Configuration Register 4 / OTG 硬件配置寄存器 4
    /// Address offset: 0x050
    pub const GHWCFG4: usize = 0x050;

    /// OTG Core LPM Configuration Register / OTG 核心 LPM 配置寄存器
    /// Address offset: 0x054
    pub const GLPMCFG: usize = 0x054;

    /// OTG Power Down Register / OTG 电源关闭寄存器
    /// Address offset: 0x058
    pub const GPWRDN: usize = 0x058;

    /// OTG Global DFIFO Configuration Register / OTG 全局 DFIFO 配置寄存器
    /// Address offset: 0x05C
    pub const GDFIFOCFG: usize = 0x05C;

    /// OTG Host Periodic Transmit FIFO Size Register / OTG 主机周期发送 FIFO 大小寄存器
    /// Address offset: 0x100
    pub const HPTXFSIZ: usize = 0x100;

    /// OTG Device IN Endpoint Transmit FIFO x Size Register / OTG 设备输入端点发送 FIFO x 大小寄存器
    /// Address offset: 0x104 + (x * 0x04)
    pub const DIEPTXF1: usize = 0x104;
    pub const DIEPTXF2: usize = 0x108;
    pub const DIEPTXF3: usize = 0x10C;
    pub const DIEPTXF4: usize = 0x110;
    pub const DIEPTXF5: usize = 0x114;
    pub const DIEPTXF6: usize = 0x118;
    pub const DIEPTXF7: usize = 0x11C;

    /// OTG Power and Clock Gating Control Register / OTG 电源和时钟门控控制寄存器
    /// Address offset: 0xE00
    pub const PCGCCTL: usize = 0xE00;
}

// ============================================================================
// USB OTG Device Mode Registers / USB OTG 设备模式寄存器
// ============================================================================

/// USB OTG Device Mode Registers / USB OTG 设备模式寄存器偏移
pub mod device_reg {
    /// OTG Device Configuration Register / OTG 设备配置寄存器
    /// Address offset: 0x800
    pub const DCFG: usize = 0x800;

    /// OTG Device Control Register / OTG 设备控制寄存器
    /// Address offset: 0x804
    pub const DCTL: usize = 0x804;

    /// OTG Device Status Register / OTG 设备状态寄存器
    /// Address offset: 0x808
    pub const DSTS: usize = 0x808;

    /// OTG Device IN Endpoint Common Interrupt Mask Register / OTG 设备输入端点公共中断屏蔽寄存器
    /// Address offset: 0x810
    pub const DIEPMSK: usize = 0x810;

    /// OTG Device OUT Endpoint Common Interrupt Mask Register / OTG 设备输出端点公共中断屏蔽寄存器
    /// Address offset: 0x814
    pub const DOEPMSK: usize = 0x814;

    /// OTG Device All Endpoints Interrupt Register / OTG 设备所有端点中断寄存器
    /// Address offset: 0x818
    pub const DAINT: usize = 0x818;

    /// OTG All Endpoints Interrupt Mask Register / OTG 所有端点中断屏蔽寄存器
    /// Address offset: 0x81C
    pub const DAINTMSK: usize = 0x81C;

    /// OTG Device VBUS Discharge Time Register / OTG 设备 VBUS 放电时间寄存器
    /// Address offset: 0x828
    pub const DVBUSDIS: usize = 0x828;

    /// OTG Device VBUS Pulsing Time Register / OTG 设备 VBUS 脉冲时间寄存器
    /// Address offset: 0x82C
    pub const DVBUSPULSE: usize = 0x82C;

    /// OTG Device Threshold Control Register / OTG 设备阈值控制寄存器
    /// Address offset: 0x830
    pub const DTHRCTL: usize = 0x830;

    /// OTG Device IN Endpoint FIFO Empty Interrupt Mask Register / OTG 设备输入端点 FIFO 空中断屏蔽寄存器
    /// Address offset: 0x834
    pub const DIEPEMPMSK: usize = 0x834;

    /// OTG Device Each Endpoint Interrupt Register / OTG 设备每个端点中断寄存器
    /// Address offset: 0x838
    pub const DEACHINT: usize = 0x838;

    /// OTG Device Each Endpoint Interrupt Mask Register / OTG 设备每个端点中断屏蔽寄存器
    /// Address offset: 0x83C
    pub const DEACHINTMSK: usize = 0x83C;

    /// OTG Device IN Endpoint 1 Control Register / OTG 设备输入端点 1 控制寄存器
    /// Address offset: 0x840
    pub const DIEPCTL1: usize = 0x840;
}

// ============================================================================
// USB OTG Endpoint Registers / USB OTG 端点寄存器
// ============================================================================

/// USB OTG Endpoint Registers / USB OTG 端点寄存器偏移
pub mod ep_reg {
    /// FIFO offset for endpoint / 端点的 FIFO 偏移
    pub const FIFO_BASE: usize = 0x1000;

    /// Maximum number of endpoints / 最大端点数
    pub const MAX_EP_COUNT: usize = 8;

    /// Device IN Endpoint Control Register (x = 0-7) / 设备输入端点控制寄存器
    pub const DIEPCTL: usize = 0x900;
    /// Device IN Endpoint Interrupt Register / 设备输入端点中断寄存器
    pub const DIEPINT: usize = 0x908;
    /// Device IN Endpoint Transfer Size Register / 设备输入端点传输大小寄存器
    pub const DIEPTSIZ: usize = 0x910;
    /// Device IN Endpoint DMA Address Register / 设备输入端点 DMA 地址寄存器
    pub const DIEPDMA: usize = 0x914;
    /// Device TX FIFO Status Register / 设备 TX FIFO 状态寄存器
    pub const DTXFSTS: usize = 0x918;

    /// Device OUT Endpoint Control Register (x = 0-7) / 设备输出端点控制寄存器
    pub const DOEPCTL: usize = 0xB00;
    /// Device OUT Endpoint Interrupt Register / 设备输出端点中断寄存器
    pub const DOEPINT: usize = 0xB08;
    /// Device OUT Endpoint Transfer Size Register / 设备输出端点传输大小寄存器
    pub const DOEPTSIZ: usize = 0xB10;
    /// Device OUT Endpoint DMA Address Register / 设备输出端点 DMA 地址寄存器
    pub const DOEPDMA: usize = 0xB14;
}

// ============================================================================
// USB OTG Host Mode Registers / USB OTG 主机模式寄存器
// ============================================================================

/// USB OTG Host Mode Registers / USB OTG 主机模式寄存器偏移
/// Reference: RM0456 Chapter 72.14 / 参考: RM0456 第72.14节
pub mod host_reg {
    /// OTG Host Configuration Register / OTG 主机配置寄存器
    /// Address offset: 0x400
    pub const HCFG: usize = 0x400;

    /// OTG Host Frame Interval Register / OTG 主机帧间隔寄存器
    /// Address offset: 0x404
    pub const HFIR: usize = 0x404;

    /// OTG Host Frame Number/Frame Time Remaining Register / OTG 主机帧号/帧剩余时间寄存器
    /// Address offset: 0x408
    pub const HFNUM: usize = 0x408;

    /// OTG Host Periodic Transmit FIFO/Queue Status Register / OTG 主机周期发送 FIFO/队列状态寄存器
    /// Address offset: 0x410
    pub const HPTXSTS: usize = 0x410;

    /// OTG Host All Channels Interrupt Register / OTG 主机所有通道中断寄存器
    /// Address offset: 0x414
    pub const HAINT: usize = 0x414;

    /// OTG Host All Channels Interrupt Mask Register / OTG 主机所有通道中断屏蔽寄存器
    /// Address offset: 0x418
    pub const HAINTMSK: usize = 0x418;

    /// OTG Host Port Control and Status Register / OTG 主机端口控制与状态寄存器
    /// Address offset: 0x440
    pub const HPRT: usize = 0x440;

    /// OTG Host Channel x Characteristics Register (x = 0-11) / OTG 主机通道 x 特性寄存器
    /// Address offset: 0x500 + (x * 0x20)
    pub const HCCHAR: usize = 0x500;

    /// OTG Host Channel x Interrupt Register / OTG 主机通道 x 中断寄存器
    /// Address offset: 0x508 + (x * 0x20)
    pub const HCINT: usize = 0x508;

    /// OTG Host Channel x Interrupt Mask Register / OTG 主机通道 x 中断屏蔽寄存器
    /// Address offset: 0x50C + (x * 0x20)
    pub const HCINTMSK: usize = 0x50C;

    /// OTG Host Channel x Transfer Size Register / OTG 主机通道 x 传输大小寄存器
    /// Address offset: 0x510 + (x * 0x20)
    pub const HCTSIZ: usize = 0x510;

    /// OTG Host Channel x DMA Address Register / OTG 主机通道 x DMA 地址寄存器
    /// Address offset: 0x514 + (x * 0x20)
    pub const HCDMA: usize = 0x514;
}

// ============================================================================
// USB Register Bit Definitions / USB 寄存器位定义
// ============================================================================

/// GAHBCFG register bits / GAHBCFG 寄存器位
/// Reference: RM0456 Chapter 72.12.4 / 参考: RM0456 第72.12.4节
pub mod gahbcfg_bits {
    pub const GINT: u32 = 1 << 0;          /// Global interrupt mask / 全局中断屏蔽
    pub const TXFELVL: u32 = 1 << 7;       /// TX FIFO empty level / TX FIFO 空级别
    pub const PTXFELVL: u32 = 1 << 8;      /// Periodic TX FIFO empty level / 周期 TX FIFO 空级别
    pub const DMAEN: u32 = 1 << 5;          /// DMA enable / DMA 使能
    pub const HBSTLEN_SHIFT: u32 = 1;      /// Burst length / 突发长度
    pub const HBSTLEN_MASK: u32 = 0xF << 1; /// Burst length mask / 突发长度掩码
}

/// GUSBCFG register bits / GUSBCFG 寄存器位
/// Reference: RM0456 Chapter 72.12.6 / 参考: RM0456 第72.12.6节
pub mod gusbcfg_bits {
    pub const FDMOD: u32 = 1 << 30;        /// Force device mode / 强制设备模式
    pub const FHMOD: u32 = 1 << 29;        /// Force host mode / 强制主机模式
    pub const TRTIM: u32 = 1 << 29;        /// Reserved, must be 1 / 保留，必须为 1
    pub const HNPEN: u32 = 1 << 9;         /// HNP enable / HNP 使能
    pub const SRPEN: u32 = 1 << 8;         /// SRP enable / SRP 使能
    pub const PHY16: u32 = 1 << 7;         /// PHY 16-bit interface / PHY 16 位接口
    pub const TOCAL_SHIFT: u32 = 0;        /// FS timeout calibration shift / FS 超时校准位移
    pub const TOCAL_MASK: u32 = 0b111 << 0; /// FS timeout calibration mask / FS 超时校准掩码
    pub const SRPCAP: u32 = 1 << 8;        /// SRP capability / SRP 能力
    pub const ULPIFSLS: u32 = 1 << 17;     /// ULPI FS/LS select / ULPI FS/LS 选择
    pub const ULPIAR: u32 = 1 << 18;       /// ULPI Auto Resume / ULPI 自动恢复
    pub const ULPIEVBUSD: u32 = 1 << 19;    /// ULPI External VBUS Drive / ULPI 外部 VBUS 驱动
    pub const ULPIEVBUSI: u32 = 1 << 20;    /// ULPI External VBUS Indicator / ULPI 外部 VBUS 指示
    pub const TERM125SEL: u32 = 1 << 27;   /// Term 125 selection / 终端 125 选择
}

/// GRSTCTL register bits / GRSTCTL 寄存器位
/// Reference: RM0456 Chapter 72.12.8 / 参考: RM0456 第72.12.8节
pub mod grstctl_bits {
    pub const CSRST: u32 = 1 << 0;         /// Core soft reset / 核心软复位
    pub const PSRST: u32 = 1 << 1;         /// Peripheral soft reset / 外设软复位
    pub const FCRST: u32 = 1 << 4;         /// Host all channels reset / 主机所有通道复位
    pub const RXFFLSH: u32 = 1 << 4;       /// Flush receive FIFO / 刷新接收 FIFO
    pub const TXFFLSH: u32 = 1 << 5;       /// Flush transmit FIFO / 刷新发送 FIFO
    pub const TXFNUM_SHIFT: u32 = 6;       /// TX FIFO number shift / TX FIFO 编号位移
    pub const TXFNUM_MASK: u32 = 0x1F << 6; /// TX FIFO number mask / TX FIFO 编号掩码
    pub const AHBIDL: u32 = 1 << 31;        /// AHB master idle / AHB 主机空闲
    pub const DMAREQ: u32 = 1 << 30;        /// DMA request signal / DMA 请求信号
}

/// GOTGCTL register bits / GOTGCTL 寄存器位
/// Reference: RM0456 Chapter 72.12.1 / 参考: RM0456 第72.12.1节
pub mod gotgctl_bits {
    pub const SRQ: u32 = 1 << 0;           /// Session request / 会话请求
    pub const SRSS: u32 = 1 << 1;          /// Session request success / 会话请求成功
    pub const HNGSCS: u32 = 1 << 8;        /// Host negotiation success / 主机协商成功
    pub const HNPRQ: u32 = 1 << 9;         /// Host negotiation request / 主机协商请求
    pub const HSHA: u32 = 1 << 10;         /// Host set HNP enable / 主机设置 HNP 使能
    pub const DHNPEN: u32 = 1 << 10;       /// Device HNP enable / 设备 HNP 使能
    pub const CGOUTNAK: u32 = 1 << 7;       /// Clear global OUT NAK / 清除全局 OUT NAK
    pub const CGINAK: u32 = 1 << 8;        /// Clear global IN NAK / 清除全局 IN NAK
    pub const SGINAK: u32 = 1 << 7;        /// Set global IN NAK / 设置全局 IN NAK
    pub const SGOUTNAK: u32 = 1 << 7;       /// Set global OUT NAK / 设置全局 OUT NAK
}

/// GINTSTS/GINTMSK register bits / GINTSTS/GINTMSK 寄存器位
/// Reference: RM0456 Chapter 72.12.3 / 参考: RM0456 第72.12.3节
pub mod gint_bits {
    pub const CMOD: u32 = 1 << 0;              /// Current mode / 当前模式 (0: device, 1: host)
    pub const MISOTGINT: u32 = 1 << 1;        /// Mode mismatch interrupt / 模式不匹配中断
    pub const HPRTINT: u32 = 1 << 3;           /// Host port interrupt / 主机端口中断
    pub const RXFLVL: u32 = 1 << 4;            /// Receive FIFO non-empty / 接收 FIFO 非空
    pub const SOFO: u32 = 1 << 3;             /// Start of frame / 帧开始
    pub const GINNAKEFF: u32 = 1 << 6;         /// IN endpoint NAK effective / 输入端点 NAK 生效
    pub const GOUTNAKEFF: u32 = 1 << 7;       /// OUT endpoint NAK effective / 输出端点 NAK 生效
    pub const USBRST: u32 = 1 << 12;           /// USB reset / USB 复位
    pub const ENUMDNE: u32 = 1 << 13;         /// Speed enumeration done / 速度枚举完成
    pub const ISOODRP: u32 = 1 << 14;         /// Isochronous OUT packet dropped / 同步 OUT 包丢失
    pub const EOPF: u32 = 1 << 15;             /// End of periodic frame / 周期帧结束
    pub const IEPINT: u32 = 1 << 18;           /// IN endpoint interrupt / 输入端点中断
    pub const OEPINT: u32 = 1 << 19;          /// OUT endpoint interrupt / 输出端点中断
    pub const ISOINC: u32 = 1 << 20;           /// Incomplete isochronous IN transfer / 不完整同步 IN 传输
    pub const INCOMPISOOUT: u32 = 1 << 21;    /// Incomplete isochronous OUT transfer / 不完整同步 OUT 传输
    pub const FETCH: u32 = 1 << 22;            /// Data fetch / 数据获取
    pub const RXSTSFULL: u32 = 1 << 23;       /// Receive status queue full / 接收状态队列满
    pub const PSTUPIDLE: u32 = 1 << 24;        /// Host idle / 主机空闲
    pub const HCHINT: u32 = 1 << 25;           /// Host channel interrupt / 主机通道中断
    pub const DISCONNINT: u32 = 1 << 29;      /// Disconnect detected interrupt / 检测到断开中断
    pub const SESREQDONE: u32 = 1 << 30;      /// Session request done / 会话请求完成
}

/// GCCFG register bits / GCCFG 寄存器位
/// Reference: RM0456 Chapter 72.12.7 / 参考: RM0456 第72.12.7节
pub mod gccfg_bits {
    pub const PWRDWN: u32 = 1 << 16;        /// Power down / 电源关闭 (0: active, 1: power down)
    pub const VBUSBSEN: u32 = 1 << 19;      /// VBUS B-sens enable / VBUS B 传感使能
    pub const VBUSASEN: u32 = 1 << 18;      /// VBUS A-sens enable / VBUS A 传感使能
    pub const SOFOEN: u32 = 1 << 20;        /// SOF output enable / SOF 输出使能
    pub const NOUT0: u32 = 1 << 21;         /// Number of OUT endpoints 0 / OUT 端点 0 编号
    pub const I2CIPOEN: u32 = 1 << 22;       /// I2C interface pad only / 仅 I2C 接口引脚
    pub const I2CEN: u32 = 1 << 23;          /// I2C enable / I2C 使能
    pub const FORCEDEVMODE: u32 = 1 << 28;  /// Forced device mode / 强制设备模式
    pub const FORCEDHOSTMODE: u32 = 1 << 29; /// Forced host mode / 强制主机模式
    pub const CMPOUT: u32 = 1 << 30;         /// Comparator output / 比较器输出
}

/// DCFG register bits / DCFG 寄存器位
/// Reference: RM0456 Chapter 72.13.1 / 参考: RM0456 第72.13.1节
pub mod dcfg_bits {
    pub const DSPD_SHIFT: u32 = 0;           /// Device speed shift / 设备速度位移
    pub const DSPD_MASK: u32 = 0b11 << 0;    /// Device speed mask / 设备速度掩码
    pub const DSPD_FS: u32 = 0b11 << 0;      /// Full-speed / 全速
    pub const DSPD_LS: u32 = 0b10 << 0;      /// Low-speed / 低速
    pub const DSPD_HS: u32 = 0b00 << 0;      /// High-speed / 高速

    pub const DAD_SHIFT: u32 = 4;            /// Device address shift / 设备地址位移
    pub const DAD_MASK: u32 = 0x7F << 4;     /// Device address mask / 设备地址掩码

    pub const NZLSOHSK: u32 = 1 << 9;        /// Non-zero-length status OUT handshake / 非零长度状态 OUT 握手

    pub const PERFRINT_SHIFT: u32 = 11;      /// Periodic frame interval shift / 周期帧间隔位移
    pub const PERFRINT_MASK: u32 = 0b11 << 11; /// Periodic frame interval mask / 周期帧间隔掩码
}

/// DCTL register bits / DCTL 寄存器位
pub mod dctl_bits {
    pub const RWUSIG: u32 = 1 << 0;          /// Remote wakeup signaling / 远程唤醒信号
    pub const SDIS: u32 = 1 << 1;           /// Soft disconnect / 软断开
    pub const GINSTS: u32 = 1 << 2;         /// Global IN NAK status / 全局输入 NAK 状态
    pub const GONSTS: u32 = 1 << 3;          /// Global OUT NAK status / 全局输出 NAK 状态
    pub const TCTL: u32 = 0b111 << 4;        /// Test control / 测试控制
    pub const SGINAK: u32 = 1 << 7;          /// Set global IN NAK / 设置全局输入 NAK
    pub const CGINAK: u32 = 1 << 8;          /// Clear global IN NAK / 清除全局输入 NAK
    pub const SGONAK: u32 = 1 << 9;          /// Set global OUT NAK / 设置全局输出 NAK
    pub const CGONAK: u32 = 1 << 10;         /// Clear global OUT NAK / 清除全局输出 NAK
    pub const POMODE: u32 = 1 << 11;         /// Power-on mode / 上电模式
}

/// DSTS register bits / DSTS 寄存器位
pub mod dsts_bits {
    pub const SUSPSTS: u32 = 1 << 0;         /// Suspend status / 挂起状态
    pub const ENUMSPD_SHIFT: u32 = 2;       /// Enumerated speed shift / 枚举速度位移
    pub const ENUMSPD_MASK: u32 = 0b11 << 2; /// Enumerated speed mask / 枚举速度掩码
    pub const ERRATICERR: u32 = 1 << 5;      /// Erratic error / 错误错误
    pub const FNRSOF_SHIFT: u32 = 8;         /// Frame number of SOF shift / SOF 帧号位移
    pub const FNRSOF_MASK: u32 = 0x3FFF << 8; /// Frame number of SOF mask / SOF 帧号掩码
}

/// DIEPCTL/DOEPCTL register bits / DIEPCTL/DOEPCTL 寄存器位
pub mod epctl_bits {
    pub const EPENA: u32 = 1 << 31;         /// Endpoint enable / 端点使能
    pub const EPDIS: u32 = 1 << 30;          /// Endpoint disable / 端点禁用
    pub const SODDFRM: u32 = 1 << 29;        /// Set odd frame / 设置奇数帧
    pub const SDBUTF: u32 = 1 << 28;        /// Set double buffer / 设置双缓冲
    pub const SERRQSTS: u32 = 1 << 27;       /// Stall / 暂停
    pub const STALL: u32 = 1 << 21;          /// STALL handshake / STALL 握手
    pub const TXFNUM_SHIFT: u32 = 22;        /// TX FIFO number shift / TX FIFO 编号位移
    pub const TXFNUM_MASK: u32 = 0xF << 22;  /// TX FIFO number mask / TX FIFO 编号掩码
    pub const CNAK: u32 = 1 << 26;           /// Clear NAK / 清除 NAK
    pub const SNPM: u32 = 1 << 20;           /// Snoop mode / 监听模式
    pub const EPTYP_SHIFT: u32 = 18;         /// Endpoint type shift / 端点类型位移
    pub const EPTYP_MASK: u32 = 0b11 << 18;  /// Endpoint type mask / 端点类型掩码
    pub const EPTYP_CONTROL: u32 = 0b00 << 18; /// Control endpoint / 控制端点
    pub const EPTYP_ISO: u32 = 0b01 << 18;   /// Isochronous endpoint / 同步端点
    pub const EPTYP_BULK: u32 = 0b10 << 18;  /// Bulk endpoint / 批量端点
    pub const EPTYP_INTERRUPT: u32 = 0b11 << 18; /// Interrupt endpoint / 中断端点
    pub const USBAEP: u32 = 1 << 15;         /// USB endpoint active / USB 端点激活
    pub const MPSIZ_SHIFT: u32 = 0;          /// Maximum packet size shift / 最大包大小位移
    pub const MPSIZ_MASK: u32 = 0x7FF << 0;  /// Maximum packet size mask / 最大包大小掩码
}

/// DIEPINT/DOEPINT register bits / DIEPINT/DOEPINT 寄存器位
pub mod epint_bits {
    pub const XFERCMP: u32 = 1 << 0;         /// Transfer complete / 传输完成
    pub const EPDISD: u32 = 1 << 1;          /// Endpoint disabled / 端点禁用
    pub const TOGERR: u32 = 1 << 3;          /// Toggle error / 翻转错误
    pub const INTKNEPTXF: u32 = 1 << 4;      /// IN endpoint NAK effective / 输入端点 NAK 生效
    pub const INTKNEPTXFM: u32 = 1 << 5;    /// IN token received when TXF empty / TXF 空时收到 IN token
    pub const BNAINTR: u32 = 1 << 7;         /// Buffer not available / 缓冲区不可用
    pub const PINGERR: u32 = 1 << 12;        /// PING error / PING 错误
    pub const DATATGLERR: u32 = 1 << 13;     /// Data toggle error / 数据翻转错误
    pub const BBLERR: u32 = 1 << 12;         /// Babble error / 婴儿错误
}

/// DIEPTSIZ/DOEPTSIZ register bits / DIEPTSIZ/DOEPTSIZ 寄存器位
pub mod eptsiz_bits {
    pub const XFRSIZ_SHIFT: u32 = 0;         /// Transfer size shift / 传输大小位移
    pub const XFRSIZ_MASK: u32 = 0x7FFFF << 0; /// Transfer size mask / 传输大小掩码
    pub const PKTCNT_SHIFT: u32 = 19;       /// Packet count shift / 包计数位移
    pub const PKTCNT_MASK: u32 = 0x3FF << 19; /// Packet count mask / 包计数掩码
    pub const STUPCNT_SHIFT: u32 = 29;       /// Setup packet count shift / 设置包计数位移
    pub const STUPCNT_MASK: u32 = 0x3 << 29;  /// Setup packet count mask / 设置包计数掩码
}

/// RX Status Packet / 接收状态包
/// Reference: RM0456 Chapter 72.12.5 / 参考: RM0456 第72.12.5节
pub mod rxsts_bits {
    pub const PKTSTS_SHIFT: u32 = 17;        /// Packet status shift / 包状态位移
    pub const PKTSTS_MASK: u32 = 0xF << 17;  /// Packet status mask / 包状态掩码
    pub const PKTSTS_OUT: u32 = 0x02 << 17;  /// OUT data packet received / 收到 OUT 数据包
    pub const PKTSTS_OUT_COMP: u32 = 0x03 << 17; /// OUT transfer completed / OUT 传输完成
    pub const PKTSTS_SETUP: u32 = 0x04 << 17; /// SETUP transaction completed / SETUP 事务完成
    pub const PKTSTS_SETUP_COMP: u32 = 0x06 << 17; /// SETUP transfer completed / SETUP 传输完成
    pub const DFNEOF: u32 = 0x05 << 17;       /// Data frame / 数据帧
    pub const PKTSTS_HCTSIZ: u32 = 0x03 << 17; /// HCTSIZ packet / HCTSIZ 包
    pub const DPID_SHIFT: u32 = 15;          /// Data PID shift / 数据 PID 位移
    pub const DPID_MASK: u32 = 0x3 << 15;     /// Data PID mask / 数据 PID 掩码
    pub const BCNT_SHIFT: u32 = 4;            /// Byte count shift / 字节计数位移
    pub const BCNT_MASK: u32 = 0x7FF << 4;   /// Byte count mask / 字节计数掩码
    pub const EPNUM_SHIFT: u32 = 0;          /// Endpoint number shift / 端点编号位移
    pub const EPNUM_MASK: u32 = 0xF << 0;    /// Endpoint number mask / 端点编号掩码
}

/// Host Port Control and Status Register bits / 主机端口控制与状态寄存器位
/// Reference: RM0456 Chapter 72.14.10 / 参考: RM0456 第72.14.10节
pub mod hprt_bits {
    pub const PCSTS: u32 = 1 << 0;           /// Port connect status / 端口连接状态
    pub const PCDET: u32 = 1 << 1;           /// Port connect detected / 端口连接检测
    pub const PENA: u32 = 1 << 2;             /// Port enable / 端口使能
    pub const PEMST: u32 = 1 << 3;            /// Port enable / 端口使能 (master)
    pub const POCCHG: u32 = 1 << 4;           /// Port overcurrent change / 端口过流变化
    pub const POCA: u32 = 1 << 5;             /// Port overcurrent active / 端口过流激活
    pub const PFE: u32 = 1 << 6;              /// Port folding enable / 端口折叠使能
    pub const PSPD_SHIFT: u32 = 17;           /// Port speed shift / 端口速度位移
    pub const PSPD_MASK: u32 = 0x3 << 17;     /// Port speed mask / 端口速度掩码
    pub const PSPD_FS: u32 = 0x1 << 17;       /// Full-speed / 全速
    pub const PSPD_LS: u32 = 0x2 << 17;       /// Low-speed / 低速
    pub const PSPD_HS: u32 = 0x0 << 17;       /// High-speed / 高速
    pub const PTCTL_SHIFT: u32 = 20;          /// Port test control shift / 端口测试控制位移
    pub const PTCTL_MASK: u32 = 0xF << 20;    /// Port test control mask / 端口测试控制掩码
    pub const PPWR: u32 = 1 << 12;            /// Port power / 端口电源
    pub const PRST: u32 = 1 << 8;             /// Port reset / 端口复位
    pub const PLSTS_SHIFT: u32 = 10;           /// Port line status shift / 端口线路状态位移
    pub const PLSTS_MASK: u32 = 0x3 << 10;    /// Port line status mask / 端口线路状态掩码
}

/// Host Channel Characteristics Register bits / 主机通道特性寄存器位
/// Reference: RM0456 Chapter 72.14.5 / 参考: RM0456 第72.14.5节
pub mod hcchar_bits {
    pub const MPSIZ_SHIFT: u32 = 0;           /// Max packet size shift / 最大包大小位移
    pub const MPSIZ_MASK: u32 = 0x7FF << 0;   /// Max packet size mask / 最大包大小掩码
    pub const EPNUM_SHIFT: u32 = 11;          /// Endpoint number shift / 端点编号位移
    pub const EPNUM_MASK: u32 = 0xF << 11;    /// Endpoint number mask / 端点编号掩码
    pub const EPDIR: u32 = 1 << 15;           /// Endpoint direction / 端点方向 (0: OUT, 1: IN)
    pub const EPTYP_SHIFT: u32 = 18;           /// Endpoint type shift / 端点类型位移
    pub const EPTYP_MASK: u32 = 0x3 << 18;     /// Endpoint type mask / 端点类型掩码
    pub const EPTYP_CTRL: u32 = 0x0 << 18;    /// Control / 控制
    pub const EPTYP_ISO: u32 = 0x1 << 18;     /// Isochronous / 同步
    pub const EPTYP_BULK: u32 = 0x2 << 18;     /// Bulk / 批量
    pub const EPTYP_INTR: u32 = 0x3 << 18;     /// Interrupt / 中断
    pub const DAD_SHIFT: u32 = 22;             /// Device address shift / 设备地址位移
    pub const DAD_MASK: u32 = 0x7F << 22;     /// Device address mask / 设备地址掩码
    pub const ODDFRM: u32 = 1 << 29;          /// Odd frame / 奇数帧
    pub const CHDIS: u32 = 1 << 30;            /// Channel disable / 通道禁用
    pub const CHENA: u32 = 1 << 31;           /// Channel enable / 通道使能
}

/// Host Channel Interrupt Register bits / 主机通道中断寄存器位
/// Reference: RM0456 Chapter 72.14.7 / 参考: RM0456 第72.14.7节
pub mod hcint_bits {
    pub const XFERCMP: u32 = 1 << 0;           /// Transfer complete / 传输完成
    pub const CHH: u32 = 1 << 1;               /// Channel halted / 通道停止
    pub const STALL: u32 = 1 << 3;             /// STALL received / 收到 STALL
    pub const NAK: u32 = 1 << 4;               /// NAK received / 收到 NAK
    pub const ACK: u32 = 1 << 5;               /// ACK received / 收到 ACK
    pub const TXERR: u32 = 1 << 7;             /// Transaction error / 事务错误
    pub const BBERR: u32 = 1 << 8;             /// Babble error / 婴儿错误
    pub const FRMOR: u32 = 1 << 9;             /// Frame overrun / 帧溢出
    pub const DATATGLERR: u32 = 1 << 10;        /// Data toggle error / 数据翻转错误
}

/// Host Configuration Register bits / 主机配置寄存器位
/// Reference: RM0456 Chapter 72.14.1 / 参考: RM0456 第72.14.1节
pub mod hcfg_bits {
    pub const FSLSPCS_SHIFT: u32 = 0;          /// FS/LS PHY clock select shift / FS/LS PHY 时钟选择位移
    pub const FSLSPCS_MASK: u32 = 0x3 << 0;     /// FS/LS PHY clock select mask / FS/LS PHY 时钟选择掩码
    pub const FSLSPCS_48MHz: u32 = 0x1 << 0;   /// 48 MHz / 48 MHz
    pub const FSLSPCS_6MHz: u32 = 0x2 << 0;    /// 6 MHz / 6 MHz
    pub const FSLSSUP: u32 = 1 << 2;           /// FS/LS only support / 仅支持 FS/LS
}

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// USB Speed / USB 速度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UsbSpeed {
    /// High Speed (480 Mbps) / 高速 (480 Mbps)
    High = 0,
    /// Full Speed (12 Mbps) / 全速 (12 Mbps)
    Full = 3,
    /// Low Speed (1.5 Mbps) / 低速 (1.5 Mbps)
    Low = 2,
}

/// USB Endpoint Type / USB 端点类型
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EpType {
    /// Control endpoint / 控制端点
    Control = 0,
    /// Isochronous endpoint / 同步端点
    Isochronous = 1,
    /// Bulk endpoint / 批量端点
    Bulk = 2,
    /// Interrupt endpoint / 中断端点
    Interrupt = 3,
}

/// USB Endpoint Direction / USB 端点方向
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EpDirection {
    /// OUT (host to device) / OUT (主机到设备)
    Out = 0,
    /// IN (device to host) / IN (设备到主机)
    In = 1,
}

/// USB Device State / USB 设备状态
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceState {
    /// Default state / 默认状态
    Default,
    /// Addressed state / 寻址状态
    Addressed,
    /// Configured state / 配置状态
    Configured,
    /// Suspended state / 挂起状态
    Suspended,
}

/// USB OTG Mode / USB OTG 模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OtgMode {
    /// Device mode / 设备模式
    Device,
    /// Host mode / 主机模式
    Host,
    /// OTG dual-role mode / OTG 双角色模式
    Otg,
}

/// USB Packet / USB 数据包
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UsbPacketStatus {
    /// OUT data packet received / 收到 OUT 数据包
    OutData = 2,
    /// OUT transfer completed / OUT 传输完成
    OutComplete = 3,
    /// SETUP transaction completed / SETUP 事务完成
    SetupComplete = 4,
    /// SETUP transfer completed / SETUP 传输完成
    SetupTransferComplete = 6,
    /// Data toggle error / 数据翻转错误
    DataToggleError = 10,
}

/// USB Host Port Status / USB 主机端口状态
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HostPortSpeed {
    /// High Speed / 高速
    High,
    /// Full Speed / 全速
    Full,
    /// Low Speed / 低速
    Low,
}

// ============================================================================
// Data Structures / 数据结构
// ============================================================================

/// USB Setup Packet / USB 设置包
/// Standard USB setup packet structure / 标准 USB 设置包结构
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SetupPacket {
    /// bmRequestType: Characteristics of request / 请求特性
    pub bm_request_type: u8,
    /// bRequest: Specific request / 特定请求
    pub b_request: u8,
    /// wValue: Field that varies according to request / 根据请求变化的字段
    pub w_value: u16,
    /// wIndex: Field that varies according to request / 根据请求变化的字段
    pub w_index: u16,
    /// wNumber of bytes to transfer / 要传输的字节数
    pub w_length: u16,
}

/// USB Endpoint Configuration / USB 端点配置
#[derive(Clone, Copy, Debug)]
pub struct EpConfig {
    /// Endpoint number / 端点编号 (0-7)
    pub ep_num: u8,
    /// Endpoint direction / 端点方向
    pub direction: EpDirection,
    /// Endpoint type / 端点类型
    pub ep_type: EpType,
    /// Maximum packet size / 最大包大小
    pub max_packet_size: u16,
    /// TX FIFO number (for IN endpoints) / TX FIFO 编号 (用于输入端点)
    pub tx_fifo_num: u8,
}

/// USB Configuration / USB 配置
#[derive(Clone, Copy, Debug)]
pub struct UsbConfig {
    /// USB speed / USB 速度
    pub speed: UsbSpeed,
    /// Device address (for device mode) / 设备地址 (用于设备模式)
    pub device_address: u8,
    /// RX FIFO size (in words) / RX FIFO 大小 (以字为单位)
    pub rx_fifo_size: u16,
    /// TX FIFO size for each endpoint (in words) / 每个端点的 TX FIFO 大小 (以字为单位)
    pub tx_fifo_sizes: [u16; 8],
    /// Enable DMA / 使能 DMA
    pub dma_enabled: bool,
}

impl Default for UsbConfig {
    fn default() -> Self {
        UsbConfig {
            speed: UsbSpeed::Full,
            device_address: 0,
            rx_fifo_size: 128,  // 512 bytes
            tx_fifo_sizes: [64, 64, 64, 64, 64, 64, 64, 64], // 256 bytes each
            dma_enabled: false,
        }
    }
}

// ============================================================================
// USB OTG Driver / USB OTG 驱动
// ============================================================================

/// USB OTG Driver / USB OTG 驱动
pub struct UsbOtg {
    /// Base address / 基地址
    base: usize,
}

impl UsbOtg {
    /// Create new USB OTG instance / 创建新的 USB OTG 实例
    /// 
    /// # Arguments
    /// * `base` - USB OTG base address / USB OTG 基地址
    pub const fn new(base: usize) -> Self {
        UsbOtg { base }
    }

    /// Create USB OTG FS instance / 创建 USB OTG FS 实例
    pub const fn new_fs() -> Self {
        UsbOtg { base: USB_OTG_FS_BASE }
    }

    /// Create USB OTG HS instance / 创建 USB OTG HS 实例
    pub const fn new_hs() -> Self {
        UsbOtg { base: USB_OTG_HS_BASE }
    }

    /// Get register address / 获取寄存器地址
    #[inline]
    fn reg(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    /// Read register / 读寄存器
    #[inline]
    fn read(&self, offset: usize) -> u32 {
        unsafe { read_volatile(self.reg(offset)) }
    }

    /// Write register / 写寄存器
    #[inline]
    fn write(&self, offset: usize, value: u32) {
        unsafe { write_volatile(self.reg(offset), value) }
    }

    /// Read modify write register / 读修改写寄存器
    #[inline]
    fn modify(&self, offset: usize, clear: u32, set: u32) {
        let value = self.read(offset);
        self.write(offset, (value & !clear) | set);
    }

    /// Enable USB clock / 使能 USB 时钟
    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr |= 1 << 12; // USBOTGFSEN
        }
    }

    /// Disable USB clock / 禁用 USB 时钟
    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr &= !(1 << 12);
        }
    }

    /// Initialize USB OTG peripheral / 初始化 USB OTG 外设
    /// 
    /// # Arguments
    /// * `config` - USB configuration / USB 配置
    pub fn init(&self, config: &UsbConfig) {
        // Wait for AHB master idle
        while (self.read(global_reg::GRSTCTL) & grstctl_bits::AHBIDL) == 0 {}

        // Core soft reset
        self.modify(global_reg::GRSTCTL, 0, grstctl_bits::CSRST);
        while (self.read(global_reg::GRSTCTL) & grstctl_bits::CSRST) != 0 {}

        // Wait for AHB master idle again
        while (self.read(global_reg::GRSTCTL) & grstctl_bits::AHBIDL) == 0 {}

        // Configure USB frequency
        self.modify(global_reg::GUSBCFG, 0, 0 << 0); // 48 MHz PHY clock

        // Enable internal PHY (clear PWRDWN to power up)
        self.modify(global_reg::GCCFG, gccfg_bits::PWRDWN, 0);

        // Configure RX FIFO size
        self.write(global_reg::GRXFSIZ, config.rx_fifo_size);

        // Configure TX FIFO sizes
        let mut tx_fifo_base = config.rx_fifo_size;
        for i in 0..8 {
            let offset = global_reg::DIEPTXF1 + i * 4;
            let size = if i < config.tx_fifo_sizes.len() {
                config.tx_fifo_sizes[i]
            } else {
                64
            };
            self.write(offset, (size << 16) | tx_fifo_base);
            tx_fifo_base += size;
        }

        // Flush all FIFOs
        self.flush_tx_fifo(0x10); // All TX FIFOs
        self.flush_rx_fifo();

        // Clear all interrupts
        self.write(global_reg::GINTSTS, 0xFFFFFFFF);

        // Configure device speed
        self.modify(device_reg::DCFG, dcfg_bits::DSPD_MASK,
            (config.speed as u32) << dcfg_bits::DSPD_SHIFT);

        // Configure DMA if enabled
        if config.dma_enabled {
            self.modify(global_reg::GAHBCFG, 0, gahbcfg_bits::GINT);
        }
    }

    /// Configure USB for device mode / 配置 USB 为设备模式
    pub fn set_device_mode(&self) {
        // Force device mode
        self.modify(global_reg::GUSBCFG, gusbcfg_bits::FHMOD, gusbcfg_bits::FDMOD);

        // Wait for device mode
        while (self.read(global_reg::GINTSTS) & gint_bits::CMOD) != 0 {}
    }

    /// Configure USB for host mode / 配置 USB 为主机模式
    pub fn set_host_mode(&self) {
        // Force host mode
        self.modify(global_reg::GUSBCFG, gusbcfg_bits::FDMOD, gusbcfg_bits::FHMOD);

        // Wait for host mode
        while (self.read(global_reg::GINTSTS) & gint_bits::CMOD) == 0 {}
    }

    /// Connect device (clear soft disconnect) / 连接设备 (清除软断开)
    pub fn connect(&self) {
        self.modify(device_reg::DCTL, dctl_bits::SDIS, 0);
    }

    /// Disconnect device (soft disconnect) / 断开设备 (软断开)
    pub fn disconnect(&self) {
        self.modify(device_reg::DCTL, 0, dctl_bits::SDIS);
    }

    /// Set device address / 设置设备地址
    /// 
    /// # Arguments
    /// * `address` - Device address / 设备地址
    pub fn set_device_address(&self, address: u8) {
        self.modify(device_reg::DCFG, dcfg_bits::DAD_MASK,
            (address as u32) << dcfg_bits::DAD_SHIFT);
    }

    /// Configure endpoint / 配置端点
    /// 
    /// # Arguments
    /// * `config` - Endpoint configuration / 端点配置
    pub fn configure_endpoint(&self, config: &EpConfig) {
        let ep_offset = if config.direction == EpDirection::In {
            ep_reg::DIEPCTL + (config.ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (config.ep_num as usize * 0x20)
        };

        let value = (config.ep_type as u32) << epctl_bits::EPTYP_SHIFT
            | (config.max_packet_size as u32) & epctl_bits::MPSIZ_MASK
            | epctl_bits::USBAEP
            | (config.tx_fifo_num as u32) << epctl_bits::TXFNUM_SHIFT;

        self.write(ep_offset, value);
    }

    /// Enable endpoint / 使能端点
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    pub fn enable_endpoint(&self, ep_num: u8, direction: EpDirection) {
        let ep_offset = if direction == EpDirection::In {
            ep_reg::DIEPCTL + (ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (ep_num as usize * 0x20)
        };

        self.modify(ep_offset, 0, epctl_bits::EPENA | epctl_bits::CNAK);
    }

    /// Disable endpoint / 禁用端点
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    pub fn disable_endpoint(&self, ep_num: u8, direction: EpDirection) {
        let ep_offset = if direction == EpDirection::In {
            ep_reg::DIEPCTL + (ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (ep_num as usize * 0x20)
        };

        self.modify(ep_offset, 0, epctl_bits::EPDIS);
    }

    /// Set STALL for endpoint / 设置端点 STALL
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    pub fn stall_endpoint(&self, ep_num: u8, direction: EpDirection) {
        let ep_offset = if direction == EpDirection::In {
            ep_reg::DIEPCTL + (ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (ep_num as usize * 0x20)
        };

        self.modify(ep_offset, 0, epctl_bits::STALL);
    }

    /// Clear STALL for endpoint / 清除端点 STALL
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    pub fn clear_stall(&self, ep_num: u8, direction: EpDirection) {
        let ep_offset = if direction == EpDirection::In {
            ep_reg::DIEPCTL + (ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (ep_num as usize * 0x20)
        };

        self.modify(ep_offset, epctl_bits::STALL, 0);
    }

    /// Write data to IN endpoint FIFO / 向输入端点 FIFO 写数据
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `data` - Data to write / 要写入的数据
    pub fn write_to_fifo(&self, ep_num: u8, data: &[u8]) {
        let fifo = self.base + ep_reg::FIFO_BASE + (ep_num as usize * 0x1000);
        let fifo_ptr = fifo as *mut u32;

        unsafe {
            for chunk in data.chunks(4) {
                let mut word: u32 = 0;
                for (i, &byte) in chunk.iter().enumerate() {
                    word |= (byte as u32) << (i * 8);
                }
                write_volatile(fifo_ptr, word);
            }
        }
    }

    /// Read data from OUT endpoint FIFO / 从输出端点 FIFO 读数据
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `buffer` - Buffer to store data / 存储数据的缓冲区
    /// 
    /// # Returns
    /// Number of bytes read / 读取的字节数
    pub fn read_from_fifo(&self, ep_num: u8, buffer: &mut [u8]) -> usize {
        let grxstsp = self.read(global_reg::GRXSTSP);
        let byte_count = ((grxstsp >> 4) & 0x7FF) as usize;
        let pkt_status = (grxstsp >> 17) & 0xF;

        if pkt_status != 0x02 && pkt_status != 0x04 {
            return 0;
        }

        let fifo = self.base + ep_reg::FIFO_BASE;
        let fifo_ptr = fifo as *const u32;

        unsafe {
            let words_to_read = (byte_count + 3) / 4;
            for i in 0..words_to_read.min(buffer.len() / 4 + 1) {
                let word = read_volatile(fifo_ptr);
                let offset = i * 4;
                for j in 0..4 {
                    if offset + j < buffer.len() && offset + j < byte_count {
                        buffer[offset + j] = (word >> (j * 8)) as u8;
                    }
                }
            }
        }

        byte_count
    }

    /// Set transfer size for IN endpoint / 设置输入端点传输大小
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `size` - Transfer size in bytes / 传输大小 (字节)
    /// * `packet_count` - Number of packets / 包数量
    pub fn set_in_transfer_size(&self, ep_num: u8, size: u32, packet_count: u32) {
        let offset = ep_reg::DIEPTSIZ + (ep_num as usize * 0x20);
        let value = (size & eptsiz_bits::XFRSIZ_MASK)
            | ((packet_count << eptsiz_bits::PKTCNT_SHIFT) & eptsiz_bits::PKTCNT_MASK);
        self.write(offset, value);
    }

    /// Set transfer size for OUT endpoint / 设置输出端点传输大小
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `size` - Transfer size in bytes / 传输大小 (字节)
    /// * `packet_count` - Number of packets / 包数量
    pub fn set_out_transfer_size(&self, ep_num: u8, size: u32, packet_count: u32) {
        let offset = ep_reg::DOEPTSIZ + (ep_num as usize * 0x20);
        let value = (size & eptsiz_bits::XFRSIZ_MASK)
            | ((packet_count << eptsiz_bits::PKTCNT_SHIFT) & eptsiz_bits::PKTCNT_MASK);

        // For control endpoint, also set setup packet count
        if ep_num == 0 {
            self.write(offset, value | (3 << eptsiz_bits::STUPCNT_SHIFT));
        } else {
            self.write(offset, value);
        }
    }

    /// Flush TX FIFO / 刷新 TX FIFO
    /// 
    /// # Arguments
    /// * `fifo_num` - FIFO number (0-15, 0x10 for all) / FIFO 编号 (0-15，0x10 表示全部)
    pub fn flush_tx_fifo(&self, fifo_num: u8) {
        let value = (fifo_num as u32) << grstctl_bits::TXFNUM_SHIFT | grstctl_bits::TXFFLSH;
        self.modify(global_reg::GRSTCTL, 0, value);
        while (self.read(global_reg::GRSTCTL) & grstctl_bits::TXFFLSH) != 0 {}
    }

    /// Flush RX FIFO / 刷新 RX FIFO
    pub fn flush_rx_fifo(&self) {
        self.modify(global_reg::GRSTCTL, 0, grstctl_bits::RXFFLSH);
        while (self.read(global_reg::GRSTCTL) & grstctl_bits::RXFFLSH) != 0 {}
    }

    /// Enable global interrupt / 使能全局中断
    pub fn enable_global_interrupt(&self) {
        self.modify(global_reg::GAHBCFG, 0, gahbcfg_bits::GINT);
    }

    /// Disable global interrupt / 禁用全局中断
    pub fn disable_global_interrupt(&self) {
        self.modify(global_reg::GAHBCFG, gahbcfg_bits::GINT, 0);
    }

    /// Enable interrupt / 使能中断
    /// 
    /// # Arguments
    /// * `mask` - Interrupt mask / 中断屏蔽
    pub fn enable_interrupt(&self, mask: u32) {
        self.modify(global_reg::GINTMSK, 0, mask);
    }

    /// Disable interrupt / 禁用中断
    /// 
    /// # Arguments
    /// * `mask` - Interrupt mask / 中断屏蔽
    pub fn disable_interrupt(&self, mask: u32) {
        self.modify(global_reg::GINTMSK, mask, 0);
    }

    /// Get interrupt status / 获取中断状态
    pub fn get_interrupt_status(&self) -> u32 {
        self.read(global_reg::GINTSTS)
    }

    /// Clear interrupt / 清除中断
    /// 
    /// # Arguments
    /// * `mask` - Interrupt mask / 中断屏蔽
    pub fn clear_interrupt(&self, mask: u32) {
        self.write(global_reg::GINTSTS, mask);
    }

    /// Get device status / 获取设备状态
    pub fn get_device_status(&self) -> u32 {
        self.read(device_reg::DSTS)
    }

    /// Get device IN endpoint interrupt status / 获取设备输入端点中断状态
    pub fn get_in_endpoint_interrupt(&self) -> u32 {
        self.read(device_reg::DIEPINT)
    }

    /// Get device OUT endpoint interrupt status / 获取设备输出端点中断状态
    pub fn get_out_endpoint_interrupt(&self) -> u32 {
        self.read(device_reg::DOEPINT)
    }

    /// Get all endpoints interrupt status / 获取所有端点中断状态
    pub fn get_all_endpoints_interrupt(&self) -> u32 {
        self.read(device_reg::DAINT)
    }

    /// Set global IN NAK / 设置全局输入 NAK
    pub fn set_global_in_nak(&self) {
        self.modify(device_reg::DCTL, 0, dctl_bits::SGINAK);
    }

    /// Clear global IN NAK / 清除全局输入 NAK
    pub fn clear_global_in_nak(&self) {
        self.modify(device_reg::DCTL, dctl_bits::SGINAK, 0);
    }

    /// Set global OUT NAK / 设置全局输出 NAK
    pub fn set_global_out_nak(&self) {
        self.modify(device_reg::DCTL, 0, dctl_bits::SGONAK);
    }

    /// Clear global OUT NAK / 清除全局输出 NAK
    pub fn clear_global_out_nak(&self) {
        self.modify(device_reg::DCTL, dctl_bits::SGONAK, 0);
    }

    /// Check if connected (not in soft disconnect) / 检查是否连接 (未软断开)
    pub fn is_connected(&self) -> bool {
        (self.read(device_reg::DCTL) & dctl_bits::SDIS) == 0
    }

    /// Check if suspend / 检查是否挂起
    pub fn is_suspended(&self) -> bool {
        (self.read(device_reg::DSTS) & dsts_bits::SUSPSTS) != 0
    }

    /// Activate endpoint / 激活端点
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    pub fn activate_endpoint(&self, ep_num: u8, direction: EpDirection) {
        let ep_offset = if direction == EpDirection::In {
            ep_reg::DIEPCTL + (ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (ep_num as usize * 0x20)
        };

        self.modify(ep_offset, 0, epctl_bits::USBAEP | epctl_bits::EPENA | epctl_bits::CNAK);
    }

    /// Deactivate endpoint / 停用端点
    /// 
    /// # Arguments
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    pub fn deactivate_endpoint(&self, ep_num: u8, direction: EpDirection) {
        let ep_offset = if direction == EpDirection::In {
            ep_reg::DIEPCTL + (ep_num as usize * 0x20)
        } else {
            ep_reg::DOEPCTL + (ep_num as usize * 0x20)
        };

        self.modify(ep_offset, epctl_bits::EPENA, 0);
    }

    /// Configure host channel / 配置主机通道
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    /// * `device_address` - USB device address / USB 设备地址
    /// * `ep_num` - Endpoint number / 端点编号
    /// * `direction` - Endpoint direction / 端点方向
    /// * `ep_type` - Endpoint type / 端点类型
    /// * `max_packet_size` - Maximum packet size / 最大包大小
    pub fn configure_host_channel(
        &self,
        channel: u8,
        device_address: u8,
        ep_num: u8,
        direction: EpDirection,
        ep_type: EpType,
        max_packet_size: u16,
    ) {
        let offset = host_reg::HCCHAR + (channel as usize * 0x20);
        let value = (max_packet_size as u32 & hcchar_bits::MPSIZ_MASK)
            | ((ep_num as u32) << hcchar_bits::EPNUM_SHIFT)
            | if direction == EpDirection::In { hcchar_bits::EPDIR } else { 0 }
            | ((ep_type as u32) << hcchar_bits::EPTYP_SHIFT)
            | ((device_address as u32) << hcchar_bits::DAD_SHIFT);
        self.write(offset, value);
    }

    /// Enable host channel / 使能主机通道
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    pub fn enable_host_channel(&self, channel: u8) {
        let offset = host_reg::HCCHAR + (channel as usize * 0x20);
        self.modify(offset, 0, hcchar_bits::CHENA);
    }

    /// Disable host channel / 禁用主机通道
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    pub fn disable_host_channel(&self, channel: u8) {
        let offset = host_reg::HCCHAR + (channel as usize * 0x20);
        self.modify(offset, 0, hcchar_bits::CHDIS);
    }

    /// Set host channel transfer size / 设置主机通道传输大小
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    /// * `packet_count` - Number of packets / 包数量
    /// * `transfer_size` - Transfer size in bytes / 传输大小 (字节)
    pub fn set_host_channel_transfer_size(&self, channel: u8, packet_count: u32, transfer_size: u32) {
        let offset = host_reg::HCTSIZ + (channel as usize * 0x20);
        let value = (transfer_size & 0x7FFFF)
            | ((packet_count & 0x3FF) << 19);
        self.write(offset, value);
    }

    /// Get host channel interrupt status / 获取主机通道中断状态
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    pub fn get_host_channel_interrupt(&self, channel: u8) -> u32 {
        let offset = host_reg::HCINT + (channel as usize * 0x20);
        self.read(offset)
    }

    /// Clear host channel interrupt / 清除主机通道中断
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    /// * `mask` - Interrupt bits to clear / 要清除的中断位
    pub fn clear_host_channel_interrupt(&self, channel: u8, mask: u32) {
        let offset = host_reg::HCINT + (channel as usize * 0x20);
        self.write(offset, mask);
    }

    /// Enable host channel interrupt / 使能主机通道中断
    /// 
    /// # Arguments
    /// * `channel` - Channel number (0-11) / 通道编号 (0-11)
    /// * `mask` - Interrupt mask / 中断屏蔽
    pub fn enable_host_channel_interrupt(&self, channel: u8, mask: u32) {
        let offset = host_reg::HCINTMSK + (channel as usize * 0x20);
        self.write(offset, mask);
    }

    /// Get host port status / 获取主机端口状态
    pub fn get_host_port_status(&self) -> u32 {
        self.read(host_reg::HPRT)
    }

    /// Set host port power / 设置主机端口电源
    pub fn set_host_port_power(&self) {
        self.modify(host_reg::HPRT, 0, hprt_bits::PPWR);
    }

    /// Clear host port power / 清除主机端口电源
    pub fn clear_host_port_power(&self) {
        self.modify(host_reg::HPRT, hprt_bits::PPWR, 0);
    }

    /// Issue host port reset / 发起主机端口复位
    /// Note: Uses software delay - in production, use a timer or cortex_m::asm::delay / 注意: 使用软件延时 - 生产环境应使用定时器或 cortex_m::asm::delay
    pub fn host_port_reset(&self) {
        self.modify(host_reg::HPRT, 0, hprt_bits::PRST);
        let _ = self.read(host_reg::HPRT);
        let mut delay: u32 = 200000;
        while delay > 0 {
            delay -= 1;
        }
        self.modify(host_reg::HPRT, hprt_bits::PRST, 0);
    }

    /// Check host port connection / 检查主机端口连接
    pub fn is_port_connected(&self) -> bool {
        (self.read(host_reg::HPRT) & hprt_bits::PCSTS) != 0
    }

    /// Get host port speed / 获取主机端口速度
    pub fn get_port_speed(&self) -> UsbSpeed {
        let speed = (self.read(host_reg::HPRT) >> 17) & 0x3;
        match speed {
            0 => UsbSpeed::High,
            1 => UsbSpeed::Full,
            2 => UsbSpeed::Low,
            _ => UsbSpeed::Full,
        }
    }

    /// Configure host frame interval / 配置主机帧间隔
    /// 
    /// # Arguments
    /// * `frame_interval` - Frame interval in PHY clocks / 帧间隔 (PHY 时钟)
    pub fn set_host_frame_interval(&self, frame_interval: u32) {
        self.write(host_reg::HFIR, frame_interval & 0xFFFF);
    }

    /// Get host frame number / 获取主机帧号
    pub fn get_host_frame_number(&self) -> u16 {
        (self.read(host_reg::HFNUM) & 0x3FFF) as u16
    }

    /// Configure host for FS/LS mode / 配置主机为 FS/LS 模式
    /// 
    /// # Arguments
    /// * `use_48mhz` - Use 48 MHz PHY clock / 使用 48 MHz PHY 时钟
    pub fn configure_host_fs_mode(&self, use_48mhz: bool) {
        let value = if use_48mhz {
            hcfg_bits::FSLSPCS_48MHz
        } else {
            hcfg_bits::FSLSPCS_6MHz
        };
        self.modify(host_reg::HCFG, hcfg_bits::FSLSPCS_MASK, value);
    }
}

// ============================================================================
// Convenience Functions / 便捷函数
// ============================================================================

/// Initialize USB OTG FS in device mode / 初始化 USB OTG FS 为设备模式
pub fn init_usb_fs_device() -> UsbOtg {
    let usb = UsbOtg::new_fs();
    usb.enable_clock();
    usb.init(&UsbConfig::default());
    usb.set_device_mode();
    usb.connect();
    usb
}

/// Initialize USB OTG FS in device mode with config / 使用配置初始化 USB OTG FS 为设备模式
pub fn init_usb_fs_device_with_config(config: &UsbConfig) -> UsbOtg {
    let usb = UsbOtg::new_fs();
    usb.enable_clock();
    usb.init(config);
    usb.set_device_mode();
    usb.connect();
    usb
}

/// Initialize USB OTG FS in host mode / 初始化 USB OTG FS 为主机模式
pub fn init_usb_fs_host() -> UsbOtg {
    let usb = UsbOtg::new_fs();
    usb.enable_clock();
    usb.init(&UsbConfig::default());
    usb.set_host_mode();
    usb
}

/// Initialize USB OTG HS in device mode / 初始化 USB OTG HS 为设备模式
pub fn init_usb_hs_device() -> UsbOtg {
    let usb = UsbOtg::new_hs();
    usb.enable_clock();
    usb.init(&UsbConfig::default());
    usb.set_device_mode();
    usb.connect();
    usb
}

/// Initialize USB OTG HS in device mode with config / 使用配置初始化 USB OTG HS 为设备模式
pub fn init_usb_hs_device_with_config(config: &UsbConfig) -> UsbOtg {
    let usb = UsbOtg::new_hs();
    usb.enable_clock();
    usb.init(config);
    usb.set_device_mode();
    usb.connect();
    usb
}

/// Initialize USB OTG HS in host mode / 初始化 USB OTG HS 为主机模式
pub fn init_usb_hs_host() -> UsbOtg {
    let usb = UsbOtg::new_hs();
    usb.enable_clock();
    usb.init(&UsbConfig::default());
    usb.set_host_mode();
    usb
}

/// Create USB configuration for Full-Speed / 创建全速 USB 配置
pub fn config_fs() -> UsbConfig {
    UsbConfig {
        speed: UsbSpeed::Full,
        device_address: 0,
        rx_fifo_size: 128,
        tx_fifo_sizes: [64, 64, 64, 64, 64, 64, 64, 64],
        dma_enabled: false,
    }
}

/// Create USB configuration for High-Speed / 创建高速 USB 配置
pub fn config_hs() -> UsbConfig {
    UsbConfig {
        speed: UsbSpeed::High,
        device_address: 0,
        rx_fifo_size: 256,
        tx_fifo_sizes: [128, 128, 128, 128, 128, 128, 128, 128],
        dma_enabled: true,
    }
}

/// Create USB configuration for High-Speed with DMA / 创建带 DMA 的高速 USB 配置
pub fn config_hs_dma() -> UsbConfig {
    UsbConfig {
        speed: UsbSpeed::High,
        device_address: 0,
        rx_fifo_size: 512,
        tx_fifo_sizes: [256, 256, 256, 256, 256, 256, 256, 256],
        dma_enabled: true,
    }
}
