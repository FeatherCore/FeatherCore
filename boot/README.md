# FeatherCore Boot 构建说明

## 使用 FeatherCore 构建工具

**重要**: Boot 必须使用 FeatherCore 构建工具进行构建，该工具会自动生成链接脚本和设备树配置。

### 前置要求

```bash
# 1. 安装 Rust nightly 工具链
rustup install nightly
rustup default nightly

# 2. 安装目标架构
rustup target add thumbv7em-none-eabihf  # STM32F429 (Cortex-M4F)

# 3. 安装 LLVM 工具（用于生成二进制文件）
rustup component add llvm-tools-preview

# 4. 安装构建工具
cd /home/uan/develop/FeatherCore_v01/FeatherCore/build_tool
cargo install --path .
```

### 构建流程

#### 1. 查看支持的板卡

```bash
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore list-boards
```

#### 2. 查看特定板卡信息

```bash
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore show-board stm32f429i-disc
```

输出示例：
```
Board: stm32f429i-disc
====================
Chip: stm32f4 (STMicroelectronics)
CPU: cortex-m4 (FPU: enabled)
Clock: 168000000 Hz
Flash: 2048 KB @ 0x08000000
RAM: 192 KB @ 0x20000000
Boot: 256 KB @ 0x08000000
Kernel: 1024 KB @ 0x08040000
Target: thumbv7em-none-eabihf
```

#### 3. 生成配置文件

```bash
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore generate stm32f429i-disc
```

这将生成：
- `boot/link.x` - 链接脚本
- `kernel/link.x` - 内核链接脚本
- `common/generated/src/devicetree.rs` - 设备树信息

#### 4. 构建 Boot

```bash
# 构建 STM32F429 版本
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore build stm32f429i-disc boot

# 或构建所有（boot + kernel）
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore build stm32f429i-disc all
```

#### 5. 生成二进制文件

```bash
cd boot
cargo objcopy --release -- -O binary feathercore-boot.bin
cargo objcopy --release -- -O ihex feathercore-boot.hex
```

#### 6. 烧录

```bash
# 使用 OpenOCD
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg \
  -c "program feathercore-boot.bin 0x08000000 verify reset exit"

# 或使用 st-flash
st-flash write feathercore-boot.bin 0x08000000
```

## 板级配置

### STM32F429I-DISC

配置文件：`/home/uan/develop/FeatherCore_v01/FeatherCore/platform/board/stm32/stm32f429i-disc/stm32f429i-disc_defconfig.toml`

```toml
[board]
name = "stm32f429i-disc"
vendor = "st"
family = "stm32f4"
mcu = "stm32f429vgt6"

[config]
arm_mpu = true
hw_stack_protection = true
serial = true
console = true
uart_console = true

[pinout]
led3 = "PG13"
led4 = "PG14"
user_button = "PA0"
dbg_uart_tx = "PA9"
dbg_uart_rx = "PA10"
```

## 内存布局

构建工具会根据板级配置自动生成链接脚本。

### STM32F429 内存布局

```
+------------------+ 0x08000000
| Bootloader       | 256KB
+------------------+ 0x08040000
| Kernel           | 1MB
+------------------+ 0x08140000
| Filesystem       | 1MB
+------------------+ 0x08200000

+------------------+ 0x20000000
| SRAM             | 192KB
+------------------+

+------------------+ 0x10000000
| CCM SRAM         | 64KB
+------------------+
```

## 清理构建

```bash
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore clean
```

## 调试

### 串口输出

Boot 通过 USART1 (PA9/PA10) 输出调试信息：
- 波特率：115200
- 数据位：8
- 停止位：1
- 校验位：无

### 查看输出

```bash
# Linux
screen /dev/ttyUSB0 115200
# 或
minicom -D /dev/ttyUSB0 -b 115200

# Windows
PuTTY -> Serial -> COMx -> 115200
```

## 常见问题

### 1. 编译错误 "cannot find macro `asm`"

确保使用了 nightly 工具链：
```bash
rustup default nightly
```

### 2. 链接错误 "undefined reference"

确保先运行 generate 生成链接脚本：
```bash
feathercore-build -r /home/uan/develop/FeatherCore_v01/FeatherCore generate stm32f429i-disc
```

### 3. 烧录失败

检查 ST-Link 连接：
```bash
st-info --probe
```

## 参考

- [构建工具源码](../build_tool/)
- [板级配置](../platform/board/stm32/stm32f429i-disc/)
- [Boot 文档](docs/BOOT.md)
