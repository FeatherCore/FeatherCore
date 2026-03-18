//! NVIC - Nested Vectored Interrupt Controller
//! 嵌套向量中断控制器
//!
//! # Overview / 概述
//! STM32U5 Nested Vectored Interrupt Controller (NVIC) provides interrupt management
//! with support for up to 240 external interrupts and 16 programmable priority levels.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 22: Nested vectored interrupt controller (NVIC)
//! 
//! ## Interrupt Features / 中断特性
//! - Up to 240 external interrupts
//! - 16 programmable priority levels (4-bit)
//! 
//! ## Security / 安全特性
//! - TrustZone Security Extension support
//! 
//! # Reference / 参考
//! - RM0456 Chapter 22: Nested vectored interrupt controller (NVIC)
//! - RM0456 Section 22.1: NVIC introduction
//! - RM0456 Section 22.2: NVIC main features
//! - RM0456 Section 22.3: NVIC functional description
//! - RM0456 Section 22.4: NVIC registers

/// NVIC base address / NVIC 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const NVIC_BASE: usize = 0xE000_E100;
/// NVIC base address for secure state (when TrustZone is enabled)
pub const NVIC_BASE_S: usize = 0xE002_E100;

/// NVIC register offsets
//! Reference: RM0456 Section 22.4: NVIC register map
pub mod reg {
    /// Interrupt set enable registers (0-31)
    //! Reference: RM0456 Section 22.4.1: NVIC ISERx
    pub const ISER0: usize = 0x000;
    pub const ISER1: usize = 0x004;
    pub const ISER2: usize = 0x008;
    pub const ISER3: usize = 0x00C;
    pub const ISER4: usize = 0x010;
    pub const ISER5: usize = 0x014;
    pub const ISER6: usize = 0x018;
    pub const ISER7: usize = 0x01C;

    /// Interrupt clear enable registers (0-31)
    //! Reference: RM0456 Section 22.4.2: NVIC ICERx
    pub const ICER0: usize = 0x080;
    pub const ICER1: usize = 0x084;
    pub const ICER2: usize = 0x088;
    pub const ICER3: usize = 0x08C;
    pub const ICER4: usize = 0x090;
    pub const ICER5: usize = 0x094;
    pub const ICER6: usize = 0x098;
    pub const ICER7: usize = 0x09C;

    /// Interrupt set pending registers
    //! Reference: RM0456 Section 22.4.3: NVIC ISPRx
    pub const ISPR0: usize = 0x100;
    pub const ISPR1: usize = 0x104;
    pub const ISPR2: usize = 0x108;
    pub const ISPR3: usize = 0x10C;

    /// Interrupt clear pending registers
    pub const ICPR0: usize = 0x180;
    pub const ICPR1: usize = 0x184;
    pub const ICPR2: usize = 0x188;
    pub const ICPR3: usize = 0x18C;

    /// Interrupt active bit registers
    pub const IABR0: usize = 0x200;
    pub const IABR1: usize = 0x204;
    pub const IABR2: usize = 0x208;
    pub const IABR3: usize = 0x20C;

    /// Interrupt priority registers
    pub const IPR0: usize = 0x300;
    pub const IPR1: usize = 0x304;
    pub const IPR2: usize = 0x308;
    pub const IPR3: usize = 0x30C;
    pub const IPR4: usize = 0x310;
    pub const IPR5: usize = 0x314;
    pub const IPR6: usize = 0x318;
    pub const IPR7: usize = 0x31C;
}

/// System Control Block (SCB) base address
pub const SCB_BASE: usize = 0xE000_ED00;

/// SCB register offsets
pub mod scb_reg {
    /// CPUID base register
    pub const CPUID: usize = 0x000;
    /// Interrupt control and state register
    pub const ICSR: usize = 0x004;
    /// Vector table offset register
    pub const VTOR: usize = 0x008;
    /// Application interrupt and reset control register
    pub const AIRCR: usize = 0x00C;
    /// System control register
    pub const SCR: usize = 0x010;
    /// Configuration and control register
    pub const CCR: usize = 0x014;
    /// System handler priority registers
    pub const SHPR1: usize = 0x018;
    pub const SHPR2: usize = 0x01C;
    pub const SHPR3: usize = 0x020;
    /// System handler control and state register
    pub const SHCSR: usize = 0x024;
}

/// Initialize NVIC
pub fn init() {
    // Set priority grouping: 4 bits for preemption priority, 0 bits for subpriority
    set_priority_grouping(0b011);
}

/// Set priority grouping
///
/// # Arguments
/// * `grouping` - Priority grouping value (0-7)
///   - 0: 7 bits preemption, 1 bit subpriority
///   - 1: 6 bits preemption, 2 bits subpriority
///   - ...
///   - 7: 0 bits preemption, 8 bits subpriority (not valid for 4-bit priority)
pub fn set_priority_grouping(grouping: u8) {
    unsafe {
        let aircr = (SCB_BASE + scb_reg::AIRCR) as *mut u32;
        let mut val = core::ptr::read_volatile(aircr);
        val &= !(0xFFFF << 0); // Clear VECTKEY and PRIGROUP
        val |= 0x05FA << 16;   // VECTKEY
        val |= (grouping as u32) << 8; // PRIGROUP
        core::ptr::write_volatile(aircr, val);
    }
}

