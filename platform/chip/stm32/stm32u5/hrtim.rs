//! HRTIM - High Resolution Timer
//! 高分辨率定时器
//!
//! ## STM32U5 HRTIM 特性 / Features
//! - **定时器单元 / Timer Units:**
//!   - 1 个 Master 定时器 (全局控制)
//!   - 5 个 Timer A-E (独立通道)
//!
//! - **时基分辨率 / Timebase Resolution:**
//!   - 最高 184 ps 分辨率
//!   - 可编程预分频器 (1, 2, 4 分频)
//!
//! - **PWM 功能 / PWM Features:**
//!   - 多达 10 路 PWM 输出
//!   - 4 个比较寄存器
//!   - 死区时间控制
//!   - 故障保护功能
//!
//! - **触发功能 / Trigger Features:**
//!   - ADC/DAC 触发
//!   - 定时器同步
//!   - 突发模式 (Burst Mode)
//!
//! - **中断和事件 / Interrupts and Events:**
//!   - 丰富的中断源
//!   - DMA 支持
//!
//! ## Reference / 参考
//! - RM0456 Reference Manual, Chapter 37: High resolution timer (HRTIM)

#![no_std]

use core::ptr::{read_volatile, write_volatile};

/// HRTIM base address / HRTIM 基地址
/// AHB2 bus, accessible at 0x4003_6C00
pub const HRTIM_BASE: usize = 0x4003_6C00;

/// HRTIM Master Timer base address / HRTIM 主定时器基地址
pub const HRTIM_MASTER_BASE: usize = HRTIM_BASE + 0x000;
/// HRTIM Timer A base address / HRTIM 定时器 A 基地址
pub const HRTIM_TIMA_BASE: usize = HRTIM_BASE + 0x080;
/// HRTIM Timer B base address / HRTIM 定时器 B 基地址
pub const HRTIM_TIMB_BASE: usize = HRTIM_BASE + 0x100;
/// HRTIM Timer C base address / HRTIM 定时器 C 基地址
pub const HRTIM_TIMC_BASE: usize = HRTIM_BASE + 0x180;
/// HRTIM Timer D base address / HRTIM 定时器 D 基地址
pub const HRTIM_TIMD_BASE: usize = HRTIM_BASE + 0x200;
/// HRTIM Timer E base address / HRTIM 定时器 E 基地址
pub const HRTIM_TIME_BASE: usize = HRTIM_BASE + 0x280;
/// HRTIM Common registers base address / HRTIM 公共寄存器基地址
pub const HRTIM_COMMON_BASE: usize = HRTIM_BASE + 0x380;

// ============================================================================
// HRTIM Register Map / HRTIM 寄存器映射
// ============================================================================

/// HRTIM Timer Register Map / HRTIM 定时器寄存器映射
#[repr(C)]
pub struct HrtimTimerRegs {
    /// Timer Control Register / 定时器控制寄存器
    pub cr: u32,
    /// Timer Interrupt Status Register / 定时器中断状态寄存器
    pub isr: u32,
    /// Timer Interrupt Clear Register / 定时器中断清除寄存器
    pub icr: u32,
    /// Timer DMA/Interrupt Enable Register / 定时器 DMA/中断使能寄存器
    pub dier: u32,
    /// Timer Counter Register / 定时器计数器寄存器
    pub cnt: u32,
    /// Timer Period Register / 定时器周期寄存器
    pub per: u32,
    /// Timer Repetition Register / 定时器重复寄存器
    pub rep: u32,
    /// Timer Compare 1 Register / 定时器比较器 1 寄存器
    pub cmp1: u32,
    /// Timer Compare 1 Compound Register / 定时器比较器 1 复合寄存器
    pub cmp1c: u32,
    /// Timer Compare 2 Register / 定时器比较器 2 寄存器
    pub cmp2: u32,
    /// Timer Compare 3 Register / 定时器比较器 3 寄存器
    pub cmp3: u32,
    /// Timer Compare 4 Register / 定时器比较器 4 寄存器
    pub cmp4: u32,
    /// Timer Capture 1 Register / 定时器捕获 1 寄存器
    pub cpt1: u32,
    /// Timer Capture 2 Register / 定时器捕获 2 寄存器
    pub cpt2: u32,
    /// Timer Deadtime Register / 定时器死区时间寄存器
    pub dtr: u32,
    /// Timer Output 1 Set Register / 定时器输出 1 设置寄存器
    pub set1r: u32,
    /// Timer Output 1 Reset Register / 定时器输出 1 复位寄存器
    pub rst1r: u32,
    /// Timer Output 2 Set Register / 定时器输出 2 设置寄存器
    pub set2r: u32,
    /// Timer Output 2 Reset Register / 定时器输出 2 复位寄存器
    pub rst2r: u32,
    /// Timer External Event Filter Register 1 / 定时器外部事件滤波器寄存器 1
    pub eefr1: u32,
    /// Timer External Event Filter Register 2 / 定时器外部事件滤波器寄存器 2
    pub eefr2: u32,
    /// Timer Output Register / 定时器输出寄存器
    pub rstr: u32,
    /// Timer Chopper Register / 定时器斩波寄存器
    pub chpr: u32,
    /// Timer Capture 1 Control Register / 定时器捕获 1 控制寄存器
    pub cpt1cr: u32,
    /// Timer Capture 2 Control Register / 定时器捕获 2 控制寄存器
    pub cpt2cr: u32,
    /// Timer Output Configuration Register / 定时器输出配置寄存器
    pub outr: u32,
    /// Timer Fault Register / 定时器故障寄存器
    pub fltr: u32,
}

