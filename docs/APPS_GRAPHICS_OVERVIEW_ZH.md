# Apps Graphics 概览

本文整理 `apps/graphics` 下的图形相关组件，并说明它们和 FeatherCore 图形路线的关系。

`apps/graphics` 是应用侧图形层。它位于 `nuttx/graphics` 这个 NuttX 内核/系统图形层之上，包含 UI engine、window manager、图像库、输入测试工具和显示工具。

## 总体结论

`apps/graphics` 不是一个单一图形框架，而是一组图形栈和辅助库的集合：

- 传统 NX/NxWidgets 桌面组件
- LVGL、PDCurses 等第三方图形/UI/TUI 库
- 图像和颜色转换库
- 输入/调试工具
- 显示和截图工具
- Feather 原生引擎：`wing` 和 `pinion`

对 FeatherCore 的方向来说，关键拆分是：

```text
apps/graphics/wing      Feather desktop/UI engine
apps/graphics/pinion    Feather game engine
apps/graphics/nx*       existing NuttX NX UI/window ecosystem
apps/graphics/lvgl      independent third-party UI library
apps/graphics/lib*      image/color helper libraries
```

Wing 和 Pinion 应被视为 Feather 原生 engine。它们可以借鉴 NX 生态，但不应变成 NxWidgets、NxWM、Twm4NX 或 LVGL 的 wrapper。

## 和 `nuttx/graphics` 的分层关系

推荐的理解模型：

```text
应用和 demo
        |
apps/graphics
  Wing / Pinion / LVGL / NxWidgets / NxWM / tools / image libs
        |
公开 NuttX 图形 API
  framebuffer / LCD / NX client APIs / input devices
        |
nuttx/graphics
  NX / NXBE / NXGLIB / NxTerm
        |
drivers and hardware
```

`apps/graphics` 里的组件应该依赖公开 NuttX headers 和 device APIs，而不是 `nuttx/graphics` 内部私有实现文件。

## 顶层 Kconfig

顶层 `apps/graphics/Kconfig` 汇总这些模块：

| 模块 | 配置入口 |
|---|---|
| `ft80x` | `CONFIG_GRAPHICS_FT80X` |
| `frender` | `CONFIG_GRAPHICS_FRENDER` |
| `input` | `CONFIG_GRAPHICS_INPUT_*` |
| `jpgresizetool` | `CONFIG_GRAPHICS_JPGRESIZETOOL` |
| `libjpeg` | `CONFIG_LIBJPEG` |
| `libyuv` | `CONFIG_LIBYUV` |
| `lvgl` | `CONFIG_GRAPHICS_LVGL` |
| `nxglyphs` | `CONFIG_NXGLYPHS_*` |
| `nxwidgets` | `CONFIG_NXWIDGETS` |
| `nxwm` | `CONFIG_NXWM` |
| `pdcurs34` | `CONFIG_GRAPHICS_PDCURSES` |
| `pinion` | `CONFIG_GRAPHICS_PINION` |
| `screenshot` | `CONFIG_GRAPHICS_SCREENSHOT` |
| `slcd` | `CONFIG_GRAPHICS_SLCD` |
| `tiff` | `CONFIG_TIFF` |
| `twm4nx` | `CONFIG_GRAPHICS_TWM4NX` |
| `wing` | `CONFIG_GRAPHICS_WING` |

## Feather 原生引擎

### `apps/graphics/frender`

FRender 是 FeatherCore 的应用侧公共渲染核心。

它吸收 FGFX 的 command list、software fallback、backend capability planning 思想，但不直接放入 `nuttx/graphics`。

当前职责：

- compact command list
- RGBA8888 software surface
- clear/fill/stroke/text/clip/triangle commands
- software executor
- framebuffer presenter
- Wing 当前已经通过 FRender 执行渲染

推荐方向：

```text
Wing / Pinion render builder
        |
FRender command list
        |
software executor / future hardware planner
        |
framebuffer presenter / LCD / accelerator backend
```

### `apps/graphics/wing`

Wing 是 FeatherCore 的原生 desktop/UI engine。

当前职责：

- 拥有自己的 public API 和 scene model
- 提供 root、panel、label、button、image、custom 等基础 node 类型
- 提供简单 style 和 rectangle primitives
- 渲染到 RGBA software surface
- 当前 demo 位于 `apps/examples/wingdemo`

推荐方向：

