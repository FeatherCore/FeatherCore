# ESP-Hosted NG ESP32C5 + STM32H7S78-DK Porting Notes

本文记录当前在 STM32H7S78-DK 上通过 SPI 驱动 ESP32C5 ESP-Hosted NG 固件的移植过程。目标链路是：

```text
wpa_supplicant
  -> libnl
  -> nl80211
  -> cfg80211
  -> esp_hosted_ng upstream host driver
  -> NuttX SPI transport
  -> STM32H7S78-DK SPI4
  -> ESP32C5 network_adapter firmware
  -> Wi-Fi firmware
```

当前工作树位于：

```text
/home/uan/Feather-develop-WIFI/FeatherCore_ESP
```

ESP-Hosted NG 上游源码位于：

```text
/home/uan/Feather-develop-WIFI/third/esp-hosted/esp_hosted_ng
```

ESP-IDF 位于：

```text
/home/uan/Feather-develop-WIFI/esp/esp-idf
```

## 当前状态

截至 2026-06-10：

- ESP32C5 固件已经能以 SPI 模式启动，并打印 `Using SPI interface`、`Using SPI MODE 2`。
- H7S78-DK 侧 SPI4 transport 能初始化，日志类似：

```text
esp_hosted_ng: SPI transport ready bus=4 freq=10000000 mode=2 hs=0 dr=0
esp_hosted_ng: chipset=ESP32C5 id=17 over SPI
esp_hosted_ng: SPI clock adjusted to 30000000 Hz
```

- ESP boot-up event 能被 H7 侧收到并解析。
- `wlan0` 已能通过 cfg80211 path 注册：

```text
nuttx: esp_cfg80211_add_iface: registered wlan0 mac=... iftype=2
```

- `wpa_supplicant -Dnl80211 -iwlan0 -c/etc/wpa_supplicant.conf -dd` 已能进入 nl80211/cfg80211 驱动路径。
- regdomain/channel 初始化问题已处理，scan request 不再在 nl80211/cfg80211 层因为信道全部 disabled 被直接拒绝。
- `wpa_supplicant` 已完成 WPA2-PSK 4-way handshake，日志出现 `WPA: Key negotiation completed`、`CTRL-EVENT-CONNECTED` 和 `EAPOL authentication completed - result=SUCCESS`。
- PTK/GTK 已能通过 nl80211/cfg80211 下发到 ESP-Hosted NG host driver，并由 ESP32C5 firmware 安装。
- `wlan0` 已通过 DHCP 获得可用 IPv4 配置，普通 IP 数据面已经验证到 `ping` 路由器成功、
  DNS 解析成功，并且 `ping baidu.com` 10/10 收包、0% 丢包。
- 当前已跑通的完整链路是：`wpa_supplicant -> libnl -> nl80211 -> cfg80211 -> esp_hosted_ng host driver -> NuttX SPI transport -> ESP32C5 network_adapter firmware -> real Wi-Fi AP`。
- 最新 NuttX 侧已补齐 ESP-Hosted NG driver 的 `dump_survey` 和 `flush_pmksa` cfg80211 ops；
  `NL80211_CMD_GET_SURVEY`、`NL80211_CMD_FLUSH_PMKSA` 不再需要依赖 unsupported fallback。
- 最新复测镜像 `Jun 10 2026 02:31:15` 已验证 `GET_SURVEY ret=0`、`FLUSH_PMKSA ret=0`、
  PTK/GTK `NEW_KEY ret=0`、`SET_STATION ret=0`、DHCP 成功，以及 `ping baidu.com`
  10/10 收包、0% 丢包。
- 当前仍可见的 `DEL_KEY ret=-67` 是 Linux 对齐的未连接阶段 key cleanup 返回；
  `SET_REKEY_OFFLOAD ret=-95` 是 Linux 对齐的可选 GTK rekey offload unsupported 返回。

## ESP32C5 固件构建

ESP32C5 侧使用 ESP-Hosted NG 的 `network_adapter` 固件。工程目录：

```text
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter
```

### 1. 加载 ESP-IDF 环境

从仓库根目录执行：

```sh
cd /home/uan/Feather-develop-WIFI
. esp/esp-idf/export.sh
```

如果 ESP-IDF 环境尚未安装，ESP-Hosted NG 官方流程是进入：

```sh
cd third/esp-hosted/esp_hosted_ng/esp/esp_driver
./setup.sh
```

但本工程已经使用本地 `esp/esp-idf`，因此优先使用上面的本地 IDF。

### 2. 配置 ESP32C5 target

```sh
cd /home/uan/Feather-develop-WIFI/third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter
idf.py set-target esp32c5
```

当前 `sdkconfig` 关键项：

```text
CONFIG_IDF_TARGET="esp32c5"
CONFIG_IDF_TARGET_ESP32C5=y
CONFIG_ESP_SPI_GPIO_HANDSHAKE=3
CONFIG_ESP_SPI_GPIO_DATA_READY=4
CONFIG_ESP_SPI_TX_Q_SIZE=20
CONFIG_ESP_SPI_RX_Q_SIZE=20
CONFIG_ESP_SPI_CHECKSUM=y
CONFIG_BT_ENABLED=y
```

ESP32C5 SPI 固件代码中的默认引脚来自：

```text
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter/main/spi_slave_api.c
```

当前 ESP32C5 SPI 引脚定义：

```text
MOSI: GPIO7
MISO: GPIO2
CLK : GPIO6
CS  : GPIO10
HS  : GPIO3
DR  : GPIO4
```

启动日志应看到：

```text
FW_SPI: Using SPI interface
FW_SPI: Using SPI MODE 2
FW_SPI: SPI Ctrl:1 mode: 2, GPIOs: MOSI: 7, MISO: 2, CS: 10, CLK: 6 HS: 3 DR: 4
FW_SPI: Hosted SPI queue size: Tx:20 Rx:20
```

### 3. 构建固件

```sh
idf.py build
```

构建输出位于：

```text
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter/build
```

关键产物：

```text
bootloader/bootloader.bin
partition_table/partition-table.bin
ota_data_initial.bin
network_adapter.bin
```

当前 `flash_args`：

```text
--flash_mode dio --flash_freq 80m --flash_size 4MB
0x2000 bootloader/bootloader.bin
0x10000 network_adapter.bin
0x8000 partition_table/partition-table.bin
0xd000 ota_data_initial.bin
```

### 4. 下载固件

先确认串口：

```sh
ls /dev/ttyACM* /dev/ttyUSB*
```

然后烧录：

```sh
idf.py -p /dev/ttyACM0 flash
```

串口可能是 `/dev/ttyACM0`、`/dev/ttyACM1` 或 `/dev/ttyUSB0`，以实际设备为准。

也可以直接 flash + monitor：

```sh
idf.py -p /dev/ttyACM0 flash monitor
```

## H7S78-DK 硬件连接

ESP-Hosted NG SPI 不只是 4 线 SPI，还需要 3 根辅助线：

- `HS` / handshake：ESP32C5 输出，表示 SPI slave 已经准备好进行一次 transaction。
- `DR` / data-ready：ESP32C5 输出，表示 ESP 有数据要发给 host。
- `EN` / reset：H7 输出，控制 ESP32C5 复位，低有效。

### 当前工程使用的连接

H7S78-DK 侧代码位于：

```text
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/src/stm32_spi.c
FeatherCore_ESP/nuttx/arch/arm/src/stm32h7rs/stm32_spi.c
```

当前板级代码使用 SPI4，并把 SPI4 的三个 AF 引脚配置在 `GPIOE 12..14`，片选使用 `PF8`：

```text
SPI4 signals : PE12 / PE13 / PE14, AF5
CS           : PF8, GPIO output, active low
HS           : PF1, GPIO input, rising-edge interrupt
DR           : PF2, GPIO input, rising-edge interrupt
EN/reset     : PF3, GPIO output, active low
```

当前代码注释将这些信号映射到 Arduino 口：

```text
D10 / PF8 : SPI chip select
D2  / PF1 : ESP handshake
D4  / PF2 : ESP data-ready
D7  / PF3 : ESP EN/reset
```

ESP32C5 侧连接：

| Signal | H7S78-DK side | ESP32C5 side | Direction |
| --- | --- | --- | --- |
| SPI SCLK | SPI4 AF pin, currently PE12..PE14 group | GPIO6 | H7 -> ESP |
| SPI MOSI | SPI4 AF pin, currently PE12..PE14 group | GPIO7 | H7 -> ESP |
| SPI MISO | SPI4 AF pin, currently PE12..PE14 group | GPIO2 | ESP -> H7 |
| SPI CS | PF8 / Arduino D10 | GPIO10 | H7 -> ESP |
| HS | PF1 / Arduino D2 | GPIO3 | ESP -> H7 |
| DR | PF2 / Arduino D4 | GPIO4 | ESP -> H7 |
| EN/reset | PF3 / Arduino D7 | EN | H7 -> ESP |
| GND | GND | GND | common ground |
| 3V3 | 3V3 | 3V3 | power |

注意：当前 `defconfig` 中仍有：

```text
CONFIG_WL_ESP_HOSTED_NG_HANDSHAKE_PIN=2
CONFIG_WL_ESP_HOSTED_NG_DATA_READY_PIN=4
CONFIG_WL_ESP_HOSTED_NG_RESET_PIN=7
```

这些值目前没有真正驱动板级 GPIO 选择，真正生效的是 `stm32_spi.c` 中写死的 PF1/PF2/PF3。后续需要把这些配置项整理成真实的 board binding，避免配置和代码注释不一致。

### SPI 协议要点

来自 ESP-Hosted NG SPI 协议：

- SPI 是 full-duplex，host 每次 transaction 同时发送和接收 1600 byte 级别的 buffer。
- ESP32C5 是 SPI slave，不能主动向 H7 发送时钟；H7 必须主动发起 transaction 才能把 ESP response clock 出来。
- Host 不应在 HS 低时发起 transaction。
- DR 高表示 ESP 有有效 packet 要给 host。
- 如果 H7 有命令要发，即使 DR 低，也需要在 HS 高时发起 SPI transaction。
- 如果命令发出后 response 不是同一笔 transaction 返回，host 需要继续在 HS/DR 条件允许时轮询 SPI，把 response 读回来。

## H7S78-DK NuttX 构建

H7 侧构建脚本：

```text
FeatherCore_ESP/tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng.sh
```

构建命令：

```sh
cd /home/uan/Feather-develop-WIFI
./FeatherCore_ESP/tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng.sh -j8
```

输出：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin
```

烧录地址：

```text
stm32h7s78-dk-nxboot.bin              -> internal Flash 0x08000000
stm32h7s78-dk-nsh-esp_hosted_ng.bin   -> XSPI2 NOR 0x70000000
```

`stm32h7s78-dk-nsh-esp_hosted_ng.bin` 是 `[NXboot header][NuttX app raw binary]`，应用 vector table 通常在：

```text
0x70000000 + 0x400 = 0x70000400
```

## H7S78-DK defconfig

配置目录：

```text
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/configs/nsh-esp_hosted_ng
```

关键配置：

```text
CONFIG_NET=y
CONFIG_NET_NETLINK=y
CONFIG_NETLINK_GENERIC=y
CONFIG_NETLINK_ROUTE=y
CONFIG_NET_PKT=y
CONFIG_NETDEV_IFINDEX=y
CONFIG_NETDEV_WIRELESS_IOCTL=y

CONFIG_WIRELESS=y
CONFIG_DRIVERS_WIRELESS=y
CONFIG_DRIVERS_IEEE80211=y
CONFIG_WIRELESS_IEEE80211_CFG80211_LINUX=y
CONFIG_WIRELESS_ESP_HOSTED_NG=y
CONFIG_WIRELESS_ESP_HOSTED_NG_SPI=y

CONFIG_WL_ESP_HOSTED_NG=y
CONFIG_WL_ESP_HOSTED_NG_LINUX_CFG80211=y
CONFIG_WL_ESP_HOSTED_NG_SPI_DEV=4
CONFIG_WL_ESP_HOSTED_NG_SPI_FREQUENCY=10000000
CONFIG_WL_ESP_HOSTED_NG_SPI_MODE=2
CONFIG_WL_ESP_HOSTED_NG_IFNAME="wlan0"

CONFIG_WIRELESS_WIFI_PORTS=y
CONFIG_WIRELESS_WIFI_LIBNL=y
CONFIG_WIRELESS_WIFI_WPA_SUPPLICANT=y
CONFIG_SPI=y
CONFIG_SPI_EXCHANGE=y
```

## wpa_supplicant 配置

当前 app 镜像通过 ROMFS 内置：

```text
/etc/wpa_supplicant.conf
```

源文件：

```text
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/src/etc/wpa_supplicant.conf
```

示例内容：

```conf
ap_scan=1

network={
    ssid="YOUR_SSID"
    psk="YOUR_PASSWORD"
    key_mgmt=WPA-PSK
}
```

运行：

```sh
wpa_supplicant -Dnl80211 -iwlan0 -c/etc/wpa_supplicant.conf -dd
```

## 代码移植内容

### 1. STM32H7RS SPI4 master

新增/补充：

```text
FeatherCore_ESP/nuttx/arch/arm/src/stm32h7rs/stm32_spi.c
FeatherCore_ESP/nuttx/arch/arm/src/stm32h7rs/stm32_spi.h
FeatherCore_ESP/nuttx/arch/arm/src/stm32h7rs/Kconfig
```

主要内容：

- 增加 `CONFIG_STM32H7RS_SPI4`。
- 实现 `stm32_spibus_initialize(4)`。
- 实现 `SPI_EXCHANGE()` 所需 full-duplex transfer。
- 配置 SPI4 clock、GPIO AF5、mode、bits、frequency。

### 2. STM32H7S78-DK board SPI/GPIO binding

文件：

```text
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/src/stm32_spi.c
```

主要内容：

- 配置 SPI4 CS：PF8。
- 配置 ESP-Hosted NG 辅助 GPIO：
  - PF1 handshake input。
  - PF2 data-ready input。
  - PF3 EN/reset output。
- 通过 `stm32_gpiosetevent()` 给 HS/DR 添加 rising-edge interrupt。
- 提供 ESP-Hosted NG transport 需要的 board hooks：

```c
int esp_hosted_ng_board_initialize(void);
void esp_hosted_ng_board_reset(bool reset);
bool esp_hosted_ng_board_handshake_ready(void);
bool esp_hosted_ng_board_data_ready(void);
FAR struct spi_dev_s *esp_hosted_ng_spibus_initialize(int bus);
```

### 3. ESP-Hosted NG NuttX transport

文件：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_transport.c
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_transport.h
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_proto.h
```

主要内容：

- 初始化 SPI bus、mode、bits、frequency。
- 等待 HS 高电平后执行 `SPI_EXCHANGE()`。
- 维护 1600 byte TX/RX buffer。
- 通过 weak board hooks 和具体板级解耦。

### 4. Linux-style ESP-Hosted NG host path

