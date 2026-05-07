# STM32H7S78-DK NuttX Porting Notes

本文档记录 `/home/uan-wsl2/nuttx-work` 中 STM32H7S78-DK 的 NuttX 移植状态。当前目标是让内部 Flash 中的 `nxboot-loader` 对齐 Cube `Template_XIP_Custom/Boot` 的核心能力：UART4、XSPI2 NOR、XSPI1 PSRAM，然后从外部 XSPI2 NOR 读取 app image 并跳转。

## 当前分支

```text
nuttx: /home/uan-wsl2/nuttx-work/nuttx -> vendor/stm32h7rs-bringup
apps:  /home/uan-wsl2/nuttx-work/apps  -> vendor/stm32h7rs-bringup
```

## 目录设计

H7RS 按独立 SoC family 维护，不并入旧 `stm32h7`：

```text
nuttx/arch/arm/src/stm32h7rs
nuttx/boards/arm/stm32h7rs/stm32h7s78-dk
```

这个设计和 STM32CubeH7RS 的组织方式一致：

```text
/home/uan-wsl2/third/STM32CubeH7RS/Drivers/CMSIS/Device/ST/STM32H7RSxx
/home/uan-wsl2/third/STM32CubeH7RS/Drivers/STM32H7RSxx_HAL_Driver
/home/uan-wsl2/third/STM32CubeH7RS/Drivers/BSP/STM32H7S78-DK
```

旧 `stm32h7` 目录只作为 NuttX 代码风格参考；寄存器、时钟、XSPI、PWR、GPIO 等事实来源以 Cube H7RS CMSIS 头和 STM32H7S78-DK BSP 为准。本次移植没有直接引入 Cube HAL/LL 源码。

## NXboot 布局

当前保留两个主要配置：

```text
stm32h7s78-dk:nxboot-loader
stm32h7s78-dk:nxboot-app
```

loader 链接到内部 Flash：

```text
FLASH: 0x08000000, 64 KiB
SRAM:  0x24000000, 456 KiB
```

app 链接到外部 NOR primary slot 的 NXboot header 之后：

```text
XSPI2 NOR base: 0x70000000
NXboot header:  0x400
App link addr:  0x70000400
PSRAM window:   0x90000000, 32 MiB
```

外部 NOR 仍按只读 boot medium 使用。app 镜像先通过 STM32CubeProgrammer/external loader 烧写到外部 NOR，NuttX 内部 erase/write OTA 更新留到后续阶段。

PSRAM 本轮只由 loader/app 初始化到 memory-mapped 可访问状态。真正把 `0x90000000..0x92000000` 加入 NuttX heap 应由 app 侧在架构初始化阶段完成，不能由 bootloader 代替 app 管理运行期 heap。

## 当前已完成

SoC/board 基础：

```text
- ARCH_CHIP_STM32H7RS
- ARCH_CHIP_STM32H7S7L8
- ARCH_BOARD_STM32H7S78_DK
- STM32H7RS memory map / IRQ / chip.h / board.h 基础定义
- STM32H7S78-DK nxboot-loader / nxboot-app 配置和链接脚本
```

运行基础：

```text
- RCC 600 MHz 基础时钟路径
- UART4 early output，PD1 TX / PD0 RX，AF8，115200 8N1
- SysTick 系统节拍
- BOARDIOC_BOOT_IMAGE 跳转框架：设置 VTOR、切换 MSP、跳转 app reset handler
```

外部 NOR boot-read：

