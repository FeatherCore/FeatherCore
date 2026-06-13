# FRender 实现计划

本文是 `apps/graphics/frender` 的实现蓝图。

`frender` 是 FeatherCore 的应用侧公共 Render backend，目标是吸收 FGFX 的核心思想，服务于 WING GUI 和 Pinion。

## 当前状态

当前 `apps/graphics/frender` 已落地第一阶段 seed。

当前已存在的相关模块：

```text
apps/graphics/frender
apps/graphics/wing
apps/graphics/pinion
apps/examples/frender_demo
apps/examples/wing_gui_demo
apps/examples/pinion_demo
```

当前 FRender 已经具备：

```text
RGBA8888 memory surface
append-only command list
classified backend capability declaration
backend registry
software backend
clear / fill_rect / stroke_rect / fill_quad / stroke_quad / blit / push_clip / pop_clip
framebuffer present adapter
NuttX framebuffer capability adapter
RGBA8888 到 RGB565/RGB24/RGB32/RGBA32/RGBT32 的基础 present 转换
FNV-1a checksum helper
frender_demo NSH demo
```

当前 FRender 尚未具备：

```text
NX present adapter
DMA2D/GPU2D adapter
real planner split
resource table
WING GUI integration
Pinion integration
```

## 能力分类与注册

NuttX graphics 当前没有统一的 capability registry，因此 FRender 不等待底层提供一个不存在的总入口。

当前策略是：

```text
FRender 定义自己的能力分类
FRender backend/adapter 主动声明或探测能力
FRender registry 保存这些能力
WING/Pinion 后续只查询 FRender registry
```

能力按类别拆分，而不是继续使用一个扁平 mask：

```text
draw_caps
  command list / clear / fill rect / stroke rect / fill quad / stroke quad / clip / blit / text / mesh / shader

present_caps
  framebuffer / LCD / NX / update rect / vsync

memory_caps
  surface / mmap / direct pointer / pixel write

blend_caps
  global alpha / pixel alpha / color key

transform_caps
  scale / rotate / flip

sync_caps
  update rect / vsync wait / fence

format_caps
  RGBA8888 / RGB565 / RGB24 / RGB32 / RGBT32 / RGBA32
```

当前 registry API：

```text
fr_backend_registry_reset()
fr_backend_register()
fr_backend_register_builtin()
fr_backend_register_nuttx_graphics()
fr_backend_register_fb_presenter()
fr_backend_registry_count()
fr_backend_registry_get()
fr_backend_registry_find()
```

当前能力查询 API：

```text
fr_backend_supports()
  兼容旧 command-kind 查询，同时读取 legacy caps 和分类 draw_caps。

fr_backend_supports_draw()
fr_backend_supports_present()
fr_backend_supports_memory()
fr_backend_supports_blend()
fr_backend_supports_transform()
fr_backend_supports_sync()
fr_backend_supports_format()
  用于后续 planner / WING / Pinion 按分类查询能力。
```

当前 builtin 注册流程：

```text
fr_backend_register_builtin()
  -> 注册 software backend
  -> fr_backend_register_nuttx_graphics()
```

当前 NuttX graphics adapter 注册项：

```text
nuttx-graphics
  编译期概览项，汇总当前配置中可见的 NuttX graphics 出口。

nuttx-framebuffer-config
  编译期 framebuffer 出口声明，来自 CONFIG_VIDEO_FB / CONFIG_FB_UPDATE。
  只说明系统支持 framebuffer 路径，不说明真实分辨率和当前像素格式。

nuttx-framebuffer
  运行期 framebuffer 设备能力，来自 fr_fb_presenter_open("/dev/fb0") 后的
  FBIOGET_VIDEOINFO / FBIOGET_PLANEINFO / mmap 结果。

nuttx-lcd
  编译期 LCD device 出口声明，来自 CONFIG_LCD_DEV。

nuttx-nx
  编译期 NX compositor/window 能力声明，来自 CONFIG_NX、CONFIG_NX_UPDATE、
  CONFIG_NX_RAMBACKED、CONFIG_NXFONTS、CONFIG_NX_DISABLE_*BPP。

nuttx-dma2d
  编译期 2D 加速器声明，来自 STM32 DMA2D 相关配置。
  当前仅作为能力声明，不代表 FRender 已经能提交 DMA2D 命令。

nuttx-gpu2d
  编译期 GPU2D/NemaP 加速器声明，来自 STM32U5/N6/H7RS GPU2D 配置。
  当前仅作为能力声明，不代表 FRender 已经能提交 GPU2D command list。
```

