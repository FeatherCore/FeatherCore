# NuttX sim BT/BLE hwsim 移植入口

## 当前落地内容

本目录新增 4 个和 Wi-Fi hwsim 角色一致的构建入口：

```bash
tools/firmware/sim/build-bt1.sh
tools/firmware/sim/build-bt2.sh
tools/firmware/sim/build-ble1.sh
tools/firmware/sim/build-ble2.sh
```

它们分别构建以下独立 defconfig：

```text
sim:hwsim_bt1
sim:hwsim_bt2
sim:hwsim_ble1
sim:hwsim_ble2
```

输出文件沿用 Wi-Fi hwsim 约定：

```text
build/nuttx-sim-bt1
build/nuttx-sim-bt2
build/nuttx-sim-ble1
build/nuttx-sim-ble2
```

同时新增 Linux-port 专用 nsh 命令骨架：

```text
btctl
btaudio
```

这两个命令已经进入四套 defconfig，并通过 `linux_bt_*` 语义 API 进入
Linux-port core。当前必须遵守 upstream-first 规则：能从 Linux
`net/bluetooth` 直接移植的协议逻辑优先直接移植；只有 NuttX/Unix 兼容缺口
才放入 compat/shim。core 当前会把 HCI/L2CAP/GATT/A2DP/ISO 语义动作编码成
`SIM_BTHWSIM` 公共文件介质里的 synthetic records，用来先验证四个独立
sim 终端之间的 hwsim 数据路径。第一阶段已经具备 Linux-like HCI control、
ACL/L2CAP、SMP、ATT/GATT、LE advertising、A2DP media、ISO record 的端到端
注入/读取能力，并新增最小 `hci_conn`、`l2cap_chan`、`att_db` 风格对象表、
`iso_path` 风格 BIG/BIS 表、本地 HCI event ring 和最小 Linux mgmt
controller settings；完整 Linux `net/bluetooth` 后端正在按 staged Kconfig
逐步接入。

## 语义目标

最终目标不是简单转发 H4 字节流，也不是从零重写 NuttX 原生蓝牙，而是直接移植
Linux Bluetooth host/controller 语义：

```text
NuttX apps / BlueZ-like nsh command
  -> Linux Bluetooth socket / mgmt / profile API surface
  -> imported Linux net/bluetooth host semantics
  -> imported Linux drivers/bluetooth/hci_vhci.c
  -> SIM_BTHWSIM public-file ACL/ADV/ISO medium
  -> peer upstream VHCI write_iter()/hci_recv_frame()
  -> peer imported Linux Bluetooth stack
```

这和 Wi-Fi `mac80211_hwsim` 的文件介质模型对应：

```text
Wi-Fi: hwsim-bss.bin + hwsim-frames.bin
BT:    bt-hwsim-ctrl.bin + bt-hwsim-acl.bin + bt-hwsim-iso.bin
BLE:   bt-hwsim-adv.bin + bt-hwsim-acl.bin + bt-hwsim-iso.bin
```

建议公共文件 record 头：

```c
struct bthwsim_record {
  uint32_t magic;      /* "BTHS" */
  uint16_t version;    /* 1 */
  uint16_t type;       /* EVT, ACL, ISO, ADV, CTRL */
  uint16_t src;        /* bt1/bt2/ble1/ble2 role id */
  uint16_t dst;        /* peer id or broadcast */
  uint32_t seq;
  uint64_t timestamp_us;
  uint32_t payload_len;
  uint32_t crc32;
  uint8_t  payload[];  /* HCI payload or simulated air packet */
};
```

## 当前 defconfig 分层

第一阶段配置不再启用 NuttX 原来的 `wireless/bluetooth`、`BTSAK`、`NimBLE`
或 `SIM_HCISOCKET` 路径，而是建立 Linux-port 专用命名空间：

- `hwsim_bt1/hwsim_bt2` 启用 `NET_LINUX_BLUETOOTH`、`SIM_BTHWSIM`，mode 为 `bredr`。
- `hwsim_ble1/hwsim_ble2` 启用 `NET_LINUX_BLUETOOTH`、`SIM_BTHWSIM`，mode 为 `le`。
- 每个配置设置独立 `CONFIG_LIBC_HOSTNAME` 和 `CONFIG_SIM_BTHWSIM_ROLE`，用于公共文件介质里的源/目的过滤。

`SIM_BTHWSIM` 当前已经创建公共介质文件：

```text
/tmp/nuttx-bthwsim/bt-hwsim-ctrl.bin
/tmp/nuttx-bthwsim/bt-hwsim-adv.bin
/tmp/nuttx-bthwsim/bt-hwsim-acl.bin
/tmp/nuttx-bthwsim/bt-hwsim-iso.bin
```

并为每个 role/type 维护独立读取 offset 文件，同时为 Linux-like 连接语义
维护状态文件：

```text
/tmp/nuttx-bthwsim/linux-bt-conn-<self>-<peer>.state
```

当前连接 handle 使用稳定公式生成，保证两个 sim 端对同一条连接得到一致
handle：

```text
handle = 0x0040 + min(self, peer) * 16 + max(self, peer)
```

后续要在该后端上补齐 Linux HCI controller event、ACL、advertising、ISO/BIS
调度语义。

## 后续驱动实现拆分

1. `CONFIG_SIM_BTHWSIM` 已新增。
2. `arch/sim/src/sim` 已新增 BT hwsim hostfile 后端：
   - 已创建公共介质文件。
   - 已支持 append-only 写入公共介质文件。
   - 已支持每个 role/type 保存独立 read offset。
   - 已支持按 role 跳过自己写入的 record。
   - 已支持 broadcast record 读取。
   - 已支持 raw binary record 读取，并使用独立 `.raw.roleX.off` offset，避免 `btctl poll` 文本调试消费 upstream HCI 注入数据。
   - 已支持 Linux-like connection state 文件。
   - 后续还要补精确 fanout、丢包/延迟/信道策略。
