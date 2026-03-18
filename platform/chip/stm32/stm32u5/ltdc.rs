//! LTDC - LCD-TFT Display Controller
//! LCD-TFT 显示控制器
//!
//! ## STM32U5 LTDC 特性 / Features
//! - **显示输出 / Display Output:**
//!   - 24-bit RGB 并行输出
//!   - 8 位每像素 (RGB888)
//!
//! - **显示层 / Display Layers:**
//!   - 2 个独立显示层 (Layer 1, Layer 2)
//!   - 可编程窗口位置和尺寸
//!   - 硬件混合功能 (Blending) 支持透明度
//!   - 颜色查找表 (CLUT) 支持
//!
//! - **特性 / Features:**
//!   - 可编程 HSYNC, VSYNC, DE, PCLK 极性
//!   - 多种颜色格式支持 (ARGB8888, RGB888, RGB565, ARGB1555, ARGB4444, L8, AL44, AL88)
//!   - 垂直消隐中断支持
//!   - FIFO 错误检测
//!   - 可配置背景色
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 43: LCD-TFT display controller (LTDC)
//!   - Register map: RM0456, Section 43.7, pages 1823-1847
//!   - Synchronization Size Configuration Register (SSCR): RM0456, Section 43.7.2, page 1824
//!   - Global Control Register (GCR): RM0456, Section 43.7.5, page 1826
//!   - Layer Control Register (LxCR): RM0456, Section 43.7.12, page 1831
//!   - Interrupt management: RM0456, Section 43.5, page 1816

/// LTDC base address / LTDC 基地址
pub const LTDC_BASE: usize = 0x4001_6800;

/// LTDC register offsets
pub mod reg {
    /// LTDC Identification Register (IDR)
    /// RM0456, Section 43.7.1, page 1823
    pub const IDR: usize = 0x00;
    /// LTDC Synchronization Size Configuration Register (SSCR)
    /// RM0456, Section 43.7.2, page 1824
    pub const SSCR: usize = 0x08;
    /// LTDC Back Porch Configuration Register (BPCR)
    /// RM0456, Section 43.7.3, page 1825
    pub const BPCR: usize = 0x0C;
    /// LTDC Active Width Configuration Register (AWCR)
    /// RM0456, Section 43.7.4, page 1825
    pub const AWCR: usize = 0x10;
    /// LTDC Total Width Configuration Register (TWCR)
    /// RM0456, Section 43.7.5, page 1826
    pub const TWCR: usize = 0x14;
    /// LTDC Global Control Register (GCR)
    /// RM0456, Section 43.7.6, page 1826
    pub const GCR: usize = 0x18;
    /// LTDC Shadow Reload Configuration Register (SRCR)
    /// RM0456, Section 43.7.7, page 1828
    pub const SRCR: usize = 0x24;
    /// LTDC Background Color Configuration Register (BCCR)
    /// RM0456, Section 43.7.8, page 1829
    pub const BCCR: usize = 0x2C;
    /// LTDC Interrupt Enable Register (IER)
    /// RM0456, Section 43.7.9, page 1829
    pub const IER: usize = 0x34;
    /// LTDC Interrupt Status Register (ISR)
    /// RM0456, Section 43.7.10, page 1830
    pub const ISR: usize = 0x38;
    /// LTDC Interrupt Clear Register (ICR)
    /// RM0456, Section 43.7.11, page 1830
    pub const ICR: usize = 0x3C;
    /// LTDC Line Interrupt Position Configuration Register (LIPCR)
    /// RM0456, Section 43.7.12, page 1831
    pub const LIPCR: usize = 0x40;
    /// LTDC Current Position Status Register (CPSR)
    /// RM0456, Section 43.7.13, page 1832
    pub const CPSR: usize = 0x44;
    /// LTDC Current Display Status Register (CDSR)
    /// RM0456, Section 43.7.14, page 1833
    pub const CDSR: usize = 0x48;
}

