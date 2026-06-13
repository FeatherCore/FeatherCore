# WING GUI 实现进度与更新计划

更新时间：2026-06-12

本文用于记录 WING GUI 从当前 seed 实现走向可用 GUI 库的真实进度、下一步计划和每次更新后的验证结果。

本文不是架构总览。架构和设计依据请参考：

- `FeatherCore/docs/WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md`
- `FeatherCore/docs/WING_GUI_DESIGN_REFERENCES_ZH.md`
- `FeatherCore/docs/FRENDER_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/plans/WING_GUI_PROGRESS_AND_PLAN_ZH.md`

## 1. 当前目标

当前第一目标是先实现可独立运行的 WING GUI，而不是先实现 WING Desktop。

也就是说：

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
  framebuffer / simulator display / input foundation
```

WING Desktop 后续应作为一个大型 WING GUI 应用存在，不是 WING GUI 程序的强制运行环境。

## 2. 当前实现状态

### 2.1 已经具备的能力

当前 `apps/graphics/wing` 已经具备第一阶段 GUI runtime 雏形：

- `wing_gui_t` GUI 执行句柄。
- `wing_gui_tick()` 推进时间。
- `wing_gui_step()` 推进输入、事件、布局、绘制。
- `wing_gui_handle()` 作为应用侧周期性调用的 GUI handler。
- `wing_gui_frame_t` 作为 handler 的 frame 诊断输出。
- `wing_gui_set_input_reader()` 注册输入 provider。
- `wing_gui_poll_input()` 拉取输入 provider 并放入 input queue。
- `wing_gui_is_running()` / `wing_gui_request_stop()` 控制 demo 生命周期。
- `wing_obj_t` 对象树。
- object child / sibling 链接。
- object bounds。
- object state。
- object flags。
- object invalidation。
- dirty rect 查询。
- draw traversal。
- hit test。
- input queue。
- input provider。
- event queue。
- event bubbling。
- stop propagation。
- focus gained / focus lost。
- keyboard focus traversal。
- key down / key up。
- pointer down / move / up。
- click event。
- software timer。
- linear animation。
- fixed / horizontal stack / vertical stack layout。
- 默认主题 seed。
- 基础组件：box、button、label、panel、progress、slider、scrollbar、switch、checkbox。
- `wing_gui_demo` 支持 headless 多帧验证。
- `wing_gui_demo` 可展示 dirty rect 在 tick / step 前后的生命周期。

### 2.2 当前 FRender 对 WING GUI 的支撑状态

当前 `apps/graphics/frender` 已经可以作为 WING GUI 第一阶段渲染底座：

- 支持 command list。
- 支持 clear / fill rect / stroke rect / clip。
- 支持 software backend。
- 支持 capability declaration。
- 支持 backend registry。
- 支持 NuttX framebuffer present adapter。
- 支持 sim 下执行 demo 后再打开窗口。
- 支持关闭窗口后返回 NSH。

当前限制：

- NuttX graphics 目前主要是 present / capability adapter。
- 绘制命令还没有直接提交给 NXBE / DMA2D / GPU2D。
- FRender planner 还没有完整实现。
- text / image / blend / path / gradient 还需要扩展。

### 2.3 当前 wing_gui_demo 定位

`apps/examples/wing_gui_demo` 是 WING GUI 第一阶段验证模板。

它应该验证：

- GUI 初始化。
- 应用主循环调用 `wing_gui_tick()` / `wing_gui_step()`。
- object tree 创建。
- 基础组件绘制。
- input queue。
- event queue。
- pointer event。
- keyboard event。
- focus。
- event bubbling。
- stop propagation。
- timer。
- animation。
- layout。
- dirty / redraw。
- full redraw 与后续局部 invalidation。
- FRender command list 输出。
- framebuffer present。
- 关闭窗口后返回 NSH。

`wing_gui_demo` 不应该成为桌面系统，也不应该把 GUI runtime 应该负责的事情写在 demo 里。

当前验证记录：

```text
2026-06-12：
  已通过 ./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean 构建。
  已通过 env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing' 执行验证。
  headless 环境下 demo 固定运行 5 帧。
  首帧 dirty 覆盖 320x240 全屏。
  timer 将 progress 更新到 90。
  animation 在 tick=66/99/132 触发 line object 局部 dirty。
  tick=132 输出 width animation completed by WING runtime。
  每次 wing_gui_step() 渲染后 dirty 变为 none。
