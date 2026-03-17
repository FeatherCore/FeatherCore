//! MDF - Multi-Function Digital Filter
//! 多功能数字滤波器
//!
//! ## STM32U5 MDF 特性 / Features
//! - **滤波器单元 / Filter Units:**
//!   - 多达 6 个独立滤波器 (F0-F5)
//!   - 支持 SINC1-SINC5 阶滤波器
//!   - 可编程过采样率 (OSR)
//!
//! - **时钟源 / Clock Sources:**
//!   - 内部时钟生成器
//!   - 外部时钟输入
//!   - 可编程时钟分频
//!
//! - **输出 / Output:**
//!   - DMA 传输支持
//!   - 单次或循环模式
//!   - 多路输出
//!
//! - **触发源 / Trigger Sources:**
//!   - 软件触发
//!   - 硬件触发
//!   - 多达 8 个触发源
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 41: Multi-function digital filter (MDF)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// MDF1 base address / MDF1 基地址
/// AHB2 bus, accessible at 0x4003_5000
pub const MDF1_BASE: usize = 0x4003_5000;

/// MDF Filter 0 base address / MDF 滤波器 0 基地址
pub const MDF_FLT0_BASE: usize = MDF1_BASE + 0x000;
/// MDF Filter 1 base address / MDF 滤波器 1 基地址
pub const MDF_FLT1_BASE: usize = MDF1_BASE + 0x100;
/// MDF Filter 2 base address / MDF 滤波器 2 基地址
pub const MDF_FLT2_BASE: usize = MDF1_BASE + 0x200;
/// MDF Filter 3 base address / MDF 滤波器 3 基地址
pub const MDF_FLT3_BASE: usize = MDF1_BASE + 0x300;
/// MDF Filter 4 base address / MDF 滤波器 4 基地址
pub const MDF_FLT4_BASE: usize = MDF1_BASE + 0x400;
/// MDF Filter 5 base address / MDF 滤波器 5 基地址
pub const MDF_FLT5_BASE: usize = MDF1_BASE + 0x500;

/// MDF Filter Register Map / MDF 滤波器寄存器映射
#[repr(C)]
pub struct MdfFilterRegs {
    /// Digital Filter Control Register / 数字滤波器控制寄存器
    pub dfltcr: u32,
    /// Digital Filter Channel Instance Control Register / 数字滤波器通道实例控制寄存器
    pub dfltcicr: u32,
    /// Digital Filter MICSFILT Control Register / 数字滤波器 MICSFILT 控制寄存器
    pub dfltmsicr: u32,
    /// Digital Filter Mode Register / 数字滤波器模式寄存器
    pub dfltmdr: u32,
    /// Digital Filter Duration Register / 数字滤波器持续时间寄存器
    pub dfltdur: u32,
    /// Digital Filter Interrupt Enable Register / 数字滤波器中断使能寄存器
    pub dfltier: u32,
    /// Digital Filter Interrupt Status Register / 数字滤波器中断状态寄存器
    pub dfltisr: u32,
    /// Digital Filter Status Interrupt Flag Register / 数字滤波器状态中断标志寄存器
    pub dfltsifr: u32,
    /// Digital Filter Interrupt Flag Register / 数字滤波器中断标志寄存器
    pub dfltifr: u32,
    /// Reserved / 保留
    reserved1: u32,
    /// Digital Filter INCIR Register / 数字滤波器 INCIR 寄存器
    pub dfltincr: u32,
    /// Reserved / 保留
    reserved2: u32,
    /// Digital Filter RSFCL Control Register / 数字滤波器 RSFCL 控制寄存器
    pub dfltrsfcr: u32,
    /// Reserved / 保留
    reserved3: [u32; 2],
    /// Digital Filter Output Right Data Register / 数字滤波器输出右数据寄存器
    pub dfltoutr: u32,
    /// Digital Filter Output Right Host Data Register / 数字滤波器输出右主机数据寄存器
    pub dfltrdhr: u32,
    /// Reserved / 保留
    reserved4: [u32; 2],
    /// Digital Filter Output Data Register / 数字滤波器输出数据寄存器
    pub dfltoutdr: u32,
    /// Reserved / 保留
    reserved5: [u32; 3],
    /// Digital Filter Trigger Control Register / 数字滤波器触发控制寄存器
    pub dfltrgcr: u32,
    /// Reserved / 保留
    reserved6: [u32; 3],
    /// Digital Filter Global Status Register / 数字滤波器全局状态寄存器
    pub dfltgsr: u32,
    /// Reserved / 保留
    reserved7: [u32; 11],
    /// Digital Filter Output Level Detector Control Register / 数字滤波器输出电平检测器控制寄存器
    pub dfltoldcr: u32,
    /// Digital Filter Output Level Detector Value Register / 数字滤波器输出电平检测器值寄存器
    pub dfltoldval: u32,
    /// Digital Filter Output Level Detector Minimum Register / 数字滤波器输出电平检测器最小值寄存器
    pub dfltoldmin: u32,
    /// Digital Filter Output Level Detector Maximum Register / 数字滤波器输出电平检测器最大值寄存器
    pub dfltoldmax: u32,
}

