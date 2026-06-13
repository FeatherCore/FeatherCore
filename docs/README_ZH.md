# FeatherCore 文档索引

本文是 `FeatherCore/docs` 当前有效文档的索引。

## 当前图形架构核心文档

```text
progress/README_ZH.md
  计划与进度文档目录。

progress/GRAPHICS_ROADMAP_AND_PROGRESS_ZH.md
  当前图形路线总计划、真实进度、下一步任务和 milestone 状态。

FEATHER_RENDER_CORE_DECISION_ZH.md
  Feather 图形渲染核心的架构决策：frender 放在 apps/graphics，不放进 nuttx/graphics。

FRENDER_IMPLEMENTATION_ZH.md
  FRender 的实现计划：command list、planner、software backend、硬件 adapter 路线。

WING_GUI_AND_DESKTOP_ARCHITECTURE_ZH.md
  WING GUI 与 WING Desktop 的分层关系。

WING_IMPLEMENTATION_ZH.md
  WING GUI 的实现路线，包含从 HoneyGUI/LVGL/TouchGFX 吸收的设计思想。

PINION_IMPLEMENTATION_ZH.md
  Pinion 游戏引擎的实现路线，以及它如何基于 FRender。

NUTTX_GRAPHICS_OVERVIEW_ZH.md
  NuttX graphics 当前能力，以及它和 FRender/WING/Pinion 的边界。

APPS_GRAPHICS_OVERVIEW_ZH.md
  apps/graphics 下现有图形模块概览。
```

## 板级和硬件移植记录

```text
STM32_GPU2D_NEMAP_PORTING_RESULT.md
STM32H7S78_DK_PORTING.md
STM32N6570_DK_PORTING.md
STM32U5X9J_DK_PORTING.md
EK_RA8P1_PORTING.md
NUTTX_GIT_WORKFLOW.md
```

这些文档不属于 WING/FRender/Pinion 核心路线，但仍是有效的硬件和工作流记录。

## 已移出的旧过程文档

旧 Wing 原型重构、旧 wingdemo、重复英文概览和重复渲染策略文档已移出当前 docs 目录，位置：

```text
FeatherCore/tmp/docs_removed
```

这些文档仅作为历史参考，不再作为当前实现依据。