```text
- 新增 H7RS XSPI/XSPIM/PWR/SBS/RCC/GPIO 最小寄存器定义
- 新增 stm32h7rs_xspi2_nor_initialize()
- STM32H7S78-DK XSPI2 NOR 引脚初始化，GPION AF9 XSPIM_P2
- 使能 XSPIM1/XSPIM2 power、CSI、SBS compensation cell、SBS high-speed IO、XSPI1/XSPI2/XSPIM 时钟
- XSPI2 startup 阶段使用 PLL2S 200 MHz / prescaler 3 = 50 MHz
- 先用 1S_1S_1S 执行 reset、JEDEC ID、MX66UW1G45G CFG2 配置
- JEDEC ID 已确认应为 c2 81 3b，对应 Macronix MX66UW1G45G
- 按 Cube custom memory 配置切换到 optional 8D_8D_8D / DQS / 16-bit instruction
- XSPI2 optional 阶段使用 prescaler 0 = 200 MHz
- 进入 XSPI2 OPI/DTR memory-mapped read mode，使 0x70000000 可直接读
- 读取 0x70000000 的 header word 做最小诊断
- XSPI2 already memory-mapped 时直接复用，避免 app 跳转后重复 reset NOR
```

外部 PSRAM：

```text
- 新增 stm32h7rs_xspi1_psram_initialize()
- STM32H7S78-DK XSPI1 PSRAM 引脚初始化，GPIOO/GPIOP AF9 XSPIM_P1
- XSPI1 startup 阶段使用 PLL2S 200 MHz / prescaler 3 = 50 MHz
- 按 APS256 custom memory 配置 MA0=0x11、MA4=0x20、MA8=0x40
- MR 配置流程是先读、按 mask 合成、DTR 双字节重复写入、再读回校验
- XSPI1 optional 阶段使用 prescaler 0 = 200 MHz
- 进入 8S_8D_16D memory-mapped read/write mode，使 0x90000000 可读写
- loader 执行 32-byte 保存、写入 pattern、读回、恢复的最小破坏性自检
- app 侧只检测 mapped 状态，不重复做破坏性自检
- 预留 CONFIG_STM32H7RS_PSRAM_HEAP，默认关闭，本轮没有把 PSRAM 加入 heap
```

外存初始化顺序：

```text
stm32h7rs_extmem_initialize()
  -> stm32h7rs_xspi2_nor_initialize()
       -> XSPIM/CSI/SBS common setup
       -> XSPI2 NOR OPI/DTR memory-mapped read
  -> stm32h7rs_xspi1_psram_initialize()
       -> XSPIM/CSI/SBS common setup
       -> XSPI1 PSRAM 8S_8D_16D memory-mapped read/write
  -> stm32h7rs_register_ota_partitions()
```

OTA 分区注册被放到 FLASH 和 PSRAM 都初始化成功之后，避免 boot 阶段外存半成功时仍暴露 `/dev/ota*`。

本轮 XSPI2 NOR 初始化已从第一阶段 1-line boot-read 升级为 Cube `Template_XIP_Custom/Boot` 风格的 OPI/DTR memory-mapped read。高速 delay block calibration、完整 NOR erase/write MTD 和 OTA 更新写入仍未实现。

## 2026-05-06 XSPI FLASH + PSRAM Cube 对齐

本轮按 Cube `Template_XIP_Custom/Boot` 的 custom memory 配置重写外存关键路径，但没有引入 Cube HAL/LL/EMM `.c` 源码。

关键修复：

```text
- 新增 XSPI_DCR2 prescaler mask 和运行时 prescaler 切换
- 修正 memory size：NOR device size 26，PSRAM device size 24
- 新增 XSPI_CCR_DMODE_16_LINES，用于 PSRAM 8S_8D_16D optional 模式
- NOR startup: 1S_1S_1S / 50 MHz，reset、JEDEC ID、CFG2 配置
- NOR optional: 8D_8D_8D / 200 MHz，read 0xEE11 dummy 20，write 0x12ED
- PSRAM startup: 8S_8D_8D / 50 MHz，MR read/write dummy 4，DQS enable
- PSRAM optional: 8S_8D_16D / 200 MHz，read 0x00，write 0xA0，dummy 6
- PSRAM DTR register write 第二个 byte 现在复制第一个 byte，不再写 0
- /dev/ota0..2 只在 XSPI2 NOR 和 XSPI1 PSRAM 都成功后注册
```

构建验证：

