//! GTZC - Global TrustZone Controller / 全局 TrustZone 控制器
//!
//! ## STM32U5 GTZC 特性 / Features
//! - **TZIC**: TrustZone Interrupt Controller / TrustZone 中断控制器
//! - **MPCWM**: Memory Protection Controller Wrapper / 存储器保护控制器包装器
//! - **TDC**: TrustZone Decorator / TrustZone 装饰器
//! - **ETZPC**: Extended TrustZone Peripheral Controller / 扩展 TrustZone 外设控制器
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 3: Global TrustZone controller (GTZC)
//! - STM32U5 Reference Manual for specific GTZC features

#![no_std]

use core::ptr::{read_volatile, write_volatile};

// ============================================================================
// GTZC Base Addresses / GTZC 基地址
// ============================================================================

/// TZIC base address / TZIC 基地址
/// TrustZone Interrupt Controller
pub const GTZC_TZIC_BASE: usize = 0x4000_0000;

/// MPCWM1 base address / MPCWM1 基地址
/// Memory Protection Controller Wrapper 1
pub const GTZC_MPCWM1_BASE: usize = 0x4003_0000;

/// TDC1 base address / TDC1 基地址
/// TrustZone Decorator 1
pub const GTZC_TDC1_BASE: usize = 0x4003_1000;

/// ETZPC base address / ETZPC 基地址
/// Extended TrustZone Peripheral Controller
pub const GTZC_ETZPC_BASE: usize = 0x4003_2000;

// ============================================================================
// Register Offsets / 寄存器偏移
// ============================================================================

/// TZIC Register Offsets / TZIC 寄存器偏移
/// Reference: RM0456 Chapter 3.3 / 参考: RM0456 第3.3节
pub mod tzic_reg {
    pub const IER1: usize = 0x00;
    pub const IER2: usize = 0x04;
    pub const IER3: usize = 0x08;
    pub const IER4: usize = 0x0C;
    pub const ISR1: usize = 0x20;
    pub const ISR2: usize = 0x24;
    pub const ISR3: usize = 0x28;
    pub const ISR4: usize = 0x2C;
    pub const IPC1: usize = 0x40;
    pub const IPC2: usize = 0x44;
    pub const IPC3: usize = 0x48;
    pub const IPC4: usize = 0x4C;
    pub const IPC5: usize = 0x50;
    pub const IPC6: usize = 0x54;
    pub const IPC7: usize = 0x58;
    pub const IPC8: usize = 0x5C;
    pub const IPC9: usize = 0x60;
    pub const IPC10: usize = 0x64;
    pub const IPC11: usize = 0x68;
    pub const IPC12: usize = 0x6C;
    pub const IPC13: usize = 0x70;
    pub const IPC14: usize = 0x74;
    pub const IPC15: usize = 0x78;
    pub const IPC16: usize = 0x7C;
}

/// MPCWM1 Register Offsets / MPCWM1 寄存器偏移
/// Reference: RM0456 Chapter 3.4 / 参考: RM0456 第3.4节
pub mod mpcwm_reg {
    pub const CR: usize = 0x00;
    pub const SR: usize = 0x04;
    pub const ECCIER: usize = 0x08;
    pub const ECCISR: usize = 0x0C;
    pub const ECCIPR: usize = 0x10;
    pub const ECCDAIR: usize = 0x14;
    pub const ECCDEAR: usize = 0x18;
    pub const SR1: usize = 0x40;
    pub const SR2: usize = 0x44;
    pub const SR3: usize = 0x48;
    pub const SR4: usize = 0x4C;
    pub const SR5: usize = 0x50;
    pub const SR6: usize = 0x54;
    pub const SR7: usize = 0x58;
    pub const SR8: usize = 0x5C;
    pub const CR1: usize = 0x80;
    pub const CR2: usize = 0x84;
    pub const CR3: usize = 0x88;
    pub const CR4: usize = 0x8C;
    pub const CR5: usize = 0x90;
    pub const CR6: usize = 0x94;
    pub const CR7: usize = 0x98;
    pub const CR8: usize = 0x9C;
}

/// TDC1 Register Offsets / TDC1 寄存器偏移
/// Reference: RM0456 Chapter 3.5 / 参考: RM0456 第3.5节
pub mod tdc_reg {
    pub const CR: usize = 0x00;
    pub const SR: usize = 0x04;
}

