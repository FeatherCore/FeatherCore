//! FMAC - Filter Math Accelerator
//! 滤波器数学加速器
//!
//! ## STM32U5 FMAC 特性 / Features
//! - **支持滤波器类型 / Supported Filter Types:**
//!   - FIR (Finite Impulse Response) 有限冲激响应滤波器
//!   - IIR (Infinite Impulse Response) 无限冲激响应滤波器
//!
//! - **IIR 模式 / IIR Modes:**
//!   - Direct Form I / 直接型 I
//!   - Direct Form II / 直接型 II
//!   - Direct Form II Fast / 直接型 II 快速
//!
//! - **缓冲器 / Buffers:**
//!   - X1 buffer (输入数据) / X1 缓冲区 (输入数据)
//!   - X2 buffer (辅助输入) / X2 缓冲区 (辅助输入)
//!   - Y buffer (输出数据) / Y 缓冲区 (输出数据)
//!
//! - **功能 / Features:**
//!   - 可配置数据精度
//!   - DMA 支持
//!   - 饱和检测
//!   - 溢出保护
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 39: Filter math accelerator (FMAC)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// FMAC base address / FMAC 基地址
/// AHB1 bus, accessible at 0x4004_1400
pub const FMAC_BASE: usize = 0x4004_1400;

// ============================================================================
// FMAC Register Map / FMAC 寄存器映射
// ============================================================================

/// FMAC register structure / FMAC 寄存器结构
#[repr(C)]
pub struct FmacRegs {
    /// FMAC X1 Buffer Register 1 / FMAC X1 缓冲区寄存器 1
    pub x1y1: u32,
    /// FMAC X2/Y2 Buffer Register / FMAC X2/Y2 缓冲区寄存器
    pub x2y2: u32,
    /// FMAC X3/Y3 Buffer Register / FMAC X3/Y3 缓冲区寄存器
    pub x3y3: u32,
    /// FMAC X4/Y4 Buffer Register / FMAC X4/Y4 缓冲区寄存器
    pub x4y4: u32,
    /// FMAC X5/Y5 Buffer Register / FMAC X5/Y5 缓冲区寄存器
    pub x5y5: u32,
    /// FMAC X6/Y6 Buffer Register / FMAC X6/Y6 缓冲区寄存器
    pub x6y6: u32,
    /// FMAC X7/Y7 Buffer Register / FMAC X7/Y7 缓冲区寄存器
    pub x7y7: u32,
    /// FMAC X8/Y8 Buffer Register / FMAC X8/Y8 缓冲区寄存器
    pub x8y8: u32,
    /// FMAC X9/Y9 Buffer Register / FMAC X9/Y9 缓冲区寄存器
    pub x9y9: u32,
    /// FMAC X10/Y10 Buffer Register / FMAC X10/Y10 缓冲区寄存器
    pub x10y10: u32,
    /// FMAC X11/Y11 Buffer Register / FMAC X11/Y11 缓冲区寄存器
    pub x11y11: u32,
    /// FMAC X12/Y12 Buffer Register / FMAC X12/Y12 缓冲区寄存器
    pub x12y12: u32,
    /// FMAC X13/Y13 Buffer Register / FMAC X13/Y13 缓冲区寄存器
    pub x13y13: u32,
    /// FMAC X14/Y14 Buffer Register / FMAC X14/Y14 缓冲区寄存器
    pub x14y14: u32,
    /// FMAC X15/Y15 Buffer Register / FMAC X15/Y15 缓冲区寄存器
    pub x15y15: u32,
    /// FMAC X16/Y16 Buffer Register / FMAC X16/Y16 缓冲区寄存器
    pub x16y16: u32,
    /// FMAC Control Register / FMAC 控制寄存器
    pub cr: u32,
    /// FMAC Status Register / FMAC 状态寄存器
    pub sr: u32,
    /// FMAC Write Data Register / FMAC 写数据寄存器
    pub wdata: u32,
    /// FMAC Read Data Register / FMAC 读数据寄存器
    pub rdata: u32,
}

/// FMAC instance / FMAC 实例
pub struct Fmac;

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// FMAC function / FMAC 功能
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Function {
    /// Convolution FIR / 卷积 FIR
    ConvFir = 0,
    /// IIR Direct Form I / IIR 直接型 I
    ConvIirDirect1 = 1,
    /// IIR Direct Form II / IIR 直接型 II
    ConvIirDirect2 = 2,
    /// IIR Direct Form I Fast / IIR 直接型 I 快速
    ConvIirDirect1Fast = 3,
}

/// P parameter (Input preprocessing) / P 参数 (输入预处理)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P {
    /// P = 1 / P = 1
    P1 = 0,
    /// P = 2 / P = 2
    P2 = 1,
    /// P = 4 / P = 4
    P4 = 2,
    /// P = 8 / P = 8
    P8 = 3,
}