当前 adapter 能力分类：

```text
software
  draw: command list / clear / fill rect / stroke rect / fill quad / stroke quad / blit / clip
  memory: surface / pixel write
  format: RGBA8888

nuttx-framebuffer-config
  present: framebuffer / update rect
  memory: mmap / direct pointer / pixel write
  format: RGB565 / RGB24 / RGB32 / RGBT32 / RGBA32

nuttx-framebuffer
  present: framebuffer / update rect
  memory: mmap 或 direct pointer / pixel write
  format: 由 /dev/fb0 当前格式决定
  max size: 由 /dev/fb0 当前分辨率决定

nuttx-lcd
  present: LCD
  memory: pixel write

nuttx-nx
  draw: fill rect / stroke rect / clip / bitmap blit / optional text
  present: NX / optional update rect
  memory: optional RAM-backed surface

nuttx-dma2d
  draw: fill rect，部分配置可声明 blit/blend
  sync: fence-like completion

nuttx-gpu2d
  draw: command list / clear / fill rect / blit / clip
  blend: global alpha / pixel alpha / color key
  transform: scale
  sync: fence-like completion
```

当前 framebuffer 探测流程：

```text
fr_fb_presenter_open("/dev/fb0")
  -> open framebuffer
  -> FBIOGET_VIDEOINFO
  -> FBIOGET_PLANEINFO
  -> mmap 或 direct fbmem

fr_backend_caps_from_fb_presenter()
  -> 根据实际 xres/yres/fmt/bpp/stride/mmap/update rect 生成 nuttx-framebuffer caps

fr_backend_register_fb_presenter()
  -> 注册或更新 nuttx-framebuffer backend
```

注意：

```text
nuttx-graphics 是编译期/概览能力，表示当前系统可能有 framebuffer/NX 等显示出口。
nuttx-framebuffer 是打开 /dev/fb0 后得到的运行期真实 framebuffer 能力。
当前仍没有把 FRender command list 直接提交给 nuttx/graphics。
```

## 定位

`frender` 负责：

```text
surface
command list
resource handle
backend capability
planner
software backend
fallback
framebuffer/NX present adapter
future DMA2D/GPU2D adapter
```

`frender` 不负责：

```text
WING GUI object tree / widget / layout / style
WING Desktop window manager / launcher / package manager
Pinion game loop / scene / sprite / tilemap / camera
NuttX framebuffer/LCD/DMA2D/GPU2D driver implementation
```

## 推荐目录结构

最终建议：

```text
apps/graphics/frender/
  Kconfig
  Make.defs
  Makefile
  CMakeLists.txt
  README.md

  include/frender/
    frender.h
    frender_surface.h
    frender_command.h
    frender_backend.h
    frender_caps.h
    frender_resource.h
    frender_planner.h
    frender_present.h

  src/core/
    frender_surface.c
    frender_command.c
    frender_resource.c

  src/planner/
    frender_planner.c
    frender_capability.c
    frender_fallback.c
    frender_cost.c

  src/backends/software/
    frender_sw_backend.c

  src/backends/nuttx/
    frender_fb_present.c
    frender_nx_present.c
    frender_nuttx_caps.c

  src/backends/hw/
    frender_dma2d_backend.c
    frender_gpu2d_backend.c
```

