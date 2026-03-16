# FeatherCore 文档索引

这是 FeatherCore 项目的文档索引，按类别组织。

## 核心文档 / Core Documentation

### 1. 主 README
- **文件**: [`README.md`](../README.md)
- **内容**: 项目概述、架构说明、构建流程、设备树说明
- **适用**: 所有用户和开发者
- **状态**: ✅ 完整

### 2. 快速开始
- **文件**: [`docs/README.md`](README.md)
- **内容**: 快速开始指南、常用命令参考
- **适用**: 新用户
- **状态**: ✅ 完整

### 3. 架构整理报告
- **文件**: [`docs/架构整理完成报告.md`](架构整理完成报告.md)
- **内容**: 详细的架构设计决策、链接脚本模板说明、设备树模块设计
- **适用**: 架构开发者
- **状态**: ✅ 完整

## 子项目文档 / Subproject Documentation

### 4. Boot 文档
- **文件**: [`boot/README.md`](../boot/README.md)
- **内容**: Bootloader 说明
- **状态**: ✅ 保留

- **文件**: [`boot/docs/BOOT.md`](../boot/docs/BOOT.md)
- **内容**: 详细的 Boot 流程说明
- **状态**: ✅ 保留

- **文件**: [`boot/BUILD.md`](../boot/BUILD.md)
- **内容**: Boot 构建指南
- **状态**: ✅ 保留

### 5. Build Tool 文档
- **文件**: [`build_tool/docs/DEVICE_TREE_PARSER.md`](../build_tool/docs/DEVICE_TREE_PARSER.md)
- **内容**: 设备树解析器使用说明
- **适用**: build_tool 开发者
- **状态**: ✅ 保留

### 6. Kernel 文档
- **文件**: [`kernel/docs/RUST_ASYNC_PROCESS_THREAD.md`](../kernel/docs/RUST_ASYNC_PROCESS_THREAD.md)
- **内容**: Rust 异步进程线程实现
- **状态**: ✅ 保留

## 技术设计文档 / Technical Design Documents

### 7. MMU 设计
- **文件**: [`common/docs/MMU_DESIGN.md`](../common/docs/MMU_DESIGN.md)
- **内容**: MMU 架构设计
- **状态**: ✅ 保留

- **文件**: [`common/docs/MMU_IMPLEMENTATION.md`](../common/docs/MMU_IMPLEMENTATION.md)
- **内容**: MMU 实现细节
- **状态**: ✅ 保留

## 已删除的文档 / Deleted Documents

以下文档已删除，因为它们包含过时或重复的内容：

- `docs/ARCHITECTURE.md` - 已过时，内容已整合到主 README
- `docs/ARCHITECTURE_SUMMARY.md` - 重复
- `docs/BUILD_GUIDE.md` - 已过时，构建流程在主 README 中
- `docs/COMMON_STRUCTURE.md` - 已过时
- `docs/COMMON_STRUCTURE_CORRECTED.md` - 临时文档
- `docs/FINAL_ARCHITECTURE.md` - 临时文档
- `docs/IMPLEMENTATION_SUMMARY.md` - 已过时
- `docs/PROJECT_STRUCTURE.md` - 已过时
- `docs/REFACTORING_PLAN.md` - 计划文档，已完成
- `docs/CARGO_STRUCTURE.md` - 已过时
- `docs/ARCHITECTURE_FIX.md` - 临时文档
- `docs/COMMON_REFACTORING_COMPLETE.md` - 临时文档
- `common/arch/ARCH_COMPLETION_REPORT.md` - 临时文档
- `common/COMMON_LIB_COMPLETION_REPORT.md` - 临时文档
- `common/arch/docs/架构说明.md` - 重复
- `common/arch/docs/ARCHITECTURE.md` - 重复
- `docs/通用 BOOT 架构设计.md` - 临时文档
- `docs/ROOTFS_EXPLANATION.md` - 已过时
- `docs/DESIGN.md` - 内容已整合

## 文档维护指南

### 文档分类
1. **核心文档** - 所有用户都需要阅读
2. **子项目文档** - 特定子项目的开发者需要
3. **技术设计文档** - 架构和技术决策参考

### 文档更新原则
1. 避免创建临时性文档（如"完成报告"、"整理报告"）
2. 架构变更时，直接更新主 README 和相关技术文档
3. 删除过时的文档，避免混淆
4. 保持文档索引的实时更新

### 推荐阅读顺序
1. 新用户：`README.md` → `docs/README.md`
2. 开发者：`README.md` → `docs/架构整理完成报告.md` → 子项目文档
3. 架构师：`docs/架构整理完成报告.md` → 技术设计文档

---

**最后更新**: 2024-01-XX  
**维护者**: FeatherCore Contributors
