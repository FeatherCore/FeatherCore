# WING GUI Phase 1 验收清单：Runtime Closure

更新时间：2026-06-12

本文用于定义 WING GUI Phase 1 的闭环验收标准。

Phase 1 的目标不是继续堆功能，而是证明 WING GUI 作为一个独立 GUI 库已经具备稳定的 runtime 闭环，可以从 NSH 启动、打开显示、接收输入、推进事件/动画/脏区/渲染，并通过 FRender 和 NuttX graphics 完成显示。

相关文档：

- `FeatherCore/docs/plans/WING_GUI_CORE_DESIGN_ROADMAP_ZH.md`
- `FeatherCore/docs/plans/WING_GUI_PROGRESS_AND_PLAN_ZH.md`
- `FeatherCore/docs/progress/WING_GUI_PROGRESS_AND_UPDATE_PLAN_ZH.md`
- `FeatherCore/docs/WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md`
- `FeatherCore/docs/WING_GUI_DESIGN_REFERENCES_ZH.md`
- `FeatherCore/docs/FRENDER_IMPLEMENTATION_ZH.md`
- `FeatherCore/docs/FEATHER_RENDER_CORE_DECISION_ZH.md`

## 1. Phase 1 定义

阶段名称：

```text
WING GUI Phase 1: Runtime Closure
```

中文名称：

```text
WING GUI 第一阶段：运行时闭环
```

阶段闭环：

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

## 2. Phase 1 完成条件

Phase 1 完成必须同时满足：

1. WING GUI 可以作为独立 GUI 库运行，不依赖 WING Desktop。
2. `wing_gui_demo` 可以从 NSH 启动并打开 X11 framebuffer 窗口。
3. 点击/关闭 X11 窗口可以进入 WING input/event queue，并退出 demo 回到 NSH。
4. WING GUI runtime handle 能统一推进 tick、timer、animation、input、event、layout、dirty、render。
5. object tree、visible/enabled/state、focus、capture、event bubbling、close request 可以稳定工作。
6. dirty rect list、dirty merge、chunk redraw、clip stack、present rect list 可以稳定工作。
7. WING GUI 不直接写 framebuffer，而是通过 render frontend / render node / FRender command path 绘制。
8. 默认 object space 原则进入核心：2D 是默认 object space 下的 identity transform 状态。
9. z-index + projected depth + sibling order 的排序原则可被 demo 和应用显式使用。
10. Phase 1 所需基础 widget 可以在 demo 中被输入、状态、绘制和脏区机制覆盖。
11. demo 中的设计值和 WING core 默认机制保持边界清晰。
12. 文档记录 Phase 1 当前状态、已验证项和剩余缺口。

## 3. 验收矩阵