3. NuttX 侧已新增 Linux-port BT core 命名空间和 upstream import 区：
   - Linux kernel 侧分层固定为：`net/bluetooth` 语义进入 `nuttx/wireless/linux_bluetooth`，`drivers/bluetooth` 语义进入 `nuttx/drivers/bluetooth`；`apps/wireless/linux_bluetooth` 只放 nsh/BlueZ-like 用户态验证工具。
   - BT port 的通用 Linux compat 不应重复发明 Wi-Fi/mac80211 已经实现的内容；`wireless/linux_bluetooth` 和 `drivers/bluetooth` 的 Make/CMake 已把 `wireless/ieee80211/include` 作为 fallback include 路径，优先复用已有 `linux/*.h` 兼容层。
   - 上游 Linux 源码已导入 `wireless/linux_bluetooth/upstream/net_bluetooth`。
   - 上游 Linux 头文件已导入 `wireless/linux_bluetooth/upstream/include_net_bluetooth`。
   - 上游 Linux 虚拟 HCI driver 已导入 `wireless/linux_bluetooth/upstream/drivers_bluetooth/hci_vhci.c`。
   - `hci_vhci.c` 的构建归属已迁到 `nuttx/drivers/bluetooth`，保持和 Linux `drivers/bluetooth` 同层；`drivers/bluetooth/linux_bt_upstream_vhci.c` 只保留 NuttX-facing VHCI open/read/write/pump 边界。
   - compat 骨架位于 `wireless/linux_bluetooth/upstream/compat`。
   - compat 当前已补入最低层 `linux/types.h`、`linux/kernel.h`、`linux/list.h`、`linux/slab.h`、`linux/err.h`、`linux/skbuff.h`、`linux/spinlock.h`、`linux/mutex.h`、`linux/atomic.h`、`linux/refcount.h`、`linux/workqueue.h`、`linux/wait.h`、`linux/idr.h`、`linux/rculist.h`、`linux/srcu.h`、`linux/leds.h`、`linux/unaligned.h`、`linux/fs.h`、`linux/miscdevice.h`、`linux/debugfs.h`、`net/sock.h` 等头骨架，并映射 `<net/bluetooth/*.h>` 到导入的上游头文件。
   - compat debugfs attribute 已从空 stub 推进为可读写 `file_operations`；`kstrtobool_from_user()`、`kstrtoull_from_user()` 覆盖 `hci_vhci.c` 的 force_suspend/force_wakeup/msft_opcode/aosp_capable 输入路径。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_SOURCES` 是逐步编译上游文件的总开关，默认关闭，避免未完成 compat 时破坏 sim bring-up。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_LIB` 第一阶段只编 `upstream/net_bluetooth/lib.c`。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_EIR`、`CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_CODEC` 和 `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI` 单独控制，分别等待 Linux `hci_core.h/struct hci_dev/uuid list`、`sk_buff/HCI sync`、`miscdevice/file operation/sk_buff queue` compat 就绪后再启用。
   - `CONFIG_DRIVERS_BLUETOOTH_LINUX_HCI_VHCI` 是 NuttX driver 层的 VHCI 源文件构建开关，默认跟随 `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI`。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_AF` 当前在四个 hwsim defconfig 默认关闭；Linux 原始 `af_bluetooth.c` 已保留在 upstream import 区和 staged Kconfig 中，但第一阶段不直接编入 sim 镜像，避免它和当前已经可运行的 NuttX-facing `PF_BLUETOOTH`/HCI/L2CAP/ISO shim 双重注册。后续补齐 Linux socket core compat 后，再用 upstream `bt_init()` 接管该层。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI_AUTOSTART` 可在 `SIM_BTHWSIM` ready 后自动 open upstream VHCI；四个 hwsim defconfig 已默认打开，使每个 nsh 终端启动时都有 upstream VHCI miscdevice 实例。
   - 已新增 `drivers/bluetooth/linux_bt_upstream_vhci.c` 作为 NuttX-facing VHCI port 边界；`btctl upstream` 可从 nsh 查看当前 upstream VHCI 接入状态。
   - compat 里的 `<net/bluetooth/hci_core.h>` 当前是 VHCI 阶段 wrapper：保留 upstream `hci.h`/`bluetooth.h`，声明 `hci_vhci.c` 所需的 `struct hci_dev`、quirk、register/free、`hci_recv_frame()` 边界。
   - `linux_bt_upstream_hci.c` 维护共享 HCI device registry，避免 header static inline 让每个 upstream 编译单元各自拥有一份假 controller 状态；`btctl upstream` 会输出已注册 `hci_dev`、bus、quirks、rx/tx 计数。
   - `btctl upstream` 还会输出 AF/VHCI 状态：`upstream-af` 行显示 `PF_BLUETOOTH` 是否已经通过 upstream `bt_init()` 注册；VHCI 行里的 `m2h` 表示 medium->host write_iter 次数，`h2m` 表示 host->medium drain 次数，`read-empty` 表示 `vhci_read()` 当前无 H4 数据。
   - compat `module_misc_device(vhci_miscdev)` 现在会生成可调用注册函数；`btctl upstream open` 或 `NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI_AUTOSTART` 会注册 upstream `vhci` miscdevice 并沿 `hci_vhci.c` 自己的 `vhci_fops.open()` 创建默认 VHCI 实例。
   - `hci_recv_frame()` 当前按 Linux VHCI 方向作为 controller/medium -> host 的入口，只做 HCI RX 侧 staging/accounting，不把收到的 packet 回写公共介质。
   - `btctl upstream poll` 会从 raw 公共介质读取 acl/iso record，封装成 H4 packet 后走 upstream `vhci_fops.write_iter()`；完整 Linux `hci_core.c` event queue/socket wakeup 后续再接管。
   - `btctl upstream read` 会走 upstream `vhci_fops.read()`，读取 host->controller 方向排队的 H4 字节流。
   - `btctl upstream drain` 会走 upstream `vhci_fops.read()`，解析 H4 packet type，只把 ACL/ISO host->controller 数据写回 `SIM_BTHWSIM` 公共介质；HCI command/event/vendor 保持在本地 host/controller 控制面，空队列的 `-EAGAIN` 会被视为 drain 完成。
   - `btctl upstream create [opcode]` 会通过 upstream `vhci_fops.write_iter()` 写入 H4 vendor packet，让 imported `hci_vhci.c` 自己执行 `vhci_create_device()`。
   - `btctl upstream pump` 是 `drain` + `poll` 的组合命令，用于两个独立 nsh 终端手动推进公共文件介质。
   - `btctl upstream socket hci|l2cap|iso` 会从用户态触发 upstream `PF_BLUETOOTH` family 的 `create()` 路径，用于观察 BlueZ 继续接入时的 socket 边界。当前 `hci` 已通过 guarded staging `hci_sock_init()` 走 `proto_register()`、注册 `BTPROTO_HCI` 并走 upstream `bt_sock_alloc()`；HCI staging create 也已补 `proto_ops`、`SS_UNCONNECTED` state 和 release op，probe 会输出 `state/ops/sk-state/sk-proto`。`btctl upstream socket hci raw|user|monitor|control|logging [dev]` 会继续触发 staging bind，RAW/USER 绑定到具体 HCI index，CONTROL/MONITOR/LOGGING 绑定到 `HCI_DEV_NONE`，并在绑定具体 hdev 时调用 `getname(peer=0)` 读回 Linux `sockaddr_hci`。`btctl upstream socket-filter <dev> <type-mask> <event-mask0> <event-mask1> [opcode]` 会创建 RAW HCI socket，走 `setsockopt(SOL_HCI, HCI_FILTER)` / `getsockopt(SOL_HCI, HCI_FILTER)`，并让 controller->host RAW fanout 按 packet type、event mask 和 command opcode 过滤。`l2cap` 现在也会通过 guarded staging `l2cap_init()` 注册 `BTPROTO_L2CAP`，并用 upstream `bt_sock_alloc()` 分配 `struct l2cap_pinfo`；`btctl upstream socket l2cap <psm> [cid]` 会构造 imported upstream `struct sockaddr_l2` bind 请求，记录本地 PSM/CID，并把 socket 推进到 `BT_BOUND`。`btctl upstream l2cap-send <psm> <cid> <handle> <hex...>` 会创建并 bind L2CAP socket，通过 socket `sendmsg()` 把 payload 包装成 HCI ACL data + L2CAP basic header，再进入 upstream VHCI send path，用于观察 BlueZ L2CAP/A2DP socket -> kernel -> VHCI 方向。controller/medium -> host 方向也有协议 socket fanout：`hci_recv_frame()` 收到 ACL 后会解析 ACL/L2CAP header，并把 L2CAP payload 按 CID 入队到 matching bound L2CAP socket；收到 ISO 后会解析 HCI ISO header，并把 ISO SDU payload 入队到 bound ISO socket。`btctl upstream l2cap-bind <psm> <cid> <handle>` 和 `btctl upstream iso-bind <addr-type> <handle>` 会在当前 sim 实例里保留一个 staging 协议 socket，便于另一个终端发送、当前终端 pump/poll 后用 `btctl upstream l2cap-recv [max]` 或 `btctl upstream iso-recv [max]` 读取队列 payload。`iso` 现在也会通过 guarded staging `iso_init()` 注册 `BTPROTO_ISO`；`btctl upstream socket iso [addr-type]` 会用 imported upstream `struct sockaddr_iso` bind 请求记录本地地址类型，并把 socket 推进到 `BT_BOUND`。`btctl upstream iso-send <addr-type> <handle> <hex...>` 会创建并 bind ISO socket，通过 socket `sendmsg()` 把 payload 包装成 HCI ISO data packet，再进入 upstream VHCI send path，用于观察 BlueZ ISO socket -> kernel -> VHCI 方向。L2CAP/ISO 的完整 connect/session/reassembly/flow-control 语义仍保持 staging，等待后续启用 upstream L2CAP socket/channel 和 upstream `iso.c` 文件。
   - `btctl upstream mgmt-socket <opcode> [index] [param]` 会创建 upstream `PF_BLUETOOTH/BTPROTO_HCI` socket，bind 到 `HCI_CHANNEL_CONTROL`，再用 Linux `struct mgmt_hdr` 形状走 HCI socket `sendmsg()` staging 边界。当前 `mgmt_init()` 会像 Linux 一样注册 fallback `hci_mgmt_chan`，control bind 也要求 channel 已注册；sendmsg 经 channel handler table 分发，并执行 upstream 风格的 handler data length、`MGMT_INDEX_NONE` 和 controller index 校验。`READ_VERSION`、`READ_COMMANDS`、`READ_INDEX_LIST`、`READ_INFO` 已能返回 Linux mgmt binary command-complete 形状，其中全局命令默认使用 `MGMT_INDEX_NONE=0xffff`，controller 命令默认使用 index 0。设置类命令暂时转发到已有最小 `linux_bt_mgmt_dispatch()`，随后会把 `MGMT_EV_CMD_COMPLETE` skb 放入 socket receive queue，并可通过 staging `recvmsg()` 读回；成功设置还会额外排入 `MGMT_EV_NEW_SETTINGS`。`START_DISCOVERY` / `STOP_DISCOVERY` 现在维护一份 staging discovery state，并向其他 control socket 入队 `MGMT_EV_DISCOVERING`。它是 BlueZ mgmt control-channel 的语义探针，不表示完整 upstream `mgmt.c`/`hci_sock.c` 已经接管。
   - `btctl upstream mgmt-poll-discovery [max]` 会在 discovery active 时读取 `SIM_BTHWSIM` ADV raw record，并把 `HCI_LE_ADV_REPORT ... name=...` 转成 Linux 形状 `MGMT_EV_DEVICE_FOUND`。当前地址由 peer sim role 合成稳定 LE public address，EIR 使用 Complete Local Name；后续应由 upstream `hci_event.c` 的 LE Advertising Report 解析和 upstream `mgmt_device_found()` 接管。
   - `PAIR_DEVICE` / `CANCEL_PAIR_DEVICE` / `UNPAIR_DEVICE` 已补第一层 staging：`btctl upstream mgmt-socket 0x0019 0 <addr-seed>` 会发送 Linux `mgmt_cp_pair_device`，使用合成 LE public address 和 NoInputNoOutput IO capability，创建或更新 staging device-list entry 并立即标记 paired；`0x001a` 会走 `MGMT_OP_CANCEL_PAIR_DEVICE`，当前 immediate-complete 模式下通常无 pending pair 可取消；`0x001b` 会发送 `mgmt_cp_unpair_device`，默认 `disconnect=1`，清除 paired 状态，必要时清理 matching connection snapshot，并向其他 control socket 入队 `MGMT_EV_DEVICE_UNPAIRED`。后续应由 upstream pending command、SMP 和 key distribution 接管。
   - compat `struct proto` 已补 `owner/obj_size`，`sk_alloc()` 会按 `proto->obj_size` 分配更大的 socket 私有对象；这对后续 upstream `hci_sock.c` 的 `struct hci_pinfo`、L2CAP/ISO socket 私有结构是必要条件。
   - 已为 upstream `hci_sock.c` 补直接 include 缺口：导入 Linux `hci_mon.h`，新增 `linux/compat.h` 的 `compat_ptr()`，并把 `linux/utsname.h` 移入共享 `wireless/linux_compat`，提供 `release/machine`。
   - 已继续补 upstream `hci_sock.c` 的 runtime 兼容：`hci_dev` wrapper 现在有 `flags/dev_flags/promisc`、`hci_dev_get()` 和 flag helpers；socket compat 现在有 `proto_ops`、`msghdr/msg_iter`、`sk_write_queue`、`sock_queue_rcv_skb()`、`sock_orphan()`、`datagram_poll()` 和标准 `sock_no_*` helpers。
   - HCI monitor/logging staging 已补第一层：`HCI_CHANNEL_MONITOR` socket 会登记到本地 monitor list，control socket open/close、mgmt command 和 mgmt event 会按 `hci_mon_hdr` 形状广播为 `HCI_MON_CTRL_OPEN/CLOSE/COMMAND/EVENT`；`HCI_CHANNEL_LOGGING` sendmsg 会校验 logging frame，并按 `HCI_MON_USER_LOGGING` 转发给 monitor socket。这是后续接入 upstream `hci_sock.c` monitor replay 和 BlueZ `btmon` 可观测面的入口。
   - RAW/USER HCI socket TX staging 已补第一层：`HCI_CHANNEL_RAW` / `HCI_CHANNEL_USER` sendmsg 现在接受 Linux HCI socket 的 H4 形状，首字节为 HCI packet type，后续 payload 进入绑定 `hci_dev->send()`。发送前会检查已绑定 hdev、`HCI_UP` 和合法 packet type；通过 `btctl upstream mgmt-socket 0x0005 0 1` 设置 powered 时会同步 `hci_dev_open()`，使 RAW/USER TX 能按 Linux 语义从 user socket 到 upstream VHCI read queue。
   - RAW/USER HCI socket RX fanout 也已补第一层：`hci_recv_frame()` 在 controller/medium -> host 方向更新 HCI 统计后，会把匹配 `hci_dev` 的 RAW/USER socket 加入 receive queue，数据仍使用 Linux raw socket 可见的 H4 形状。RAW channel 接收 command/event/ACL/SCO/ISO，USER channel 接收 event/ACL/SCO/ISO/DRV；同时会向 monitor socket 广播 RX monitor event。`btctl upstream` 会输出 `hci-data-socket-register/unregister/rx` 计数。
   - `btctl upstream send cmd|acl|iso|event <payload>` 会构造 Linux `sk_buff`、设置 `hci_skb_pkt_type()`，再调用默认 `hci_dev->send()`，作为后续 upstream mgmt/hci_sock host->driver TX 的占位入口。
   - `btctl upstream sendhex cmd|acl|iso|event <hex...>` 和 `send` 使用同一条 TX 边界，但 payload 是真实字节，例如 HCI Reset command payload 为 `03 0c 00`。
   - `btaudio upstream-a2dp-source start` 会先走 staged `BTPROTO_L2CAP` socket `sendmsg()`，再把 synthetic A2DP media 打包成 HCI ACL + L2CAP CID `0x0041`，继续走 upstream VHCI `send -> readq -> drain -> hwsim ACL -> peer poll -> write_iter`。
   - `btaudio upstream-a2dp-sink start|read|stop` 会在接收端保持 staged L2CAP socket：`start` bind/listen A2DP media CID，`read` 先 poll hwsim 介质再从 socket receive queue 读取 payload，`stop` release socket。
   - `btaudio upstream-le-broadcast-source start [big] [bis]` 会先走 staged `BTPROTO_ISO` socket `sendmsg()`，再把 synthetic LE Audio media 打包成 HCI ISO，继续走 upstream VHCI ISO 数据面。
   - `btaudio upstream-le-broadcast-sink sync|start|stop [big] [bis] [max]` 会在接收端保持 staged ISO socket：`sync` bind/connect BIG/BIS handle，`start` poll ISO 介质并 recv ISO SDU，`stop` release socket。
   - `hwsim_bt1`、`hwsim_bt2`、`hwsim_ble1`、`hwsim_ble2` defconfig 现在显式启用 `NET_LINUX_BLUETOOTH_UPSTREAM_SOURCES`、`NET_LINUX_BLUETOOTH_UPSTREAM_LIB`、`NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI`、`NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI_AUTOSTART`、`DRIVERS_BLUETOOTH_LINUX` 和 `DRIVERS_BLUETOOTH_LINUX_HCI_VHCI`，并显式关闭 `NET_LINUX_BLUETOOTH_UPSTREAM_AF`，让四个 sim 入口默认带起 upstream `hci_vhci.c` 第一阶段和可运行的 staged socket/control/data 面。
   - 已为剩余 upstream host stack 文件补齐分阶段构建开关，其中 `AF_BLUETOOTH`、`HCI_CORE`、`HCI_SOCK`、`MGMT`、`L2CAP`、`SMP`、`ISO` 当前仍默认关闭。后续每打开一组，就以 Linux upstream 源文件替换一块临时 shim。
   - `AF_BLUETOOTH` 阶段已补第一层 socket compat：`sock_register()` / `sock_unregister()` staging registry、共享 `init_net`、`struct sock/socket/net_proto_family`、skb receive/error queue、wait/poll/ioctl/proc/seq 外壳，以及可调用的 `subsys_initcall(bt_init)` bridge。不过 Linux 原始 `af_bluetooth.c` 仍需要继续补完整 socket core、wait/ioctl/proc 和 `bt_sock` 依赖；当前 hwsim defconfig 暂不启用该源文件。
4. 当前临时 sim/hwsim shim：
   - `btctl` 触发 HCI control、LE ADV、ACL/L2CAP、ATT/GATT synthetic records。
   - `btaudio` 触发 A2DP ACL media 和 LE Audio ISO synthetic records。
   - connect/disconnect 已有 request/event/state 的第一阶段语义。
   - ACL poll 已能处理 `L2CAP_ECHO_REQ`、`ATT_READ_REQ`、`ATT_WRITE_REQ`、`AVDTP_START`，并生成对应 response record。
   - L2CAP signaling 已能处理 `CONN_REQ/RSP`、`CONFIG_REQ/RSP`、`DISCONN_REQ/RSP`，并推进 channel state。
   - SMP skeleton 已能处理 `PAIRING_REQ/RSP/CONFIRM/RANDOM` 的 Just Works 流程，并在 connection table 标记 `paired=1`。
   - 已新增本地对象表：connection table、L2CAP channel table、minimal ATT attribute table。
   - 已新增 ISO/BIG/BIS path table：记录 BIG、BIS、handle、source/sink、streaming/synced、codec 和 seq。
   - `btctl state` 可观察当前 role、connection、L2CAP channel、ATT DB 与 ISO path。
   - `btctl events` 可消费本地 HCI event ring，用于承载 command status/complete、connection complete、completed packets、BIG/BIS events 等本机 host 可见事件。
   - `btctl mgmt` 通过最小 Linux mgmt opcode dispatch 操作 controller settings：powered、connectable、discoverable、bondable、BR/EDR、LE、advertising。
5. 下一步在 NuttX 侧补齐更完整的 Linux-like HCI controller：
   - 优先适配 `upstream/net_bluetooth/{mgmt.c,hci_core.c,hci_event.c,hci_sock.c,l2cap_core.c,smp.c,iso.c}`。
  - 同步适配 `upstream/drivers_bluetooth/hci_vhci.c`，策略对齐 Wi-Fi `mac80211_hwsim_linux.c`：保持 Linux driver 源码接近原样，外侧只加 `drivers/bluetooth/linux_bt_upstream_vhci.c` 这样的 NuttX initialize/bind 层。
  - 当前 VHCI 阶段 wrapper 不能替代完整 Linux `hci_core.c`；它只是为了让 `hci_vhci.c` 的 driver 边界先和 NuttX sim/hwsim 介质对接。
   - 当前 host->driver TX 已有 `linux_bt_upstream_hci_send()` 入口；后续要由 upstream `mgmt.c` / `hci_sock.c` 生成真实 HCI command/ACL/ISO payload，而不是继续用调试字符串。
  - 当前 raw medium poll 已经进入 upstream `hci_vhci.c` 的 `write_iter()` / `hci_recv_frame()` 边界；后续要换成 upstream `hci_core.c` 的真实 rx work、event dispatch、L2CAP/ISO fanout。
   - 当前已有最小 command complete/status event ring；后续要替换成真正 HCI event queue 和 socket wakeup。
   - 当前已有最小 mgmt opcode dispatch facade；后续要替换成导入的 upstream `mgmt.c` socket opcode/event 分发。
   - staged build 顺序建议为：`AF_BLUETOOTH` -> `HCI_CORE` -> `HCI_SOCK/MGMT` -> `L2CAP` -> `SMP` -> `ISO`，每一步都以 upstream 源文件接管当前 shim；其中 `AF_BLUETOOTH` 已有兼容层雏形但当前默认关闭，`BTPROTO_HCI` 已有临时 create/bind/sendmsg 边界，`sk_alloc/proto_register`、`hci_sock.c` 直接 include 兼容和部分 HCI device/socket runtime 兼容已推进，下一步应继续拆 upstream `hci_sock.c` 的 mgmt channel、monitor replay、binary response/event queue 和 recvmsg 依赖。
   - 当前 `HCI_CHANNEL_CONTROL` 的 staging `sendmsg()` 已能接收 Linux mgmt header，并通过 fallback `hci_mgmt_chan` 支持 `READ_VERSION`、`READ_COMMANDS`、`READ_INDEX_LIST`、`READ_INFO`、`SET_POWERED`、`SET_DISCOVERABLE`、`SET_CONNECTABLE`、`SET_BONDABLE`、`SET_LE`、`SET_ADVERTISING`、`SET_BREDR`、`PAIR_DEVICE`、`CANCEL_PAIR_DEVICE`、`UNPAIR_DEVICE`、`START_DISCOVERY`、`STOP_DISCOVERY`、`BLOCK_DEVICE`、`UNBLOCK_DEVICE`、`GET_CONN_INFO`、`GET_CLOCK_INFO`、`ADD_DEVICE`、`REMOVE_DEVICE`、`GET_DEVICE_FLAGS`、`SET_DEVICE_FLAGS` 等 opcode。staging `recvmsg()` 现在会从 `sk_receive_queue` 取出 mgmt event skb，调用方缓冲不足时设置 `MSG_TRUNC`；`btctl upstream mgmt-socket` 使用较大的接收缓冲，能观察当前 `READ_INFO` / `READ_COMMANDS` binary response，并输出 recv flags。`btctl upstream` 状态会记录 `hci-mgmt-socket-cmd/event/recv`、`hci-mgmt-chan-register/unregister`、`hci-monitor-register/unregister/event` 和 `hci-data-socket-register/unregister/rx` 计数。这个边界服务于“一终端一个 sim”的 BlueZ-like 控制启动路径：每个 bt/ble sim 都能拥有自己的 upstream AF/VHCI/HCI control socket、RAW/USER HCI socket 和 monitor 可观测入口，再通过公共文件 ACL/ADV/ISO 介质和另一个 sim 通信。
   - ACL credit、完整 L2CAP socket/channel lifecycle、真实 SMP key distribution/security mode、完整 ATT/GATT database。
   - LE advertising report、connection complete、BIG/BIS/ISO events。
6. BT 基础闭环：
   - nsh 中用 Linux-port `btctl` 扫描、连接、发送基础 ACL payload。
7. BLE 基础闭环：
   - nsh 中用 Linux-port `btctl` 完成 advertise/scan/connect/GATT read-write。
8. Audio 第一阶段：
   - A2DP：先模拟 AVDTP signaling + L2CAP media channel，synthetic frame 通过 hwsim ACL record。
   - LE Audio：先模拟 BIG/BIS + ISO SDU，synthetic frame 通过 hwsim ISO record。

## 预期 nsh 使用方式

四个终端分别运行：

```bash
build/nuttx-sim-bt1
build/nuttx-sim-bt2
build/nuttx-sim-ble1
build/nuttx-sim-ble2
```

BT 基础命令目标：

```text
nsh> btctl info
nsh> btctl mgmt status
nsh> btctl mgmt power on
nsh> btctl mgmt connectable on
nsh> btctl mgmt discoverable on
nsh> btctl state
nsh> btctl events
nsh> btctl upstream
nsh> btctl upstream open
nsh> btctl upstream create
nsh> btctl upstream send acl synthetic_payload
nsh> btctl upstream sendhex cmd 03 0c 00
nsh> btctl upstream read
nsh> btctl upstream drain
nsh> btctl upstream poll
nsh> btctl upstream pump
nsh> btctl upstream socket hci
nsh> btctl upstream socket hci raw 0
nsh> btctl upstream socket hci control
nsh> btctl upstream mgmt-socket 0x0001
nsh> btctl upstream mgmt-socket 0x0002
nsh> btctl upstream mgmt-socket 0x0003
nsh> btctl upstream mgmt-socket 0x0004 0
nsh> btctl upstream mgmt-socket 0x0005 0 1
nsh> btctl upstream socket-send raw 0 cmd 03 0c 00
nsh> btctl upstream socket-filter 0 0xffffffff 0xffffffff 0xffffffff
nsh> btctl upstream socket-ioctl 0
nsh> btctl upstream socket-ioctl 0 up
nsh> btctl upstream socket-ioctl 0 down
nsh> btctl upstream socket-ioctl 0 reset
nsh> btctl upstream socket-ioctl 0 restat
nsh> btctl upstream socket-ioctl 0 scan 3
nsh> btctl upstream socket-ioctl 0 linkmode 0x8001
nsh> btctl upstream socket-ioctl 0 aclmtu 0x00400010
nsh> btctl upstream socket-ioctl 0 connlist
nsh> btctl upstream socket-ioctl 0 conninfo acl
nsh> btctl upstream socket-ioctl 0 conninfo bis
nsh> btctl upstream socket-ioctl 0 authinfo
nsh> btctl upstream socket-ioctl 0 block 1
nsh> btctl upstream socket-ioctl 0 unblock 1
nsh> btctl upstream mgmt-socket 0x0019 0 1
nsh> btctl upstream mgmt-socket 0x001a 0 1
nsh> btctl upstream mgmt-socket 0x001b 0 1
nsh> btctl upstream mgmt-socket 0x0023 0 7
nsh> btctl upstream mgmt-poll-discovery
nsh> btctl upstream mgmt-socket 0x0024 0 7
nsh> btctl upstream mgmt-socket 0x0026 0 1
nsh> btctl upstream mgmt-socket 0x0027 0 1
nsh> btctl upstream mgmt-socket 0x0031 0 0
nsh> btctl upstream mgmt-socket 0x0032 0 0
nsh> btctl upstream mgmt-socket 0x0033 0 1
nsh> btctl upstream mgmt-socket 0x004f 0 1
nsh> btctl upstream mgmt-socket 0x0050 0 1
nsh> btctl upstream mgmt-socket 0x0034 0 1
nsh> btctl upstream mgmt-listen
nsh> btctl upstream mgmt-read
nsh> btctl upstream mgmt-close
nsh> btctl upstream socket hci monitor
nsh> btctl upstream socket l2cap
nsh> btctl upstream socket iso
nsh> btctl upstream l2cap-bind 0x0019 0x0041 0x0040
nsh> btctl upstream l2cap-connect 0x0019 0x0041
nsh> btctl upstream l2cap-listen
nsh> btctl upstream l2cap-write 01 02 03 04
nsh> btctl upstream l2cap-send 0x0019 0x0041 0x0040 01 02 03 04
nsh> btctl upstream l2cap-recv
nsh> btctl upstream l2cap-close
nsh> btctl upstream iso-bind 0 0x0101
nsh> btctl upstream iso-connect 0
nsh> btctl upstream iso-write 01 02 03 04
nsh> btctl upstream iso-send 0 0x0101 01 02 03 04
nsh> btctl upstream iso-recv
nsh> btctl upstream iso-close
nsh> btctl scan bredr
nsh> btctl connect <peer>
nsh> btctl scan bredr
nsh> btctl pair <peer>
nsh> btctl l2cap-connect <peer> <psm>
nsh> btctl l2cap-disconnect <peer> <cid>
nsh> btctl l2cap-send <peer> <payload>
nsh> btctl l2cap-echo <peer> <payload>
nsh> btctl poll ctrl
nsh> btctl poll acl
```

两终端 L2CAP/A2DP-like staging 数据面示例：

```text
# 终端 A / 接收端，保持一个 L2CAP socket
nsh> btctl upstream l2cap-bind 0x0019 0x0041 0x0040
nsh> btctl upstream l2cap-connect 0x0019 0x0041

