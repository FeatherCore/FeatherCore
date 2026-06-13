# Feather 图形栈进度与更新计划

本文档用于记录 Feather 图形栈当前实现进度、阶段目标、下一步计划与每次更新记录。

它不是架构设计文档，也不是 API 说明文档，而是一个持续维护的项目进度页。

## 1. 当前总体方向

Feather 图形栈按以下层次推进：

```text
Independent UI App / WING Desktop / Pinion Game
        |
        v
WING GUI / Pinion Engine
        |
        v
FRender
  command list / render planner / software fallback / hardware capability adapter
        |
        v
NuttX graphics
  framebuffer / LCD / NX / DMA2D / GPU2D / input / timer
        |
        v
Hardware
  panel / touch / keys / accelerator
```

当前共识：

- `FRender` 是 Feather 侧独立的渲染底座，位于 `apps/graphics/frender`。
- `FRender` 吸收 FGFX 的核心思想：绘制命令列表、能力声明、后端选择、软件 fallback、未来硬件加速接入。
- `WING GUI` 是 GUI 库，不强制依赖 WING Desktop。
- `WING Desktop` 是基于 WING GUI 的可选桌面系统，是一个大型基础 WING 应用。
- `Pinion` 是游戏引擎，也应基于 FRender，而不是直接绑定某个具体显示后端。
- `NuttX graphics` 负责底层显示、Framebuffer、LCD、NX、输入、DMA2D/GPU2D 等能力。

## 2. 核心文档索引

当前建议保留并持续维护的核心文档：

- `FeatherCore/docs/NUTTX_GRAPHICS_OVERVIEW_ZH.md`
- `FeatherCore/docs/FRENDER_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/FEATHER_RENDER_CORE_DECISION_ZH.md`
- `FeatherCore/docs/WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md`
- `FeatherCore/docs/WING_GUI_DESIGN_REFERENCES_ZH.md`
- `FeatherCore/docs/WING_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/PINION_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/plans/FEATHER_GRAPHICS_PROGRESS_AND_PLAN_ZH.md`
- `FeatherCore/docs/progress/WING_GUI_IMPLEMENTATION_PROGRESS_AND_PLAN_ZH.md`

其中本文档负责记录进度和计划。

## 3. 当前完成状态

### 3.1 FRender

当前状态：第一阶段可作为 WING GUI 的渲染底座继续向上实现。

已完成：

- 建立 `apps/graphics/frender` 模块。
- 建立 FRender 基础 API。
- 建立绘制命令列表。
- 支持基础软件渲染后端。
- 支持能力声明结构。
- 支持 backend registry。
- 支持 NuttX framebuffer 能力探测。
- 支持 framebuffer present 路径。
- 支持 `frender_demo` 从 NSH 执行。
- `frender_demo` 能展示当前命令列表、软件渲染、framebuffer present 和能力注册状态。
- sim 下 demo 执行后才打开 framebuffer/X11 窗口，关闭窗口后返回 NSH。

当前限制：

- NuttX graphics 目前主要作为 present/capability adapter 接入。
- FRender 还没有把绘制命令直接提交给 DMA2D/GPU2D/NXBE 等硬件路径。
- 还没有完整 render planner。
- 还没有批处理合并、裁剪合并、脏块合并的正式策略。
- 还没有异步提交、同步 fence 或多缓冲策略。

下一步：

- 明确 FRender command list 的稳定 ABI/API。
- 增加更完整的 primitive：line、text、image、blend、clip stack。
- 增加 render planner，决定命令走 software、framebuffer、DMA2D、GPU2D 或未来后端。
- 增加后端 capability query 与 fallback 决策。
- 增加更清晰的 demo 输出与可视化测试场景。

### 3.2 WING GUI

当前状态：正在基于 FRender 实现第一阶段 GUI 库。

已完成或已确定方向：