第一阶段已实现：

```text
include/frender/frender.h
src/frender.c
apps/examples/frender_demo
```

## 第一阶段最小能力

目标是先形成一个可验证闭环：

```text
create surface
create command list
push clear / fill_rect / stroke_rect
execute by software backend
compute checksum
run from NSH demo
```

第一阶段 command：

```text
FR_CMD_CLEAR
FR_CMD_FILL_RECT
FR_CMD_STROKE_RECT
FR_CMD_PUSH_CLIP
FR_CMD_POP_CLIP
```

第一阶段 surface format：

```text
FR_FORMAT_RGBA8888
```

第一阶段 backend：

```text
software backend
classified NuttX graphics capability adapters
framebuffer present adapter
```

第一阶段 demo：

```text
apps/examples/frender_demo
```

`frender_demo` 会打印：

```text
software capability snapshot
nuttx graphics capability snapshot
rendered command count
RGBA8888 checksum
```

注意：当前 NuttX graphics 还没有通用 command-list submit 接口，所以第一阶段不是把 FRender command list 直接交给 `nuttx/graphics`。当前模式是：

```text
上层 -> FRender command list -> FRender software backend -> memory surface
memory surface -> framebuffer present adapter -> /dev/fb0 或 sim X11 framebuffer
```

后续 framebuffer/NX/DMA2D/GPU2D adapter 会在 FRender 内把 frame plan 翻译成公开 NuttX graphics API、driver ioctl 或 board accelerator callback。

## 当前 sim 验证记录

已完成 sim 构建：

```sh
./tools/firmware/sim/build-wing.sh
```

构建输出：

```text
executable: ../build/sim-wing
demos:      frender_demo, wing_gui_demo
```

已执行 NSH 验证：

```sh
timeout 8s bash -lc 'printf "frender_demo\n" | FeatherCore/build/sim-wing'
```

关键输出：

```text
frender_demo: software caps name=software mask=0x0000007f format=0
frender_demo: nuttx caps name=nuttx-graphics mask=0x00000080 format=0
frender_demo: stage=clear commands=1 checksum=0x6d3c61c5
frender_demo: stage=fill_rect commands=2 checksum=0xb0ec85c5
frender_demo: stage=stroke_rect commands=3 checksum=0x5ca9cdc5
frender_demo: stage=clip_push_fill_pop commands=9 checksum=0x2d8737c5
frender_demo: final rendered 320x240 commands=9 checksum=0x2d8737c5
frender_demo: framebuffer 640x480 fmt=13 bpp=32 stride=2560
frender_demo: framebuffer present ok
```

说明：

```text
sim X11 framebuffer 现在采用按需显示策略：
系统启动和进入 NSH 时不主动映射窗口；
执行 frender_demo 后，demo 打开 /dev/fb0，窗口才显示；
用户点击 X 关闭窗口后，frender_demo 退出并返回 NSH。
```

当前验证状态：

```text
已完成 build-wing.sh --no-clean 构建验证。
窗口关闭交互路径已经接入 sim X11 ClientMessage/DestroyNotify 事件。
```

```text
frender_demo 已能从 NSH 执行
FRender command list 和 software backend 可用
FRender framebuffer present adapter 可用
当前 NuttX graphics 仍是 present/capability adapter target，不是 command-list submit target
```

## 后续 command 扩展

第二阶段：

```text
FR_CMD_BLIT
FR_CMD_BLIT_RESOURCE
FR_CMD_DRAW_GLYPH
FR_CMD_DRAW_TEXT_RUN
FR_CMD_DRAW_LINE
FR_CMD_DRAW_PATH
FR_CMD_BEGIN_LAYER
FR_CMD_END_LAYER
FR_CMD_SET_OPACITY
FR_CMD_SET_TRANSFORM
```

第三阶段：

