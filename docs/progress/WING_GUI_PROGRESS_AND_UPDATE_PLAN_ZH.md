# WING GUI 进度记录与更新计划

更新时间：2026-06-12

本文用于记录 WING GUI 当前真实进度、近期更新、验证记录和下一步计划。

本文不是完整架构设计文档。架构边界、设计来源和长期方案请参考：

- `FeatherCore/docs/WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md`
- `FeatherCore/docs/WING_GUI_DESIGN_REFERENCES_ZH.md`
- `FeatherCore/docs/FRENDER_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/FEATHER_RENDER_CORE_DECISION_ZH.md`
- `FeatherCore/docs/plans/WING_GUI_PROGRESS_AND_PLAN_ZH.md`

Phase 1 固定验收脚本：

- `FeatherCore/tools/firmware/sim/validate-wing-phase1.sh`

## 0. 最新验证记录

### 2026-06-12：修复连续动画运行时输入看起来失效的问题

问题现象：

- `wing_gui_demo` 打开无限循环动画后，X11 窗口可以收到鼠标输入，但拖动 slider / scrollbar 时控件视觉上不稳定或像没有响应。
- 关闭循环动画后，鼠标输入又恢复正常。

根因：

- 输入本身没有被动画抢占，也不应该存在“输入优先 / 动画优先”的设计。
- 真正问题在 `wing_gui_handle()` 的 present 区域取值时机：旧逻辑在 `wing_gui_tick()` 之后、`wing_gui_step()` 之前缓存 present rect。
- `wing_gui_step()` 内部才会派发输入事件；输入事件可能在同一帧继续产生新的 dirty rect。
- 当连续动画每帧都产生 dirty rect 时，旧 present rect 只覆盖动画 dirty；输入导致的 dirty 虽然已经画进 memory surface，但没有被同步 present 到 framebuffer / X11 窗口，因此看起来像输入失效。

修复：

- `wing_gui_render_dirty()` 在清除 dirty 之前保存本帧实际 redraw rect 到 `gui->last_redraw_rects[]`。
- `wing_gui_handle()` 不再使用 tick 后、step 前的 dirty 快照作为 present rect，而是使用渲染完成后的真实 redraw rect。
- 若渲染因命令容量或成本策略合并为 union redraw，则 present 同步使用同一个实际 redraw rect。
- 增加 `x11-input drag` 操作，发送 pointer down、带 ButtonMask 的 motion、pointer up，覆盖真实拖动场景。
- `validate-wing-phase1.sh` 新增无限动画运行中的 slider / scrollbar 拖动验证，并只检查拖动之后新增的日志，避免旧日志误判。

验证结果：

- 已重新构建 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过并生成 `FeatherCore/build/sim-wing`。
- 已运行 `./FeatherCore/tools/firmware/sim/validate-wing-phase1.sh --keep-log`，脚本内部再次重新构建 `sim-wing` 并通过。
- 验证过程中确认连续动画仍在运行：观察到 `continuous pulse animation segment=1 completed` 以及后续 segment 日志。
- 验证过程中确认 slider 拖动有效：观察到 `type=pointer_move pressed=yes point=520,196` 与拖动之后新增的 `wing_slider value changed`。
- 验证过程中确认 scrollbar 拖动有效：观察到 `type=pointer_move pressed=yes point=520,226` 与拖动之后新增的 `wing_scrollbar value changed`。
- 验证过程中确认拖动与动画同帧时 present rect 覆盖最终实际 redraw：日志中可见 drag 后 `dirty present x=20 y=70 w=600 h=140/162`，并出现 `present rect list count=1/2`。
- 验证过程中确认关闭 X11 窗口后 demo 退出并返回 NSH：观察到 `root received close request through WING input/event queue` 与 `app task exit`。

验证日志：

- `/tmp/tmp.G8U4YIDLPZ/wing-phase1.log`

### 2026-06-12：WING triangle render frontend seed

本次更新把 FRender triangle seed 上提到 WING render frontend，让 WING object tree 自身可以提交 triangle primitive。

已完成：

- WING 新增 `wing_triangle2d_t`。
- WING render 新增 `wing_gui_fill_triangle()`。
- `wing_gui_demo` 新增一个普通 `wing_obj_t` custom draw callback，通过 `wing_gui_fill_triangle()` 提交 triangle primitive。
- demo 继续避免新增 `wing_3d_view` 或专用 3D widget：triangle 是普通 object draw path 里的 render primitive。

验证结果：

- 已重新构建 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过并生成 `FeatherCore/build/sim-wing`。
- 已运行 `./FeatherCore/build/sim-wing`，从 NSH 执行 `wing_gui_demo`。
- `wing_gui_demo` 日志确认 `triangle primitive object uses WING render frontend -> FRender fill_triangle command seed bounds=314,96,42,36 opacity=210`。
- 首帧 command count 从旧路径增加到 `commands=180`，说明 triangle primitive 已进入 WING -> FRender command list。
- timer、repeat timer、animation、lifecycle create/delete、dirty-list redraw、present rect、projected hit-test、image quad path 正常。
- 已通过 X11 输入路径确认 mouse move、pointer down/up、keyboard Right 和 close request 可进入 WING input/event queue。
- 关闭 X11 framebuffer 窗口后，demo 收到 close request，应用任务退出并返回 NSH。

当前限制：

- WING triangle frontend 当前是 2D screen-space primitive。
- 后续 mesh / shader / GPU 3D path 还需要 vertex buffer、index buffer、UV、material/shader abstraction 和更明确的 depth/camera command 语义。
- 当前收益是 WING 层已经不再只能提交 rect/quad/image，未来 mesh 可以先拆为 triangle command seed 再逐步升级为批量 mesh command。

### 2026-06-12：FRender triangle seed

本次更新继续为后续 mesh / shader / GPU 3D backend 打底，先把最小三角形图元放进 FRender，而不是在 WING widget 层新增临时 3D 模块。

已完成：

- FRender 新增 `fr_triangle_t`。
- FRender 新增 `FR_CMD_FILL_TRIANGLE`。
- FRender 新增 `FR_CAP_FILL_TRIANGLE` / `FR_DRAW_CAP_FILL_TRIANGLE`。
- FRender software backend 新增 triangle scanline fill fallback。
- `fr_backend_supports()` 已能识别 `FR_CMD_FILL_TRIANGLE`。
- `frender_demo` 新增 `fill_triangle` stage，并在 draw caps 可读打印中显示 `fill_triangle`。

验证结果：

- 已重新构建 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过并生成 `FeatherCore/build/sim-wing`。
- 已运行 `./FeatherCore/build/sim-wing`，从 NSH 执行 `frender_demo`。
- `frender_demo` 日志确认 software caps 包含 `fill_triangle`。
- `frender_demo` 日志确认 `stage=fill_triangle commands=7 checksum=0xf47b55c5`。
- `frender_demo` framebuffer present 成功，关闭 X11 framebuffer 窗口后返回 NSH。
- 已继续从同一 sim 的 NSH 执行 `wing_gui_demo` 做回归验证。
- `wing_gui_demo` 日志确认 timer、repeat timer、animation、lifecycle create/delete、dirty-list redraw、present rect、projected hit-test、image quad path 正常。
- 已通过 X11 输入路径确认 mouse/click、keyboard Right 和 close request 可进入 WING input/event queue，关闭窗口后 demo 退出并返回 NSH。

当前限制：

- `FR_CMD_FILL_TRIANGLE` 当前只支持纯色填充。
- 还没有 indexed mesh / vertex buffer / UV / shader / depth buffer。
- triangle seed 的意义是稳定最小图元和 capability 语义，后续 WING/Pinion 可以在不改应用层的前提下把 quad 拆成 triangle 或提交 mesh。

### 2026-06-12：清理旧 3D 兼容命名的文档残留

本次没有修改源码，只同步进度文档与当前代码事实。

当前代码事实：

- WING core 不再提供 `transform3d` 兼容 API。
- 公开空间变换入口统一为 `wing_space_transform_t` 与 `wing_space_transform_*()`。
- object 侧统一使用 `wing_obj_set_space_transform()`、`wing_obj_get_space_transform()`、`wing_obj_get_world_space_transform()`、`wing_obj_space_transform_is_identity()`、`wing_obj_reset_space_transform()`。
- 普通 2D widget 是默认 object space 下的 identity transform 状态。
- 空间效果属于 object space + runtime camera + projection + FRender command，不属于独立 `wing_3d.c` 或 `wing_3d_view.c`。

由于本次只修改文档，没有 C 源码、头文件、构建配置或 demo 行为变更，因此未重复构建 `sim-wing`。

### 2026-06-12：FRender image quad 与 WING image object space path

本次更新继续坚持“默认 3D/object space 是 WING GUI core 能力，而不是外挂 3D widget”的方向。

已完成：

- FRender 新增 `FR_CMD_BLIT_QUAD` / `FR_CAP_BLIT_QUAD` / `FR_DRAW_CAP_BLIT_QUAD`。
- FRender software backend 新增 image quad 第一阶段 fallback：按 projected quad 的扫描线区域采样源 image，并叠加 global alpha。
- WING render 新增 `wing_gui_blit_quad_alpha()`，用于把 image resource 作为 textured quad 提交给 FRender。
- `wing_image_t` 在 object space transform 非 identity 时走 FRender image quad path；identity 状态仍走普通 blit fast path。
- `wing_gui_demo` 为 image widget 设置 `rotation_y=-12`、`z=10`，并输出 image quad path 的能力说明。

验证结果：

- 已重新构建 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过并生成 `FeatherCore/build/sim-wing`。
- 已运行 `./FeatherCore/build/sim-wing`，从 NSH 执行 `wing_gui_demo`。
- demo 日志确认 image widget 进入 `FRender image quad command when object space is non-identity` 路径。
- demo 日志确认 timer、repeat timer、animation、lifecycle create/delete、dirty-list redraw、present rect、projected hit-test、same z-index depth ordering 均正常执行。
- 已通过 `x11-input.sh` 发送 mouse move、pointer click、keyboard Right，WING input provider 捕获并派发到 event queue。
- 已通过 `x11-input.sh ... close` 关闭 X11 framebuffer 窗口，demo 收到 close request，应用任务退出并返回 NSH。

当前限制：

- `FR_CMD_BLIT_QUAD` software fallback 是第一阶段保守扫描线采样，不是 perspective-correct texture mapping。
- mesh / shader / GPU 3D 还未接入；本次只先把 command / API / widget path 打通，避免后续补 3D 后端时重构 WING app 层。

## 1. 当前主线

当前第一阶段主线是：

```text
先实现可独立运行的 WING GUI
再基于 WING GUI 实现 WING Desktop
```

WING GUI 是一个 GUI 库。任何 WING GUI 程序都可以从 NSH 或普通应用入口直接启动，不强制依赖 WING Desktop。

WING Desktop 是建立在 WING GUI 之上的默认桌面系统，是一个可选的大型基础 WING GUI 应用。需要桌面安装、下载、窗口管理、任务栏、启动器时，再进入 WING Desktop 层。

当前运行链路保持为：

```text
NSH / normal app entry
        |
        v
wing_gui_demo
        |
        v
WING GUI runtime
  object tree / input / event / timer / animation / layout / dirty / render frontend
        |
        v
FRender
  command list / software fallback / capability / framebuffer present
        |
        v
NuttX graphics
  framebuffer / simulator display / low-level graphics foundation
```

## 2. 当前能力矩阵

### 2.1 FRender

`apps/graphics/frender` 已经可以作为 WING GUI 第一阶段渲染底座。

已经具备：

- command list。
- clear / fill rect / stroke rect / fill quad / fill triangle / blit / blit quad / clip。
- software backend。
- software backend 的 `FR_CMD_CLEAR` 已遵守当前 clip stack，可支撑 dirty-list chunk redraw。
- software backend 已支持 `FR_CMD_BLIT` 的 nearest-neighbor scale 和 per-pixel alpha blend。
- software backend 已支持 `FR_CMD_FILL_TRIANGLE` 的第一阶段 scanline fill fallback。
- software backend 已支持 `FR_CMD_BLIT_QUAD` 的第一阶段 image quad fallback。
- backend capability declaration。
- backend registry。
- NuttX graphics capability adapter。
- framebuffer present adapter。
- `frender_demo` 可从 NSH 执行。
- `frender_demo` 已覆盖 `FR_CMD_FILL_QUAD`，可展示 FRender 当前 quad 图元能力。
- sim 下执行 demo 后才打开 framebuffer / X11 窗口。
- 关闭窗口后 demo 退出并返回 NSH。

当前限制：

- NuttX graphics 目前主要作为 present / capability adapter。
- 绘制命令还没有直接提交给 NXBE / DMA2D / GPU2D。
- planner / cost policy / hardware fallback 还没有完成。
- text 已有 `wing_font_t` / builtin 5x7 font resource seed；image 已有 blit 与 image quad seed；path / gradient / mesh / shader 等高级命令还没有完成。

### 2.2 WING GUI runtime

`apps/graphics/wing` 已经进入 GUI runtime 第一阶段。

已经具备：