/// ETZPC Register Offsets / ETZPC 寄存器偏移
/// Reference: RM0456 Chapter 3.6 / 参考: RM0456 第3.6节
pub mod etzpc_reg {
    pub const CR: usize = 0x00;
    pub const DECPROT0: usize = 0x10;
    pub const DECPROT1: usize = 0x14;
    pub const DECPROT2: usize = 0x18;
    pub const DECPROT3: usize = 0x1C;
    pub const DECPROT4: usize = 0x20;
    pub const DECPROT5: usize = 0x24;
    pub const DECPROT6: usize = 0x28;
    pub const DECPROT7: usize = 0x2C;
    pub const DECPROT8: usize = 0x30;
    pub const DECPROT9: usize = 0x34;
    pub const DECPROT10: usize = 0x38;
    pub const DECPROT11: usize = 0x3C;
    pub const DECPROT12: usize = 0x40;
    pub const DECPROT13: usize = 0x44;
    pub const DECPROT14: usize = 0x48;
    pub const DECPROT15: usize = 0x4C;
    pub const DECPROT16: usize = 0x50;
    pub const DECPROT17: usize = 0x54;
    pub const DECPROT18: usize = 0x58;
    pub const DECPROT19: usize = 0x5C;
    pub const DECPROT20: usize = 0x60;
    pub const DECPROT21: usize = 0x64;
    pub const DECPROT22: usize = 0x68;
    pub const DECPROT23: usize = 0x6C;
    pub const DECPROT24: usize = 0x70;
    pub const DECPROT25: usize = 0x74;
    pub const DECPROT26: usize = 0x78;
    pub const DECPROT27: usize = 0x7C;
    pub const DECPROT28: usize = 0x80;
    pub const DECPROT29: usize = 0x84;
    pub const DECPROT30: usize = 0x88;
    pub const DECPROT31: usize = 0x8C;
    pub const MCU_DECPROT: usize = 0xA0;
    pub const MCU_DECPROT2: usize = 0xA4;
}

// ============================================================================
// Register Bit Definitions / 寄存器位定义
// ============================================================================

/// TZIC Interrupt Enable Register bits / TZIC 中断使能寄存器位
pub mod tzic_ier_bits {
    pub const IT_PENDING1: u32 = 1 << 0;
    pub const IT_PENDING2: u32 = 1 << 1;
    pub const IT_PENDING3: u32 = 1 << 2;
    pub const IT_PENDING4: u32 = 1 << 3;
    pub const IT_PENDING5: u32 = 1 << 4;
    pub const IT_PENDING6: u32 = 1 << 5;
    pub const IT_PENDING7: u32 = 1 << 6;
    pub const IT_PENDING8: u32 = 1 << 7;
    pub const IT_PENDING9: u32 = 1 << 8;
    pub const IT_PENDING10: u32 = 1 << 9;
    pub const IT_PENDING11: u32 = 1 << 10;
    pub const IT_PENDING12: u32 = 1 << 11;
    pub const IT_PENDING13: u32 = 1 << 12;
    pub const IT_PENDING14: u32 = 1 << 13;
    pub const IT_PENDING15: u32 = 1 << 14;
    pub const IT_PENDING16: u32 = 1 << 15;
    pub const IT_PENDING17: u32 = 1 << 16;
    pub const IT_PENDING18: u32 = 1 << 17;
    pub const IT_PENDING19: u32 = 1 << 18;
    pub const IT_PENDING20: u32 = 1 << 19;
    pub const IT_PENDING21: u32 = 1 << 20;
    pub const IT_PENDING22: u32 = 1 << 21;
    pub const IT_PENDING23: u32 = 1 << 22;
    pub const IT_PENDING24: u32 = 1 << 23;
    pub const IT_PENDING25: u32 = 1 << 24;
    pub const IT_PENDING26: u32 = 1 << 25;
    pub const IT_PENDING27: u32 = 1 << 26;
    pub const IT_PENDING28: u32 = 1 << 27;
    pub const IT_PENDING29: u32 = 1 << 28;
    pub const IT_PENDING30: u32 = 1 << 29;
    pub const IT_PENDING31: u32 = 1 << 30;
}

/// TZIC Interrupt Status Register bits / TZIC 中断状态寄存器位
pub mod tzic_isr_bits {
    pub const IT_PENDING1: u32 = 1 << 0;
    pub const IT_PENDING2: u32 = 1 << 1;
    pub const IT_PENDING3: u32 = 1 << 2;
    pub const IT_PENDING4: u32 = 1 << 3;
    pub const IT_PENDING5: u32 = 1 << 4;
    pub const IT_PENDING6: u32 = 1 << 5;
    pub const IT_PENDING7: u32 = 1 << 6;
    pub const IT_PENDING8: u32 = 1 << 7;
}

