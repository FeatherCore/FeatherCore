# STM32 GPU2D/NemaP Porting Result

本文记录当前可提交版本的代码改动、四款 STM32 GPU2D/NemaP 实测能力，以及 `nemap_demo` 的验证边界。

## 当前结论

`nemap_demo` 已在四个目标上完成 phase 0-29 实机验证，并输出 `PASS all phases, restored colorbar`：

| 芯片/板级 | Framebuffer | GPU2D identity | CONFIG / CONFIGH | 资源放置 | 结论 |
|---|---|---|---|---|---|
| STM32U5G9 / STM32U5x9J-DK | 480x480 RGB565, PSRAM `0xa0000000`, stride 960 | `ID=0x86362000`, `IP_VERSION=0x00210011` | `0x74010104` / `0x000003e3` | command/ring/offscreen/source/mask/format buffer 均在 internal SRAM `0x200xxxxx` | phase 0-29 全通过；strict RGB24/BGR24 source pass；radial bit present，但 `VG=0` |
| STM32U5A9 / STM32U5A9J-DK | 480x480 RGB565, PSRAM `0xa0000000`, stride 960 | `ID=0x86362000`, `IP_VERSION=0x00010009` | `0x74010104` / `0x00000003` | command/ring/offscreen/source/mask/format buffer 均在 internal SRAM `0x200xxxxx` | phase 0-29 全通过；strict RGB24/BGR24 source pass；无 tex-channels/radial/VG |
| STM32N6 / STM32N6570-DK | 800x480 RGB565, PSRAM `0x90000000`, stride 1600 | `ID=0x86362000`, `IP_VERSION=0x00010009` | `0xf4010104` / `0x00000023` | framebuffer 后、PSRAM heap 前的 GPU-visible arena | phase 0-29 全通过；AXI64 RGB24/BGR24 source cross-beat 行为按 limited pass 记录 |
| STM32H7S7/H7RS / STM32H7S78-DK | 800x480 RGB565, PSRAM `0x90000000`, stride 1600 | `ID=0x86362000`, `IP_VERSION=0x00010009` | `0xf4010104` / `0x00000023` | framebuffer 后、PSRAM heap 前的 GPU-visible arena | phase 0-29 全通过；AXI64 RGB24/BGR24 source cross-beat 行为与 N6 一致 |

四款芯片的 raw GPU2D/NemaP 覆盖面已经收敛。当前没有任何一款实测实例报告 `CONFIG.VG=1` 或 `CONFIG.ZBUF=1`，因此本轮代码不把 NemaVG path/Bezier/stroke/fill rule 或硬件 Z-buffer 当成 raw GPU2D 已验证能力。

## 代码改动清单

### 待提交文件组

`FeatherCore/apps`：

- `apps/examples/nemap_demo/CMakeLists.txt`
- `apps/examples/nemap_demo/Kconfig`
- `apps/examples/nemap_demo/Make.defs`
- `apps/examples/nemap_demo/Makefile`
- `apps/examples/nemap_demo/nemap_demo_main.c`
- `apps/examples/nemap_demo/nemap_demo_tsc6_partial1.inc`
- `apps/examples/nemap_demo/nemap_demo_tsc6a_alpha.inc`

`FeatherCore/nuttx`：

- `arch/arm/src/stm32u5/stm32_gpu2d.c`
- `arch/arm/src/stm32u5/stm32_gpu2d.h`
- `arch/arm/src/stm32u5/hardware/stm32_gpu2d.h`
- `arch/arm/src/stm32n6/stm32_gpu2d.c`
- `arch/arm/src/stm32n6/stm32_gpu2d.h`
- `arch/arm/src/stm32n6/hardware/stm32_gpu2d.h`
- `arch/arm/src/stm32n6/Kconfig`
- `arch/arm/src/stm32n6/Make.defs`
- `arch/arm/src/stm32n6/hardware/stm32n6xxx_memorymap.h`
- `arch/arm/src/stm32n6/hardware/stm32n6xxx_rcc.h`
- `arch/arm/src/stm32h7rs/stm32_gpu2d.c`
- `arch/arm/src/stm32h7rs/stm32_gpu2d.h`
- `arch/arm/src/stm32h7rs/hardware/stm32_gpu2d.h`
- `arch/arm/src/stm32h7rs/Kconfig`
- `arch/arm/src/stm32h7rs/Make.defs`
- `arch/arm/src/stm32h7rs/hardware/stm32_memorymap.h`
- `arch/arm/src/stm32h7rs/hardware/stm32_rcc.h`
- `arch/arm/include/stm32h7rs/stm32h7s7xx_irq.h`
- `boards/arm/stm32u5/stm32u5x9j-dk/configs/gpu2d-debug/`
- `boards/arm/stm32n6/stm32n6570-dk/configs/gpu2d-debug/`
- `boards/arm/stm32h7rs/stm32h7s78-dk/configs/gpu2d-debug/`

`FeatherCore` 主仓库：

