# Pinion 实现计划

本文记录 Pinion 的定位和实现路线。

## 定位

Pinion 是 FeatherCore 的原生轻量级游戏引擎。

它位于：

```text
apps/graphics/pinion
```

Pinion 不替代 WING，也不承担桌面 UI。它专注于：

```text
game loop
scene
sprite
tilemap
camera
animation
input mapping
resource lifecycle
game render phase construction
```

## 与 FRender 的关系

Pinion 应该基于 FRender，而不是自己直接面对 framebuffer / NX / DMA2D / GPU2D。

目标分层：

```text
Game App
        |
        v
Pinion
  game loop / scene / sprite / tilemap / camera
        |
        v
FRender
  command list / planner / software fallback / hardware adapter
        |
        v
NuttX graphics
  framebuffer / LCD / NX / DMA2D / GPU2D
```

Pinion 决定“游戏世界里应该画什么”。

FRender 决定“这些绘制命令怎么在当前硬件上执行”。

## 当前代码状态

当前相关目录：

```text
apps/graphics/pinion
apps/examples/pinion_demo
```

Pinion 当前应保持轻量 seed，不要在自身内部继续扩大一套独立 renderer。后续渲染能力应逐步迁移到 FRender。

## Pinion 应负责的内容

```text
engine frame counter
target FPS / frame time
fixed timestep / variable timestep
scene/layer/camera
sprite/tilemap
animation timeline
input action mapping
resource handle
render phase construction
```

## Pinion 不应负责的内容

```text
framebuffer format conversion
DMA2D/GPU2D capability 判断
software fallback island
present/update ioctl 细节
render planner/cost policy
通用 blit/fill/clip backend
```

这些应属于 FRender。

## 第一阶段目标

在 FRender 落地前，Pinion 可以保留最小 demo 作为概念验证。

FRender 落地后，Pinion 的第一阶段目标应改为：

```text
1. Pinion engine frame loop
2. Pinion scene/sprite seed
3. Pinion render builder
4. 输出 frender command list
5. 通过 frender software backend 渲染
6. pinion_demo 从直接写 surface 改为走 frender
```

## 建议 API 方向

```c
typedef struct pinion_engine_s pinion_engine_t;
typedef struct pinion_scene_s pinion_scene_t;
typedef struct pinion_sprite_s pinion_sprite_t;
typedef struct pinion_camera_s pinion_camera_t;

typedef void (*pinion_update_cb)(pinion_engine_t *engine,
                                 void *userdata);

typedef void (*pinion_render_cb)(pinion_engine_t *engine,
                                 pinion_scene_t *scene,
                                 void *userdata);
```

渲染阶段不直接写 framebuffer，而是：

```text
pinion_scene
  -> pinion_render_builder
  -> frender_command_list
```

## 实施路线

```text
Milestone 1: 保持 pinion_demo 可用于验证基本游戏循环
Milestone 2: 等 frender 最小实现落地
Milestone 3: Pinion 输出 frender command list
Milestone 4: 增加 sprite / tilemap / camera
Milestone 5: 增加 input action system
Milestone 6: 增加 resource/asset 管理
Milestone 7: 接入可选 2D/3D profile
```

## 和 WING 的关系

WING 和 Pinion 并列：

```text
WING GUI
  GUI object tree / layout / style / event / animation

Pinion
  game scene / sprite / tilemap / camera / frame loop
```

两者共享 FRender，不共享高层模型。

```text
WING 不承担 game loop
Pinion 不承担 desktop/window manager
```

## 当前推荐下一步

```text
1. 先实现 FRender 最小 backend
2. 再让 Pinion demo 改为输出 FRender command list
3. 然后补 sprite/tilemap/camera/game loop
```
