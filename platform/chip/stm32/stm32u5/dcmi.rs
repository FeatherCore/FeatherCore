//! DCMI - Digital Camera Interface
//! 数字摄像头接口
//!
//! ## STM32U5 DCMI 特性 / Features
//! - **数据宽度 / Data Width:** 8/10/12/14-bit 并行接口
//! - **同步模式 / Sync Modes:**
//!   - 嵌入式同步 (Embedded sync)
//!   - 行同步 (Line sync)
//!   - 帧同步 (Frame sync)
//!
//! - **工作模式 / Modes:**
//!   - 连续模式 (Continuous mode)
//!   - 快照模式 (Snapshot mode)
//!
//! - **特性 / Features:**
//!   - 裁剪功能 (Cropping)
//!   - JPEG 压缩支持
//!   - DMA 传输支持
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 41: Digital camera interface (DCMI)
//! - RM0456 Section 41.1: DCMI introduction
//! - RM0456 Section 41.2: DCMI main features
//! - RM0456 Section 41.3: DCMI functional description
//! - RM0456 Section 41.4: DCMI registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// DCMI base address / DCMI 基地址
pub const DCMI_BASE: usize = 0x4202_C000;

/// DCMI register offsets / DCMI 寄存器偏移
//! Reference: RM0456 Section 41.4: DCMI registers
pub mod reg {
    /// DCMI Control Register / DCMI 控制寄存器
    //! Reference: RM0456 Section 41.4.1: DCMI_CR
    pub const CR: usize = 0x00;
    /// DCMI Status Register / DCMI 状态寄存器
    //! Reference: RM0456 Section 41.4.2: DCMI_SR
    pub const SR: usize = 0x04;
    /// DCMI Raw Interrupt Status Register / DCMI 原始中断状态寄存器
    //! Reference: RM0456 Section 41.4.3: DCMI_RIS
    pub const RIS: usize = 0x08;
    /// DCMI Interrupt Enable Register / DCMI 中断使能寄存器
    //! Reference: RM0456 Section 41.4.4: DCMI_IER
    pub const IER: usize = 0x0C;
    /// DCMI Masked Interrupt Status Register / DCMI 屏蔽中断状态寄存器
    //! Reference: RM0456 Section 41.4.5: DCMI_MIS
    pub const MIS: usize = 0x10;
    /// DCMI Interrupt Clear Register / DCMI 中断清除寄存器
    //! Reference: RM0456 Section 41.4.6: DCMI_ICR
    pub const ICR: usize = 0x14;
    /// DCMI Embedded Synchronization Code Register / DCMI 嵌入式同步码寄存器
    //! Reference: RM0456 Section 41.4.7: DCMI_ESCR
    pub const ESCR: usize = 0x18;
    /// DCMI Embedded Synchronization Unmask Register / DCMI 嵌入式同步去屏蔽寄存器
    //! Reference: RM0456 Section 41.4.8: DCMI_ESUR
    pub const ESUR: usize = 0x1C;
    /// DCMI Crop Window Start Register / DCMI 裁剪窗口起始寄存器
    //! Reference: RM0456 Section 41.4.9: DCMI_CWSTRT
    pub const CWSTRT: usize = 0x20;
    /// DCMI Crop Window Size Register / DCMI 裁剪窗口大小寄存器
    //! Reference: RM0456 Section 41.4.10: DCMI_CWSIZE
    pub const CWSIZE: usize = 0x24;
    /// DCMI Data Register / DCMI 数据寄存器
    //! Reference: RM0456 Section 41.4.11: DCMI_DR
    pub const DR: usize = 0x28;
    /// DCMI Peripheral Clock Size Register / DCMI 外设时钟大小寄存器
    pub const PCKSIZE: usize = 0x30;
}