```

## 3. 设计参考落地规则

WING GUI 应从 LVGL / TouchGFX / HoneyGUI 中提取思想，但不复制实现。

### 3.1 从 LVGL 吸收

- 应用周期性调用 GUI handler。
- GUI runtime 内部维护 timer / animation / input / event / redraw。
- object tree 是 GUI 核心。
- state style 驱动组件视觉状态。
- 输入设备和显示设备抽象。
- 局部刷新优先。

落地到 WING：

- `wing_gui_t` 是应用持有的 GUI 执行句柄。
- `wing_gui_handle()` 是第一阶段应用侧 handler。
- `wing_gui_tick()` / `wing_gui_step()` 是 handler 内部阶段。
- `wing_theme_t` 是第一阶段主题资源容器。
- `wing_theme_init_default()` 提供默认主题。
- object / event / style / layout / dirty 都在 WING GUI 内部推进。

### 3.2 从 TouchGFX 吸收

- widget / container 产品化。
- 复杂组件独立封装。
- 每个组件有 demo 和使用方式。
- 组件不只是绘制矩形，而应能表达真实产品 UI。

落地到 WING：

- 先稳定基础组件。
- 再增加 progress、slider、scrollbar、switch、checkbox 等交互组件。
- 后续补 gauge、graph、list、carousel、keyboard 等产品级组件。

### 3.3 从 HoneyGUI 吸收

- GUI engine 分层。
- message / listener / input / render / resource 分离。
- dirty region 是刷新模型的一部分。
- 保留 vector / matrix / 2.5D / 3D 扩展口。
- 重视 simulator 和工具链。

落地到 WING：

- WING GUI 不直接操作 framebuffer。
- WING GUI 输出 FRender command list。
- FRender 再根据能力走 software / framebuffer / 未来硬件加速。
- 后续保留高级渲染 profile，而不是把 3D 能力硬塞进基础 widget。

## 4. 下一阶段计划

### Phase 1：稳定 WING GUI runtime

状态：进行中。

目标：

- 固化 `wing_gui_t` 执行模型。
- 明确 `handler -> tick/timer/animation -> layout -> input -> event -> render -> present` 顺序。
- 让 demo 只创建对象和投递输入，不直接承担 GUI 调度。
- 保证每次更新都能通过 sim 构建和 `wing_gui_demo` 验证。

验收标准：

- `wing_gui_demo` 可从 NSH 执行。
- demo 执行后打开窗口。
- demo 能完成输入、事件、动画、定时器、绘制验证。
- 关闭窗口后回到 NSH。

### Phase 2：完善 widget 基础层

状态：进行中。

目标：

- 抽出统一 widget helper / widget base。
- 减少 button / progress / slider / scrollbar / switch / checkbox 的重复代码。
- 固化 value widget 公共逻辑。
- 完善 focus / pressed / disabled 状态视觉反馈。
- 给可交互组件补 keyboard 操作。

验收标准：

- value 组件的 range / value / event 行为一致。
- focusable 组件能通过 Tab / 方向键 / Enter / Space 操作。
- demo 日志能展示 value changed / key handled / focus changed。

### Phase 3：完善 style / theme

状态：未开始。

目标：

- 建立更正式的 style selector。
- 支持 normal / pressed / focused / disabled。
- 支持基础 theme。
- 让 widget 使用统一 style 解析，不在每个组件里散落状态判断。

当前进展：

- 已新增 `wing_theme_t` 和 `wing_theme_init_default()`。
- `wing_gui_demo` 已使用默认主题初始化 root、header、panel、button、progress、slider、scrollbar、switch、checkbox 等组件样式。
- 主题仍是 seed，尚未支持继承、切换、selector cascade。

验收标准：

- demo 可以切换至少一套 theme。
- 同一组件在不同状态下有稳定视觉差异。
- 组件代码不再手写大量重复状态样式逻辑。

### Phase 4：完善 layout

状态：未开始。

目标：

- 在 fixed / stack 基础上补充更可用的布局策略。
- 支持 padding / margin / gap / alignment。
- 支持基础 flex-like 行为。

验收标准：

- demo 中 card / row / column 不再依赖大量手写坐标。
- 窗口尺寸变化时 layout 能重新计算。

### Phase 5：文本和资源系统

状态：未开始。

目标：

- 替换当前矩形 glyph seed。
- 增加字体资源描述。
- 支持 UTF-8 路线规划。
- 增加 image resource seed。

验收标准：

- label 可以使用真实字体或更接近真实字体的 bitmap font。
- demo 可以显示图片资源或图标资源。

### Phase 6：真实输入接入

状态：未开始。

目标：

- 将 NuttX touch / mouse / keyboard 输入转换为 `wing_input_event_t`。
- 保留 synthetic input 作为测试路径。
- 明确输入设备注册和读取策略。

验收标准：

- sim 下可接收真实鼠标/键盘事件。
- demo 中手动点击/按键能触发 WING event。

### Phase 7：WING Desktop 前置条件

状态：未开始。

目标：

- 在 WING GUI 足够稳定后，再开始 WING Desktop。
- Desktop 作为可选大型 WING GUI 应用。
- Desktop 负责 launcher / app registry / window manager / status area。

验收标准：

- 普通 WING GUI 应用仍可从 NSH 独立运行。
- 打包后的 WING GUI 应用可以进入 Desktop 管理环境。

## 5. 每次实现后的固定验证

每次修改 WING GUI 或 FRender 代码后，需要执行：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
```