```text
./tools/configure.sh stm32h7s78-dk:nxboot-loader
make -j8
nuttx.bin = 54904 bytes，内部 64 KiB Flash 使用率 83.78%

./tools/configure.sh stm32h7s78-dk:nxboot-app
make -j8
nuttx.bin = 83480 bytes
```

当前保留的构建产物：

```text
/home/uan-wsl2/nuttx-work/nuttx/nuttx.bin              当前为 nxboot-loader
/home/uan-wsl2/nuttx-work/nuttx-nxboot-loader.bin      nxboot-loader 备份
/home/uan-wsl2/nuttx-work/nuttx-nxboot-app.bin         nxboot-app 未加 NXboot header 的原始 app 镜像
```

## OTA 分区

当前 `/dev/ota*` 来自外部 NOR memory-mapped 只读 MTD shim：

```text
/dev/ota0 -> primary image slot   offset 0x00000000, size 32 MiB
/dev/ota1 -> secondary image slot offset 0x02000000, size 32 MiB
/dev/ota2 -> tertiary image slot  offset 0x04000000, size 32 MiB
```

STM32H7S78-DK 板载 XSPI2 NOR 是 128 MiB。NXboot 当前实现会同时打开 primary、secondary、tertiary 三个 slot，因此不能使用 3 x 64 MiB 的布局；那会让 `/dev/ota2` 从 NOR 末尾之后开始，导致分区注册失败。

注册流程：

```text
stm32h7rs_extmem_initialize()
  -> stm32h7rs_xspi2_nor_initialize()
  -> stm32h7rs_register_ota_partitions()
  -> stm32h7rs_xspi1_psram_initialize()
```

也就是说，NXboot loader 访问 `/dev/ota0` 前会先配置 XSPI2 NOR memory-mapped read mode 并注册 OTA 分区，随后继续配置 XSPI1 PSRAM。为了对齐 Cube `Template_XIP_Custom/Boot` 的 boot 能力，当前策略已经调整为：PSRAM 初始化或 self-test 失败会让 board bring-up 返回错误，不再假装外存全部正常。

## 2026-05-06 PSRAM 命令格式修正

根据 Cube `Drivers/BSP/Components/aps256xx/aps256xx.c` 对齐了 APS256XX 的关键事务格式：

```text
- XSPIM CR 启用 CSSEL override，并保持 XSPI1 -> P1、XSPI2 -> P2，二者都走 NCS1。
- PSRAM reset 使用 8-line instruction + 8-line 24-bit address。
- PSRAM mode register write 使用 8-line instruction/address/data，address/data DTR，DataLength=2，DQS disabled，dummy=0。
- PSRAM mode register read 使用 8-line instruction/address/data，address/data DTR，DataLength=2，DQS enabled，dummy=latency-1。
- PSRAM memory-mapped read/write dummy cycles 按 Cube driver 的 latency-1 方式设置。
- PSRAM 初始化失败现在会阻塞 NXboot board bring-up。
```

这轮构建只验证了代码可编译；是否真正通过仍以 UART4 上板日志为准。期望的下一步日志是 MR0/MR4/MR8 不再读回 `00`，并出现 `XSPI1 PSRAM self-test passed`。

## 2026-05-06 NOR JEDEC 诊断修正

`XSPI2 NOR JEDEC ID 00 00 00` 不视为成功。已调整为：

```text
- 先发送 Octal/DTR 16-bit reset sequence: 0x6699, 0x9966。
- 再发送 SPI reset sequence: 0x66, 0x99。
- 随后读取 SPI 1-1-1 JEDEC ID。
- 如果 JEDEC ID 为 00 00 00 或 FF FF FF，直接返回 -EIO，不再继续注册为正常 NOR。
```

这样可以覆盖 flash 被 external loader 或前一次 app 留在 OPI/DTR 模式的情况。如果仍读到全 0，下一步重点检查 XSPI2 command phase 的片选/时钟/协议配置，而不是继续相信 memory-mapped 数据。

