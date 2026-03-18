//! GPU2D - Graphics Processing Unit 2D
//! 2D 图形处理单元
//!
//! ## STM32U5 GPU2D 特性 / Features
//! - **功能 / Functions:
//!   - 像素格式转换 (Pixel format conversion)
//!   - 图像混合 (Image blending)
//!   - 颜色填充 (Color filling)
//!   - 源地址到目标地址复制 (Source to destination copy)
//!   - 镜像和旋转 (Mirroring and rotation)
//!
//! - **支持的像素格式 / Supported Pixel Formats:
//!   - ARGB8888
//!   - RGB888
//!   - RGB565
//!   - ARGB1555
//!   - ARGB4444
//!   - L8
//!   - AL44
//!   - AL88
//!
//! - **混合模式 / Blending Modes:
//!   - 无混合 (No blending)
//!   - 常量 Alpha 混合 (Constant alpha blending)
//!   - 像素 Alpha 混合 (Pixel alpha blending)
//!   - 常量和像素 Alpha 组合 (Combined constant and pixel alpha)
//!
//! - **特性 / Features:
//!   - 行偏移 (Line offset)
//!   - 裁剪窗口 (Clipping window)
//!   - DMA2D 中断支持
//!   - AHB 总线主控
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 45: Chrom-ART Accelerator (DMA2D)
//! - RM0456 Section 45.1: DMA2D introduction
//! - RM0456 Section 45.2: DMA2D main features
//! - RM0456 Section 45.3: DMA2D functional description
//! - RM0456 Section 45.4: DMA2D registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// GPU2D (DMA2D) base address / GPU2D (DMA2D) 基地址
pub const DMA2D_BASE: usize = 0x4002_B000;

/// DMA2D register offsets / DMA2D 寄存器偏移
//! Reference: RM0456 Section 45.4: DMA2D registers
pub mod reg {
    /// DMA2D Control Register / DMA2D 控制寄存器
    //! Reference: RM0456 Section 45.4.1: DMA2D_CR
    pub const CR: usize = 0x00;

    /// DMA2D Interrupt Status Register / DMA2D 中断状态寄存器
    //! Reference: RM0456 Section 45.4.2: DMA2D_ISR
    pub const ISR: usize = 0x04;

    /// DMA2D Interrupt Flag Clear Register / DMA2D 中断标志清除寄存器
    //! Reference: RM0456 Section 45.4.3: DMA2D_IFCR
    pub const IFCR: usize = 0x08;

    /// DMA2D Foreground Memory Address Register / DMA2D 前景内存地址寄存器
    //! Reference: RM0456 Section 45.4.4: DMA2D_FGMAR
    pub const FGMAR: usize = 0x0C;

    /// DMA2D Foreground Offset Register / DMA2D 前景偏移寄存器
    //! Reference: RM0456 Section 45.4.5: DMA2D_FGOR
    pub const FGOR: usize = 0x10;

    /// DMA2D Background Memory Address Register / DMA2D 背景内存地址寄存器
    //! Reference: RM0456 Section 45.4.6: DMA2D_BGMAR
    pub const BGMAR: usize = 0x14;

    /// DMA2D Background Offset Register / DMA2D 背景偏移寄存器
    //! Reference: RM0456 Section 45.4.7: DMA2D_BGOR
    pub const BGOR: usize = 0x18;

    /// DMA2D Foreground PFC Control Register / DMA2D 前景 PFC 控制寄存器
    //! Reference: RM0456 Section 45.4.8: DMA2D_FGPFCCR
    pub const FGPFCCR: usize = 0x1C;

    /// DMA2D Foreground Color Register / DMA2D 前景颜色寄存器
    //! Reference: RM0456 Section 45.4.9: DMA2D_FGCOLOR
    pub const FGCOLOR: usize = 0x20;

    /// DMA2D Background PFC Control Register / DMA2D 背景 PFC 控制寄存器
    //! Reference: RM0456 Section 45.4.10: DMA2D_BGPFCCR
    pub const BGPFCCR: usize = 0x24;

