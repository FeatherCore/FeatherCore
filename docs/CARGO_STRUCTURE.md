# FeatherCore OS Cargo 项目管理机制

## Cargo 项目类型

### 1. Library (库) - `lib.rs`

**特点**:
- 编译为 `.rlib` 静态库
- 提供 API 供其他 crate 使用
- **不能直接执行**
- 可以导出函数、结构体、trait 等

**示例**: `common`

```toml
# common/Cargo.toml
[package]
name = "feathercore-common"

[lib]
name = "feathercore_common"
path = "src/lib.rs"

[dependencies]
# ...
```

**编译产物**:
```
target/<target>/debug/libfeathercore_common.rlib
```

**使用方式**:
```rust
// 其他 crate 通过 Cargo.toml 依赖
[dependencies]
feathercore-common = { path = "../common" }

// 代码中使用
use feathercore_common::platform::PlatformManager;
```

---

### 2. Binary (二进制) - `main.rs`

**特点**:
- 编译为可执行文件
- 包含 `main` 函数或 `#[start]` 入口
- 可以直接运行 (或在嵌入式系统中作为固件)

**示例**: `boot`, `kernel`

```toml
# boot/Cargo.toml
[package]
name = "feathercore-boot"

[[bin]]
name = "feathercore-boot"
path = "src/main.rs"

[dependencies]
feathercore-common = { path = "../common" }
```

**编译产物**:
```
target/<target>/debug/feathercore-boot    # Linux/macOS
target/<target>/debug/feathercore-boot.exe # Windows
```

**入口点**:
```rust
// 标准 Rust 程序
fn main() -> ! {
    // ...
}

// 或嵌入式 no_std 程序
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // ...
}
```

---

### 3. Multiple Binaries (多个二进制) - rootfs 模式

**特点**:
- 一个 Cargo.toml 定义多个 `[[bin]]`
- 每个 binary 独立编译
- 共享相同的依赖和构建配置

**示例**: `rootfs`

```toml
# rootfs/Cargo.toml
[package]
name = "feathercore-rootfs"

# 依赖所有工具共享的库
[dependencies]
feathercore-common = { path = "../common" }
feathercore-kernel = { path = "../kernel" }

# 定义多个二进制
[[bin]]
name = "sh"
path = "src/shell/main.rs"
required-features = ["shell"]

[[bin]]
name = "ls"
path = "src/coreutils/ls.rs"
required-features = ["coreutils"]

[[bin]]
name = "cat"
path = "src/coreutils/cat.rs"
required-features = ["coreutils"]

# ... 更多工具

[features]
shell = []
coreutils = []
```

**编译产物**:
```bash
# 编译所有工具
cargo build --release --features "shell,coreutils"

# 生成:
target/<target>/release/feathercore-rootfs-sh
target/<target>/release/feathercore-rootfs-ls
target/<target>/release/feathercore-rootfs-cat
# ...
```

**编译单个工具**:
```bash
# 只编译 sh
cargo build --release --bin sh --features "shell"

# 只编译 ls
cargo build --release --bin ls --features "coreutils"
```

---

## FeatherCore 项目结构

```
FeatherCore/
├── common/                    # Library (库)
│   ├── Cargo.toml             # [lib]
│   └── src/
│       └── lib.rs             # 库的根
│
├── boot/                      # Binary (二进制)
│   ├── Cargo.toml             # [[bin]]
│   └── src/
│       └── main.rs            # boot 入口
│
├── kernel/                    # Library + Binary
│   ├── Cargo.toml             # [lib] + [[bin]]
│   └── src/
│       ├── lib.rs             # 内核库 (syscall, vfs, driver)
│       └── main.rs            # 内核入口 (kernel_main)
│
└── rootfs/                    # Multiple Binaries (多个二进制)
    ├── Cargo.toml             # 多个 [[bin]]
    └── src/
        ├── shell/
        │   └── main.rs        # sh 工具
        └── coreutils/
            ├── ls.rs          # ls 工具
            ├── cat.rs         # cat 工具
            └── ...            # 其他工具
```

---

## 编译流程详解

### 1. common (库)

```toml
# common/Cargo.toml
[package]
name = "feathercore-common"

[lib]
name = "feathercore_common"
path = "src/lib.rs"
```

**编译命令**:
```bash
cd common
cargo build --release
```

**产物**:
```
target/release/libfeathercore_common.rlib
```

**被依赖**:
- boot 依赖 common
- kernel 依赖 common
- rootfs 依赖 common

---

### 2. boot (单个二进制)

```toml
# boot/Cargo.toml
[package]
name = "feathercore-boot"

[[bin]]
name = "feathercore-boot"
path = "src/main.rs"

[dependencies]
feathercore-common = { path = "../common" }
```

**编译命令**:
```bash
cd boot
cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf
```

**产物**:
```
target/thumbv7em-none-eabihf/release/feathercore-boot
```

---

### 3. kernel (库 + 二进制)

**为什么 kernel 既是库又是二进制？**

- **库**: 提供 syscall、vfs、driver 等模块供 rootfs 使用
- **二进制**: 作为可执行的内核镜像

```toml
# kernel/Cargo.toml
[package]
name = "feathercore-kernel"

# 库配置 - 提供 API
[lib]
name = "feathercore_kernel"
path = "src/lib.rs"

# 二进制配置 - 可执行内核
[[bin]]
name = "feathercore-kernel"
path = "src/main.rs"

[dependencies]
feathercore-common = { path = "../common" }
```

**编译命令**:
```bash
# 编译库 (供 rootfs 使用)
cd kernel
cargo build --release --lib

# 编译二进制 (作为内核镜像)
cargo build --release --bin feathercore-kernel
```

**产物**:
```
target/release/libfeathercore_kernel.rlib     # 库
target/release/feathercore-kernel             # 二进制
```

