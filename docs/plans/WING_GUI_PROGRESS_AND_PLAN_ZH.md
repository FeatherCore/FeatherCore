# WING GUI 进度与计划

更新时间：2026-06-12

本文用于记录 WING GUI、FRender、WING Desktop 和 Pinion 相关工作的当前进度、下一步计划与阶段验收标准。

持续更新的执行进度与最新验证记录请参考：

- `FeatherCore/docs/progress/WING_GUI_PROGRESS_AND_UPDATE_PLAN_ZH.md`

Phase 1 运行时闭环验收清单请参考：

- `FeatherCore/docs/plans/WING_GUI_PHASE1_ACCEPTANCE_ZH.md`

架构原则请参考：

- `FeatherCore/docs/plans/WING_GUI_CORE_DESIGN_ROADMAP_ZH.md`
- `FeatherCore/docs/WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md`
- `FeatherCore/docs/WING_GUI_DESIGN_REFERENCES_ZH.md`
- `FeatherCore/docs/FRENDER_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/FEATHER_RENDER_CORE_DECISION_ZH.md`

## 1. 当前总体判断

当前方向已经明确：

```text
Independent WING GUI App
WING Desktop App
        |
        v
WING GUI
        |
        v
FRender
        |
        v
NuttX graphics
        |
        v
Hardware
```

其中：

- WING GUI 是 GUI 库。
- WING Desktop 是建立在 WING GUI 之上的默认桌面系统。
- WING GUI 应用不强制依赖 WING Desktop。
- FRender 是 WING GUI 和 Pinion 共享的渲染底座。
- NuttX graphics 提供 framebuffer / LCD / NX / DMA2D / GPU2D / input 等底层能力。

当前最重要的工作不是继续堆 Desktop，而是先把 WING GUI 的基础执行模型、对象树、事件、脏区和绘制路径做稳。

最新进展：

