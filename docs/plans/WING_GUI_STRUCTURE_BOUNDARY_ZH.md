# WING GUI 文件结构边界整理

本文记录 Phase 1 完成后，WING GUI 允许继续维护的结构边界清理原则。目标不是重新设计 WING，而是让后续 bugfix、边界清理、验证脚本维护和必要稳定性修正有清晰落点。

## 1. 核心原则

WING GUI 当前按职责分为四类实现：

- `src/core`：GUI runtime 核心，包括对象树、事件、输入、焦点、捕获、dirty、layout、render、theme、timer、animation、text edit model、默认 3D/2D 空间语义等。
- `src/widgets`：公开 widget 实现，例如 button、label、slider、scrollbar、text input、panel、card 等。
- `src/widgets/internal`：只服务 widget 层的内部支撑件，包括 widget 初始化、widget 状态/style/value helper、可复用 value input/drag 行为等。
- `src/wing_gui.c`：WING GUI 实例生命周期、主 tick/dispatch/render 推进入口。

## 2. 为什么移除 `src/behaviors`

`src/behaviors` 这个名字容易让人误解为 WING runtime 的独立行为系统，但当前内容本质上只是 widget 层的复用输入逻辑。

因此 Phase 1 后将它收敛到：

- `src/widgets/internal/wing_value_input.c`
- `src/widgets/internal/wing_value_input.h`

这样表达更直接：它不是 GUI core，也不是独立 behavior framework，而是 widget 层内部复用的 value-input helper。

## 3. 为什么把 `wing_widget*.c` 移出 `src/core`

`wing_widget.c`、`wing_widget_input.c`、`wing_widget_style.c`、`wing_widget_value.c` 这些文件虽然支撑多个 widget，但它们的职责不是 GUI runtime core，而是 widget 层的公共实现基础。

新的归属是：

- `src/widgets/internal/wing_widget.c`
- `src/widgets/internal/wing_widget_input.c`
- `src/widgets/internal/wing_widget_style.c`
- `src/widgets/internal/wing_widget_value.c`

公开头文件暂时仍保留在 `include/wing/core`，原因是 Phase 1 后不做大规模 API 破坏。后续若进入 API 整理阶段，可以再评估是否迁移为 `include/wing/widgets/...`。

## 4. `wing_text_input.c` 的归属

`wing_text_input.c` 保持在 `src/widgets`。

原因：text input 是一个具体 widget。它内部可以复用 core 的 `wing_text_edit` 文本编辑模型，但 widget 本身不应进入 `src/core`。这条边界要保持清楚：

- `wing_text_edit`：文本编辑模型，属于 core 能力。
- `wing_text_input`：可见 UI 控件，属于 widgets。

## 5. Phase 1 后允许继续做的事

允许：

- bugfix。
- 边界清理。
- 验证脚本维护。
- 必要稳定性修正。
- 不改变架构方向的文件归位和命名收敛。

不建议：

- 继续在 WING GUI 里扩展大型新 feature。
- 在 `src/core` 里继续增加 widget 专属逻辑。
- 在没有 FRender/NuttX graphics 能力补齐前，把复杂渲染能力直接堆进 WING GUI。

## 6. 当前阶段结构目标

WING GUI Phase 1 的稳定边界应是：

```text
apps/graphics/wing
  include/wing
    wing.h
    core/              public core/runtime headers
    widgets/           public widget headers

  src
    wing_gui.c         GUI instance lifecycle and tick entry
    core/              runtime core only
    widgets/           concrete widgets
    widgets/internal/  widget-only shared implementation helpers
```

这不是最终完美结构，但它足够清楚，可以支撑后续把主要精力切回 FRender 和 NuttX graphics。
