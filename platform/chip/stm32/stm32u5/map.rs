#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

//! STM32U5 Series Memory Map
//!
//! This module provides complete peripheral address mapping for the STM32U5 series
//! microcontrollers based on ARM Cortex-M33 core with TrustZone.
//!
//! # Address Space Overview
//! - 0x0000_0000 - 0x1FFF_FFFF: Code/Flash region (512MB)
//! - 0x2000_0000 - 0x3FFF_FFFF: SRAM region (512MB)
//! - 0x4000_0000 - 0x4FFF_FFFF: Peripherals - Non-secure alias (256MB)
//! - 0x5000_0000 - 0x5FFF_FFFF: Peripherals - Secure alias (256MB)
//! - 0x6000_0000 - 0x9FFF_FFFF: External memory (1GB)
//!
//! # Device Variants
//! The STM32U5 series includes multiple variants with different peripherals:
//! - STM32U5xx: Base series (U5x5, U5x6)
//! - STM32U5Fx: With LCD-TFT/DCMI (U5F7, U5F9)
//! - STM32U5Gx: With OctoSPI/HSPI (U5G7, U5G9)
//! - STM32U5Jx: Full-featured (U5J9)
//!
//! # Reference
//! - RM0456 Chapter 2: Memory and bus architecture
//! - RM0456 Chapter 2, Table 1-4: Peripheral register boundary addresses

pub mod chip {
    pub const FAMILY: &str = "STM32U5";
    pub const VENDOR: &str = "STMicroelectronics";
    pub const CORE: &str = "Cortex-M33";
    pub const CORE_VERSION: &str = "r0p4";
    pub const DSP: bool = true;
    pub const FPU: bool = true;
    pub const FPU_DP: bool = true;
}

pub mod core {
    pub const PPB_BASE: usize = 0xE000_0000;

    pub const FPB_BASE: usize = PPB_BASE + 0x02000;
    pub const DWT_BASE: usize = PPB_BASE + 0x01000;
    pub const SCB_BASE: usize = PPB_BASE + 0x0ED00;
    pub const SCB_ACTLR: usize = PPB_BASE + 0x0ED08;
    pub const SYSTICK_BASE: usize = PPB_BASE + 0x0E010;
    pub const NVIC_BASE: usize = PPB_BASE + 0x0E100;
    pub const MPU_BASE: usize = PPB_BASE + 0x0ED90;
    pub const FPU_BASE: usize = PPB_BASE + 0x0EF30;
    pub const FPU_CPACR: usize = PPB_BASE + 0x0ED88;
    pub const SAU_BASE: usize = PPB_BASE + 0x0EDD0;
    pub const DCB_BASE: usize = PPB_BASE + 0x0EDF0;
    pub const ITM_BASE: usize = PPB_BASE + 0x0FB00;
    pub const ITM_TER: usize = PPB_BASE + 0x0E000;
    pub const ITM_TPR: usize = PPB_BASE + 0x0E040;
    pub const ITM_TCR: usize = PPB_BASE + 0x0E080;
    pub const TPIU_BASE: usize = PPB_BASE + 0x0E0000;
    pub const TPIU_SPSR: usize = PPB_BASE + 0x0E004;
    pub const TPIU_SPPR: usize = PPB_BASE + 0x0E0F0;
    pub const TPIU_TYPE: usize = PPB_BASE + 0x0EFC4;
    pub const ETM_BASE: usize = PPB_BASE + 0x0E1000;
    pub const ETM_CONFIG: usize = PPB_BASE + 0x0E004;
    pub const ETM_TRIGGER: usize = PPB_BASE + 0x0E008;
    pub const ETM_CR: usize = PPB_BASE + 0x0E010;
    pub const ETM_TEEVR: usize = PPB_BASE + 0x0E020;
    pub const ETM_TECR1: usize = PPB_BASE + 0x0E024;
    pub const ETM_TRACEIDCTRL: usize = PPB_BASE + 0x0E040;
    pub const ETM_TSCTL: usize = PPB_BASE + 0x0E180;
}

pub mod ppb {
    pub const DBGMCU_BASE: usize = 0xE004_2000;
    pub const DES_BASE: usize = 0xE001_0000;
    pub const UID_BASE: usize = 0xE001_8000;
    pub const FLASHSIZE_BASE: usize = 0xE001_8010;
    pub const PACKAGE_BASE: usize = 0xE001_8014;
    pub const REV_ID_BASE: usize = 0xE001_8000;
    pub const DEV_ID_BASE: usize = 0xE001_8004;
}