    /// DMA2D Background Color Register / DMA2D 背景颜色寄存器
    //! Reference: RM0456 Section 45.4.11: DMA2D_BGCOLOR
    pub const BGCOLOR: usize = 0x28;

    /// DMA2D Foreground CLUT Memory Address Register / DMA2D 前景 CLUT 内存地址寄存器
    //! Reference: RM0456 Section 45.4.12: DMA2D_FGCMAR
    pub const FGCMAR: usize = 0x2C;

    /// DMA2D Background CLUT Memory Address Register / DMA2D 背景 CLUT 内存地址寄存器
    //! Reference: RM0456 Section 45.4.13: DMA2D_BGCMAR
    pub const BGCMAR: usize = 0x30;

    /// DMA2D Output Memory Address Register / DMA2D 输出内存地址寄存器
    //! Reference: RM0456 Section 45.4.14: DMA2D_OMAR
    pub const OMAR: usize = 0x34;

    /// DMA2D Output Offset Register / DMA2D 输出偏移寄存器
    //! Reference: RM0456 Section 45.4.15: DMA2D_OOR
    pub const OOR: usize = 0x38;

    /// DMA2D Number of Line Register / DMA2D 行数寄存器
    //! Reference: RM0456 Section 45.4.16: DMA2D_NLR
    pub const NLR: usize = 0x3C;

    /// DMA2D Line Watermark Register / DMA2D 行水印寄存器
    //! Reference: RM0456 Section 45.4.17: DMA2D_LWR
    pub const LWR: usize = 0x40;

    /// DMA2D Output PFC Control Register / DMA2D 输出 PFC 控制寄存器
    //! Reference: RM0456 Section 45.4.18: DMA2D_OPFCCR
    pub const OPFCCR: usize = 0x44;

    /// DMA2D Output Color Register / DMA2D 输出颜色寄存器
    //! Reference: RM0456 Section 45.4.19: DMA2D_OCOLR
    pub const OCOLR: usize = 0x48;

    /// DMA2D Line Status Register / DMA2D 行状态寄存器
    pub const LSR: usize = 0x4C;

    /// DMA2D AHB Master Timer Configuration Register / DMA2D AHB 主定时器配置寄存器
    pub const AMTCR: usize = 0x50;
}

/// CR Register Bit Definitions / CR 寄存器位定义
//! Reference: RM0456 Section 45.4.1
pub mod cr_bits {
    /// Start / 开始
    pub const START: u32 = 1 << 0;
    /// Suspend / 暂停
    pub const SUSP: u32 = 1 << 1;
    /// Abort / 中止
    pub const ABORT: u32 = 1 << 2;
    /// Transfer error interrupt enable / 传输错误中断使能
    pub const TEIE: u32 = 1 << 8;
    /// Transfer complete interrupt enable / 传输完成中断使能
    pub const TCIE: u32 = 1 << 9;
    /// Transfer watermark interrupt enable / 传输水印中断使能
    pub const TWIE: u32 = 1 << 10;
    /// CLUT access error interrupt enable / CLUT 访问错误中断使能
    pub const CAEIE: u32 = 1 << 11;
    /// Configuration error interrupt enable / 配置错误中断使能
    pub const CEIE: u32 = 1 << 12;
    /// Mode / 模式
    pub const MODE: u32 = 0b11 << 16;
}

/// ISR Register Bit Definitions / ISR 寄存器位定义
//! Reference: RM0456 Section 45.4.2
pub mod isr_bits {
    /// Transfer error flag / 传输错误标志
    pub const TEIF: u32 = 1 << 0;
    /// Transfer complete flag / 传输完成标志
    pub const TCIF: u32 = 1 << 1;
    /// Transfer watermark flag / 传输水印标志
    pub const TWIF: u32 = 1 << 2;
    /// CLUT access error flag / CLUT 访问错误标志
    pub const CAEIF: u32 = 1 << 3;
    /// Configuration error flag / 配置错误标志
    pub const CEIF: u32 = 1 << 4;
}