# 终端 B / 发送端，保持一个 L2CAP socket 并多次发送
nsh> btctl upstream l2cap-bind 0x0019 0x0041 0x0040
nsh> btctl upstream l2cap-connect 0x0019 0x0041
nsh> btctl upstream l2cap-write 01 02 03 04
nsh> btctl upstream pump

# 终端 A / 接收端，从公共文件介质 poll 回 host，再读 protocol socket queue
nsh> btctl upstream pump
nsh> btctl upstream l2cap-recv
nsh> btctl upstream l2cap-close
```

两终端 ISO/LE Audio-like staging 数据面示例：

```text
# 终端 A / 接收端，保持一个 ISO socket
nsh> btctl upstream iso-bind 0 0x0101
nsh> btctl upstream iso-connect 0

# 终端 B / 发送端，保持一个 ISO socket 并多次发送
nsh> btctl upstream iso-bind 0 0x0101
nsh> btctl upstream iso-connect 0
nsh> btctl upstream iso-write 01 02 03 04
nsh> btctl upstream pump

# 终端 A / 接收端，从公共文件介质 poll 回 host，再读 protocol socket queue
nsh> btctl upstream pump
nsh> btctl upstream iso-recv
nsh> btctl upstream iso-close
```

BLE 基础命令目标：

```text
nsh> btctl advertise start
nsh> btctl mgmt advertising on
nsh> btctl state
nsh> btctl scan le
nsh> btctl connect <peer>
nsh> btctl pair <peer>
nsh> btctl gatt-read [peer] <handle>
nsh> btctl gatt-write [peer] <handle> <payload>
nsh> btctl poll adv
nsh> btctl poll acl
```

Audio 命令目标：

```text
nsh> btaudio a2dp-source start [peer]
nsh> btaudio a2dp-sink start
nsh> btaudio upstream-a2dp-source start
nsh> btaudio upstream-a2dp-sink start|read|stop [max]
nsh> btaudio le-broadcast-source create|start|stop [big] [bis]
nsh> btaudio le-broadcast-sink sync|start|stop [big] [bis]
nsh> btaudio upstream-le-broadcast-source start [big] [bis]
nsh> btaudio upstream-le-broadcast-sink sync|start|stop [big] [bis] [max]
```

两终端 upstream 音频 socket 数据面示例：

```text
# A2DP-like receiver terminal
nsh> btaudio upstream-a2dp-sink start
nsh> btaudio upstream-a2dp-sink read
nsh> btaudio upstream-a2dp-sink stop