pub mod memory {
    pub const FLASH_BASE: usize = 0x0800_0000;
    pub const FLASH_BANK1_BASE: usize = 0x0800_0000;
    pub const FLASH_BANK2_BASE: usize = 0x0810_0000;
    pub const FLASH_OPTION_BASE: usize = 0x0FFF_F000;
    pub const FLASH_OTP_BASE: usize = 0x0FFF_F800;

    pub const SRAM1_BASE: usize = 0x2000_0000;
    pub const SRAM2_BASE: usize = 0x200C_0000;
    pub const SRAM3_BASE: usize = 0x200D_0000;
    pub const SRAM4_BASE: usize = 0x2010_0000;

    pub const BKPSRAM_BASE: usize = 0x4002_4000;
    pub const PKARAM_BASE: usize = 0x2004_0000;
    pub const PKARAM_SIZE: usize = 32 * 1024;

    pub mod size {
        pub const FLASH_256K: usize = 256 * 1024;
        pub const FLASH_512K: usize = 512 * 1024;
        pub const FLASH_1M: usize = 1024 * 1024;
        pub const FLASH_2M: usize = 2 * 1024 * 1024;
        pub const FLASH_4M: usize = 4 * 1024 * 1024;

        pub const SRAM_192K: usize = 192 * 1024;
        pub const SRAM_256K: usize = 256 * 1024;
        pub const SRAM_320K: usize = 320 * 1024;
        pub const SRAM_384K: usize = 384 * 1024;
        pub const SRAM_640K: usize = 640 * 1024;
        pub const SRAM_768K: usize = 768 * 1024;
        pub const SRAM_1M: usize = 1024 * 1024;

        pub const BKPSRAM_2K: usize = 2 * 1024;
    }
}

pub mod ns {
    pub const PERIPH_BASE: usize = 0x4000_0000;

    pub const APB1_PERIPH_BASE: usize = PERIPH_BASE + 0x0000;
    pub const APB2_PERIPH_BASE: usize = PERIPH_BASE + 0x1000;
    pub const APB3_PERIPH_BASE: usize = PERIPH_BASE + 0x0800;
    pub const AHB1_PERIPH_BASE: usize = PERIPH_BASE + 0x2000;
    pub const AHB2_PERIPH_BASE: usize = PERIPH_BASE + 0x4000;
    pub const AHB3_PERIPH_BASE: usize = PERIPH_BASE + 0x6000;
    pub const AHB4_PERIPH_BASE: usize = PERIPH_BASE + 0x8000;
    pub const AHB5_PERIPH_BASE: usize = PERIPH_BASE + 0xA000;

    pub mod apb1 {
        use super::APB1_PERIPH_BASE;

        pub const TIM2_BASE: usize = APB1_PERIPH_BASE + 0x0000;
        pub const TIM3_BASE: usize = APB1_PERIPH_BASE + 0x0400;
        pub const TIM4_BASE: usize = APB1_PERIPH_BASE + 0x0800;
        pub const TIM5_BASE: usize = APB1_PERIPH_BASE + 0x0C00;
        pub const TIM6_BASE: usize = APB1_PERIPH_BASE + 0x1000;
        pub const TIM7_BASE: usize = APB1_PERIPH_BASE + 0x1400;
        pub const RTC_BASE: usize = APB1_PERIPH_BASE + 0x1800;
        pub const TAMP_BASE: usize = APB1_PERIPH_BASE + 0x1C00;
        pub const WWDG_BASE: usize = APB1_PERIPH_BASE + 0x2C00;
        pub const IWDG_BASE: usize = APB1_PERIPH_BASE + 0x3000;
        pub const SPI2_BASE: usize = APB1_PERIPH_BASE + 0x3800;
        pub const SPI3_BASE: usize = APB1_PERIPH_BASE + 0x3C00;
        pub const USART2_BASE: usize = APB1_PERIPH_BASE + 0x4400;
        pub const USART3_BASE: usize = APB1_PERIPH_BASE + 0x4800;
        pub const UART4_BASE: usize = APB1_PERIPH_BASE + 0x4C00;
        pub const UART5_BASE: usize = APB1_PERIPH_BASE + 0x5000;
        pub const I2C1_BASE: usize = APB1_PERIPH_BASE + 0x5400;
        pub const I2C2_BASE: usize = APB1_PERIPH_BASE + 0x5800;
        pub const I2C3_BASE: usize = APB1_PERIPH_BASE + 0x5C00;
        pub const CRS_BASE: usize = APB1_PERIPH_BASE + 0x6000;
        pub const CAN1_BASE: usize = APB1_PERIPH_BASE + 0x6400;
        pub const FDCAN1_BASE: usize = CAN1_BASE;
        pub const CAN2_BASE: usize = APB1_PERIPH_BASE + 0x6800;
        pub const FDCAN2_BASE: usize = CAN2_BASE;
        pub const USB_BASE: usize = APB1_PERIPH_BASE + 0x6C00;
        pub const PWR_BASE: usize = APB1_PERIPH_BASE + 0x7000;
        pub const DAC1_BASE: usize = APB1_PERIPH_BASE + 0x7400;
        pub const OPAMP1_BASE: usize = APB1_PERIPH_BASE + 0x7800;
        pub const OPAMP2_BASE: usize = APB1_PERIPH_BASE + 0x7C00;
        pub const LPTIM1_BASE: usize = APB1_PERIPH_BASE + 0x8000;
        pub const LPTIM2_BASE: usize = APB1_PERIPH_BASE + 0x8400;
        pub const LPTIM3_BASE: usize = APB1_PERIPH_BASE + 0x8800;
        pub const I2C4_BASE: usize = APB1_PERIPH_BASE + 0x8C00;
        pub const LPUART1_BASE: usize = APB1_PERIPH_BASE + 0x9000;
        pub const UCPD1_BASE: usize = APB1_PERIPH_BASE + 0xA000;
        pub const UCPD2_BASE: usize = APB1_PERIPH_BASE + 0xA400;
    }

