# NuttX Git Workflow for FeatherCore

本文档记录当前 WSL 环境中 Apache NuttX / NuttX Apps 的本地工作区状态，以及 FeatherCore 后续维护私有开发分支并同步 Apache 主线的推荐流程。

## 当前环境

当前环境是 WSL2：

```text
Linux uan-win 6.6.87.2-microsoft-standard-WSL2
```

当前工作区路径：

```text
/home/uan-wsl2/nuttx-work/
├── nuttx/
└── apps/
```

其中：

```text
nuttx/ -> Apache NuttX 核心仓库
apps/  -> Apache NuttX Apps 应用仓库
```

NuttX 构建系统通常期望 `nuttx/` 和 `apps/` 并列放置，因此后续构建、配置和 BSP 开发都建议在这个目录结构下进行。

## 当前远程仓库配置

### nuttx

路径：

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
```

远程：

```text
origin   ssh://git@ssh.github.com:443/FeatherCore/nuttx.git
upstream https://github.com/apache/nuttx.git
```

其中：

```text
origin   -> FeatherCore 自己的远程仓库，用于 push 私有分支和产品分支
upstream -> Apache NuttX 主线仓库，只用于 fetch / merge 主线更新
```

`upstream` 的 push URL 已设置为 `no_push`，用于避免误推 Apache 主线：

```text
upstream no_push (push)
```

### apps

路径：

```bash
cd /home/uan-wsl2/nuttx-work/apps
```

远程：

```text
origin   ssh://git@ssh.github.com:443/FeatherCore/nuttx-apps.git
upstream https://github.com/apache/nuttx-apps.git
```

同样：

```text
origin   -> FeatherCore 自己的 nuttx-apps 仓库
upstream -> Apache nuttx-apps 主线仓库，只用于 fetch / merge
```

`upstream` 的 push URL 也已设置为 `no_push`。

## 当前分支状态

两个仓库当前都位于开发分支：

```text
vendor/stm32n6-bringup
```

并且已经推送到 FeatherCore 远程仓库：

```text
FeatherCore/nuttx      -> vendor/stm32n6-bringup
FeatherCore/nuttx-apps -> vendor/stm32n6-bringup
```

这个分支用于 STM32N6 BSP / board bring-up 开发。

另外，两个仓库中也已经创建并推送了以下开发分支：

```text
vendor/stm32h7rs-bringup
vendor/stm32u5x9-bringup
vendor/ra8p-bringup
```

这些分支分别用于 STM32H7RS、STM32U5x9 和 RA8P bring-up 开发，当前都以 `master` 为基线创建。

## 从零搭建时的完整流程

如果以后需要在另一台机器或一个干净目录重新搭建，可以按以下流程操作。

### 1. 创建工作区

```bash
mkdir -p /home/uan-wsl2/nuttx-work
cd /home/uan-wsl2/nuttx-work
```

### 2. 拉取 Apache 主线

```bash
git clone https://github.com/apache/nuttx.git nuttx
git clone https://github.com/apache/nuttx-apps.git apps
```

### 3. 将 Apache 远程改名为 upstream

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
git remote rename origin upstream
git remote set-url --push upstream no_push

cd /home/uan-wsl2/nuttx-work/apps
git remote rename origin upstream
git remote set-url --push upstream no_push
```

### 4. 添加 FeatherCore 远程为 origin

当前 WSL 环境中 GitHub SSH 22 端口偶尔不可达，因此推荐使用 GitHub SSH over 443：

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
git remote add origin ssh://git@ssh.github.com:443/FeatherCore/nuttx.git

cd /home/uan-wsl2/nuttx-work/apps
git remote add origin ssh://git@ssh.github.com:443/FeatherCore/nuttx-apps.git
```

如果普通 SSH 22 端口稳定，也可以使用：

```text
git@github.com:FeatherCore/nuttx.git
git@github.com:FeatherCore/nuttx-apps.git
```

### 5. 推送 master 到 FeatherCore

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
git checkout master
git push -u origin master

cd /home/uan-wsl2/nuttx-work/apps
git checkout master
git push -u origin master
```

### 6. 创建并推送开发分支

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
git checkout -b vendor/stm32n6-bringup
git push -u origin vendor/stm32n6-bringup

