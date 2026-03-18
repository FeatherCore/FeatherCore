//! DMA2D V1 - Chrom-ART Accelerator (DMA2D)
//! Chrom-ART 加速器 (DMA2D) - 版本1
//!
//! # Overview / 概述
//! The DMA2D (Chrom-ART Accelerator) is a dedicated graphics DMA controller
//! that offloads the CPU from memory-to-memory data transfers, image composition,
//! and pixel format conversion.
//!
//! # Features / 功能特性
//! - Memory-to-memory transfer
//! - Memory-to-peripheral transfer
//! - Pixel format conversion
//! - Alpha blending
//! - Clipping
//! - Color filling
//!
//! # Reference / 参考
//! - RM0456 Chapter 19: Chrom-ART accelerator (DMA2D)
//! - RM0456 Chapter 20: Chrom-ART accelerator (DMA2D) - V1

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// DMA2D base address / DMA2D 基地址
/// Reference: RM0456 Chapter 2, Table 1
pub const DMA2D_BASE: usize = 0x4002_B000;

#[derive(Clone, Copy, Debug)]
pub enum Dma2dMode {
    MemoryToMemory,
    MemoryToMemoryPFC,
    RegisterToMemory,
    MemoryToMemoryPFCBlend,
}

#[derive(Clone, Copy, Debug)]
pub enum Dma2dColorMode {
    ARGB8888,
    RGB888,
    RGB565,
    ARGB1555,
    ARGB4444,
    L8,
    AL44,
    AL88,
    L4,
    A8,
}

#[derive(Clone, Copy, Debug)]
pub struct Dma2dConfig {
    pub mode: Dma2dMode,
    pub output_color_mode: Dma2dColorMode,
    pub output_line_offset: u16,
    pub number_of_lines: u16,
    pub pixel_per_line: u16,
}

impl Default for Dma2dConfig {
    fn default() -> Self {
        Self {
            mode: Dma2dMode::MemoryToMemory,
            output_color_mode: Dma2dColorMode::ARGB8888,
            output_line_offset: 0,
            number_of_lines: 0,
            pixel_per_line: 0,
        }
    }
}

impl Dma2dConfig {
    pub fn configure(&self) {
        unsafe {
            let cr = DMA2D_BASE as *mut u32;
            let mode_val = match self.mode {
                Dma2dMode::MemoryToMemory => 0x0,
                Dma2dMode::MemoryToMemoryPFC => 0x1,
                Dma2dMode::RegisterToMemory => 0x2,
                Dma2dMode::MemoryToMemoryPFCBlend => 0x3,
            };
            write_volatile(cr, mode_val << 16);
            
            let ocmar = (DMA2D_BASE + 0x10) as *mut u32;
            write_volatile(ocmar, 0);
            
            let ocmr = (DMA2D_BASE + 0x14) as *mut u32;
            let color_mode_val = match self.output_color_mode {
                Dma2dColorMode::ARGB8888 => 0x0,
                Dma2dColorMode::RGB888 => 0x1,
                Dma2dColorMode::RGB565 => 0x2,
                Dma2dColorMode::ARGB1555 => 0x3,
                Dma2dColorMode::ARGB4444 => 0x4,
                Dma2dColorMode::L8 => 0x5,
                Dma2dColorMode::AL44 => 0x6,
                Dma2dColorMode::AL88 => 0x7,
                Dma2dColorMode::L4 => 0x8,
                Dma2dColorMode::A8 => 0x9,
            };
            write_volatile(ocmr, color_mode_val);
            
            let oor = (DMA2D_BASE + 0x18) as *mut u32;
            write_volatile(oor, self.output_line_offset as u32);
            
            let nlr = (DMA2D_BASE + 0x1C) as *mut u32;
            write_volatile(nlr, 
                ((self.number_of_lines as u32) << 16) | 
                (self.pixel_per_line as u32)
            );
        }
    }
}

pub fn init() {
    unsafe {
        let rcc = (0x4002_1014 as *mut u32);
        let val = read_volatile(rcc);
        write_volatile(rcc, val | (1 << 18));
    }
}

pub fn start_transfer(config: &Dma2dConfig, src_addr: usize, dst_addr: usize) {
    config.configure();
    
    unsafe {
        let fgmar = (DMA2D_BASE + 0x08) as *mut u32;
        write_volatile(fgmar, src_addr as u32);
        
        let omr = (DMA2D_BASE + 0x0C) as *mut u32;
        write_volatile(omr, dst_addr as u32);
        
        let cr = DMA2D_BASE as *mut u32;
        write_volatile(cr, read_volatile(cr) | 1);
    }
}

pub fn is_transfer_complete() -> bool {
    unsafe {
        let isr = (DMA2D_BASE + 0x20) as *const u32;
        (read_volatile(isr) & 0x1) != 0
    }
}

pub fn clear_transfer_complete() {
    unsafe {
        let icr = (DMA2D_BASE + 0x24) as *mut u32;
        write_volatile(icr, 0x1);
    }
}
