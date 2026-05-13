# STM32H7S78-DK NuttX Porting Notes

本文档记录 `/home/uan-wsl2/Feather` 中 STM32H7S78-DK 的 NuttX 移植状态。当前目标是让内部 Flash 中的 `nxboot` 对齐 Cube `Template_XIP_Custom/Boot` 的核心能力：UART4、XSPI2 NOR、XSPI1 PSRAM，然后从外部 XSPI2 NOR 读取 NSH app image 并跳转。

## 当前分支

```text
Feather: /home/uan-wsl2/Feather       -> main
nuttx:   /home/uan-wsl2/Feather/nuttx -> develop
apps:    /home/uan-wsl2/Feather/apps  -> develop
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

分层约定：

```text
- arch/arm/src/stm32h7rs/stm32h7rs_xspi.c
  只保留 H7RS XSPI/XSPIM 控制器级通用 helper。
- boards/arm/stm32h7rs/stm32h7s78-dk/src/stm32h7s78_xspi.c
  放 STM32H7S78-DK 板载 MX66UW1G45G NOR、APS256XX PSRAM 的命令、
  mode register、pinmux、自检和 Cube 参数对齐策略。
- boards/arm/stm32h7rs/stm32h7s78-dk/src/stm32h7s78_heap.c
  放本板 PSRAM heap 的 arm_addregion() 策略。
```

## NXboot 布局

当前保留两个主要配置：

```text
stm32h7s78-dk:nxboot
stm32h7s78-dk:nsh
```

`nxboot` 链接到内部 Flash：

```text
FLASH: 0x08000000, 64 KiB
SRAM:  0x24000000, 456 KiB
```

`nsh` 链接到外部 NOR primary slot 的 NXboot header 之后：

```text
XSPI2 NOR base: 0x70000000
NXboot header:  0x400
App link addr:  0x70000400
PSRAM window:   0x90000000, 32 MiB
```

外部 NOR 仍按只读 boot medium 使用。app 镜像先通过 STM32CubeProgrammer/external loader 烧写到外部 NOR，NuttX 内部 erase/write OTA 更新留到后续阶段。

PSRAM 由 loader 建立 memory-mapped 通路，app 侧复用已有映射。`nsh`
配置在 `arm_addregion()` 阶段把 `0x90001000..0x91ffffff` 加入 NuttX
heap；`nxboot` 只负责启动所需的外存初始化，不管理 app 运行期 heap。

## 当前已完成

SoC/board 基础：

```text
- ARCH_CHIP_STM32H7RS
- ARCH_CHIP_STM32H7S7L8
- ARCH_BOARD_STM32H7S78_DK
- STM32H7RS memory map / IRQ / chip.h / board.h 基础定义
- STM32H7S78-DK nxboot / nsh 配置和链接脚本
```

运行基础：

```text
- RCC 600 MHz 基础时钟路径
- UART4 early output，PD1 TX / PD0 RX，AF8，115200 8N1
- SysTick 系统节拍
- BOARDIOC_BOOT_IMAGE 跳转框架：设置 VTOR、切换 MSP、跳转 app reset handler
- 启动阶段配置基础 MPU/cache：internal Flash、AXI SRAM、XSPI2 NOR、XSPI1 PSRAM
- loader 跳转 app 前关闭 SysTick/NVIC pending、I-cache、D-cache 和 MPU
```

外部 NOR boot-read：

```text
- 新增 H7RS XSPI/XSPIM/PWR/SBS/RCC/GPIO 最小寄存器定义
- 新增 board 侧 stm32h7s78_xspi2_nor_initialize()
- STM32H7S78-DK XSPI2 NOR 引脚初始化，GPION AF9 XSPIM_P2
- 使能 XSPIM1/XSPIM2 power、CSI、SBS compensation cell、SBS high-speed IO、XSPI1/XSPI2/XSPIM 时钟
- XSPI2 startup 阶段使用 PLL2S 200 MHz / prescaler 3 = 50 MHz
- 先用 1S_1S_1S 执行 reset、JEDEC ID、MX66UW1G45G CFG2 配置
- JEDEC ID 已确认应为 c2 81 3b，对应 Macronix MX66UW1G45G
- 按 Cube custom memory 配置切换到 optional 8D_8D_8D / DQS / 16-bit instruction
- XSPI2 optional 阶段使用 prescaler 0 = 200 MHz
- XSPI2 controller 参数对齐 Cube ExtMem Manager 最终状态：
  CS high time 8、FIFO threshold 2、CS boundary none