/// MDF Common Register Map / MDF 公共寄存器映射
#[repr(C)]
pub struct MdfCommonRegs {
    /// Global Control Register / 全局控制寄存器
    pub gcr: u32,
    /// Clock Generator Control Register / 时钟生成控制寄存器
    pub ckgcr: u32,
    /// Spare Control Register / 备用控制寄存器
    pub sparecr: u32,
    /// Spare Value Register / 备用值寄存器
    pub spareval: u32,
    /// Filter Instance Control Register x / 滤波器实例控制寄存器 x
    pub idcr: u32,
}

/// MDF instance / MDF 实例
pub struct Mdf;

/// Filter selection / 滤波器选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Filter {
    /// Filter 0 / 滤波器 0
    F0 = 0,
    /// Filter 1 / 滤波器 1
    F1 = 1,
    /// Filter 2 / 滤波器 2
    F2 = 2,
    /// Filter 3 / 滤波器 3
    F3 = 3,
    /// Filter 4 / 滤波器 4
    F4 = 4,
    /// Filter 5 / 滤波器 5
    F5 = 5,
}

/// SINC filter order / SINC 滤波器阶数
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SincOrder {
    /// Fast SINC order / 快速 SINC 阶
    FastSinc = 0,
    /// SINC1 order / SINC1 阶
    Sinc1 = 1,
    /// SINC2 order / SINC2 阶
    Sinc2 = 2,
    /// SINC3 order / SINC3 阶
    Sinc3 = 3,
    /// SINC4 order / SINC4 阶
    Sinc4 = 4,
    /// SINC5 order / SINC5 阶
    Sinc5 = 5,
}

/// Clock output prescaler / 时钟输出预分频器
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CkoPrescaler {
    /// Clock divide by 1 / 时钟 1 分频
    Div1 = 0,
    /// Clock divide by 2 / 时钟 2 分频
    Div2 = 1,
    /// Clock divide by 4 / 时钟 4 分频
    Div4 = 2,
    /// Clock divide by 8 / 时钟 8 分频
    Div8 = 3,
    /// Clock divide by 16 / 时钟 16 分频
    Div16 = 4,
    /// Clock divide by 32 / 时钟 32 分频
    Div32 = 5,
    /// Clock divide by 64 / 时钟 64 分频
    Div64 = 6,
    /// Clock divide by 128 / 时钟 128 分频
    Div128 = 7,
}

/// Clock output source / 时钟输出源
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CkoSrc {
    /// Clock output 0 / 时钟输出 0
    Cko0 = 0,
    /// Clock output 1 / 时钟输出 1
    Cko1 = 1,
    /// Clock output 2 / 时钟输出 2
    Cko2 = 2,
    /// Clock output 3 / 时钟输出 3
    Cko3 = 3,
    /// Clock output 4 / 时钟输出 4
    Cko4 = 4,
    /// Clock output 5 / 时钟输出 5
    Cko5 = 5,
}

/// Trigger source / 触发源
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Trgsrc {
    /// Trigger source 0 / 触发源 0
    Trg0 = 0,
    /// Trigger source 1 / 触发源 1
    Trg1 = 1,
    /// Trigger source 2 / 触发源 2
    Trg2 = 2,
    /// Trigger source 3 / 触发源 3
    Trg3 = 3,
    /// Trigger source 4 / 触发源 4
    Trg4 = 4,
    /// Trigger source 5 / 触发源 5
    Trg5 = 5,
    /// Trigger source 6 / 触发源 6
    Trg6 = 6,
    /// Trigger source 7 / 触发源 7
    Trg7 = 7,
}

/// DMA mode / DMA 模式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DmaMode {
    /// DMA disabled / DMA 禁用
    Disabled = 0,
    /// DMA one-shot mode / DMA 单次模式
    OneShot = 1,
    /// DMA circular mode / DMA 循环模式
    Circular = 2,
}

