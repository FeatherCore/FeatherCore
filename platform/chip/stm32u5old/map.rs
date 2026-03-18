//! STM32U5 Memory Map and Register Definitions
//! STM32U5 内存映射和寄存器定义
//!
//! This module provides complete address mapping for the STM32U5 series
//! microcontroller, including both non-secure and secure peripherals.
//!
//! # Address Space Overview / 地址空间概览
//! - 0x0000_0000 - 0x1FFF_FFFF: Code/Flash region (512MB)
//! - 0x2000_0000 - 0x3FFF_FFFF: SRAM region (512MB)
//! - 0x4000_0000 - 0x4FFF_FFFF: Peripherals (256MB) - Non-secure alias
//! - 0x5000_0000 - 0x5FFF_FFFF: Peripherals (256MB) - Secure alias
//! - 0x6000_0000 - 0x9FFF_FFFF: External memory (1GB)
//!
//! # Reference / 参考
//! - RM0456 Chapter 2: Memory and bus architecture
//! - RM0456 Chapter 2, Table 1: STM32U5 peripheral register boundary addresses

//! Core peripherals base addresses / 内核外设基地址
pub mod core {
    //! Debug Flash Patch and Breakpoint (FPB)
    pub const FPB_BASE: usize = 0xE000_2000;

    //! Data Watchpoint and Trace (DWT)
    pub const DWT_BASE: usize = 0xE000_1000;

    //! System Control Block (SCB)
    pub const SCB_BASE: usize = 0xE000_ED00;

    //! SysTick Timer
    pub const SYSTICK_BASE: usize = 0xE000_E010;

    //! Nested Vectored Interrupt Controller (NVIC)
    pub const NVIC_BASE: usize = 0xE000_E100;

    //! Floating Point Unit (FPU)
    pub const FPU_BASE: usize = 0xE000_EF30;

    //! Memory Protection Unit (MPU)
    pub const MPU_BASE: usize = 0xE000_ED90;

    //! TrustZone Address Space Controller (SAU)
    pub const SAU_BASE: usize = 0xE000_EDD0;

    //! Debug DebugControl Block (DCB)
    pub const DCB_BASE: usize = 0xE000_EDF0;
}

//! PPB (Private Peripheral Bus) peripherals / PPB 外设
pub mod ppb {
    //! Debug MCU Configuration (DBGMCU)
    pub const DBGMCU_BASE: usize = 0xE004_2000;

    //! Device Electronic Signature (DES)
    pub const DES_BASE: usize = 0xE001_0000;

    //! Unique Device ID (UDID)
    pub const UID_BASE: usize = 0xE001_8000;

    //! Flash Size
    pub const FLASHSIZE_BASE: usize = 0xE001_8010;

    //! Package Type
    pub const PACKAGE_BASE: usize = 0xE001_8014;
}

//! Non-Secure Base Addresses / 非安全域基地址
//!
//! STM32U5 uses TrustZone, with peripherals having both non-secure (NS) and secure (S) aliases.
//! Non-secure peripherals are accessed via 0x4000_0000 - 0x4FFF_FFFF.
//! Secure peripherals are accessed via 0x5000_0000 - 0x5FFF_FFFF.
pub mod ns {
    //! Base address for all non-secure peripherals
    pub const PERIPH_BASE: usize = 0x4000_0000;

    //! APB1 peripheral base
    pub const APB1_PERIPH_BASE: usize = PERIPH_BASE + 0x0000;

    //! APB2 peripheral base
    pub const APB2_PERIPH_BASE: usize = PERIPH_BASE + 0x1000;

    //! AHB1 peripheral base
    pub const AHB1_PERIPH_BASE: usize = PERIPH_BASE + 0x2000;

    //! AHB2 peripheral base
    pub const AHB2_PERIPH_BASE: usize = PERIPH_BASE + 0x4000;

    //! AHB3 peripheral base (external memory)
    pub const AHB3_PERIPH_BASE: usize = PERIPH_BASE + 0x6000;

    // APB1 Peripherals / APB1 外设

    //! TIM2 base address
    //! Reference: RM0456 Chapter 55
    pub const TIM2_BASE: usize = APB1_PERIPH_BASE + 0x0000;

    //! TIM3 base address
    pub const TIM3_BASE: usize = APB1_PERIPH_BASE + 0x0400;

    //! TIM4 base address
    pub const TIM4_BASE: usize = APB1_PERIPH_BASE + 0x0800;

    //! TIM5 base address
    pub const TIM5_BASE: usize = APB1_PERIPH_BASE + 0x0C00;

    //! TIM6 base address
    //! Reference: RM0456 Chapter 57
    pub const TIM6_BASE: usize = APB1_PERIPH_BASE + 0x1000;

    //! TIM7 base address
    pub const TIM7_BASE: usize = APB1_PERIPH_BASE + 0x1400;

    //! RTC base address
    //! Reference: RM0456 Chapter 63
    pub const RTC_BASE: usize = APB1_PERIPH_BASE + 0x1800;

    //! WWDG base address
    //! Reference: RM0456 Chapter 62
    pub const WWDG_BASE: usize = APB1_PERIPH_BASE + 0x2C00;

    //! IWDG base address
    //! Reference: RM0456 Chapter 61
    pub const IWDG_BASE: usize = APB1_PERIPH_BASE + 0x3000;

    //! SPI2 base address
    //! Reference: RM0456 Chapter 68
    pub const SPI2_BASE: usize = APB1_PERIPH_BASE + 0x3800;

    //! SPI3 base address
    pub const SPI3_BASE: usize = APB1_PERIPH_BASE + 0x3C00;

    //! USART2 base address
    //! Reference: RM0456 Chapter 66-67
    pub const USART2_BASE: usize = APB1_PERIPH_BASE + 0x4400;

    //! USART3 base address
    pub const USART3_BASE: usize = APB1_PERIPH_BASE + 0x4800;

    //! UART4 base address
    pub const UART4_BASE: usize = APB1_PERIPH_BASE + 0x4C00;

    //! UART5 base address
    pub const UART5_BASE: usize = APB1_PERIPH_BASE + 0x5000;

    //! I2C1 base address
    //! Reference: RM0456 Chapter 65
    pub const I2C1_BASE: usize = APB1_PERIPH_BASE + 0x5400;

    //! I2C2 base address
    pub const I2C2_BASE: usize = APB1_PERIPH_BASE + 0x5800;

    //! I2C3 base address
    pub const I2C3_BASE: usize = APB1_PERIPH_BASE + 0x5C00;

    //! CRS base address
    //! Reference: RM0456 Chapter 12
    pub const CRS_BASE: usize = APB1_PERIPH_BASE + 0x6000;

    //! CAN1 (FDCAN) base address
    //! Reference: RM0456 Chapter 70
    pub const CAN1_BASE: usize = APB1_PERIPH_BASE + 0x6400;

    //! CAN2 (FDCAN) base address
    pub const CAN2_BASE: usize = APB1_PERIPH_BASE + 0x6800;

    //! USB base address
    //! Reference: RM0456 Chapter 71-74
    pub const USB_BASE: usize = APB1_PERIPH_BASE + 0x6800;

    //! PWR base address
    //! Reference: RM0456 Chapter 10
    pub const PWR_BASE: usize = APB1_PERIPH_BASE + 0x7000;

    //! DAC1 base address
    //! Reference: RM0456 Chapter 35
    pub const DAC1_BASE: usize = APB1_PERIPH_BASE + 0x7400;

    //! OPAMP1 base address
    //! Reference: RM0456 Chapter 38
    pub const OPAMP1_BASE: usize = APB1_PERIPH_BASE + 0x7800;

    //! OPAMP2 base address
    pub const OPAMP2_BASE: usize = APB1_PERIPH_BASE + 0x7C00;

    //! LPTIM1 base address
    //! Reference: RM0456 Chapter 58
    pub const LPTIM1_BASE: usize = APB1_PERIPH_BASE + 0x8000;

    //! LPTIM2 base address
    pub const LPTIM2_BASE: usize = APB1_PERIPH_BASE + 0x8400;

    //! LPTIM3 base address
    pub const LPTIM3_BASE: usize = APB1_PERIPH_BASE + 0x8800;

    //! I2C4 base address
    pub const I2C4_BASE: usize = APB1_PERIPH_BASE + 0x8C00;

    //! LPUART1 base address
    //! Reference: RM0456 Chapter 67
    pub const LPUART1_BASE: usize = APB1_PERIPH_BASE + 0x9000;

    // APB2 Peripherals / APB2 外设

    //! TIM1 base address
    //! Reference: RM0456 Chapter 54
    pub const TIM1_BASE: usize = APB2_PERIPH_BASE + 0x0000;

    //! TIM8 base address
    pub const TIM8_BASE: usize = APB2_PERIPH_BASE + 0x0400;

    //! SPI1 base address
    pub const SPI1_BASE: usize = APB2_PERIPH_BASE + 0x3000;

    //! TIM15 base address
    //! Reference: RM0456 Chapter 56
    pub const TIM15_BASE: usize = APB2_PERIPH_BASE + 0x4000;

    //! TIM16 base address
    pub const TIM16_BASE: usize = APB2_PERIPH_BASE + 0x4400;

    //! TIM17 base address
    pub const TIM17_BASE: usize = APB2_PERIPH_BASE + 0x4800;

    //! USART1 base address
    pub const USART1_BASE: usize = APB2_PERIPH_BASE + 0x3800;

    //! SAI1 base address
    //! Reference: RM0456 Chapter 69
    pub const SAI1_BASE: usize = APB2_PERIPH_BASE + 0x5400;

    //! SAI2 base address
    pub const SAI2_BASE: usize = APB2_PERIPH_BASE + 0x5800;

    // AHB1 Peripherals / AHB1 外设

    //! GPDMA1 base address
    //! Reference: RM0456 Chapter 17
    pub const GPDMA1_BASE: usize = AHB1_PERIPH_BASE + 0x0000;

    //! DMAMUX1 base address
    pub const DMAMUX1_BASE: usize = AHB1_PERIPH_BASE + 0x0400;

    //! CORDIC base address
    //! Reference: RM0456 Chapter 25
    pub const CORDIC_BASE: usize = AHB1_PERIPH_BASE + 0x0C00;