/// HRTIM Master Register Map / HRTIM 主定时器寄存器映射
#[repr(C)]
pub struct HrtimMasterRegs {
    /// Master Control Register / 主控制寄存器
    pub cr: u32,
    /// Master Interrupt Status Register / 主中断状态寄存器
    pub isr: u32,
    /// Master Interrupt Clear Register / 主中断清除寄存器
    pub icr: u32,
    /// Master DMA/Interrupt Enable Register / 主 DMA/中断使能寄存器
    pub dier: u32,
    /// Master Counter Register / 主计数器寄存器
    pub cnt: u32,
    /// Master Period Register / 主周期寄存器
    pub per: u32,
    /// Master Repetition Register / 主重复寄存器
    pub rep: u32,
    /// Master Compare 1 Register / 主比较器 1 寄存器
    pub cmp1: u32,
    /// Master Compare 2 Register / 主比较器 2 寄存器
    pub cmp2: u32,
    /// Master Compare 3 Register / 主比较器 3 寄存器
    pub cmp3: u32,
    /// Master Compare 4 Register / 主比较器 4 寄存器
    pub cmp4: u32,
}

/// HRTIM Common Register Map / HRTIM 公共寄存器映射
#[repr(C)]
pub struct HrtimCommonRegs {
    /// Common Control Register 1 / 公共控制寄存器 1
    pub cr1: u32,
    /// Common Control Register 2 / 公共控制寄存器 2
    pub cr2: u32,
    /// Common Interrupt Status Register / 公共中断状态寄存器
    pub isr: u32,
    /// Common Interrupt Clear Register / 公共中断清除寄存器
    pub icr: u32,
    /// Common Interrupt Enable Register / 公共中断使能寄存器
    pub ier: u32,
    /// Common Output Enable Register / 公共输出使能寄存器
    pub oenr: u32,
    /// Common Output Disable Register / 公共输出禁用寄存器
    pub odisr: u32,
    /// Common Output Disable Status Register / 公共输出禁用状态寄存器
    pub odsr: u32,
    /// Burst Mode Control Register / 突发模式控制寄存器
    pub bmcr: u32,
    /// Burst Mode Trigger Register / 突发模式触发寄存器
    pub bmtrgr: u32,
    /// Burst Mode Compare Register / 突发模式比较寄存器
    pub bmcmpcr: u32,
    /// Burst Mode Period Register / 突发模式周期寄存器
    pub bmper: u32,
    /// Common EE Control Register 1 / 公共 EE 控制寄存器 1
    pub eecr1: u32,
    /// Common EE Control Register 2 / 公共 EE 控制寄存器 2
    pub eecr2: u32,
    /// Common EE Control Register 3 / 公共 EE 控制寄存器 3
    pub eecr3: u32,
    /// ADC Trigger 1 Register / ADC 触发器 1 寄存器
    pub adc1r: u32,
    /// ADC Trigger 2 Register / ADC 触发器 2 寄存器
    pub adc2r: u32,
    /// ADC Trigger 3 Register / ADC 触发器 3 寄存器
    pub adc3r: u32,
    /// ADC Trigger 4 Register / ADC 触发器 4 寄存器
    pub adc4r: u32,
    /// DLL Control Register / DLL 控制寄存器
    pub dllcr: u32,
    /// Fault Input Register 1 / 故障输入寄存器 1
    pub fltinr1: u32,
    /// Fault Input Register 2 / 故障输入寄存器 2
    pub fltinr2: u32,
    /// Burst Mode Update Period Register / 突发模式更新周期寄存器
    pub bdmupr: u32,
    /// Burst Mode Update Duty Cycle Register / 突发模式更新占空比寄存器
    pub bdtupr: u32,
    /// Burst Mode Compare Period Register / 突发模式比较周期寄存器
    pub bdcmpcr: u32,
    /// Burst Mode DMA Update Register / 突发模式 DMA 更新寄存器
    pub bdmadr: u32,
}

