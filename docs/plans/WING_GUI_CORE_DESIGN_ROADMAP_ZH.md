# WING GUI 核心设计原则、设计路线与阶段计划

更新时间：2026-06-12

本文用于固化当前 WING GUI、WING Desktop、FRender、NuttX graphics 与后续 Pinion 的核心设计原则、阶段路线和第一阶段验收节点。

本文不是逐条更新日志。具体实现进度请参考：

- `FeatherCore/docs/progress/WING_GUI_PROGRESS_AND_UPDATE_PLAN_ZH.md`
- `FeatherCore/docs/plans/WING_GUI_PROGRESS_AND_PLAN_ZH.md`

相关架构文档请参考：

- `FeatherCore/docs/plans/WING_GUI_PHASE1_ACCEPTANCE_ZH.md`
- `FeatherCore/docs/WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md`
- `FeatherCore/docs/WING_GUI_DESIGN_REFERENCES_ZH.md`
- `FeatherCore/docs/FRENDER_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/FEATHER_RENDER_CORE_DECISION_ZH.md`

## 1. 当前核心结论

当前第一阶段的目标不是实现完整桌面系统，也不是立刻实现完整 3D 引擎，而是先让 WING GUI 作为一个独立 GUI 库完成运行时闭环。

截至 2026-06-12，`WING GUI Phase 1: Runtime Closure` 已完成。固定验收脚本为：

```sh
./FeatherCore/tools/firmware/sim/validate-wing-phase1.sh
```

Phase 1 完成后，WING GUI 暂停功能扩张；后续主线切换到 FRender planner / capability / fallback，以及 NuttX graphics capability / present / accelerator 接入点。

第一阶段闭环如下：

```text
nsh / normal app entry
        |
        v
wing_gui_demo
        |
        v
WING GUI runtime handle
  object tree / input / event / timer / animation / dirty / render frontend
        |
        v
FRender
  command list / software fallback / capability / framebuffer present
        |
        v
NuttX graphics
  framebuffer / simulator display / low-level graphics foundation
        |
        v
Hardware / simulator window
```

这个阶段完成后，才应该把主要精力切到 FRender planner、NuttX graphics 能力声明、硬件加速后端和后续 WING Desktop / Pinion。

## 2. 核心设计原则

### 2.1 WING GUI 是 GUI 库，不是 Desktop

WING GUI 是基础 GUI 库。

任何 WING GUI 程序都可以作为普通 NuttX app 从 `nsh` 或其他应用入口直接启动，不强制依赖 WING Desktop。

WING Desktop 是建立在 WING GUI 之上的默认桌面系统，是一个可选的大型基础 WING GUI 应用。

因此二者关系是：

```text
Independent WING GUI App        WING Desktop
          \                         /
           \                       /
            v                     v
                  WING GUI
```

WING Desktop 可以提供窗口管理、任务栏、启动器、应用安装、下载、打包、桌面运行环境等能力。但这些能力不应该污染 WING GUI 的核心库边界。

### 2.2 WING GUI 程序不强制运行在 Desktop 里

WING GUI 应用有两种运行方式：

- 直接运行：从 `nsh` 或普通 app entry 启动，不经过 WING Desktop。
- 桌面运行：经过 WING Desktop 的应用包装、安装、启动和生命周期管理。

如果一个 WING GUI 程序要进入 WING Desktop 环境，它需要额外的打包 / manifest / install / launch 描述。

如果不需要 Desktop，它仍然应该可以直接执行。

### 2.3 WING GUI 必须有自己的 runtime handle

WING GUI 应该提供类似 GUI handler / runtime handle 的核心对象，例如 `wing_gui_t`。

应用层不应该自己维护 GUI 内部细节，只需要周期性调用 WING GUI 的 handler。

推荐模型：

```text
app main loop
  -> wing_gui_handle(gui, elapsed_ms, &frame)
       -> tick timers
       -> advance animations
       -> poll input provider
       -> dispatch input events
       -> update focus/capture/state
       -> run layout when needed
       -> collect dirty rects
       -> emit render commands/render nodes
       -> submit to FRender
       -> present dirty rects
```

