# WING GUI 设计参考：LVGL / TouchGFX / HoneyGUI

本文记录 WING GUI 应该如何吸收 `/third/lvgl`、`/third/touchgfx-open-repository` 和 `/third/HoneyGUI` 的优秀设计特质。

目标不是复制其中任何一个 GUI，而是把三者适合 Feather / NuttX / FRender 的思想提取出来，形成 WING GUI 自己的结构。

## 1. WING GUI 的定位

WING GUI 是 Feather 的通用 GUI 库。

它应该支持两种使用方式：

1. 独立 WING GUI 应用

   应用可以直接从 NSH 或其他入口启动，不需要依赖 WING Desktop。

2. WING Desktop 应用

   应用经过额外打包后，可以安装、展示、启动于 WING Desktop 桌面系统中。

WING Desktop 不是 WING GUI 的强制运行环境。WING Desktop 是一个建立在 WING GUI 之上的默认桌面系统实现。

## 2. 三个参考项目分别提供什么价值

### 2.1 LVGL：学习 GUI 内核组织方式

LVGL 的核心价值是它非常适合 MCU / RTOS 环境：

- C 语言优先。
- 低内存占用。
- 对显示驱动、输入驱动、tick、timer、object tree、event、style、layout、dirty redraw 都有清晰抽象。
- 应用只需要周期性调用 GUI handler，GUI 内部推进定时器、动画、事件和刷新。

WING GUI 应该重点吸收 LVGL 的这些思想：

- `wing_gui_t` 作为 GUI 执行句柄。
- 应用线程循环中调用 `wing_gui_tick()` / `wing_gui_step()`。
- GUI 内部维护软件定时器、动画、输入队列、事件派发、脏区、绘制调度。
- 使用轻量 object tree 表达 UI 层级。
- 使用状态化 style 系统表达普通态、按下态、聚焦态、禁用态等。
- 使用 display / input 抽象隔离硬件。
- 支持局部刷新，而不是每帧整屏重绘。

### 2.2 TouchGFX：学习组件产品化方式

TouchGFX 的开源仓库重点不是完整核心，而是大量 widget、container、mixin 和示例组件。

它的价值在于：

- 每个复杂控件都被封装为独立组件。
- 组件通常有自己的源码、头文件、说明、截图、示例。
- 面向真实产品 UI，重视仪表盘、图表、轮盘、轮播、二维码、进度条等高级控件。
- 强调可复用 UI 模板，而不只是基础 rectangle / label / button。

WING GUI 应该重点吸收 TouchGFX 的这些思想：

- `widgets/` 中每个控件都应独立、可复用、可演示。
- `containers/` 用于组合多个 widget，形成更高层 UI 单元。
- `mixins/` 或 behavior trait 用于复用拖动、点击、滚动、动画、焦点等行为。
- 每个重要 widget 都应该有 demo 和文档。
- WING GUI 不能停留在基础绘制能力，应逐步提供面向产品 UI 的高级组件。

### 2.3 HoneyGUI：学习引擎化、高级图形和工具链思想

HoneyGUI 的价值在于它更像一个完整 GUI 引擎：

- 有 engine / dc / input / server / core / widget 等分层。
- 有消息、监听器、输入算法、平台 API 包装。
- 有 vector graphics、matrix/math、2.5D / 3D 示例和资源转换工具。
- 强调 PC simulator、低代码、资源工具、配置化。
- 明确关注 dirty region optimization。

WING GUI 应该重点吸收 HoneyGUI 的这些思想：

- GUI 内部要有 engine 化结构，而不是把绘制、事件、组件混在一起。
- 输入、消息、事件、渲染、资源应分层。
- 把 2D 视为默认 3D 空间中的 identity transform 状态；camera / transform / projection 应进入 GUI core，而不是作为后期外挂的 3D view。
- 保留未来 vector graphics / mesh / shader / GPU 3D backend 的扩展口。
- 资源转换和工具链要尽早进入设计，而不是后期临时补。
- dirty region 不是优化小修小补，而应该是 GUI 刷新模型的一部分。
- simulator / demo / visual tools 对 GUI 迭代速度非常关键。

## 3. WING GUI 总体架构

推荐结构：

```text
Independent WING GUI App
WING Desktop App
        |
        v
WING GUI
  runtime handle / object tree / event / input
  style / layout / timer / animation / dirty region
  widgets / containers / resources / render frontend
        |
        v
FRender
  command list / render planner / backend capability
  software fallback / framebuffer present / hardware acceleration adapter
        |
        v
NuttX graphics
  framebuffer / LCD / NX / DMA2D / GPU2D / input / timer
        |
        v
Hardware
  panel / touch / keys / accelerator
```

WING GUI 不应该直接操作 framebuffer，也不应该直接知道 DMA2D / GPU2D 的细节。

WING GUI 的职责是把 UI 状态转换为 FRender command list。

FRender 的职责是根据后端能力选择软件绘制、硬件加速或 framebuffer present 路径。

NuttX graphics 的职责是提供实际显示、输入、底层图形设备和硬件加速入口。