- 进入 XSPI2 OPI/DTR memory-mapped read mode，使 0x70000000 可直接读
- 读取 0x70000000 的 header word 做最小诊断
- XSPI2 already memory-mapped 时直接复用，避免 app 跳转后重复 reset NOR
```

外部 PSRAM：

```text
- 新增 board 侧 stm32h7s78_xspi1_psram_initialize()
- STM32H7S78-DK XSPI1 PSRAM 引脚初始化，GPIOO/GPIOP AF9 XSPIM_P1
- XSPI1 startup 阶段使用 PLL2S 200 MHz / prescaler 3 = 50 MHz
- 按 APS256 custom memory 配置 MA0=0x11、MA4=0x20、MA8=0x40
- MR 配置流程是先读、按 mask 合成、DTR 双字节重复写入、再读回校验
- XSPI1 optional 阶段使用 prescaler 0 = 200 MHz
- 进入 8S_8D_16D memory-mapped read/write mode，使 0x90000000 可读写
- loader 执行 32-byte 保存、写入 pattern、读回、恢复的最小破坏性自检
- app 侧只检测 mapped 状态，不重复做破坏性自检
- nsh 启用 CONFIG_STM32H7RS_PSRAM_HEAP 和 CONFIG_MM_REGIONS=2
- 0x90001000..0x91ffffff 作为 app 额外 heap，前 4 KiB 保留给 bring-up 自测
```

外存初始化顺序：

```text
stm32h7rs_extmem_initialize()
  -> stm32h7s78_xspi2_nor_initialize()
       -> XSPIM/CSI/SBS common setup
       -> XSPI2 NOR OPI/DTR memory-mapped read
  -> stm32h7s78_xspi1_psram_initialize()
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
./tools/configure.sh stm32h7s78-dk:nxboot
make -j8
nuttx.bin = 54904 bytes，内部 64 KiB Flash 使用率 83.78%

./tools/configure.sh stm32h7s78-dk:nsh
make -j8
nuttx.bin = 88144 bytes
```

当前统一构建入口：

```text
cd /home/uan-wsl2/Feather
./tools/firmware/stm32h7s78-dk/build-nsh.sh -j 8
```

输出产物：

```text
/home/uan-wsl2/Feather/build/stm32h7s78-dk-nxboot.bin
  raw NuttX NXboot，烧录到内部 Flash 0x08000000

/home/uan-wsl2/Feather/build/stm32h7s78-dk-nsh.bin
  [NXboot header][NuttX app raw binary]，烧录到外部 XSPI2 NOR 0x70000000
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
  -> stm32h7s78_xspi2_nor_initialize()
  -> stm32h7s78_xspi1_psram_initialize()
  -> stm32h7rs_register_ota_partitions()
```

也就是说，NXboot loader 访问 `/dev/ota0` 前会先配置 XSPI2 NOR
memory-mapped read mode，再配置 XSPI1 PSRAM。只有 NOR 和 PSRAM 都初始化成功
后才注册 OTA 分区。为了对齐 Cube `Template_XIP_Custom/Boot` 的 boot 能力，
PSRAM 初始化或 self-test 失败会让 board bring-up 返回错误，不再假装外存全部正常。

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

统一构建脚本会依次执行：

```bash
cd /home/uan-wsl2/Feather/nuttx

make distclean
./tools/configure.sh stm32h7s78-dk:nxboot
make -j8

make distclean
./tools/configure.sh stm32h7s78-dk:nsh
make -j8
```

当前构建结果和输出约定：

```text
nxboot:
  flash used: 56428 B / 64 KiB = 86.10%
  sram used:  7832 B / 456 KiB = 1.68%
  binary:     /home/uan-wsl2/Feather/build/stm32h7s78-dk-nxboot.bin
  size:       56428 bytes