- `wing_gui_t` GUI 执行句柄。
- `wing_gui_tick()` 推进时间。
- `wing_gui_step()` 推进 layout / input / event / render。
- `wing_gui_step()` 现在只负责 runtime 调度，dirty-list chunk redraw 执行已下沉到 render 模块。
- `wing_gui_handle()` 作为应用侧周期性调用的 GUI handler，内部串起 tick 和 step。
- `wing_gui_frame_t` 返回 handler 内部 frame 诊断信息。
- WING render 已通过 FRender 提供 clear / fill rect / stroke rect / fill quad / blit / image quad 的第一阶段统一提交路径。
- WING render 已通过 FRender 提供 `wing_gui_fill_triangle()`，用于提交最小 triangle primitive seed。
- `wing_gui_set_input_reader()` 注册输入 provider。
- `wing_gui_poll_input()` 在 handler 内部拉取 provider 输入并放入 input queue。
- `wing_gui_set_theme()` / `wing_gui_get_theme()` 保存当前 GUI 主题指针。
- `wing_value_model_t` 作为数值型 widget 的统一 value model seed。
- `wing_gui_request_stop()` / `wing_gui_is_running()`。
- object tree。
- object bounds / state / flags。
- object invalidation。
- dirty rect 基础记录。
- dirty invalidation / dirty list / dirty merge / dirty clear 实现已迁移到 `src/core/wing_dirty.c`，并新增 `include/wing/core/wing_dirty.h` 作为专属模块头。
- dirty rect list 第一阶段记录。
- dirty rect merge policy 第一阶段记录。
- dirty-list chunk redraw 第一阶段。
- render context API 与 dirty-list chunk redraw 执行路径已迁移到 `src/core/wing_render.c`，并新增 `include/wing/core/wing_render.h` 作为专属模块头。
- draw traversal。
- hit test。
- input queue。
- input provider。
- input reader / input queue / pointer-key dispatch 实现已迁移到 `src/core/wing_input.c`，并新增 `include/wing/core/wing_input.h` 作为专属模块头。
- focus traversal / focus state / focus gained/lost event 实现已迁移到 `src/core/wing_focus.c`，并新增 `include/wing/core/wing_focus.h` 作为专属模块头。
- event queue。
- event queue 的 post / dispatch / stop propagation 实现已迁移到 `src/core/wing_event.c`，并新增 `include/wing/core/wing_event.h` 作为专属模块头。
- event bubbling。
- stop propagation。
- focus gained / focus lost。
- keyboard focus traversal。
- key down / key up。
- pointer down / move / up。
- 正式 pointer capture 第一版：`wing_gui_capture_pointer()` / `wing_gui_release_pointer()` / `wing_gui_cancel_pointer_capture()` / `wing_gui_get_pointer_capture()`。
- pointer capture / release / cancel 实现已迁移到 `src/core/wing_capture.c`，并新增 `include/wing/core/wing_capture.h` 作为专属模块头。
- pointer capture lifecycle 事件：`WING_EVENT_POINTER_CAPTURED` / `WING_EVENT_POINTER_RELEASED` / `WING_EVENT_POINTER_CANCELLED`。
- click event。
- software timer。
- repeat timer 和 `wing_gui_timer_stop()` 自停止路径。
- timer 注册与停止实现已迁移到 `src/core/wing_timer.c`，并新增 `include/wing/core/wing_timer.h` 作为专属模块头。
- linear animation。
- animation easing/path seed：`wing_gui_anim_start_path()` 支持 linear / ease-in / ease-out / ease-in-out。
- animation 启动与停止实现已迁移到 `src/core/wing_animation.c`，并新增 `include/wing/core/wing_animation.h` 作为专属模块头。
- fixed / horizontal stack / vertical stack / center layout。
- `wing_theme_t` 默认主题 seed。
- `wing_theme_init_default()` 初始化基础组件和状态样式。
- `wing_theme_init_high_contrast()` 初始化高对比主题 seed。
- 基础组件：box、button、label、panel、progress、slider、scrollbar、switch、checkbox。
- 新增 `wing_text_edit_t` 文本编辑 core helper：固定调用方 buffer、length、cursor、selection range、insert/backspace/delete/left/right 从 widget 中拆出，为后续 IME / clipboard / multiline 编辑打底。
- 新增 `wing_text_input_t` 单行文本输入 seed：组件拥有固定调用方 buffer、selection highlight 子对象、label 子对象、cursor 子对象，复用 `wing_text_edit_t` 处理编辑状态，支持 pointer focus、select-all、visible selection highlight、selection replace、printable key、left/right cursor、backspace/delete 和 `WING_EVENT_VALUE_CHANGED`。
- progress / slider / scrollbar 已共享 `wing_value_model_t` 处理 range / value / step。
- widget base 已开始承载共享 value dispatch：numeric value 和 boolean value 更新会统一执行 value 更新、object invalidation 和 `WING_EVENT_VALUE_CHANGED` 派发。
- widget base 已开始承载共享 pointer lifecycle helper：pointer down / up / cancel 会统一维护 pressed state 并阻止不必要冒泡。
- widget base 已开始承载共享 state style 容器：pressed / focused / disabled 样式的初始化、存储和选择由统一 helper 处理。
- widget base 已开始承载 checked state seed：state style 容器支持 `WING_OBJ_STATE_CHECKED`，switch / checkbox 会把 boolean value 同步到 object state。
- widget base 已开始承载 selected / active state seed：state style 容器支持 `WING_OBJ_STATE_SELECTED` 和 `WING_OBJ_STATE_ACTIVE`，普通 box 也可以通过通用 object state 获得选中/激活视觉。
- object state 变化已具备第一版可观察事件：`WING_EVENT_STATE_CHANGED` 携带 `wing_state_event_t`，可报告 old/new/changed bitmask，供组件、桌面和应用响应状态变化。
- object bounds / space transform 变化已具备第一版可观察事件：`WING_EVENT_BOUNDS_CHANGED` 携带 `wing_bounds_event_t`，`WING_EVENT_SPACE_TRANSFORM_CHANGED` 携带 `wing_space_transform_event_t`，用于响应 layout、animation 和默认 object space 属性变化；旧 transform event 名称已移除。
- runtime camera 变化已具备第一版可观察事件：`WING_EVENT_CAMERA_CHANGED` 携带 `wing_camera_event_t`，`wing_gui_set_camera()` 会在 camera 改变前后触发全局 redraw，使默认 object space 的 camera 成为可观察、可刷新的一等 runtime 状态。
- camera equality 已收敛到 core space：`wing_camera_equal()` 统一判断 runtime camera 是否发生真实变化，`wing_gui_set_camera()` 复用该入口跳过 no-op camera 更新。
- 新增 `wing_space_transform_equal()` core space API，并让 `wing_obj_set_space_transform()` 复用它过滤 no-op transform；transform changed event 的比较规则开始归入默认空间层。
- object enabled flag 已开始和 disabled state 同步：`wing_obj_set_flags()` 清除 `WING_OBJ_FLAG_ENABLED` 时会设置 `WING_OBJ_STATE_DISABLED`，恢复 enabled 时会清除 disabled state。
- object enabled 状态已提供公开 API：`wing_obj_set_enabled()` / `wing_obj_is_enabled()`。
- object visible 状态已提供公开 API：`wing_obj_set_visible()` / `wing_obj_is_visible()`。
- object selected / active 状态已提供公开 API：`wing_obj_set_selected()` / `wing_obj_is_selected()` / `wing_obj_set_active()` / `wing_obj_is_active()`。
- object lifecycle 已提供第一版公开销毁 API：`wing_obj_destroy_tree()`。
- object 首次绑定 GUI runtime 时会同步派发 `WING_EVENT_CREATE`，销毁动态子树时会同步派发 `WING_EVENT_DELETE`。
- `wing_gui_get_dirty_rect()` 可观察当前 dirty rect。
- dirty 相关公开观测 API 仍通过 `<wing/wing.h>` 兼容导出，同时已具备 `wing/core/wing_dirty.h` 专属模块头。
- `wing_gui_get_dirty_rect_count()` / `wing_gui_get_dirty_rect_at()` 可观察 dirty list。
- `wing_gui_get_dirty_merge_count()` 可观察 dirty list 合并次数。
- `wing_gui_frame_t.redraw_count` 可观察当前 frame 实际重绘分块数量。
- `wing_gui_demo` 可在无 framebuffer presenter 的 headless 验证中运行固定多帧。
- `wing_gui_demo` 可打印 handler 前、tick 后、handler 后的 dirty union、dirty list count、dirty merge count 和 redraw chunk 生命周期。
- `wing_gui_demo` 可打印 render command capacity fallback 状态，用于观察 dirty-list redraw 是否因 command list 容量预算退回 union redraw。
- `wing_gui_demo` 可打印 render redraw cost fallback 状态，用于观察 dirty-list redraw 是否因面积/命令成本策略退回 union redraw。
- `wing_gui_demo` 已使用 button 的 center layout 自动居中摆放 `GO` label 子对象，验证嵌套对象布局不再依赖 root 级绝对坐标。
- 新增 `WING_LAYOUT_FILL`，父对象可以把可见子对象拉伸到自身 padding 后的内容区域。
- `wing_gui_demo` 已增加 fill-layout badge 子树，并在首帧打印 layout 后的 child bounds，用于验证 fill layout 真实生效。

当前限制：

- `<wing/wing.h>` 仍然承担过多聚合职责，下一阶段需要逐步拆成 core / runtime / input / event / animation / widgets 等专属头文件，同时保留 `wing.h` 作为兼容聚合入口。
- dirty-list redraw 第一阶段已落地，但还没有 tile scheduler、cost threshold、command capacity fallback 和硬件 planner。
- dirty-list redraw 已新增 command capacity fallback 第一版：当 dirty-list chunk 数超过当前 FRender command list 的保守容量预算时，本帧退回 union dirty redraw，并通过 `wing_gui_frame_t` 暴露 planned/actual chunks 与 fallback 标志。
- dirty-list redraw 已新增 redraw cost policy 第一版：当 union dirty 面积相对 dirty-list 总面积足够划算时，本帧退回 union dirty redraw，减少重复遍历 object tree 和重复提交 command list 的成本。
- style / theme 仍然是 seed 阶段，还不是完整继承式 theme / selector / cascade 系统。
- text 已从 label 私有矩形 glyph 推进到轻量 `wing_font_t` 资源 seed，并具备最小 UTF-8 codepoint 解码、显式换行测量/绘制、bounds 内空格优先自动换行、单行 ellipsis 截断、`wing_text_edit_t` 编辑模型、selection range、可见 selection highlight 和单行 text input seed；但还没有字体转换器、字距、复杂脚本、IME、复制粘贴、多行编辑和 FRender text command。
- animation 只有线性插值，没有 easing / timeline / property binding。
- input 主要依赖 synthetic demo，尚未接真实 NuttX touch / mouse / keyboard。
- widget base 还需要继续抽象，减少组件重复代码。
- WING Desktop 尚未开始重建。

### 2.3 wing_gui_demo

`apps/examples/wing_gui_demo` 是 WING GUI 第一阶段验证模板。

它的职责是展示 WING GUI runtime 的能力，而不是实现桌面系统。

当前已经覆盖：