// ============================================================================
// Enumerations / 枚举类型
// ============================================================================

/// HRTIM Timer selection / HRTIM 定时器选择
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Timer {
    /// Master Timer / 主定时器
    Master = 0,
    /// Timer A / 定时器 A
    A = 1,
    /// Timer B / 定时器 B
    B = 2,
    /// Timer C / 定时器 C
    C = 3,
    /// Timer D / 定时器 D
    D = 4,
    /// Timer E / 定时器 E
    E = 5,
}

/// Timer prescaler / 定时器预分频器
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prescaler {
    /// Clock divide by 1 / 时钟 1 分频
    Div1 = 0,
    /// Clock divide by 2 / 时钟 2 分频
    Div2 = 1,
    /// Clock divide by 4 / 时钟 4 分频
    Div4 = 2,
}

/// External event source / 外部事件源
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    /// Event source 1 / 事件源 1
    Src1 = 0,
    /// Event source 2 / 事件源 2
    Src2 = 1,
    /// Event source 3 / 事件源 3
    Src3 = 2,
    /// Event source 4 / 事件源 4
    Src4 = 3,
}

/// HRTIM instance / HRTIM 实例
pub struct Hrtim;

/// Timer configuration / 定时器配置
#[derive(Clone, Copy, Debug)]
pub struct TimerConfig {
    /// Timer prescaler / 定时器预分频器
    pub prescaler: Prescaler,
    /// Timer period / 定时器周期
    pub period: u16,
    /// Timer repetition counter / 定时器重复计数器
    pub repetition: u8,
    /// Continuous mode / 连续模式
    pub continuous: bool,
}

impl Default for TimerConfig {
    fn default() -> Self {
        TimerConfig {
            prescaler: Prescaler::Div1,
            period: 0xFFFF,
            repetition: 0,
            continuous: true,
        }
    }
}

/// PWM configuration / PWM 配置
#[derive(Clone, Copy, Debug)]
pub struct PwmConfig {
    /// Compare 1 value / 比较器 1 值
    pub cmp1: u16,
    /// Compare 2 value / 比较器 2 值
    pub cmp2: u16,
    /// Compare 3 value / 比较器 3 值
    pub cmp3: u16,
    /// Compare 4 value / 比较器 4 值
    pub cmp4: u16,
    /// Output 1 polarity / 输出 1 极性
    pub output1_polarity: bool,
    /// Output 2 polarity / 输出 2 极性
    pub output2_polarity: bool,
    /// Output 1 idle state / 输出 1 空闲状态
    pub output1_idle_state: bool,
    /// Output 2 idle state / 输出 2 空闲状态
    pub output2_idle_state: bool,
    /// Fault enabled / 故障使能
    pub fault_enabled: bool,
    /// Deadtime rising / 上升沿死区时间
    pub deadtime_rising: u16,
    /// Deadtime falling / 下降沿死区时间
    pub deadtime_falling: u16,
}

impl Default for PwmConfig {
    fn default() -> Self {
        PwmConfig {
            cmp1: 0,
            cmp2: 0,
            cmp3: 0,
            cmp4: 0,
            output1_polarity: false,
            output2_polarity: false,
            output1_idle_state: false,
            output2_idle_state: false,
            fault_enabled: true,
            deadtime_rising: 0,
            deadtime_falling: 0,
        }
    }
}

impl Hrtim {
    /// Create new HRTIM instance / 创建新的 HRTIM 实例
    pub const fn new() -> Self {
        Hrtim
    }

    /// Get Master timer registers / 获取主定时器寄存器
    fn master_regs(&self) -> &mut HrtimMasterRegs {
        unsafe { &mut *(HRTIM_MASTER_BASE as *mut HrtimMasterRegs) }
    }

