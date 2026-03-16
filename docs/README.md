# FeatherCore OS 快速开始指南

## 快速开始

### 1. 环境准备

```bash
# 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装嵌入式目标
rustup target add thumbv7em-none-eabihf   # ARM Cortex-M4/M7
rustup target add thumbv8m.main-none-eabi # ARM Cortex-M55/M85
rustup target add riscv32imac-unknown-none-elf  # RISC-V
```

### 2. 构建系统

#### 完整构建 (推荐)
```bash
cd FeatherCore

# 为 STM32F429I 开发板构建
./build.sh build-all stm32f429i-disc thumbv7em-none-eabihf
```

#### 分步构建
```bash
# 1. 编译构建工具
./build.sh build-tool

# 2. 生成板级配置
./build.sh generate stm32f429i-disc

# 3. 编译 bootloader
./build.sh build-boot stm32f429i-disc thumbv7em-none-eabihf

# 4. 编译内核
./build.sh build-kernel stm32f429i-disc thumbv7em-none-eabihf
```

### 3. 查看支持的开发板

```bash
./build.sh list-boards
```

输出示例:
```
Supported boards:
- stm32f429i-disc (STM32F429I Discovery)
- stm32h7s78-dk (STM32H7S78 Discovery Kit)
- stm32n6570-dk (STM32N6570 Discovery Kit)
- esp32-c3-devkitc (ESP32-C3 DevKitC)
- ...
```

### 4. 清理构建产物

```bash
./build.sh clean
```

## 目录结构速查

```
FeatherCore/
├── boot/              # Bootloader - 硬件初始化和内核加载
├── kernel/            # 内核 - 任务调度、内存管理、系统调用
├── rootfs/            # 根文件系统 - Shell 和基本工具
├── build_tool/        # 构建工具 - 配置生成和链接脚本
├── common/            # 公共库 - 平台抽象和设备树
├── build.sh           # 构建脚本
└── docs/              # 文档
    ├── DESIGN.md      # 设计文档
    └── README.md      # 本文件
```

## 系统调用速查

### 进程管理
```rust
SYS_EXIT = 1      // 退出进程
SYS_GETPID = 20   // 获取进程 ID
```

### 文件操作
```rust
SYS_READ = 3      // 读取文件
SYS_WRITE = 4     // 写入文件
SYS_OPEN = 5      // 打开文件
SYS_CLOSE = 6     // 关闭文件
```

### 内存管理
```rust
SYS_MMAP = 90     // 内存映射
SYS_MUNMAP = 91   // 取消映射
SYS_BRK = 93      // 修改数据段
```

## Rootfs 工具速查

### 基本命令
```bash
sh          # Shell 解释器
ls          # 列出目录内容
cat         # 显示文件内容
echo        # 显示文本
cd          # 切换目录
pwd         # 显示当前目录
```

### 文件操作
```bash
mkdir       # 创建目录
rm          # 删除文件
cp          # 复制文件
mv          # 移动文件
```

### 系统信息
```bash
ps          # 显示进程状态
mount       # 挂载文件系统
```

## 配置文件

### /etc/passwd - 用户信息
```
root:x:0:0:root:/root:/bin/sh
daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin
```

### /etc/profile - 环境变量
```bash
export PATH=/usr/local/sbin:/usr/local/bin:/usr/bin:/bin:/sbin
export PS1='\u@\h:\w\$ '
```

### /etc/rc.local - 启动脚本
```bash
#!/bin/sh
mkdir -p /var/log
chmod 1777 /tmp
```

## 启动流程

```
1. CPU 复位
   ↓
2. Bootloader (_start)
   ├─ 硬件初始化
   ├─ 解析设备树
   └─ 加载内核
   ↓
3. Kernel (kernel_main)
   ├─ 初始化内存
   ├─ 初始化中断
   ├─ 初始化驱动
   └─ 启动调度器
   ↓
4. 用户空间 (/init)
   ├─ 挂载文件系统
   ├─ 执行启动脚本
   └─ 启动 Shell
```

## 开发指南

### 添加新驱动

1. 在 `kernel/src/driver/` 创建驱动文件
2. 实现 `DeviceOps` trait
3. 注册设备:
```rust
use feathercore_kernel::driver::{Device, DeviceType, DeviceOps, register_device};

struct MyDevice;

impl DeviceOps for MyDevice {
    fn open(&self) -> Result<(), ()> { Ok(()) }
    fn close(&self) -> Result<(), ()> { Ok(()) }
    fn read(&self, buf: &mut [u8]) -> Result<usize, ()> { Ok(0) }
    fn write(&self, buf: &[u8]) -> Result<usize, ()> { Ok(buf.len()) }
    fn ioctl(&self, cmd: u32, arg: usize) -> Result<usize, ()> { Ok(0) }
}

let device = Device::new(
    "my_device".to_string(),
    DeviceType::Character,
    100,  // major
    0,    // minor
    &MyDevice,
);
register_device(device)?;
```

### 添加新系统调用

1. 在 `kernel/src/syscall/mod.rs` 添加系统调用号
2. 实现处理函数:
```rust
fn sys_my_call(args: &SyscallArgs) -> SyscallResult {
    let param = args.arg0;
    // 实现功能
    SyscallResult { ret0: result, ret1: 0 }
}
```

3. 在 `handle_syscall()` 中注册

### 添加新命令

1. 在 `rootfs/src/coreutils/` 创建命令文件
2. 在 `rootfs/Cargo.toml` 添加 binary 条目:
```toml
[[bin]]
name = "mycmd"
path = "src/coreutils/mycmd.rs"
required-features = ["coreutils"]
```

## 调试技巧

### 启用日志
```rust
use feathercore_kernel::log::{set_max_level, Level};
set_max_level(Level::Debug);
```

### 查看内核日志
```rust
info!("Initialization complete");
debug!("Memory size: {} bytes", memory_size);
error!("Failed to open device");
```

### QEMU 调试 (如果支持)
```bash
# ARM Cortex-M
qemu-system-arm -cpu cortex-m4 -machine stm32f429i-disc \
  -kernel boot/target/thumbv7em-none-eabihf/release/feathercore-boot \
  -semihosting -serial stdio

# RISC-V
qemu-system-riscv32 -cpu rv32 -machine virt \
  -kernel boot/target/riscv32imac-unknown-none-elf/release/feathercore-boot \
  -semihosting -serial stdio
```

## 常见问题

### Q: 编译失败 "target not found"
A: 使用 `rustup target add <target>` 安装目标平台

### Q: 如何查看支持的开发板?
A: 运行 `./build.sh list-boards`

### Q: 如何清理构建产物?
A: 运行 `./build.sh clean`

### Q: 如何添加新架构支持?
A: 
1. 在 `common/arch/` 创建新架构目录
2. 实现架构相关的初始化和异常处理
3. 在 `Cargo.toml` 添加 feature
4. 更新设备树配置

## 参考资源

- **设计文档**: [docs/DESIGN.md](docs/DESIGN.md)
- **实现总结**: [docs/IMPLEMENTATION_SUMMARY.md](docs/IMPLEMENTATION_SUMMARY.md)
- **主 README**: [README.md](../README.md)

## 下一步

1. 阅读 [DESIGN.md](docs/DESIGN.md) 了解完整架构
2. 选择一个开发板开始构建
3. 尝试修改和添加新功能
4. 贡献代码到项目

## 获取帮助

查看文档:
```bash
cat docs/DESIGN.md
cat docs/IMPLEMENTATION_SUMMARY.md
```

构建帮助:
```bash
./build.sh help
```