    pub mod apb2 {
        use super::APB2_PERIPH_BASE;

        pub const TIM1_BASE: usize = APB2_PERIPH_BASE + 0x0000;
        pub const TIM8_BASE: usize = APB2_PERIPH_BASE + 0x0400;
        pub const SPI1_BASE: usize = APB2_PERIPH_BASE + 0x3000;
        pub const USART1_BASE: usize = APB2_PERIPH_BASE + 0x3800;
        pub const TIM15_BASE: usize = APB2_PERIPH_BASE + 0x4000;
        pub const TIM16_BASE: usize = APB2_PERIPH_BASE + 0x4400;
        pub const TIM17_BASE: usize = APB2_PERIPH_BASE + 0x4800;
        pub const SAI1_BASE: usize = APB2_PERIPH_BASE + 0x5400;
        pub const SAI2_BASE: usize = APB2_PERIPH_BASE + 0x5800;
        pub const SAI3_BASE: usize = APB2_PERIPH_BASE + 0x5C00;
    }

    pub mod apb3 {
        use super::APB3_PERIPH_BASE;

        pub const LPTIM4_BASE: usize = APB3_PERIPH_BASE + 0x0000;
        pub const LPTIM5_BASE: usize = APB3_PERIPH_BASE + 0x0400;
        pub const LPTIM6_BASE: usize = APB3_PERIPH_BASE + 0x0800;
        pub const VDDIO2_BASE: usize = APB3_PERIPH_BASE + 0x1000;
        pub const SYSCFG_BASE: usize = APB3_PERIPH_BASE + 0x2000;
        pub const EXTI_BASE: usize = APB3_PERIPH_BASE + 0x2400;
        pub const EXTI_D1_BASE: usize = EXTI_BASE;
        pub const EXTI_D2_BASE: usize = APB3_PERIPH_BASE + 0x2800;
        pub const EXTI_D3_BASE: usize = APB3_PERIPH_BASE + 0x2C00;
        pub const RCC_BASE: usize = APB3_PERIPH_BASE + 0x3000;
        pub const RCC_CRS_BASE: usize = APB3_PERIPH_BASE + 0x3800;
    }

    pub mod ahb1 {
        use super::AHB1_PERIPH_BASE;

        pub const GPDMA1_BASE: usize = AHB1_PERIPH_BASE + 0x0000;
        pub const DMAMUX1_BASE: usize = AHB1_PERIPH_BASE + 0x0400;
        pub const CORDIC_BASE: usize = AHB1_PERIPH_BASE + 0x0C00;
        pub const FMAC_BASE: usize = AHB1_PERIPH_BASE + 0x1000;
        pub const MDF1_BASE: usize = AHB1_PERIPH_BASE + 0x1400;
        pub const ADF1_BASE: usize = AHB1_PERIPH_BASE + 0x1800;
        pub const FLASH_BASE: usize = AHB1_PERIPH_BASE + 0x2000;
        pub const ICACHE_BASE: usize = AHB1_PERIPH_BASE + 0x2400;
        pub const DCACHE_BASE: usize = AHB1_PERIPH_BASE + 0x2800;
        pub const CRC_BASE: usize = AHB1_PERIPH_BASE + 0x3000;
        pub const TSC_BASE: usize = AHB1_PERIPH_BASE + 0x4000;
        pub const RAMCFG_BASE: usize = AHB1_PERIPH_BASE + 0x4400;
        pub const BKPSRAM_BASE: usize = AHB1_PERIPH_BASE + 0x4800;
        pub const DPRAM1_BASE: usize = AHB1_PERIPH_BASE + 0x5000;
        pub const DPRAM2_BASE: usize = AHB1_PERIPH_BASE + 0x5400;
        pub const DMA2D_BASE: usize = AHB1_PERIPH_BASE + 0xB000;
    }

