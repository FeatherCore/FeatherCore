# rootfs 的 Cargo 定位

## 问题：rootfs 是什么？

**答案**: rootfs 是一个 **Multiple Binaries (多二进制)** Cargo 项目

## Cargo 项目类型对比

| 组件     | 类型                | 源码         | 编译产物                  | 用途               |
|----------|---------------------|--------------|---------------------------|--------------------|
| common   | Library             | `src/lib.rs` | `.rlib` 库文件            | 提供公共 API       |
| boot     | Binary              | `src/main.rs` | 可执行文件              | Bootloader         |
| kernel   | Library + Binary    | `src/lib.rs` + `src/main.rs` | `.rlib` + 可执行文件 | 内核库 + 内核镜像  |
| **rootfs** | **Multiple Binaries** | **多个 `src/*/main.rs`** | **多个可执行文件** | **用户空间工具集** |

## rootfs 的多 Binary 模式

### Cargo.toml 配置

```toml
[package]
name = "feathercore-rootfs"

# 依赖共享库
[dependencies]
feathercore-common = { path = "../common" }
feathercore-kernel = { path = "../kernel" }  # 使用 kernel 的库

# 定义多个二进制工具
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
default = ["shell", "coreutils"]
shell = []
coreutils = []
```

### 编译产物

```bash
# 编译所有工具
cargo build --release --features "shell,coreutils"

# 生成 12 个独立的可执行文件:
target/<target>/release/
├── feathercore-rootfs-sh      # Shell
├── feathercore-rootfs-ls      # 列出目录
├── feathercore-rootfs-cat     # 显示文件
├── feathercore-rootfs-echo    # 输出文本
├── feathercore-rootfs-cd      # 切换目录
├── feathercore-rootfs-pwd     # 显示路径
├── feathercore-rootfs-mkdir   # 创建目录
├── feathercore-rootfs-rm      # 删除文件
├── feathercore-rootfs-cp      # 复制文件
├── feathercore-rootfs-mv      # 移动文件
├── feathercore-rootfs-ps      # 显示进程
└── feathercore-rootfs-mount   # 挂载文件系统
```

### 为什么使用多 Binary 模式？

#### ✅ 优点

1. **共享依赖**
   - 所有工具共享 `feathercore-common` 和 `feathercore-kernel`
   - 避免重复定义依赖

2. **统一构建**
   - 一个 `Cargo.toml` 管理所有工具
   - 统一的构建配置和目标平台

3. **独立编译**
   ```bash
   # 可以单独编译某个工具
   cargo build --release --bin sh --features "shell"
   cargo build --release --bin ls --features "coreutils"
   ```

4. **代码复用**
   - 工具间可以共享辅助函数
   - 可以创建公共的 `src/lib.rs` (如果需要)

#### ❌ 对比：如果每个工具一个 Cargo 项目

```
rootfs/
├── sh/
│   ├── Cargo.toml          # 重复定义依赖
│   └── src/main.rs
├── ls/
│   ├── Cargo.toml          # 重复定义依赖
│   └── src/main.rs
└── cat/
    ├── Cargo.toml          # 重复定义依赖
    └── src/main.rs

# 问题:
# - 每个项目都要定义相同的依赖
# - 构建配置分散
# - 难以统一管理
```

## 构建命令

### 1. 构建所有组件

```bash
# common (库)
cd common && cargo build --release

# boot (二进制)
cd boot && cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf

# kernel (库 + 二进制)
cd kernel
cargo build --release --lib                    # 库 (供 rootfs 使用)
cargo build --release --bin feathercore-kernel # 二进制 (内核镜像)

# rootfs (多个二进制)
cd rootfs && cargo build --release --features "shell,coreutils"
```

### 2. rootfs 安装到文件系统

```bash
# 编译后，将工具复制到 rootfs 目录
cd rootfs
TARGET_DIR=target/<target>/release

# 安装到 bin/
cp $TARGET_DIR/feathercore-rootfs-sh ../../rootfs/bin/sh
cp $TARGET_DIR/feathercore-rootfs-ls ../../rootfs/bin/ls
cp $TARGET_DIR/feathercore-rootfs-cat ../../rootfs/bin/cat
# ... 其他工具
```

## 依赖关系

```
编译时依赖:

feathercore-common (lib)
    │
    ├─→ feathercore-boot (bin)
    │
    ├─→ feathercore-kernel (lib + bin)
    │       │
    │       └─ (lib API)
    │
    └─→ feathercore-rootfs (多 bin)
            │
            └─ 使用 kernel 的 syscall 接口
```

## rootfs 工具如何使用 kernel API

```rust
// rootfs/src/coreutils/ls.rs
#![no_std]
#![no_main]

use feathercore_kernel::syscall;  // 使用 kernel 提供的 syscall

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 通过 syscall 打开目录
    let fd = syscall::sys_open(b"/\0" as *const u8, 0, 0);
    
    // 通过 syscall 读取目录内容
    // ...
    
    loop {}
}
```

## 总结

### rootfs 的定位

| 属性       | 值                          |
|------------|-----------------------------|
| **类型**   | Multiple Binaries (多二进制) |
| **源码**   | `src/shell/main.rs`, `src/coreutils/*.rs` |
| **产物**   | 12 个独立的可执行文件        |
| **依赖**   | common (库) + kernel (库)   |
| **用途**   | 提供用户空间工具集           |

### 与其他组件的关系

```
boot (bin) ─┐
            ├─→ 运行时协作
kernel (bin)─┘
    │
    └─ (lib) ─→ rootfs (多 bin)
                   │
                   └─ 通过 syscall 使用 kernel 服务
```

### 关键特性

✅ **多二进制**: 一个 Cargo.toml 管理 12 个工具  
✅ **共享依赖**: 所有工具共享 common 和 kernel  
✅ **独立编译**: 可以单独编译每个工具  
✅ **no_std**: 所有工具都是 `#![no_std]`  
✅ **syscall 接口**: 通过 kernel 库访问系统调用  

这就是 rootfs 在 Cargo 项目结构中的定位！
