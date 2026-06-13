# Feather Render Core 架构决策

本文记录 FeatherCore 当前图形路线的核心决策：将 FGFX 的核心思想吸收到 Feather 应用侧公共渲染核心中，而不是直接塞进 `nuttx/graphics`。

公共渲染核心暂定名称：

```text
frender
```

目标位置：

```text
apps/graphics/frender
```

## 当前代码状态

截至本文档整理时，`apps/graphics/frender` 尚未作为目录落地。

当前已经存在：

```text
apps/graphics/wing
apps/graphics/pinion
apps/examples/wing_gui_demo
apps/examples/pinion_demo
```

因此本文是架构决策和后续实现依据，不是已完成实现清单。

## 决策结论

推荐路线：

```text
Independent UI App / WING Desktop / Game App
        |
        v
WING GUI / Pinion
  生成绘制意图和渲染命令
        |
        v
frender
  command list / planner / fallback / software backend / hardware adapter
        |
        v
NuttX public graphics APIs
  framebuffer / LCD / NX / optional accelerator APIs
        |
        v
nuttx/graphics + drivers
```

不推荐路线：

```text
直接把 command list / planner / fallback / render graph 塞进 nuttx/graphics
```

原因：

```text
NuttX graphics 负责设备和显示基础设施
frender 负责绘制意图、执行规划和 fallback
WING GUI 负责 GUI 语义
Pinion 负责游戏语义
```

## 为什么需要独立 frender

WING GUI 和 Pinion 都需要绘制能力。如果它们各自直接写 pixels 或各自对接 framebuffer/DMA2D/GPU2D，会很快出现重复和混乱：

```text
两套 surface
两套 blit
两套 dirty region
两套 framebuffer present
两套 hardware capability 判断
两套 software fallback
```

因此应该沉淀一个共同的 Render backend：

```text
WING GUI
  object tree / style / layout / event / animation
        |
        v
      frender
        ^
        |
Pinion
  scene / sprite / tilemap / camera / game loop
```

WING GUI 和 Pinion 输出不同的上层语义，但最终都应该交给同一个或兼容的 command list / planner / backend 执行。

## 从 FGFX 吸收什么

这里吸收的是思想，不是恢复旧目录。

应该吸收：

```text
display list / command list
planner-first
software-defined correctness
hardware as optional fast path
explicit backend capability
local fallback before whole-frame fallback
command island
resource contract
cost policy
2D/3D profile
诊断友好的计划输出
```

最重要原则：

```text
软件后端定义正确性，硬件后端只是更快路径。
```

## 为什么不是传统 linked list

“绘制串成链表”这个方向是对的，但更准确的目标应该是 command stream。

不建议第一版使用频繁 malloc/free 的链表：

```c
struct cmd
{
  struct cmd *next;
};
```

更推荐：

```text
固定容量 command buffer
append-only command list
arena-style resource table
dry-run capacity check
```

原因：

```text
嵌入式内存更可控
cache locality 更好
更容易统计容量和 overflow
更适合后续 hardware encoder 顺序遍历
```

## 和 nuttx/graphics 的边界

`nuttx/graphics` 当前主要提供：

```text
NX / NXBE / NXGLIB / NxTerm
framebuffer / LCD 相关基础设施
window redraw / input routing / update hook
```

它可以支撑 frender 第一阶段：

```text
software backend
memory surface
framebuffer present
basic update
optional NX present
```

但它目前不直接提供：

```text
Feather command list
render planner
backend capability contract
software fallback island
resource ownership/cache/barrier contract
2D/3D profile planner
```

这些应先在 `apps/graphics/frender` 中实现。等多个上层和真实硬件反复需要同一类底层机制时，再把稳定、通用、硬件相关的接口下沉到 NuttX graphics 或 driver 层。

## 何时修改 nuttx/graphics

适合补进 `nuttx/graphics` 或 driver 层的能力：

```text
公开的 framebuffer/LCD capability query
公开的 DMA2D/GPU2D ioctl 或 callback contract
cache clean/invalidate helper
buffer ownership / sync primitive
FBIO_UPDATE / dirty update 能力完善
通用 accelerator wrapper
```

不适合放进 `nuttx/graphics` 的能力：

```text
WING object tree
Pinion scene/ECS
UI style/layout/animation
游戏 sprite/tile/camera
Feather render planner policy
GUI/Desktop package lifecycle
```

## 当前推荐下一步

```text
1. 创建 apps/graphics/frender
2. 创建 apps/examples/frender_demo
3. 实现最小 surface + command list + software backend
4. 让 wing_gui_demo 改为通过 frender 渲染
5. 让 Pinion 后续也输出 frender command list
6. 再接 framebuffer present / NX present / DMA2D / GPU2D
```