    //! FMAC base address
    //! Reference: RM0456 Chapter 26
    pub const FMAC_BASE: usize = AHB1_PERIPH_BASE + 0x1000;

    //! MDF1 base address
    //! Reference: RM0456 Chapter 39
    pub const MDF1_BASE: usize = AHB1_PERIPH_BASE + 0x1400;

    //! ADF1 base address
    //! Reference: RM0456 Chapter 40
    pub const ADF1_BASE: usize = AHB1_PERIPH_BASE + 0x1800;

    //! Flash Interface base address
    //! Reference: RM0456 Chapter 7
    pub const FLASH_BASE: usize = AHB1_PERIPH_BASE + 0x2000;

    //! ICACHE base address
    //! Reference: RM0456 Chapter 8
    pub const ICACHE_BASE: usize = AHB1_PERIPH_BASE + 0x2400;

    //! DCACHE base address
    //! Reference: RM0456 Chapter 9
    pub const DCACHE_BASE: usize = AHB1_PERIPH_BASE + 0x2400;

    //! CRC base address
    //! Reference: RM0456 Chapter 24
    pub const CRC_BASE: usize = AHB1_PERIPH_BASE + 0x3000;

    //! TSC base address
    //! Reference: RM0456 Chapter 47
    pub const TSC_BASE: usize = AHB1_PERIPH_BASE + 0x4000;

    //! RAMCFG base address
    //! Reference: RM0456 Chapter 6
    pub const RAMCFG_BASE: usize = AHB1_PERIPH_BASE + 0x4400;

    //! DMA2D base address
    //! Reference: RM0456 Chapter 44-45
    pub const DMA2D_BASE: usize = AHB1_PERIPH_BASE + 0xB000;

    // AHB2 Peripherals / AHB2 外设

    //! GPIO Port A base address
    //! Reference: RM0456 Chapter 13
    pub const GPIOA_BASE: usize = AHB2_PERIPH_BASE + 0x0000;

    //! GPIO Port B base address
    pub const GPIOB_BASE: usize = AHB2_PERIPH_BASE + 0x0400;

    //! GPIO Port C base address
    pub const GPIOC_BASE: usize = AHB2_PERIPH_BASE + 0x0800;

    //! GPIO Port D base address
    pub const GPIOD_BASE: usize = AHB2_PERIPH_BASE + 0x0C00;

    //! GPIO Port E base address
    pub const GPIOE_BASE: usize = AHB2_PERIPH_BASE + 0x1000;

    //! GPIO Port F base address
    pub const GPIOF_BASE: usize = AHB2_PERIPH_BASE + 0x1400;

    //! GPIO Port G base address
    pub const GPIOG_BASE: usize = AHB2_PERIPH_BASE + 0x1800;

    //! GPIO Port H base address
    pub const GPIOH_BASE: usize = AHB2_PERIPH_BASE + 0x1C00;

    //! GPIO Port I base address
    pub const GPIOI_BASE: usize = AHB2_PERIPH_BASE + 0x2000;

    // AHB2 Peripheral 2 (AHB2ENR2) / AHB2 外设2

    //! ADC1 base address
    //! Reference: RM0456 Chapter 33
    pub const ADC1_BASE: usize = AHB2_PERIPH_BASE + 0x8000;

    //! ADC2 base address
    pub const ADC2_BASE: usize = AHB2_PERIPH_BASE + 0x8400;

    //! ADC4 base address
    //! Reference: RM0456 Chapter 34
    pub const ADC4_BASE: usize = AHB2_PERIPH_BASE + 0x8800;

    //! AES base address
    //! Reference: RM0456 Chapter 49
    pub const AES_BASE: usize = AHB2_PERIPH_BASE + 0x8C00;

    //! HASH base address
    //! Reference: RM0456 Chapter 51
    pub const HASH_BASE: usize = AHB2_PERIPH_BASE + 0x9400;

    //! RNG base address
    //! Reference: RM0456 Chapter 48
    pub const RNG_BASE: usize = AHB2_PERIPH_BASE + 0x9800;

    //! SAES base address
    //! Reference: RM0456 Chapter 50
    pub const SAES_BASE: usize = AHB2_PERIPH_BASE + 0x9C00;

    //! PKA base address
    //! Reference: RM0456 Chapter 53
    pub const PKA_BASE: usize = AHB2_PERIPH_BASE + 0xA000;

    //! OTFDEC1 base address
    //! Reference: RM0456 Chapter 52
    pub const OTFDEC1_BASE: usize = AHB2_PERIPH_BASE + 0xA400;

    //! OTFDEC2 base address
    pub const OTFDEC2_BASE: usize = AHB2_PERIPH_BASE + 0xA800;

    //! SDMMC1 base address
    //! Reference: RM0456 Chapter 31
    pub const SDMMC1_BASE: usize = AHB2_PERIPH_BASE + 0xC000;

    //! SDMMC2 base address
    pub const SDMMC2_BASE: usize = AHB2_PERIPH_BASE + 0xC400;

    // AHB3 Peripherals / AHB3 外设

    //! FMC base address
    //! Reference: RM0456 Chapter 27
    pub const FMC_BASE: usize = AHB3_PERIPH_BASE + 0x0000;

    //! OCTOSPI1 base address
    //! Reference: RM0456 Chapter 28
    pub const OCTOSPI1_BASE: usize = AHB3_PERIPH_BASE + 0x0400;

    //! OCTOSPI2 base address
    pub const OCTOSPI2_BASE: usize = AHB3_PERIPH_BASE + 0x0800;

    //! XSPI base address (OctoSPI in QSPI mode)
    pub const XSPI1_BASE: usize = AHB3_PERIPH_BASE + 0x0400;

    //! SDMMC1 alternate base
    pub const SDMMC1_ALT_BASE: usize = AHB3_PERIPH_BASE + 0x1000;

    //! SDMMC2 alternate base
    pub const SDMMC2_ALT_BASE: usize = AHB3_PERIPH_BASE + 0x1400;

    //! LTDC base address
    //! Reference: RM0456 Chapter 43
    pub const LTDC_BASE: usize = AHB3_PERIPH_BASE + 0x6800;

    //! DSI base address
    //! Reference: RM0456 Chapter 46
    pub const DSI_BASE: usize = AHB3_PERIPH_BASE + 0x6C00;

    //! DCMI base address
    //! Reference: RM0456 Chapter 41
    pub const DCMI_BASE: usize = AHB3_PERIPH_BASE + 0xC000;

    //! PSSI base address
    //! Reference: RM0456 Chapter 42
    pub const PSSI_BASE: usize = AHB3_PERIPH_BASE + 0xC400;

    // Other Peripherals / 其他外设

    //! SYSCFG base address
    //! Reference: RM0456 Chapter 15
    pub const SYSCFG_BASE: usize = 0x4002_0000;

    //! COMP1 base address
    //! Reference: RM0456 Chapter 37
    pub const COMP1_BASE: usize = 0x4000_9200;

    //! COMP2 base address
    pub const COMP2_BASE: usize = 0x4000_9204;

    //! VREFBUF base address
    //! Reference: RM0456 Chapter 36
    pub const VREFBUF_BASE: usize = 0x4000_7030;

    //! TAMP base address
    //! Reference: RM0456 Chapter 63
    pub const TAMP_BASE: usize = 0x4200_0400;

    //! Backup registers base
    pub const BKP_REG_BASE: usize = 0x4200_0080;

    //! EXTI base address
    //! Reference: RM0456 Chapter 23
    pub const EXTI_BASE: usize = 0x4002_0400;

    //! NVIC base address
    pub const NVIC_BASE: usize = 0x4002_1000;

    //! IRTIM base address
    pub const IRTIM_BASE: usize = 0x4001_5800;

    //! USB OTG FS base address
    //! Reference: RM0456 Chapter 72
    pub const USB_OTG_FS_BASE: usize = 0x4204_0000;

    //! USB OTG HS base address
    //! Reference: RM0456 Chapter 73
    pub const USB_OTG_HS_BASE: usize = 0x4204_4000;

    //! HSPI1 base address
    pub const HSPI1_BASE: usize = 0x4202_0000;

    //! GPU2D base address
    pub const GPU2D_BASE: usize = 0x4202_4000;

    //! JPEG base address
    pub const JPEG_BASE: usize = 0x5200_4000;

    //! DLYB1 base address
    pub const DLYB1_BASE: usize = 0x4201_6400;

    //! DLYB2 base address
    pub const DLYB2_BASE: usize = 0x4201_6800;

    //! HRTIM base address
    //! Reference: RM0456 Chapter 37
    pub const HRTIM_BASE: usize = 0x4001_7400;

    //! GFXTIM base address
    pub const GFXTIM_BASE: usize = 0x5000_0000;

    //! GFXMMU base address
    pub const GFXMMU_BASE: usize = 0x5100_0000;

    //! UCPD1 base address
    //! Reference: RM0456 Chapter 74
    pub const UCPD1_BASE: usize = 0x4000_DC00;

    //! UCPD2 base address
    pub const UCPD2_BASE: usize = 0x4000_E000;

    //! LPDMA1 base address
    //! Reference: RM0456 Chapter 18
    pub const LPDMA1_BASE: usize = 0x4002_7000;
}

//! Secure Base Addresses / 安全域基地址
//!
//! Secure peripherals use the same offset but with 0x5000_0000 base.
//! Reference: RM0456 Chapter 2, Table 2
pub mod s {
    //! Base address for all secure peripherals
    pub const PERIPH_BASE_SEC: usize = 0x5000_0000;

    //! APB1 secure peripheral base
    pub const APB1_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x0000;

    //! APB2 secure peripheral base
    pub const APB2_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x1000;

    //! AHB1 secure peripheral base
    pub const AHB1_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x2000;

    //! AHB2 secure peripheral base
    pub const AHB2_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x4000;

    //! AHB3 secure peripheral base
    pub const AHB3_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x6000;

    // Secure APB1 Peripherals / 安全 APB1 外设

    //! TIM2 secure base
    pub const TIM2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0000;

    //! TIM3 secure base
    pub const TIM3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0400;

    //! TIM4 secure base
    pub const TIM4_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0800;