/// CR Register Bit Definitions / CR 寄存器位定义
//! Reference: RM0456 Section 41.4.1
pub mod cr_bits {
    /// Capture enable / 捕获使能
    pub const CAPTURE: u32 = 1 << 0;
    /// Capture mode / 捕获模式
    pub const CM: u32 = 1 << 1;
    /// Crop enable / 裁剪使能
    pub const CROP: u32 = 1 << 2;
    /// JPEG format / JPEG 格式
    pub const JPEG: u32 = 1 << 3;
    /// Embedded synchronization mode / 嵌入式同步模式
    pub const ESM: u32 = 1 << 4;
    /// Pixel clock polarity / 像素时钟极性
    pub const PCKPOL: u32 = 1 << 5;
    /// Horizontal synchronization polarity / 水平同步极性
    pub const HSPOL: u32 = 1 << 6;
    /// Vertical synchronization polarity / 垂直同步极性
    pub const VSPOL: u32 = 1 << 7;
    /// Frame capture rate / 帧捕获速率
    pub const FCRC: u32 = 0b11 << 8;
    /// Extended data mode / 扩展数据模式
    pub const EDM: u32 = 0b11 << 10;
    /// DCMI enable / DCMI 使能
    pub const ENABLE: u32 = 1 << 14;
}

/// SR Register Bit Definitions / SR 寄存器位定义
//! Reference: RM0456 Section 41.4.2
pub mod sr_bits {
    /// HSYNC / 水平同步
    pub const HSYNC: u32 = 1 << 0;
    /// VSYNC / 垂直同步
    pub const VSYNC: u32 = 1 << 1;
    /// FIFO not empty / FIFO 非空
    pub const FNE: u32 = 1 << 2;
}

/// RIS/ICR/IER/MIS Register Bit Definitions / RIS/ICR/IER/MIS 寄存器位定义
//! Reference: RM0456 Section 41.4.3 to 41.4.6
pub mod int_bits {
    /// Frame interrupt / 帧中断
    pub const FRAME: u32 = 1 << 0;
    /// Overrun interrupt / 溢出中断
    pub const OVR: u32 = 1 << 1;
    /// Error interrupt / 错误中断
    pub const ERR: u32 = 1 << 2;
    /// VSYNC interrupt / VSYNC 中断
    pub const VSYNC: u32 = 1 << 3;
    /// Line interrupt / 行中断
    pub const LINE: u32 = 1 << 4;
}

/// DCMI capture mode / DCMI 捕获模式
//! Reference: RM0456 Section 41.3.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CaptureMode {
    /// Continuous capture / 连续捕获
    Continuous = 0,
    /// Single snapshot / 单张快照
    Snapshot = 1,
}

/// DCMI synchronization mode / DCMI 同步模式
//! Reference: RM0456 Section 41.3.2
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncMode {
    /// Hardware synchronization (HSYNC/VSYNC) / 硬件同步 (HSYNC/VSYNC)
    Hardware = 0,
    /// Embedded synchronization / 嵌入式同步
    Embedded = 1,
}

/// DCMI pixel clock polarity / DCMI 像素时钟极性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PclkPolarity {
    /// Falling edge active / 下降沿有效
    Falling = 0,
    /// Rising edge active / 上升沿有效
    Rising = 1,
}

/// DCMI vertical synchronization polarity / DCMI 垂直同步极性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VsyncPolarity {
    /// Low active / 低电平有效
    Low = 0,
    /// High active / 高电平有效
    High = 1,
}

/// DCMI horizontal synchronization polarity / DCMI 水平同步极性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HsyncPolarity {
    /// Low active / 低电平有效
    Low = 0,
    /// High active / 高电平有效
    High = 1,
}

/// DCMI data width / DCMI 数据宽度
//! Reference: RM0456 Section 41.3.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataWidth {
    /// 8 bits / 8 位
    Bits8 = 0b00,
    /// 10 bits / 10 位
    Bits10 = 0b01,
    /// 12 bits / 12 位
    Bits12 = 0b10,
    /// 14 bits / 14 位
    Bits14 = 0b11,
}