# A2DP-like sender terminal
nsh> btaudio upstream-a2dp-source start
nsh> btctl upstream pump

# LE Audio-like receiver terminal
nsh> btaudio upstream-le-broadcast-sink sync 0 1
nsh> btaudio upstream-le-broadcast-sink start 0 1
nsh> btaudio upstream-le-broadcast-sink stop

# LE Audio-like sender terminal
nsh> btaudio upstream-le-broadcast-source start 0 1
nsh> btctl upstream pump
```

这些 `btctl`/`btaudio` 命令已经作为 nsh 入口新增；后续需要把命令接入
Linux-port 栈核心。语义应对齐 Linux BlueZ 已验证过的 A2DP MediaTransport
与 LE Audio ISO socket 数据路径。

当前 synthetic record 行为：

- `btctl info` 调用 `linux_bt_info()` 输出当前 role/mode/capability。
- `btctl mgmt status` 调用 `linux_bt_mgmt_status()` 输出 Linux-like mgmt index、settings、supported settings。
- `btctl mgmt power|connectable|discoverable|bondable|le|bredr on|off` 调用对应 `linux_bt_mgmt_set_*()`，更新 controller settings，并向本地 event ring 写入 `MGMT_EV_NEW_SETTINGS`。
- `btctl mgmt advertising on|off` 通过 `linux_bt_advertise_start/stop()` 启停 LE advertising，同时更新 mgmt advertising setting。
- `btctl state` 调用 `linux_bt_state()` 输出本地 Linux-like connection table、L2CAP channel table 和 ATT DB。
- `btctl events` 调用 `linux_bt_events()` 输出并清空本地 HCI event ring；该 ring 不走公共文件介质，用于模拟 controller 上报给本机 host 的事件。
- `btctl advertise start` 调用 `linux_bt_advertise_start()`，写入 LE advertising semantic record。
- `btctl scan le` 调用 `linux_bt_scan_le()`，读取 peer/broadcast ADV record，并同时推进 LE control 面的 connect/disconnect state。
- `btctl scan bredr` 调用 `linux_bt_scan_bredr()`，读取 control record；如果读到发给本 role 的 `HCI_CMD_CONNECT`，会记录 connected state，并回写 `HCI_EVT_CONN_COMPLETE` 给发起端；如果读到 `HCI_CMD_DISCONNECT`，会清理 state，并回写 `HCI_EVT_DISCONN_COMPLETE`。
- `btctl connect <peer>` 调用 `linux_bt_connect()`，写入 `HCI_CMD_CONNECT` semantic record，并把本端 state 标记为 `connecting`，等待 peer 侧 scan/process 后再收到 complete event；收到 `HCI_EVT_CONN_COMPLETE` 后本端 state 切换为 `connected`。
- `btctl disconnect <peer>` 调用 `linux_bt_disconnect()`，写入 `HCI_CMD_DISCONNECT` semantic record，并清理本端 state。
- `btctl pair <peer>` 调用 `linux_bt_pair()`，通过 fixed SMP channel `cid=0x0006` 发起 Just Works pairing skeleton；对端 `btctl poll acl` 推进 `SMP_PAIRING_RSP/CONFIRM/RANDOM`，最终在 connection table 标记 `paired=1`。
- `btctl l2cap-connect <peer> <psm>` 调用 `linux_bt_l2cap_connect()`，通过 signaling CID `0x0001` 发起 `L2CAP_CONN_REQ`，对端 `btctl poll acl` 生成 `CONN_RSP` 和 `CONFIG_REQ`，本端再 poll 生成 `CONFIG_RSP`，最后 channel 进入 `open`。
- `btctl l2cap-disconnect <peer> <cid>` 调用 `linux_bt_l2cap_disconnect()`，通过 signaling CID `0x0001` 发送 `L2CAP_DISCONN_REQ`，对端 poll 后生成 `DISCONN_RSP` 并关闭 channel。
- `btctl l2cap-send <peer> <payload>` 调用 `linux_bt_l2cap_send()`，写入 `HCI_ACL_DATA` + L2CAP semantic record，并在本端打开 dynamic L2CAP channel `cid=0x0040 psm=0x1001`。
- `btctl l2cap-echo <peer> <payload>` 调用 `linux_bt_l2cap_echo()`，写入 signaling CID `L2CAP_ECHO_REQ`；对端 `btctl poll acl` 会生成 `L2CAP_ECHO_RSP`。
- `btctl gatt-read/write` 调用 `linux_bt_gatt_read/write()`，写入 broadcast ATT semantic record。
- `btctl gatt-read <peer> <handle>` 与 `btctl gatt-write <peer> <handle> <payload>` 会按 peer 生成 ACL handle，并在本端打开 fixed ATT channel `cid=0x0004`；对端 `btctl poll acl` 会从 minimal ATT DB 生成 `ATT_READ_RSP`、`ATT_WRITE_RSP` 或 `ATT_ERROR_RSP`。
- `btaudio a2dp-source start` 调用 `linux_bt_a2dp_source_start()`，写入 broadcast `HCI_ACL_DATA` + AVDTP/A2DP media synthetic frame，主要用于早期调试。
- `btaudio a2dp-source start <peer>` 调用 `linux_bt_a2dp_source_start_peer()`，按连接 peer 生成稳定 ACL handle，打开 AVDTP media channel `cid=0x0041 psm=0x0019`，并把 A2DP media synthetic frame 发给指定对端。
- `btaudio a2dp-sink start` 调用 `linux_bt_a2dp_sink_poll()`，读取 ACL media synthetic frame；如果收到 `AVDTP_START`，会生成 `AVDTP_START_RSP state=streaming`。
- `btaudio le-broadcast-source create [big] [bis]` 调用 `linux_bt_le_broadcast_source_create()`，创建 source ISO path，并生成 `HCI_EVT_LE_CREATE_BIG_COMPLETE`。
- `btaudio le-broadcast-source start [big] [bis]` 调用 `linux_bt_le_broadcast_source_start_path()`，确保 source ISO path 处于 `streaming`，递增 seq，并写入 `HCI_ISO_DATA` + BIG/BIS synthetic frame。
- `btaudio le-broadcast-source stop [big] [bis]` 调用 `linux_bt_le_broadcast_source_stop()`，清理 source ISO path，并生成 `HCI_EVT_LE_TERMINATE_BIG_COMPLETE`。
- `btaudio le-broadcast-sink sync [big] [bis]` 调用 `linux_bt_le_broadcast_sink_sync()`，创建 sink ISO path 并进入 `syncing`。
- `btaudio le-broadcast-sink start [big] [bis]` 调用 `linux_bt_le_broadcast_sink_poll_path()`，读取 ISO synthetic frame；收到记录后 sink ISO path 进入 `synced` 并累计 seq。
- `btaudio le-broadcast-sink stop [big] [bis]` 调用 `linux_bt_le_broadcast_sink_stop()`，清理 sink ISO path，并生成 `HCI_EVT_LE_BIG_SYNC_LOST`。

## 第一阶段双终端操作示例

BT1 连接 BT2：

```text
bt1 nsh> btctl mgmt power on
bt2 nsh> btctl mgmt power on
bt1 nsh> btctl mgmt connectable on
bt2 nsh> btctl mgmt connectable on
bt1 nsh> btctl connect 2
bt1 nsh> btctl events
bt2 nsh> btctl scan bredr
bt2 nsh> btctl events
bt1 nsh> btctl scan bredr
bt1 nsh> btctl events
```

L2CAP Echo：

```text
bt1 nsh> btctl l2cap-echo 2 hello
bt2 nsh> btctl poll acl
bt1 nsh> btctl poll acl
```

L2CAP dynamic channel：

```text
bt1 nsh> btctl l2cap-connect 2 0x1001
bt2 nsh> btctl poll acl
bt1 nsh> btctl poll acl
bt2 nsh> btctl poll acl
bt1 nsh> btctl state
bt2 nsh> btctl state
bt1 nsh> btctl l2cap-disconnect 2 0x0040
bt2 nsh> btctl poll acl
bt1 nsh> btctl poll acl
```

BLE advertise/connect/GATT：

```text
ble1 nsh> btctl mgmt power on
ble2 nsh> btctl mgmt power on
ble1 nsh> btctl mgmt le on
ble2 nsh> btctl mgmt le on
ble2 nsh> btctl mgmt advertising on
ble1 nsh> btctl scan le
ble1 nsh> btctl connect 4
ble2 nsh> btctl scan le
ble1 nsh> btctl scan le
ble1 nsh> btctl pair 4
ble2 nsh> btctl poll acl
ble1 nsh> btctl poll acl
ble2 nsh> btctl poll acl
ble1 nsh> btctl poll acl
ble1 nsh> btctl gatt-read 4 0x0001
ble1 nsh> btctl events
ble2 nsh> btctl poll acl
ble2 nsh> btctl events
ble1 nsh> btctl poll acl
ble1 nsh> btctl events
ble1 nsh> btctl state
ble2 nsh> btctl state
```

A2DP synthetic media：

```text
bt1 nsh> btaudio a2dp-source start 2
bt2 nsh> btaudio a2dp-sink start
bt1 nsh> btctl poll acl
```

LE Audio broadcast synthetic ISO：

```text
ble1 nsh> btaudio le-broadcast-source create 0 1
ble1 nsh> btctl state
ble2 nsh> btaudio le-broadcast-sink sync 0 1
ble2 nsh> btctl state
ble1 nsh> btaudio le-broadcast-source start 0 1
ble1 nsh> btctl events
ble2 nsh> btaudio le-broadcast-sink start 0 1
ble2 nsh> btctl events
ble2 nsh> btctl state
ble1 nsh> btaudio le-broadcast-source stop 0 1
ble2 nsh> btaudio le-broadcast-sink stop 0 1
```

## 2026-06-11 mgmt IO capability staging

本轮继续补齐 Linux mgmt control-channel 的配对前置语义：

```text
MGMT_OP_SET_IO_CAPABILITY 0x0018
  payload: struct mgmt_cp_set_io_capability { io_capability }
  response: MGMT_EV_CMD_COMPLETE + 原 payload
  state: 更新 staging controller 默认 IO capability