cd /home/uan-wsl2/nuttx-work/apps
git checkout -b vendor/stm32n6-bringup
git push -u origin vendor/stm32n6-bringup
```

## 日常开发流程

平时不要直接在 `master` 上开发。`master` 只用于跟踪 Apache 主线；自己的 BSP、板级移植、产品配置应放在独立分支上。

推荐当前开发分支：

```text
vendor/stm32n6-bringup
```

日常开发：

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
git checkout vendor/stm32n6-bringup

# 修改代码
git status
git add <files>
git commit -s -m "arm: stm32n6: add initial board support"
git push
```

如果 `apps` 有配套修改：

```bash
cd /home/uan-wsl2/nuttx-work/apps
git checkout vendor/stm32n6-bringup

# 修改代码
git status
git add <files>
git commit -s -m "examples: add stm32n6 bringup support"
git push
```

## 同步 Apache 主线

当需要合入 Apache NuttX 最新主线改动时，先更新本地 `master`，再把 `master` 合入自己的开发分支。

### nuttx

```bash
cd /home/uan-wsl2/nuttx-work/nuttx

git checkout master
git fetch upstream
git merge --ff-only upstream/master
git push origin master

git checkout vendor/stm32n6-bringup
git merge master

# 如果有冲突，解决冲突后执行：
# git status
# git add <resolved-files>
# git commit

git push
```

### apps

```bash
cd /home/uan-wsl2/nuttx-work/apps

git checkout master
git fetch upstream
git merge --ff-only upstream/master
git push origin master

git checkout vendor/stm32n6-bringup
git merge master

# 如果有冲突，解决冲突后执行：
# git status
# git add <resolved-files>
# git commit

git push
```

## merge 还是 rebase

建议：

```text
多人共享、长期维护的 BSP 分支：使用 merge
个人短期实验分支：可以使用 rebase
准备向 Apache NuttX 提交 PR：可以在提交前整理 commits / rebase
公司或组织内部产品分支：使用 merge upstream/master 更稳
```

当前 `vendor/stm32n6-bringup` 属于长期 BSP bring-up 分支，推荐使用：

```bash
git merge master
```

不要频繁 rebase 已经推送给别人使用的分支。

## 推荐长期分支结构

可以按以下方式维护：

```text
master
  跟踪 Apache NuttX 主线，不直接开发

vendor/base
  FeatherCore 自己的干净基线，可放少量通用补丁

vendor/stm32n6-bringup
  STM32N6 SoC / board / memory / OSPI / SDRAM bring-up

vendor/ra8p1-bringup
  RA8P1 SoC / board bring-up

product/<board-or-product>
  具体产品配置、默认 defconfig、应用集成
```

同步路径建议保持清晰：

```text
upstream/master
      ↓
origin/master
      ↓
vendor/base
      ↓
vendor/stm32n6-bringup
      ↓
product/<board-or-product>
```

如果当前只做 STM32N6 bring-up，可以先只维护：

```text
master
vendor/stm32n6-bringup
```

后续产品化或多芯片并行时，再拆出 `vendor/base` 和 `product/*`。

## 常用检查命令

检查当前分支：

```bash
git branch --show-current
```

检查远程：

```bash
git remote -v
```

检查工作区状态：

```bash
git status --short --branch
```

检查当前分支跟踪关系：

```bash
git branch -vv
```

检查最近提交：

```bash
git log -1 --oneline
```

## 当前已完成事项

本机已经完成：

```text
1. 确认当前环境是 WSL2
2. 创建 /home/uan-wsl2/nuttx-work
3. 从 Apache 主线拉取 nuttx
4. 从 Apache 主线拉取 nuttx-apps
5. 将 Apache 远程从 origin 改为 upstream
6. 将 upstream push URL 设置为 no_push
7. 添加 FeatherCore 远程为 origin
8. 推送 master 到 FeatherCore/nuttx
9. 推送 master 到 FeatherCore/nuttx-apps
10. 创建 vendor/stm32n6-bringup 分支
11. 推送 vendor/stm32n6-bringup 到两个 FeatherCore 仓库
12. 将本地两个仓库切换到 vendor/stm32n6-bringup
13. 创建并推送 vendor/stm32h7rs-bringup 到两个 FeatherCore 仓库
14. 创建并推送 vendor/stm32u5x9-bringup 到两个 FeatherCore 仓库
15. 创建并推送 vendor/ra8p-bringup 到两个 FeatherCore 仓库
```

当前可以直接开始在以下路径进行开发：

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
git checkout vendor/stm32n6-bringup
```

如果涉及 NuttX Apps：

```bash
cd /home/uan-wsl2/nuttx-work/apps
git checkout vendor/stm32n6-bringup
```