## 4. WING GUI 应提取的核心特质

| 来源 | 应吸收的特质 | WING 中的落点 |
| --- | --- | --- |
| LVGL | GUI handler / tick 驱动 | `wing_gui_t`、`wing_gui_tick()`、`wing_gui_step()` |
| LVGL | object tree | `wing_obj_t` / `wing_widget_t` |
| LVGL | event bubbling / input abstraction | `wing_event_t`、`wing_input_event_t` |
| LVGL | style with state | `wing_style_t`、state selector |
| LVGL | layout | simple layout first，后续 flex/grid |
| LVGL | invalidation / partial redraw | dirty rect list / dirty merge |
| TouchGFX | widgets / containers / mixins | `widgets/`、`containers/`、`behaviors/` |
| TouchGFX | product-level complex widgets | gauge、graph、carousel、wheel、progress |
| TouchGFX | each widget has demo/doc | `apps/examples/wing_*_demo` |
| HoneyGUI | engine / server / message 分层 | GUI runtime、message queue、event dispatcher |
| HoneyGUI | matrix / vector / 2.5D / 3D | 默认 object space / camera / transform / projection，未来 mesh / shader backend |
| HoneyGUI | resource tools | image/font/theme converter |
| HoneyGUI | dirty region optimization | dirty region as first-class subsystem |

## 5. `wing_gui_t` 应该成为 GUI 执行句柄

每一个 WING GUI 应用都应该拥有自己的 `wing_gui_t`。

应用线程负责循环调用 WING GUI，WING GUI 负责内部推进 UI 状态。

推荐模型：

```c
while (wing_gui_is_running(&gui))
  {
    wing_gui_tick(&gui, elapsed_ms);
    wing_gui_step(&gui);
    usleep(frame_interval_us);
  }
```

`wing_gui_t` 应逐步维护这些内容：

- 当前 tick。
- 软件 timer 列表。
- animation 列表。
- input queue。
- event queue。
- object tree 根节点。
- focus / capture 状态。
- dirty region 列表。
- FRender command list。
- display / presenter 绑定。
- resource manager 引用。
- 当前主题 / style context。

当前已有的 `wing_gui_t` 可以视为第一阶段种子实现。

后续不应该让 demo 自己维护动画、脏区和绘制调度。demo 应该只是创建 GUI、创建对象、绑定事件，然后循环调用 GUI handler。

## 6. Object tree 设计

WING GUI 应该有统一对象基类。

建议基础字段：

```text
wing_obj_t
  parent
  first_child
  next_sibling
  bounds
  flags
  state
  style
  event_cb
  draw_cb
  layout_cb
  user_data
```

Widget、container、window、desktop item 都应该建立在 object tree 上。

这样可以统一处理：

- 坐标变换。
- hit test。
- focus。
- event bubbling。
- invalidation。
- layout。
- draw traversal。

## 7. Event / input 设计

WING GUI 需要把硬件输入转换为统一事件。

建议输入事件类型：

- pointer down。
- pointer move。
- pointer up。
- key down。
- key up。
- encoder rotate。
- focus next / previous。
- close request。

事件派发建议分为：

- input event：来自硬件或 simulator。
- gui event：对象生命周期、点击、值变化、布局变化、动画完成。
- system event：窗口关闭、屏幕变化、资源变化。

事件路径建议支持：

- target dispatch。
- bubble to parent。
- optional capture。
- stop propagation。

## 8. Style / layout 设计

WING GUI 应吸收 LVGL 的 style 思想，但第一版不要做得过重。

第一阶段 style：

- background color。
- border color。
- border width。
- radius。
- padding。
- text color。
- font。
- opacity。

第二阶段 style：

- normal / pressed / focused / disabled / checked 状态。
- theme。
- transition。

第一阶段 layout：

- fixed。
- stack vertical。
- stack horizontal。
- center。
- fill。

第二阶段 layout：

- flex。
- grid。
- anchor。

## 9. Dirty region 和绘制调度

Dirty region 应该是 WING GUI 的核心机制。

推荐流程：

```text
object state changed
        |
        v
wing_obj_invalidate(obj)
        |
        v
object bounds -> dirty rect
        |
        v
dirty rect merge / clip
        |
        v
draw traversal only for affected objects
        |
        v
emit FRender commands with dirty clip
        |
        v
FRender execute / present
```

这里吸收的是 LVGL 的 invalidation 思想和 HoneyGUI 对 dirty region optimization 的重视。

TouchGFX 的复杂控件也依赖这个能力，否则高级控件会很容易变慢。

## 10. Render frontend 设计

WING GUI 的 draw 函数不应该直接画像素。

Widget draw callback 应只生成语义化绘制命令：

- fill rect。
- stroke rect。
- line。
- image。
- text。
- path。
- transform。
- clip。

这些命令进入 FRender command list。

FRender 再决定：

- 软件绘制。
- framebuffer present。
- DMA2D 加速。
- GPU2D 加速。
- 未来 vector / 2.5D / 3D backend。

## 11. Widget / container 体系

WING GUI 应该从一开始就把 widget 做成可扩展体系。