    //! TIM5 secure base
    pub const TIM5_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0C00;

    //! TIM6 secure base
    pub const TIM6_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1000;

    //! TIM7 secure base
    pub const TIM7_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1400;

    //! RTC secure base
    pub const RTC_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1800;

    //! WWDG secure base
    pub const WWDG_BASE: usize = APB1_PERIPH_BASE_SEC + 0x2C00;

    //! IWDG secure base
    pub const IWDG_BASE: usize = APB1_PERIPH_BASE_SEC + 0x3000;

    //! SPI2 secure base
    pub const SPI2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x3800;

    //! SPI3 secure base
    pub const SPI3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x3C00;

    //! USART2 secure base
    pub const USART2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x4400;

    //! USART3 secure base
    pub const USART3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x4800;

    //! UART4 secure base
    pub const UART4_BASE: usize = APB1_PERIPH_BASE_SEC + 0x4C00;

    //! UART5 secure base
    pub const UART5_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5000;

    //! I2C1 secure base
    pub const I2C1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5400;

    //! I2C2 secure base
    pub const I2C2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5800;

    //! I2C3 secure base
    pub const I2C3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5C00;

    //! CRS secure base
    pub const CRS_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6000;

    //! CAN1 secure base
    pub const CAN1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6400;

    //! CAN2 secure base
    pub const CAN2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6800;

    //! PWR secure base
    pub const PWR_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7000;

    //! DAC1 secure base
    pub const DAC1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7400;

    //! OPAMP1 secure base
    pub const OPAMP1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7800;

    //! OPAMP2 secure base
    pub const OPAMP2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7C00;

    //! LPTIM1 secure base
    pub const LPTIM1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8000;

    //! LPTIM2 secure base
    pub const LPTIM2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8400;

    //! LPTIM3 secure base
    pub const LPTIM3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8800;

    //! I2C4 secure base
    pub const I2C4_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8C00;

    //! LPUART1 secure base
    pub const LPUART1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x9000;

    // Secure APB2 Peripherals / 安全 APB2 外设

    //! TIM1 secure base
    pub const TIM1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x0000;

    //! TIM8 secure base
    pub const TIM8_BASE: usize = APB2_PERIPH_BASE_SEC + 0x0400;

    //! SPI1 secure base
    pub const SPI1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x3000;

    //! TIM15 secure base
    pub const TIM15_BASE: usize = APB2_PERIPH_BASE_SEC + 0x4000;

    //! TIM16 secure base
    pub const TIM16_BASE: usize = APB2_PERIPH_BASE_SEC + 0x4400;

    //! TIM17 secure base
    pub const TIM17_BASE: usize = APB2_PERIPH_BASE_SEC + 0x4800;

    //! USART1 secure base
    pub const USART1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x3800;

    //! SAI1 secure base
    pub const SAI1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x5400;

    //! SAI2 secure base
    pub const SAI2_BASE: usize = APB2_PERIPH_BASE_SEC + 0x5800;

    // Secure AHB1 Peripherals / 安全 AHB1 外设

    //! GPDMA1 secure base
    pub const GPDMA1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x0000;

    //! DMAMUX1 secure base
    pub const DMAMUX1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x0400;

    //! CORDIC secure base
    pub const CORDIC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x0C00;

    //! FMAC secure base
    pub const FMAC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x1000;

    //! MDF1 secure base
    pub const MDF1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x1400;

    //! ADF1 secure base
    pub const ADF1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x1800;

    //! FLASH secure base
    pub const FLASH_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x2000;

    //! ICACHE secure base
    pub const ICACHE_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x2400;

    //! DCACHE secure base
    pub const DCACHE_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x2400;

    //! CRC secure base
    pub const CRC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x3000;

    //! TSC secure base
    pub const TSC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x4000;

    //! RAMCFG secure base
    pub const RAMCFG_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x4400;

    //! DMA2D secure base
    pub const DMA2D_BASE: usize = AHB1_PERIPH_BASE_SEC + 0xB000;

    // Secure AHB2 Peripherals / 安全 AHB2 外设

    //! GPIOA secure base
    pub const GPIOA_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0000;

    //! GPIOB secure base
    pub const GPIOB_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0400;

    //! GPIOC secure base
    pub const GPIOC_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0800;

    //! GPIOD secure base
    pub const GPIOD_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0C00;

    //! GPIOE secure base
    pub const GPIOE_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1000;

    //! GPIOF secure base
    pub const GPIOF_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1400;

    //! GPIOG secure base
    pub const GPIOG_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1800;

    //! GPIOH secure base
    pub const GPIOH_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1C00;

    //! GPIOI secure base
    pub const GPIOI_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x2000;

    //! ADC1 secure base
    pub const ADC1_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8000;

    //! ADC2 secure base
    pub const ADC2_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8400;

    //! ADC4 secure base
    pub const ADC4_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8800;

    //! AES secure base
    pub const AES_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8C00;

    //! HASH secure base
    pub const HASH_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x9400;

    //! RNG secure base
    pub const RNG_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x9800;

    //! SAES secure base
    pub const SAES_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x9C00;

    //! PKA secure base
    pub const PKA_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xA000;

    //! OTFDEC1 secure base
    pub const OTFDEC1_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xA400;

    //! OTFDEC2 secure base
    pub const OTFDEC2_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xA800;

    //! SDMMC1 secure base
    pub const SDMMC1_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xC000;

    //! SDMMC2 secure base
    pub const SDMMC2_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xC400;

    // Secure AHB3 Peripherals / 安全 AHB3 外设

    //! FMC secure base
    pub const FMC_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0000;

    //! OCTOSPI1 secure base
    pub const OCTOSPI1_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0400;

    //! OCTOSPI2 secure base
    pub const OCTOSPI2_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0800;

    //! SDMMC1 alternate secure base
    pub const SDMMC1_ALT_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x1000;

    //! SDMMC2 alternate secure base
    pub const SDMMC2_ALT_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x1400;

    //! LTDC secure base
    pub const LTDC_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x6800;

    //! DSI secure base
    pub const DSI_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x6C00;

    //! DCMI secure base
    pub const DCMI_BASE: usize = AHB3_PERIPH_BASE_SEC + 0xC000;

    //! PSSI secure base
    pub const PSSI_BASE: usize = AHB3_PERIPH_BASE_SEC + 0xC400;

    // Other Secure Peripherals / 其他安全外设

    //! SYSCFG secure base
    pub const SYSCFG_BASE: usize = 0x5002_0000;

    //! COMP1 secure base
    pub const COMP1_BASE: usize = 0x5000_9200;

    //! COMP2 secure base
    pub const COMP2_BASE: usize = 0x5000_9204;

    //! VREFBUF secure base
    pub const VREFBUF_BASE: usize = 0x5000_7030;

    //! TAMP secure base
    pub const TAMP_BASE: usize = 0x5200_0400;

    //! Backup registers secure base
    pub const BKP_REG_BASE: usize = 0x5200_0080;

    //! EXTI secure base
    pub const EXTI_BASE: usize = 0x5002_0400;

    //! NVIC secure base
    pub const NVIC_BASE: usize = 0x5002_1000;

    //! IRTIM secure base
    pub const IRTIM_BASE: usize = 0x5001_5800;

    //! USB OTG FS secure base
    pub const USB_OTG_FS_BASE: usize = 0x5204_0000;

    //! USB OTG HS secure base
    pub const USB_OTG_HS_BASE: usize = 0x5204_4000;

    //! HSPI1 secure base
    pub const HSPI1_BASE: usize = 0x5202_0000;

    //! GPU2D secure base
    pub const GPU2D_BASE: usize = 0x5202_4000;

    //! JPEG secure base
    pub const JPEG_BASE: usize = 0x5300_4000;

    //! HRTIM secure base
    pub const HRTIM_BASE: usize = 0x5001_7400;

    //! UCPD1 secure base
    pub const UCPD1_BASE: usize = 0x5000_DC00;

    //! UCPD2 secure base
    pub const UCPD2_BASE: usize = 0x5000_E000;

    //! LPDMA1 secure base
    pub const LPDMA1_BASE: usize = 0x5002_7000;
}

//! RCC Register Definitions / RCC 寄存器定义
//!
//! Reference: RM0456 Chapter 11
pub mod rcc {
    //! RCC base (non-secure)
    pub const RCC_BASE: usize = 0x4002_1000;

    //! RCC base (secure)
    pub const RCC_BASE_SEC: usize = 0x5002_1000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Clock Control Register
        //! Reference: RM0456 Section 11.10.1
        pub const CR: usize = 0x00;

        //! Internal Clock Sources Calibration Register 1
        pub const ICSCR1: usize = 0x08;

        //! Internal Clock Sources Calibration Register 2
        pub const ICSCR2: usize = 0x0C;

        //! Internal Clock Sources Calibration Register 3
        pub const ICSCR3: usize = 0x10;

        //! Clock Recovery RC Register
        pub const CRRCR: usize = 0x14;

        //! Clock Configuration Register 1
        pub const CFGR1: usize = 0x1C;

        //! Clock Configuration Register 2
        pub const CFGR2: usize = 0x20;

        //! Clock Configuration Register 3
        pub const CFGR3: usize = 0x24;

        //! PLL1 Configuration Register
        pub const PLL1CFGR: usize = 0x28;

        //! PLL2 Configuration Register
        pub const PLL2CFGR: usize = 0x2C;

        //! PLL3 Configuration Register
        pub const PLL3CFGR: usize = 0x30;

        //! PLL1 Divider Configuration Register
        pub const PLL1DIVR: usize = 0x34;

        //! PLL1 Fractional Divider Register
        pub const PLL1FRACR: usize = 0x38;

        //! PLL2 Divider Configuration Register
        pub const PLL2DIVR: usize = 0x3C;

        //! PLL2 Fractional Divider Register
        pub const PLL2FRACR: usize = 0x40;

        //! PLL3 Divider Configuration Register
        pub const PLL3DIVR: usize = 0x44;

        //! PLL3 Fractional Divider Register
        pub const PLL3FRACR: usize = 0x48;

        //! Clock Interrupt Enable Register
        pub const CIER: usize = 0x50;