- GUI 初始化。
- 应用主循环。
- `wing_gui_handle()`。
- object tree 创建。
- 基础组件绘制。
- input queue。
- input provider。
- event queue。
- pointer event。
- keyboard event。
- focus。
- text input。
- event bubbling。
- stop propagation。
- timer。
- repeat timer。
- timer stop。
- animation。
- animation easing/path。
- layout。
- dirty / redraw。
- 默认主题 seed。
- 运行时主题切换 seed。
- FRender command list 输出。
- framebuffer present 尝试。
- headless 环境下的多帧自动验证。
- timer / animation / theme switch 触发 invalidation。
- repeat timer 周期性触发 invalidation，并在 callback 内调用 `wing_gui_timer_stop()` 自停。
- ease-out width animation 触发 object bounds 更新和 dirty redraw。
- timer 动态挂载 toast 子树、隐藏 toast 子树并随后销毁，覆盖 object lifecycle create/delete event、visibility、动态 object tree 和 dirty redraw。
- 移除 `wing_gui_draw_demo_scene()` 历史遗留 API，render core 不再包含固定坐标、固定颜色的默认 demo 画面；demo 画面统一由 `wing_gui_demo` 通过 object tree、widget、theme 和显式设计参数构建。
- 新增 `include/wing/core/wing_object.h`，object tree / bounds / flags / state / layout / hit-test / draw traversal 等公开 API 开始拥有独立 core 头文件；`wing.h` 继续作为聚合入口保持兼容。
- 清理 `wing.h` 中已经没有实现的 `wing_gui_draw_demo_scene()` 过期声明，避免公共 API 暴露不存在的 demo 绘制函数。
- 新增 `include/wing/core/wing_runtime.h`，`wing_gui_t` 执行句柄相关的 create/destroy/root/theme/handler/dirty observation API 开始拥有独立 runtime 头文件；`wing.h` 继续聚合该头保持兼容。
- `wing.h` 已开始聚合 `include/wing/core/wing_input.h`，input reader / poll / enqueue / dispatch 公开 API 不再重复塞在 `wing.h` 主声明区。
- `wing.h` 已开始聚合 `include/wing/core/wing_event.h`，event queue / dispatch / stop propagation 公开 API 不再重复塞在 `wing.h` 主声明区。
- WING GUI core 不再保留固定 `12px` slider knob fallback，未显式配置 knob size 时按控件高度推导；具体 demo 视觉尺寸由 `wing_gui_demo` 显式设置。
- `wing_gui_demo` 开始集中维护 demo design constants，控件位置、尺寸、padding、timer 节奏、动画目标和输入脚本坐标不再散落在初始化代码中。
- 明确默认空间能力是 WING GUI core 的一等能力，3D 不再表述为边缘扩展：普通 2D widget 是默认 3D 空间下的 identity transform 状态，后续 mesh / shader / GPU 3D backend 可以继续消费同一套 object/camera/transform 数据。
- HoneyGUI 的 Lite3D 参考路径已确认：model 内部持有 viewport / camera / world，应用通过 global transform callback 初始化 camera 和 world，再由 Lite3D push/draw 阶段完成 camera matrix、perspective 和 screen transform；WING 后续应吸收这一结构，而不是直接把完整 Lite3D 场景系统塞进 GUI core。
- 新增 WING core space seed：`wing_vec3_t`、`wing_viewport_t`、`wing_camera_t`、`wing_space_transform_t`、`wing_project_point()`，并让 `wing_obj_t` 默认携带 identity `space_transform`。
- 新增 runtime camera：`wing_gui_t` 现在持有默认 `wing_camera_t`，并提供 `wing_gui_set_camera()` / `wing_gui_get_camera()`，避免每个 widget 自行临时创建空间 camera。
- 新增 `wing_space_transform_compose()`，用于把父对象空间状态与子对象本地 transform 组合成 world transform，后续 mesh / shader / GPU 3D backend 可以复用同一套 transform 组合语义。
- 新增 `wing_space_transform_apply_point()` core space API，局部点进入默认空间的 scale / rotation / translation 过程不再藏在 projection 内部；`wing_project_point()` 现在复用该入口后再执行 camera / perspective。
- 新增 `wing_project_point_with_depth()` core space API，点投影可以公开返回 projected depth；这让后续 mesh vertex、shader path、GPU 3D backend、picking 和空间排序有统一 depth 来源。
- 新增 `wing_obj_compare_space_order()` object API，并让 object tree draw/hit traversal 开始使用 `z-index + projected average depth` 的默认空间排序规则：z-index 优先，同 z-index 内远处对象先绘制、近处对象优先命中。
- `wing_gui_demo` 新增同 z-index depth sorting 验证场景：两个重叠 `wing_card_t` 只通过 object transform 的 `translation.z` 区分前后，输入脚本点击重叠区域应命中 projected depth 更近的 front card。
- WING GUI 默认 input queue seed 从 32 提升到 64，避免能力覆盖型 demo 在首帧密集 synthetic input 下触发 `-ENOSPC`；真实产品配置后续仍可通过 `CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE` 调整。
- 新增 `wing_project_rect_projected_quad()` core space API，矩形平面投影现在能返回每个角点的 screen/depth 顶点；2D quad API 只是从该结果中取 screen 点。
- 新增 `wing_projected_quad_average_depth()` core space API，projected quad 的代表 depth 由默认空间统一计算，避免 render/picking/widget 各自重复实现。
- 新增 object transform identity/reset API：`wing_obj_space_transform_is_identity()` / `wing_obj_reset_space_transform()`，用于明确普通 2D object 是默认 core space 下的 identity transform 状态。
- 新增 `wing_space_transform_is_identity()` core space API，并让 object identity 查询复用它；普通 2D 默认状态现在由 space 层统一定义。
- 新增 object world transform API：`wing_obj_get_world_space_transform()`，第一阶段通过 parent chain 组合 transform，使默认空间能力开始进入 object tree 继承语义。
- 新增 object screen bounds API：`wing_obj_get_screen_bounds()`，identity transform 下严格返回原始 2D bounds，非 identity transform 下返回投影后的屏幕包围盒，作为默认空间 dirty / hit / render culling 的第一阶段底座。
- 新增 object projected quad API：`wing_obj_project_quad()`，object 可以直接返回带 screen/depth 顶点的 projected quad；旧 `wing_obj_project_quad2d()` 保持 2D 兼容视图并复用该路径。
- 新增 object projected depth API：`wing_obj_get_projected_depth()`，object 可直接返回 projected average depth，后续 z-index、depth sort、picking 可以组合使用。
- 新增 object projected quad API：`wing_obj_project_quad2d()`，把 object world transform + runtime camera 投影为 2D quad，作为 card / 未来 mesh seed 共享的投影入口。
- 新增 object projected hit-test API：`wing_obj_contains_point()`，identity object 使用原始 2D rect，非 identity object 使用 projected quad；`wing_obj_hit_test()` 已接入该路径，避免空间控件交互只停留在轴对齐 screen bounds。
- 新增 `wing_quad2d_contains_point()` core space API，object projected hit-test 复用默认空间几何判断；后续 mesh / shader picking 可以共享该几何入口。
- 新增 `wing_card_t` 第一版 core widget：当前用 WING camera/transform/project 做软件投影，再通过 FRender `FR_CMD_FILL_QUAD` 绘制投影后的 space card 面；未来 FRender 增加 mesh / shader / GPU 3D backend 后可替换执行路径，不要求 WING app 大改。
- `wing_card_t` 绘制已改为消费 `wing_obj_project_quad2d()`，不再自行临时创建 camera / 拼 object transform，dirty screen bounds 与实际 quad 绘制开始共享同一套 object projection/runtime camera 路径。
- `wing_gui_demo` 已把 space card 的旋转动画切到 `wing_obj_set_space_transform()` 路径，验证空间变换是 `wing_obj_t` 的默认一等属性，而不是 `wing_card_t` 的孤立专属能力。
- `wing_gui_demo` 已通过 switch / checkbox toggle 日志验证 `WING_OBJ_STATE_CHECKED` 与 boolean value 同步，value changed 事件回调内可观察到更新后的 checked object state。
- `wing_gui_demo` 已增加 space card projected pointer input，点击点先通过 `wing_obj_contains_point()` 验证 projected quad 命中，再通过 input queue / hit-test / event queue 输出 `space card received click through projected quad hit-test`。
- WING 默认空间能力继续从“独立 3D 模块”收敛为 core object space：当前工作树不保留 `wing_3d.c` / `wing_3d_view.c`，demo 也改用 `space_anim` 命名，避免把 3D UI 效果误建模成单独 view/widget 子系统。
- `WING_GUI_DESIGN_REFERENCES_ZH.md` 已同步修正：普通 widget 被明确为默认 3D 空间中的 identity transform 状态；camera / transform / projection 属于 WING GUI core，后续 mesh / shader / GPU 3D backend 应消费同一套 object space 数据。
- 默认 `CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE` 已提升到 64，且 sim 构建脚本已同步写入 64，保证 `wing_gui_demo` 可以同时覆盖 button、checkbox、switch、space card、同 z-index depth sorting、focus、slider、scrollbar、capture cancel 和 input provider 路径，不因验证脚本规模增长提前耗尽队列。
- 默认 `CONFIG_GRAPHICS_WING_EVENT_QUEUE_SIZE` 已从 64 提升到 128，保证 demo 一次性输入脚本生成的 focus / click / value / capture lifecycle / bubbled event 不会在首帧验证中耗尽事件队列。
- `wing_gui_demo` 已调用 `wing_obj_reset_space_transform()` 并输出 root object 的 transform identity 状态，用于验证普通 2D widget 可回到 core space identity transform 状态。
- `wing_gui_demo` 已输出 runtime camera、root identity screen bounds、space card projected dirty bounds、space card projected quad 和 space card world transform，用于验证默认空间 camera / bounds / quad / world transform 已进入 object runtime 观测路径。
- `wing_gui_demo` 已通过临时调整并恢复 runtime camera focal length 验证 `wing_gui_set_camera()` 会派发 `WING_EVENT_CAMERA_CHANGED`，并触发默认 object space 的重绘路径。
- `wing_gui_demo` 已输出 `wing_camera_equal()` 的 restored 检查结果，并通过 no-op camera update 事件计数保持不变来验证 camera equality 属于 core space 语义而不是 runtime 私有判断。
- 新增 object space transform 命名入口：`wing_space_transform_t`、`wing_obj_set_space_transform()`、`wing_obj_get_space_transform()`、`wing_obj_get_world_space_transform()`、`wing_obj_space_transform_is_identity()`、`wing_obj_reset_space_transform()`；旧 `transform3d` API 已移除，demo 和新实现统一表达默认 object space。
- `wing_obj_t` 内部字段已从 `space transform` 收敛为 `space_transform`，保留旧 `wing_obj_*transform3d*` API 作为兼容层；WING core 自身继续把默认 2D/3D 统一表达为 object space。
- 底层 transform 结构主 tag 已从 `wing_transform3d_s` 收敛为 `wing_space_transform_s`；旧 `wing_transform3d_t` typedef 已移除。
- 新增 core space transform 命名入口：`wing_space_transform_init()`、`wing_space_transform_is_identity()`、`wing_space_transform_equal()`、`wing_space_transform_compose()`、`wing_space_transform_apply_point()`；旧 `wing_transform3d_*` 保留兼容，后续新代码优先使用 `wing_space_transform_*`。
- `wing_space_transform_*` 已成为 core space transform 的主实现入口；旧 `wing_transform3d_*` API 退为兼容包装层，避免默认空间能力继续被实现层建模为外置 3D 扩展。
- core space 投影路径的函数定义和 identity fallback 已收敛为 `wing_space_transform_t` / `wing_space_transform_*`，投影、dirty bounds 和 picking 不再在实现层依赖旧 `transform3d` 主命名。
- 新增 space transform 事件主命名：`WING_EVENT_SPACE_TRANSFORM_CHANGED` 和 `wing_space_transform_event_t`；旧 `WING_EVENT_TRANSFORM_CHANGED` / `wing_transform_event_t` 与新命名共享同一事件值和 payload 结构，作为兼容入口保留。
- `wing_card_t` 内部实现和公开组件头参数命名已从历史 `view` 收敛为 `card`，避免继续暗示存在独立的 `3D view` 特例；它现在只是使用默认空间能力的普通 widget。
- `wing_card_t` 已移除 card 私有 camera / transform API，后续所有 widget 的空间状态都应统一走 `wing_obj_t` transform 与 `wing_gui_t` runtime camera；2D 控件就是默认 3D 空间里的 identity transform 状态。
- 新增 `wing_scroll_view_t` 第一阶段能力：基于 object clip-children 提供 viewport，并通过 offset 推动内容子对象；`wing_gui_demo` 增加 scroll view 验证日志，后续可扩展为滚动列表、窗口内容区和桌面区域。
- `wing_scroll_view_t` 已接入 focused key / encoder 输入，支持 content size、step 和 offset clamp；`wing_gui_demo` 通过 pointer focus、keyboard right 和 encoder rotate 验证 viewport offset 更新。
- 新增 `WING_EVENT_SCROLL_CHANGED` / `wing_scroll_event_t`，scroll view offset 变化会同步携带 old/new offset 与 max offset；`wing_gui_demo` 已输出 scroll changed 日志验证应用可观察滚动状态。
- `wing_scroll_view_t` 已新增 `wing_scroll_view_scroll_by()` / `wing_scroll_view_get_max_offset()`，并在 `wing_gui_demo` 中验证相对滚动会被 clamp 到 max offset，避免上层应用直接依赖内部字段。
- 新增 `wing_obj_set_z_index()` / `wing_obj_get_z_index()`，让 sibling 的绘制和命中顺序显式进入 WING 默认空间模型：默认 `z_index=0` 保持历史插入顺序，较高 `z_index` 后绘制并优先命中。`wing_gui_demo` 已给 space card 设置较高 z-index，用于验证 projected hit-test 与空间层级可以共存。
- 新增 `wing_box_style_t.opacity` 第一阶段样式透明度，公共 `wing_widget_draw_style()` 会在 fill/stroke 前计算 alpha，`wing_card_t` 的 projected quad/edge 也接入同一条 style opacity 路径。FRender software backend 已补 source-over alpha blend，使 opacity 不只是日志能力，而是真正改变软件 framebuffer 像素。
- 新增 object tree 继承透明度 seed：`wing_obj_t.opacity`、`wing_obj_set_opacity()`、`wing_obj_get_opacity()`、`wing_obj_get_effective_opacity()` 已落地，颜色型 widget 绘制会同时乘入 style opacity 与 object effective opacity；`wing_gui_demo` 已让 fill panel 子树验证父对象 opacity 会继承到 badge 子对象。
- 新增 pointer hover seed：`WING_OBJ_STATE_HOVERED`、`WING_EVENT_POINTER_ENTER`、`WING_EVENT_POINTER_LEAVE` 和 runtime `hovered_obj`，pointer move/down/up 会维护 enter/leave 与 hovered state。公共 state-style 选择已支持 hovered，`wing_gui_demo` 已用 button hover enter/leave 展示 hovered style 与事件日志。
- 新增 encoder rotate input seed：`WING_INPUT_ENCODER_ROTATE` 进入统一 input queue，`wing_gui_dispatch_input()` 会把它派发为 `WING_EVENT_ENCODER_ROTATE` 给 focused object；slider / scrollbar 已复用 value model step 消费 encoder delta，`wing_gui_demo` 已加入 slider 和 scrollbar 的 encoder 日志验证。
- 新增 close request input seed：`wing_gui_request_close()` 进入统一 input queue，`wing_gui_dispatch_input()` 会把它派发为 `WING_EVENT_CLOSE_REQUEST` 给 root object，再默认请求 runtime 停止；`wing_gui_demo` 的 framebuffer/X11 窗口关闭已经不再直接调用 stop，而是走 WING input/event queue。
- 新增 `wing_image_t` 基础 image widget seed：WING GUI 现在可以接收静态 RGBA 像素资源，并通过 FRender `FR_CMD_BLIT` 绘制；`wing_gui_demo` 已加入 4x4 静态 image resource，后续可替换为 texture cache / resource command / hardware blit planner。
- 新增 `wing_font_t` / `wing_bitmap_glyph_t` / `wing_font_builtin_5x7()` core font seed：`wing_label_t` 不再私有维护 glyph 表，而是通过 font resource 查询 glyph、测量文本并执行对齐绘制；`wing_gui_demo` 已输出 builtin font 名称、文本测量结果和 label 对齐状态。
- 新增 `wing_text_next_codepoint()` 最小 UTF-8 解码入口：`wing_font_measure_text()` 和 `wing_label_t` 绘制都按 codepoint 遍历字符串，ASCII 继续走 builtin glyph，当前 builtin 字体缺失的非 ASCII codepoint 会走 fallback glyph；`wing_gui_demo` 已用 escaped UTF-8 样例输出 codepoint count 和测量结果。
- `wing_font_measure_text()` 和 `wing_label_t` 已支持显式 `\n` 多行文本：测量时取最大行宽并累计 line height，绘制时每行单独按 label align 计算水平位置；`wing_gui_demo` 已输出 multiline measurement 样例。
- 新增 `wing_label_text_mode_e` 与 `wing_label_set_text_mode()` / `wing_label_get_layout_size()`：`WING_LABEL_TEXT_MODE_WRAP` 会在 label bounds 内优先按空格断行，必要时再按 glyph advance 兜底硬断行，`WING_LABEL_TEXT_MODE_ELLIPSIS` 会在单行放不下时绘制 `...`。`wing_gui_demo` 已新增 wrap / ellipsis label 样例和布局日志。
- 新增 `wing_text_input_t` 单行文本输入 seed：它作为普通 WING widget 进入 object tree，内部复用 box/selection highlight/label/cursor 子对象，并把 buffer / cursor / selection / edit command 委托给 `wing_text_edit_t` core helper；demo 通过 select-all 初始选区、visible selection highlight 和 synthetic pointer/key 输入验证 selection replace、focus、cursor、printable key、left/right、backspace/delete、dirty redraw 和 value changed event。
- FRender 已新增 `fr_cmd_blit_alpha()` / command `global_alpha`，WING 已新增 `wing_gui_blit_alpha()`；`wing_image_t` 现在会使用 object effective opacity 执行 global-alpha blit，`wing_gui_demo` 已设置 image opacity=196 验证图片资源也进入统一透明度继承路径。
- 新增轻量 `wing_image_resource_t` 描述层，`wing_image_t` 推荐通过 `wing_image_init_resource()` / `wing_image_set_resource()` 消费图片资源；旧像素指针入口继续保留为兼容包装，内部转成 inline resource，为后续资源转换工具、texture cache 和硬件 blit planner 留出稳定入口。
- FRender 已新增 `FR_CMD_FILL_QUAD` / `FR_DRAW_CAP_FILL_QUAD` 第一阶段图元种子，WING card widget 直接提交 projected quad 给统一 Render backend。
- `frender_demo` 已新增 `fill_quad` stage，FRender 基础 demo 和 WING card widget demo 都进入同一条 quad command 验证链。
- FRender 已新增 `FR_CMD_BLIT` / `FR_CAP_BLIT` 第一阶段图元种子，WING image widget 直接提交 RGBA image blit 给统一 Render backend。
- `frender_demo` 已新增 `blit` stage，FRender 基础 demo 和 WING image widget demo 都进入同一条 blit command 验证链。

## 3. 最新更新记录

### 2026-06-12

已完成：

