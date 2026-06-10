# Linux Wi-Fi Stack Porting Details

本文集中记录当前 FeatherCore/NuttX 对 Linux 无线协议栈的移植细节。
`WIFI_HWSIM_ARCHITECTURE.md` 解释运行链路和 hostfs 空气介质；本文更关注“移植了哪些代码、
接入了哪些 NuttX 机制、做了哪些兼容层、还有哪些边界和清理项”。

## 移植目标

当前目标不是先写一套 NuttX 原生轻量 Wi-Fi 栈，而是先把 Linux 这套成熟路径整体搬进来，
让完整用户态和内核态无线链路在 NuttX sim 下跑通：

```text
wpa_supplicant-2.11 / hostapd-2.11
  -> libnl-3.2.25
  -> nl80211
  -> cfg80211
  -> mac80211
  -> mac80211_hwsim
  -> NuttX sim virtual netdev
```

这样做的意义是先获得一个能验证协议行为的基线，包括 open/WPA2/WPA3/WPS/WNM/P2P/Enterprise/
HT/VHT/HE/S1G/PS/TWT 等，然后再决定哪些部分需要 NuttX 原生重构。

## 上游来源

当前导入来源：

- Linux wireless stack:
  - `third/linux-7.0.10/net/wireless`
  - `third/linux-7.0.10/net/mac80211`
  - `third/linux-7.0.10/drivers/net/wireless/virtual`
- Wi-Fi userspace:
  - `third/wpa_supplicant-2.11`
  - `third/hostapd-2.11`
  - `third/libnl-3.2.25`

当前落地目录：

```text
FeatherCore/nuttx/wireless/ieee80211/cfg80211/
FeatherCore/nuttx/wireless/ieee80211/mac80211/
FeatherCore/nuttx/wireless/ieee80211/include/
FeatherCore/nuttx/drivers/wireless/virtual/
FeatherCore/apps/wireless/wifi/
```

移植策略是尽量保留 Linux vendor drop 的原始形状，把 NuttX 适配集中放在 compat 头、
桥接文件、Kconfig/Make.defs 和少量明确标注的 sim/hwsim 修改里。

## 顶层目录分工

### `nuttx/wireless/ieee80211`

这是 NuttX 内核侧 Linux IEEE 802.11 栈的主目录：

```text
nuttx/wireless/ieee80211/
  Kconfig
  Make.defs
  CMakeLists.txt
  genetlink_bridge.c
  ieee80211_linux_init.c
  netdevice_compat.c
  cfg80211/
  mac80211/
  include/
```

关键职责：

- 收纳 Linux cfg80211/nl80211 和 mac80211 vendor drop。
- 提供 Linux-style headers 和 NuttX compat shim。
- 统一初始化 cfg80211、nl80211、mac80211、mac80211_hwsim。
- 把 Linux `struct net_device` / `sk_buff` 和 NuttX `netdev_lowerhalf_s` / `netpkt_t`
  接起来。
- 把 Linux generic netlink family 接到 NuttX netlink。

### `nuttx/drivers/wireless/virtual`

这是 NuttX sim 使用的虚拟无线硬件目录：

```text
nuttx/drivers/wireless/virtual/
  Kconfig
  Make.defs
  virtual_hwsim.c
  mac80211_hwsim_linux.c
  mac80211_hwsim_linux.h
  virt_wifi_linux.c
```

当前真正参与 full-stack hwsim 验证的是：

- `virtual_hwsim.c`
- `mac80211_hwsim_linux.c`

`virt_wifi_linux.c` 保留为 Linux virtual wireless vendor reference，
当前不是主要验证路径。

### `apps/wireless/wifi`

这是 NuttX apps 侧 userspace Wi-Fi 移植目录：

```text
apps/wireless/wifi/
  Kconfig
  Make.defs
  Makefile
  common/
  libnl-3.2.25/
  wpa_supplicant-2.11/
  hostapd-2.11/
  wpa_hostapd_sources.mk
```