- WING GUI Core Phase 1 已开始实现。
- 已加入 `wing_obj_t`、object tree、draw callback、event callback 和 dirty rect 基础。
- `wing_gui_demo` 已开始从“手写绘制场景”转为“创建 object tree 后由 GUI runtime 遍历绘制”。
- 已加入第一个基础组件 `wing_box_t`，用于承载 fill / stroke / clear 这类最基础的矩形 UI 组件。
- 已加入第一个基础交互组件 `wing_button_t`，用于封装 focusable box、状态样式和用户事件回调。
- 已加入第一个基础文本组件 `wing_label_t`，当前通过 `wing_font_t` builtin 5x7 font resource 输出轻量 ASCII 文本。
- 已加入第一个基础图片组件 `wing_image_t`，当前接收静态 RGBA 像素资源并提交为 FRender blit command。
- 已加入第一个基础容器组件 `wing_panel_t`，用于封装可绘制 panel 和 layout 承载语义。
- 已加入 pointer input 雏形、object hit test、pressed state 和 click event 派发。
- 已加入 `wing_gui_t` 内部 input queue，输入可以先入队，再由 `wing_gui_step()` 统一处理。
- input reader / input queue / pointer-key dispatch 实现已从 `wing_gui.c` 拆到 `src/core/wing_input.c`，并新增 `include/wing/core/wing_input.h` 作为 input 模块头。
- focus traversal / focus state / focus event 实现已从 `wing_gui.c` 拆到 `src/core/wing_focus.c`，并新增 `include/wing/core/wing_focus.h` 作为 focus 模块头。
- pointer capture / release / cancel 实现已从 `wing_gui.c` 拆到 `src/core/wing_capture.c`，并新增 `include/wing/core/wing_capture.h` 作为 capture 模块头。
- 已加入 `wing_gui_t` 内部 event queue，input dispatch 会投递 object event，再由 `wing_gui_step()` 统一派发。
- event queue 的 post / dispatch / stop propagation 实现已从 `wing_gui.c` 拆到 `src/core/wing_event.c`，并新增 `include/wing/core/wing_event.h` 作为 event 模块头。
- 已加入 object event bubbling，事件可以从 target 向 parent 传播。
- 已加入 `wing_event_stop_propagation()`，对象可以停止事件继续向上冒泡。
- 已加入 `wing_gui_set_focus()`，pointer down 可以统一切换 focused object。
- 已加入 `WING_EVENT_FOCUS_GAINED` / `WING_EVENT_FOCUS_LOST`，焦点变化会通过 event queue 通知对象。
- 已加入 `wing_box_t` 状态样式雏形，基础矩形组件可以根据 pressed / focused / disabled 状态自动选择样式。
- 已加入键盘输入雏形，支持 `WING_INPUT_KEY_DOWN` / `WING_INPUT_KEY_UP`。
- 已加入 `wing_gui_focus_next()`，Tab / 方向键可以在 focusable object 之间切换焦点。
- 已加入 `WING_EVENT_KEY_DOWN` / `WING_EVENT_KEY_UP`，focused object 可以接收键盘事件。
- 已加入 Enter / Space 触发 focused object click 的基础路径。
- 已加入 `wing_gui_t` 内部软件 timer 池，`wing_gui_tick()` 可以推进 one-shot / repeat timer。
- `wing_gui_demo` 已通过 repeat timer 验证周期性回调，并在第 3 次 tick 中调用 `wing_gui_timer_stop()` 自停。
- timer 注册与停止实现已从 `wing_gui.c` 拆到 `src/core/wing_timer.c`，并新增 `include/wing/core/wing_timer.h` 作为模块化头文件边界的第一步。
- 已加入 `wing_gui_t` 内部 animation 池，`wing_gui_tick()` 可以推进线性插值动画。
- 已加入 `wing_gui_anim_start_path()` 和 `WING_ANIM_PATH_LINEAR/EASE_IN/EASE_OUT/EASE_IN_OUT`，animation runtime 已开始支持 easing/path。
- animation 启动与停止实现已从 `wing_gui.c` 拆到 `src/core/wing_animation.c`，并新增 `include/wing/core/wing_animation.h` 作为 animation 模块头。
- 已加入 `wing_gui_handle()`，应用可以周期性调用单一 GUI handler 推进 runtime。
- 已加入 `wing_gui_frame_t`，用于返回 handler 内部 frame 诊断信息。
- 已加入 `wing_gui_set_input_reader()` / `wing_gui_poll_input()`，handler 可以从输入 provider 拉取事件。
- 已加入最小 layout 雏形，支持 fixed / vertical stack / horizontal stack。
- 已加入 `WING_LAYOUT_CENTER`，父对象可以把可见子对象居中放置在自身 bounds 内。
- 已加入 `WING_LAYOUT_FILL`，父对象可以把可见子对象拉伸到自身 padding 后的内容区域。
- 已加入 `wing_theme_t` 和 `wing_theme_init_default()`，作为第一阶段 theme seed。
- 已加入 `wing_theme_init_high_contrast()`、`wing_gui_set_theme()` / `wing_gui_get_theme()`，作为运行时主题切换 seed。
- 已加入 `wing_value_model_t`，progress / slider / scrollbar 已共享 range / value / step 存储与更新路径。
- 已加入 `wing_widget_update_value()` / `wing_widget_update_bool()`，progress / slider / scrollbar / switch / checkbox 已共享 value update、invalidation 和 value changed event 派发路径。
- 已加入 dirty rect list 第一阶段，WING GUI runtime 可以记录多个 invalidated rect，并继续维护 union dirty 兼容现有渲染路径。
- dirty invalidation / dirty list / dirty merge / dirty clear 实现已从 `wing_gui.c` 拆到 `src/core/wing_dirty.c`，并新增 `include/wing/core/wing_dirty.h` 作为 dirty 模块头。
- 已加入 dirty rect merge policy 第一阶段，重叠或相邻 dirty rect 会合并，list 满时退化为 union dirty。
- 已加入 dirty-list chunk redraw 第一阶段，`wing_gui_step()` 可以按 dirty rect 分块设置 clip 并绘制 object tree。
- 已新增 object clip-children 第一阶段：`wing_obj_set_clip_children()` / `wing_obj_get_clip_children()` 可让父对象用自身 screen bounds 对子树绘制进行嵌套裁剪，支撑后续 scroll/list/window/content-area。
- 已加入 frame-level present rect list 第一阶段，`wing_gui_frame_t` 会记录本帧 render 前 union dirty rect 和 dirty-list rects，`wing_gui_demo` 的 framebuffer present 使用逐个 `fr_fb_presenter_present_rect()` + `FBIO_UPDATE` 提交 dirty-list 区域。
- render context API 与 dirty-list chunk redraw 执行路径已从 `wing_gui.c` 拆到 `src/core/wing_render.c`，并新增 `include/wing/core/wing_render.h` 作为 render 模块头。
- FRender software backend 的 `FR_CMD_CLEAR` 已遵守当前 clip stack，避免 dirty-list redraw 时局部 clear 误清全屏。
- WING render context 的 clip stack 已支持嵌套 push/pop，dirty chunk clip 与 object clip-children 可以叠加使用。
- `wing_gui_frame_t.redraw_count` 已加入，用于观察当前 frame 实际 redraw chunk 数。
- WING render 已新增 `wing_gui_flush_frender_software()`、`wing_gui_render_command_count()`、`wing_surface_checksum_rgba8888()` 薄封装；`wing_gui_demo` 不再直接执行 FRender software backend，而是通过 WING render 将当前 command list 交给 FRender，保持“WING 组织 UI 和绘制命令，FRender 负责后端执行”的边界。
- WING GUI 后续默认验证路径必须以 X11 framebuffer 窗口作为输入输出载体：输出先占满整个窗口，输入默认覆盖鼠标、键盘和触摸语义；`wing_gui_demo` 需要继续把 X11 窗口打开、输入捕获、关闭窗口回到 NSH 作为固定验证项。
- X11 framebuffer 输入第一阶段已接入：sim X11 framebuffer 暴露非阻塞鼠标/键盘/滚轮事件轮询，FRender framebuffer presenter 提供 `fr_fb_presenter_poll_input()`，`wing_gui_demo` 的 input reader 会先读取 X11 窗口真实输入，再回退到 demo synthetic input，保持自动验证和真实窗口交互同时可用。
- X11 framebuffer 输出第一阶段已接入窗口尺寸：`wing_gui_demo` 会先打开 framebuffer presenter，成功后用 `presenter.xres/yres` 创建 WING surface、root bounds 和默认 camera viewport；无 framebuffer 时继续使用配置中的 headless fallback 尺寸。
- `wing_gui_demo` 的默认 object space 展示已改为随当前 framebuffer 尺寸动态布局：space card / depth card 不再依赖旧 320x240 固定位置，synthetic input 会使用 WING core 实际投影出的 quad 中心点验证 projected hit-test 和同 z-index depth hit ordering。
- `wing_gui_demo` 的 FRender command list 容量已从 512 提升到 2048，用于支撑 640x480 X11 全窗口输出下的全能力展示、dirty chunk redraw、空间 quad、文本、图片和多 widget 同帧验证。
- WING render dirty planner 已新增运行时 ENOSPC fallback：dirty-list redraw 如果在实际生成 FRender command 时溢出，会中止当前 pass 并回退到 union dirty redraw 重新提交，避免把 dirty chunk 估算误差直接暴露为应用层 `wing_gui_handle()` 失败。
- WING input/event 循环已改为小步推进：`wing_gui_step()` 每处理一个 input 就立即 dispatch 其产生的事件，避免全能力 demo 的 synthetic input 与 X11 真实输入同帧堆积撑满 event queue；X11 input polling 也新增队列背压，input queue 满时停止继续读取而不是返回 `-ENOSPC`。
- `wing_gui_demo` 已增加 headless 多帧验证路径，无 framebuffer presenter 时固定运行 5 帧后退出。
- `wing_gui_demo` 已增加 dirty rect 可观测日志，可以展示 before tick / after tick before step / after step。
- `wing_gui_demo` 已增加 dirty list count 和 dirty merge count 可观测日志，可以展示 dirty list 在 tick/event/animation 后累积、合并，并在 render 后清空。
- `wing_gui_demo` 已增加 redraw chunk 可观测日志，可以展示 dirty-list chunk redraw 是否真实进入绘制路径。
- 已通过 headless 验证确认首帧 full dirty，后续 animation 触发局部 dirty，`wing_gui_step()` 渲染后 dirty 清空。
- 已通过 headless 验证确认 timer 和 animation 可以在多帧 runtime 中完整完成。
- 已通过 headless 验证确认运行时 theme switch 会触发 widget tree invalidation。
- 已通过 headless 验证确认 progress / slider / scrollbar 仍保持原有 value changed 事件输出，同时内部已经走统一 value model 和 widget base value dispatch。
- 已通过 headless 验证确认 switch / checkbox 仍保持 boolean value changed 事件输出，同时内部已经走 widget base boolean dispatch。
- 已通过 headless 验证确认 dirty list count 可以反映首帧、theme switch 和 animation 局部 invalidation，并在 handler 后清空。
- 已通过 headless 验证确认 dirty merge policy 会把首帧、theme switch 和 animation 的重叠 dirty rect 合并为更少的 dirty list entries。
- 已通过 headless 验证确认 dirty-list redraw chunk count 在有 dirty 时为 1 或 2，在无 dirty 时为 0。
- 已通过保留 `DISPLAY=:0` 的 sim 验证确认 `wing_gui_demo` 从 NSH 启动后可以进入 framebuffer / X11 路径。
- `wing_gui_demo` 已优化 framebuffer / X11 idle 等待关闭路径：无 dirty 时只输出一次等待关闭提示，不再无限刷屏。
- 已通过脚本化 X11 `WM_DELETE_WINDOW` 验证模拟窗口管理器点击关闭按钮，`wing_gui_demo` 可打印 `framebuffer window closed`、退出 demo 并返回 NSH。
- 已重新构建并实际从 NSH 执行 `wing_gui_demo` 验证 X11 framebuffer 路径：直接运行 `./FeatherCore/build/sim-wing` 会打开 X11 窗口；`env -u DISPLAY` 是 headless 验证路径，不会打开窗口。
- 已通过 slider / scrollbar 越界拖拽验证 pointer capture 第一阶段行为：pointer down 后，move/up 即使离开 widget bounds 也仍由 pressed object 接收。
- 已新增正式 pointer capture API：`wing_gui_capture_pointer()` / `wing_gui_release_pointer()` / `wing_gui_cancel_pointer_capture()` / `wing_gui_get_pointer_capture()`。
- 已新增 pointer capture lifecycle 事件：`WING_EVENT_POINTER_CAPTURED` / `WING_EVENT_POINTER_RELEASED` / `WING_EVENT_POINTER_CANCELLED`。
- `wing_gui_demo` 已通过 slider / scrollbar captured / released 日志验证 capture lifecycle event 进入 WING event queue。
- 已新增内部 `wing_widget_handle_pointer_lifecycle()`，开始把 TouchGFX 风格的 behavior / mixin 思路收敛进 widget base。
- switch / checkbox 已使用 pointer lifecycle helper 统一处理 pointer down / up / cancel 的 pressed state。
- slider / scrollbar 已支持 pointer cancel 清理 pressed state，并由 `wing_gui_demo` 的 capture cancel 脚本验证。
- 已新增 `wing_widget_state_style_t`，state style 的初始化、存储和按状态选择开始收敛到 widget base。
- box / button / slider / scrollbar 已迁移到统一 state style 容器，demo 的运行时 theme 切换也改为通过公开 widget API 更新状态样式。
- `wing_obj_set_flags()` 已同步 `WING_OBJ_FLAG_ENABLED` 与 `WING_OBJ_STATE_DISABLED`，enabled flag 变化会驱动 disabled state style。
- 已新增 `wing_obj_set_enabled()` / `wing_obj_is_enabled()`，应用可以通过公开 object API 切换 enabled 状态，不再需要直接操作 flags。
- `wing_gui_demo` 已通过 WING timer 在运行时禁用 button，并输出 active disabled style，用于验证 enabled flag / disabled state / state style selection 已串联。
- 已新增 `wing_obj_set_visible()` / `wing_obj_is_visible()`，应用可以通过公开 object API 切换 visible 状态，不再需要直接操作 flags。
- `wing_gui_demo` 已通过 WING timer 隐藏 toast 子树，并验证 visible flag 可以驱动 hit/draw skip 与 dirty redraw。
- 已新增 `wing_obj_destroy_tree()`，用于递归销毁动态 UI 子树。
- 已新增 `wing_tick` core 模块，`wing_gui_tick()` 不再留在 `wing_gui.c`，timer / animation 推进路径有了独立维护边界。
- 已新增 `wing/widgets/wing_widgets.h` 公开组件 API 聚合头，组件 API 不再全部直接塞在 `wing.h` 内。
- 已开始清理 widget 实现中的演示硬编码：slider track height、scrollbar min thumb length 等视觉尺寸进入可配置 API，并由 demo 显式设置。
- 已移除 `wing_gui_draw_demo_scene()` 历史遗留 API，WING render core 不再承载固定坐标、固定颜色的默认 demo 画面；演示视觉统一放到 `wing_gui_demo`、theme 或后续应用配置中。
- 已新增 `wing/core/wing_object.h`，object tree 相关公开 API 从 `wing.h` 的主声明区拆出；`wing.h` 保留为聚合入口。
- 已清理 `wing.h` 中过期的 `wing_gui_draw_demo_scene()` 声明，避免公共 API 暴露不存在的历史 demo scene。
- 已新增 `wing/core/wing_runtime.h`，GUI 执行句柄相关公开 API 从 `wing.h` 的主声明区拆出；应用仍可通过 `wing.h` 一站式包含。
- 已让 `wing.h` 聚合 `wing/core/wing_input.h`，input 相关公开 API 从 `wing.h` 主声明区拆出，后续真实 NuttX 输入接入可以围绕 input 模块继续扩展。
- 已让 `wing.h` 聚合 `wing/core/wing_event.h`，event queue、dispatch 和 stop propagation 公开 API 从 `wing.h` 主声明区拆出，后续消息/事件扩展有了更清晰入口。
- 已新增 `wing/core/wing_text_edit.h`，文本编辑模型从 text input widget 拆出；widget 负责 object/focus/render/selection highlight，core helper 负责固定 buffer、length、cursor、selection range 和编辑命令。
- 已开始将“demo 设计值”和“WING GUI 机制默认值”分离：slider 默认 knob size 改为基于控件高度推导，`wing_gui_demo` 集中声明控件几何、padding、动画、timer 和输入脚本常量。
- 已明确 WING GUI 需要原生默认 object space 核心能力：2D 是默认空间中的 identity transform 特殊状态，第一阶段目标不是完整 3D 游戏引擎，而是提供 `wing_card` / camera / viewport / space transform / perspective / quad render command seed，使桌面可以实现空间卡片、app switcher、图标和 preview。
- HoneyGUI 的 Lite3D camera 集成方式作为参考：3D model 持有 camera 和 viewport，应用在 global transform callback 中设置 camera/world，render push 阶段组合 camera matrix 与 world matrix，并在 face transform 中完成 perspective 和 screen 映射。
- 已开始落地 WING core space seed：object 自带默认 identity `space_transform`，普通 2D 控件无需额外代码即可处于默认 3D 状态；`wing_card_t` 使用 camera/space transform/project API 生成 projected quad，并通过 FRender `FR_CMD_FILL_QUAD` 呈现，后续可升级到 FRender mesh / shader / GPU 3D path。
- `wing_gui_t` 已持有默认 runtime camera，并提供 `wing_gui_set_camera()` / `wing_gui_get_camera()`；object screen bounds 优先使用绑定 GUI 的 runtime camera，未绑定 GUI 时保留 fallback camera。
- `wing_gui_set_camera()` 已新增 `WING_EVENT_CAMERA_CHANGED` / `wing_camera_event_t` 可观察路径，并在 camera 改变前后触发全局 redraw，使默认 object space 的 camera 更新不再只是静态配置。
- `wing_camera_equal()` 已进入 core space API，runtime camera 的 no-op 过滤由默认空间层统一判断。
- 已新增 `wing_obj_space_transform_is_identity()` / `wing_obj_reset_space_transform()`，并在 `wing_gui_demo` 中覆盖 reset 后 identity 查询，将普通 2D object 的默认空间状态固化为可查询、可重置的公开 object API。
- 已新增 `wing_space_transform_is_identity()` core space API，object 的 identity 查询改为复用 core space 判断，避免普通 2D 默认状态的判断散落在 object 私有逻辑中。
- 已新增 `wing_space_transform_compose()` / `wing_obj_get_world_space_transform()`，让 object space transform 从单对象状态推进到 parent chain world transform 组合语义。
- 已新增 `wing_space_transform_apply_point()` core space API，将局部点经过 scale / rotation / translation 进入默认空间的过程独立出来；`wing_project_point()` 只继续负责 camera / perspective 映射。
- 已新增 `wing_project_point_with_depth()` core space API，让默认空间投影可以同时返回 screen point 与 projected depth；后续 mesh / shader / GPU 3D backend、picking 和排序不需要重新实现 camera depth 计算。
- 已新增 `wing_project_rect_projected_quad()` core space API，矩形平面投影可以返回 4 个 screen/depth 顶点；旧 `wing_project_rect_quad()` 保持 2D 兼容视图并复用该 richer path。
- 已新增 `wing_projected_quad_average_depth()` core space API，为 projected quad 提供统一代表 depth，后续 render planner / picking / 空间排序不需要重复平均四角 depth。
- 已新增 `wing_obj_compare_space_order()` object API，并让 object tree draw/hit traversal 在同一 z-index 组内使用 projected average depth 细排；z-index 继续作为显式层级优先级，projected depth 作为默认 3D 空间中的前后关系基础。
- `wing_gui_demo` 已新增两个同 z-index、不同 projected depth 的重叠 card，用于实际验证默认空间中远处对象先绘制、近处对象优先命中的排序规则。
- WING GUI 默认 input queue seed 已从 32 提升到 64，用于支撑第一阶段 demo 的密集输入脚本；该值仍保留 `CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE` 可配置入口，sim 构建脚本也已同步使用 64，避免每次重新构建时覆盖回旧值。
- 已新增 `wing_obj_get_screen_bounds()`，identity transform 下保持 2D bounds 不变，非 identity transform 下计算投影 screen bounds，并接入 object invalidate、bounds/transform 更新、dirty culling 和 hit-test 的第一阶段路径。
- `wing_obj_set_bounds()` 的旧/新 redraw bounds 已统一走 object screen bounds 路径，旧 bounds invalidation 会保留 parent chain world space transform 语义，避免父对象带空间变换时只按本地 transform 估算旧区域。
- 已新增 `wing_obj_project_quad()` object API，object 可以直接产出带 screen/depth 顶点的 projected quad；旧 `wing_obj_project_quad2d()` 保持兼容并复用该 richer object path。
- 已新增 `wing_obj_get_projected_depth()` object API，object 可直接返回 projected average depth，为后续 z-index + depth 的空间排序策略打底。
- 已新增 `wing_obj_project_quad2d()`，让 object world transform + runtime camera 可以直接产出 projected quad，供 widget 绘制和后续 FRender 3D primitive 共享。
- 已新增 `wing_obj_contains_point()`，identity object 走普通 2D rect 命中，非 identity object 走 projected quad 命中；`wing_obj_hit_test()` 已复用该路径，使空间控件交互不再只依赖轴对齐包围盒。
- 已新增 `wing_quad2d_contains_point()` core space API，projected quad 精确命中从 object 私有逻辑上移到默认空间几何层；`wing_obj_contains_point()` 继续保留 object 入口但复用 core space 判断。
- `wing_card_t` 绘制已切到 `wing_obj_project_quad2d()`，不再自行临时创建 camera / 拼 object transform，dirty screen bounds 与实际 card quad 绘制开始共享同一条 projection path。
- `wing_card_t` 已移除 card 私有 camera / transform API，空间状态统一由 `wing_obj_t` 的 transform 和 `wing_gui_t` 的 runtime camera 管理，避免把默认 3D 能力误建模成某个独立 3D view/widget 的私有能力。
- `wing_gui_demo` 已开始通过 `wing_obj_set_space_transform()` 驱动 space card 动画，验证空间变换属于 `wing_obj_t` 默认能力；`wing_card_t` 只是当前用于展示 projected quad 的普通 widget。
- 已新增 object z-index/layer seed：`wing_obj_set_z_index()` / `wing_obj_get_z_index()`，object tree 的 draw/hit traversal 会按 sibling z-index 分组执行；默认 0 保持原行为，较高 z-index 在默认空间中更靠前显示并优先接收命中。
- 已新增 style opacity seed：`wing_box_style_t.opacity` 默认 255，公共 style 绘制和 `wing_card_t` quad/edge 绘制都会将 opacity 乘入 RGBA alpha；FRender software fallback 已支持 source-over alpha blend，为 toast、浮层、窗口阴影和空间卡片透明效果打底。
- 已新增 object tree 继承透明度 seed：`wing_obj_t.opacity` 默认 255，`wing_obj_set_opacity()` 会触发子树重绘，`wing_obj_get_effective_opacity()` 会沿 parent chain 组合父子透明度。box / progress / slider / scrollbar / switch / checkbox / card / label 的颜色型绘制已经通过 `wing_widget_style_color_for_obj()` 同时消费 style opacity 和 object effective opacity。
- 已新增 pointer hover seed：runtime 维护 `hovered_obj`，pointer enter/leave 事件会驱动 `WING_OBJ_STATE_HOVERED`，state-style 公共 helper 也支持 hovered 样式；这为桌面图标、窗口控件和鼠标环境下的可视反馈打底。
- 已新增 checked state seed：switch / checkbox 的 boolean value 会同步进入 `WING_OBJ_STATE_CHECKED`，value changed 回调可观察 object state 已经更新；state-style 公共 helper 也开始支持 checked 样式槽。
- 已新增 encoder rotate input seed：`WING_INPUT_ENCODER_ROTATE` 会派发为 `WING_EVENT_ENCODER_ROTATE` 给 focused object，slider / scrollbar 复用 value model step 响应旋钮输入，贴近 LVGL 在 MCU/RTOS 上的 encoder indev 模型。
- 已新增 selected / active state seed：`wing_obj_set_selected()` / `wing_obj_is_selected()` / `wing_obj_set_active()` / `wing_obj_is_active()` 进入 object API，state-style 公共 helper 支持 selected / active 样式槽，为桌面图标选中、菜单项选中和窗口激活状态打底。
- 已新增 state changed event seed：`WING_EVENT_STATE_CHANGED` 会携带 `wing_state_event_t`，报告 object state 的 old/new/changed bitmask；`wing_obj_set_state()` 以及 enabled flag 引起的 disabled state 变化都会同步派发该事件。
- 已新增 object property changed event seed：`WING_EVENT_BOUNDS_CHANGED` 携带 `wing_bounds_event_t`，`WING_EVENT_SPACE_TRANSFORM_CHANGED` 携带 `wing_space_transform_event_t`，用于观察 layout / animation / object space transform 等核心属性变化。
- 已新增 `wing_space_transform_equal()` core space API，`wing_obj_set_space_transform()` 使用该入口过滤 no-op transform，space transform changed event 的比较语义不再由 object 私有函数维护。
- 已移除旧 `WING_EVENT_TRANSFORM_CHANGED` / `wing_transform_event_t` 兼容别名，默认空间事件只保留 `WING_EVENT_SPACE_TRANSFORM_CHANGED` / `wing_space_transform_event_t`。
- 已新增 app/window close request seed：`wing_gui_request_close()` 会把 `WING_INPUT_CLOSE_REQUEST` 放入 input queue，runtime 派发 `WING_EVENT_CLOSE_REQUEST` 后默认请求停止，真实 framebuffer/X11 窗口关闭不再绕过 WING input/event queue。
- 已确认 WING 不保留独立 `wing_3d.c` / `wing_3d_view.c` 路径；默认空间能力统一收敛在 `src/core/wing_space.c`、`wing_obj_t` transform、`wing_gui_t` camera 和普通 widget 绘制路径中。
- 已新增 object space transform 命名入口：`wing_space_transform_t` 与 `wing_obj_set_space_transform()` 等 API。旧 `transform3d` 入口不再保留兼容，新 demo 和后续 WING GUI 代码统一使用 `space transform` 表达默认空间能力，避免把 3D UI 误建模成独立扩展。
- `wing_obj_t` 内部字段已从历史 `transform3d` 收敛为 `space_transform`，旧 `wing_obj_*transform3d*` API 已移除；WING core 自身继续把默认 2D/3D 统一表达为 object space。
- 底层 transform 结构主 tag 已从历史 3D 命名收敛为 `wing_space_transform_s`；旧 `wing_transform3d_t` typedef 已移除，避免继续把默认空间能力暴露成外挂式 3D 扩展。
- 已新增 core space transform 命名入口：`wing_space_transform_init()` / `wing_space_transform_is_identity()` / `wing_space_transform_equal()` / `wing_space_transform_compose()` / `wing_space_transform_apply_point()`。旧 `wing_transform3d_*` 兼容包装层已移除，后续新实现只使用 `wing_space_transform_*` 表达默认空间能力。
- `wing_space_transform_*` 已成为 core space transform 的唯一实现入口，避免默认空间能力继续被实现层建模为外置 3D 扩展。
- core space 投影路径的函数定义和 identity fallback 已收敛为 `wing_space_transform_t` / `wing_space_transform_*`，投影、dirty bounds 和 picking 不再在实现层依赖旧历史命名。
- 已新增 `wing_scroll_view_t` 第一阶段：复用 object clip-children 作为 viewport，通过 `wing_scroll_view_set_offset()` 移动内容子对象，为 list、window content area 和桌面滚动区域打底。
- `wing_scroll_view_t` 已新增可聚焦输入路径：通过 content size / step 限制 offset 范围，并消费 focused key / encoder 事件移动 viewport 内容。
- 已新增 `WING_EVENT_SCROLL_CHANGED` 和 `wing_scroll_event_t`，scroll view offset 改变可被应用同步观察，为列表、窗口内容区和桌面滚动区域的状态联动打底。
- `wing_scroll_view_t` 已新增 `wing_scroll_view_scroll_by()` 和 `wing_scroll_view_get_max_offset()`，上层应用可以通过公开 API 执行相对滚动和查询 clamp 边界，不需要了解内部 offset 计算。
- 已新增 `wing_project_rect_quad()`，把“2D widget bounds 作为默认空间平面投影成 quad”的能力提升到 core space API；object screen bounds / card 绘制 / 后续 mesh 或 shader 路径可以共享同一套投影入口。
- 已新增 `wing_quad2d_get_bounds()` / `wing_project_rect_bounds()`，把 projected quad 到保守 screen dirty bounds 的计算收敛到 core space，object dirty culling 不再私有维护空间投影包围盒逻辑。
- `wing_obj_set_space_transform()` 已按子树触发旧/新 screen bounds invalidation，父对象 space transform 改变时不会只重绘父对象自身。
- `wing_gui_demo` 已输出 runtime camera、root identity screen bounds、space card projected dirty bounds、space card projected quad 和 space card world transform，让默认空间 camera / bounds / quad / world transform 行为可在 NSH 日志中直接验证。
- `wing_gui_demo` 已通过临时调整并恢复 runtime camera focal length 验证 camera changed event 和默认 object space redraw 路径，并输出 `wing_camera_equal()` 的 restored 检查结果；同时通过 no-op camera update 事件计数保持不变验证冗余 camera 更新会被 core space equality 过滤。
- `wing_card_t` 的实现命名已从历史 `view` 收敛为 `card`，用于强调它不是独立 3D view 层，而是 WING GUI 默认空间体系中的普通组件。
- 已新增 FRender quad 图元种子：`FR_CMD_FILL_QUAD` / `FR_DRAW_CAP_FILL_QUAD`。WING card widget 把投影后的四边形面提交给 Render backend，由 FRender software fallback 当前执行，后续由 GPU2D/GPU3D/mesh/shader 后端接管。
- 已新增 FRender triangle 图元种子：`FR_CMD_FILL_TRIANGLE` / `FR_CAP_FILL_TRIANGLE` / `FR_DRAW_CAP_FILL_TRIANGLE`。三角形作为未来 mesh / shader / GPU 3D path 的最小图元，当前由 FRender software fallback 执行扫描线填充，后续可由硬件后端接管。
- 已新增 WING render triangle frontend seed：`wing_triangle2d_t` / `wing_gui_fill_triangle()`。WING object tree 可以通过普通 custom object draw callback 提交 triangle primitive 到 FRender，不需要新增独立 3D view/widget。
- `frender_demo` 已新增 `fill_quad` stage，用于从 NSH 直接验证 FRender quad command、software fallback 和 framebuffer present 链路。
- `frender_demo` 已新增 `fill_triangle` stage，用于从 NSH 直接验证 FRender triangle command、software fallback 和 framebuffer present 链路。
- 已新增 FRender blit 图元种子：`FR_CMD_BLIT` / `FR_CAP_BLIT`。WING image widget 把静态 RGBA image resource 提交给统一 Render backend，由 FRender software fallback 当前执行 nearest-neighbor scale 和 per-pixel alpha blend。
- FRender 已新增 `fr_cmd_blit_alpha()` / command `global_alpha`，WING 已新增 `wing_gui_blit_alpha()`；`wing_image_t` 现在会使用 object effective opacity 执行 global-alpha blit，`wing_gui_demo` 已设置 image opacity=196 验证图片资源也进入统一透明度继承路径。
- 已新增 FRender image quad 图元种子：`FR_CMD_BLIT_QUAD` / `FR_CAP_BLIT_QUAD` / `FR_DRAW_CAP_BLIT_QUAD`。当普通 image widget 设置非 identity object space transform 时，WING 会把 image resource 作为 textured quad 提交给 FRender；当前 software fallback 使用保守 scanline sampling，后续 mesh / shader / GPU 3D backend 可以接管同一命令。
- WING render 已新增 `wing_gui_blit_quad_alpha()`，`wing_image_t` 会在 object space 非 identity 时走 image quad path，identity 状态继续保留普通 blit fast path；这进一步落实“2D 是默认 3D/object space 的特殊状态”，而不是新增独立 `wing_3d_view`。
- `wing_gui_demo` 已让 image widget 设置 `rotation_y=-12`、`z=10`，并在 NSH 日志中输出 `FRender image quad command when object space is non-identity`，用于验证图片资源也进入默认 object space + FRender command 链路。
- 2026-06-12 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证：X11 framebuffer 窗口打开、timer / repeat timer / animation / lifecycle / dirty-list redraw / present rect / projected hit-test / image quad path 正常；随后通过 X11 脚本验证 mouse move、pointer down/up、keyboard Right 和 close request 均进入 WING input/event queue，关闭窗口后 demo 退出并返回 NSH。
- 2026-06-12 已重新构建 `sim-wing` 并从 NSH 执行 `frender_demo` 验证：software caps 打印包含 `fill_triangle`，`stage=fill_triangle` 成功生成 checksum，framebuffer present 成功，关闭 X11 窗口后返回 NSH；随后再次执行 `wing_gui_demo` 回归验证 WING GUI runtime、X11 输入和 close request 均正常。
- 2026-06-12 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证 WING triangle frontend：日志输出 `triangle primitive object uses WING render frontend -> FRender fill_triangle command seed`，首帧 command count 包含新增 triangle command；X11 mouse/click、keyboard Right 和 close request 均验证通过。
- `frender_demo` 已新增 `blit` stage，用于从 NSH 直接验证 FRender blit command、software fallback 和 framebuffer present 链路。
- object 首次绑定 GUI runtime 时会触发 `WING_EVENT_CREATE`，`wing_obj_destroy_tree()` 会对子树触发 `WING_EVENT_DELETE`。
- `wing_gui_demo` 已通过 WING timer 动态挂载 toast 子树，隐藏后再通过 timer 销毁 toast 子树，用于验证 create/delete lifecycle、动态 object tree、visibility 和 dirty redraw。

