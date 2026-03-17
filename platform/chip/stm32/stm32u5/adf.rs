//! ADF - Audio Digital Filter
//! 音频数字滤波器
//!
//! ## STM32U5 ADF 特性 / Features
//! - **SINC 滤波器 / SINC Filter:**
//!   - 支持 Sinc1-Sinc5 阶滤波器
//!   - 可编程过采样率 (OSR)
//!   - 可编程高通滤波器 (HPF)
//!
//! - **时钟源 / Clock Sources:**
//!   - 内部时钟生成器
//!   - 外部时钟输入
//!   - 可编程时钟分频 (1-128 分频)
//!
//! - **输出 / Output:**
//!   - DMA 传输支持
//!   - 单次或循环模式
//!   - 左右声道数据输出
//!
//! - **触发源 / Trigger Sources:**
//!   - 软件触发
//!   - 硬件触发 (定时器, 外部中断等)
//!   - 多达 8 个触发源
//!
//! - **辅助功能 / Auxiliary Features:**
//!   - 输出电平检测 (OLD)
//!   - 可编程增益控制
//!   - 偏置校准
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 40: Audio digital filter (ADF)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// ADF1 base address / ADF1 基地址
/// AHB1 bus, accessible at 0x4004_2000
pub const ADF1_BASE: usize = 0x4004_2000;

// ============================================================================
// ADF Register Map / ADF 寄存器映射
// ============================================================================

/// ADF register structure / ADF 寄存器结构
#[repr(C)]
pub struct AdfRegs {
    /// ADF Global Control Register / ADF 全局控制寄存器
    pub gcr: u32,
    /// ADF Clock Generator Control Register / ADF 时钟生成控制寄存器
    pub ckgcr: u32,
    /// ADF Spare Control Register / ADF 备用控制寄存器
    pub sparecr: u32,
    /// ADF Spare Value Register / ADF 备用值寄存器
    pub spareval: u32,
    /// ADF Filter Instance Control Register x / ADF 滤波器实例控制寄存器 x
    pub idcr: u32,
    /// Reserved / 保留
    pub reserved1: [u32; 3],
    /// ADF Digital Filter Control Register / ADF 数字滤波器控制寄存器
    pub dfltcr: u32,
    /// ADF Digital Filter Channel Instance Control Register / ADF 数字滤波器通道实例控制寄存器
    pub dfltcicr: u32,
    /// ADF Digital Filter MICSFILT Control Register / ADF 数字滤波器 MICSFILT 控制寄存器
    pub dfltmsicr: u32,
    /// ADF Digital Filter Mode Register / ADF 数字滤波器模式寄存器
    pub dfltmdr: u32,
    /// ADF Digital Filter Duration Register / ADF 数字滤波器持续时间寄存器
    pub dfltdur: u32,
    /// ADF Digital Filter Interrupt Enable Register / ADF 数字滤波器中断使能寄存器
    pub dfltier: u32,
    /// ADF Digital Filter Interrupt Status Register / ADF 数字滤波器中断状态寄存器
    pub dfltisr: u32,
    /// ADF Digital Filter Status Interrupt Flag Register / ADF 数字滤波器状态中断标志寄存器
    pub dfltsifr: u32,
    /// ADF Digital Filter Interrupt Flag Register / ADF 数字滤波器中断标志寄存器
    pub dfltifr: u32,
    /// Reserved / 保留
    pub reserved2: u32,
    /// ADF Digital Filter INCIR Register / ADF 数字滤波器 INCIR 寄存器
    pub dfltincr: u32,
    /// Reserved / 保留
    pub reserved3: u32,
    /// ADF Digital Filter RSFCL Control Register / ADF 数字滤波器 RSFCL 控制寄存器
    pub dfltrsfcr: u32,
    /// Reserved / 保留
    pub reserved4: [u32; 2],
    /// ADF Digital Filter Output Right Data Register / ADF 数字滤波器输出右数据寄存器
    pub dfltoutr: u32,
    /// ADF Digital Filter Output Right Host Data Register / ADF 数字滤波器输出右主机数据寄存器
    pub dfltrdhr: u32,
    /// Reserved / 保留
    pub reserved5: [u32; 2],
    /// ADF Digital Filter Output Data Register / ADF 数字滤波器输出数据寄存器
    pub dfltoutdr: u32,
    /// Reserved / 保留
    pub reserved6: [u32; 3],
    /// ADF Digital Filter Trigger Control Register / ADF 数字滤波器触发控制寄存器
    pub dfltrgcr: u32,
    pub reserved7: [u32; 3],
    pub dfltgsr: u32,
    pub reserved8: [u32; 11],
    pub dfltoldcr: u32,
    pub dfltoldval: u32,
    pub dfltoldmin: u32,
    pub dfltoldmax: u32,
}

pub struct Adf;

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

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

/// Processing delay / 处理延迟
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Procdly {
    /// Delay 0 / 延迟 0
    Delay0 = 0,
    /// Delay 1 / 延迟 1
    Delay1 = 1,
    /// Delay 2 / 延迟 2
    Delay2 = 2,
    /// Delay 3 / 延迟 3
    Delay3 = 3,
}