- 明确 WING GUI 和 WING Desktop 分层。
- 明确 WING GUI 应先于 WING Desktop 实现。
- 明确 FRender 是 WING GUI 和 Pinion 共享渲染底座。
- 明确 FRender 不放入 `nuttx/graphics`，而是放在 `apps/graphics/frender`。
- 整理 FRender 与 NuttX graphics 的边界。
- 建立 WING GUI 基础执行模型。
- 建立 `wing_gui_demo` 作为 NSH 可执行验证模板。
- 新增本文作为后续开发进度和更新计划入口。
- 新增 `wing_gui_handle()`，应用可以用一个 handler 调用推进 timer、animation、layout、input、event、render。
- 新增 `wing_gui_frame_t`，用于 demo 和调试查看 handler 内部 dirty 状态。
- 新增 `wing_gui_set_input_reader()` 和 `wing_gui_poll_input()`，为后续接入 NuttX touch / mouse / keyboard 预留统一输入源接口。
- `wing_gui_handle()` 已在 tick 后、step 前自动 poll input provider，使 provider 输入和已有 input queue 输入进入同一套事件分发。
- `wing_gui_demo` 已注册 demo input provider，并验证 provider 拉取的 key event 可以驱动 scrollbar value changed。
- 新增 `wing_theme_t` 和 `wing_theme_init_default()`，将 root/header/panel/widget/state style 的默认视觉资源集中到主题对象。
- 新增 `wing_theme_init_high_contrast()`，提供高对比主题 seed。
- 新增 `wing_gui_set_theme()` / `wing_gui_get_theme()`，让 GUI runtime 持有当前主题指针。
- `wing_gui_demo` 已改为使用默认主题初始化主要组件样式，不再散落手写颜色 helper。
- `wing_gui_demo` 已通过 WING one-shot timer 在运行时切换到 high contrast 主题，并触发 widget tree invalidation。
- 新增 `wing_value_model_t`，作为数值型 widget 的统一 range / value / step 存储模型。
- progress / slider / scrollbar 已接入 `wing_value_model_t`，外部 API 保持 `wing_progress_get_value()` / `wing_slider_set_value()` / `wing_scrollbar_set_step()` 等形式不变。
- `wing_gui_demo` 已增加 value model 能力日志，明确 progress / slider / scrollbar 共用同一套数值模型路径。
- 新增 `wing_widget_update_value()` / `wing_widget_update_bool()`，将 progress / slider / scrollbar / switch / checkbox 的 value 更新、invalidation 和 value changed event 派发收敛到 widget base。
- `wing_gui_demo` 已增加 widget base value dispatch 日志，并通过 numeric 与 boolean widget 的 value changed 输出验证行为保持不变。
- 新增 dirty rect list，GUI runtime 会保留多个 invalidated rect，同时继续维护 union dirty 兼容现有 render 路径。
- 新增 `wing_gui_get_dirty_rect_count()` / `wing_gui_get_dirty_rect_at()`，用于调试和后续 dirty-list redraw。
- `wing_gui_frame_t` 已增加 dirty count 诊断字段，demo 可以展示 before handler / after tick / after handler 的 dirty list count。
- `wing_gui_demo` 已增加 dirty list count 日志，验证 render 后 dirty list 会被清空。
- 新增 dirty rect merge policy：新 dirty rect 与已有 rect 相交或边缘相接时合并，dirty list 满时退化为当前 union dirty。
- 新增 `wing_gui_get_dirty_merge_count()` 和 `wing_gui_frame_t` merge count 诊断字段。
- `wing_gui_demo` 已增加 dirty merge count 日志，验证首帧、theme switch 和 animation 局部 invalidation 都会触发合并。
- FRender software backend 的 `FR_CMD_CLEAR` 已改为遵守当前 clip stack，避免 dirty-list redraw 时局部 clear 误清全屏。
- `wing_gui_step()` 已从单 union dirty redraw 升级为 dirty-list chunk redraw：一次 begin/end，中间按 dirty rect 设置 clip、执行 render callback、遍历 object tree、再恢复 clip。
- WING render context 的 clip stack 已支持嵌套 push/pop，`wing_gui_set_clip()` 会与当前 clip 相交，`wing_gui_reset_clip()` 会恢复上一层 clip；dirty chunk clip 和 object clip 可以叠加。
- 新增 object clip-children seed：`wing_obj_set_clip_children()` / `wing_obj_get_clip_children()` 让父对象可以把子树绘制裁剪到自身 screen bounds，`wing_gui_demo` 已加入一个溢出 child 的 clip panel 验证场景。
- `wing_gui_frame_t` 新增 `redraw_count`，用于记录当前 frame 实际执行的 redraw chunk 数。
- `wing_gui_demo` 已增加 redraw chunks 日志，可区分“tick 后已有 dirty list 数量”和“step 内 layout/input/event 后实际重绘分块数量”。
- `wing_gui_demo` 增加 dirty rect 日志，展示 before handler / after tick before step / after handler。
- `wing_gui_demo` 在无 framebuffer presenter 时固定运行 5 帧，保证自动验证可以覆盖 timer、animation、theme switch 和后续局部重绘。
- 已验证首帧 full dirty，theme switch 触发 full dirty，后续 animation 触发局部 dirty，`wing_gui_step()` 渲染后 dirty 清空。
- 已验证 132ms 时 width animation 完成，并输出 `width animation completed by WING runtime`。
- 已在保留 `DISPLAY=:0` 的环境下重新构建并从 NSH 执行 `wing_gui_demo`，确认进入 framebuffer / X11 路径，日志输出 `framebuffer 640x480 fmt=13 bpp=32 stride=2560`。
- `wing_gui_demo` 已优化 framebuffer / X11 等待关闭路径：每帧开头也轮询窗口关闭状态；无 dirty 的 idle frame 只输出一次等待窗口关闭提示，不再无限刷 dirty 诊断日志。
- 已通过脚本化 X11 `WM_DELETE_WINDOW` 验证模拟窗口管理器点击关闭按钮，`wing_gui_demo` 可打印 `framebuffer window closed`、退出 demo 并返回 NSH。
- 已重新构建并实际从 NSH 执行 `wing_gui_demo` 验证 X11 framebuffer 路径：直接运行 `./FeatherCore/build/sim-wing` 时会打开 X11 窗口；`env -u DISPLAY` 路径是 headless 验证，不会打开窗口。
- 已验证 slider / scrollbar 的 pointer capture 第一阶段行为：pointer down 后，move/up 即使移动到 widget bounds 之外，仍由 pressed object 接收并完成 drag lifecycle。
- 新增正式 pointer capture API 和 runtime 状态 `captured_obj`，pointer down 默认建立 capture，pointer move/up 优先派发给 capture target，pointer up 后释放 capture。
- `wing_gui_demo` 已输出 slider / scrollbar 的 pointer captured / released 日志，验证 capture lifecycle event 已进入 WING event queue。
- 新增内部 `wing_widget_handle_pointer_lifecycle()`，switch / checkbox 已改用该 helper 统一处理 pointer down / up / cancel。
- slider / scrollbar 已补充 `WING_EVENT_POINTER_CANCELLED` 处理，capture 被抢占时会清理 pressed state 并停止传播。
- `wing_gui_demo` 已新增 capture cancel 脚本：slider pointer down 后由 scrollbar pointer down 抢占 capture，验证 slider 收到 pointer cancelled，scrollbar 随后 captured / released。
- 新增 `WING_LAYOUT_CENTER`，`wing_obj_layout_tree()` 可以把可见子对象居中放置在父对象 bounds 内。
- `wing_gui_demo` 已将 `GO` label 从 root 子对象改为 button 子对象，并由 button 的 center layout 自动定位。
- 已重新构建并从 NSH 执行 headless 与 X11 两条 `wing_gui_demo` 验证路径，确认 center layout 不影响输入、事件、动画、定时器、dirty redraw 和 framebuffer 关闭回到 NSH。
- 已重新构建并从 NSH 执行 headless 与 X11 两条 `wing_gui_demo` 验证路径，确认默认空间路径下同 z-index projected depth sorting 可以让前景 card 优先命中，X11 `WM_DELETE_WINDOW` 仍通过 `WING_EVENT_CLOSE_REQUEST` 退出并回到 NSH。
- 新增 `wing_widget_state_style_t`，box / button / slider / scrollbar 已迁移到统一 state style 容器。
- demo 的运行时 theme 切换已改为调用 `wing_button_set_state_style()` / `wing_slider_set_state_style()` / `wing_scrollbar_set_state_style()`，不再直接修改 widget 内部状态样式字段。
- `wing_gui_demo` 已新增能力日志：`widget base owns state style storage and state-driven style selection`。
- `wing_gui_demo` 已新增 disabled timer：输入脚本完成后由 WING timer 清除 button 的 enabled flag，并打印 active disabled style，验证 flag / state / style selection 联动。
- `wing_gui_demo` 已改用 `wing_obj_set_enabled(false)` 禁用 label 输入与运行时禁用 button，不再直接操作 enabled flag。

## 4. 最新验证记录

构建命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
```

结果：通过。

执行命令：

```sh
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

结果：通过。

说明：无 `DISPLAY` 下 `ERROR: fb_register() failed: -19` 是预期行为，表示当前环境没有 X11 framebuffer presenter，但 demo 会进入 headless 多帧验证路径。

关键输出：

```text
wing_gui_demo: input provider registered events=1 source=demo-script
wing_gui_demo: theme switch timer scheduled period=66ms target=high_contrast
wing_gui_demo: timer scheduled period=33ms repeat=0 target_progress=90
wing_gui_demo: animation scheduled property=line_width from=64 to=118 duration=132ms
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: app task entered WING GUI loop
wing_gui_demo: headless validation runs 5 frames before exit
wing_gui_demo: progress/slider/scrollbar share wing_value_model_t for range/value/step handling
wing_gui_demo: widget base dispatches value updates, invalidation and value changed events for numeric and boolean widgets
wing_gui_demo: widget base owns state style storage and state-driven style selection
wing_gui_demo: wing_obj_set_enabled drives enabled flag, disabled state and disabled style selection
wing_gui_demo: dirty system tracks union rect, dirty rect list count, merge count and redraw chunks
wing_gui_demo: pointer capture keeps slider/scrollbar drag active after pointer leaves widget bounds
wing_gui_demo: button uses center layout to place GO label child
wing_gui_demo: label widget text=GO uses rectangle glyphs and does not capture input
wing_gui_demo: progress timer updated wing_progress value=90
wing_gui_demo: input provider emitted index=1 type=key_down point=0,0 key=1001
wing_gui_demo: input queued label=slider pointer drag outside capture type=pointer_move point=340,196 key=0
wing_gui_demo: wing_slider pointer captured by WING runtime
wing_gui_demo: wing_slider value changed from 7 to 100 by value input
wing_gui_demo: wing_slider pointer interaction completed
wing_gui_demo: wing_slider pointer released by WING runtime
wing_gui_demo: input queued label=scrollbar pointer drag outside capture type=pointer_move point=340,226 key=0
wing_gui_demo: wing_scrollbar pointer captured by WING runtime
wing_gui_demo: wing_scrollbar value changed from 0 to 100 by value input
wing_gui_demo: wing_scrollbar pointer interaction completed
wing_gui_demo: wing_scrollbar pointer released by WING runtime
wing_gui_demo: input queued label=slider pointer down before capture cancel type=pointer_down point=44,196 key=0
wing_gui_demo: input queued label=scrollbar pointer down cancels slider capture type=pointer_down point=40,226 key=0
wing_gui_demo: widget pointer lifecycle helper clears pressed state on pointer up/cancel
wing_gui_demo: wing_slider pointer capture cancelled by WING runtime
wing_gui_demo: handler input polled=1
wing_gui_demo: dirty after handler none
wing_gui_demo: redraw chunks this frame count=1
wing_gui_demo: theme timer switched active theme to high_contrast and invalidated widget tree
wing_gui_demo: dirty after tick before step x=0 y=0 w=320 h=240
wing_gui_demo: dirty list after tick before step count=1
wing_gui_demo: dirty merge after tick before step count=5
wing_gui_demo: dirty list after handler count=0
wing_gui_demo: redraw chunks this frame count=1
wing_gui_demo: frame tick=66 commands=58 checksum=0x527335c5
wing_gui_demo: dirty after tick before step x=124 y=70 w=104 h=14
wing_gui_demo: dirty list after tick before step count=1
wing_gui_demo: dirty merge after tick before step count=1
wing_gui_demo: redraw chunks this frame count=2
wing_gui_demo: frame tick=99 commands=12 checksum=0xf926d5c5
wing_gui_demo: width animation completed by WING runtime
wing_gui_demo: redraw chunks this frame count=0
wing_gui_demo: app task exit
```

这次验证证明：

- `wing_gui_demo` 不是一次性绘制 demo，而是通过 WING GUI handler 大循环推进。
- 输入 provider 进入了 WING GUI runtime，而不是 demo 手动调用 widget。
- timer 在 WING GUI runtime 中触发 progress 更新和 theme switch。
- animation 在 WING GUI runtime 中按 frame 推进。
- theme switch 会通过 invalidation 触发重绘。
- progress / slider / scrollbar 保持原有事件输出，但内部已经走统一 value model。
- progress / slider / scrollbar / switch / checkbox 的 value changed 事件仍正常输出，但 value update / invalidate / event dispatch 已收敛到 widget base。
- dirty list 会记录并合并 invalidated rect，并在 render 后被清理。
- dirty-list redraw 已经进入 `wing_gui_step()` 的真实绘制路径，不再只是诊断统计。
- `redraw_count=0` 表示当前 frame 没有 dirty，不需要提交 command list。
- headless 环境也能完成自动验证。

X11 / framebuffer 路径补充验证：

```sh
./FeatherCore/build/sim-wing
```

在 NSH 中执行：

```text
wing_gui_demo
```

然后从宿主侧向标题为 `NuttX` 的 X11 窗口发送 `WM_DELETE_WINDOW` ClientMessage，模拟窗口管理器点击关闭按钮。

结果：进入 framebuffer / X11 路径，收到窗口关闭事件后退出 demo 并返回 NSH。

注意：如果执行的是下面的 headless 命令，则不会打开 X11 窗口，这是预期行为：

```sh
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

需要真实窗口时，应保留 `DISPLAY`，直接启动 sim，再在 NSH 中输入 `wing_gui_demo`：

```sh
./FeatherCore/build/sim-wing
```

关键输出：

```text
wing_gui_demo: framebuffer 640x480 fmt=13 bpp=32 stride=2560
wing_gui_demo: close the framebuffer window to exit demo
wing_gui_demo: idle frame no redraw; waiting for framebuffer window close
wing_gui_demo: framebuffer window closed
wing_gui_demo: app task exit
nsh>
```

这次验证证明：

- 当前环境 `DISPLAY=:0` 时，`wing_gui_demo` 可以从 NSH 进入 framebuffer / X11 present 路径。
- demo 在无 dirty 的 idle 状态不会继续刷屏，而是等待 framebuffer 窗口关闭。
- 该结果不同于 `env -u DISPLAY` 的 headless 验证。
- `fr_fb_presenter_window_closed()` 可以捕获 X11 `WM_DELETE_WINDOW`，并让 demo 关闭 framebuffer presenter、退出应用任务、返回 NSH。

## 5. 下一步计划

### P0：稳定 WING GUI handler 模型

目标：

- `wing_gui_t` 负责维护 GUI runtime 状态。
- 应用只负责创建 GUI、注册输入源、周期性调用 `wing_gui_handle()`。
- handler 内部推进 timer、animation、input、event、layout、dirty、render。

当前状态：基本完成第一版。

下一步：

- 把真实 NuttX input provider 接入 `wing_gui_set_input_reader()`。
- 明确 handler 返回值和错误码语义。
- 增加 frame pacing / idle 策略。

### P0：补齐 widget base

目标：

- 抽象所有组件共享的 value、style、state、event、draw helper。
- 降低 button / slider / scrollbar / switch / checkbox 的重复代码。

下一步：

- 新增 `wing_widget_t` 或等价 widget base。
- 抽象 state style apply。
- 继续补充 capture owner 变更、device lost、multi-pointer 等高级场景。

当前状态：

- `wing_value_model_t` 第一阶段已落地。
- progress / slider / scrollbar 已接入统一 value model。
- `wing_widget_update_value()` / `wing_widget_update_bool()` 已落地。
- progress / slider / scrollbar / switch / checkbox 已通过 widget base 统一执行 value 更新、invalidation 和 value changed event 派发。
- box / button / slider / scrollbar 已通过 widget base 统一执行 state style 存储和状态选择。
- object enabled flag 已通过 `wing_obj_set_flags()` 与 disabled state/style 选择联动。
- 应用层可通过 `wing_obj_set_enabled()` / `wing_obj_is_enabled()` 使用 enabled 状态，而不是直接修改 flags。
- pointer capture 第一版已从 `pressed_obj` 隐式路径升级为公开 API 和 `captured_obj` runtime 状态。
- slider / scrollbar 已通过越界拖拽验证正式 pointer capture lifecycle。
- `wing_widget_handle_pointer_lifecycle()` 已落地，作为 TouchGFX behavior / mixin 思路在 WING 内部的第一步。
- `wing_widget_state_style_t` 已落地，作为 LVGL 状态化 style 思路在 WING widget base 内部的第一步。
- switch / checkbox 已通过 pointer lifecycle helper 统一处理 pressed state。
- slider / scrollbar 已支持 pointer cancel 清理 pressed state。
- button disabled style 已通过 runtime timer 验证，后续可继续补充更系统的 disabled input guard 和公开 enable/disable API。
- 仍未形成公开的 `wing_widget_t` 继承结构，后续继续收敛 draw helper、更完整的 drag lifecycle 和高级 capture 场景。

### P0：升级 dirty system

目标：

- 从单矩形 union 升级到 dirty region list。
- 为局部重绘、tile redraw、未来硬件加速 planner 打基础。

下一步：

- 增加 tile redraw policy，避免 dirty rect 过碎时 command 数量失控。
- 增加 command capacity fallback，dirty list 过多时退化为 union redraw。
- 增加 redraw cost threshold，为后续 software / DMA2D / GPU2D planner 提供策略输入。
- 继续把 dirty-list present 扩展为 tile present 策略，让 framebuffer / NX / 未来硬件路径可以按 tile/cost policy 提交必要区域。

当前状态：

- dirty rect list 第一阶段已落地。
- dirty rect merge policy 第一阶段已落地。
- `wing_gui_get_dirty_rect_count()` / `wing_gui_get_dirty_rect_at()` 已可查询 dirty list。
- `wing_gui_get_dirty_merge_count()` 已可查询当前 frame 的 dirty merge count。
- `wing_gui_frame_t` 已新增本帧 `present_rect` / `present_rects[]` / `present_rect_count` / `has_present_rect`，用于把 render 前的 union dirty rect 和 dirty-list rects 传给 presenter；`wing_gui_demo` 的 framebuffer present 已从 full-screen present 改为逐个 `fr_fb_presenter_present_rect()` + `FBIO_UPDATE` dirty-list 路径。
- `wing_obj_invalidate()`、`wing_obj_set_bounds()`、`wing_obj_set_space_transform()`、dirty culling 和 hit-test 已开始使用 `wing_obj_get_screen_bounds()`，使默认空间 transform 不再只影响绘制，也会影响第一阶段 redraw bounds。
- `wing_obj_set_bounds()` 的旧/新 redraw bounds 已统一走 object screen bounds 路径，旧 bounds invalidation 会保留 parent chain world space transform 语义，避免父对象带空间变换时只按本地 transform 估算旧区域。
- `wing_obj_get_screen_bounds()` 已优先使用 object 绑定的 `wing_gui_t` runtime camera；未绑定 GUI 时仍保留本地 fallback camera。
- `wing_obj_set_space_transform()` 已按子树触发旧/新 screen bounds invalidation，父对象 transform 改变时不会只重绘父对象自身。
- FRender `FR_CMD_CLEAR` 已支持 clip-aware clear。
- `wing_gui_step()` 已按 dirty list 执行 chunk redraw。
- `wing_gui_demo` 已输出 dirty list count、merge count 和 redraw chunk count。
- tile redraw / planner / cost policy 还没有完成。

### P1：真实输入接入

目标：

- 不再只依赖 synthetic demo input。
- 让 WING GUI 可以从 NuttX touch / mouse / keyboard 获取输入。

下一步：

- 实现 NuttX pointer provider。
- 实现 keyboard provider。
- 建立 key map。
- 明确 input device open/close 生命周期。

### P1：完善 theme / style

目标：

- 从 theme seed 演进到可继承、可覆盖、可切换的 style system。

下一步：

- 明确 object local style 与 theme style 的优先级。
- 支持 inherited style。
- 支持 state selector。
- 支持运行时主题切换的统一刷新入口。

### P1：完善 text / font

目标：

- `wing_label_t` 从 builtin bitmap font seed 继续升级为真实文本组件。

下一步：

- 扩展字体资源接口，支持转换工具产出的 glyph atlas / bitmap font。
- 文本测量。
- 扩展文本能力到更完整的 word wrap 策略、截断策略、光标移动和后续 text input。
- label 对齐与裁剪。

### P2：WING Desktop 重建

目标：

- 在 WING GUI 稳定后，再实现 WING Desktop。

下一步：

- desktop shell。
- app package / launcher。
- window manager。
- task bar / status area。
- desktop demo。

## 6. 每次迭代必须执行的验证

涉及 WING GUI / FRender / sim 配置的代码修改后，必须执行：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
```