---

### 4. rootfs (多个二进制)

```toml
# rootfs/Cargo.toml
[package]
name = "feathercore-rootfs"

[dependencies]
feathercore-common = { path = "../common" }
feathercore-kernel = { path = "../kernel" }  # 使用 kernel 的库

[[bin]]
name = "sh"
path = "src/shell/main.rs"
required-features = ["shell"]

[[bin]]
name = "ls"
path = "src/coreutils/ls.rs"
required-features = ["coreutils"]

# ... 其他工具
```

**编译命令**:
```bash
cd rootfs

# 编译所有工具
cargo build --release --features "shell,coreutils"

# 编译单个工具
cargo build --release --bin sh --features "shell"
cargo build --release --bin ls --features "coreutils"
```

**产物**:
```
target/release/feathercore-rootfs-sh
target/release/feathercore-rootfs-ls
target/release/feathercore-rootfs-cat
# ...
```

---

## 依赖关系图

```
                    ┌──────────────┐
                    │   common     │
                    │   (lib)      │
                    └──────┬───────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 │
┌──────────────┐   ┌──────────────┐          │
│    boot      │   │   kernel     │          │
│   (bin)      │   │  (lib+bin)   │          │
│              │   │              │          │
│ 只依赖 common │   │ 依赖 common  │          │
│              │   │              │          │
└──────────────┘   └──────┬───────┘          │
                         │ (库 API)           │
                         │                  │
                         ▼                  │
                  ┌──────────────┐          │
                  │   rootfs     │◄─────────┘
                  │  (多 binary)  │
                  │              │
                  │ 依赖 common  │
                  │ 和 kernel 库  │
                  └──────────────┘
```

---

## 关键概念

### 1. **为什么 kernel 既是 lib 又是 bin？**

**原因**:
- **作为 bin**: kernel 需要作为可执行镜像被 boot 加载
- **作为 lib**: kernel 需要导出 syscall、vfs 等 API 供 rootfs 使用

**实现**:
```toml
# kernel/Cargo.toml
[lib]  # 库模式 - 提供 API
name = "feathercore_kernel"
path = "src/lib.rs"

[[bin]]  # 二进制模式 - 可执行
name = "feathercore-kernel"
path = "src/main.rs"
```

---

### 2. **为什么 rootfs 使用多 binary 模式？**

**原因**:
1. **共享依赖**: 所有工具共享 common 和 kernel 依赖
2. **统一构建**: 一个 Cargo.toml 管理所有工具
3. **独立编译**: 可以单独编译每个工具
4. **代码复用**: 工具间可以共享代码

**对比**:
```rust
// ❌ 方案 1: 每个工具一个 Cargo 项目
rootfs/
├── sh/Cargo.toml
├── ls/Cargo.toml
└── cat/Cargo.toml
// 问题：依赖重复，构建复杂

// ✅ 方案 2: 多 binary 模式
rootfs/
├── Cargo.toml      # 一个配置文件
└── src/
    ├── sh/main.rs
    ├── ls/main.rs
    └── cat/main.rs
// 优点：依赖共享，构建简单
```

---

### 3. **Feature 系统的作用**

**为什么使用 feature？**

```toml
# rootfs/Cargo.toml
[features]
shell = []
coreutils = []

[[bin]]
name = "sh"
required-features = ["shell"]

[[bin]]
name = "ls"
required-features = ["coreutils"]
```

**好处**:
1. **可选编译**: 可以选择性编译某些工具
2. **减少体积**: 不需要的工具不会编译
3. **灵活配置**: 根据不同板卡选择不同工具集

**使用**:
```bash
# 只编译 shell
cargo build --features "shell"

# 只编译 coreutils
cargo build --features "coreutils"

# 编译所有
cargo build --features "shell,coreutils"
```

---

## 完整构建命令

### 构建所有组件

```bash
# 1. 构建 common (库)
cd common
cargo build --release

# 2. 构建 boot (二进制)
cd ../boot
cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf

# 3. 构建 kernel (库 + 二进制)
cd ../kernel
cargo build --release --lib  # 库 (供 rootfs 使用)
cargo build --release --bin feathercore-kernel  # 二进制 (内核镜像)

# 4. 构建 rootfs (多个二进制)
cd ../rootfs
cargo build --release --features "shell,coreutils"
```

### 构建产物

```
target/
├── release/
│   ├── libfeathercore_common.rlib          # common 库
│   ├── libfeathercore_kernel.rlib          # kernel 库
│   └── feathercore-rootfs-sh               # rootfs 工具
│   └── feathercore-rootfs-ls
│   └── ...
│
└── thumbv7em-none-eabihf/
    └── release/
        ├── feathercore-boot                # boot 二进制
        └── feathercore-kernel              # kernel 二进制
```

---

## 总结

### Cargo 项目类型对比

| 项目     | 类型         | 产物          | 用途               |
|----------|--------------|---------------|--------------------|
| common   | Library      | `.rlib`       | 提供公共 API       |
| boot     | Binary       | 可执行文件    | Bootloader         |
| kernel   | Lib + Binary | `.rlib` + bin | 内核库 + 内核镜像  |
| rootfs   | 多 Binary    | 多个 bin      | 用户空间工具集     |

### 依赖关系

```
common (lib)
  ├─ boot (bin)
  ├─ kernel (lib + bin)
  └─ rootfs (多 bin) → 依赖 kernel (lib)
```

### 关键设计

1. **common**: 纯库，提供公共功能
2. **boot**: 纯二进制，只依赖 common
3. **kernel**: 库 + 二进制，库供 rootfs 使用
4. **rootfs**: 多二进制，每个工具独立但共享依赖

这种设计保证了清晰的职责分离和灵活的构建配置！