- WING GUI 不等于 WING Desktop。
- WING GUI 应能从 NSH 直接启动独立 UI 程序。
- 每个 WING GUI 程序可以是一个普通可执行应用。
- 应用拥有自己的主循环。
- WING GUI 提供一个 GUI 句柄，由应用循环周期性调用 `wing_gui_handle()` 推进。
- GUI 句柄负责维护对象树、事件队列、输入 provider、软件定时器、动画、脏块和绘制。
- `wing_gui_demo` 是第一阶段验证模板。
- `wing_gui_demo` 应验证：初始化、对象树、事件、定时器、动画、脏块、绘制、present。
- `wing_gui_demo` 现在可以在 headless 环境下固定运行多帧，便于自动验证 timer、animation 和 dirty/redraw。

当前已具备的基础能力：

- `wing_gui_t` GUI 执行句柄。
- `wing_gui_handle()` GUI handler。
- `wing_gui_frame_t` frame 诊断信息。
- input provider 抽象。
- object tree。
- dirty rect。
- draw traversal。
- pointer input。
- handler 内部 input provider poll。
- click event。
- event bubbling。
- stop propagation。
- focus state。
- keyboard input。
- focus traversal。
- software timer。
- linear animation。
- fixed / horizontal stack / vertical stack layout。
- dirty rect 查询与 demo 日志展示。
- 默认主题 seed。
- 基础组件：box、button、label、panel、progress、slider、scrollbar、switch、checkbox。

当前限制：

- 样式系统仍然很轻量。
- 主题系统已有 seed，但尚未支持继承、切换或 selector cascade。
- 文本渲染只是非常早期的矩形 glyph 验证。
- 输入设备抽象还不完整。
- 已有 input provider 抽象，但尚未接真实 NuttX touch / mouse / keyboard。
- 没有完整布局约束系统。
- 没有正式资源系统。
- 没有完整生命周期和应用打包模型。
- WING Desktop 还不应该开始重实现，先稳定 WING GUI。

下一步：

- 固化 `wing_gui_t` 的 tick/dispatch/render/present 流程。
- 增加更规范的 widget 基类策略。
- 完善 style/state/theme。
- 将当前 `wing_theme_t` seed 扩展为可切换、可继承的主题系统。
- 完善 event 类型和事件分发。
- 完善 layout。
- 完善 text/image/resource。
- 保持 `wing_gui_demo` 作为最小独立 GUI 应用验证模板。

### 3.3 WING Desktop

当前状态：暂缓。

原因：

- WING Desktop 应建立在稳定的 WING GUI 之上。
- 现在直接开发桌面系统会把 GUI 库、桌面壳、渲染底座、demo 混在一起。
- 当前优先级应是 WING GUI，而不是 Desktop。

后续目标：

- 作为可选桌面系统实现。
- 提供默认 launcher、window manager、task bar、status area、app registry。
- 支持 WING GUI 应用打包后进入 Desktop 环境。
- 不阻止 WING GUI 应用从 NSH 独立运行。

### 3.4 Pinion

当前状态：设计阶段。

当前共识：

- Pinion 是游戏引擎。
- Pinion 与 WING GUI 平级。
- Pinion 应基于 FRender。
- Pinion 不应直接绑定 NuttX framebuffer 或具体 LCD driver。
- Pinion 可复用 FRender 的 command list、capability query、software fallback 和未来硬件加速路径。

后续目标：

- 建立 Pinion 最小 engine loop。
- 建立 scene/entity/sprite/tilemap 基础结构。
- 建立 input、timer、update、render pipeline。
- 建立 `pinion_demo`。
- 与 WING GUI 共享 FRender，但不共享 GUI object tree。

## 4. 阶段计划

### Phase 0：清理和方向确认

状态：基本完成。

目标：

- 放弃 fgfx 旧路径。
- 明确 FRender 是新的 Feather render backend。
- 明确 WING GUI 和 WING Desktop 分层。
- 明确 Pinion 也基于 FRender。
- 清理混乱实现，保留必要文档。

### Phase 1：FRender 最小可用