```

`btctl upstream mgmt-socket` 现在可以先设置 IO capability，再触发 synthetic pair：

```bash
btctl upstream mgmt-socket 0x0018 0 3
btctl upstream mgmt-socket 0x0019 0 1
```

其中 `0x0019` 的 `PAIR_DEVICE` payload 不再固定写死 NoInputNoOutput，而是使用最近一次 `SET_IO_CAPABILITY` 保存的 controller 默认值。这一步仍然是 staging immediate-complete pair，后续要继续由 upstream SMP、pending command 和 key distribution 接管。

## 2026-06-11 mgmt user confirmation staging

继续补齐 Linux pairing control path：当 `PAIR_DEVICE` 使用的 IO capability 不是 `NoInputNoOutput(0x03)` 时，staging device entry 不再立即标记 paired，而是进入 pending confirmation 状态，并向其他 control socket 广播：

```text
MGMT_EV_USER_CONFIRM_REQUEST 0x000f
```

随后可从 nsh 发送确认或否认：

```bash
btctl upstream mgmt-socket 0x0018 0 1
btctl upstream mgmt-socket 0x0019 0 1
btctl upstream mgmt-socket 0x001c 0 1

btctl upstream mgmt-socket 0x001d 0 1
```

`0x001c` 对应 `MGMT_OP_USER_CONFIRM_REPLY`，会把 pending entry 标记为 paired；`0x001d` 对应 `MGMT_OP_USER_CONFIRM_NEG_REPLY`，会清除 pending 状态并保持 unpaired。当前 command-complete 时机仍是 staging 简化模型，后续要继续替换为 upstream `mgmt_pending_add()`、SMP 和 HCI user confirmation command complete 语义。

## 2026-06-11 mgmt pending-command staging

本轮把 pairing staging 从单纯 device-state pending 推进到更接近 Linux `mgmt_pending_add()` 的形态：

```text
PAIR_DEVICE(non-NoInputNoOutput)
  -> 保存发起 command 的 control socket / opcode / index / addr
  -> 广播 MGMT_EV_USER_CONFIRM_REQUEST
  -> 不立即返回 PAIR_DEVICE command-complete