/// Enable interrupt
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
pub fn enable_irq(irqn: u8) {
    if irqn >= 240 {
        return;
    }

    unsafe {
        let iser = (NVIC_BASE + reg::ISER0 + ((irqn as usize / 32) * 4)) as *mut u32;
        core::ptr::write_volatile(iser, 1 << (irqn % 32));
    }
}

/// Disable interrupt
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
pub fn disable_irq(irqn: u8) {
    if irqn >= 240 {
        return;
    }

    unsafe {
        let icer = (NVIC_BASE + reg::ICER0 + ((irqn as usize / 32) * 4)) as *mut u32;
        core::ptr::write_volatile(icer, 1 << (irqn % 32));
    }
}

/// Set interrupt priority
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
/// * `priority` - Priority value (0-15, lower is higher priority)
pub fn set_priority(irqn: u8, priority: u8) {
    if irqn >= 240 {
        return;
    }

    unsafe {
        let ipr = (NVIC_BASE + reg::IPR0 + ((irqn as usize / 4) * 4)) as *mut u32;
        let shift = (irqn % 4) * 8;
        let mut val = core::ptr::read_volatile(ipr);
        val &= !(0xFF << shift);
        val |= ((priority as u32) << 4) << shift; // Priority is in bits [7:4]
        core::ptr::write_volatile(ipr, val);
    }
}

/// Get interrupt priority
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
///
/// # Returns
/// Priority value (0-15)
pub fn get_priority(irqn: u8) -> u8 {
    if irqn >= 240 {
        return 0;
    }

    unsafe {
        let ipr = (NVIC_BASE + reg::IPR0 + ((irqn as usize / 4) * 4)) as *mut u32;
        let shift = (irqn % 4) * 8;
        let val = core::ptr::read_volatile(ipr);
        ((val >> shift) >> 4) as u8
    }
}

/// Check if interrupt is pending
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
pub fn is_pending(irqn: u8) -> bool {
    if irqn >= 240 {
        return false;
    }

    unsafe {
        let ispr = (NVIC_BASE + reg::ISPR0 + ((irqn as usize / 32) * 4)) as *mut u32;
        let val = core::ptr::read_volatile(ispr);
        (val & (1 << (irqn % 32))) != 0
    }
}

/// Set interrupt pending
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
pub fn set_pending(irqn: u8) {
    if irqn >= 240 {
        return;
    }

    unsafe {
        let ispr = (NVIC_BASE + reg::ISPR0 + ((irqn as usize / 32) * 4)) as *mut u32;
        core::ptr::write_volatile(ispr, 1 << (irqn % 32));
    }
}

/// Clear interrupt pending
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
pub fn clear_pending(irqn: u8) {
    if irqn >= 240 {
        return;
    }

    unsafe {
        let icpr = (NVIC_BASE + reg::ICPR0 + ((irqn as usize / 32) * 4)) as *mut u32;
        core::ptr::write_volatile(icpr, 1 << (irqn % 32));
    }
}

/// Check if interrupt is active
///
/// # Arguments
/// * `irqn` - Interrupt number (0-239)
pub fn is_active(irqn: u8) -> bool {
    if irqn >= 240 {
        return false;
    }

    unsafe {
        let iabr = (NVIC_BASE + reg::IABR0 + ((irqn as usize / 32) * 4)) as *mut u32;
        let val = core::ptr::read_volatile(iabr);
        (val & (1 << (irqn % 32))) != 0
    }
}

/// Set vector table offset
///
/// # Arguments
/// * `offset` - Vector table offset address (must be aligned to 512 bytes)
pub fn set_vector_table_offset(offset: u32) {
    unsafe {
        let vtor = (SCB_BASE + scb_reg::VTOR) as *mut u32;
        core::ptr::write_volatile(vtor, offset);
    }
}

/// Get vector table offset
pub fn get_vector_table_offset() -> u32 {
    unsafe {
        let vtor = (SCB_BASE + scb_reg::VTOR) as *mut u32;
        core::ptr::read_volatile(vtor)
    }
}

/// Enable all interrupts (clear PRIMASK)
pub fn enable_interrupts() {
    unsafe {
        core::arch::asm!("cpsie i");
    }
}

/// Disable all interrupts (set PRIMASK)
pub fn disable_interrupts() {
    unsafe {
        core::arch::asm!("cpsid i");
    }
}

/// System reset
pub fn system_reset() -> ! {
    unsafe {
        let aircr = (SCB_BASE + scb_reg::AIRCR) as *mut u32;
        core::ptr::write_volatile(aircr, 0x05FA_0004);
    }
    loop {}
}

