# WING GUI 与 WING Desktop 架构澄清

本文用于重新明确 WING 的定位。

当前结论：

```text
WING 不应该直接等同于桌面系统。

WING 应该首先是一个 GUI 库。
WING Desktop 是建立在 WING GUI 之上的默认桌面系统实现。
```

也就是说，WING 应该分成两层：

```text
WING GUI
  通用 GUI / 图形应用框架

WING Desktop
  基于 WING GUI 实现的默认桌面系统模板
```

## 一、总体分层

```text
Independent UI App
  使用 WING GUI 开发的普通 UI 应用

WING Desktop
  默认桌面系统 / 桌面模板 / 最大的基础 WING 应用

        |
        v

WING GUI
  object tree / event / layout / style / animation / render frontend

        |
        v

Render backend
  command list / render planner / fallback / software / hardware acceleration

        |
        v

NuttX graphics
  framebuffer / LCD / NX / DMA2D / GPU2D / input / timer

        |
        v

Hardware
  panel / touch / keys / accelerator
```

## 二、WING GUI 的定位

WING GUI 是基础库。

它应该提供：

- object tree。
- widget/component 基础。
- event dispatch。
- input abstraction。
- layout。
- style/theme。
- animation/tick。
- invalidation/dirty tracking。
- render frontend。
- display/input/timer/memory/assets port。

WING GUI 应该允许开发者直接开发独立 UI 应用程序。

例如：

```text
Calculator App
Settings App
Music App
Device Dashboard
Game Launcher
Custom Industrial Panel
```

这些应用可以直接基于 WING GUI 运行，并不一定需要 WING Desktop。

换句话说：

```text
任何 WING GUI 程序都不强制要求运行在 WING Desktop 上。
```

WING GUI 程序可以通过 NSH、系统启动脚本、应用入口函数或其他平台机制直接启动。

WING Desktop 只是 WING 提供的一个完善、可选、默认的桌面系统环境。

## 三、WING Desktop 的定位

WING Desktop 是一个基于 WING GUI 的默认桌面系统。

它可以理解为：

```text
WING Desktop = 最大的、最基础的、官方默认的 WING 应用程序
```

它提供一个默认桌面模板，包括：

- desktop root。
- window manager。
- taskbar / dock。
- launcher。
- app container。
- focus/activation policy。
- app lifecycle glue。
- notification / status area。
- desktop theme。
- optional tile desktop。

WING Desktop 不是 WING GUI 本身。

如果用户不需要桌面系统，可以关闭 WING Desktop，只使用 WING GUI 开发独立应用。

## 四、独立 WING 应用与桌面关系

### 情况一：只使用 WING GUI

```text
App
  |
WING GUI
  |
Render backend / NuttX graphics
```

这种模式适合：

- 单一全屏 UI。
- 嵌入式仪表盘。
- 设置界面。
- 简单控制面板。
- 不需要多窗口/桌面管理的设备。

特点：

- 没有桌面载体。
- App 自己拥有 root scene。
- App 自己决定全屏布局和导航。
- App 可以从 NSH、init script、board app launcher 或其他系统入口直接启动。
- 不需要安装到 WING Desktop。

### 情况二：使用 WING Desktop

```text
App
  |
WING Desktop app container
  |
WING Desktop
  |
WING GUI
  |
Render backend / NuttX graphics
```

这种模式适合：

- 多应用环境。
- 多窗口。
- launcher。
- app switching。
- notification/status。
- 桌面级 UI shell。

特点：

- WING Desktop 提供桌面载体。
- App 运行在 Desktop 的 app container 或 window 中。
- Desktop 管理 focus、activation、window、taskbar、launcher。
- App 需要多一步打包/注册操作，才能支持桌面安装、下载、图标展示、启动器入口、权限/资源声明等桌面级能力。

这时应用不再只是一个普通 WING GUI 程序，而是：

```text
WING Desktop App Package
```

它可以包含：

- app metadata。
- app icon。
- entry point。
- resource manifest。
- permission/capability declaration。
- install/update information。
- optional desktop integration hints。

### 情况三：用户实现自己的 Desktop

```text
Custom Desktop
  |
WING GUI
```

用户可以不用 WING Desktop，自己实现桌面系统。

但自定义桌面必须提供 WING Desktop 模板中的“必须项”，否则独立 WING 应用没有标准载体。

## 五、WING Desktop 模板的必须项与可选项

### 必须项

一个桌面系统至少需要：

- desktop root。
- app container。
- focus manager。
- input routing。
- render root。
- lifecycle entry。
- app activation policy。

这些是“让 WING 应用能被承载”的最低要求。

### 推荐项

推荐提供：

- window manager。
- taskbar 或 dock。
- launcher。
- status area。
- notification area。
- default theme。

这些不是最低运行要求，但构成正常桌面体验。

### 可选项

可以按需启用：

- tile desktop。
- desktop widgets。
- wallpaper。
- compositor effects。
- 2.5D/3D desktop affordance。
- app switcher。
- virtual desktop。
- global menu。

## 六、对现有代码的处理