应用线程负责“调用 WING GUI”，WING GUI 负责“推进 UI 系统”。

### 2.4 默认 3D / object space 是核心能力，不是扩展能力

WING GUI 的核心原则是：2D 是默认 3D/object space 下的 identity transform 特殊状态。

也就是说，普通 2D widget 不应该走一条独立于 3D 的旁路。它们只是处在默认 camera、默认 object space、默认 identity transform 下。

推荐理解：

```text
普通 2D UI
  = object space 中 z=0、identity transform、orthographic-like default state 的特殊情况

空间 UI / 2.5D / 3D UI
  = 同一 object tree、同一 camera、同一 render frontend 下的非 identity transform 情况
```

因此不应该新增独立的 `wing_3d_view` 或专门的外挂 3D widget 来承载核心 3D 能力。

3D 相关能力应该自然落在：

- object space transform
- runtime camera
- projection
- projected quad / depth
- hit test / picking
- z-index + projected depth ordering
- render node / material
- FRender primitive / mesh / shader capability

### 2.5 WING GUI 不直接写 framebuffer

WING GUI 负责 UI 语义和渲染意图，不应该直接操作像素或 framebuffer。

WING GUI 应该输出 render frontend 描述：

- render node
- material
- primitive
- image resource
- clip
- dirty region
- command list seed

然后交给 FRender。

```text
WING GUI object/widget draw
        |
        v
WING render frontend
        |
        v
FRender command list / render planner
        |
        v
software / framebuffer / DMA2D / GPU2D / future GPU3D backend
```

这样后续补充硬件加速、mesh、shader、GPU 3D 时，不需要大面积重构 WING GUI 应用层。

### 2.6 FRender 是 WING 和 Pinion 共享的渲染底座

FRender 不属于 WING 私有实现，也不应该被做成某个 demo 的绘制 helper。

FRender 是 Feather 图形体系中的渲染底座，未来同时服务：

- WING GUI
- WING Desktop
- Pinion game engine
- 其他需要绘制能力的 app/library

FRender 负责：

- command list
- render node / primitive 承载
- backend capability
- render planner
- fallback policy
- software backend
- framebuffer present adapter
- future DMA2D / GPU2D / GPU3D backend adapter

### 2.7 NuttX graphics 是底层显示和硬件能力层

NuttX graphics 不应该承载 WING 的 object tree、layout、style、event 或 Desktop 语义。

它应该提供：

- framebuffer / LCD / NX 等显示基础
- update / present 能力
- input 设备基础
- DMA2D / GPU2D / GPU3D 等硬件能力入口
- buffer / cache / sync / accelerator wrapper
- 能力查询或能力声明

WING 和 Pinion 不应该依赖 `nuttx/graphics` 的内部私有结构，而应该通过公开能力、设备接口、present adapter 或后续扩展 API 对接。

### 2.8 能力声明和 fallback 是长期核心机制

硬件加速能力是不确定的。

因此架构上必须默认存在：

```text
capability query
  -> render planner
  -> native backend if supported
  -> fallback decomposition if partially supported
  -> software fallback if unsupported
```

软件后端不是临时方案，而是正确性基线。

硬件后端不是替代 WING/FRender 的实现，而是 FRender planner 的执行目标之一。

### 2.9 脏区、裁剪和 present 是 GUI 性能核心

WING GUI 第一阶段必须把 dirty 机制做成核心能力，而不是后期优化。

需要长期保留的机制：

- object invalidation
- dirty rect list
- dirty merge
- chunk redraw
- clip stack
- object clip-children
- present rect list
- framebuffer update rect

Desktop、复杂 widget、动画和空间 UI 都会依赖这些基础能力。

### 2.10 文件和模块边界必须清晰

WING GUI 不能继续把所有 API 都塞进 `wing.h`，也不能把所有逻辑都塞进 `wing_gui.c`。

推荐边界：

```text
include/wing/wing.h
  聚合入口

include/wing/core/
  runtime / object / input / event / focus / capture / timer / animation
  dirty / render / render_node / space / theme / text_edit

include/wing/widgets/
  widget 聚合头和具体 widget API

src/core/
  GUI runtime 核心机制

src/widgets/
  具体组件实现
```

