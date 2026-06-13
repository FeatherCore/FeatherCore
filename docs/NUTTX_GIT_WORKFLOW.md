# Feather NuttX Git Workflow

本文档记录 FeatherCore 当前的 NuttX 开发工作流。现在以
`/home/uan-wsl2/Feather` 作为主开发入口；`nuttx` 和 `apps` 作为
Feather super-project 下的 Git submodule 维护。

## 当前工作区

主工作区：

```text
/home/uan-wsl2/Feather/
├── nuttx/      submodule -> FeatherCore/nuttx
├── apps/       submodule -> FeatherCore/nuttx-apps
├── tools/
├── docs/
└── build/
```

当前子模块锁定状态：

```text
nuttx: e2aa04fb4fc11d279c2648d83c1829ae933454a0
apps:  7bae6e591d557035c7d44be84f49163bbd883f84
```

`nuttx` 和 `apps` 来自各自仓库的 `develop` 分支。

旧目录 `/home/uan-wsl2/nuttx-work/nuttx` 和
`/home/uan-wsl2/nuttx-work/apps` 仍可作为历史/临时工作区参考，但后续主线开发
应以 `/home/uan-wsl2/Feather` 为准。

## 仓库关系

Feather super-project：

```text
origin -> ssh://git@ssh.github.com:443/FeatherCore/Feather.git
branch -> main
```

NuttX core submodule：

```text
path   -> /home/uan-wsl2/Feather/nuttx
origin -> ssh://git@ssh.github.com:443/FeatherCore/nuttx.git
branch -> develop
```

NuttX apps submodule：

```text
path   -> /home/uan-wsl2/Feather/apps
origin -> ssh://git@ssh.github.com:443/FeatherCore/nuttx-apps.git
branch -> develop
```

`nuttx` 和 `apps` 仓库仍建议保留 Apache 主线远程：

```text
upstream -> https://github.com/apache/nuttx.git
upstream -> https://github.com/apache/nuttx-apps.git
```

`upstream` 只用于 fetch/merge Apache 主线，push URL 应保持 `no_push`。

## 当前分支策略

现在远程私有分支已收敛为：

```text
master
develop
```

以前的分支已经删除：

```text
vendor/stm32h7rs-bringup
vendor/stm32u5x9-bringup
vendor/stm32n6-bringup
vendor/ra8p-bringup
```

后续板级开发都在 `develop` 上继续：

```text
STM32H7S78-DK
STM32U5x9J-DK
STM32N6570-DK
RA8P
```

`master` 只用于跟踪 Apache 主线，不直接开发。

## 从零克隆

推荐直接克隆 Feather super-project：

```bash
cd /home/uan-wsl2
git clone --recurse-submodules ssh://git@ssh.github.com:443/FeatherCore/Feather.git
cd Feather
```

如果已经克隆但没有初始化 submodule：

```bash
cd /home/uan-wsl2/Feather
git submodule update --init --recursive
```

确认状态：

```bash
git status --short --branch
git submodule status
```

## 日常开发流程

进入 super-project：

```bash
cd /home/uan-wsl2/Feather
```

更新 super-project 和 submodule：

```bash
git pull --ff-only
git submodule update --init --recursive
```

进入 `nuttx` 子模块开发：

```bash
cd /home/uan-wsl2/Feather/nuttx
git checkout develop
git pull --ff-only
```

如果涉及 apps：

```bash
cd /home/uan-wsl2/Feather/apps
git checkout develop
git pull --ff-only
```

提交 `nuttx` 修改：

```bash
cd /home/uan-wsl2/Feather/nuttx
git status --short
git add <files>
git commit -s -m "arm: stm32h7rs: describe the change"
git push
```

提交 `apps` 修改：

```bash
cd /home/uan-wsl2/Feather/apps
git status --short
git add <files>
git commit -s -m "apps: describe the change"
git push
```

更新 Feather super-project 记录的 submodule commit：

