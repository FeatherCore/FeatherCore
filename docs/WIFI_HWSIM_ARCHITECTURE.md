# NuttX Wi-Fi HWSIM Architecture

本文记录当前 FeatherCore NuttX sim 下虚拟 Wi-Fi 硬件链路的实现机制。
它描述的是当前代码里的可运行方案，不是最终理想形态。操作步骤看
`docs/WIFI_HWSIM_MANUAL_RECIPES.md`，功能覆盖和证据看
`docs/WIFI_HWSIM_FULL_VALIDATION_MATRIX.md`，移植过程中的问题和修复历史看
`docs/WIFI_HWSIM_PORTING_PROGRESS.md`。

## 目标

当前目标是在多个独立 `nuttx-sim-*` 进程中跑出接近 Linux
`mac80211_hwsim` 的验证环境：

```text
hostapd-2.11 / wpa_supplicant-2.11 / ping / iperf
  -> NuttX userspace socket/libnl/l2_packet
  -> NuttX netlink / generic netlink / packet socket
  -> nl80211
  -> cfg80211
  -> mac80211
  -> mac80211_hwsim
  -> hostfs shared medium
  -> peer nuttx-sim process
```

Linux 原版 `mac80211_hwsim.ko radios=2` 的关键特征是：所有 radios 都在同一个内核实例里，
AP、STA、P2P 等用户态进程共享同一个内核级模拟 RF 世界。NuttX sim 当前是多个 host 进程：
`nuttx-sim-ap`、`nuttx-sim-sta1`、`nuttx-sim-sta2` 分别有自己的地址空间，
所以不能直接依赖进程内全局链表来传递无线帧。

因此当前方案引入了一个 hostfs 文件介质，让多个 sim 进程通过共同挂载的 `/h`
交换 BSS 记录和 802.11 帧。

## 代码分层

主要代码位置：

- `nuttx/drivers/wireless/virtual/virtual_hwsim.c`
  - NuttX sim lower-half 入口。
  - 负责把 NuttX `netdev_lowerhalf_s` 绑定到 Linux compat `struct net_device`。
  - `ifup`、TX、RX、WEXT ioctl 都转发到 `ieee80211_linux_*` 桥接函数。
- `nuttx/drivers/wireless/virtual/mac80211_hwsim_linux.c`
  - 从 Linux `drivers/net/wireless/virtual/mac80211_hwsim.c` 移植来的 hwsim 主体。
  - 加了 NuttX sim 专用的 hostfs 共享介质、PS/TWT/DCM/S1G 等验证辅助逻辑。
- `nuttx/wireless/ieee80211/ieee80211_linux_init.c`
  - 统一初始化 cfg80211、nl80211、mac80211、mac80211_hwsim。
- `nuttx/wireless/ieee80211/netdevice_compat.c`
  - NuttX `netpkt_t` 和 Linux `sk_buff` / `net_device` 之间的桥。
  - 数据面 TX/RX 主要在这里跨过 NuttX 网络栈和 Linux wireless stack 的边界。
- `nuttx/wireless/ieee80211/genetlink_bridge.c`
  - 把导入的 Linux generic netlink family 接到 NuttX generic netlink 实现。
- `nuttx/net/netlink/netlink_generic.c`
  - NuttX 侧 generic netlink family 注册、查询、收发实现。
- `apps/wireless/wifi/`
  - 移植的 `wpa_supplicant-2.11`、`hostapd-2.11`、`libnl-3.2.25` 以及公共 compat 头文件。
- `tools/firmware/sim/build-*.sh`
  - 生成 AP、STA、P2P、dynps 等不同角色的 `nuttx-sim-*` 镜像。

## 进程模型

一次 AP + STA1 + STA2 验证通常包含三个独立 host 进程：

```text
Terminal A: build/nuttx-sim-ap
Terminal B: build/nuttx-sim-sta1
Terminal C: build/nuttx-sim-sta2
```

每个进程启动后都在 NSH 里执行：

```text
mount -t hostfs -o fs=. /h
```

如果这三个进程都从 `FeatherCore/tools/firmware/sim` 启动，那么它们看到的 `/h`
就是同一个 host 目录。于是：

- AP 在 `/h/hwsim-bss.bin` 发布 Beacon/Probe Response 对应的 BSS 记录。
- AP/STA 在 `/h/hwsim-frames.bin` 发布认证、关联、EAPOL、ARP/IP、Action 等 802.11 帧。
- 其他进程周期性读取这些文件，把匹配自己频道/BSSID/地址的记录注入本地 mac80211 RX。