专属功能应该有专属文件。`wing.h` 应该是应用友好的聚合入口，不应该成为所有声明的堆场。

## 3. WING GUI 从其他 GUI/图形库吸收的设计特质

### 3.1 从 LVGL 吸收

吸收方向：

- handler/tick 模型
- object tree
- event bubbling / focused object
- input device abstraction
- dirty invalidation
- widget state
- style state selector
- layout seed
- encoder / key / pointer 统一输入思想

需要适配到 WING 的地方：

- LVGL 的 2D widget 模型需要被放入 WING 默认 object space 语境。
- WING 不应照搬 LVGL draw backend，而是通过 FRender 作为统一 render backend。
- WING 的 Desktop 不是 LVGL screen 的直接复制，而是建立在 WING GUI 上的大型 app。

### 3.2 从 TouchGFX 吸收

吸收方向：

- widget / container / mixin 式组件组织
- 产品级组件封装
- 每个 widget 有明确行为边界
- 动画和 invalidation 配合
- demo 可以作为组件验证入口

需要适配到 WING 的地方：

- TouchGFX 面向特定 MCU GUI 产品栈，WING 需要同时服务 Desktop 和 Pinion 共享渲染底座。
- TouchGFX 的绘制路径不能直接照搬，WING 应该统一输出 FRender command/render node。
- TouchGFX 的组件思想可以吸收，但 WING 的 3D/object space 是核心原则，组件默认需要能进入 object space。

### 3.3 从 HoneyGUI 吸收

吸收方向：

- engine/server/message 分层意识
- camera / viewport / matrix / 2.5D / 3D UI 思想
- resource/tooling 意识
- dirty 优化和 simulator 验证
- UI 具备空间表达能力

需要适配到 WING 的地方：

- WING 不应把 3D 当作一个单独 widget 或扩展模块，而应该把 object space 放入 core。
- HoneyGUI 的 camera 思路可以吸收，但 WING 的 camera 应该属于 runtime/object space，而不是某个私有 view。
- WING 的 render path 必须落到 FRender，后续再由 FRender 对接 NuttX graphics / hardware backend。

## 4. 设计路线

### 4.1 总体路线

```text
Phase 1: WING GUI Runtime Closure
  先让 WING GUI 作为独立 GUI 库跑通

Phase 2: FRender Planner and Capability Closure
  补 render planner、capability、fallback、backend policy

Phase 3: NuttX graphics Capability Integration
  补 framebuffer/LCD/NX/DMA2D/GPU2D/GPU3D 能力声明和公开接入

Phase 4: WING Desktop
  基于稳定 WING GUI 实现可选桌面系统

Phase 5: Pinion
  基于 FRender 和 NuttX graphics 实现游戏引擎

Phase 6: Advanced 3D / Resource / Toolchain
  补 mesh、shader、资源包、布局工具、复杂动画和硬件加速策略
```

### 4.2 当前最重要的路线约束

当前不要继续堆 Desktop。

当前不要继续无限加 widget。

当前不要把 3D 做成外挂模块。

当前不要把 FRender 和 WING 混成一个库。

当前不要让 WING 直接依赖 NuttX graphics 内部实现。

当前应该先把 WING GUI runtime、FRender 边界、NuttX graphics 边界打稳。

## 5. 第一阶段节点：WING GUI Runtime Closure

建议第一阶段节点名称：

```text
WING GUI Phase 1: Runtime Closure
```

中文名称：

```text
WING GUI 第一阶段：运行时闭环
```

### 5.1 阶段目标

第一阶段目标是证明：

WING GUI 可以作为一个独立 GUI 库，被普通 NuttX app 调用，从 `nsh` 启动，打开显示窗口，接收输入，推进自己的 GUI 大循环，并通过 `frender -> nuttx/graphics` 完成绘制和显示。

这个阶段的完成标准不是“功能很多”，而是“边界清楚、闭环稳定、后续不用推倒”。