/// TZIC IPC Register bits / TZIC IPC 寄存器位
pub mod tzic_ipc_bits {
    pub const LOCK: u32 = 1 << 31;
    pub const PRIORITY_SHIFT: u32 = 24;
    pub const PRIORITY_MASK: u32 = 0xF << 24;
    pub const IRQ0_SEC: u32 = 1 << 0;
    pub const IRQ1_SEC: u32 = 1 << 1;
    pub const IRQ2_SEC: u32 = 1 << 2;
    pub const IRQ3_SEC: u32 = 1 << 3;
    pub const IRQ4_SEC: u32 = 1 << 4;
    pub const IRQ5_SEC: u32 = 1 << 5;
    pub const IRQ6_SEC: u32 = 1 << 6;
    pub const IRQ7_SEC: u32 = 1 << 7;
}

/// MPCWM Control Register bits / MPCWM 控制寄存器位
pub mod mpcwm_cr_bits {
    pub const MPCWMEN: u32 = 1 << 0;
    pub const TZEN: u32 = 1 << 1;
    pub const MPCWM_STATE: u32 = 1 << 16;
    pub const MPCWM_LOCK: u32 = 1 << 31;
}

/// MPCWM Status Register bits / MPCWM 状态寄存器位
pub mod mpcwm_sr_bits {
    pub const MPCWM_RDY: u32 = 1 << 0;
    pub const MPCWM_BUSY: u32 = 1 << 1;
}

/// TDC Control Register bits / TDC 控制寄存器位
pub mod tdc_cr_bits {
    pub const TDCEN: u32 = 1 << 0;
    pub const TDC_STATE: u32 = 1 << 16;
    pub const TDC_LOCK: u32 = 1 << 31;
}

/// ETZPC Control Register bits / ETZPC 控制寄存器位
pub mod etzpc_cr_bits {
    pub const ETZPC_LOCK: u32 = 1 << 31;
}

/// ETZPC Decrot Register bits / ETZPC 解除保护寄存器位
pub mod etzpc_decprot_bits {
    pub const LOCK: u32 = 1 << 0;
    pub const SEC: u32 = 1 << 1;
    pub const PROTVEC0: u32 = 1 << 16;
    pub const PROTVEC1: u32 = 1 << 17;
    pub const PROTVEC2: u32 = 1 << 18;
    pub const PROTVEC3: u32 = 1 << 19;
    pub const PROTVEC4: u32 = 1 << 20;
    pub const PROTVEC5: u32 = 1 << 21;
    pub const PROTVEC6: u32 = 1 << 22;
    pub const PROTVEC7: u32 = 1 << 23;
}

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// GTZC Peripheral Secure Attribution / GTZC 外设安全属性
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecureAttribute {
    /// Secure / 安全
    Secure = 0,
    /// Non-Secure / 非安全
    NonSecure = 1,
}

/// GTZC Lock Status / GTZC 锁定状态
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LockStatus {
    /// Unlocked / 未锁定
    Unlocked = 0,
    /// Locked / 已锁定
    Locked = 1,
}

/// GTZC IRQ Configuration / GTZC IRQ 配置
#[derive(Clone, Copy, Debug)]
pub struct IrqConfig {
    pub secure: bool,
    pub priority: u8,
}

/// GTZC Memory Region Configuration / GTZC 存储器区域配置
#[derive(Clone, Copy, Debug)]
pub struct MemoryRegionConfig {
    pub secure: bool,
    pub read_enabled: bool,
    pub write_enabled: bool,
    pub exec_enabled: bool,
}

// ============================================================================
// GTZC Driver / GTZC 驱动
// ============================================================================

/// GTZC Driver / GTZC 驱动
pub struct Gtzc;

impl Gtzc {
    pub const fn new() -> Self {
        Self
    }

    fn tzic_reg(&self, offset: usize) -> *mut u32 {
        (GTZC_TZIC_BASE + offset) as *mut u32
    }

    fn mpcwm_reg(&self, offset: usize) -> *mut u32 {
        (GTZC_MPCWM1_BASE + offset) as *mut u32
    }

    fn tdc_reg(&self, offset: usize) -> *mut u32 {
        (GTZC_TDC1_BASE + offset) as *mut u32
    }

    fn etzpc_reg(&self, offset: usize) -> *mut u32 {
        (GTZC_ETZPC_BASE + offset) as *mut u32
    }

    // ============================================================================
    // TZIC Functions / TZIC 功能
    // ============================================================================