并执行：

```sh
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

无 `DISPLAY` 环境下出现以下信息属于预期：

```text
ERROR: fb_register() failed: -19
```

这是无 X11 display 时 framebuffer simulator 注册失败，不代表 WING GUI 逻辑验证失败。

验证输出应重点观察：

- `wing_gui_demo` 是否启动。
- event callback 是否触发。
- focus gained / lost 是否触发。
- key down / key up 是否触发。
- timer 是否触发。
- animation 是否推进。
- value changed 是否触发。
- final frame 是否输出 command count 和 checksum。
- demo 是否能正常 `poweroff` 退出。

## 6. 更新记录

### 2026-06-12：建立 WING GUI 专用进度文档

完成：

- 新增本文档，作为 WING GUI 实现进度和更新计划的专用入口。
- 明确 `wing_gui_demo` 是第一阶段验证模板，不是 Desktop。
- 明确 WING GUI 当前应继续基于 FRender 和 NuttX graphics 推进。
- 明确每次代码修改后的固定构建和验证命令。

验证：

- 本次仅新增文档，未修改代码，未执行 sim 构建。

下一步：

- 继续完善 WING GUI widget 基础层。
- 继续保持每次代码修改后构建并执行 `wing_gui_demo`。
- 将每次实现结果追加到本文档更新记录。

### 2026-06-12：Slider / Scrollbar Focused 状态可视反馈

完成：

- 为 `wing_slider_t` 增加 focused state style 存储。
- 新增 `wing_slider_set_state_style()`，当前支持 `WING_OBJ_STATE_FOCUSED`。
- 为 `wing_scrollbar_t` 增加 focused state style 存储。
- 新增 `wing_scrollbar_set_state_style()`，当前支持 `WING_OBJ_STATE_FOCUSED`。
- slider / scrollbar draw 阶段在 focused 状态下叠加 focused outline。
- `wing_gui_demo` 为 slider / scrollbar 配置 focused 样式。
- `wing_gui_demo` 增加 focus gained 日志，用于验证焦点事件和 focused 样式路径。

设计意义：

- value widget 不再只是能响应 pointer/key，也开始具备状态驱动的视觉反馈。
- 当前接口先对齐 `wing_button_set_state_style()` 的方向，为后续统一 widget state style / theme 铺路。
- focused outline 通过 FRender command list 输出，因此仍保持 WING GUI -> FRender -> NuttX graphics 的分层。

本轮验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本轮验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: slider and scrollbar expose focused state visual feedback
wing_gui_demo: wing_slider focus gained and focused style is visible while focused
wing_gui_demo: wing_slider value changed from 83 to 88 by value input
wing_gui_demo: wing_slider handled key step key=1001
wing_gui_demo: wing_slider value changed from 88 to 83 by value input
wing_gui_demo: wing_slider handled key step key=1000
wing_gui_demo: wing_scrollbar focus gained and focused style is visible while focused
wing_gui_demo: wing_scrollbar value changed from 75 to 80 by value input
wing_gui_demo: wing_scrollbar handled key step key=1001
wing_gui_demo: wing_scrollbar value changed from 80 to 75 by value input
wing_gui_demo: wing_scrollbar handled key step key=1000
wing_gui_demo: frame tick=33 commands=58 checksum=0x4b8f67c5
wing_gui_demo: app task exit
```

说明：