```text
Wing scene graph / layout / widgets / focus
        |
Wing renderer abstraction
        |
framebuffer backend first
        |
optional NX or acceleration backend later
```

Wing 应负责 Feather 的 desktop shell、panel、window、application surface、focus policy、UI style 和 compositor behavior。

Wing 不应该被实现成 LVGL、NxWidgets、NxWM 或 Twm4NX。这些是独立图形栈。

### `apps/graphics/pinion`

Pinion 是 FeatherCore 的轻量级 game engine layer。

当前职责：

- 提供小型 game-oriented C API
- 拥有 game surfaces 和简单 2D drawing primitives
- 计划扩展到 sprites、tile maps、frame timing、input routing、scene/ECS integration
- 当前 demo 位于 `apps/examples/pinion_demo`

推荐方向：

```text
Pinion game loop / scene / sprite / tile systems
        |
Pinion renderer abstraction
        |
software framebuffer backend first
        |
optional DMA2D/GPU2D backend later
```

对游戏来说，直接 framebuffer 渲染通常比 NX windowing 更简单，因为游戏通常拥有整个屏幕，并自己控制 frame loop。

## NX 生态组件

这些模块是现有 NuttX 应用侧 UI/window 生态，建立在 `CONFIG_NX` 之上。

### `apps/graphics/nxwidgets`

NxWidgets 是面向 NX 的 C++ widget toolkit。

重要特点：

- 需要 `CONFIG_NX=y`
- 需要 C++ 支持，即 `CONFIG_HAVE_CXX=y`
- 提供 widgets、widget control、graphics ports、colors、fonts、timing 和 event behavior
- 支持 flicker reduction、server startup、event waiting、BPP、greyscale/RGB、custom fonts、fill colors、edge colors、text colors、cursor control、keyboard buffer size、double-click timing、repeat timing 等配置

在 FeatherCore 中的作用：

- 可作为 embedded widget toolkit 的参考
- 如果某些 board/demo 已依赖传统 NuttX NX UI 栈，它仍然有用
- 如果 Wing 要成为 Feather 原生桌面 engine，则不建议把 NxWidgets 作为 Wing 基础

### `apps/graphics/nxwm`

NxWM 是基于 NX 和 NxWidgets 的小型 window manager。

重要特点：

- 依赖 `CONFIG_NXWIDGETS=y`
- 提供 taskbar/start-window 风格的 embedded desktop model
- 支持 icons、app windows、toolbar height、taskbar placement、minimize/stop buttons、colors、fonts、background images
- 历史上面向小型嵌入式 LCD 产品，其中通常只有一个 app window 处于主要显示状态

在 FeatherCore 中的作用：

- 可作为 NuttX desktop/window manager 结构参考
- 可借鉴 taskbar/application launching 思路
- 如果 Feather 想拥有自己的 desktop identity 和 API，则不应让 NxWM 替代 Wing

### `apps/graphics/twm4nx`

Twm4NX 是另一个 NX-based window manager，风格上接近传统 X11/TWM。

重要特点：

- 依赖 `CONFIG_NX=y` 和 `CONFIG_HAVE_CXX=y`
- 选择 `CONFIG_NX_RAMBACKED=y`
- 选择 `CONFIG_NXWIDGETS=y`
- 支持 priority、stack size、network initialization、VNC server mode、keyboard/mouse enablement、mouse/touchscreen mode、device paths、classic/contemporary appearance、icon manager columns、debug output 等配置

在 FeatherCore 中的作用：

- 可作为更传统 window manager 模型的参考
- 适合测试 NX 的 mouse/touch/VNC 路径
- 不是 Wing 的目标基础

### `apps/graphics/nxglyphs`

NxGlyphs 存储 NX 生态使用的图形 glyph/icon 资源。

重要特点：

- 包含 image assets 和生成/源码形式的 glyph code
- 通过 `CONFIG_NXGLYPHS_LARGE_ICONS` 支持大图标
- 通过 `CONFIG_NXGLYPHS_BACKGROUNDCOLOR` 配置背景色

在 FeatherCore 中的作用：

- 对 NxWM/Twm4NX 和传统 NX demos 有用
- 不应作为 Wing 或 Pinion 的通用 asset system

### `apps/graphics/slcd`

SLCD 是 segment LCD emulation 组件。

重要特点：