/// Q parameter (Coefficient preprocessing) / Q 参数 (系数预处理)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Q {
    /// Q = 1 / Q = 1
    Q1 = 0,
    /// Q = 2 / Q = 2
    Q2 = 1,
    /// Q = 4 / Q = 4
    Q4 = 2,
    /// Q = 8 / Q = 8
    Q8 = 3,
}

/// R parameter (Output postprocessing) / R 参数 (输出后处理)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R {
    /// R = 1 / R = 1
    R1 = 0,
    /// R = 2 / R = 2
    R2 = 1,
    /// R = 4 / R = 4
    R4 = 2,
    /// R = 8 / R = 8
    R8 = 3,
}

/// FMAC configuration / FMAC 配置
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Function selection / 功能选择
    pub func: Function,
    /// P parameter (Input preprocessing) / P 参数 (输入预处理)
    pub p: P,
    /// Q parameter (Coefficient preprocessing) / Q 参数 (系数预处理)
    pub q: Q,
    /// R parameter (Output postprocessing) / R 参数 (输出后处理)
    pub r: R,
    /// X1 buffer base address / X1 缓冲区基地址
    pub x1_buf_base: u8,
    /// X1 buffer size / X1 缓冲区大小
    pub x1_buf_size: u8,
    /// Y buffer base address / Y 缓冲区基地址
    pub y_buf_base: u8,
    /// Y buffer size / Y 缓冲区大小
    pub y_buf_size: u8,
    /// X2 buffer base address / X2 缓冲区基地址
    pub x2_buf_base: u8,
    /// X2 buffer size / X2 缓冲区大小
    pub x2_buf_size: u8,
    /// Clip enable / 裁剪使能
    pub clip: bool,
    /// Saturation enable / 饱和使能
    pub sat: bool,
    /// DMA input enable / DMA 输入使能
    pub din_dma: bool,
    /// DMA output enable / DMA 输出使能
    pub dout_dma: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            func: Function::ConvFir,
            p: P::P1,
            q: Q::Q1,
            r: R::R1,
            x1_buf_base: 0,
            x1_buf_size: 16,
            y_buf_base: 16,
            y_buf_size: 16,
            x2_buf_base: 32,
            x2_buf_size: 16,
            clip: false,
            sat: false,
            din_dma: false,
            dout_dma: false,
        }
    }
}

impl Fmac {
    /// Create new FMAC instance / 创建新的 FMAC 实例
    pub const fn new() -> Self {
        Fmac
    }

    /// Get FMAC registers / 获取 FMAC 寄存器
    fn regs(&self) -> &mut FmacRegs {
        unsafe { &mut *(FMAC_BASE as *mut FmacRegs) }
    }