### 5.2 必须完成的验收项

必须完成：

1. `wing_gui_demo` 可以从 `nsh` 执行启动。
2. 执行 demo 时才打开 X11 framebuffer 窗口。
3. 点击窗口关闭按钮后退出 demo，回到 `nsh`。
4. WING GUI 有自己的执行句柄，例如 `wing_gui_t`。
5. demo 自己的循环只负责反复调用 WING GUI 的 step/tick/handle。
6. WING GUI 内部负责 timer、animation、input、event、dirty、render。
7. object tree 可以正常管理父子对象。
8. 基础事件可以走通：pointer、key、focus、capture、value changed、close。
9. 基础 widget 可以验证：panel、button、label、progress、slider、scrollbar、switch、checkbox、text input、scroll view、card/custom geometry。
10. 脏区更新、裁剪、局部重绘、present rect 能正常工作。
11. 默认 3D/object space 原则已经进入架构：2D 是默认 3D 空间状态，而不是另一个旁路系统。
12. WING 不直接写 framebuffer，而是通过 render frontend 生成 render node / draw command，再交给 FRender。
13. 文档和进度计划同步记录当前状态。

### 5.3 第一阶段不做的内容

第一阶段不继续扩展：

1. WING Desktop。
2. 应用安装、打包、下载、桌面启动器。
3. Pinion 游戏引擎。
4. 完整 mesh / shader / GPU 3D。
5. 完整 flex/grid 布局系统。
6. 复杂 widget 套件。
7. 硬件加速后端。
8. NX backend。
9. 大规模主题系统和资源工具链。

这些内容不是不重要，而是不应该在 WING GUI runtime 闭环稳定前继续混进来。

### 5.4 当前状态判断

当前 WING GUI 已经接近 Phase 1 的收口点，但还没有完全封版。

已经具备：

1. `wing_gui_demo` 可以从 `nsh` 启动。
2. X11 framebuffer 可以打开/关闭。
3. 输入事件可以进入 WING GUI。
4. timer、animation、dirty、render path 已经跑通。
5. FRender 已经作为 WING 的渲染底座接入。
6. 默认 3D/object space、runtime camera、projection、render node 的种子已经开始进入核心结构。

还需要补齐或稳定：

1. 图层排序、z-index、projected depth、父子裁剪、overlap 行为要继续稳定。
2. widget 背景覆盖问题要收敛，不能靠 demo 层绕开。
3. render node、material、transform、primitive 的边界要继续整理清楚。
4. demo 中写死的视觉数值要继续迁移为 demo 自己的配置或 theme/style，而不是 WING GUI 核心默认行为。
5. 文档中需要明确 Phase 1 验收清单，避免继续无限加功能。

### 5.5 Phase 1 完成后的切换条件

当 Phase 1 验收通过后，WING GUI 应暂停功能扩张，只保留必要 bugfix 和边界清理。

随后重点切换到：

1. FRender capability / planner / fallback。
2. FRender command profile：2D profile、default object-space profile、future 3D profile。
3. NuttX graphics public capability query。
4. framebuffer / LCD / NX / update / input / cache / sync 能力整理。
5. DMA2D / GPU2D / future GPU3D backend 接入点。
6. WING Desktop 的最小桌面 app 模板。
7. Pinion 的最小 game loop / scene / render path。

## 6. FRender 后续计划

FRender 下一阶段重点不是继续作为软件绘制 helper，而是成为真正的 render core。

需要补充：

1. backend capability 分类更清晰。
2. command list 到 frame plan 的中间层。
3. planner 根据 capability 拆解 command。
4. fallback 统计和诊断日志。
5. software backend 作为 correctness baseline。
6. framebuffer present adapter 保持简单稳定。
7. DMA2D / GPU2D backend 通过 capability 接入。
8. 为 mesh / shader / GPU 3D 预留 command profile。

FRender 的长期结构应为：

```text
WING / Pinion
    |
    v
FRender command list / render nodes
    |
    v
Render planner
    |
    +-- software backend
    +-- framebuffer present adapter
    +-- DMA2D backend
    +-- GPU2D backend
    +-- future GPU3D / mesh / shader backend
```

