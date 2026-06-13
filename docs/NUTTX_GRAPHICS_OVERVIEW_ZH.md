# NuttX Graphics 概览

本文整理 `nuttx/graphics` 下的图形能力，并说明它和 FeatherCore 计划中的
`wing` 桌面引擎、`pinion` 游戏引擎之间的关系。

## 总体结论

`nuttx/graphics` 提供的是 NuttX 原生的 NX 图形系统。它是一个面向嵌入式的
小型窗口和绘图基础设施，建立在 framebuffer 或 LCD driver 之上；它不是
Linux 世界中 GL/Vulkan/Mesa/Wayland/X11 那类完整 3D/桌面图形栈。

对 FeatherCore 来说，这意味着：

- `wing` 应该位于 `apps/graphics/wing`，作为桌面/UI 引擎。
- `pinion` 应该位于 `apps/graphics/pinion`，作为游戏引擎。
- 两者都应该通过 framebuffer、LCD、NX、update hook 等公开接口使用 NuttX 图形能力。
- 两者都不应该直接 include `nuttx/graphics` 内部私有实现文件。

推荐的分层目标是：

```text
桌面 / 游戏应用
  Wing desktop / Pinion game engine
        |
应用侧图形后端
  framebuffer backend / optional NX backend / optional accelerator backend
        |
NuttX 图形层
  NX / NXBE / NXGLIB / NxTerm / framebuffer or LCD driver APIs
        |
硬件
  framebuffer / LCD controller / DMA2D / GPU2D / panel / touch / keys
```

## 目录结构

`nuttx/graphics` 当前主要包含这些部分：

| 路径 | 作用 |
|---|---|
| `nuttx/graphics/Kconfig` | NX 图形系统的配置入口，包括像素深度、输入、窗口边框、NxTerm、NX server 等选项。 |
| `nuttx/graphics/nxglib` | 底层图形辅助库：设置像素、填充/复制/移动/读取矩形、填充梯形、光标绘制/擦除/备份，以及每窗口 framebuffer 辅助逻辑。 |
| `nuttx/graphics/nxbe` | NX backend/window engine：窗口排序、裁剪、重绘、填充、bitmap、移动、大小/可见性、光标、modal 状态、display update 通知。 |
| `nuttx/graphics/nxmu` | 多用户 NX 支持：启动 NX server、打开窗口、转发 client/server 消息、处理键盘/鼠标输入、redraw/report 回调。 |
| `nuttx/graphics/nxterm` | 基于 NX 窗口的终端支持：把 NX window 包装成类似终端的 driver，支持字体渲染、滚动、重绘、VT100、可选键盘输入路由。 |

## `nxglib`：像素和矩形基础绘图

`nxglib` 是该目录中最低层的绘图辅助层。

它为多种像素深度提供实现：

- 1 bpp
- 2 bpp
- 4 bpp
- 8 bpp
- 16 bpp
- 24 bpp
- 32 bpp

核心操作包括：

- 设置单个像素
- 填充矩形
- 读取矩形
- 复制矩形
- 移动矩形
- 填充梯形
- 在启用软件光标时绘制、擦除、备份光标图像

重要子目录：

| 路径 | 含义 |
|---|---|
| `nxglib/fb` | 面向 framebuffer 设备的操作。 |
| `nxglib/lcd` | 面向 LCD 设备的操作。 |
| `nxglib/pwfb` | `CONFIG_NX_RAMBACKED=y` 时使用的 per-window framebuffer 辅助逻辑。 |
| `nxglib/cursor` | 软件光标辅助实现。 |

对 Wing/Pinion 来说，`nxglib` 最好被视为 NX 的内部支撑层。应用侧自研渲染器通常不应该直接调用私有 `nxglib` 文件。更合适的做法是：渲染到应用侧 surface，然后通过 `/dev/fb0` 等公开后端提交；如果确实需要，也应该添加干净的公开 backend wrapper。

## `nxbe`：NX 后端和窗口引擎

`nxbe` 是 NX 窗口管理和组合绘制的核心后端。它维护窗口状态，并判断哪些区域需要绘制。

主要职责包括：

- 将 bitmap 绘制到窗口
- 填充矩形和梯形
- 将绘制裁剪到可见区域
- 创建/关闭窗口
- 提升/降低窗口层级
- modal window 行为
- 移动和调整窗口大小
- 设置窗口位置、大小、可见性
- 判断区域是否可见
- 将 redraw 传播给下层窗口
- 在底层支持时从显示设备读取矩形
- 配置软件/硬件光标时处理光标
- `CONFIG_NX_UPDATE=y` 时通知外部 display update hook

相关配置：

| 配置 | 作用 |
|---|---|
| `CONFIG_NX_RAMBACKED` | 启用 per-window RAM backing buffer。可以减少 redraw callback 复杂度，但消耗更多 RAM。 |
| `CONFIG_NX_SWCURSOR` | 为 framebuffer 设备和指定 bpp 模式启用软件光标。 |
| `CONFIG_NX_HWCURSOR` | 实验性的硬件光标路径。 |
| `CONFIG_NX_UPDATE` | 启用 update 通知，适用于串行 LCD、VNC 类输出或 framebuffer update callback。 |