然后执行：

```sh
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

这条命令是 headless 验证，故意移除 `DISPLAY`，不会打开 X11 窗口。无窗口环境下出现 `ERROR: fb_register() failed: -19` 属于预期。

涉及 framebuffer / X11 / 关闭窗口逻辑时，还必须执行真实 X11 验证：

```sh
./FeatherCore/build/sim-wing
```

在 NSH 中执行：

```text
wing_gui_demo
```

需要观察：

- 日志出现 `framebuffer 640x480 fmt=13 bpp=32 stride=2560` 或等价 framebuffer 信息。
- 日志出现 `close the framebuffer window to exit demo`。
- 点击 X11 窗口关闭按钮，或发送 `WM_DELETE_WINDOW` 后，日志出现 `framebuffer window closed`。
- demo 输出 `app task exit` 并返回 `nsh>`。

需要观察：

- `wing_gui_demo` 能从 NSH 启动。
- headless 环境下可以跑满多帧并退出。
- input provider 有输入输出。
- pointer / keyboard / focus / bubbling / stop propagation 正常。
- timer 正常触发。
- animation 正常完成。
- theme switch 正常触发 invalidation。
- dirty rect 在 render 后清空。
- demo 最终输出 `app task exit`。

## 7. 当前判断

当前 WING GUI 已经可以继续向上实现第一版 GUI 能力。

但它还不应该立刻进入 WING Desktop。下一阶段更应该继续补强：

- widget base。
- dirty region list。
- 真实输入 provider。
- theme / style 系统。
- text / font。

等这些稳定后，再开始 WING Desktop，会比现在直接堆桌面更干净，也更不容易再次混乱。

- 新增 `include/wing/widgets/wing_widgets.h`，组件公开 API 开始从 `<wing/wing.h>` 拆出，`wing.h` 保持兼容聚合入口。

- slider/scrollbar 的视觉尺寸参数已从实现硬编码改为可配置，并由 `wing_gui_demo` 显式设计。
- 2026-06-12：设计参考文档中的 optional 3D profile 表述已收敛为默认 object space 表述；第一阶段可以不实现完整 3D 游戏引擎，但 3D UI 不是后期外挂，2D 是默认空间下的 identity transform 特例。
- 2026-06-12：新增 `wing_project_rect_quad()` core space API，将 2D widget bounds 正式建模为默认 3D 空间中的平面 quad；`wing_obj_project_quad_from_bounds()` 改为复用该入口，避免 object 私有实现继续承载空间投影语义。
- 2026-06-12：新增 `wing_quad2d_get_bounds()` / `wing_project_rect_bounds()` core space API，将 projected quad 到保守 screen dirty bounds 的计算从 object 私有逻辑提升到默认空间层。
- 2026-06-12：新增 `wing_space_transform_apply_point()` core space API，将 transform apply 与 camera projection 解耦，为后续 mesh vertex transform / shader path / picking 复用同一套默认空间点变换入口打底。
- 2026-06-12：新增 `wing_project_point_with_depth()` core space API，并在 `wing_gui_demo` 输出 space card center 的 screen point / depth，开始把 depth 作为默认空间能力的一等验证项。
- 2026-06-12：新增 `wing_project_rect_projected_quad()` core space API，并在 `wing_gui_demo` 输出 space card 四角 projected depth，为后续 mesh / shader / GPU path 的顶点数据模型打底。
- 2026-06-12：新增 `wing_obj_project_quad()` object API，并让 `wing_gui_demo` 通过 object 层验证 space card 四角 depth，避免 widget/render 后续各自重复组合 bounds/world transform/camera。
- 2026-06-12：新增 `wing_projected_quad_average_depth()` / `wing_obj_get_projected_depth()`，并在 `wing_gui_demo` 输出 object projected average depth，为后续空间排序和 picking 提供统一 depth 代表值。

## 2026-06-12 更新：Text Input 键盘语义收紧与验证

- `wing_text_input_t` 已收紧键盘输入边界：只把可打印 ASCII 字符交给 `wing_text_edit_t` 插入，方向键 / Backspace / Delete 继续作为编辑键，Enter 等非文本控制键不会污染固定单行 buffer。
- `wing_gui_demo` 已增加 text input key-down 日志和 synthetic Enter 控制键验证：脚本输入 `A` / `B` / Left / Backspace / `C` 后文本为 `CB`，随后 Enter 只产生 key-down 日志，文本保持 `CB`。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证：X11 framebuffer 窗口打开，timer / animation / dirty list / redraw chunk / present rect / lifecycle / close request 路径均正常。
- 已通过 X11 `WM_DELETE_WINDOW` 验证关闭窗口路径：窗口关闭事件进入 WING input/event queue，root 收到 `WING_EVENT_CLOSE_REQUEST`，demo 退出并回到 `nsh>`。
- 本轮环境缺少 `xdotool`，所以没有脚本化注入真实鼠标移动；真实 X11 close request 路径已验证，后续若需要自动化鼠标/键盘事件验证，可补一个轻量 X11 输入注入工具或在 FRender presenter 增加测试输入钩子。

## 2026-06-12 更新：项目内 X11 输入自动化工具

- 新增 `FeatherCore/tools/firmware/sim/x11-input.c` 和 `x11-input.sh`，用于自动查找 `NuttX` X11 窗口并发送 `move` / `click` / `key` / `close` 动作，减少对 `xdotool` 等外部工具的依赖。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证 X11 输出路径。
- 已使用 `x11-input.sh NuttX move ...` 验证真实 X11 pointer move 能被 simulator framebuffer 捕获，日志出现 `x11 input provider emitted type=pointer_move source=mouse`，随后进入 WING input queue / handler / dirty-present 观测路径。
- 已使用 `x11-input.sh NuttX close` 验证窗口关闭自动化路径，root 收到 `WING_EVENT_CLOSE_REQUEST`，demo 正常退出并回到 `nsh>`。
- 当前环境缺少 XTest 头/库，`click` / `key` 动作仍只能通过 XSendEvent best-effort 发送，尚未形成真实 X11 点击和键盘输入的强验证；后续需要补 XTest 支持或在 simulator framebuffer / FRender presenter 增加等效测试输入后端。

## 2026-06-12 更新：X11 输入工具可选 XTest 支持

- `FeatherCore/tools/firmware/sim/x11-input.c` 已增加可选 XTest 支持：有 `X11/extensions/XTest.h` 与 `libXtst` 时，`click` 和 `key` 会使用 XTest fake input，避免普通 XSendEvent 被 simulator / X server 输入路径过滤。
- `x11-input.sh` 会自动检测 XTest 头/库：存在时以 `-DHAVE_XTEST -lXtst` 编译，不存在时退回普通 Xlib 模式。
- 当前环境没有 XTest，因此本轮真实 X11 强验证仍覆盖 pointer move 与 close request；click/key 的真实 X11 强验证仍待安装 XTest 或补 simulator/FRender presenter 测试输入后端。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证：真实 X11 pointer move 进入 `x11 input provider emitted type=pointer_move source=mouse`；窗口 close 进入 `WING_EVENT_CLOSE_REQUEST`，demo 退出回到 `nsh>`。

## 2026-06-12 更新：X11 Client Window 输入路径验证通过

- `x11-input.c` 已改为优先选择最深层匹配窗口，避免把 pointer / button / key 事件发给窗口管理器装饰层。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo`。
- 真实 X11 pointer move 自动化验证通过：`x11-input.sh NuttX move 260 132` 进入 WING input provider 后日志为 `type=pointer_move source=mouse point=260,132`。
- 真实 X11 click 自动化验证通过：`x11-input.sh NuttX click 50 86 1` 产生 `pointer_down` / `pointer_up`，并触发 WING click bubbling，日志出现 `card received bubbled click and stopped propagation`。
- 真实 X11 keyboard 自动化验证通过：`x11-input.sh NuttX key Right` 产生 `type=key_down source=keyboard key=1001`。
- 真实 X11 close 自动化验证通过：`x11-input.sh NuttX close` 触发 framebuffer close、WING `WING_EVENT_CLOSE_REQUEST`、`app task exit`，最终回到 `nsh>`。
- 当前环境仍缺少 XTest，但 client window 命中修正后，普通 Xlib/XSendEvent 模式已经能覆盖本阶段 move / click / key / close 验证；XTest 仍作为后续更强真实输入注入路径保留。

## 2026-06-12 更新：Card 绘制路径继续收敛为默认 Object Space

- `wing_card_t` 继续保留为普通 widget：它不代表独立 `wing_3d_view` 或独立 3D 模块，只是当前 demo 用来展示默认 object space 的一个组件。
- `wing_card` 的 edge stroke 已从旧的轴对齐 rect fallback 改为 `wing_gui_draw_quad()`，直接提交 projected quad stroke。
- `wing_card` 的 edge fill 已从 `wing_gui_fill_rect()` 改为 `wing_gui_fill_quad()`，通过 projected ridge quad 表达薄边缘效果。
- 这让 card 的 front / back / edge 绘制全部走 object projection + WING render API + FRender command path，和普通非 identity widget style 的 quad fill/stroke 路径保持一致。
- 已重新构建 `sim-wing`，并从 NSH 执行 `wing_gui_demo` 验证默认空间、动画、timer、dirty redraw、present rect、projected hit-test、click bubbling 均正常。
- 已使用 `x11-input.sh` 再次验证真实 X11 move / click / key / close：输入进入 WING input provider，click 触发 bubbling，close 触发 WING close request 并回到 `nsh>`。

## 2026-06-12 更新：Label Glyph 进入默认 Object Space 绘制

- 新增 `wing_widget_fill_rect_for_obj()`，把“普通 rect 在 object space transform 下投影为 quad，否则保持 rect 快路径”的逻辑收敛为共享 helper。
- `wing_widget_draw_style_for_obj()` 的 fill 路径开始复用该 helper，减少普通 widget style 和其他 widget 绘制路径之间的投影逻辑分叉。
- `wing_label` 的 bitmap glyph pixel 绘制已改为 `wing_widget_fill_rect_for_obj()`，因此 label 自身或父对象存在非 identity object space transform 时，文本像素会通过 `wing_gui_fill_quad()` 进入 FRender command path。
- `wing_gui_demo` 的 wrap label 已设置轻量默认空间变换：`rotation_y=10`、`z=6`，用于验证文本不再只是 axis-aligned 2D 特例。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证：日志出现 `wrap label uses default object space rotation_y=10 z=6 and bitmap glyph pixels render through shared projected quad fill helper`，同时 timer、animation、dirty redraw、present、event、projected hit-test 均正常。
- 已使用 `x11-input.sh` 验证真实 X11 move / click / key / close：输入进入 WING input provider，click 触发 bubbling，close 触发 WING close request 并回到 `nsh>`。

## 2026-06-12 进度：默认 3D/object space 收敛完成一次验证

本次确认并验证：WING GUI 的 3D 能力不是拓展模块，而是 core object space 的默认能力。当前工作树不依赖独立 `wing_3d.c` 或 `wing_3d_view.c`，demo 中的空间 triangle 通过普通 object draw callback 使用 runtime camera 和 `wing_project_triangle()` 投影，再提交到 WING render frontend / FRender `fill_triangle`。

已完成验证：

- `sim-wing` 构建成功。
- NSH 中执行 `wing_gui_demo` 成功打开 framebuffer/X11 窗口。
- 日志确认 triangle primitive 路径：runtime camera + core space projection -> WING render frontend -> FRender fill_triangle command。
- 合成输入、timer、animation、dirty redraw、framebuffer present 均正常。
- 真实 X11 move / click / key Right 输入进入 WING runtime。
- X11 close 后 demo 收到 close request，退出并回到 NSH。

下一步保持方向：

- 所有空间 UI 能力继续放在 `wing_space.c`、object transform、camera、dirty、render frontend 中。
- 普通 widget 默认就是 object-space widget，identity transform 即 2D 状态。
- 后续补 mesh / shader / GPU3D 时，不修改 WING app 的基本 object/widget 模型，只扩展 FRender command 和 backend 执行能力。

## 2026-06-12 进度：triangle dirty bounds 进入默认 object space

本次新增 WING core space triangle bounds helper，让 triangle primitive 不只是能绘制，也能拥有统一的 conservative dirty bounds 计算入口。

代码进展：

- 新增 `wing_triangle2d_get_bounds()`。
- 新增 `wing_projected_triangle_get_bounds()`。
- 新增 `wing_project_triangle_bounds()`。
- `wing_gui_demo` 的 triangle custom object 抽出共享 projection helper，draw path 和日志共用同一套 runtime camera / core space projection。
- 修正 demo triangle 的 object-space translation：根据 depth / focal length 从目标屏幕中心反投影，避免投影 bounds 跑出预期屏幕区域。

验证结果：

- `sim-wing` 构建成功。
- NSH 执行 `wing_gui_demo` 成功打开 X11/framebuffer 窗口。
- projected triangle dirty bounds 输出为 `x=296 y=71 w=78 h=90`，深度输出为 `270/274/266`。
- demo 的输入、事件、timer、animation、dirty redraw、present 和 close request 路径通过。
- 使用 X11 脚本发送 move、click、Right key、close，demo 正常处理并回到 NSH。

下一步建议：

- 将 triangle/projected geometry 的 bounds helper 继续用于后续 mesh dirty planning。
- 后续如果引入 triangle hit-test、mesh batch 或 shader path，应复用 core space 的 projected vertex / projected bounds 数据结构。

## 2026-06-12 进度：object screen bounds callback 接入 dirty 模型

本次新增 object-level screen bounds callback seed，让 custom primitive 可以把自己的投影后脏区交给 WING object tree，而不是只由 demo 打印 bounds。

代码进展：

- 新增 `wing_obj_screen_bounds_fn_t`。
- 新增 `wing_obj_set_screen_bounds_cb()`。
- `wing_obj_t` 新增 `screen_bounds` callback 字段。
- `wing_obj_get_screen_bounds()` 优先使用 custom callback，失败时回退到默认 rect projection。
- `wing_gui_demo` 的 triangle object 注册 `wing_gui_demo_triangle_screen_bounds()`，复用 runtime camera + core space projection + projected triangle bounds。

验证结果：