## 六、WING GUI App 与 WING Desktop App Package

需要明确区分两个概念：

```text
WING GUI App
  普通 WING GUI 应用程序
  可直接运行
  不依赖 WING Desktop

WING Desktop App Package
  面向 WING Desktop 环境的应用包
  在 WING GUI App 基础上增加 metadata / icon / manifest / install 信息
  可被 Desktop 安装、下载、展示和启动
```

### WING GUI App

普通 WING GUI App 只需要依赖 WING GUI。

启动方式可以是：

```text
NSH command
init script
board-specific launcher
test/demo entry
direct app main
```

它不需要：

- Desktop app metadata。
- Desktop launcher icon。
- Desktop install manifest。
- Desktop app container。

### WING Desktop App Package

如果一个 WING GUI App 想进入 WING Desktop 管理体系，则需要打包成 Desktop App Package。

这一步不是运行 WING GUI 的必要条件，而是接入桌面系统的必要条件。

Desktop App Package 解决的问题是：

- 如何安装。
- 如何下载。
- 如何显示图标。
- 如何出现在 launcher。
- 如何声明入口。
- 如何声明资源。
- 如何由 Desktop 启动、暂停、关闭或切换。

因此关系应该是：

```text
WING GUI App
  可直接运行

WING GUI App + Desktop packaging
  可被 WING Desktop 管理
```

## 七、对现有代码的处理

当前已移动到：

```text
FeatherCore/tmp/frender
FeatherCore/tmp/wing
FeatherCore/tmp/wingdemo
FeatherCore/tmp/wing_desktop_demo
```

这些代码应视为：

```text
prototype archive
```

它们可以用于参考：

- 之前的 API 草案。
- demo 场景。
- render command 思路。
- dirty region 思路。
- Tile/3D Card 试验。

但不应直接作为新 WING 架构继续堆功能。

## 八、新目录建议

后续重新实现时，建议目录结构改为：

```text
apps/graphics/wing/
  include/wing/
    wing.h
    wing_app.h
    wing_object.h
    wing_event.h
    wing_style.h
    wing_layout.h
    wing_render.h

  src/core/
    object tree
    event
    input
    layout
    style
    tick
    invalidation

  src/widgets/
    basic widgets

  src/render/
    render frontend
    command list
    render planner
    software backend
    backend capability

  src/port/
    nuttx display/input/timer/memory/assets port

apps/graphics/wing_desktop/
  include/wing_desktop/
    wing_desktop.h
    wing_desktop_template.h
    wing_desktop_package.h

  src/
    desktop root
    window manager
    launcher
    taskbar
    notification
    app container
    package registry
    installer/downloader integration
    default theme

apps/examples/wing_gui_demo/
  standalone WING GUI app demo

apps/examples/wing_desktop_demo/
  WING Desktop demo
```

也可以先把 `wing_desktop` 放在：

```text
apps/graphics/wing/desktop/
```

但从概念上，应清楚它是建立在 WING GUI 之上的默认应用，而不是 GUI core 本身。

## 九、吸收 HoneyGUI / LVGL / FGFX 的方式

新 WING 不是简单复制某个库，而是综合三者思想。

### HoneyGUI 启发

吸收：

- GUI engine 分层。
- core/widget/engine/dc/input/server 思路。
- 2.5D/3D 能力作为引擎能力。
- port layer。
- feature 裁剪。

### LVGL 启发

吸收：

- object tree。
- event model。
- style/theme。
- layout。
- invalid area。
- display/input driver。
- widget API 稳定性。

### FGFX / third/graphics 启发

吸收：

- command list。
- backend capability matrix。
- fallback planner。
- resource manifest。
- software oracle / golden tests。
- cost policy。
- hardware backend 不确定时的规划机制。

## 十、第一阶段 POC

不要一开始实现完整桌面。

第一阶段只做：

```text
WING GUI POC
  object tree
  event dispatch
  style/layout
  render command list
  software framebuffer backend
  one button
  one panel

WING Desktop POC
  desktop root
  app container
  one window
  launcher placeholder
  one packaged app metadata record
```

完成标准：

- WING GUI 可以独立运行一个简单 UI app。
- WING Desktop 可以承载一个 WING GUI app。
- GUI app 不依赖 Desktop 也可以运行。
- Desktop 关闭后，WING GUI 仍可用。
- 同一个 GUI app 可以通过直接入口运行，也可以通过 Desktop package metadata 被 Desktop 启动。

## 十一、核心结论

最终应该形成：

```text
WING GUI 是库。
WING Desktop 是基于 WING GUI 的默认桌面应用/模板。
独立 WING 应用可以不依赖 WING Desktop。
独立 WING 应用可以从 NSH/init/board launcher 等方式直接启动。
如果需要桌面载体，可以使用默认 WING Desktop。
如果希望被 WING Desktop 安装、下载、展示和启动，则需要额外 Desktop packaging。
如果不使用默认 Desktop，用户必须实现 Desktop 模板中的必须项。
```

这才是后续重新实现 WING 前必须先明确的基础。
