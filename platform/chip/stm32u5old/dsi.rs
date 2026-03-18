//! DSI - Display Serial Interface
//! 显示串行接口
//!
//! ## STM32U5 DSI 特性 / Features
//! - **接口类型 / Interface Type: MIPI DSI v1.1
//! - **数据通道 / Data Lanes: 最多 4 个数据通道
//! - **时钟通道 / Clock Lane: 1 个时钟通道
//! - **视频模式 / Video Modes:
//!   - Non-burst mode with sync pulses
//!   - Non-burst mode with sync events
//!   - Burst mode
//!
//! - **命令模式 / Command Mode: 支持 DCS 和通用命令
//! - **特性 / Features:
//!   - Low-power mode (LPM)
//!   - Ultra-low-power mode (ULPM)
//!   - Tearing effect (TE) 信号支持
//!   - CRC 校验
//!   - ECC 纠错
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 44: Display serial interface (DSI)
//! - RM0456 Section 44.1: DSI introduction
//! - RM0456 Section 44.2: DSI main features
//! - RM0456 Section 44.3: DSI functional description
//! - RM0456 Section 44.4: DSI registers

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// DSI base address / DSI 基地址
pub const DSI_BASE: usize = 0x4201_7000;

/// DSI register offsets / DSI 寄存器偏移
//! Reference: RM0456 Section 44.4: DSI registers
pub mod reg {
    /// DSI Host Configuration Register / DSI 主机配置寄存器
    pub const HCR: usize = 0x00;
    /// DSI Version Register / DSI 版本寄存器
    pub const VCR: usize = 0x04;
    /// DSI Clock Control Register / DSI 时钟控制寄存器
    pub const CCR: usize = 0x08;
    /// DSI Low-Power Control Register / DSI 低功耗控制寄存器
    pub const LPCR: usize = 0x0C;
    /// DSI Power Control Register / DSI 电源控制寄存器
    pub const PCR: usize = 0x10;
    /// DSI Generic Header Configuration Register / DSI 通用头配置寄存器
    pub const GHCR: usize = 0x14;
    /// DSI Generic Payload Data Register 0 / DSI 通用有效载荷数据寄存器 0
    pub const GPDR: usize = 0x18;
    /// DSI Generic Payload Data Register 1 / DSI 通用有效载荷数据寄存器 1
    pub const GPSR: usize = 0x1C;
    /// DSI Host Timeout Counter Configuration Register / DSI 主机超时计数器配置寄存器
    pub const TCCR0: usize = 0x20;
    /// DSI Host Timeout Counter Configuration Register 1 / DSI 主机超时计数器配置寄存器 1
    pub const TCCR1: usize = 0x24;
    /// DSI Clock Lane Configuration Register / DSI 时钟通道配置寄存器
    pub const CLCR: usize = 0x28;
    /// DSI Clock Lane Timer Configuration Register / DSI 时钟通道定时器配置寄存器
    pub const CLTCR: usize = 0x2C;
    /// DSI D-PHY Configuration Register / DSI D-PHY 配置寄存器
    pub const DPC: usize = 0x30;
    /// DSI D-PHY Timeout Configuration Register / DSI D-PHY 超时配置寄存器
    pub const DPT: usize = 0x34;
    /// DSI D-PHY Status Register / DSI D-PHY 状态寄存器
    pub const DPSC: usize = 0x38;
    /// DSI Interrupt Enable Register 0 / DSI 中断使能寄存器 0
    pub const IER0: usize = 0x3C;
    /// DSI Interrupt Enable Register 1 / DSI 中断使能寄存器 1
    pub const IER1: usize = 0x40;
    /// DSI Status Register 0 / DSI 状态寄存器 0
    pub const SR0: usize = 0x44;
    /// DSI Status Register 1 / DSI 状态寄存器 1
    pub const SR1: usize = 0x48;
    /// DSI Interrupt Flag Clear Register 0 / DSI 中断标志清除寄存器 0
    pub const IFCR0: usize = 0x4C;
    /// DSI Interrupt Flag Clear Register 1 / DSI 中断标志清除寄存器 1
    pub const IFCR1: usize = 0x50;
    /// DSI Video Mode Configuration Register / DSI 视频模式配置寄存器
    pub const VMCR: usize = 0x54;
    /// DSI Video Packet Configuration Register / DSI 视频数据包配置寄存器
    pub const VPCR: usize = 0x58;
    /// DSI Video Mode Configuration Register 1 / DSI 视频模式配置寄存器 1
    pub const VCCR: usize = 0x5C;
    /// DSI Video Mode Configuration Register 2 / DSI 视频模式配置寄存器 2
    pub const VNPCR: usize = 0x60;
    /// DSI Video Mode Configuration Register 3 / DSI 视频模式配置寄存器 3
    pub const VHPCCR: usize = 0x64;
    /// DSI Video Mode Configuration Register 4 / DSI 视频模式配置寄存器 4
    pub const VVPCCR: usize = 0x68;
    /// DSI Video Mode Configuration Register 5 / DSI 视频模式配置寄存器 5
    pub const VFWCR: usize = 0x6C;
    /// DSI Wrapper Configuration Register / DSI 包装器配置寄存器
    pub const WCFGR: usize = 0x70;
    /// DSI Wrapper Control Register / DSI 包装器控制寄存器
    pub const WCR: usize = 0x74;
    /// DSI Wrapper Interrupt Enable Register / DSI 包装器中断使能寄存器
    pub const WIER: usize = 0x78;
    /// DSI Wrapper Status and Interrupt Flag Register / DSI 包装器状态和中断标志寄存器
    pub const WISR: usize = 0x7C;
    /// DSI Wrapper Interrupt Flag Clear Register / DSI 包装器中断标志清除寄存器
    pub const WIFCR: usize = 0x80;
}

