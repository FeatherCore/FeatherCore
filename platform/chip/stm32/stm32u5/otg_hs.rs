//! USB OTG HS - USB On-The-Go High-Speed Controller
//! USB 高速接口控制器
//!
//! # Overview / 概述
//! The USB OTG High-Speed controller provides USB 2.0 high-speed (480 Mbps) connectivity
//! with built-in high-speed PHY (available only on specific models: STM32U59x/5Ax/5Fx/5Gx).
//!
//! # Features / 功能特性
//! - USB 2.0 High-Speed (480 Mbps)
//! - Built-in high-speed PHY (特定型号)
//! - Dedicated FIFO
//! - DMA support
//! - High-speed support
//! - Power management
//! - Battery charging detection (BCD)
//! - Device and Host modes
//!
//! # Reference / 参考
//! - RM0456 Chapter 73: USB on-the-go high-speed (OTG_HS)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// USB OTG HS base address / USB OTG HS 基地址
/// AHB2 bus, accessible at 0x4204_4000
/// Reference: RM0456 Chapter 2, Table 1
/// Note: Only available on specific models (STM32U59x/5Ax/5Fx/5Gx)
pub const USB_OTG_HS_BASE: usize = 0x4204_4000;

pub mod hs_reg {
    pub const GOTGCTL: usize = 0x000;
    pub const GOTGINT: usize = 0x004;
    pub const GAHBCFG: usize = 0x008;
    pub const GUSBCFG: usize = 0x00C;
    pub const GRSTCTL: usize = 0x010;
    pub const GINTSTS: usize = 0x014;
    pub const GINTMSK: usize = 0x018;
    pub const GRXSTSR: usize = 0x01C;
    pub const GRXSTSP: usize = 0x020;
    pub const GRXFSIZ: usize = 0x024;
    pub const HNPTXFSIZ: usize = 0x028;
    pub const HNPTXSTS: usize = 0x02C;
    pub const GCCFG: usize = 0x038;
    pub const CID: usize = 0x03C;
    pub const GHWCFG1: usize = 0x044;
    pub const GHWCFG2: usize = 0x048;
    pub const GHWCFG3: usize = 0x04C;
    pub const GHWCFG4: usize = 0x050;
    pub const GLPMCFG: usize = 0x054;
    pub const GPWRDN: usize = 0x058;
    pub const GDFIFOCFG: usize = 0x05C;
    pub const HPTXFSIZ: usize = 0x100;
    pub const DIEPTXF1: usize = 0x104;
    pub const PCGCCTL: usize = 0xE00;
}

pub mod device_reg {
    pub const DCFG: usize = 0x800;
    pub const DCTL: usize = 0x804;
    pub const DSTS: usize = 0x808;
    pub const DIEPMSK: usize = 0x810;
    pub const DOEPMSK: usize = 0x814;
    pub const DAINT: usize = 0x818;
    pub const DAINTMSK: usize = 0x81C;
    pub const DVBUSDIS: usize = 0x828;
    pub const DVBUSPULSE: usize = 0x82C;
    pub const DTHRCTL: usize = 0x830;
    pub const DIEPEMPMSK: usize = 0x834;
    pub const DEACHINT: usize = 0x838;
}

pub mod host_reg {
    pub const HCFG: usize = 0x500;
    pub const HFIR: usize = 0x504;
    pub const HPTXSTS: usize = 0x508;
    pub const HAINT: usize = 0x514;
    pub const HAINTMSK: usize = 0x518;
    pub const HPRT: usize = 0x540;
}

#[derive(Clone, Copy, Debug)]
pub enum UsbHsSpeed {
    HighSpeed,
    FullSpeed,
    LowSpeed,
}

#[derive(Clone, Copy, Debug)]
pub enum UsbHsPhyType {
    Internal,
    External,
}

pub struct UsbHsConfig {
    pub speed: UsbHsSpeed,
    pub dma_enable: bool,
    pub phy_type: UsbHsPhyType,
    pub rx_fifo_size: u16,
    pub tx_fifo_size: u16,
    pub host_channel_count: u8,
}