状态：进行中，已可支撑 WING GUI 第一版。

目标：

- 软件 command list 能稳定绘制。
- 能声明 software/backend/framebuffer 能力。
- 能接入 NuttX framebuffer present。
- 能从 NSH 运行 demo。
- 能作为 WING GUI 第一阶段渲染底座。

验收标准：

- `frender_demo` 可从 NSH 执行。
- demo 执行时才打开窗口。
- 关闭窗口后返回 NSH。
- demo 输出清楚说明 backend、capability、command delivery path。

### Phase 2：WING GUI 最小可用

状态：进行中。

目标：

- 建立 `wing_gui_t` 执行句柄。
- 建立应用主循环调用模式。
- 建立 object tree。
- 建立 input/event/timer/animation/dirty/render 基础流程。
- 建立基础 widget。
- 建立 `wing_gui_demo`。

验收标准：

- `wing_gui_demo` 可从 NSH 执行。
- demo 能通过 WING GUI 主循环推进 UI。
- demo 通过 `wing_gui_handle()` 推进 UI。
- demo 能展示对象树、事件、定时器、动画、脏块、绘制。
- demo 使用 FRender，而不是直接绕过 FRender 绘制。
- headless 验证能覆盖多帧 runtime，看到 timer 更新、animation 完成、dirty 局部刷新和 render 后 dirty 清空。

### Phase 3：WING GUI 工程化

状态：未开始。

目标：

- 完善 widget/component 架构。
- 完善 style/theme。
- 完善 layout。
- 完善 text/image/resource。
- 完善 input device adapter。
- 完善 application lifecycle。

验收标准：

- 可以基于 WING GUI 写多个独立 UI app。
- app 不需要 WING Desktop 也能运行。
- app 可以被 Desktop 包装后安装/启动。

### Phase 4：WING Desktop

状态：未开始。

目标：

- 基于 WING GUI 实现默认桌面系统。
- 提供 launcher、window manager、task bar、app registry。
- 支持应用打包、安装、启动。

验收标准：

- WING Desktop 是一个可选应用环境。
- 独立 WING GUI app 不被强制依赖 Desktop。
- Desktop 可以承载打包后的 WING GUI app。

### Phase 5：Pinion

状态：未开始。

目标：

- 基于 FRender 建立游戏引擎第一版。
- 建立 game loop、scene、sprite、tilemap、input。
- 建立 `pinion_demo`。

验收标准：

- Pinion demo 可从 NSH 执行。
- Pinion 通过 FRender 绘制。
- Pinion 与 WING GUI 共用渲染底座，但架构独立。

## 5. 每次更新记录格式

之后每次推进代码或文档，建议在本文档追加一条记录：

```text
YYYY-MM-DD
- 范围：修改了哪些模块。
- 内容：完成了什么。
- 验证：执行了哪些构建或 demo。
- 结果：通过 / 失败 / 已知问题。
- 下一步：下一次建议做什么。
```

## 6. 更新记录

### 2026-06-12

范围：图形栈规划文档。

内容：

- 新增本文档，用于统一记录 Feather 图形栈进度与后续计划。
- 明确 FRender、WING GUI、WING Desktop、Pinion、NuttX graphics 的分层关系。
- 明确当前阶段优先级：先稳定 FRender，再实现 WING GUI，暂缓 WING Desktop。
- 明确 WING GUI 程序不强制依赖 WING Desktop，可从 NSH 独立运行。
- 明确 Pinion 也应基于 FRender。

验证：

- 文档新增，无需构建。

结果：

- 进度与计划有了独立维护位置。

下一步：

- 继续按 Phase 2 推进 WING GUI。
- 优先固化 `wing_gui_t` 主循环职责。
- 保持 `wing_gui_demo` 作为 NSH 下的最小验证模板。

### 2026-06-12

范围：WING GUI widget 层与 `wing_gui_demo`。

内容：