USER_CONFIRM_REPLY
  -> 标记 paired
  -> 给原 PAIR_DEVICE socket 补发 MGMT_EV_CMD_COMPLETE(success)

USER_CONFIRM_NEG_REPLY / CANCEL_PAIR_DEVICE / REMOVE_DEVICE
  -> 清理 pending
  -> 给原 PAIR_DEVICE socket 补发失败或取消形态的 command-complete
```

这比前一版更接近 upstream `net/bluetooth/mgmt.c` 的 pending command ownership，但仍然没有接入真正的 SMP pairing method、HCI user-confirm command、key distribution 和持久 key store。下一步应把 pending entry 与 imported `smp.c` 的 method/key flow 对接。

## 2026-06-11 persistent mgmt-send staging

新增常驻 control socket 发送入口：

```bash
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x001c 0 1
btctl upstream mgmt-read
btctl upstream mgmt-close
```

`mgmt-send` 和 `mgmt-socket` 的区别是：`mgmt-send` 复用 `mgmt-listen` 打开的同一个 HCI control socket。这样 pending `PAIR_DEVICE` 保存的 socket owner 就能在后续 `USER_CONFIRM_REPLY` 后收到原 pair command-complete，更接近 BlueZ daemon 的常驻 mgmt socket 使用方式。

## 2026-06-11 mgmt passkey reply staging

新增 staged passkey pairing branch。当前 IO capability 约定：

```text
0x02: KeyboardOnly -> MGMT_EV_USER_PASSKEY_REQUEST
0x03: NoInputNoOutput -> immediate staged paired
其它非 0x03: MGMT_EV_USER_CONFIRM_REQUEST
```

可用 persistent mgmt socket 观察 passkey 路径：

```bash
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 2
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x001e 0 1
btctl upstream mgmt-read
btctl upstream mgmt-close
```

`0x001e` 对应 `MGMT_OP_USER_PASSKEY_REPLY`，当前 staged passkey 固定为 `123456`；`0x001f` 对应 `MGMT_OP_USER_PASSKEY_NEG_REPLY`，会失败并清除 pending pair。后续要把 passkey request/reply 与 upstream `smp.c` 的 method selection、confirm/random/key distribution 连接起来。

## 2026-06-11 staged LE LTK distribution

pairing 成功路径现在会生成一条 staged LE Long Term Key，并通过 mgmt control socket 广播：

```text
MGMT_EV_NEW_LONG_TERM_KEY 0x000a
  store_hint = 1
  key.addr = peer LE address
  key.type = unauthenticated 或 authenticated
  key.enc_size = 16