## 2. 已完成内容

### 2.1 FRender 第一阶段

当前 FRender 已经具备第一版可用基础。

已完成：

- command list 基础结构。
- software backend。
- fill quad command 与 software fallback。
- fill triangle command 与 software fallback。
- blit command 与 software fallback。
- image quad blit command 与第一阶段 software fallback。
- backend capability 声明。
- backend registry。
- NuttX graphics capability adapter。
- framebuffer present adapter。
- `frender_demo` 可从 NSH 执行。
- `frender_demo` 可以展示当前支持的绘制能力。
- `frender_demo` 可以通过 framebuffer / X11 simulator 窗口显示。
- 点击关闭窗口后可以退出 demo 并返回 NSH。

当前定位：

```text
FRender 负责组织绘制命令、声明后端能力、执行软件 fallback、对接 framebuffer present。
```

当前限制：

- NuttX graphics 还不是 command-list submit target。
- DMA2D / GPU2D 加速路径还没有真正接入 FRender planner。
- 文本、图片、路径、渐变、复杂 clip 等命令还没有完善。
- backend cost policy 还没有形成。

### 2.2 WING GUI 种子实现

当前 WING GUI 已经从“demo 自己画”转向“GUI runtime 调度绘制”的方向。

已完成：

- `wing_gui_t` 作为 GUI 执行句柄的初始实现。
- `wing_obj_t` 作为 object tree 基类的初始实现。
- `wing_gui_tick()`。
- `wing_gui_step()`。
- `wing_gui_request_stop()`。
- `wing_gui_is_running()`。
- `wing_gui_set_root()`。
- `wing_obj_add_child()` / `wing_obj_remove_child()`。
- `wing_obj_invalidate()`。
- dirty rect 基础记录。
- dirty rect list 第一阶段记录。
- dirty invalidation / dirty list / dirty merge / dirty clear 已具备独立 core 模块实现。
- dirty rect merge policy 第一阶段记录。
- dirty-list chunk redraw 第一阶段。
- render context API 与 dirty-list chunk redraw 执行路径已具备独立 core 模块实现。
- object draw traversal。
- object event callback 入口。
- `wing_box_t` 基础组件。
- `wing_button_t` 基础组件。
- `wing_label_t` 基础组件。
- `wing_image_t` 静态 RGBA 资源组件第一阶段。
- `wing_panel_t` 基础组件。
- `wing_obj_hit_test()`。
- `wing_gui_enqueue_input()`。
- `wing_gui_dispatch_input()`。
- `wing_gui_step()` 处理 input queue。
- `wing_gui_post_event()` / `wing_gui_dispatch_events()`。
- `wing_gui_step()` 处理 event queue。
- `wing_obj_bubble_event()`。
- `wing_event_stop_propagation()` / `wing_event_is_stopped()`。
- `wing_gui_set_focus()`。
- `wing_gui_timer_start()` / `wing_gui_timer_stop()`。
- `wing_gui_tick()` 推进软件 timer。
- `wing_gui_anim_start()` / `wing_gui_anim_stop()`。
- `wing_gui_tick()` 推进线性 animation。
- `wing_obj_set_layout()`。
- `wing_obj_layout_tree()`。
- `wing_gui_step()` 在绘制前执行 layout。
- `wing_gui_step()` 现在按 layout -> input queue -> event queue -> render 的顺序推进，hit test 使用布局后的对象坐标。
- `wing_value_model_t`。
- progress / slider / scrollbar 共享统一 value model。
- `wing_widget_handle_pointer_lifecycle()`。
- `wing_widget_state_style_t`。
- switch / checkbox 共享 pointer down / up / cancel pressed-state lifecycle helper。
- box / button / slider / scrollbar 共享 widget base state style 存储和状态选择路径。
- object enabled flag 与 disabled state 已同步，disabled style 可以通过普通 `wing_obj_set_flags()` 触发。
- object enabled 状态已有公开 API：`wing_obj_set_enabled()` / `wing_obj_is_enabled()`。
- `wing_gui_get_dirty_rect_count()` / `wing_gui_get_dirty_rect_at()`。
- `wing_gui_get_dirty_merge_count()`。
- `wing_gui_frame_t.redraw_count`。
- pointer down / move / up 基础事件。
- pointer capture / release / cancel 第一版 API。
- pointer capture lifecycle 事件。
- focus API 与 pointer capture API 已具备独立 core 模块实现，`wing_gui.c` 不再直接承载这两类专属运行时能力。
- click event 基础派发。
- click event 支持 object bubbling。
- object event callback 可以停止传播。
- focus gained / focus lost 基础事件。
- pointer down 会切换 focused object 并更新 `WING_OBJ_STATE_FOCUSED`。
- `wing_gui_demo` 可从 NSH 执行。
- `wing_gui_demo` 通过 FRender command list 生成绘制内容。
- `wing_gui_demo` 可以 present 到 framebuffer / simulator 窗口。
- demo 循环可以按固定 tick 间隔推进。
- `wing_gui_create()` 已允许不传手写 render callback，支持纯 object tree 渲染路径。
- demo 已使用 `wing_box_t` 创建 root、header、progress object。
- demo 已使用 `wing_panel_t` 创建 card container，并承载 horizontal stack layout。
- demo 已使用 `wing_button_t` 创建 button object，不再把普通 box 当作按钮使用。
- demo 已使用 `wing_label_t` 绘制 `GO` 文本，并验证 label 可以绘制在 button 上方且不参与 input hit-test。
- demo 已通过一次 synthetic pointer click 入队验证 button-like object 的 event callback 与 invalidation 路径。
- demo 已通过 input queue -> event queue -> object callback 验证事件统一派发路径。
- demo 已通过 button -> card 的 click 冒泡验证 event bubbling。
- demo 已通过 card 停止传播验证 stop propagation，root 不应收到该 click。
- demo 已通过 synthetic pointer down 验证 button focus gained 事件。
- demo 已通过 focused state style 验证对象状态可以驱动组件绘制样式。
- demo 已通过 synthetic Tab / Enter 验证键盘输入可以走 focus traversal 和 focused object click 路径。
- demo 中 `wing_button_t` 默认具备 `WING_OBJ_FLAG_FOCUSABLE`，用于验证 focus traversal 不再依赖 pointer hit target。
- demo 已通过 one-shot timer 验证 tick-driven runtime 可以修改 object bounds 并触发 invalidation。
- demo 已通过 repeat timer 验证 tick-driven runtime 可以周期性修改 object style，并可在 timer callback 内调用 `wing_gui_timer_stop()` 停止自身。
- demo 已通过 width animation 验证 tick-driven runtime 可以按时间插值更新 object bounds。
- demo 已通过 ease-out width animation 验证 `wing_gui_anim_start_path()` 可以驱动非线性动画路径。
- demo 已使用 horizontal stack layout 让 card 自动摆放部分子对象。
- demo 已使用 center layout 让 button 自动居中摆放 `GO` label 子对象，验证子对象不再依赖 root 级绝对坐标。
- demo 已通过 headless 5 帧验证展示 timer、animation、dirty/redraw 生命周期。
- demo 已打印每帧 dirty before tick / after tick before step / after step，用于证明局部 invalidation 与 render 后清理行为。
- demo 主循环已切换为调用 `wing_gui_handle()`，并通过 `wing_gui_frame_t` 打印 dirty before handler / after tick before step / after handler。
- demo 已注册 input provider，并验证 provider key event 进入 scrollbar value changed / key handled 路径。
- demo 已改为通过默认主题初始化主要组件样式，并输出 default theme initialized 日志。

当前定位：

```text
wing_gui_demo 只是应用模板。
真正的 GUI 状态推进应该由 wing_gui_t 负责。
```

当前限制：

- 公共 API 仍以 `<wing/wing.h>` 聚合头为主，后续需要继续拆分为 core / runtime / input / event / animation / widgets 等模块头。
- 还没有正式 widget base，`wing_button_t` 目前是直接组合 `wing_box_t` 的第一版交互组件。
- `wing_label_t` 目前已接入轻量 `wing_font_t` / builtin 5x7 font resource、文本测量、水平对齐、最小 UTF-8 codepoint 解码、显式换行、bounds 内空格优先自动换行和单行 ellipsis 截断；还没有字体转换器、字距、复杂脚本和 FRender text command。
- `wing_image_t` 目前已从裸像素指针推进到轻量 `wing_image_resource_t` 描述层，仍支持旧 `wing_image_init(... pixels ...)` 兼容入口；image 已下沉到 FRender blit command，并支持 object effective opacity -> blit global alpha。后续还需要接入图片资源转换工具、texture cache 和硬件 blit planner。
- `wing_panel_t` 目前是直接组合 `wing_box_t` 的第一版容器组件，还没有独立 container base / child policy。
- 还没有完整 event dispatch。
- 还没有接入真实 NuttX touch / mouse / keyboard 输入设备。
- keyboard input 目前还是 synthetic demo 输入，还没有接入真实设备和 key map。
- input provider 抽象已经存在，但真实 NuttX touch / mouse / keyboard provider 尚未实现。
- layout 目前已有 fixed / vertical stack / horizontal stack / center，还没有 flex / grid / anchor。
- animation 目前只有线性插值，还没有 easing / timeline / property binding。
- 还没有 animation list。
- dirty rect list / merge policy 仍是第一阶段版本。
- 当前 dirty/redraw 已支持 dirty-list chunk redraw，并具备 dirty-list 多区域 present 提交；但还没有 tile scheduler、cost policy、command capacity fallback 和 tile present。
- `wing_gui_handle()` 当前仍是同步 handler，尚未接入真实输入设备 poll、present 策略或异步调度。
- 当前 handler 已支持通用 input provider poll，但 provider 仍是 demo 级模拟输入。
- pointer capture 已有第一版单指针语义，但还没有 multi-pointer、lost capture、设备断开和复杂 cancel 策略。
- 还没有完整 style selector / theme / inherited style，当前只有 `wing_box_t` 的状态样式种子实现。
- 已有默认 theme seed，但还没有 inherited style、theme switching 或 selector cascade。
- `wing_box_t` 仍然只是基础组件，还不是完整 widget / style / layout 体系。

### 2.3 架构文档

已完成：

- WING GUI 与 WING Desktop 边界澄清。
- FRender 作为渲染底座的决策文档。
- NuttX graphics 能力梳理。
- apps/graphics 当前内容梳理。
- WING GUI 参考 LVGL / TouchGFX / HoneyGUI 的设计提取文档。
- Pinion 实现方向文档。

当前限制：

- 架构文档和实现进度曾经混在一起。
- 需要持续维护本文作为唯一进度入口。

## 3. 当前代码主线

### 3.1 FRender

位置：

```text
FeatherCore/apps/graphics/frender
FeatherCore/apps/examples/frender_demo
```

职责：

- command list。
- render capability。
- software fallback。
- framebuffer present。
- 未来硬件加速 planner。

### 3.2 WING GUI

位置：

```text
FeatherCore/apps/graphics/wing
FeatherCore/apps/examples/wing_gui_demo
```

职责：

- GUI runtime。
- object tree。
- widget。
- event。
- input。
- style。
- layout。
- dirty region。
- animation。
- render frontend。

### 3.3 WING Desktop

计划位置：

```text
FeatherCore/apps/graphics/wing_desktop
FeatherCore/apps/examples/wing_desktop_demo
```

职责：

- 默认桌面系统。
- app container。
- window manager。
- launcher。
- taskbar。
- package registry。

当前状态：

- 暂不优先实现。
- 等 WING GUI 第一阶段稳定后再启动。

### 3.4 Pinion

计划位置：

```text
FeatherCore/apps/graphics/pinion
FeatherCore/apps/examples/pinion_demo
```

职责：

- 基于 FRender 的轻量游戏引擎。
- scene。
- sprite。
- input。
- update loop。
- animation。
- collision。

当前状态：

- 暂不优先实现。
- 等 FRender 和 WING GUI 基础稳定后再推进。

## 4. 第一阶段目标：WING GUI Core

第一阶段目标是让 WING GUI 从“能画 demo”变成“真正的 GUI 库雏形”。

必须实现：

- `wing_obj_t`。
- object tree。
- object bounds。
- object flags。
- object state。
- child append / remove。
- draw traversal。
- event callback。
- invalidation。
- dirty rect 基础。
- root object。
- demo 使用 object API 创建 UI。

第一阶段不追求：

- 完整 style。
- 完整 layout。
- 完整 widget set。
- 复杂动画。
- 多窗口 Desktop。
- 3D 桌面效果。

## 5. 第一阶段推荐任务拆分

### Task 1：定义 object 基类

新增或整理：

```text
apps/graphics/wing/include/wing/wing_object.h
apps/graphics/wing/src/core/wing_object.c
```

目标：

- 定义 `wing_obj_t`。
- 支持 parent / child / sibling。
- 支持 bounds。
- 支持 visible / enabled / dirty flags。
- 支持 user data。

验收：

- demo 可以创建 root、panel、button-like object。
- object 可以形成树。

### Task 2：定义 draw traversal

新增或整理：

```text
apps/graphics/wing/src/core/wing_draw.c
```

目标：

- 从 root 遍历 object tree。
- 每个 object 调用 draw callback。
- draw callback 只输出 FRender command。
- 支持基础 clip。

验收：

- demo 不再直接手写完整场景。
- demo 通过 object tree 生成同等画面。

### Task 3：定义 invalidation

新增或整理：

```text
apps/graphics/wing/src/core/wing_invalidate.c
```

目标：

- `wing_obj_invalidate(obj)`。
- object bounds 转 dirty rect。
- GUI runtime 持有 dirty state。
- 没有 dirty 时 `wing_gui_step()` 不重绘。

验收：

- 修改 object 状态后触发重绘。
- 没有状态变化时不重复绘制。

### Task 4：定义基础事件

新增或整理：

```text
apps/graphics/wing/include/wing/wing_event.h
apps/graphics/wing/src/core/wing_event.c
```

目标：

- 定义 `wing_event_t`。
- 支持 create / delete / draw / click / value changed 等基础事件。
- 支持 event callback。

验收：

- demo 中 button-like object 可以响应一个模拟事件或简单输入事件。

### Task 5：整理 demo 模板

更新：

```text
apps/examples/wing_gui_demo
```

目标：

- demo 展示标准 WING GUI App 写法。
- 创建 `wing_gui_t`。
- 创建 root object。
- 创建几个基础 object。
- 主循环只调用 `wing_gui_handle()`，present 仍由应用或上层 shell 控制。

验收：

- demo 不再承担 GUI runtime 逻辑。
- demo 可作为以后 WING GUI App 模板。

## 6. 第二阶段目标：Style / Layout / Input

第二阶段目标是让 WING GUI 具备真实 UI 框架的基础能力。

计划实现：

- `wing_style_t`。
- background / border / padding / text color。
- normal / pressed / focused / disabled state。
- fixed layout。
- vertical stack。
- horizontal stack。
- center layout。
- pointer input。
- keyboard input。
- hit test。
- focus。
- event bubbling。

验收：

- demo 可以创建一个 panel。
- panel 中可以自动布局 label / button，button 可以自动居中摆放内部 label。
- pointer 或键盘事件可以改变 button 状态。

## 7. 第三阶段目标：Timer / Animation / Dirty Region

第三阶段目标是让 GUI runtime 自己推进动态 UI。

计划实现：

- `wing_timer_t`。
- `wing_anim_t`。
- easing。
- dirty rect list。
- dirty rect merge。
- clipped rendering。
- partial present 准备。

验收：

- demo 可以实现一个自动动画。
- 动画不由 demo 手动重画，而由 WING GUI runtime 推进。
- dirty region 可以输出日志或调试信息。