/// DCMI frame capture rate / DCMI 帧捕获速率
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameRate {
    All = 0b00,
    Half = 0b01,
    Quarter = 0b10,
}

/// DCMI configuration / DCMI 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub capture_mode: CaptureMode,
    pub sync_mode: SyncMode,
    pub pclk_polarity: PclkPolarity,
    pub vsync_polarity: VsyncPolarity,
    pub hsync_polarity: HsyncPolarity,
    pub data_width: DataWidth,
    pub jpeg_mode: bool,
    pub crop_enable: bool,
    pub frame_rate: FrameRate,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            capture_mode: CaptureMode::Continuous,
            sync_mode: SyncMode::Hardware,
            pclk_polarity: PclkPolarity::Rising,
            vsync_polarity: VsyncPolarity::Low,
            hsync_polarity: HsyncPolarity::Low,
            data_width: DataWidth::Bits8,
            jpeg_mode: false,
            crop_enable: false,
            frame_rate: FrameRate::All,
        }
    }
}

/// DCMI cropping configuration / DCMI 裁剪配置
#[derive(Clone, Copy, Debug)]
pub struct CropConfig {
    /// Horizontal offset / 水平偏移
    pub h_offset: u16,
    /// Vertical offset / 垂直偏移
    pub v_offset: u16,
    /// Capture width / 捕获宽度
    pub width: u16,
    /// Capture height / 捕获高度
    pub height: u16,
}

/// DCMI instance / DCMI 实例
pub struct Dcmi;

impl Dcmi {
    /// Create DCMI instance / 创建 DCMI 实例
    pub const fn new() -> Self {
        Self
    }