/// Filter configuration / 滤波器配置
#[derive(Clone, Copy, Debug)]
pub struct FilterConfig {
    /// SINC filter order / SINC 滤波器阶数
    pub sinc_order: SincOrder,
    /// SINC over-sampling ratio / SINC 过采样率
    pub sinc_osr: u16,
    /// High-pass filter cutoff / 高通滤波器截止频率
    pub hpf_cutoff: u8,
    /// Clock output prescaler / 时钟输出预分频器
    pub cko_prescaler: CkoPrescaler,
    /// Clock output source / 时钟输出源
    pub cko_src: CkoSrc,
    /// DMA enable / DMA 使能
    pub dmaden: bool,
    /// DMA mode / DMA 模式
    pub dma_mode: DmaMode,
    /// Output level detector enable / 输出电平检测器使能
    pub old_enable: bool,
    /// Output level detector threshold / 输出电平检测器阈值
    pub old_threshold: u16,
}

impl Default for FilterConfig {
    fn default() -> Self {
        FilterConfig {
            sinc_order: SincOrder::Sinc3,
            sinc_osr: 64,
            hpf_cutoff: 0,
            cko_prescaler: CkoPrescaler::Div1,
            cko_src: CkoSrc::Cko0,
            dmaden: false,
            dma_mode: DmaMode::Disabled,
            old_enable: false,
            old_threshold: 0,
        }
    }
}

pub struct ClockConfig {
    pub ckgden: bool,
    pub ckgmod: bool,
    pub ckgen: u8,
    pub ckgosr: u8,
}

impl Default for ClockConfig {
    fn default() -> Self {
        ClockConfig {
            ckgden: false,
            ckgmod: false,
            ckgen: 0,
            ckgosr: 0,
        }
    }
}

impl Mdf {
    pub fn new() -> Self {
        Mdf
    }

    fn filter_regs(&self, filter: Filter) -> &mut MdfFilterRegs {
        let base = match filter {
            Filter::F0 => MDF_FLT0_BASE,
            Filter::F1 => MDF_FLT1_BASE,
            Filter::F2 => MDF_FLT2_BASE,
            Filter::F3 => MDF_FLT3_BASE,
            Filter::F4 => MDF_FLT4_BASE,
            Filter::F5 => MDF_FLT5_BASE,
        };
        unsafe { &mut *(base as *mut MdfFilterRegs) }
    }

    fn common_regs(&self) -> &mut MdfCommonRegs {
        unsafe { &mut *(MDF1_BASE as *mut MdfCommonRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr |= 1 << 25;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr &= !(1 << 25);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1rstr = rcc_base.add(0x90 / 4);
            *ahb1rstr |= 1 << 25;
            *ahb1rstr &= !(1 << 25);
        }
    }

    pub fn configure_clock(&self, config: &ClockConfig) {
        let ckgcr = (config.ckgden as u32) << 31
            | (config.ckgmod as u32) << 30
            | (config.ckgen as u32) << 16
            | (config.ckgosr as u32) << 0;
        unsafe { write_volatile(&mut self.common_regs().ckgcr, ckgcr) };
    }

    pub fn configure_filter(&self, filter: Filter, config: &FilterConfig) {
        unsafe {
            let dfltcr = (config.sinc_order as u32) << 13
                | (config.sinc_osr as u32) << 0;
            write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr);

            let dfltcicr = (config.hpf_cutoff as u32) << 0;
            write_volatile(&mut self.filter_regs(filter).dfltcicr, dfltcicr);

            let dfltmdr = (config.cko_prescaler as u32) << 4
                | (config.cko_src as u32) << 0;
            write_volatile(&mut self.filter_regs(filter).dfltmdr, dfltmdr);

            let dmaden_bit = if config.dmaden { 1u32 << 14 } else { 0 };
            let dma_mode_bits = (config.dma_mode as u32) << 12;
            write_volatile(&mut self.filter_regs(filter).dfltcr,
                read_volatile(&self.filter_regs(filter).dfltcr) | dmaden_bit | dma_mode_bits);

            if config.old_enable {
                let dfltoldcr = (1u32 << 31) | (config.old_threshold as u32);
                write_volatile(&mut self.filter_regs(filter).dfltoldcr, dfltoldcr);
            }
        }
    }