        //! Clock Interrupt Flag Register
        pub const CIFR: usize = 0x54;

        //! Clock Interrupt Clear Register
        pub const CICR: usize = 0x58;

        //! AHB1 Peripheral Reset Register
        pub const AHB1RSTR: usize = 0x60;

        //! AHB2 Peripheral Reset Register 1
        pub const AHB2RSTR1: usize = 0x64;

        //! AHB2 Peripheral Reset Register 2
        pub const AHB2RSTR2: usize = 0x68;

        //! AHB3 Peripheral Reset Register
        pub const AHB3RSTR: usize = 0x6C;

        //! APB1 Peripheral Reset Register 1
        pub const APB1RSTR1: usize = 0x74;

        //! APB1 Peripheral Reset Register 2
        pub const APB1RSTR2: usize = 0x78;

        //! APB2 Peripheral Reset Register
        pub const APB2RSTR: usize = 0x7C;

        //! APB3 Peripheral Reset Register
        pub const APB3RSTR: usize = 0x80;

        //! AHB1 Peripheral Clock Enable Register
        pub const AHB1ENR: usize = 0x88;

        //! AHB2 Peripheral Clock Enable Register 1
        pub const AHB2ENR1: usize = 0x8C;

        //! AHB2 Peripheral Clock Enable Register 2
        pub const AHB2ENR2: usize = 0x90;

        //! AHB3 Peripheral Clock Enable Register
        pub const AHB3ENR: usize = 0x94;

        //! APB1 Peripheral Clock Enable Register 1
        pub const APB1ENR1: usize = 0x9C;

        //! APB1 Peripheral Clock Enable Register 2
        pub const APB1ENR2: usize = 0xA0;

        //! APB2 Peripheral Clock Enable Register
        pub const APB2ENR: usize = 0xA4;

        //! APB3 Peripheral Clock Enable Register
        pub const APB3ENR: usize = 0xA8;

        //! AHB1 Peripheral Sleep/Stop Mode Clock Enable Register
        pub const AHB1SMENR: usize = 0xB0;

        //! AHB2 Peripheral Sleep/Stop Mode Clock Enable Register 1
        pub const AHB2SMENR1: usize = 0xB4;

        //! AHB2 Peripheral Sleep/Stop Mode Clock Enable Register 2
        pub const AHB2SMENR2: usize = 0xB8;

        //! AHB3 Peripheral Sleep/Stop Mode Clock Enable Register
        pub const AHB3SMENR: usize = 0xBC;

        //! APB1 Peripheral Sleep/Stop Mode Clock Enable Register 1
        pub const APB1SMENR1: usize = 0xC4;

        //! APB1 Peripheral Sleep/Stop Mode Clock Enable Register 2
        pub const APB1SMENR2: usize = 0xC8;

        //! APB2 Peripheral Sleep/Stop Mode Clock Enable Register
        pub const APB2SMENR: usize = 0xCC;

        //! APB3 Peripheral Sleep/Stop Mode Clock Enable Register
        pub const APB3SMENR: usize = 0xD0;

        //! SRD Autonomous Mode Register
        pub const SRDAMR: usize = 0xD8;

        //! Peripheral Independent Clock Configuration Register 1
        pub const CCIPR1: usize = 0xE0;

        //! Peripheral Independent Clock Configuration Register 2
        pub const CCIPR2: usize = 0xE4;

        //! Peripheral Independent Clock Configuration Register 3
        pub const CCIPR3: usize = 0xE8;

        //! Backup Domain Control Register
        pub const BDCR: usize = 0xF0;

        //! Control/Status Register
        pub const CSR: usize = 0xF4;

        //! RCC Security Configuration Register
        pub const SECCFGR: usize = 0x110;

        //! RCC Privilege Configuration Register
        pub const PRIVCFGR: usize = 0x114;
    }

    //! CR Register Bit Definitions
    //! Reference: RM0456 Section 11.10.1
    pub mod cr_bits {
        //! MSIS clock enable
        pub const MSISON: u32 = 1 << 0;

        //! MSIK always enable
        pub const MSIKERON: u32 = 1 << 1;

        //! MSIS clock ready
        pub const MSISRDY: u32 = 1 << 2;

        //! MSIS PLL enable
        pub const MSIPLLEN: u32 = 1 << 3;

        //! MSIK clock enable
        pub const MSIKON: u32 = 1 << 4;

        //! MSIK clock ready
        pub const MSIKRDY: u32 = 1 << 5;

        //! HSI16 clock enable
        pub const HSION: u32 = 1 << 8;

        //! HSI16 always enable
        pub const HSIKERON: u32 = 1 << 9;

        //! HSI16 clock ready
        pub const HSIRDY: u32 = 1 << 10;

        //! HSI48 clock enable
        pub const HSI48ON: u32 = 1 << 12;

        //! HSI48 clock ready
        pub const HSI48RDY: u32 = 1 << 13;

        //! SHSI clock enable
        pub const SHSION: u32 = 1 << 14;

        //! SHSI clock ready
        pub const SHSIRDY: u32 = 1 << 15;

        //! HSE clock enable
        pub const HSEON: u32 = 1 << 16;

        //! HSE clock ready
        pub const HSERDY: u32 = 1 << 17;

        //! HSE crystal bypass
        pub const HSEBYP: u32 = 1 << 18;

        //! Clock security system enable
        pub const CSSON: u32 = 1 << 19;

        //! PLL1 enable
        pub const PLL1ON: u32 = 1 << 24;

        //! PLL1 clock ready
        pub const PLL1RDY: u32 = 1 << 25;

        //! PLL2 enable
        pub const PLL2ON: u32 = 1 << 26;

        //! PLL2 clock ready
        pub const PLL2RDY: u32 = 1 << 27;

        //! PLL3 enable
        pub const PLL3ON: u32 = 1 << 28;

        //! PLL3 clock ready
        pub const PLL3RDY: u32 = 1 << 29;
    }
}

//! GPIO Register Definitions / GPIO 寄存器定义
//!
//! Reference: RM0456 Chapter 13
pub mod gpio {
    //! GPIO Register offsets / 寄存器偏移
    pub mod reg {
        //! GPIO port mode register
        //! Reference: RM0456 Section 13.4.1
        pub const MODER: usize = 0x00;

        //! GPIO port output type register
        //! Reference: RM0456 Section 13.4.2
        pub const OTYPER: usize = 0x04;

        //! GPIO port output speed register
        //! Reference: RM0456 Section 13.4.3
        pub const OSPEEDR: usize = 0x08;

        //! GPIO port pull-up/pull-down register
        //! Reference: RM0456 Section 13.4.4
        pub const PUPDR: usize = 0x0C;

        //! GPIO port input data register
        //! Reference: RM0456 Section 13.4.5
        pub const IDR: usize = 0x10;

        //! GPIO port output data register
        //! Reference: RM0456 Section 13.4.6
        pub const ODR: usize = 0x14;

        //! GPIO port bit set/reset register
        //! Reference: RM0456 Section 13.4.7
        pub const BSRR: usize = 0x18;

        //! GPIO port configuration lock register
        //! Reference: RM0456 Section 13.4.8
        pub const LCKR: usize = 0x1C;

        //! GPIO alternate function low register
        //! Reference: RM0456 Section 13.4.9
        pub const AFRL: usize = 0x20;

        //! GPIO alternate function high register
        pub const AFRH: usize = 0x24;

        //! GPIO port bit reset register
        //! Reference: RM0456 Section 13.4.10
        pub const BRR: usize = 0x28;

        //! GPIO port secure configuration register
        //! Reference: RM0456 Section 13.4.11
        pub const SECCFGR: usize = 0x30;
    }

    //! MODER Register Bit Descriptions / MODER 寄存器位描述
    pub mod moder_bits {
        //! Pin 0 mode
        pub const MODER0: u32 = 0b11 << 0;
        //! Pin 1 mode
        pub const MODER1: u32 = 0b11 << 2;
        //! Pin 2 mode
        pub const MODER2: u32 = 0b11 << 4;
        //! Pin 3 mode
        pub const MODER3: u32 = 0b11 << 6;
        //! Pin 4 mode
        pub const MODER4: u32 = 0b11 << 8;
        //! Pin 5 mode
        pub const MODER5: u32 = 0b11 << 10;
        //! Pin 6 mode
        pub const MODER6: u32 = 0b11 << 12;
        //! Pin 7 mode
        pub const MODER7: u32 = 0b11 << 14;
        //! Pin 8 mode
        pub const MODER8: u32 = 0b11 << 16;
        //! Pin 9 mode
        pub const MODER9: u32 = 0b11 << 18;
        //! Pin 10 mode
        pub const MODER10: u32 = 0b11 << 20;
        //! Pin 11 mode
        pub const MODER11: u32 = 0b11 << 22;
        //! Pin 12 mode
        pub const MODER12: u32 = 0b11 << 24;
        //! Pin 13 mode
        pub const MODER13: u32 = 0b11 << 26;
        //! Pin 14 mode
        pub const MODER14: u32 = 0b11 << 28;
        //! Pin 15 mode
        pub const MODER15: u32 = 0b11 << 30;
    }

    //! OTYPER Register Bit Descriptions / OTYPER 寄存器位描述
    pub mod otyper_bits {
        //! Pin 0 output type (0: Push-pull, 1: Open-drain)
        pub const OT0: u32 = 1 << 0;
        //! Pin 1 output type
        pub const OT1: u32 = 1 << 1;
        //! Pin 2 output type
        pub const OT2: u32 = 1 << 2;
        //! Pin 3 output type
        pub const OT3: u32 = 1 << 3;
        //! Pin 4 output type
        pub const OT4: u32 = 1 << 4;
        //! Pin 5 output type
        pub const OT5: u32 = 1 << 5;
        //! Pin 6 output type
        pub const OT6: u32 = 1 << 6;
        //! Pin 7 output type
        pub const OT7: u32 = 1 << 7;
        //! Pin 8 output type
        pub const OT8: u32 = 1 << 8;
        //! Pin 9 output type
        pub const OT9: u32 = 1 << 9;
        //! Pin 10 output type
        pub const OT10: u32 = 1 << 10;
        //! Pin 11 output type
        pub const OT11: u32 = 1 << 11;
        //! Pin 12 output type
        pub const OT12: u32 = 1 << 12;
        //! Pin 13 output type
        pub const OT13: u32 = 1 << 13;
        //! Pin 14 output type
        pub const OT14: u32 = 1 << 14;
        //! Pin 15 output type
        pub const OT15: u32 = 1 << 15;
    }

