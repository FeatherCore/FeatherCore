//! USB OTG FS - USB On-The-Go Full Speed
//! USB 全速接口
//!
//! STM32U5 USB OTG FS 特性：
//! - USB 2.0 Full Speed (12 Mbps)
//! - 支持 Device、Host、OTG 模式
//! - 8 个双向端点
//! - 专用 1.25KB FIFO
//! - 支持 SOF 输出

/// USB OTG FS base address
pub const USB_OTG_FS_BASE: usize = 0x4204_0000;

/// USB OTG FS global registers
pub mod global_reg {
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
    pub const DIEPTXF0: usize = 0x028;
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
    pub const DIEPTXF1: usize = 0x104;
    pub const DIEPTXF2: usize = 0x108;
    pub const DIEPTXF3: usize = 0x10C;
    pub const DIEPTXF4: usize = 0x110;
    pub const DIEPTXF5: usize = 0x114;
    pub const DIEPTXF6: usize = 0x118;
    pub const DIEPTXF7: usize = 0x11C;
}

/// USB OTG FS device registers
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
    pub const DEACHINTMSK: usize = 0x83C;
    pub const DINEP1MSK: usize = 0x844;
}

/// USB OTG FS endpoint registers
pub mod ep_reg {
    // IN endpoints
    pub const DIEPCTL0: usize = 0x900;
    pub const DIEPINT0: usize = 0x908;
    pub const DIEPTSIZ0: usize = 0x910;
    pub const DIEPDMA0: usize = 0x914;
    pub const DTXFSTS0: usize = 0x918;
    
    // OUT endpoints
    pub const DOEPCTL0: usize = 0xB00;
    pub const DOEPINT0: usize = 0xB08;
    pub const DOEPTSIZ0: usize = 0xB10;
    pub const DOEPDMA0: usize = 0xB14;
}

/// USB speed
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UsbSpeed {
    FullSpeed = 3,
}

/// USB endpoint type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EpType {
    Control = 0,
    Isochronous = 1,
    Bulk = 2,
    Interrupt = 3,
}

/// USB endpoint direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EpDirection {
    Out = 0,
    In = 1,
}

/// USB device state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceState {
    Default,
    Addressed,
    Configured,
    Suspended,
}

/// USB setup packet
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SetupPacket {
    pub bm_request_type: u8,
    pub b_request: u8,
    pub w_value: u16,
    pub w_index: u16,
    pub w_length: u16,
}

/// USB instance
pub struct Usb;

impl Usb {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize USB OTG FS in device mode
    pub fn init_device(&self) {
        // Enable USB clock
        crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::USB_OTG_FS);