## 8. 第四阶段目标：基础 Widget

计划实现：

- label。
- button。
- panel。
- image。
- progress。
- slider。

目录建议：

```text
apps/graphics/wing/src/widgets
apps/graphics/wing/include/wing/widgets
```

验收：

- 每个 widget 有最小 demo。
- `wing_gui_demo` 可以组合多个 widget。

## 9. 第五阶段目标：WING Desktop POC

前置条件：

- WING GUI Core 稳定。
- 基础 widget 可用。
- input / focus / dirty redraw 可用。

计划实现：

- desktop root。
- app container。
- one window。
- title bar。
- launcher placeholder。
- one app metadata record。

验收：

- 同一个 WING GUI app 可以直接从 NSH 启动。
- 同一个 WING GUI app 可以被 WING Desktop container 承载。

## 10. 第六阶段目标：Pinion POC

前置条件：

- FRender command list 稳定。
- framebuffer present 稳定。
- input 基础可用。

计划实现：

- game loop。
- scene。
- sprite。
- input。
- animation。
- simple collision。

验收：

- `pinion_demo` 可以从 NSH 启动。
- demo 可以显示 moving sprite。
- demo 使用 FRender 而不是直接写 framebuffer。

## 11. 当前优先级

当前最高优先级：

```text
WING GUI Core Phase 1
```

具体顺序：

1. `wing_obj_t`。
2. object tree。
3. draw traversal。
4. invalidation。
5. event callback。
6. 更新 `wing_gui_demo` 为标准 GUI App 模板。

暂时不要优先做：

- WING Desktop。
- Pinion。
- 大量 widgets。
- 3D 桌面。
- 复杂主题系统。
- 资源打包系统。

## 12. 待决策问题

### 12.1 WING GUI 是否保留单头文件入口

候选方案：

- 继续使用 `wing/wing.h` 作为统一入口。
- 同时增加细分头文件，例如 `wing_object.h`、`wing_event.h`、`wing_style.h`。

推荐：

```text
保留 wing.h 作为总入口，同时拆分内部模块头文件。
```

### 12.2 object 与 widget 是否分离

候选方案：

- `wing_obj_t` 是所有 UI 对象基类。
- `wing_widget_t` 是更高层封装。

推荐：

```text
先只有 wing_obj_t。
等基础稳定后再引入 widget 层。
```

### 12.3 WING Desktop 是否放在 wing 内部

候选方案：

- `apps/graphics/wing/desktop`
- `apps/graphics/wing_desktop`

推荐：

```text
概念上独立，代码上可以先放 wing/desktop，稳定后再拆出 wing_desktop。
```

### 12.4 默认空间 / 高级 3D 能力放在哪里

候选方案：

- 默认 object space、camera、projection、depth sort：WING GUI core。
- projected quad / blit / future mesh command：FRender。
- GPU2D / GPU3D / DMA / display present：NuttX graphics backend 或板级驱动能力声明。
- 桌面级 app switcher / card transition / window effect：WING Desktop effects。
- 游戏场景、game object、physics、game loop：Pinion。

推荐：

```text
默认空间能力必须放进 WING GUI core。
普通 2D widget 是默认空间下的 identity transform 平面对象。
高级 mesh / shader / GPU 3D 不直接塞进 widget，而是通过 FRender advanced command 和 backend capability 逐步接入。
```

## 13. 阶段完成标准

WING GUI Phase 1 完成时应满足：

- `wing_gui_demo` 可以从 NSH 启动。
- demo 创建 GUI runtime。
- demo 创建 object tree。
- GUI runtime 负责 tick / step。
- object draw callback 生成 FRender command。
- FRender 完成软件绘制和 framebuffer present。
- 关闭 simulator 窗口后 demo 返回 NSH。
- 没有 dirty 时不会重复绘制。
- headless 验证和真实 X11 验证要分开记录：`env -u DISPLAY` 不会打开窗口；保留 `DISPLAY` 直接运行 sim 并从 NSH 执行 `wing_gui_demo` 才能验证 X11 窗口打开和关闭。
- slider / scrollbar 等拖拽 widget 应验证 pointer capture：pointer move/up 离开 widget bounds 后仍能完成交互，并输出 pointer captured / released lifecycle 日志。

完成后再进入 Phase 2。

## 14. 维护规则

本文应作为当前 WING/FRender/Pinion 计划的唯一进度入口。

每完成一个阶段，应更新：

- 已完成内容。
- 当前限制。
- 下一阶段任务。
- 验收状态。
- 新增待决策问题。

架构原则不要频繁写进本文，应放入架构文档。

实现细节不要全部写进本文，应放入对应实现文档。

### 2026-06-12：补充 progress widget

范围：`apps/graphics/wing`、`apps/examples/wing_gui_demo`。

内容：

- 新增 `wing_progress_t` 组件，放入 WING GUI widget 层。
- `wing_progress_t` 维护 frame style、fill style、range、value 和 padding。
- `wing_progress_t` 通过 FRender 绘制 frame 与 fill，不再要求 demo 手工拼接两个 box。
- `wing_progress_set_value()` 会触发对象脏标记，并发送 `WING_EVENT_VALUE_CHANGED`。
- `wing_gui_demo` 的进度条从 box 组合迁移为 progress widget。
- `wing_gui_demo` 使用 WING timer 推进 progress value，用于验证 GUI 句柄维护软件定时器和组件状态更新。

验证计划：

- 重新构建 sim：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 执行 demo：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`

下一步：

- 继续完善 WING GUI 的 widget 基类策略。
- 继续补充 style/theme 的统一抽象。
- 后续可基于 progress 的 value/range 模型扩展 slider、scrollbar、meter 等组件。

验证结果：

- 构建通过：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 运行通过：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 属于预期现象。
- `wing_gui_demo` 输出 `progress timer updated wing_progress value=90`。
- `wing_gui_demo` 输出一帧 `commands=45 checksum=0x626d2fc5`。
- demo 正常退出回到 NSH 并执行 `poweroff`。

### 2026-06-12：补充 slider widget

范围：`apps/graphics/wing`、`apps/examples/wing_gui_demo`。

内容：

- 新增 `wing_slider_t` 组件，作为第一批可交互 value widget。
- `wing_slider_t` 维护 track style、fill style、knob style、range、value、padding 和 knob size。
- `wing_slider_t` 自己处理 pointer down/move/up，根据 pointer x 坐标更新 value。
- `wing_slider_set_value()` 会触发对象脏标记，并发送 `WING_EVENT_VALUE_CHANGED`。
- `wing_gui_demo` 新增 slider 场景，使用合成 pointer 输入验证 value 更新和 pointer 交互完成事件。
- 修正 progress range 归一化边界，避免 `min == UINT16_MAX` 且 `max <= min` 时溢出。

验证计划：

- 重新构建 sim：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 执行 demo：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`

下一步：

- 继续抽象 value widget 的公共行为，避免 progress、slider、scrollbar 后续重复实现 range/value 逻辑。
- 继续建立更统一的 style/theme 入口。

验证结果：

- 构建通过：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 首次运行发现 slider pointer up 后 click 会继续冒泡到 root。
- 已修正：`wing_slider_t` 对 pointer/click 事件调用 `wing_event_stop_propagation()`。
- 修正后重新构建通过。
- 修正后运行通过：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 属于预期现象。
- `wing_gui_demo` 输出 `wing_slider value changed to 7 by pointer input`。
- `wing_gui_demo` 输出 `wing_slider value changed to 83 by pointer input`。
- `wing_gui_demo` 输出 `wing_slider pointer interaction completed`。
- slider 交互不再冒泡到 root click。
- demo 输出一帧 `commands=49 checksum=0x435e23c5`，正常退出回到 NSH 并执行 `poweroff`。

### 2026-06-12：抽取内部 value/range helper

范围：`apps/graphics/wing/src/core`、`apps/graphics/wing/src/widgets`。

内容：

- 新增内部 `wing_value` helper。
- 统一处理 value widget 的 range normalize、clamp、value-to-offset、offset-to-value。
- `wing_progress_t` 改为使用 `wing_value` helper。
- `wing_slider_t` 改为使用 `wing_value` helper。
- 对外 `wing_progress_t` 和 `wing_slider_t` API 保持不变。

意义：

- 减少 progress、slider、后续 scrollbar/meter 等组件中的重复边界处理。
- 为 WING GUI 的 value widget 系列建立内部公共模型。
- 保持公共 API 稳定，避免过早暴露内部实现细节。

验证计划：

- 重新构建 sim：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 执行 demo：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`

验证结果：

- 构建通过：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 运行通过：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 属于预期现象。
- `wing_progress_t` 继续输出 `progress timer updated wing_progress value=90`。
- `wing_slider_t` 继续输出 `wing_slider value changed to 7 by pointer input` 和 `wing_slider value changed to 83 by pointer input`。
- demo 输出一帧 `commands=49 checksum=0x435e23c5`，正常退出回到 NSH 并执行 `poweroff`。

### 2026-06-12：补充 scrollbar widget

范围：`apps/graphics/wing`、`apps/examples/wing_gui_demo`。

内容：

- 新增 `wing_scrollbar_t` 组件，作为第三个基于内部 value/range helper 的 value widget。
- 新增 `wing_axis_e`，scrollbar 结构预留 horizontal/vertical 方向能力。
- `wing_scrollbar_t` 维护 track style、thumb style、range、value、page size、padding 和 axis。
- `wing_scrollbar_t` 复用 `wing_value` helper 计算 thumb 位置和 pointer 到 value 的映射。
- `wing_scrollbar_t` 自己处理 pointer down/move/up，并阻止 pointer/click 继续冒泡。
- `wing_gui_demo` 新增 scrollbar 场景，使用合成 pointer 输入验证 value changed 和交互完成事件。

验证计划：

- 重新构建 sim：`./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`
- 执行 demo：`env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`

下一步：

- 后续可以把 progress、slider、scrollbar 的 value changed 通知进一步规范为统一 value event payload。
- 后续可以继续补充 theme/style token，减少 demo 内大量手写颜色。

## 2026-06-12 更新：Scrollbar 与队列容量修正

本轮在 WING GUI 第一阶段中补充了 `wing_scrollbar_t`，它与 `wing_progress_t`、`wing_slider_t` 共享内部 value/range 映射模型，继续验证 WING GUI 可以基于 FRender 承载基础控件绘制与输入交互。

### 新增内容

- 新增 `wing_scrollbar_t` 公共组件接口。
- 支持水平/垂直方向枚举 `wing_axis_t`。
- 支持 track/thumb style、range、value、page_size、padding、event callback。
- 指针 down/move/up 会被 scrollbar 消费，避免继续冒泡到父节点/root。
- value 变化触发 `WING_EVENT_VALUE_CHANGED`。
- `wing_gui_demo` 新增 scrollbar 示例，用合成 pointer 输入验证 value 更新路径。

### 运行时队列修正

- `WING_GUI_INPUT_QUEUE_SIZE` 从 8 提升到 16。
- 原因：demo 中 button、slider、scrollbar 连续注入合成输入事件，8 个槽位会导致后半段 pointer move/up 入队失败。
- `WING_GUI_EVENT_QUEUE_SIZE` 从 16 提升到 32。
- 原因：输入事件会派生 click、focus、key、value changed 等 GUI 事件，16 个槽位在 scrollbar 加入后不够，曾出现 `wing_gui_demo: step failed: -28`。

### 构建与验证

- 已重新执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 属于预期，demo 会跳过 framebuffer present。
- 最终验证通过，关键日志如下：

```text
wing_gui_demo: scrollbar widget reuses WING value model and consumes pointer input
wing_gui_demo: wing_scrollbar value changed to 0 by pointer input
wing_gui_demo: wing_scrollbar value changed to 75 by pointer input
wing_gui_demo: wing_scrollbar pointer interaction completed
wing_gui_demo: frame tick=33 commands=52 checksum=0x36bfa7c5
wing_gui_demo: app task exit
```

### 当前结论

WING GUI 第一阶段已经具备继续向上推进的基础：对象树、事件队列、输入队列、焦点/key、timer、animation、布局、基础控件、value 控件族，以及 FRender 绘制路径。下一步应该继续把控件模型和应用运行句柄收敛，而不是立刻进入 Desktop 层。

### 下一步建议

1. 把输入/事件队列大小改成 Kconfig 可配置项，而不是固定宏。
2. 为 value 类组件补充统一 getter 与统一 value event payload。
3. 增加 `wing_switch_t` 或 `wing_checkbox_t`，验证布尔状态控件。
4. 开始整理 WING GUI app handle 的公开生命周期接口：init、tick、step、request_redraw、dispatch_input、deinit。

## 2026-06-12 更新：输入队列与事件队列 Kconfig 化

本轮把 WING GUI 运行时的两个核心队列容量从公共头文件里的固定值整理为 Kconfig 配置项。

### 新增配置项

- `CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE`
  - 默认值：16
  - 用途：控制每个 WING GUI 实例可缓存的原始输入事件数量。
  - 覆盖内容：pointer down/move/up、key down/up。

- `CONFIG_GRAPHICS_WING_EVENT_QUEUE_SIZE`
  - 默认值：32
  - 用途：控制每个 WING GUI 实例可缓存的 GUI 事件数量。
  - 覆盖内容：click、focus、key、value changed、bubbled object event 等。

### 设计原因

输入队列和事件队列属于 WING GUI app handle 的运行时能力，而不是某个具体 widget 的实现细节。把它们做成 Kconfig 项后，不同 board、不同 demo、不同 Desktop 配置可以按资源规模调整容量。

这也符合后续方向：WING GUI 应该提供稳定的应用运行句柄，由应用线程反复调用 tick/step，队列容量、timer 数量、animation 数量等运行时资源应逐步从硬编码迁移到可配置能力。

### 下一步

- 继续把 `WING_GUI_TIMER_MAX`、`WING_GUI_ANIM_MAX` 也迁移为配置项。
- 为事件队列补充更清晰的 value event payload，减少 widget 内部字段地址直接作为 event data 的情况。

### 本轮验证结果

- 已执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`，运行通过。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 仍为预期现象。
- Kconfig 化后默认队列容量保持有效，未再出现事件队列溢出。

关键输出：

```text
wing_gui_demo: wing_slider value changed to 7 by pointer input
wing_gui_demo: wing_slider value changed to 83 by pointer input
wing_gui_demo: wing_scrollbar value changed to 0 by pointer input
wing_gui_demo: wing_scrollbar value changed to 75 by pointer input
wing_gui_demo: frame tick=33 commands=52 checksum=0x36bfa7c5
wing_gui_demo: app task exit
```

## 2026-06-12 更新：Timer 与 Animation 槽位 Kconfig 化

本轮继续整理 WING GUI app handle 的运行时资源，把 timer 和 animation 槽位从固定宏迁移为 Kconfig 配置项。

### 新增配置项

- `CONFIG_GRAPHICS_WING_TIMER_MAX`
  - 默认值：8
  - 用途：控制每个 WING GUI 实例可同时维护的软件 timer 数量。
  - 对应场景：周期刷新、延迟状态变更、闪烁、进度更新、未来 widget 内部 timer。

- `CONFIG_GRAPHICS_WING_ANIM_MAX`
  - 默认值：8
  - 用途：控制每个 WING GUI 实例可同时维护的 animation 数量。
  - 对应场景：属性动画、过渡动画、控件状态动画、未来 Desktop 动效。

### 设计原因

WING GUI 的主循环不只是 draw 调用，而是一个应用拥有的 GUI runtime handle。这个 handle 需要维护 input queue、event queue、timer、animation、dirty state 和 render frontend。把这些资源配置化，有利于同一套 WING GUI 同时服务小资源 board、普通 NSH UI app、以及未来 WING Desktop。

### 当前运行时资源配置状态

```text
CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE = 16
CONFIG_GRAPHICS_WING_EVENT_QUEUE_SIZE = 32
CONFIG_GRAPHICS_WING_TIMER_MAX        = 8
CONFIG_GRAPHICS_WING_ANIM_MAX         = 8
```

### 本轮验证结果

- 已执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`，运行通过。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 仍为预期现象。
- timer 配置化后，progress timer 仍正常更新。
- slider、scrollbar、button、focus/key、event bubbling 路径保持稳定。

关键输出：

```text
wing_gui_demo: progress timer updated wing_progress value=90
wing_gui_demo: wing_slider value changed to 83 by pointer input
wing_gui_demo: wing_scrollbar value changed to 75 by pointer input
wing_gui_demo: frame tick=33 commands=52 checksum=0x36bfa7c5
wing_gui_demo: app task exit
```

## 2026-06-12 更新：统一 Value Changed 事件 Payload

本轮整理 `WING_EVENT_VALUE_CHANGED` 的事件数据格式，新增公共结构 `wing_value_event_t`。

### 新增公共事件数据结构

```c
struct wing_value_event_s
{
  uint16_t old_value;
  uint16_t value;
  uint16_t min;
  uint16_t max;
};
```

### 接入范围

- `wing_progress_t`
- `wing_slider_t`
- `wing_scrollbar_t`
- `wing_gui_demo` 的 slider/scrollbar 事件日志

### 设计原因

此前 value changed 事件直接把控件内部 `value` 字段地址放入 `event->data`。这种方式虽然可以工作，但应用层只能猜测 `event->data` 的类型，也拿不到 old value、range 等上下文。

统一 payload 后，WING GUI 的 value 控件族开始拥有一致的事件语义：应用在事件回调内可以读取 old/current/range，未来也可以更自然地扩展为 knob drag、scroll page、progress update 等更复杂交互。

### 注意

当前 `wing_obj_send_event()` 是同步派发，因此 `wing_value_event_t` payload 的生命周期限定在事件回调期间。应用如果需要长期保存，应复制 payload 内容，而不是保存指针。

### 本轮验证结果

- 已执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`，运行通过。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 仍为预期现象。
- `wing_gui_demo` 已能从 `wing_value_event_t` 中读取 old/current value。
- FRender 输出保持稳定，checksum 未变化。