- `sim-wing` 构建成功。
- NSH 执行 `wing_gui_demo` 成功打开 X11/framebuffer 窗口。
- 日志确认 custom object screen bounds callback 输出 projected triangle dirty bounds。
- demo 的输入、事件、timer、animation、dirty redraw、present 和 close request 路径通过。
- X11 脚本发送 move、click、Right key、close，demo 正常处理并回到 NSH。

下一步建议：

- 给 custom primitive 补充 contains/hit-test callback seed，避免非矩形 primitive 只用 conservative bounds 命中。
- 后续 mesh / shader / vector path 都应使用同一个 screen bounds callback 或更高层 render-node bounds 接口。

## 2026-06-12 进度记录：默认 3D 几何对象的 bounds / hit-test 验证

本次实现重点是让 WING 的对象模型继续靠近默认 3D GUI，而不是把 3D 当作独立扩展。

完成事项：

- 新增 object 自定义 screen bounds 回调，让 projected triangle 这类非矩形对象可以向 dirty 系统报告自己的保守屏幕 bounds。
- 新增 object 自定义 contains point 回调，让 projected triangle 可以绕过普通矩形 hit-test，使用投影后的几何形状进行精确命中。
- 新增 triangle projected geometry contains API，作为未来 custom primitive / mesh hit-test 的最小种子。
- `wing_gui_demo` 已接入自定义 projected triangle dirty bounds 与 hit-test，并打印验证日志。
- `x11-input` 工具新增 close target 修正与 `frameclose` 动作，用于后续更接近真实用户点击窗口 X 的验证。

验证记录：

- `sim-wing` 已重新构建成功。
- 从 NSH 执行 `wing_gui_demo` 后，demo 打开窗口并进入 app loop。
- 日志覆盖 WING GUI 的 timer、animation、event、dirty redraw、present、widget state、input queue、projected object space、triangle primitive 等路径。
- 本轮自动化 close 验证未完全通过：宿主环境缺少 XTest / xdotool / wmctrl，合成点击窗口管理器 frame 没有触发窗口关闭。该项需要后续补齐验证工具或手动验证后再标记为通过。

风险与跟进：

- 目前 WING 运行逻辑本身没有因为本次改动出现构建或 demo 启动问题。
- 自动化关闭验证工具仍不够可靠；后续应优先补齐 XTest 支持，或实现不依赖窗口管理器 decoration 的 sim close 控制路径。
- 继续保持 3D 为默认核心语义，避免出现 `wing_3d_view` 这类把 3D 割裂成特殊控件的设计。

## 2026-06-12 进度记录：close 自动化验证通过

本次重点解决 WING GUI demo 的 X11 close 验证问题。上一轮发现 close 自动化不稳定，并且 synthetic destroy 可能让 sim 在 `X_ShmPutImage` 阶段触发 `BadDrawable` 直接退出。

完成事项：

- 修正 `x11-input close`，不再默认发送 synthetic `DestroyNotify`。
- 新增 `x11-input destroy` 作为显式测试动作。
- 保留 `WM_DELETE_WINDOW` 与 `_NET_CLOSE_WINDOW` 两种标准关闭请求。
- 修复 NuttX sim framebuffer update：present 前检查 window closed，present 时捕获 X11 `BadDrawable` / `BadWindow`，转为 window closed 状态，而不是让 Xlib abort。

验证记录：

- 重新构建 `sim-wing` 成功。
- 从 NSH 执行 `wing_gui_demo` 成功。
- demo 覆盖默认 3D core space、camera、projected bounds、projected hit-test、custom triangle primitive、FRender backend、dirty/present、timer、repeat timer、animation、lifecycle、widgets、focus、pointer capture、input queue 等路径。
- X11 输入验证通过：真实宿主 pointer move/down/up、keyboard Right 都进入 WING input provider，并触发 click / dirty / present 相关日志。
- X11 close 验证通过：执行 `x11-input close` 后 demo 打印 `framebuffer window closed`、`root received close request through WING input/event queue`、`app task exit`，并回到 `nsh>`。

风险与跟进：

- `frameclose` 仍依赖 XTest 才能真正模拟点击窗口管理器 decoration；当前环境没有 XTest，所以主要自动化路径使用 `WM_DELETE_WINDOW` / `_NET_CLOSE_WINDOW`。
- `poweroff` 时出现 gcov profile checksum 提示，后续如需要干净日志，可清理旧 gcda 或调整 build 脚本处理 profile 文件。

## 2026-06-12 进度记录：WING object callback 初始化与 custom geometry 语义确认

本次小步修复聚焦 WING object 的底层稳定性和默认 3D 语义表达。

完成事项：

- `wing_obj_init()` 现在显式清空 `screen_bounds` 与 `contains_point` callback。
- 确认 custom projected triangle 已经通过普通 WING object 的 `draw`、`screen_bounds`、`contains_point` 三类 callback 接入 object tree。
- `wing_gui_demo` 增加日志，明确 custom geometry 是普通 WING object，不是独立 3D view。

验证记录：

- `sim-wing` 重新构建成功。
- 从 NSH 执行 `wing_gui_demo` 成功。
- timer、repeat timer、animation、lifecycle、dirty list、present rect、widget event、camera event、space transform event、projected triangle bounds/hit-test 均出现在日志中。
- 真实 X11 pointer / keyboard 输入进入 WING input provider。
- `x11-input close` 后 demo 清洁退出并返回 NSH。

设计结论：

- 当前方向正确：不需要 `wing_3d.c` / `wing_3d_view.c`。
- 3D 是 WING object 的默认空间语义；2D 是默认 camera 下的平面对象状态。
- 后续 mesh、material、shader 应该继续沿普通 object + render backend 的路径增强，而不是新建一个割裂的 3D 子系统。

## 2026-06-12 进度：默认 3D 架构确认与 demo 验证

本次更新确认 WING GUI 不再保留单独的 3D view 架构边界。3D 是 WING core 的默认空间模型，2D 是默认 camera + z=0 平面的特殊状态。自定义三角形 demo 继续作为普通 WING object 运行，通过 draw、screen-bounds、contains callbacks 接入对象树、脏区和渲染链路。

已完成：

- `wing_obj_set_screen_bounds_cb()` 在 GUI 已绑定时同时脏化旧/新 screen bounds。
- `wing_obj_set_user_data()` 在 user_data 影响投影/绘制时同时脏化旧/新 screen bounds。
- `wing_gui_demo` 增加自定义 geometry dirty 语义日志。
- `sim-wing` 已重新构建。
- 从 NSH 执行 `wing_gui_demo` 已验证：X11 窗口打开、真实 X11 pointer/key 输入进入 WING input provider、dirty/present 正常、窗口 close 转换为 WING close request，demo 退出并回到 NSH。

观察到的关键日志：

- `custom geometry is a normal WING object using draw/screen-bounds/hit-test callbacks, not a separate 3D view`
- `custom geometry screen-bounds/user-data updates dirty both old and new projected bounds`
- `x11 input provider emitted ...`
- `framebuffer window closed`
- `root received close request through WING input/event queue`
- `app task exit`

后续计划：

- 清理或折叠 `wing_3d.c`、`wing_3d_view.c` 这类独立 3D 文件边界。
- 将保留价值迁移到 core object-space、camera/projection、render frontend。
- 保持 WING GUI API 面向普通对象模型，避免 app 层感知 2D/3D 两套模型。

## 2026-06-12 进度：默认 2D 状态进入 core space API

本次继续收敛 WING GUI 的默认 3D/object space 架构。当前工作树已确认不存在 `wing_3d.c` / `wing_3d_view.c` 源文件或 Makefile 引用。新增 `wing_space_transform_is_default_2d()`，用于明确表达：2D 是默认空间中的 identity transform 状态，而不是旧兼容层或独立 3D view 的反面。

已完成：

- 新增 `wing_space_transform_is_default_2d()` prototype。
- 新增 `wing_space_transform_is_default_2d()` 实现，当前语义等价于 identity transform。
- 文档记录默认 2D plane / 默认 3D object space 的统一关系。

后续计划：

- 将普通 widget 的 projected quad 绘制继续统一到 core space + WING render frontend。
- 为后续 mesh / shader / GPU3D 保留 FRender command/backend 扩展点，避免 WING GUI 层拆出第二套空间模型。

验证结果：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过，输出 `sim-wing` 大小约 5285072 bytes。
- `./FeatherCore/build/sim-wing` 进入 NSH 后执行 `wing_gui_demo` 通过。
- demo 输出确认 timer、repeat timer、animation、lifecycle、dirty list、dirty present、render command fallback、object space projection、自定义 triangle、widgets 和 synthetic input 全部推进。
- 使用 `x11-input.sh` 注入真实 pointer move/click 和 keyboard Right，WING input provider 收到 `pointer_move`、`pointer_down/up`、`key_down`，并触发 widget focus/capture/value change/root click。
- 使用 `x11-input.sh NuttX close` 关闭窗口，demo 输出 `framebuffer window closed`、`root received close request through WING input/event queue`、`app task exit`，并回到 NSH。

## 2026-06-12 进度：对象级 default-2D 语义 API

本次在 core space transform API 之上新增 object 层语义入口 `wing_obj_is_default_2d()`。这样 WING 上层可以询问“这个对象是否处于默认 2D 平面状态”，而不是直接把 2D/3D 误解成两套 UI 模型。

已完成：

- `wing_object.h` 新增 `wing_obj_is_default_2d()`。
- `wing_object.c` 实现 `wing_obj_is_default_2d()`，内部复用 `wing_space_transform_is_default_2d()`。
- `wing_gui_demo` 增加日志：root object default 2D plane API reports root=yes in default object space。

设计含义：

- 2D 继续作为默认 object space 下的 identity plane 表达。
- 3D 不作为独立 view/widget 子系统出现。
- 后续普通 widget、space card、custom geometry、mesh/shader/GPU3D 都继续共享 object tree、camera/projection、dirty 和 FRender backend。

## 2026-06-12 进度：补充单次与循环动画 demo 覆盖

本次根据目标补充 `wing_gui_demo` 的动画验证覆盖。此前 demo 已有一次性宽度动画和空间 transform 动画，也有 repeat timer，但 repeat timer 不等价于循环动画。因此新增 `line_secondary` ping-pong 循环动画：done callback 在第一段完成后重启第二段，第二段完成后停止。

已完成：

- 新增 `wing_gui_demo_loop_width_anim_s`。
- 新增 `wing_gui_demo_loop_width_anim_apply()`。
- 新增 `wing_gui_demo_loop_width_anim_done()`，负责 segment 计数、重启下一段、完成后停止。
- demo 调度 `line_secondary_width` loop animation，日志输出 `loop animation scheduled`、每段 completed、最终 completed。
- 能力摘要日志增加 loop animation 说明。

验证重点：

- 单次动画：`line_primary` width ease_out。
- 空间动画：`space_card` rotation_y ease_in_out。
- 循环动画：`line_secondary` width ping-pong。
- 重复定时器：repeat timer 修改 line style 并 stop itself。

验证结果：

- 已重新执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过，输出 `sim-wing` 大小约 5295152 bytes。
- 已启动 `./FeatherCore/build/sim-wing`，从 NSH 执行 `wing_gui_demo`。
- demo synthetic input 覆盖 button、checkbox、switch、text input、scroll view、space card projected hit-test、same z-index depth hit-test、slider、scrollbar、pointer capture、keyboard 和 encoder 路径。
- demo timer 覆盖 one-shot progress timer、repeat timer、disable timer、lifecycle create/hide/destroy timer。
- demo animation 覆盖一次性 width animation、space transform animation、done-callback 驱动的 ping-pong loop animation。
- 已观察到 `loop animation scheduled property=line_secondary_width ...`、`loop width animation segment=1/2 completed direction=forward`、`loop width animation segment=2/2 completed direction=backward`、`loop width animation completed after 2 segments`。
- dirty/present 在单次动画、循环动画、timer、lifecycle 和输入后均有对应日志，包含 dirty list、merge count、redraw chunks、present rect list。
- 已通过 `x11-input.sh` 注入真实 pointer move/click 和 keyboard Right，WING input provider 收到 `pointer_move`、`pointer_down/up`、`key_down`，并触发 bubbled click。
- 已通过 `x11-input.sh NuttX close` 关闭窗口，demo 输出 `framebuffer window closed`、`root received close request through WING input/event queue`、`app task exit`，并回到 NSH。

## 2026-06-12 进度：progress 拖拽与输入事件覆盖

本次补齐 `wing_progress_t` 的交互能力，让 demo 不再只是通过 timer 更新 progress，而是可以通过模拟输入直接拖拽和调整 progress。

已完成：

- 新增 `wing_progress_event_fn_t`。
- `wing_progress_t` 增加 event callback 与 event_arg。
- `wing_progress.c` 增加 pointer/key/encoder 事件处理。
- `wing_progress_set_step()` / `wing_progress_get_step()` / `wing_progress_set_event_cb()` 接入 widgets API。
- `wing_gui_demo` 新增 progress event 日志，覆盖 value changed、key、encoder、focus、pointer capture/release、pointer up。
- `wing_gui_demo` synthetic input 新增 progress pointer down/drag/up、keyboard right、encoder rotate。

验证意图：

- progress/slider/scrollbar 都共享 `wing_value_model_t` 的 range/value/step/input 语义。
- progress 拖拽会触发 `WING_EVENT_VALUE_CHANGED`，并通过 dirty/present 刷新。
- keyboard/encoder 输入也能影响 progress value，满足“可被输入影响的控件都要验证”的目标。

## 2026-06-12 默认 3D/object space 文件边界确认与运行验证

本次根据架构判断继续确认：WING GUI 不应把 3D 作为 `wing_3d.c` 或 `wing_3d_view.c` 这类独立扩展模块拆出。当前工作树已确认 `apps/graphics/wing` 下不存在 `wing_3d.c` / `wing_3d_view.c` 源文件，也没有对应 Makefile/API 引用。3D 能力继续以内建 object space 的形式存在：普通 2D widget 是默认 camera、z=0、identity transform 下的特殊呈现。

本次验证结果：

- 已重新构建 `sim-wing`，构建通过，编译路径包含 `wing_space.c`、`wing_object.c`、`wing_render.c`、`wing_progress.c`、`wing_gui_demo_main.c`。
- 已启动 `./FeatherCore/build/sim-wing`，从 NSH 执行 `wing_gui_demo`。
- demo 日志确认默认空间能力走 core 路径：`core space treats 2D widgets as identity space transform objects by default`、`root object default 2D plane API reports root=yes in default object space`、`runtime camera viewport ... focal=640`、`space card projected quad ...`、`custom geometry is a normal WING object ... not a separate 3D view`。
- demo 日志确认 WING GUI 继续覆盖 timer、repeat timer、一次性 animation、loop animation、dirty redraw、present rect、event bubbling、focus、state style、projected hit-test、FRender quad/triangle command seed。
- demo 日志确认可输入控件路径覆盖 progress、slider、scrollbar、button、checkbox、switch、text input、scroll view；progress 已支持 pointer/key/encoder 输入并输出 value changed / focus / capture / release 事件。
- 已通过 X11 输入脚本注入真实 pointer move、click 和 keyboard Right；日志出现 `x11 input provider emitted ... pointer_move`、`pending type=pointer_down`、`pending type=pointer_up`、`type=key_down ... key=1001`，并触发 bubbled click。
- 已通过 X11 close 关闭窗口；日志出现 `framebuffer window closed`、`root received close request through WING input/event queue`、`app task exit`，并回到 `nsh>`。

结论：当前方向正确，WING 不需要新增 `wing_3d.c` / `wing_3d_view.c`。后续如果补 mesh、shader 或 GPU 3D backend，应继续消费同一套 `wing_space.c` / object transform / runtime camera / render frontend / FRender command-list 数据，而不是引入独立 3D view 子系统。

## 2026-06-12 普通 object 统一 geometry callback 入口