    //! OSPEEDR Register Bit Descriptions / OSPEEDR 寄存器位描述
    pub mod ospeedr_bits {
        //! Pin 0 speed
        pub const OSPEEDR0: u32 = 0b11 << 0;
        //! Pin 1 speed
        pub const OSPEEDR1: u32 = 0b11 << 2;
        //! Pin 2 speed
        pub const OSPEEDR2: u32 = 0b11 << 4;
        //! Pin 3 speed
        pub const OSPEEDR3: u32 = 0b11 << 6;
        //! Pin 4 speed
        pub const OSPEEDR4: u32 = 0b11 << 8;
        //! Pin 5 speed
        pub const OSPEEDR5: u32 = 0b11 << 10;
        //! Pin 6 speed
        pub const OSPEEDR6: u32 = 0b11 << 12;
        //! Pin 7 speed
        pub const OSPEEDR7: u32 = 0b11 << 14;
    }

    //! PUPDR Register Bit Descriptions / PUPDR 寄存器位描述
    pub mod pupdr_bits {
        //! Pin 0 pull-up/pull-down
        pub const PUPDR0: u32 = 0b11 << 0;
        //! Pin 1 pull-up/pull-down
        pub const PUPDR1: u32 = 0b11 << 2;
        //! Pin 2 pull-up/pull-down
        pub const PUPDR2: u32 = 0b11 << 4;
        //! Pin 3 pull-up/pull-down
        pub const PUPDR3: u32 = 0b11 << 6;
        //! Pin 4 pull-up/pull-down
        pub const PUPDR4: u32 = 0b11 << 8;
        //! Pin 5 pull-up/pull-down
        pub const PUPDR5: u32 = 0b11 << 10;
        //! Pin 6 pull-up/pull-down
        pub const PUPDR6: u32 = 0b11 << 12;
        //! Pin 7 pull-up/pull-down
        pub const PUPDR7: u32 = 0b11 << 14;
    }

    //! BSRR Register Bit Descriptions / BSRR 寄存器位描述
    pub mod bsrr_bits {
        //! Pin 0 set
        pub const BS0: u32 = 1 << 0;
        //! Pin 1 set
        pub const BS1: u32 = 1 << 1;
        //! Pin 2 set
        pub const BS2: u32 = 1 << 2;
        //! Pin 3 set
        pub const BS3: u32 = 1 << 3;
        //! Pin 4 set
        pub const BS4: u32 = 1 << 4;
        //! Pin 5 set
        pub const BS5: u32 = 1 << 5;
        //! Pin 6 set
        pub const BS6: u32 = 1 << 6;
        //! Pin 7 set
        pub const BS7: u32 = 1 << 7;
        //! Pin 8 set
        pub const BS8: u32 = 1 << 8;
        //! Pin 9 set
        pub const BS9: u32 = 1 << 9;
        //! Pin 10 set
        pub const BS10: u32 = 1 << 10;
        //! Pin 11 set
        pub const BS11: u32 = 1 << 11;
        //! Pin 12 set
        pub const BS12: u32 = 1 << 12;
        //! Pin 13 set
        pub const BS13: u32 = 1 << 13;
        //! Pin 14 set
        pub const BS14: u32 = 1 << 14;
        //! Pin 15 set
        pub const BS15: u32 = 1 << 15;
        //! Pin 0 reset
        pub const BR0: u32 = 1 << 16;
        //! Pin 1 reset
        pub const BR1: u32 = 1 << 17;
        //! Pin 2 reset
        pub const BR2: u32 = 1 << 18;
        //! Pin 3 reset
        pub const BR3: u32 = 1 << 19;
        //! Pin 4 reset
        pub const BR4: u32 = 1 << 20;
        //! Pin 5 reset
        pub const BR5: u32 = 1 << 21;
        //! Pin 6 reset
        pub const BR6: u32 = 1 << 22;
        //! Pin 7 reset
        pub const BR7: u32 = 1 << 23;
        //! Pin 8 reset
        pub const BR8: u32 = 1 << 24;
        //! Pin 9 reset
        pub const BR9: u32 = 1 << 25;
        //! Pin 10 reset
        pub const BR10: u32 = 1 << 26;
        //! Pin 11 reset
        pub const BR11: u32 = 1 << 27;
        //! Pin 12 reset
        pub const BR12: u32 = 1 << 28;
        //! Pin 13 reset
        pub const BR13: u32 = 1 << 29;
        //! Pin 14 reset
        pub const BR14: u32 = 1 << 30;
        //! Pin 15 reset
        pub const BR15: u32 = 1 << 31;
    }
}

//! PWR Register Definitions / PWR 寄存器定义
//!
//! Reference: RM0456 Chapter 10
pub mod pwr {
    //! PWR base (non-secure)
    pub const PWR_BASE: usize = 0x4002_0000;

    //! PWR base (secure)
    pub const PWR_BASE_SEC: usize = 0x5002_0000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Power Control Register 1
        //! Reference: RM0456 Section 10.8.1
        pub const CR1: usize = 0x00;

        //! Power Control Register 2
        pub const CR2: usize = 0x04;

        //! Power Control Register 3
        pub const CR3: usize = 0x08;

        //! Power Voltage Scaling Register
        //! Reference: RM0456 Section 10.8.4
        pub const VOSR: usize = 0x0C;

        //! Power Supply Voltage Monitoring Control Register
        pub const SVMCR: usize = 0x10;

        //! Power Wakeup Control Register 1
        pub const WUCR1: usize = 0x14;

        //! Power Wakeup Control Register 2
        pub const WUCR2: usize = 0x18;

        //! Power Wakeup Control Register 3
        pub const WUCR3: usize = 0x1C;

        //! Power Backup Domain Control Register 1
        pub const BDCR1: usize = 0x20;

        //! Power Backup Domain Control Register 2
        pub const BDCR2: usize = 0x24;

        //! Power Disable Backup Domain Register
        pub const DBPR: usize = 0x28;

        //! Power USB Type-C and Power Delivery Register
        pub const UCPDR: usize = 0x2C;

        //! Power Security Configuration Register
        pub const SECCFGR: usize = 0x30;

        //! Power Privilege Configuration Register
        pub const PRIVCFGR: usize = 0x34;

        //! Power Status Register
        pub const SR: usize = 0x38;

        //! Power Supply Voltage Monitoring Status Register
        pub const SVMSR: usize = 0x3C;

        //! Power Backup Domain Status Register
        pub const BDSR: usize = 0x40;

        //! Power Wakeup Status Register
        pub const WUSR: usize = 0x44;

        //! Power Wakeup Status Clear Register
        pub const WUSCR: usize = 0x48;

        //! Power Application Pull-up/Pull-down Configuration Register
        pub const APCR: usize = 0x4C;

        //! Power Port A Pull Control Register
        pub const PUCR_A: usize = 0x50;
        pub const PDCR_A: usize = 0x54;
        pub const PUCR_B: usize = 0x58;
        pub const PDCR_B: usize = 0x5C;
        pub const PUCR_C: usize = 0x60;
        pub const PDCR_C: usize = 0x64;
        pub const PUCR_D: usize = 0x68;
        pub const PDCR_D: usize = 0x6C;
        pub const PUCR_E: usize = 0x70;
        pub const PDCR_E: usize = 0x74;
        pub const PUCR_F: usize = 0x78;
        pub const PDCR_F: usize = 0x7C;
        pub const PUCR_G: usize = 0x80;
        pub const PDCR_G: usize = 0x84;
        pub const PUCR_H: usize = 0x88;
        pub const PDCR_H: usize = 0x8C;
        pub const PUCR_I: usize = 0x90;
        pub const PDCR_I: usize = 0x94;

        //! Power Control Register 4
        pub const CR4: usize = 0xA8;

        //! Power Control Register 5
        pub const CR5: usize = 0xAC;
    }

    //! CR1 Register Bit Descriptions / CR1 寄存器位描述
    pub mod cr1_bits {
        //! Low power mode selection
        pub const LPMS: u32 = 0b111 << 0;
        //! VBAT enable
        pub const VBE: u32 = 1 << 8;
        //! VBAT reset
        pub const VBR: u32 = 1 << 9;
        //! Disable backup domain write protection
        pub const DBP: u32 = 1 << 8;
        //! Low-power mode forced in Flash power-down
        pub const LPmf: u32 = 1 << 11;
        //! Apply pull-up/pull-down setting for all ports
        pub const APC: u32 = 1 << 15;
        //! Run mode in VOS1 selection
        pub const RUN_SS: u32 = 1 << 16;
        //! Run mode in VOS1 range selection
        pub const RUN_R1: u32 = 1 << 17;
        //! VOS1 range ready
        pub const R1RSEL: u32 = 1 << 18;
    }

    //! CR3 Register Bit Descriptions / CR3 寄存器位描述
    pub mod cr3_bits {
        //! External interrupt wakeup from all lines
        pub const EIWUL: u32 = 1 << 0;
        //! Power supply selection
        pub const REGSEL: u32 = 1 << 1;
        //! Flash fast startup enable
        pub const FSTEN: u32 = 1 << 2;
        //! USB Type-C Power Delivery dead battery disable
        pub const UCPD_DBDIS: u32 = 1 << 6;
        //! USB Type-C Power Delivery standby mode
        pub const UCPD_STDBY: u32 = 1 << 7;
        //! Wakeup pin 1 enable
        pub const EWUP1: u32 = 1 << 8;
        //! Wakeup pin 2 enable
        pub const EWUP2: u32 = 1 << 9;
        //! Wakeup pin 3 enable
        pub const EWUP3: u32 = 1 << 10;
        //! Wakeup pin 4 enable
        pub const EWUP4: u32 = 1 << 11;
        //! Wakeup pin 5 enable
        pub const EWUP5: u32 = 1 << 12;
        //! Wakeup pin 6 enable
        pub const EWUP6: u32 = 1 << 13;
        //! Wakeup pin 7 enable
        pub const EWUP7: u32 = 1 << 14;
        //! Wakeup pin 8 enable
        pub const EWUP8: u32 = 1 << 15;
    }

    //! VOSR Register Bit Descriptions / VOSR 寄存器位描述
    pub mod vosr_bits {
        //! Voltage scaling selection
        pub const VOS: u32 = 0b11 << 16;
        //! Voltage scaling ready flag
        pub const VOSRDY: u32 = 1 << 15;
        //! Voltage scaling busy flag
        pub const VOSY: u32 = 1 << 14;
        //! COMP and VREFOUT enable
        pub const COMP1_VREFOUTEN: u32 = 1 << 31;
    }

    //! SR Register Bit Descriptions / SR 寄存器位描述
    pub mod sr_bits {
        //! PVD output
        pub const PVDO: u32 = 1 << 0;
        //! Voltage scaling flag
        pub const VOSF: u32 = 1 << 1;
        //! Regulator flag
        pub const REGS: u32 = 1 << 4;
        //! Flash ready flag
        pub const FST_RDY: u32 = 1 << 5;
        //! Stop flag
        pub const STOPF: u32 = 1 << 6;
        //! Standby flag
        pub const SBF: u32 = 1 << 7;
        //! Wakeup from interrupt flag
        pub const WUFI: u32 = 1 << 8;
        //! Wakeup flag 1
        pub const WKUPF1: u32 = 1 << 16;
        //! Wakeup flag 2
        pub const WKUPF2: u32 = 1 << 17;
        //! Wakeup flag 3
        pub const WKUPF3: u32 = 1 << 18;
    }
}