| 项目 | 验收要求 | 当前状态 | 证据 | 剩余缺口 |
| --- | --- | --- | --- | --- |
| 独立运行 | WING GUI app 不依赖 WING Desktop，可从 NSH 直接启动 | 已通过 | `wing_gui_demo` 是 NSH builtin app，运行链路不经过 Desktop | 继续保持 Desktop 不反向污染 WING GUI |
| 显示启动 | 执行 demo 时才打开 X11 framebuffer 窗口 | 已通过 | sim 中从 NSH 执行 `wing_gui_demo` 后打开 framebuffer/X11 path | 需要后续保持 frender/nuttx graphics 改动不破坏该行为 |
| 关闭返回 | X11 close request 进入 WING input/event queue，demo 退出回到 NSH | 已通过 | 日志包含 `root received close request through WING input/event queue`、`app task exit`、`nsh>` | close 日志仍可进一步去重，但不阻塞 Phase 1 |
| runtime handle | app loop 通过 WING handler 推进 GUI | 已通过 | `wing_gui_handle()` 串起 tick/step/frame 诊断 | 后续可继续收敛 app loop 模板 |
| timer | one-shot / repeat timer 可运行并触发 UI 更新 | 已通过 | demo 中 progress timer、repeat timer 日志 | 无 Phase 1 阻塞项 |
| animation | animation runtime 可推进 bounds / space transform | 已通过 | line width animation、loop animation、space card animation 日志 | 无 Phase 1 阻塞项 |
| input | pointer/key/encoder/close request 可进入统一 input path | 已通过 | demo synthetic input + X11 provider 日志 | 后续真实 touch/key device 接入属于 NuttX/input 后续阶段 |
| event | object event、bubbling、stop propagation、value changed、state changed 可运行 | 已通过 | demo 多个 widget event 日志 | 无 Phase 1 阻塞项 |
| focus/capture | focus 切换、pointer capture/release/cancel 可运行 | 已通过 | slider/scrollbar capture lifecycle 日志 | 无 Phase 1 阻塞项 |
| object tree | parent/child、create/delete、visible/enabled 可运行 | 已通过 | toast create/delete、visible hide、enabled disable 日志 | 无 Phase 1 阻塞项 |
| dirty | dirty list、merge、redraw chunk、union fallback 可运行 | 已通过 | demo dirty before/after handler、redraw chunks、fallback 日志 | 后续可补更细的可视化/统计，但不阻塞 Phase 1 |
| clip | object clip-children + FRender clip stack 可运行 | 已通过 | scroll view / clip panel 日志 | 无 Phase 1 阻塞项 |
| present | frame present rect list 驱动 framebuffer present | 已通过 | demo present rect list 日志 | 后续 NuttX graphics update 能力还需整理 |
| render boundary | WING 输出 render frontend / FRender command，不直接写 framebuffer | 已通过 | WING render frontend 调用 FRender command list；demo 日志描述 path | 后续需要 FRender planner/capability 完整化 |
| default object space | 2D 是默认 object space identity 状态 | 已通过 | camera/project/space transform/projected quad/depth/hit-test 日志 | mesh/shader/GPU 3D 属后续阶段 |
| z-order | 应用显式 z-index，core 按 z-index + projected depth 排序 | 已通过 | demo 已为 progress/slider/scrollbar 使用显式 z-layer | 后续 Desktop layer manager 仍需系统化设计 |
| basic widgets | label/button/image/panel/progress/slider/scrollbar/switch/checkbox/text input/scroll view/card/custom geometry 可验证 | 已通过 | `wing_gui_demo` 覆盖这些 widget 和路径 | 复杂 widget 不属于 Phase 1 |
| text edit | text input 使用 core text edit helper，而非 widget 私有乱写 | 已通过 | `wing_text_edit_t` 与 demo text input 日志 | 多行/IME/clipboard 不属于 Phase 1 |
| style/state | state style、hover/focus/pressed/checked/selected/active 可运行 | 已通过 | demo state changed 与 style 日志 | 主题系统完整化属于后续阶段 |
| layout | fixed/stack/center/fill 可运行 | 已通过 | demo panel/card/button/fill layout 日志 | flex/grid 不属于 Phase 1 |
| module boundary | core/widget/render/input/event/timer/dirty 等已拆分 | 已通过 | `include/wing/core/*`、`src/core/*`、`src/widgets/*`；tick/focus/capture/timer/animation API 已由专属头承载 | 后续新增 API 不应重新堆回 `wing.h` 主体 |
| demo/core 边界 | demo 视觉值尽量放在 demo，不进入 core 默认行为 | 部分通过 | demo 已集中大量常量和显式 z-layer | 仍需继续清理残余硬编码和 demo-only 逻辑 |
| FRender 第一阶段 | software backend + command list + framebuffer present 可服务 WING GUI | 已通过 | `frender_demo` 和 `wing_gui_demo` 已验证 | planner/capability/fallback policy 还不是完整阶段 |
| NuttX graphics 第一阶段 | framebuffer/sim X11 可支撑显示闭环 | 已通过 | sim framebuffer path 可打开/present/close | 能力声明、accelerator、NX backend 属后续阶段 |

## 4. Phase 1 当前剩余缺口

Phase 1 还需要优先处理以下缺口：

1. 继续清理 demo/core 边界，避免 demo-only 视觉参数或临时逻辑进入 core。
2. 对图层排序、dirty redraw、present rect、close request 形成最终验证记录。
3. 明确 Phase 1 完成后冻结 WING GUI 功能扩张，转向 FRender planner/capability 和 NuttX graphics capability。

## 5. Phase 1 不包含内容

以下内容不作为 Phase 1 阻塞项：

1. WING Desktop。
2. 应用安装、打包、下载、启动器。
3. Pinion 游戏引擎。
4. 完整 mesh / shader / GPU 3D。
5. 完整 flex/grid 布局。
6. 复杂 widget 套件。
7. DMA2D / GPU2D / GPU3D 硬件后端。
8. NX backend。
9. 大规模主题系统和资源工具链。
10. IME、clipboard、多窗口桌面管理。

## 6. 固定验证流程

每次宣称 Phase 1 闭环可用前，应优先执行自动验收脚本：

```sh
./FeatherCore/tools/firmware/sim/validate-wing-phase1.sh
```

该脚本会执行以下流程：

1. 构建 `sim-wing`。
2. 启动 NuttX sim NSH。
3. 从 NSH 执行 `wing_gui_demo`。
4. 检查 runtime summary、framebuffer、X11 input provider、explicit control z-layers、object space、render node、dirty、redraw chunks 和 present rect 日志。
5. 通过 X11 工具发送窗口关闭请求。
6. 检查 close request 进入 WING input/event queue。
7. 确认 demo 退出并返回 NSH。

手工验证流程如下：

```sh
./FeatherCore/tools/firmware/sim/build-wing.sh --no-clean
./FeatherCore/build/sim-wing
```

在 NSH 中执行：

```text
wing_gui_demo
```

验证要求：