/// DSI Video Mode / DSI 视频模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoMode {
    /// Non-burst mode with sync pulses / 带同步脉冲的非突发模式
    NonBurstSyncPulses = 0,
    /// Non-burst mode with sync events / 带同步事件的非突发模式
    NonBurstSyncEvents = 1,
    /// Burst mode / 突发模式
    Burst = 2,
}

/// DSI Color Coding / DSI 颜色编码
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorCoding {
    /// RGB565
    Rgb565 = 0,
    /// RGB666 packed
    Rgb666Packed = 1,
    /// RGB666 loosely packed
    Rgb666Loose = 2,
    /// RGB888
    Rgb888 = 3,
}

/// DSI Instance / DSI 实例
pub struct Dsi;

impl Dsi {
    /// Create DSI instance / 创建 DSI 实例
    pub const fn new() -> Self {
        Self
    }

    /// Initialize DSI Host / 初始化 DSI 主机
    pub fn init_host(&self) {
        unsafe {
            let hcr = (DSI_BASE + reg::HCR) as *mut u32;
            let mut val = read_volatile(hcr);
            val |= 1 << 0;
            write_volatile(hcr, val);
        }
    }

    /// Set video mode / 设置视频模式
    pub fn set_video_mode(&self, mode: VideoMode) {
        unsafe {
            let vmcr = (DSI_BASE + reg::VMCR) as *mut u32;
            let mut val = read_volatile(vmcr);
            val &= !(0b11 << 0);
            val |= (mode as u32) << 0;
            write_volatile(vmcr, val);
        }
    }

    /// Set color coding / 设置颜色编码
    pub fn set_color_coding(&self, coding: ColorCoding) {
        unsafe {
            let vpcr = (DSI_BASE + reg::VPCR) as *mut u32;
            let mut val = read_volatile(vpcr);
            val &= !(0b11 << 0);
            val |= (coding as u32) << 0;
            write_volatile(vpcr, val);
        }
    }

    /// Enable DSI / 使能 DSI
    pub fn enable(&self) {
        unsafe {
            let wcr = (DSI_BASE + reg::WCR) as *mut u32;
            let mut val = read_volatile(wcr);
            val |= 1 << 0;
            write_volatile(wcr, val);
        }
    }

    /// Disable DSI / 禁用 DSI
    pub fn disable(&self) {
        unsafe {
            let wcr = (DSI_BASE + reg::WCR) as *mut u32;
            let mut val = read_volatile(wcr);
            val &= !(1 << 0);
            write_volatile(wcr, val);
        }
    }

    /// Enable DPI / 使能 DPI
    pub fn enable_dpi(&self) {
        unsafe {
            let wcfgr = (DSI_BASE + reg::WCFGR) as *mut u32;
            let mut val = read_volatile(wcfgr);
            val |= 1 << 0;
            write_volatile(wcfgr, val);
        }
    }

    /// Disable DPI / 禁用 DPI
    pub fn disable_dpi(&self) {
        unsafe {
            let wcfgr = (DSI_BASE + reg::WCFGR) as *mut u32;
            let mut val = read_volatile(wcfgr);
            val &= !(1 << 0);
            write_volatile(wcfgr, val);
        }
    }

    /// Configure video parameters / 配置视频参数
    pub fn configure_video(&self, h_active: u16, v_active: u16, h_sync: u16, v_sync: u16, h_back_porch: u16, v_back_porch: u16) {
        unsafe {
            let vccr = (DSI_BASE + reg::VCCR) as *mut u32;
            write_volatile(vccr, h_active as u32);

            let vnpcr = (DSI_BASE + reg::VNPCR) as *mut u32;
            write_volatile(vnpcr, v_active as u32);

            let vhpccr = (DSI_BASE + reg::VHPCCR) as *mut u32;
            let mut val = (h_sync as u32) << 16;
            val |= h_back_porch as u32;
            write_volatile(vhpccr, val);

            let vvpccr = (DSI_BASE + reg::VVPCCR) as *mut u32;
            let mut val = (v_sync as u32) << 16;
            val |= v_back_porch as u32;
            write_volatile(vvpccr, val);
        }
    }

    /// Check if DSI is ready / 检查 DSI 是否就绪
    pub fn is_ready(&self) -> bool {
        unsafe {
            let wisr = (DSI_BASE + reg::WISR) as *const u32;
            (read_volatile(wisr) & (1 << 2)) != 0
        }
    }

    /// Enable low-power mode / 使能低功耗模式
    pub fn enable_low_power(&self) {
        unsafe {
            let lpcr = (DSI_BASE + reg::LPCR) as *mut u32;
            let mut val = read_volatile(lpcr);
            val |= 1 << 0;
            write_volatile(lpcr, val);
        }
    }

    /// Disable low-power mode / 禁用低功耗模式
    pub fn disable_low_power(&self) {
        unsafe {
            let lpcr = (DSI_BASE + reg::LPCR) as *mut u32;
            let mut val = read_volatile(lpcr);
            val &= !(1 << 0);
            write_volatile(lpcr, val);
        }
    }
}

/// Initialize DSI for MIPI display / 为 MIPI 显示器初始化 DSI
pub fn init_dsi_default() {
    let dsi = Dsi::new();
    dsi.init_host();
    dsi.set_video_mode(VideoMode::NonBurstSyncPulses);
    dsi.set_color_coding(ColorCoding::Rgb888);
    dsi.enable_dpi();
    dsi.enable();
}