```

触发路径包括：

```text
NoInputNoOutput immediate pair success
USER_CONFIRM_REPLY success
USER_PASSKEY_REPLY success
```

这一步让 BlueZ/mgmt 可观察到 bonding key distribution 的第一块形状。当前 LTK 值仍由 staging 按 peer address 确定性生成，不代表真实 SMP confirm/random 派生结果；IRK、CSRK、BR/EDR link key 和持久 key store 还未接入。

## 2026-06-11 BT/BLE hwsim usecase test matrix

新增用例生成入口：

```bash
tools/firmware/sim/test-bt-hwsim-usecases.sh list
tools/firmware/sim/test-bt-hwsim-usecases.sh write
tools/firmware/sim/test-bt-hwsim-usecases.sh show bt-basic
```

默认会把每个用例的 per-terminal nsh 命令文件写到：

```text
build/bt-hwsim-usecases
```

当前覆盖矩阵：

```text
bt-basic:     bt1/bt2 BR/EDR mgmt、scan/connect/pair、L2CAP echo、state/events
ble-basic:    ble1/ble2 LE advertising/scan/connect/pair、GATT read/write
mgmt-noio:    persistent mgmt socket、NoInputNoOutput pair、LTK event
mgmt-confirm: persistent mgmt socket、user-confirm pair、pending complete
mgmt-passkey: persistent mgmt socket、passkey pair、pending complete
a2dp:         upstream L2CAP socket A2DP-like payload path
le-audio:     upstream ISO socket LE Audio-like payload path
```

运行模型仍保持和目标一致：一个 sim 占一个终端。脚本只生成命令文件，不自动启动四个 sim，也不替代真实日志判定。后续可以在这个矩阵上继续叠加 tmux/expect 自动化和 PASS/FAIL 日志解析。

## 2026-06-11 BT/BLE hwsim log validator

新增用例日志校验器：

```bash
tools/firmware/sim/validate-bt-hwsim-usecases.py --list
tools/firmware/sim/validate-bt-hwsim-usecases.py \
  --log-dir build/bt-hwsim-usecases