关键职责：

- 编译 libnl、wpa_supplicant、hostapd。
- 提供 common NuttX port headers，避免直接污染上游导入头文件。
- 使用 Linux `nl80211.h` UAPI 与 NuttX 内核侧 nl80211/cfg80211 对接。
- 在 NuttX sim 下启用足够多的 hostapd/wpa_supplicant 功能用于验证。

## Kconfig 接入

### 内核侧 IEEE 802.11

`nuttx/wireless/ieee80211/Kconfig` 增加核心开关：

```text
CONFIG_WIRELESS_IEEE80211_CFG80211_LINUX
CONFIG_WIRELESS_IEEE80211_NL80211_METADATA_ONLY
CONFIG_NL80211_TESTMODE
CONFIG_WIRELESS_IEEE80211_MAC80211_LINUX
```

含义：

- `CFG80211_LINUX`：启用 Linux cfg80211/nl80211 vendor drop。
- `NL80211_METADATA_ONLY`：早期用于只注册 nl80211 generic-netlink metadata，
  让 libnl/hostapd/wpa_supplicant 能完成 family discovery。
- `NL80211_TESTMODE`：启用 nl80211 testmode，用于 hwsim 私有验证命令。
- `MAC80211_LINUX`：启用 Linux mac80211 vendor drop，并自动 select cfg80211。

### 虚拟无线驱动

`nuttx/drivers/wireless/virtual/Kconfig` 增加：

```text
CONFIG_WL_VIRTUAL
CONFIG_WL_NUTTX_HWSIM
CONFIG_WL_NUTTX_HWSIM_DEBUG
CONFIG_WL_NUTTX_HWSIM_RADIO_BASE
CONFIG_WL_NUTTX_HWSIM_RADIOS
CONFIG_WL_NUTTX_HWSIM_CHANNELS
CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES
CONFIG_WL_NUTTX_HWSIM_MLO_SUPPORT
CONFIG_WL_NUTTX_HWSIM_MULTI_RADIO_SUPPORT
CONFIG_WL_NUTTX_HWSIM_AMPDU_PROOF
CONFIG_WL_NUTTX_HWSIM_AMSDU_PROOF
CONFIG_WL_NUTTX_HWSIM_DCM_PROOF
CONFIG_WL_NUTTX_HWSIM_PS_PROOF
CONFIG_WL_VIRTUAL_LINUX_VENDOR_DROP
```

其中 `WL_NUTTX_HWSIM` 是 NuttX sim full-stack hwsim 的主开关。
`RADIO_BASE` 用于让 AP/STA1/STA2 这些独立 sim 进程生成不同 MAC。
`MEDIUM_MAX_BYTES` 控制 hostfs shared frame medium 的上限，当前默认 64 MiB。

### userspace Wi-Fi apps

`apps/wireless/wifi/Kconfig` 增加：

```text
CONFIG_WIRELESS_WIFI_PORTS
CONFIG_WIRELESS_WIFI_LIBNL
CONFIG_WIRELESS_WIFI_WPA_SUPPLICANT
CONFIG_WIRELESS_WIFI_HOSTAPD
CONFIG_WIRELESS_WIFI_BUILD_EXPERIMENTAL
```

`BUILD_EXPERIMENTAL` 会选择 libnl、wpa_supplicant 和 hostapd，方便 sim 验证镜像一次性打开。

## Make/CMake 接入

### `nuttx/wireless/ieee80211/Make.defs`

当启用 Linux cfg80211/mac80211 移植时，统一加入：

```text
CSRCS += ieee80211/genetlink_bridge.c
CSRCS += ieee80211/ieee80211_linux_init.c
CSRCS += ieee80211/netdevice_compat.c
CFLAGS += -include wireless/ieee80211/include/linux/cfg80211_compat.h
```

并 include：

```text
ieee80211/cfg80211/Make.defs
ieee80211/mac80211/Make.defs
```

