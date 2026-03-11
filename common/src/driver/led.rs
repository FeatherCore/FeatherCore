//! LED driver for FeatherCore
//! FeatherCore LED 驱动
//! 
//! This module provides an LED driver that can be bound to a device tree node.
//! 此模块提供了一个可以绑定到设备树节点的 LED 驱动。

use crate::devicetree::{DeviceTreeNode, DriverBind};
use alloc::string::{String, ToString};

/// LED driver
/// LED 驱动
pub struct LedDriver {
    /// GPIO port
    /// GPIO 端口
    port: String,
    /// GPIO pin
    /// GPIO 引脚
    pin: u32,
    /// Active low flag
    /// 低电平有效标志
    active_low: bool,
}

impl LedDriver {
    /// Turn on the LED
    /// 打开 LED
    pub fn on(&self) {
        // TODO: Implement GPIO control
        // TODO: 实现 GPIO 控制
        // println!("Turning on LED on port {} pin {}", self.port, self.pin);
    }
    
    /// Turn off the LED
    /// 关闭 LED
    pub fn off(&self) {
        // TODO: Implement GPIO control
        // TODO: 实现 GPIO 控制
        // println!("Turning off LED on port {} pin {}", self.port, self.pin);
    }
    
    /// Toggle the LED
    /// 切换 LED 状态
    pub fn toggle(&self) {
        // TODO: Implement GPIO control
        // TODO: 实现 GPIO 控制
        // println!("Toggling LED on port {} pin {}", self.port, self.pin);
    }
}

impl DriverBind for LedDriver {
    fn bind(node: &DeviceTreeNode) -> Result<Self, String> {
        // Get port property
        let port = node.properties
            .iter()
            .find(|p| p.name == "port")
            .and_then(|p| match &p.value {
                crate::devicetree::PropertyValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .ok_or("Missing 'port' property".to_string())?;
        
        // Get pin property
        let pin = node.properties
            .iter()
            .find(|p| p.name == "pin")
            .and_then(|p| match &p.value {
                crate::devicetree::PropertyValue::Integer(i) => Some(*i as u32),
                _ => None,
            })
            .ok_or("Missing 'pin' property".to_string())?;
        
        // Get active-low property (default: false)
        let active_low = node.properties
            .iter()
            .find(|p| p.name == "active-low")
            .and_then(|p| match &p.value {
                crate::devicetree::PropertyValue::Boolean(b) => Some(*b),
                crate::devicetree::PropertyValue::Integer(i) => Some(*i != 0),
                _ => None,
            })
            .unwrap_or(false);
        
        Ok(Self {
            port,
            pin,
            active_low,
        })
    }
}