    pub mod ahb2 {
        use super::AHB2_PERIPH_BASE;

        pub const GPIOA_BASE: usize = AHB2_PERIPH_BASE + 0x0000;
        pub const GPIOB_BASE: usize = AHB2_PERIPH_BASE + 0x0400;
        pub const GPIOC_BASE: usize = AHB2_PERIPH_BASE + 0x0800;
        pub const GPIOD_BASE: usize = AHB2_PERIPH_BASE + 0x0C00;
        pub const GPIOE_BASE: usize = AHB2_PERIPH_BASE + 0x1000;
        pub const GPIOF_BASE: usize = AHB2_PERIPH_BASE + 0x1400;
        pub const GPIOG_BASE: usize = AHB2_PERIPH_BASE + 0x1800;
        pub const GPIOH_BASE: usize = AHB2_PERIPH_BASE + 0x1C00;
        pub const GPIOI_BASE: usize = AHB2_PERIPH_BASE + 0x2000;
    }

    pub mod ahb2enr2 {
        use super::AHB2_PERIPH_BASE;

        pub const ADC1_BASE: usize = AHB2_PERIPH_BASE + 0x8000;
        pub const ADC2_BASE: usize = AHB2_PERIPH_BASE + 0x8400;
        pub const ADC12_BASE: usize = ADC1_BASE;
        pub const ADC4_BASE: usize = AHB2_PERIPH_BASE + 0x8800;
        pub const AES_BASE: usize = AHB2_PERIPH_BASE + 0x8C00;
        pub const HASH_BASE: usize = AHB2_PERIPH_BASE + 0x9400;
        pub const RNG_BASE: usize = AHB2_PERIPH_BASE + 0x9800;
        pub const SAES_BASE: usize = AHB2_PERIPH_BASE + 0x9C00;
        pub const PKA_BASE: usize = AHB2_PERIPH_BASE + 0xA000;
        pub const OTFDEC1_BASE: usize = AHB2_PERIPH_BASE + 0xA400;
        pub const OTFDEC2_BASE: usize = AHB2_PERIPH_BASE + 0xA800;
        pub const SDMMC1_BASE: usize = AHB2_PERIPH_BASE + 0xC000;
        pub const SDMMC2_BASE: usize = AHB2_PERIPH_BASE + 0xC400;
    }

    pub mod ahb3 {
        use super::AHB3_PERIPH_BASE;

        pub const FMC_BASE: usize = AHB3_PERIPH_BASE + 0x0000;
        pub const OCTOSPI1_BASE: usize = AHB3_PERIPH_BASE + 0x0400;
        pub const XSPI1_BASE: usize = AHB3_PERIPH_BASE + 0x0400;
        pub const OCTOSPI2_BASE: usize = AHB3_PERIPH_BASE + 0x0800;
        pub const XSPI2_BASE: usize = AHB3_PERIPH_BASE + 0x0800;
        pub const SDMMC1_ALT_BASE: usize = AHB3_PERIPH_BASE + 0x1000;
        pub const SDMMC2_ALT_BASE: usize = AHB3_PERIPH_BASE + 0x1400;
        pub const GPU2D_BASE: usize = AHB3_PERIPH_BASE + 0x2000;
        pub const LTDC_BASE: usize = AHB3_PERIPH_BASE + 0x6800;
        pub const DSI_BASE: usize = AHB3_PERIPH_BASE + 0x6C00;
        pub const DCMI_BASE: usize = AHB3_PERIPH_BASE + 0xC000;
        pub const PSSI_BASE: usize = AHB3_PERIPH_BASE + 0xC400;
    }

    pub mod ahb4 {
        use super::AHB4_PERIPH_BASE;

        pub const HSPI1_BASE: usize = AHB4_PERIPH_BASE + 0x0000;
        pub const HSPI2_BASE: usize = AHB4_PERIPH_BASE + 0x0400;
    }

    pub mod ahb5 {
        use super::AHB5_PERIPH_BASE;

        pub const GPIO_BASE: usize = AHB5_PERIPH_BASE + 0x0000;
        pub const LPGPIO_BASE: usize = AHB5_PERIPH_BASE + 0x0400;
    }

    pub const GTZC_TZIC_BASE: usize = 0x4000_0000;
    pub const GTZC_MPCWM1_BASE: usize = 0x4003_0000;
    pub const GTZC_MPCWM2_BASE: usize = 0x4003_1000;
    pub const GTZC_ETZPC_BASE: usize = 0x4003_2000;
    pub const GTZC_TDC1_BASE: usize = 0x4003_3000;