/// LTDC register bit definitions
pub mod bits {
    /// Global Control Register (GCR) bits
    pub mod gcr {
        /// LTDC Enable (LTDCEN)
        pub const LTDCEN: u32 = 1 << 0;
        /// Dithering Enable (DEN)
        pub const DEN: u32 = 1 << 16;
        /// Dither Blue Width (DBW)
        pub const DBW: u32 = 0b111 << 28;
        /// Dither Green Width (DGW)
        pub const DGW: u32 = 0b111 << 24;
        /// Dither Red Width (DRW)
        pub const DRW: u32 = 0b111 << 20;
    }

    /// Shadow Reload Configuration Register (SRCR) bits
    pub mod srcr {
        /// Immediate Reload (IMR)
        pub const IMR: u32 = 1 << 0;
        /// Vertical Blanking Reload (VBR)
        pub const VBR: u32 = 1 << 1;
    }

    /// Interrupt Enable Register (IER) bits
    pub mod ier {
        /// Line Interrupt Enable (LIE)
        pub const LIE: u32 = 1 << 0;
        /// FIFO Underrun Interrupt Enable (FUIE)
        pub const FUIE: u32 = 1 << 1;
        /// Transfer Error Interrupt Enable (TERRIE)
        pub const TERRIE: u32 = 1 << 2;
        /// Register Reload Interrupt Enable (RRIE)
        pub const RRIE: u32 = 1 << 3;
    }

    /// Interrupt Status Register (ISR) bits
    pub mod isr {
        /// Line Interrupt Flag (LIF)
        pub const LIF: u32 = 1 << 0;
        /// FIFO Underrun Interrupt Flag (FUIF)
        pub const FUIF: u32 = 1 << 1;
        /// Transfer Error Interrupt Flag (TERRIF)
        pub const TERRIF: u32 = 1 << 2;
        /// Register Reload Interrupt Flag (RRIF)
        pub const RRIF: u32 = 1 << 3;
    }

    /// Interrupt Clear Register (ICR) bits
    pub mod icr {
        /// Clear Line Interrupt Flag (CLIF)
        pub const CLIF: u32 = 1 << 0;
        /// Clear FIFO Underrun Interrupt Flag (CFUIF)
        pub const CFUIF: u32 = 1 << 1;
        /// Clear Transfer Error Interrupt Flag (CTERRIF)
        pub const CTERRIF: u32 = 1 << 2;
        /// Clear Register Reload Interrupt Flag (CRRIF)
        pub const CRRIF: u32 = 1 << 3;
    }

    /// Current Display Status Register (CDSR) bits
    pub mod cdsr {
        /// Vertical Blanking Status (VDE)
        pub const VDES: u32 = 1 << 0;
        /// Horizontal Blanking Status (HDES)
        pub const HDES: u32 = 1 << 1;
        /// Vertical Sync Status (VSYNCS)
        pub const VSYNCS: u32 = 1 << 2;
        /// Horizontal Sync Status (HSYNCS)
        pub const HSYNCS: u32 = 1 << 3;
    }

    /// Layer Control Register (LxCR) bits
    pub mod lxcr {
        /// Layer Enable (LEN)
        pub const LEN: u32 = 1 << 0;
        /// Color Keying Enable (COLKEN)
        pub const COLKEN: u32 = 1 << 1;
        /// CLUT Enable (CLUTEN)
        pub const CLUTEN: u32 = 1 << 4;
    }

    /// Layer Pixel Format Configuration Register (LxPFCR) bits
    pub mod lxpfcr {
        /// Pixel Format (PF)
        pub const PF_MASK: u32 = 0b111 << 0;
        pub const PF_ARGB8888: u32 = 0b000 << 0;
        pub const PF_RGB888: u32 = 0b001 << 0;
        pub const PF_RGB565: u32 = 0b010 << 0;
        pub const PF_ARGB1555: u32 = 0b011 << 0;
        pub const PF_ARGB4444: u32 = 0b100 << 0;
        pub const PF_L8: u32 = 0b101 << 0;
        pub const PF_AL44: u32 = 0b110 << 0;
        pub const PF_AL88: u32 = 0b111 << 0;
    }

