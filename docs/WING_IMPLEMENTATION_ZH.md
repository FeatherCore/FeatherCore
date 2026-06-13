# WING 实现计划

本文记录 WING 当前应该采用的实现方向。

## 定位

WING 首先是 GUI 库，不是桌面系统本身。

```text
WING GUI
  object tree / event / layout / style / animation / render frontend

WING Desktop
  基于 WING GUI 实现的可选默认桌面系统
```

任何 WING GUI 程序都可以直接从 NSH、init、board launcher 等方式启动，不强制依赖 WING Desktop。

如果一个 WING GUI 程序想进入 WING Desktop 的安装、下载、launcher、icon、权限和生命周期体系，则需要额外打包为 Desktop App Package。

## 当前代码状态

当前相关目录：

```text
apps/graphics/wing
apps/examples/wing_gui_demo
```

`wing_gui_demo` 是第一阶段验证入口。它应保持为不依赖 WING Desktop 的 NSH demo。

## 分层目标

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

FRender
  command list / render planner / fallback / software / hardware acceleration

        |
        v

NuttX graphics
  framebuffer / LCD / NX / DMA2D / GPU2D / input / timer
```

## 从 HoneyGUI / LVGL / TouchGFX 吸收什么

WING GUI 不复制任何一个库，但吸收它们的优秀思想。

从 LVGL 吸收：

```text
object tree
event system
state/style system
layout system
display/input/timer 抽象
轻量、可移植、无强依赖
```

从 HoneyGUI 吸收：

```text
嵌入式 GUI 的视觉表现力
2.5D/3D widget 预留
SVG/vector/Lottie/particle/glass/video 等能力方向
acceleration engine 边界意识
PC sim 和工具链意识
```

从 TouchGFX 吸收：

```text
面向资源受限 MCU 的组件模板
widget / container / mixin 分层
高级产品组件思路
work-buffer 和资源预算契约
动画 easing / duration / transition 思路
```

## 推荐目录结构

```text
apps/graphics/wing/
  include/wing/
    wing.h
    wing_obj.h
    wing_event.h
    wing_style.h
    wing_layout.h
    wing_anim.h
    wing_display.h
    wing_widget.h

  src/core/
    wing_obj.c
    wing_tree.c
    wing_event.c
    wing_focus.c
    wing_app.c

  src/style/
    wing_style.c
    wing_theme.c

  src/layout/
    wing_layout_abs.c
    wing_layout_box.c

  src/anim/
    wing_anim.c
    wing_timeline.c
    wing_easing.c

  src/render/
    wing_render_frontend.c

  src/widgets/
    wing_label.c
    wing_button.c
    wing_image.c
    wing_panel.c
    wing_canvas.c

  src/views/
    wing_view.c
    wing_view_stack.c
```

注意：`wing/src/render` 只负责 GUI render frontend，把 object tree 展开成 FRender command list。真正的 planner/backend/fallback 不应放在 WING 内部。

## 第一阶段目标

第一阶段只做 WING GUI，不做 WING Desktop。

```text
1. wing_surface / wing_context / 基础绘制 seed
2. wing_obj / object tree
3. wing_event / pointer/key/focus
4. wing_style / 基础状态样式
5. wing_layout_abs / box layout
6. wing_label / wing_button / wing_panel
7. wing_render_frontend 输出 frender command list
8. wing_gui_demo 从直接画 rect 升级为 object tree demo
```

## WING Desktop 后续定位

WING Desktop 是 WING GUI 上的一个大型可选应用。

它负责：

```text
launcher
taskbar / dock
window manager
wallpaper
app package manager
notification
settings
desktop shell
```

它不应该污染 WING GUI 的基础层。

## 避免的错误方向

```text
不要把 WING GUI 直接做成 Desktop
不要让 widget 直接调用 framebuffer/DMA2D/GPU2D
不要把 render planner 塞进每个 widget
不要第一阶段追求完整控件数量
不要第一阶段追求完整 3D/粒子/视频效果
```

## 当前推荐下一步

```text
1. 先落地 frender 最小实现
2. 让 wing_gui_demo 通过 frender 渲染
3. 再补 WING object tree / event / style / layout
4. 最后再开始 WING Desktop 模板
```
