//! DMA - Direct Memory Access
//! 直接内存访问控制器
//!
//! # Overview / 概述
//! STM32U5 Direct Memory Access (DMA) controllers provide high-speed data transfer
//! between memory and peripherals without CPU intervention.
//! 
//! # Features / 功能特性
//! Reference: RM0456 Chapter 11: DMA controller (GPDMA) and Chapter 19: Low-power DMA (LPDMA)
//! 
//! ## DMA Controllers / DMA控制器
//! - **GPDMA1:** 16-channel General Purpose DMA
//! - **LPDMA1:** 4-channel Low Power DMA
//! 
//! ## Transfer Types / 传输类型
//! - Memory to Memory
//! - Peripheral to Memory
//! - Memory to Peripheral
//! 
//! ## Advanced Features / 高级特性
//! - Linked-list mode
//! - Circular mode
//! - Double buffering
//! - FIFO support
//! - Burst transfer
//! 
//! # Reference / 参考
//! - RM0456 Chapter 11: DMA controller (GPDMA)
//! - RM0456 Chapter 19: Low-power DMA (LPDMA)
//! - RM0456 Section 11.1: GPDMA introduction
//! - RM0456 Section 14.1: LPDMA introduction

/// GPDMA1 base address / GPDMA1 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const GPDMA1_BASE: usize = 0x4002_1000;
/// LPDMA1 base address / LPDMA1 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const LPDMA1_BASE: usize = 0x4002_7000;

/// DMAMUX1 base address / DMAMUX1 基地址
//! Reference: RM0456 Chapter 2, Table 1
pub const DMAMUX1_BASE: usize = 0x4002_0800;

/// DMA channel register offsets / DMA 通道寄存器偏移 (per channel)
pub mod ch_reg {
    /// DMA channel x linked-list address register / DMA 通道 x 链接列表地址寄存器
    pub const LBAR: usize = 0x00;
    /// DMA channel x flag clear register
    pub const FCR: usize = 0x04;
    /// DMA channel x status register
    pub const SR: usize = 0x08;
    /// DMA channel x control register
    pub const CR: usize = 0x0C;
}

/// DMA channel configuration structure
#[derive(Clone, Copy, Debug)]
pub struct ChannelConfig {
    /// Source address
    pub source_addr: usize,
    /// Destination address
    pub dest_addr: usize,
    /// Number of data units to transfer
    pub buffer_size: u32,
    /// Source data width (0=byte, 1=half-word, 2=word)
    pub source_width: u8,
    /// Destination data width (0=byte, 1=half-word, 2=word)
    pub dest_width: u8,
    /// Source increment (0=fixed, 1=increment)
    pub source_inc: bool,
    /// Destination increment (0=fixed, 1=increment)
    pub dest_inc: bool,
    /// Transfer mode (0=normal, 1=circular)
    pub circular: bool,
    /// Priority level (0=low, 1=medium, 2=high, 3=very high)
    pub priority: u8,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            source_addr: 0,
            dest_addr: 0,
            buffer_size: 0,
            source_width: 0,
            dest_width: 0,
            source_inc: true,
            dest_inc: true,
            circular: false,
            priority: 1,
        }
    }
}

/// DMA channel
pub struct Channel {
    /// DMA controller base address
    dma_base: usize,
    /// Channel number (0-15 for GPDMA1, 0-3 for LPDMA1)
    channel: u8,
}

impl Channel {
    /// Create GPDMA1 channel
    pub const fn gpdma1_ch(channel: u8) -> Self {
        assert!(channel < 16, "GPDMA1 channel must be 0-15");
        Self {
            dma_base: GPDMA1_BASE,
            channel,
        }
    }

    /// Create LPDMA1 channel
    pub const fn lpdma1_ch(channel: u8) -> Self {
        assert!(channel < 4, "LPDMA1 channel must be 0-3");
        Self {
            dma_base: LPDMA1_BASE,
            channel,
        }
    }

    /// Get channel base address
    fn ch_base(&self) -> usize {
        // Each channel has 0x40 bytes of registers
        self.dma_base + 0x50 + (self.channel as usize * 0x40)
    }

    /// Initialize DMA channel
    pub fn init(&self, config: &ChannelConfig) {
        unsafe {
            // Disable channel first
            self.disable();

            // Wait for channel to be disabled
            while self.is_enabled() {}

            // Clear all flags
            self.clear_flags();

            // Configure linked-list base address
            let lbar = (self.ch_base() + ch_reg::LBAR) as *mut u32;
            core::ptr::write_volatile(lbar, config.source_addr as u32);

            // Configure control register
            let cr = (self.ch_base() + ch_reg::CR) as *mut u32;
            let mut cr_val = 0;
            cr_val |= (config.source_width as u32) << 0;  // SWIDTH
            cr_val |= (config.dest_width as u32) << 3;    // DWIDTH
            cr_val |= (config.source_inc as u32) << 6;    // SINC
            cr_val |= (config.dest_inc as u32) << 7;      // DINC
            cr_val |= (config.circular as u32) << 8;      // CIRC
            cr_val |= (config.priority as u32) << 22;     // PL
            core::ptr::write_volatile(cr, cr_val);

            // Note: In real implementation, need to set up linked list nodes
            // for source/dest addresses and buffer size
        }
    }

