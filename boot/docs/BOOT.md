# FeatherCore Bootloader

FeatherCore OS 的引导加载程序，参考 U-Boot 的初始化流程设计。

## 架构设计

### 启动流程

```
+------------------+
|  CPU Reset       |
+------------------+
         |
         v
+------------------+
|  Vector Table    |  <- 中断向量表 (ARM Cortex-M)
+------------------+
         |
         v
+------------------+
|  _start (ASM)    |  <- 设置栈指针
+------------------+
         |
         v
+------------------+
|  boot_main (Rust)|  <- 主引导函数
+------------------+
         |
    +----+----+
    |         |
    v         v
+-------+ +-------+
| Clock | | GPIO  |  <- 硬件初始化
+-------+ +-------+
    |         |
    v         v
+-------+ +-------+
| USART | | DRAM  |  <- 串口、内存初始化
+-------+ +-------+
    |
    v
+------------------+
|  Load Kernel     |  <- 从 FLASH 加载内核
+------------------+
         |
         v
+------------------+
|  Jump to Kernel  |  <- 跳转到内核
+------------------+
```

### 目录结构

```
boot/
├── Cargo.toml              # Cargo 配置
├── Cargo.lock              # 依赖锁定
├── stm32f429.ld           # 链接脚本 (STM32F429)
├── src/
│   ├── main.rs            # 主入口（平台选择）
│   └── stm32f429_main.rs  # STM32F429 特定实现
└── docs/
    └── BOOT.md            # 本文档
```

## 支持的平台

### STM32F429 (Cortex-M4F)

- **开发板**: STM32F429I-DISCO
- **Flash**: 2MB
- **SRAM**: 192KB + 64KB CCM
- **时钟**: 168MHz (HSE + PLL)

### 其他平台（待实现）

- STM32H7S78-DK (Cortex-M7)
- STM32N6570-DK (Cortex-M85)
- ESP32-C3 (RISC-V)
- ESP32-C6 (RISC-V)

## 内存布局

### STM32F429

```
+------------------+ 0x08000000
| Bootloader       |
| (256KB)          |
+------------------+ 0x08040000
| Kernel           |
| (1MB)            |
+------------------+ 0x08140000
| Filesystem       |
| (1MB)            |
+------------------+ 0x08200000

+------------------+ 0x20000000
| SRAM             |
| (192KB)          |
| - Boot (16KB)    |
| - Kernel (128KB) |
| - Heap (48KB)    |
+------------------+

+------------------+ 0x10000000
| CCM SRAM         |
| (64KB)           |
+------------------+
```

## 构建

### 前置要求

```bash
# 安装 Rust 工具链
rustup install nightly
rustup default nightly
rustup target add thumbv7em-none-eabihf

# 安装 LLVM 工具链（用于 objcopy）
rustup component add llvm-tools-preview
```

### 编译

```bash
# 编译 STM32F429 版本
cd boot
cargo build --release --features stm32f429i-disc

# 生成二进制文件
cargo objcopy --release -- -O binary feathercore-boot.bin
cargo objcopy --release -- -O ihex feathercore-boot.hex
```

### 烧录

```bash
# 使用 OpenOCD 烧录
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg \
  -c "program feathercore-boot.bin 0x08000000 verify reset exit"

# 或使用 ST-Link Utility
st-flash write feathercore-boot.bin 0x08000000
```

## 平台特定实现

### 时钟初始化

```rust
// STM32F429: HSE (8MHz) -> PLL -> 168MHz
unsafe fn clock_init() {
    // 1. 使能 HSE
    RCC.cr |= HSEON;
    while RCC.cr & HSERDY == 0 {}
    
    // 2. 配置 PLL
    RCC.pllcfgr = PLLM_8 | PLLN_336 | PLLP_2 | PLLSRC_HSE;
    
    // 3. 使能 PLL
    RCC.cr |= PLLON;
    while RCC.cr & PLLRDY == 0 {}
    
    // 4. 选择 PLL 作为系统时钟
    RCC.cfgr |= SW_PLL;
    while RCC.cfgr & SWS != SWS_PLL {}
}
```

### GPIO 初始化

```rust
// STM32F429: LED (PD12-PD15)
unsafe fn gpio_init() {
    // 使能 GPIOD 时钟
    RCC.ahb1enr |= GPIODEN;
    
    // 配置 PD12-PD15 为输出模式
    GPIOD.moder |= 0b01010101 << 24;
}
```

### 串口初始化

```rust
// STM32F429: USART1 @ 115200
unsafe fn usart_init() {
    // 使能 USART1 时钟
    RCC.apb2enr |= USART1EN;
    
    // 配置波特率：168MHz / (16 * 115200) = 91
    USART1.brr = 91;
    
    // 使能 TX, RX, UE
    USART1.cr1 = TE | RE | UE;
}
```

## 内核加载

### 内核位置

```
Flash: 0x08040000 (Bootloader 之后)
加载到：0x20000000 (SRAM 起始)
```

### 加载流程

```rust
unsafe fn load_kernel() {
    const KERNEL_FLASH: *const u8 = 0x08040000 as *const u8;
    const KERNEL_RAM: *mut u8 = 0x20000000 as *mut u8;
    const KERNEL_SIZE: usize = 64 * 1024;
    
    // 从 FLASH 复制到 SRAM
    ptr::copy_nonoverlapping(KERNEL_FLASH, KERNEL_RAM, KERNEL_SIZE);
}
```

### 跳转到内核

```rust
unsafe fn jump_to_kernel() {
    const KERNEL_ENTRY: *const () = 0x20000000 as *const ();
    
    // 获取内核的 MSP 和复位向量
    let kernel_msp = ptr::read_volatile(KERNEL_ENTRY as *const u32);
    let kernel_reset = ptr::read_volatile((KERNEL_ENTRY as *const u32).add(1));
    
    // 设置 MSP 并跳转
    asm!("msr msp, {}", in(reg) kernel_msp);
    asm!("bx {}", in(reg) kernel_reset);
}
```

## 调试

### 串口输出

Bootloader 通过 USART1 输出调试信息：

```
FeatherCore Boot
STM32F429 Discovery
GPIO initialized
LED test...
Loading kernel...
Kernel loaded to 0x20000000
Jumping to kernel...
Kernel MSP: 0x20003000
Kernel Reset: 0x08040141
Jumping...
```

### Panic 处理

发生错误时：

1. 通过串口输出 panic 信息
2. LED 快速闪烁指示错误
3. 系统停机

## 参考

### U-Boot 相关文件

- `board/st/stm32f429-discovery/stm32f429-discovery.c`
- `board/st/stm32f429-discovery/led.c`
- `arch/arm/cpu/armv7m/start.S`
- `arch/arm/mach-stm32f4/clock.c`

### STM32F429 参考手册

- RM0090: STM32F405/415, STM32F407/417, STM32F427/437, STM32F429/439
- 时钟树图：Figure 10
- 内存映射：Figure 1

## 开发计划

- [x] STM32F429 基本启动
- [x] 时钟初始化
- [x] GPIO 初始化
- [x] 串口调试输出
- [x] 内核加载框架
- [ ] 内核跳转实现
- [ ] 设备树支持
- [ ] 多平台支持
- [ ] 网络启动（TFTP）
- [ ] SD 卡启动
- [ ] USB DFU 升级

## 许可证

MIT OR Apache-2.0
