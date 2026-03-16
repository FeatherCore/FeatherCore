# FeatherCore MMU 抽象层设计

## 概述

FeatherCore 需要同时支持：
1. **带 MMU 的平台** (如 ARM Cortex-A, RISC-V with MMU)
2. **不带 MMU 的平台** (如 ARM Cortex-M, RISC-V without MMU)

关键设计原则：**编译时特性 + 运行时抽象**

## 平台分类

### 不带 MMU 的平台 (no_mmu)

**特点**:
- 物理地址 = 虚拟地址 (1:1 映射)
- 无内存保护
- 无页表管理
- 简单快速

**典型平台**:
- ARM Cortex-M0/M3/M4/M7/M23/M33
- RISC-V (无 MMU 配置)
- 嵌入式 MCU

### 带 MMU 的平台 (with_mmu)

**特点**:
- 虚拟地址 ≠ 物理地址
- 支持内存保护
- 需要页表管理
- 支持进程隔离

**典型平台**:
- ARM Cortex-A5/A7/A8/A15/A53/A72
- RISC-V (Sv32/Sv39)
- x86_64

## 架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                    内核代码 (Kernel Code)                    │
│                                                              │
│  // 统一的内存管理 API                                        │
│  let vaddr = mmu::translate(paddr);                         │
│  mmu::protect_region(region, flags);                        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              MMU 抽象层 (MMU Abstraction Layer)               │
│                                                              │
│  #[cfg(feature = "no_mmu")]                                 │
│  pub mod no_mmu;  // 空实现                                  │
│                                                              │
│  #[cfg(feature = "with_mmu")]                               │
│  pub mod with_mmu; // 完整实现                               │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
┌──────────────────────────┐  ┌──────────────────────────┐
│   no_mmu 模块 (空实现)    │  │  with_mmu 模块 (完整实现)  │
│                          │  │                          │
│  - translate() = 恒等映射 │  │  - 页表管理               │
│  - protect() = 无操作    │  │  - TLB 管理                │
│  - enable() = 无操作     │  │  - 地址转换               │
│                          │  │  - 内存保护               │
└──────────────────────────┘  └──────────────────────────┘
                │                           │
                ▼                           ▼
        物理地址直接访问            MMU 硬件管理
        (Cortex-M)                (Cortex-A, x86)
```

## 实现方案

### 1. Cargo Feature 配置

```toml
# common/Cargo.toml
[features]
# MMU 支持
no_mmu = []  # 不带 MMU 的平台
with_mmu = []  # 带 MMU 的平台

# 架构 + MMU 组合
armv7-m = ["arm", "no_mmu"]           # Cortex-M4/M7 (无 MMU)
armv7-a = ["arm", "with_mmu"]         # Cortex-A7 (有 MMU)
armv8-a = ["arm", "with_mmu"]         # Cortex-A53/A72 (有 MMU)

riscv-no-mmu = ["riscv", "no_mmu"]    # RISC-V (无 MMU)
riscv-sv32 = ["riscv", "with_mmu"]    # RISC-V Sv32 (有 MMU)
riscv-sv39 = ["riscv", "with_mmu"]    # RISC-V Sv39 (有 MMU)
```

### 2. MMU Trait 定义

```rust
// common/src/mmu/mod.rs
#![no_std]

use core::fmt::Debug;

/// 物理地址
pub type PhysAddr = usize;

/// 虚拟地址
pub type VirtAddr = usize;

/// 内存保护标志
#[derive(Debug, Clone, Copy)]
pub struct MemFlags {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user_accessible: bool,
}

impl MemFlags {
    pub const fn default_kernel_rw() -> Self {
        MemFlags {
            read: true,
            write: true,
            execute: false,
            user_accessible: false,
        }
    }
    
    pub const fn default_user_rw() -> Self {
        MemFlags {
            read: true,
            write: true,
            execute: false,
            user_accessible: true,
        }
    }
    
    pub const fn default_code() -> Self {
        MemFlags {
            read: true,
            write: false,
            execute: true,
            user_accessible: true,
        }
    }
}

/// MMU 操作 Trait
pub trait MmuOperations: Debug {
    /// 初始化 MMU
    fn init(&mut self);
    
    /// 启用 MMU
    fn enable(&mut self);
    
    /// 禁用 MMU
    fn disable(&mut self);
    
    /// 检查 MMU 是否启用
    fn is_enabled(&self) -> bool;
    
    /// 物理地址到虚拟地址的转换
    fn translate(&self, paddr: PhysAddr) -> VirtAddr;
    