- 新增 `wing_progress_t` 组件。
- 将 demo 中由两个 box 手工组合的进度条替换为 WING GUI 原生 progress widget。
- 使用 WING timer 推进 progress value，验证 GUI 句柄对定时器、组件状态、脏标记和绘制流程的维护。

验证：

- 本次代码修改后需要重新构建 sim 并执行 `wing_gui_demo`。

结果：

- 构建通过。
- 无 DISPLAY 环境下运行 `wing_gui_demo` 通过。
- `wing_progress_t` 已进入 demo 验证路径。

下一步：

- 根据构建和运行结果更新验证状态。

### 2026-06-12

范围：WING GUI 可交互 widget。

内容：

- 新增 `wing_slider_t`，验证 WING GUI 组件可以自己处理 pointer 输入并更新内部 value。
- `wing_gui_demo` 增加 slider 场景，覆盖 pointer down/move/up 到 value changed 的路径。
- 修正 progress range 的极端边界归一化。

验证：

- 本次代码修改后需要重新构建 sim 并执行 `wing_gui_demo`。

结果：

- 待构建与运行验证。

下一步：

- 根据构建和运行结果更新验证状态。

验证结果：

- 构建通过：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 运行通过：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 属于预期现象。
- slider value changed 路径已验证。
- slider pointer/click 冒泡已阻止，不再误触发 root click。
- demo 输出一帧 `commands=49 checksum=0x435e23c5`，正常退出。

### 2026-06-12

范围：WING GUI value widget 内部公共逻辑。

内容：

- 新增 `wing_value` 内部 helper。
- `wing_progress_t` 和 `wing_slider_t` 共用 range/value 计算。
- 保持对外 API 不变，为后续 scrollbar、meter 等组件减少重复实现。

验证：

- 本次代码修改后需要重新构建 sim 并执行 `wing_gui_demo`。

结果：

- 待构建与运行验证。

验证结果：

- 构建通过：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 运行通过：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`
- progress 与 slider 行为保持稳定。
- demo 输出一帧 `commands=49 checksum=0x435e23c5`，正常退出。

### 2026-06-12

范围：WING GUI value widget 系列。

内容：

- 新增 `wing_scrollbar_t`，验证 `wing_value` 内部 helper 可以支撑第三个 value widget。
- `wing_gui_demo` 新增 scrollbar pointer 交互路径。
- scrollbar 预留 horizontal/vertical axis，当前 demo 使用 horizontal。

验证：

- 本次代码修改后需要重新构建 sim 并执行 `wing_gui_demo`。

结果：

- 待构建与运行验证。

## 2026-06-12 更新：WING GUI Scrollbar 验证

本轮继续基于当前 FRender 和 NuttX graphics adapter 推进 WING GUI 第一阶段，没有改动 FRender 对 NuttX graphics 的职责边界。

### 完成内容

- WING GUI 新增 `wing_scrollbar_t`。
- Scrollbar 复用 WING 内部 value/range 映射 helper。
- `wing_gui_demo` 新增 scrollbar 示例，并通过合成 pointer 输入验证 value 变化。
- WING GUI 输入队列扩大到 16，事件队列扩大到 32，以支撑 button、slider、scrollbar 连续交互产生的事件流。

### 验证结果

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 在无 DISPLAY 环境下运行通过，framebuffer 注册失败 `-19` 为预期现象。
- 关键输出：

```text
wing_gui_demo: wing_scrollbar value changed to 0 by pointer input
wing_gui_demo: wing_scrollbar value changed to 75 by pointer input
wing_gui_demo: wing_scrollbar pointer interaction completed
wing_gui_demo: frame tick=33 commands=52 checksum=0x36bfa7c5
```

### 架构状态

当前路径仍然保持为：

```text
WING GUI widget/object/input/event/layout
        |
        v
FRender command list / software backend
        |
        v