    pub const VREFBUF_BASE: usize = 0x4000_7030;
    pub const COMP1_BASE: usize = 0x4000_9200;
    pub const COMP2_BASE: usize = 0x4000_9204;
    pub const COMP12_BASE: usize = COMP1_BASE;

    pub const IRTIM_BASE: usize = 0x4001_5800;
    pub const HRTIM_BASE: usize = 0x4001_7400;

    pub const USB_OTG_FS_BASE: usize = 0x4204_0000;
    pub const USB_OTG_HS_BASE: usize = 0x4204_4000;

    pub const DLYB1_BASE: usize = 0x4201_6400;
    pub const DLYB2_BASE: usize = 0x4201_6800;

    pub const TAMP_NS_BASE: usize = 0x4200_0400;
    pub const RTC_NS_BASE: usize = 0x4200_0000;

    pub const LPDMA1_BASE: usize = 0x4002_7000;
    pub const GPDMA2_BASE: usize = 0x4002_8000;
    pub const DMAMUX2_BASE: usize = 0x4002_8400;
    pub const LPDMA2_BASE: usize = 0x4002_9000;

    pub const I3C1_BASE: usize = 0x400A_0000;
    pub const I3C2_BASE: usize = 0x400A_0400;

    pub const SMPS_BASE: usize = 0x4000_5000;
    pub const SMPS_I2C_BASE: usize = 0x4000_5400;
}

pub mod s {
    pub const PERIPH_BASE_SEC: usize = 0x5000_0000;

    pub const APB1_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x0000;
    pub const APB2_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x1000;
    pub const APB3_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x0800;
    pub const AHB1_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x2000;
    pub const AHB2_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x4000;
    pub const AHB3_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x6000;
    pub const AHB4_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0x8000;
    pub const AHB5_PERIPH_BASE_SEC: usize = PERIPH_BASE_SEC + 0xA000;

    pub mod apb1 {
        use super::APB1_PERIPH_BASE_SEC;

        pub const TIM2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0000;
        pub const TIM3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0400;
        pub const TIM4_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0800;
        pub const TIM5_BASE: usize = APB1_PERIPH_BASE_SEC + 0x0C00;
        pub const TIM6_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1000;
        pub const TIM7_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1400;
        pub const RTC_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1800;
        pub const TAMP_BASE: usize = APB1_PERIPH_BASE_SEC + 0x1C00;
        pub const WWDG_BASE: usize = APB1_PERIPH_BASE_SEC + 0x2C00;
        pub const IWDG_BASE: usize = APB1_PERIPH_BASE_SEC + 0x3000;
        pub const SPI2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x3800;
        pub const SPI3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x3C00;
        pub const USART2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x4400;
        pub const USART3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x4800;
        pub const UART4_BASE: usize = APB1_PERIPH_BASE_SEC + 0x4C00;
        pub const UART5_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5000;
        pub const I2C1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5400;
        pub const I2C2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5800;
        pub const I2C3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x5C00;
        pub const CRS_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6000;
        pub const CAN1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6400;
        pub const FDCAN1_BASE: usize = CAN1_BASE;
        pub const CAN2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6800;
        pub const FDCAN2_BASE: usize = CAN2_BASE;
        pub const USB_BASE: usize = APB1_PERIPH_BASE_SEC + 0x6C00;
        pub const PWR_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7000;
        pub const DAC1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7400;
        pub const OPAMP1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7800;
        pub const OPAMP2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x7C00;
        pub const LPTIM1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8000;
        pub const LPTIM2_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8400;
        pub const LPTIM3_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8800;
        pub const I2C4_BASE: usize = APB1_PERIPH_BASE_SEC + 0x8C00;
        pub const LPUART1_BASE: usize = APB1_PERIPH_BASE_SEC + 0x9000;
        pub const UCPD1_BASE: usize = APB1_PERIPH_BASE_SEC + 0xA000;
        pub const UCPD2_BASE: usize = APB1_PERIPH_BASE_SEC + 0xA400;
    }

    pub mod apb2 {
        use super::APB2_PERIPH_BASE_SEC;

        pub const TIM1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x0000;
        pub const TIM8_BASE: usize = APB2_PERIPH_BASE_SEC + 0x0400;
        pub const SPI1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x3000;
        pub const USART1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x3800;
        pub const TIM15_BASE: usize = APB2_PERIPH_BASE_SEC + 0x4000;
        pub const TIM16_BASE: usize = APB2_PERIPH_BASE_SEC + 0x4400;
        pub const TIM17_BASE: usize = APB2_PERIPH_BASE_SEC + 0x4800;
        pub const SAI1_BASE: usize = APB2_PERIPH_BASE_SEC + 0x5400;
        pub const SAI2_BASE: usize = APB2_PERIPH_BASE_SEC + 0x5800;
        pub const SAI3_BASE: usize = APB2_PERIPH_BASE_SEC + 0x5C00;
    }