```bash
cd /home/uan-wsl2/Feather
git status --short
git add nuttx apps
git commit -s -m "Update NuttX submodules"
git push
```

注意：对子模块提交并 push 以后，还需要在 Feather 根目录提交一次
submodule 指针更新，否则 Feather 远程不会记录新的组合版本。

## 构建方式

NuttX 构建仍在 `nuttx/` 子模块中执行。`apps/` 与 `nuttx/` 并列，符合
NuttX 默认目录布局。

H7RS NXboot 与 NSH app 可以用 Feather 工程级脚本一次完成构建、打包和复制：

```bash
cd /home/uan-wsl2/Feather
./tools/firmware/stm32h7s78-dk/build-nsh.sh -j 8
```

输出统一写入 Feather 根目录的 `build/`：

```text
build/stm32h7s78-dk-nxboot.bin
build/stm32h7s78-dk-nsh.bin
```

烧录关系：

```text
0x08000000 -> stm32h7s78-dk-nxboot.bin
0x70000000 -> stm32h7s78-dk-nsh.bin
```

`stm32h7s78-dk-nsh.bin` 的结构是 `[NXboot header][NuttX app raw binary]`；
raw app 只作为构建过程中的 `nuttx.bin` 中间产物存在，不复制到 `build/`。

## 同步 Apache 主线

同步 `nuttx`：

```bash
cd /home/uan-wsl2/Feather/nuttx

git checkout master
git fetch upstream
git merge --ff-only upstream/master
git push origin master

git checkout develop
git merge master
git push origin develop
```

同步 `apps`：

```bash
cd /home/uan-wsl2/Feather/apps

git checkout master
git fetch upstream
git merge --ff-only upstream/master
git push origin master

git checkout develop
git merge master
git push origin develop
```

更新 Feather submodule 指针：

```bash
cd /home/uan-wsl2/Feather
git add nuttx apps
git commit -s -m "Update NuttX upstream baseline"
git push
```

如果 `git merge master` 产生冲突，解决后：

```bash
git status
git add <resolved-files>
git commit
git push
```

## Super-project 常用命令

查看 Feather 状态：

```bash
cd /home/uan-wsl2/Feather
git status --short --branch
git submodule status
```

查看子模块是否有未提交修改：

```bash
git submodule foreach 'git status --short --branch'
```

把子模块更新到远程 `develop` 最新 commit：

```bash
git submodule update --remote --merge
```

查看 Feather 当前记录的子模块 commit：

```bash
git ls-tree HEAD nuttx apps
```

## 分支使用建议

当前阶段统一在 `develop` 上开发，减少多个 vendor 分支之间反复同步的成本。

如果后续需要长期并行维护多个产品或芯片线，可以再从 `develop` 拆出：

```text
product/<product-name>
feature/<short-topic>
hotfix/<short-topic>
```

但除非确实需要，不再恢复 `vendor/*-bringup` 多分支模型。

## 提交前检查

在 `nuttx` 或 `apps` 提交前：

```bash
git status --short --branch
git diff --check
```

确认没有加入构建产物：

```text
*.o
*.a
.depend
Make.dep
nuttx
nuttx.bin
nuttx.hex
nuttx.map
```

在 Feather 根目录提交 submodule 指针前：

```bash
cd /home/uan-wsl2/Feather
git status --short --branch
git submodule status
```

## 当前已完成事项

```text
1. 创建 FeatherCore/nuttx 和 FeatherCore/nuttx-apps。
2. 创建 FeatherCore/Feather super-project。
3. 将 nuttx 和 apps 作为 Feather 的 submodule。
4. 删除远程和本地 vendor/* bring-up 分支。
5. 将 H7RS、U5x9、N6、RA8P 开发内容合并到 develop。
6. Feather main 记录 nuttx/apps develop 的当前 commit。
7. 将移植文档移动到 /home/uan-wsl2/Feather/docs。
```