    pub fn enable_filter(&self, filter: Filter) {
        unsafe {
            let dfltcr = read_volatile(&self.filter_regs(filter).dfltcr);
            write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr | (1 << 31));
        }
    }

    pub fn disable_filter(&self, filter: Filter) {
        unsafe {
            let dfltcr = read_volatile(&self.filter_regs(filter).dfltcr);
            write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr & !(1 << 31));
        }
    }

    pub fn is_filter_enabled(&self, filter: Filter) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltcr) & (1 << 31)) != 0
        }
    }

    pub fn get_data(&self, filter: Filter) -> i32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoutdr) as i32 }
    }

    pub fn is_data_available(&self, filter: Filter) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltisr) & 0x01) != 0
        }
    }

    pub fn clear_data_ready(&self, filter: Filter) {
        unsafe {
            write_volatile(&mut self.filter_regs(filter).dfltifr, 0x01);
        }
    }

    pub fn enable_interrupt(&self, filter: Filter, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.filter_regs(filter).dfltier);
            write_volatile(&mut self.filter_regs(filter).dfltier, dfltier | (1 << source));
        }
    }

    pub fn disable_interrupt(&self, filter: Filter, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.filter_regs(filter).dfltier);
            write_volatile(&mut self.filter_regs(filter).dfltier, dfltier & !(1 << source));
        }
    }

    pub fn is_interrupt_active(&self, filter: Filter, source: u8) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltisr) & (1 << source)) != 0
        }
    }

    pub fn clear_interrupt(&self, filter: Filter, source: u8) {
        unsafe {
            write_volatile(&mut self.filter_regs(filter).dfltifr, 1 << source);
        }
    }

    pub fn get_old_value(&self, filter: Filter) -> i16 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoldval) as i16 }
    }

    pub fn get_old_min(&self, filter: Filter) -> i16 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoldmin) as i16 }
    }

    pub fn get_old_max(&self, filter: Filter) -> i16 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoldmax) as i16 }
    }

    pub fn reset_old_counters(&self, filter: Filter) {
        unsafe {
            let dfltoldcr = read_volatile(&self.filter_regs(filter).dfltoldcr);
            write_volatile(&mut self.filter_regs(filter).dfltoldcr, dfltoldcr | (1 << 30));
        }
    }

    pub fn configure_trigger(&self, filter: Filter, trgsrc: Trgsrc, trgsens: bool) {
        unsafe {
            let dfltrgcr = (trgsrc as u32) << 4 | (trgsens as u32) << 1 | 1;
            write_volatile(&mut self.filter_regs(filter).dfltrgcr, dfltrgcr);
        }
    }

    pub fn software_trigger(&self, filter: Filter) {
        unsafe {
            let dfltrgcr = read_volatile(&self.filter_regs(filter).dfltrgcr);
            write_volatile(&mut self.filter_regs(filter).dfltrgcr, dfltrgcr | (1 << 2));
        }
    }

    pub fn is_trigger_active(&self, filter: Filter) -> bool {
        unsafe {
            (read_volatile(&self.filter_regs(filter).dfltgsr) & 0x01) != 0
        }
    }

    pub fn get_raw_counter(&self, filter: Filter) -> u32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltincr) }
    }

    pub fn set_acquisition_mode(&self, filter: Filter, continuous: bool) {
        unsafe {
            let dfltcr = read_volatile(&self.filter_regs(filter).dfltcr);
            if continuous {
                write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr | (1 << 30));
            } else {
                write_volatile(&mut self.filter_regs(filter).dfltcr, dfltcr & !(1 << 30));
            }
        }
    }

    pub fn set_scale_factor(&self, filter: Filter, scale: u8) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.filter_regs(filter).dfltrsfcr);
            write_volatile(&mut self.filter_regs(filter).dfltrsfcr,
                (dfltrsfcr & !0xFF) | (scale as u32));
        }
    }

    pub fn enable_scale_offset(&self, filter: Filter, enable: bool) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.filter_regs(filter).dfltrsfcr);
            if enable {
                write_volatile(&mut self.filter_regs(filter).dfltrsfcr, dfltrsfcr | (1 << 31));
            } else {
                write_volatile(&mut self.filter_regs(filter).dfltrsfcr, dfltrsfcr & !(1 << 31));
            }
        }
    }

    pub fn set_offset(&self, filter: Filter, offset: i16) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.filter_regs(filter).dfltrsfcr);
            write_volatile(&mut self.filter_regs(filter).dfltrsfcr,
                (dfltrsfcr & !0xFFFF_0000) | ((offset as u32) << 16));
        }
    }

    pub fn get_data_right_aligned(&self, filter: Filter) -> i32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltoutr) as i32 }
    }

    pub fn get_data_raw(&self, filter: Filter) -> i32 {
        unsafe { read_volatile(&self.filter_regs(filter).dfltrdhr) as i32 }
    }
}
