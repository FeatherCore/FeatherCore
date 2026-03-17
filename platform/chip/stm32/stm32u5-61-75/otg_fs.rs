//! USB OTG FS - USB On-The-Go Full-Speed Controller
//! USB 全速接口控制器
//!
//! # Overview / 概述
//! The USB OTG Full-Speed controller provides USB 2.0 full-speed (12 Mbps) connectivity
//! with built-in full-speed PHY.
//!
//! # Features / 功能特性
//! - USB 2.0 Full-Speed (12 Mbps)
//! - Built-in full-speed PHY
//! - 8 bidirectional endpoints
//! - Dedicated FIFO (1.25KB)
//! - DMA support
//! - SOF output
//! - Power management
//! - Battery charging detection (BCD)
//! - Device and Host modes
//!
//! # Reference / 参考
//! - RM0456 Chapter 72: USB on-the-go full-speed (OTG_FS)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// USB OTG FS base address / USB OTG FS 基地址
/// AHB2 bus, accessible at 0x4204_0000
/// Reference: RM0456 Chapter 2, Table 1
pub const USB_OTG_FS_BASE: usize = 0x4204_0000;

pub mod fs_reg {
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
    pub const DIEPTXF0: usize = 0x028;
    pub const HNPTXSTS: usize = 0x02C;
    pub const GCCFG: usize = 0x038;
    pub const CID: usize = 0x03C;
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
    pub const GINTMSK2: usize = 0x844;
}

pub mod endpoint_reg {
    pub const DIEPCTL0: usize = 0x900;
    pub const DIEPCTL1: usize = 0x920;
    pub const DIEPCTL2: usize = 0x940;
    pub const DIEPCTL3: usize = 0x960;
    pub const DIEPCTL4: usize = 0x980;
    pub const DIEPCTL5: usize = 0x9A0;
    pub const DIEPCTL6: usize = 0x9C0;
    pub const DIEPCTL7: usize = 0x9E0;
    pub const DOEPCTL0: usize = 0xB00;
    pub const DOEPCTL1: usize = 0xB20;
    pub const DOEPCTL2: usize = 0xB40;
    pub const DOEPCTL3: usize = 0xB60;
    pub const DOEPCTL4: usize = 0xB80;
    pub const DOEPCTL5: usize = 0xBA0;
    pub const DOEPCTL6: usize = 0xBC0;
    pub const DOEPCTL7: usize = 0xBE0;
}

#[derive(Clone, Copy, Debug)]
pub enum UsbFsSpeed {
    FullSpeed,
    LowSpeed,
}

#[derive(Clone, Copy, Debug)]
pub enum UsbFsDeviceAddr {
    Addr0 = 0,
    Addr1 = 1,
    Addr2 = 2,
    Addr3 = 3,
    Addr4 = 4,
    Addr5 = 5,
    Addr6 = 6,
    Addr7 = 7,
}

pub struct UsbFsConfig {
    pub speed: UsbFsSpeed,
    pub device_address: UsbFsDeviceAddr,
    pub dma_enable: bool,
    pub rx_fifo_size: u16,
    pub tx_fifo_size: u16,
}

impl Default for UsbFsConfig {
    fn default() -> Self {
        Self {
            speed: UsbFsSpeed::FullSpeed,
            device_address: UsbFsDeviceAddr::Addr0,
            dma_enable: true,
            rx_fifo_size: 512,
            tx_fifo_size: 256,
        }
    }
}

pub fn init() {
    unsafe {
        let rcc = (0x4002_1014 as *mut u32);
        let val = read_volatile(rcc);
        write_volatile(rcc, val | (1 << 25));
        
        let gotgctl = USB_OTG_FS_BASE as *mut u32;
        write_volatile(gotgctl, 0x0010_0000);
        
        let grstctl = (USB_OTG_FS_BASE + 0x10) as *mut u32;
        write_volatile(grstctl, 1 << 0);
        while read_volatile(grstctl) & (1 << 0) != 0 {}
        
        let gccfg = (USB_OTG_FS_BASE + 0x38) as *mut u32;
        write_volatile(gccfg, 1 << 19);
    }
}

pub fn configure(config: &UsbFsConfig) {
    unsafe {
        let dcfg = (USB_OTG_FS_BASE + 0x800) as *mut u32;
        let mut val: u32 = 0;
        
        match config.speed {
            UsbFsSpeed::FullSpeed => val |= 0 << 0,
            UsbFsSpeed::LowSpeed => val |= 1 << 0,
        }
        
        val |= (config.device_address as u32 & 0x7F) << 4;
        
        write_volatile(dcfg, val);
        
        let gahbcfg = (USB_OTG_FS_BASE + 0x08) as *mut u32;
        let ahb_val = read_volatile(gahbcfg);
        if config.dma_enable {
            write_volatile(gahbcfg, ahb_val | 1);
        } else {
            write_volatile(gahbcfg, ahb_val & !1);
        }
        
        let rxfsiz = (USB_OTG_FS_BASE + 0x24) as *mut u32;
        write_volatile(rxfsiz, config.rx_fifo_size as u32);
        
        let dieptxf0 = (USB_OTG_FS_BASE + 0x28) as *mut u32;
        write_volatile(dieptxf0, (config.tx_fifo_size as u32) << 16);
    }
}

pub fn enable_device() {
    unsafe {
        let dctl = (USB_OTG_FS_BASE + 0x804) as *mut u32;
        let val = read_volatile(dctl);
        write_volatile(dctl, val & !(1 << 0));
    }
}

pub fn disable_device() {
    unsafe {
        let dctl = (USB_OTG_FS_BASE + 0x804) as *mut u32;
        let val = read_volatile(dctl);
        write_volatile(dctl, val | (1 << 0));
    }
}

pub fn is_connected() -> bool {
    unsafe {
        let gotgctl = USB_OTG_FS_BASE as *const u32;
        (read_volatile(gotgctl) & (1 << 19)) != 0
    }
}

pub fn is_session_valid() -> bool {
    unsafe {
        let gotgctl = USB_OTG_FS_BASE as *const u32;
        (read_volatile(gotgctl) & (1 << 20)) != 0
    }
}

pub fn get_device_address() -> u8 {
    unsafe {
        let dcfg = (USB_OTG_FS_BASE + 0x800) as *const u32;
        ((read_volatile(dcfg) >> 4) & 0x7F) as u8
    }
}

pub fn set_device_address(addr: UsbFsDeviceAddr) {
    unsafe {
        let dcfg = (USB_OTG_FS_BASE + 0x800) as *mut u32;
        let val = read_volatile(dcfg);
        write_volatile(dcfg, (val & !(0x7F << 4)) | ((addr as u32 & 0x7F) << 4));
    }
}