impl Default for UsbHsConfig {
    fn default() -> Self {
        Self {
            speed: UsbHsSpeed::HighSpeed,
            dma_enable: true,
            phy_type: UsbHsPhyType::Internal,
            rx_fifo_size: 1024,
            tx_fifo_size: 512,
            host_channel_count: 8,
        }
    }
}

pub fn init() {
    unsafe {
        let rcc = (0x4002_1014 as *mut u32);
        let val = read_volatile(rcc);
        write_volatile(rcc, val | (1 << 27));
        
        let gotgctl = USB_OTG_HS_BASE as *mut u32;
        write_volatile(gotgctl, 0x0010_0000);
        
        let grstctl = (USB_OTG_HS_BASE + 0x10) as *mut u32;
        write_volatile(grstctl, 1 << 0);
        while read_volatile(grstctl) & (1 << 0) != 0 {}
        
        let gccfg = (USB_OTG_HS_BASE + 0x38) as *mut u32;
        write_volatile(gccfg, 1 << 16);
    }
}

pub fn configure(config: &UsbHsConfig) {
    unsafe {
        let dcfg = (USB_OTG_HS_BASE + 0x800) as *mut u32;
        let mut val: u32 = 0;
        
        match config.speed {
            UsbHsSpeed::HighSpeed => val |= 0 << 0,
            UsbHsSpeed::FullSpeed => val |= 1 << 0,
            UsbHsSpeed::LowSpeed => val |= 2 << 0,
        }
        
        write_volatile(dcfg, val);
        
        let gahbcfg = (USB_OTG_HS_BASE + 0x08) as *mut u32;
        let ahb_val = read_volatile(gahbcfg);
        if config.dma_enable {
            write_volatile(gahbcfg, ahb_val | 1);
        } else {
            write_volatile(gahbcfg, ahb_val & !1);
        }
        
        let rxfsiz = (USB_OTG_HS_BASE + 0x24) as *mut u32;
        write_volatile(rxfsiz, config.rx_fifo_size as u32);
        
        let dieptxf0 = (USB_OTG_HS_BASE + 0x28) as *mut u32;
        write_volatile(dieptxf0, (config.tx_fifo_size as u32) << 16);
    }
}

pub fn enable_device() {
    unsafe {
        let dctl = (USB_OTG_HS_BASE + 0x804) as *mut u32;
        let val = read_volatile(dctl);
        write_volatile(dctl, val & !(1 << 0));
    }
}

pub fn disable_device() {
    unsafe {
        let dctl = (USB_OTG_HS_BASE + 0x804) as *mut u32;
        let val = read_volatile(dctl);
        write_volatile(dctl, val | (1 << 0));
    }
}

pub fn enable_host() {
    unsafe {
        let hcfg = (USB_OTG_HS_BASE + 0x500) as *mut u32;
        let val = read_volatile(hcfg);
        write_volatile(hcfg, val & !(1 << 0));
    }
}

pub fn is_connected() -> bool {
    unsafe {
        let gotgctl = USB_OTG_HS_BASE as *const u32;
        (read_volatile(gotgctl) & (1 << 19)) != 0
    }
}

pub fn is_session_valid() -> bool {
    unsafe {
        let gotgctl = USB_OTG_HS_BASE as *const u32;
        (read_volatile(gotgctl) & (1 << 20)) != 0
    }
}

pub fn get_speed() -> UsbHsSpeed {
    unsafe {
        let dsts = (USB_OTG_HS_BASE + 0x808) as *const u32;
        let speed_val = read_volatile(dsts) & 0x3;
        match speed_val {
            0 => UsbHsSpeed::HighSpeed,
            1 => UsbHsSpeed::FullSpeed,
            2 => UsbHsSpeed::LowSpeed,
            _ => UsbHsSpeed::HighSpeed,
        }
    }
}