这里的 `-include cfg80211_compat.h` 很关键：它让大部分 Linux 源文件在不逐个改 include 的情况下，
获得 NuttX 适配宏、类型和函数声明。

### `nuttx/drivers/wireless/virtual/Make.defs`

当启用 `CONFIG_WL_NUTTX_HWSIM`：

```text
CSRCS += virtual_hwsim.c
CSRCS += mac80211_hwsim_linux.c
```

也就是说当前 sim 验证路径实际编译 Linux hwsim 移植主体。

### `apps/wireless/wifi/Makefile`

apps 侧加入：

```text
CFLAGS += apps/wireless/wifi/common/include
CFLAGS += apps/wireless/wifi/libnl-3.2.25/include
CFLAGS += apps/wireless/wifi/libnl-3.2.25/lib
CFLAGS += apps/wireless/wifi/wpa_supplicant-2.11/src
CFLAGS += apps/wireless/wifi/hostapd-2.11/hostapd
CFLAGS += nuttx/wireless/ieee80211/include/uapi
CFLAGS += -include nuttx_libnl_compat.h
```

`wpa_hostapd_sources.mk` 手工列出需要编进 NuttX app image 的 wpa_supplicant/hostapd 源文件。
这比直接复用 upstream Makefile 更可控，也更适合 NuttX builtin application 构建方式。

## Linux compat headers

`nuttx/wireless/ieee80211/include/` 下提供大量 Linux 头兼容：

```text
include/linux/
include/net/
include/crypto/
include/uapi/linux/
include/trace/
include/kunit/
```

覆盖的主要能力包括：

- 基础类型、bitops、list、rbtree、xarray、idr。
- `sk_buff`、`net_device`、`wireless_dev`、`wiphy`、`cfg80211`、`mac80211`。
- mutex/spinlock/RCU/workqueue/timer/jiffies/ktime。
- debugfs/sysfs/module/module_param/platform_device/rfkill 等 stub 或轻量实现。
- crypto/aead/hash/skcipher/arc4/aes/cmac/gmac 等 mac80211 需要的接口。
- Linux UAPI：`nl80211.h`、`genetlink.h`、`netlink.h`、`wireless.h`、`rfkill.h`。

当前核心原则是：优先让 Linux 源码继续 include 它熟悉的 Linux 头，
而不是把 Linux 源码大量改写成 NuttX 风格。

## 初始化桥

`ieee80211_linux_init.c` 提供统一入口：

```text
ieee80211_linux_initialize()
  -> cfg80211_linux_initialize()
  -> mac80211_linux_initialize()
  -> mac80211_hwsim_linux_initialize()
```

这个函数由 `virtual_hwsim_init()` 调用。它保证一个 sim 进程内的 Linux wireless 子系统按顺序初始化。
非 metadata-only 模式下会用 static `initialized` 防止重复初始化。

## NuttX netdev 和 Linux net_device 桥

`netdevice_compat.c` 是数据面桥的核心。

### 绑定

`virtual_hwsim_ifup()` 会调用：

```text
ieee80211_linux_sync_lowerhalf_mac(ifname, lower)
ieee80211_linux_bind_lowerhalf(ifname, lower)
ieee80211_linux_set_netdev_flags(ifname, true)
```

这样 NuttX `wlan0` lower-half 和 Linux compat `struct net_device` 绑定起来。

一个实际踩过的点：NuttX `<net/if.h>` 的 `IFF_UP` bit 和 Linux compat 的 up bit 不一致。
因此 `ieee80211_linux_set_netdev_flags()` 需要同时设置 Linux compat up bit 和 NuttX up bit，
否则 nl80211 侧会认为 interface down，scan trigger 返回错误。

### TX

NuttX 网络栈发包：

```text
netpkt_t
  -> ieee80211_linux_transmit_netpkt()
  -> copy to sk_buff
  -> linux_dev->netdev_ops->ndo_start_xmit()
  -> mac80211 TX
```