    pub mod apb3 {
        use super::APB3_PERIPH_BASE_SEC;

        pub const LPTIM4_BASE: usize = APB3_PERIPH_BASE_SEC + 0x0000;
        pub const LPTIM5_BASE: usize = APB3_PERIPH_BASE_SEC + 0x0400;
        pub const LPTIM6_BASE: usize = APB3_PERIPH_BASE_SEC + 0x0800;
        pub const VDDIO2_BASE: usize = APB3_PERIPH_BASE_SEC + 0x1000;
        pub const SYSCFG_BASE: usize = APB3_PERIPH_BASE_SEC + 0x2000;
        pub const EXTI_BASE: usize = APB3_PERIPH_BASE_SEC + 0x2400;
        pub const EXTI_D1_BASE: usize = EXTI_BASE;
        pub const EXTI_D2_BASE: usize = APB3_PERIPH_BASE_SEC + 0x2800;
        pub const EXTI_D3_BASE: usize = APB3_PERIPH_BASE_SEC + 0x2C00;
        pub const RCC_BASE: usize = APB3_PERIPH_BASE_SEC + 0x3000;
        pub const RCC_CRS_BASE: usize = APB3_PERIPH_BASE_SEC + 0x3800;
    }

    pub mod ahb1 {
        use super::AHB1_PERIPH_BASE_SEC;

        pub const GPDMA1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x0000;
        pub const DMAMUX1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x0400;
        pub const CORDIC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x0C00;
        pub const FMAC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x1000;
        pub const MDF1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x1400;
        pub const ADF1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x1800;
        pub const FLASH_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x2000;
        pub const ICACHE_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x2400;
        pub const DCACHE_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x2800;
        pub const CRC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x3000;
        pub const TSC_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x4000;
        pub const RAMCFG_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x4400;
        pub const BKPSRAM_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x4800;
        pub const DPRAM1_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x5000;
        pub const DPRAM2_BASE: usize = AHB1_PERIPH_BASE_SEC + 0x5400;
        pub const DMA2D_BASE: usize = AHB1_PERIPH_BASE_SEC + 0xB000;
    }

    pub mod ahb2 {
        use super::AHB2_PERIPH_BASE_SEC;

        pub const GPIOA_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0000;
        pub const GPIOB_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0400;
        pub const GPIOC_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0800;
        pub const GPIOD_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x0C00;
        pub const GPIOE_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1000;
        pub const GPIOF_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1400;
        pub const GPIOG_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1800;
        pub const GPIOH_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x1C00;
        pub const GPIOI_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x2000;
    }

    pub mod ahb2enr2 {
        use super::AHB2_PERIPH_BASE_SEC;

        pub const ADC1_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8000;
        pub const ADC2_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8400;
        pub const ADC12_BASE: usize = ADC1_BASE;
        pub const ADC4_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8800;
        pub const AES_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x8C00;
        pub const HASH_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x9400;
        pub const RNG_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x9800;
        pub const SAES_BASE: usize = AHB2_PERIPH_BASE_SEC + 0x9C00;
        pub const PKA_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xA000;
        pub const OTFDEC1_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xA400;
        pub const OTFDEC2_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xA800;
        pub const SDMMC1_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xC000;
        pub const SDMMC2_BASE: usize = AHB2_PERIPH_BASE_SEC + 0xC400;
    }

    pub mod ahb3 {
        use super::AHB3_PERIPH_BASE_SEC;

        pub const FMC_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0000;
        pub const OCTOSPI1_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0400;
        pub const XSPI1_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0400;
        pub const OCTOSPI2_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0800;
        pub const XSPI2_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x0800;
        pub const SDMMC1_ALT_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x1000;
        pub const SDMMC2_ALT_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x1400;
        pub const GPU2D_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x2000;
        pub const LTDC_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x6800;
        pub const DSI_BASE: usize = AHB3_PERIPH_BASE_SEC + 0x6C00;
        pub const DCMI_BASE: usize = AHB3_PERIPH_BASE_SEC + 0xC000;
        pub const PSSI_BASE: usize = AHB3_PERIPH_BASE_SEC + 0xC400;
    }

    pub mod ahb4 {
        use super::AHB4_PERIPH_BASE_SEC;

        pub const HSPI1_BASE: usize = AHB4_PERIPH_BASE_SEC + 0x0000;
        pub const HSPI2_BASE: usize = AHB4_PERIPH_BASE_SEC + 0x0400;
    }