optional NuttX graphics framebuffer present
```

FRender 仍是 WING GUI 与 NuttX graphics 之间的渲染底座，NuttX graphics 目前主要提供 framebuffer/present 能力与后续硬件加速接入点。

## 2026-06-12 更新：WING GUI 运行时资源配置化

本轮没有改变 FRender 与 NuttX graphics 的职责边界，主要推进 WING GUI 自身运行时资源管理。

### 完成内容

- 新增 `CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE`，默认 16。
- 新增 `CONFIG_GRAPHICS_WING_EVENT_QUEUE_SIZE`，默认 32。
- `wing.h` 保留 fallback，避免非标准配置场景下缺少宏导致编译失败。

### 意义

WING GUI 后续会作为独立 GUI 库被普通应用、WING Desktop、以及可能的系统 UI 共同使用。运行时队列容量不能长期固定在源码里，应该由目标系统根据内存、输入频率、控件复杂度和 Desktop/App 场景调节。

### 本轮验证结果

- `build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 自动执行通过。
- WING GUI 队列容量迁移为 Kconfig 后，slider 与 scrollbar 连续输入事件仍可完整处理。
- 最终帧输出：`frame tick=33 commands=52 checksum=0x36bfa7c5`。

## 2026-06-12 更新：WING GUI timer/animation 资源配置化

本轮继续推进 WING GUI 运行时资源配置化。

### 完成内容

- 新增 `CONFIG_GRAPHICS_WING_TIMER_MAX`，默认 8。
- 新增 `CONFIG_GRAPHICS_WING_ANIM_MAX`，默认 8。
- `wing.h` 保留 fallback，保证非标准配置环境仍可编译。

### 意义

WING GUI 的 app handle 逐渐形成清晰边界：应用线程负责调用 tick/step，WING GUI 负责维护输入、事件、timer、animation、dirty 和 render frontend。资源容量由配置系统决定，而不是由源码常量固定。

### 本轮验证结果

- `build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 自动执行通过。
- WING GUI timer/animation 槽位迁移为 Kconfig 后，现有 progress timer、输入事件与 FRender 输出保持稳定。
- 最终帧输出：`frame tick=33 commands=52 checksum=0x36bfa7c5`。

## 2026-06-12 更新：WING Value 事件语义收口

本轮没有改变 FRender 或 NuttX graphics 路径，重点是提升 WING GUI 公共事件 API 的稳定性。

### 完成内容

- 新增 `wing_value_event_t`。
- `wing_progress_t`、`wing_slider_t`、`wing_scrollbar_t` 的 `WING_EVENT_VALUE_CHANGED` 统一携带 old/current/range。
- `wing_gui_demo` 改为读取统一 payload。

### 意义

WING GUI 从“控件能画、能响应”继续向“公共 API 可长期使用”推进。统一事件 payload 是后续应用、Desktop、Pinion 工具 UI 复用 WING 控件时需要的基础语义。

### 本轮验证结果

- `build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 自动执行通过。
- `wing_value_event_t` 已被 slider/scrollbar demo 回调正确读取。
- FRender 输出稳定：`frame tick=33 commands=52 checksum=0x36bfa7c5`。

## 2026-06-12 更新：WING Switch 控件

本轮继续推进 WING GUI 控件层，没有改变 FRender 与 NuttX graphics 的底层边界。

### 完成内容

- 新增 `wing_switch_t` 布尔状态控件。
- 接入 `wing_gui_demo`。
- 复用 `wing_value_event_t` 作为 checked 状态变化 payload。

### 意义

WING GUI 控件族从纯绘制、连续 value 控件，扩展到可交互布尔状态控件。这个控件是未来系统设置、Desktop 开关、应用配置界面的基础组件。

### 本轮验证结果

