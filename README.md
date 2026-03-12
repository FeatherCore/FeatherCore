# FeatherCore

A Rust-based Unix-like Real-Time Operating System (RTOS) designed for embedded systems.

## 概述 / Overview

FeatherCore 是一个完全使用 Rust 编写的嵌入式实时操作系统（RTOS），遵循模块化设计原则。

FeatherCore is a Rust-based Real-Time Operating System (RTOS) designed for embedded systems, following modular design principles.

## 核心特性 / Features

- **Rust 实现 / Rust Implementation**: 完全使用 Rust 编写，保证内存安全和性能
- **类 Unix API / Unix-like API**: 提供熟悉的 Unix 系统调用和抽象
- **实时能力 / Real-Time Capabilities**: 抢占式调度，支持可配置优先级
- **多架构支持 / Multi-Architecture Support**:
  - ARMv7-M (Cortex-M3/M4/M7)
  - ARMv8-M (Cortex-M23/M33/M55/M85)
  - RISC-V (RV32IMAC)
- **内存管理 / Memory Management**: 动态内存分配，支持可配置的分配器
- **虚拟文件系统 / Virtual File System (VFS)**: 支持多种文件系统
- **POSIX 兼容 / POSIX Compliance**: 部分 POSIX 标准兼容

## 架构说明 / Architecture

### 构建系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                     Host System (x86_64)                    │
│                                                              │
│  ┌─────────────────┐                                         │
│  │   build_tool   │  主机工具 (std, 依赖主机平台)            │
│  │  (Rust stable) │  Host tool (std, host-dependent)       │
│  └────────┬────────┘                                         │
│           │                                                  │
│           │ 解析 / Parse                                     │
│           ▼                                                  │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              platform/ (配置文件目录)                    │ │
│  │   board/    - 板级配置文件 (*_defconfig.toml)           │ │
│  │   chip/     - 芯片配置文件 (*_defconfig.toml)            │ │
│  │   *.dts     - 设备树文件 (.toml 格式)                    │ │
│  └─────────────────────┬───────────────────────────────────┘ │
│                        │ 生成 / Generate                     │
│                        ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              generated/ (生成代码目录)                    │ │
│  │   devicetree.rs - 设备树解析代码                         │ │
│  └─────────────────────────────────────────────────────────┘ │
│                        │                                     │
│           ┌───────────┴───────────┐                         │
│           ▼                       ▼                          │
│  ┌──────────────────┐   ┌──────────────────┐                 │
│  │  boot/link.x     │   │ kernel/link.x   │                 │
│  │  (链接脚本)       │   │  (链接脚本)       │                 │
│  └────────┬─────────┘   └────────┬─────────┘                 │
└───────────┼──────────────────────┼──────────────────────────┘
            │ 编译 / Compile        │ 编译 / Compile
            ▼                       ▼
┌─────────────────────────────────────────────────────────────┐
│                   Target System (Embedded)                  │
│                                                              │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │    boot     │   │   kernel    │   │   common    │       │
│  │ (no_std)    │   │ (no_std)    │   │  (no_std)   │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    arch/ (架构层)                        ││
│  │  arm/      - ARM Cortex-M 汇编和初始化代码               ││
│  │  riscv/    - RISC-V 汇编和初始化代码                     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 目录结构 / Project Structure