    /// Layer Blending Factors Configuration Register (LxBFCR) bits
    pub mod lxbfcr {
        /// Blending Factor 1 (BF1)
        pub const BF1_MASK: u32 = 0b111 << 8;
        pub const BF1_CONSTANT_ALPHA: u32 = 0b100 << 8;
        pub const BF1_PIXEL_ALPHA: u32 = 0b110 << 8;
        /// Blending Factor 2 (BF2)
        pub const BF2_MASK: u32 = 0b111 << 0;
        pub const BF2_CONSTANT_ALPHA: u32 = 0b101 << 0;
        pub const BF2_PIXEL_ALPHA: u32 = 0b111 << 0;
    }
}

/// Layer register offsets (Layer 1 base)
/// Layer 2 offsets = Layer 1 offset + 0x80
pub mod layer_reg {
    /// Layer x Control Register (LxCR)
    /// RM0456, Section 43.7.15, page 1834
    pub const L1CR: usize = 0x84;
    /// Layer x Window Horizontal Position Configuration Register (LxWHPCR)
    /// RM0456, Section 43.7.16, page 1834
    pub const L1WHPCR: usize = 0x88;
    /// Layer x Window Vertical Position Configuration Register (LxWVPCR)
    /// RM0456, Section 43.7.17, page 1835
    pub const L1WVPCR: usize = 0x8C;
    /// Layer x Color Keying Configuration Register (LxCKCR)
    /// RM0456, Section 43.7.18, page 1836
    pub const L1CKCR: usize = 0x90;
    /// Layer x Pixel Format Configuration Register (LxPFCR)
    /// RM0456, Section 43.7.19, page 1836
    pub const L1PFCR: usize = 0x94;
    /// Layer x Constant Alpha Configuration Register (LxCACR)
    /// RM0456, Section 43.7.20, page 1837
    pub const L1CACR: usize = 0x98;
    /// Layer x Default Color Configuration Register (LxDCCR)
    /// RM0456, Section 43.7.21, page 1837
    pub const L1DCCR: usize = 0x9C;
    /// Layer x Blending Factors Configuration Register (LxBFCR)
    /// RM0456, Section 43.7.22, page 1838
    pub const L1BFCR: usize = 0xA0;
    /// Layer x Color Frame Buffer Address Register (LxCFBAR)
    /// RM0456, Section 43.7.23, page 1839
    pub const L1CFBAR: usize = 0xAC;
    /// Layer x Color Frame Buffer Length Register (LxCFBLR)
    /// RM0456, Section 43.7.24, page 1840
    pub const L1CFBLR: usize = 0xB0;
    /// Layer x Color Frame Buffer Line Number Register (LxCFBLNR)
    /// RM0456, Section 43.7.25, page 1840
    pub const L1CFBLNR: usize = 0xB4;
    /// Layer x CLUT Write Register (LxCLUTWR)
    /// RM0456, Section 43.7.26, page 1841
    pub const L1CLUTWR: usize = 0xC4;
}

/// Layer 2 offset
pub const LAYER2_OFFSET: usize = 0x80;

/// LTDC layer
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layer {
    Layer1 = 0,
    Layer2 = 1,
}

/// Pixel format
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
    /// L8 (8-bit luminance)
    L8 = 0b101,
    /// AL44 (4-bit alpha, 4-bit luminance)
    Al44 = 0b110,
    /// AL88 (8-bit alpha, 8-bit luminance)
    Al88 = 0b111,
}

/// Display timing configuration
#[derive(Clone, Copy, Debug)]
pub struct TimingConfig {
    /// Horizontal sync width
    pub hsync: u16,
    /// Horizontal back porch
    pub hbp: u16,
    /// Active width (display resolution width)
    pub active_width: u16,
    /// Total width
    pub total_width: u16,
    /// Vertical sync height
    pub vsync: u16,
    /// Vertical back porch
    pub vbp: u16,
    /// Active height (display resolution height)
    pub active_height: u16,
    /// Total height
    pub total_height: u16,
}

/// Layer configuration
#[derive(Clone, Copy, Debug)]
pub struct LayerConfig {
    /// Window horizontal start position
    pub window_x0: u16,
    /// Window horizontal stop position
    pub window_x1: u16,
    /// Window vertical start position
    pub window_y0: u16,
    /// Window vertical stop position
    pub window_y1: u16,
    /// Pixel format
    pub pixel_format: PixelFormat,
    /// Frame buffer start address
    pub frame_buffer: usize,
    /// Frame buffer line length (in bytes + 3)
    pub line_length: u16,
    /// Frame buffer line count
    pub line_count: u16,
    /// Constant alpha value (0-255)
    pub constant_alpha: u8,
}