```text
FR_CMD_DRAW_SPRITE_BATCH
FR_CMD_DRAW_TILEMAP
FR_CMD_BEGIN_3D_PASS
FR_CMD_DRAW_MESH
FR_CMD_END_3D_PASS
FR_CMD_POST_PROCESS
```

## Backend capability

`frender_backend_caps` 应显式描述后端能力：

```text
can_fill
can_stroke
can_blit
can_alpha_blend
can_color_key
can_clip
can_scale
can_rotate
can_affine
can_render_to_texture
can_async_submit
can_present
can_readback
supports_2d_profile
supports_3d_profile
preferred_format
alignment
max_texture_width
max_texture_height
max_command_count
```

原则：

```text
硬件只声明真实支持的能力
planner 根据能力选择 native path 或 software fallback
software backend 永远是正确性基准
```

## Planner 路线

第一阶段 planner 可以非常简单：

```text
所有命令走 software backend
```

但接口要提前保持：

```text
command list
  -> planner
  -> frame plan
  -> backend execute
```

后续再扩展：

```text
native chain
fallback chain
command island
resource requirement
cost policy
```

## 和 WING GUI 的关系

WING GUI 不应该长期直接写 pixels。

目标：

```text
WING object tree
  -> layout
  -> style resolve
  -> render frontend
  -> frender command list
  -> frender planner/backend
```

`wing_gui_demo` 后续应从“直接画到软件 surface”升级为：

```text
创建 WING GUI object tree
生成 frender command list
用 frender software backend 渲染
输出 checksum 或 present
```

## 和 Pinion 的关系

Pinion 不应该维护独立 framebuffer/software renderer 路线。

目标：

```text
Pinion scene / sprite / tilemap / camera
  -> Pinion render builder
  -> frender command list
  -> frender planner/backend
```

Pinion 决定游戏世界画什么，frender 决定怎么在当前 backend 上画。

## 和 NuttX graphics 的关系

`frender` 位于 `apps/graphics`，不进入 `nuttx/graphics`。

推荐边界：

```text
frender
  查询/使用公开 NuttX graphics 能力

nuttx/graphics + drivers
  提供 framebuffer / LCD / NX / DMA2D / GPU2D / update / sync 等机制
```

第一阶段可以不查询真实硬件能力，只提供 software caps。

后续通过：

```text
ioctl
board callback
Kconfig static caps
device node query
公开 accelerator adapter
```

把 NuttX/board 能力转换为 `frender_backend_caps`。

## Kconfig 建议

```text
config GRAPHICS_FRENDER
  bool "Feather render backend"
  default n

config GRAPHICS_FRENDER_SW
  bool "FRender software backend"
  default y
  depends on GRAPHICS_FRENDER

config GRAPHICS_FRENDER_FB_PRESENT
  bool "FRender framebuffer present backend"
  default n
  depends on GRAPHICS_FRENDER

config GRAPHICS_FRENDER_NX_PRESENT
  bool "FRender NX present backend"
  default n
  depends on GRAPHICS_FRENDER

config GRAPHICS_FRENDER_DMA2D
  bool "FRender DMA2D backend"
  default n
  depends on GRAPHICS_FRENDER

config GRAPHICS_FRENDER_GPU2D
  bool "FRender GPU2D backend"
  default n
  depends on GRAPHICS_FRENDER
```

WING GUI 和 Pinion 后续应：

```text
select GRAPHICS_FRENDER
select GRAPHICS_FRENDER_SW
```

## 实施路线

```text
Milestone 1: apps/graphics/frender + frender_demo
Milestone 2: wing_gui_demo 改为通过 frender 渲染
Milestone 3: framebuffer present adapter
Milestone 4: Pinion 接入 frender
Milestone 5: capability planner / dirty / fallback stats
Milestone 6: DMA2D/GPU2D adapter
Milestone 7: 2D/3D profile
```