这就是当前“hostfs 文件当无线空气介质”的核心。

## 初始化链路

sim board 初始化网络设备时，如果配置启用了 `CONFIG_WL_NUTTX_HWSIM`，
会走虚拟 hwsim lower-half：

```text
arch/sim/src/sim/sim_netdriver.c
  -> virtual_hwsim_init()

drivers/wireless/virtual/virtual_hwsim.c
  -> ieee80211_linux_initialize()

wireless/ieee80211/ieee80211_linux_init.c
  -> cfg80211_linux_initialize()
  -> mac80211_linux_initialize()
  -> mac80211_hwsim_linux_initialize()
```

`virtual_hwsim_ifup()` 做三件关键事：

```text
ieee80211_linux_sync_lowerhalf_mac(ifname, lower)
ieee80211_linux_bind_lowerhalf(ifname, lower)
ieee80211_linux_set_netdev_flags(ifname, true)
```

这样 NuttX 网络设备名，例如 `wlan0`，和导入 Linux wireless stack 里的
`struct net_device` 绑定起来。后续 NuttX 网络栈发出的 Ethernet 包可以进入
Linux mac80211，Linux mac80211 解出来的 Ethernet 包也可以回到 NuttX 网络栈。

## 控制面

控制面主要用于 hostapd/wpa_supplicant 创建接口、扫描、认证、关联、启动 AP、
设置 key、P2P、WPS、WNM、TWT 等：

```text
hostapd / wpa_supplicant
  -> libnl-3.2.25
  -> socket(AF_NETLINK, NETLINK_GENERIC)
  -> NuttX netlink sockif
  -> net/netlink/netlink_generic.c
  -> wireless/ieee80211/genetlink_bridge.c
  -> cfg80211/nl80211.c
  -> cfg80211 rdev ops
  -> mac80211 cfg ops
  -> mac80211_hwsim driver ops
```

几个关键点：

- `nl80211` 是 hostapd/wpa_supplicant 的主入口。
- `cfg80211` 负责 wiphy、interface、scan、AP/STA 状态、regulatory、key 等中间层语义。
- `mac80211` 负责 802.11 MLME、加密封装/解封装、聚合、省电、管理帧处理等软件 MAC 行为。
- `mac80211_hwsim` 扮演虚拟硬件，把 mac80211 要发到空中的 802.11 帧写入共享介质。

轻量 `wifi_ap_demo` / `wifi_sta_demo` 走 WEXT 路径，不是当前完整验证主路径。
当前完整路径以 hostapd/wpa_supplicant 的 nl80211 控制面为准。

## 数据面 TX

以 STA ping AP 为例，发送方向大致是：

```text
ping / iperf
  -> NuttX TCP/IP
  -> wlan0 lower-half transmit
  -> virtual_hwsim_transmit()
  -> ieee80211_linux_transmit_netpkt()
  -> copy netpkt_t payload into sk_buff
  -> linux_dev->netdev_ops->ndo_start_xmit()
  -> mac80211 TX path
  -> mac80211_hwsim TX callback
  -> append 802.11 frame record to /h/hwsim-frames.bin
```

`netdevice_compat.c` 在这里完成 `netpkt_t` 到 `sk_buff` 的转换。
`mac80211` 会把 NuttX 网络栈给出的 Ethernet 帧转换成 802.11 帧；
如果是 WPA/WPA2/WPA3 数据，它还会走相应的 key、PN、MIC、EAPOL/数据帧逻辑。

最终写入 `/h/hwsim-frames.bin` 的是已经经过 mac80211 处理的 802.11 帧，
不是简单的 Ethernet 原包。

## 数据面 RX

接收方向由 peer 进程轮询共享介质触发：

```text
/h/hwsim-frames.bin
  -> peer mac80211_hwsim poll work
  -> read new frame records from previous offset
  -> channel/address/BSSID/filter checks
  -> mac80211_hwsim_rx()
  -> ieee80211_rx_irqsafe()
  -> mac80211 RX path
  -> decrypt/defrag/decap to Ethernet
  -> netif_rx()
  -> ieee80211_linux_netif_rx()
  -> queue sk_buff for bound NuttX lower-half
  -> virtual_hwsim_receive()
  -> copy sk_buff into netpkt_t
  -> NuttX TCP/IP
  -> ping / iperf socket
```

这里有两个方向的格式转换：

