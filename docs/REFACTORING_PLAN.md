# FeatherCore 架构重构计划

## 当前状态

### ✅ 已完成

1. **MMU 抽象层设计完成**
   - Trait 定义
   - no_mmu 实现
   - arm_mmu 实现

2. **子库结构初步建立**
   - `common/arch/arm/` - ARM 架构子库
   - `common/arch/riscv/` - RISC-V 架构子库
   - `common/driver/` - 驱动子库
   - `common/mmu/` - MMU 子库 (新建)
   - `common/platform/` - 平台支持子库

### ⚠️ 需要改进的问题

1. **common/src/ 目录还存在代码**
   - `common/src/mmu/` 应该移除，代码移到 `common/mmu/src/`
   - `common/src/platform/` 应该移除，代码移到 `common/platform/src/`
   - `common/src/lib.rs` 应该只做 re-export

2. **common/Cargo.toml 过于复杂**
   - 有重复的 features 定义
   - 需要简化为纯聚合配置

3. **boot 模块组织不够清晰**
   - 目前只有 main.rs
   - 需要按功能模块组织

## 目标架构

### common 作为纯聚合库

```
common/
├── Cargo.toml              # 定义 workspace 和 features
├── src/
│   └── lib.rs              # 只做 re-export，无实际代码
├── arch/                   # 架构子库 (独立 Cargo 项目)
│   ├── arm/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── cpu/
│   │       ├── mmu/
│   │       └── cache/
│   └── riscv/
│       ├── Cargo.toml
│       └── src/
│           └── ...
├── driver/                 # 驱动子库 (独立 Cargo 项目)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── gpio/
│       ├── spi/
│       └── uart/
├── mmu/                    # MMU 子库 (独立 Cargo 项目) ⭐ 新建
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── types.rs
│       ├── no_mmu.rs
│       └── arm_mmu.rs
└── platform/               # 平台子库 (独立 Cargo 项目)
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── stm32f4/
        ├── stm32h7/
        └── ...
```

### boot 模块组织

```
boot/
├── Cargo.toml
└── src/
    ├── main.rs             # 入口点
    ├── lib.rs              # 可选：共享代码
    ├── mod.rs              # 根模块声明
    ├── arch/               # 架构相关
    │   ├── mod.rs
    │   ├── init.rs         # 架构初始化
    │   └── vectors.rs      # 中断向量表
    ├── drivers/            # Boot 阶段驱动
    │   ├── mod.rs
    │   ├── uart.rs         # 串口驱动
    │   ├── flash.rs        # Flash 驱动
    │   └── clock.rs        # 时钟驱动
    └── loader/             # 内核加载器
        ├── mod.rs
        ├── elf.rs          # ELF 解析
        └── load.rs         # 加载逻辑
```

## 重构步骤

### 步骤 1: 完成 mmu 子库

```bash
# 1. 移动代码
cp common/src/mmu/*.rs common/mmu/src/

# 2. 创建 types.rs (从 mod.rs 分离类型定义)
# 3. 更新 no_mmu.rs 和 arm_mmu.rs

# 4. 删除旧的 common/src/mmu/ 目录
rm -rf common/src/mmu/
```

### 步骤 2: 简化 common/Cargo.toml

```toml
[package]
name = "feathercore-common"
version = "0.1.0"
edition = "2021"

# 注意：这是聚合库，依赖都是子库
[dependencies]
feathercore-arch-arm = { path = "./arch/arm", optional = true }
feathercore-arch-riscv = { path = "./arch/riscv", optional = true }
feathercore-driver = { path = "./driver", optional = true }
feathercore-mmu = { path = "./mmu", optional = true }
feathercore-platform = { path = "./platform", optional = true }

[features]
# 架构选择
arm = ["feathercore-arch-arm"]
riscv = ["feathercore-arch-riscv"]

# MMU 选择
no_mmu = ["feathercore-mmu/no_mmu"]
with_mmu = ["feathercore-mmu/with_mmu"]

# 平台选择 (通过子库 features 传递)
stm32f4 = ["arm", "feathercore-platform/stm32f4"]
stm32h7 = ["arm", "feathercore-platform/stm32h7"]
```

