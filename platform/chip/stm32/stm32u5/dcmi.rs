//! DCMI - Digital Camera Interface
//! 数字摄像头接口
//!
//! STM32U5 DCMI 特性：
//! - 支持 8/10/12/14-bit 并行接口
//! - 支持嵌入式或行/帧同步
//! - 支持连续或快照模式
//! - 支持裁剪功能
//! - 支持 JPEG 压缩
//! - 支持 DMA 传输

/// DCMI base address
pub const DCMI_BASE: usize = 0x4202_C000;

/// DCMI register offsets
pub mod reg {
    pub const CR: usize = 0x00;
    pub const SR: usize = 0x04;
    pub const RIS: usize = 0x08;
    pub const IER: usize = 0x0C;
    pub const MIS: usize = 0x10;
    pub const ICR: usize = 0x14;
    pub const ESCR: usize = 0x18;
    pub const ESUR: usize = 0x1C;
    pub const CWSTRT: usize = 0x20;
    pub const CWSIZE: usize = 0x24;
    pub const DR: usize = 0x28;
    pub const PCKSIZE: usize = 0x30;
}

/// DCMI capture mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CaptureMode {
    /// Continuous capture
    Continuous = 0,
    /// Single snapshot
    Snapshot = 1,
}

/// DCMI synchronization mode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncMode {
    /// Hardware synchronization (HSYNC/VSYNC)
    Hardware = 0,
    /// Embedded synchronization
    Embedded = 1,
}

/// DCMI pixel clock polarity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PclkPolarity {
    /// Falling edge active
    Falling = 0,
    /// Rising edge active
    Rising = 1,
}

/// DCMI vertical synchronization polarity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VsyncPolarity {
    /// Low active
    Low = 0,
    /// High active
    High = 1,
}

/// DCMI horizontal synchronization polarity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HsyncPolarity {
    /// Low active
    Low = 0,
    /// High active
    High = 1,
}

/// DCMI data width
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataWidth {
    /// 8 bits
    Bits8 = 0b00,
    /// 10 bits
    Bits10 = 0b01,
    /// 12 bits
    Bits12 = 0b10,
    /// 14 bits
    Bits14 = 0b11,
}

/// DCMI configuration
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
        }
    }
}

/// DCMI cropping configuration
#[derive(Clone, Copy, Debug)]
pub struct CropConfig {
    /// Horizontal offset
    pub h_offset: u16,
    /// Vertical offset
    pub v_offset: u16,
    /// Capture width
    pub width: u16,
    /// Capture height
    pub height: u16,
}

/// DCMI instance
pub struct Dcmi;

impl Dcmi {
    pub const fn new() -> Self {
        Self
    }

    /// Initialize DCMI
    pub fn init(&self, config: &Config) {
        // Enable DCMI clock
        crate::rcc::enable_ahb2_clock(crate::rcc::ahb2::DCMI);

        unsafe {
            // Configure CR
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = 0;
            val |= (config.capture_mode as u32) << 1;
            val |= (config.sync_mode as u32) << 5;
            val |= (config.pclk_polarity as u32) << 4;
            val |= (config.vsync_polarity as u32) << 3;
            val |= (config.hsync_polarity as u32) << 2;
            val |= (config.data_width as u32) << 10;
            if config.jpeg_mode {
                val |= 1 << 3;
            }
            if config.crop_enable {
                val |= 1 << 2;
            }
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Configure cropping
    pub fn configure_crop(&self, crop: &CropConfig) {
        unsafe {
            // Configure crop window start
            let cwstrt = (DCMI_BASE + reg::CWSTRT) as *mut u32;
            let mut val = 0;
            val |= (crop.h_offset as u32) << 16;
            val |= (crop.v_offset as u32) << 0;
            core::ptr::write_volatile(cwstrt, val);

            // Configure crop window size
            let cwsize = (DCMI_BASE + reg::CWSIZE) as *mut u32;
            let mut val = 0;
            val |= ((crop.width - 1) as u32) << 16;
            val |= ((crop.height - 1) as u32) << 0;
            core::ptr::write_volatile(cwsize, val);
        }
    }

    /// Enable DCMI
    pub fn enable(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 14; // DCMIEN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable DCMI
    pub fn disable(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 14);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Start capture
    pub fn start_capture(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // CAPTURE
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Stop capture
    pub fn stop_capture(&self) {
        unsafe {
            let cr = (DCMI_BASE + reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 0);
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Check if frame captured
    pub fn is_frame_captured(&self) -> bool {
        unsafe {
            let ris = (DCMI_BASE + reg::RIS) as *mut u32;
            let val = core::ptr::read_volatile(ris);
            (val & (1 << 0)) != 0 // FRAME_RIS
        }
    }

    /// Clear frame captured flag
    pub fn clear_frame_flag(&self) {
        unsafe {
            let icr = (DCMI_BASE + reg::ICR) as *mut u32;
            core::ptr::write_volatile(icr, 1 << 0);
        }
    }

    /// Enable interrupt
    pub fn enable_interrupt(&self, line_int: bool, vsync_int: bool, frame_int: bool) {
        unsafe {
            let ier = (DCMI_BASE + reg::IER) as *mut u32;
            let mut val = 0;
            if line_int {
                val |= 1 << 4;
            }
            if vsync_int {
                val |= 1 << 1;
            }
            if frame_int {
                val |= 1 << 0;
            }
            core::ptr::write_volatile(ier, val);
        }
    }

    /// Read data
    pub fn read_data(&self) -> u32 {
        unsafe {
            let dr = (DCMI_BASE + reg::DR) as *mut u32;
            core::ptr::read_volatile(dr)
        }
    }

    /// Check if data available
    pub fn is_data_available(&self) -> bool {
        unsafe {
            let ris = (DCMI_BASE + reg::RIS) as *mut u32;
            let val = core::ptr::read_volatile(ris);
            (val & (1 << 2)) != 0
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
    };

    dcmi.init(&config);
    dcmi.enable();
}

/// Initialize DCMI with DMA for continuous capture
pub fn init_dcmi_dma(frame_buffer: usize, width: u16, height: u16) {
    let dcmi = Dcmi::new();

    let config = Config::default();
    dcmi.init(&config);

    // Configure cropping if needed
    if width != 0 && height != 0 {
        let crop = CropConfig {
            h_offset: 0,
            v_offset: 0,
            width,
            height,
        };
        dcmi.configure_crop(&crop);
    }

    // Enable frame interrupt
    dcmi.enable_interrupt(false, false, true);

    // Enable DCMI
    dcmi.enable();

    // Start capture
    dcmi.start_capture();
}