    pub mod ahb5 {
        use super::AHB5_PERIPH_BASE_SEC;

        pub const GPIO_BASE: usize = AHB5_PERIPH_BASE_SEC + 0x0000;
        pub const LPGPIO_BASE: usize = AHB5_PERIPH_BASE_SEC + 0x0400;
    }

    pub const GTZC_TZIC_BASE: usize = 0x5000_0000;
    pub const GTZC_MPCWM1_BASE: usize = 0x5003_0000;
    pub const GTZC_MPCWM2_BASE: usize = 0x5003_1000;
    pub const GTZC_ETZPC_BASE: usize = 0x5003_2000;
    pub const GTZC_TDC1_BASE: usize = 0x5003_3000;

    pub const VREFBUF_BASE: usize = 0x5000_7030;
    pub const COMP1_BASE: usize = 0x5000_9200;
    pub const COMP2_BASE: usize = 0x5000_9204;
    pub const COMP12_BASE: usize = COMP1_BASE;

    pub const IRTIM_BASE: usize = 0x5001_5800;
    pub const HRTIM_BASE: usize = 0x5001_7400;

    pub const USB_OTG_FS_BASE: usize = 0x5204_0000;
    pub const USB_OTG_HS_BASE: usize = 0x5204_4000;

    pub const DLYB1_BASE: usize = 0x5201_6400;
    pub const DLYB2_BASE: usize = 0x5201_6800;

    pub const TAMP_S_BASE: usize = 0x5200_0400;
    pub const RTC_S_BASE: usize = 0x5200_0000;

    pub const LPDMA1_BASE: usize = 0x5002_7000;
    pub const GPDMA2_BASE: usize = 0x5002_8000;
    pub const DMAMUX2_BASE: usize = 0x5002_8400;
    pub const LPDMA2_BASE: usize = 0x5002_9000;

    pub const I3C1_BASE: usize = 0x500A_0000;
    pub const I3C2_BASE: usize = 0x500A_0400;

    pub const SMPS_BASE: usize = 0x5000_5000;
    pub const SMPS_I2C_BASE: usize = 0x5000_5400;
}

#[cfg(feature = "stm32u575")]
pub mod stm32u575 {
    pub const DEVICE_NAME: &str = "STM32U575";
    pub const DIE_ID: u8 = 0x1;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = false;
        pub const HAS_I3C: bool = false;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_2M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u585")]
pub mod stm32u585 {
    pub const DEVICE_NAME: &str = "STM32U585";
    pub const DIE_ID: u8 = 0x2;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = true;
        pub const HAS_I3C: bool = true;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_2M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5a5")]
pub mod stm32u5a5 {
    pub const DEVICE_NAME: &str = "STM32U5A5";
    pub const DIE_ID: u8 = 0x3;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = false;
        pub const HAS_DUAL_BANK: bool = false;
        pub const HAS_I3C: bool = false;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = false;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_1M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_256K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5a6")]
pub mod stm32u5a6 {
    pub const DEVICE_NAME: &str = "STM32U5A6";
    pub const DIE_ID: u8 = 0x4;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = false;
        pub const HAS_DUAL_BANK: bool = false;
        pub const HAS_I3C: bool = true;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = false;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_2M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_256K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5f7")]
pub mod stm32u5f7 {
    pub const DEVICE_NAME: &str = "STM32U5F7";
    pub const DIE_ID: u8 = 0x5;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = true;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = true;
        pub const HAS_JPEG: bool = true;
        pub const HAS_DCMI: bool = true;
        pub const HAS_PSSI: bool = true;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = false;
        pub const HAS_I3C: bool = false;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_2M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5f9")]
pub mod stm32u5f9 {
    pub const DEVICE_NAME: &str = "STM32U5F9";
    pub const DIE_ID: u8 = 0x6;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = true;
        pub const HAS_DSI: bool = true;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = true;
        pub const HAS_JPEG: bool = true;
        pub const HAS_DCMI: bool = true;
        pub const HAS_PSSI: bool = true;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = true;
        pub const HAS_I3C: bool = true;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_4M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5g7")]
pub mod stm32u5g7 {
    pub const DEVICE_NAME: &str = "STM32U5G7";
    pub const DIE_ID: u8 = 0x7;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = true;
        pub const HAS_HSPI: bool = true;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = false;
        pub const HAS_I3C: bool = false;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_2M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5g9")]
pub mod stm32u5g9 {
    pub const DEVICE_NAME: &str = "STM32U5G9";
    pub const DIE_ID: u8 = 0x8;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = true;
        pub const HAS_HSPI: bool = true;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = true;
        pub const HAS_I3C: bool = true;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_4M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(feature = "stm32u5j9")]
pub mod stm32u5j9 {
    pub const DEVICE_NAME: &str = "STM32U5J9";
    pub const DIE_ID: u8 = 0x9;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = true;
        pub const HAS_DSI: bool = true;
        pub const HAS_OCTOSPI: bool = true;
        pub const HAS_HSPI: bool = true;
        pub const HAS_GPU2D: bool = true;
        pub const HAS_JPEG: bool = true;
        pub const HAS_DCMI: bool = true;
        pub const HAS_PSSI: bool = true;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = true;
        pub const HAS_PKA: bool = true;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = true;
        pub const HAS_DUAL_BANK: bool = true;
        pub const HAS_I3C: bool = true;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = true;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_4M;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_768K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

#[cfg(all(feature = "stm32u575", not(any(feature = "stm32u585", feature = "stm32u5a5", feature = "stm32u5a6", feature = "stm32u5f7", feature = "stm32u5f9", feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u575::*;
}

#[cfg(all(feature = "stm32u585", not(any(feature = "stm32u5a5", feature = "stm32u5a6", feature = "stm32u5f7", feature = "stm32u5f9", feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u585::*;
}

#[cfg(all(feature = "stm32u5a5", not(any(feature = "stm32u5a6", feature = "stm32u5f7", feature = "stm32u5f9", feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u5a5::*;
}

#[cfg(all(feature = "stm32u5a6", not(any(feature = "stm32u5f7", feature = "stm32u5f9", feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u5a6::*;
}

#[cfg(all(feature = "stm32u5f7", not(any(feature = "stm32u5f9", feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u5f7::*;
}

#[cfg(all(feature = "stm32u5f9", not(any(feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u5f9::*;
}

#[cfg(all(feature = "stm32u5g7", not(any(feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u5g7::*;
}

#[cfg(all(feature = "stm32u5g9", not(feature = "stm32u5j9")))]
pub mod device {
    pub use super::stm32u5g9::*;
}

#[cfg(feature = "stm32u5j9")]
pub mod device {
    pub use super::stm32u5j9::*;
}

#[cfg(not(any(feature = "stm32u575", feature = "stm32u585", feature = "stm32u5a5", feature = "stm32u5a6", feature = "stm32u5f7", feature = "stm32u5f9", feature = "stm32u5g7", feature = "stm32u5g9", feature = "stm32u5j9")))]
pub mod device {
    pub const DEVICE_NAME: &str = "STM32U5xx";
    pub const DIE_ID: u8 = 0x0;