/// DMA2D Mode / DMA2D 模式
//! Reference: RM0456 Section 45.3.1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    /// Memory-to-memory with PFC / 内存到内存（带 PFC）
    MemToMemPfc = 0b00,
    /// Memory-to-memory with PFC and blending / 内存到内存（带 PFC 和混合）
    MemToMemPfcBlend = 0b01,
    /// Register-to-memory / 寄存器到内存
    RegToMem = 0b10,
    /// Memory-to-memory / 内存到内存
    MemToMem = 0b11,
}

/// Pixel Format / 像素格式
//! Reference: RM0456 Section 45.3.3
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PixelFormat {
    /// ARGB8888
    Argb8888 = 0b000,
    /// RGB888
    Rgb888 = 0b001,
    /// RGB565
    Rgb565 = 0b010,
    /// ARGB1555
    Argb1555 = 0b011,
    /// ARGB4444
    Argb4444 = 0b100,
    /// L8
    L8 = 0b101,
    /// AL44
    Al44 = 0b110,
    /// AL88
    Al88 = 0b111,
}

/// DMA2D Instance / DMA2D 实例
pub struct Dma2d;

impl Dma2d {
    /// Create DMA2D instance / 创建 DMA2D 实例
    pub const fn new() -> Self {
        Self
    }

