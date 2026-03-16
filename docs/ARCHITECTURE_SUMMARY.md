# FeatherCore 架构总结

## 项目架构

```
FeatherCore/
├── boot/                    # 二进制目标 (Bootloader)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # 入口点
│       └── mod.rs           # 模块组织
│
├── kernel/                  # 二进制目标 (操作系统内核)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── lib.rs
│
├── common/                  # 聚合库 (Workspace)
│   ├── Cargo.toml           # 定义 workspace
│   ├── src/
│   │   └── lib.rs           # 只做 re-export
│   ├── arch/                # 架构子库
│   │   ├── arm/
│   │   └── riscv/
│   ├── driver/              # 驱动子库
│   ├── mmu/                 # MMU 子库 ⭐
│   └── platform/            # 平台子库
│
└── rootfs/                  # 多二进制目标
    └── Cargo.toml
```

## 核心设计原则

### 1. boot 是二进制目标

- **产物**: 可执行的 ELF 文件
- **入口**: `boot/src/main.rs` 的 `_start()` 函数
- **模块组织**: 在 `boot/src/` 下可以有多个模块
- **依赖**: 只依赖 `common` 聚合库

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod loader;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // boot 逻辑
}
```

### 2. common 是纯聚合库

- **本身无功能代码**: 只是 re-export 子库
- **子库独立**: 每个子库是独立的 Cargo 项目
- **统一入口**: 通过 common 统一访问所有子库

```rust
// common/src/lib.rs
#![no_std]

// 只 re-export，无实际代码
#[cfg(feature = "arm")]
pub use feathercore_arch_arm as arch;

#[cfg(feature = "mmu")]
pub use feathercore_mmu as mmu;

#[cfg(feature = "driver")]
pub use feathercore_driver as driver;
```

### 3. 子库是独立 Cargo 项目

每个子库有自己的：
- `Cargo.toml`
- `src/lib.rs`
- 模块组织

```
common/mmu/
├── Cargo.toml          # 独立的 Cargo 项目
└── src/
    ├── lib.rs
    ├── types.rs
    ├── no_mmu.rs
    └── arm_mmu.rs
```

## 依赖关系

```
boot (二进制)
  └─→ common (聚合库)
      ├─→ arch/arm (子库)
      ├─→ mmu (子库)
      ├─→ driver (子库)
      └─→ platform (子库)
```

## Cargo Features

```toml
# common/Cargo.toml
[features]
# 架构
arm = ["feathercore-arch-arm"]
riscv = ["feathercore-arch-riscv"]

# MMU
no_mmu = ["feathercore-mmu/no_mmu"]
with_mmu = ["feathercore-mmu/with_mmu"]

# 平台
stm32f4 = ["arm", "feathercore-platform/stm32f4"]
stm32h7 = ["arm", "feathercore-platform/stm32h7"]
```

## 使用示例

### boot 中使用 common

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

use feathercore_common::arch::cpu;
use feathercore_common::mmu;
use feathercore_common::driver::uart;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化 CPU
    cpu::init();
    
    // 初始化 MMU (如果支持)
    #[cfg(feature = "with_mmu")]
    mmu::init();
    
    // 初始化串口
    let uart = uart::UART::new(0x40011000);
    uart.write(b"Boot starting...\n");
}
```

### 子库的模块组织

```rust
// common/mmu/src/lib.rs
#![no_std]

mod types;
mod no_mmu;
mod arm_mmu;

pub use types::*;
pub use no_mmu::NoMmu;
pub use arm_mmu::ArmMmu;
```

## 编译命令

### 不带 MMU 的平台

```bash
# STM32F4 (Cortex-M4, no MMU)
cd boot
cargo build --release \
  --features stm32f4,no_mmu \
  --target thumbv7em-none-eabihf
```

### 带 MMU 的平台

```bash
# Raspberry Pi 3 (Cortex-A53, with MMU)
cd boot
cargo build --release \
  --features raspi3,with_mmu \
  --target aarch64-unknown-none
```

## 关键优势

### 1. 模块化

- 每个子库独立开发、测试
- 清晰的职责分离
- 易于维护和扩展

### 2. 灵活性

- 通过 features 选择组件
- 可以添加新架构、新平台
- 不会引入不需要的代码

### 3. 可维护性

- common 很简单，只是聚合
- 功能代码都在子库中
- 每个子库有清晰结构

### 4. 编译优化

- 不需要的子库不会编译
- LTO 跨子库优化
- 编译缓存有效

## 文件清单

### 核心文档

- `docs/PROJECT_STRUCTURE.md` - 项目结构详解
- `docs/REFACTORING_PLAN.md` - 重构计划
- `docs/ARCHITECTURE_SUMMARY.md` - 本文件

### common 子库

- `common/arch/arm/` - ARM 架构支持
- `common/arch/riscv/` - RISC-V 架构支持
- `common/driver/` - 驱动框架
- `common/mmu/` - MMU 抽象层 ⭐
- `common/platform/` - 平台支持

### 二进制目标

- `boot/` - Bootloader
- `kernel/` - 操作系统内核
- `rootfs/` - 用户空间工具

## 总结

✅ **架构清晰**: boot(二进制) + common(聚合库) + 子库  
✅ **职责分离**: common 只聚合，功能在子库  
✅ **模块化**: 每个子库独立开发和测试  
✅ **灵活性**: 通过 features 灵活配置  
✅ **可维护**: 清晰的模块边界  

FeatherCore 现在有了清晰的架构，易于开发和维护！