/// LTDC instance
pub struct Ltdc;

impl Ltdc {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize LTDC with timing configuration
    pub fn init(&self, timing: &TimingConfig) {
        // Enable LTDC clock
        crate::rcc::enable_apb2_clock(crate::rcc::apb2::LTDC);

        unsafe {
            // Configure SSCR
            let sscr = (LTDC_BASE + reg::SSCR) as *mut u32;
            let mut val = 0;
            val |= (timing.hsync as u32 - 1) << 16;
            val |= (timing.vsync as u32 - 1) << 0;
            core::ptr::write_volatile(sscr, val);

            // Configure BPCR
            let bpcr = (LTDC_BASE + reg::BPCR) as *mut u32;
            let mut val = 0;
            val |= (timing.hsync as u32 + timing.hbp as u32 - 1) << 16;
            val |= (timing.vsync as u32 + timing.vbp as u32 - 1) << 0;
            core::ptr::write_volatile(bpcr, val);

            // Configure AWCR
            let awcr = (LTDC_BASE + reg::AWCR) as *mut u32;
            let mut val = 0;
            val |= (timing.hsync as u32 + timing.hbp as u32 + timing.active_width as u32 - 1) << 16;
            val |= (timing.vsync as u32 + timing.vbp as u32 + timing.active_height as u32 - 1) << 0;
            core::ptr::write_volatile(awcr, val);

            // Configure TWCR
            let twcr = (LTDC_BASE + reg::TWCR) as *mut u32;
            let mut val = 0;
            val |= (timing.total_width as u32 - 1) << 16;
            val |= (timing.total_height as u32 - 1) << 0;
            core::ptr::write_volatile(twcr, val);
        }
    }

    /// Configure layer
    pub fn configure_layer(&self, layer: Layer, config: &LayerConfig) {
        unsafe {
            let layer_offset = if layer == Layer::Layer1 { 0 } else { LAYER2_OFFSET };

            // Configure window horizontal position
            let whpcr = (LTDC_BASE + layer_reg::L1WHPCR + layer_offset) as *mut u32;
            let mut val = 0;
            val |= (config.window_x1 as u32) << 16;
            val |= (config.window_x0 as u32) << 0;
            core::ptr::write_volatile(whpcr, val);

            // Configure window vertical position
            let wvpcr = (LTDC_BASE + layer_reg::L1WVPCR + layer_offset) as *mut u32;
            let mut val = 0;
            val |= (config.window_y1 as u32) << 16;
            val |= (config.window_y0 as u32) << 0;
            core::ptr::write_volatile(wvpcr, val);

            // Configure pixel format
            let pfcr = (LTDC_BASE + layer_reg::L1PFCR + layer_offset) as *mut u32;
            core::ptr::write_volatile(pfcr, config.pixel_format as u32);

            // Configure constant alpha
            let cacr = (LTDC_BASE + layer_reg::L1CACR + layer_offset) as *mut u32;
            core::ptr::write_volatile(cacr, config.constant_alpha as u32);

            // Configure frame buffer address
            let cfbar = (LTDC_BASE + layer_reg::L1CFBAR + layer_offset) as *mut u32;
            core::ptr::write_volatile(cfbar, config.frame_buffer as u32);

            // Configure frame buffer line length
            let cfblr = (LTDC_BASE + layer_reg::L1CFBLR + layer_offset) as *mut u32;
            let mut val = 0;
            val |= (config.line_length as u32) << 16;
            val |= (config.line_length as u32 + 3) << 0;
            core::ptr::write_volatile(cfblr, val);

            // Configure frame buffer line count
            let cfblnr = (LTDC_BASE + layer_reg::L1CFBLNR + layer_offset) as *mut u32;
            core::ptr::write_volatile(cfblnr, config.line_count as u32);
        }
    }