    /// Enable DMA channel
    pub fn enable(&self) {
        unsafe {
            let cr = (self.ch_base() + ch_reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 0; // EN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable DMA channel
    pub fn disable(&self) {
        unsafe {
            let cr = (self.ch_base() + ch_reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 0); // EN
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Check if channel is enabled
    pub fn is_enabled(&self) -> bool {
        unsafe {
            let cr = (self.ch_base() + ch_reg::CR) as *mut u32;
            let val = core::ptr::read_volatile(cr);
            (val & (1 << 0)) != 0
        }
    }

    /// Clear all interrupt flags
    pub fn clear_flags(&self) {
        unsafe {
            let fcr = (self.ch_base() + ch_reg::FCR) as *mut u32;
            core::ptr::write_volatile(fcr, 0x0000_007F); // Clear all flags
        }
    }

    /// Check if transfer complete
    pub fn is_transfer_complete(&self) -> bool {
        unsafe {
            let sr = (self.ch_base() + ch_reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 0)) != 0 // TCF
        }
    }

    /// Check if half transfer
    pub fn is_half_transfer(&self) -> bool {
        unsafe {
            let sr = (self.ch_base() + ch_reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 1)) != 0 // HTF
        }
    }

    /// Check if transfer error
    pub fn is_transfer_error(&self) -> bool {
        unsafe {
            let sr = (self.ch_base() + ch_reg::SR) as *mut u32;
            let val = core::ptr::read_volatile(sr);
            (val & (1 << 2)) != 0 // TEF
        }
    }

    /// Enable transfer complete interrupt
    pub fn enable_tc_interrupt(&self) {
        unsafe {
            let cr = (self.ch_base() + ch_reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val |= 1 << 17; // TCIE
            core::ptr::write_volatile(cr, val);
        }
    }

    /// Disable transfer complete interrupt
    pub fn disable_tc_interrupt(&self) {
        unsafe {
            let cr = (self.ch_base() + ch_reg::CR) as *mut u32;
            let mut val = core::ptr::read_volatile(cr);
            val &= !(1 << 17); // TCIE
            core::ptr::write_volatile(cr, val);
        }
    }
}

/// DMAMUX request generators
pub mod dmamux_req {
    pub const MEM2MEM: u8 = 0;
    pub const GPDMA1_CH0_TCF: u8 = 1;
    pub const GPDMA1_CH1_TCF: u8 = 2;
    pub const TIM2_UP: u8 = 3;
    pub const TIM2_TRG: u8 = 4;
    pub const TIM2_CH1: u8 = 5;
    pub const TIM2_CH2: u8 = 6;
    pub const TIM2_CH3: u8 = 7;
    pub const TIM2_CH4: u8 = 8;
    pub const TIM3_UP: u8 = 9;
    pub const TIM3_TRG: u8 = 10;
    pub const TIM3_CH1: u8 = 11;
    pub const TIM3_CH2: u8 = 12;
    pub const TIM3_CH3: u8 = 13;
    pub const TIM3_CH4: u8 = 14;
    pub const TIM4_UP: u8 = 15;
    pub const TIM4_TRG: u8 = 16;
    pub const TIM4_CH1: u8 = 17;
    pub const TIM4_CH2: u8 = 18;
    pub const TIM4_CH3: u8 = 19;
    pub const TIM5_UP: u8 = 20;
    pub const TIM5_TRG: u8 = 21;
    pub const TIM5_CH1: u8 = 22;
    pub const TIM5_CH2: u8 = 23;
    pub const TIM5_CH3: u8 = 24;
    pub const TIM5_CH4: u8 = 25;
    pub const TIM6_UP: u8 = 26;
    pub const TIM7_UP: u8 = 27;
    pub const TIM8_UP: u8 = 28;
    pub const TIM8_CH1: u8 = 29;
    pub const TIM8_CH2: u8 = 30;
    pub const TIM8_CH3: u8 = 31;
    pub const TIM8_CH4: u8 = 32;
    pub const TIM15_UP: u8 = 33;
    pub const TIM15_CH1: u8 = 34;
    pub const TIM15_CH2: u8 = 35;
    pub const TIM15_COM: u8 = 36;
    pub const TIM15_TRIG: u8 = 37;
    pub const TIM16_UP: u8 = 38;
    pub const TIM16_CH1: u8 = 39;
    pub const TIM17_UP: u8 = 40;
    pub const TIM17_CH1: u8 = 41;
    pub const USART1_RX: u8 = 42;
    pub const USART1_TX: u8 = 43;
    pub const USART2_RX: u8 = 44;
    pub const USART2_TX: u8 = 45;
    pub const USART3_RX: u8 = 46;
    pub const USART3_TX: u8 = 47;
    pub const UART4_RX: u8 = 48;
    pub const UART4_TX: u8 = 49;
    pub const UART5_RX: u8 = 50;
    pub const UART5_TX: u8 = 51;
    pub const LPUART1_RX: u8 = 52;
    pub const LPUART1_TX: u8 = 53;
    pub const SPI1_RX: u8 = 54;
    pub const SPI1_TX: u8 = 55;
    pub const SPI2_RX: u8 = 56;
    pub const SPI2_TX: u8 = 57;
    pub const SPI3_RX: u8 = 58;
    pub const SPI3_TX: u8 = 59;
    pub const I2C1_RX: u8 = 60;
    pub const I2C1_TX: u8 = 61;
    pub const I2C2_RX: u8 = 62;
    pub const I2C2_TX: u8 = 63;
    pub const I2C3_RX: u8 = 64;
    pub const I2C3_TX: u8 = 65;
    pub const ADC1: u8 = 66;
    pub const ADC4: u8 = 67;
    pub const DAC1_CH1: u8 = 68;
    pub const DAC1_CH2: u8 = 69;
    pub const SAI1_A: u8 = 70;
    pub const SAI1_B: u8 = 71;
    pub const SAI2_A: u8 = 72;
    pub const SAI2_B: u8 = 73;
    pub const OCTOSPI1: u8 = 74;
    pub const OCTOSPI2: u8 = 75;
    pub const SDMMC1: u8 = 76;
    pub const SDMMC2: u8 = 77;
    pub const HASH_IN: u8 = 78;
    pub const AES_IN: u8 = 79;
    pub const AES_OUT: u8 = 80;
    pub const DCMI: u8 = 81;
    pub const LPTIM1_IC1: u8 = 82;
    pub const LPTIM1_IC2: u8 = 83;
    pub const LPTIM1_UE: u8 = 84;
    pub const LPTIM2_IC1: u8 = 85;
    pub const LPTIM2_IC2: u8 = 86;
    pub const LPTIM2_UE: u8 = 87;
    pub const LPTIM3_IC1: u8 = 88;
    pub const LPTIM3_IC2: u8 = 89;
    pub const LPTIM3_UE: u8 = 90;
    pub const FDCAN1_RX: u8 = 91;
    pub const FDCAN1_TX: u8 = 92;
    pub const CORDIC_READ: u8 = 93;
    pub const CORDIC_WRITE: u8 = 94;
    pub const I3C1_RX: u8 = 95;
    pub const I3C1_TX: u8 = 96;
    pub const I3C1_TC: u8 = 97;
    pub const I3C1_RS: u8 = 98;
    pub const LPTIM4_IC1: u8 = 99;
    pub const LPTIM4_UE: u8 = 100;
    pub const LPTIM5_IC1: u8 = 101;
    pub const LPTIM5_UE: u8 = 102;
    pub const ADF1_FLT0: u8 = 103;
}

/// Configure DMAMUX for a channel
pub fn dmamux_config_channel(channel: u8, request_id: u8) {
    assert!(channel < 16, "DMAMUX channel must be 0-15");
    
    unsafe {
        let ccr = (DMAMUX1_BASE + (channel as usize * 4)) as *mut u32;
        let mut val = core::ptr::read_volatile(ccr);
        val &= !(0x7F << 0); // Clear DMAREQ_ID
        val |= (request_id as u32) << 0;
        core::ptr::write_volatile(ccr, val);
    }
}

/// Initialize DMA controller
pub fn init() {
    // Enable DMA clocks
    crate::rcc::enable_ahb1_clock(crate::rcc::ahb1::DMA1);
    crate::rcc::enable_ahb1_clock(crate::rcc::ahb1::DMA2);
    crate::rcc::enable_ahb1_clock(crate::rcc::ahb1::DMAMUX1);
}

/// Memory to memory transfer
pub fn mem2mem_transfer(src: usize, dst: usize, len: u32) {
    let ch = Channel::gpdma1_ch(0);
    
    // Configure DMAMUX for memory-to-memory
    dmamux_config_channel(0, dmamux_req::MEM2MEM);
    
    let config = ChannelConfig {
        source_addr: src,
        dest_addr: dst,
        buffer_size: len,
        source_width: 2, // Word
        dest_width: 2,   // Word
        source_inc: true,
        dest_inc: true,
        circular: false,
        priority: 3,
    };
    
    ch.init(&config);
    ch.enable();
    
    // Wait for completion
    while !ch.is_transfer_complete() {}
    ch.clear_flags();
}
