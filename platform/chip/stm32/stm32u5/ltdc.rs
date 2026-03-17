//! LTDC - LCD-TFT Display Controller
//! LCD-TFT 显示控制器
//!
//! ## STM32U5 LTDC 特性 / Features
//! - **显示输出 / Display Output:**
//!   - 24-bit RGB 并行输出
//!   - 8 位每像素 (RGB888)
//!
//! - **显示层 / Display Layers:**
//!   - 2 个独立显示层
//!   - 窗口和混合功能 (Blending)
//!
//! - **特性 / Features:**
//!   - 可编程极性 (Programmable polarity)
//!   - 多种颜色格式支持
//!   - 硬件加速
//!   - 中断支持
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 56: LCD-TFT display controller (LTDC)

/// LTDC base address / LTDC 基地址
pub const LTDC_BASE: usize = 0x4001_6800;

/// LTDC register offsets
pub mod reg {
    /// LTDC Synchronization Size Configuration Register
    pub const SSCR: usize = 0x08;
    /// LTDC Back Porch Configuration Register
    pub const BPCR: usize = 0x0C;
    /// LTDC Active Width Configuration Register
    pub const AWCR: usize = 0x10;
    /// LTDC Total Width Configuration Register
    pub const TWCR: usize = 0x14;
    /// LTDC Global Control Register
    pub const GCR: usize = 0x18;
    /// LTDC Shadow Reload Configuration Register
    pub const SRCR: usize = 0x24;
    /// LTDC Background Color Configuration Register
    pub const BCCR: usize = 0x2C;
    /// LTDC Interrupt Enable Register
    pub const IER: usize = 0x34;
    /// LTDC Interrupt Status Register
    pub const ISR: usize = 0x38;
    /// LTDC Interrupt Clear Register
    pub const ICR: usize = 0x3C;
    /// LTDC Line Interrupt Position Configuration Register
    pub const LIPCR: usize = 0x40;
    /// LTDC Current Position Status Register
    pub const CPSR: usize = 0x44;
    /// LTDC Current Display Status Register
    pub const CDSR: usize = 0x48;
}

/// Layer register offsets (Layer 1 base)
pub mod layer_reg {
    pub const L1CR: usize = 0x84;
    pub const L1WHPCR: usize = 0x88;
    pub const L1WVPCR: usize = 0x8C;
    pub const L1CKCR: usize = 0x90;
    pub const L1PFCR: usize = 0x94;
    pub const L1CACR: usize = 0x98;
    pub const L1DCCR: usize = 0x9C;
    pub const L1BFCR: usize = 0xA0;
    pub const L1CFBAR: usize = 0xAC;
    pub const L1CFBLR: usize = 0xB0;
    pub const L1CFBLNR: usize = 0xB4;
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
    pub fn is_vblank(&self) -> bool {
        unsafe {
            let cdsr = (LTDC_BASE + reg::CDSR) as *mut u32;
            let val = core::ptr::read_volatile(cdsr);
            (val & (1 << 0)) != 0
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