这里保留 Ethernet payload，交给 mac80211 转成 802.11 帧。

### RX

Linux mac80211 收包后：

```text
mac80211 RX
  -> netif_rx()
  -> ieee80211_linux_netif_rx()
  -> queue sk_buff for bound lower-half
  -> ieee80211_linux_receive_netpkt()
  -> copy to netpkt_t
  -> NuttX TCP/IP
```

这条路径让 ping/iperf 这种普通 NuttX socket 流量可以经过 Linux mac80211 解封装后回到 NuttX 网络栈。

### WEXT

`virtual_hwsim.c` 也把 NuttX wireless ioctl 转到：

```text
ieee80211_linux_wext_ioctl()
  -> wext_handle_ioctl()
```

这让简单 WEXT demo 能走到导入的 cfg80211 WEXT compat 层。
不过当前完整验证以 hostapd/wpa_supplicant 的 nl80211 路径为准，WEXT demo 不是 full-stack 通过标准。

## Generic Netlink 和 nl80211

为了让 libnl 和 hostapd/wpa_supplicant 能在 NuttX 里找到 `nl80211` family，
NuttX netlink 做了多处增强：

- 新增 `net/netlink/netlink_generic.c`。
- 扩展 `net/netlink/Kconfig`、`Make.defs`。
- 增强 `netlink_conn.c`、`netlink_sockif.c`、`netlink.h`。
- 扩展公开头：
  - `include/nuttx/net/netlink.h`
  - `include/netpacket/netlink.h`
  - `include/sys/socket.h`
- packet socket 路径补充：
  - `net/pkt/pkt_recvmsg.c`
- socket option 支持补充：
  - `net/socket/setsockopt.c`

`genetlink_bridge.c` 的作用是把导入 Linux 栈里的 `struct genl_family`
注册到 NuttX generic netlink 实现上，让 `CTRL_CMD_GETFAMILY`、family id、
multicast groups、dump/doit 回调等按 Linux nl80211 预期工作。

## cfg80211/nl80211 移植

`nuttx/wireless/ieee80211/cfg80211/` 来自 Linux `net/wireless`。

当前保留了大量原始文件，例如：

```text
core.c
nl80211.c
scan.c
reg.c
mlme.c
util.c
sme.c
ap.c
chan.c
wext-*.c
```

主要适配点：

- 通过 `cfg80211_compat.h` 提供 Linux kernel API shim。
- 通过 `genetlink_bridge.c` 接到 NuttX generic netlink。
- 通过 `netdevice_compat.c` 接到 NuttX netdev。
- debugfs/sysfs 相关能力以 stub 或轻量实现满足编译和运行。
- regulatory、scan、BSS、interface、key、AP/STA 等逻辑尽量保持 Linux 原语义。
- `NL80211_TESTMODE` 用于 hwsim 私有验证控制。

`include/uapi/linux/nl80211.h` 是 userspace 和 kernel-side 移植共同使用的 UAPI 基线。

## mac80211 移植

`nuttx/wireless/ieee80211/mac80211/` 来自 Linux `net/mac80211`。

当前保留的核心能力包括：

- STA/AP interface 管理。
- MLME、认证、关联、重关联。
- key install、CCMP/TKIP/GCMP/CMAC/GMAC 相关路径。
- EAPOL 和普通数据帧的 TX/RX 封装/解封装。
- HT/VHT/HE/S1G 能力处理。
- A-MPDU/A-MSDU 相关路径。
- power-save、PS-Poll、NullFunc、TWT 验证相关路径。
- minstrel/rate control 和 queue/airtime 等辅助模块。

移植重点不是重写 mac80211，而是补齐它依赖的 Linux kernel API：

- `sk_buff` 生命周期和 headroom/tailroom。
- workqueue/timer/jiffies。
- RCU/list/mutex/spinlock。
- `net_device` ops。
- crypto API。
- cfg80211/mac80211 driver ops。