    pub mod features {
        pub const HAS_CRYPTO: bool = true;
        pub const HAS_USB: bool = true;
        pub const HAS_LTDC: bool = false;
        pub const HAS_DSI: bool = false;
        pub const HAS_OCTOSPI: bool = false;
        pub const HAS_HSPI: bool = false;
        pub const HAS_GPU2D: bool = false;
        pub const HAS_JPEG: bool = false;
        pub const HAS_DCMI: bool = false;
        pub const HAS_PSSI: bool = false;
        pub const HAS_ETH: bool = false;
        pub const HAS_OTFDEC: bool = false;
        pub const HAS_PKA: bool = false;
        pub const HAS_SDMMC: bool = true;
        pub const HAS_FMC: bool = false;
        pub const HAS_DUAL_BANK: bool = false;
        pub const HAS_I3C: bool = false;
        pub const HAS_LPGPIO: bool = true;
        pub const HAS_SMPS: bool = false;

        pub const GPIO_PORT_COUNT: usize = 9;
        pub const ADC_COUNT: usize = 3;
        pub const TIM_COUNT: usize = 17;
    }

    pub mod memory {
        use super::super::memory;
        pub const FLASH_BASE: usize = memory::FLASH_BASE;
        pub const FLASH_SIZE: usize = memory::size::FLASH_512K;
        pub const SRAM1_SIZE: usize = memory::size::SRAM_192K;
        pub const SRAM2_SIZE: usize = memory::size::SRAM_64K;
        pub const BKPSRAM_SIZE: usize = memory::size::BKPSRAM_2K;
    }
}

pub mod devices {
    pub const STM32U575: &str = "STM32U575";
    pub const STM32U585: &str = "STM32U585";
    pub const STM32U5A5: &str = "STM32U5A5";
    pub const STM32U5A6: &str = "STM32U5A6";
    pub const STM32U5F7: &str = "STM32U5F7";
    pub const STM32U5F9: &str = "STM32U5F9";
    pub const STM32U5G7: &str = "STM32U5G7";
    pub const STM32U5G9: &str = "STM32U5G9";
    pub const STM32U5J9: &str = "STM32U5J9";
}

#[cfg(feature = "default")]
pub mod defaults {
    pub use super::device::*;
}