- 构建通过。
- `wing_gui_demo` 运行通过。
- 无 DISPLAY 环境下 framebuffer present 跳过仍为预期现象。
- slider / scrollbar 的 focus gained、pointer value changed、keyboard step、final render 均完成验证。
- final command count 从 57 增加到 58，符合 focused outline 额外绘制命令的预期。

下一步：

- 为 value widget 增加可配置 step 属性，替换当前硬编码 step=5。
- 继续把 state style 逻辑收拢到 `wing_widget` 公共层。
- 规划真实 NuttX input adapter，将真实 touch/key 事件送入 `wing_gui_enqueue_input()`。

### 2026-06-12：Slider / Scrollbar Step 属性化

完成：

- `wing_slider_t` 增加 `step` 属性。
- `wing_scrollbar_t` 增加 `step` 属性。
- 新增 `wing_slider_set_step()` / `wing_slider_get_step()`。
- 新增 `wing_scrollbar_set_step()` / `wing_scrollbar_get_step()`。
- slider / scrollbar 键盘方向键步进从硬编码 `5` 改为读取组件自身 `step` 属性。
- `step=0` 会被规范化为 `1`，避免控件进入不可步进状态。
- `wing_gui_demo` 将 slider step 配置为 `7`，scrollbar step 配置为 `9`，用于验证 step 已经成为 widget property。

设计意义：

- value widget 的行为参数开始从事件处理代码中抽离出来，向可配置控件属性演进。
- 这更接近 LVGL / TouchGFX 中控件自身持有交互参数的模式。
- 后续 theme / style / builder / resource tools 可以继续基于这些属性生成 UI，而不需要修改事件代码。

本轮验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本轮验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: slider step=7 scrollbar step=9 are widget properties
wing_gui_demo: wing_slider focus gained and focused style is visible while focused
wing_gui_demo: wing_slider value changed from 83 to 90 by value input
wing_gui_demo: wing_slider handled key step key=1001
wing_gui_demo: wing_slider value changed from 90 to 83 by value input
wing_gui_demo: wing_slider handled key step key=1000
wing_gui_demo: wing_scrollbar focus gained and focused style is visible while focused
wing_gui_demo: wing_scrollbar value changed from 75 to 84 by value input
wing_gui_demo: wing_scrollbar handled key step key=1001
wing_gui_demo: wing_scrollbar value changed from 84 to 75 by value input
wing_gui_demo: wing_scrollbar handled key step key=1000
wing_gui_demo: frame tick=33 commands=58 checksum=0x4b8f67c5
wing_gui_demo: app task exit
```

说明：

- 构建通过。
- `wing_gui_demo` 运行通过。
- slider 按配置 step=7 完成 `83 -> 90 -> 83` 往返。
- scrollbar 按配置 step=9 完成 `75 -> 84 -> 75` 往返。
- 最终 checksum 保持 `0x4b8f67c5`，说明 step 往返后最终视觉状态保持稳定。

下一步：

- 继续把 focused / pressed / disabled 的状态样式选择收拢到 `wing_widget` 公共层。
- 开始准备真实 NuttX input adapter 的接口边界。
- 后续可把 step / page_size / padding / knob_size 纳入更系统的 widget property 模型。

### 2026-06-12：State Style 公共解析 helper

完成：

- `wing_widget` 新增 `wing_widget_select_state_style()`。
- `wing_widget` 新增 `wing_widget_store_state_style()`。
- `wing_box_get_active_style()` 改为复用公共 state-style 选择逻辑。
- `wing_box_set_state_style()` 改为复用公共 state-style 存储逻辑。
- `wing_box_draw()` 改为复用 `wing_widget_draw_style()`，不再手写 fill / stroke / clear 绘制分支。
- `wing_slider_t` focused overlay 改为复用公共 state-style 选择逻辑。
- `wing_scrollbar_t` focused overlay 改为复用公共 state-style 选择逻辑。

设计意义：

- `pressed / focused / disabled` 的优先级和选择规则开始从各个控件中抽离出来。
- button / panel 因为基于 `wing_box_t`，也自动复用新的公共 state-style 解析路径。
- slider / scrollbar 仍然只支持 focused overlay，但已经接入同一个公共选择入口，后续扩展 pressed / disabled 时不需要再各写一套判断。
- 这一步让 WING GUI 更接近“widget base + state selector”的方向，也更符合 LVGL 风格的状态样式模型。

本轮验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本轮验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: slider and scrollbar expose focused state visual feedback
wing_gui_demo: slider step=7 scrollbar step=9 are widget properties
wing_gui_demo: progress timer updated wing_progress value=90
wing_gui_demo: wing_button focus gained by WING runtime and state style active fill=219,107,73
wing_gui_demo: wing_button click event handled by WING widget and updated focused style
wing_gui_demo: card received bubbled click and stopped propagation
wing_gui_demo: wing_slider focus gained and focused style is visible while focused
wing_gui_demo: wing_slider value changed from 83 to 90 by value input
wing_gui_demo: wing_scrollbar focus gained and focused style is visible while focused
wing_gui_demo: wing_scrollbar value changed from 75 to 84 by value input
wing_gui_demo: frame tick=33 commands=58 checksum=0x4b8f67c5
wing_gui_demo: app task exit
```