## mac80211_hwsim 移植

`nuttx/drivers/wireless/virtual/mac80211_hwsim_linux.c` 来自 Linux
`drivers/net/wireless/virtual/mac80211_hwsim.c`，是当前 NuttX sim 虚拟硬件的核心。

原版 Linux hwsim 假设 AP/STA radios 位于同一个 Linux kernel instance。
NuttX sim 的 AP/STA 是多个独立 host 进程，所以这里增加了 NuttX 专用 hostfs shared medium：

```text
/h/hwsim-bss.bin
/h/hwsim-frames.bin
```

它负责把每个进程内 mac80211 要“发到空中”的 802.11 帧写成文件记录，
其他进程再轮询读取并注入本地 mac80211 RX。

详细运行机制见 `WIFI_HWSIM_ARCHITECTURE.md`。本文只记录移植含义：

- 保留 Linux hwsim radio/wiphy/ops 模型。
- 用 Kconfig 替代 Linux module parameter 的运行时参数。
- 用 hostfs medium 替代 Linux 内核全局 radio list 的跨进程共享能力。
- 增加 AP/STA/P2P/PS/TWT/DCM/S1G/AMPDU/AMSDU 等验证辅助开关和日志。
- 让 `virtual_hwsim.c` 对 NuttX 提供标准 netdev lower-half。

## userspace 移植

### libnl 3.2.25

`apps/wireless/wifi/libnl-3.2.25/` 导入 libnl。

适配点：

- `nuttx_libnl_compat.h` 补齐 NuttX 缺少或语义不同的 socket/netlink 宏。
- 对 libnl 的部分全局符号做 `nuttx_libnl_*` 重命名，避免和 NuttX 内核侧 netlink/generic-netlink
  同一镜像链接时发生符号冲突。
- 实现 `nuttx_wifi_if_nametoindex()` 并将 `if_nametoindex` 映射过去。
- 编译 genl、route/link 等 hostapd/wpa_supplicant nl80211 驱动需要的子集。

### wpa_supplicant 2.11

`apps/wireless/wifi/wpa_supplicant-2.11/` 导入 wpa_supplicant。

启用的主要功能通过 `wpa_hostapd_sources.mk` 管理，包括：

- `CONFIG_DRIVER_NL80211`
- `CONFIG_CTRL_IFACE`
- `CONFIG_CTRL_IFACE_UDP` for sim
- internal crypto/TLS/libtommath
- WPA2/WPA3 SAE/SAE-PK/OWE
- PMF/OCV/FILS/ERP
- P2P/DPP/GAS/HS20/interworking
- WNM/WPS/EAP peer methods
- HT/VHT/HE/TWT 相关 userspace 支持

sim 下使用 UDP control interface，避免依赖 Unix domain socket path 的目标差异。

### hostapd 2.11

`apps/wireless/wifi/hostapd-2.11/` 导入 hostapd。

启用的主要功能包括：

- AP 模式和 AP MLME。
- WPA/WPA2/WPA3/OWE/SAE/SAE-PK。
- PMF/OCV/FILS/FT/WNM/WPS。
- Enterprise EAP server。
- HS20/ANQP/GAS server。
- HT/VHT/HE/S1G 配置验证。

hostapd 通过 nl80211 `START_AP`、key install、management frame registration 等命令驱动
cfg80211/mac80211/hwsim。

### common port headers

`apps/wireless/wifi/common/include/` 当前集中放置公共移植头：

```text
nuttx_wifi_port.h
nuttx_wifi_config.h
nuttx_libnl_compat.h
```

这里保存 NuttX app-side 的公共宏、版本标识、libnl compat 和配置约定。
原则是把 NuttX 专用定义放在 common 里，而不是到处修改 upstream userspace header。

## NuttX core 改动