/// Filter configuration / 滤波器配置
#[derive(Clone, Copy, Debug)]
pub struct FilterConfig {
    /// SINC filter order / SINC 滤波器阶数
    pub sinc_order: SincOrder,
    pub sinc_osr: u16,
    pub hpf_cutoff: u8,
    pub cko_prescaler: CkoPrescaler,
    pub cko_src: CkoSrc,
    pub dmaden: bool,
    pub dma_mode: DmaMode,
    pub old_enable: bool,
    pub old_threshold: u16,
    pub procdly: Procdly,
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
            procdly: Procdly::Delay0,
        }
    }
}

pub struct ClockConfig {
    pub ckgden: bool,
    pub ckgmod: bool,
    pub ckgen: u8,
    pub ckgosr: u8,
    pub ckgfreq: u8,
}

impl Default for ClockConfig {
    fn default() -> Self {
        ClockConfig {
            ckgden: false,
            ckgmod: false,
            ckgen: 0,
            ckgosr: 0,
            ckgfreq: 0,
        }
    }
}

pub struct AudioConfig {
    pub sample_rate: u32,
    pub bit_depth: u8,
    pub channels: u8,
    pub stereo_mode: bool,
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            sample_rate: 48000,
            bit_depth: 16,
            channels: 2,
            stereo_mode: true,
        }
    }
}

impl Adf {
    /// Create new ADF instance / 创建新的 ADF 实例
    pub const fn new() -> Self {
        Adf
    }