    /// Get Timer registers / 获取定时器寄存器
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    fn timer_regs(&self, timer: Timer) -> &mut HrtimTimerRegs {
        let base = match timer {
            Timer::A => HRTIM_TIMA_BASE,
            Timer::B => HRTIM_TIMB_BASE,
            Timer::C => HRTIM_TIMC_BASE,
            Timer::D => HRTIM_TIMD_BASE,
            Timer::E => HRTIM_TIME_BASE,
            _ => HRTIM_TIMA_BASE,
        };
        unsafe { &mut *(base as *mut HrtimTimerRegs) }
    }

    /// Get Common registers / 获取公共寄存器
    fn common_regs(&self) -> &mut HrtimCommonRegs {
        unsafe { &mut *(HRTIM_COMMON_BASE as *mut HrtimCommonRegs) }
    }

    /// Enable HRTIM clock / 使能 HRTIM 时钟
    pub fn enable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr |= 1 << 29;  // HRTIMEN bit
        }
    }

    /// Disable HRTIM clock / 禁用 HRTIM 时钟
    pub fn disable_clock(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2enr = rcc_base.add(0xDC / 4);
            *ahb2enr &= !(1 << 29);
        }
    }

    /// Reset HRTIM / 复位 HRTIM
    pub fn reset(&self) {
        let rcc_base: *mut u32 = 0x4002_1000 as *mut u32;
        unsafe {
            let ahb2rstr = rcc_base.add(0x94 / 4);
            *ahb2rstr |= 1 << 29;
            *ahb2rstr &= !(1 << 29);
        }
    }

    /// Calibrate DLL / 校准 DLL
    pub fn calibrate_dll(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().dllcr, 0x0000_0001);
            while (read_volatile(&self.common_regs().isr) & 0x01) == 0 {}
            write_volatile(&mut self.common_regs().icr, 0x01);
        }
    }

    /// Configure Master timer / 配置主定时器
    /// 
    /// # Arguments
    /// * `config` - Timer configuration / 定时器配置
    pub fn configure_master(&self, config: &TimerConfig) {
        unsafe {
            let cr = (config.prescaler as u32) << 3
                | (if config.continuous { 0 } else { 1 }) << 2;
            write_volatile(&mut self.master_regs().cr, cr);
            write_volatile(&mut self.master_regs().per, config.period as u32);
            write_volatile(&mut self.master_regs().rep, config.repetition as u32);
        }
    }

    /// Configure Timer / 配置定时器
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `config` - Timer configuration / 定时器配置
    pub fn configure_timer(&self, timer: Timer, config: &TimerConfig) {
        if timer == Timer::Master {
            self.configure_master(config);
            return;
        }
        unsafe {
            let cr = (config.prescaler as u32) << 3
                | (if config.continuous { 0 } else { 1 }) << 2;
            write_volatile(&mut self.timer_regs(timer).cr, cr);
            write_volatile(&mut self.timer_regs(timer).per, config.period as u32);
            write_volatile(&mut self.timer_regs(timer).rep, config.repetition as u32);
        }
    }

    /// Configure PWM / 配置 PWM
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `config` - PWM configuration / PWM 配置
    pub fn configure_pwm(&self, timer: Timer, config: &PwmConfig) {
        if timer == Timer::Master {
            return;
        }
        unsafe {
            write_volatile(&mut self.timer_regs(timer).cmp1, config.cmp1 as u32);
            write_volatile(&mut self.timer_regs(timer).cmp2, config.cmp2 as u32);
            write_volatile(&mut self.timer_regs(timer).cmp3, config.cmp3 as u32);
            write_volatile(&mut self.timer_regs(timer).cmp4, config.cmp4 as u32);

            let outr = (config.output1_polarity as u32) << 1
                | (config.output2_polarity as u32) << 3
                | (config.output1_idle_state as u32) << 8
                | (config.output2_idle_state as u32) << 9;
            write_volatile(&mut self.timer_regs(timer).outr, outr);

            let dtr = (config.deadtime_rising as u32) << 16
                | (config.deadtime_falling as u32);
            write_volatile(&mut self.timer_regs(timer).dtr, dtr);
        }
    }

    /// Start Timer / 启动定时器
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    pub fn start_timer(&self, timer: Timer) {
        let bit = match timer {
            Timer::Master => 0,
            Timer::A => 1,
            Timer::B => 2,
            Timer::C => 3,
            Timer::D => 4,
            Timer::E => 5,
        };
        unsafe {
            let cr1 = read_volatile(&self.common_regs().cr1);
            write_volatile(&mut self.common_regs().cr1, cr1 | (1 << bit));
        }
    }

    /// Stop Timer / 停止定时器
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    pub fn stop_timer(&self, timer: Timer) {
        let bit = match timer {
            Timer::Master => 0,
            Timer::A => 1,
            Timer::B => 2,
            Timer::C => 3,
            Timer::D => 4,
            Timer::E => 5,
        };
        unsafe {
            let cr1 = read_volatile(&self.common_regs().cr1);
            write_volatile(&mut self.common_regs().cr1, cr1 & !(1 << bit));
        }
    }

    /// Enable Outputs / 使能输出
    pub fn enable_outputs(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().oenr, 0x3F);
        }
    }

    /// Disable Outputs / 禁用输出
    pub fn disable_outputs(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().odisr, 0x3F);
        }
    }

    /// Check if Outputs are enabled / 检查输出是否使能
    pub fn are_outputs_enabled(&self) -> bool {
        unsafe { (read_volatile(&self.common_regs().odsr) & 0x3F) == 0x3F }
    }

    /// Set Compare value / 设置比较值
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `cmp` - Compare number (1-4) / 比较器编号 (1-4)
    /// * `value` - Compare value / 比较值
    pub fn set_compare(&self, timer: Timer, cmp: u8, value: u16) {
        if timer == Timer::Master {
            match cmp {
                1 => unsafe { write_volatile(&mut self.master_regs().cmp1, value as u32) },
                2 => unsafe { write_volatile(&mut self.master_regs().cmp2, value as u32) },
                3 => unsafe { write_volatile(&mut self.master_regs().cmp3, value as u32) },
                4 => unsafe { write_volatile(&mut self.master_regs().cmp4, value as u32) },
                _ => {}
            }
        } else {
            match cmp {
                1 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp1, value as u32) },
                2 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp2, value as u32) },
                3 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp3, value as u32) },
                4 => unsafe { write_volatile(&mut self.timer_regs(timer).cmp4, value as u32) },
                _ => {}
            }
        }
    }

    /// Get Counter value / 获取计数器值
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    pub fn get_counter(&self, timer: Timer) -> u16 {
        if timer == Timer::Master {
            unsafe { read_volatile(&self.master_regs().cnt) as u16 }
        } else {
            unsafe { read_volatile(&self.timer_regs(timer).cnt) as u16 }
        }
    }

    /// Set Period / 设置周期
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `period` - Period value / 周期值
    pub fn set_period(&self, timer: Timer, period: u16) {
        if timer == Timer::Master {
            unsafe { write_volatile(&mut self.master_regs().per, period as u32) }
        } else {
            unsafe { write_volatile(&mut self.timer_regs(timer).per, period as u32) }
        }
    }

    /// Enable Interrupt / 使能中断
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `source` - Interrupt source / 中断源
    pub fn enable_interrupt(&self, timer: Timer, source: u8) {
        let dier_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe {
                let dier = read_volatile(&self.master_regs().dier);
                write_volatile(&mut self.master_regs().dier, dier | dier_bit);
            }
        } else {
            unsafe {
                let dier = read_volatile(&self.timer_regs(timer).dier);
                write_volatile(&mut self.timer_regs(timer).dier, dier | dier_bit);
            }
        }
    }

    /// Disable Interrupt / 禁用中断
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `source` - Interrupt source / 中断源
    pub fn disable_interrupt(&self, timer: Timer, source: u8) {
        let dier_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe {
                let dier = read_volatile(&self.master_regs().dier);
                write_volatile(&mut self.master_regs().dier, dier & !dier_bit);
            }
        } else {
            unsafe {
                let dier = read_volatile(&self.timer_regs(timer).dier);
                write_volatile(&mut self.timer_regs(timer).dier, dier & !dier_bit);
            }
        }
    }

    /// Clear Interrupt / 清除中断
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `source` - Interrupt source / 中断源
    pub fn clear_interrupt(&self, timer: Timer, source: u8) {
        let icr_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe { write_volatile(&mut self.master_regs().icr, icr_bit) }
        } else {
            unsafe { write_volatile(&mut self.timer_regs(timer).icr, icr_bit) }
        }
    }

    /// Check if Interrupt is active / 检查中断是否激活
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `source` - Interrupt source / 中断源
    pub fn is_interrupt_active(&self, timer: Timer, source: u8) -> bool {
        let isr_bit = 1u32 << source;
        if timer == Timer::Master {
            unsafe { (read_volatile(&self.master_regs().isr) & isr_bit) != 0 }
        } else {
            unsafe { (read_volatile(&self.timer_regs(timer).isr) & isr_bit) != 0 }
        }
    }

    /// Configure ADC trigger / 配置 ADC 触发
    /// 
    /// # Arguments
    /// * `adc_num` - ADC number (1-4) / ADC 编号 (1-4)
    /// * `event` - Event / 事件
    /// * `timer` - Timer selection / 定时器选择
    /// * `compare` - Compare number / 比较器编号
    pub fn configure_adc_trigger(&self, adc_num: u8, event: u8, timer: Timer, compare: u8) {
        let timer_bits = match timer {
            Timer::Master => 0,
            Timer::A => 1,
            Timer::B => 2,
            Timer::C => 3,
            Timer::D => 4,
            Timer::E => 5,
        };
        let value = (timer_bits << 4) | (compare as u32);

        unsafe {
            match adc_num {
                1 => {
                    let adc1r = read_volatile(&self.common_regs().adc1r);
                    write_volatile(&mut self.common_regs().adc1r, adc1r | (1 << event));
                }
                2 => {
                    let adc2r = read_volatile(&self.common_regs().adc2r);
                    write_volatile(&mut self.common_regs().adc2r, adc2r | (1 << event));
                }
                3 => {
                    let adc3r = read_volatile(&self.common_regs().adc3r);
                    write_volatile(&mut self.common_regs().adc3r, adc3r | (1 << event));
                }
                4 => {
                    let adc4r = read_volatile(&self.common_regs().adc4r);
                    write_volatile(&mut self.common_regs().adc4r, adc4r | (1 << event));
                }
                _ => {}
            }
        }
    }

    /// Enable Fault / 使能故障
    /// 
    /// # Arguments
    /// * `fault` - Fault number / 故障编号
    pub fn enable_fault(&self, fault: u8) {
        unsafe {
            let fltinr1 = read_volatile(&self.common_regs().fltinr1);
            write_volatile(&mut self.common_regs().fltinr1, fltinr1 | (1 << (fault * 8)));
        }
    }

    /// Disable Fault / 禁用故障
    /// 
    /// # Arguments
    /// * `fault` - Fault number / 故障编号
    pub fn disable_fault(&self, fault: u8) {
        unsafe {
            let fltinr1 = read_volatile(&self.common_regs().fltinr1);
            write_volatile(&mut self.common_regs().fltinr1, fltinr1 & !(1 << (fault * 8)));
        }
    }

    /// Enter Burst Mode / 进入突发模式
    pub fn enter_burst_mode(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().bmtrgr, 0x01);
        }
    }

    /// Exit Burst Mode / 退出突发模式
    pub fn exit_burst_mode(&self) {
        unsafe {
            write_volatile(&mut self.common_regs().bmtrgr, 0x02);
        }
    }

    /// Configure Burst Mode / 配置突发模式
    /// 
    /// # Arguments
    /// * `prescaler` - Prescaler / 预分频器
    /// * `period` - Period / 周期
    pub fn configure_burst_mode(&self, prescaler: u8, period: u16) {
        unsafe {
            let bmcr = (prescaler as u32) << 16 | (period as u32);
            write_volatile(&mut self.common_regs().bmcr, bmcr);
        }
    }

    /// Set Duty Cycle / 设置占空比
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `duty_percent` - Duty cycle in percent / 占空比百分比
    pub fn set_duty_cycle(&self, timer: Timer, duty_percent: f32) {
        if timer == Timer::Master {
            return;
        }
        let period = unsafe { read_volatile(&self.timer_regs(timer).per) as u16 };
        let compare = (period as f32 * duty_percent / 100.0) as u16;
        unsafe { write_volatile(&mut self.timer_regs(timer).cmp1, compare as u32) };
    }

    /// Update Frequency / 更新频率
    /// 
    /// # Arguments
    /// * `timer` - Timer selection / 定时器选择
    /// * `frequency_hz` - Target frequency in Hz / 目标频率 (Hz)
    /// * `sysclk_mhz` - System clock in MHz / 系统时钟 (MHz)
    pub fn update_frequency(&self, timer: Timer, frequency_hz: u32, sysclk_mhz: u32) {
        let prescaler = unsafe { (read_volatile(&self.timer_regs(timer).cr) >> 3) & 0x03 };
        let prescale_factor = match prescaler {
            0 => 1,
            1 => 2,
            2 => 4,
            _ => 1,
        };
        let period = (sysclk_mhz * 1_000_000) / (frequency_hz * prescale_factor as u32) - 1;
        self.set_period(timer, period as u16);
    }
}