补充：后续又把 regular command 配置顺序对齐 HAL 行为。配置 CCR/TCR/IR/AR 前先把 `FMODE` 清为 `0`，只在 transmit/receive 阶段切到 indirect write/read；JEDEC blank 时会额外打印 XSPI2 `CR/SR/DCR/CCR/TCR/DLR`，用于定位命令是否按预期发出。

## 2026-05-06 XSPI 时钟与 NOR reset delay 对齐 Cube

实测日志出现：

```text
XSPI2 NOR JEDEC ID 00 00 00
XSPI2 NOR JEDEC ID is blank
XSPI2 regs CR=10000001 SR=00000000 DCR1=011b0100 DCR2=00000001 CCR=01000001 TCR=00000000 DLR=00000002
```

这说明 XSPI2 regular command 基本已经进入 indirect read 状态，但 NOR 没有返回有效 ID。进一步对照 Cube `Template_XIP_Custom/Boot` 后，本轮补齐两个关键差异：

```text
- RCC 增加 PLL2：HSI / 4 * 25 / 2 = PLL2S 200 MHz。
- XSPI1/XSPI2 kernel clock 改为 PLL2S，不再使用 HCLK。
- XSPI startup prescaler 改为 3，即 200 MHz / (3 + 1) = 50 MHz，匹配 Cube EXTMEM StartupConfig.Frequency。
- MX66UW1G45G 的 reset delay 改为 100 ms，匹配 Cube ResetDelay。
```

注意：XSPIM 的 `NCS1` override 表达方式是 `CSSEL_OVR_EN=1` 且 `O1/O2=0`，所以当前只置 `CSSEL_OVR_EN` 是 NCS1，不是 NCS2。

## 构建验证

已执行：

```bash
cd /home/uan-wsl2/nuttx-work/nuttx

make distclean
./tools/configure.sh stm32h7s78-dk:nxboot-loader
make -j8

make distclean
./tools/configure.sh stm32h7s78-dk:nxboot-app
make -j8
```

当前构建结果：

```text
nxboot-loader:
  flash used: 54904 B / 64 KiB = 83.78%
  sram used:  7832 B / 456 KiB = 1.68%
  binary:     /home/uan-wsl2/nuttx-work/nuttx-nxboot-loader.bin
  size:       54904 bytes

nxboot-app:
  flash used: 83480 B / 32767 KiB = 0.25%
  sram used:  8024 B / 456 KiB = 1.72%
  link addr:  0x70000400
  extram used: 0 B / 32 MiB
  raw binary: /home/uan-wsl2/nuttx-work/nuttx-nxboot-app.bin
  raw size:   83480 bytes

packaged app image:
  binary:     /home/uan-wsl2/nuttx-work/stm32h7s78-dk-nxboot-app.bin
  size:       84504 bytes
  program at: 0x70000000
  header:     0x400 bytes
```

## 2026-05-07 上板验证状态

当前 NXboot loader 到外部 NOR app 的启动链路已经跑通。

已验证日志：

```text
H7RS
STM32H7S78-DK bring-up skeleton
XSPI2 NOR JEDEC ID c2 81 3b
XSPI2 NOR OPI/DTR config readback 02
XSPI2 NOR mapped 0x70000000 header[0]=0x534f584e
XSPI1 PSRAM MR00000000 initial 08 write 11 readback 11
XSPI1 PSRAM MR00000004 initial 40 write 20 readback 20
XSPI1 PSRAM MR00000008 initial 05 write 45 readback 45
XSPI1 PSRAM self-test passed
XSPI1 PSRAM mapped at 0x90000000
*** nxboot ***
Validating image.
Found bootable image, boot from primary.
Boot vector msp=0x24002358 reset=0x70000995 vtor=0x70000400
H7RS
STM32H7S78-DK bring-up skeleton
XSPI2 NOR already memory-mapped
XSPI1 PSRAM already memory-mapped
```

结论：