- `build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 自动执行通过。
- `wing_switch_t` pointer click 切换通过。
- `wing_value_event_t` 可同时服务连续 value 控件和布尔状态控件。
- 最终帧输出：`frame tick=33 commands=55 checksum=0x33bac7c5`。

## 2026-06-12 更新：WING Checkbox 控件

本轮继续推进 WING GUI 控件层，没有改变 FRender 与 NuttX graphics 的职责边界。

### 完成内容

- 新增 `wing_checkbox_t` 布尔状态控件。
- 接入 `wing_gui_demo`。
- 复用 `wing_value_event_t` 作为 checked 状态变化 payload。

### 意义

WING GUI 当前已经覆盖基础绘制控件、连续 value 控件，以及 switch/checkbox 两类布尔状态控件。后续可以在这个控件族基础上继续做表单、设置面板和 Desktop 控制区。

### 本轮验证结果

- `build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 自动执行通过。
- `wing_checkbox_t` pointer click 切换通过。
- checkbox 与 switch 都复用 `wing_value_event_t`。
- 最终帧输出：`frame tick=33 commands=58 checksum=0x1f5727c5`。

## 2026-06-12 更新：WING Value Getter API

本轮继续推进 WING GUI 公共 API 稳定性，没有改变 FRender 与 NuttX graphics 的职责边界。

### 完成内容

- 为 progress/slider/scrollbar 补充 value/range getter。
- 为 scrollbar 补充 page size getter。
- `wing_gui_demo` 已验证 getter 返回初始状态。

### 验证结果