为了让 Linux wireless stack 能在 NuttX 里运行，除了新增目录外，还修改了若干 NuttX core 文件。
当前 `nuttx` submodule 中可见的主要修改包括：

```text
arch/sim/src/sim/sim_netdriver.c
boards/sim/sim/sim/scripts/Make.defs
drivers/net/netdev_upperhalf.c
drivers/wireless/Kconfig
drivers/wireless/Make.defs
include/netpacket/netlink.h
include/nuttx/net/netlink.h
include/sys/socket.h
net/netlink/Kconfig
net/netlink/Make.defs
net/netlink/netlink.h
net/netlink/netlink_conn.c
net/netlink/netlink_sockif.c
net/netlink/netlink_generic.c
net/pkt/pkt_recvmsg.c
net/socket/setsockopt.c
sched/environ/env_getenv.c
sched/environ/env_getenvironptr.c
wireless/Kconfig
wireless/Makefile
```

大类作用：

- sim board 网络初始化接入 `virtual_hwsim_init()`。
- wireless build system 接入 `wireless/ieee80211` 和 `drivers/wireless/virtual`。
- netlink/generic netlink 能支撑 libnl/nl80211。
- packet socket 能支撑 hostapd/wpa_supplicant l2 packet/EAPOL 路径。
- socket options 补齐 userspace 依赖。
- env/environ 适配 userspace 程序读取环境变量。
- netdev upper-half/WEXT 路径补齐 minimal demo 和 ioctl 兼容需要。

## sim board 配置和构建脚本

新增的 sim defconfig 主要位于：

```text
nuttx/boards/sim/sim/sim/configs/hwsim_ap/
nuttx/boards/sim/sim/sim/configs/hwsim_sta1/
nuttx/boards/sim/sim/sim/configs/hwsim_sta2/
nuttx/boards/sim/sim/sim/configs/hwsim_sta3/
nuttx/boards/sim/sim/sim/configs/hwsim_p2p1/
nuttx/boards/sim/sim/sim/configs/hwsim_p2p2/
nuttx/boards/sim/sim/sim/configs/hwsim_ap1/
nuttx/boards/sim/sim/sim/configs/hwsim_ap2/
nuttx/boards/sim/sim/sim/configs/hwsim_dynps_ap/
nuttx/boards/sim/sim/sim/configs/hwsim_dynps_sta1/
```

新增的构建脚本主要位于：

```text
tools/firmware/sim/build-ap.sh
tools/firmware/sim/build-sta1.sh
tools/firmware/sim/build-sta2.sh
tools/firmware/sim/build-sta3.sh
tools/firmware/sim/build-p2p1.sh
tools/firmware/sim/build-p2p2.sh
tools/firmware/sim/build-ap1.sh
tools/firmware/sim/build-ap2.sh
tools/firmware/sim/build-dynps-ap.sh
tools/firmware/sim/build-dynps-sta1.sh
tools/firmware/sim/build-hwsim-role.sh
```

注意：NuttX tree 只有一个当前 `.config` 和生成 include 状态。
这些角色构建必须顺序执行，不能 AP/STA1/STA2 并发构建，否则可能破坏生成状态。

## sim 配置文件和证书

`tools/firmware/sim/` 下还新增了大量 hostapd/wpa_supplicant 配置：

- open auth
- WPA2/WPA3/OWE/transition
- PMF/OCV/FILS/FT
- WPS/WNM/HS20/DPP/P2P
- HT/VHT/HE/S1G
- Enterprise EAP TLS/PEAP/TTLS/SuiteB

以及 EAP/Enterprise 验证用证书目录：

```text
tools/firmware/sim/eap-tls-certs/
tools/firmware/sim/suiteb-rsa3072-certs/
```

这些配置通过 hostfs mount 暴露给每个 `nuttx-sim-*`：

```text
mount -t hostfs -o fs=. /h
hostapd -dd /h/hostapd-hwsim.conf &
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim.conf &
```

## examples 和测试程序

当前 apps 侧新增：