        unsafe {
            // Enable USB PHY clock
            let gccfg = (USB_OTG_FS_BASE + global_reg::GCCFG) as *mut u32;
            let mut val = core::ptr::read_volatile(gccfg);
            val |= 1 << 16; // PWRDWN
            val |= 1 << 19; // VBUSBSEN
            core::ptr::write_volatile(gccfg, val);

            // Wait for AHB idle
            let grstctl = (USB_OTG_FS_BASE + global_reg::GRSTCTL) as *mut u32;
            while (core::ptr::read_volatile(grstctl) & (1 << 31)) == 0 {}

            // Core soft reset
            let mut val = core::ptr::read_volatile(grstctl);
            val |= 1 << 0; // CSRST
            core::ptr::write_volatile(grstctl, val);
            while (core::ptr::read_volatile(grstctl) & (1 << 0)) != 0 {}

            // Wait for AHB idle again
            while (core::ptr::read_volatile(grstctl) & (1 << 31)) == 0 {}

            // Set device mode
            let gusbcfg = (USB_OTG_FS_BASE + global_reg::GUSBCFG) as *mut u32;
            let mut val = core::ptr::read_volatile(gusbcfg);
            val &= !(1 << 30); // Clear FHMOD
            val |= 1 << 30;    // Set FDMOD
            core::ptr::write_volatile(gusbcfg, val);

            // Wait for device mode
            let gotgctl = (USB_OTG_FS_BASE + global_reg::GOTGCTL) as *mut u32;
            while (core::ptr::read_volatile(gotgctl) & (1 << 21)) == 0 {}

            // Configure device
            let dcfg = (USB_OTG_FS_BASE + device_reg::DCFG) as *mut u32;
            let mut val = core::ptr::read_volatile(dcfg);
            val &= !(0b11 << 0); // Clear DSPD
            val |= (UsbSpeed::FullSpeed as u32) << 0;
            val &= !(0x7F << 4); // Clear DAD
            core::ptr::write_volatile(dcfg, val);

            // Configure FIFO sizes
            // RX FIFO
            let grxfsiz = (USB_OTG_FS_BASE + global_reg::GRXFSIZ) as *mut u32;
            core::ptr::write_volatile(grxfsiz, 128); // 128 words = 512 bytes

            // TX FIFO 0 (EP0)
            let dieptxf0 = (USB_OTG_FS_BASE + global_reg::DIEPTXF0) as *mut u32;
            core::ptr::write_volatile(dieptxf0, (64 << 16) | 128);

            // Flush FIFOs
            let grstctl = (USB_OTG_FS_BASE + global_reg::GRSTCTL) as *mut u32;
            let mut val = core::ptr::read_volatile(grstctl);
            val |= 0b101 << 5; // TXFNUM = 5 (all TX FIFOs), TXFFLSH
            val |= 1 << 4;     // RXFFLSH
            core::ptr::write_volatile(grstctl, val);
            while (core::ptr::read_volatile(grstctl) & ((1 << 5) | (1 << 4))) != 0 {}

            // Clear all pending interrupts
            let gintsts = (USB_OTG_FS_BASE + global_reg::GINTSTS) as *mut u32;
            core::ptr::write_volatile(gintsts, 0xFFFFFFFF);

            // Enable interrupts
            let gintmsk = (USB_OTG_FS_BASE + global_reg::GINTMSK) as *mut u32;
            let mut val = 0;
            val |= 1 << 12; // USBRST
            val |= 1 << 13; // ENUMDNE
            val |= 1 << 4;  // RXFLVL
            val |= 1 << 18; // IEPINT
            val |= 1 << 19; // OEPINT
            core::ptr::write_volatile(gintmsk, val);

            // Enable global interrupt
            let gahbcfg = (USB_OTG_FS_BASE + global_reg::GAHBCFG) as *mut u32;
            let mut val = core::ptr::read_volatile(gahbcfg);
            val |= 1 << 0; // GINTMSK
            core::ptr::write_volatile(gahbcfg, val);

            // Connect device
            let dctl = (USB_OTG_FS_BASE + device_reg::DCTL) as *mut u32;
            let mut val = core::ptr::read_volatile(dctl);
            val &= !(1 << 1); // Clear SDIS (soft disconnect)
            core::ptr::write_volatile(dctl, val);
        }
    }

    /// Configure endpoint 0 (control endpoint)
    pub fn configure_ep0(&self, max_packet_size: u16) {
        unsafe {
            // Configure IN endpoint 0
            let diepctl0 = (USB_OTG_FS_BASE + ep_reg::DIEPCTL0) as *mut u32;
            let mut val = core::ptr::read_volatile(diepctl0);
            val &= !(0b11 << 18); // Clear EPTYP
            val |= (EpType::Control as u32) << 18;
            val &= !(0x3FF << 0); // Clear MPSIZ
            val |= max_packet_size as u32;
            core::ptr::write_volatile(diepctl0, val);

            // Configure OUT endpoint 0
            let doepctl0 = (USB_OTG_FS_BASE + ep_reg::DOEPCTL0) as *mut u32;
            let mut val = core::ptr::read_volatile(doepctl0);
            val &= !(0b11 << 18);
            val |= (EpType::Control as u32) << 18;
            val &= !(0x3FF << 0);
            val |= max_packet_size as u32;
            val |= 1 << 31; // EPENA
            val |= 1 << 15; // USBAEP
            core::ptr::write_volatile(doepctl0, val);

            // Set up RX FIFO for SETUP packets
            let doeptsiz0 = (USB_OTG_FS_BASE + ep_reg::DOEPTSIZ0) as *mut u32;
            core::ptr::write_volatile(doeptsiz0, (3 << 29) | (8 << 19) | 64);
        }
    }

    /// Write data to IN endpoint
    pub fn write_ep(&self, ep_num: u8, data: &[u8]) {
        unsafe {
            // Write to FIFO
            let fifo = (USB_OTG_FS_BASE + 0x1000 + (ep_num as usize * 0x1000)) as *mut u32;
            
            for chunk in data.chunks(4) {
                let mut word: u32 = 0;
                for (i, &byte) in chunk.iter().enumerate() {
                    word |= (byte as u32) << (i * 8);
                }
                core::ptr::write_volatile(fifo, word);
            }

            // Set transfer size
            let dieptsiz = (USB_OTG_FS_BASE + ep_reg::DIEPTSIZ0 + (ep_num as usize * 0x20)) as *mut u32;
            let mut val = core::ptr::read_volatile(dieptsiz);
            val &= !(0x7F << 19); // Clear PKTCNT
            val |= 1 << 19;       // 1 packet
            val &= !(0x7FFFF << 0); // Clear XFRSIZ
            val |= data.len() as u32;
            core::ptr::write_volatile(dieptsiz, val);

            // Enable endpoint
            let diepctl = (USB_OTG_FS_BASE + ep_reg::DIEPCTL0 + (ep_num as usize * 0x20)) as *mut u32;
            let mut val = core::ptr::read_volatile(diepctl);
            val |= 1 << 31; // EPENA
            val |= 1 << 26; // CNAK
            core::ptr::write_volatile(diepctl, val);
        }
    }

    /// Read data from OUT endpoint
    pub fn read_ep(&self, ep_num: u8, buffer: &mut [u8]) -> usize {
        unsafe {
            let grxstsp = (USB_OTG_FS_BASE + global_reg::GRXSTSP) as *mut u32;
            let rx_status = core::ptr::read_volatile(grxstsp);
            
            let byte_count = ((rx_status >> 4) & 0x7FF) as usize;
            let pkt_status = (rx_status >> 17) & 0xF;

            if pkt_status == 0b0010 { // OUT data packet received
                let fifo = (USB_OTG_FS_BASE + 0x1000 + (ep_num as usize * 0x1000)) as *mut u32;
                
                let words_to_read = (byte_count + 3) / 4;
                for i in 0..words_to_read {
                    let word = core::ptr::read_volatile(fifo);
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
    }

    /// Set device address
    pub fn set_address(&self, address: u8) {
        unsafe {
            let dcfg = (USB_OTG_FS_BASE + device_reg::DCFG) as *mut u32;
            let mut val = core::ptr::read_volatile(dcfg);
            val &= !(0x7F << 4); // Clear DAD
            val |= (address as u32) << 4;
            core::ptr::write_volatile(dcfg, val);
        }
    }

    /// Get interrupt status
    pub fn get_interrupt_status(&self) -> u32 {
        unsafe {
            let gintsts = (USB_OTG_FS_BASE + global_reg::GINTSTS) as *mut u32;
            core::ptr::read_volatile(gintsts)
        }
    }

    /// Clear interrupt status
    pub fn clear_interrupt(&self, mask: u32) {
        unsafe {
            let gintsts = (USB_OTG_FS_BASE + global_reg::GINTSTS) as *mut u32;
            core::ptr::write_volatile(gintsts, mask);
        }
    }
}

/// USB device class trait
pub trait UsbDeviceClass {
    fn init(&mut self);
    fn setup(&mut self, setup: &SetupPacket) -> Option<&[u8]>;
    fn data_in(&mut self, ep_num: u8, data: &[u8]);
    fn data_out(&mut self, ep_num: u8, data: &[u8]);
}

/// Initialize USB in device mode
pub fn init_usb_device() {
    let usb = Usb::new();
    usb.init_device();
    usb.configure_ep0(64);
}