    /// Enable layer
    pub fn enable_layer(&self, layer: Layer) {
        unsafe {
            let layer_offset = if layer == Layer::Layer1 { 0 } else { LAYER2_OFFSET };
            let lcr = (LTDC_BASE + layer_reg::L1CR + layer_offset) as *mut u32;
            let mut val = core::ptr::read_volatile(lcr);
            val |= 1 << 0; // LEN
            core::ptr::write_volatile(lcr, val);
        }
    }

    /// Disable layer
    pub fn disable_layer(&self, layer: Layer) {
        unsafe {
            let layer_offset = if layer == Layer::Layer1 { 0 } else { LAYER2_OFFSET };
            let lcr = (LTDC_BASE + layer_reg::L1CR + layer_offset) as *mut u32;
            let mut val = core::ptr::read_volatile(lcr);
            val &= !(1 << 0);
            core::ptr::write_volatile(lcr, val);
        }
    }

    /// Enable LTDC
    pub fn enable(&self) {
        unsafe {
            let gcr = (LTDC_BASE + reg::GCR) as *mut u32;
            let mut val = core::ptr::read_volatile(gcr);
            val |= 1 << 0; // LTDCEN
            core::ptr::write_volatile(gcr, val);
        }
    }

    /// Disable LTDC
    pub fn disable(&self) {
        unsafe {
            let gcr = (LTDC_BASE + reg::GCR) as *mut u32;
            let mut val = core::ptr::read_volatile(gcr);
            val &= !(1 << 0);
            core::ptr::write_volatile(gcr, val);
        }
    }

    /// Set background color
    pub fn set_background_color(&self, red: u8, green: u8, blue: u8) {
        unsafe {
            let bccr = (LTDC_BASE + reg::BCCR) as *mut u32;
            let mut val = 0;
            val |= (red as u32) << 16;
            val |= (green as u32) << 8;
            val |= (blue as u32) << 0;
            core::ptr::write_volatile(bccr, val);
        }
    }

    /// Reload shadow registers
    pub fn reload(&self, immediate: bool) {
        unsafe {
            let srcr = (LTDC_BASE + reg::SRCR) as *mut u32;
            if immediate {
                core::ptr::write_volatile(srcr, 1 << 0); // IMR
            } else {
                core::ptr::write_volatile(srcr, 1 << 1); // VBR
            }
        }
    }

    /// Check if vertical blanking
    /// RM0456, Section 43.7.14, page 1833: CDSR register
    pub fn is_vblank(&self) -> bool {
        unsafe {
            let cdsr = (LTDC_BASE + reg::CDSR) as *mut u32;
            let val = core::ptr::read_volatile(cdsr);
            (val & bits::cdsr::VDES) != 0
        }
    }

    /// Enable dithering
    /// RM0456, Section 43.7.6, page 1826: GCR register, DEN bit
    pub fn enable_dithering(&self) {
        unsafe {
            let gcr = (LTDC_BASE + reg::GCR) as *mut u32;
            let mut val = core::ptr::read_volatile(gcr);
            val |= bits::gcr::DEN;
            core::ptr::write_volatile(gcr, val);
        }
    }

    /// Disable dithering
    pub fn disable_dithering(&self) {
        unsafe {
            let gcr = (LTDC_BASE + reg::GCR) as *mut u32;
            let mut val = core::ptr::read_volatile(gcr);
            val &= !bits::gcr::DEN;
            core::ptr::write_volatile(gcr, val);
        }
    }

    /// Enable interrupts
    /// RM0456, Section 43.7.9, page 1829: IER register
    pub fn enable_interrupts(&self, line: bool, fifo_underrun: bool, transfer_error: bool, register_reload: bool) {
        unsafe {
            let ier = (LTDC_BASE + reg::IER) as *mut u32;
            let mut val = 0;
            if line { val |= bits::ier::LIE; }
            if fifo_underrun { val |= bits::ier::FUIE; }
            if transfer_error { val |= bits::ier::TERRIE; }
            if register_reload { val |= bits::ier::RRIE; }
            core::ptr::write_volatile(ier, val);
        }
    }

    /// Disable all interrupts
    pub fn disable_all_interrupts(&self) {
        unsafe {
            let ier = (LTDC_BASE + reg::IER) as *mut u32;
            core::ptr::write_volatile(ier, 0);
        }
    }