- 共享介质到 mac80211：读取文件里的 802.11 帧，构造 RX status 后注入 mac80211。
- mac80211 到 NuttX：mac80211 解封装后调用 `netif_rx()`，compat 层把 skb 排到 lower-half
  的 RX 队列里，NuttX 网络栈再取回 `netpkt_t`。

## hostfs 共享介质

当前 hwsim 共享介质有两个主要文件：

```text
/h/hwsim-bss.bin
/h/hwsim-frames.bin
```

常量定义在 `drivers/wireless/virtual/mac80211_hwsim_linux.c`：

```text
NUTTX_HWSIM_MEDIUM_BSS_PATH        "/h/hwsim-bss.bin"
NUTTX_HWSIM_MEDIUM_FRAME_PATH      "/h/hwsim-frames.bin"
NUTTX_HWSIM_MEDIUM_MAGIC           0x4e485753
NUTTX_HWSIM_MEDIUM_VERSION         1
NUTTX_HWSIM_MEDIUM_MAX_FRAME       4096
NUTTX_HWSIM_MEDIUM_MAX_BSS_RECORDS 16
NUTTX_HWSIM_MEDIUM_MAX_RECORDS_PER_POLL 512
NUTTX_HWSIM_MEDIUM_POLL_MS         20
CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES 64 MiB default
```

### BSS 记录

`/h/hwsim-bss.bin` 保存固定数量的 BSS 记录，每条记录包含：

```text
magic
version
len
freq
freq_offset
band
signal
frame[4096]
```

frame 里保存 Beacon 或 Probe Response。AP 启动后，当 hwsim 看到
`BSS_CHANGED_BEACON_ENABLED`，会生成一次 beacon template 并发布到
`hwsim-bss.bin`。STA 扫描时读取该文件，频点匹配后调用
`cfg80211_inform_bss_frame_data()` 和 `mac80211_hwsim_rx()`，从而让
wpa_supplicant 看到 scan result。

### Frame 记录

`/h/hwsim-frames.bin` 是 append-only 风格的帧日志，每条记录包含：

```text
magic
version
len
freq
freq_offset
band
signal
src[6]
dst[6]
bssid[6]
frame[4096]
```

它承载的不只是管理帧，还包括：

- Authentication / Association / Reassociation
- Probe Request / Probe Response / Beacon
- EAPOL
- ARP / IPv4 / ICMP / TCP / UDP
- 802.11 Action frame，例如 WNM、TWT、P2P、DPP、WPS 相关帧
- PS-Poll、NullFunc、QoS/data 等 mac80211 省电和聚合验证需要的帧

每个 sim 进程维护自己的读取 offset。读者不加全局锁，遇到半条记录或不完整写入时下一轮重试。
写者使用 `open(O_CREAT | O_EXCL)` 形式的文件锁保护 append 和 bounded truncate。

### 为什么不用一个共享内存链表

Linux hwsim 可以依赖内核全局状态，因为 AP/STA 都在同一个 kernel 里。
NuttX sim 的 AP/STA 是不同 host 进程；如果要共享内存，需要额外设计跨进程 daemon、
共享内存段、socket broker 或真实 netlink/virtio 后端。

hostfs 文件介质的优点是：

- 不需要额外后台进程。
- 每个 sim 终端只要 mount 同一个目录即可互通。
- 失败时可以直接查看 `hwsim-bss.bin` / `hwsim-frames.bin`。
- 很适合当前“把完整 nl80211/cfg80211/mac80211/hwsim 跑起来”的阶段。

缺点也很明确：

- 轮询粒度和 hostfs IO 会限制吞吐。
- debug-heavy iperf 容易触发 medium 上限和截断路径。
- 文件不是严格实时介质，时序测试需要留 margin。
- 长时间高压 full-duplex 场景仍需要做 medium/backlog/performance 调优。

## 扫描和关联

AP 启动：

```text
hostapd
  -> NL80211_CMD_START_AP
  -> cfg80211 start_ap
  -> mac80211 start_ap / beacon setup
  -> mac80211_hwsim publishes BSS to /h/hwsim-bss.bin
```

STA 扫描：

```text
wpa_supplicant
  -> NL80211_CMD_TRIGGER_SCAN
  -> cfg80211 scan
  -> mac80211 hw_scan
  -> mac80211_hwsim hw_scan_work
  -> read /h/hwsim-bss.bin
  -> cfg80211_inform_bss_frame_data()
  -> scan result visible to wpa_supplicant
```

