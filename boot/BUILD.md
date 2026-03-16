# FeatherCore Boot 构建指南

## 快速开始

### 1. 构建 Boot

```bash
cd /home/uan/develop/FeatherCore_v01/FeatherCore
feathercore-build -r $(pwd) build stm32f429i-disc boot
```

### 2. 生成二进制

```bash
cd boot
cargo objcopy --release -- -O binary feathercore-boot.bin
```

### 3. 烧录

```bash
st-flash write target/thumbv7em-none-eabihf/release/feathercore-boot.bin 0x08000000
```

## 完整文档

详细文档请查看 [README.md](README.md)