关键输出：

```text
wing_gui_demo: wing_slider value changed from 25 to 7 by pointer input
wing_gui_demo: wing_slider value changed from 7 to 83 by pointer input
wing_gui_demo: wing_scrollbar value changed from 20 to 0 by pointer input
wing_gui_demo: wing_scrollbar value changed from 0 to 75 by pointer input
wing_gui_demo: frame tick=33 commands=52 checksum=0x36bfa7c5
wing_gui_demo: app task exit
```

## 2026-06-12 更新：新增 Switch 布尔状态控件

本轮新增 `wing_switch_t`，用于验证 WING GUI 的布尔状态控件模型。

### 新增公共组件

- `wing_switch_t`
- `wing_switch_init()`
- `wing_switch_obj()`
- `wing_switch_set_checked()`
- `wing_switch_get_checked()`
- `wing_switch_set_off_style()`
- `wing_switch_set_on_style()`
- `wing_switch_set_knob_style()`
- `wing_switch_set_padding()`
- `wing_switch_set_knob_size()`
- `wing_switch_set_event_cb()`

### 行为

- 支持 off/on track style。
- 支持 knob style。
- 支持 pointer down/up pressed state。
- 支持 click 切换 checked 状态。
- 支持 Enter/Space key down 切换 checked 状态。
- 状态变化时发送 `WING_EVENT_VALUE_CHANGED`。
- value changed 事件复用 `wing_value_event_t`，其中 `min=0`、`max=1`。

### Demo 更新

`wing_gui_demo` 在 header 右侧新增 switch，并注入一次 pointer click 验证状态从 0 切换为 1。

### 设计意义

`wing_switch_t` 是 WING GUI 第一个布尔状态控件。它与 progress/slider/scrollbar 一起证明：WING 的 value event payload 可以覆盖连续值控件和离散布尔控件，后续 checkbox、toggle button、settings UI 可以复用同一事件语义。

### 本轮验证结果

- 已执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`，运行通过。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 仍为预期现象。
- `wing_switch_t` 已被编译进 WING GUI，并在 demo 中完成 pointer click 切换。
- switch 复用 `wing_value_event_t`，成功输出 old/current boolean value。
- 新增 switch 绘制后，FRender 命令数从 52 增加到 55，checksum 随之变化。

关键输出：

```text
wing_gui_demo: switch widget reuses WING value event payload for boolean state
wing_gui_demo: wing_switch value changed from 0 to 1 by click input
wing_gui_demo: frame tick=33 commands=55 checksum=0x33bac7c5
wing_gui_demo: app task exit
```

## 2026-06-12 更新：新增 Checkbox 布尔状态控件

本轮新增 `wing_checkbox_t`，继续完善 WING GUI 的基础控件族。

### 新增公共组件

- `wing_checkbox_t`
- `wing_checkbox_init()`
- `wing_checkbox_obj()`
- `wing_checkbox_set_checked()`
- `wing_checkbox_get_checked()`
- `wing_checkbox_set_box_style()`
- `wing_checkbox_set_checked_style()`
- `wing_checkbox_set_mark_style()`
- `wing_checkbox_set_padding()`
- `wing_checkbox_set_event_cb()`

### 行为

- 支持 unchecked/checked box style。
- 支持 checked mark style。
- 支持 pointer down/up pressed state。
- 支持 click 切换 checked 状态。
- 支持 Enter/Space key down 切换 checked 状态。
- 状态变化时发送 `WING_EVENT_VALUE_CHANGED`。
- value changed 事件复用 `wing_value_event_t`，其中 `min=0`、`max=1`。

### Demo 更新

`wing_gui_demo` 在 header 左侧新增 checkbox，并注入一次 pointer click 验证状态从 0 切换为 1。header 右侧的 switch 继续保留，用于验证多个布尔控件共享同一 value event 语义。

### 设计意义

`wing_checkbox_t` 与 `wing_switch_t` 共同覆盖了两类常见布尔控件：显式勾选和开关切换。它们都复用 WING 的 object tree、input/event、focus/key、dirty redraw 和 `wing_value_event_t`，说明布尔控件可以在当前 WING GUI runtime 上自然扩展。

### 本轮验证结果

- 已执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`，运行通过。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 仍为预期现象。
- `wing_checkbox_t` 已被编译进 WING GUI，并在 demo 中完成 pointer click 切换。
- checkbox 和 switch 都成功复用 `wing_value_event_t`。
- 新增 checkbox 绘制后，FRender 命令数从 55 增加到 58。

关键输出：

```text
wing_gui_demo: checkbox widget reuses WING value event payload for checked state
wing_gui_demo: wing_checkbox value changed from 0 to 1 by click input
wing_gui_demo: wing_switch value changed from 0 to 1 by click input
wing_gui_demo: frame tick=33 commands=58 checksum=0x1f5727c5
wing_gui_demo: app task exit
```

## 2026-06-12 更新：Value 控件 Getter API

本轮为连续 value 控件补充公共 getter，使应用除了响应 `WING_EVENT_VALUE_CHANGED` 外，也可以主动查询当前状态。

### 新增 API

- `wing_progress_get_value()`
- `wing_progress_get_range()`
- `wing_slider_get_value()`
- `wing_slider_get_range()`
- `wing_scrollbar_get_value()`
- `wing_scrollbar_get_range()`
- `wing_scrollbar_get_page_size()`

### Demo 更新

`wing_gui_demo` 在进入主循环前输出 progress、slider、scrollbar 的初始 value/range/page，用于验证 getter API。

### 设计意义

事件 payload 解决的是“状态变化时如何通知应用”；getter API 解决的是“应用何时都能主动读取控件状态”。这两者共同构成 WING GUI value 控件的基础公共语义。

### 本轮验证结果

- 已执行 `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean`，构建通过。
- 已执行 `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'`，运行通过。
- 无 DISPLAY 环境下 `ERROR: fb_register() failed: -19` 仍为预期现象。
- getter 输出正确，FRender checksum 保持稳定。

关键输出：

```text
wing_gui_demo: progress getter value=70 range=0..100
wing_gui_demo: slider getter value=25 range=0..100
wing_gui_demo: scrollbar getter value=20 range=0..100 page=25
wing_gui_demo: frame tick=33 commands=58 checksum=0x1f5727c5
wing_gui_demo: app task exit
```

## 2026-06-12 阶段更新：WING GUI 可交互控件与键盘激活路径

本次更新目标：继续把 `wing_gui_demo` 从静态绘制验证推进到可交互 GUI runtime 验证，重点覆盖 value widget、boolean widget、键盘激活路径和运行时资源配置。

已完成：

- 新增 `wing_progress_t`、`wing_slider_t`、`wing_scrollbar_t` 作为第一批 value widget。
- 新增 `wing_switch_t`、`wing_checkbox_t` 作为第一批 boolean widget。
- 新增统一 value helper，用于 clamp、ratio、track 坐标映射。
- 新增 `wing_value_event_t`，用于 `WING_EVENT_VALUE_CHANGED` 事件携带 old/value/min/max。
- 为 progress、slider、scrollbar 增加 getter，demo 可以打印当前值和范围。
- 为 switch、checkbox 增加 checked getter。
- `wing_gui_demo` 已展示 panel、button、label、progress、slider、scrollbar、switch、checkbox。
- `wing_gui_demo` 已验证 pointer click、event bubbling、stop propagation、focus gained、keyboard Enter/Space、timer、animation、layout、FRender command 输出。
- checkbox/switch 的 Space/Enter 激活路径已收敛为统一 click 路径，避免 key down 和 synthesized click 各触发一次导致双切换。
- WING runtime 资源配置已提高到当前 demo 可承载范围：input queue 16，event queue 48，timer 8，animation 8。
- `tools/firmware/sim/build-wing.sh` 已显式写入 WING runtime 资源配置，避免 `--no-clean` 复用旧 `.config` 时继续沿用过小队列。

踩坑记录：

- 完整 keyboard/pointer synthetic demo 会一次性塞入较多 input 和派生 event，旧的 event queue 32 会触发 `-28`。
- 单纯修改 Kconfig default 不会自动改变已有 `.config` 中的旧值，sim 构建脚本需要显式设置关键 WING 配置。
- checkbox/switch 如果在 `WING_EVENT_KEY_DOWN` 中直接 toggle，同时 runtime 又把 Space/Enter 合成为 click，就会出现一次键盘输入切两次的 bug。
- 当前结论是：Enter/Space 应只负责产生统一 activation/click 语义，控件状态变更只放在 click 路径中。

本次验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本次验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: app task entered WING GUI loop
wing_gui_demo: frame interval=33ms
wing_gui_demo: progress getter value=70 range=0..100
wing_gui_demo: slider getter value=25 range=0..100
wing_gui_demo: scrollbar getter value=20 range=0..100 page=25
wing_gui_demo: progress timer updated wing_progress value=90
wing_gui_demo: wing_checkbox value changed from 0 to 1 by toggle input
wing_gui_demo: wing_checkbox value changed from 1 to 0 by toggle input
wing_gui_demo: wing_switch value changed from 0 to 1 by toggle input
wing_gui_demo: wing_switch value changed from 1 to 0 by toggle input
wing_gui_demo: wing_button received key down=13 from focused input path
wing_gui_demo: wing_slider value changed from 25 to 7 by pointer input
wing_gui_demo: wing_slider value changed from 7 to 83 by pointer input
wing_gui_demo: wing_scrollbar value changed from 20 to 0 by pointer input
wing_gui_demo: wing_scrollbar value changed from 0 to 75 by pointer input
wing_gui_demo: frame tick=33 commands=57 checksum=0x849463c5
wing_gui_demo: app task exit
```

说明：

- `ERROR: fb_register() failed: -19` 是无 DISPLAY 验证场景下的预期现象。
- `framebuffer present skipped: -2` 表示 demo 在无 framebuffer present 的环境下继续走软件渲染验证路径。
- checkbox/switch 各出现两次 value changed，符合预期：一次来自 pointer click，一次来自 Space/Enter 合成 click。
- 本阶段可以继续向 WING GUI 的 widget base、style/theme、layout、input adapter 推进。

下一步计划：

- 抽象 widget base，避免每个控件都重复处理 object、state、event、style。
- 将 value widget 进一步统一为可复用 value model。
- 增加 disabled/hover/pressed/focused 的一致状态样式策略。
- 扩展 text/font 基础能力，把 builtin font seed 推进到可转换字体资源、更完整的 word wrap、文本输入和 FRender text command。
- 增加真实输入设备 adapter，将 NuttX touch/mouse/key 输入转换为 WING input queue。
- 保持 `wing_gui_demo` 作为 NSH 可执行最小 GUI 应用模板，每次新增能力都先在该 demo 中形成可验证路径。

## 2026-06-12 更新：抽出第一版 Widget/Value 内部 Helper

本轮目标是落实上一阶段计划中的“抽象 widget base / value model”第一步，但先不大改公共 API，也不引入完整继承体系。当前选择是新增内部 helper，让现有控件先共享重复逻辑。

新增内部文件：

- `apps/graphics/wing/src/core/wing_widget.h`
- `apps/graphics/wing/src/core/wing_widget.c`

`wing_widget` 当前职责：

- 统一初始化 `wing_obj_t` 的 draw/event/user_data/focusable 字段。
- 统一绘制 `wing_box_style_t` 的 fill / stroke / clear。
- 统一 pressed state 的设置与清除。
- 统一 Enter/Space activation key 的停止传播逻辑。
- 统一 numeric value 更新后的 object invalidation 与 `WING_EVENT_VALUE_CHANGED` 派发。
- 统一 boolean value 更新后的 object invalidation 与 `WING_EVENT_VALUE_CHANGED` 派发。

扩展 `wing_value`：

- `wing_value_init_storage()`：统一初始化 min/max/value。
- `wing_value_update_range()`：统一更新范围并重新 clamp 当前值。
- `wing_value_update()`：统一 value set、clamp、old/new payload 生成。
- `wing_value_update_bool()`：统一 boolean old/new payload 生成。

已迁移复用的控件：

- `wing_progress_t`
- `wing_slider_t`
- `wing_scrollbar_t`
- `wing_switch_t`
- `wing_checkbox_t`

设计意义：

- 这不是完整 widget base，但已经把控件重复的 object 初始化、style 绘制、pressed state、activation key、value event payload、invalidation 和 value changed event dispatch 逻辑收拢到 core helper。
- 后续真正做 widget base / theme / state style 时，可以继续沿着 `wing_widget` 扩展，而不是在每个控件里各写一份。
- 本轮保持 public API 稳定，`wing_gui_demo` 增加一条 widget base value dispatch 日志，用于证明 numeric 与 boolean widget 都走共享 value dispatch 路径。

本轮验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本轮验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: progress getter value=70 range=0..100
wing_gui_demo: slider getter value=25 range=0..100
wing_gui_demo: scrollbar getter value=20 range=0..100 page=25
wing_gui_demo: wing_checkbox value changed from 0 to 1 by toggle input
wing_gui_demo: wing_checkbox value changed from 1 to 0 by toggle input
wing_gui_demo: wing_switch value changed from 0 to 1 by toggle input
wing_gui_demo: wing_switch value changed from 1 to 0 by toggle input
wing_gui_demo: wing_slider value changed from 25 to 7 by pointer input
wing_gui_demo: wing_slider value changed from 7 to 83 by pointer input
wing_gui_demo: wing_scrollbar value changed from 20 to 0 by pointer input
wing_gui_demo: wing_scrollbar value changed from 0 to 75 by pointer input
wing_gui_demo: frame tick=33 commands=57 checksum=0x849463c5
wing_gui_demo: app task exit
```

说明：

- 构建通过。
- `wing_gui_demo` 运行通过。
- 无 DISPLAY 环境下 framebuffer present 跳过仍为预期现象。
- refactor 后最终 command 数和 checksum 与上一阶段一致，说明行为保持稳定。

下一步计划：

- 在 `wing_widget` 上继续沉淀 state style / disabled / focused / pressed 的统一处理。
- 将 button、panel、box 的样式选择逻辑逐步接入 `wing_widget`，避免 style 体系继续分散。
- 为 slider/scrollbar 增加 keyboard step 输入，让 value widget 不只依赖 pointer。
- 准备真实 NuttX input adapter 的最小接口。

## 2026-06-12 更新：Value Widget 键盘步进与 Focus Key Policy

本轮继续推进 WING GUI 的可访问输入模型，使 value widget 不再只依赖 pointer 输入。

完成内容：

- `wing_slider_t` 改为 focusable widget。
- `wing_scrollbar_t` 改为 focusable widget。
- `wing_slider_t` 支持方向键步进：Right/Up 增加，Left/Down 减少。
- `wing_scrollbar_t` 支持方向键步进：Right/Down 增加，Left/Up 减少。
- 新增 `wing_value_step()`，统一处理 min/max/value/step 的安全加减和 clamp。
- WING key dispatch policy 调整为：`Tab` 负责焦点遍历；如果存在 focused object，方向键优先发送给 focused object；只有没有 focused object 时，方向键才作为初始焦点遍历 fallback。
- WING runtime 队列容量提高到 input queue 24、event queue 64，用于承载更完整的 synthetic input 验证。
- `wing_gui_demo` 增加 slider/scrollbar 的 pointer + keyboard step 验证。

设计意义：

- 这让 WING GUI 开始具备“焦点控件处理键盘输入”的基本 GUI 语义。
- `Tab` 与方向键职责分离，避免 value widget 想处理方向键时被 runtime 提前拿去做焦点切换。
- slider/scrollbar 当前使用固定 step=5，后续可抽象为 widget 属性或 style/theme 配置。
- 这一设计更接近 LVGL / TouchGFX 等 GUI 框架中“焦点对象优先消费输入”的模型。