### 步骤 3: 简化 common/src/lib.rs

```rust
//! FeatherCore Common Library
//! 
//! This is an aggregation library that only re-exports sub-libraries.
//! 本身不包含功能代码，只是聚合子库。

#![no_std]

// 根据 feature 导出架构
#[cfg(feature = "arm")]
pub use feathercore_arch_arm as arch_arm;

#[cfg(feature = "riscv")]
pub use feathercore_arch_riscv as arch_riscv;

// 导出 MMU 库
#[cfg(feature = "mmu")]
pub use feathercore_mmu as mmu;

// 导出驱动库
#[cfg(feature = "driver")]
pub use feathercore_driver as driver;

// 导出平台库
#[cfg(feature = "platform")]
pub use feathercore_platform as platform;
```

### 步骤 4: 优化 boot 模块组织

创建 boot 的模块结构：

```rust
// boot/src/main.rs
#![no_std]
#![no_main]

mod arch;
mod drivers;
mod loader;

use feathercore_common::platform;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. 架构初始化
    arch::init();
    
    // 2. 初始化串口
    let uart = drivers::uart::UART::new(platform::stm32f4::USART1_BASE);
    uart.write(b"FeatherCore Boot\n");
    
    // 3. 加载内核
    loader::load_kernel();
}
```

### 步骤 5: 更新文档

更新所有相关文档说明新的架构：
- docs/PROJECT_STRUCTURE.md
- common/docs/MMU_IMPLEMENTATION.md
- README.md

## 依赖关系图

```
boot (二进制)
  │
  └─→ feathercore-common (聚合库)
      │
      ├─→ feathercore-arch-arm (子库)
      │   └─→ src/
      │       ├── cpu/
      │       ├── mmu/
      │       └── cache/
      │
      ├─→ feathercore-mmu (子库) ⭐
      │   └─→ src/
      │       ├── types.rs
      │       ├── no_mmu.rs
      │       └── arm_mmu.rs
      │
      ├─→ feathercore-driver (子库)
      │   └─→ src/
      │       ├── gpio/
      │       ├── spi/
      │       └── uart/
      │
      └─→ feathercore-platform (子库)
          └─→ src/
              ├── stm32f4/
              ├── stm32h7/
              └── ...
```

## 关键原则

### 1. common 是纯聚合库

❌ **不应该**:
```rust
// common/src/mmu/mod.rs - 错误！功能代码不应该在这里
pub struct MemFlags { ... }
```

✅ **应该**:
```rust
// common/src/lib.rs - 正确！只做 re-export
#[cfg(feature = "mmu")]
pub use feathercore_mmu as mmu;
```

### 2. 子库是独立的 Cargo 项目

每个子库有自己的：
- `Cargo.toml`
- `src/lib.rs`
- 模块组织

### 3. boot 有自己的模块

boot 的模块组织在 `boot/src/` 下：
- 可以有多个模块
- 可以有 lib.rs (可选)
- main.rs 是入口点

## 检查清单

- [ ] 移动 `common/src/mmu/` 到 `common/mmu/src/`
- [ ] 创建 `common/mmu/src/types.rs`
- [ ] 更新 `common/mmu/src/lib.rs`
- [ ] 删除 `common/src/mmu/` 目录
- [ ] 简化 `common/Cargo.toml`
- [ ] 简化 `common/src/lib.rs`
- [ ] 优化 boot 模块组织
- [ ] 更新文档

## 总结

重构后的架构：
- ✅ common 是纯聚合库，无功能代码
- ✅ 每个子库是独立的 Cargo 项目
- ✅ boot 有清晰的模块组织
- ✅ 依赖关系清晰明确

这样保证了良好的模块化和可维护性！