第一阶段基础 widget：

- label。
- button。
- panel。
- image。
- progress bar。
- slider。

第二阶段产品级 widget：

- gauge。
- graph。
- carousel。
- wheel selector。
- list。
- tab view。
- dialog。

第三阶段桌面相关组件：

- window。
- title bar。
- launcher icon。
- task bar。
- notification。
- menu。

这些组件都属于 WING GUI 或 WING Desktop 的不同层级，不应该混在一个目录里。

## 12. Resource / tooling 设计

WING GUI 应该尽早设计资源系统。

从 TouchGFX 和 HoneyGUI 可以吸收：

- 图片资源转换。
- 字体资源转换。
- 主题资源。
- UI 组件 demo。
- simulator 预览。
- 未来可视化工具。

第一阶段可以只支持静态编译进来的资源。

后续再支持：

- package resource。
- desktop app bundle。
- runtime load。
- resource cache。

## 13. 默认空间与 2.5D / 3D 能力边界

WING GUI 应该从框架层面默认具备空间能力：普通 2D widget 不是另一套系统，而是默认 3D 空间中的 identity transform 状态。

这意味着第一版不一定要实现完整 mesh / shader / GPU 3D 管线，但 object / camera / transform / projection / depth / picking 的抽象必须属于 GUI core。后续补充 FRender 和 NuttX graphics 的 3D 能力时，WING 不应该大面积重构应用和 widget API。

推荐方式：

- 基础 GUI 是 object tree + 默认空间模型，而不是纯 2D tree。
- 每个 object 默认拥有 transform / z-index / opacity 等空间属性。
- `wing_gui_t` 持有 runtime camera；widget 不应该私有创建 camera。
- `wing_space.c` / object projection API 负责把 bounds 建模为默认空间中的平面 quad。
- FRender command list 先支持 fill quad / blit 等 2.5D 种子，后续扩展 mesh / shader / GPU 3D command。
- 不新增 `wing_3d.c` / `wing_3d_view.c` 这种独立 3D 子系统；如果需要可配置视口，应该使用 `viewport` / `scene` / `layer` 语义，而不是把 3D 当作特殊 widget。
- 硬件不支持时可以 fallback 到软件或静态资源。

这样 WING GUI 可以在第一阶段保持可落地的 2D 软件 fallback，同时保证 3D UI 效果是核心空间模型的自然延伸，而不是后期推倒重来的扩展。

## 14. 推荐实现阶段

### Phase 0：已有基础

- `wing_gui_t` 基础执行句柄。
- `wing_gui_tick()`。
- `wing_gui_step()`。
- FRender command list 输出。
- `wing_gui_demo` 从 NSH 启动。

### Phase 1：GUI 内核

- `wing_obj_t`。
- object tree。
- bounds / flags / state。
- event callback。
- draw traversal。
- basic invalidation。

### Phase 2：输入和事件

- input event queue。
- pointer / key / encoder。
- hit test。
- focus。
- event bubbling。

### Phase 3：样式和布局

- `wing_style_t`。
- basic style props。
- stateful style。
- simple layout。

### Phase 4：脏区和动画

- dirty rect list。
- dirty merge。
- timer。
- animation。
- easing。

### Phase 5：基础 widgets

- label。
- button。
- panel。
- image。
- progress。
- slider。

### Phase 6：高级 widgets 和桌面基础

- window。
- title bar。
- list。
- launcher。
- task bar。
- dialog。

### Phase 7：高级图形

- vector path。
- transform / camera / projection 继续增强。
- image rotation / scale。
- mesh / shader command seed。
- 3D asset pipeline prototype。
- GPU 3D backend adapter。

## 15. 明确不做什么

第一版 WING GUI 不应该做这些事：

- 不直接操作硬件 framebuffer。
- 不绕过 FRender 自己做软件渲染。
- 不把 WING Desktop 和 WING GUI 混成一个库。
- 不复制 LVGL / TouchGFX / HoneyGUI 的完整 API。
- 不一开始就实现完整 3D 游戏引擎或复杂 asset pipeline。
- 不把 3D 能力做成独立 `wing_3d.c` / `wing_3d_view.c` 子系统；空间能力应该属于 WING GUI core。
- 不让 demo 承担 GUI runtime 的职责。

## 16. 结论

WING GUI 的设计可以这样理解：

- 学 LVGL 的内核：轻量、C-first、tick-driven、object tree、event、style、layout、dirty redraw。
- 学 TouchGFX 的组件：widget / container / mixin、复杂产品级控件、每个控件可复用可演示。
- 学 HoneyGUI 的野心：engine 分层、消息机制、工具链、dirty region、vector / matrix / 默认空间 / camera / 2.5D / 3D 能力。

最终 WING GUI 应该是：

```text
一个面向 NuttX / Feather 的轻量 GUI 引擎，
上层支持独立应用和 WING Desktop，
下层通过 FRender 对接 software / framebuffer / DMA2D / GPU2D，
中间具备 object tree、event、style、layout、animation、dirty region 和 widget 体系。
```