本次继续推进默认 3D/object space 架构，目标是让自定义几何成为普通 `wing_obj_t` 的核心能力，而不是独立 `wing_3d_view` 或额外 3D 模块。

已完成内容：

- 新增 `wing_obj_set_geometry_cb()` object API，一次性绑定 `draw`、`screen_bounds`、`contains_point` 三类几何回调。
- 该 API 在对象已接入 GUI 时会同时采集旧 screen bounds 与新 screen bounds 并触发 dirty invalidation，避免几何回调更新后出现旧区域残影。
- `wing_gui_demo` 的 custom projected triangle 已从散绑 `wing_obj_set_draw_cb()` / `wing_obj_set_screen_bounds_cb()` / `wing_obj_set_contains_point_cb()` 改为使用 `wing_obj_set_geometry_cb()`。
- demo 日志更新为 `custom geometry is a normal WING object using unified geometry callbacks, not a separate 3D view`，明确 custom geometry 是普通 object 能力。

验证结果：

- 已重新构建 `sim-wing`，构建通过。
- 已启动 `./FeatherCore/build/sim-wing`，从 NSH 执行 `wing_gui_demo`。
- demo 日志确认 custom triangle 继续覆盖 projected dirty bounds、precise hit-test、runtime camera、core space projection、FRender triangle command seed。
- demo 日志确认 timer、repeat timer、一次性 animation、loop animation、dirty redraw、present rect、button/progress/slider/scrollbar/text input/checkbox/switch/scroll view 事件继续正常。
- 已通过 X11 输入脚本注入 pointer move、click、keyboard Right；日志确认 X11 input provider 收到 pointer move/down/up 与 key down，并触发 bubbled click。
- 已通过 X11 close 关闭窗口；日志出现 `framebuffer window closed`、`root received close request through WING input/event queue`、`app task exit`，并回到 `nsh>`。

结论：`wing_obj_set_geometry_cb()` 是后续 mesh/material/shader 的更合适入口种子。它仍然属于普通 WING object API，不引入独立 3D view 边界。

## 2026-06-12: WING GUI 图层排序与 demo 覆盖问题修正

- 结论：WING GUI core 已有对象树内的局部排序机制，不是单纯依赖父子关系。
- 当前排序范围：同一父对象下的 sibling 使用 `z_index` 排序；相同 `z_index` 时使用投影 depth 排序。
- 绘制顺序：父对象先绘制，子对象按 sibling 排序绘制，前景对象后绘制。
- 命中顺序：hit-test 使用绘制顺序的反向路径，优先命中前景对象。
- 本次问题：`wing_gui_demo` 中 space/depth card 与下方 progress/slider/scrollbar 视觉区域重叠，且装饰卡片的 z 层高于默认控件层，导致拖动下方控件时看起来被异常覆盖。
- 修正：为 progress/slider/scrollbar 显式设置 `WING_GUI_DEMO_CONTROL_Z_INDEX=20`，高于 space card 的展示层；space card 保持在投影稳定区，不再强行移动到导致投影跑出屏幕的上方 rail。
- 验证：重新构建 `sim-wing`，从 NSH 执行 `wing_gui_demo`；日志确认 `space card screen dirty bounds x=281 y=203 w=82 h=75` 在屏幕内，且 `progress/slider/scrollbar use z-index=20` 生效。
- 验证：真实 X11 输入路径触发 slider/scrollbar pointer capture、连续 value changed、release；关闭 X11 窗口后 demo 收到 close request 并返回 NSH。
- 后续：WING GUI 当前只提供对象树局部排序。WING Desktop 阶段仍需要独立的 window layer/topmost/modal/overlay/cursor/compositor layer 管理。

## 2026-06-12: object layer 常量与 value 控件 focused 覆盖修正

- 新增 `wing_obj_layer_e`，为 object-level `z_index` 提供公共语义层：background、content、decoration、control、overlay、modal、cursor。
- `wing_gui_demo` 不再使用孤立的 demo 裸数字表达 control 层，改为使用 `WING_OBJ_LAYER_CONTROL`；space card 使用 `WING_OBJ_LAYER_DECORATION`，depth sorting demo 使用 content 上的局部偏移。
- 注意：这仍然是 WING GUI object tree 内部的 layer seed，不等价于 WING Desktop 的 window-level layer manager。
- 修正 slider/scrollbar 拖动时 focused state style 使用 panel/fill 样式导致整块背景覆盖控件内容的问题。
- 默认主题和 high-contrast 主题中，`slider_focused` 与 `scrollbar_focused` 改为纯 stroke focus ring，只强调焦点，不重新填充控件背景。
- 验证：重新构建 `sim-wing` 成功；从 NSH 执行 `wing_gui_demo`，synthetic input 覆盖 progress/slider/scrollbar 拖动、button、checkbox、switch、text input、scroll view、space/depth hit-test、timer、animation、dirty/present。
- 验证：真实 X11 click/key 输入触发 slider 和 scrollbar 的 pointer capture、value changed、release、keyboard step；关闭 X11 窗口后 demo 返回 NSH。

## 2026-06-12: value 控件 focused ring 可观测验证

- `wing_gui_demo` 新增 focused ring 样式摘要日志，直接输出 slider/scrollbar focused style 的 `has_fill`、`has_stroke` 与 `stroke_width`。
- 该日志用于防止 value 控件状态样式再次退化为 filled panel，造成拖动时 focused 背景覆盖轨道/滑块或产生局部刷新视觉污染。
- 验证日志：`value widget focused rings are stroke-only slider_fill=no slider_stroke=yes slider_stroke_width=2 scrollbar_fill=no scrollbar_stroke=yes scrollbar_stroke_width=1`。
- 重新构建 `sim-wing` 成功；从 NSH 执行 `wing_gui_demo`，synthetic input 覆盖全部已实现交互控件、事件、timer、单次动画、loop 动画、dirty/present。
- 真实 X11 输入验证 slider/scrollbar click、pointer capture、value changed、release，以及 keyboard Right step；关闭 X11 窗口后返回 NSH。

## 2026-06-12: value 控件 state style 分阶段绘制

- 新增 `wing_widget_draw_style_background_for_obj()` 与 `wing_widget_draw_style_overlay_for_obj()`。
- value 控件状态样式形成明确绘制契约：`fill/clear` 只能作为内容下方的状态背景，`stroke` 只能作为内容上方的状态 ring。
- `wing_slider` 与 `wing_scrollbar` 改为先绘制 state background，再绘制 track/fill/thumb/knob，最后绘制 state overlay。
- 这样即便未来主题为 focused/pressed/hovered 提供填充色，也不会在拖动时把滑轨、滑块或 thumb 盖住。
- `wing_gui_demo` 新增日志：`value widget state style is staged as background fill before content and overlay stroke after content`，用于验证该框架约束。
- 验证：重新构建 `sim-wing` 成功；从 NSH 执行 `wing_gui_demo`，日志确认 staged state style、stroke-only focused ring、synthetic input、timer、单次动画、loop animation、dirty/present。
- 验证：真实 X11 click 操作 slider/scrollbar，触发 pointer capture、value changed、release；真实 X11 keyboard Right 触发 focused scrollbar key step；关闭 X11 窗口后 demo 退出并返回 NSH。

## 2026-06-12: value input behavior 种子

- 新增内部 helper `wing_widget_value_input.c/.h`，作为 TouchGFX mixin/behavior 思想在 WING 中的第一颗种子。
- progress、slider、scrollbar 不再各自重复实现 key/encoder step 输入循环，而是复用 `wing_widget_value_handle_step_input()`。
- pointer-to-value 仍保留在具体控件中，因为 progress/slider/scrollbar 的几何映射不同；本次只收敛通用的 step input behavior。
- 该 helper 目前保持在 `src/core` 内部，不暴露为应用 API，避免过早冻结 behavior/mixin 接口。
- `wing_gui_demo` 新增日志：`progress/slider/scrollbar share WING value input behavior for key and encoder step handling`。
- 验证：重新构建 `sim-wing` 成功，构建输出确认 `wing_widget_value_input.c` 已编译。
- 验证：从 NSH 执行 `wing_gui_demo`，synthetic input 覆盖 progress/slider/scrollbar 的 pointer/key/encoder 路径，日志继续输出 key step、encoder delta、value changed、capture/release。
- 验证：真实 X11 click 操作 slider/scrollbar，真实 X11 keyboard Right 触发 focused scrollbar key step；关闭 X11 窗口后 demo 收到 close request 并返回 NSH。

## 2026-06-12: pointer value drag behavior 种子

- `wing_widget_value_input.c/.h` 继续扩展，新增 `wing_widget_value_handle_pointer_drag()`。
- progress、slider、scrollbar 不再各自重复实现 pointer down/move/up/cancel 的 pressed、capture、update、release 生命周期。
- 具体的 point-to-value 几何映射仍留在各控件内部，behavior 只管理通用 pointer drag 状态机。
- `wing_gui_demo` 新增日志：`progress/slider/scrollbar share WING pointer drag behavior for pressed/capture/update/release lifecycle`。
- 这一步进一步吸收 TouchGFX behavior/mixin 的思想，但仍保持为 WING 内部 helper，不公开给应用 API。
- 验证：重新构建 `sim-wing` 成功，progress、slider、scrollbar 与 `wing_widget_value_input.c` 均重新编译。
- 验证：从 NSH 执行 `wing_gui_demo`，synthetic input 覆盖 progress/slider/scrollbar 的 pointer drag、capture/release、cancel、key、encoder、value changed。
- 验证：真实 X11 pointer 操作 slider/scrollbar，触发 focus、pointer capture、value changed、release；真实 X11 keyboard Right 触发 focused scrollbar key step；关闭 X11 窗口后 demo 收到 close request 并返回 NSH。

## 2026-06-12: behaviors 目录分层

- 将 `wing_widget_value_input.c/.h` 从 `src/core` 移动到 `src/behaviors`。
- `src/core` 继续表达 GUI runtime、object、event、dirty、layout、render、timer、animation 等核心机制。
- `src/behaviors` 用于存放可复用 widget 行为/mixin 种子，当前包含 value step input 与 pointer drag lifecycle。
- 默认 3D/object space 原则继续作为吸收外部 GUI 设计的前置过滤器：LVGL、TouchGFX、HoneyGUI 的优秀机制不能原样搬成传统 2D GUI 分支，而应先映射到 WING 的 object tree、runtime camera、space transform、dirty、event 和 FRender backend 语境中。
- 已重新构建 `sim-wing`，构建通过，编译输出包含 `apps/graphics/wing/src/behaviors/wing_widget_value_input.c`。
- 已从 NSH 执行 `wing_gui_demo`，日志确认 `reusable widget behaviors live under WING src/behaviors instead of core runtime`。
- synthetic input 已覆盖 progress / slider / scrollbar 的 pointer drag、capture/release、cancel、key、encoder、value changed，同时覆盖 timer、repeat timer、单次 animation、loop animation、dirty/present。
- 真实 X11 输入已验证：点击下方 slider 与 scrollbar 后分别触发 focus、pointer capture、value changed、release；随后关闭 X11 窗口，demo 输出 `framebuffer window closed`、`root received close request through WING input/event queue`、`app task exit` 并回到 NSH。

## 2026-06-12: Render node / material seed

- 新增 `wing_render_node_t`、`wing_render_material_t` 和 `wing_gui_submit_render_node()`，作为 WING render frontend 的第一版描述型提交入口。
- 当前 render node 支持第一阶段已有 primitive：fill rect、fill quad、fill triangle、draw quad、blit、blit quad；执行路径仍 dispatch 到现有 FRender command，未新增独立 3D 子系统或绕开 FRender。
- `wing_render_material_t` 当前只保存 color + opacity，用作后续 material / shader / texture state 的最小种子；第一版仅解析为最终 RGBA。
- `wing_gui_demo` 的 custom projected triangle 已从直接调用 `wing_gui_fill_triangle()` 改为提交 `WING_RENDER_NODE_FILL_TRIANGLE`，验证 custom geometry 可以走普通 object space projection -> render node/material seed -> FRender primitive。
- 这一步吸收 HoneyGUI 引擎化 / render push 思想，但按 WING 语境翻译为默认 object space 与统一 render backend，而不是引入独立 `wing_3d_view`。
- 已重新构建 `sim-wing`，构建通过，重新编译 `wing_render.c` 与 `wing_gui_demo_main.c`。
- 已从 NSH 执行 `wing_gui_demo`，日志确认 `triangle primitive object uses runtime camera + core space projection -> WING render node/material seed -> FRender fill_triangle command seed`。
- synthetic input 已覆盖 button、checkbox、switch、text input、scroll view、space card、depth card、progress、slider、scrollbar，以及 timer、repeat timer、单次 animation、loop animation、lifecycle、dirty/present。
- 真实 X11 输入已验证：点击 slider / scrollbar 触发 focus、pointer capture、value changed、release，Right 键触发 focused scrollbar step；关闭窗口后 demo 通过 close request 回到 NSH。

## 2026-06-12: Render node 头文件边界整理

- 将 render node/material 类型从 `<wing/wing.h>` 主体移动到 `include/wing/core/wing_render_node.h`。
- `<wing/wing.h>` 继续聚合该子头，保持现有 WING GUI 应用兼容；render node 的概念边界则归入 core render frontend。
- 新子头说明 render node 位于 object/widget draw callback 与 FRender command 之间：WING 负责 object space、dirty culling 和 widget state，FRender 继续负责统一后端执行。
- 该整理继续落实“头文件不要全塞在一起”的结构原则，也为后续 material / mesh / shader descriptor 扩展保留更清楚的文件边界。
- 已重新构建 `sim-wing`，构建通过。
- 已从 NSH 执行 `wing_gui_demo`，确认 render node/material seed 日志仍出现，synthetic input、timer、单次 animation、loop animation、dirty/present 均正常。
- 真实 X11 输入已验证：slider / scrollbar click、keyboard Right 进入 WING input provider 并触发 value 控件事件；关闭 X11 窗口后 demo 收到 close request、退出并回到 NSH。

## 2026-06-12: Render frontend 函数声明边界整理

- 将 `<wing/wing.h>` 主体中的 render frontend 函数重复声明移除，改为通过 `include/wing/core/wing_render.h` 聚合暴露。
- `wing_render.h` 现在继续作为 surface/context、clip、primitive、render node/material、blit、software flush、checksum 和 dirty render 的公开边界。
- `<wing/wing.h>` 仍作为兼容聚合入口，应用侧包含 `<wing/wing.h>` 仍可访问 render API。
- 该整理继续贯彻“专属功能放专属头文件”的原则，避免 render frontend、runtime、input、event、object、space、widgets 全部坍缩进一个聚合头主体。
- 已重新构建 `sim-wing`，构建通过。
- 已从 NSH 执行 `wing_gui_demo`，确认 render node/material seed、默认 object space、synthetic input、timer、单次 animation、loop animation、dirty/present 正常。
- 真实 X11 输入已验证：slider / scrollbar click、keyboard Right 进入 WING input provider 并触发 value 控件事件；关闭 X11 窗口后 demo 收到 close request、退出并回到 NSH。
- `wing_progress`、`wing_slider`、`wing_scrollbar` 更新为引用 `../behaviors/wing_widget_value_input.h`。
- `wing_gui_demo` 新增日志：`reusable widget behaviors live under WING src/behaviors instead of core runtime`。

### 2026-06-12：Phase 1 control z-layer 稳定性收口

本次更新针对 WING GUI Phase 1 runtime closure 中的图层稳定性问题，先把 `wing_gui_demo` 的 value controls 从隐式同层插入顺序，改为显式 demo 视觉层级。

已完成：

- `wing_gui_demo` 新增 progress / slider / scrollbar 的独立 z-index 常量。
- progress、slider、scrollbar 不再全部使用同一个 `WING_GUI_DEMO_CONTROL_Z_INDEX`。
- demo 现在明确表达：上方 value control 的 z-index 高于下方 value control，避免下方控件的 focused/pressed state ring 或背景重绘在 overlap/dirty redraw 场景中覆盖上方控件。
- 保持 WING core 原则不变：core 仍只提供 z-index + projected depth + sibling order；demo 负责声明自己的视觉层级。

