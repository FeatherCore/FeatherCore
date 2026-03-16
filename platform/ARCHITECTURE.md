# FeatherCore Platform 架构说明

## 目录结构

```
platform/
├── Cargo.toml                 # Workspace 配置
├── src/
│   └── lib.rs                 # Meta-crate 入口（只重新导出）
├── board/                     # 板级子库
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs             # Board crate 入口
│   └── stm32/                 # STM32 模块
│       ├── stm32f429i-disc/   # 具体板级模块
│       │   └── mod.rs
│       ├── stm32h7s78-dk/
│       │   └── mod.rs
│       └── stm32n6570-dk/
│           └── mod.rs
└── chip/                      # 芯片级子库
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs             # Chip crate 入口
    └── stm32/                 # STM32 模块
        ├── stm32f4/           # 具体芯片模块
        │   ├── mod.rs
        │   ├── rcc.rs
        │   └── gpio.rs
        ├── stm32h7rs/
        │   └── mod.rs
        └── stm32n6/
            └── mod.rs
```

## 架构层次

### 1. Platform (总库 - Meta-crate)
- **位置**: `platform/`
- **功能**: 无实际功能，仅重新导出 board 和 chip 子库
- **依赖**: `feathercore-platform-board`, `feathercore-platform-chip`
- **Features**: 提供 convenience features 如 `stm32f429i-disc`

### 2. Board (子库)
- **位置**: `platform/board/`
- **功能**: 提供板级初始化代码
- **结构**: 
  - `board/stm32/` - STM32 板级模块
  - `board/stm32/stm32f429i-disc/` - 具体板级模块
- **Features**: 每个板级一个 feature

### 3. Chip (子库)
- **位置**: `platform/chip/`
- **功能**: 提供芯片级驱动和初始化
- **结构**:
  - `chip/stm32/` - STM32 芯片模块
  - `chip/stm32/stm32f4/` - 具体芯片模块
- **Features**: 每个芯片系列一个 feature

### 4. Modules (模块)
- **位置**: `board/stm32/stm32f429i-disc/`, `chip/stm32/stm32f4/` 等
- **功能**: 实际的代码实现
- **形式**: `.rs` 文件或带 `mod.rs` 的目录

## 使用方式

### 在 Cargo.toml 中依赖

```toml
[dependencies]
feathercore-platform = { path = "platform", features = ["stm32f429i-disc"] }
```

### 在代码中使用

```rust
#![no_std]

use feathercore_platform::board::stm32::stm32f429i_disc;
use feathercore_platform::chip::stm32::stm32f4;

fn main() {
    // Initialize chip first
    stm32f4::init();
    
    // Then initialize board
    stm32f429i_disc::init();
    
    // Access board information
    let board_name = stm32f429i_disc::info::BOARD_NAME;
    
    // Access chip information
    let cpu_core = stm32f4::info::CPU_CORE;
}
```

## Features 说明

### Board Features
- `board-stm32f429i-disc` - STM32F429I-DISCO 板
- `board-stm32h7s78-dk` - STM32H7S78-DK 板
- `board-stm32n6570-dk` - STM32N6570-DK 板

### Chip Features
- `chip-stm32f4` - STM32F4 系列芯片
- `chip-stm32h7rs` - STM32H7RS 系列芯片
- `chip-stm32n6` - STM32N6 系列芯片

### Convenience Features
- `stm32f429i-disc` = `board-stm32f429i-disc` + `chip-stm32f4`
- `stm32h7s78-dk` = `board-stm32h7s78-dk` + `chip-stm32h7rs`
- `stm32n6570-dk` = `board-stm32n6570-dk` + `chip-stm32n6`

## 与 common/platform 的关系

`common/platform/` 是适配层，根据编译配置引用 `platform/` 的内容：

```toml
# common/platform/Cargo.toml
[dependencies]
feathercore-platform = { path = "../../platform", features = ["stm32f429i-disc"] }
feathercore-common = { path = ".." }
```

```rust
// common/platform/src/lib.rs
pub use feathercore_platform as platform;

// 重新导出给 boot/kernel 使用
pub use platform::board;
pub use platform::chip;
```

## 优势

1. ✅ **清晰的层次**: platform → board/chip → stm32 → specific
2. ✅ **模块化**: 每个板级/芯片都是独立的模块
3. ✅ **可复用**: chip 代码可以被多个 board 共享
4. ✅ **灵活性**: 通过 Cargo features 选择配置
5. ✅ **no_std**: 所有代码都是 no_std
6. ✅ **易于维护**: 清晰的目录结构和职责分离

## 添加新板级的步骤

1. 在 `platform/board/stm32/` 创建新目录：
   ```bash
   mkdir platform/board/stm32/stm32new-board
   ```

2. 创建 `mod.rs` 文件：
   ```rust
   //! STM32NEW Board Support
   
   #![no_std]
   
   pub fn init() {
       // Initialization code
   }
   
   pub mod info {
       pub const BOARD_NAME: &str = "STM32NEW-BOARD";
   }
   ```

3. 在 `platform/board/src/lib.rs` 中添加模块导出：
   ```rust
   pub mod stm32 {
       #[cfg(feature = "stm32new-board")]
       pub mod stm32new_board;
   }
   ```

4. 在 `platform/board/Cargo.toml` 中添加 feature：
   ```toml
   [features]
   stm32new-board = []
   ```

5. 在 `platform/Cargo.toml` 中添加 convenience feature：
   ```toml
   [features]
   stm32new-board = ["board-stm32new-board", "chip-stm32xxx"]
   ```

## 总结

Platform 架构采用三层设计：
- **总库** (platform/) - 提供统一入口
- **子库** (board/, chip/) - 组织代码
- **模块** (stm32/, stm32f429i-disc/) - 实际实现

这种设计既保持了清晰的层次结构，又提供了灵活的配置方式。