tools/firmware/sim/validate-bt-hwsim-usecases.py \
  --log-dir build/bt-hwsim-usecases --case mgmt-passkey --json
```

日志命名约定：

```text
<case>.<role>.log
```

例如：

```text
bt-basic.bt1.log
bt-basic.bt2.log
mgmt-passkey.ble1.log
le-audio.ble2.log
```

当前 validator 检查每个用例的关键用户可见输出：mgmt command/event、pair pending complete、LTK event、L2CAP/A2DP payload path 和 ISO/LE Audio payload path。它还会把 `PANIC`、`ASSERT`、`btctl: ... failed:`、`btaudio: ... failed:` 视为失败。真实四终端运行后，应把终端输出保存为上述日志，再运行 validator 作为 PASS/FAIL 记录入口。

## 2026-06-11 BT/BLE hwsim usecase runner

新增真实 sim 用例 runner：

```bash
tools/firmware/sim/run-bt-hwsim-usecases.py --case mgmt-passkey
tools/firmware/sim/run-bt-hwsim-usecases.py --case bt-basic --case a2dp
tools/firmware/sim/run-bt-hwsim-usecases.py
```

runner 会：

```text
1. 调用 test-bt-hwsim-usecases.sh write 生成 nsh 命令文件。
2. 按用例启动需要的 build/nuttx-sim-<role> 进程。
3. 向每个 role 的 stdin 写入对应 <case>.<role>.nsh 命令。
4. 保存 <case>.<role>.log。
5. 调用 validate-bt-hwsim-usecases.py 输出 PASS/FAIL。
```

默认日志目录：

```text
build/bt-hwsim-usecases
```

注意：这个 runner 是自动化入口，会真实启动 sim 进程；运行前应先完成对应构建入口：

```bash
tools/firmware/sim/build-bt1.sh
tools/firmware/sim/build-bt2.sh
tools/firmware/sim/build-ble1.sh
tools/firmware/sim/build-ble2.sh
```

本轮只新增 runner，没有实际启动四个 sim，也没有记录真实 PASS/FAIL。

## 2026-06-11 BT/BLE hwsim runner result manifest

`run-bt-hwsim-usecases.py` 现在会在日志目录写入机器可读结果清单：

```text
build/bt-hwsim-usecases/run-results.json
```

清单字段包括：

```text
cases: 本次运行的 case 列表
results: 每个 case 的 role、日志路径和 run_error
validate_rc: validate-bt-hwsim-usecases.py 返回码
passed: runner 和 validator 都成功时为 true
```

后续真实测试记录应同时保留：

```text
<case>.<role>.log
run-results.json
validator 文本输出或 --json 输出
```

这让“测试各个用例”的结果可以作为可审计证据，而不是只依赖终端肉眼观察。

## 2026-06-11 BT/BLE hwsim test preflight result

本轮执行了轻量 preflight 检查，结果：

```text
MISSING build/nuttx-sim-bt1
MISSING build/nuttx-sim-bt2
MISSING build/nuttx-sim-ble1
MISSING build/nuttx-sim-ble2
python syntax: PASS
usecase generator list: PASS
validator list: PASS
```

因此当前不能直接启动真实 usecase runner；需要先构建四个角色。新增 preflight 入口：

```bash
tools/firmware/sim/preflight-bt-hwsim-usecases.sh
```

构建入口仍是：

```bash
tools/firmware/sim/build-bt1.sh
tools/firmware/sim/build-bt2.sh
tools/firmware/sim/build-ble1.sh
tools/firmware/sim/build-ble2.sh
```