```text
- XSPI2 NOR JEDEC ID 已对上 MX66UW1G45G：c2 81 3b。
- XSPI2 NOR 已进入 OPI/DTR memory-mapped read，0x70000000 可读到 NXboot header。
- XSPI1 PSRAM MR0/MR4/MR8 配置和 32-byte memory-mapped 自检通过。
- NXboot 能识别 /dev/ota0 primary image，并跳转到 0x70000400 app。
- app 启动后能检测到 XSPI2 NOR 和 XSPI1 PSRAM already memory-mapped，未破坏 bootloader 已建立的外存状态。
```

## 2026-05-07 UART4 console 与 PSRAM heap 补齐

本轮补齐的是 app 侧基础可用性：

```text
nxboot-loader:
  - 继续保持极简 early/syslog UART4 输出。
  - 不启用完整 UART4 serial lower-half。
  - 不管理 app 的 PSRAM heap。

nxboot-app:
  - 启用 CONFIG_STM32H7RS_UART4_SERIALDRIVER。
  - UART4 PD1/PD0 AF8 115200 8N1 注册为 /dev/console 和 /dev/ttyS0。
  - 启用 CONFIG_STM32H7RS_PSRAM_HEAP 和 CONFIG_MM_REGIONS=2。
  - app 早期 arm_addregion() 阶段把 0x90000000..0x91ffffff 全量 32 MiB 加入 heap。
```

PSRAM heap 策略：

```text
- 主 heap 仍来自内部 SRAM，确保 app 在 PSRAM 初始化失败时仍可启动。
- 若 XSPI1 PSRAM 已由 loader 初始化为 memory-mapped，app 侧直接复用。
- 若 PSRAM addregion 阶段初始化失败，只打印错误并跳过额外 heap，不强制 panic。
- 当前不预留 framebuffer 区，完整 32 MiB PSRAM 都作为 app heap。
```

上板后建议验证：

```bash
ls /dev
free
help
```

期望：

```text
- /dev/console 和 /dev/ttyS0 存在。
- UART4 能进入可交互 NSH。
- free 显示 heap 相比纯内部 SRAM 明显增加，约增加 32 MiB。
- /dev/ota0..2 仍保持 NXboot 只读 XIP image slot 语义。
```

## 当前限制

```text
- NXboot 仍打印 "Progress percent requested but no previous progress start"，不影响启动，后续可关闭 progress percent 或补 progress start。
- loader 跳转 app 时 UART 偶发单字节乱码，疑似串口/时钟/缓冲切换瞬间毛刺，不影响启动。
- UART4 serial lower-half 第一版仅覆盖 UART4 console/NSH，不支持 DMA、flow control 或完整 termios 扩展。
- XSPI1 PSRAM 已加入 app heap，但尚未做 MPU/cache/DLYB 稳定性优化。
- XSPI2 NOR 只读 XIP/boot-read 已通，NuttX 内部 erase/write MTD 和 OTA 更新写入尚未实现。
- 高速 DLYB calibration、MPU/cache 策略仍需针对 200 MHz OPI/DTR 做后续加固。
```

## 当前烧录产物

```text
内部 Flash 0x08000000:
  /home/uan-wsl2/nuttx-work/nuttx-nxboot-loader.bin

外部 XSPI2 NOR 0x70000000:
  /home/uan-wsl2/nuttx-work/stm32h7s78-dk-nxboot-app.bin

不要把 raw app 烧到 0x70000000:
  /home/uan-wsl2/nuttx-work/nuttx-nxboot-app.bin
```

## 后续任务

```text
1. 处理 NXboot progress percent 提示，或在 loader 配置中关闭百分比输出。
2. 上板验证 UART4 NSH 输入输出和 PSRAM heap 的长时间稳定性。
3. 补 XSPI DLYB calibration、MPU/cache 策略，提升 200 MHz OPI/DTR 稳定性。
4. 实现 XSPI2 NOR erase/write MTD，用于后续 OTA 更新。
5. 继续补 LED/button、SDMMC、LCD、Ethernet、USB 等板级外设。
```