    pub fn tzic_enable_irq(&self, irq: u8) {
        if irq < 32 {
            unsafe {
                let reg = self.tzic_reg(tzic_reg::IER1);
                write_volatile(reg, 1 << irq);
            }
        }
    }

    pub fn tzic_disable_irq(&self, irq: u8) {
        if irq < 32 {
            unsafe {
                let reg = self.tzic_reg(tzic_reg::IER1);
                let val = read_volatile(reg);
                write_volatile(reg, val & !(1 << irq));
            }
        }
    }

    pub fn tzic_get_irq_status(&self, irq: u8) -> bool {
        if irq < 32 {
            unsafe {
                let reg = self.tzic_reg(tzic_reg::ISR1);
                return (read_volatile(reg) & (1 << irq)) != 0;
            }
        }
        false
    }

    pub fn tzic_clear_irq(&self, irq: u8) {
        if irq < 32 {
            unsafe {
                let reg = self.tzic_reg(tzic_reg::ISR1);
                write_volatile(reg, 1 << irq);
            }
        }
    }

    pub fn tzic_configure_irq(&self, irq: u8, config: &IrqConfig) {
        if irq < 32 {
            unsafe {
                let reg_offset = tzic_reg::IPC1 + (irq as usize / 4) * 4;
                let reg = self.tzic_reg(reg_offset);
                let bit_offset = (irq as usize % 4) * 8;
                
                let mut val = read_volatile(reg);
                val &= !(0xFF << bit_offset);
                val |= (config.priority as u32 & 0xF) << (bit_offset + 4);
                val |= if config.secure { 1 << bit_offset } else { 0 };
                write_volatile(reg, val);
            }
        }
    }

    // ============================================================================
    // MPCWM Functions / MPCWM 功能
    // ============================================================================

    pub fn mpcwm_enable(&self) {
        unsafe {
            let reg = self.mpcwm_reg(mpcwm_reg::CR);
            write_volatile(reg, mpcwm_cr_bits::MPCWMEN);
        }
    }

    pub fn mpcwm_disable(&self) {
        unsafe {
            let reg = self.mpcwm_reg(mpcwm_reg::CR);
            write_volatile(reg, 0);
        }
    }

    pub fn mpcwm_is_ready(&self) -> bool {
        unsafe {
            let reg = self.mpcwm_reg(mpcwm_reg::SR);
            (read_volatile(reg) & mpcwm_sr_bits::MPCWM_RDY) != 0
        }
    }

    pub fn mpcwm_is_busy(&self) -> bool {
        unsafe {
            let reg = self.mpcwm_reg(mpcwm_reg::SR);
            (read_volatile(reg) & mpcwm_sr_bits::MPCWM_BUSY) != 0
        }
    }

    pub fn mpcwm_set_secure_region(&self, region: u8, secure: bool) {
        if region < 8 {
            unsafe {
                let reg_offset = mpcwm_reg::SR1 + (region as usize * 4);
                let reg = self.mpcwm_reg(reg_offset);
                let val = if secure { 0x3 } else { 0 };
                write_volatile(reg, val);
            }
        }
    }

    pub fn mpcwm_get_secure_region(&self, region: u8) -> bool {
        if region < 8 {
            unsafe {
                let reg_offset = mpcwm_reg::SR1 + (region as usize * 4);
                let reg = self.mpcwm_reg(reg_offset);
                return read_volatile(reg) == 0x3;
            }
        }
        false
    }

    pub fn mpcwm_lock_region(&self, region: u8) {
        if region < 8 {
            unsafe {
                let reg_offset = mpcwm_reg::CR1 + (region as usize * 4);
                let reg = self.mpcwm_reg(reg_offset);
                write_volatile(reg, 1 << 31);
            }
        }
    }

    // ============================================================================
    // TDC Functions / TDC 功能
    // ============================================================================

    pub fn tdc_enable(&self) {
        unsafe {
            let reg = self.tdc_reg(tdc_reg::CR);
            write_volatile(reg, tdc_cr_bits::TDCEN);
        }
    }

    pub fn tdc_disable(&self) {
        unsafe {
            let reg = self.tdc_reg(tdc_reg::CR);
            write_volatile(reg, 0);
        }
    }

    // ============================================================================
    // ETZPC Functions / ETZPC 功能
    // ============================================================================