nsh:
  flash used: 114412 B / 32767 KiB = 0.34%
  sram used:  8696 B / 456 KiB = 1.86%
  link addr:  0x70000400
  extram used: 0 B / 32 MiB
  raw binary: /home/uan-wsl2/Feather/nuttx/nuttx.bin
  raw size:   114412 bytes

packaged app image:
  binary:     /home/uan-wsl2/Feather/build/stm32h7s78-dk-nsh.bin
  size:       115436 bytes
  program at: 0x70000000
  header:     0x400 bytes
  structure:  [NXboot header][NuttX app raw binary]
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
nxboot:
  - 继续保持极简 early/syslog UART4 输出。
  - 不启用完整 UART4 serial lower-half。
  - 不管理 app 的 PSRAM heap。

nsh:
  - 启用 CONFIG_STM32H7RS_UART4_SERIALDRIVER。
  - UART4 PD1/PD0 AF8 115200 8N1 注册为 /dev/console 和 /dev/ttyS0。
  - 启用 CONFIG_STM32H7RS_PSRAM_HEAP 和 CONFIG_MM_REGIONS=2。
  - app 早期 arm_addregion() 阶段把 0x90001000..0x91ffffff 加入 heap，保留前 4 KiB 作为 bring-up 自测/保护窗口。
  - 启用并挂载 procfs 到 /proc，保证 NSH free/ps 命令可用。
  - 启用 ramtest、ramspeed、memstress，用于 PSRAM 容量、带宽和 heap 压力验证。
```

PSRAM heap 策略：

```text
- 主 heap 仍来自内部 SRAM，确保 app 在 PSRAM 初始化失败时仍可启动。
- 若 XSPI1 PSRAM 已由 loader 初始化为 memory-mapped，app 侧复用已有映射并运行扩展自测，不重新配置 XSPI 控制器。
- 若 PSRAM addregion 阶段初始化失败，只打印错误并跳过额外 heap，不强制 panic。
- 当前保留 0x90000000..0x90000fff，不交给 malloc；其余 PSRAM 作为 app heap。
- PSRAM memory-mapped 数据通路由 nxboot 建立，按 Cube `Template_XIP_Custom`
  对齐为 200 MHz；XSPI2 NOR XIP 同样保持 200 MHz。
```

上板后建议验证：

```bash
ls /dev
ls /proc
free
ps
ramtest -w -s 16777216
ramspeed -a -s 4194304 -n 100
memstress -m 65536 -n 128 -x 1 -t 1000 &
help
```

期望：

```text
- /dev/console 和 /dev/ttyS0 存在。
- /proc 存在，free/ps 不再被 NSH 编译期 gating 隐式禁用。
- UART4 能进入可交互 NSH。
- free 显示 heap 相比纯内部 SRAM 明显增加，约增加 32 MiB（减去 4 KiB 保留窗口和 allocator 开销）。
- ps 显示当前任务列表，便于确认 NSH/idle 任务状态。
- ramtest/ramspeed 可从 heap 分配大块内存验证 PSRAM，避免直接裸写 0x90000000 破坏 allocator。
- memstress 是持续压力测试，建议后台运行，用 ps 确认任务状态，需要停止时用 kill 结束。
- /dev/ota0..2 仍保持 NXboot 只读 XIP image slot 语义。
```

## 2026-05-10 XSPI1 PSRAM 长块访问收紧

上板 `free/ps` 已确认 PSRAM heap 能注册，但 `ramtest -w -s 1048576`
在 heap buffer 起始后约 `+0x30` 处出现固定读回错误。这说明 boot -> app 的
memory-mapped 继承路径已经成立，但 PSRAM 控制器参数还不足以支撑 NuttX heap
上的长块连续读写。

本轮调整：

```text
- app 仍只复用 boot 已建立的 XSPI1 memory-mapped 状态，不再 remap。
- XSPI1 PSRAM memory-mapped 配置补齐 DCR4 refresh。
- XSPI1 PSRAM memory-mapped read/write 使用 linear burst opcode：read 0x20、write 0xa0。
- XSPI1 PSRAM 后续改回 Cube `Template_XIP_Custom` 参数：
  optional prescaler 0、FIFO threshold 2、HAL_XSPI_BONDARYOF_8KB。