## 7. NuttX graphics 后续计划

NuttX graphics 当前可以支撑第一阶段显示闭环，但还不足以直接支撑完整 FRender hardware backend。

需要补充：

1. framebuffer 能力查询。
2. LCD / framebuffer format、stride、xres、yres、update capability 暴露。
3. present/update 行为整理。
4. input 与窗口关闭事件路径稳定。
5. DMA2D / GPU2D 能力声明或 wrapper。
6. cache / sync / buffer ownership 约束。
7. 为后续 GPU3D / mesh / shader path 预留 capability 表达。

NuttX graphics 不应该做：

1. WING object tree。
2. WING style / layout / event。
3. WING Desktop。
4. Pinion ECS / scene。
5. FRender planner policy。

## 8. WING Desktop 后续计划

WING Desktop 应该等 WING GUI runtime closure 后再开始。

WING Desktop 是一个可选的大型基础 WING GUI 应用。

它应该包含：

1. desktop root。
2. window manager。
3. layer manager。
4. focus / active window。
5. launcher。
6. taskbar / dock。
7. app manifest / install / launch。
8. optional compositor effects。
9. desktop settings / theme。

WING Desktop 不应该反向污染 WING GUI 核心库。

## 9. Pinion 后续计划

Pinion 是游戏引擎，不是 WING Desktop 的子模块。

Pinion 应该基于 FRender 和 NuttX graphics 实现：

1. game loop。
2. scene / entity / component。
3. sprite / tile / animation。
4. input mapping。
5. resource loading。
6. camera。
7. collision seed。
8. render submission。

Pinion 和 WING GUI 可以共享 FRender，但不应该共享 GUI widget 语义。

未来在 3D 能力成熟后，Pinion 可以更直接地使用 mesh / shader / GPU backend。

## 10. 当前建议行动

近期建议按如下顺序推进：

1. 收口 WING GUI Phase 1。
2. 修正图层排序、overlap、widget 背景覆盖等 runtime 稳定性问题。
3. 整理 demo 中写死的视觉数值，继续移动到 demo 配置或 theme/style。
4. 完成 Phase 1 验收文档和验证记录。
5. 暂停 WING GUI 功能扩张。
6. 转向 FRender planner / capability。
7. 转向 NuttX graphics capability / present / accelerator 接入点。
8. 再回到 WING Desktop 和 Pinion。

## 11. 阶段性结论

当前最危险的不是代码难，而是边界继续变糊。

因此第一阶段的核心不是继续增加功能，而是把以下事实固定下来：

- WING GUI 是独立 GUI 库。
- WING Desktop 是可选桌面应用。
- WING GUI 程序可以不经过 Desktop 直接运行。
- WING GUI 有自己的 runtime handle。
- WING GUI 的 2D/3D 统一在默认 object space 里。
- WING GUI 不直接写 framebuffer。
- FRender 是共享 render backend。
- NuttX graphics 是底层显示和硬件能力层。
- Phase 1 的目标是 runtime closure，不是功能堆叠。

当这些原则稳定后，后续补 FRender、NuttX graphics、WING Desktop 和 Pinion 才不会反复推倒重来。

## 12. Phase 1 层级稳定性补充原则

Phase 1 期间，WING GUI core 提供稳定排序语义，应用或 demo 必须显式表达自己的视觉层级。

排序原则保持为：

```text
z-index
  -> projected depth in default object space
  -> sibling order as final stable tie-breaker
```

应用层不应该依赖“控件刚好后创建所以盖在上面”的偶然行为。对于 progress、slider、scrollbar、toast、overlay、modal、cursor 这类天然存在层级关系的对象，demo 或应用应显式设置 z-index。

`wing_gui_demo` 的 value controls 已采用该原则：progress / slider / scrollbar 分别使用独立 z-index，让上方控件在 redraw/focus/pressed state 场景下保持视觉稳定。

这个原则也为后续 WING Desktop 打底：Desktop layer manager 应该只是更系统地分配 z-index/layer，而不是重写 WING GUI object tree 的排序模型。