//! EXTI Register Definitions / EXTI 寄存器定义
//!
//! Reference: RM0456 Chapter 23
pub mod exti {
    //! EXTI base (non-secure)
    pub const EXTI_BASE: usize = 0x4002_0400;

    //! EXTI base (secure)
    pub const EXTI_BASE_SEC: usize = 0x5002_0400;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Interrupt mask register
        pub const IMR1: usize = 0x00;
        //! Event mask register
        pub const EMR1: usize = 0x04;
        //! Rising trigger selection register 1
        pub const RTSR1: usize = 0x08;
        //! Falling trigger selection register 1
        pub const FTSR1: usize = 0x0C;
        //! Software interrupt event register 1
        pub const SWIER1: usize = 0x10;
        //! Pending register 1
        pub const PR1: usize = 0x14;
        //! Interrupt mask register 2
        pub const IMR2: usize = 0x20;
        //! Event mask register 2
        pub const EMR2: usize = 0x24;
        //! Rising trigger selection register 2
        pub const RTSR2: usize = 0x28;
        //! Falling trigger selection register 2
        pub const FTSR2: usize = 0x2C;
        //! Software interrupt event register 2
        pub const SWIER2: usize = 0x30;
        //! Pending register 2
        pub const PR2: usize = 0x34;
        //! Interrupt mask register 3
        pub const IMR3: usize = 0x40;
        //! Event mask register 3
        pub const EMR3: usize = 0x44;
        //! Rising trigger selection register 3
        pub const RTSR3: usize = 0x48;
        //! Falling trigger selection register 3
        pub const FTSR3: usize = 0x4C;
        //! Software interrupt event register 3
        pub const SWIER3: usize = 0x50;
        //! Pending register 3
        pub const PR3: usize = 0x54;
        //! Secure configuration register
        pub const SECCFGR: usize = 0x80;
        //! Privilege configuration register
        pub const PRIVCFGR: usize = 0x84;
    }
}

//! DMA Register Definitions / DMA 寄存器定义
//!
//! Reference: RM0456 Chapter 17-18
pub mod dma {
    //! GPDMA1 base
    pub const GPDMA1_BASE: usize = 0x4002_1000;

    //! LPDMA1 base
    pub const LPDMA1_BASE: usize = 0x4002_7000;

    //! DMAMUX1 base
    pub const DMAMUX1_BASE: usize = 0x4002_0800;

    //! Register offsets (common for GPDMA/LPDMA) / 寄存器偏移
    pub mod reg {
        //! L-channel base address register
        pub const LBAR: usize = 0x00;
        //! Channel x FCR
        pub const FCR: usize = 0x04;
        //! Channel x status register
        pub const SR: usize = 0x08;
        //! Channel x control register
        pub const CR: usize = 0x0C;
    }
}

//! Flash Register Definitions / Flash 寄存器定义
//!
//! Reference: RM0456 Chapter 7
pub mod flash {
    //! Flash base (non-secure)
    pub const FLASH_BASE: usize = 0x4002_2000;

    //! Flash base (secure)
    pub const FLASH_BASE_SEC: usize = 0x5002_2000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Access control register
        pub const ACR: usize = 0x00;
        //! Power-down key register
        pub const PDKEYR: usize = 0x04;
        //! Program/erase key register
        pub const PEKEYR: usize = 0x08;
        //! Program/erase lock key register
        pub const PRARKEYR: usize = 0x0C;
        //! Option key register
        pub const OPTKEYR: usize = 0x10;
        //! Status register
        pub const SR: usize = 0x14;
        //! Control register
        pub const CR: usize = 0x18;
        //! ECC register
        pub const ECCR: usize = 0x1C;
        //! Boot address register 0
        pub const BOOT0: usize = 0x24;
        //! Boot address register 1
        pub const BOOT1: usize = 0x28;
        //! Option byte register
        pub const OPTR: usize = 0x2C;
        //! PCROP start address register
        pub const PCROP1SR: usize = 0x30;
        //! PCROP end address register
        pub const PCROP1ER: usize = 0x34;
        //! WRP1 area A address register
        pub const WRP1AR: usize = 0x38;
        //! WRP1 area B address register
        pub const WRP1BR: usize = 0x3C;
        //! WRP2 area A address register
        pub const WRP2AR: usize = 0x40;
        //! WRP2 area B address register
        pub const WRP2BR: usize = 0x44;
        //! WRP3 area A address register
        pub const WRP3AR: usize = 0x48;
        //! WRP3 area B address register
        pub const WRP3BR: usize = 0x4C;
        //! OEM key register
        pub const OEMKEYR: usize = 0x60;
        //! Securable memory area register
        pub const SECURE_AREA: usize = 0x70;
        //! Securable memory lock register
        pub const SECLOCK_AREA: usize = 0x74;
        //! Privilege configuration register
        pub const PRIVCFGR: usize = 0x80;
    }

    //! ACR Register Bit Descriptions / ACR 寄存器位描述
    pub mod acr_bits {
        //! Latency (wait states)
        pub const LATENCY: u32 = 0b1111 << 0;
        //! Instruction cache enable
        pub const ICEN: u32 = 1 << 8;
        //! Instruction cache reset
        pub const ICRST: u32 = 1 << 11;
        //! Data cache enable
        pub const DCEN: u32 = 1 << 10;
        //! Data cache reset
        pub const DCRST: u32 = 1 << 12;
        //! Prefetch enable
        pub const PRFTEN: u32 = 1 << 20;
    }
}

//! AES Register Definitions / AES 寄存器定义
//!
//! Reference: RM0456 Chapter 49
pub mod aes {
    //! AES base (non-secure)
    pub const AES_BASE: usize = 0x4202_8C00;

    //! AES base (secure)
    pub const AES_BASE_SEC: usize = 0x5202_8C00;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register
        pub const CR: usize = 0x00;
        //! Status register
        pub const SR: usize = 0x04;
        //! Data input register
        pub const DINR: usize = 0x08;
        //! Data output register
        pub const DOUTR: usize = 0x0C;
        //! Key register 0
        pub const KEYR0: usize = 0x10;
        //! Key register 1
        pub const KEYR1: usize = 0x14;
        //! Key register 2
        pub const KEYR2: usize = 0x18;
        //! Key register 3
        pub const KEYR3: usize = 0x1C;
        //! Initialization vector register 0
        pub const IVR0: usize = 0x20;
        //! Initialization vector register 1
        pub const IVR1: usize = 0x24;
        //! Initialization vector register 2
        pub const IVR2: usize = 0x28;
        //! Initialization vector register 3
        pub const IVR3: usize = 0x2C;
        //! Key register 4
        pub const KEYR4: usize = 0x30;
        //! Key register 5
        pub const KEYR5: usize = 0x34;
        //! Key register 6
        pub const KEYR6: usize = 0x38;
        //! Key register 7
        pub const KEYR7: usize = 0x3C;
        //! Suspend register
        pub const SUSPR: usize = 0x40;
        //! Configuration register
        pub const CFGR: usize = 0x100;
        //! Key extended register 0
        pub const KEYEXT0R: usize = 0x200;
    }
}

//! RNG Register Definitions / RNG 寄存器定义
//!
//! Reference: RM0456 Chapter 48
pub mod rng {
    //! RNG base (non-secure)
    pub const RNG_BASE: usize = 0x4202_9800;

    //! RNG base (secure)
    pub const RNG_BASE_SEC: usize = 0x5202_9800;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register
        pub const CR: usize = 0x00;
        //! Status register
        pub const SR: usize = 0x04;
        //! Data register
        pub const DR: usize = 0x08;
    }

    //! CR Register Bit Descriptions / CR 寄存器位描述
    pub mod cr_bits {
        //! RNG enable
        pub const RNGEN: u32 = 1 << 2;
        //! Interrupt enable
        pub const IE: u32 = 1 << 3;
    }

    //! SR Register Bit Descriptions / SR 寄存器位描述
    pub mod sr_bits {
        //! Data ready
        pub const DRDY: u32 = 1 << 0;
        //! Clock error
        pub const SECS: u32 = 1 << 1;
        //! Seed error
        pub const CEIS: u32 = 1 << 5;
    }
}

//! PKA Register Definitions / PKA 寄存器定义
//!
//! Reference: RM0456 Chapter 53
pub mod pka {
    //! PKA base
    pub const PKA_BASE: usize = 0x4202_A000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! PKA control register
        pub const CR: usize = 0x00;
        //! PKA status register
        pub const SR: usize = 0x04;
        //! PKA clear flag register
        pub const CLRFR: usize = 0x08;
    }
}