    fn regs(&self) -> &mut AdfRegs {
        unsafe { &mut *(ADF1_BASE as *mut AdfRegs) }
    }

    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr |= 1 << 24;
        }
    }

    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr &= !(1 << 24);
        }
    }

    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1rstr = rcc_base.add(0x90 / 4);
            *ahb1rstr |= 1 << 24;
            *ahb1rstr &= !(1 << 24);
        }
    }

    pub fn configure_clock(&self, config: &ClockConfig) {
        let ckgcr = (config.ckgden as u32) << 31
            | (config.ckgmod as u32) << 30
            | (config.ckgen as u32) << 16
            | (config.ckgosr as u32) << 8
            | (config.ckgfreq as u32) << 0;
        unsafe { write_volatile(&mut self.regs().ckgcr, ckgcr) };
    }

    pub fn configure_filter(&self, config: &FilterConfig) {
        unsafe {
            let dfltcr = (config.sinc_order as u32) << 13
                | (config.sinc_osr as u32) << 0
                | (config.procdly as u32) << 4;
            write_volatile(&mut self.regs().dfltcr, dfltcr);

            let dfltcicr = (config.hpf_cutoff as u32) << 0;
            write_volatile(&mut self.regs().dfltcicr, dfltcicr);

            let dfltmdr = (config.cko_prescaler as u32) << 4
                | (config.cko_src as u32) << 0;
            write_volatile(&mut self.regs().dfltmdr, dfltmdr);

            let dmaden_bit = if config.dmaden { 1u32 << 14 } else { 0 };
            let dma_mode_bits = (config.dma_mode as u32) << 12;
            write_volatile(&mut self.regs().dfltcr,
                read_volatile(&self.regs().dfltcr) | dmaden_bit | dma_mode_bits);

            if config.old_enable {
                let dfltoldcr = (1u32 << 31) | (config.old_threshold as u32);
                write_volatile(&mut self.regs().dfltoldcr, dfltoldcr);
            }
        }
    }

    pub fn enable(&self) {
        unsafe {
            let dfltcr = read_volatile(&self.regs().dfltcr);
            write_volatile(&mut self.regs().dfltcr, dfltcr | (1 << 31));
        }
    }

    pub fn disable(&self) {
        unsafe {
            let dfltcr = read_volatile(&self.regs().dfltcr);
            write_volatile(&mut self.regs().dfltcr, dfltcr & !(1 << 31));
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltcr) & (1 << 31)) != 0
        }
    }

    pub fn get_data(&self) -> i32 {
        unsafe { read_volatile(&self.regs().dfltoutdr) as i32 }
    }

    pub fn is_data_available(&self) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltisr) & 0x01) != 0
        }
    }

    pub fn clear_data_ready(&self) {
        unsafe {
            write_volatile(&mut self.regs().dfltifr, 0x01);
        }
    }

    pub fn enable_interrupt(&self, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.regs().dfltier);
            write_volatile(&mut self.regs().dfltier, dfltier | (1 << source));
        }
    }

    pub fn disable_interrupt(&self, source: u8) {
        unsafe {
            let dfltier = read_volatile(&self.regs().dfltier);
            write_volatile(&mut self.regs().dfltier, dfltier & !(1 << source));
        }
    }

    pub fn is_interrupt_active(&self, source: u8) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltisr) & (1 << source)) != 0
        }
    }

    pub fn clear_interrupt(&self, source: u8) {
        unsafe {
            write_volatile(&mut self.regs().dfltifr, 1 << source);
        }
    }

    pub fn get_old_value(&self) -> i16 {
        unsafe { read_volatile(&self.regs().dfltoldval) as i16 }
    }

    pub fn get_old_min(&self) -> i16 {
        unsafe { read_volatile(&self.regs().dfltoldmin) as i16 }
    }

    pub fn get_old_max(&self) -> i16 {
        unsafe { read_volatile(&self.regs().dfltoldmax) as i16 }
    }

    pub fn reset_old_counters(&self) {
        unsafe {
            let dfltoldcr = read_volatile(&self.regs().dfltoldcr);
            write_volatile(&mut self.regs().dfltoldcr, dfltoldcr | (1 << 30));
        }
    }

    pub fn configure_trigger(&self, trgsrc: Trgsrc, trgsens: bool) {
        unsafe {
            let dfltrgcr = (trgsrc as u32) << 4 | (trgsens as u32) << 1 | 1;
            write_volatile(&mut self.regs().dfltrgcr, dfltrgcr);
        }
    }

    pub fn software_trigger(&self) {
        unsafe {
            let dfltrgcr = read_volatile(&self.regs().dfltrgcr);
            write_volatile(&mut self.regs().dfltrgcr, dfltrgcr | (1 << 2));
        }
    }

    pub fn is_trigger_active(&self) -> bool {
        unsafe {
            (read_volatile(&self.regs().dfltgsr) & 0x01) != 0
        }
    }

    pub fn get_raw_counter(&self) -> u32 {
        unsafe { read_volatile(&self.regs().dfltincr) }
    }

    pub fn set_acquisition_mode(&self, continuous: bool) {
        unsafe {
            let dfltcr = read_volatile(&self.regs().dfltcr);
            if continuous {
                write_volatile(&mut self.regs().dfltcr, dfltcr | (1 << 30));
            } else {
                write_volatile(&mut self.regs().dfltcr, dfltcr & !(1 << 30));
            }
        }
    }

    pub fn set_scale_factor(&self, scale: u8) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.regs().dfltrsfcr);
            write_volatile(&mut self.regs().dfltrsfcr,
                (dfltrsfcr & !0xFF) | (scale as u32));
        }
    }

    pub fn enable_scale_offset(&self, enable: bool) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.regs().dfltrsfcr);
            if enable {
                write_volatile(&mut self.regs().dfltrsfcr, dfltrsfcr | (1 << 31));
            } else {
                write_volatile(&mut self.regs().dfltrsfcr, dfltrsfcr & !(1 << 31));
            }
        }
    }

    pub fn set_offset(&self, offset: i16) {
        unsafe {
            let dfltrsfcr = read_volatile(&self.regs().dfltrsfcr);
            write_volatile(&mut self.regs().dfltrsfcr,
                (dfltrsfcr & !0xFFFF_0000) | ((offset as u32) << 16));
        }
    }

    pub fn get_data_right_aligned(&self) -> i32 {
        unsafe { read_volatile(&self.regs().dfltoutr) as i32 }
    }

    pub fn get_data_raw(&self) -> i32 {
        unsafe { read_volatile(&self.regs().dfltrdhr) as i32 }
    }

    pub fn configure_audio(&self, audio_config: &AudioConfig) {
        let osr = match audio_config.sample_rate {
            8000 => 768,
            16000 => 384,
            32000 => 192,
            44100 => 136,
            48000 => 128,
            96000 => 64,
            _ => 128,
        };

        let sinc_order = if audio_config.sample_rate >= 96000 {
            SincOrder::Sinc2
        } else {
            SincOrder::Sinc3
        };

        let filter_config = FilterConfig {
            sinc_order,
            sinc_osr: osr,
            hpf_cutoff: 1,
            cko_prescaler: CkoPrescaler::Div1,
            cko_src: CkoSrc::Cko0,
            dmaden: true,
            dma_mode: DmaMode::Circular,
            old_enable: false,
            old_threshold: 0,
            procdly: Procdly::Delay0,
        };

        self.configure_filter(&filter_config);
    }

    pub fn set_gain(&self, gain_db: i8) {
        let scale = if gain_db >= 0 {
            (1u8 << (gain_db as u8 / 6)).min(255)
        } else {
            0
        };
        self.set_scale_factor(scale);
    }

    pub fn calibrate_offset(&self) -> i16 {
        unsafe {
            let dfltoldcr = read_volatile(&self.regs().dfltoldcr);
            write_volatile(&mut self.regs().dfltoldcr, dfltoldcr | (1 << 30));
            while (read_volatile(&self.regs().dfltoldcr) & (1 << 30)) != 0 {}
            read_volatile(&self.regs().dfltoldval) as i16
        }
    }

    pub fn wait_for_data(&self) -> i32 {
        while !self.is_data_available() {}
        let data = self.get_data();
        self.clear_data_ready();
        data
    }

    pub fn read_samples(&self, buffer: &mut [i32]) -> usize {
        let mut count = 0;
        for sample in buffer.iter_mut() {
            if self.is_data_available() {
                *sample = self.get_data();
                self.clear_data_ready();
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