```text
apps/examples/wifi_hwsim_ap/
apps/examples/wifi_hwsim_sta/
apps/examples/hwsim_testmode/
```

定位：

- `wifi_hwsim_ap` / `wifi_hwsim_sta` 是轻量 demo，用于验证 NuttX wireless/WEXT 边界。
- `hwsim_testmode` 用于 nl80211 testmode 和 hwsim 私有命令验证。
- 完整协议栈验证仍以 hostapd/wpa_supplicant 为准。

另外 `apps/netutils/iperf/iperf.c` 做了修改，让 iperf client 支持绑定源 IP。
这对多接口/P2P/STA-to-STA 验证很重要，否则流量可能从错误的本地地址发出。

## 当前验证状态

当前已记录的 full-stack 验证看：

```text
docs/WIFI_HWSIM_FULL_VALIDATION_MATRIX.md
```

其中 open auth 已证明：

- AP + STA1 + STA2 三个独立 `nuttx-sim-*` 进程可关联。
- STA-to-AP ping/iperf 可通。
- STA1 <-> STA2 ping/iperf 可通。
- AP + STA1 + STA2 + STA3 多 STA 也有通过证据。

更广的矩阵还覆盖：

- WPA2/WPA3/OWE/transition。
- PMF/OCV/FILS/FT/WPS/WNM/HS20/Enterprise。
- P2P GO/client。
- HT/VHT/HE/S1G。
- A-MPDU/A-MSDU/DCM/PS/TWT 等模拟器能力验证。

## 已知限制

### hostfs medium 不是最终硬件模型

当前 `/h/hwsim-bss.bin` 和 `/h/hwsim-frames.bin` 是 NuttX sim 多进程验证 transport。
它解决的是“多个独立 host 进程如何共享无线空气”的问题，不是最终真实硬件驱动形态。

限制包括：

- 文件 IO 和 20 ms 轮询影响吞吐和时序。
- debug-heavy iperf 可能触发 64 MiB shared medium truncate guard。
- TWT/PS 这类时序敏感功能需要更大的时间 margin。
- 长时 full-duplex TCP 仍需要 medium/backlog/performance 调优。

### WEXT demo 不是完整证明

`wifi_ap_demo` / `wifi_sta_demo` 走 WEXT/轻量路径。
完整 Linux Wi-Fi 协议栈验证应以 hostapd/wpa_supplicant 的 nl80211 路径为准。

### 仍有源码树清理项

当前源码目录里能看到一些构建产物和可疑临时项：

- `nuttx/wireless/ieee80211/cfg80211/*.o`、`*.gcno`
- `nuttx/wireless/ieee80211/mac80211/*.o`、`*.gcno`
- `nuttx/wireless/ieee80211/*.o`
- `nuttx/boards/sim/sim/sim/src/src`
- `apps/examples/hello_d/hello_d_main.d` 被删除

这些是否纳入最终提交需要单独确认。正常情况下 `.o/.gcno` 不应该作为源码提交；
`src/src` 这类自指向或生成痕迹也应在提交前清掉。

## 后续重构建议

当前阶段建议继续把 full-stack Linux 行为跑稳。之后如果要做原生化，可以按风险从低到高拆：

1. 固化 UAPI 和 userspace：保留 hostapd/wpa_supplicant/libnl 对 nl80211 的使用方式。
2. 把 NuttX generic netlink 和 packet socket 能力整理成可复用内核能力。
3. 把 Linux compat layer 中稳定的基础设施拆成更小模块。
4. 将 hostfs hwsim medium 抽象成独立 backend，后续可替换为 socket broker 或 shared-memory ring。
5. 再逐步评估 cfg80211/mac80211 哪些部分值得 NuttX 原生重写。

换句话说，当前移植的价值是先得到完整、可验证、接近 Linux 行为的无线协议基线；
后续原生重构应该以这条基线作为回归标准，而不是重新从空白状态摸索。