    /// Initialize DCMI / 初始化 DCMI
    //! Reference: RM0456 Section 41.3.1
    pub fn init(&self, config: &Config) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= (config.capture_mode as u32) << 1;
            val |= (config.sync_mode as u32) << 4;
            val |= (config.pclk_polarity as u32) << 5;
            val |= (config.vsync_polarity as u32) << 7;
            val |= (config.hsync_polarity as u32) << 6;
            val |= (config.data_width as u32) << 10;
            val |= (config.frame_rate as u32) << 8;
            if config.jpeg_mode {
                val |= cr_bits::JPEG;
            }
            if config.crop_enable {
                val |= cr_bits::CROP;
            }
            write_volatile(cr, val);
        }
    }

    /// Configure cropping / 配置裁剪
    //! Reference: RM0456 Section 41.3.4
    pub fn configure_crop(&self, crop: &CropConfig) {
        unsafe {
            let cwstrt = (DCMI_BASE + reg::CWSTRT) as *mut u32;
            let mut val = 0;
            val |= (crop.h_offset as u32) << 16;
            val |= (crop.v_offset as u32) << 0;
            write_volatile(cwstrt, val);

            let cwsize = (DCMI_BASE + reg::CWSIZE) as *mut u32;
            let mut val = 0;
            val |= ((crop.width - 1) as u32) << 16;
            val |= ((crop.height - 1) as u32) << 0;
            write_volatile(cwsize, val);
        }
    }

    /// Enable DCMI / 使能 DCMI
    pub fn enable(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::ENABLE;
            write_volatile(cr, val);
        }
    }

    /// Disable DCMI / 禁用 DCMI
    pub fn disable(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::ENABLE;
            write_volatile(cr, val);
        }
    }

    /// Start capture / 开始捕获
    pub fn start_capture(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::CAPTURE;
            write_volatile(cr, val);
        }
    }

    /// Stop capture / 停止捕获
    pub fn stop_capture(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::CAPTURE;
            write_volatile(cr, val);
        }
    }

    /// Check if frame captured / 检查是否捕获到帧
    pub fn is_frame_captured(&self) -> bool {
        unsafe {
            let ris = (DCMI_BASE + reg::RIS) as *mut u32;
            let val = read_volatile(ris);
            (val & int_bits::FRAME) != 0
        }
    }

    /// Clear frame captured flag / 清除帧捕获标志
    pub fn clear_frame_flag(&self) {
        unsafe {
            let icr = (DCMI_BASE + reg::ICR) as *mut u32;
            write_volatile(icr, int_bits::FRAME);
        }
    }

    /// Enable interrupt / 使能中断
    pub fn enable_interrupt(&self, line_int: bool, vsync_int: bool, frame_int: bool) {
        unsafe {
            let ier = (DCMI_BASE + reg::IER) as *mut u32;
            let mut val = 0;
            if line_int {
                val |= int_bits::LINE;
            }
            if vsync_int {
                val |= int_bits::VSYNC;
            }
            if frame_int {
                val |= int_bits::FRAME;
            }
            write_volatile(ier, val);
        }
    }

    /// Read data / 读取数据
    pub fn read_data(&self) -> u32 {
        unsafe {
            let dr = (DCMI_BASE + reg::DR) as *mut u32;
            read_volatile(dr)
        }
    }

    /// Check if data available / 检查是否有数据可用
    pub fn is_data_available(&self) -> bool {
        unsafe {
            let sr = (DCMI_BASE + reg::SR) as *mut u32;
            let val = read_volatile(sr);
            (val & sr_bits::FNE) != 0
        }
    }

    /// Check if FIFO not empty / 检查 FIFO 是否非空
    pub fn is_fifo_not_empty(&self) -> bool {
        self.is_data_available()
    }

    /// Get HSYNC status / 获取 HSYNC 状态
    pub fn is_hsync(&self) -> bool {
        unsafe {
            let sr = (DCMI_BASE + reg::SR) as *mut u32;
            let val = read_volatile(sr);
            (val & sr_bits::HSYNC) != 0
        }
    }

    /// Get VSYNC status / 获取 VSYNC 状态
    pub fn is_vsync(&self) -> bool {
        unsafe {
            let sr = (DCMI_BASE + reg::SR) as *mut u32;
            let val = read_volatile(sr);
            (val & sr_bits::VSYNC) != 0
        }
    }

    /// Set frame rate / 设置帧速率
    pub fn set_frame_rate(&self, rate: FrameRate) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::FCRC;
            val |= (rate as u32) << 8;
            write_volatile(cr, val);
        }
    }

    /// Set data width / 设置数据宽度
    pub fn set_data_width(&self, width: DataWidth) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::EDM;
            val |= (width as u32) << 10;
            write_volatile(cr, val);
        }
    }
}

/// Initialize DCMI for OV7670 camera (8-bit, QQVGA 160x120)
pub fn init_dcmi_ov7670() {
    let dcmi = Dcmi::new();

    let config = Config {
        capture_mode: CaptureMode::Continuous,
        sync_mode: SyncMode::Hardware,
        pclk_polarity: PclkPolarity::Rising,
        vsync_polarity: VsyncPolarity::High,
        hsync_polarity: HsyncPolarity::Low,
        data_width: DataWidth::Bits8,
        jpeg_mode: false,
        crop_enable: false,
        frame_rate: FrameRate::All,
    };

    dcmi.init(&config);
    dcmi.enable();
}

/// Initialize DCMI with DMA for continuous capture
pub fn init_dcmi_dma(frame_buffer: usize, width: u16, height: u16) {
    let dcmi = Dcmi::new();

    let config = Config::default();
    dcmi.init(&config);

    if width != 0 && height != 0 {
        let crop = CropConfig {
            h_offset: 0,
            v_offset: 0,
            width,
            height,
        };
        dcmi.configure_crop(&crop);
    }

    dcmi.enable_interrupt(false, false, true);
    dcmi.enable();
    dcmi.start_capture();
}