本轮验证命令：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'
```

本轮验证结果：

```text
ERROR: fb_register() failed: -19
wing_gui_demo: framebuffer present skipped: -2
wing_gui_demo: slider widget consumes pointer/key input and emits value changed events
wing_gui_demo: scrollbar widget reuses WING value model and consumes pointer/key input
wing_gui_demo: widget base dispatches value updates, invalidation and value changed events for numeric and boolean widgets
wing_gui_demo: wing_checkbox value changed from 0 to 1 by toggle input
wing_gui_demo: wing_switch value changed from 0 to 1 by toggle input
wing_gui_demo: wing_slider value changed from 25 to 7 by value input
wing_gui_demo: wing_slider value changed from 7 to 83 by value input
wing_gui_demo: wing_slider pointer interaction completed
wing_gui_demo: wing_slider value changed from 83 to 88 by value input
wing_gui_demo: wing_slider handled key step key=1001
wing_gui_demo: wing_slider value changed from 88 to 83 by value input
wing_gui_demo: wing_slider handled key step key=1000
wing_gui_demo: wing_scrollbar value changed from 20 to 0 by value input
wing_gui_demo: wing_scrollbar value changed from 0 to 75 by value input
wing_gui_demo: wing_scrollbar pointer interaction completed
wing_gui_demo: wing_scrollbar value changed from 75 to 80 by value input
wing_gui_demo: wing_scrollbar handled key step key=1001
wing_gui_demo: wing_scrollbar value changed from 80 to 75 by value input
wing_gui_demo: wing_scrollbar handled key step key=1000
wing_gui_demo: frame tick=33 commands=57 checksum=0x849463c5
wing_gui_demo: app task exit
```

说明：

- 构建通过。
- `wing_gui_demo` 运行通过。
- 无 DISPLAY 环境下 framebuffer present 跳过仍为预期现象。
- slider 和 scrollbar 都完成 pointer 改值后，再通过方向键 +5/-5 往返验证。
- 最终 checksum 与上一阶段一致，说明键盘步进往返后最终画面保持稳定。

下一步计划：

- 为 value widget 增加可配置 step 属性，而不是硬编码 step=5。
- 继续完善 focus/state style，让 slider/scrollbar 获得 focused 可视反馈。
- 准备 NuttX input adapter，将真实 key/touch 事件送入 `wing_gui_enqueue_input()`。

## 2026-06-12 更新：Slider / Scrollbar Focused 状态可视反馈

本轮继续推进 WING GUI Phase 2 的 state style 和可交互组件体验。

完成内容：

- `wing_slider_t` 增加 focused state style。
- `wing_scrollbar_t` 增加 focused state style。
- 新增 `wing_slider_set_state_style()`。
- 新增 `wing_scrollbar_set_state_style()`。
- slider / scrollbar 在 focused 状态下通过 FRender command list 叠加 focused outline。
- `wing_gui_demo` 已配置 slider / scrollbar focused 样式，并输出 focus gained 验证日志。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- slider focus gained 日志出现。
- scrollbar focus gained 日志出现。
- slider 键盘步进仍通过：`83 -> 88 -> 83`。
- scrollbar 键盘步进仍通过：`75 -> 80 -> 75`。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- 为 value widget 增加可配置 step 属性。
- 将 focused / pressed / disabled 的 state style 解析进一步收拢到 `wing_widget`。
- 开始准备真实 NuttX input adapter。

## 2026-06-12 更新：Slider / Scrollbar Step 属性化

本轮继续推进 WING GUI Phase 2 的 value widget 工程化。

完成内容：

- `wing_slider_t` 增加可配置 step 属性。
- `wing_scrollbar_t` 增加可配置 step 属性。
- 新增 `wing_slider_set_step()` / `wing_slider_get_step()`。
- 新增 `wing_scrollbar_set_step()` / `wing_scrollbar_get_step()`。
- 方向键步进从硬编码 `5` 改为读取 widget property。
- `wing_gui_demo` 配置 slider step=7、scrollbar step=9，并输出 getter 验证日志。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- slider 配置步进验证：`83 -> 90 -> 83`。
- scrollbar 配置步进验证：`75 -> 84 -> 75`。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- 将 focused / pressed / disabled 的状态样式解析进一步收拢到 `wing_widget`。
- 开始准备真实 NuttX input adapter。
- 逐步建立 widget property 模型。

## 2026-06-12 更新：State Style 公共解析 helper

本轮继续推进 WING GUI Phase 2 的 widget/state/style 公共层。

完成内容：

- `wing_widget` 新增 state-style 选择 helper。
- `wing_widget` 新增 state-style 存储 helper。
- `wing_box_t` 的 active style 选择迁移到公共 helper。
- `wing_box_t` 的 state style 设置迁移到公共 helper。
- `wing_box_t` 绘制复用 `wing_widget_draw_style()`。
- slider / scrollbar focused overlay 改为复用公共 state-style 选择逻辑。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- button focused style 验证通过。
- slider focused overlay 验证通过。
- scrollbar focused overlay 验证通过。
- slider 配置步进仍验证：`83 -> 90 -> 83`。
- scrollbar 配置步进仍验证：`75 -> 84 -> 75`。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- 准备真实 NuttX input adapter。
- 继续整理 widget property 模型。
- 让 `wing_gui_demo` 分段展示 WING GUI 当前全部能力。

## 2026-06-12 更新：wing_gui_demo 能力展示分段输出

本轮继续推进 `wing_gui_demo` 作为 WING GUI 能力展示台的定位。

完成内容：

- demo 新增 setup / synthetic input script / runtime capability summary 分段输出。
- demo 每次投递 synthetic input 时输出 label、input type、point、key。
- demo 输出 surface、command list capacity、frame interval。
- demo 输出 timer schedule。
- demo 输出 animation schedule。

验证：

- `./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean` 构建通过。
- `env -u DISPLAY bash -lc 'printf "wing_gui_demo\npoweroff\n" | ./FeatherCore/build/sim-wing'` 运行通过。
- input queued 日志覆盖 pointer down/up/move、key down、Tab、Enter、direction keys。
- timer / animation 调度日志出现。
- 事件、焦点、冒泡、stop propagation、value widget、state style、FRender render 仍验证通过。
- 最终输出 `frame tick=33 commands=58 checksum=0x4b8f67c5`。

下一步优先级：

- 准备真实 NuttX input adapter。
- 补充 dirty rect / partial redraw 的显式展示。
- 继续把 demo 分段整理成 WING GUI 当前能力清单。

## 2026-06-12 更新：X11 输入适配层 Pointer Move 合并

本轮继续推进 WING GUI 的输入抽象，目标是让真实窗口输入先经过 adapter 整理，再进入 WING GUI runtime，而不是把硬件产生的高频事件原样灌入 input queue。

完成内容：

- `wing_gui_demo` 的 X11 input provider 新增连续 `pointer_move` 合并逻辑。
- 同一轮轮询中连续 mouse move 只保留最新坐标，减少 input queue 压力和 demo 日志噪声。
- 如果合并过程中遇到 pointer down / pointer up / key / encoder 等边界事件，会放入 pending slot，下一次 provider 调用再交给 WING，避免丢失语义事件。
- synthetic input script 不经过该合并路径，仍保留精确事件顺序，用于验证 hover enter / leave、focus、capture、value widget 等语义。

设计依据：

- 吸收 LVGL 的输入设备抽象思想：GUI runtime 消费整理后的输入状态和事件，不直接承受平台噪声。
- 吸收 HoneyGUI 的 engine/input 分层思想：平台窗口输入先在 adapter 层规整，再进入 GUI core。
- 为后续真实 NuttX touch / mouse / keyboard adapter 留出同样模式：触摸滑动和鼠标移动可以合并，按键、按下、抬起、关闭请求必须保持顺序。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证 X11 窗口仍正常打开并占满 framebuffer 输出。
- 验证 synthetic input、事件、动画、timer、dirty redraw、FRender present 仍通过。
- 验证真实 X11 pointer move 日志包含 `coalesced_moves` 字段。
- 验证关闭 X11 窗口后通过 WING close request 回到 NSH。

## 2026-06-12 更新：WING Core Input Adapter Helper

本轮把上一阶段只存在于 `wing_gui_demo` 的 X11 输入整理逻辑，下沉为 WING core input 模块中的可复用 adapter helper。

完成内容：

- 新增 `wing_input_adapter_t`，用于保存平台输入适配层的 pending 事件和 pointer move 合并统计。
- 新增 `wing_input_adapter_init()` 初始化入口。
- 新增 `wing_input_adapter_take_pending()` / `wing_input_adapter_store_pending()`，用于保持 pointer down / pointer up / key / encoder 等边界事件顺序。
- 新增 `wing_input_adapter_merge_pointer_move()`，用于把连续 pointer move 合并成最新坐标。
- 新增 `wing_input_adapter_get_coalesced_moves()`，用于 demo 和后续调试观察输入合并效果。
- `wing_gui_demo` 的 X11 input provider 改为复用 WING core helper，不再私有维护 pending slot 和合并计数。

设计意义：

- 这让 WING GUI 的输入接入从 demo hack 前进到 core input adapter seed。
- 后续真实 NuttX touch / mouse / keyboard adapter 可以复用同一套 pending / coalescing 语义。
- 保持 synthetic input script 精确事件顺序，同时让真实 X11 pointer move 这类高频输入先被规整再进入 GUI runtime。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证 synthetic input、事件、动画、timer、dirty redraw、FRender present、object space transform 仍通过。
- 注入或手动移动 X11 pointer，验证日志继续输出 `coalesced_moves`。
- 关闭 X11 窗口，验证通过 WING close request 回到 NSH。

## 2026-06-12 更新：WING Input Source 语义

本轮继续把输入系统从“泛 pointer 事件”推进到“可区分来源的 WING input event”。

完成内容：

- `wing_input_event_t` 新增 `source` 字段。
- 新增 `WING_INPUT_SOURCE_MOUSE` / `WING_INPUT_SOURCE_TOUCH` / `WING_INPUT_SOURCE_KEYBOARD` / `WING_INPUT_SOURCE_ENCODER` / `WING_INPUT_SOURCE_SYSTEM`。
- X11 framebuffer input provider 将真实窗口 pointer 标记为 `mouse`，key 标记为 `keyboard`，encoder 标记为 `encoder`。
- synthetic demo script 将 pointer 交互标记为 `touch`，键盘和 encoder 事件分别标记为 `keyboard` / `encoder`。
- close request 由 WING core 标记为 `system`。
- demo 输入日志输出 `source=`，用于验证 mouse / touch / keyboard / encoder / system 语义已经进入 WING 输入路径。

设计意义：

- WING GUI 不把触摸、鼠标、键盘、系统关闭混成同一种来源。
- widget 仍然可以先消费统一 pointer/key 语义，平台差异留在 input adapter 层。
- 后续接入 NuttX 真实 touch / mouse / keyboard 驱动时，不需要重构 widget 事件模型。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证 synthetic input 日志包含 `source=touch` / `source=keyboard` / `source=encoder`。
- 验证 X11 pointer move 日志包含 `source=mouse` 和 `coalesced_moves`。
- 关闭 X11 窗口，验证通过 WING close request 回到 NSH。

## 2026-06-12 更新：Input Source 进入 Widget Event 日志

本轮继续验证输入来源语义不是停留在 provider 层，而是随 `wing_input_event_t` 进入 WING event dispatch 和 widget callback。

完成内容：

- `wing_gui_demo` 的 button pointer enter / leave 日志输出 input source。
- button focused key path 日志输出 input source。
- slider / scrollbar 的 key step 与 encoder rotate 日志输出 input source。
- scroll view 的 key / encoder 处理日志输出 input source。

设计意义：

- 鼠标、触摸、键盘、encoder 来源已经跟随事件穿过 WING input queue 和 event queue。
- widget 仍使用统一的 pointer/key/encoder 行为模型，但可以在需要时依据 source 做差异化策略。
- 这为后续真实 NuttX input adapter、触摸 hover 策略、鼠标 hover 策略和桌面级输入路由留下了清晰入口。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证 widget callback 日志包含 `source=touch` / `source=keyboard` / `source=encoder`。
- 注入 X11 pointer move，验证真实窗口输入仍输出 `source=mouse` 和 `coalesced_moves`。
- 关闭 X11 窗口，验证 demo 退出并回到 NSH。

## 2026-06-12 更新：Mouse Hover 与 Touch Pointer 语义分离

本轮让上一阶段新增的 input source 开始影响 WING core 行为，而不只是日志字段。

完成内容：

- WING core input dispatch 新增 hover source 策略。
- `WING_INPUT_SOURCE_MOUSE` 和 `WING_INPUT_SOURCE_UNKNOWN` 可以触发 pointer enter / leave 与 hovered state。
- `WING_INPUT_SOURCE_TOUCH` 不再触发 hover enter / leave，只保留 pointer down / move / up、focus、capture、click 等触摸交互语义。
- `wing_gui_demo` 的 hover 测试输入改为 `source=mouse`。
- `wing_gui_demo` 的点击、拖动、capture 测试继续使用 `source=touch`。

设计意义：

- 鼠标 hover 与触摸点击/拖动正式从 WING core 行为上区分开。
- 后续接入真实触摸屏时，手指移动不会误触发桌面鼠标悬停效果。
- 后续桌面环境仍可以用 mouse source 实现 hover state、tooltip、菜单预选等桌面行为。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证 hover 日志为 `source=mouse`。
- 验证 touch 点击、slider/scrollbar drag、pointer capture、click、focus 仍通过。
- 注入 X11 pointer move，验证真实窗口 mouse hover 仍可触发 pointer enter / leave。
- 关闭 X11 窗口，验证 demo 退出并回到 NSH。

## 2026-06-12 更新：Slider / Scrollbar Drag 与 Mouse Hover 分离

验证 mouse-only hover 策略时，真实 X11 mouse move 暴露出一个 widget 行为问题：鼠标未按下时经过 slider 区域也会触发 value change。

完成内容：

- `wing_slider` 不再对未 pressed 的 `WING_EVENT_POINTER_MOVE` 改值。
- `wing_scrollbar` 不再对未 pressed 的 `WING_EVENT_POINTER_MOVE` 改值。
- slider / scrollbar 现在只在 pointer down，或 pressed/captured drag 的 pointer move / pointer up 中根据坐标更新 value。

设计意义：

- mouse hover 只负责 hovered state 和 pointer enter / leave，不会误触发 value widget 行为。
- touch drag 仍然通过 pointer down 开启 pressed/capture，再通过 move/up 更新 value。
- 这让桌面鼠标语义与触摸拖动语义进一步分离。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证 synthetic touch slider/scrollbar drag、capture、key、encoder 仍通过。
- 注入 X11 mouse move，验证 hover 仍输出 `source=mouse`，且未按下 mouse move 不再触发 slider/scrollbar value changed。
- 关闭 X11 窗口，验证 demo 退出并回到 NSH。

## 2026-06-12 更新：Input Source 归一化兜底

本轮继续强化 WING input source 语义，避免应用或平台 adapter 漏填 `source` 时让事件行为变得不确定。

完成内容：

- WING core input 新增 source normalize helper。
- `WING_INPUT_KEY_DOWN` / `WING_INPUT_KEY_UP` 的 unknown source 自动归一为 `keyboard`。
- `WING_INPUT_ENCODER_ROTATE` 的 unknown source 自动归一为 `encoder`。
- `WING_INPUT_CLOSE_REQUEST` 的 unknown source 自动归一为 `system`。
- pointer unknown 保持 unknown，并继续按兼容策略允许 hover。
- `wing_gui_enqueue_input()`、`wing_input_adapter_store_pending()`、`wing_gui_dispatch_input()` 都会走归一化路径。

设计意义：

- 应用层或平台 adapter 少填 source 时，key / encoder / close request 仍能进入稳定语义。
- pointer source 仍保留 mouse/touch 的显式差异，同时 unknown 保持旧应用兼容。
- 这让 WING GUI 的输入模型更适合后续接入真实 NuttX touch / mouse / keyboard adapter。

验证计划：

- 重新构建 sim。
- 从 NSH 执行 `wing_gui_demo`。
- 验证现有 mouse / touch / keyboard / encoder / system 路径不回退。
- 注入 X11 mouse move，验证 mouse hover 仍可用且未按下 move 不改变 slider/scrollbar value。
- 关闭 X11 窗口，验证 demo 退出并回到 NSH。

### 本轮更新：普通 Widget Style 进入默认空间投影路径

- 已确认不新增 `wing_3d.c` / `wing_3d_view.c`，默认 3D 能力继续收敛在 WING object space、runtime camera、projection、dirty 和 render path 中。
- `wing_widget_draw_style_for_obj()` 已开始识别 object 的 world space transform：identity transform 保持原有 `fill_rect` 快路径，非 identity transform 会把传入 style rect 投影为 quad 并提交 `wing_gui_fill_quad()`。
- 这意味着 `wing_box` / panel / button / slider track / scrollbar thumb 等复用公共 box style 的普通 widget，不再需要变成专门的 3D widget，也可以自然进入默认空间平面投影路径。
- 当前 FRender 还没有 quad stroke/path stroke，所以非 identity style stroke 暂时使用 projected bounds 做保守描边；后续应在 FRender 增加 quad/path stroke primitive 后替换。
- `wing_gui_demo` 已给普通 `fill_panel` 设置 space transform，用于验证普通 box 也能通过共享 style helper 进入 projected quad 绘制路径，而不是只有 `wing_card` 具备空间效果。

### 本轮更新：FRender Quad Stroke 与普通 Widget 空间描边

- FRender 已新增 `FR_CMD_STROKE_QUAD` / `FR_CAP_STROKE_QUAD` / `FR_DRAW_CAP_STROKE_QUAD`，software backend 当前用四条 clipped line fallback 完成 quad 描边。
- WING render API 已新增 `wing_gui_draw_quad()`，把 projected quad stroke 作为 Render backend command 提交，而不是在 WING widget 层自己退化为轴对齐 bounds 描边。
- `wing_widget_draw_style_for_obj()` 的非 identity object space 路径现在同时支持 projected quad fill 和 projected quad stroke；普通 `wing_box`、panel、button、slider track、scrollbar thumb 等复用 box style 的 widget 都能共享默认空间绘制路径。
- 这一步继续避免新增 `wing_3d_view` 等外挂概念：空间效果属于 object space + FRender command，不属于某个专用 widget。
- 当前 quad stroke 是 Phase 1 software fallback，后续可由 FRender path/stroke、mesh/shader 或 GPU backend 接管。

### 本轮更新：frender_demo 覆盖 Stroke Quad

- `frender_demo` 已把 `FR_DRAW_CAP_STROKE_QUAD` 纳入可读能力输出，software backend 的 draw caps 会显示 `stroke_quad`。
- `frender_demo` 已新增 `stroke_quad` stage，直接从 NSH 验证 `fr_cmd_stroke_quad()`、FRender command list、software fallback 和 framebuffer present 链路。
- 这让 WING 普通 widget 的非 identity style stroke 不再只是 WING 侧使用新 API，而是在底层 FRender demo 中也有独立可观察验证点。
- 已收紧 `wing_text_input_t` 的键盘语义：文本输入控件只把可打印字符交给 `wing_text_edit_t`，方向键 / Backspace / Delete 继续作为编辑键处理，Enter 等非文本控制键不会污染固定单行 buffer；`wing_gui_demo` 已加入 text input key-down 日志和 Enter 控制键验证脚本。
- 已新增项目内 X11 输入验证工具 `tools/firmware/sim/x11-input.sh` / `x11-input.c`，用于在没有 `xdotool` 的环境中自动查找 `NuttX` X11 窗口并发送 move / click / key / close 动作。当前已通过 `XWarpPointer` 验证真实 X11 pointer move 可以被 simulator framebuffer 捕获并进入 `wing_gui_demo` 的 FRender presenter input path；close request 也可通过同一工具验证。由于当前环境缺少 XTest，click/key 的真实 X11 注入仍是 best-effort XSendEvent，后续要补 XTest 或等效输入后端来完成键盘/点击自动化强验证。
- `tools/firmware/sim/x11-input.sh` 已支持按环境自动启用 XTest：若系统存在 `X11/extensions/XTest.h` 和 `libXtst`，click/key 会通过 `XTestFakeButtonEvent` / `XTestFakeKeyEvent` 走真实输入注入；当前环境没有 XTest，因此 click/key 仍只保留 best-effort XSendEvent，强验证范围仍是 X11 pointer move 和 close request。已重新构建并从 NSH 执行 `wing_gui_demo`，确认 pointer move 进入 `x11 input provider`，close request 进入 WING input/event queue 并回到 NSH。

### 本轮更新：X11 Client Window 输入验证通过

- `tools/firmware/sim/x11-input.c` 的窗口查找策略已改为优先选择最深层匹配窗口，避免把事件发给窗口管理器装饰框而不是 NuttX framebuffer client window。
- 已重新构建 `sim-wing`，并从 NSH 执行 `wing_gui_demo` 完成真实 X11 验证。
- `x11-input.sh NuttX move 260 132` 现在进入 WING input provider 时保持精确 client 坐标：`type=pointer_move source=mouse point=260,132`。
- `x11-input.sh NuttX click 50 86 1` 已能产生 `pointer_down` / `pointer_up`，并触发 WING click bubbling：日志出现 `card received bubbled click and stopped propagation`。
- `x11-input.sh NuttX key Right` 已能产生 `type=key_down source=keyboard key=1001`，真实 X11 键盘事件进入 WING input provider。
- `x11-input.sh NuttX close` 已验证关闭路径：framebuffer window closed -> WING close request event -> app task exit -> 回到 `nsh>`。
- 当前环境仍没有 XTest 头/库，但 client window 命中修正后，普通 Xlib/XSendEvent 模式已经足够覆盖本轮 move / click / key / close 自动化验证；后续仍可保留 XTest fast path 作为更接近真实桌面输入的增强路径。

### 本轮更新：WING Card 边缘绘制收敛到 Object Space Quad

- 已继续收敛 `wing_card_t` 的实现语义：它不是独立 3D view，也不是单独的 3D 子系统，而是使用默认 object space 能力的普通 widget。
- `wing_card` 的 edge stroke 不再退化为轴对齐 screen rect，而是通过 `wing_gui_draw_quad()` 提交 FRender quad stroke command。
- `wing_card` 的 edge fill 不再使用 `fill_rect`，而是构造一个很薄的 projected ridge quad，并通过 `wing_gui_fill_quad()` 提交。
- 这让 `wing_card` 的 front/back/edge 都沿着 object space projection -> WING render API -> FRender command 的同一条路径执行，后续 mesh / shader / GPU3D backend 接入时不需要为 card 保留特殊 3D 绘制分支。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证：默认空间、动画、timer、dirty redraw、present、projected hit-test、click bubbling 均正常。
- 已再次使用 `x11-input.sh` 验证真实 X11 move / click / key / close：输入进入 WING input provider，click 触发 bubbling，close 触发 `WING_EVENT_CLOSE_REQUEST` 并回到 `nsh>`。

### 本轮更新：Label Glyph 绘制进入默认 Object Space

- 已新增 `wing_widget_fill_rect_for_obj()` 共享 helper：调用者给出 object + rect，helper 会在 object world space transform 非 identity 时投影为 quad 并提交 `wing_gui_fill_quad()`，否则保持 `wing_gui_fill_rect()` 快路径。
- `wing_widget_draw_style_for_obj()` 的 fill 路径开始复用该 helper，避免 box style 与其他 widget 各自维护 rect projection 逻辑。
- `wing_label` 的 bitmap glyph pixel 绘制已从直接 `wing_gui_fill_rect()` 改为 `wing_widget_fill_rect_for_obj()`；因此普通 label 自身或父对象带默认 object space transform 时，文字像素也能通过 projected quad fill 进入 FRender command path。
- `wing_gui_demo` 的 wrap label 已设置轻量 `space_transform`，用于验证普通文本不是 2D 特例外挂，而是默认空间中的普通 object。
- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo` 验证：日志出现 `wrap label uses default object space rotation_y=10 z=6 and bitmap glyph pixels render through shared projected quad fill helper`，同时 timer / animation / dirty redraw / present / event / projected hit-test 均正常。
- 已使用 `x11-input.sh` 再次验证真实 X11 move / click / key / close：输入进入 WING input provider，click 触发 bubbling，close 触发 WING close request 并回到 `nsh>`。