```
FeatherCore/
├── platform/                    # 平台配置文件目录 (非 Cargo 管理)
│   │                            # Platform config directory (not Cargo managed)
│   ├── board/                  # 板级配置文件
│   │   │                        # Board configuration files
│   │   ├── stm32/             # STMicroelectronics STM32 系列
│   │   │   ├── stm32f429i-disc/
│   │   │   │   ├── stm32f429i-disc_defconfig.toml
│   │   │   │   └── stm32f429i-disc.dts
│   │   │   ├── stm32h7s78-dk/
│   │   │   ├── stm32n6570-dk/
│   │   │   ├── stm32u5a9j-dk/
│   │   │   └── stm32u5g9j-dk1/
│   │   ├── esp/                # Espressif ESP32 系列
│   │   │   ├── esp32-c3-devkitc/
│   │   │   ├── esp32-c5-devkitc/
│   │   │   └── esp32-c6-devkitm/
│   │   ├── nxp/                # NXP 系列
│   │   │   ├── mimxrt1170-evkb/
│   │   │   └── frdm-rw612/
│   │   └── renesas/            # Renesas RA 系列
│   │       └── ek-ra8p1/
│   │
│   └── chip/                   # 芯片通用配置文件
│       │                        # Chip common configuration files
│       ├── stm32/              # STM32 芯片系列
│       │   ├── stm32f4/
│       │   │   ├── stm32f4_defconfig.toml
│       │   │   └── stm32f4.dts
│       │   ├── stm32h7rs/
│       │   ├── stm32n6/
│       │   └── stm32u5/
│       ├── esp/                # ESP 芯片系列
│       │   ├── esp32c3/
│       │   ├── esp32c5/
│       │   └── esp32c6/
│       ├── nxp/                # NXP 芯片系列
│       │   ├── imxrt1170/
│       │   └── rw612/
│       └── renesas/            # Renesas 芯片系列
│           └── ra8/
│
├── build_tool/                  # 构建工具项目 (Cargo 管理, std)
│   │                            # Build tool project (Cargo managed, std)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs            # 主入口
│   │   ├── config.rs          # 配置解析模块
│   │   ├── linker.rs          # 链接脚本生成模块
│   │   ├── device_tree.rs     # 设备树生成模块
│   │   └── root_path.rs       # 路径管理模块
│   └── scripts/                # 构建脚本
│
├── common/                      # 公共库 (Cargo 管理, no_std)
│   │                            # Common library (Cargo managed, no_std)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs             # 公共接口定义
│   │   ├── io.rs              # I/O 接口
│   │   ├── mem.rs             # 内存管理
│   │   ├── sync.rs            # 同步原语
│   │   ├── async.rs           # 异步运行时
│   │   ├── error.rs           # 错误类型
│   │   └── generated/         # 生成的代码
│   │       └── devicetree.rs  # 设备树解析代码
│   │
│   └── arch/                   # 架构相关代码
│       ├── arm/               # ARM 架构
│       │   ├── Cargo.toml
│       │   └── src/
│       │       └── lib.rs     # ARM 初始化和异常处理
│       └── riscv/             # RISC-V 架构
│           ├── Cargo.toml
│           └── src/
│               └── lib.rs     # RISC-V 初始化和异常处理
│
├── boot/                       # Bootloader (Cargo 管理, no_std)
│   │                            # Bootloader (Cargo managed, no_std)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs            # Bootloader 入口
│   ├── link.x.in              # 链接脚本模板
│   └── link.x                 # 生成的链接脚本
│
├── kernel/                     # Kernel (Cargo 管理, no_std)
│   │                            # Kernel (Cargo managed, no_std)
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs            # Kernel 入口
│   ├── link.x.in              # 链接脚本模板
│   └── link.x                 # 生成的链接脚本
│
└── docs/                       # 文档目录
    └── *.md
```

## 构建流程 / Build Process

### 构建顺序 / Build Order

```
1. build_tool (std)          ──解析──>  platform/
2. build_tool (std)          ──生成──>  common/src/generated/
3. build_tool (std)          ──生成──>  boot/link.x
4. build_tool (std)          ──生成──>  kernel/link.x
5. boot (no_std, target)     ──编译──>  boot.elf
6. kernel (no_std, target)   ──编译──>  kernel.elf
```

### 详细步骤 / Detailed Steps

#### 步骤 1: 编译构建工具 / Build Build Tool

```bash
# 进入构建工具目录
cd build_tool

# 编译构建工具（生成主机平台可执行文件）
cargo build --release

# 构建工具位于: target/release/feathercore-build
```

#### 步骤 2: 列出支持的开发板 / List Supported Boards

```bash
# 使用 -r 参数指定 FeatherCore 根目录
./target/release/feathercore-build -r /path/to/FeatherCore list-boards
```

输出示例:
```
Supported boards:
- stm32f429i-disc (STM32F429I Discovery)
- stm32h7s78-dk (STM32H7S78 Discovery Kit)
- stm32n6570-dk (STM32N6570 Discovery Kit)
- stm32u5a9j-dk (STM32U5A9J Discovery Kit)
- stm32u5g9j-dk1 (STM32U5G9J Discovery Kit)
- esp32-c3-devkitc (ESP32-C3 DevKitC)
- esp32-c5-devkitc (ESP32-C5 DevKitC)
- esp32-c6-devkitm (ESP32-C6 DevKitM)
- mimxrt1170-evkb (i.MX RT1170 EVKB)
- frdm-rw612 (FRDM-RW612)
- ek-ra8p1 (EK-RA8P1)
```

#### 步骤 3: 生成配置文件 / Generate Configuration

```bash
# 为指定开发板生成配置
./target/release/feathercore-build -r /path/to/FeatherCore generate <board-name>

# 示例
./target/release/feathercore-build -r /home/uan/develop/FeatherCore/FeatherCore generate stm32f429i-disc
```

生成的文件:
- `boot/link.x` - Boot 链接脚本
- `kernel/link.x` - Kernel 链接脚本
- `common/src/generated/devicetree.rs` - 设备树代码

#### 步骤 4: 构建 Boot 镜像 / Build Boot Image

```bash
# 进入 boot 目录
cd ../boot

# 编译 boot 镜像（需要指定目标平台）
# ARM Cortex-M4/M7
cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf

# ARM Cortex-M55/M85
cargo build --release --features stm32n6570-dk --target thumbv8m.main-none-eabi

# 输出: boot/target/<target>/release/feathercore-boot
```

#### 步骤 5: 构建 Kernel 镜像 / Build Kernel Image

```bash
# 进入 kernel 目录
cd ../kernel

# 编译 kernel 镜像
cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf

# 输出: kernel/target/<target>/release/feathercore-kernel
```

### 完整构建示例 / Complete Build Example