    /// 虚拟地址到物理地址的转换
    fn translate_back(&self, vaddr: VirtAddr) -> Option<PhysAddr>;
    
    /// 映射物理地址到虚拟地址
    fn map(&mut self, vaddr: VirtAddr, paddr: PhysAddr, flags: MemFlags);
    
    /// 取消映射
    fn unmap(&mut self, vaddr: VirtAddr);
    
    /// 保护内存区域
    fn protect(&mut self, vaddr: VirtAddr, size: usize, flags: MemFlags);
    
    /// 刷新 TLB
    fn flush_tlb(&mut self);
    
    /// 刷新 TLB 中的单个条目
    fn flush_tlb_one(&mut self, vaddr: VirtAddr);
}
```

### 3. no_mmu 实现 (空实现)

```rust
// common/src/mmu/no_mmu.rs
#![no_std]

use super::{MmuOperations, PhysAddr, VirtAddr, MemFlags};

/// 不带 MMU 的实现
#[derive(Debug)]
pub struct NoMmu;

impl NoMmu {
    pub const fn new() -> Self {
        NoMmu
    }
}

impl MmuOperations for NoMmu {
    fn init(&mut self) {
        // 无 MMU，无需初始化
    }
    
    fn enable(&mut self) {
        // 无 MMU，无需启用
    }
    
    fn disable(&mut self) {
        // 无 MMU，无需禁用
    }
    
    fn is_enabled(&self) -> bool {
        false
    }
    
    /// 物理地址 = 虚拟地址 (1:1 映射)
    fn translate(&self, paddr: PhysAddr) -> VirtAddr {
        paddr
    }
    
    fn translate_back(&self, vaddr: VirtAddr) -> Option<PhysAddr> {
        Some(vaddr)
    }
    
    fn map(&mut self, _vaddr: VirtAddr, _paddr: PhysAddr, _flags: MemFlags) {
        // 无 MMU，无需映射
    }
    
    fn unmap(&mut self, _vaddr: VirtAddr) {
        // 无 MMU，无需取消映射
    }
    
    fn protect(&mut self, _vaddr: VirtAddr, _size: usize, _flags: MemFlags) {
        // 无 MMU，无法保护
    }
    
    fn flush_tlb(&mut self) {
        // 无 MMU，无 TLB
    }
    
    fn flush_tlb_one(&mut self, _vaddr: VirtAddr) {
        // 无 MMU，无 TLB
    }
}
```

### 4. with_mmu 实现 (以 ARM Cortex-A 为例)

```rust
// common/src/mmu/arm_mmu.rs
#![no_std]

use super::{MmuOperations, PhysAddr, VirtAddr, MemFlags};

/// ARM MMU 实现 (ARMv7-A/ARMv8-A)
#[derive(Debug)]
pub struct ArmMmu {
    /// 页表基地址
    ttbr0: PhysAddr,
    /// 是否启用
    enabled: bool,
}

impl ArmMmu {
    pub const fn new() -> Self {
        ArmMmu {
            ttbr0: 0,
            enabled: false,
        }
    }
    
    /// 设置页表基地址
    pub fn set_ttbr0(&mut self, paddr: PhysAddr) {
        self.ttbr0 = paddr;
    }
}

impl MmuOperations for ArmMmu {
    fn init(&mut self) {
        // 初始化页表
        // 创建恒等映射 (identity mapping)
        for i in 0..4096 {
            let vaddr = i * 4096;
            let paddr = i * 4096;
            self.map(vaddr, paddr, MemFlags::default_kernel_rw());
        }
    }
    
    fn enable(&mut self) {
        unsafe {
            // ARMv7-A: 设置 TTBR0
            // ARMv8-A: 设置 TTBR0_EL1
            #[cfg(target_arch = "arm")]
            core::arch::asm!(
                "mcr p15, 0, {0}, c2, c0, 0",
                in(reg) self.ttbr0,
            );
            
            // 设置内存属性
            #[cfg(target_arch = "arm")]
            core::arch::asm!(
                "mcr p15, 0, {0}, c3, c0, 0",  // DACR
                in(reg) 0x55555555,  // 所有域为 Client
            );
            
            // 启用 MMU
            #[cfg(target_arch = "arm")]
            core::arch::asm!(
                "mrc p15, 0, {0}, c1, c0, 0",  // 读取 SCTLR
                out(reg) _,
            );
            
            self.enabled = true;
        }
        
        // 同步流水线
        unsafe {
            core::arch::asm!(
                "dsb",
                "isb",
            );
        }
    }
    
