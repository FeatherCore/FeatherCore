//! DES - Device Electronic Signature
//! 设备电子签名
//!
//! # Overview / 概述
//! STM32U5 Device Electronic Signature (DES) contains factory-programmed information
//! including unique device ID, memory size, and calibration data.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 76: Device electronic signature (DES)
//! 
//! ## Device Information / 设备信息
//! - 96-bit unique device identifier (UID)
//! - Device ID code
//! - Flash memory size
//! - Package information
//! 
//! ## Calibration Data / 校准数据
//! - Internal voltage reference calibration
//! - Internal temperature sensor calibration
//! - Internal oscillator calibration
//! 
//! # Reference / 参考
//! - RM0456 Chapter 76: Device electronic signature (DES)
//! - RM0456 Section 76.1: DES introduction
//! - RM0456 Section 76.2: DES main features
//! - RM0456 Section 76.3: DES functional description
//! - RM0456 Section 76.4: DES registers

/// DES base address / DES 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const DES_BASE: usize = 0x420C_0000;

/// DES register offsets / DES 寄存器偏移
//! Reference: RM0456 Section 76.4: DES register map
pub mod reg {
    /// DES ID register / DES ID 寄存器
    //! Reference: RM0456 Section 76.4.1: Device electronic signature register (DES_IDR)
    pub const IDR: usize = 0x00;
    /// Flash size register / Flash 大小寄存器
    //! Reference: RM0456 Section 76.4.2: Flash size register (DES_FSR)
    pub const FSR: usize = 0x04;
}

/// Device Electronic Signature instance / 设备电子签名实例
pub struct Des;

impl Des {
    /// Create DES instance / 创建 DES 实例
    pub const fn new() -> Self {
        Self
    }

    /// Get 96-bit unique device ID (UID) / 获取96位设备唯一标识符
    /// 
    /// Returns a 96-bit (12 bytes) unique identifier that is factory-programmed
    /// and unique for each device.
    pub fn get_unique_id(&self) -> [u32; 3] {
        unsafe {
            let uid_base = 0x420C_7FE0 as *const u32;
            [
                core::ptr::read_volatile(uid_base),
                core::ptr::read_volatile(uid_base.add(1)),
                core::ptr::read_volatile(uid_base.add(2)),
            ]
        }
    }

    /// Get unique ID as bytes / 获取唯一ID为字节数组
    pub fn get_unique_id_bytes(&self) -> [u8; 12] {
        let id = self.get_unique_id();
        let mut bytes = [0u8; 12];
        
        bytes[0..4].copy_from_slice(&id[0].to_le_bytes());
        bytes[4..8].copy_from_slice(&id[1].to_le_bytes());
        bytes[8..12].copy_from_slice(&id[2].to_le_bytes());
        
        bytes
    }

    /// Get flash size in kilobytes / 获取 Flash 大小（KB）
    pub fn get_flash_size(&self) -> u16 {
        unsafe {
            let fsr = (DES_BASE + reg::FSR) as *const u16;
            core::ptr::read_volatile(fsr)
        }
    }

    /// Get package type / 获取封装类型
    pub fn get_package_type(&self) -> u8 {
        unsafe {
            let idr = (DES_BASE + reg::IDR) as *const u32;
            ((core::ptr::read_volatile(idr) >> 16) & 0x0F) as u8
        }
    }

    /// Get product revision / 获取产品修订版本
    pub fn get_product_revision(&self) -> u8 {
        unsafe {
            let idr = (DES_BASE + reg::IDR) as *const u32;
            ((core::ptr::read_volatile(idr) >> 20) & 0xFF) as u8
        }
    }

    /// Get device ID / 获取设备 ID
    pub fn get_device_id(&self) -> u16 {
        unsafe {
            let idr = (DES_BASE + reg::IDR) as *const u32;
            (core::ptr::read_volatile(idr) & 0x0FFF) as u16
        }
    }
}

/// Initialize DES / 初始化 DES
pub fn init_des_default() -> Des {
    Des::new()
}