/// STM32U5 Interrupt Numbers
pub mod irq {
    pub const WWDG: u8 = 0;
    pub const PVD_PVM: u8 = 1;
    pub const RTC: u8 = 2;
    pub const RTC_S: u8 = 3;
    pub const TAMP: u8 = 4;
    pub const RAMCFG: u8 = 5;
    pub const FLASH: u8 = 6;
    pub const FLASH_S: u8 = 7;
    pub const GTZC: u8 = 8;
    pub const RCC: u8 = 9;
    pub const RCC_S: u8 = 10;
    pub const EXTI0: u8 = 11;
    pub const EXTI1: u8 = 12;
    pub const EXTI2: u8 = 13;
    pub const EXTI3: u8 = 14;
    pub const EXTI4: u8 = 15;
    pub const EXTI5: u8 = 16;
    pub const EXTI6: u8 = 17;
    pub const EXTI7: u8 = 18;
    pub const EXTI8: u8 = 19;
    pub const EXTI9: u8 = 20;
    pub const EXTI10: u8 = 21;
    pub const EXTI11: u8 = 22;
    pub const EXTI12: u8 = 23;
    pub const EXTI13: u8 = 24;
    pub const EXTI14: u8 = 25;
    pub const EXTI15: u8 = 26;
    pub const IWDG: u8 = 27;
    pub const SAES: u8 = 28;
    pub const GPDMA1_Channel0: u8 = 29;
    pub const GPDMA1_Channel1: u8 = 30;
    pub const GPDMA1_Channel2: u8 = 31;
    pub const GPDMA1_Channel3: u8 = 32;
    pub const GPDMA1_Channel4: u8 = 33;
    pub const GPDMA1_Channel5: u8 = 34;
    pub const GPDMA1_Channel6: u8 = 35;
    pub const GPDMA1_Channel7: u8 = 36;
    pub const ADC1: u8 = 37;
    pub const DAC1: u8 = 38;
    pub const FDCAN1_IT0: u8 = 39;
    pub const FDCAN1_IT1: u8 = 40;
    pub const TIM1_BRK: u8 = 41;
    pub const TIM1_UP: u8 = 42;
    pub const TIM1_TRG_COM: u8 = 43;
    pub const TIM1_CC: u8 = 44;
    pub const TIM2: u8 = 45;
    pub const TIM3: u8 = 46;
    pub const TIM4: u8 = 47;
    pub const TIM5: u8 = 48;
    pub const TIM6: u8 = 49;
    pub const TIM7: u8 = 50;
    pub const TIM8_BRK: u8 = 51;
    pub const TIM8_UP: u8 = 52;
    pub const TIM8_TRG_COM: u8 = 53;
    pub const TIM8_CC: u8 = 54;
    pub const I2C1_EV: u8 = 55;
    pub const I2C1_ER: u8 = 56;
    pub const I2C2_EV: u8 = 57;
    pub const I2C2_ER: u8 = 58;
    pub const SPI1: u8 = 59;
    pub const SPI2: u8 = 60;
    pub const USART1: u8 = 61;
    pub const USART2: u8 = 62;
    pub const USART3: u8 = 63;
    pub const UART4: u8 = 64;
    pub const UART5: u8 = 65;
    pub const LPUART1: u8 = 66;
    pub const LPTIM1: u8 = 67;
    pub const LPTIM2: u8 = 68;
    pub const TIM15: u8 = 69;
    pub const TIM16: u8 = 70;
    pub const TIM17: u8 = 71;
    pub const COMP: u8 = 72;
    pub const OTG_FS: u8 = 73;
    pub const CRS: u8 = 74;
    pub const FMC: u8 = 75;
    pub const OCTOSPI1: u8 = 76;
    pub const PWR_S3WU: u8 = 77;
    pub const SDMMC1: u8 = 78;
    pub const SDMMC2: u8 = 79;
    pub const GPDMA1_Channel8: u8 = 80;
    pub const GPDMA1_Channel9: u8 = 81;
    pub const GPDMA1_Channel10: u8 = 82;
    pub const GPDMA1_Channel11: u8 = 83;
    pub const GPDMA1_Channel12: u8 = 84;
    pub const GPDMA1_Channel13: u8 = 85;
    pub const GPDMA1_Channel14: u8 = 86;
    pub const GPDMA1_Channel15: u8 = 87;
    pub const I2C3_EV: u8 = 88;
    pub const I2C3_ER: u8 = 89;
    pub const SAI1: u8 = 90;
    pub const SAI2: u8 = 91;
    pub const TSC: u8 = 92;
    pub const AES: u8 = 93;
    pub const RNG: u8 = 94;
    pub const FPU: u8 = 95;
    pub const HASH: u8 = 96;
    pub const PKA: u8 = 97;
    pub const CEC: u8 = 98;
    pub const TIM12: u8 = 99;
    pub const TIM13: u8 = 100;
    pub const TIM14: u8 = 101;
    pub const I3C1_EV: u8 = 102;
    pub const I3C1_ER: u8 = 103;
    pub const I2C4_EV: u8 = 104;
    pub const I2C4_ER: u8 = 105;
    pub const LPTIM3: u8 = 106;
    pub const LPTIM4: u8 = 107;
    pub const LPTIM5: u8 = 108;
    pub const LPTIM6: u8 = 109;
}