    fn disable(&mut self) {
        unsafe {
            // 禁用 MMU
            #[cfg(target_arch = "arm")]
            core::arch::asm!(
                "mrc p15, 0, {0}, c1, c0, 0",  // 读取 SCTLR
                out(reg) _,
                "mcr p15, 0, {0}, c1, c0, 0",  // 写入 SCTLR (清除 M 位)
            );
            
            self.enabled = false;
        }
    }
    
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    fn translate(&self, paddr: PhysAddr) -> VirtAddr {
        // TODO: 查询页表
        paddr  // 简化：暂时恒等映射
    }
    
    fn translate_back(&self, vaddr: VirtAddr) -> Option<PhysAddr> {
        // TODO: 查询页表
        Some(vaddr)  // 简化：暂时恒等映射
    }
    
    fn map(&mut self, vaddr: VirtAddr, paddr: PhysAddr, flags: MemFlags) {
        // TODO: 创建页表条目
        // ARMv7-A: Section descriptor (1MB)
        // ARMv8-A: Block descriptor
        
        let entry = if flags.user_accessible {
            0b11  // AP[2:1] = 11 (EL0/EL1 可访问)
        } else {
            0b01  // AP[2:1] = 01 (仅 EL1 可访问)
        } | if flags.write {
            0  // AP[1] = 0 (可写)
        } else {
            0b100  // AP[2] = 1 (只读)
        } | paddr;
        
        // 写入页表
        // TODO: 实际实现
    }
    
    fn unmap(&mut self, vaddr: VirtAddr) {
        // TODO: 清除页表条目
        self.flush_tlb_one(vaddr);
    }
    
    fn protect(&mut self, vaddr: VirtAddr, size: usize, flags: MemFlags) {
        // TODO: 修改页表条目的权限位
    }
    
    fn flush_tlb(&mut self) {
        unsafe {
            #[cfg(target_arch = "arm")]
            core::arch::asm!(
                "mcr p15, 0, {0}, c8, c7, 0",  // TLBIALL
                in(reg) 0,
                "dsb",
                "isb",
            );
        }
    }
    
    fn flush_tlb_one(&mut self, vaddr: VirtAddr) {
        unsafe {
            #[cfg(target_arch = "arm")]
            core::arch::asm!(
                "mcr p15, 0, {0}, c8, c6, 1",  // TLIMVA
                in(reg) vaddr,
                "dsb",
                "isb",
            );
        }
    }
}
```

### 5. 统一的 MMU 接口

```rust
// common/src/mmu/mod.rs
#[cfg(feature = "no_mmu")]
mod no_mmu;
#[cfg(feature = "with_mmu")]
mod arm_mmu;

#[cfg(feature = "no_mmu")]
pub use no_mmu::NoMmu;
#[cfg(feature = "with_mmu")]
pub use arm_mmu::ArmMmu;

/// 全局 MMU 实例
#[cfg(feature = "no_mmu")]
static mut GLOBAL_MMU: NoMmu = NoMmu::new();

#[cfg(feature = "with_mmu")]
static mut GLOBAL_MMU: ArmMmu = ArmMmu::new();

/// 获取全局 MMU
pub fn get_mmu() -> &'static mut dyn MmuOperations {
    unsafe { &mut GLOBAL_MMU }
}

/// 辅助函数：物理地址转虚拟地址
pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    get_mmu().translate(paddr)
}

/// 辅助函数：虚拟地址转物理地址
pub fn virt_to_phys(vaddr: VirtAddr) -> Option<PhysAddr> {
    get_mmu().translate_back(vaddr)
}

/// 辅助函数：映射内存
pub fn map_memory(vaddr: VirtAddr, paddr: PhysAddr, flags: MemFlags) {
    get_mmu().map(vaddr, paddr, flags);
}

/// 辅助函数：保护内存区域
pub fn protect_memory(vaddr: VirtAddr, size: usize, flags: MemFlags) {
    get_mmu().protect(vaddr, size, flags);
}
```

## 平台特性配置

### 不带 MMU 的平台

```toml
# platform/board/stm32f429i-disc/stm32f429i-disc_defconfig.toml
[board]
name = "stm32f429i-disc"
chip = "stm32f4"

[features]
# Cortex-M7, 无 MMU
arch = "armv7-em"
mmu = "no_mmu"

[memory]
flash_base = 0x08000000
sram_base = 0x20000000
```

### 带 MMU 的平台

```toml
# platform/board/raspi3/raspi3_defconfig.toml
[board]
name = "raspi3"
chip = "bcm2837"

[features]
# Cortex-A53, 有 MMU
arch = "armv8-a"
mmu = "with_mmu"