//! HASH Register Definitions / HASH 寄存器定义
//!
//! Reference: RM0456 Chapter 51
pub mod hash {
    //! HASH base
    pub const HASH_BASE: usize = 0x4202_9400;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register
        pub const CR: usize = 0x00;
        //! DIN status register
        pub const SR: usize = 0x04;
        //! Data input register
        pub const DIN: usize = 0x08;
        //! Start register
        pub const STR: usize = 0x0C;
        //! Digest register (x8)
        pub const HR0: usize = 0x310;
        pub const HR1: usize = 0x314;
        pub const HR2: usize = 0x318;
        pub const HR3: usize = 0x31C;
        pub const HR4: usize = 0x320;
        pub const HR5: usize = 0x324;
        pub const HR6: usize = 0x328;
        pub const HR7: usize = 0x32C;
        //! IMR
        pub const IMR: usize = 0x20;
        //! RISR
        pub const RISR: usize = 0x24;
        //! MISR
        pub const MISR: usize = 0x28;
    }
}

//! ADC Register Definitions / ADC 寄存器定义
//!
//! Reference: RM0456 Chapter 33-34
pub mod adc {
    //! ADC1 base
    pub const ADC1_BASE: usize = 0x4202_8000;

    //! ADC2 base
    pub const ADC2_BASE: usize = 0x4202_8400;

    //! ADC4 base
    pub const ADC4_BASE: usize = 0x4202_8C00;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! ISR
        pub const ISR: usize = 0x00;
        //! IER
        pub const IER: usize = 0x04;
        //! CR
        pub const CR: usize = 0x08;
        //! CFGR
        pub const CFGR: usize = 0x0C;
        //! CFGR2
        pub const CFGR2: usize = 0x10;
        //! SMPR
        pub const SMPR: usize = 0x14;
        //! TR1
        pub const TR1: usize = 0x20;
        //! TR2
        pub const TR2: usize = 0x24;
        //! TR3
        pub const TR3: usize = 0x28;
        //! CHSELR
        pub const CHSELR: usize = 0x30;
        //! TR
        pub const TR: usize = 0x38;
        //! DR
        pub const DR: usize = 0x40;
    }
}

//! DAC Register Definitions / DAC 寄存器定义
//!
//! Reference: RM0456 Chapter 35
pub mod dac {
    //! DAC1 base
    pub const DAC1_BASE: usize = 0x4002_7400;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register
        pub const CR: usize = 0x00;
        //! Software trigger register
        pub const SWTRGR: usize = 0x04;
        //! Channel 1 data holding register
        pub const DHR12R1: usize = 0x08;
        //! Channel 1 data holding register left-aligned
        pub const DHR12L1: usize = 0x0C;
        //! Channel 1 data holding register 8-bit
        pub const DHR8R1: usize = 0x14;
        //! Channel 1 data output register
        pub const DOR1: usize = 0x20;
        //! Status register
        pub const SR: usize = 0x24;
    }
}

//! RTC Register Definitions / RTC 寄存器定义
//!
//! Reference: RM0456 Chapter 63
pub mod rtc {
    //! RTC base
    pub const RTC_BASE: usize = 0x4200_0000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Time register
        pub const TR: usize = 0x00;
        //! Date register
        pub const DR: usize = 0x04;
        //! Control register
        pub const CR: usize = 0x08;
        //! Initialization and control register
        pub const ISR: usize = 0x0C;
        //! Prescaler register
        pub const PRER: usize = 0x10;
        //! Wakeup timer register
        pub const WUTR: usize = 0x14;
        //! Calibration register
        pub const CALIBR: usize = 0x18;
        //! Alarm A register
        pub const ALRMAR: usize = 0x20;
        //! Alarm B register
        pub const ALRMBR: usize = 0x24;
        //! Write protection register
        pub const WPR: usize = 0x24;
        //! Sub second register
        pub const SSR: usize = 0x28;
        //! Shift control register
        pub const SHIFTR: usize = 0x2C;
        //! Time register 2
        pub const TR2: usize = 0x40;
        //! Date register 2
        pub const DR2: usize = 0x44;
    }
}

//! TAMP Register Definitions / TAMP 寄存器定义
//!
//! Reference: RM0456 Chapter 63
pub mod tamp {
    //! TAMP base
    pub const TAMP_BASE: usize = 0x4200_0400;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register 1
        pub const CR1: usize = 0x00;
        //! Control register 2
        pub const CR2: usize = 0x04;
        //! Control register 3
        pub const CR3: usize = 0x08;
        //! Filter control register
        pub const FLTCR: usize = 0x0C;
        //! Interrupt enable register
        pub const IER: usize = 0x10;
        //! Status register
        pub const SR: usize = 0x14;
        //! Clear flag register
        pub const CLR: usize = 0x18;
        //! Backup register (x32)
        pub const BKP0R: usize = 0x100;
    }
}

//! I2C Register Definitions / I2C 寄存器定义
//!
//! Reference: RM0456 Chapter 65
pub mod i2c {
    //! I2C1 base
    pub const I2C1_BASE: usize = 0x4000_5400;

    //! I2C2 base
    pub const I2C2_BASE: usize = 0x4000_5800;

    //! I2C3 base
    pub const I2C3_BASE: usize = 0x4000_5C00;

    //! I2C4 base
    pub const I2C4_BASE: usize = 0x4000_8400;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register 1
        pub const CR1: usize = 0x00;
        //! Control register 2
        pub const CR2: usize = 0x04;
        //! Own address register 1
        pub const OAR1: usize = 0x08;
        //! Own address register 2
        pub const OAR2: usize = 0x0C;
        //! Timing register
        pub const TIMINGR: usize = 0x10;
        //! Timeout register
        pub const TIMEOUTR: usize = 0x14;
        //! Interrupt and status register
        pub const ISR: usize = 0x18;
        //! Interrupt clear register
        pub const ICR: usize = 0x1C;
        //! PEC register
        pub const PECR: usize = 0x20;
        //! Receive data register
        pub const RXDR: usize = 0x24;
        //! Transmit data register
        pub const TXDR: usize = 0x28;
    }
}

//! SPI Register Definitions / SPI 寄存器定义
//!
//! Reference: RM0456 Chapter 68
pub mod spi {
    //! SPI1 base
    pub const SPI1_BASE: usize = 0x4001_3000;

    //! SPI2 base
    pub const SPI2_BASE: usize = 0x4000_3800;

    //! SPI3 base
    pub const SPI3_BASE: usize = 0x4000_3C00;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register 1
        pub const CR1: usize = 0x00;
        //! Control register 2
        pub const CR2: usize = 0x04;
        //! Status register
        pub const SR: usize = 0x08;
        //! Data register
        pub const DR: usize = 0x0C;
        //! CRC polynomial register
        pub const CRCPOLY: usize = 0x10;
        //! CRC transfer register
        pub const RXCRC: usize = 0x14;
        //! CRC transfer register
        pub const TXCRC: usize = 0x18;
        //! I2S configuration register
        pub const I2SCFGR: usize = 0x1C;
    }
}

//! USART/UART Register Definitions / USART/UART 寄存器定义
//!
//! Reference: RM0456 Chapter 66-67
pub mod usart {
    //! USART1 base
    pub const USART1_BASE: usize = 0x4001_3800;

    //! USART2 base
    pub const USART2_BASE: usize = 0x4000_4400;

    //! LPUART1 base
    pub const LPUART1_BASE: usize = 0x4000_8000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register 1
        pub const CR1: usize = 0x00;
        //! Control register 2
        pub const CR2: usize = 0x04;
        //! Control register 3
        pub const CR3: usize = 0x08;
        //! Baud rate register
        pub const BRR: usize = 0x0C;
        //! Guard time and prescaler register
        pub const GTPR: usize = 0x10;
        //! Receiver timeout register
        pub const RTOR: usize = 0x14;
        //! Request register
        pub const RQR: usize = 0x18;
        //! Interrupt and status register
        pub const ISR: usize = 0x1C;
        //! Interrupt flag clear register
        pub const ICR: usize = 0x20;
        //! Receive data register
        pub const RDR: usize = 0x24;
        //! Transmit data register
        pub const TDR: usize = 0x28;
        //! Prescaler register
        pub const PRESC: usize = 0x2C;
    }
}

//! TIM Register Definitions / TIM 寄存器定义
//!
//! Reference: RM0456 Chapter 54-58
pub mod tim {
    //! TIM1 base
    pub const TIM1_BASE: usize = 0x4001_0000;

    //! TIM2 base
    pub const TIM2_BASE: usize = 0x4000_0000;

    //! TIM3 base
    pub const TIM3_BASE: usize = 0x4000_0400;

    //! TIM4 base
    pub const TIM4_BASE: usize = 0x4000_0800;

    //! TIM5 base
    pub const TIM5_BASE: usize = 0x4000_0C00;

    //! TIM6 base
    pub const TIM6_BASE: usize = 0x4000_1000;

    //! TIM7 base
    pub const TIM7_BASE: usize = 0x4000_1400;

    //! TIM8 base
    pub const TIM8_BASE: usize = 0x4001_3400;

    //! TIM15 base
    pub const TIM15_BASE: usize = 0x4001_4000;

    //! TIM16 base
    pub const TIM16_BASE: usize = 0x4001_4400;

    //! TIM17 base
    pub const TIM17_BASE: usize = 0x4001_4800;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register 1
        pub const CR1: usize = 0x00;
        //! Control register 2
        pub const CR2: usize = 0x04;
        //! Slave mode control register
        pub const SMCR: usize = 0x08;
        //! DMA/interrupt enable register
        pub const DIER: usize = 0x0C;
        //! Status register
        pub const SR: usize = 0x10;
        //! Event generation register
        pub const EGR: usize = 0x14;
        //! Capture/compare mode register 1
        pub const CCMR1: usize = 0x18;
        //! Capture/compare mode register 2
        pub const CCMR2: usize = 0x1C;
        //! Capture/compare enable register
        pub const CCER: usize = 0x20;
        //! Counter
        pub const CNT: usize = 0x24;
        //! Prescaler
        pub const PSC: usize = 0x28;
        //! Auto-reload register
        pub const ARR: usize = 0x2C;
        //! Repetition counter register
        pub const RCR: usize = 0x30;
        //! Capture/compare register 1
        pub const CCR1: usize = 0x34;
        //! Capture/compare register 2
        pub const CCR2: usize = 0x38;
        //! Capture/compare register 3
        pub const CCR3: usize = 0x3C;
        //! Capture/compare register 4
        pub const CCR4: usize = 0x40;
        //! Break and dead-time register
        pub const BDTR: usize = 0x44;
        //! DMA control register
        pub const DCR: usize = 0x48;
        //! DMA address for full transfer
        pub const DMAR: usize = 0x4C;
    }
}