    /// Get interrupt status
    /// RM0456, Section 43.7.10, page 1830: ISR register
    pub fn get_interrupt_status(&self) -> (bool, bool, bool, bool) {
        unsafe {
            let isr = (LTDC_BASE + reg::ISR) as *mut u32;
            let val = core::ptr::read_volatile(isr);
            (
                (val & bits::isr::LIF) != 0,
                (val & bits::isr::FUIF) != 0,
                (val & bits::isr::TERRIF) != 0,
                (val & bits::isr::RRIF) != 0,
            )
        }
    }

    /// Clear interrupt flags
    /// RM0456, Section 43.7.11, page 1830: ICR register
    pub fn clear_interrupt_flags(&self, line: bool, fifo_underrun: bool, transfer_error: bool, register_reload: bool) {
        unsafe {
            let icr = (LTDC_BASE + reg::ICR) as *mut u32;
            let mut val = 0;
            if line { val |= bits::icr::CLIF; }
            if fifo_underrun { val |= bits::icr::CFUIF; }
            if transfer_error { val |= bits::icr::CTERRIF; }
            if register_reload { val |= bits::icr::CRRIF; }
            core::ptr::write_volatile(icr, val);
        }
    }

    /// Set line interrupt position
    /// RM0456, Section 43.7.12, page 1831: LIPCR register
    pub fn set_line_interrupt_position(&self, line: u16) {
        unsafe {
            let lipcr = (LTDC_BASE + reg::LIPCR) as *mut u32;
            core::ptr::write_volatile(lipcr, line as u32);
        }
    }

    /// Configure blending factors for a layer
    /// RM0456, Section 43.7.22, page 1838: LxBFCR register
    pub fn configure_blending(&self, layer: Layer, bf1_constant: bool, bf2_constant: bool) {
        unsafe {
            let layer_offset = if layer == Layer::Layer1 { 0 } else { LAYER2_OFFSET };
            let bfcr = (LTDC_BASE + layer_reg::L1BFCR + layer_offset) as *mut u32;
            let mut val = 0;
            val |= if bf1_constant { bits::lxbfcr::BF1_CONSTANT_ALPHA } else { bits::lxbfcr::BF1_PIXEL_ALPHA };
            val |= if bf2_constant { bits::lxbfcr::BF2_CONSTANT_ALPHA } else { bits::lxbfcr::BF2_PIXEL_ALPHA };
            core::ptr::write_volatile(bfcr, val);
        }
    }
}

/// Common display resolutions
pub mod resolutions {
    use super::TimingConfig;

    /// 480x272 (4.3" display)
    pub const WVGA_480X272: TimingConfig = TimingConfig {
        hsync: 41,
        hbp: 2,
        active_width: 480,
        total_width: 525,
        vsync: 10,
        vbp: 2,
        active_height: 272,
        total_height: 286,
    };

    /// 800x480 (5" display)
    pub const WVGA_800X480: TimingConfig = TimingConfig {
        hsync: 128,
        hbp: 88,
        active_width: 800,
        total_width: 1056,
        vsync: 2,
        vbp: 32,
        active_height: 480,
        total_height: 525,
    };

    /// 640x480 (VGA)
    pub const VGA_640X480: TimingConfig = TimingConfig {
        hsync: 96,
        hbp: 48,
        active_width: 640,
        total_width: 800,
        vsync: 2,
        vbp: 33,
        active_height: 480,
        total_height: 525,
    };
}

/// Initialize LTDC for 480x272 display
pub fn init_ltdc_480x272(frame_buffer: usize) {
    let ltdc = Ltdc::new();

    // Initialize LTDC timing
    ltdc.init(&resolutions::WVGA_480X272);

    // Configure layer 1
    let layer_config = LayerConfig {
        window_x0: 0,
        window_x1: 480,
        window_y0: 0,
        window_y1: 272,
        pixel_format: PixelFormat::Rgb565,
        frame_buffer,
        line_length: 480 * 2,
        line_count: 272,
        constant_alpha: 255,
    };

    ltdc.configure_layer(Layer::Layer1, &layer_config);
    ltdc.enable_layer(Layer::Layer1);

    // Set background color to black
    ltdc.set_background_color(0, 0, 0);

    // Enable LTDC
    ltdc.enable();
}