[memory]
ram_base = 0x00000000
ram_size = 0x40000000  # 1GB

[mmu]
# 页表配置
page_size = 4096
ttbr0_base = 0x00000000
```

## 内核使用示例

### 1. 初始化 MMU

```rust
// kernel/src/main.rs
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // 初始化 MMU (如果支持)
    #[cfg(feature = "with_mmu")]
    {
        use feathercore_common::mmu::{get_mmu, MemFlags};
        
        let mmu = get_mmu();
        mmu.init();
        
        // 映射设备内存
        mmu.map(
            0xFFFF0000,  // 虚拟地址
            0x3F000000,  // 物理地址 (BCM2837 外设)
            MemFlags::default_kernel_rw(),
        );
        
        // 启用 MMU
        mmu.enable();
    }
    
    // 继续内核初始化
    init_kernel();
}
```

### 2. 访问设备内存

```rust
// kernel/src/driver/uart.rs
use feathercore_common::mmu::phys_to_virt;

pub struct Uart {
    base: usize,
}

impl Uart {
    pub fn new(paddr: usize) -> Self {
        // 转换为虚拟地址
        let vaddr = phys_to_virt(paddr);
        Uart { base: vaddr }
    }
    
    fn write(&self, byte: u8) {
        unsafe {
            // 直接访问虚拟地址
            // no_mmu: vaddr == paddr
            // with_mmu: vaddr 已映射到 paddr
            core::ptr::write_volatile(self.base as *mut u8, byte);
        }
    }
}
```

### 3. 进程内存隔离 (仅 with_mmu)

```rust
// kernel/src/process.rs
use feathercore_common::mmu::{get_mmu, MemFlags};

pub struct Process {
    pid: Pid,
    page_table: PhysAddr,  // 进程的页表
}

impl Process {
    pub fn new() -> Self {
        let mut mmu = get_mmu();
        
        // 创建新的页表
        let page_table = allocate_page_table();
        
        // 设置进程页表
        mmu.set_ttbr0(page_table);
        
        Process {
            pid: allocate_pid(),
            page_table,
        }
    }
    
    pub fn switch_to(&self) {
        unsafe {
            // 切换到进程的页表
            #[cfg(target_arch = "aarch64")]
            core::arch::asm!(
                "msr TTBR0_EL1, {0}",
                in(reg) self.page_table,
                "isb",
            );
            
            // 刷新 TLB
            get_mmu().flush_tlb();
        }
    }
}
```

## 编译配置

### 不带 MMU 的编译

```bash
# STM32F429 (Cortex-M7, no MMU)
cargo build --release \
  --features stm32f429i-disc,no_mmu \
  --target thumbv7em-none-eabihf
```

### 带 MMU 的编译

```bash
# Raspberry Pi 3 (Cortex-A53, with MMU)
cargo build --release \
  --features raspi3,with_mmu \
  --target aarch64-unknown-none
```

## 总结

### 设计优势

✅ **统一的 API**: 内核代码不需要关心是否有 MMU  
✅ **编译时优化**: no_mmu 的代码会被完全优化掉  
✅ **零开销**: no_mmu 平台的 translate() 只是恒等映射  
✅ **可扩展**: 容易添加新的 MMU 后端 (RISC-V, x86)  

### 支持的平台

| 平台 | 架构 | MMU | Feature |
|------|------|-----|---------|
| STM32F4 | Cortex-M4 | ❌ | `armv7-m,no_mmu` |
| STM32F7 | Cortex-M7 | ❌ | `armv7-em,no_mmu` |
| STM32H7 | Cortex-M7 | ❌ | `armv7-em,no_mmu` |
| Raspberry Pi 3 | Cortex-A53 | ✅ | `armv8-a,with_mmu` |
| Raspberry Pi 4 | Cortex-A72 | ✅ | `armv8-a,with_mmu` |
| RISC-V (无 MMU) | RV32IMAC | ❌ | `riscv,no_mmu` |
| RISC-V (Sv32) | RV32IMAC | ✅ | `riscv-sv32,with_mmu` |

### 实现路径

1. **阶段 1**: 创建 MMU trait 和 no_mmu 实现
2. **阶段 2**: 实现 ARM MMU (ARMv7-A/ARMv8-A)
3. **阶段 3**: 实现 RISC-V MMU (Sv32/Sv39)
4. **阶段 4**: 更新内核代码使用统一的 MMU API
5. **阶段 5**: 添加进程内存保护支持

这样就可以同时支持带 MMU 和不带 MMU 的平台了！