//! LPTIM Register Definitions / LPTIM 寄存器定义
//!
//! Reference: RM0456 Chapter 58
pub mod lptim {
    //! LPTIM1 base
    pub const LPTIM1_BASE: usize = 0x4000_8000;

    //! LPTIM2 base
    pub const LPTIM2_BASE: usize = 0x4000_8400;

    //! LPTIM3 base
    pub const LPTIM3_BASE: usize = 0x4000_8800;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register
        pub const CR: usize = 0x00;
        //! Interrupt enable register
        pub const IER: usize = 0x04;
        //! Status register
        pub const SR: usize = 0x08;
        //! Clear flag register
        pub const CLR: usize = 0x0C;
        //! Configuration register
        pub const CFGR: usize = 0x10;
        //! Compare register
        pub const CMP: usize = 0x14;
        //! Autoreload register
        pub const ARR: usize = 0x18;
        //! Counter register
        pub const CNT: usize = 0x1C;
    }
}

//! USB Register Definitions / USB 寄存器定义
//!
//! Reference: RM0456 Chapter 72-73
pub mod usb {
    //! USB OTG FS base
    pub const USB_OTG_FS_BASE: usize = 0x4204_0000;

    //! USB OTG HS base
    pub const USB_OTG_HS_BASE: usize = 0x4204_4000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Host configuration register
        pub const HCFG: usize = 0x00;
        //! Host frame interval register
        pub const HFIR: usize = 0x04;
        //! Host frame number/frame time status register
        pub const HFNUM: usize = 0x08;
        //! Host channel x characteristics register
        pub const HCCHAR0: usize = 0x10;
        //! Host channel x interrupt register
        pub const HCINT0: usize = 0x18;
        //! Host channel x transfer size register
        pub const HCTSIZ0: usize = 0x20;
        //! Device configuration register
        pub const DCFG: usize = 0x00;
        //! Device control register
        pub const DCTL: usize = 0x04;
        //! Device status register
        pub const DSTS: usize = 0x08;
        //! Device IN endpoint common mask register
        pub const DIEPMSK: usize = 0x10;
        //! Device OUT endpoint common mask register
        pub const DOEPMSK: usize = 0x14;
        //! Device all endpoints interrupt register
        pub const DAINT: usize = 0x18;
        //! OTG control and status register
        pub const GOTGCTL: usize = 0x00;
        //! OTG interrupt register
        pub const GOTGINT: usize = 0x04;
        //! Core AHB configuration register
        pub const GAHBCFG: usize = 0x08;
        //! Core USB configuration register
        pub const GUSBCFG: usize = 0x0C;
        //! Core reset register
        pub const GRSTCTL: usize = 0x10;
        //! Core interrupt register
        pub const GINTSTS: usize = 0x14;
        //! Core interrupt mask register
        pub const GINTMSK: usize = 0x18;
    }
}

//! FDCAN Register Definitions / FDCAN 寄存器定义
//!
//! Reference: RM0456 Chapter 70
pub mod fdcan {
    //! FDCAN1 base
    pub const FDCAN1_BASE: usize = 0x4000_A400;

    //! FDCAN2 base
    pub const FDCAN2_BASE: usize = 0x4000_A800;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Core release register
        pub const CREL: usize = 0x00;
        //! Endian register
        pub const ENDN: usize = 0x04;
        //! Data bit timing and prescaler register
        pub const DBTP: usize = 0x0C;
        //! Test register
        pub const TEST: usize = 0x10;
        //! RAM watcher dog
        pub const RWD: usize = 0x14;
        //! CC control register
        pub const CCCR: usize = 0x18;
        //! Nominal bit timing and prescaler register
        pub const NBTP: usize = 0x1C;
        //! Timestamp counter configuration register
        pub const TSCC: usize = 0x20;
        //! Timestamp counter value register
        pub const TSCV: usize = 0x24;
        //! Error counter register
        pub const ECR: usize = 0x28;
        //! Protocol status register
        pub const PSR: usize = 0x2C;
        //! Transmitter delay compensation register
        pub const TDCR: usize = 0x34;
        //! Interrupt register
        pub const IR: usize = 0x40;
        //! Interrupt enable register
        pub const IER: usize = 0x44;
        //! Interrupt line select register
        pub const ILS: usize = 0x48;
        //! Interrupt line enable register
        pub const ILE: usize = 0x4C;
    }
}

//! SYSCFG Register Definitions / SYSCFG 寄存器定义
//!
//! Reference: RM0456 Chapter 15
pub mod syscfg {
    //! SYSCFG base
    pub const SYSCFG_BASE: usize = 0x4002_0000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! SYSCFG configuration register 1
        pub const CFGR1: usize = 0x00;
        //! SYSCFG external interrupt configuration register 1
        pub const EXTICR1: usize = 0x08;
        //! SYSCFG external interrupt configuration register 2
        pub const EXTICR2: usize = 0x0C;
        //! SYSCFG external interrupt configuration register 3
        pub const EXTICR3: usize = 0x10;
        //! SYSCFG external interrupt configuration register 4
        pub const EXTICR4: usize = 0x14;
        //! SYSCFG configuration register 2
        pub const CFGR2: usize = 0x20;
        //! SYSCFG configuration register 3
        pub const CFGR3: usize = 0x24;
        //! SYSCFG configuration register 4
        pub const CFGR4: usize = 0x28;
    }
}

//! GTZC Register Definitions / GTZC 寄存器定义
//!
//! Reference: RM0456 Chapter 5
pub mod gtzc {
    //! GTZC TZIC base
    pub const GTZC_TZIC_BASE: usize = 0x4000_0000;

    //! GTZC MPCWM1 base
    pub const GTZC_MPCWM1_BASE: usize = 0x4003_0000;

    //! GTZC TDC1 base
    pub const GTZC_TDC1_BASE: usize = 0x4003_1000;

    //! GTZC ETZPC base
    pub const GTZC_ETZPC_BASE: usize = 0x4003_2000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! TZIC control register
        pub const CR: usize = 0x00;
        //! TZIC configuration register
        pub const CFGR: usize = 0x04;
        //! TZIC interrupt enable register
        pub const IER: usize = 0x08;
        //! TZIC status register
        pub const SR: usize = 0x0C;
        //! TZIC flag clear register
        pub const FCR: usize = 0x10;
    }
}

//! ICACHE Register Definitions / ICACHE 寄存器定义
//!
//! Reference: RM0456 Chapter 8
pub mod icache {
    //! ICACHE base
    pub const ICACHE_BASE: usize = 0x4002_3400;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! Control register
        pub const CR: usize = 0x00;
        //! Status register
        pub const SR: usize = 0x04;
        //! Interrupt enable register
        pub const IER: usize = 0x08;
        //! Flag clear register
        pub const FCR: usize = 0x0C;
        //! Monitor control register
        pub const MCR: usize = 0x20;
        //! Monitor hit count register
        pub const MHCR: usize = 0x28;
        //! Monitor miss count register
        pub const MMCR: usize = 0x2C;
    }
}

//! CORDIC Register Definitions / CORDIC 寄存器定义
//!
//! Reference: RM0456 Chapter 25
pub mod cordic {
    //! CORDIC base
    pub const CORDIC_BASE: usize = 0x4002_0C00;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! CORDIC control and status register
        pub const CSR: usize = 0x00;
        //! CORDIC WDATA register
        pub const WDATA: usize = 0x04;
        //! CORDIC RDATA register
        pub const RDATA: usize = 0x08;
    }
}

//! FMAC Register Definitions / FMAC 寄存器定义
//!
//! Reference: RM0456 Chapter 26
pub mod fmac {
    //! FMAC base
    pub const FMAC_BASE: usize = 0x4002_1000;

    //! Register offsets / 寄存器偏移
    pub mod reg {
        //! FMAC control register
        pub const CR: usize = 0x00;
        //! FMAC status register
        pub const SR: usize = 0x04;
        //! FMAC write data register
        pub const WDATA: usize = 0x08;
        //! FMAC read data register
        pub const RDATA: usize = 0x0C;
        //! FMAC buffer configuration register
        pub const BUFx: usize = 0x20;
    }
}

//! Memory and bus architecture / 内存和总线架构
//!
//! Reference: RM0456 Chapter 2
pub mod memory {
    //! Flash memory base address / 闪存基地址
    pub const FLASH_BASE: usize = 0x0800_0000;

    //! Flash size (varies by device) / 闪存大小(因设备而异)
    pub const FLASH_SIZE: usize = 4 * 1024 * 1024; // 4 MB max

    //! SRAM1 base address / SRAM1基地址
    pub const SRAM1_BASE: usize = 0x2000_0000;

    //! SRAM1 size / SRAM1大小
    pub const SRAM1_SIZE: usize = 768 * 1024; // 768KB

    //! SRAM2 base address / SRAM2基地址
    pub const SRAM2_BASE: usize = 0x200C_0000;

    //! SRAM2 size / SRAM2大小
    pub const SRAM2_SIZE: usize = 64 * 1024; // 64KB

    //! SRAM3 base address / SRAM3基地址
    pub const SRAM3_BASE: usize = 0x200D_0000;

    //! SRAM3 size / SRAM3大小 (on some variants)
    pub const SRAM3_SIZE: usize = 320 * 1024; // 320KB

    //! Backup SRAM base address / 备份SRAM基地址
    pub const BKPSRAM_BASE: usize = 0x4002_4000;

    //! Backup SRAM size / 备份SRAM大小
    pub const BKPSRAM_SIZE: usize = 2 * 1024; // 2KB
}