STA 关联：

```text
wpa_supplicant
  -> nl80211 connect/auth/assoc flow
  -> mac80211 emits Authentication / Association frames
  -> STA writes records to /h/hwsim-frames.bin
  -> AP polls and injects frames into its mac80211 RX
  -> hostapd receives AP-STA-CONNECTED after assoc success
```

加密网络继续通过同一个 frame medium 传 EAPOL。4-way handshake 完成后，
mac80211 使用已安装 key 对数据帧进行封装/解封装，ping/iperf 继续走数据面。

## AP 转发和 STA-to-STA

STA1 ping STA2 时，链路不是 STA1 直接写给 STA2 就结束，而是经过 AP 的 802.11 转发语义：

```text
STA1 NuttX IP
  -> STA1 mac80211 data TX
  -> /h/hwsim-frames.bin
  -> AP mac80211 RX
  -> AP NuttX IP / forwarding path
  -> AP mac80211 data TX toward STA2
  -> /h/hwsim-frames.bin
  -> STA2 mac80211 RX
  -> STA2 NuttX IP
```

当前验证矩阵里已经有 AP + STA1 + STA2、AP + STA1 + STA2 + STA3 的 ping/iperf 证据。
这说明当前链路不仅能完成关联，也能让 AP 在多 STA 数据面中承担转发角色。

## 和 Linux 原版 hwsim 的差异

相同点：

- 用户态仍然是 hostapd/wpa_supplicant + libnl + nl80211。
- 中间层仍然是 cfg80211 + mac80211。
- 虚拟硬件仍然是从 Linux `mac80211_hwsim` 移植来的模型。
- 管理帧、EAPOL、数据帧尽量让 mac80211 原始逻辑处理。

不同点：

- Linux 原版 radios 在同一个 kernel global hwsim 实例里。
- NuttX sim radios 分散在多个 host process 里。
- 当前 NuttX 通过 hostfs 文件传递“空中帧”，而不是内核内存链表或 hwsim netlink multicast。
- 当前 throughput 受 hostfs、轮询、debug log、sim 调度影响，不能代表真实 PHY 性能。

## 调试信号

常见有效日志：

```text
hwsim-debug: published BSS frame ... path=/h/hwsim-bss.bin
hwsim-debug: injected shared BSS frame ... path=/h/hwsim-bss.bin
hwsim-debug: published shared frame ...
hwsim-debug: injected shared frame ...
netdevice_compat: tx lower wlan0 ...
netdevice_compat: rx linux wlan0 ...
netlink_generic: registered family nl80211 ...
genl_bridge: netlink_generic_register(...)
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED ...
CTRL-EVENT-CONNECTED
EAPOL-4WAY-HS-COMPLETED
```

常见问题定位：

- STA 扫描 0 BSS：先确认每个 sim 都 mount 了同一个 `/h`，再看 AP 是否发布
  `hwsim-bss.bin`。
- AP/STA 关联卡住：看 `/h/hwsim-frames.bin` 是否有认证/关联帧，AP 是否注入 shared frame。
- EAPOL 卡住：看 0x888e 帧是否通过 shared frame medium，hostapd/wpa_supplicant 是否完成 key install。
- ping 第一次失败：可能是 ARP warm-up 或省电/TWT 窗口时序问题，通常需要重复 ping 或先跑短 ping 预热。
- iperf 吞吐低：当前多半是 hostfs 介质、debug log、sim 调度和 TCP 窗口共同影响，不等价于协议失败。
- 出现 `truncating shared frame medium`：说明达到配置的共享介质上限，当前默认 64 MiB。

## 当前限制

当前方案已经能支撑大量功能验证，但仍有边界：

- hostfs medium 是验证 transport，不是最终硬件抽象。
- 长时间高压 full-duplex TCP 仍可能受 medium backlog 影响。
- 自动省电/TWT 类测试对时间窗口敏感，sim/hostfs 轮询会放大边界问题。
- minimal WEXT demo 不能代表完整 hostapd/wpa_supplicant 路径。
- throughput 只能用于发现回归和粗略比较，不能作为真实无线性能指标。

如果后续要把这套 sim 做成更稳定的长期验证平台，建议把 hostfs medium 抽象成独立模块，
再评估是否替换为 Unix domain socket broker、共享内存 ring、或一个专门的 hwsim daemon。
这样可以保留当前 cfg80211/mac80211/hwsim 语义，同时减少文件 IO 和轮询带来的时序噪声。