    pub fn etzpc_set_peripheral_secure(&self, periph_id: u8, secure: bool) {
        if periph_id < 32 {
            unsafe {
                let reg_offset = etzpc_reg::DECPROT0 + (periph_id as usize / 4) * 4;
                let reg = self.etzpc_reg(reg_offset);
                let bit_offset = (periph_id as usize % 4) * 8;
                
                let mut val = read_volatile(reg);
                val &= !(0x3 << bit_offset);
                val |= if secure { 0x1 } else { 0x2 } << bit_offset;
                write_volatile(reg, val);
            }
        }
    }

    pub fn etzpc_get_peripheral_secure(&self, periph_id: u8) -> bool {
        if periph_id < 32 {
            unsafe {
                let reg_offset = etzpc_reg::DECPROT0 + (periph_id as usize / 4) * 4;
                let reg = self.etzpc_reg(reg_offset);
                let bit_offset = (periph_id as usize % 4) * 8;
                let val = (read_volatile(reg) >> bit_offset) & 0x3;
                return val == 0x1;
            }
        }
        false
    }

    pub fn etzpc_lock_peripheral(&self, periph_id: u8) {
        if periph_id < 32 {
            unsafe {
                let reg_offset = etzpc_reg::DECPROT0 + (periph_id as usize / 4) * 4;
                let reg = self.etzpc_reg(reg_offset);
                let bit_offset = (periph_id as usize % 4) * 8;
                
                let mut val = read_volatile(reg);
                val |= 1 << (bit_offset + 2);
                write_volatile(reg, val);
            }
        }
    }

    pub fn etzpc_is_locked(&self, periph_id: u8) -> bool {
        if periph_id < 32 {
            unsafe {
                let reg_offset = etzpc_reg::DECPROT0 + (periph_id as usize / 4) * 4;
                let reg = self.etzpc_reg(reg_offset);
                let bit_offset = (periph_id as usize % 4) * 8;
                return (read_volatile(reg) & (1 << (bit_offset + 2))) != 0;
            }
        }
        false
    }

    pub fn etzpc_lock_all(&self) {
        unsafe {
            let reg = self.etzpc_reg(etzpc_reg::CR);
            write_volatile(reg, etzpc_cr_bits::ETZPC_LOCK);
        }
    }

    pub fn etzpc_unlock_all(&self) {
        unsafe {
            let reg = self.etzpc_reg(etzpc_reg::CR);
            write_volatile(reg, 0);
        }
    }
}

// ============================================================================
// GTZC IRQ Numbers / GTZC IRQ 编号
// ============================================================================

/// GTZC IRQ numbers / GTZC IRQ 编号
pub mod irq {
    pub const TZIC1_NS: u8 = 0;
    pub const TZIC2_NS: u8 = 1;
    pub const TZIC3_NS: u8 = 2;
    pub const TZIC4_NS: u8 = 3;
    pub const TZIC1_S: u8 = 4;
    pub const TZIC2_S: u8 = 5;
    pub const TZIC3_S: u8 = 6;
    pub const TZIC4_S: u8 = 7;
}

// ============================================================================
// GTZC Peripheral IDs / GTZC 外设 ID
// ============================================================================

/// ETZPC Peripheral IDs / ETZPC 外设 ID
/// Reference: RM0456 Table 57 / 参考: RM0456 表57
pub mod periph_id {
    pub const WWDG: u8 = 0;
    pub const IWDG: u8 = 1;
    pub const RTC: u8 = 2;
    pub const RCC: u8 = 3;
    pub const PWR: u8 = 4;
    pub const EXTI: u8 = 5;
    pub const COMP: u8 = 6;
    pub const ADC1: u8 = 7;
    pub const ADC2: u8 = 8;
    pub const DAC1: u8 = 9;
    pub const DAC2: u8 = 10;
    pub const VREFBUF: u8 = 11;
    pub const TIM1: u8 = 12;
    pub const TIM2: u8 = 13;
    pub const TIM3: u8 = 14;
    pub const TIM4: u8 = 15;
    pub const TIM5: u8 = 16;
    pub const TIM6: u8 = 17;
    pub const TIM7: u8 = 18;
    pub const TIM8: u8 = 19;
    pub const TIM15: u8 = 20;
    pub const TIM16: u8 = 21;
    pub const TIM17: u8 = 22;
    pub const SPI1: u8 = 23;
    pub const SPI2: u8 = 24;
    pub const SPI3: u8 = 25;
    pub const I2C1: u8 = 26;
    pub const I2C2: u8 = 27;
    pub const I2C3: u8 = 28;
    pub const USART1: u8 = 29;
    pub const USART2: u8 = 30;
    pub const USART3: u8 = 31;
}