对 Wing 来说，`nxbe` 在概念上接近窗口/合成后端。但如果 Wing 想拥有自己的 scene graph、layout、style 和桌面行为，就不应该变成 `nxbe` 的薄封装。更好的拆分是：

```text
Wing scene/layout/widgets/compositor policy
        |
Wing backend abstraction
        |
framebuffer backend now, optional NX backend later
```

## `nxmu`：NX 多用户 server 支持

`nxmu` 实现 NX server/client 的多用户通信部分。它负责启动 NX server，并在客户端和后端之间路由消息/事件。

主要职责包括：

- 启动 NX server
- 打开 client window
- 发送 client 和 window 消息
- 处理 redraw 消息
- 报告窗口位置变化
- 请求和释放 background window
- 转发键盘输入
- 转发鼠标/触摸输入

相关配置：

| 配置 | 作用 |
|---|---|
| `CONFIG_NX_BLOCKING` | 让 `nx_eventhandler()` 阻塞直到收到消息。 |
| `CONFIG_NX_MXSERVERMSGS` | server message queue 最大消息数。 |
| `CONFIG_NX_MXCLIENTMSGS` | client message queue 最大消息数。 |
| `CONFIG_NXSTART_SERVERPRIO` | NX server 线程优先级。 |
| `CONFIG_NXSTART_SERVERSTACK` | NX server 线程栈大小。 |
| `CONFIG_NXSTART_EXTERNINIT` | 允许 board-specific 代码通过 `board_graphics_setup()` 初始化显示硬件。 |
| `CONFIG_NXSTART_DEVNO` | 启动时使用的 LCD device number。 |

对 FeatherCore 来说，如果 Wing 以后想使用现有 NX server 模型，`nxmu` 会很重要。但 Wing 和 Pinion 的第一阶段，用直接 framebuffer 后端会更简单，也更容易掌控。

## `nxterm`：NX 上的终端窗口

`nxterm` 提供一个基于 NX 的 terminal driver。当图形系统需要终端窗口，或者需要把 console 输出显示在 NX window 内时，它很有用。

主要职责包括：

- 注册 NX terminal device
- 使用字体绘制终端文本
- 缓存渲染后的 glyph
- 处理滚动和清屏
- 重绘终端内容
- 调整终端窗口大小
- 基本 VT100 处理
- 可选 NX 键盘输入路由

重要选项：

| 配置 | 作用 |
|---|---|
| `CONFIG_NXTERM` | 启用 NxTerm。 |
| `CONFIG_NXTERM_BPP` | NxTerm 使用的像素深度。 |
| `CONFIG_NXTERM_MXCHARS` | 为 redraw 记住的最大字符数。 |
| `CONFIG_NXTERM_CACHESIZE` | 字体 glyph cache 大小。 |
| `CONFIG_NXTERM_NOWRAP` | 长行截断而不是自动换行。 |
| `CONFIG_NXTERM_NXKBDIN` | 通过 NX focus/window callback 路由键盘输入，而不是直接从 console 输入。 |

对 Wing 来说，NxTerm 以后可以作为终端窗口应用的基础，但不适合作为通用 widget 或游戏渲染器的基础。

## 顶层 NX 配置

主开关是：

```text
CONFIG_NX=y
```

启用 `CONFIG_NX` 会选择：

```text
CONFIG_NXGLIB=y
CONFIG_NXFONTS=y
```

重要顶层选项：

| 配置 | 含义 |
|---|---|
| `CONFIG_NX_LCDDRIVER` | 当启用 LCD 支持时，使用 LCD driver interface，而不是 framebuffer interface。 |
| `CONFIG_NX_NDISPLAYS` | 最大显示数量，默认 1。 |
| `CONFIG_NX_NPLANES` | color plane 数量，通常为 1。 |
| `CONFIG_NX_RAMBACKED` | 启用可选 RAM-backed windows。 |
| `CONFIG_NX_BGCOLOR` | 初始背景色。 |
| `CONFIG_NX_ANTIALIASING` | 在支持的 bpp 和 framebuffer 设备上启用线条抗锯齿。 |
| `CONFIG_NX_WRITEONLY` | 表示图形设备不能 read back。 |
| `CONFIG_NX_UPDATE` | 启用 display update hooks。 |
| `CONFIG_NX_PACKEDMSFIRST` | 小于 8 bpp 像素格式的 packing 顺序。 |

像素深度通过这些 disable 开关控制：

```text
CONFIG_NX_DISABLE_1BPP
CONFIG_NX_DISABLE_2BPP
CONFIG_NX_DISABLE_4BPP
CONFIG_NX_DISABLE_8BPP
CONFIG_NX_DISABLE_16BPP
CONFIG_NX_DISABLE_24BPP
CONFIG_NX_DISABLE_32BPP
```

默认配置会关闭不少像素深度以节省内存。对 Wing 和 Pinion 来说，实际目标通常是 `16 bpp` RGB565 或 `32 bpp` XRGB/RGBA，具体取决于 board framebuffer 格式和内存带宽。

## 输入支持