```bash
# 1. 编译构建工具
cd build_tool
cargo build --release
cd ..

# 2. 列出支持的开发板
./build_tool/target/release/feathercore-build -r $(pwd) list-boards

# 3. 为 stm32f429i-disc 生成配置
./build_tool/target/release/feathercore-build -r $(pwd) generate stm32f429i-disc

# 4. 构建 boot 镜像
cd boot
cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf

# 5. 构建 kernel 镜像
cd ../kernel
cargo build --release --features stm32f429i-disc --target thumbv7em-none-eabihf
```

## 设备树说明 / Device Tree Specification

### 设计原则 / Design Principles

设备树的核心是定义**硬件属性**，不定义驱动逻辑：

- **定义 / Defines**: 寄存器地址、中断号、时钟、兼容标识、GPIO 引脚等
- **不定义 / Does NOT Define**: 驱动逻辑、初始化顺序、具体操作流程

驱动代码通过「通用逻辑 + 适配层」处理不同 SoC 的外设操作差异，适配层由设备树的 `compatible` 属性触发。

### 配置层次 / Configuration Hierarchy

```
chip 配置 (通用)          board 配置 (定制)
┌─────────────────┐       ┌─────────────────┐
│ [chip]          │       │ chip = "xxx"    │
│ name = "stm32f4"│       │                 │
│ vendor = "st"   │       │ [board]         │
│                 │       │ name = "xxx"    │
│ [cpu]           │       │                 │
│ core = "cortex-m4"      │ [supported]     │
│ frequency = 180MHz      │ gpio = true     │
│                 │       │ uart = true     │
│ [memory]        │       │                 │
│ flash = 2MB     │       │ [pinout]        │
│ sram = 192KB    │       │ led1 = "PG13"   │
│                 │       │ uart_tx = "PA9" │
│ [bootloader]    │       │                 │
│ size = 64KB     │       └─────────────────┘
└─────────────────┘              ▲
        │                       │
        │  继承 / Inherit       │
        │  覆盖 / Override     │
        └───────────────────────┘
```

### 设备树文件格式 / Device Tree File Format

设备树文件使用 `.toml` 格式（而非 Linux 的 `.dts`），保证语法一致性：

```toml
// 设备树示例 / Device Tree Example
// STM32F4 设备树配置

/ {
    compatible = "st,stm32f405", "st,stm32f4";

    // 时钟定义 / Clock definitions
    clocks {
        #address-cells = <1>;
        #size-cells = <0>;

        hsi: hsi@0 {
            compatible = "st,stm32f4-hsi";
            clock-frequency = <16000000>;
        };
    };

    // GPIO 端口定义 / GPIO port definitions
    gpioa: gpio@40020000 {
        compatible = "st,stm32f4-gpio";
        reg = <0x40020000 0x400>;
        interrupts = <0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15>;
    };

    // 串口定义 / UART definitions
    usart1: serial@40011000 {
        compatible = "st,stm32f4-usart";
        reg = <0x40011000 0x400>;
        interrupts = <37>;
        clocks = <&rcc 0>;
    };

    // I2C 定义 / I2C definitions
    i2c1: i2c@40005400 {
        compatible = "st,stm32f4-i2c";
        reg = <0x40005400 0x400>;
        interrupts = <31 32>;
        clocks = <&rcc 0>;
    };

    // SPI 定义 / SPI definitions
    spi1: spi@40013000 {
        compatible = "st,stm32f4-spi";
        reg = <0x40013000 0x400>;
        interrupts = <35 36>;
        clocks = <&rcc 0>;
    };
};
```

## 依赖要求 / Dependencies

### 必须 / Required

- **Rust 工具链**: stable 版本
- **嵌入式目标**: 根据目标芯片安装对应 target

```bash
# 安装 Rust
rustup default stable

# 安装嵌入式目标
rustup target add thumbv7em-none-eabihf   # ARM Cortex-M4/M7
rustup target add thumbv8m.main-none-eabi # ARM Cortex-M55/M85
rustup target add riscv32imac-unknown-none-elf  # RISC-V
```

### 可选 / Optional

- **ARM GCC**: 用于生成最终二进制文件
- **QEMU**: 用于模拟运行

## 代码规范 / Coding Standards

### 安全原则 / Safety Principles

1. **禁止外部依赖**: 不依赖任何外部 crates（除了 Rust 标准库）
2. **避免 unsafe**: 仅在底层寄存器访问和必要场景使用 unsafe
3. **架构代码集中**: 汇编代码和架构相关代码集中在 `arch/` 目录

### 注释规范 / Comment Standards

- 所有接口代码需提供中英文注释
- 配置文件需提供中英文说明
- 链接脚本模板需提供注释说明每个段的作用

## 许可 / License

FeatherCore 采用 MIT 或 Apache 2.0 双重许可。

FeatherCore is dual-licensed under the MIT License and Apache License 2.0.

## 贡献 / Contributing

欢迎贡献代码！请查看 Contributing Guide 获取更多信息。

Contributions are welcome! Please see the Contributing Guide for more information.