- app 侧 "already memory-mapped" 日志会打印继承到的 refresh 值。
- PSRAM self-test 增加 0x90001040 附近覆盖，用于命中当前 ramtest 暴露的问题地址段。
```

重新烧录后期望 boot/app 日志包含：

```text
XSPI1 PSRAM mapped at 0x90000000 refresh=396
...
XSPI1 PSRAM already memory-mapped refresh=396
XSPI1 PSRAM self-test passed
```

如果看到 `refresh=0` 或旧的 `mapped at 0x90000000` 日志，说明内部 Flash 中的
NXboot 还没有更新；PSRAM 映射参数必须由 boot 阶段建立，单独更新 app 不够。

建议按递增规模验证：

```bash
ramtest -w -a 0x90000000 -s 4096
ramtest -w -s 1024
ramtest -w -s 65536
ramtest -w -s 1048576
ramtest -w -s 16777216
ramspeed -a -s 1048576 -n 16
```

`memstress` 是持续压力测试，建议等上述 ramtest 通过后再跑：

```bash
memstress -m 4096 -n 64 -x 1 -t 1000 &
ps
pkill memstress
```

## 2026-05-12 Cube XIP Custom 启动策略收敛

本轮继续按 Cube `Template_XIP_Custom/Boot` 收敛启动阶段配置，重点是让
`nxboot` 和 `nsh` 的 MPU/cache、配置命名和文档保持一致。

代码和配置调整：

```text
- stm32h7rs_start.c 增加基础 MPU/cache setup。
- MPU 区域覆盖 internal Flash、AXI SRAM、XSPI2 NOR、XSPI1 PSRAM。
- internal Flash 和 XSPI2 NOR 配置为 cacheable/executable。
- AXI SRAM 和 XSPI1 PSRAM 配置为 cacheable/XN。
- I-cache/D-cache enable/disable 路径均受 CONFIG_ARMV7M_ICACHE/DCACHE 保护。
- nxboot 和 nsh defconfig 都启用 CONFIG_ARM_MPU、16 regions、I-cache、D-cache。
- board_boot_image() 在跳转 app 前关闭 D-cache、I-cache 和 MPU，避免继承 loader 侧状态。
- XSPI D-cache flush helper 在未启用 D-cache 时退化为 no-op。
- 配置命名稳定为 stm32h7s78-dk:nxboot 和 stm32h7s78-dk:nsh；
  旧 nxboot-loader/nxboot-app 配置已经不再作为当前入口。
- board README 已补齐 build、memory map、option bytes 和 NXboot image 说明。
```

XSPI2 NOR controller 参数继续做严格对齐：

```text
- XSPI2 kernel clock 来自 PLL2S = 200 MHz。
- startup 阶段 prescaler 3，保持 50 MHz。
- optional/memory-mapped 阶段 prescaler 0，NOR 跑 200 MHz。
- MX66UW1G45G memory-mapped 模式保持 8D_8D_8D / DQS / dummy 20。
- CubeMX 初始 MX_XSPI2_Init() 里是 CSHT=2、FIFO=4；随后 ExtMem Manager
  custom driver 会 HAL_XSPI_DeInit/Init 并重配为最终有效状态。
- ChipSelectHighTimeCycle 对齐 custom memory object 的 MemChipSelectHighTimeCycle=8。
- FIFO threshold 对齐 custom driver 的 FifoThresholdByte=2。
- ChipSelectBoundary、MaxTran、Refresh 保持 Cube 的 0/none。
```

PSRAM 速度参数也重新对齐 Cube `Template_XIP_Custom`：

```text
- XSPI1 kernel clock 仍来自 PLL2S = 200 MHz。
- startup 阶段 prescaler 3，保持 50 MHz。
- optional/memory-mapped 阶段 prescaler 改回 0，PSRAM 跑 200 MHz。
- APS256 memory-mapped 模式保持 8S_8D_16D / DQS / dummy 6。
- FIFO threshold 改回 Cube 的 FifoThresholdByte=2。
- chip-select boundary 改回 Cube 的 HAL_XSPI_BONDARYOF_8KB。
- 预期 DCR4 refresh 从 196 变为 396。
```

本地验证：

```text
- ./tools/configure.sh -E stm32h7s78-dk:nxboot && make -j$(nproc) 通过。
- ./tools/configure.sh -E stm32h7s78-dk:nsh && make -j$(nproc) 通过。
- ./tools/firmware/stm32h7s78-dk/build-nsh.sh -j $(nproc) 通过，已生成 build/ 下两个烧录产物。
- 当前 .config 已恢复为 stm32h7s78-dk:nsh。
- nxstyle 检查 stm32h7rs_start.c、stm32h7rs_xspi.c 和 board bring-up/boot image 通过。
- git diff --check 通过。
```

尚未完成：

```text
- 这轮还没有重新上板烧录验证，启动链路仍需用 UART4 日志确认。
- 直接运行 apps/boot/nxboot/tools/nximage.py 需要 Python semantic_version；
  Feather build-nsh.sh 已验证会使用内置 fallback 生成 NXboot header，但手动调用
  nximage.py 前仍建议安装 requirements.txt。