- `docs/STM32_GPU2D_NEMAP_PORTING_RESULT.md`
- `tools/firmware/stm32u5x9j-dk/build-gpu2d-debug.sh`
- `tools/firmware/stm32n6570-dk/build-gpu2d-debug.sh`
- `tools/firmware/stm32h7s78-dk/build-gpu2d-debug.sh`
- `tools/vendor/stmicro/stm32cubeprogrammer/VERSIONS.md`

### NuttX GPU2D 后端

- `nuttx/arch/arm/src/stm32u5/stm32_gpu2d.c`
  - 扩展 STM32U5 GPU2D backend 的 command-list/root-ring 提交、完成等待、debug snapshot 与寄存器读写支持。
  - U5 保持每次提交前重建 root ring 的模型，scratch buffer 使用 internal SRAM；这是 U5G9/U5A9 当前稳定通过的路径。

- `nuttx/arch/arm/src/stm32u5/hardware/stm32_gpu2d.h`
  - 补齐当前 demo 需要的 GPU2D/NemaP register offset、status/debug/config bit 定义。

- `nuttx/arch/arm/src/stm32n6/stm32_gpu2d.c`、`nuttx/arch/arm/src/stm32n6/hardware/stm32_gpu2d.h`
  - 新增 STM32N6 GPU2D/NemaP backend。
  - 采用持久 root ring、tail append、全局单调 CLID、ring cache clean/order。
  - 资源必须放在 PSRAM GPU-visible 区域；静态 SRAM scratch 不作为 N6 GPU target。

- `nuttx/arch/arm/src/stm32h7rs/stm32_gpu2d.c`、`nuttx/arch/arm/src/stm32h7rs/hardware/stm32_gpu2d.h`
  - 新增 STM32H7RS/H7S7 GPU2D/NemaP backend。
  - 集成 GPU2D base address、RCC AHB5 clock/reset、completion/error IRQ，与 ST HAL/TouchGFX 的 `ITCTRL.CLC -> CLID` 完成语义对齐。
  - 与 N6 一样使用 PSRAM GPU-visible arena、持久 root ring 和单调 CLID。

- `nuttx/arch/arm/src/stm32u5/stm32_gpu2d.h`、`nuttx/arch/arm/src/stm32n6/Kconfig`、`nuttx/arch/arm/src/stm32h7rs/Kconfig`、`Make.defs`、memory map、RCC、IRQ 相关文件
  - 接入 `CONFIG_STM32U5_GPU2D`、`CONFIG_STM32N6_GPU2D`、`CONFIG_STM32H7RS_GPU2D`。
  - 让对应 arch build 自动编入 GPU2D backend。

### `nemap_demo`

- 新增 `apps/examples/nemap_demo/`
  - 统一命令：`nemap_demo`。
  - 后端选择：`CONFIG_EXAMPLES_NEMAP_DEMO_STM32U5`、`CONFIG_EXAMPLES_NEMAP_DEMO_STM32N6`、`CONFIG_EXAMPLES_NEMAP_DEMO_STM32H7RS`。
  - 全量验证开关：`CONFIG_EXAMPLES_NEMAP_DEMO_RUN_VERIFIED_PHASES=y`。
  - offscreen 验证开关：`CONFIG_EXAMPLES_NEMAP_DEMO_RUN_OFFSCREEN_PHASES=y`。

`nemap_demo` 当前覆盖：

- phase 0-4：GPU2D identity/config decode、minimal command-list、IRQ completion。
- phase 5-10：RGB565 fill、visible pan/update、clip/dirty、source blit、ROP alpha blend、scale/filter。
- phase 11-14：triangle AA、destination/source color key、TEX3 A8 stencil/mask。
- phase 15-18：32/16/8/24-bit source format sweep、TSC6/TSC6A decompress、texture wrap clamp/repeat/border/mirror。
- phase 19-20：debug/status snapshot、depth capability safe skip。
- phase 21-23：affine textured box、textured quad、projective textured quad。
- phase 24-25：raw `DRAW_LINE` primitive、gradient register path。
- phase 26-27：VG capability boundary、NemaDC/display boundary。
- phase 28-29：textured triangle、circle raster span。

### Board Configs 与构建入口

- `nuttx/boards/arm/stm32u5/stm32u5x9j-dk/configs/gpu2d-debug/`
  - U5 GPU2D debug 配置，启用 framebuffer、GPU2D backend 与 `nemap_demo` 全量 phase。

- `nuttx/boards/arm/stm32n6/stm32n6570-dk/configs/gpu2d-debug/`
  - N6 GPU2D debug 配置，启用 PSRAM framebuffer、GPU2D backend 与 `nemap_demo` 全量 phase。

- `nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/configs/gpu2d-debug/`
  - H7RS GPU2D debug 配置，启用 PSRAM framebuffer、GPU2D backend 与 `nemap_demo` 全量 phase。

- `tools/firmware/stm32u5x9j-dk/build-gpu2d-debug.sh`
  - 生成 `build/stm32u5x9j-dk-gpu2d-debug.bin`，烧录到 internal Flash `0x08000000`。