    /// Enable FMAC clock / 使能 FMAC 时钟
    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr |= 1 << 16;  // FMACEN bit
        }
    }

    /// Disable FMAC clock / 禁用 FMAC 时钟
    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1enr = rcc_base.add(0xD8 / 4);
            *ahb1enr &= !(1 << 16);
        }
    }

    /// Reset FMAC / 复位 FMAC
    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb1rstr = rcc_base.add(0x90 / 4);
            *ahb1rstr |= 1 << 16;
            *ahb1rstr &= !(1 << 16);
        }
    }

    /// Configure FMAC / 配置 FMAC
    /// 
    /// # Arguments
    /// * `config` - FMAC configuration / FMAC 配置
    pub fn configure(&self, config: &Config) {
        let cr = (config.func as u32) << 0
            | (config.p as u32) << 4
            | (config.q as u32) << 6
            | (config.r as u32) << 8
            | (config.x1_buf_base as u32) << 16
            | (config.x1_buf_size as u32) << 24
            | (config.clip as u32) << 12
            | (config.sat as u32) << 13
            | (config.din_dma as u32) << 14
            | (config.dout_dma as u32) << 15;
        unsafe { write_volatile(&mut self.regs().cr, cr) };

        let x2y2 = (config.x2_buf_base as u32) << 16 | (config.x2_buf_size as u32) << 24;
        unsafe { write_volatile(&mut self.regs().x2y2, x2y2) };

        let x3y3 = (config.y_buf_base as u32) << 16 | (config.y_buf_size as u32) << 24;
        unsafe { write_volatile(&mut self.regs().x3y3, x3y3) };
    }

    /// Enable FMAC / 使能 FMAC
    pub fn enable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 31));
        }
    }

    /// Disable FMAC / 禁用 FMAC
    pub fn disable(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr & !(1 << 31));
        }
    }

    /// Check if FMAC is ready / 检查 FMAC 是否就绪
    pub fn is_ready(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) & 0x01) != 0 }
    }

    /// Write data / 写数据
    /// 
    /// # Arguments
    /// * `data` - Data to write / 要写入的数据
    pub fn write_data(&self, data: i16) {
        while !self.is_ready() {}
        unsafe { write_volatile(&mut self.regs().wdata, data as u32) };
    }

    /// Read data / 读数据
    /// 
    /// # Returns
    /// Read data / 读取的数据
    pub fn read_data(&self) -> i16 {
        while !self.is_ready() {}
        unsafe { read_volatile(&self.regs().rdata) as i16 }
    }

    /// Write coefficient / 写系数
    /// 
    /// # Arguments
    /// * `index` - Coefficient index / 系数索引
    /// * `coeff` - Coefficient value / 系数值
    pub fn write_coeff(&self, index: usize, coeff: i16) {
        let addr = FMAC_BASE + 0x100 + index * 4;
        unsafe { write_volatile(addr as *mut u32, coeff as u32) };
    }

    /// Read coefficient / 读系数
    /// 
    /// # Arguments
    /// * `index` - Coefficient index / 系数索引
    /// 
    /// # Returns
    /// Coefficient value / 系数值
    pub fn read_coeff(&self, index: usize) -> i16 {
        let addr = FMAC_BASE + 0x100 + index * 4;
        unsafe { read_volatile(addr as *const u32) as i16 }
    }

    /// Clear X1 buffer / 清除 X1 缓冲区
    pub fn clear_x1_buffer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 30));
            while (read_volatile(&self.regs().cr) & (1 << 30)) != 0 {}
        }
    }

    /// Clear X2 buffer / 清除 X2 缓冲区
    pub fn clear_x2_buffer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 29));
            while (read_volatile(&self.regs().cr) & (1 << 29)) != 0 {}
        }
    }

    /// Clear Y buffer / 清除 Y 缓冲区
    pub fn clear_y_buffer(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 28));
            while (read_volatile(&self.regs().cr) & (1 << 28)) != 0 {}
        }
    }

    /// Reset filter / 复位滤波器
    pub fn reset_filter(&self) {
        unsafe {
            let cr = read_volatile(&self.regs().cr);
            write_volatile(&mut self.regs().cr, cr | (1 << 27));
            while (read_volatile(&self.regs().cr) & (1 << 27)) != 0 {}
        }
    }

    /// Get X1 buffer full watermark / 获取 X1 缓冲区满水印
    pub fn get_x1_full_wm(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 4) & 0x03) as u8 }
    }

    /// Get Y buffer empty watermark / 获取 Y 缓冲区空水印
    pub fn get_y_empty_wm(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 6) & 0x03) as u8 }
    }

    /// Check if X1 buffer is full / 检查 X1 缓冲区是否满
    pub fn is_x1_full(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 8) & 0x01 != 0 }
    }

    /// Check if X2 buffer is full / 检查 X2 缓冲区是否满
    pub fn is_x2_full(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 9) & 0x01 != 0 }
    }

    /// Check if Y buffer is empty / 检查 Y 缓冲区是否空
    pub fn is_y_empty(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 10) & 0x01 != 0 }
    }

    /// Check saturation flag / 检查饱和标志
    pub fn is_saturation(&self) -> bool {
        unsafe { (read_volatile(&self.regs().sr) >> 11) & 0x01 != 0 }
    }

    /// Clear saturation flag / 清除饱和标志
    pub fn clear_saturation_flag(&self) {
        unsafe { write_volatile(&mut self.regs().sr, 1 << 11) };
    }

    /// Get unread samples count / 获取未读样本数
    pub fn get_unread_samples(&self) -> u8 {
        unsafe { ((read_volatile(&self.regs().sr) >> 16) & 0xFF) as u8 }
    }

    /// Load coefficients / 加载系数
    /// 
    /// # Arguments
    /// * `coeffs` - Coefficient array / 系数数组
    pub fn load_coefficients(&self, coeffs: &[i16]) {
        for (i, &coeff) in coeffs.iter().enumerate() {
            self.write_coeff(i, coeff);
        }
    }

    /// Process single sample / 处理单个样本
    /// 
    /// # Arguments
    /// * `sample` - Input sample / 输入样本
    /// 
    /// # Returns
    /// Output sample / 输出样本
    pub fn process_sample(&self, sample: i16) -> i16 {
        self.write_data(sample);
        self.read_data()
    }

    /// Process buffer / 处理缓冲区
    /// 
    /// # Arguments
    /// * `input` - Input buffer / 输入缓冲区
    /// * `output` - Output buffer / 输出缓冲区
    pub fn process_buffer(&self, input: &[i16], output: &mut [i16]) {
        for (i, &sample) in input.iter().enumerate() {
            if i < output.len() {
                output[i] = self.process_sample(sample);
            }
        }
    }
}