1. demo 启动后打开 X11 framebuffer 窗口。
2. 日志打印 runtime capability summary。
3. 日志包含 explicit control z-layers。
4. 日志包含 object space / camera / projected quad / depth / hit-test。
5. 日志包含 timer / repeat timer / animation。
6. 日志包含 input provider / pointer / key / encoder。
7. 日志包含 dirty before/after handler、redraw chunks、present rect list。
8. 日志包含 close request event。
9. 关闭窗口后返回 `nsh>`。

可选补充：

```text
frender_demo
```

用于验证 FRender command list、software backend 和 framebuffer present path。

## 7. Phase 1 完成后的行动

Phase 1 完成后：

1. WING GUI 暂停功能扩张，只保留 bugfix、边界清理和必要稳定性修正。
2. 优先推进 FRender planner / capability / fallback。
3. 优先推进 NuttX graphics public capability、framebuffer update、accelerator wrapper。
4. 再进入 WING Desktop 最小桌面模板。
5. 再进入 Pinion 最小 game loop / scene / render path。

## 8. 当前结论

WING GUI Phase 1 已经接近闭环完成。

当前尚不应标记为最终完成，因为还需要把验收流程固定下来，并继续收敛 demo/core 边界和 API 文件边界。

下一步应围绕本清单逐项关闭剩余缺口，而不是继续增加新 widget 或 Desktop 功能。

## 9. 模块边界验收补充记录

2026-06-12 已完成一轮 public header boundary 收口：

- `wing.h` 保持为聚合入口。
- tick、focus、capture、timer、animation 的公开函数声明由对应 `include/wing/core/wing_*.h` 承载。
- `wing.h` 通过 include 聚合这些专属头，应用仍然可以只包含 `wing/wing.h`。

该项推进了验收矩阵中的“module boundary”要求，但后续仍需继续观察：新增 API 不应重新堆回 `wing.h` 主体。

## 10. 固定验收脚本执行记录

2026-06-12 已执行：

```sh
./FeatherCore/tools/firmware/sim/validate-wing-phase1.sh
```

验证结果：通过。

脚本已确认：

1. `sim-wing` 构建通过。
2. NuttX sim NSH 启动。
3. `wing_gui_demo` 从 NSH 执行。
4. framebuffer / X11 input provider 进入运行路径。
5. runtime summary、explicit control z-layers、default object space、render node/material seed、dirty、redraw chunks、present rect 日志均出现。
6. X11 close request 进入 WING input/event queue。
7. demo 输出 `app task exit`。
8. 脚本输出 `WING GUI Phase 1 runtime closure validation passed`。

该记录关闭了“建立最终验收脚本或固定手工验证流程”的 Phase 1 缺口。

## 11. Phase 1 最终收口记录

收口日期：2026-06-12

Phase 1 最终结论：

```text
WING GUI Phase 1: Runtime Closure 已完成。
```

完成依据：

1. `wing_gui_demo` 是独立 NSH builtin app，不依赖 WING Desktop。
2. 固定验收脚本 `FeatherCore/tools/firmware/sim/validate-wing-phase1.sh` 已通过。
3. 验收脚本覆盖构建、NSH 启动、`wing_gui_demo` 执行、X11 framebuffer、input provider、default object space、render node/material seed、dirty/redraw/present 和 close request。
4. close request 已确认进入 WING input/event queue，并使 demo 正常退出。
5. `wing.h` 已收敛为聚合入口，tick/focus/capture/timer/animation 等公开 API 由专属 core 头文件承载。
6. demo/core 边界已满足 Phase 1 要求：demo 视觉几何、z-layer、动画、timer 和输入脚本常量保留在 `wing_gui_demo`，WING core 不再承载旧 demo scene 或 demo-only 绘制入口。

Phase 1 关闭后的约束：

1. WING GUI 暂停功能扩张。
2. 后续 WING GUI 只接受 bugfix、边界清理、验证脚本维护和必要稳定性修正。
3. 新 widget、Desktop、Pinion、完整 mesh/shader/GPU 3D、复杂布局、资源工具链不再进入 Phase 1。
4. 后续主线切换到 FRender planner / capability / fallback，以及 NuttX graphics capability / present / accelerator 接入点。
5. 若后续修改 WING GUI runtime，必须重新执行 `validate-wing-phase1.sh`。

最终验收矩阵状态：

| 项目 | 最终状态 |
| --- | --- |
| 独立运行 | 已通过 |
| 显示启动 | 已通过 |
| 关闭返回 | 已通过 |
| runtime handle | 已通过 |
| timer | 已通过 |
| animation | 已通过 |
| input | 已通过 |
| event | 已通过 |
| focus/capture | 已通过 |
| object tree | 已通过 |
| dirty | 已通过 |
| clip | 已通过 |
| present | 已通过 |
| render boundary | 已通过 |
| default object space | 已通过 |
| z-order | 已通过 |
| basic widgets | 已通过 |
| text edit | 已通过 |
| style/state | 已通过 |
| layout | 已通过 |
| module boundary | 已通过 |
| demo/core 边界 | 已通过 |
| FRender 第一阶段 | 已通过 |
| NuttX graphics 第一阶段 | 已通过 |