上游 host driver 移植目录：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/upstream_host
```

来源：

```text
third/esp-hosted/esp_hosted_ng/host
```

核心文件：

```text
main.c
esp_cmd.c
esp_cfg80211.c
esp_utils.c
esp_stats.c
esp_log.c
include/*.h
```

NuttX 适配文件：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_linux_if.c
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/upstream_host/include/esp_kernel_port.h
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_bt_stub.c
```

主要内容：

- 将 ESP-Hosted NG 上游 Linux driver 接到 NuttX 已移植的 cfg80211/nl80211。
- 通过 `esp_linux_if_write()` 发送 command/data skb。
- 通过 `esp_linux_if_read()` 从 SPI RX queue 取包。
- 通过 `esp_hosted_ng_board_spi_ready()` 把 HS/DR interrupt 转成 ESP driver RX work。
- 在同步 command wait 期间，如果 `adapter->cur_cmd != NULL`，允许主动轮询 SPI，避免 command response 因 DR 边沿丢失或 DR 低电平而一直读不到。

### 5. wpa_supplicant / libnl

目录：

```text
FeatherCore_ESP/apps/wireless/wifi
```

来源：

```text
third/wpa_supplicant-2.11
third/hostapd-2.11
third/libnl-3.2.25
```

主要内容：

- 移植 `libnl-3.2.25` 必需源文件。
- 移植 `wpa_supplicant-2.11`、`wpa_cli`、`wpa_passphrase`。
- 保留 hostapd 源和构建清单，但当前 H7S78-DK bring-up 重点是 STA path。
- 公共 NuttX 兼容头位于：

```text
FeatherCore_ESP/apps/wireless/wifi/common/include
```

## 调试过程记录

### 阶段 1：SPI bus hook 缺失

现象：

```text
esp_hosted_ng: SPI bus 4 unavailable; board SPI hook missing
esp_hosted_ng: SPI transport not ready yet: -19
```

原因：

- 只有通用 transport，没有 STM32H7S78-DK 的 `esp_hosted_ng_spibus_initialize()`。
- H7RS 普通 SPI4 master 尚未接入。

处理：

- 增加 STM32H7RS SPI4 master。
- 在 board 层返回 `stm32_spibus_initialize(4)`。

### 阶段 2：早期 demo path 不是目标链路

现象：

```text
esp32_wifi_demo connect SSID PASSWORD
```

可以下发简单 connect 请求，但这是早期 Wireless Extensions / demo path，不是目标的 Linux nl80211 path。

处理：

- 切换目标到：

```text
wpa_supplicant -> nl80211 -> cfg80211 -> esp_hosted_ng
```

- `defconfig` 启用 `CONFIG_WL_ESP_HOSTED_NG_LINUX_CFG80211=y`。
- app 镜像中内置 `wpa_supplicant`。

### 阶段 3：缺少 wpa_supplicant.conf

现象：

```text
Failed to open config file '/etc/wpa_supplicant.conf'
Failed to read or parse configuration '/etc/wpa_supplicant.conf'.
```

处理：

- 将 `etc/wpa_supplicant.conf` 加入 board ROMFS：

```text
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/src/CMakeLists.txt
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk/src/Makefile
```

### 阶段 4：NuttX netlink socket 支持不完整

现象：

```text
netlink: Failed to open netlink socket: Protocol not supported
Failed to initialize driver 'nl80211'
```

处理：

- 启用 `CONFIG_NET_NETLINK`、`CONFIG_NETLINK_GENERIC`、`CONFIG_NETLINK_ROUTE`。
- 使 `nl80211` generic netlink family 能在 NuttX 中注册。

### 阶段 5：wlan0 尚未注册

现象：

```text
ifconfig
```

无 `wlan0`，wpa_supplicant 报：

```text
Could not read interface wlan0 flags
nl80211: finish_drv_init failed for wlan0
```

处理：

- 在 ESP boot-up event 后执行 `esp_add_card()`。
- 通过 `esp_cfg80211_add_iface()` 创建 `wlan0`。
- 修复 interface 初始化时序，避免 cfg80211 发 NEW_INTERFACE 期间重入固件 command path。

### 阶段 6：启动期间 panic

现象：

```text
Assertion failed panic ... task: AppBringUp
```

高概率触发点：

- `esp_cfg80211_add_iface()` 注册过程中 cfg80211/nl80211 查询 TX power。
- 此时 ESP init 尚未完成，`get_tx_power` 重入固件 command path，和当前初始化 command 竞争。

处理：

- `ESP_INIT_DONE` 之前，`esp_cfg80211_get_tx_power()` 只返回本地缓存默认值。
- `ESP_INIT_DONE` 之后，再通过 `cmd_get_tx_power()` 从固件同步真实 TX power。

说明：

- 这不是永久绕过 TX power。
- 这是避免 cfg80211 interface registration 期间重入 command path。
- 真实 TX power 仍应在初始化完成后获取。

### 阶段 7：command response 串包或迟到

现象：

```text
Command response not expected=1
```

原因：

- command timeout 后 late response 可能到达。
- 如果不校验 response cmd code，后续 command 可能误消费旧 response。

处理：

- 在 `process_cmd_resp()` 中校验：

```text
response header cmd_code == adapter->cur_cmd->cmd_code
```

- mismatch 时丢弃并打印：

```text
Command response mismatch expected=X got=Y
```

### 阶段 8：regdomain/channel 被禁用导致 scan trigger 失败

现象：

```text
nl80211: Scan trigger failed: ret=-22
```

并且 wpa_supplicant 打印可用频点时大量 channel 处于 disabled。

处理：

- 在 ESP wiphy 注册前应用 custom regulatory domain：

```text
2.4GHz: 2412..2472
5GHz  : 5180..5240, 5260..5320, 5500..5720, 5745..5885
```

相关代码：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cfg80211.c
```

### 阶段 9：SPI command response timeout

现象：

H7 侧：

```text
wait_and_decode_cmd_resp: Command[0xF] timed out
cmd_get_tx_power: wait_and_decode_cmd_resp(...) failure
wait_and_decode_cmd_resp: Command[0x4] timed out
cmd_scan_request: wait_and_decode_cmd_resp(...) failure
```

ESP32C5 侧同时能看到：

```text
FW_MAIN: Get Tx power command
FW_MAIN: INIT Interface command
```

分析：

- ESP 固件已经收到并处理 command。
- H7 侧没有稳定读回 response。
- SPI slave 不能主动 clock response，必须由 H7 再发起 transaction。
- 原 NuttX read path 在 `ESP_INIT_DONE` 后只在 DR 高时读 SPI；如果 response 没拉高 DR、DR 边沿丢失，或 host 只靠 interrupt，就会 timeout。

处理：

- 在 `esp_linux_if_read()` 中加入 `command_pending`：

```text
adapter->cur_cmd != NULL
```

- 同步 command 等待期间，只要 HS ready，就允许 H7 主动发空帧轮询 SPI，把真实 response clock 出来。

相关代码：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_linux_if.c
```

## 期望启动日志

H7 侧理想启动片段：

```text
esp_hosted_ng: SPI transport ready bus=4 freq=10000000 mode=2 hs=0 dr=0
esp_hosted_ng: chipset=ESP32C5 id=17 over SPI
esp_hosted_ng: SPI clock adjusted to 30000000 Hz
process_esp_bootup_event: Received ESP boot-up event
process_event_esp_bootup: Boot-up Event tag: ...
check_esp_version: ESP-Hosted Version: NG-...
esp_cfg80211_add_iface: registered wlan0 mac=... iftype=2
print_capabilities: Capabilities: ...
```

ESP32C5 侧理想启动片段：

```text
FW_SPI: Using SPI interface
FW_SPI: Using SPI MODE 2
FW_SPI: SPI Ctrl:1 mode: 2, GPIOs: MOSI: 7, MISO: 2, CS: 10, CLK: 6 HS: 3 DR: 4
FW_MAIN: Initial set up done
FW_MAIN: INIT Interface command
FW_CMD: Setting STA mode
FW_CMD: station started
```

## 验证命令

H7 NSH：

```sh
ifconfig
cat /etc/wpa_supplicant.conf
wpa_supplicant -Dnl80211 -iwlan0 -c/etc/wpa_supplicant.conf -dd
```

连接成功后继续验证：

```sh
ifconfig wlan0
ping <router-ip>
ping 8.8.8.8
```

如果 DHCP 已启用且接口关联完成，后续可补：

```sh
ifup wlan0
```

或按当前 NuttX 网络初始化方式手动配置 IPv4。

## 未完成事项和计划

### P0：继续验证 EAPOL 2/4 host -> ESP -> AP

2026-06-08 最新日志显示，扫描、认证、关联和 AP -> STA 的 EAPOL 1/4 都已经打通：

```text
wlan0: Associated with 68:dd:b7:9b:60:a5
EAPOL RX esp->host packet_type=EAPOL len=113
EAPOL TX host->esp len=135
```

ESP32C5 固件侧也能看到 AP 重复发 EAPOL 1/4：

```text
STA EAPOL callback len=113
STA EAPOL -> host len=113
```

但是固件侧没有看到 host 发回来的 EAPOL 2/4，也没有进入 EAPOL 3/4、PTK/GTK key install，最后 AP 以 reason 15 断开。wpa_supplicant 打印 `WRONG_KEY` 是 4-way handshake 超时后的通用判断，不一定代表 PSK 真错。

约束调整：ESP32C5 firmware 不做功能修改，只添加必要调试日志。host -> ESP 的
EAPOL 仍走原固件已经支持的 `PACKET_TYPE_DATA` 数据路径，H7 host driver 负责确保
payload 带 Ethernet header。

已补充：

- H7 host driver 发送 ethertype `0x888e` 时打印 `packet_type=DATA`，并继续经原 DATA path 下发。
- ESP32C5 firmware 仅增加日志：DATA payload 中检测到 EtherType `0x888e` 时打印 `Host EAPOL DATA -> STA ...`。
- ESP32C5 firmware 仅增加日志：AP -> host EAPOL callback、disconnect BSSID/cache、key op 参数。

关键日志判据：

```text
# H7
EAPOL TX host->esp packet_type=DATA ... len=135

# ESP32C5
Host EAPOL DATA -> STA len=135 connected=0 assoc=1
STA EAPOL callback len=169

# H7
add_key idx=0 pairwise=1
pairwise key installed
CTRL-EVENT-CONNECTED
```

2026-06-08 曾经临时验证过 `PACKET_TYPE_EAPOL` firmware 分支，确认 EAPOL 2/4 和
4/4 能从 H7 送到 ESP32C5。但该方案需要修改 ESP32C5 firmware 功能，已撤回。
当前保留的正式方案是 `PACKET_TYPE_DATA` + 调试日志：

```text
# H7
EAPOL TX host->esp packet_type=DATA ... len=135
EAPOL TX host->esp packet_type=DATA ... len=113

# ESP32C5
Host EAPOL DATA -> STA len=135 connected=0 assoc=1
STA EAPOL callback len=169
Host EAPOL DATA -> STA len=113 connected=0 assoc=1
```

新的阻塞点移动到 key install 后半段：

```text
# H7
cmd_add_key: add_key idx=0 pairwise=1 algo=3 len=16 seq_len=6 mac=<AP-BSSID>
cmd_add_key: pairwise key installed
cmd_add_key: add_key idx=1 pairwise=0 algo=3 len=16 seq_len=6 mac=ff:ff:ff:ff:ff:ff

# ESP32C5
FW_MAIN: Add key request
FW_CMD: Setting PTK algo=3 index=0
FW_MAIN: Add key request
FW_CMD: Setting GTK [1]
Guru Meditation Error: Store access fault
```

`addr2line` 将 ESP32C5 crash 定位到 `cmd.c:set_key_internal()` 调用
`esp_wifi_set_sta_key_internal()` 安装 GTK 后进入 IDF WiFi 内部
`ieee80211_set_sta_gtk_index()`。同时日志显示 ESP32C5 在 GTK 到达前已经收到
`WIFI_EVENT_STA_DISCONNECTED reason=204` 并执行过 AP BSSID cleanup，H7 侧 group key
也可能因为广播地址不复制而把全 0 MAC 送进 firmware。

已补充修复和约束：

- H7 `struct esp_wifi_device` 增加 `bssid`，在 connect/assoc event 中保存 AP BSSID。
- H7 `cmd_add_key()` 对 STA group key 优先使用保存的 AP BSSID，不再使用广播地址兜底。
- H7 对 reason 204 仍保持延迟处理，真正处理断开时才清理 `bssid`。
- ESP32C5 firmware 不再修改 disconnect cleanup、key install 参数、packet type 分支或 ABI 结构体。
- ESP32C5 firmware 只保留日志，用来观察 key op 的 MAC、cached AP BSSID、host DATA/EAPOL 是否到达。

本轮已重新构建并烧录 ESP32C5 firmware；固件行为保持原始，仅包含日志：

```text
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter/build/network_adapter.bin
```

也已重新构建 H7 app：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin
```

下一轮验证重点：

```text
# H7 侧应看到 group key 使用 AP BSSID
add_key idx=1 pairwise=0 ... mac=<AP-BSSID>

# ESP32C5 侧观察实际收到的 key 参数
Key op iface=... algo=3 index=1 ... mac=<AP-BSSID> cached_ap=<AP-BSSID>

# 成功路径
WPA: Key negotiation completed
CTRL-EVENT-CONNECTED
```

### P0：2026-06-08 M4 后未安装 PTK 的异常分析

最新 H7/ESP32C5 对照日志将阻塞点继续向后推进：EAPOL 2/4 和 4/4 都已经能从 H7
送到 ESP32C5，ESP32C5 侧明确打印：

```text
Host EAPOL DATA -> STA len=135 connected=0 assoc=1
STA EAPOL callback len=169
Host EAPOL DATA -> STA len=113 connected=0 assoc=1
```

这说明当前问题不再是 host -> ESP 的 EAPOL DATA path 完全不通。新的第一异常点是：

```text
wpa_supplicant 收到 M3
  -> H7 发送 M4
  -> ESP32C5 已收到 M4
  -> H7 host 侧没有继续完成 PTK/GTK key install
  -> ESP32C5 约 10 秒后以 reason 204 断开
  -> H7 后续出现 PC=0/LR=0 崩溃
```

当前判断：

- `reason=204` 是 ESP/路由器侧等待 4-way handshake/key install 完成超时后的结果，不是第一根因。
- H7 崩溃时 `PC=0`、`LR=0`，更像空函数指针、任务上下文损坏、内存破坏或断开事件竞争后的副作用。
- 线程栈确实是风险项，而且这份日志里的 `nsh_main STACKSIZE 2000` 是十进制打印，约 2 KiB，
  不是 `0x2000`。`CONFIG_SYSTEM_NSH_STACKSIZE` 只影响作为 builtin app 启动的 `nsh`；当前
  defconfig 使用 `CONFIG_INIT_ENTRYPOINT="nsh_main"`，启动 shell 线程实际受
  `CONFIG_INIT_STACKSIZE` 控制。
- 已将 `CONFIG_INIT_STACKSIZE=16384` 固化到 `nsh-esp_hosted_ng` defconfig，并重新构建。这样
  作为 init 线程运行的 `nsh_main` 也会拿到 16 KiB 栈，不再是旧日志里的 2 KiB。
- `wpa_supplicant` 自身也存在栈偏紧风险。旧日志中任务表显示 `wpa_supplicant ... STACKSIZE
  8088`，对应约 8 KiB；而当前端口把大量 RSN/EAPOL、libnl、nl80211 driver glue 都放在
  `wpa_supplicant` 主任务里执行。已新增 `CONFIG_WIRELESS_WIFI_WPA_SUPPLICANT_STACKSIZE`
  Kconfig，并在 `nsh-esp_hosted_ng` defconfig 固化为 16 KiB。生成的 builtin registry 已确认：
  `{ "wpa_supplicant", 90, 16384, wpa_supplicant_main }`。
- 栈太小可以解释“NSH 回车后死机/崩溃”的一部分现象，但不能单独解释 EAPOL M4 后没有继续
  PTK/GTK key install 以及 reason 204。因此后续仍需同时验证 skb 生命周期、RX queue 并发和
  M4 返回后的 wpa_supplicant 状态机日志。
- 静态代码路径显示，wpa_supplicant 的标准 RSN 流程是在收到 M3 后先发送 M4，然后再
  `wpa_supplicant_install_ptk()`。因此如果 M4 的 `send()` 没有正常返回，或返回后状态机
  被破坏，就不会看到后续 `set_key`/`add_key`。

当前缺失的成功日志链为：

```text
l2_packet: EAPOL RAW TX ... ret=...
NuttX: wpa_eapol_key_send ret=...
NuttX: WPA msg 4/4 sent, continue to PTK install ...
NuttX: supplicant set_key ...
NuttX: nl80211 set_key ...
nuttx: add_key ...
nuttx: pairwise key installed
```

为定位这一断点，H7 侧已增加只读诊断日志，不改变 ESP32C5 firmware 行为，也不改变
wpa_supplicant 协议状态机：

- `apps/wireless/wifi/wpa_supplicant-2.11/src/l2_packet/l2_packet_linux.c`
  - 打印 EAPOL raw `send()` 进入和返回。
- `apps/wireless/wifi/wpa_supplicant-2.11/src/rsn_supp/wpa.c`
  - 打印 `wpa_eapol_key_send()` 返回值。
  - 在两个 `wpa_supplicant_send_4_of_4()` 分支都打印 M4 发送返回后是否继续进入 PTK install。
  - 在两个 PTK install 分支都打印返回值。
- `nuttx/wireless/ieee80211/netdevice_compat.c`
  - 打印 EAPOL `ndo_start_xmit()` 返回值。
- `nuttx/drivers/wireless/esp_hosted_ng/upstream_host/main.c`
  - 打印 EAPOL `esp_send_packet()` 返回值。

已重新构建 H7 app：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin
size: 1820548 bytes
```

当前构建确认：

```text
CONFIG_INIT_STACKSIZE=16384
CONFIG_SYSTEM_NSH_STACKSIZE=16384
CONFIG_WIRELESS_WIFI_WPA_SUPPLICANT_STACKSIZE=16384
```

#### 2026-06-08 进一步定位：host TX skb 生命周期异常

对 `M4 TX -> ESP received M4 -> H7 没有继续 key install -> NSH 回车后 PC=0/LR=0`
这条链路继续反查后，发现一个明确的 host 侧内存生命周期问题：

```text
netdevice_compat
  -> ndo_start_xmit(skb)
    -> process_tx_packet(skb)
      -> esp_send_packet(adapter, skb)
        -> esp_linux_if_write(adapter, skb)
          -> SPI exchange
          -> dev_kfree_skb(skb)
      -> process_tx_packet() 返回后继续读取 skb->len 更新 tx_bytes
```

当前 NuttX SPI `if_ops->write()` 的语义是“消费并释放 skb”。因此
`process_tx_packet()` 在 `esp_send_packet()` 返回后继续访问 `skb->len` 是
use-after-free。这个问题在普通数据量少时可能只表现为偶现异常，但 4-way handshake 阶段
M2/M4、cmd response、disconnect event、NSH 输入会密集交错，释放后访问更容易破坏后续控制流。
旧日志里 `PC=0/LR=0`、崩溃栈落在 `nsh_main/read/netinit_bringup/dhcpc_addhostname` 附近，
也符合“前面网络路径已经破坏了内存/上下文，后面在无关入口点爆炸”的形态。

已修复：

```text
nuttx/drivers/wireless/esp_hosted_ng/upstream_host/main.c
```

修复方式是在发送前缓存原始 payload 长度，发送返回后只使用缓存值更新统计，不再访问已被
`if_ops->write()` 消费的 `skb`。

同类调用点审计：

- `esp_cmd_work()`：发送 command skb 后立即将 `cmd_node->cmd_skb = NULL`，符合当前 write
  消费 skb 的语义。
- `esp_stats.c` raw throughput：发送后只使用常量统计长度，没有继续访问 `tx_skb`。
- 数据帧 TX path：已修复 `tx_bytes += skb->len` 的释放后读取。

#### 2026-06-08 继续定位：NETDEV_TX_BUSY 不能当作成功

继续审计 TX path 后，又发现一个和 Linux/NuttX 语义差异有关的风险点：

```text
NuttX packet socket/raw EAPOL TX
  -> ieee80211_linux_transmit_netpkt()
    -> ndo_start_xmit(skb)
      -> esp_hard_start_xmit()
        -> process_tx_packet()
          -> netif_queue_stopped() 或 host_sleep
          -> return NETDEV_TX_BUSY
```

在 Linux 中，`ndo_start_xmit()` 返回 `NETDEV_TX_BUSY` 的语义是“driver 没有接管 skb”，上层
qdisc/core 仍负责后续重试或释放。但当前 NuttX compat 层是直接调用 `ndo_start_xmit()`，
没有 Linux qdisc 帮忙，因此如果把 `NETDEV_TX_BUSY` 当作成功返回，就会造成两个问题：

- 临时分配的 skb 没有被释放，形成泄漏。
- 更严重的是，对 PF_PACKET/EAPOL 发送方来说，这个帧既没有发出去，也没有明确报错，4-way
  handshake 可能直接表现为 M4 或后续 key install 超时。

已修复：

```text
nuttx/wireless/ieee80211/netdevice_compat.c
```

修复策略：

- `ieee80211_linux_transmit_netpkt()` 收到 `NETDEV_TX_BUSY` 时释放临时 skb 和 TX netpkt，并返回
  `-EAGAIN`。
- `ieee80211_linux_dev_queue_xmit()` 收到 `NETDEV_TX_BUSY` 时释放 skb，并返回 `-EAGAIN`。
- 只有 `NETDEV_TX_OK/0` 被转换成 NuttX `OK`，其它非成功值不再被静默当成成功。

#### 2026-06-08 继续定位：sk_buff_head 队列缺少锁保护

继续审计 RX/command response path 时发现，compat 层的 `struct sk_buff_head` 虽然带有
`lock` 字段，但 `skb_queue_tail()`、`skb_dequeue()`、`skb_queue_head()` 实现没有使用这把锁。
这会影响 ESP-Hosted NG 的几个关键路径：

```text
SPI exchange 收到 response/event
  -> esp_linux_if_queue_rx()
    -> skb_queue_tail(&g_esp_rxq, rxskb)

command wait loop 或 RX work
  -> esp_linux_if_read()
    -> skb_dequeue(&g_esp_rxq)
```

`g_esp_rxq` 可能同时被同步 command wait loop、HS/DR 触发的 RX work、主动空帧轮询访问。无锁
链表入队/出队在高频 M3/M4、command response、disconnect event 交错时可能导致：

- response 已经从 SPI 读到，但队列链表被破坏或帧被丢，H7 侧表现为 command timeout。
- 队列链表损坏后，后续在无关位置触发 `PC=0/LR=0` 或 NSH 回车后卡死。

已修复：

```text
nuttx/wireless/ieee80211/include/linux/cfg80211_compat.h
```

修复策略：

- `__skb_queue_head_init()` 初始化 `sk_buff_head.lock`。
- `skb_queue_tail()`、`skb_queue_head()`、`skb_dequeue()` 使用队列自带 lock。
- 保留 `__skb_queue_tail()`、`__skb_dequeue()` 作为无锁版本，供已经明确在外层锁保护下的路径使用。
- `skb_queue_purge()` 改为 dequeue 后释放 skb，匹配 Linux 语义，避免 shutdown/error path 泄漏。

下一轮板测应优先确认：

```text
EAPOL TX esp_send_packet ret=0 len=113
netdevice_compat: EAPOL tx ndo_start_xmit ret=0 ...
l2_packet: EAPOL RAW TX ... ret=113
NuttX: wpa_eapol_key_send ret=0 ...
NuttX: WPA msg 4/4 sent, continue to PTK install ...
NuttX: WPA PTK install result=...
```

如果修复后不再出现 `PC=0/LR=0`，但仍然没有 `PTK install`，继续沿
`wpa_eapol_key_send()` 返回值和 `wpa_supplicant_process_3_of_4()` 两个 M4 分支定位。
如果已经进入 `PTK install` 但失败，则转向 `wpa_supplicant_set_key()`、
`driver_nl80211_set_key()`、`nl80211_set_key()`、`esp_cfg80211_add_key()`、
`cmd_add_key()` 的 command response。

下一轮判读规则：

```text
只有 "EAPOL RAW TX enter"，没有 "... ret=..."
  -> 卡在 PF_PACKET send()/NuttX lowerhalf TX/SPI exchange 返回路径。

有 "... ret=..."，没有 "WPA msg 4/4 sent"
  -> wpa_eapol_key_send()/ether_send 返回链或其后续状态机损坏。

有 "WPA msg 4/4 sent"，但没有 "supplicant set_key"/"nl80211 set_key"
  -> wpa_supplicant_install_ptk() 前后的条件或状态损坏。

有 "supplicant set_key"/"nl80211 set_key"，但没有 "add_key"/"pairwise key installed"
  -> nl80211/cfg80211 到 esp_hosted_ng cmd_add_key 路径失败。

有 "add_key"，但无固件 response 或出现 command timeout
  -> SPI RX/HS/DR/command response 方向仍有漏读或时序问题。
```

静态风险点：

- NuttX `netdev_lowerhalf_s.transmit()` 语义是非阻塞、lowerhalf 接管 `netpkt` 后稍后释放；
  当前 compat 层为了适配 Linux `ndo_start_xmit()`，在 `transmit()` 内同步调用
  `ndo_start_xmit()` 并立即 `netpkt_free()`。这条路径 M2 能跑通，但在 M4、command
  response、disconnect event 密集交错时仍可能暴露同步/重入问题。
- SPI path 是全双工 exchange。当前 `esp_linux_if_exchange()` 会尝试把同一次 exchange
  读回来的 RX frame 入队，但一次只处理一个 frame，后续帧依赖 HS/DR 中断或主动空帧轮询。
  如果 HS/DR 边沿漏触发，command response 或 event 可能延迟，导致 host 等待超时。
- 旧日志中的 `PC=0/LR=0` 不是典型栈溢出形态，更像已经发生了控制流破坏。栈放大应保留，
  但后续仍需根据新日志确认是否存在 TX/RX 回调重入或 skb/netpkt 生命周期问题。

#### 2026-06-08 最新板测：key install 链路已跑通，但 M4 TX 返回仍有时序窗口

最新 H7S7/ESP32C5 对照日志出现了一个正向突破：至少有一轮完整跑通 4-way handshake，并且
ESP32C5 firmware 收到了 PTK/GTK key install：

```text
H7:
NuttX: WPA msg 4/4 sent, continue to PTK install key_info=0x13ca
NuttX: supplicant set_key alg=3 key_idx=0 set_tx=1 ...
nuttx: cmd_add_key: pairwise key installed
NuttX: WPA PTK install result=0
NuttX: supplicant set_key alg=3 key_idx=1 set_tx=0 ...
wlan0: WPA: Key negotiation completed with 68:dd:b7:9b:60:a5 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 68:dd:b7:9b:60:a5 completed

ESP32C5:
FW_MAIN: Add key request
FW_CMD: Setting PTK algo=3 index=0
FW_MAIN: Add key request
FW_CMD: Setting GTK [1]
wifi:connected with Wakaka
FW_CMD: Wifi Station Connected event!!
```

这说明：

- `wpa_supplicant -> nl80211 -> cfg80211 -> esp_hosted_ng cmd_add_key -> ESP32C5 firmware`
  这条 key install 链路本身已经可用。
- 之前的“完全没有进入 PTK install”不是协议栈必然缺口，而是时序/阻塞/并发造成的偶发断点。
- 栈扩大后本轮日志没有再出现旧的 `nsh_main STACKSIZE 2000` 崩溃栈，说明栈问题至少已经被压住。

但同一批日志里仍有失败轮次。失败轮的关键顺序是：

```text
H7:
EAPOL TX host->esp ... msg=M4 len=113
process_disconnect_event: Disconnect event ... [reason:204]
firmware reported reason 204 before key install; hold userspace disconnect ...
EAPOL TX esp_send_packet ret=0 len=113
WPA msg 4/4 sent, continue to PTK install ...
supplicant set_key ...
cmd_add_key: drop add_key ... because firmware already reported disconnect reason=204

ESP32C5:
Host EAPOL DATA -> STA len=113 connected=0 assoc=1
约 9 秒后:
STA disconnected [204]
```

这个顺序说明第一根因更精确地落在：

```text
wpa_supplicant 调用 PF_PACKET send(M4)
  -> NuttX netdevice compat
    -> esp_hosted_ng process_tx_packet()
      -> esp_send_packet()
        -> SPI/IF exchange
          -> 返回过慢
  -> wpa_supplicant 才继续执行 PTK/GTK add_key
```

也就是说，失败轮并不是 key install 代码不存在，而是 M4 的同步发送调用返回太晚；等
`wpa_supplicant` 有机会下发 PTK/GTK 时，ESP32C5 firmware 已经上报 204。

最新对照日志进一步证明，M4 返回过慢会把 key install 推到 ESP32C5 firmware 的
handshake timeout 窗口之后。失败轮中 H7 侧 M4 发送调用最终返回前后，ESP32C5 侧已经打印：

```text
Host EAPOL DATA -> STA len=113 connected=0 assoc=1
STA disconnected [204]
cleanup_ap_bssid
```

早期调试中曾尝试继续发送 late `add_key`，ESP32C5 firmware 在 GTK 安装阶段崩溃：

```text
Guru Meditation Error: Core 0 panic'ed (Store access fault)
```

用当前固件 ELF 反查：

```sh
riscv32-esp-elf-addr2line -pfiaC \
  -e third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter/build/network_adapter.elf \
  0x42077b22 0x420781bc 0x4200e44a
```

定位为：

```text
0x42077b22: ieee80211_set_sta_gtk_index at ??:?
0x420781bc: ppInstallKey at ??:?
0x4200e44a: set_key_internal at .../network_adapter/main/cmd.c:1878
```

因此 host 侧策略分为两层：

- reason 204 早到时仍暂缓上报给 userspace，避免 wpa_supplicant 立即走断开清理。
- `disconnect_event_pending` 只表示“有一个 deferred disconnect event”，不能被当作接口已经
  断开的硬状态去本地拒绝 `cmd_add_key()`。Linux 路径中 key install 应继续下发，真正结果由
  driver/firmware command response 决定。
- 真正根因仍是 M4 TX 同步调用返回过慢；修复方向是让 M4 `send()` 在 firmware 204 超时前返回，
  让 wpa_supplicant 能按正常顺序安装 PTK/GTK。

同时新增 H7 host 侧耗时诊断：

- `esp_hosted_ng: if exchange tx_eapol=... lock_ms=... transport_ms=... enqueue_ms=... unlock_ms=... wake_ms=... total_ms=...`
- `esp_hosted_ng: transport exchange slow ... lock_ms=... hs_ms=... spi_ms=... total_ms=...`

本轮失败日志已经看到：

```text
if exchange tx_eapol=1 txlen=128 lock_ms=0 total_ms=9560 ret=0 hs=0 dr=0
```

这说明 M4 并没有卡在外层 `g_esp_if_lock` 上，而是在 `esp_linux_if_exchange()` 内部或紧随
transport 后的 RX queue 处理上耗时约 9.56 秒。下一轮日志需要重点看新增的分段字段：

```text
if exchange tx_eapol=1 transport_ms=... enqueue_ms=... unlock_ms=... wake_ms=... total_ms=...
transport exchange slow lock_ms=... hs_ms=... spi_ms=...
```

判读：

- `lock_ms` 大：卡在 H7 host 侧互斥锁，说明 RX work/command/data TX 抢 SPI exchange。
- `transport_ms` 大：卡在 `esp_hosted_ng_transport_exchange()`，继续看 transport slow 的
  `hs_ms`/`spi_ms`。
- `enqueue_ms` 大：SPI transport 已返回，但 RX skb 分配、复制或入队异常变慢。
- `wake_ms` 大：RX work 投递/触发阻塞，需要继续查 `esp_process_new_packet_intr()` 和
  NuttX workqueue 适配层。
- `hs_ms` 大：卡在 ESP SPI handshake ready，优先查 HS GPIO/中断/ESP slave ready 时序。
- `spi_ms` 大：卡在 `SPI_EXCHANGE()` 本身，优先查 SPI4 DMA/CS/clock/transfer size。
- 三者都小但仍 204：转向 ESP firmware 内部 WPA timeout/hosted protocol response 时序。

#### 2026-06-08 进一步定位：M4 卡在 RX work 触发路径，不是 SPI transaction

最新分段日志确认，失败轮 M4 的 SPI transport 本身只耗时约 10 ms，真正拖住
`wpa_supplicant` 的是 transport 返回后的 RX queue/work 路径：

```text
if exchange tx_eapol=1 txlen=128 lock_ms=0 transport_ms=10 queue_ms=7610 total_ms=7620 ret=0
```

而 M2 阶段也能看到同类但较轻的现象：

```text
if exchange tx_eapol=1 txlen=148 lock_ms=0 transport_ms=0 queue_ms=780 total_ms=780 ret=0
if exchange tx_eapol=1 txlen=148 lock_ms=0 transport_ms=10 queue_ms=880 total_ms=890 ret=0
```

这说明问题不在 SPI4 一笔 transaction 或 HS 等待，而是 `esp_linux_if_exchange()` 在 TX 同步
路径中处理同一笔 exchange 读回来的 RX frame 时，顺手触发 RX work，导致 PF_PACKET
`send(M4)` 被 RX/事件处理拖住。由于 wpa_supplicant 的正常顺序是：

```text
send(M4) 返回
  -> set PTK
  -> set GTK
```

只要 `send(M4)` 被拖到 ESP32C5 firmware 的 handshake timeout 之后，后续 key install 就必然
太晚。

已修复：

- `esp_linux_if_queue_rx()` 现在只负责校验、复制并把 RX skb 放入 `g_esp_rxq`。
- `esp_linux_if_exchange()` 在释放 `g_esp_if_lock` 后才调用 `esp_process_new_packet_intr()`。
- 耗时日志从 `queue_ms` 细化为 `enqueue_ms`、`unlock_ms`、`wake_ms`，下一轮可以继续确认
  是否还被 workqueue 投递拖住。

本次修复后重新构建 H7 app：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin              size: 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin  size: 1820724 bytes
```

当前最新 H7 app 构建产物：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin              size: 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin  size: 1817188 bytes
```

### 2026-06-09：已跑通状态

当前日志和实测已经证明，以下链路完成：

```text
ESP32C5 SPI boot
  -> H7 SPI transport ready
  -> ESP boot-up event decoded
  -> wlan0 registered through cfg80211
  -> wpa_supplicant initializes nl80211
  -> scan/auth/assoc
  -> EAPOL 1/4, 2/4, 3/4, 4/4
  -> PTK/GTK install
  -> CTRL-EVENT-CONNECTED
  -> IP data path
  -> DHCP
  -> DNS
  -> ping router / baidu.com passed
```

H7 侧关键日志：

```text
WPA: Key negotiation completed with <AP-BSSID> [PTK=CCMP GTK=CCMP]
wlan0: State: GROUP_HANDSHAKE -> COMPLETED
CTRL-EVENT-CONNECTED
EAPOL authentication completed - result=SUCCESS
inet addr:192.168.0.113 DRaddr:192.168.0.1 Mask:255.255.255.0
10 packets transmitted, 10 received, 0% packet loss
```

ESP32C5 侧关键日志：

```text
FW_MAIN: Add key request
FW_CMD: Setting PTK
FW_CMD: Setting GTK
wifi:connected with ...
FW_CMD: Wifi Station Connected event!!
```

随后 `ifconfig wlan0` 能看到 `RUNNING` 和 DHCP 地址，`ping` 路由器和外部域名成功，说明普通
Ethernet/IP frame、ARP、DNS、ICMP 已能穿过 NuttX netdev、ESP-Hosted SPI transport 和
ESP32C5 Wi-Fi firmware。

### 根本原因复盘

这轮问题不是单点 bug，而是几个 Linux 语义在 NuttX 上缺失后叠加出来的。

#### 根因 1：AF_PACKET poll 语义不兼容

`wpa_supplicant` 在 nl80211 driver 初始化期间会创建多个 fd，并进入同一个 `poll()` 集合。
其中包括一个未绑定的 packet socket：

```text
AF_PACKET / SOCK_DGRAM / protocol 0
```

这个 fd 主要用于 Linux packet socket 的辅助行为，例如 TX status/error queue。Linux 允许
这种未绑定 socket 存在，也允许它参与 `poll()`。原 NuttX 实现把这类 socket 当成无设备，
`poll_setup()` 期间返回 `-ENODEV`。结果是整个 `wpa_supplicant` 的 poll setup 被提前破坏，
真正负责接收 EAPOL 的 fd 没有稳定进入等待队列。

修复后行为：

```text
pkt_pollsetup: unbound packet fd=9 ... ret=0
pkt_netpoll: EAPOL fd=10 setup=1 ...
pkt_pollsetup: EAPOL ...
pkt_input: EAPOL matched ... poll_waiters=1
pkt_recvmsg: readahead EAPOL ...
```

结论：必须在 NuttX 的 packet socket 层补齐 Linux 兼容语义，而不是在
`wpa_supplicant` 里绕过 fd 或删功能。

#### 根因 2：SO_ATTACH_FILTER 缺失

`wpa_supplicant` 的 `l2_packet_linux` 会对 EAPOL socket 设置 classic BPF filter，只接收
目标协议和接口上的帧。NuttX 原先没有 `SO_ATTACH_FILTER`，导致初始化日志出现：

```text
l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Protocol not available
```

这里不能简单忽略。虽然 EAPOL 可以靠协议号先跑起来，但完整 Linux 模式需要支持这个
socket option，否则后续会在更复杂的 packet socket 场景中继续偏离 Linux 行为。

当前处理方向是：在 NuttX socket/pkt 层接受并保存 filter 描述，让 EAPOL path 先按
Linux 风格初始化；后续可继续补全真正的 BPF 执行器。

#### 根因 3：EAPOL TX 同步路径被 RX work 拖住

早期失败时，EAPOL M4 已经从 H7 发到 ESP32C5，但 `send(M4)` 返回太晚。由于
`wpa_supplicant` 的正常顺序是：

```text
send(M4) returns
  -> install PTK
  -> install GTK
```

如果 `send(M4)` 被拖到 AP/ESP32C5 的 WPA timeout 之后，后续 key install 就会变成 late key。
失败日志中能看到：

```text
if exchange tx_eapol=1 txlen=128 lock_ms=0 transport_ms=10 queue_ms=7610 total_ms=7620 ret=0
```

这说明 SPI transaction 本身没有卡住，真正耗时发生在 transport 返回后的 RX queue/work
触发路径。修复方式是把 TX 同步路径拆开：

- `esp_linux_if_queue_rx()` 只做 RX skb 校验、复制和入队。
- `esp_linux_if_exchange()` 释放 `g_esp_if_lock` 后再触发 `esp_process_new_packet_intr()`。
- 避免 EAPOL TX 的同步返回被 RX work/事件处理拖住。

这次修复后，`wpa_supplicant` 能及时安装 PTK/GTK，4-way handshake 完成。

#### 根因 4：deferred disconnect 不能毒化 key install 状态

在 M4 返回过慢的失败轮里，ESP32C5 firmware 可能先上报 disconnect reason 204。这个事件
和 wpa_supplicant 后续 `NEW_KEY` 存在时序竞争。

host 侧需要保留的状态保护是：

- 保存 association event 中的 AP BSSID。
- STA group key 优先使用保存的 AP BSSID，不用广播地址或全 0 MAC 兜底。
- reason 204 早到时可以暂缓上报给 userspace，避免立即打断正在进行的 4-way handshake。

但 `disconnect_event_pending` 不能作为本地拒绝 `NEW_KEY` 的依据。Linux 对齐行为是：
`NEW_KEY` 继续走 driver command path，成功或失败由 firmware command response 决定；host
不能自己返回 `-ENOTCONN`，否则 wpa_supplicant 会把 host 人为制造的失败误判成 `WRONG_KEY`。
真正让连接稳定成功的修复仍然是前面的 M4 TX 返回时序修复。

#### 根因 5：AF_PACKET sendto(sockaddr_ll) 语义缺失

连接完成后，`wpa_supplicant` 会按 Linux packet socket 方式发送 WNM keep-alive 等二层帧。
原 NuttX packet socket 只覆盖了较窄的发送路径，`AF_PACKET/SOCK_RAW + sendto(sockaddr_ll)`
没有完整补齐，曾经表现为：

```text
sendto: Address family not supported by protocol
```

这会让连接后的 Linux 模式继续偏离真实行为。当前已经在 NuttX packet socket 层补齐：

- `SOCK_RAW` 允许携带 `sockaddr_ll` 指定 ifindex/protocol。
- 发送时按 Linux 语义构造/保留二层头。
- buffered 与 unbuffered packet TX path 都走同一套地址解析和 L2 header 处理逻辑。

修复后，当前日志中已经不再出现 `Address family not supported by protocol`。

#### 根因 6：nl80211 事件 socket 收到本地请求回声

当前 NuttX generic netlink 仍是轻量桥接实现，事件 socket 偶尔能收到带 `NLM_F_REQUEST`
标志的本地请求回声。典型表现是：

```text
nl80211: Drv Event 2 (NL80211_CMD_SET_WIPHY) received for wlan0
nl80211: Ignored unknown event (cmd=2)
```

`NL80211_CMD_SET_WIPHY` 是 userspace 到 kernel 的同步请求，不是 Linux nl80211 multicast
event。继续把它交给事件状态机只会制造噪声。当前处理是在 NuttX 的
`process_global_event()` 中跳过带 `NLM_F_REQUEST` 的消息，使请求/响应通道和事件通道重新
分清；真正的 scan、MLME、connect/disconnect 等 multicast event 不受影响。

同时把 PMKSA unsupported 日志中的 `NuttX hwsim` 改为 `NuttX nl80211 port`，避免在
ESP32C5 硬件链路上误导判断。

#### 构建清理：bitfield 兼容宏重复定义

ESP-Hosted NG host driver 和 cfg80211 头文件同时包含 `cfg80211_compat.h` 与 `linux/bitfield.h`
时，曾经出现：

```text
warning: "u8_get_bits" redefined
warning: "u16_get_bits" redefined
```

这不是运行时 bug，但会污染构建日志，淹没真正有意义的 warning。当前 `linux/bitfield.h`
已经改为带 `#ifndef` 保护的定义，保留 Linux 风格的 `FIELD_GET()`/`u8_get_bits()` 接口，同时
不再和 compat 兜底宏重复定义。

#### 2026-06-09 代码收口：Linux 行为对齐与 warning 清理

当前后续修复按以下原则推进：

- Linux 已有的 userspace ABI、socket 语义、nl80211/cfg80211 行为，NuttX 缺失时在
  NuttX 原生层或驱动层补齐。
- `wpa_supplicant`、libnl 这类 Linux userspace 移植代码只做必要的 NuttX 编译/ABI 适配，
  不在 userspace 假装 kernel/driver 功能已经成功。
- ESP32C5 firmware 不做功能性修改，只允许保留必要诊断日志。

本轮已经收口的点：

- 移除 `wpa_supplicant` nl80211 路径里对 PMKSA add/delete/flush unsupported 的假成功处理。
  如果 ESP driver/cfg80211 当前不支持 PMKSA offload，就应返回 Linux 等价的
  `-EOPNOTSUPP`/unsupported 结果；如果后续需要该能力，应在 driver/cfg80211 command
  path 中实现，而不是在 userspace 静默吞掉。
- 补齐 NuttX netlink socket option 语义：
  `NETLINK_ADD_MEMBERSHIP`/`NETLINK_DROP_MEMBERSHIP` 继续按 Linux 组号加入或退出 multicast
  group；新增 `NETLINK_EXT_ACK`、`NETLINK_CAP_ACK`、`NETLINK_NO_ENOBUFS` 的 per-socket 状态保存。
  这些选项当前不再被误解释成 group membership，避免 libnl/wpa_supplicant 对 netlink
  能力探测时产生非 Linux 行为。
- 修正 netlink 和 AF_PACKET connection 从 pool 复用时未明确清零的问题。Linux 每个新 socket
  都是干净状态；NuttX pool object 复用时如果不清零，可能遗留旧的 groups、filter、txstatus
  或新加的 netlink flags。
- 补齐 AF_PACKET `PACKET_*` 类型语义。原 NuttX packet socket 在 BPF `SKF_AD_PKTTYPE` 和
  `sockaddr_ll.sll_pkttype` 上固定当作 `PACKET_HOST`，这和 Linux 不一致。当前根据以太网
  frame 源/目的地址计算 `PACKET_HOST`、`PACKET_BROADCAST`、`PACKET_MULTICAST`、
  `PACKET_OTHERHOST`、`PACKET_OUTGOING`，并用于 packet BPF filter 和 `recvmsg()` 返回的
  `sockaddr_ll`。这对应当前日志里的 `WARNING: ICMP packet with unknown type: 8` 方向：
  如果本机发出的 echo request 被当作普通入站包，IPv4/ICMP 层就会看到不该消费的 request。
- 修正 libnl 在 NuttX 静态 app 构建下的 `__init`/`__exit` 处理。NuttX 当前不是 Linux ELF
  shared-library constructor/destructor 运行模型，因此仅在 `__NuttX__` 下把这些函数标记为
  unused；Linux 平台仍保留 constructor/destructor 语义。
- 修正 NuttX `strerror_r()` ABI 适配，避免沿用 GNU `strerror_r()` 返回 `char *` 的假设。
- 清理 wpa_supplicant、libnl、cfg80211/nl80211 中的格式化类型、变量 shadow、
  misleading-indentation 和未使用 debug 变量等 warning。
- `build-nsh-esp_hosted_ng.sh` 中 `semantic_version` Python module 缺失时走 NXboot header
  fallback 是脚本支持的正常路径，日志从 `WARNING` 改为 `INFO`，避免把可接受的 fallback
  混入真正异常的 warning 列表。

最新本地构建验证：

```text
./FeatherCore_ESP/tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng.sh -j8
```

结果：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1815516 bytes
```

对 `/tmp/feather_esp_build14.log` 扫描 `warning:`、`WARNING:`、`error:`，当前无命中。

#### 2026-06-09 继续排查：NuttX 对齐 Linux socket/netdev 行为

最新 H7S7 日志已经证明链路能走到：

- ESP32C5 SPI boot event 正常。
- `wlan0` 通过 ESP-Hosted NG host driver 注册。
- `wpa_supplicant -Dnl80211` 能完成 scan/auth/assoc。
- WPA2-PSK 4-way handshake 至少一次能完成，PTK/GTK 能下发。
- DHCP、DNS、外网 `ping baidu.com` 可用。

但日志里仍有两个 Linux 行为差异需要收口：

1. `nl80211: Event message available` 在 NSH 命令间隙仍会出现。  
   这说明 libnl 的 netlink event fd 可能被空唤醒。Linux `poll()` 只返回调用者请求的事件；
   NuttX netlink poll 之前无条件把 `POLLOUT` 放进即时事件，容易让只等 `POLLIN` 的事件 fd
   醒来但没有消息可读。当前已经改成只按 `fds->events` 报告 `POLLOUT`/`POLLIN`。

2. `ping` 期间出现：

   ```text
   WARNING: ICMP packet with unknown type: 8
   ```

   ICMP type 8 是本机发出的 Echo Request。Linux 普通 netdev RX 路径不会把源 MAC 等于本机
   `dev_addr` 的以太帧再送进 IPv4/ICMP raw receive path；AF_PACKET 若要看 outgoing 包，
   应从 TX/packet socket 语义单独标记为 `PACKET_OUTGOING`。当前 NuttX Linux netdev compat
   入口 `ieee80211_linux_netif_rx()` 已补齐这个行为：如果驱动/总线回灌了源 MAC 等于本机
   MAC 的以太帧，直接丢弃，不再交给普通 L3 input。这个改动针对的是 NuttX netdev 语义，
   不是 ping 应用层绕过。

3. `RTM_SETLINK` 的 `IFLA_OPERSTATE`/`IFLA_LINKMODE` 以前只是接受请求，没有保存，也不会在
   `RTM_GETLINK` 中回读。Linux 下这两个字段是 per-interface 可见状态，wpa_supplicant/libnl
   会把它作为接口状态的一部分处理。当前在 NuttX `NETLINK_ROUTE` 层补了 per-ifindex 状态缓存：
   `RTM_SETLINK` 写入，`RTM_GETLINK` 枚举时带回；未设置时按接口 up/down 推导默认
   `IF_OPER_UP`/`IF_OPER_DOWN`。

4. Linux data frame filter sysctl 以前在 NuttX 上只是打印“不支持”并返回成功，这不符合“行为对齐”
   原则。Linux wpa_supplicant 会通过：

   - `/proc/sys/net/ipv4/conf/<if>/drop_unicast_in_l2_multicast`
   - `/proc/sys/net/ipv6/conf/<if>/drop_unicast_in_l2_multicast`
   - `/proc/sys/net/ipv4/conf/<if>/drop_gratuitous_arp`
   - `/proc/sys/net/ipv6/conf/<if>/drop_unsolicited_na`

   配置 per-interface 数据帧过滤。NuttX 没有这套 procfs sysctl，因此当前在
   `ieee80211_linux_set_data_frame_filters()` 中实现原生等价状态，并在
   `ieee80211_linux_netif_rx()` 入普通 L3 前执行过滤：

   - GTK filter：丢弃 L2 multicast/broadcast 承载的 IPv4/IPv6 unicast 数据帧。
   - ARP filter：丢弃 gratuitous ARP。
   - NA filter：丢弃 unsolicited IPv6 Neighbor Advertisement。

   wpa_supplicant 的 NuttX 分支现在调用这个接口，不再跳过 Linux 原有行为。

5. `nl80211: Event message available` 仍有另一类空醒根因：Linux netlink multicast group
   是按 netlink protocol 隔离的，`NETLINK_ROUTE`、`NETLINK_GENERIC`、`NETLINK_NETFILTER`
   各自拥有独立 group 编号空间。NuttX 之前只按 group bit 过滤广播接收者，没有检查 socket
   protocol，因此 `RTNLGRP_LINK`/地址/邻居等 route 事件如果 group bit 与 generic netlink
   订阅组碰撞，可能误唤醒 nl80211 event socket。当前已经在 NuttX netlink connection 中记录
   socket protocol，并新增 `netlink_add_broadcast_protocol()`：

   - route family 广播只投递给 `NETLINK_ROUTE` socket。
   - generic/nl80211 广播只投递给 `NETLINK_GENERIC` socket。
   - netfilter 广播只投递给 `NETLINK_NETFILTER` socket。
   - 保留旧 `netlink_add_broadcast()` 作为兼容 wrapper，但实际发送点已改成协议感知版本。

   这对齐的是 Linux “group id scoped by protocol”的行为，不是过滤日志噪声。

6. NuttX 原先只允许少数已编译 backend 的 netlink protocol 创建 socket，例如
   `NETLINK_ROUTE`、`NETLINK_GENERIC`、`NETLINK_NETFILTER`。Linux 的 AF_NETLINK 则有完整
   protocol namespace，用户态会通过这些协议号做能力探测。当前已经在 NuttX 侧补齐 Linux
   已定义协议号的 socket setup 支持：

   - `NETLINK_USERSOCK`、`NETLINK_FIREWALL`、`NETLINK_SOCK_DIAG`/`NETLINK_INET_DIAG`、
     `NETLINK_NFLOG`、`NETLINK_XFRM`、`NETLINK_SELINUX`、`NETLINK_ISCSI`、
     `NETLINK_AUDIT`、`NETLINK_FIB_LOOKUP`、`NETLINK_CONNECTOR`、`NETLINK_IP6_FW`、
     `NETLINK_DNRTMSG`、`NETLINK_KOBJECT_UEVENT`、`NETLINK_SCSITRANSPORT`、
     `NETLINK_ECRYPTFS`、`NETLINK_RDMA`、`NETLINK_CRYPTO`、`NETLINK_SMC` 等协议号
     现在不再在 `socket(PF_NETLINK, ..., proto)` 阶段直接返回 `-EPROTONOSUPPORT`。
   - 已有 backend 的 `NETLINK_ROUTE`、`NETLINK_GENERIC`、`NETLINK_NETFILTER` 继续走各自
     send/recv 实现。
   - 尚未实现 backend 的协议可以创建 socket，但发送具体请求时返回 `-EOPNOTSUPP`。这对齐
     Linux 用户态“先打开 socket 再探测能力”的行为，同时不会把缺失 backend 伪装成成功。
   - 公共 `netpacket/netlink.h` 补齐 Linux ABI：`NETLINK_INET_DIAG`、`MAX_LINKS`、
     `NETLINK_BROADCAST_ERROR`、`NETLINK_RX_RING`、`NETLINK_TX_RING`、
     `NETLINK_LISTEN_ALL_NSID`、`NETLINK_LIST_MEMBERSHIPS`、`NETLINK_GET_STRICT_CHK`、
     `struct nl_pktinfo`、`struct nl_mmap_req`、`struct nl_mmap_hdr` 和 mmap 状态定义。
   - `NETLINK_BROADCAST_ERROR`、`NETLINK_LISTEN_ALL_NSID`、`NETLINK_GET_STRICT_CHK` 当前作为
     per-socket 状态保存；`NETLINK_PKTINFO` 作为 per-socket 状态保存，`recvmsg()` 按 Linux
     语义返回 `SOL_NETLINK/NETLINK_PKTINFO` control message；`NETLINK_LIST_MEMBERSHIPS`
     支持 getsockopt 回读当前组订阅 bitmap。`NETLINK_RX_RING`、`NETLINK_TX_RING`
     仍明确返回 `-EOPNOTSUPP`，等待真正需要 mmap ring 语义时再按 Linux 行为补齐。

7. 最新板端日志重新复现了 `WARNING: ICMP packet with unknown type: 8`。继续追到
   `apps/netutils/ping/icmp_ping.c` 后确认根因不是 ESP32C5 或路由器返回了异常 ICMP，而是
   ping 设置 raw ICMP filter 的 socket level 和 Linux 不一致：

   - Linux raw ICMP filter 在 `third/linux-7.0.10/net/ipv4/raw.c` 中挂在 `SOL_RAW` 层，
     对应 `setsockopt(fd, SOL_RAW, ICMP_FILTER, ...)`。
   - NuttX 原代码使用 `setsockopt(fd, IPPROTO_ICMP, ICMP_FILTER, ...)`。在当前 ABI 中
     `IPPROTO_ICMP == 1`，同时 `SOL_SOCKET == 1`，这会先落进 socket-level option 路径，
     而 `ICMP_FILTER == 1` 又和 `SO_BROADCAST == 1` 数值碰撞，导致 ICMP raw filter 没有
     真正装到 ICMP socket 上。
   - filter 未生效时，若底层出现本机 Echo Request 的回灌帧，ping raw socket 就可能读到
     ICMP type 8，并打印 unknown type。

   当前已把 NuttX ping 改为 Linux 等价的 `SOL_RAW + ICMP_FILTER`。这个修的是 NuttX/Linux
   socket API 行为差异，不是在 ICMP receive path 中丢日志。

8. 继续参考 `third/linux-7.0.10/net/netlink/af_netlink.c` 补齐 netlink socket option
   回读路径。NuttX 之前只有 `setsockopt()`，`g_netlink_sockif.si_getsockopt` 为空，
   导致 Linux 用户态无法按标准方式回读 netlink option 状态：

   - 新增 `netlink_getsockopt()`，支持 `NETLINK_PKTINFO`、`NETLINK_BROADCAST_ERROR`、
     `NETLINK_NO_ENOBUFS`、`NETLINK_CAP_ACK`、`NETLINK_EXT_ACK`、
     `NETLINK_LISTEN_ALL_NSID`、`NETLINK_GET_STRICT_CHK`。
   - 新增 `NETLINK_LIST_MEMBERSHIPS` getsockopt。当前 NuttX 组数为 32，返回一个
     Linux 兼容的 32-bit bitmap；buffer 太小时按 Linux 行为只回填所需长度，不把能力探测
     误报成 unsupported。
   - `NETLINK_PKTINFO` 不再返回 `-EOPNOTSUPP`：setsockopt 保存 per-socket flag，
     netlink response 记录 multicast group，recvmsg 在用户提供 control buffer 时附加
     `struct nl_pktinfo`。

同时清理了 `netdevice_compat.c` 中阶段性的前 24 包 RX 打印，只保留开关化/必要日志。

本轮构建验证：

```text
./FeatherCore_ESP/tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng.sh -j8
```

结果：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1816004 bytes
```

对 `/tmp/feather_esp_build23.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。

追加 `ICMP_FILTER` socket level 修正后的 build24：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1816004 bytes
```

对 `/tmp/feather_esp_build24.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。

追加 netlink getsockopt/`NETLINK_PKTINFO`/`NETLINK_LIST_MEMBERSHIPS` 修正后的 build25：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1816172 bytes
```

对 `/tmp/feather_esp_build25.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。

build25 上板后出现新的卡点：

```text
nl80211: Kernel version: NuttX 0.0.0 (...)
nl80211: Maximum supported attribute ID: 348
```

从 wpa_supplicant/libnl 继续反查，卡住位置在 `NL80211_CMD_GET_WIPHY` dump 能力读取阶段。
`libnl-3.2.25/lib/nl.c` 在 `NL_MSG_PEEK` 打开时会先调用：

```text
recvmsg(fd, &msg, MSG_PEEK | MSG_TRUNC)
```

它依赖 Linux netlink 的两个 datagram 语义：

- `MSG_PEEK` 只查看队首消息，不把消息从 socket receive queue 中移除。
- `MSG_TRUNC` 在 buffer 不足时返回完整 datagram 长度，并设置 `msg_flags & MSG_TRUNC`。

Linux 对应行为在 `third/linux-7.0.10/net/netlink/af_netlink.c` 中体现为：copy 长度不足时设置
`MSG_TRUNC`，如果调用方传入 `MSG_TRUNC`，返回完整 `data_skb->len`。NuttX 原 netlink
`recvmsg()` 没有实现这两个语义，`MSG_PEEK | MSG_TRUNC` 会被当成普通读取，导致 libnl
第一次探测长度时就把 GET_WIPHY multipart 响应从队列中消费掉。随后 libnl 第二次真正读取时，
队列已经前移，可能跳过能力 dump 的一部分或终止消息，最终表现为 wpa_supplicant 停在
`Maximum supported attribute ID` 后等待后续响应。

当前修正：

- 新增 `netlink_trypeek_response()`/`netlink_peek_response()`，实现 NuttX 原生的非破坏性队首读取。
- `netlink_recvmsg()` 根据 `MSG_PEEK` 选择 peek 或 dequeue；peek 路径不释放队列节点。
- buffer 小于 netlink message 长度时设置 `msg->msg_flags |= MSG_TRUNC`。
- 调用方传入 `MSG_TRUNC` 时返回完整 netlink message 长度，对齐 Linux `af_netlink.c`。
- 保留 `NETLINK_PKTINFO` control message 行为，peek 路径同样可读取 cmsg，但不改变队列状态。

追加 `MSG_PEEK`/`MSG_TRUNC` netlink receive 语义修正后的 build26：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1816364 bytes
```

对 `/tmp/feather_esp_build26.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。

#### build28：wpa_supplicant 停在 Maximum supported attribute ID

最新硬件日志中，`wpa_supplicant -Dnl80211 ... -dd` 停在：

```text
nl80211: Kernel version: NuttX ...
nl80211: Maximum supported attribute ID: 348
```

这个位置对应 `wpa_driver_nl80211_init_nl_global()` 读取 generic netlink family/cache 完成后的阶段。
代码下一步会释放 family cache、注册 nl80211 event socket，然后创建 ioctl socket 并进入接口初始化。
如果日志停在这里，说明 libnl 已经拿到了 `nl80211` family 信息，但后续某个 netlink fd 没有收到
预期响应或事件，用户态在等待。

本轮定位出的 NuttX/Linux 行为差异是 netlink port ID 的命名空间：

- Linux 中 netlink port ID 是按 protocol 隔离的。`NETLINK_ROUTE`、`NETLINK_GENERIC`、
  `NETLINK_NETFILTER` 等协议各自有独立的 port ID 空间。
- `wpa_supplicant`/libnl 初始化时会同时创建 route netlink socket 和 generic netlink socket。
  如果 NuttX 只按数值 `pid` 匹配 response，而不匹配 protocol，那么两个 socket 数值 port ID
  碰撞时，generic nl80211 reply 可能被投递到 route socket，真正等待 generic reply 的 libnl
  socket 就会一直等下去。
- 这类问题不一定每次都触发，取决于任务 ID、socket 创建顺序和自动绑定 port ID。

当前修正：

- `netlink_add_response_pid()` 增加 `protocol` 参数，response 投递时同时匹配
  `conn->protocol` 和 `conn->pid`。
- generic netlink bridge 的 unicast reply 明确使用 `NETLINK_GENERIC` 投递。
- `netlink_bind()` 的自动 port ID 分配改成 protocol-scoped：优先使用当前 task ID；
  若同 protocol 已占用，则从高位自动分配一个不冲突的 port ID。
- 显式 bind 非零 `nl_pid` 时，如果同 protocol 已有相同 port ID，返回 `-EADDRINUSE`，
  对齐 Linux “同协议内唯一”的行为。
- 在 `wpa_supplicant` nl80211 初始化边界新增标准 `MSG_DEBUG` 阶段日志，便于硬件串口继续定位：

```text
nl80211: Generic netlink cache initialized
nl80211: Register event netlink socket
nl80211: Generic netlink global init done
nl80211: Creating ioctl socket
nl80211: Global driver init complete
```

这些日志只在 `-dd` 下出现，属于 Linux userspace 风格的阶段性 debug，不改变 ESP32C5 固件行为。
ESP32C5 firmware 本轮没有做功能性修改。

追加 protocol-scoped netlink port ID 修正后的 build28：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1816796 bytes
```

对 `/tmp/feather_esp_build28.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。

#### build30：卡点推进到 global init 之后

使用 build28 后，硬件日志已经继续打印：

```text
nl80211: Generic netlink cache initialized
nl80211: Register event netlink socket
nl80211: Generic netlink global init done
nl80211: Creating ioctl socket
nl80211: Global driver init complete
```

这说明上一轮 `NETLINK_GENERIC` response 投递被错误路由的问题已经不再阻塞
`wpa_driver_nl80211_init_nl_global()`。当前新的卡点位于 `nl80211_global_init()` 返回之后，
也就是 wpa_supplicant 即将进入 per-interface driver 初始化：

```text
wpa_driver_nl80211_init()
  -> wpa_driver_nl80211_drv_init()
     -> nl80211_init_bss()
     -> wpa_driver_nl80211_finish_drv_init()
        -> NL80211_CMD_GET_INTERFACE
        -> NL80211_CMD_GET_WIPHY / capability dump
        -> set mode / interface up / read MAC
```

由于 NuttX 上没有标准 Linux `/sys/class/net/<ifname>/device/driver`，原版
`Initialize interface ... (driver: ...)` 日志不一定出现。因此 build30 补充了以下 `-dd`
阶段日志，便于继续把阻塞点定位到具体 Linux 行为：

```text
nl80211: Begin interface init for wlan0
nl80211: Initialize BSS object for wlan0
nl80211: Initialize connect event handle for wlan0
nl80211: Finish low-level driver init for wlan0
nl80211: finish_drv_init start ifname=wlan0 ifindex=...
nl80211: initial interface mode for wlan0 is ...
nl80211: Query driver capabilities for wlan0
nl80211: Send netlink command type=... cmd=... flags=...
nl80211: Waiting for netlink command cmd=... seq=...
nl80211: Netlink command cmd=... seq=... completed ret=...
```

这些日志只在 wpa_supplicant `-dd` 下出现，仍然不改变 ESP32C5 firmware 功能。下一轮硬件日志
如果停在 `Waiting for netlink command cmd=<N>`，就说明 NuttX 的 nl80211/cfg80211 对应命令
没有按 Linux netlink 语义返回 valid/ack/finish；应按该 `cmd` 反查
`third/linux-7.0.10/net/wireless/nl80211.c` 和当前
`FeatherCore_ESP/nuttx/wireless/ieee80211/cfg80211/nl80211.c` 的差异。

追加 per-interface init 边界日志后的 build30：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1818348 bytes
```

对 `/tmp/feather_esp_build30.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。

#### build34：修复 `NL80211_CMD_GET_INTERFACE` 等待不返回

build30 之后硬件日志进一步定位到：

```text
nl80211: finish_drv_init start ifname=wlan0 ifindex=1 first=1
nl80211: Send netlink command type=19 cmd=5 flags=0x0
nl80211: Waiting for netlink command cmd=5 seq=...
```

`cmd=5` 是 `NL80211_CMD_GET_INTERFACE`。Linux 原生路径中，wpa_supplicant 在
`wpa_driver_nl80211_finish_drv_init()` 阶段会用它读取接口模式、ifindex、MAC 等基础信息。
这不是 ESP32C5 firmware 命令，也不是 SPI 事务超时；如果这里没有返回，说明 H7 侧
`NETLINK_GENERIC/nl80211` 没有按 Linux netlink datagram 语义把 response/ACK 放回
对应 socket。

本轮定位到根因是 NuttX netlink 内部 response ABI 在不同移植文件中出现了旧拷贝：

```text
struct netlink_response_s {
    sq_entry_t flink;
    struct nlmsghdr msg;
};
```

而当前 NuttX netlink core 已经扩展为：

```text
struct netlink_response_s {
    sq_entry_t flink;
    uint32_t group;
    struct nlmsghdr msg;
};
```

`genetlink_bridge.c`/`nl80211_metadata.c` 如果继续使用旧布局分配 response，
`netlink_recvmsg()` 会按新布局读取 `msg`，导致 `nlmsg_len/type/seq` 偏移错误。
表现出来就是 wpa_supplicant 端一直等待 `NL80211_CMD_GET_INTERFACE` 的有效响应。

修复方式：

- 新增 `nuttx/include/nuttx/net/netlink_kernel.h`，把 `NETLINK_HANDLE`、
  `struct netlink_response_s`、generic-netlink 注册结构和相关内核 helper 原型集中放在一个
  kernel-side 头文件中。
- `nuttx/include/nuttx/net/netlink.h` 继续作为 NuttX 用户态/内核 netlink 入口，但内部 ABI
  统一引用 `netlink_kernel.h`。
- `wireless/ieee80211/genetlink_bridge.c` 和 `wireless/ieee80211/cfg80211/nl80211_metadata.c`
  不再维护本地旧结构定义，改为包含 `netlink_kernel.h`。
- 因 ieee80211 使用 Linux UAPI 的 `struct nlmsghdr/sockaddr_nl`，这两个文件必须先包含
  `linux/netlink.h`，再包含 `nuttx/net/netlink_kernel.h`，避免把 NuttX `netpacket/netlink.h`
  和 Linux UAPI 混进同一个编译单元。

本轮没有修改 ESP32C5 firmware 功能。当前构建验证：

```text
make -j8
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin 1818340 bytes
```

对 `/tmp/feather_esp_make34.log` 扫描 `warning:`、`WARNING:`、`error:`、`ERROR:`，当前无命中。
重新烧录 H7 app 后，预期 `cmd=5` 应继续打印：

```text
nl80211: Netlink command cmd=5 seq=... completed ret=0
nl80211: initial interface mode for wlan0 is ...
```

如果下一轮仍有卡点，应继续看最后一条 `Waiting for netlink command cmd=<N>`，按该命令对齐
Linux `third/linux-7.0.10/net/wireless/nl80211.c` 中对应 handler 的 response/ACK/finish 行为。

#### build35：补齐 ESP-Hosted NG survey/PMKSA flush 原生行为

最新硬件日志已经显示主连接链路跑通：

```text
NL80211_CMD_TRIGGER_SCAN ret=0
NL80211_CMD_GET_SCAN ret=0
NL80211_CMD_AUTHENTICATE ret=0
NL80211_CMD_ASSOCIATE ret=0
NL80211_CMD_NEW_KEY ret=0
NL80211_CMD_SET_STATION ret=0
WPA: Key negotiation completed
CTRL-EVENT-CONNECTED
ifconfig wlan0 dhcp
ping baidu.com -> 10 packets transmitted, 10 received, 0% packet loss
```

本轮继续按 Linux 源码核对剩余非零返回：

```text
cmd= 12 ret= -67 count=6   NL80211_CMD_DEL_KEY
cmd= 50 ret= -95 count=1   NL80211_CMD_GET_SURVEY
cmd= 54 ret= -95 count=1   NL80211_CMD_FLUSH_PMKSA
cmd= 79 ret= -95 count=1   NL80211_CMD_SET_REKEY_OFFLOAD
```

对照 `third/linux-7.0.10/net/wireless/nl80211.c` 后的判断：

- `DEL_KEY -ENOLINK`：Linux `nl80211_key_allowed()` 在 station 未 connected 且没有
  `NL80211_EXT_FEATURE_ASSOC_FRAME_ENCRYPTION` 时返回 `-ENOLINK`。wpa_supplicant 启动清理
  旧 key 时会触发这类删除请求，因此这是 Linux 对齐行为，不应强行改成成功。
- `GET_SURVEY -EOPNOTSUPP`：Linux 在 driver 没有 `dump_survey` callback 时返回
  `-EOPNOTSUPP`。为了让 NuttX 能原生提供 Linux survey dump，本轮给 ESP-Hosted NG cfg80211
  ops 增加 `esp_cfg80211_dump_survey()`，按 wiphy 的 2.4G/5G channel table 逐项返回
  `NL80211_CMD_NEW_SURVEY_RESULTS`；当前没有 firmware airtime/noise 统计时只上报 frequency，
  若 channel 等于当前 BSS channel，则设置 `SURVEY_INFO_IN_USE`。
- `FLUSH_PMKSA -EOPNOTSUPP`：ESP-Hosted NG firmware 没有 PMKSA cache 命令，但 wpa 启动时
  只是清空 cache。清空 host 侧空 cache 可以按 Linux driver no-op 语义成功返回。本轮新增
  `esp_cfg80211_flush_pmksa()` 返回 0；没有实现 `set_pmksa/del_pmksa` 假成功，避免让上层误以为
  firmware 已缓存 PMKSA。
- `SET_REKEY_OFFLOAD -EOPNOTSUPP`：ESP-Hosted NG host/firmware 当前没有 GTK rekey offload 命令。
  Linux driver 可合法返回 `-EOPNOTSUPP`，wpa_supplicant 会回退到用户态 rekey，所以这里保持
  明确 unsupported，而不是伪造 offload 成功。

本轮代码改动：

```text
nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cfg80211.c
  esp_cfg80211_dump_survey()
  esp_cfg80211_flush_pmksa()
  cfg80211_ops.dump_survey
  cfg80211_ops.flush_pmksa

nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cmd.c
  CMD_RESPONSE_BUSY        -> -EBUSY
  CMD_RESPONSE_UNSUPPORTED -> -EOPNOTSUPP
  CMD_RESPONSE_INVALID     -> -EINVAL
  CMD_RESPONSE_FAIL        -> -EIO
```

构建验证：

```text
cd /home/uan/Feather-develop-WIFI/FeatherCore_ESP/nuttx
make -j8
LD: nuttx
flash: 1817500 B
sram: 46616 B
CP: nuttx.hex
CP: nuttx.bin
```

下一轮硬件验证预期：

```text
NL80211_CMD_GET_SURVEY completed ret=0
NL80211_CMD_FLUSH_PMKSA completed ret=0
NL80211_CMD_SET_REKEY_OFFLOAD completed ret=-95
```

其中 `SET_REKEY_OFFLOAD ret=-95` 是当前 ESP32C5 firmware 能力下的 Linux 对齐结果。

#### Linux 有而 NuttX 当前仍需对齐的能力审计

按“Linux 有的，NuttX 不支持就原生实现”的原则，当前分为三类：

已补齐：

- `NETLINK_ADD_MEMBERSHIP`/`DROP_MEMBERSHIP` 和 `NETLINK_EXT_ACK`、`NETLINK_CAP_ACK`、
  `NETLINK_NO_ENOBUFS` sockopt 语义。
- `NETLINK_PKTINFO` set/get 与 `SOL_NETLINK/NETLINK_PKTINFO` cmsg，以及
  `NETLINK_LIST_MEMBERSHIPS` getsockopt 回读。
- Linux AF_NETLINK protocol namespace 的 socket setup：除 `NETLINK_UNUSED` 外，已定义协议号
  都可创建 socket；未实现 backend 的协议在具体请求阶段返回 `-EOPNOTSUPP`。
- Linux netlink 公共 ABI 中的 socket option、`NETLINK_INET_DIAG` alias、mmap 结构体和状态定义。
- Linux netlink `recvmsg(MSG_PEEK | MSG_TRUNC)` datagram 语义：peek 不消费消息，trunc 返回完整
  message 长度并设置 `MSG_TRUNC`。
- Linux netlink port ID 按 protocol 隔离：`NETLINK_ROUTE` 和 `NETLINK_GENERIC` 的数值
  port ID 不再互相误投递 response。
- netlink/AF_PACKET socket pool object 清零。
- AF_PACKET `PACKET_HOST`、`PACKET_BROADCAST`、`PACKET_MULTICAST`、`PACKET_OTHERHOST`、
  `PACKET_OUTGOING` 分类，含 BPF `SKF_AD_PKTTYPE` 和 `sockaddr_ll.sll_pkttype`。
- Linux netdev RX 不把本机源 MAC 帧送进普通 L3 input。
- Linux data frame filter sysctl 的等价 per-interface 行为：GTK unicast-in-L2-multicast、
  gratuitous ARP、unsolicited IPv6 NA 过滤。
- netlink multicast group 按 protocol 隔离：`NETLINK_ROUTE`、`NETLINK_GENERIC`、
  `NETLINK_NETFILTER` 的组空间不再互相误投递。
- raw ICMP `ICMP_FILTER` 使用 Linux 的 `SOL_RAW` socket level，不再误走 `SOL_SOCKET`
  option namespace，避免 ping raw socket 继续接收本机 Echo Request type 8。
- rfkill 基础查询、rtnetlink `IFLA_OPERSTATE`/`IFLA_LINKMODE` SET/GET、
  SIOCGIF flags/MAC/MTU 等 wpa_supplicant 初始化路径。
- nl80211/cfg80211 scan、auth、assoc、connect/disconnect、key install、scan result dump、
  regulatory/basic wiphy capability dump。
- ESP-Hosted NG `dump_survey`：按 wiphy channel table 原生返回 Linux
  `NL80211_CMD_NEW_SURVEY_RESULTS`。
- ESP-Hosted NG `flush_pmksa`：以 host 侧空 cache no-op 方式对齐 Linux driver 行为。
- ESP-Hosted NG firmware command status 到 Linux errno 的映射：不再把 busy/unsupported/invalid/fail
  全部折成 `-1`，而是分别向 cfg80211/nl80211 上层返回 `-EBUSY`、`-EOPNOTSUPP`、
  `-EINVAL`、`-EIO`。

正在硬件验证：

- 首轮 4-way handshake 偶发 `reason:15 / 4WAY_HANDSHAKE_TIMEOUT`。日志显示 supplicant 已发送
  M4，AP 仍重传 M3，随后自动重连成功。重点继续看 SPI TX 完成时序、EAPOL TX 是否被及时
  clock 到 ESP32C5、以及 RX workqueue 是否饥饿。
- `nl80211: Event message available` 空醒已经先后修复了 poll event mask 和 protocol-scoped
  multicast 两个可见根因。最新日志里 `Event message available` 多数后续都有真实
  `Drv Event`；命令间隙仍偶有 debug 提示，继续观察是否还有无消息可读的空醒。
- 最新用户日志中 `NuttX does not expose Linux data frame filter sysctls` 已消失，`ping` 的
  ICMP type 8 warning 在 build24 后也已消失，外网 `ping baidu.com` 10/10，0% packet loss。

待实现/待决策：

- rekey offload：
  当前 ESP-Hosted NG/cfg80211 正确返回不支持，wpa_supplicant 会打印
  `Driver does not support rekey offload`。如果要完全对齐支持，需要确认 ESP32C5 firmware
  是否提供 GTK rekey offload 命令；若固件不支持，NuttX 应保持 Linux 等价 unsupported。
- 更完整 rtnetlink multicast：
  目前已覆盖 wpa 初始化和 operstate 所需路径；后续如 `wpa_cli`、热插拔、多接口、AP/P2P
  需要更多 `RTM_*` event，应在 NuttX netlink route family 中补齐，而不是在 libnl/wpa 里绕过。
- 真实 backend：
  当前 Linux AF_NETLINK protocol namespace 已经补齐到 socket 创建层；`NETLINK_USERSOCK`、
  `NETLINK_SOCK_DIAG`、`NETLINK_KOBJECT_UEVENT`、`NETLINK_AUDIT`、`NETLINK_XFRM` 等协议
  如果后续被 Linux 用户态真实使用，需要分别实现对应 NuttX backend，而不是只保留 socket
  shell。
- management frame multicast/registration：
  STA 基础连接路径已可用；P2P、SAE、remain-on-channel、offchannel action frame 仍有大量
  Linux nl80211 能力需要逐项接 cfg80211/rdev 或明确返回 `-EOPNOTSUPP`。

本轮显式缺口扫描：

```text
rg -n "not supported|unsupported|not implemented|EOPNOTSUPP|ENOTSUP|TODO|FIXME" \
  FeatherCore_ESP/apps/wireless/wifi/wpa_supplicant-2.11/src/drivers \
  FeatherCore_ESP/nuttx/wireless/ieee80211 \
  FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng \
  FeatherCore_ESP/nuttx/net/netlink \
  FeatherCore_ESP/nuttx/net/pkt
```

按当前 STA + ESP32C5 SPI 路径分类：

- 必须补齐，且本轮已经补齐：
  netlink poll 事件掩码、rtnetlink operstate/linkmode、self-source RX drop、
  Linux data-frame filter sysctl 等价行为、netlink multicast protocol namespace 隔离、
  `NETLINK_PKTINFO`/`NETLINK_LIST_MEMBERSHIPS`、ESP-Hosted NG survey dump、PMKSA flush。
- 必须补齐，但需要下一轮按日志触发点继续做：
  management frame registration 的完整 ACK/事件路径、packet socket 多 iov/更多 sockopt、
  更完整的 rtnetlink event 属性覆盖。
- 可以按 Linux 方式返回 unsupported，不应伪造成功：
  ESP32C5 firmware 未暴露的 GTK rekey offload、FTM ranging、mesh、IBSS、OCB、AP/VLAN、
  monitor/radiotap 注入、P2P remain-on-channel/offchannel action frame。
- 暂不影响当前 STA 联网路径：
  `PACKET_MMAP`、共享 IOB 优化、hostapd/AP 高级特性、sched_scan/offload scan。

这里的判断原则是：如果 Linux 用户态依赖该行为完成当前 STA 连接/收发，就在 NuttX 原生实现；
如果 Linux cfg80211/driver 也允许硬件返回不支持，则保持明确 `-EOPNOTSUPP`，让上层走标准降级路径。

最新运行进度：

- `wpa_supplicant -Dnl80211 -iwlan0 -c/etc/wpa_supplicant.conf -dd &` 能完成 scan/auth/assoc。
- WPA2-PSK 4-way handshake 能完成，PTK/GTK 能下发到 ESP-Hosted NG host driver 和
  ESP32C5 firmware。
- `CTRL-EVENT-CONNECTED`、DHCP、DNS 解析、路由器 ping 已经跑通。
- 外网域名 ping 已经可用；最新日志中 `ping baidu.com` 10/10 收包、0% packet loss，
  未再出现 `ICMP packet with unknown type: 8`。当前仍需继续观察 `nl80211: Event message available`
  在 `-dd` 下是否只是正常事件提示，以及长时间运行是否还有 command timeout/late reply。

### 已排除的方向

- 不是 ESP32C5 firmware 必须功能性修改。当前固件只允许保留必要日志，功能行为保持
  ESP-Hosted NG 原始路径。
- 不是 PSK 错误。早期 `WRONG_KEY` 是 4-way handshake 超时后的副作用。
- 不是单纯 SPI4 transaction 太慢。分段日志显示 `transport_ms` 很小，慢点在 RX queue/work。
- 不是只需要扩大线程栈。栈大小已经调大，真正阻塞点来自 packet socket/poll 语义和 TX/RX
  同步路径。

### 后续验证项

已经完成：

- scan/auth/assoc。
- WPA2-PSK 4-way handshake。
- PTK/GTK install。
- `CTRL-EVENT-CONNECTED`。
- DHCP 获取 `wlan0` IPv4 配置。
- `ping` 路由器。
- DNS 解析。
- `ping baidu.com` 已可用；不同轮次日志中出现过 10/10 和 8/10，需要继续做长时间稳定性统计。

继续验证：

- DHCP 是否能多次重连后稳定获取地址。
- `ping` 外网 IP 和外部域名的长时间稳定性。
- TCP/UDP `iperf` 单向和双向吞吐。
- AP 断电/路由器重启后的 disconnect/reconnect。
- 长时间运行时是否还有 command timeout、SPI RX 堆积或 workqueue 饥饿。
- 继续清理阶段性 debug log，仅保留必要的错误日志和少量 trace 开关。

#### build36：继续收口 Linux socket ABI 和错误语义

本轮继续按 `third/linux-7.0.10` 对照 NuttX 侧 netlink/packet socket，重点处理
“Linux 有语义，但 NuttX 移植层还暴露 `ENOSYS` 或常量缺失”的点。这里的原则是：

- socket ABI 已存在但某个 capability 不支持时，返回 Linux 用户态可理解的
  `-EOPNOTSUPP`、`-ENOPROTOOPT`、`-EAFNOSUPPORT`、`-EINVAL` 等错误。
- Linux packet/netlink 真实支持且可以在 NuttX 内核态保存状态的，补 NuttX 原生状态。
- 不能由 host 侧伪造 firmware 能力的项目，不返回假成功。

代码补齐：

```text
nuttx/include/netpacket/packet.h
  PACKET_MR_PROMISC
  PACKET_MR_ALLMULTI
  PACKET_MR_UNICAST

nuttx/net/pkt/pkt_setsockopt.c
  PACKET_ADD_MEMBERSHIP/PACKET_DROP_MEMBERSHIP 支持 Linux 的
  MULTICAST/PROMISC/ALLMULTI/UNICAST membership 类型。
  MULTICAST/UNICAST 走设备 d_addmac/d_rmmac；缺少硬件过滤 hook 时返回 -EOPNOTSUPP。
  PROMISC/ALLMULTI 更新 net_driver_s.d_flags 中的 IFF_PROMISC/IFF_ALLMULTI。
  非法 mr_type 从原来的 -ENOSYS 改为 Linux 风格 -EINVAL。

nuttx/net/pkt/pkt_recvmsg.c
  非 RAW/DGRAM packet socket 类型从 -ENOSYS 改为 -EOPNOTSUPP。
  recvmsg flags 按 Linux packet socket 限定为
  MSG_PEEK/MSG_DONTWAIT/MSG_TRUNC/MSG_ERRQUEUE，其它 flag 返回 -EINVAL。

nuttx/net/netlink/netlink_route.c
  未实现的 RTM_* command 默认从 -ENOSYS 改为 -EOPNOTSUPP。

nuttx/net/netlink/netlink_netfilter.c
  未实现的 netfilter subsystem/type 从 -ENOSYS 改为 -EOPNOTSUPP；
  不支持的 conntrack address family 返回 -EAFNOSUPPORT。

nuttx/net/netlink/netlink_sockif.c
  netlink sendmsg/recvmsg 遇到 MSG_OOB 时按 Linux 返回 -EOPNOTSUPP。
```

对照 Linux 依据：

```text
third/linux-7.0.10/include/uapi/linux/if_packet.h
third/linux-7.0.10/net/packet/af_packet.c
third/linux-7.0.10/net/netlink/af_netlink.c
```

当前扫描结果：

```text
rg -n "ENOSYS" \
  FeatherCore_ESP/nuttx/net/netlink \
  FeatherCore_ESP/nuttx/net/pkt \
  FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/upstream_host

无命中
```

构建验证：

```text
cd /home/uan/Feather-develop-WIFI/FeatherCore_ESP/nuttx
make -j8
LD: nuttx
flash: 1817540 B
sram: 46616 B
CP: nuttx.hex
CP: nuttx.bin
```

下一步继续观察硬件日志中是否还有用户态可见的：

```text
Protocol not supported
Operation not supported
Function not implemented
Waiting for netlink command cmd=...
```

如果仍有卡点，优先根据最后一个 `nl80211 cmd=<N>` 查
`third/linux-7.0.10/net/wireless/nl80211.c` 对应 handler，判断应补 NuttX backend、
cfg80211/rdev callback，还是保持 Linux 允许的 `-EOPNOTSUPP` 降级。

#### build37：修正 route multicast 的跨协议 NLMSG_DONE 空事件

本轮用户日志的 app 镜像时间为：

```text
stm32h7s78-dk: image=app ... Jun  9 2026 19:54:13
```

这是 build35/build36 修复前的镜像，因此日志中的：

```text
NL80211_CMD_FLUSH_PMKSA completed ret=-95
NL80211_CMD_GET_SURVEY completed ret=-95
```

属于旧镜像现象。build35 已补 `flush_pmksa` 和 `dump_survey`，需要烧录新镜像后复测。

这份日志同时暴露了连接成功后仍偶发：

```text
nl80211: Event message available
```

但后面没有对应 `Drv Event ...` 的情况。继续对照 Linux netlink 后定位到一个 NuttX
route multicast 语义偏差：

- Linux `rtnl_notify()` / `netlink_broadcast()` 对 multicast event 只广播事件 skb。
- `NLMSG_DONE` 只用于 dump response 结束，不用于普通 multicast event。
- NuttX 当前 route notify 先用 `netlink_add_broadcast_protocol(NETLINK_ROUTE, ...)`
  发送正文，但随后又调用 `netlink_add_terminator(NULL, NULL, group)`。
- `handle=NULL` 会让 terminator 退化为 protocol wildcard，可能把 `NLMSG_DONE`
  投递给同 group bit 的 `NETLINK_GENERIC/nl80211` event socket，从而让 wpa_supplicant
  看到“有事件可读”，但读到的不是 nl80211 driver event。

本轮修复：

```text
nuttx/net/netlink/netlink_route.c
  route multicast notify 不再追加 NLMSG_DONE terminator：
    netlink_device_notify()
    netlink_device_notify_ipaddr()
    netlink_route_notify()
    netlink_neigh_notify()
    netlink_ipv6_prefix_notify()

nuttx/net/netlink/netlink_conn.c
  netlink_add_terminator(handle, req, group) 对 group>0 要求 handle 非 NULL。
  如果传入 handle=NULL，释放 terminator 并返回 -EINVAL，避免未来再次跨协议广播。
```

对照 Linux 依据：

```text
third/linux-7.0.10/net/netlink/af_netlink.c
third/linux-7.0.10/net/core/rtnetlink.c
third/linux-7.0.10/net/ipv4/devinet.c
third/linux-7.0.10/net/ipv6/addrconf.c
```

构建验证：

```text
cd /home/uan/Feather-develop-WIFI/FeatherCore_ESP/nuttx
make -j8
LD: nuttx
flash: 1817516 B
sram: 46616 B
CP: nuttx.hex
CP: nuttx.bin
```

复测预期：

```text
NL80211_CMD_FLUSH_PMKSA completed ret=0
NL80211_CMD_GET_SURVEY completed ret=0
NL80211_CMD_SET_REKEY_OFFLOAD completed ret=-95
```

`SET_REKEY_OFFLOAD ret=-95` 仍是正确的 Linux 降级行为，因为当前 ESP32C5
ESP-Hosted NG firmware 没有 GTK rekey offload 能力。连接后如果还有
`nl80211: Event message available`，后面应该跟随真实 `Drv Event ...`；如果仍出现
无 `Drv Event` 的空醒，需要继续抓该 event socket 收到的 `nlmsg_type/cmd/group`。

#### build38：复核重复旧镜像日志

本轮再次提供的 H7S7 日志仍是同一个旧 app 镜像：

```text
stm32h7s78-dk: image=app ... Jun  9 2026 19:54:13
```

因此该日志不能验证 build35/build36/build37 的新改动。按命令返回统计：

```text
cmd=12 ret=-67 count=6   NL80211_CMD_DEL_KEY
cmd=50 ret=-95 count=1   NL80211_CMD_GET_SURVEY
cmd=54 ret=-95 count=1   NL80211_CMD_FLUSH_PMKSA
cmd=79 ret=-95 count=1   NL80211_CMD_SET_REKEY_OFFLOAD
```

分类如下：

- `DEL_KEY -67`：对应 Linux `-ENOLINK`，wpa_supplicant 启动阶段清理未连接 key 时出现，
  是 Linux 对齐行为。
- `GET_SURVEY -95`：旧镜像未包含 build35 的 `dump_survey`，新镜像应返回 0。
- `FLUSH_PMKSA -95`：旧镜像未包含 build35 的 `flush_pmksa`，新镜像应返回 0。
- `SET_REKEY_OFFLOAD -95`：当前 ESP32C5 firmware 没有 GTK rekey offload，Linux driver
  可返回 `-EOPNOTSUPP`，wpa_supplicant 会使用用户态 rekey。

该日志中基础链路已经完整跑通：

```text
NL80211_CMD_TRIGGER_SCAN ret=0
NL80211_CMD_GET_SCAN ret=0
NL80211_CMD_AUTHENTICATE ret=0
NL80211_CMD_ASSOCIATE ret=0
NL80211_CMD_NEW_KEY ret=0
NL80211_CMD_SET_STATION ret=0
WPA: Key negotiation completed
CTRL-EVENT-CONNECTED
ifconfig wlan0 dhcp
ping baidu.com -> 10 packets transmitted, 10 received, 0% packet loss
```

日志末尾的空：

```text
nl80211: Event message available
```

仍发生在旧镜像中。build37 已修正 route multicast 后追加 `NLMSG_DONE` 并跨协议投递的根因。
下一次必须先确认 app 镜像时间晚于 build37，再根据是否仍存在空醒决定是否继续抓
`nlmsg_type/cmd/group`。

#### build39：2026-06-10 新镜像完整连接验证和日志清理

本轮 H7S7 日志已确认烧录的是新 app 镜像：

```text
stm32h7s78-dk: image=app ... Jun 10 2026 02:14:22
```

相比旧镜像，nl80211 初始化已经完整推进，不再卡在 generic netlink/cache、
`GET_INTERFACE`、`GET_WIPHY` 或 `REGISTER_FRAME` 阶段。关键命令链路：

```text
NL80211_CMD_GET_INTERFACE        ret=0
NL80211_CMD_GET_PROTOCOL_FEATURES ret=0
NL80211_CMD_GET_WIPHY            ret=0
NL80211_CMD_SET_INTERFACE        ret=0
NL80211_CMD_REGISTER_FRAME       ret=0
NL80211_CMD_TRIGGER_SCAN         ret=0
NL80211_CMD_AUTHENTICATE         ret=0
NL80211_CMD_ASSOCIATE            ret=0
NL80211_CMD_NEW_KEY              ret=0
NL80211_CMD_SET_STATION          ret=0
```

用户态 WPA2-PSK 状态机已经完成：

```text
WPA: Key negotiation completed with 68:dd:b7:9b:60:a5 [PTK=CCMP GTK=CCMP]
CTRL-EVENT-CONNECTED - Connection to 68:dd:b7:9b:60:a5 completed
EAPOL authentication completed - result=SUCCESS
```

数据面也已经验证：

```text
ifconfig wlan0 dhcp
ping baidu.com
10 packets transmitted, 9 received, 10% packet loss
```

这说明当前完整链路已经跑通：

```text
wpa_supplicant
  -> libnl
  -> AF_NETLINK/NETLINK_GENERIC
  -> nl80211
  -> cfg80211
  -> ESP-Hosted NG Linux-style host driver
  -> NuttX AF_PACKET/EAPOL + Ethernet data path
  -> NuttX SPI4 transport
  -> ESP32C5 ESP-Hosted NG firmware
  -> real Wi-Fi AP
  -> DHCP/DNS/IP traffic
```

当前日志里仍可见的两个非 0 返回：

```text
NL80211_CMD_DEL_KEY             ret=-67
NL80211_CMD_SET_REKEY_OFFLOAD   ret=-95
```

分类如下：

- `DEL_KEY -67`：Linux 对齐的 `-ENOLINK`。这是 wpa_supplicant 初始化阶段清理旧 key
  时可能出现的正常结果，不阻塞后续连接。
- `SET_REKEY_OFFLOAD -95`：Linux 对齐的 `-EOPNOTSUPP`。GTK rekey offload 是驱动/固件
  可选能力；当前 ESP32C5 ESP-Hosted NG firmware 没有暴露该 offload，wpa_supplicant 会
  保持用户态 rekey，不影响普通 WPA2-PSK 连接。

本轮还清理了 Linux vendor drop 中用于 bring-up 的临时输出，避免运行日志被非原生调试信息污染：

```text
nuttx/wireless/ieee80211/cfg80211/mlme.c
  清理 REGISTER_FRAME / cfg80211_rx_mgmt_ext 临时 nuttx_hwsim_debugf。

nuttx/wireless/ieee80211/cfg80211/core.c
  清理 rdev lookup / wiphy_work 临时 nuttx_hwsim_debugf。

nuttx/wireless/ieee80211/cfg80211/nl80211_metadata.c
  清理 nl80211_init() 裸 printf。

nuttx/wireless/ieee80211/cfg80211/nl80211.c
  清理 wdev/rdev lookup、supported commands、START_AP、pre_doit、
  set_station/frame_tx_status 等 bring-up 阶段的临时 nuttx_hwsim_debugf。
```

对齐原则：

- Linux 核心已有、NuttX 缺失的 Unix/socket/netlink/packet 语义，应在 NuttX 原生层补齐。
- Linux 里也属于驱动/固件可选能力的功能，不在 NuttX 核心伪造支持；应按 Linux 返回
  `-EOPNOTSUPP`，让 wpa_supplicant 使用标准降级路径。
- 后续如果要消除 `SET_REKEY_OFFLOAD -95`，需要 ESP32C5 firmware/ESP-Hosted NG 协议真正
  增加 GTK rekey offload command 与事件，不应只在 host 侧返回成功。

#### build40：2026-06-10 M4 后 reason 204 与 PTK install race 修正

最新 H7S7/ESP32C5 对照日志显示，链路已经不是卡在 nl80211 初始化、扫描、认证或关联阶段。
当前失败轮可以推进到：

```text
RX message 3 of 4-Way Handshake
WPA: Sending EAPOL-Key 4/4
ESP32C5: Host EAPOL DATA -> STA len=113 connected=0 assoc=1
ESP32C5: STA disconnected [204]
WPA: Installing PTK to the driver
nl80211: NEW_KEY
cmd_add_key: drop add_key ... because firmware already reported disconnect reason=204
```

这里的根因不是 `NEW_KEY` 路径缺失，而是 host 侧把 deferred disconnect 当成了硬断开状态：

- `EVENT_STA_DISCONNECT reason=204` 早于 PTK install 到达。
- host 将 `disconnect_event_pending` 置位后，在 `cmd_add_key()` 中直接返回 `-ENOTCONN`。
- wpa_supplicant 因此把 host 人为制造的 key install 失败解释成 `WRONG_KEY`。

对齐 Linux 后的行为：

- `disconnect_event_pending` 只表示“disconnect event 暂缓上报 userspace”，不能阻止
  `NL80211_CMD_NEW_KEY` 进入 driver command path。
- `cmd_add_key()` 即使看到 deferred reason 204，也继续把 key command 送到 ESP32C5 firmware。
- `cmd_add_key()` 不再把所有失败折叠成 `-EINVAL`，而是返回 `wait_and_decode_cmd_resp()` 解析出的
  Linux 风格错误码，例如 `-EBUSY`、`-EOPNOTSUPP`、`-EINVAL`、`-EIO` 或 timeout。
- pairwise key 安装成功后清理 deferred disconnect 标记，避免旧事件污染后续 GTK/data path。

本轮代码修改：

```text
nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cmd.c
  process_disconnect_event():
    reason 204 before PTK install only defers userspace disconnect.

  cmd_add_key():
    allow add_key after deferred disconnect.
    return the real firmware/command error.
    clear deferred disconnect after pairwise key install succeeds.
```

本轮重新构建通过：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin              size: 57164 bytes
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin  size: 1818548 bytes
```

下一轮板测判读：

```text
期望:
  cmd_add_key idx=0 pairwise=1 -> ret=0
  NL80211_CMD_NEW_KEY ret=0
  WPA: Key negotiation completed
  CTRL-EVENT-CONNECTED

如果仍失败:
  看 cmd_add_key 返回的是固件错误还是 command timeout。
  如果 M4 send 仍出现 unlock_ms/total_ms 约 7s，继续追 esp_linux_if_exchange()
	  释放锁后的调度/串口日志/工作队列阻塞，而不是再在 key install 上做本地拒绝。
	```

#### build41：2026-06-10 新镜像完整通过与剩余返回码复核

本轮 H7S7 日志确认使用的是更新后的 app 镜像：

```text
stm32h7s78-dk: image=app ... Jun 10 2026 02:31:15
```

复测结果显示，之前缺失的 NuttX/Linux 语义已经生效：

```text
NL80211_CMD_GET_INTERFACE          ret=0
NL80211_CMD_GET_PROTOCOL_FEATURES  ret=0
NL80211_CMD_GET_WIPHY              ret=0
NL80211_CMD_SET_INTERFACE          ret=0
NL80211_CMD_REGISTER_FRAME         ret=0
NL80211_CMD_FLUSH_PMKSA            ret=0
NL80211_CMD_TRIGGER_SCAN           ret=0
NL80211_CMD_GET_SURVEY             ret=0
NL80211_CMD_AUTHENTICATE           ret=0
NL80211_CMD_ASSOCIATE              ret=0
NL80211_CMD_NEW_KEY                ret=0
NL80211_CMD_SET_STATION            ret=0
```

用户态和数据面也完整通过：

```text
WPA: Key negotiation completed with 68:dd:b7:9b:60:a5 [PTK=CCMP GTK=CCMP]
CTRL-EVENT-CONNECTED - Connection to 68:dd:b7:9b:60:a5 completed
EAPOL authentication completed - result=SUCCESS
ifconfig wlan0 dhcp
ping baidu.com
10 packets transmitted, 10 received, 0% packet loss
```

这轮日志里剩余两个非零返回：

```text
NL80211_CMD_DEL_KEY             ret=-67
NL80211_CMD_SET_REKEY_OFFLOAD   ret=-95
```

复核 Linux 源码后的分类：

- `DEL_KEY ret=-67`：这是 Linux 对齐的 `-ENOLINK`。`third/linux-7.0.10/net/wireless/nl80211.c`
  中 `nl80211_del_key()` 会先走 `nl80211_key_allowed()`；station 尚未 connected 且没有
  `NL80211_EXT_FEATURE_ASSOC_FRAME_ENCRYPTION` 时返回 `-ENOLINK`。如果已经允许删 key，
  再进入 mac80211 的 `ieee80211_del_key()`，不存在的 key 才返回 `-ENOENT`。因此
  wpa_supplicant 初始化阶段清理旧 key 的 `-67` 不应强行改成成功。
- `SET_REKEY_OFFLOAD ret=-95`：这是 Linux 对齐的 `-EOPNOTSUPP`。`third/linux-7.0.10/net/wireless/nl80211.c`
  的 `nl80211_set_rekey_data()` 在 driver 没有 `set_rekey_data` ops 时返回
  `-EOPNOTSUPP`；`third/linux-7.0.10/net/mac80211/cfg.c` 也按同样语义处理。当前
  ESP32C5 ESP-Hosted NG firmware 没有 GTK rekey offload command/事件，host 侧不能伪造
  offload 成功。wpa_supplicant 会保留用户态 rekey，普通 WPA2-PSK 连接不受影响。

事件路径也已经从“空醒”变成正常行为：日志中的 `nl80211: Event message available`
后续能看到 scan/auth/assoc/connect 等真实 driver event，不再是之前没有 `Drv Event`
跟随的异常 wakeup。

本轮结论：

- `AF_NETLINK/NETLINK_GENERIC`、nl80211 family/cache、event socket、route socket、
  rfkill query、operstate、`REGISTER_FRAME`、`GET_WIPHY` dump、`GET_SURVEY` dump、
  `FLUSH_PMKSA`、`AF_PACKET` EAPOL bind/filter、EAPOL RX/TX 和 Ethernet/IP 数据面均已跑通。
- 当前日志没有新的必须靠 NuttX 原生实现补齐的 Linux 行为缺口。
- 后续若要让 `SET_REKEY_OFFLOAD` 返回 0，必须先扩展 ESP32C5 firmware 和 ESP-Hosted NG
  host/firmware 协议，真正实现 GTK rekey offload；只改 NuttX 返回码会破坏 Linux 语义。

#### 当前临时实现和待收敛点

下面只列真正需要后续收敛的项目；Linux 对齐的 `-ENOLINK`、`-EOPNOTSUPP` 不算临时修复。

1. `esp_cfg80211_del_key()` 仍是空实现。

   位置：

   ```text
   nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cfg80211.c
   ```

   当前函数直接 `return 0`，这来自 ESP-Hosted NG 上游 host driver。当前连接流程中，
   wpa_supplicant 启动阶段的 `DEL_KEY ret=-67` 在 nl80211 层按 Linux 语义提前返回，
   所以没有走到 driver；PTK/GTK install 已通过 `add_key` 成功。但从完整 Linux 行为看，
   已连接后的 key 删除应接到 `cmd_del_key()` 或至少维护 host key 状态，不能长期空成功。

2. BT/BLE 是 stub。

   位置：

   ```text
   nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_bt_stub.c
   ```

   ESP32C5 firmware 已报告 `BT/BLE`、`HCI over SPI`、`BLE only` 能力，但当前
   `esp_init_bt()`、`esp_deinit_bt()` 和 HCI counter update 都是 no-op。WLAN 主链路不依赖
   它；如果后续验证 BLE，需要补 NuttX HCI-over-SPI adapter。

3. TX power 能力还有两个临时边界。

   位置：

   ```text
   nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cfg80211.c
   ```

   当前 `MAX_TX_POWER_MBM`/`MIN_TX_POWER_MBM` 是 host 侧常量，文件内也标了
   `TODO get MAX_TX_POWER_MBM from Firmware for future chips`。另外 `get_tx_power`
   在 `ESP_INIT_DONE` 前只返回本地缓存，避免启动期重入 command path；初始化完成后会再访问
   firmware。后续应从 firmware capability 或 chip profile 获取真实范围，并在 init done
   后做一次明确同步。

4. 固件时间同步暂未启用。

   位置：

   ```text
   nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cfg80211.c
   ```

   `cmd_update_fw_time()` 当前包在 `#ifdef TODO` 下，没有参与初始化。普通 STA/WPA2 数据面不受影响；
   但如果后续验证 WoWLAN、host sleep、日志时间戳或固件侧依赖 RTC 的功能，需要按 Linux/ESP-Hosted
   语义补上 NuttX 时间同步。

5. wiphy 能力数字仍有待从固件能力收敛。

   位置：

   ```text
   nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cfg80211.c
   ```

   `max_scan_ssids`、`max_scan_ie_len`、`max_sched_scan_ssids` 当前是 host 侧固定值，代码中保留
   `TODO: check and finalize the numbers`。现有扫描、关联已通过；后续应对照 ESP32C5 firmware
   capability 或 ESP-Hosted NG 协议定义修正。

6. SPI exchange 慢路径日志仍是 bring-up 诊断。

   位置：

   ```text
   nuttx/drivers/wireless/esp_hosted_ng/esp_hosted_ng_linux_if.c
   ```

   `esp_linux_if_exchange()` 对超过 100 ms 的交换打印 `esp_hosted_ng: if exchange slow ...`。
   这帮助定位过 M4/EAPOL 时序问题。链路稳定后可以保留为 debug Kconfig/trace 开关，或改成
   更低频的统计项，避免正常运行日志噪声。

7. netlink/mac80211/hwsim 命名的调试开关应统一命名。

   位置：

   ```text
   nuttx/net/netlink/netlink_sockif.c
   nuttx/wireless/ieee80211/include/linux/cfg80211_compat.h
   nuttx/wireless/ieee80211/genetlink_bridge.c
   nuttx/wireless/ieee80211/mac80211/*.c
   ```

   当前大量 `hwsim-debug` / `nuttx_hwsim_debugf()` 已受 `CONFIG_WL_NUTTX_HWSIM_DEBUG`
   保护，默认不输出，不影响功能。但其中一部分已经服务于通用 `nl80211/cfg80211/mac80211`
   移植，不再只属于 hwsim。后续应改名为通用 WLAN/Linux-stack debug 开关，或把不再需要的
   bring-up trace 删除。

8. AF_PACKET 还有性能/完整性待补齐项。

   位置：

   ```text
   nuttx/net/pkt/pkt.h
   nuttx/net/pkt/pkt_input.c
   nuttx/net/pkt/pkt_recvmsg.c
   ```

   当前已经支持 wpa_supplicant 需要的 `AF_PACKET`、EAPOL、`SO_ATTACH_FILTER`、
   `MSG_ERRQUEUE` TX status 和 poll 语义。剩余 TODO 包括 `PACKET_MMAP`、shared IOB clone
   优化，以及 `sockaddr_ll` 接收路径整理。这些不是当前连接问题的临时绕过，但属于
   Linux AF_PACKET 行为继续补齐的 backlog。

### P2：BT/BLE path

ESP32C5 固件能力中包含 BT/BLE：

```text
BT/BLE
HCI over SPI
BLE only
```

当前 NuttX 侧主要关注 WLAN，BT 侧暂用 stub。后续如需 BLE，要补 HCI over SPI 的 NuttX adapter。

## 关键文件索引

ESP32C5 固件：

```text
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter/main/spi_slave_api.c
third/esp-hosted/esp_hosted_ng/esp/esp_driver/network_adapter/sdkconfig
```

ESP-Hosted NG 上游 host：

```text
third/esp-hosted/esp_hosted_ng/host
```

NuttX ESP-Hosted NG driver：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng
```

NuttX cfg80211/nl80211：

```text
FeatherCore_ESP/nuttx/wireless/ieee80211
```

wpa_supplicant/libnl：

```text
FeatherCore_ESP/apps/wireless/wifi
```

STM32H7S78-DK board：

```text
FeatherCore_ESP/nuttx/boards/arm/stm32h7rs/stm32h7s78-dk
```

H7S78-DK 构建脚本：

```text
FeatherCore_ESP/tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng.sh
```

## 2026-06-10 hwsim 回归后 ESP32C5 关联异常

现象：

```text
wpa_supplicant -> nl80211 -> cfg80211 初始化成功
scan 成功，BSS 表中能看到目标 AP
auth 命令发送成功，ESP32C5 固件收到 Auth request
第一次 associate 在 H7S7 侧返回 -ENOENT
第二次 associate 能发送到 ESP32C5，但固件随后 reason=4 断开
```

日志关键点：

```text
nl80211: Netlink command cmd=38 ... completed ret=-2
wlan0: nl80211: MLME command failed (assoc): ret=-2 (No such file or directory)
nuttx: process_disconnect_event: Disconnect event for ssid Wakaka [reason:2]
```

根因：

ESP-Hosted host 侧 `process_disconnect_event()` 中沿用了上游 Linux host driver
里的 `cfg80211_bss_flush()`。在 NuttX 当前移植里，ESP 固件可能先上报一次 auth
阶段 disconnect，再由 wpa_supplicant 继续处理已经收到的 auth response 并发起
associate。此时如果 host 侧把 wiphy 的 BSS cache 整体清空，`nl80211_associate()`
会在 `nl80211_assoc_bss()` 中找不到刚扫描到的 AP，进而返回 `-ENOENT`。

Linux cfg80211 的语义不是“每次 disconnect 都清空全部 scan BSS”。BSS cache 应由
scan aging、显式 flush 或 regulatory/device teardown 管理；普通 deauth/disconnect
不能破坏随后同一轮 SME 流程需要使用的 BSS 引用。

修复：

```text
FeatherCore_ESP/nuttx/drivers/wireless/esp_hosted_ng/upstream_host/esp_cmd.c
```

删除 disconnect 路径里的全局 `cfg80211_bss_flush()`，保留：

```text
esp_port_close()
local disconnect -> CFG80211_DISCONNECTED()
remote disconnect -> dummy deauth MLME event
```

这样对齐 Linux cfg80211 行为：disconnect 只改变连接状态和上报 MLME 事件，不破坏
scan BSS cache。该修复也避免 hwsim/mac80211 回归中为了更严格 BSS 生命周期管理
而影响 ESP-Hosted 的 STA 关联路径。

### 进一步澄清：首次 reason 15 不是路由器残留断链流程

新的 H7S7/ESP32C5 对照日志显示，`cfg80211_bss_flush()` 修复后，第一次连接已经不再
失败于 `associate ret=-ENOENT`：

```text
scan -> auth -> assoc 均返回 0
wlan0: WPA: Key negotiation completed ... [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED
EAPOL authentication completed - result=SUCCESS
ESP32C5: wifi:connected with Wakaka
ESP32C5: Wifi Station Connected event!!
```

随后失败发生在已经进入 connected/4-way 之后。AP 持续重发 EAPOL-Key message 3/4，
H7S7 上的 `wpa_supplicant` 也持续回复 message 4/4，并多次打印
`Key negotiation completed`。最后 ESP32C5 固件上报：

```text
STA disconnected [15]
```

`reason 15` 是 4-way handshake timeout。这里不能解释成“路由器识别到旧连接后主动
清理一次”，因为日志里没有看到一个独立、正常的 AP 断链清理流程；更准确的结论是：

- 首轮 scan/auth/assoc 已经成功。
- 首轮 EAPOL 不是完全不通，M1/M2/M3/M4 都能在 H7S7 和 ESP32C5 日志中看到。
- AP 仍重发 M3，说明 AP 没有及时收到或没有接受某次 M4。
- H7S7 本地状态机已经认为 key negotiation 完成，但 ESP/AP 最终以 reason 15 拆链。

当前优先怀疑点：

1. `esp_hosted_ng` host 侧 EAPOL TX 返回时序仍偏慢。日志中仍可见：

   ```text
   esp_hosted_ng: if exchange slow txlen=128 ... total_ms=160/360/640 ret=0
   ```

   其中 SPI transaction 本身不一定慢，慢点出现在 exchange 调用整体返回路径。对
   WPA2 来说，M4 返回过慢会导致 `wpa_supplicant` 的 key install、firmware TX 完成和
   AP 的 replay/timeout 窗口出现竞态。

2. ESP32C5 firmware 的连接状态切换与 host EAPOL DATA 下发存在边界窗口。日志中先出现
   `connected=0 assoc=1` 的 M2/M4，下发 key 后变成 `connected=1 assoc=0`，之后 AP
   继续重发 M3，host 又继续发送 M4。需要继续确认 `connected=1 assoc=0` 状态下的
   EAPOL DATA 是否仍被 ESP-IDF Wi-Fi 栈按控制端口帧稳定发送到 AP。

3. 后续单次 `FW_SPI: Drop invalid pkt: len=43 offset=7` 发生在连接成功较久之后，暂时
   不是首轮 reason 15 的直接证据，但如果重复出现，需要单独检查 SPI 帧边界/同步恢复。

下一步验证应避免修改 ESP32C5 功能逻辑，只保留必要日志：

- 在 H7S7 host 侧给 EAPOL TX 加 message 类型标记和返回耗时，区分 M2 与 M4。
- 在 ESP32C5 固件侧仅记录 `esp_wifi_internal_tx()` 对 EAPOL DATA 的返回值，确认 M4
  是否被 Wi-Fi 栈接受。
- 对比第一次失败轮和第二次成功轮的 M4 下发耗时、`esp_wifi_internal_tx()` 返回值、
  AP 是否继续重发 M3。

### 与 hwsim `__be16` 修复的关联

用户指出：修复 hwsim/mac80211 之前，ESP32C5 没有出现“第一次 connected 后 reason 15、
第二轮才稳定”的现象。这个线索成立，说明优先怀疑点应从路由器/ESP 固件转回
NuttX 的共用 Linux 兼容层。

hwsim 回归修复的核心是：Linux `skb->protocol` 是 `__be16`，所有 control-port/EAPOL
比较都应使用 `cpu_to_be16(ETH_P_*)` 后的值。此前只修正了 lower-to-Linux TX bridge：

```text
netdevice_compat.c:
  skb->protocol = cpu_to_be16(parsed_ethertype)
```

但 ESP-Hosted RX 路径不是从 `netdevice_compat.c` 进入，而是在
`drivers/wireless/esp_hosted_ng/upstream_host/main.c` 中直接调用：

```text
skb->protocol = eth_type_trans(skb, priv->ndev);
netif_rx(skb);
```

因此 `eth_type_trans()` 本身也必须返回 Linux `__be16`。当前补齐：

```text
cfg80211_compat.h:
  proto = (skb->data[12] << 8) | skb->data[13];
  return cpu_to_be16(proto);
```

这不是 ESP 特判，而是把 hwsim 修复的同一条 Linux 语义补完整。否则 hwsim 的某些路径
用 `cpu_to_be16()`，ESP 的直接 RX 路径却保留 host-order `0x888e`，会造成 EAPOL/
packet socket/cfg80211 事件链中的协议号混用。该混用可以解释 hwsim 修复后 ESP 首轮
EAPOL 行为发生变化。

### 最新定位：EAPOL 路径必须回到 Linux `AF_PACKET/SOCK_DGRAM` 语义

2026-06-10 07:04 左右的新日志显示，问题点继续收敛：

```text
scan -> auth -> assoc 均成功
AP -> STA: EAPOL-Key message 1/4
H7S7 -> ESP32C5: EAPOL-Key message 2/4, payload 121, Ethernet frame 135
AP -> STA: 继续重发 message 1/4
最终 reason 15
```

这一次没有进入 message 3/4，也没有进入 key install。也就是说当前异常不是“AP 没收到
M4”，而是“AP 没接受或没看到 M2”。ESP32C5 固件侧能看到：

```text
Host EAPOL DATA -> STA len=135 connected=0 assoc=1
```

长度 `135 = Ethernet header 14 + EAPOL payload 121`，说明 H7S7 已经把带二层头的
EAPOL M2 交给了 ESP firmware。需要继续保证这个帧完全符合 Linux packet socket 语义。

排查发现 `wpa_supplicant/src/l2_packet/l2_packet_linux.c` 里存在 NuttX 专用特例：

```text
protocol == ETH_P_PAE 时强行使用 SOCK_RAW
发送时手工拼 Ethernet header
接收时再猜测并剥 Ethernet header
```

这条路径是早期为了绕过 NuttX `AF_PACKET/SOCK_DGRAM` 能力不足的临时实现，但它不再符合
“Linux 有的，NuttX 原生对齐 Linux”的原则。Linux 原生行为是：

- `AF_PACKET/SOCK_DGRAM`：用户态读写 payload，内核根据 `sockaddr_ll` 构造/剥离二层头。
- `AF_PACKET/SOCK_RAW`：用户态读写完整二层帧，内核不再自动补 Ethernet header。

本次修复：

```text
apps/wireless/wifi/wpa_supplicant-2.11/src/l2_packet/l2_packet_linux.c
  - 删除 NuttX raw-EAPOL 特例。
  - EAPOL 在 l2_hdr=0 时和 Linux 一样使用 SOCK_DGRAM。
  - 删除 recvfrom 后基于 payload 猜测/剥离 Ethernet header 的逻辑。

nuttx/net/pkt/pkt_sendmsg_unbuffered.c
nuttx/net/pkt/pkt_sendmsg_buffered.c
  - 补齐 Linux AF_PACKET TX 语义。
  - 只有 SOCK_DGRAM + sockaddr_ll 才由内核构造 Ethernet header。
  - SOCK_RAW 即使带 sockaddr_ll，也认为用户 buffer 已经包含完整二层帧。
```

这不是针对 ESP 的绕过，而是把 NuttX packet socket 行为补齐到 Linux。这样 hwsim/mac80211
和 ESP-Hosted NG 共用同一套 `AF_PACKET` 规则，避免为了 hwsim 修一个方向、ESP 又走另一套
临时 raw-EAPOL 规则。

构建结果：

```text
FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng.bin
size:   1818372 bytes
sha256: d0371f106bb10d73130372e9e025c2ac783d9d6b2df78ab810283032f2718f88

FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin
sha256: a779ad41a83a418c2a1d3d8c2f4f124b535d53f364fe0f8a663c74a2fc0ec14a
```

下一轮板上验证重点：

- `l2_packet_receive` 的 RX 长度应保持为 EAPOL payload 长度，而不是完整 Ethernet frame。
- `Host EAPOL DATA -> STA len=135` 仍应出现，但 AP 应从 M1 继续推进到 M3。
- 若仍失败，需要在 ESP32C5 固件侧只增加日志，打印 `esp_wifi_internal_tx()` 对该 EAPOL
  frame 的返回值，不改变 firmware 功能逻辑。