- 依赖 `CONFIG_NX=y`
- 依赖 C++ 支持
- 选择 `CONFIG_NXWIDGETS=y`
- 基于 NX/NxWidgets 思路实现 segment-LCD-like drawing

在 FeatherCore 中的作用：

- 适合 segment LCD simulation 或特殊显示 demo
- 不是 Wing/Pinion 路线的核心

## 第三方 UI 和 TUI 库

### `apps/graphics/lvgl`

LVGL 是独立的第三方 embedded UI library。

重要特点：

- 通过 `CONFIG_GRAPHICS_LVGL` 启用
- 拥有自己的 configuration model、color formats、memory settings、OS settings、draw engines 和可选 acceleration paths
- 支持 software rendering 和大量 LVGL-specific rendering options
- 在配置支持时，可通过 LVGL 自己的抽象接入 Arm-2D、VG-Lite、PXP、Dave2D 等加速路径

在 FeatherCore 中的作用：

- 可作为独立 UI 选项或兼容/demo 栈
- 应与 Wing 保持分离
- Wing 不应依赖 LVGL、包装 LVGL 或表现为 LVGL facade

### `apps/graphics/pdcurs34`

PDCurses 是 text user interface library。

重要特点：

- 通过 `CONFIG_GRAPHICS_PDCURSES` 启用
- 选择 NuttX font support
- 提供 curses 风格 TUI programming model
- 适用于 terminal-like interface，而不是图形桌面 widgets

在 FeatherCore 中的作用：

- 适合 text-mode tools 和 diagnostics
- 可能可用于 terminal 或 console applications
- 不是 desktop compositor 或 game renderer

## 图像、颜色和文件格式库

### `apps/graphics/libjpeg`

Libjpeg 提供 JPEG decoding/encoding 支持。

重要特点：

- 通过 `CONFIG_LIBJPEG` 启用
- 有 version 和 temporary-directory 配置
- 可支持需要 JPEG assets 的 tools 或 apps

在 FeatherCore 中的作用：

- 以后可用于 Wing 或 Pinion 加载 image assets
- 可以把解码后的图像数据送入 engine-owned surfaces

### `apps/graphics/jpgresizetool`

JPGResizeTool 是 JPEG resize 工具。

重要特点：

- 通过 `CONFIG_GRAPHICS_JPGRESIZETOOL` 启用
- 依赖 `CONFIG_LIBJPEG`

在 FeatherCore 中的作用：

- 适合 preprocessing 或 resizing JPEG assets
- 除非某个 app 显式使用，否则不属于 runtime rendering path

### `apps/graphics/libyuv`

Libyuv 提供 image format conversion 和 scaling 功能。

重要特点：

- 通过 `CONFIG_LIBYUV` 启用
- 有 branch/source 配置选项
- 适合 YUV/RGB conversion、scaling 和 image manipulation paths

在 FeatherCore 中的作用：

- 对 camera/video paths 有用
- 如果 Pinion 或 Wing 以后需要快速颜色转换或图像缩放，它会有用

### `apps/graphics/tiff`

TIFF 提供 TIFF file generation 支持。

重要特点：

- 通过 `CONFIG_TIFF` 启用
- 包含 initialization、strips、finalization 和 utility 逻辑

在 FeatherCore 中的作用：

- 主要支持 screenshot/file output paths
- 不是 rendering backend

## 显示和截图工具

### `apps/graphics/screenshot`

Screenshot 将 NX framebuffer/window output 捕获成 TIFF。

重要特点：

- 通过 `CONFIG_GRAPHICS_SCREENSHOT` 启用
- 依赖 `CONFIG_TIFF=y`
- 依赖 `CONFIG_NX=y`
- 可配置 width、height、format

在 FeatherCore 中的作用：

- 适合 NX-based screenshot capture
- 可作为未来 Wing/Pinion screenshot features 的参考
- 当前依赖 NX，因此并不是自动通用的 framebuffer screenshot tool

## 输入工具

### `apps/graphics/input`

`input` 目录包含图形/输入测试和生成工具。

重要入口：

| 配置 | 作用 |
|---|---|
| `CONFIG_GRAPHICS_INPUT_GENERATOR` | input tools 共享的 generator support。 |
| `CONFIG_GRAPHICS_INPUT_MONKEY` | monkey/randomized input tool，选择 uinput touch/buttons 和 print extensions。 |
| `CONFIG_GRAPHICS_INPUT_TOOL` | input tool，支持 uinput touch/buttons/mouse。 |
| `CONFIG_GRAPHICS_INPUT_GETEVENT` | event monitor，类似 Linux `getevent` 的用途。 |