阶段意义：

- 这是 Phase 1 的层级稳定性收口，不是 Desktop window manager。
- 该改动确认 WING GUI runtime 的 object tree 排序应该可被应用显式控制，而不是依赖控件创建顺序的偶然结果。
- 后续 WING Desktop layer manager 可以在同一 z-index/depth 机制上继续建立 window / modal / overlay / cursor 层。

### 2026-06-12：Phase 1 public header boundary 收口

本次更新继续推进 WING GUI Phase 1 runtime closure 的模块边界整理。

已完成：

- `wing.h` 继续保留为应用侧聚合入口。
- `wing.h` 不再直接声明 tick / focus / pointer capture / timer / animation 的函数原型。
- tick / focus / capture / timer / animation 相关公开 API 改由对应 `include/wing/core/wing_*.h` 专属头文件提供。
- `wing.h` 聚合 `wing/core/wing_capture.h`、`wing/core/wing_focus.h`、`wing/core/wing_tick.h`、`wing/core/wing_theme.h`、`wing/core/wing_text_edit.h`，让应用仍可一站式包含 `wing/wing.h`。

阶段意义：

- 继续把 `wing.h` 从“所有 API 的堆场”收敛为“聚合入口”。
- 保持 WING GUI core 的功能边界清晰，方便 Phase 1 后冻结 GUI 功能扩张，并切换到 FRender / NuttX graphics 能力补充。

### 2026-06-12：Phase 1 固定验收脚本通过

本次更新新增并执行 WING GUI Phase 1 固定验收脚本。

新增脚本：

- `FeatherCore/tools/firmware/sim/validate-wing-phase1.sh`

脚本覆盖：

- 构建 `sim-wing`。
- 启动 NuttX sim NSH。
- 从 NSH 执行 `wing_gui_demo`。
- 检查 runtime summary、framebuffer、X11 input provider、explicit control z-layers、default object space、render node/material seed、dirty、redraw chunk 和 present rect 日志。
- 通过 `x11-input.sh` 发送 X11 close request。
- 检查 close request 进入 WING input/event queue。
- 检查 demo 退出。

验证结果：

- 已执行 `./FeatherCore/tools/firmware/sim/validate-wing-phase1.sh`。
- 构建通过。
- `wing_gui_demo` 从 NSH 启动成功。
- X11 framebuffer 路径成功进入。
- runtime / object space / input / timer / animation / dirty / present / close request 关键日志全部观察到。
- 关闭 X11 窗口后，demo 输出 `root received close request through WING input/event queue` 和 `app task exit`。
- 脚本最终输出 `WING GUI Phase 1 runtime closure validation passed`。

阶段意义：

- Phase 1 已拥有固定、可重复执行的闭环验收入口。
- 后续宣称 WING GUI runtime closure 可用时，优先以该脚本作为证据。

### 2026-06-12：WING GUI Phase 1 Runtime Closure 收口完成

本次基于 `FeatherCore/docs/plans/WING_GUI_PHASE1_ACCEPTANCE_ZH.md` 完成 Phase 1 最终收口。

最终结论：

- `WING GUI Phase 1: Runtime Closure` 已完成。
- 固定验收脚本 `FeatherCore/tools/firmware/sim/validate-wing-phase1.sh` 已通过。
- WING GUI 作为独立 GUI 库可以从 NSH 启动 demo、打开 X11 framebuffer、处理 input/event/timer/animation/dirty/render/present，并通过 close request 退出回到 NSH。
- WING GUI Phase 1 之后暂停功能扩张，只保留 bugfix、边界清理、验证脚本维护和必要稳定性修正。

下一阶段主线：

- FRender planner / capability / fallback。
- NuttX graphics capability / present / accelerator 接入点。
- 后续再进入 WING Desktop 和 Pinion。

## Phase 1 后结构边界清理：widget support 从 core/behaviors 收敛

本次清理属于 Phase 1 后允许保留的“边界清理 / 必要稳定性修正”范围，不新增 WING GUI 大型 feature。

完成项：

- 移除 `apps/graphics/wing/src/behaviors` 目录。
- 将 widget 专属复用输入行为移动为 `apps/graphics/wing/src/widgets/internal/wing_value_input.c/.h`。
- 将 `wing_widget.c`、`wing_widget_input.c`、`wing_widget_style.c`、`wing_widget_value.c` 从 `src/core` 移动到 `src/widgets/internal`。
- `src/core` 重新收敛为 runtime core，不再承载 widget 专属实现文件。
- `wing_text_input.c` 保持在 `src/widgets`，因为它是具体 widget；文本编辑模型仍由 `src/core/wing_text_edit.c` 负责。
- 新增结构说明文档：`docs/plans/WING_GUI_STRUCTURE_BOUNDARY_ZH.md`。

判断：

- 这次清理不改变公开 API。
- 这次清理不改变 Phase 1 验收目标。
- 后续若要移动公开 widget helper headers，应放到单独 API cleanup 阶段处理。

## Phase 1 后 demo 可见性修正：持续循环动画

背景：

- `wing_gui_demo` 之前虽然验证了 animation/timer 能力，但 `loop width animation` 只执行有限的两段，完成后进入 idle。
- 从用户肉眼观察角度，这不像一个“循环运行的动画”，容易误判为 WING GUI handle 没有持续推进动画。

完成项：

- 将 `wing_gui_demo` 中的 `line_secondary_width` 动画改为持续 pulse 动画。
- 持续 pulse 动画在 `line_secondary` 宽度之间往返，作为肉眼可见的运行中动画。
- 动画日志从有限段落改为 `continuous pulse animation ... repeat=forever`。
- 验证脚本不再等待 `idle frame no redraw`，改为等待持续动画至少完成一段后再发送 X11 close request。

判断：

- 这属于 Phase 1 后允许的 demo 可见性修正和验证脚本维护。
- 未改变 WING GUI core 架构。
- 未新增 WING GUI 大型 feature。

## Phase 1 后 demo 稳定性修正：持续动画期间输入可响应

背景：

- 持续 pulse 动画启用后，`wing_gui_demo` 每帧都会产生 redraw。
- 原 demo 只在 idle frame 分支中 `usleep(WING_GUI_DEMO_FRAME_MS)`。
- 持续动画运行时不会进入 idle 分支，导致 demo 主循环可能高速空转，影响 X11 输入处理和交互响应。

修正：

- 在有 framebuffer presenter 的情况下，即使当前帧发生 redraw/present，也在帧尾 `usleep(WING_GUI_DEMO_FRAME_MS * 1000)`。
- 这样 demo 保持约 30 FPS 的节奏，动画继续循环，同时窗口系统和 WING input reader 有机会正常处理输入。

判断：

- 这是 demo 主循环节流和输入响应稳定性修正。
- 未修改 WING GUI core。
- 属于 Phase 1 后允许的必要稳定性修正。

## Phase 1 后 demo 输入链路修正：接入 X11 presenter input provider

背景：

- 持续动画和帧 pacing 修正后，用户交互仍然没有反馈。
- 根因不在 WING GUI core 的 `wing_gui_handle()` 是否每帧检查输入，而在 demo wiring：`input_provider.presenter` 被设置为 `NULL`。
- 因此 `wing_gui_demo_input_provider()` 虽然每帧被调用，但不会从 X11/framebuffer presenter poll 用户输入。

修正：

- 有 framebuffer presenter 时，将 `input_provider.presenter` 设置为 `&presenter`。
- 没有 presenter 的 headless 场景仍保持 `NULL`，只使用 demo-script 输入。

判断：

- 这是 demo 输入链路接线 bugfix。
- 未改变 WING GUI core。
- 属于 Phase 1 后允许的必要稳定性修正。

## Phase 1 验证脚本增强：覆盖普通 X11 输入

背景：

- 之前验证脚本只通过 X11 close request 验证窗口关闭路径。
- close request 由 demo 主循环直接检测 framebuffer window closed，因此不能证明普通鼠标/键盘输入已经进入 WING input provider。

增强：

- `validate-wing-phase1.sh` 在发送 close request 之前，先向 NuttX X11 窗口发送一次普通 click。
- 脚本等待 `wing_gui_demo: x11 input provider emitted type=pointer_down source=mouse`。
- 这可以验证 X11/framebuffer presenter 的普通输入事件确实进入 WING GUI input reader。

判断：

- 这是 Phase 1 后验证脚本维护。
- 目标是避免“窗口能关，但普通输入没接入”的假通过。

## Phase 1 后 sim X11 输入修正：poll input 前同步 X server 事件

背景：

- `wing_gui_demo` 已将 X11/framebuffer presenter 接入 input provider，但强化验证中的普通 X11 click 仍未被观察到。
- `fr_fb_presenter_poll_input()` 调用 `sim_x11pollinput()`。
- `sim_x11pollinput()` 直接 `XCheckWindowEvent()`，没有先从 X server 同步外部 client 投递的事件。

修正：

- 在 `sim_x11pollinput()` 检查本地事件队列前调用 `XSync(g_display, False)`。
- 目标是把外部 X11 client 发来的 pointer/key 事件拉入当前 Display connection 的本地队列，再由 `XCheckWindowEvent()` 取出。

判断：

- 这是 NuttX sim X11 framebuffer 输入接收路径的稳定性修正。
- 它补齐了 FRender/WING GUI demo 依赖的普通 X11 输入验证路径。

## 2026-06-12 Phase 1 后 sim X11 framebuffer 输入抓取修正

- 现象：`wing_gui_demo` 动画循环运行时，验证脚本发送 X11 click 后日志仍持续显示 `handler input polled=0`，说明输入没有进入 `frender` presenter poll 路径。
- 判断：关闭窗口能退出只证明 `sim_x11pollwindowclosed()` 可用，不能证明普通 pointer/key 输入链路可用。
- 修正：`nuttx/arch/sim/src/sim/posix/sim_x11framebuffer.c` 中 X11 framebuffer 窗口现在始终抓取 Button1 事件，而不再依赖 legacy `CONFIG_SIM_TOUCHSCREEN` / `CONFIG_SIM_AJOYSTICK` / `CONFIG_SIM_BUTTONS` 配置。
- 目标：让 `apps/graphics/frender` 的 framebuffer presenter 可以稳定通过 `sim_x11pollinput()` 获取普通鼠标输入，进而让 WING GUI 的 `wing_gui_handle()` 在动画循环中同时推进输入、事件、动画、脏块和绘制。

## 2026-06-12 Phase 1 后输入诊断增强：实时 raw mouse trace

- 背景：用户手动运行 `wing_gui_demo` 时仍反馈按键、拖动滑轨没有明显反应，仅凭控件回调日志不足以判断输入是否进入 WING。
- 调整：在 `apps/examples/wing_gui_demo/wing_gui_demo_main.c` 的 `fr_fb_presenter_poll_input()` 入口后增加 raw input trace。
- 日志格式：`wing_gui_demo: x11 raw input seq=<n> type=<pointer_move|pointer_down|pointer_up|...> pressed=<yes|no> point=<x,y> button=<button> key=<key> encoder=<delta>`。
- 目的：把输入问题分成两段定位：如果没有 raw 日志，问题在 `frender/sim_x11` 输入采集以下；如果有 raw 日志但控件无反应，问题在 WING hit-test / focus / capture / widget event 消费链路。

## 2026-06-12 Phase 1 后输入排查：临时关闭循环 pulse 动画

- 背景：用户手动运行 `wing_gui_demo` 时反馈仍看不到任何鼠标输入；用户指出在循环动画开启前，鼠标控制控件曾可正常工作。
- 调整：`apps/examples/wing_gui_demo/wing_gui_demo_main.c` 新增 `WING_GUI_DEMO_ENABLE_LOOP_ANIMATION`，当前默认置为 `0`，保留循环动画实现但不再启动无限 pulse。
- 调整：循环动画 done 回调在禁用状态下不会重启动画，避免一次启动后继续续命。
- 调整：Phase 1 sim 验证脚本改为等待 `continuous pulse animation disabled for input debugging`，不再等待循环动画 segment 日志。
- 目的：先移除持续动画对输入排查的干扰，保留 raw mouse trace，用于确认移动、按下、松开是否进入 `wing_gui_demo` 输入入口。

## 2026-06-12 Phase 1 验证工具修正：X11 focus 失败不阻断 click

- 现象：关闭循环 pulse 动画后，Phase 1 验证脚本在发送 X11 click 时，`x11-input` 触发 `X_SetInputFocus BadMatch` 并退出。
- 判断：这是测试辅助工具对某些 X11 窗口强制设置焦点导致的非致命错误，不应阻断鼠标 motion/button 事件发送。
- 修正：`tools/firmware/sim/x11-input.c` 新增 `x11_input_try_focus()`，对 `XSetInputFocus()` 使用临时 X error trap；focus 失败不再中断 motion/click/key 发送流程。

## 2026-06-12 Phase 1 验证脚本修正：等待 pending pointer down

- 现象：raw 输入已显示 `pointer_down`，但输入 provider 为了保持 move/down 顺序会先 emit move，再通过 pending emit pointer down。
- 调整：Phase 1 验证脚本现在等待 `x11 input provider emitted pending type=pointer_down source=mouse`，与当前输入 adapter 行为一致。

## 2026-06-12 Phase 1 后 runtime 事件泵修正：输入、timer、animation 统一帧源

- 设计修正：不采用“输入优先 / 动画优先”的模型。WING GUI 将输入、timer、animation 都视为 runtime tick 的帧源，由统一事件泵收集和推进。
- WING core 调整：`wing_gui_handle()` 现在先调用 `wing_gui_poll_input()` 将平台输入泵入 WING 输入队列，再调用 `wing_gui_tick()` 推进 timer/animation，最后由 `wing_gui_step()` 统一 layout、dispatch input/event、render dirty。
- 参考模型：LVGL 的 `lv_timer_handler()`、TouchGFX 的 frame tick、HoneyGUI 的 scene update/render 都不是让输入和动画互相抢优先级，而是在一个 runtime tick 中统一推进输入、时间源和绘制提交。
- demo 调整：`wing_gui_demo` 新增 `WING_GUI_DEMO_TRACE_EACH_FRAME`，默认关闭逐帧 dirty/redraw/present/frame checksum 日志。raw mouse trace 仍实时输出，输入帧仍输出诊断。这样避免循环动画或频繁 redraw 时 stdout 成为 sim/X11 输入体验的瓶颈。
- 当前策略：循环 pulse 动画仍按排查要求默认关闭；后续可将 `WING_GUI_DEMO_ENABLE_LOOP_ANIMATION` 打开作为压力测试，验证统一事件泵在持续动画下仍能稳定处理输入。

## 2026-06-12 Phase 1 后循环动画重新打开并收敛启动路径

- 调整：`apps/examples/wing_gui_demo/wing_gui_demo_main.c` 中 `WING_GUI_DEMO_ENABLE_LOOP_ANIMATION` 重新置为 `1`，恢复无限 ping-pong pulse 动画。
- 重构：循环动画的初始启动和 done 回调重启统一走 `wing_gui_demo_loop_width_anim_start()`，避免启动路径和重启路径各自拼装 duration/path/apply/done 参数。
- 保留：`WING_GUI_DEMO_TRACE_EACH_FRAME` 仍默认置为 `0`，避免无限动画每帧 dirty/redraw 时刷屏影响 sim/X11 输入体验。
- 验证脚本：`tools/firmware/sim/validate-wing-phase1.sh` 重新等待 `continuous pulse animation scheduled` 和 `continuous pulse animation segment=1 completed`，让循环动画重新成为 Phase 1 运行时压力项。
- 设计原则：输入、timer、animation 都是 WING runtime 的帧源。循环动画恢复后应继续通过统一事件泵推进，不允许动画 dirty/render 阻断平台输入进入 WING 队列。