## 2026-06-12 更新：默认 3D/object space 不再拆成独立模块

本次确认 WING GUI 不需要新增或保留独立 `wing_3d.c` / `wing_3d_view.c` 路径。3D 能力应作为 WING GUI core 的默认 object space 能力存在：普通 2D 控件是默认 3D 空间下的 identity transform 状态，空间效果通过 `wing_space.c`、object space transform、runtime camera、projection、dirty 和 render frontend 统一表达。

当前代码方向：

- `src/core/wing_space.c` 负责默认空间、投影、深度与 geometry helper。
- `wing_obj_t` 持有 object space transform，普通 widget 默认 identity。
- `wing_gui_t` 持有 runtime camera，GUI 应用通过同一 camera 推进 2D/3D UI。
- `wing_card`、projected image quad、custom triangle 都是普通 object/widget 绘制路径，不是专用 3D view。
- triangle demo 已改为 runtime camera + core space projection -> WING render frontend -> FRender `fill_triangle` command。

验证结果：

- 已重新构建 `sim-wing`。
- 已从 NSH 执行 `wing_gui_demo`。
- demo 日志确认 triangle primitive 使用 runtime camera + core space projection，而不是独立 3D widget。
- X11 pointer move / click / keyboard Right 可进入 WING input provider 和事件系统。
- 点击 X11 窗口关闭按钮后，WING 收到 close request，demo 退出并回到 NSH。

后续原则：

- 不新增 `wing_3d.c`、`wing_3d_view.c`、`wing_3d_widget` 这类外挂式命名。
- 后续 mesh、shader、GPU 3D 后端仍接入同一套 object space / camera / render command 数据模型。
- 如果需要专门展示空间能力，应放在 demo 或普通 widget 的 draw path 中，而不是创建独立 3D 子系统。

## 2026-06-12 更新：triangle projected bounds 纳入 core space

本次继续推进默认 3D/object space 与 dirty 刷新模型的闭环。此前 FRender 和 WING render 已有 triangle primitive seed，本次把 triangle 的保守屏幕 bounds 计算收敛到 WING core space，避免未来 mesh / shader / GPU3D 路径在 demo、widget 或 backend 中各自维护脏区估算。

新增能力：

- `wing_triangle2d_get_bounds()`：对 screen-space triangle 计算保守 bounds。
- `wing_projected_triangle_get_bounds()`：对带 screen/depth 的 projected triangle 计算保守 bounds。
- `wing_project_triangle_bounds()`：从 local 3D triangle points 经过 camera / object-space transform 直接得到 conservative screen dirty bounds。

同步调整：

- `wing_gui_demo` 的 custom triangle object 继续作为普通 object draw path，不新增 `wing_3d_view`。
- demo 抽出 triangle projection helper，draw path 和日志共用 runtime camera + core space projection。
- 修正 demo triangle 定位：根据 camera depth / focal length 从目标屏幕中心反推 object-space translation，避免把屏幕坐标直接当世界坐标导致 projected bounds 偏离。

验证结果：

- 已重新构建 `sim-wing`。
- 已从 NSH 执行 `wing_gui_demo`。
- 日志确认 `projected triangle dirty bounds x=296 y=71 w=78 h=90 depth0=270 depth1=274 depth2=266 through core space`。
- synthetic input、timer、repeat timer、animation、lifecycle、dirty redraw、present rect 和 framebuffer present 正常。
- 真实 X11 pointer move / click / keyboard Right 进入 WING input provider。
- X11 close 后 WING 收到 close request，demo 退出并回到 NSH。

## 2026-06-12 更新：custom object screen bounds callback seed

本次把 triangle projected bounds 从“demo 可观测日志”推进到 WING object / dirty 模型中。WING object 新增可选 screen bounds callback，`wing_obj_get_screen_bounds()` 会优先使用对象自定义 bounds，再回退到默认 object bounds + world space transform + camera projection。

新增能力：

- `wing_obj_screen_bounds_fn_t`：对象可提供自定义 conservative screen bounds。
- `wing_obj_set_screen_bounds_cb()`：应用或 widget 可注册自定义 screen bounds callback。
- `wing_obj_get_screen_bounds()` 现在统一承载默认 projected rect bounds 和 custom primitive bounds。

当前用途：

- `wing_gui_demo` 的 custom triangle object 注册 screen bounds callback。
- triangle object 的 dirty / culling / clip 入口现在通过 projected triangle bounds，而不是只靠本地矩形 bounds。
- 该机制为后续 mesh、shader、vector path、复杂 widget 和非矩形 primitive 的 dirty planning 提供了统一入口。

验证结果：

- 已重新构建 `sim-wing`。
- 已从 NSH 执行 `wing_gui_demo`。
- 日志确认 `custom object screen bounds callback reports projected triangle dirty bounds x=296 y=71 w=78 h=90 depth0=270 depth1=274 depth2=266 through core space`。
- synthetic input、timer、repeat timer、animation、lifecycle、dirty redraw、present rect 和 framebuffer present 正常。
- 真实 X11 pointer move / click / keyboard Right 进入 WING input provider。
- X11 close 后 WING 收到 close request，demo 退出并回到 NSH。

## 2026-06-12 更新：默认 3D 语义下的自定义几何 bounds / hit-test 种子

本次推进继续遵循“WING 默认就是 3D GUI，2D 只是默认相机下的平面对象”原则，不再把 3D 作为独立扩展模块或特殊 widget 拆出。

已完成内容：

- 在 WING object core 中加入自定义 screen bounds 回调机制，用于让非矩形 projected primitive 提供自己的保守屏幕脏区。
- 在 WING object core 中加入自定义 contains point 回调机制，用于让非矩形 projected primitive 提供精确命中测试。
- 在 core space 中加入 triangle projected geometry 判断能力，使三角形对象可以基于投影后三角形进行 hit-test。
- `wing_gui_demo` 中的自定义三角形对象已经使用 runtime camera + core space projection，并通过 WING render frontend 提交到 FRender triangle primitive。
- `wing_gui_demo` 日志已经展示 projected triangle dirty bounds，以及 projected triangle 精确 hit-test：顶点命中为 yes，保守 bounds 左上角命中为 no。
- `tools/firmware/sim/x11-input.c` 增强了窗口关闭验证工具，新增 `frameclose` 动作，并让 `close` 尽量定位到支持 `WM_DELETE_WINDOW` 的 client/top-level 窗口。

验证结果：

- 已重新构建 `sim-wing`，构建成功。
- 已从 NSH 执行 `wing_gui_demo`，demo 成功进入 WING GUI app loop。
- 日志确认 timer、repeat timer、animation、camera event、space transform event、dirty list、present rect、widgets、focus、keyboard/pointer synthetic input、projected quad hit-test、same z-index depth order、custom triangle primitive 都已进入当前 demo 展示路径。
- X11 窗口树确认存在外层 window manager frame 与内层 NuttX client 窗口。
- 当前环境未安装 `xdotool` / `wmctrl`，并且 `x11-input` 未编入 XTest 支持，因此自动化脚本无法可靠模拟“点击窗口管理器 X 按钮”。本次没有把“点 X 后回到 NSH”标记为自动化验证通过，后续需要补齐 XTest 依赖或增加更可靠的宿主窗口关闭工具。

下一步计划：

- 不引入 `wing_3d.c` / `wing_3d_view.c` 这类把 3D 当扩展的文件结构；将 camera、projection、geometry、transform、depth/hit-test 能力继续收敛到 `core/wing_space.*`、`core/wing_object.*`、`core/wing_render.*` 等默认核心路径。
- 继续把 custom geometry 的 bounds / hit-test / draw callback 机制抽象为 WING 原生对象能力，为后续 mesh、shader、GPU 3D backend 做铺垫。
- 补齐 X11 自动化验证工具，确保真实 close path 可稳定验证，避免再次把运行时验证卡在宿主窗口工具缺失上。

## 2026-06-12 更新：X11 close 验证闭环与 sim framebuffer 关闭健壮性

本次推进补齐了上一轮留下的 close 自动化验证缺口，并修复了 NuttX sim framebuffer 在窗口关闭与 present 并发时可能被 Xlib 默认错误处理器终止的问题。

已完成内容：

- `tools/firmware/sim/x11-input.c`
  - `close` 保持标准窗口关闭路径：先发送 `WM_DELETE_WINDOW`，再发送 `_NET_CLOSE_WINDOW`。
  - 新增单独的 `destroy` 动作用于明确测试 synthetic `DestroyNotify`，但不再作为普通 `close` 的默认 fallback，避免把真实窗口关闭和测试注入混在一起。
  - 保留 `frameclose` 动作，后续如果宿主有 XTest 支持，可继续用于模拟真实点击窗口管理器关闭按钮。

- `nuttx/arch/sim/src/sim/posix/sim_x11framebuffer.c`
  - `sim_x11update()` present 前先调用 `sim_x11pollwindowclosed()`。
  - present 阶段临时安装 X error handler，捕获 `BadDrawable` / `BadWindow`，将其转为 `g_window_closed = true` 与 `-ESHUTDOWN` 返回，而不是让 Xlib 默认 abort 整个 sim。
  - 该修复让窗口关闭和 framebuffer present 并发时更健壮，符合 WING GUI 后续持续自动化验证需求。

验证结果：

- 已重新构建 `sim-wing`，构建成功。
- 从 NSH 执行 `wing_gui_demo`，demo 成功打开 X11 framebuffer 窗口并进入 WING GUI app loop。
- 日志确认默认 camera、core space、projected quad、custom projected triangle bounds/hit-test、FRender triangle primitive、widget tree、timer、repeat timer、animation、lifecycle、dirty list、present rect、focus、pointer capture、keyboard/pointer input 等路径均被覆盖。
- 通过 `x11-input` 注入真实宿主 X11 输入，日志确认：
  - `pointer_move` 进入 WING input provider。
  - `pointer_down` / `pointer_up` 进入 WING，并触发 bubbled click。
  - `key_down` 进入 WING input provider。
- 执行 `x11-input close` 后，日志确认：
  - `framebuffer window closed`
  - `root received close request through WING input/event queue`
  - `app task exit`
  - 返回 `nsh>`
- `poweroff` 后出现一次 gcov profile checksum 提示，属于旧 profile 覆盖提示，不影响本次功能验证。

下一步计划：