在 FeatherCore 中的作用：

- 适合测试 touch/mouse/key event delivery
- 以后可用于 fuzzing Wing input behavior
- 可用于验证 Pinion game input paths

推荐未来使用方式：

```text
input device
  -> graphics input test/generator tools
  -> Feather input abstraction
  -> Wing focus/routing or Pinion controls
```

## 特殊硬件支持

### `apps/graphics/ft80x`

FT80x 是 FTDI/BridgeTek FT80x graphics controller 芯片库。

重要特点：

- 通过 `CONFIG_GRAPHICS_FT80X` 启用
- 提供 command buffer、display-list、RAM command/display/global memory、registers、GPIO、audio、backlight、co-processor、touch helpers
- 支持 buffer size、signal numbers、audio buffer offset/size、debug output levels 等配置

在 FeatherCore 中的作用：

- 仅对带 FT80x 类外部显示控制器的 board 有用
- 不是通用 Wing/Pinion backend
- 如果这类硬件重要，可作为 specialized backend

## 按类别归纳

| 类别 | 模块 |
|---|---|
| Feather 原生引擎 | `wing`, `pinion` |
| Feather 渲染核心 | `frender` |
| NX widget/window 生态 | `nxwidgets`, `nxwm`, `twm4nx`, `nxglyphs`, `slcd`, `screenshot` |
| 第三方 UI/TUI | `lvgl`, `pdcurs34` |
| 图像和转换库 | `libjpeg`, `jpgresizetool`, `libyuv`, `tiff` |
| 输入工具 | `input` |
| 硬件专用图形支持 | `ft80x` |

## 推荐的 FeatherCore 策略

### 保持为独立图形栈

这些应保持为独立选项，不应合并进 Wing：

```text
apps/graphics/lvgl
apps/graphics/nxwidgets
apps/graphics/nxwm
apps/graphics/twm4nx
apps/graphics/pdcurs34
```

它们可以共存于代码树中，但 Wing 不应该依赖它们。

### 作为参考或可选后端

这些对 Wing/Pinion 设计有参考价值：

```text
nxwidgets    widget toolkit ideas
nxwm         embedded desktop/taskbar ideas
twm4nx       more traditional window manager ideas
screenshot   capture/output utility ideas
input        event testing/fuzzing ideas
```

### 作为支持库使用

这些以后可能成为 Wing/Pinion 的实用支持库：

```text
libjpeg      image asset loading
libyuv       scaling and color conversion
tiff         screenshot/export output
input        input testing and generated events
```

### 把 Wing 和 Pinion 作为 Feather 一等 API

Wing 和 Pinion 应定义 Feather 应用面向的 API：

```text
Wing: desktop, widgets, windows, compositor policy
Pinion: games, sprites, scenes, frame loop, game input
```

底层差异应该隐藏在 backend interface 后面：

```text
frender
  command list / software executor / future hardware planner

wing_backend_fb.c
wing_backend_nx.c        optional later
wing_backend_accel.c     optional later

pinion_backend_fb.c
pinion_backend_dma2d.c   optional later
pinion_backend_gpu2d.c   optional later
```

## 和当前路线的关系

当前路线建议是：

1. 保持 `wing` 和 `pinion` 位于 `apps/graphics` 下。
2. 使用 `frender` 作为 Wing/Pinion 共享的应用侧渲染命令核心。
3. 保持 LVGL 独立，且与 Wing 无关。
4. 保留 NX/NxWidgets/NxWM/Twm4NX 作为现有 NuttX 生态组件和参考，而不是新 Feather desktop 的身份。
5. 先为 FRender/Wing/Pinion 构建 framebuffer present/backend，使用公开 NuttX framebuffer APIs。
6. 以后再添加可选 NX、DMA2D、GPU2D 或 board-specific acceleration backend，且不改变高层 Wing/Pinion APIs。

## 一句话总结

`apps/graphics` 是应用侧图形工具箱。Wing 和 Pinion 应成为这个工具箱中 Feather 原生的高层 engine；NX、LVGL、PDCurses、图像库、输入工具和特殊硬件支持则作为独立可复用组件存在于它们周围。
