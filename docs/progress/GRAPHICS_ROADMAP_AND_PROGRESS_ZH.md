# Feather 图形路线计划与进度

本文记录 FeatherCore 当前图形系统的总计划、真实进度和下一步任务。

## 当前总架构

目标架构：

```text
Independent UI App
  使用 WING GUI 开发的普通 UI 应用

WING Desktop
  默认桌面系统 / 桌面模板 / 最大的基础 WING 应用

Game App
  使用 Pinion 开发的游戏应用

        |
        v

WING GUI / Pinion
  GUI 语义 / 游戏语义

        |
        v

FRender
  command list / planner / fallback / software backend / hardware adapter

        |
        v

NuttX graphics
  framebuffer / LCD / NX / DMA2D / GPU2D / input / timer

        |
        v

Hardware
  panel / touch / keys / accelerator
```

核心决策：

```text
FRender 放在 apps/graphics/frender
WING GUI 放在 apps/graphics/wing
Pinion 放在 apps/graphics/pinion
Render backend 不放进 nuttx/graphics
nuttx/graphics 负责底层显示和硬件机制
```

## 当前真实代码状态

当前已经存在：

```text
apps/graphics/frender
apps/graphics/wing
apps/graphics/pinion
apps/examples/frender_demo
apps/examples/wing_gui_demo
apps/examples/pinion_demo
```

当前尚未存在：

```text
FRender NX present adapter
FRender DMA2D/GPU2D adapter
WING GUI -> FRender integration
Pinion -> FRender integration
```

当前 `wing_gui_demo` 是 WING GUI 第一阶段验证入口，应保持为不依赖 WING Desktop 的 NSH demo。

当前 `pinion_demo` 是 Pinion seed demo，后续应迁移到基于 FRender 的渲染路径。

## 已完成

### 文档和架构整理

已完成：

```text
WING GUI 与 WING Desktop 分层澄清
FRender 独立于 WING/Pinion 的架构决策
FRender 不进入 nuttx/graphics 的边界确认
Pinion 基于 FRender 的方向确认
NuttX graphics 当前能力整理
apps/graphics 当前能力整理
旧 Wing 原型/重构过程文档移出当前 docs
新增 docs/progress 作为计划和进度目录
```

### WING GUI seed

已完成：

```text
apps/graphics/wing 重新作为 WING GUI seed
include/wing/wing.h
src/wing_gui.c
software RGBA8888 surface
基础 clear/fill/draw rect
wing_gui_demo NSH demo
sim defconfig 启用 wing_gui_demo
tools/firmware/sim/build-wing.sh 改为构建 WING GUI demo
```

注意：

```text
当前 WING GUI 还不是完整 object tree GUI
当前 WING GUI 还没有接 FRender
当前 WING Desktop 尚未开始重建
```

### FRender first-stage seed

已完成：

```text
apps/graphics/frender
apps/examples/frender_demo
RGBA8888 memory surface
append-only command list
backend capability declaration
software backend
clear/fill_rect/stroke_rect/clip commands
framebuffer present adapter
sim framebuffer present
sim X11 framebuffer 按需开窗：执行 frender_demo 才显示窗口，关闭窗口后 demo 退出
checksum demo
sim defconfig 启用 frender_demo
```

### 旧实现隔离

已完成：

```text
旧 frender / wing / wingdemo / wing_desktop_demo 已移到 FeatherCore/tmp
当前 docs 中旧过程文档已移到 FeatherCore/tmp/docs_removed
```

## 当前进行中

当前主线应该进入：

```text
WING GUI 接入 FRender
```

原因：

```text
WING GUI 和 Pinion 都应该基于 FRender
不能继续让 WING GUI 自己承担 render backend
也不应该让 Pinion 自己维护独立 framebuffer renderer
```

## 下一步优先级

### P0：落地 FRender 最小闭环

目标目录：

```text
apps/graphics/frender
apps/examples/frender_demo
```

最小能力：

```text
RGBA8888 memory surface
append-only command list
clear
fill_rect
stroke_rect
software backend
checksum demo
Kconfig / Makefile / CMakeLists
```

完成条件：

```text
frender_demo 可以从 NSH 执行
frender_demo 可以创建 command list
frender_demo 可以通过 software backend 渲染
frender_demo 可以输出 checksum
```

状态：

```text
已完成代码落地
已完成 sim 构建
已完成 NSH frender_demo 验证
已完成 frender_demo 窗口生命周期调整：NSH 启动不弹窗，demo 执行时弹窗，关闭窗口后返回 NSH
```

### P1：WING GUI 接入 FRender

目标：

```text
wing_gui_demo 不再直接写 pixels
WING GUI 输出 frender command list
FRender software backend 执行绘制
```

完成条件：

```text
wing_gui_demo 仍可从 NSH 执行
输出 checksum
WING GUI 与 Render backend 职责分离
```

### P2：补 WING GUI 核心模型

目标：

```text
wing_obj
object tree
event system
style seed
layout seed
label/button/panel seed
render frontend
```

完成条件：

```text
wing_gui_demo 创建 object tree
layout 后生成 frender command list
支持最小 pointer/key synthetic event
```

### P3：Pinion 接入 FRender

目标：

```text
Pinion render builder 输出 frender command list
pinion_demo 通过 FRender software backend 渲染
```

完成条件：

```text
pinion_demo 不直接维护独立 software renderer 路线
Pinion 和 WING GUI 共享 FRender surface/command/backend
```

### P4：Framebuffer present adapter

目标：

```text
FRender 增加 framebuffer present adapter
支持 /dev/fb0
支持常见 framebuffer format 转换
支持 dirty rect update seed
```

完成条件：

```text
frender_demo 或 wing_gui_demo 可以在 sim framebuffer 上显示
软件 surface 到 framebuffer present 由 FRender 负责
```

### P5：Backend capability / planner seed

目标：

```text
frender_backend_caps
frender_plan
software caps
framebuffer present caps
dirty-aware linear planner
```

完成条件：

```text
command list -> planner -> plan -> backend execute
即使第一版所有 draw 都走 software，也保留 planner 边界
```

### P6：硬件加速 adapter

目标：

```text
DMA2D fill/blit adapter
GPU2D/NemaGFX/VG-Lite adapter seed
capability query
fallback path
```

完成条件：

```text
硬件能力只作为 fast path
software backend 仍可完整执行
planner 能识别 unsupported command 并 fallback
```

## 暂不做

当前不做：

```text
完整 WING Desktop
跨进程 compositor/server
完整 app package manager
完整 3D pipeline
完整 Pinion ECS
完整字体排版系统
真实 DMA2D/GPU2D submit
大规模修改 nuttx/graphics
```

原因：

```text
现在最关键的是先把 WING/Pinion/FRender 分层立稳
```

## 已废弃或暂时移出的路线

不再采用：

```text
旧 fgfx 直接作为 apps/graphics 子系统恢复
WING GUI 直接承担 render backend
Pinion 直接维护独立 framebuffer renderer
把 Render backend 放进 nuttx/graphics
直接从 WING Desktop 开始实现
旧 wingdemo 作为当前主验证入口
```

历史内容位置：

```text
FeatherCore/tmp
FeatherCore/tmp/docs_removed
```

## 当前判断

当前第一步已从“创建 FRender”推进到：

```text
让 WING GUI 接入 FRender
```

理由：

```text
WING GUI 已经有最小 seed
Pinion 已经有独立 seed
但两者缺共同渲染底座
FRender 是下一步的架构枢纽
```