- 继续把 3D 作为默认核心语义，而不是独立 `wing_3d` 模块或 `wing_3d_view` 控件。
- 将更多 geometry/object 能力收敛为 WING object 的默认能力：geometry bounds、precise hit-test、draw callback / render node、depth sort、future mesh/material。
- 后续实现时继续保持每次改动后构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo`，同时验证输入、输出、事件、动画、定时器、dirty、close 回到 NSH。

## 2026-06-12 更新：custom geometry 作为普通 WING object 的稳定性修正

本次推进继续确认 WING 的 3D 能力不是独立扩展模块，而是 object tree 的默认空间语义。当前 custom projected triangle 已经不是 `wing_3d_view` 这类特殊控件，而是普通 `wing_obj_t` 通过统一 object callback 接入：

- `draw` callback：提交自定义几何绘制命令。
- `screen_bounds` callback：向 dirty 系统报告 projected geometry 的保守屏幕 bounds。
- `contains_point` callback：向 hit-test 系统报告 projected geometry 的精确命中判断。

已完成内容：

- 修正 `wing_obj_init()`，显式初始化 `screen_bounds = NULL` 与 `contains_point = NULL`。
- 该修正避免栈上或静态对象初始化后携带未定义 callback 指针，从而导致 dirty bounds / hit-test 走到随机地址。
- `wing_gui_demo` 增加日志说明：custom geometry 是普通 WING object 使用 draw/screen-bounds/hit-test callbacks 接入，不是单独的 3D view。

验证结果：

- 已重新构建 `sim-wing`，构建成功。
- 从 NSH 执行 `wing_gui_demo`，demo 成功进入 app loop。
- 日志确认：默认 camera、core space、projected quad、custom triangle screen bounds、custom triangle precise hit-test、FRender triangle primitive、widgets、timer、animation、lifecycle、dirty/present 全部跑通。
- 真实 X11 输入验证通过：pointer move/down/up 与 keyboard Right 进入 WING input provider，并触发 click / dirty / present 路径。
- X11 close 验证通过：`framebuffer window closed` -> `root received close request through WING input/event queue` -> `app task exit` -> 返回 `nsh>`。

下一步计划：

- 继续把 geometry/bounds/hit-test/draw 视为 WING object 的核心能力，为后续 mesh/material/shader 做铺垫。
- 后续可以把 custom geometry 的常用模式沉淀为更明确的 geometry descriptor，但不要拆成 `wing_3d.c` 或 `wing_3d_view.c`。
- 继续保持每次代码改动后构建并运行 `wing_gui_demo` 的闭环验证。

## 2026-06-12 更新：默认 3D 语义与自定义几何脏区修正

- WING GUI 的方向确认：不再把 3D 作为 `wing_3d.c` 或 `wing_3d_view.c` 这类独立扩展模块维护，而是把 3D 作为核心对象空间语义。
- 2D widget 被视为默认 3D 空间中的特殊情况：默认 camera、z=0 平面、identity space transform。
- 自定义几何也应是普通 `wing_obj_t`：通过 draw/screen-bounds/hit-test callbacks 接入 object tree、event dispatch、dirty、render frontend 和 FRender。
- 已修正 `wing_obj_set_screen_bounds_cb()`：对象接入 GUI 后更新 screen-bounds callback 时，会同时脏化旧 bounds 和新 bounds，避免投影几何缩小/移动后残影。
- 已修正 `wing_obj_set_user_data()`：user_data 影响自定义投影/绘制/命中时，也会同时脏化旧 screen bounds 和新 screen bounds。
- `wing_gui_demo` 已补充日志，明确自定义 geometry 的 screen-bounds/user-data 更新会覆盖旧新 projected bounds dirty 语义。
- 已重新构建 `sim-wing`，并从 NSH 执行 `wing_gui_demo` 验证通过：X11 输出、真实 pointer/key 输入、dirty/present、animation/timer、close request 回到 NSH 均正常。

下一步：

- 收敛 WING 源码结构，避免出现独立 3D view/component 边界；如果已有 `wing_3d.c` / `wing_3d_view.c`，应将其中仍有价值的数学/投影能力并入 core object-space/projection/render frontend，再删除或停用这些文件。
- 继续把 mesh、shader、GPU 3D 预留为 FRender / NuttX graphics 后端能力，而不是 WING GUI 上层应用模型的特殊分支。

## 2026-06-12 更新：默认 2D 状态显式进入 core space API

本次检查确认当前源码中已经没有 `src/core/wing_3d.c` 和 `src/widgets/wing_3d_view.c`，Makefile 也没有引用独立 3D 模块。为了把这一架构约束继续固化，新增 `wing_space_transform_is_default_2d()`：

- 默认 2D 不再被表达为独立兼容模式，而是默认 object space 中的 identity transform 状态。
- 普通 widget、card、custom triangle、未来 mesh/shader/GPU3D 都继续共享 object space、runtime camera、projection、dirty 和 FRender command path。
- 后续如果需要判断对象是否处于默认 2D plane 状态，应使用 core space API，而不是新增 `wing_3d_view` 或类似特殊组件。

下一步：

- 继续把 widgets 的空间绘制路径收敛到 shared projected quad / triangle helpers。
- 继续扩展 FRender primitive/capability，为 mesh/material/shader 做后端种子，但不改变 WING app 的普通 object/widget 模型。

验证结果：

- 已重新构建 `sim-wing` 并从 NSH 执行 `wing_gui_demo`。
- 已验证 X11 framebuffer 输出、真实 pointer/key 输入、WING event queue、timer、animation、dirty/present、窗口 close request 和回到 NSH 的完整链路。

## 2026-06-12 更新：对象级默认 2D 平面语义 API

本次继续把默认 3D/object space 能力收敛到 WING core，而不是拆成独立 3D view。新增 `wing_obj_is_default_2d()`：

- `wing_space_transform_is_default_2d()` 是 core space 的 transform 级判断。
- `wing_obj_is_default_2d()` 是 object 层语义判断，用于表达对象是否处在默认 camera/object space 下的 2D 平面状态。
- 当前 default 2D 语义等价于 identity space transform；后续如果默认平面语义需要扩展，也可以集中调整 core space/object API，不需要改 app/widget 的概念模型。
- `wing_gui_demo` 已输出 root object default 2D plane 判断，便于从 NSH 日志确认该语义路径。

下一步：

- 继续让普通 widget 绘制路径优先使用 object-level space/default-2D 语义，而不是直接判断某个私有 3D 模式。
- 后续 mesh / material / shader 接入时仍沿 object + render command/backend 路线推进。

## 2026-06-12 更新：demo 增加循环动画验证

根据最新目标，`wing_gui_demo` 需要同时验证单次动画和循环/重复动画。本次在现有一次性 `line_primary` width animation 和 `space_card` transform animation 基础上，新增 `line_secondary` ping-pong width loop animation：

- 第一段从 min width 动画到 max width。
- done callback 中自动启动第二段，从 max width 动画回 min width。
- 完成指定 segment 数后停止，并输出每段完成日志。
- 该路径验证 animation runtime 的 apply、done、restart、bounds changed、dirty/present 链路。

这与 repeat timer 分开验证：repeat timer 继续验证软件定时器重复触发和主动 stop；loop animation 验证动画完成回调驱动的重复动画形态。

验证结果：

- `wing_gui_demo` 已重新通过 sim/NSH 验证。
- 本次验证明确覆盖：合成输入操作可交互控件、真实 X11 pointer/key 输入、单次动画、循环动画、repeat timer、dirty/present、窗口关闭回到 NSH。

## 2026-06-12 更新：progress 纳入可交互输入验证

根据最新目标，`wing_gui_demo` 的模拟输入需要尽量触发所有可触发事件，而不仅是展示控件。本次将 `wing_progress_t` 从纯显示型 value widget 扩展为可交互 value widget：

- progress 支持 pointer down/move/up 根据横向位置更新 value。
- progress 支持 keyboard left/right/up/down 按 step 调整 value。
- progress 支持 encoder rotate 按 step 调整 value。
- progress 支持 event callback，demo 可记录 `VALUE_CHANGED`、focus、pointer capture/release、key、encoder、pointer up 等事件。
- `wing_gui_demo` synthetic input 新增 progress pointer drag、keyboard right、encoder rotate，保证 progress 也被模拟输入真实操作。

这一步继续复用 WING value model，与 slider / scrollbar 的 value 更新、事件派发和 dirty/present 机制保持一致。

## 2026-06-12 本轮计划更新：默认 3D/object space 文件边界

本轮确认并验证：WING GUI 的 3D 能力不是外挂扩展，也不是独立 `wing_3d_view` 控件。当前工作树不存在 `wing_3d.c` / `wing_3d_view.c`，默认 3D/object space 能力已经放在 core space、object transform、runtime camera、dirty bounds、hit-test 和 render frontend 路径中。

已完成验证：

- `sim-wing` 重新构建通过。
- NSH 执行 `wing_gui_demo` 通过。
- demo 覆盖默认 2D plane、runtime camera、projected quad、space card、custom triangle、dirty/present、timer、一次性动画、loop 动画、button/progress/slider/scrollbar/text input/checkbox/switch/scroll view 事件。
- X11 真实 pointer/key 输入进入 WING input provider。
- X11 close 触发 WING close request，demo 退出并回到 NSH。

后续计划保持：

- 继续把 3D 作为 WING GUI core object space 能力推进。
- 不新增 `wing_3d.c` / `wing_3d_view.c` 这类会让 3D 变成特殊插件或特殊 widget 的边界。
- 后续 mesh / shader / GPU 3D backend 统一接入 FRender 和 NuttX graphics 能力声明，不改变 WING app 的 object/camera/transform 模型。

## 2026-06-12 更新：统一 geometry callback 作为普通 object 能力

本轮新增 `wing_obj_set_geometry_cb()`，把 custom geometry 所需的 draw / screen-bounds / hit-test 三件套收敛为普通 object 的统一几何入口。这样自定义三角形、未来 mesh、material 或 shader path 都可以继续沿 `wing_obj_t -> object space -> render frontend -> FRender` 路径推进，而不是拆成 `wing_3d_view`。

已完成验证：

- `sim-wing` 重新构建通过。
- NSH 执行 `wing_gui_demo` 通过。
- demo 覆盖 unified geometry callback、custom projected triangle、default object space、runtime camera、dirty/present、timer、一次性动画、loop 动画、可交互 widget 事件。
- X11 真实 pointer/key 输入进入 WING input provider。
- X11 close 触发 WING close request，demo 退出并回到 NSH。

下一步计划：

- 继续把 geometry descriptor / render node / material seed 收敛到普通 object 能力。
- 不新增独立 3D view；后续 3D backend 能力应从 FRender capability 和 NuttX graphics 能力声明进入。

## 2026-06-12 更新：对象层排序与 Desktop 层管理边界

- 已确认：WING GUI core 的基础层级排序已实现为 object tree sibling ordering，不只是父子关系。
- 已确认：同父节点对象按 `z_index` 与 projected depth 参与绘制和 hit-test 排序。
- 已修正：`wing_gui_demo` 将 progress/slider/scrollbar 放入显式 control z-layer，避免装饰性 space/depth card 覆盖可交互 value 控件。
- 已验证：`wing_gui_demo` 在 NSH 中启动、打开 X11 framebuffer、响应真实 X11 slider/scrollbar 拖动、关闭窗口后返回 NSH。
- 下一步计划：保持 WING GUI 的 object-level z/depth 排序简洁稳定；等进入 WING Desktop 时再引入 window-level layer manager，包括普通窗口、置顶窗口、modal、overlay、tooltip、cursor 与 compositor 顺序。

## 2026-06-12 更新：object layer seed 与 value 控件状态绘制

- 已完成：在 WING GUI public API 中加入 object-level layer seed，避免 demo/app 继续散落裸 `z_index` 魔法数。
- 已完成：`wing_gui_demo` 的可交互 value 控件使用 `WING_OBJ_LAYER_CONTROL`，装饰性 space card 使用 `WING_OBJ_LAYER_DECORATION`。
- 已完成：slider/scrollbar 的 focused state style 从 filled panel 改为 stroke-only focus ring，避免拖动时状态背景覆盖轨道/滑块或造成局部刷新视觉污染。
- 计划保持：object-level layer seed 只解决 GUI 对象树内排序语义；WING Desktop 阶段继续单独设计 window/modal/overlay/cursor/compositor layer manager。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，验证 synthetic 输入、真实 X11 slider/scrollbar click/key、事件、timer、单次动画、loop 动画、dirty/present、窗口关闭回到 NSH。

## 2026-06-12 更新：focused ring 约束纳入 demo 验证

- 已完成：`wing_gui_demo` 输出 value 控件 focused ring 的 fill/stroke 约束，明确 slider/scrollbar focused state 是 stroke-only。
- 已验证：日志显示 slider/scrollbar focused style 均 `has_fill=no`、`has_stroke=yes`，避免拖动时整块状态背景覆盖控件内容。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 slider/scrollbar click/key、事件、timer、单次动画、loop 动画、dirty/present、关闭窗口回到 NSH。
- 后续计划：继续把这类视觉/交互约束转成 demo 可观测日志，减少仅靠肉眼截图判断的问题。

## 2026-06-12 更新：value 控件状态样式分阶段绘制

- 已完成：WING widget style helper 增加 background/overlay 分阶段绘制入口。
- 已完成：slider/scrollbar 的状态样式不再作为单块 box 最后绘制，而是 fill/clear 先于控件内容，stroke 后于控件内容。
- 计划保持：后续新增可拖拽 value widget 时应复用同一契约，避免 focused/pressed/hovered 背景覆盖自身内容或污染 dirty 局部刷新结果。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 输入、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。

## 2026-06-12 更新：value input behavior/mixin 种子

- 已完成：新增内部 value input behavior helper，吸收 TouchGFX mixin/behavior 的复用思想，但先不公开为稳定应用 API。
- 已完成：progress、slider、scrollbar 共享 key/encoder step 输入处理，减少三个 value 控件内部重复逻辑。
- 计划保持：后续把 pointer drag、pressed lifecycle、capture/release 等继续提取为更清晰的 WING behavior 层，但每一步都必须保持 demo 可观测和 sim 可验证。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 输入、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。

## 2026-06-12 更新：pointer value drag behavior/mixin 种子

- 已完成：新增内部 pointer drag behavior helper，收敛 progress/slider/scrollbar 的 pointer down/move/up/cancel 生命周期。
- 已完成：三个 value 控件共享 pressed/capture/update/release 状态机，但保留各自 point-to-value 几何映射。
- 计划保持：behavior 层先服务内部 widget 去重，后续再评估是否形成公开 `behaviors/` API。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 输入、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。

## 2026-06-12 更新：behaviors 目录分层

- 已完成：新增 `src/behaviors` 分层，并将 value input / pointer drag behavior 种子移出 `src/core`。
- 已完成：Makefile 增加 behaviors VPATH，progress/slider/scrollbar 更新为引用 behaviors 头文件。
- 计划保持：core 只放 GUI runtime 与基础机制；可复用 widget 行为逐步进入 behaviors；产品级组合控件后续进入 containers/widgets 边界。
- 原则补充：默认 3D/object space 是 WING 的核心原则，也是提取 LVGL / TouchGFX / HoneyGUI 优秀特性的适配前提；外部机制进入 WING 前应先被翻译为 object tree、runtime camera、space transform、dirty/event、render backend 的统一语义。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 输入、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。

## 2026-06-12 更新：Render node / material seed

- 已完成：新增 WING render node/material seed API，提供 `wing_render_node_t`、`wing_render_material_t` 与 `wing_gui_submit_render_node()`。
- 已完成：render node 第一阶段覆盖 fill rect、fill quad、fill triangle、draw quad、blit、blit quad，并统一 dispatch 到现有 FRender command path。
- 已完成：`wing_gui_demo` custom projected triangle 改为通过 `WING_RENDER_NODE_FILL_TRIANGLE` 提交，证明自定义几何可以沿普通 object space -> render node/material -> FRender 路径工作。
- 计划保持：render node 是未来 geometry descriptor / material / mesh / shader 的入口种子，不是独立 3D view；后续 mesh / shader / GPU 3D backend 继续扩展 FRender capability 与 NuttX graphics backend，而不改变 WING app 的 object/camera/transform 模型。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 slider/scrollbar click、keyboard Right、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。

## 2026-06-12 更新：Render node 头文件边界整理

- 已完成：新增 `include/wing/core/wing_render_node.h`，将 render node/material 类型从 `<wing/wing.h>` 主体移入 core render frontend 边界。
- 已完成：`<wing/wing.h>` 继续聚合 render node 子头，保持应用侧 include 兼容。
- 计划保持：后续新增 material、mesh、shader descriptor 时优先扩展 render node/render frontend 子头，不继续把专属结构堆进聚合头主体。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 slider/scrollbar click、keyboard Right、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。

## 2026-06-12 更新：Render frontend 函数声明边界整理

- 已完成：`<wing/wing.h>` 不再直接重复声明 render frontend 函数，改为聚合 `wing/core/wing_render.h`。
- 已完成：render frontend API 边界继续归入 `wing_render.h`，包括 surface/context、clip、primitive、render node/material、blit、software flush、checksum 和 dirty render。
- 计划保持：聚合头负责兼容与一站式包含；专属模块头负责功能边界。后续 runtime / theme / timer / animation 等也应逐步按同样原则收敛，避免 `wing.h` 主体继续膨胀。
- 已验证：重新构建 sim，并从 NSH 执行 `wing_gui_demo`，覆盖 synthetic 输入、真实 X11 slider/scrollbar click、keyboard Right、timer、单次动画、loop 动画、dirty/present 与关闭窗口回到 NSH。