    /// Configure mode / 配置模式
    pub fn set_mode(&self, mode: Mode) {
        unsafe {
            let cr = (DMA2D_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !cr_bits::MODE;
            val |= (mode as u32) << 16;
            write_volatile(cr, val);
        }
    }

    /// Configure output color mode / 配置输出颜色模式
    pub fn set_output_color_mode(&self, format: PixelFormat) {
        unsafe {
            let opfccr = (DMA2D_BASE + reg::OPFCCR) as *mut u32;
            let mut val = read_volatile(opfccr);
            val &= !(0b111 << 0);
            val |= format as u32;
            write_volatile(opfccr, val);
        }
    }

    /// Configure foreground memory address / 配置前景内存地址
    pub fn set_foreground_address(&self, address: usize) {
        unsafe {
            let fgmar = (DMA2D_BASE + reg::FGMAR) as *mut u32;
            write_volatile(fgmar, address as u32);
        }
    }

    /// Configure background memory address / 配置背景内存地址
    pub fn set_background_address(&self, address: usize) {
        unsafe {
            let bgmar = (DMA2D_BASE + reg::BGMAR) as *mut u32;
            write_volatile(bgmar, address as u32);
        }
    }

    /// Configure output memory address / 配置输出内存地址
    pub fn set_output_address(&self, address: usize) {
        unsafe {
            let omar = (DMA2D_BASE + reg::OMAR) as *mut u32;
            write_volatile(omar, address as u32);
        }
    }

    /// Configure output size / 配置输出大小
    pub fn set_output_size(&self, width: u16, height: u16) {
        unsafe {
            let nlr = (DMA2D_BASE + reg::NLR) as *mut u32;
            let val = ((width as u32) << 16) | (height as u32);
            write_volatile(nlr, val);
        }
    }

    /// Configure foreground offset / 配置前景偏移
    pub fn set_foreground_offset(&self, offset: u16) {
        unsafe {
            let fgor = (DMA2D_BASE + reg::FGOR) as *mut u32;
            write_volatile(fgor, offset as u32);
        }
    }

    /// Configure background offset / 配置背景偏移
    pub fn set_background_offset(&self, offset: u16) {
        unsafe {
            let bgor = (DMA2D_BASE + reg::BGOR) as *mut u32;
            write_volatile(bgor, offset as u32);
        }
    }

    /// Configure output offset / 配置输出偏移
    pub fn set_output_offset(&self, offset: u16) {
        unsafe {
            let oor = (DMA2D_BASE + reg::OOR) as *mut u32;
            write_volatile(oor, offset as u32);
        }
    }

    /// Configure foreground color mode / 配置前景颜色模式
    pub fn set_foreground_color_mode(&self, format: PixelFormat) {
        unsafe {
            let fgpfccr = (DMA2D_BASE + reg::FGPFCCR) as *mut u32;
            let mut val = read_volatile(fgpfccr);
            val &= !(0b111 << 0);
            val |= format as u32;
            write_volatile(fgpfccr, val);
        }
    }

    /// Configure background color mode / 配置背景颜色模式
    pub fn set_background_color_mode(&self, format: PixelFormat) {
        unsafe {
            let bgpfccr = (DMA2D_BASE + reg::BGPFCCR) as *mut u32;
            let mut val = read_volatile(bgpfccr);
            val &= !(0b111 << 0);
            val |= format as u32;
            write_volatile(bgpfccr, val);
        }
    }

    /// Set output color for register-to-memory mode / 为寄存器到内存模式设置输出颜色
    pub fn set_output_color(&self, color: u32) {
        unsafe {
            let ocolr = (DMA2D_BASE + reg::OCOLR) as *mut u32;
            write_volatile(ocolr, color);
        }
    }

    /// Start transfer / 开始传输
    pub fn start(&self) {
        unsafe {
            let cr = (DMA2D_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val |= cr_bits::START;
            write_volatile(cr, val);
        }
    }

    /// Check if transfer is complete / 检查传输是否完成
    pub fn is_complete(&self) -> bool {
        unsafe {
            let isr = (DMA2D_BASE + reg::ISR) as *const u32;
            (read_volatile(isr) & isr_bits::TCIF) != 0
        }
    }

    /// Check for transfer error / 检查传输错误
    pub fn has_error(&self) -> bool {
        unsafe {
            let isr = (DMA2D_BASE + reg::ISR) as *const u32;
            (read_volatile(isr) & isr_bits::TEIF) != 0
        }
    }

    /// Clear transfer complete flag / 清除传输完成标志
    pub fn clear_complete(&self) {
        unsafe {
            let ifcr = (DMA2D_BASE + reg::IFCR) as *mut u32;
            write_volatile(ifcr, isr_bits::TCIF);
        }
    }

    /// Clear transfer error flag / 清除传输错误标志
    pub fn clear_error(&self) {
        unsafe {
            let ifcr = (DMA2D_BASE + reg::IFCR) as *mut u32;
            write_volatile(ifcr, isr_bits::TEIF);
        }
    }

    /// Simple copy operation / 简单复制操作
    pub fn copy(&self, src: usize, dst: usize, width: u16, height: u16) {
        self.set_mode(Mode::MemToMem);
        self.set_foreground_address(src);
        self.set_output_address(dst);
        self.set_output_size(width, height);
        self.set_foreground_offset(width);
        self.set_output_offset(width);
        self.start();
        while !self.is_complete() {}
        self.clear_complete();
    }

    /// Fill operation / 填充操作
    pub fn fill(&self, dst: usize, width: u16, height: u16, color: u32) {
        self.set_mode(Mode::RegToMem);
        self.set_output_address(dst);
        self.set_output_size(width, height);
        self.set_output_offset(width);
        self.set_output_color(color);
        self.start();
        while !self.is_complete() {}
        self.clear_complete();
    }

    /// Enable interrupts / 使能中断
    pub fn enable_interrupts(&self, transfer_complete: bool, transfer_error: bool) {
        unsafe {
            let cr = (DMA2D_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            if transfer_complete {
                val |= cr_bits::TCIE;
            }
            if transfer_error {
                val |= cr_bits::TEIE;
            }
            write_volatile(cr, val);
        }
    }

    /// Disable interrupts / 禁用中断
    pub fn disable_interrupts(&self) {
        unsafe {
            let cr = (DMA2D_BASE + reg::CR) as *mut u32;
            let mut val = read_volatile(cr);
            val &= !(cr_bits::TCIE | cr_bits::TEIE);
            write_volatile(cr, val);
        }
    }
}

/// Initialize DMA2D / 初始化 DMA2D
pub fn init_dma2d() {
    let dma2d = Dma2d::new();
    dma2d.set_mode(Mode::MemToMem);
}