NX 提供 pointer 和 keyboard 输入配置：

| 配置 | 含义 |
|---|---|
| `CONFIG_NX_XYINPUT_MOUSE` | X/Y 输入是 mouse-like。 |
| `CONFIG_NX_XYINPUT_TOUCHSCREEN` | X/Y 输入是 touchscreen-like。 |
| `CONFIG_NX_KBD` | 启用 keyboard/keypad 输入支持。 |

对 FeatherCore 来说，输入最好抽象在 NX 之上：

```text
/dev/input* or board input driver
        |
Feather input abstraction
        |
Wing pointer/key focus or Pinion game input
```

这样 Wing/Pinion 既可以使用直接 framebuffer 渲染，也可以以后切换到 NX backend。

## 和 Wing 的关系

Wing 是桌面/UI 引擎。它需要：

- scene tree 或 entity tree
- layout
- styling
- text
- invalidation 和 dirty rectangles
- input routing 和 focus
- window/application model
- compositor policy
- framebuffer 或 display backend

`nuttx/graphics` 最适合在 backend 层帮助 Wing：

| Wing 需求 | NuttX graphics 支持 |
|---|---|
| 显示输出 | framebuffer 或 LCD driver APIs，可选 NX display update hooks。 |
| 窗口后端 | 如果 Wing 以后想集成 NX window，可以考虑 NX/NXBE。 |
| dirty region 思路 | NXBE redraw/clipping 逻辑是有价值的参考。 |
| 文本终端应用 | NxTerm 可用于终端窗口。 |
| pointer/key input | NX 有输入能力，但 Wing 应保留自己的输入抽象。 |

推荐的第一阶段 Wing 路径：

```text
Wing scene graph
  -> Wing software renderer
  -> Wing framebuffer backend
  -> /dev/fb0 and FBIO_UPDATE when available
```

推荐的后续 Wing 路径：

```text
Wing scene graph
  -> Wing compositor/backend interface
  -> framebuffer backend
  -> optional NX backend
  -> optional DMA2D/GPU2D acceleration backend
```

## 和 Pinion 的关系

Pinion 是游戏引擎。它需要：

- 可预测的 frame loop
- sprite/tile rendering
- animation
- input polling/events
- 以后可接 audio
- asset/resource management
- 面向低功耗板子的可选 dirty rectangles
- 可选全屏 double buffering

`nuttx/graphics` 对 Pinion 的帮助主要在 display access 和 update hooks，而不是 NX window 语义。

推荐的第一阶段 Pinion 路径：

```text
Pinion world/game loop
  -> Pinion software renderer
  -> RGBA/RGB565 frame surface
  -> framebuffer backend
  -> /dev/fb0
```

推荐的后续 Pinion 路径：

```text
Pinion renderer abstraction
  -> software backend
  -> DMA2D/GPU2D blit/fill backend where available
  -> framebuffer update/pan backend
```

对游戏来说，直接 framebuffer 通常比 NX 更简单，因为游戏通常拥有整个屏幕并连续渲染。

## `nuttx/graphics` 不是什么

`nuttx/graphics` 当前不是：

- OpenGL
- Vulkan
- Mesa
- Wayland
- X11
- 高层 UI toolkit
- 完整 game engine
- retained-mode desktop shell
- GPU scheduler 或 Linux DRM/KMS 类系统

它更接近：

```text
小型嵌入式 windowing + drawing primitives + framebuffer/LCD abstraction
```

这足以支撑 FeatherCore 构建自己的嵌入式桌面/游戏图形栈，但 Wing 和 Pinion 仍然需要自己的高层 engine 代码。

## 实用设计规则

保持这个边界：

```text
apps/graphics/wing and apps/graphics/pinion
  可以使用公开 NuttX headers 和 device APIs
  不应该 include nuttx/graphics/nxbe、nxglib、nxmu、nxterm 私有文件
```

好的依赖：

```c
#include <nuttx/video/fb.h>
#include <nuttx/lcd/lcd.h>
```

潜在的未来公开集成点：

```text
NX client APIs
framebuffer ioctls
LCD driver APIs
FBIO_UPDATE
board display initialization
board input initialization
```

应避免的依赖：

```c
#include "nxbe.h"
#include "nxmu.h"
#include "nxglib.h"
#include "nxterm.h"
```

除非我们明确决定做 NuttX 内部组件，否则这些都应视为实现细节。

## 建议的下一步

1. 保持 Wing 和 Pinion 作为 `apps/graphics` 下的应用侧 engine。
2. 添加明确的 backend 文件，例如 `wing_backend_fb.c` 和 `pinion_backend_fb.c`，不要把 framebuffer 代码长期混在 demo 里。
3. 第一阶段使用 `/dev/fb0`、`FBIOGET_VIDEOINFO`、`FBIOGET_PLANEINFO`、`FBIO_UPDATE` 作为稳定后端契约。
4. 补充 RGB565 和 XRGB8888 转换路径，因为嵌入式 board 常见这两种 framebuffer 格式。
5. 把 NX 视为 Wing 的可选未来后端，而不是 Wing 的身份。
6. 把直接 framebuffer 作为 Pinion 的默认后端。