- `build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 自动执行通过。
- getter 日志正确，最终帧输出保持：`frame tick=33 commands=58 checksum=0x1f5727c5`。

## 2026-06-12 更新记录：WING GUI Phase 2 继续推进

本次更新聚焦 WING GUI，而不是 WING Desktop。当前仍坚持先把 GUI 库做稳，再在其上构建桌面系统。

完成内容：

- WING GUI 已基于 FRender 继续扩展 widget 能力。
- 已加入 value 类控件：progress、slider、scrollbar。
- 已加入 boolean 类控件：switch、checkbox。
- `wing_gui_demo` 继续作为 NSH 独立 GUI 应用模板，不依赖 WING Desktop。
- demo 已覆盖 object tree、layout、input queue、event queue、focus/key、timer、animation、widget event、FRender command 输出。
- sim 构建脚本已显式设置 WING runtime 资源，避免增量构建复用旧 `.config` 导致队列容量不一致。

验证结果：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `wing_gui_demo` 可从 NSH 启动并退出。
- 无 DISPLAY 环境下 framebuffer present 跳过，但软件渲染、事件和 command list 验证通过。
- checkbox/switch 的 keyboard activation 双切换问题已修正。

当前判断：

- FRender 当前已足够作为 WING GUI Phase 2 的渲染底座继续往上开发。
- WING GUI 还不是完整 GUI 框架，但已经具备 runtime 句柄、对象树、事件、输入、定时器、动画、布局和基础控件的可验证骨架。
- 下一阶段不应该马上做 WING Desktop，而应该继续完善 WING GUI 的 widget base、style/theme、layout、text/resource/input adapter。

下一步优先级：

- 优先级 1：整理 WING widget base 和 value model，降低控件重复代码。
- 优先级 2：完善 style/state/theme，使控件状态样式统一。
- 优先级 3：接入真实 NuttX input adapter，让 demo 不只依赖 synthetic input。
- 优先级 4：补 text/font/resource，替换临时矩形 glyph。
- 优先级 5：在 WING GUI 稳定后，再启动 WING Desktop 的 launcher/window/app registry。

## 2026-06-12 更新记录：WING GUI 内部控件公共逻辑收拢

本次更新继续推进 WING GUI Phase 2，没有启动 WING Desktop。

完成内容：

- 新增 `apps/graphics/wing/src/core/wing_widget.{h,c}`。
- 扩展 `apps/graphics/wing/src/core/wing_value.{h,c}`。
- progress、slider、scrollbar、switch、checkbox 已复用第一版 widget/value 内部 helper。
- public API 保持稳定，demo 不需要改动。

意义：

- WING GUI 开始从“每个控件各写一套逻辑”转向“core helper 承载公共控件语义”。
- 当前 helper 覆盖 object 初始化、style 绘制、pressed state、activation key、value payload。
- 这为后续 widget base、state style、theme、input adapter 打下基础。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- 最终输出 `frame tick=33 commands=57 checksum=0x849463c5`。
- 与上一阶段行为保持一致，说明本次主要是内部结构整理，没有改变 demo 行为。

下一步优先级：

- 继续完善 WING widget/state/style 公共层。
- 为 value widget 增加 keyboard step 行为。
- 开始规划真实 NuttX input adapter。

## 2026-06-12 更新记录：WING GUI Value Widget 支持键盘步进

本次更新继续推进 WING GUI Phase 2 的输入与控件语义。

完成内容：

- slider/scrollbar 已变为 focusable widget。
- slider/scrollbar 已支持方向键步进。
- WING key dispatch 调整为 `Tab` 遍历焦点，方向键优先交给 focused object。
- 新增 `wing_value_step()`，作为 value widget 的安全步进 helper。
- WING sim 配置提高到 input queue 24、event queue 64。
- `wing_gui_demo` 已覆盖 pointer + keyboard 的 value widget 验证路径。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- slider 验证：`83 -> 88 -> 83`。
- scrollbar 验证：`75 -> 80 -> 75`。
- 最终输出 `frame tick=33 commands=57 checksum=0x849463c5`。

下一步优先级：

- 继续推进 focus/state style 的可视反馈。
- 给 value widget 增加可配置 step 属性。
- 开始规划真实 NuttX input adapter。

## 2026-06-12 更新记录：WING GUI Value Widget 支持 Focused 可视反馈

本次更新继续推进 WING GUI Phase 2 的状态样式体系。

完成内容：

- slider/scrollbar 增加 focused state style。
- 新增 `wing_slider_set_state_style()` 和 `wing_scrollbar_set_state_style()`。
- focused 状态下 slider/scrollbar 会额外输出 focused outline 绘制命令。
- `wing_gui_demo` 已验证 focus gained、pointer value changed、keyboard step 和 final render。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- value widget step 属性化。
- widget state style 公共解析。
- NuttX input adapter。

## 2026-06-12 更新记录：WING GUI Value Widget Step 属性化

本次更新继续推进 WING GUI Phase 2 的控件属性模型。

完成内容：

- slider/scrollbar 增加 step 属性。
- 新增 slider/scrollbar step set/get API。
- 方向键步进改为读取组件自身 step。
- `wing_gui_demo` 使用 slider step=7、scrollbar step=9 验证配置生效。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- slider 验证：`83 -> 90 -> 83`。
- scrollbar 验证：`75 -> 84 -> 75`。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- widget state style 公共解析。
- NuttX input adapter。
- widget property 模型。

## 2026-06-12 更新记录：WING GUI State Style 公共解析

本次更新继续推进 WING GUI Phase 2 的 widget 公共层。

完成内容：

- `wing_widget` 增加 state-style select/store helper。
- `wing_box_t` 迁移到公共 state-style helper。
- `wing_box_t` 绘制复用 `wing_widget_draw_style()`。
- slider/scrollbar focused overlay 迁移到公共 state-style 选择 helper。
- `wing_gui_demo` 继续覆盖输入、事件、状态样式、timer、animation、value widget 和 final render。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- NuttX input adapter。
- widget property 模型。
- `wing_gui_demo` 分段展示当前全部能力。

## 2026-06-12 更新记录：wing_gui_demo 能力展示增强

本次更新继续推进 `wing_gui_demo` 的验证展示能力。

完成内容：

- `wing_gui_demo` 增加 setup / input script / runtime summary 分段输出。
- synthetic input 投递时输出 label、type、point、key。
- demo 输出 timer 和 animation 调度信息。
- demo 更明确展示当前 WING GUI 的输入、事件、控件、状态样式、timer、animation、render 链路。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- NuttX input adapter。
- dirty rect / partial redraw 展示。
- widget property 模型。
