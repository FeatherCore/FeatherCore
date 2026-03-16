//! Device Driver Framework
//! Unix-like device driver interface
//! 
//! 设备驱动框架
//! 类 Unix 设备驱动接口

#![no_std]

extern crate alloc;

use alloc::string::String;
use core::any::Any;

/// Device type
/// 设备类型
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Block = 0,
    Character = 1,
    Network = 2,
}

/// Device operations
/// 设备操作
pub trait DeviceOps: Send + Sync {
    /// Open device
    /// 打开设备
    fn open(&self) -> Result<(), ()>;
    
    /// Close device
    /// 关闭设备
    fn close(&self) -> Result<(), ()>;
    
    /// Read from device
    /// 从设备读取
    fn read(&self, buf: &mut [u8]) -> Result<usize, ()>;
    
    /// Write to device
    /// 写入设备
    fn write(&self, buf: &[u8]) -> Result<usize, ()>;
    
    /// Control device (ioctl)
    /// 控制设备 (ioctl)
    fn ioctl(&self, cmd: u32, arg: usize) -> Result<usize, ()>;
}

/// Device structure
/// 设备结构
pub struct Device {
    pub name: String,
    pub dev_type: DeviceType,
    pub major: u32,
    pub minor: u32,
    pub ops: &'static dyn DeviceOps,
}

impl Device {
    pub fn new(
        name: String,
        dev_type: DeviceType,
        major: u32,
        minor: u32,
        ops: &'static dyn DeviceOps,
    ) -> Self {
        Device {
            name,
            dev_type,
            major,
            minor,
            ops,
        }
    }
}

/// Device manager
/// 设备管理器
pub struct DeviceManager {
    devices: [Option<Device>; 256],
}

impl DeviceManager {
    pub const fn new() -> Self {
        DeviceManager {
            devices: [None; 256],
        }
    }
    
    /// Register a device
    /// 注册设备
    pub fn register(&mut self, device: Device) -> Result<(), ()> {
        // Find empty slot
        // 查找空槽位
        for i in 0..256 {
            if self.devices[i].is_none() {
                self.devices[i] = Some(device);
                return Ok(());
            }
        }
        Err(())
    }
    
    /// Get device by major/minor number
    /// 通过主/次设备号获取设备
    pub fn get_device(&self, major: u32, minor: u32) -> Option<&Device> {
        self.devices.iter().find(|d| {
            if let Some(device) = d {
                device.major == major && device.minor == minor
            } else {
                false
            }
        })
    }
    
    /// Get device by name
    /// 通过名称获取设备
    pub fn get_device_by_name(&self, name: &str) -> Option<&Device> {
        self.devices.iter().find(|d| {
            if let Some(device) = d {
                device.name == name
            } else {
                false
            }
        })
    }
}

/// Global device manager
/// 全局设备管理器
static mut DEVICE_MANAGER: Option<DeviceManager> = None;

/// Initialize device manager
/// 初始化设备管理器
pub fn init() {
    unsafe {
        DEVICE_MANAGER = Some(DeviceManager::new());
    }
    crate::info!("Device manager initialized");
}

/// Get global device manager
/// 获取全局设备管理器
pub fn get_device_manager() -> &'static mut DeviceManager {
    unsafe {
        DEVICE_MANAGER.as_mut().expect("Device manager not initialized")
    }
}

/// Register a device
/// 注册设备
pub fn register_device(device: Device) -> Result<(), ()> {
    get_device_manager().register(device)
}