说明：

- 构建通过。
- `wing_gui_demo` 运行通过。
- 状态样式公共 helper 迁移后，button focused style、slider focused overlay、scrollbar focused overlay 均保持有效。
- 输入、事件、冒泡、stop propagation、timer、animation、value changed、final render 均保持验证覆盖。
- 最终 checksum 保持 `0x4b8f67c5`，说明这次是内部结构收拢，没有改变最终视觉结果。

下一步：

- 准备真实 NuttX input adapter 的接口边界。
- 继续推进 widget property 模型，把 padding / knob_size / page_size 等配置统一整理。
- 让 `wing_gui_demo` 更明确地分段展示 WING GUI 当前全部能力。

### 2026-06-12：wing_gui_demo 能力展示分段输出

完成：

- `wing_gui_demo` 新增 stage 输出，当前包括 setup、synthetic input script、runtime capability summary。
- `wing_gui_demo` 新增 synthetic input 投递 helper。
- 每条 synthetic input 现在都会输出 label、type、point、key。
- demo 明确输出 surface size、command list capacity、frame interval。
- demo 明确输出 timer 调度信息。
- demo 明确输出 animation 调度信息。

设计意义：

- `wing_gui_demo` 开始从“隐式验证 demo”转为“能力展示台”。
- 输入不再只是悄悄塞进 queue，而是明确展示 pointer/key 事件如何进入 WING GUI。
- 后续加入真实 NuttX input adapter 后，可以继续对照 synthetic input 路径验证两条输入路径是否一致。
- 这更符合当前目标：`wing_gui_demo` 应该把 WING GUI 的全部能力都呈现出来。

本轮验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本轮验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: === setup runtime, surface and command list ===
wing_gui_demo: surface=320x240 command_capacity=64 frame_interval=33ms
wing_gui_demo: === synthetic input script ===
wing_gui_demo: input queued label=button pointer down type=pointer_down point=50,86 key=0
wing_gui_demo: input queued label=checkbox keyboard space type=key_down point=0,0 key=32
wing_gui_demo: input queued label=focus traversal tab type=key_down point=0,0 key=9
wing_gui_demo: input queued label=slider keyboard right type=key_down point=0,0 key=1001
wing_gui_demo: input queued label=scrollbar keyboard left type=key_down point=0,0 key=1000
wing_gui_demo: timer scheduled period=33ms repeat=0 target_progress=90
wing_gui_demo: animation scheduled property=line_width from=64 to=118 duration=132ms
wing_gui_demo: === runtime capability summary ===
wing_gui_demo: progress timer updated wing_progress value=90
wing_gui_demo: wing_button click event handled by WING widget and updated focused style
wing_gui_demo: wing_checkbox value changed from 0 to 1 by toggle input
wing_gui_demo: wing_switch value changed from 0 to 1 by toggle input
wing_gui_demo: wing_slider value changed from 83 to 90 by value input
wing_gui_demo: wing_scrollbar value changed from 75 to 84 by value input
wing_gui_demo: frame tick=33 commands=58 checksum=0x4b8f67c5
wing_gui_demo: app task exit
```

说明：

- 构建通过。
- `wing_gui_demo` 运行通过。
- setup / input script / runtime summary 三个展示阶段均出现。
- pointer input、keyboard input、focus traversal、timer、animation、value widget、event bubbling、state style、FRender command 输出仍保持验证覆盖。
- 最终 checksum 保持 `0x4b8f67c5`。

下一步：

- 准备真实 NuttX input adapter 的接口边界。
- 继续整理 `wing_gui_demo`，让各能力段落更接近独立测试章节。
- 后续补充 dirty rect / partial redraw 的显式展示日志。