- `tools/firmware/stm32n6570-dk/build-gpu2d-debug.sh`
  - 生成 `build/stm32n6570-dk-nxboot.bin`、`build/stm32n6570-dk-gpu2d-debug.bin`、`build/stm32n6570-dk-gpu2d-debug-full.bin`。
  - NXboot 位于 XSPI2 NOR `0x70000000`，app 位于 `0x70100000`，full image 从 `0x70000000` 烧录。

- `tools/firmware/stm32h7s78-dk/build-gpu2d-debug.sh`
  - 生成 `build/stm32h7s78-dk-nxboot.bin` 和 `build/stm32h7s78-dk-gpu2d-debug.bin`。
  - NXboot 烧录 internal Flash `0x08000000`，app image 烧录 XSPI2 NOR `0x70000000`，app vector 通常为 `0x70000400`。

## NemaP 能力归因

当前 raw GPU2D/NemaP 实机已证明的能力：

- command-list/root-ring submit、CLID completion、completion IRQ。
- RGB565 framebuffer/offscreen render target 写入。
- solid fill、source blit、ROP alpha blend。
- clip/dirty region。
- point-sampled scale、bilinear filter。
- triangle rasterizer AA。
- destination/source color key。
- TEX3 A8 stencil/mask。
- RGBX/RGBA/XRGB/ARGB8888、RGBA5650/RGBA5551/RGBA4444、L8/RGB332/RGB24/BGR24 source texture。
- TSC6/TSC6A source decompress/blit。
- texture wrap clamp/repeat/border/mirror。
- affine/projective texture matrix、textured quad、textured triangle。
- raw `DRAW_LINE` primitive。
- gradient register path。
- circle raster span：由 demo 生成 span/box command 组合完成，不是 native VG circle opcode。

当前未证明或明确 absent 的能力：

- `CONFIG.VG=0`：四款芯片当前 raw GPU2D 实例都没有报告硬件 NemaVG path engine。
- `CONFIG.ZBUF=0`、`CONFIG.ZCOMPR=0`：没有硬件 Z-buffer/Z-compression 能力证据，depth 测试保持 safe skip。
- NemaDC raw register block：当前 STM32 板级显示走 LTDC/DSI framebuffer path，demo 只验证 display boundary，不把独立 NemaDC 当成已接入外设。
- 硬件原生 path/Bezier/stroke/fill-rule：规格书或 TouchGFX/NemaVG API 中的路径词表不能直接等同于当前 raw command opcode。当前圆、圆角矩形等形状在 demo/SDK 语义下按 line/box/span 组合或上层 rasterizer 处理。

## 四芯片差异

### STM32U5G9

- U5G9 的 `IP_VERSION=0x00210011`，与本地 H21.11 资料包更接近。
- `CONFIGH=0x000003e3`，实测 `aa/decompress/tex-channels/mbist/blue-wrap/radial` 置位。
- `CONFIG.VG=0`，即使有 radial bit，也不能推导为 native VG path engine。
- 没有 AXI master bit，demo 的 command/ring/offscreen scratch 必须使用 internal SRAM；framebuffer 仍在 PSRAM。

### STM32U5A9

- `IP_VERSION=0x00010009`，与 N6/H7RS revision 相同，而不是 U5G9 的 `0x00210011`。
- `CONFIGH=0x00000003`，只保留 `aa/decompress`，没有 `tex-channels/radial/mbist/blue-wrap`。
- 当前 phase 0-29 与 U5G9 功能面基本对齐；RGB24/BGR24 strict source pass。

### STM32N6

- `CONFIG=0xf4010104`，`axi-master=yes`。
- command/ring/offscreen/source/mask/format buffer 必须放入 GPU2D 可访问的 PSRAM arena。
- backend 使用持久 root ring 与全局单调 CLID；这是 N6 稳定完成 phase 0-29 的关键提交模型。
- RGB24/BGR24 source strict compare 在跨 64-bit AXI beat 处有已知差异，当前按 limited pass 记录。

### STM32H7S7/H7RS

- GPU2D capability 与 N6 对齐：`CONFIG=0xf4010104`、`CONFIGH=0x00000023`、`axi-master=yes`、`tex-channels=yes`、`VG=no`、`radial=no`。
- backend 的 clock/reset/IRQ/base address 独立于 N6/U5，完成语义对齐 ST HAL/TouchGFX 的 GPU2D completion path。
- 资源放置与 ring 模型同 N6；RGB24/BGR24 source limited 行为也同 N6。

## 提交前检查点

- 确认 `apps/examples/nemap_demo/` 下没有 `.built`、`.depend`、`Make.dep`、`*.o` 等构建产物。
- `git -C FeatherCore status --short` 中预期看到 `apps`、`nuttx` 子模块变更，以及三个 `build-gpu2d-debug.sh`/vendor version 文档变更。
- `git -C FeatherCore/apps status --short` 中预期看到 `examples/nemap_demo/` 新增。
- `git -C FeatherCore/nuttx status --short` 中预期看到 U5/N6/H7RS GPU2D backend、Kconfig/Make.defs、board `gpu2d-debug` config 相关变更。
- 四个平台实机日志均已达到 `PASS all phases, restored colorbar`，作为本提交的有效性基线。