```

## 2026-05-13 XSPI 分层整理

本轮把板级外存代码从 `arch/arm/src/stm32h7rs` 移回
`boards/arm/stm32h7rs/stm32h7s78-dk`：

```text
- arch/arm/src/stm32h7rs/stm32h7rs_xspi.c 现在只提供通用 XSPI helper：
  common setup、GPIO AF 配置、indirect command/read/write、controller
  DCR 配置、prescaler 切换和 memory-mapped 入口。
- 新增 arch/arm/src/stm32h7rs/stm32h7rs_xspi.h，作为 board 侧调用接口。
- 新增 board/src/stm32h7s78_xspi.c，承接 MX66UW1G45G NOR 和 APS256XX
  PSRAM 初始化、自检、Cube 对齐参数和具体 pinmux。
- 新增 board/src/stm32h7s78_heap.c，把 PSRAM heap 的 arm_addregion()
  策略从 arch 侧挪到 board 侧；arch 只保留 weak 空实现。
- NXboot image、OTA partition、PSRAM heap 等板级 Kconfig 从
  arch/arm/src/stm32h7rs/Kconfig 移到 STM32H7S78-DK board Kconfig。
```

本地验证：

```text
- nxstyle 检查重构涉及的 arch/board C 源和头文件通过。
- git diff --check 通过。
- ./tools/firmware/stm32h7s78-dk/build-nsh.sh -j $(nproc) 通过。
- 新输出：
  build/stm32h7s78-dk-nxboot.bin = 56540 bytes
  build/stm32h7s78-dk-nsh.bin    = 115508 bytes
```

## 当前限制

```text
- NXboot final primary validation 已补 progress start/end；预期不再打印 "Progress percent requested but no previous progress start"。
- loader 跳转 app 时 UART 偶发单字节乱码，疑似串口/时钟/缓冲切换瞬间毛刺，不影响启动。
- UART4 serial lower-half 第一版仅覆盖 UART4 console/NSH，不支持 DMA、flow control 或完整 termios 扩展。
- XSPI1 PSRAM 已加入 app heap，并补齐 refresh/CS boundary；仍需继续做长时间
  heap 压力、cache coherency 和 DLYB 稳定性验证。
- XSPI2 NOR 只读 XIP/boot-read 已通，NuttX 内部 erase/write MTD 和 OTA 更新写入尚未实现。
- 已有基础 MPU/cache 策略；高速 DLYB calibration 和 200 MHz OPI/DTR 稳定性仍需后续加固。
```

## 当前烧录产物

```text
内部 Flash 0x08000000:
  /home/uan-wsl2/Feather/build/stm32h7s78-dk-nxboot.bin

外部 XSPI2 NOR 0x70000000:
  /home/uan-wsl2/Feather/build/stm32h7s78-dk-nsh.bin
```

## 后续任务

```text
1. 重新烧录 nxboot/nsh，验证 2026-05-12 MPU/cache 配置后的 boot -> app 链路。
2. 上板验证 UART4 NSH 输入输出和 PSRAM heap 的长时间稳定性。
3. 补 XSPI DLYB calibration、cache maintenance 和 memory attribute 策略，提升 200 MHz OPI/DTR 稳定性。
4. 实现 XSPI2 NOR erase/write MTD，用于后续 OTA 更新。
5. 继续补 LED/button、SDMMC、LCD、Ethernet、USB 等板级外设。
```
