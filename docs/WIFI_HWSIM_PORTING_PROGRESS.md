# Wi-Fi hwsim porting progress

## Target

The target path in FeatherCore is aligned with `/home/uan/Feather-develop-WIFI/hwsim`:

```text
hostapd-2.11 / wpa_supplicant-2.11
  -> nl80211
  -> cfg80211
  -> mac80211
  -> mac80211_hwsim
```

The active porting areas are:

```text
apps/wireless/wifi
nuttx/wireless/ieee80211
nuttx/drivers/wireless/virtual
```

## Current Status

Last updated: 2026-06-10

The implementation is now a Linux-derived wireless stack port running inside
NuttX sim, with NuttX-side glue and compatibility stubs. The AP/STA path has
now been verified through association, ICMP ping, and NuttX iperf over the
imported hwsim/mac80211 data path. A three-process AP + STA1 + STA2 run has
also been verified with both STAs associated and passing traffic. Latest
current-session open-auth proof with `nuttx-sim-ap`, `nuttx-sim-sta1`, and
`nuttx-sim-sta2`: AP reached `AP-ENABLED`; both STAs reached
`CTRL-EVENT-CONNECTED`; AP logged both `AP-STA-CONNECTED` events; STA1->STA2
ping passed 5/5, STA2->STA1 ping passed 5/5, STA1->STA2 iperf ran for
12.04 s at ~0.58 Mbits/sec, and STA2->STA1 iperf ran for 12.04 s at
~1.18 Mbits/sec. Evidence: `/tmp/hwsim-sta2sta-ap.log`,
`/tmp/hwsim-sta2sta-sta1.log`, and `/tmp/hwsim-sta2sta-sta2.log`.
S1G / 802.11ah open-auth runtime now also reaches AP + STA1 + STA2 association
on 912 MHz, STA-to-AP ping, and sequential STA1<->STA2 ping/iperf in both
directions through the same imported hwsim/mac80211 data path.
WPA2-Personal TKIP, mixed TKIP/CCMP, and WPA2/WPA3 transition mode are now
verified with AP + STA1. Transition mode has been checked both with an SAE STA
and with a legacy WPA-PSK-only STA.
WPA3-SAE H2E is now verified with AP + STA1 using `sae_pwe=1`.
WPA3-Personal SAE AP + STA1 + STA2 runtime has also been verified with SAE
commit/confirm, the WPA 4-way handshake, ping, iperf, and sequential
STA-to-STA traffic in both directions.
WPA2-Enterprise now has EAP-PSK, EAP-TLS, PEAP/MSCHAPv2, and TTLS inner
PAP/CHAP/MSCHAP/MSCHAPv2 AP + STA1 proofs. EAP-PSK exercises method 47
through RSN 4-way handshake, ping, and STA-to-AP iperf. EAP-TLS exchanges
certificates, completes EAP method 13, installs CCMP PTK/GTK, and passes
bidirectional ping/iperf plus post-iperf ping. PEAP/MSCHAPv2 completes outer
method 25, server certificate validation, tunneled inner MSCHAPv2 success,
EAP-TLV success, CCMP keying, and bidirectional ping/iperf. All four TTLS
inner variants complete outer method 21, server certificate validation, EAP
success, CCMP keying, and bidirectional ping/iperf.
Suite-B/192-bit Enterprise now passes the current simulator runtime profile:
AP + STA1 select `WPA-EAP-SUITE-B-192`, `GCMP-256`, and `BIP-GMAC-256`,
PMF-required association succeeds, RSA3072 EAP-TLS reaches
`CTRL-EVENT-EAP-SUCCESS`, the RSN 4-way handshake completes with
`PTK=GCMP-256 GTK=GCMP-256`, and STA-to-AP ping/iperf pass. Strict CNSA TLS
profile coverage is still pending because this test uses the imported internal
TLS path's RSA3072/DHE profile rather than an ECDHE/ECDSA SHA384/GCM profile.

## 2026-06-10 Full PASS Replay

The current tree was replayed against all items that were previously recorded
as `PASS` in the validation documents.

Role images were rebuilt sequentially because all simulator roles share the
same generated NuttX `.config` and include state. Current rebuild logs:

```text
/tmp/hwsim-build-current-sta3.log       -> rc=0
/tmp/hwsim-build-current-p2p1.log       -> rc=0
/tmp/hwsim-build-current-p2p2.log       -> rc=0
/tmp/hwsim-build-current-ap1.log        -> rc=0
/tmp/hwsim-build-current-ap2.log        -> rc=0
/tmp/hwsim-build-current-dynps-ap.log   -> rc=0
/tmp/hwsim-build-current-dynps-sta1.log -> rc=0
```

The standard AP + STA1 matrix was replayed with
`tools/firmware/sim/validate-hwsim-pass-matrix.py`. The broad matrix run
completed 39/41 cases and exposed two runner mapping mistakes for OWE groups
20 and 21. The stack itself was not failing: the AP configs use SSIDs
`nuttx-hwsim-owe-g20` and `nuttx-hwsim-owe-g21`, while the runner was
incorrectly using the generic OWE STA config. After changing the runner to use
`wpa-owe-g20.conf` and `wpa-owe-g21.conf`, both cases passed. A standalone
`ft-psk` case was also added to the runner and passed.

Effective current result for previously recorded `PASS` items: no remaining
current `FAIL`. The only failed rows found in this replay were OWE group 20/21
runner-config mistakes, and both have clean PASS reruns.

Current AP + STA1 summary logs:

```text
/tmp/hwsim-pass-matrix-20260610-r2/summary.md
/tmp/hwsim-pass-owe-rerun/summary.md
/tmp/hwsim-pass-ft-rerun/summary.md
/tmp/hwsim-pass-smoke-r2/summary.md
```

The matrix replay now covers open auth, WPA2 PSK variants, PMF, OCV, FILS
first association, WPA3 SAE/H2E/SAE-PK, transition mode, OWE groups 19/20/21,
EAP-PSK, EAP-TLS, PEAP, TTLS inner PAP/CHAP/MSCHAP/MSCHAPv2, Suite-B-192,
HS20, WNM base association/data, 802.11b/g/a, HT20/HT40, VHT20/VHT80/
VHT160/VHT80+80, HE 2.4 GHz, HE 5 GHz, HE 6 GHz SAE/OWE, S1G, and single-BSS
FT-PSK.

The multi-role and action-frame flows were replayed separately:

| Flow | Result | Evidence |
| --- | --- | --- |
| Open AP + STA1 + STA2 + STA3 | PASS | `/tmp/hwsim-current-open-3sta/` |
| P2P GO/client 60-second soak | PASS | `/tmp/hwsim-current-p2p-soak60-r2/` |
| WPS PBC | PASS | `/tmp/hwsim-current-wps-pbc/` |
| DPP-PSK provisioning | PASS | `/tmp/hwsim-current-dpp-psk/` |
| FT multi-BSS roam | PASS | `/tmp/hwsim-current-ft-roam/` |
| FILS cached reconnect with STA2 online | PASS | `/tmp/hwsim-current-fils-reconnect/` |
| A-MPDU ADDBA + A-MSDU aggregation | PASS | `/tmp/hwsim-current-aggregation/` |
| TWT automatic service-period data smoke | PASS | `/tmp/hwsim-current-twt-auto/` |
| Static power-save | PASS | `/tmp/hwsim-current-static-ps/` |
| Dynamic PS / MLO-style PS hooks | PASS | `/tmp/hwsim-current-dynps/` |
| WNM BSS-TM + WNM Sleep/TFS | PASS | `/tmp/hwsim-current-wnm/` |
| HE DCM RX-status proof | PASS | `/tmp/hwsim-current-dcm/` |

Issues found during the replay:

- The original runner treated absence of a literal `iperf exit` string as a
  failure even when the iperf client/server summary was already present. The
  runner now uses ping success plus iperf summary lines as the data-plane
  proof and waits briefly for server-side shutdown logs.
- OWE group 20/21 failures were caused by runner config mapping, not by the
  wireless stack.
- The first P2P automation attempt issued `p2p_connect` before
  `P2P-DEVICE-FOUND`; waiting for peer discovery made the 60-second soak pass.
- The aggregation one-off checker reported a false failure because it matched
  `rx ADDBA_RESP status=0` too literally. The real logs contain
  `rx ADDBA_RESP ... token=1 status=0 ...`, `tx operational`, A-MSDU proof,
  ping, and iperf.
- Manual TWT service-period gating still has AP-originated ARP timing
  sensitivity if traffic is started inside a non-warmed service period. The
  automatic TWT setup/period/teardown flow passes bidirectional ping and
  iperf, so the validated PASS item remains the automatic TWT data-smoke path.

The intended full path is being built in-tree:

```text
hostapd-2.11 / wpa_supplicant-2.11
  -> NuttX/libnl port
  -> imported Linux nl80211
  -> imported Linux cfg80211
  -> imported Linux mac80211
  -> imported Linux mac80211_hwsim
```

The older NuttX native `wifi_sim` lower-half path may still exist in the tree,
but it is not the target path for this effort.

Important distinction:

- The active userspace/control-path target is Linux-derived:
  `hostapd/wpa_supplicant -> libnl/nl80211 -> cfg80211 -> mac80211 ->
  mac80211_hwsim`.
- The current sim board hook is hybrid only at the boundary: NuttX still
  exposes a `netdev_lowerhalf`, but `virtual_hwsim_init()` no longer calls the
  NuttX native `wifi_sim_init()` fallback. It initializes the imported Linux
  compat stack and binds the NuttX lower-half to the imported Linux
  `struct net_device`.
- Therefore, demo-level `wifi_ap_demo` / `wifi_sta_demo` behavior exercises
  the Linux-derived netdev boundary, while the hostapd/wpa_supplicant runtime
  validation below is the stronger proof for the full management and data
  path.

Latest validation notes:

- This is not a pure from-scratch NuttX native Wi-Fi simulator. The current
  target is NuttX sim hosting an imported Linux wireless stack, with NuttX
  demos/glue around it.
- It is also no longer validating the old NuttX native `wifi_sim_init()`
  fallback. The sim board hook now binds the NuttX lower-half to the imported
  Linux compat `wlan0`, and AP/STA traffic has been verified through that
  bridge.
- AP startup has reached `AP-ENABLED` with an open `hostapd` config.
- STA `wpa_supplicant` now reaches `NL80211_CMD_TRIGGER_SCAN` successfully.
- Netlink sequence filtering was added on the userspace libnl path so stale
  ACK/error replies are no longer consumed by later nl80211 commands.
- The Linux-compat workqueue/timer path now has a NuttX workqueue-backed
  delayed-work implementation and a real `jiffies` variable refreshed from
  NuttX system ticks.
- `mac80211_hwsim` now defaults to `channels=2` in the NuttX port so it uses
  the hwsim channel-context ops, including `.hw_scan`.
- STA scan now enters `mac80211_hwsim_hw_scan()` and `hw_scan_work()`, completes
  both scan parts, and delivers `wlan0: Event SCAN_RESULTS (3) received`.
- Single-STA scan returns `0 BSSes` when no AP is running, as expected.
- Cross-process AP/STA scan has now progressed past `0 BSSes`. A temporary
  hostfs-backed shared medium writes AP beacon/probe-response state to
  `/h/hwsim-bss.bin`, and the STA hwsim scan path injects that BSS frame while
  scanning the matching channel. The same temporary medium also carries
  STA-to-AP authentication request frames in `/h/hwsim-frames.bin`.
- With separately launched `nuttx-sim-ap` and `nuttx-sim-sta1`, AP reaches
  `AP-ENABLED`, STA receives `1 BSSes`, adds BSSID `02:00:00:00:00:01`
  with SSID `nuttx-hwsim`, and starts `NL80211_CMD_AUTHENTICATE`.
- STA authentication and association now complete with separately launched
  `nuttx-sim-ap`, `nuttx-sim-sta1`, and `nuttx-sim-sta2`. The AP delivers
  management-frame TX status back to hostapd, hostapd authorizes the STAs, and
  data frames pass through mac80211 into the NuttX lower-half.
- AP/STA simulator roles now use distinct generated hwsim MAC addresses via
  `CONFIG_WL_NUTTX_HWSIM_RADIO_BASE`. AP is
  `02:00:00:00:00:01`, STA1 is `02:00:00:00:00:02`, and STA2 is intended to be
  `02:00:00:00:00:03`. A byte-order bug in the generated address
  (`02:00:00:00:01:00`) was fixed by storing the hwsim radio index in the last
  two octets of the MAC address.
- The NuttX-visible `wlan0` lower-half now binds to the imported Linux compat
  `wlan0` netdev:

  ```text
  netdevice_compat: bind lower wlan0 -> Linux wlan0 ifindex=1
  netdevice_compat: wlan0 up after=0x3
  ```

  This removes the old native `wifi_sim_init()` lower-half fallback from the
  hwsim path and makes NuttX TX/RX enter the Linux-derived netdev/mac80211
  boundary.

Verified on 2026-06-05:

- AP/STA association through the imported stack.
- Cross-process hwsim frame exchange between separately launched
  `nuttx-sim-ap`, `nuttx-sim-sta1`, and `nuttx-sim-sta2` using the current
  hostfs-backed shared medium.
- ICMP ping from STA1 and STA2 to AP over the imported hwsim/mac80211 data
  path.
- NuttX iperf TCP traffic from STA1 and STA2 to AP over the same data path.
- ICMP ping and NuttX iperf TCP traffic between STA1 and STA2 through the AP.
- 802.11b open-auth AP/STA runtime on channel 1 / 2412 MHz, including legacy
  mode AP startup plus ping and iperf traffic.
- 802.11g open-auth AP/STA runtime on channel 1 / 2412 MHz, including dedicated
  `hostapd-g.conf` / `wpa-g.conf`, legacy no-HT/no-VHT/no-HE chandef, ping, and
  bidirectional iperf.
- 802.11a open-auth AP/STA runtime on channel 36 / 5180 MHz after applying the
  hwsim custom world regulatory domain by default in NuttX sim.
- 802.11n HT20 open-auth AP/STA runtime on channel 6 / 2437 MHz, including
  hostapd-visible STA HT capabilities plus ping and iperf traffic.
- 802.11n HT40 open-auth AP/STA runtime on channel 6 with secondary channel
  10, including 40 MHz chandef setup plus ping and iperf traffic.
- 802.11ac VHT20 open-auth AP/STA runtime on channel 36 / 5180 MHz after
  enabling the hostapd/wpa_supplicant VHT build path.
- 802.11ac VHT80 open-auth AP/STA runtime on channel 36 / 5180 MHz with center
  frequency 5210 MHz after adding the required HT40 secondary-channel config
  and widening the NuttX sim hwsim regulatory rule.
- S1G / 802.11ah open-auth AP + STA1 + STA2 runtime on 912 MHz, including
  STA1 and STA2 association, STA1->AP ping 10/10, STA1->STA2 ping 10/10,
  STA2->STA1 ping 10/10, STA1->STA2 TCP iperf, and STA2->STA1 TCP iperf.
  The current proof uses hwsim RX-status compatibility mapping for S1G frames;
  it validates the simulated nl80211/cfg80211/mac80211/hwsim path, not real
  S1G PHY modulation.
- WPA3-Personal SAE AP/STA runtime on channel 6 / 2437 MHz, including SAE
  group 19 commit/confirm, AES-CMAC EAPOL MICs, PTK/GTK/IGTK install, ping
  with 0% loss, iperf traffic, and sequential STA1<->STA2 ping/iperf.
- WPA-EAP-SUITE-B-192 runtime on channel 1 / 2412 MHz, including
  `WPA-EAP-SUITE-B-192` RSN selection, `GCMP-256` pairwise/group cipher
  selection, `BIP-GMAC-256` management group cipher selection, PMF-required
  association, EAP-TLS success, RSN 4-way completion, ping with 0% loss, and
  STA-to-AP iperf.

Current verified state:

```text
./tools/firmware/sim/build-ap.sh -j8
  -> rc=0
  -> build/nuttx-sim-ap
  -> size: 22213768 bytes

./tools/firmware/sim/build-sta1.sh -j8
  -> rc=0
  -> build/nuttx-sim-sta1
  -> size: 26159688 bytes

./tools/firmware/sim/build-sta2.sh -j8
  -> rc=0
  -> build/nuttx-sim-sta2

./tools/firmware/sim/build-sta3.sh -j8
  -> rc=0
  -> build/nuttx-sim-sta3
  -> size: 27410072 bytes

./tools/firmware/sim/build-ap1.sh -j8
  -> rc=0
  -> build/nuttx-sim-ap1
  -> size: 23120544 bytes

./tools/firmware/sim/build-ap2.sh -j8
  -> rc=0
  -> build/nuttx-sim-ap2
  -> size: 23120544 bytes

./tools/firmware/sim/build-p2p1.sh -j8
  -> rc=0
  -> build/nuttx-sim-p2p1
  -> size: 26165504 bytes

./tools/firmware/sim/build-p2p2.sh -j8
  -> rc=0
  -> build/nuttx-sim-p2p2
  -> size: 26165504 bytes

Startup smoke:
  printf 'poweroff\n' | build/nuttx-sim-{sta3,ap1,ap2,p2p1,p2p2}
  -> rc=0 for all five roles
```

Runtime smoke test in `build/nuttx-sim-ap`:

```text
hostapd: ieee80211_linux_initialize ret=0
Allowed channel: mode=1 chan=1 freq=2412 MHz max_tx_power=0 dBm
  * freq=2412
nl80211-debug: start_ap parse chandef freq=2412
genl_bridge: complete family=nl80211 cmd=15 ret=0
wlan0: interface state UNINITIALIZED->ENABLED
wlan0: AP-ENABLED
```

The AP-side NuttX lower-half MAC now synchronizes from the imported Linux
`wlan0` netdev:

```text
netdevice_compat: sync lower wlan0 mac 02:00:00:00:00:01
netdevice_compat: bind lower wlan0 -> Linux wlan0 ifindex=1
hwsim-debug: i802_init hwaddr ok 02:00:00:00:00:01
```

This means cfg80211, mac80211, mac80211_hwsim, libnl, nl80211, and hostapd now
get far enough to start an AP BSS on `wlan0`.

Cross-process AP/STA runtime validation:

```text
AP:
hwsim-debug: i802_init hwaddr ok 02:00:00:00:00:01
wlan0: AP-ENABLED
hwsim-debug: nl80211 frame_tx_status cmd=60 ifindex=1 cookie=3 ack=1 len=46
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
hwsim-debug: mac80211 sta_apply_auth_flags done ... auth=1 assoc=1 authorized=1

STA:
netdevice_compat: sync lower wlan0 mac 02:00:00:00:00:02
hwsim-debug: injected shared BSS frame freq=2412 len=77 path=/h/hwsim-bss.bin
wlan0: Event SCAN_RESULTS (3) received
nl80211: Received scan results (1 BSSes)
wlan0: BSS: Add new id 0 BSSID 02:00:00:00:00:01 SSID 'nuttx-hwsim' freq 2412
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed [id=0 id_str=]
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

Three-process AP + STA1 + STA2 runtime validation:

```text
AP:
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
hwsim-debug: mac80211 sta_apply_auth_flags done ... sta=02:00:00:00:00:02 ... authorized=1
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
hwsim-debug: mac80211 sta_apply_auth_flags done ... sta=02:00:00:00:00:03 ... authorized=1

STA1:
netdevice_compat: sync lower wlan0 mac 02:00:00:00:00:02
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed [id=0 id_str=]
1 packets transmitted, 1 received, 0% packet loss, time 1010 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec

STA2:
netdevice_compat: sync lower wlan0 mac 02:00:00:00:00:03
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed [id=0 id_str=]
1 packets transmitted, 1 received, 0% packet loss, time 1010 ms
0.00-   5.05 sec   12730368 Bytes   20.17 Mbits/sec
```

Note: both STA logs showed an initial ARP wait timeout during the first ping,
but the same ping command then received the ICMP reply and ended with 0% packet
loss. This should be watched in longer stress runs.

STA-to-STA validation through AP:

```text
STA1 -> STA2:
PING 192.168.201.3 56 bytes of data
56 bytes from 192.168.201.3: icmp_seq=0 time=130.0 ms
56 bytes from 192.168.201.3: icmp_seq=1 time=50.0 ms
56 bytes from 192.168.201.3: icmp_seq=2 time=60.0 ms
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec    5701632 Bytes    9.03 Mbits/sec

STA2 -> STA1:
PING 192.168.201.2 56 bytes of data
56 bytes from 192.168.201.2: icmp_seq=0 time=130.0 ms
56 bytes from 192.168.201.2: icmp_seq=1 time=50.0 ms
56 bytes from 192.168.201.2: icmp_seq=2 time=60.0 ms
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec    5701632 Bytes    9.03 Mbits/sec
```

Note: both directions showed initial ARP wait timeout messages before the first
ICMP reply, but each ping command still completed with 0% packet loss.

Latest 2026-06-06 STA-to-STA rerun after the 64 MiB shared-medium default:

```text
AP:
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03

STA1:
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 5 192.168.201.3 -> 5 packets transmitted, 5 received, 0% packet loss
iperf -c 192.168.201.3 -p 5201 -t 10
  0.00-  12.04 sec     868352 Bytes    0.58 Mbits/sec

STA2:
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 5 192.168.201.2 -> 5 packets transmitted, 5 received, 0% packet loss
iperf -c 192.168.201.2 -p 5202 -t 10
  0.00-  12.04 sec    1769472 Bytes    1.18 Mbits/sec
```

This latest run also showed one first-ARP warm-up miss on STA1 before the
successful 5/5 ping. It did not leave the link unhealthy and both iperf clients
returned normally.

Interpretation:

The imported userspace and kernel-control path is now proven through scan
result delivery, authentication, association, station authorization, ICMP, and
iperf:

```text
wpa_supplicant
  -> libnl
  -> NL80211_CMD_TRIGGER_SCAN / GET_SCAN
  -> cfg80211
  -> mac80211 hw_scan
  -> mac80211_hwsim RX injection
  -> cfg80211 BSS table
  -> wpa_supplicant BSS selection
  -> NL80211_CMD_AUTHENTICATE
  -> cfg80211/mlme auth
  -> mac80211 auth frame TX
  -> hostfs-backed hwsim frame record
  -> AP-side mac80211_hwsim RX injection
  -> hostapd management event
  -> AP auth/assoc response TX
  -> NL80211_CMD_FRAME_TX_STATUS back to hostapd
  -> AP-STA-CONNECTED and STA authorization
  -> Ethernet data frame delivery via netdevice_compat
  -> ping and iperf
```

The current validation covers AP, STA1, and STA2 in separate simulator
processes, including STA-to-AP and STA-to-STA traffic. Longer stress tests and
secured WPA/EAPOL modes are still pending.

802.11b / legacy validation:

Basic 802.11b runtime now passes with `hostapd-b.conf` and `wpa-b.conf`.

Evidence from `/tmp/hwsim-11b-ap.log` and `/tmp/hwsim-11b-sta1.log`:

```text
Mode: IEEE 802.11b  Channel: 1  Frequency: 2412 MHz
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
nl80211: Associated on 2412 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   6.02 sec   13303808 Bytes   17.68 Mbits/sec
```

802.11g / legacy OFDM validation:

Basic 802.11g runtime now passes with dedicated open-auth configs:
`hostapd-g.conf` and `wpa-g.conf`. This replaces the earlier baseline-only
status with a fresh AP/STA runtime proof.

Evidence from `/tmp/hwsim-11g-ap.log` and `/tmp/hwsim-11g-sta1.log`:

```text
Mode: IEEE 802.11g  Channel: 1  Frequency: 2412 MHz
nl80211: Set freq 2412 (ht_enabled=0, vht_enabled=0, he_enabled=0, eht_enabled=0, bandwidth=20 MHz, cf1=2412 MHz, cf2=0 MHz)
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: selected BSS 02:00:00:00:00:01 ssid='nuttx-hwsim-g'
nl80211: Associated on 2412 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss
AP -> STA iperf: 0.00-   6.02 sec     327680 Bytes    0.44 Mbits/sec
STA -> AP iperf: 0.00-   6.02 sec     786432 Bytes    1.05 Mbits/sec
```

This older run hit the then-known hostfs hwsim medium 8 MiB truncate path during
iperf. Newer builds use `CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES` with a 64 MiB
default, but long debug-heavy runs remain throughput/medium tuning rather than
11g functional status.

802.11a / 5 GHz validation:

The first 802.11a attempt reached hostapd channel setup but failed because the
default built-in world regulatory domain marked 5180 MHz as `NO_IR`, so AP mode
was rejected before `NL80211_CMD_START_AP`. NuttX sim does not currently have
the Linux module-parameter/regdb userspace path, so the hwsim port now defaults
`regtest` to `HWSIM_REGTEST_CUSTOM_WORLD` under `CONFIG_ARCH_SIM`.

Retest evidence:

```text
Allowed channel: mode=2 chan=36 freq=5180 MHz max_tx_power=20 dBm
Mode: IEEE 802.11a  Channel: 36  Frequency: 5180 MHz
nl80211: Set freq 5180
genl_bridge: complete family=nl80211 cmd=15 ret=0
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
nl80211: Associated on 5180 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

802.11n / HT20 validation:

Basic HT runtime now passes with `hostapd-n.conf` and `wpa-n.conf`.

```text
nl80211: Set freq 2437 (ht_enabled=1, vht_enabled=0, he_enabled=0, eht_enabled=0, bandwidth=20 MHz, cf1=2437 MHz, cf2=0 MHz)
* channel_type=1
wlan0: AP-ENABLED
HT: STA 02:00:00:00:00:02 HT Capabilities Info: 0x107e
* ht_capabilities - hexdump(len=26): 0c 00 1b ff ff 00 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
nl80211: Associated on 2437 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11321344 Bytes   17.93 Mbits/sec
```

This proves the basic 802.11n/HT20 AP/STA path. HT40 and A-MPDU/ADDBA were
tracked separately at this point because this smoke test did not force a 40 MHz
secondary channel or capture BlockAck setup.

HT40 runtime now also passes with `hostapd-n-ht40.conf` and
`wpa-n-ht40.conf`:

```text
Scan for neighboring BSSes prior to enabling 40 MHz channel
HT40: control channel: 6 (2437 MHz), secondary channel: 10 (2457 MHz)
nl80211: Set freq 2437 (ht_enabled=1, vht_enabled=0, he_enabled=0, eht_enabled=0, bandwidth=40 MHz, cf1=2447 MHz, cf2=0 MHz)
* sec_channel_offset=1
* channel_type=3
wlan0: AP-ENABLED
HT: STA 02:00:00:00:00:02 HT Capabilities Info: 0x107e
* ht_capabilities - hexdump(len=26): 6e 00 1b ff ff 00 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
nl80211: Associated on 2437 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

A-MPDU/ADDBA was still separate after this HT40 run because it did not capture
BlockAck setup/teardown evidence. Later HT20 + WMM proof with
`CONFIG_WL_NUTTX_HWSIM_AMPDU_PROOF` now validates ADDBA request/response and
hwsim `ampdu_action` runtime.

802.11ac / VHT20 validation:

The first 11ac attempt failed before AP startup because hostapd did not know
the `ieee80211ac` and `vht_oper_chwidth` config items. The NuttX userspace
source list now enables `CONFIG_IEEE80211AC` and includes
`wpa_supplicant-2.11/src/ap/ieee802_11_vht.c`, matching the upstream Makefile
rules.

Retest evidence:

```text
hw vht capab: 0x38004fa, conf vht capab: 0x0
nl80211: Set freq 5180 (ht_enabled=1, vht_enabled=1, he_enabled=0, eht_enabled=0, bandwidth=20 MHz, cf1=5180 MHz, cf2=0 MHz)
* vht_enabled=1
* bandwidth=20
* center_freq1=5180
genl_bridge: complete family=nl80211 cmd=15 ret=0
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
nl80211: Associated on 5180 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

802.11ac / VHT80 validation:

Config files:

```text
hostapd-ac-vht80.conf
wpa-ac-vht80.conf
```

Initial issues and fixes:

1. The first VHT80 attempt failed in hostapd channel setup:

```text
80/80+80 MHz: no second channel offset
Could not set channel for kernel driver
wlan0: AP-DISABLED
```

`hostapd-ac-vht80.conf` now includes
`ht_capab=[HT40+][SHORT-GI-20][SHORT-GI-40]`, giving hostapd the required
secondary-channel offset for channel 36 with VHT center segment 42.

2. The next attempt reached nl80211 with the intended 80 MHz chandef but cfg80211
rejected it:

```text
nl80211: Set freq 5180 (ht_enabled=1, vht_enabled=1, he_enabled=0, eht_enabled=0, bandwidth=80 MHz, cf1=5210 MHz, cf2=0 MHz)
nl80211: Failed to set channel (freq=5180): -22
```

The NuttX sim hwsim custom world regulatory rule for the lower 5 GHz validation
band was still limited to 40 MHz. `mac80211_hwsim_linux.c` now allows 80 MHz in
the non-DFS 5 GHz validation ranges while keeping the existing 6 GHz validation
rule.

3. Rebuilding after the regulatory change exposed an unrelated build reproducibility
problem: the hwsim sim configs selected `CONFIG_SYSTEM_ARGTABLE3`, which forced a
network download of argtable3 and failed when GitHub returned no data. The AP,
STA1, and STA2 hwsim defconfigs now explicitly keep `CONFIG_SYSTEM_ARGTABLE3`
unset.

Retest evidence:

```text
nl80211: Set freq 5180 (ht_enabled=1, vht_enabled=1, he_enabled=0, eht_enabled=0, bandwidth=80 MHz, cf1=5210 MHz, cf2=0 MHz)
* vht_enabled=1
* bandwidth=80
* center_freq1=5210
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
nl80211: Associated on 5180 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   6.02 sec   13303808 Bytes   17.68 Mbits/sec
```

802.11ac / VHT160 validation:

Config files:

```text
hostapd-ac-vht160.conf
wpa-ac-vht160.conf
```

Initial issue:

The first VHT160 AP run reached the intended chandef but cfg80211 rejected the
channel update:

```text
nl80211: Set freq 5180 (... bandwidth=160 MHz, cf1=5250 MHz, cf2=0 MHz)
NL80211_CMD_SET_WIPHY ret=-22
```

The lower 5 GHz custom world regulatory rule did not allow a 160 MHz channel.
`mac80211_hwsim_linux.c` now widens that non-DFS validation range to 160 MHz.
During the same audit, `hwsim_world_regdom_custom_04.n_reg_rules` was corrected
from 6 to 7 so it matches the actual rule table.

Retest evidence from `/tmp/hwsim-vht160-ap2.log` and
`/tmp/hwsim-vht160-sta1-2.log`:

```text
nl80211: Set freq 5180 (ht_enabled=1, vht_enabled=1, he_enabled=0, eht_enabled=0, bandwidth=160 MHz, cf1=5250 MHz, cf2=0 MHz)
* bandwidth=160
* channel_width=5
* center_freq1=5250
* center_freq2=0
wlan0: AP-ENABLED
nl80211: Associated on 5180 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
10 packets transmitted, 10 received, 0% packet loss
STA -> AP iperf: 0.00-   5.05 sec     770048 Bytes    1.22 Mbits/sec
AP -> STA iperf: 0.00-   5.05 sec     737280 Bytes    1.17 Mbits/sec
```

802.11ac / VHT80+80 validation:

Config files:

```text
hostapd-ac-vht80p80.conf
wpa-ac-vht80p80.conf
```

Retest evidence from `/tmp/hwsim-vht80p80-ap.log` and
`/tmp/hwsim-vht80p80-sta1.log`:

```text
nl80211: Set freq 5180 (ht_enabled=1, vht_enabled=1, he_enabled=0, eht_enabled=0, bandwidth=80 MHz, cf1=5210 MHz, cf2=5775 MHz)
* bandwidth=80
* channel_width=4
* center_freq1=5210
* center_freq2=5775
wlan0: AP-ENABLED
nl80211: Associated on 5180 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
10 packets transmitted, 10 received, 0% packet loss
STA -> AP iperf: 0.00-   5.05 sec     819200 Bytes    1.30 Mbits/sec
AP -> STA iperf: 0.00-   5.05 sec     196608 Bytes    0.31 Mbits/sec
```

Current VHT status: VHT20, VHT80, VHT160, and VHT80+80 all pass AP/STA
functional runtime with ping and iperf. VHT IE/capability inspection is now
covered for VHT80, VHT160, and VHT80+80:

```text
/tmp/hwsim-vht80-ie-ap.log and /tmp/hwsim-vht80-ie-sta1.log:
Beacon/Probe Response/Association Response:
  bf 0c 20 00 00 00 aa aa 00 00 aa aa 00 00
  c0 05 01 2a 00 fc ff
STA Association Request:
  * vht_capabilities - hexdump(len=12): 32 00 80 03 aa aa 00 00 aa aa 00 00
STA data path:
  3 packets transmitted, 3 received, 0% packet loss

/tmp/hwsim-vht160-ap2.log and /tmp/hwsim-vht160-sta1-2.log:
channel_width=5, center_freq1=5250, center_freq2=0
Beacon/Probe Response/Association Response:
  bf 0c 60 ... c0 05 01 2a 32 fc ff

/tmp/hwsim-vht80p80-ap.log and /tmp/hwsim-vht80p80-sta1.log:
channel_width=4, center_freq1=5210, center_freq2=5775
Beacon/Probe Response/Association Response:
  bf 0c 60 ... c0 05 01 2a 9b fc ff
```

Wide-channel throughput tuning remains separate validation work.

## What Works Now

- `hwsim_ap`, `hwsim_sta1`, and `hwsim_sta2` sim defconfigs exist.
- The configs enable full imported cfg80211/mac80211 options:

  ```text
  CONFIG_WIRELESS_IEEE80211_CFG80211_LINUX=y
  # CONFIG_WIRELESS_IEEE80211_NL80211_METADATA_ONLY is not set
  CONFIG_WIRELESS_IEEE80211_MAC80211_LINUX=y
  CONFIG_WL_NUTTX_HWSIM=y
  ```

- AP and STA demo applications exist under:

  ```text
  apps/examples/wifi_hwsim_ap
  apps/examples/wifi_hwsim_sta
  ```

  These demos currently use NuttX socket/ioctl/netlib style setup, including
  Wireless Extensions ioctls such as `SIOCSIWMODE` and `SIOCSIWESSID`.

- `hostapd-2.11` and `wpa_supplicant-2.11` have been imported under:

  ```text
  apps/wireless/wifi
  ```

  Their startup paths call `ieee80211_linux_initialize()` when the imported
  cfg80211 path is enabled.

- `mac80211_hwsim_linux.c` is now compiled into the AP sim image.
- mac80211 mesh sources are included so the imported mac80211 build links.
- mac80211 minstrel_ht rate control is now compiled and registered through an
  explicit NuttX-side mac80211 init wrapper.
- Key symbols are present in the AP image, including:

  ```text
  nl80211_init
  cfg80211_linux_initialize
  mac80211_linux_initialize
  ieee80211_register_hw
  mac80211_hwsim_linux_initialize
  init_mac80211_hwsim
  ```

## Known Gaps

- The NuttX native `wifi_sim` lower-half path is not the target path and should
  not be used as proof that the Linux nl80211/cfg80211/mac80211 path works.
  It is still enabled as a build dependency in the current `hwsim_ap`/
  `hwsim_sta*` configs through `CONFIG_DRIVERS_WIFI_SIM` and
  `CONFIG_SIM_WIFIDEV_PSEUDO`, but `drivers/wireless/virtual/virtual_hwsim.c`
  no longer calls `wifi_sim_init()` for the hwsim path.
- The generic netlink bridge now has initial command and one-shot dump dispatch,
  but it is not a complete Linux generic-netlink implementation yet.
- hostapd reaches nl80211, configures the hwsim AP interface, and reaches
  `AP-ENABLED`.
- AP-side `mac80211_hwsim` now injects STA authentication/association/data
  frames from the shared medium and hostapd receives usable management events.
- PF_PACKET/l2-packet support is sufficient for the current open-network
  hostapd/wpa_supplicant validation. WPA/EAPOL security modes still need
  separate validation.
- Real AP/STA association, ping, and iperf through the imported nl80211,
  cfg80211, mac80211, and mac80211_hwsim stack has been verified for AP +
  STA1.
- Many Linux compat helpers are semantic stubs. They are sufficient for the
  current build and AP/STA1 validation, not yet for a production-quality
  wireless stack.

## Problems Encountered

### AP associated the STA but dropped data frames as unauthorized

Observed behavior:

STA1 reached association locally, but AP-side ARP/data frames were dropped in
mac80211 because the AP station entry did not have `WLAN_STA_AUTHORIZED`.
Debug output showed hostapd had sent `NEW_STATION`/`SET_STATION` for
authenticated and associated state, but had not yet authorized the port:

```text
hwsim-debug: nl80211 STA_FLAGS2 iftype=3 len=8 mask=0xa4 set=0xa4
hwsim-debug: mac80211 sta_apply_auth_flags done ... auth=1 assoc=1 authorized=0
```

Cause:

hostapd only authorizes the open-network STA after it receives the association
response TX status event. mac80211 queued the assoc-response skb in
`local->ack_status_frames`, but the NuttX Linux-compat `idr` implementation was
a stub:

```text
idr_alloc(...) -> start
idr_find(...)  -> NULL
idr_remove(...) -> NULL
```

As a result, `ieee80211_report_ack_skb()` could not recover the saved skb and
no useful `NL80211_CMD_FRAME_TX_STATUS` event reached hostapd:

```text
hwsim-debug: mac80211 report_ack missing ack_skb status_data=1 acked=1 dropped=0
```

Fix:

`nuttx/wireless/ieee80211/include/linux/idr.h` now has a minimal linked-list
IDR implementation:

- `idr_alloc()` stores `(id, pointer)` entries and returns a free ID.
- `idr_find()` and `idr_remove()` return the stored pointer.
- `idr_for_each()` and `idr_for_each_entry()` iterate stored entries for
  cleanup and NAN-function users.

The stale empty `idr_for_each()` implementation was removed from
`cfg80211_compat.h`.

Verified:

```text
hwsim-debug: mac80211 report_ack fc=0x0010 len=46 cookie=3 acked=1 ...
hwsim-debug: nl80211 frame_tx_status cmd=60 ifindex=1 cookie=3 ack=1 len=46
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
hwsim-debug: mac80211 sta_apply_auth_flags done ... auth=1 assoc=1 authorized=1
```

After this fix, data traffic passed:

```text
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

### Data frames were not entering the NuttX network stack

Observed behavior:

After association started working, hwsim could move management frames, but
ordinary ARP/IP frames were either not published across the shared medium or
were dropped when Linux `netif_receive_skb()` reached the compatibility layer.

Cause:

- The temporary hostfs hwsim medium initially treated only management frames as
  publishable/injectable.
- `netif_receive_skb()` in `cfg80211_compat.h` still freed received skb data
  instead of bridging it back into the NuttX `wlan0` lower-half.

Fix:

- `mac80211_hwsim_linux.c` now publishes and injects data frames through the
  shared medium in addition to management frames.
- `netif_receive_skb()` and `netif_receive_skb_list()` now call
  `ieee80211_linux_netif_rx()`, which delivers converted Ethernet frames to the
  bound NuttX lower-half.

Verified:

```text
hwsim-debug: mac80211 deliver data ... proto=0x0806
netdevice_compat: rx linux wlan0 len=42 ethertype=0x0806 ...
hwsim-debug: mac80211 deliver data ... proto=0x0800
netdevice_compat: rx linux wlan0 len=98 ethertype=0x0800 ...
56 bytes from 192.168.201.1: icmp_seq=2 time=30.0 ms
```

### NuttX-visible wlan0 MAC stayed zero or used the wrong hwsim octets

Observed behavior:

`hostapd` reads the interface MAC through `SIOCGIFHWADDR`. That ioctl reads
the NuttX `net_driver_s.d_mac` value, while the imported Linux mac80211/hwsim
path owns the real `struct net_device->dev_addr`. Before synchronization,
hostapd could see `00:00:00:00:00:00` even though the imported Linux hwsim
radio had generated a valid MAC.

After adding per-role radio offsets, an additional byte-placement bug made the
AP address appear as:

```text
02:00:00:00:01:00
```

instead of:

```text
02:00:00:00:00:01
```

Cause:

- The NuttX lower-half MAC was not copied from the imported Linux netdev when
  binding `wlan0`.
- `mac80211_hwsim_new_radio()` stored the generated radio index in
  `addr[3]`/`addr[4]`, which shifted the low byte one octet too early for the
  intended `02:00:00:00:00:N` role addresses.

Fix:

- Added `ieee80211_linux_sync_lowerhalf_mac()` and called it during
  `virtual_hwsim_ifup()` / Linux-netdev lower-half binding.
- Added `CONFIG_WL_NUTTX_HWSIM_RADIO_BASE` so separately launched AP/STA sim
  processes do not all generate the same radio MAC.
- Stored the generated hwsim radio index in `addr[4]`/`addr[5]`.

Verified:

```text
AP:
netdevice_compat: sync lower wlan0 mac 02:00:00:00:00:01
hwsim-debug: i802_init hwaddr ok 02:00:00:00:00:01

STA1:
netdevice_compat: sync lower wlan0 mac 02:00:00:00:00:02
wlan0: BSS: Add new id 0 BSSID 02:00:00:00:00:01 SSID 'nuttx-hwsim' freq 2412
```

### cfg80211 initialization returned `-ENODEV`

Observed behavior:

```text
hostapd: ieee80211_linux_initialize ret=-19
```

Cause:

The Linux cfg80211 initialization path expected a valid faux device object.
The NuttX compatibility implementation returned `NULL`.

Fix:

`faux_device_create()` was changed to return a static faux device and attach
the parent/driver data, allowing cfg80211 initialization to complete.

### hostapd reached nl80211 but netlink send failed

Observed behavior:

```text
hostapd: ieee80211_linux_initialize ret=0
nl80211: Kernel version: NuttX 0.0.0 (...)
nl80211: Maximum supported attribute ID: 348
nl80211: nl_send_auto_complete() failed: Operation not supported
nl80211 driver initialization failed.
```

Meaning:

This proved hostapd reached the imported nl80211 path, but the generic netlink
bridge still returned `-EOPNOTSUPP` for command handling.

Next direction:

Implement enough generic netlink dispatch/reply support for nl80211 commands,
then continue into cfg80211/mac80211 device registration and hwsim radio
creation.

### mac80211 hwsim returned `-ENOENT` during initialization

Observed behavior:

```text
mac80211_hwsim: initializing netlink
netlink_generic: registered family MAC80211_HWSIM id=20 groups=1
genl_bridge: netlink_generic_register(MAC80211_HWSIM) ret=0 id=20
hostapd: ieee80211_linux_initialize ret=-2
```

Cause:

The imported Linux mac80211 module init path was not called. In Linux,
`subsys_initcall(ieee80211_init)` registers mac80211 support and the
`minstrel_ht` rate-control algorithm. In the current NuttX compatibility layer,
`subsys_initcall()`/`module_init()` are no-ops, so hwsim radio creation reached
`ieee80211_register_hw()` without any registered rate-control algorithm and
returned `-ENOENT`.

Fix:

- Added `rc80211_minstrel_ht.c` to the mac80211 Make/CMake build.
- Set the compat default rate-control algorithm to `"minstrel_ht"`.
- Added `linux/minmax.h` as a thin compat include needed by minstrel.
- Added `mac80211_linux_initialize()` in `mac80211/main.c`.
- `ieee80211_linux_initialize()` now calls:

  ```text
  cfg80211_linux_initialize()
  mac80211_linux_initialize()
  mac80211_hwsim_linux_initialize()
  ```

Result:

```text
hostapd: ieee80211_linux_initialize ret=0
```

The failure moved forward from hwsim/mac80211 initialization to hostapd's
nl80211 driver initialization.

### NL80211 metadata-only bridge path guarded

Problem:

`genetlink_bridge.c` still had a legacy branch that preferred
`nl80211_metadata_sendto()` for the `nl80211` family whenever that weak symbol
was present. That is only appropriate for
`CONFIG_WIRELESS_IEEE80211_NL80211_METADATA_ONLY=y`.

Fix:

The metadata send path is now compiled only under
`CONFIG_WIRELESS_IEEE80211_NL80211_METADATA_ONLY`. In the full stack build,
NL80211 requests dispatch through the imported Linux `nl80211_fam` operation
table.

### NuttX netlink headers conflict with Linux uapi netlink headers

Observed build errors:

```text
redefinition of 'struct sockaddr_nl'
redefinition of 'struct nlmsghdr'
redefinition of 'struct nlmsgerr'
redefinition of 'struct nlattr'
```

Cause:

Including the full NuttX `<nuttx/net/netlink.h>` in the Linux compatibility
bridge collides with Linux uapi netlink structures included by the imported
wireless stack.

Fix:

`genetlink_bridge.c` now avoids including the full NuttX netlink header and uses
small local ABI-compatible declarations for the NuttX response queue entry
points.

### `mac80211_hwsim_linux.c` entered the build

Previous state:

`nuttx/drivers/wireless/virtual/Make.defs` and `CMakeLists.txt` now select
`mac80211_hwsim_linux.c` when `CONFIG_WL_NUTTX_HWSIM=y`.

Previous build blockers:

```text
redefinition of 'net_eq'
fatal error: linux/virtio.h: No such file or directory
```

The first issue is a compatibility header duplication. The second comes from the
Linux hwsim optional virtio support path and needs minimal NuttX-side virtio
compatibility stubs or a local compile-time disable.

Current result:

Both issues have been moved past. The AP sim image builds with the imported
`mac80211_hwsim_linux.c` included.

### PF_PACKET was required by hostapd EAPOL setup

Observed behavior:

```text
EAPOL TX socket(PF_PACKET,SOCK_DGRAM) failed
```

Fix:

The hwsim AP/STA defconfigs and the active AP `.config` now enable packet
sockets:

```text
CONFIG_NET_PKT=y
CONFIG_NET_PKT_PREALLOC_CONNS=4
CONFIG_NET_PKT_ALLOC_CONNS=4
```

Result:

```text
hwsim-debug: i802_init eapol socket ok fd=10
```

### `if_nametoindex` duplicate symbol after enabling packet sockets

Observed behavior:

```text
multiple definition of `NXif_nametoindex'
```

Cause:

Enabling packet sockets selected NuttX netdev interface-index support, which
provides the real libc `if_nametoindex()` implementation. The Wi-Fi app compat
layer still built its fallback implementation under the NuttX namespace mapping.

Fix:

`apps/wireless/wifi/common/nuttx_wifi_compat.c` now builds the fallback only
when `CONFIG_NETDEV_IFINDEX` is not enabled.

### Linux netdev `IFF_UP` was not synchronized

Observed behavior:

```text
genl_bridge: pre_doit failed family=nl80211 cmd=20 ret=-100
```

`-100` is `ENETDOWN`. hostapd had requested the NuttX interface to be up, but
the imported Linux `struct net_device` still did not have `IFF_UP`, so
cfg80211/mac80211 `wdev_running()` checks failed.

Fix:

Added `ieee80211_linux_set_netdev_flags(ifname, up)` in
`nuttx/wireless/ieee80211/netdevice_compat.c` and called it from the
`linux_ioctl.c` `SIOCSIFFLAGS` path used by hostapd/wpa_supplicant.

Result:

```text
netdevice_compat: wlan0 up flags=0x1
nl80211: flush -> DEL_STATION wlan0 (all)
genl_bridge: complete family=nl80211 cmd=20 ret=0
```

### `/dev/urandom` was missing

Observed behavior:

```text
Could not open /dev/urandom.
Accounting initialization failed.
```

Fix:

Enabled `CONFIG_DEV_URANDOM=y` in the active AP config and all hwsim sim
defconfigs.

Result:

hostapd moved past accounting initialization and reached beacon/AP setup.

### `REGISTER_BEACONS` failed with `-ENODEV`

Observed behavior:

```text
nl80211: Register beacons command failed: ret=-19
genl_bridge: pre_doit failed family=nl80211 cmd=85 ret=-19
```

Investigation:

`NL80211_CMD_REGISTER_BEACONS` only carries `NL80211_ATTR_WIPHY`. Instrumenting
`nl80211_pre_doit()` and `cfg80211_rdev_by_wiphy_idx()` showed that hostapd was
sending:

```text
NL80211_ATTR_WIPHY=0xffffffff
```

The imported cfg80211 side had valid rdevs:

```text
candidate=1 name=nuttx
candidate=0 name=nuttx
```

Cause:

hostapd's `nl80211_get_wiphy_index()` sometimes received only the ACK for
`GET_INTERFACE` before receiving the actual `NEW_INTERFACE` reply. That left
the local `wiphy_idx` as `-1`, which was later sent to
`REGISTER_BEACONS`.

Fix:

Two changes were made in
`apps/wireless/wifi/wpa_supplicant-2.11/src/drivers/driver_nl80211.c`:

- `nl80211_get_wiphy_index()` falls back to `drv->wiphy_idx` when
  `GET_INTERFACE` returns without a wiphy index but `GET_WIPHY` has already
  populated `drv->phyname`.
- `send_and_recv()` now uses an ACK handler that can continue receiving when an
  ACK arrives before the valid reply for requests that install a valid reply
  handler.

Result:

```text
hwsim-debug: get_wiphy_index fallback drv idx=0 phy=nuttx
genl_bridge: complete family=nl80211 cmd=85 ret=0
```

### `START_AP` initially failed with `-EINVAL`

Observed behavior before the ACK/reply ordering fix:

```text
nl80211: Beacon set failed: -22
genl_bridge: complete family=nl80211 cmd=15 ret=-22
```

Investigation:

`nl80211_start_ap()` was instrumented. The failing path was that hostapd did not
send `NL80211_ATTR_WIPHY_FREQ`, and cfg80211 had no preset AP channel:

```text
nl80211-debug: start_ap no channel freq_attr=0 preset=0 valid_links=0x0
```

The missing frequency came from hostapd failing to process the `GET_WIPHY`
hardware channel list because an ACK could arrive before the valid dump reply.

Fix:

The same `send_and_recv()` ACK-before-valid handling allowed hostapd to receive
and parse the full band/channel list.

Result:

```text
Allowed channel: mode=1 chan=1 freq=2412 MHz max_tx_power=0 dBm
  * freq=2412
nl80211-debug: start_ap parse chandef freq=2412
genl_bridge: complete family=nl80211 cmd=15 ret=0
wlan0: interface state UNINITIALIZED->ENABLED
wlan0: AP-ENABLED
```

### PF_PACKET poll after AP startup

Observed behavior after `AP-ENABLED`:

```text
pkt_pollsetup: ERROR: No device found for PKT connection
```

Meaning:

AP startup through hostapd/nl80211/cfg80211/mac80211/hwsim now succeeds, but
the NuttX packet-socket poll path used by hostapd's event loop cannot map the
PF_PACKET connection back to a device.

Next direction:

Inspect the NuttX `net/pkt` connection binding and the hostapd EAPOL/packet
socket setup. The likely next fix is to make the imported hwsim netdev visible
to NuttX packet sockets or to bridge PF_PACKET bind/poll semantics for the
Linux-compatible netdev wrapper.

### Resolved blocker: cross-process hwsim medium

Observed behavior:

```text
AP:
wlan0: interface state UNINITIALIZED->ENABLED
wlan0: AP-ENABLED

STA:
start_scan hw_scan=1
mac80211_hwsim_hw_scan queue
hw_scan_work complete
wlan0: Event SCAN_RESULTS (3) received
nl80211: Received scan results (0 BSSes)
wlan0: No suitable network found
```

Meaning:

The imported userspace and kernel wireless paths are now active on both sides,
but separately launched NuttX sim binaries do not share one `mac80211_hwsim`
radio list or simulated air medium.

In the Linux reference under `/home/uan/Feather-develop-WIFI/hwsim`, the flow
loads one kernel module instance:

```text
mac80211_hwsim.ko radios=2
```

That kernel-global instance is shared by hostapd and wpa_supplicant processes.
The current NuttX sim port instead creates hwsim state inside each host process,
so `nuttx-sim-ap` can beacon locally while `nuttx-sim-sta1` scans a different
local hwsim world.

Resolution:

A temporary hostfs-backed shared medium was added for the separately launched
simulator processes. It now carries scan state, management frames, and data
frames far enough to validate AP + STA1 association, ping, and iperf. This is
still a pragmatic NuttX sim transport, not a full replacement for Linux's
kernel-global hwsim radio list.

### Native netdev fallback replaced by initial Linux netdev bridge

Observed code path:

```text
drivers/wireless/virtual/virtual_hwsim.c
  virtual_hwsim_init()
    -> ieee80211_linux_initialize()
    -> mac80211_hwsim_linux_initialize()
    -> install virtual_hwsim lower-half ops
```

Meaning:

The imported Linux wireless stack is initialized, and the board-level NuttX
network device now installs a small virtual hwsim lower-half instead of calling
the native `wifi_sim_init()` fallback.

The bridge currently does the following:

- `virtual_hwsim_ifup()` binds the NuttX lower-half name, for example `wlan0`,
  to the imported Linux compat `struct net_device` with the same name.
- NuttX lower-half TX copies the `netpkt_t` payload into an sk_buff and calls
  the Linux netdev `ndo_start_xmit()` entry point.
- Linux `netif_rx()` now queues sk_buffs for the bound NuttX lower-half instead
  of immediately freeing them.
- Linux `dev_queue_xmit()` now calls the skb's netdev `ndo_start_xmit()` when
  available instead of always dropping the frame.

This is still a minimal bridge. It is enough to stop validating the wrong
native `wifi_sim` lower-half, and AP + STA1 data traffic now proves the main
management/data path. More complex packet-socket, WPA/EAPOL, and multi-STA
semantics still need separate validation.

Problem encountered:

After replacing the fallback, STA scans started failing with:

```text
genl_bridge: pre_doit failed family=nl80211 cmd=33 ret=-100
nl80211: Scan trigger failed: ret=-100
```

Cause:

The NuttX `<net/if.h>` macro `IFF_UP` uses bit `1 << 1`, while the Linux compat
`enum net_device_flags` uses bit `1 << 0`. `ieee80211_linux_set_netdev_flags()`
was setting only the NuttX bit, so nl80211's Linux-side `netif_running()` still
considered the device down.

Fix:

`ieee80211_linux_set_netdev_flags()` now sets both the Linux compat up bit and
the NuttX up bit. Verified runtime state:

```text
netdevice_compat: wlan0 up before=0x0 linux_up=0x1 nuttx_up=0x2
netdevice_compat: wlan0 up after=0x3
```

Validation:

Both AP and STA images build after the bridge change. STA scan again reaches
the hwsim hardware scan path:

```text
hwsim-debug: start_scan hw_scan=1
hwsim-debug: mac80211_hwsim_hw_scan queue
hwsim-debug: hw_scan_work complete
wlan0: Event SCAN_RESULTS (3) received
nl80211: Received scan results (0 BSSes)
```

AP startup still reaches:

```text
wlan0: interface state UNINITIALIZED->ENABLED
wlan0: AP-ENABLED
```

Cross-process AP/STA validation at this stage still returned `0 BSSes` on the
STA side. That was resolved later by adding the hostfs-backed BSS/frame shared
medium described below.

Build note:

`--no-clean` builds can leave a stale `nuttx` ELF even after `libwireless.a`
changes. The AP/STA build helpers now remove `nuttx` and `nuttx.map` before
running `make`, forcing the final simulator ELF to relink. Also, `--no-clean`
must only be used when the current configured board already matches the target;
otherwise it can produce an output file with the requested name but the previous
board configuration.

Follow-up:

Extend the current AP + STA1 validation to STA2 and revisit the transport shape
if the hostfs-backed record format becomes too limiting.

### Cross-process scan visibility via temporary hostfs BSS record

Problem:

After the NuttX lower-half was bound to the imported Linux netdev, AP and STA
could run in separate simulator processes, but STA scans still returned:

```text
nl80211: Received scan results (0 BSSes)
wlan0: No suitable network found
```

Cause:

Each `nuttx-sim-*` executable has its own process-local `mac80211_hwsim`
radio list. Linux's reference setup uses one kernel-global
`mac80211_hwsim.ko radios=2`, so AP and STA share one in-kernel simulated RF
medium. Separate NuttX sim processes do not share that in-memory list.

First attempted hook:

`mac80211_hwsim_tx_frame_no_nl()` was taught to publish beacon/probe-response
frames to `/h/hwsim-bss.bin`, and STA `hw_scan_work()` was taught to read and
inject that record when scanning the same frequency.

Observed issue:

AP did not log `published BSS`; the actual periodic beacon TX path did not run
soon enough for the test. AP had already configured the beacon template:

```text
nl80211: Set beacon (beacon_set=0)
nl80211: beacon_int=100
wlan0: AP-ENABLED
```

Fix:

When `mac80211_hwsim_link_info_changed()` receives
`BSS_CHANGED_BEACON_ENABLED`, the NuttX hwsim port now generates one
`ieee80211_beacon_get()` template and publishes it to the hostfs BSS record.
This started as a scan-only bridge and was later extended to management and
data frames.

Validation:

```text
AP:
hwsim-debug: published BSS frame freq=2412 len=77 path=/h/hwsim-bss.bin

STA:
hwsim-debug: injected shared BSS frame freq=2412 len=77 path=/h/hwsim-bss.bin
nl80211: Received scan results (1 BSSes)
wlan0: BSS: Add new id 0 BSSID 02:00:00:00:00:01 SSID 'nuttx-hwsim' freq 2412
```

Current validation:

The same hostfs-backed approach now carries authentication, association,
EAPOL, ARP/IP, ping, and iperf data for AP + STA1 + STA2. For open
authentication, sequential STA-to-STA traffic was validated with AP + STA1 +
STA2 in three simulator processes:

```text
/tmp/hwsim-seq-sta1.log:
STA1 -> STA2: 4 packets transmitted, 4 received, 0% packet loss
STA1 -> STA2 iperf: 0.00-   6.02 sec    6750208 Bytes    8.97 Mbits/sec

/tmp/hwsim-seq-sta2.log:
STA2 -> STA1: 4 packets transmitted, 4 received, 0% packet loss
STA2 -> STA1 iperf: 0.00-   6.02 sec    6963200 Bytes    9.25 Mbits/sec
```

One issue found during this validation: running both STA-to-STA ping commands
at the same time can produce misleading `100% packet loss` and
`ICMP packet with unknown type: 8` output because both sides receive the peer's
echo request while their own ping command is waiting for echo replies. The
data path is functional when the directions are tested sequentially.

After the bounded hwsim medium fix, open-auth simultaneous bidirectional
STA-to-STA iperf was retested with AP + STA1 + STA2:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03

Baseline ping:
STA1 -> STA2: 3 packets transmitted, 3 received, 0% packet loss
STA2 -> STA1: 3 packets transmitted, 3 received, 0% packet loss

Full-duplex:
STA1 -> STA2 iperf: 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec
STA2 -> STA1 iperf: 0.00-   6.02 sec     753664 Bytes    1.00 Mbits/sec

Post-stress ping:
STA1 -> STA2: 3 packets transmitted, 3 received, 0% packet loss
STA2 -> STA1: 3 packets transmitted, 3 received, 0% packet loss
```

Current status: open-auth simultaneous STA-to-STA TCP stress is functional and
no longer a link-breaking hardening item. Throughput remains low, and the
server side can still report `tcp server recv error, error code: 11` while
closing after the peer exits. Keep this as a medium/TCP performance and iperf
close-path tuning item rather than an open-auth functional failure.

### `nl80211_post_doit()` crash after authenticate

Problem:

Once STA could see the AP BSS, `wpa_supplicant` progressed to authentication
but crashed immediately after `NL80211_CMD_AUTHENTICATE` completed:

```text
wlan0: SME: Trying to authenticate with 02:00:00:00:00:01
genl_bridge: complete family=nl80211 cmd=37 ret=0
Program received signal SIGSEGV, Segmentation fault.
#0 nlmsg_len()
#1 nl80211_post_doit()
#2 genl_bridge_dispatch()
```

Cause:

`NL80211_CMD_AUTHENTICATE` uses `NL80211_FLAG_CLEAR_SKB`. Linux clears sensitive
request payload bytes in `nl80211_post_doit()` using a normal kernel skb whose
netlink header helpers have full Linux skb semantics. The NuttX compat skb can
have `skb->data` moved by lower paths, so `nlmsg_hdr(skb)` produced an invalid
header pointer.

Fix:

- `genetlink_bridge.c` restores `skb->data`, `skb->len`, and `skb->tail` to the
  original request buffer before calling `post_doit`.
- `nl80211_post_doit()` now clears from the compat skb's `head` buffer and
  validates the copied `nlmsghdr` length before `memset()`.

Validation:

STA no longer crashes. At this stage it reached:

```text
genl_bridge: complete family=nl80211 cmd=37 ret=0
hwsim-debug: send_and_recv done type=19 cmd=37 ret=0
nl80211: Authentication request send successfully
wlan0: SME: Authentication timeout
```

That blocker was later resolved by the shared frame medium and by fixing the
`idr` compatibility layer so AP management-frame TX status reaches hostapd.

### WPA2-Personal PSK/CCMP EAPOL and encrypted data path

Problem:

Open authentication had already passed AP + STA1 + STA2 association, ping, and
iperf, but WPA2-Personal initially failed before `CTRL-EVENT-CONNECTED`. The
STA could scan, authenticate, and associate, then timed out because the EAPOL
4-way handshake did not complete.

Failure chain:

- AP-side hostapd EAPOL RX had been skipped during the earlier open-auth
  bring-up:

```text
hwsim-debug: skip AP EAPOL RX PF_PACKET socket for NuttX hwsim
```

- After enabling the AP PF_PACKET path, AP sent M1 and STA sent M2, but hostapd
  saw the reply from an all-zero source MAC and kept retrying M1:

```text
IEEE 802.1X: 121 bytes from 00:00:00:00:00:00
WPA: EAPOL-Key timeout
```

- The STA saw the same all-zero source on M1:

```text
l2_packet_receive: src=00:00:00:00:00:00 len=99
```

Root causes and fixes:

- `driver_nl80211.c`: removed the hwsim-only AP EAPOL RX skip so hostapd opens
  its PF_PACKET control-port socket.
- `netdevice_compat.c`: NuttX lower TX now fills `skb->protocol` from Ethernet
  bytes 12/13, allowing EAPOL (`0x888e`) to be published through the shared
  hwsim medium.
- `mac80211/rx.c`: control-port ethertype matching now tolerates the NuttX
  compat byte-order form, so STA-side EAPOL RX is not dropped.
- `cfg80211_compat.h`: `eth_type_trans()` no longer depends on an incomplete
  Linux `struct ethhdr`; it reads bytes 12/13 directly.
- `net/pkt/pkt_recvmsg.c`: AF_PACKET `recvfrom()` now fills
  `sockaddr_ll.sll_addr` from the Ethernet source address for both active
  callback delivery and packet readahead. This gives hostapd/wpa_supplicant the
  peer MAC address required by the WPA state machines.

Validation:

Hex PSK config first proved the PSK derivation was not the variable:

```text
AP:
IEEE 802.1X: 121 bytes from 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (2/4 Pairwise)
wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA:
l2_packet_receive: src=02:00:00:00:00:01 len=99
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
```

Normal passphrase WPA2-PSK/CCMP then passed handshake, ping, and iperf:

```text
AP:
wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (2/4 Pairwise)
wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)
hwsim-debug: mac80211 sta_apply_auth_flags done ... authorized=1
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA:
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   13385728 Bytes   21.21 Mbits/sec
```

AP + STA1 + STA2 WPA2 sequential STA-to-STA validation:

```text
STA1 -> STA2:
STA1 and STA2 both completed:
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

PING 192.168.201.3 56 bytes of data
56 bytes from 192.168.201.3: icmp_seq=0 time=110.0 ms
56 bytes from 192.168.201.3: icmp_seq=1 time=60.0 ms
56 bytes from 192.168.201.3: icmp_seq=2 time=70.0 ms
56 bytes from 192.168.201.3: icmp_seq=3 time=50.0 ms
56 bytes from 192.168.201.3: icmp_seq=4 time=60.0 ms
5 packets transmitted, 5 received, 0% packet loss, time 5050 ms
0.00-   5.05 sec    5734400 Bytes    9.08 Mbits/sec

STA2 -> STA1:
STA1 and STA2 both completed:
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

PING 192.168.201.2 56 bytes of data
56 bytes from 192.168.201.2: icmp_seq=0 time=100.0 ms
56 bytes from 192.168.201.2: icmp_seq=1 time=50.0 ms
56 bytes from 192.168.201.2: icmp_seq=2 time=60.0 ms
56 bytes from 192.168.201.2: icmp_seq=3 time=40.0 ms
56 bytes from 192.168.201.2: icmp_seq=4 time=50.0 ms
5 packets transmitted, 5 received, 0% packet loss, time 5050 ms
0.00-   5.05 sec    5832704 Bytes    9.24 Mbits/sec
```

Simultaneous WPA2-PSK/CCMP STA1<->STA2 full-duplex stress after the bounded
hwsim medium fix:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:03

Baseline ping:
STA1 -> STA2: 3 packets transmitted, 3 received, 0% packet loss
STA2 -> STA1: 3 packets transmitted, 3 received, 0% packet loss

Full-duplex:
STA1 -> STA2 iperf: 0.00-   6.02 sec     835584 Bytes    1.11 Mbits/sec
STA2 -> STA1 iperf: 0.00-   6.02 sec     196608 Bytes    0.26 Mbits/sec

Medium file:
hwsim-frames.bin: 6.9 MiB -> 8.1 KiB after bounded truncate

Post-stress ping:
STA1 -> STA2: 3 packets transmitted, 3 received, 0% packet loss
STA2 -> STA1: 3 packets transmitted, 3 received, 0% packet loss
```

Current status: WPA2 simultaneous STA-to-STA stress is now functional and no
longer breaks AP/STA ARP after the medium backlog fix. Throughput is still low,
and the weaker-direction iperf client can take several extra seconds to return
to the NSH prompt, so medium scheduling/TCP close performance remains open.

### WPA2 TKIP and mixed TKIP/CCMP

Additional AP/STA configs were added for TKIP-only and mixed-cipher validation:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-psk-tkip.conf
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-psk-tkip.conf
tools/firmware/sim/hostapd-hwsim-wpa-mixed.conf
tools/firmware/sim/wpa_supplicant-hwsim-wpa-mixed.conf
```

TKIP-only result:

```text
/tmp/hwsim-tkip-ap.log
/tmp/hwsim-tkip-sta1.log

wlan0: WPA: Selected cipher suites: group 8 pairwise 8 key_mgmt 2 proto 2
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=TKIP GTK=TKIP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   13271040 Bytes   21.02 Mbits/sec
```

Mixed TKIP/CCMP result:

```text
/tmp/hwsim-mixed-ap.log
/tmp/hwsim-mixed-sta1.log

wlan0: WPA: Selected cipher suites: group 8 pairwise 16 key_mgmt 2 proto 2
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=TKIP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

No driver code change was required for these two cases. They reuse the WPA2
EAPOL/control-port and encrypted data-path fixes described above.

### WPA2/WPA3 transition mode

Configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa-transition.conf
tools/firmware/sim/wpa_supplicant-hwsim-wpa-transition.conf
tools/firmware/sim/wpa_supplicant-hwsim-wpa-transition-psk.conf
```

SAE-capable STA result:

```text
/tmp/hwsim-transition-ap.log
/tmp/hwsim-transition-sta1.log

wlan0: RSN: using KEY_MGMT SAE
SAE: Derive keys - H2E=0 AKMP=0x400 = 000fac08 (SAE)
wlan0: WPA: using MGMT group cipher AES-128-CMAC
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11190272 Bytes   17.73 Mbits/sec
```

Legacy WPA-PSK-only STA result:

```text
/tmp/hwsim-transition-psk2-sta1.log
/tmp/hwsim-transition-psk3-sta1.log

wlan0: WPA: using KEY_MGMT WPA-PSK
wlan0: WPA: not using MGMT group cipher
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11485184 Bytes   18.19 Mbits/sec
```

Issue noted during validation:

- The PSK-only transition run with `wpa_supplicant -dd` completed association
  and key negotiation, but the very verbose supplicant output delayed the
  subsequent NSH commands enough that the timeout window ended before ping and
  iperf ran.
- The solution was to keep the `-dd` log for key-management proof and run a
  second low-noise supplicant pass for ping/iperf data-plane proof.

### WPA3-SAE H2E

Configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa3-sae-h2e.conf
tools/firmware/sim/wpa_supplicant-hwsim-wpa3-sae-h2e.conf
```

Runtime result:

```text
/tmp/hwsim-sae-h2e-ap.log
/tmp/hwsim-sae-h2e-sta1.log

SAE: Derive PT - group 19
SAE: Derive PWE from PT
SAE: Derive keys - H2E=1 AKMP=0x400 = 000fac08 (SAE)
WPA: RSNXE in EAPOL-Key - hexdump(len=3): f4 01 20
wlan0: WPA: IGTK keyid 4 pn 000000000000
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11567104 Bytes   18.32 Mbits/sec
```

Current-image refresh:

```text
/tmp/hwsim-wpa3-h2e-refresh2-ap.log
/tmp/hwsim-wpa3-h2e-refresh2-sta1.log

wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: PMKSA-CACHE-ADDED 02:00:00:00:00:01 0
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA->AP ping: 5 packets transmitted, 5 received, 0% packet loss
STA->AP iperf: 0.00-   6.02 sec     393216 Bytes    0.52 Mbits/sec
STA->AP post ping: 3 packets transmitted, 3 received, 0% packet loss
AP->STA ping: 5 packets transmitted, 5 received, 0% packet loss
AP->STA iperf: 0.00-   6.02 sec    7520256 Bytes    9.99 Mbits/sec
AP->STA post ping: 3 packets transmitted, 3 received, 0% packet loss
```

Notes:

- H2E is enabled with `sae_pwe=1`; source comments confirm this means
  hash-to-element only.
- SAE-PK is tracked separately because it needs the SAE-PK source and extra EC
  key handling in the simulator crypto shim.
- One refresh attempt queued ping/iperf before the delayed `CTRL-EVENT-CONNECTED`
  line arrived, producing ARP `Not reachable` errors before key negotiation
  completed. The fix is to gate traffic commands on the association/key
  completion log lines.
- The successful refresh still hit the configured 64 MiB shared-frame medium
  truncate guard during iperf. This is a bounded hostfs transport behavior, not
  a H2E key-management failure; both directions kept 0% ping loss after iperf.

### WPA2 PMF required, SHA256 AKM, and IGTK/BIP

Problem:

WPA2 PMF required (`ieee80211w=2`, `WPA-PSK-SHA256`) initially failed during
AP startup:

```text
nl80211: key_mgmt_suites=0x100
Failed to set beacon parameters
Interface initialization failed
```

Cleanup then tried to transmit a broadcast deauthentication frame and received:

```text
nl80211: Frame command failed: ret=-16 (Unknown error 16) (freq=2412 wait=0)
```

The deauthentication failure was secondary. The primary failure was that
hostapd did not send `NL80211_CMD_START_AP` at all after building the PMF
beacon parameters.

Cause:

The NuttX/libnl default `nlmsg_alloc()` buffer was too small for larger AP setup
messages. Plain WPA2 `START_AP` happened to reach exactly 256 bytes. PMF added
enough SSID/RSN information that one of the later `nla_put()` calls failed
inside `driver_nl80211.c`, so the function jumped to `fail` before sending
`START_AP`.

Fix:

`driver_nl80211.c` now builds AP beacon/start messages with an explicit
4096-byte netlink message:

```text
nl80211_ifindex_msg_build(drv, nlmsg_alloc_size(4096), bss->ifindex, 0, cmd)
```

This fix also gives later HT/VHT/HE/EHT beacon and association IE validation
headroom.

Validation:

```text
AP:
hwsim-debug: send_and_recv send type=19 cmd=15 flags=0x0
genl_bridge: complete family=nl80211 cmd=15 ret=0
IGTK - hexdump(len=16): [REMOVED]
wlan0: AP-ENABLED
wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)
nl80211: Set STA flags - ifname=wlan0 addr=02:00:00:00:00:02 total_flags=0x6d flags_or=0x1 flags_and=0xffffffff authorized=1
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA:
wlan0: WPA: using MGMT group cipher AES-128-CMAC
wlan0: SME: Selected AP supports MFP: require MFP
WPA: IGTK in EAPOL-Key - hexdump(len=30): [REMOVED]
wlan0: WPA: IGTK keyid 4 pn 000000000000
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.05 sec   11468800 Bytes   18.17 Mbits/sec
```

Note:

The harness command used an outer `timeout`, so the shell returned `124` after
the fixed run window. The WPA2 evidence above appears before that timeout and is
the pass/fail signal for these tests.

### WPA2 PMF optional

Problem:

The first PMF-optional test did not reach association. This was a test harness
issue rather than an 802.11/RSN negotiation failure. The NSH command line was
too long and truncated the supplicant config path:

```text
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-wpa2-psk-pmf-optional.co
Failed to open config file '/h/wpa_supplicant-hwsim-wpa2-psk-pmf-optional.co'
Failed to add interface wlan0
nsh: nf: command not found
```

Fix:

Added short config aliases for use from NSH:

```text
hostapd-pmf-opt.conf
wpa-pmf-opt.conf
```

Validation:

```text
AP:
genl_bridge: complete family=nl80211 cmd=15 ret=0
IGTK - hexdump(len=16): [REMOVED]
wlan0: AP-ENABLED
wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)
nl80211: Set STA flags - ifname=wlan0 addr=02:00:00:00:00:02 total_flags=0x6d flags_or=0x1 flags_and=0xffffffff authorized=1
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA:
wlan0: WPA: using MGMT group cipher AES-128-CMAC
wlan0: SME: Selected AP supports MFP: require MFP
WPA: IGTK in EAPOL-Key - hexdump(len=30): [REMOVED]
wlan0: WPA: IGTK keyid 4 pn 000000000000
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss, time 3030 ms
0.00-   5.06 sec   11436032 Bytes   18.08 Mbits/sec
```

### WPA3 SAE simulator enablement

Initial attempt:

To move WPA3-Personal forward, the NuttX Wi-Fi build was tested with minimal
SAE enablement:

```text
CONFIG_SAE
wpa_supplicant-2.11/src/common/sae.c
wpa_supplicant-2.11/src/common/dragonfly.c
```

Result:

The first AP build reached link time and failed because the current internal
TLS/crypto source list does not provide the SAE/Dragonfly bignum and elliptic
curve API:

```text
undefined reference to `crypto_ec_point_add'
undefined reference to `crypto_ec_point_mul'
undefined reference to `crypto_ec_point_to_bin'
undefined reference to `crypto_bignum_init'
undefined reference to `crypto_bignum_rand'
undefined reference to `crypto_bignum_legendre'
```

Fix:

For simulator builds, a small OpenSSL-backed bignum/ECC glue layer was added:

```text
apps/wireless/wifi/common/nuttx_wpa_openssl_ec.c
```

The sim board link rules add `-lcrypto` when WPA/hostapd are enabled. Two
simulator integration details were fixed:

- OpenSSL static initialization can call `getenv()` before a NuttX task TCB is
  available, so `getenv()` / `get_environ_ptr()` now return safely when there
  is no current task.
- `crypto_bignum_rand()` uses the WPA/NuttX `random_get_bytes()` path instead
  of OpenSSL `BN_rand_range()`, avoiding OpenSSL's own RNG path inside the
  NuttX sim process.
- Multiple NuttX sim instances can start from synchronized random state. The
  simulator bignum glue now mixes process-local time/address/counter data after
  `random_get_bytes()` so AP and STA SAE commits do not reflect identical
  scalars.

Build proof:

```text
./FeatherCore/tools/firmware/sim/build-ap.sh -j8    -> ap_rc=0
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8  -> sta1_rc=0
```

Runtime result:

WPA3-SAE AP + STA1 now passes with:

```text
/tmp/hwsim-sae-ap-randfix.log
/tmp/hwsim-sae-sta1-randfix.log
```

Key proof lines:

```text
SAE: State Confirmed -> Accepted for peer 02:00:00:00:00:02 (Accept Confirm)
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   12632064 Bytes   20.01 Mbits/sec
```

Two-STA WPA3-SAE validation:

```text
/tmp/hwsim-sae-2sta-ap.log
/tmp/hwsim-sae-2sta-sta1.log
/tmp/hwsim-sae-2sta-sta2.log
```

Key proof lines:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
STA1 -> STA2: 10 packets transmitted, 10 received, 0% packet loss
STA1 -> STA2: 0.00-   5.05 sec    5799936 Bytes    9.19 Mbits/sec
STA2 -> STA1: 10 packets transmitted, 10 received, 0% packet loss
STA2 -> STA1: 0.00-   5.05 sec    5718016 Bytes    9.06 Mbits/sec
```

Simultaneous WPA3-SAE STA1<->STA2 iperf stress was then retested and improved
from a link-breaking failure to a functional low-throughput pass.

Problem found during the stress retest:

- Symptom: an AP + STA1 + STA2 WPA3-SAE run associated both STAs and passed
  sequential STA-to-STA ping/iperf, but simultaneous bidirectional iperf could
  leave all AP/STA ARP traffic timing out afterwards. The file-backed medium
  grew to 2.1G:

  ```text
  -rw-rw-r-- 1 uan uan 2.1G ... hwsim-frames.bin
  ```

- Root cause: the NuttX hwsim cross-process medium used an append-only
  `/h/hwsim-frames.bin`. Each radio consumed from its own offset, but a
  full-duplex TCP run could generate frames faster than every simulator could
  drain them. Later ARP/control frames were then stuck behind a huge backlog.
- First attempted fix: protect both writers and readers with a hostfs directory
  lock. This was rejected because `mkdir` succeeded but `rmdir` did not
  reliably release the lock through this hostfs path, leaving
  `hwsim-frames.lock` behind and blocking ARP publication.
- Second attempted fix: use `open(O_CREAT | O_EXCL)` and `unlink` as a
  file-lock. This fixed stale directory locks, but putting the same lock around
  the 20 ms RX poll loop starved writers because AP/STA1/STA2 were constantly
  reacquiring the read lock.
- Final fix: keep the cross-process file lock only around writer-side append
  and bounded truncate. Readers no longer lock; they tolerate partial records
  by retrying on the next poll. The medium now has:

  ```text
  CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES = 64 MiB default
  NUTTX_HWSIM_MEDIUM_MAX_RECORDS_PER_POLL = 512
  writer-side open(O_CREAT | O_EXCL) lock + unlink unlock
  ```

Build proof after the final fix:

```text
./FeatherCore/tools/firmware/sim/build-ap.sh -j8    -> ap_rc=0
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8  -> sta1_rc=0
./FeatherCore/tools/firmware/sim/build-sta2.sh -j8  -> sta2_rc=0
```

Runtime proof after the final fix:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:03

Baseline ping:
STA1 -> STA2: 3 packets transmitted, 3 received, 0% packet loss
STA2 -> STA1: 3 packets transmitted, 3 received, 0% packet loss

Full-duplex run 1:
STA1 -> STA2 iperf: 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec
STA2 -> STA1 iperf: 0.00-   6.02 sec     720896 Bytes    0.96 Mbits/sec
Post-stress ping: both directions 3/3, 0% packet loss

Full-duplex run 2:
STA1 -> STA2 iperf: 0.00-   6.02 sec     802816 Bytes    1.07 Mbits/sec
STA2 -> STA1 iperf: 0.00-   6.02 sec     966656 Bytes    1.28 Mbits/sec
Post-stress ping: both directions 3/3, 0% packet loss
```

2026-06-06 refreshed AP/STA1 WPA3-SAE runtime smoke:

The current `nuttx-sim-ap` and `nuttx-sim-sta1` images were rerun with the
plain WPA3-SAE configs after the later role-object and medium changes:

```text
Logs:
  /tmp/hwsim-wpa3-sae-refresh-ap.log
  /tmp/hwsim-wpa3-sae-refresh-sta1.log

AP:
  hostapd /h/hostapd-hwsim-wpa3-sae.conf &
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

STA1:
  wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-wpa3-sae.conf &
  wlan0: PMKSA-CACHE-ADDED 02:00:00:00:00:01 0
  wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA1 -> AP:
  ping -c 5 192.168.201.1
    -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.201.1 -p 5101 -t 6
    -> 0.00-   6.02 sec    4096000 Bytes    5.44 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.201.1
    -> 3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.201.2
    -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.201.2 -p 5102 -t 6
    -> 0.00-   6.02 sec    4587520 Bytes    6.10 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.201.2
    -> 3 packets transmitted, 3 received, 0% packet loss
```

Problems/observations from this refresh:

- `wpa_cli -i wlan0 status` timed out once after the connection completed.
  This did not block the data path: the STA immediately passed ping and iperf.
  Treat it as a control-interface responsiveness/timing item under heavy debug
  output.
- The shared hwsim medium hit the configured 64 MiB bound during both iperf
  directions and logged `truncating shared frame medium`. Functional traffic
  still passed in both directions, so this is current expected bounded-medium
  behavior rather than a regression.

Current status: simultaneous WPA3-SAE STA-to-STA stress is functionally
passing, but throughput remains much lower than sequential STA-to-STA iperf.
Keep longer repeated stress and medium scheduling/performance tuning as open
work.

Remaining WPA3 work: longer repeated full-duplex stress, throughput tuning, and
any additional SAE-PK/OWE group reruns that are needed after later medium
changes.

### WPA3 SAE-PK AP/STA smoke

Goal:

Enable the SAE-PK variant of WPA3 SAE and prove the AP/STA security path before
moving on to data-plane and multi-STA stress.

Build/config changes:

- Enabled `CONFIG_SAE_PK` in `apps/wireless/wifi/wpa_hostapd_sources.mk`.
- Added `wpa_supplicant-2.11/src/common/sae_pk.c` to the shared
  hostapd/wpa_supplicant source list.
- Added SAE-PK AP/STA simulator configs:
  `tools/firmware/sim/hostapd-hwsim-wpa3-sae-pk.conf` and
  `tools/firmware/sim/wpa_supplicant-hwsim-wpa3-sae-pk.conf`.
- Set `sae_pwe=1` on the STA. Without this, the AP advertises SAE-H2E-only
  membership and wpa_supplicant rejects the BSS; the later log line can look
  like a misleading rate-set mismatch.

Problems found and fixes:

- AP startup originally hung inside OpenSSL EC private-key parsing/allocation
  paths in the NuttX sim environment. The simulator crypto shim now uses a
  local `nuttx_wpa_ec_key` wrapper, parses RFC5915 EC private keys and SPKI
  public keys directly, and performs ECDSA sign/verify through BN/EC_POINT
  primitives.
- STA-side SAE-PK validation originally hung while decoding the AP's compressed
  P-256 public key in `K_AP`. The shim now handles compressed points manually by
  deriving the affine Y coordinate from the curve equation and the compressed
  parity bit.
- AP and STA sim images must be built serially. Running `build-ap.sh` and
  `build-sta1.sh` in parallel corrupts the single NuttX `.config` tree and can
  leave an invalid `arch//include` path. If that happens, remove the generated
  NuttX config artifacts and rerun the build scripts one at a time.

Runtime proof:

```text
/tmp/hwsim-sae-pk-ap-final.log
/tmp/hwsim-sae-pk-sta1-final.log

wlan0: AP-ENABLED
SAE-PK: KeyAuth = Sig_AP()
SAE-PK: Valid K_AP fingerprint
SAE-PK: Valid KeyAuth signature received
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Current status: SAE-PK AP + STA1 authentication, 4-way handshake, and
bidirectional data plane are `PASS`.

2026-06-06 current-image data-plane refresh:

Pending validation items for this pass:

- WPA3 SAE-PK AP + STA1 using independent simulator objects:
  `nuttx-sim-ap` and `nuttx-sim-sta1`.
- AP bring-up with SAE-PK config and PMF required.
- STA association, SAE-PK validation, 4-way handshake, and
  `CTRL-EVENT-CONNECTED`.
- STA->AP and AP->STA ping plus TCP iperf.
- Problem/fix notes for non-blocking compat warnings, first-ARP warm-up, and
  bounded shared-medium behavior.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-sae-pk-data-ap.log
  /tmp/hwsim-sae-pk-data-sta1.log

AP:
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

STA1:
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-sae-pk' freq=2437 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA1 -> AP:
  ping -c 5 192.168.214.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.214.1 -p 5301 -t 6
    0.00-   6.02 sec    1638400 Bytes    2.18 Mbits/sec
  AP server:
    accept: 192.168.214.2:18453
    closed by the peer: 192.168.214.2:18453
    iperf exit
  ping -c 3 192.168.214.1
    3 packets transmitted, 2 received, 33% packet loss

AP -> STA1:
  ping -c 5 192.168.214.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.214.2 -p 5302 -t 6
    0.00-   6.02 sec    2457600 Bytes    3.27 Mbits/sec
  STA1 server:
    accept: 192.168.214.1:24848
    closed by the peer: 192.168.214.1:24848
    iperf exit
  ping -c 3 192.168.214.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1; the supplicant continued, associated, and completed the
  RSN handshake.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block SAE-PK association or data traffic.
- STA1->AP first ARP probe logged `arp_wait failed: -110`, but the final
  five-packet ping summary still passed 5/5.
- STA1->AP immediate post-iperf ping lost the first of three packets; AP->STA1
  post-iperf ping then passed 3/3.
- AP->STA1 iperf hit the configured 64 MiB shared-medium guard:
  `truncating shared frame medium size=67109400 limit=67108864`. The client
  completed, the server closed normally, and post-iperf ping passed 3/3, so
  this remains medium/performance tuning rather than a functional SAE-PK
  failure.

### 802.11a first runtime attempt

Attempt:

Short NSH-friendly open-auth 5 GHz configs were added:

```text
hostapd-a.conf
wpa-a.conf
```

Initial result:

AP startup was blocked during channel selection because channel 36 / 5180 MHz
was exposed with `NO_IR`:

```text
nl80211: Mode IEEE 802.11a: 5180[NO_IR] ...
Frequency 5180 (primary) not allowed for AP mode, flags: 0x853 NO-IR
wlan0: IEEE 802.11 Hardware does not support configured channel
Could not select hw_mode and channel. (-3)
wlan0: AP-DISABLED
```

STA scan still proves that the 5 GHz inventory is visible:

```text
Scan included frequencies: ... 5180 5200 5220 5240 ... 5825
```

Fix and current status:

`mac80211_hwsim_linux.c` now defaults to a custom hwsim regulatory domain under
`CONFIG_ARCH_SIM`, which clears the `NO_IR` block for the non-DFS 5 GHz
validation channel. 802.11a channel 36 and 802.11ac VHT20 channel 36 now pass
AP startup, STA association, ping, and iperf. 5 GHz 802.11ax later passed after
HE userspace support was enabled and the 5 GHz HE configuration was corrected to
enable VHT together with HE.

### 802.11ax / HE 2.4 GHz enablement

Attempt:

Short NSH-friendly HE configs were added:

```text
hostapd-ax.conf
wpa-ax.conf
```

The first runtime attempt started an AP/STA link, but hostapd silently fell
back to non-HE operation:

```text
nl80211: Set freq 2437 (... he_enabled=0 ...)
```

Root cause:

The current NuttX genetlink bridge keeps `GET_WIPHY` unsplit for transport
simplicity, while upstream nl80211 only emits the larger per-iftype HE
capability data in the large/split dump path. As a result, hostapd could see
basic band data but not the AP iftype HE capability block.

Fix:

- Force `nl80211_send_band_rateinfo(..., true)` for the current `GET_WIPHY`
  path so AP iftype data, including HE capabilities, is emitted.
- Increase the `nl80211_get_wiphy()` response allocation from 4096 to 8192
  bytes so the larger capability dump fits.
- Build hostapd/wpa_supplicant with `CONFIG_IEEE80211AX` and include
  `src/ap/ieee802_11_he.c`.

After that fix, AP startup reached real HE mode but failed while setting the
beacon:

```text
nl80211: Set freq 2437 (... he_enabled=1 ...)
nl80211-debug: start_ap parse_beacon failed err=-22 ... he_bss_color=...
nl80211: Beacon set failed: -22
```

Root cause:

`NL80211_ATTR_HE_BSS_COLOR` and the related HE OBSS PD parameter are nested
attributes. The current bridge can present those nested attributes without the
strict `NLA_F_NESTED` flag expected by `nla_parse_nested()`, causing
`-EINVAL` before the AP parameters reach mac80211.

Fix:

`nl80211_parse_he_bss_color()` and `nl80211_parse_he_obss_pd()` now use
`nla_parse_nested_deprecated()`, matching the compatibility style already used
for older nested AP parameters in the same file.

Current evidence:

```text
AP build:  ./FeatherCore/tools/firmware/sim/build-ap.sh -j8    -> ap_rc=0
STA build: ./FeatherCore/tools/firmware/sim/build-sta1.sh -j8  -> sta1_rc=0

/tmp/hwsim-ax-pass-ap.log:
nl80211: Set freq 2437 (... he_enabled=1 ...)
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
* he_capab - hexdump(len=21): ...
0.00-5.05 sec ... 17.65 Mbits/sec

/tmp/hwsim-ax-pass-sta1.log:
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss
0.00-5.05 sec 11206656 Bytes 17.75 Mbits/sec
```

Status:

802.11ax / HE 2.4 GHz basic AP+STA association, ping, and iperf now pass. 5 GHz
HE is covered in the next section. 6 GHz HE, TWT command behavior, DCM
evidence, and HE aggregation evidence remain pending.

### 802.11ax / HE 5 GHz enablement

Attempt:

`hostapd-ax-a.conf` was added for channel 36 / 5180 MHz. The first version used
HT plus HE only:

```text
hw_mode=a
channel=36
ieee80211n=1
ieee80211ax=1
```

Initial result:

The link came up and traffic passed, but it was not accepted as a real HE pass
because the AP did not receive a STA HE capability element:

```text
nl80211: Set freq 5180 (... he_enabled=1 ...)
association request: STA=02:00:00:00:00:02 ...
* ht_capabilities - hexdump(len=26): ...
```

Root cause:

For 5 GHz HE operation, the hostapd configuration also needs VHT enabled. Without
`ieee80211ac=1`, the AP can be started with `he_enabled=1`, but the STA
association path does not advertise the 5 GHz HE capability element.

Fix:

`hostapd-ax-a.conf` now enables VHT together with HE:

```text
ieee80211n=1
ieee80211ac=1
ieee80211ax=1
```

Current evidence:

```text
/tmp/hwsim-ax-a2-pass-ap.log:
Mode: IEEE 802.11a  Channel: 36  Frequency: 5180 MHz
nl80211: Set freq 5180 (... vht_enabled=1, he_enabled=1 ...)
wlan0: AP-ENABLED
* vht_capabilities - hexdump(len=12): ...
* he_capab - hexdump(len=29): ...
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

/tmp/hwsim-ax-a2-pass-sta1.log:
nl80211: Associated on 5180 MHz
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
3 packets transmitted, 3 received, 0% packet loss
0.00-5.05 sec 11206656 Bytes 17.75 Mbits/sec
```

Status:

802.11ax / HE 5 GHz basic AP+STA association, ping, and iperf now pass.

Current-image refresh:

```text
/tmp/hwsim-11ax5-refresh-ap.log
/tmp/hwsim-11ax5-refresh-sta1.log

nl80211-debug: start_ap parse chandef freq=5180
nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=... he_oper=... ht_cap=... vht_cap=...
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: Associated with 02:00:00:00:00:01
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA->AP ping: 5 packets transmitted, 5 received, 0% packet loss
STA->AP iperf: 0.00-   6.02 sec    1146880 Bytes    1.52 Mbits/sec
STA->AP post ping: 3 packets transmitted, 3 received, 0% packet loss
AP->STA ping: 5 packets transmitted, 5 received, 0% packet loss
AP->STA iperf: 0.00-   6.02 sec    1802240 Bytes    2.40 Mbits/sec
AP->STA post ping: 3 packets transmitted, 3 received, 0% packet loss
ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=1
```

Issues noted in the refresh:

- The AP `iperf -s -p 5211` command was accidentally issued twice. The second
  server failed with `tcp_selectport failed: -98` because the first listener was
  already active; the original listener served the STA->AP iperf client.
- The first STA-originated ARP probe logged one `arp_wait failed: -110` before
  the ping summary. The same command still completed 5/5 with 0% loss, so this
  remains the known first-ARP warm-up behavior rather than a HE association
  failure.

### 802.11ax / HE 6 GHz first gate

Attempt:

`hostapd-ax-6g-open.conf` was added as a minimal 6 GHz channel 1 / operating
class 131 AP configuration:

```text
op_class=131
channel=1
ieee80211ax=1
wpa=0
```

Result:

The AP does not reach channel setup. hostapd rejects the configuration during
security validation:

```text
/tmp/hwsim-ax-6g-open-ap.log:
Pre-RSNA security methods are not allowed in 6 GHz
Failed to set up interface with /h/hostapd-ax-6g-open.conf
Failed to initialize interface
```

Root cause:

This is expected hostapd behavior for 6 GHz. Open/pre-RSNA AP mode is not valid
there; PMF and a 6 GHz-valid AKM such as SAE or OWE are required.

Second result:

The original blocker was the missing SAE/Dragonfly bignum/ECC userspace crypto
API. That has since been fixed for the simulator path, and WPA3-SAE plus SAE
H2E pass on 2.4 GHz. Valid 6 GHz SAE/H2E configs were added:

```text
hostapd-ax6-sae.conf
wpa-ax6-sae.conf
```

The first 6 GHz SAE/H2E run still failed before association because the NuttX
nl80211 path forced unsplit wiphy dumps. hostapd only saw 5 GHz inventory ending
at 5925 MHz and could not select 5955 MHz:

```text
nl80211: No channel number found for frequency 5925 MHz
Hardware does not support configured mode
Could not select hw_mode and channel. (-2)
```

Fix:

`nl80211_dump_wiphy_parse()` now honors `NL80211_ATTR_SPLIT_WIPHY_DUMP`, and
`genetlink_bridge.c` repeatedly invokes dump callbacks until all split skb
responses are returned. After that, hostapd and wpa_supplicant both see the
6 GHz channel list starting at 5955 MHz.

The next 6 GHz SAE/H2E run reached authentication but failed as a reflection
attack. AP and STA generated the same SAE commit scalar from synchronized NuttX
sim random state:

```text
SAE: own commit-scalar  09 b6 b4 9b ...
SAE: Peer commit-scalar 09 b6 b4 9b ...
SAE: Drop commit message due to reflection attack
```

Fix:

`apps/wireless/wifi/common/nuttx_wpa_openssl_ec.c` keeps using
`random_get_bytes()` as the base source, then mixes process-local
time/address/counter data into the random buffer used by `crypto_bignum_rand()`.
This keeps the simulator-only OpenSSL glue on the NuttX random path while making
separate sim instances diverge.

Status:

802.11ax / HE 6 GHz SAE/H2E is now `PASS`.

Evidence from `/tmp/hwsim-ax6-sae-entropy-ap.log` and
`/tmp/hwsim-ax6-sae-entropy-sta1.log`:

```text
Mode: IEEE 802.11a  Channel: 1  Frequency: 5955 MHz
nl80211: Set freq 5955 (... he_enabled=1 ... bandwidth=20 MHz ...)
wlan0: AP-ENABLED
wlan0: BSS: Add new id 0 ... SSID 'nuttx-hwsim-ax6-sae' freq 5955
SAE: own commit-scalar  e1 13 ...
SAE: Peer commit-scalar 84 a7 ...
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
AP -> STA: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP: 0.00-   5.05 sec   12845056 Bytes   20.35 Mbits/sec
```

### OWE enablement and AP/STA runtime proof

OWE is now enabled for simulator builds with `CONFIG_OWE`, and the OWE SHA384 /
SHA512 helper sources are included so all OWE groups can link. Two configs were
added for the first AP/STA runtime pass:

```text
tools/firmware/sim/hostapd-hwsim-owe.conf
tools/firmware/sim/wpa_supplicant-hwsim-owe.conf
```

Problems found:

1. `wpa_supplicant` rejected the first config because the ported parser did not
   accept `update_config=0`; the OWE test config now omits that global setting.
2. The first ECDH implementation called OpenSSL `EC_KEY_new_by_curve_name()` /
   compressed-point helpers and could hang inside NuttX sim. The simulator
   `crypto_ecdh_*` glue now uses the existing `crypto_ec` `EC_GROUP`, `BN`, and
   `EC_POINT` primitives directly.
3. OWE uses x-only public keys for group 19. `crypto_ecdh_set_peerkey()` now
   reconstructs the missing y coordinate from the curve equation for the NIST
   P-256 prime instead of using OpenSSL compressed-point decoding.

Build proof after the OWE changes:

```text
./FeatherCore/tools/firmware/sim/build-ap.sh -j8    -> ap_rc=0
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8  -> sta1_rc=0
```

Runtime proof from `/tmp/hwsim-owe-recheck-ap.log` and
`/tmp/hwsim-owe-recheck-sta1.log`:

```text
OWE: PMKID - hexdump(len=16): 61 90 a7 22 97 47 3d 2a e5 70 47 57 33 36 63 5e
wlan0: WPA: RX message 1 of 4-Way Handshake from 02:00:00:00:00:01
wlan0: WPA: Sending EAPOL-Key 2/4
wlan0: WPA: Sending EAPOL-Key 4/4
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Data-plane proof from `/tmp/hwsim-owe-bidir-ap.log` and
`/tmp/hwsim-owe-bidir-sta1.log`:

```text
STA -> AP: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP: 0.00-   5.05 sec   12435456 Bytes   19.70 Mbits/sec
AP -> STA: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA: 0.00-   5.05 sec   11649024 Bytes   18.45 Mbits/sec
```

Status: OWE AP/STA on the 2.4 GHz hwsim path is `PASS`.

6 GHz OWE configs were added after the 2.4 GHz pass:

```text
tools/firmware/sim/hostapd-ax6-owe.conf
tools/firmware/sim/wpa-ax6-owe.conf
```

The 6 GHz OWE probe confirmed that the AP starts on channel 1 / 5955 MHz with
HE enabled, the STA scans the 6 GHz BSS, both peers derive the same OWE PMKID,
and the 4-way handshake completes:

```text
nl80211: Set freq 5955 (... he_enabled=1 ... bandwidth=20 MHz ...)
wlan0: BSS: Add new id 0 BSSID 02:00:00:00:00:01 SSID 'nuttx-hwsim-ax6-owe' freq 5955
OWE: PMKID - hexdump(len=16): b6 04 68 46 f2 c2 b6 a2 1e 0f 7e be 48 c8 da 32
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
```

Data-plane proof from `/tmp/hwsim-ax6-owe-data-ap.log` and
`/tmp/hwsim-ax6-owe-data-sta1.log`:

```text
STA -> AP: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP: 0.00-   5.06 sec   12533760 Bytes   19.82 Mbits/sec
AP -> STA: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA: 0.00-   5.05 sec   12255232 Bytes   19.41 Mbits/sec
```

Status: 6 GHz OWE AP/STA on the hwsim path is `PASS`.

Additional OWE group coverage:

```text
tools/firmware/sim/hostapd-hwsim-owe-g20.conf
tools/firmware/sim/wpa-owe-g20.conf
tools/firmware/sim/hostapd-hwsim-owe-g21.conf
tools/firmware/sim/wpa-owe-g21.conf
```

Earlier result for OWE group 20: `BLOCKED` by the simulator EC backend, not by
the OWE state machine, hostapd/supplicant config parsing, or libnl message
construction. This was later fixed with the simulator-local NuttX P-384 ECDH
wrapper described below.

Evidence from `/tmp/hwsim-owe-g20-probe-sta1.log` and
`/tmp/hwsim-owe-g20-probe-ap.log`:

```text
wlan0: SME: Authentication response: peer=02:00:00:00:00:01 auth_type=0 auth_transaction=2 status_code=0
OWE: Try to use group 20
wlan0: STA 02:00:00:00:00:02 IEEE 802.11: authenticated
```

No Association Request reaches the AP after that point.

A temporary diagnostic build added stage logs around `crypto_ecdh_init()` and
confirmed the stall point:

```text
OWE: Try to use group 20
NuttX EC: ECDH init group 20 begin
NuttX EC: ECDH init group 20 ec ready
```

The next expected stage, local public-key generation, never completed before
the simulator was powered off. Replacing the local ECDH glue with the upstream
OpenSSL EVP keygen/derive shape was also tried. That build still stopped at the
same logical stage and consumed enough heap that follow-on `ping`/`iperf`
commands reported allocation failures:

```text
ERROR: Failed to allocate memory
Out of memory!
```

The temporary EVP experiment and diagnostic prints were then removed so the
known-good OWE group 19 path remains on the lighter direct `BN` / `EC_POINT`
implementation.

The first OWE group 21 probe initially reached local P-521 public-key
generation and built a 129-byte Association Request IE, but
`NL80211_CMD_ASSOCIATE` was never sent. The NuttX/libnl default
`nlmsg_alloc()` size came from `getpagesize()`, which can be smaller in NuttX
than the 4096-byte Linux page size expected by upstream libnl users. The local
libnl port now clamps `default_msg_size` to at least 4096 bytes in
`apps/wireless/wifi/libnl-3.2.25/lib/msg.c`.

Build proof after the libnl message-size fix and sim wpa/hostapd stack update:

```text
./FeatherCore/tools/firmware/sim/build-ap.sh -j8    -> ap_rc=0
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8  -> sta1_rc=0
```

OWE group 21 association proof from `/tmp/hwsim-owe-g21-libnl-ap.log` and
`/tmp/hwsim-owe-g21-libnl-sta1.log`:

```text
OWE: PMKID - hexdump(len=16): 2a 79 21 2c b9 16 66 e4 c3 7d df d5 9c 4d 1a 5a
nl80211: Association request send successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

OWE group 21 data-plane proof was then run as separate single-direction tests
to avoid mixing it with the known simultaneous bidirectional stress issue:

```text
STA -> AP, logs /tmp/hwsim-owe-g21-sta2ap-ap.log and
/tmp/hwsim-owe-g21-sta2ap-sta1.log:
5 packets transmitted, 5 received, 0% packet loss
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec

AP -> STA, logs /tmp/hwsim-owe-g21-ap2sta-ap.log and
/tmp/hwsim-owe-g21-ap2sta-sta1.log:
5 packets transmitted, 5 received, 0% packet loss
0.00-   5.05 sec   11780096 Bytes   18.66 Mbits/sec
```

Status: OWE group 21 AP/STA on the hwsim path is `PASS`.

Group 20 initially remained blocked after the libnl message-size fix. The STA
stopped before creating the OWE Diffie-Hellman Parameter element:

```text
/tmp/hwsim-owe-g20-libnl-sta1.log:
OWE: Try to use group 20

/tmp/hwsim-owe-g20-libnl-ap.log:
wlan0: STA 02:00:00:00:00:02 IEEE 802.11: authenticated
```

Debug probes showed the local secp384r1 public-key generation was the stall.
Switching `crypto_ecdh_init()` to OpenSSL 3 `EVP_EC_gen()` was tested and
rejected because it also stalled in the NuttX sim environment for group 19
(`prime256v1`), which would have regressed already passing OWE coverage.

The accepted fix is simulator-local and group-20-specific:

- `apps/wireless/wifi/common/nuttx_wpa_p384_ecdh.c` builds the NuttX
  `crypto/ecc.c` implementation with `ECC_CURVE=secp384r1` and renamed symbols.
- `apps/wireless/wifi/common/nuttx_wpa_openssl_ec.c` routes only OWE group 20
  ECDH key generation and shared-secret derivation through that wrapper.
- OWE groups 19 and 21 remain on the already verified OpenSSL BN/EC glue.

Build proof after the P-384 wrapper:

```text
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8 -> sta1_rc=0
./FeatherCore/tools/firmware/sim/build-ap.sh -j8   -> ap_rc=0
```

Group 20 association proof from `/tmp/hwsim-owe-g20-p384-clean-ap.log` and
`/tmp/hwsim-owe-g20-p384-clean-sta1.log`:

```text
AP OWE PMKID:  4f f1 17 cd 38 9a 64 ba 4e 72 ea 1c 8e 44 7e f5
STA OWE PMKID: 4f f1 17 cd 38 9a 64 ba 4e 72 ea 1c 8e 44 7e f5
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Group 20 data-plane proof from the same logs:

```text
STA -> AP ping: 5 packets transmitted, 5 received, 0% packet loss
AP -> STA ping: 5 packets transmitted, 5 received, 0% packet loss
STA -> AP iperf: 0.00-   5.05 sec   11730944 Bytes   18.58 Mbits/sec
AP -> STA iperf: 0.00-   5.05 sec   11845632 Bytes   18.77 Mbits/sec
```

OWE group 19 was rerun after backing out the failed EVP experiment to confirm
the fix is local to group 20:

```text
/tmp/hwsim-owe-g19-regress-ap.log
/tmp/hwsim-owe-g19-regress-sta1.log
OWE PMKID matches on AP and STA.
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Status: OWE groups 19, 20, and 21 are now `PASS` for AP/STA hwsim runtime.

### P2P control-interface bring-up

Current work:

- `wpa_cli` is built into the hwsim STA image and registered as an NSH command.
- Added P2P and control-smoke configs:

```text
tools/firmware/sim/wpa_supplicant-hwsim-p2p.conf
tools/firmware/sim/wpa_supplicant-hwsim-ctrl-smoke.conf
```

Problems found and fixes:

1. Missing local socket address family:
   - First P2P control smoke failed with:

     ```text
     socket(PF_UNIX): Unknown error 97
     Address family unsupported: 1
     ```

   - Fix: enable `CONFIG_NET_LOCAL=y` in:

     ```text
     nuttx/boards/sim/sim/sim/configs/hwsim_ap/defconfig
     nuttx/boards/sim/sim/sim/configs/hwsim_sta1/defconfig
     nuttx/boards/sim/sim/sim/configs/hwsim_sta2/defconfig
     ```

2. Local-socket VFS does not implement `chmod()` for the control socket:
   - After enabling local sockets, `wpa_supplicant` reached control socket
     creation but failed with:

     ```text
     chmod[ctrl_interface=/tmp/wpa_supplicant/wlan0]: Unknown error 38
     Failed to initialize control interface '/tmp/wpa_supplicant'.
     ```

   - Fix: `wpa_supplicant/ctrl_iface_unix.c` now treats `ENOSYS`,
     `EOPNOTSUPP`, and `ENOTSUP` from socket-path `chmod()` as non-fatal on
     this VFS.

3. `AF_LOCAL`/`PF_UNIX` datagram control sockets are not a good fit for the
   current NuttX sim path:
   - `wpa_supplicant` could create the local control object, but `wpa_cli`
     could not complete reliable one-shot commands over the local socket path.
   - Fix: for `CONFIG_ARCH_SIM`, the wpa/hostapd source selection now builds
     `CONFIG_CTRL_IFACE_UDP` and uses `ctrl_iface_udp.c`. Non-sim builds keep
     the UNIX control-interface source.
   - Current sim configs use `ctrl_interface=udp:9877`.

4. NuttX flat builtins share process-global state:
   - The first UDP `wpa_cli ping` worked, but later invocations could corrupt
     state or crash because `wpa_cli` static globals, getopt state, and global
     eloop teardown persisted inside the same simulator process.
   - Fix: on NuttX, one-shot `wpa_cli` resets its static/getopt state and
     skips creating/destroying a private eloop unless running interactively or
     with action/reconnect behavior.

5. P2P device initialization now reaches the imported nl80211/mac80211 path:
   - Evidence from `/tmp/hwsim-p2p-ctrl-sta1-chmod.log`:

     ```text
     hwsim-debug: finish_drv_init ifname=p2p-dev-wlan0 ifindex=0 first=1 hostapd=0
     hwsim-debug: finish_drv_init set_mode ok mode=10
     hwsim-debug: finish_drv_init iface up ok
     genl_bridge: pre_doit failed family=nl80211 cmd=54 ret=-19
     ```

   - `cmd=54` is `NL80211_CMD_START_SCHED_SCAN`. Scheduled scan was then
     treated as optional/unsupported on this hwsim path instead of blocking
     normal supplicant/P2P control bring-up.

6. Plain UDP control-interface smoke now passes:

     ```text
     /tmp/hwsim-udp-ctrl-reset-smoke.log
     wpa_cli -i wlan0 ping
     PONG
     wpa_cli -i wlan0 status
     wpa_state=DISCONNECTED
     ip_address=192.168.201.2
     address=02:00:00:00:00:02
     ```

7. Single-STA P2P command smoke now passes far enough to exercise the P2P
   command path:

   ```text
   /tmp/hwsim-p2p-one-sta-smoke.log
   p2p-dev-wlan0 initialized
   wpa_cli -i wlan0 p2p_find 5
   OK
   wpa_cli -i wlan0 p2p_stop_find
   P2P-FIND-STOPPED
   OK
   ```

8. Dual-STA P2P discovery initially exposed two timing/transport bugs:
   - STA1 in `p2p_listen` received STA2's Probe Request and generated a Probe
     Response, but the response did not reach the Generic Netlink bridge. The
     userspace `NL80211_CMD_FRAME` message was too small/fragile and grouped
     all attribute failures into one silent path.
   - Fix: `driver_nl80211.c:nl80211_send_frame_cmd()` now allocates enough
     room for the frame command and logs/adds each required attribute
     individually before sending it through `NL80211_CMD_FRAME`.
   - Evidence from `/tmp/hwsim-p2p-listen-sta1-fix.log`:

     ```text
     nl80211: CMD_FRAME sending len=216 msg_len=1240
     hwsim-debug: roc tx probe_resp sdata=p2p-dev-wlan0 freq=2437 len=216
     hwsim-debug: published BSS frame freq=2437 len=216 path=/h/hwsim-bss.bin
     hwsim-debug: published shared frame fc=0x0050 freq=2437 len=216
     nl80211: Frame TX command accepted (no ACK)
     ```

9. After Probe Response transmit worked, STA2 could still finish its active
   scan before STA1 had time to publish the Probe Response/BSS record on the
   hostfs-backed medium.
   - Fix: `mac80211_hwsim_linux.c:hw_scan_work()` now samples
     `/h/hwsim-bss.bin` again before advancing or completing a scan channel,
     so BSS frames published during the dwell window are delivered to cfg80211.
     Active scan dwell in the simulator was also relaxed to 120 ms to account
     for two independent NuttX simulator processes being scheduled by the host.
   - Dual-STA P2P discovery now passes with STA1 listening and STA2 finding:

     ```text
     /tmp/hwsim-p2p-listen-sta1-dwell.log
     P2P: Created device entry based on Probe Req: 42:00:00:00:00:03
     wpa_cli -i wlan0 p2p_peers
     42:00:00:00:00:03

     /tmp/hwsim-p2p-find-sta2-dwell.log
     hwsim-debug: injected shared BSS frame freq=2437 len=216 path=/h/hwsim-bss.bin
     nl80211: Received scan results (1 BSSes)
     P2P-DEVICE-FOUND 42:00:00:00:00:02 p2p_dev_addr=42:00:00:00:00:02
     wpa_cli -i wlan0 p2p_peers
     42:00:00:00:00:02
     ```

10. P2P GO/client now forms a group and passes sequential bidirectional data
    traffic:

   - Problem found:

     GO-side P2P group formation previously reached EAPOL transmission and then
     failed with an `ENODEV`-class packet socket error:

     ```text
     nl80211: EAPOL TX: Unknown error 19
     ```

   - Root cause:

     The NuttX wpa_supplicant nl80211 driver path was passing the Linux compat
     netdev ifindex into `PF_PACKET sendto()`. The NuttX packet socket layer
     expects the native lower-netdev ifindex, so the group interface could exist
     in the compat registry while packet TX still failed at the native socket
     boundary.

   - Fix:

     `ieee80211_linux_if_indextonative()` was added to translate a Linux compat
     ifindex back to the native ifindex, and the NuttX EAPOL TX path in
     `driver_nl80211.c` now uses that translated value when available.

   - Evidence that the fix is active:

     ```text
     /tmp/hwsim-p2p-go-sta1.log:
     nl80211: EAPOL TX ifindex linux=4 native=3
     P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-HJ" freq=2437 go_dev_addr=42:00:00:00:00:02
     p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:03 p2p_dev_addr=42:00:00:00:00:03

     /tmp/hwsim-p2p-go-sta2.log:
     p2p-wlan0-0: CTRL-EVENT-CONNECTED - Connection to 52:00:00:00:00:02 completed
     P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-HJ" freq=2437 go_dev_addr=42:00:00:00:00:02
     ```

   - Earlier traffic result before the iperf/client-bind fix:

     ```text
     STA1 p2p-wlan0-0: inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0
     STA2 p2p-wlan0-0: inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0
     STA1 -> STA2: 10 packets transmitted, 3 received, 70% packet loss
     STA2 -> STA1: 10 packets transmitted, 3 received, 70% packet loss
     ```

   - Additional problem found:

     `iperf -c 192.168.77.x` selected the base STA source IP
     `192.168.201.x`, so the P2P client attempted to connect from the wrong
     interface and failed with `tcp client connect error, error code: 111`.
     The `-B/--bind` argument was parsed into `cfg.sip`, but client sockets did
     not actually bind that source address.

   - Fix:

     `apps/netutils/iperf/iperf.c` now binds TCP and UDP client sockets to
     `cfg.sip` when `-B <local-ip>` is provided. The client-side bind uses
     port 0 so an iperf server can still listen on port 5001 in the same NuttX
     process during bidirectional test setup.

   - Latest proof after rebuilding STA1/STA2:

     ```text
     ./FeatherCore/tools/firmware/sim/build-sta1.sh -j8 -> sta1_rc=0
     ./FeatherCore/tools/firmware/sim/build-sta2.sh -j8 -> sta2_rc=0

     /tmp/hwsim-p2p-bind-sta1.log:
     P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-YG" freq=2437 go_dev_addr=42:00:00:00:00:02
     p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:03 p2p_dev_addr=42:00:00:00:00:03
     STA1 p2p-wlan0-0: inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0

     /tmp/hwsim-p2p-bind-sta2.log:
     p2p-wlan0-0: WPA: Key negotiation completed with 52:00:00:00:00:02 [PTK=CCMP GTK=CCMP]
     P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-YG" freq=2437 go_dev_addr=42:00:00:00:00:02
     STA2 p2p-wlan0-0: inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0

     STA1 -> STA2:
     10 packets transmitted, 10 received, 0% packet loss
     iperf -c 192.168.77.2 -B 192.168.77.1 -t 6
     0.00-   6.04 sec   15712256 Bytes   20.81 Mbits/sec

     STA2 -> STA1:
     10 packets transmitted, 10 received, 0% packet loss
     iperf -c 192.168.77.1 -B 192.168.77.2 -t 6
     0.00-   6.02 sec   15761408 Bytes   20.95 Mbits/sec
     ```

   - Status:

     P2P GO/client is now `PASS` for sequential bidirectional ping and iperf
     over the P2P group interface.

   - Simultaneous bidirectional stress after the bounded hwsim medium fix:

     ```text
     STA1 / GO:
     wpa_cli -i wlan0 p2p_connect 42:00:00:00:00:03 pbc go_intent=15 freq=2437
     P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-gO" freq=2437 go_dev_addr=42:00:00:00:00:02
     p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:03 p2p_dev_addr=42:00:00:00:00:03
     p2p-wlan0-0: EAPOL-4WAY-HS-COMPLETED 52:00:00:00:00:03
     ifconfig p2p-wlan0-0 192.168.77.1 netmask 255.255.255.0

     STA2 / client:
     wpa_cli -i wlan0 p2p_connect 42:00:00:00:00:02 pbc go_intent=0 freq=2437
     p2p-wlan0-0: WPA: Key negotiation completed with 52:00:00:00:00:02 [PTK=CCMP GTK=CCMP]
     P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-gO" freq=2437 go_dev_addr=42:00:00:00:00:02
     ifconfig p2p-wlan0-0 192.168.77.2 netmask 255.255.255.0

     Baseline ping:
     GO -> client: 3 packets transmitted, 3 received, 0% packet loss
     client -> GO: 3 packets transmitted, 3 received, 0% packet loss

     Full-duplex iperf:
     GO server:     iperf -s -B 192.168.77.1 -p 5081 &
     client server: iperf -s -B 192.168.77.2 -p 5082 &
     GO -> client: iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 6
                  0.00-   6.02 sec     802816 Bytes    1.07 Mbits/sec
     client -> GO: iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 6
                  0.00-   6.02 sec     311296 Bytes    0.41 Mbits/sec

     Post-stress ping:
     GO -> client: 3 packets transmitted, 3 received, 0% packet loss
     client -> GO: 3 packets transmitted, 3 received, 0% packet loss
     ```

     Current status: P2P GO/client simultaneous bidirectional TCP stress is
     functional and no longer a missing proof item. The throughput is much
     lower than sequential P2P iperf, matching the broader simultaneous
     STA-to-STA medium performance limitation. Keep longer repeated P2P runs
     and throughput tuning as follow-up work.

11. WPS PBC AP/STA runtime validation now passes.

   - Added WPS runtime configs:

     ```text
     tools/firmware/sim/hostapd-hwsim-wps.conf
     tools/firmware/sim/wpa_supplicant-hwsim-wps.conf
     ```

   - Build proof:

     ```text
     ./FeatherCore/tools/firmware/sim/build-sta1.sh -j8 -> sta1_rc=0
     ./FeatherCore/tools/firmware/sim/build-ap.sh -j8 -> ap_rc=0
     ```

     AP/STA builds must still be run serially because the helper scripts share
     the same NuttX source/build configuration files; running AP and STA builds
     in parallel can race on `.config`.

   - The first WPS smoke exposed a real NuttX builtin interaction bug:

     ```text
     hostapd_cli -i wlan0 ping
     wlan0: AP-DISABLED
     Segmentation fault (core dumped)
     ```

     `hostapd` and `hostapd_cli` run as builtins in one flat address space, so
     one-shot `hostapd_cli` could initialize and destroy global `eloop` state
     while background `hostapd` was still using it. `hostapd_cli.c` now resets
     NuttX builtin static/getopt state on entry and skips private
     `eloop_init()` / `eloop_destroy()` for one-shot commands that do not need
     an interactive loop, action file, daemon mode, or reconnect mode.

   - Latest proof after the `hostapd_cli` fix:

     ```text
     /tmp/hwsim-wps-ap.log:
     hostapd_cli -i wlan0 ping -> PONG
     wlan0: WPS-PBC-ACTIVE
     WPS: Probe Request for PBC received from 02:00:00:00:00:02
     WPS: Negotiation completed successfully
     wlan0: WPS-REG-SUCCESS 02:00:00:00:00:02 ed0bfab2-2afe-50f2-9fcb-5dc0a86a824e
     wlan0: WPS-SUCCESS
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     AP -> STA: 10 packets transmitted, 10 received, 0% packet loss
     AP -> STA: 0.00-   6.02 sec    1245184 Bytes    1.65 Mbits/sec

     /tmp/hwsim-wps-sta1.log:
     wpa_cli -i wlan0 ping -> PONG
     wlan0: WPS-PBC-ACTIVE
     wlan0: WPS-SUCCESS
     wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     STA -> AP: 10 packets transmitted, 10 received, 0% packet loss
     STA -> AP: 0.00-   6.02 sec    1097728 Bytes    1.46 Mbits/sec
     ```

     This WPS run used `-dd` logging and hwsim debug instrumentation, so the
     throughput is treated as functional proof only.

   - 2026-06-06 repeated/negative WPS PBC refresh:

     Pending items for this refresh were AP/STA independent sim objects,
     hostapd/wpa_supplicant control-interface `PONG`, a short negative window
     where the STA starts `wps_pbc` before the AP, AP PBC completion, WPA2
     data-plane ping/iperf, and a second fresh AP/STA WPS PBC rerun.

     Negative-window proof: in `/tmp/hwsim-wps-repeat1-sta1.log`,
     `wpa_cli -i wlan0 ping` returned `PONG` and early
     `wpa_cli -i wlan0 wps_pbc` logged only `wlan0: WPS-PBC-ACTIVE` during the
     short observation window. Before AP PBC there was no `WPS-SUCCESS`, no
     `WPS-REG-SUCCESS`, no `CTRL-EVENT-CONNECTED`, and no `AP-STA-CONNECTED`.

     First positive run:

     ```text
     /tmp/hwsim-wps-repeat1-ap.log:
     hostapd_cli -i wlan0 ping -> PONG
     wlan0: WPS-PBC-ACTIVE
     wlan0: WPS-REG-SUCCESS 02:00:00:00:00:02 ed0bfab2-2afe-50f2-9fcb-5dc0a86a824e
     wlan0: WPS-SUCCESS
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
     AP -> STA ping: 5 packets transmitted, 5 received, 0% packet loss
     AP -> STA iperf: 0.00-   6.02 sec    1179648 Bytes    1.57 Mbits/sec
     AP -> STA post-iperf ping: 3 packets transmitted, 3 received, 0% packet loss

     /tmp/hwsim-wps-repeat1-sta1.log:
     wpa_cli -i wlan0 ping -> PONG
     wlan0: WPS-PBC-ACTIVE
     wlan0: WPS-SUCCESS
     wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     STA -> AP ping: 5 packets transmitted, 5 received, 0% packet loss
     STA -> AP iperf: 0.00-   6.02 sec    7192576 Bytes    9.56 Mbits/sec
     STA -> AP post-iperf ping: 3 packets transmitted, 3 received, 0% packet loss
     ```

     Second fresh AP/STA rerun:

     ```text
     /tmp/hwsim-wps-repeat2-ap.log:
     hostapd_cli -i wlan0 ping -> PONG
     wlan0: WPS-PBC-ACTIVE
     wlan0: WPS-REG-SUCCESS 02:00:00:00:00:02 ed0bfab2-2afe-50f2-9fcb-5dc0a86a824e
     wlan0: WPS-SUCCESS
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

     /tmp/hwsim-wps-repeat2-sta1.log:
     wpa_cli -i wlan0 ping -> PONG
     wlan0: WPS-PBC-ACTIVE
     wlan0: WPS-SUCCESS
     wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
     ```

     Observed caveats: both positive runs still log the historical WPS/EAP
     failure messages immediately after WPS success, but each run proceeds to
     CCMP key negotiation, connected state, AP-side four-way-handshake
     completion, and data traffic. The first ARP on the STA side may time out
     before retrying successfully. The debug-heavy bidirectional iperf run can
     hit the configured 64 MiB shared-medium truncation guard; the peer closes
     normally and post-iperf ping remains healthy, so this is kept as a bounded
     hostfs medium/performance observation rather than a WPS functional failure.

12. WNM BSS Transition Management runtime validation now passes.

   - Added WNM runtime configs:

     ```text
     tools/firmware/sim/hostapd-hwsim-wnm.conf
     tools/firmware/sim/wpa_supplicant-hwsim-wnm.conf
     ```

   - The first AP start exposed a source-list gap:

     ```text
     Line 13: unknown configuration item 'bss_transition'
     Line 14: unknown configuration item 'wnm_sleep_mode'
     ```

     The local build had STA-side `CONFIG_WNM` and `wnm_sta.c`, but not AP-side
     `CONFIG_WNM_AP` or `src/ap/wnm_ap.c`. `wpa_hostapd_sources.mk` now adds
     `-DCONFIG_WNM_AP` and compiles `wpa_supplicant-2.11/src/ap/wnm_ap.c`.

   - Build proof:

     ```text
     ./FeatherCore/tools/firmware/sim/build-ap.sh -j8 -> ap_rc=0
     ./FeatherCore/tools/firmware/sim/build-sta1.sh -j8 -> sta1_rc=0
     /tmp/hwsim-build-ap-wnm.log: CC: wpa_supplicant-2.11/src/ap/wnm_ap.c
     /tmp/hwsim-build-sta1-wnm.log: CC: wpa_supplicant-2.11/src/ap/wnm_ap.c
     ```

   - Latest AP + STA1 proof over WPA2-PSK:

     ```text
     /tmp/hwsim-wnm-ap.log:
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
     hostapd_cli -i wlan0 ping -> PONG
     WNM: Send BSS Transition Management Request to 02:00:00:00:00:02 req_mode=0x0 disassoc_timer=0 valid_int=0x1 dialog_token=1
     wlan0: BSS-TM-RESP 02:00:00:00:00:02 status_code=1 bss_termination_delay=0
     AP -> STA: 10 packets transmitted, 10 received, 0% packet loss
     AP -> STA: 0.00-   6.02 sec   14548992 Bytes   19.33 Mbits/sec

     /tmp/hwsim-wnm-sta1.log:
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     WNM: BSS Transition Management Request: dialog_token=1 request_mode=0x0 disassoc_timer=0 validity_interval=1
     wlan0: WNM: BSS Transition Management Request did not include candidates
     WNM: Send BSS Transition Management Response to 02:00:00:00:00:01 dialog_token=1 status=1 reason=0 delay=0
     STA -> AP: 10 packets transmitted, 10 received, 0% packet loss
     STA -> AP: 0.00-   6.02 sec   14680064 Bytes   19.51 Mbits/sec
     ```

     Since no candidate BSS was included, STA response `status=1` is expected
     for this smoke case. This validates the WNM action-frame path and confirms
     the data path remains usable after the exchange.

13. WNM Sleep Mode runtime validation now passes for AP + STA1, including
   non-empty TFS Request/Response IE plumbing.

   - Reused the WNM WPA2-PSK runtime configs:

     ```text
     tools/firmware/sim/hostapd-hwsim-wnm.conf
     tools/firmware/sim/wpa_supplicant-hwsim-wnm.conf
     ```

   - No-TFS AP + STA1 proof:

     ```text
     /tmp/hwsim-wnm-sleep-ap.log:
     wlan0: AP-ENABLED
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     Successfully send WNM-Sleep Response frame
     Successfully send WNM-Sleep Response frame
     wlan0: STA 02:00:00:00:00:02 WPA: group key handshake completed (RSN)
     STA -> AP server side: 0.00-   3.02 sec    8161140 Bytes   21.62 Mbits/sec
     AP -> STA client side: 0.00-   6.02 sec   14499840 Bytes   19.27 Mbits/sec

     /tmp/hwsim-wnm-sleep-sta1.log:
     wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     wpa_cli -i wlan0 ping -> PONG
     wpa_cli -i wlan0 wnm_sleep enter interval=5 -> OK
     WNM: Request to send WNM-Sleep Mode Request action=enter to 02:00:00:00:00:01
     Successfully recv WNM-Sleep Response frame (action=0, intval=5)
     wpa_cli -i wlan0 wnm_sleep exit -> OK
     WNM: Request to send WNM-Sleep Mode Request action=exit to 02:00:00:00:00:01
     Successfully recv WNM-Sleep Response frame (action=1, intval=0)
     wlan0: WPA: Not reinstalling already in-use GTK to the driver (keyidx=1 tx=0 len=16)
     STA -> AP ping: 10 packets transmitted, 10 received, 0% packet loss
     STA -> AP client side: 0.00-   6.02 sec   16220160 Bytes   21.56 Mbits/sec
     AP -> STA server side: 0.00-   3.05 sec    7363967 Bytes   19.32 Mbits/sec
     ```

   - Non-empty TFS follow-up proof:

     ```text
     /tmp/hwsim-build-wnm-tfs-ap-fix.log:
     build-ap.sh -j8 -> rc=0
     build/nuttx-sim-ap size: 23175280 bytes

     /tmp/hwsim-build-wnm-tfs-sta1-fix.log:
     build-sta1.sh -j8 -> rc=0
     build/nuttx-sim-sta1 size: 27464800 bytes

     /tmp/hwsim-wnm-tfs-ap-fix.log:
     TFS Req IE(s) found
     nl80211: WNM stored TFS Request IE len=4
     nl80211: WNM generated TFS Response IE - hexdump(len=4): 5c 02 01 00
     AP -> STA ping after WNM exit: 3 packets transmitted, 3 received, 0% packet loss

     /tmp/hwsim-wnm-tfs-sta1-fix.log:
     wpa_cli -i wlan0 wnm_sleep enter interval=5 tfs_req=5b020100
     TFS Resp IE(s) found
     nl80211: WNM accepted TFS Response IE - hexdump(len=4): 5c 02 01 00
     wpa_cli -i wlan0 wnm_sleep exit
     STA -> AP ping after WNM exit: 3 packets transmitted, 3 received, 0% packet loss
     ```

   - Problem/fix noted:

     The local nl80211 driver initially had no `.wnm_oper` callback, so
     hostapd/wpa_supplicant could exchange WNM Sleep action frames but could not
     provide driver-side TFS Request/Response IE handling. A local nl80211
     `.wnm_oper` shim now stores a received TFS Request IE, generates a matching
     TFS Response IE by changing the element ID to `WLAN_EID_TFS_RESP`, accepts
     the STA-side response, and clears state on exit.

     A second issue appeared after adding the shim: hostapd uses the TFS
     response length as an output parameter, so checking the incoming `*buf_len`
     as capacity suppressed the response IE. The helper now writes the output
     length itself. This proves non-empty TFS management-frame IE plumbing.
     Real TFS classifier/filter policy enforcement remains a separate
     follow-up.

14. A-MPDU / ADDBA runtime trigger is now proven.

   - Source capability is present: the hwsim port sets
     `IEEE80211_HW_AMPDU_AGGREGATION`, exports `mac80211_hwsim_ampdu_action()`,
     and imported mac80211 contains ADDBA/DELBA TX/RX paths.

   - Earlier HT20 + WMM AP + STA1 runtime proof:

     ```text
     /tmp/hwsim-ampdu-ap.log:
     nl80211: Set freq 2437 (ht_enabled=1, vht_enabled=0, he_enabled=0, eht_enabled=0, bandwidth=20 MHz, cf1=2437 MHz, cf2=0 MHz)
     wlan0: AP-ENABLED
     HT: STA 02:00:00:00:00:02 HT Capabilities Info: 0x107e
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     AP -> STA client side: 0.00-  15.05 sec   34373632 Bytes   18.27 Mbits/sec

     /tmp/hwsim-ampdu-sta1.log:
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     WMM AC: Valid WMM association, WMM AC is enabled
     STA -> AP client side: 0.00-  15.05 sec   33964032 Bytes   18.05 Mbits/sec
     ```

   - Earlier negative evidence:

     The same logs were searched for `ADDBA`, `DELBA`, `BlockAck`, `AMPDU`,
     `TX_START`, `TX_OPERATIONAL`, `ieee80211_start_tx_ba`, and BA session text.
     There were no matches. That run only proved HT/WMM/QoS data traffic, not a
     real BA session.

   - Fix for the proof gap:

     Added gated `CONFIG_WL_NUTTX_HWSIM_AMPDU_PROOF` trace points around
     `ieee80211_aggr_check()`, `ieee80211_start_tx_ba_session()`, ADDBA TX/RX,
     BA RX setup, and hwsim `.ampdu_action`. The first trace attempt used Linux
     `%pM` MAC formatting, which NuttX `printf` rendered as pointer-like text
     with a trailing `M`; the proof log now uses an explicit
     `%02x:%02x:%02x:%02x:%02x:%02x` formatter.

   - Current HT20 + WMM AP + STA1 runtime proof:

     ```text
     /tmp/hwsim-ampdu-proof-ap.log:
     wlan0: AP-ENABLED
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     ampdu-proof: aggr_check start tid=0 sta=02:00:00:00:00:02 queue=2 priority=0
     ampdu-proof: start_tx_ba_session request sta=02:00:00:00:00:02 tid=0 timeout=0 vif_type=3
     ampdu-proof: start_tx_ba_session queued sta=02:00:00:00:00:02 tid=0 token=1
     ampdu-proof: hwsim ampdu_action action=2 tid=0 sta=02:00:00:00:00:02 buf=0 amsdu=0 ssn=1 timeout=0
     ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 buf=64 timeout=0
     ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
     ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=0
     ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 raw_buf=64 amsdu=0 timeout=0
     ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=0
     ampdu-proof: hwsim ampdu_action action=6 tid=0 sta=02:00:00:00:00:02 buf=64 amsdu=0 ssn=0 timeout=0
     AP -> STA iperf: 0.00-   6.02 sec   13500416 Bytes   17.94 Mbits/sec

     /tmp/hwsim-ampdu-proof-sta1.log:
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     ampdu-proof: aggr_check start tid=0 sta=02:00:00:00:00:01 queue=2 priority=0
     ampdu-proof: start_tx_ba_session request sta=02:00:00:00:00:01 tid=0 timeout=0 vif_type=2
     ampdu-proof: start_tx_ba_session queued sta=02:00:00:00:00:01 tid=0 token=1
     ampdu-proof: hwsim ampdu_action action=2 tid=0 sta=02:00:00:00:00:01 buf=0 amsdu=0 ssn=1 timeout=0
     ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
     ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
     ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=0
     ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=0 timeout=0
     ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=0
     ampdu-proof: hwsim ampdu_action action=6 tid=0 sta=02:00:00:00:00:01 buf=64 amsdu=0 ssn=0 timeout=0
     ping -c 3 192.168.201.1: 3 packets transmitted, 3 received, 0% packet loss
     STA -> AP iperf: 0.00-   6.02 sec   13369344 Bytes   17.77 Mbits/sec
     ```

     Current status is `PASS` for A-MPDU/ADDBA. This particular proof did not
     cover A-MSDU because it reports `amsdu=0`; a later AP queue-backlog proof
     below covers actual A-MSDU construction.

15. HE TWT control/action validation reaches AP request/accept STA parse.

   - Added a dedicated STA config for this test:

     ```text
     tools/firmware/sim/wpa-ax-twt.conf
     ```

     It mirrors the open 11ax STA config and adds `ctrl_interface=udp:9877` plus
     `disable_scan_offload=1`. Without that, the first AX TWT attempt associated
     but `wpa_cli ping` timed out because the open AX smoke config had no UDP
     control interface.

   - Source audit:

     ```text
     hostapd-ax.conf: he_twt_responder=1
     wpa_hostapd_sources.mk: CSRCS += wpa_supplicant-2.11/wpa_supplicant/twt.c
     twt.c: wpas_twt_send_setup() / wpas_twt_send_teardown()
     twt.c and ctrl_iface.c: guarded by CONFIG_TESTING_OPTIONS
     ```

   - Runtime proof of the original control-command gap:

     ```text
     /tmp/hwsim-twt2-ap.log:
     nl80211: Set freq 2437 (ht_enabled=1, vht_enabled=0, he_enabled=1, eht_enabled=0, bandwidth=20 MHz, cf1=2437 MHz, cf2=0 MHz)
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     * he_capab - hexdump(len=21): 01 78 c8 1a 40 00 02 bf ce 00 00 00 00 00 00 00 00 fa ff fa ff

     /tmp/hwsim-twt2-sta1.log:
     ctrl_interface='udp:9877'
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     wpa_cli -i wlan0 ping -> PONG
     wpa_cli -i wlan0 twt_setup -> UNKNOWN COMMAND
     wpa_cli -i wlan0 twt_teardown -> UNKNOWN COMMAND
     ```

   - Fix:

     The sim Wi-Fi build now enables upstream `CONFIG_TESTING_OPTIONS` through
     `apps/wireless/wifi/wpa_hostapd_sources.mk`. This exposes the upstream
     `TWT_SETUP` and `TWT_TEARDOWN` control-interface handlers without enabling
     the test-only commands for non-sim builds.

   - New runtime proof:

     ```text
     /tmp/hwsim-twt3-ap.log:
     nl80211: Set freq 2437 (ht_enabled=1, vht_enabled=0, he_enabled=1, eht_enabled=0, bandwidth=20 MHz, cf1=2437 MHz, cf2=0 MHz)
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

     /tmp/hwsim-twt3-sta1.log:
     ctrl_interface='udp:9877'
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     wpa_cli -i wlan0 ping -> PONG

     wpa_cli -i wlan0 twt_setup
     TWT: Setup request, dtok: 1  exponent: 10  mantissa: 8192  min-twt: 255
     nl80211: Frame TX command accepted; cookie 0x1
     OK
     nl80211: Frame TX status: cookie=0x1 (match) (ack=1)

     wpa_cli -i wlan0 twt_teardown
     TWT: Teardown request, flags: 0x1
     nl80211: Frame TX command accepted; cookie 0x2
     OK
     nl80211: Frame TX status: cookie=0x2 (match) (ack=1)

     /tmp/hwsim-twt3-ap.log:
     hwsim-debug: injected shared frame fc=0x00d0 freq=2437 len=44 src=02:00:00:00:00:02 dst=02:00:00:00:00:01
     cfg80211_rx_mgmt_ext: frame=0x00d0 stype=13 iftype=3 len=44 data_len=20
     hwsim-debug: injected shared frame fc=0x00d0 freq=2437 len=27 src=02:00:00:00:00:02 dst=02:00:00:00:00:01
     cfg80211_rx_mgmt_ext: frame=0x00d0 stype=13 iftype=3 len=27 data_len=3
     ```

   - Follow-up fix:

     AP-side hostapd did not previously see the S1G Action frames because
     `nl80211_action_subscribe_ap()` did not register category 0x17. The sim
     build now registers S1G Action frames under `CONFIG_TESTING_OPTIONS`, and
     AP mode has a minimal sim-only TWT responder that copies the request,
     swaps the addresses, changes the TWT request type to setup command
     `Accept`, and sends the response with `hostapd_drv_send_mlme()`.

   - New runtime proof after the AP responder fix:

     ```text
     /tmp/hwsim-twt4-ap.log:
     wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
     RX_ACTION category 23 action 6 sa 02:00:00:00:00:02 da 02:00:00:00:00:01 len 44 freq 2437
     TWT: AP accept setup from 02:00:00:00:00:02 dialog=1 req_type=0x2878 len=44
     CMD_FRAME - hexdump(len=44): d0 00 ... 17 06 01 d8 0f 10 78 28 ...
     hwsim-debug: published shared frame fc=0x00d0 freq=2437 len=44 src=02:00:00:00:00:01 dst=02:00:00:00:00:02
     nl80211: Frame TX status event A1=02:00:00:00:00:02 stype=13 cookie=0x4 ack=1
     RX_ACTION category 23 action 7 sa 02:00:00:00:00:02 da 02:00:00:00:00:01 len 27 freq 2437
     TWT: AP teardown from 02:00:00:00:00:02 flags=0x01

     /tmp/hwsim-twt4-sta1.log:
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     wpa_cli -i wlan0 ping -> PONG
     wpa_cli -i wlan0 twt_setup -> OK
     hwsim-debug: injected shared frame fc=0x00d0 freq=2437 len=44 src=02:00:00:00:00:01 dst=02:00:00:00:00:02
     wpa_cli -i wlan0 twt_teardown -> OK
     ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
     ```

   - Current result:

     TWT setup/teardown control commands, STA-to-AP S1G Action frame transport,
	     AP-side setup request handling, AP Accept response transmission, STA-side
	     response-frame delivery, teardown reception, and post-exchange data-plane
	     ping are validated. Full negotiated TWT operation is still not proven
	     because the current responder is a minimal sim/test shim; STA-side TWT
	     response state handling, real schedule installation, and PS service-period
	     behavior still need dedicated validation.

   - Follow-up fix for STA-side Accept delivery and parse:

     AP was already publishing the TWT Accept frame onto the shared hwsim
     medium, but the STA supplicant did not receive it as `EVENT_RX_MGMT`
     because the sim-only S1G Action frame registration existed only on the AP
     subscribe path. The non-AP nl80211 subscribe path now registers category
     0x17 under `CONFIG_TESTING_OPTIONS`, and `wpas_twt_rx_action()` parses the
     AP setup response fields for validation logs.

   - New runtime proof after the STA receive/parse fix:

     ```text
     /tmp/hwsim-twt-rx2-ap.log:
     nl80211: Register frame type=0xd0 (WLAN_FC_STYPE_ACTION) ... match=17
     TWT: AP accept setup from 02:00:00:00:00:02 dialog=1 req_type=0x2878 len=44
     TWT: AP teardown from 02:00:00:00:00:02 flags=0x01

     /tmp/hwsim-twt-rx2-sta1.log:
     nl80211: Register frame type=0xd0 (WLAN_FC_STYPE_ACTION) ... match=17
     wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
     wpa_state=COMPLETED
     wpa_cli -i wlan0 twt_setup -> OK
     wlan0: Event RX_MGMT (18) received
     wlan0: Received Action frame: SA=02:00:00:00:00:01 Category=23 DataLen=19 freq=0 MHz
     wlan0: TWT: STA parsed setup response from 02:00:00:00:00:01 dialog=1 control=0x10 req_type=0x2878 setup_cmd=4 requestor=0 trigger=1 implicit=1 flow_type=1 flow_id=0 exponent=10 protection=0 twt=0 min_twt=255 mantissa=8192 channel=0
     wpa_cli -i wlan0 twt_teardown -> OK
     ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
     ```

   - Current result:

     TWT setup/teardown control commands, STA-to-AP S1G Action frame transport,
     AP-side setup request handling, AP Accept response transmission, STA-side
     RX_MGMT delivery, STA-side Accept field parsing, teardown reception, and
     post-exchange data-plane ping are validated. Full negotiated TWT operation
     is still not proven because the current responder is a minimal sim/test
     shim; real schedule installation and PS service-period behavior still need
     dedicated validation.

   - Follow-up fix and runtime proof for testmode-controlled TWT service
     periods:

     A private hwsim testmode switch now exposes `get_twt_sp` and
     `set_twt_sp <0|1>`. This does not install a production TWT scheduler; it
     gives the simulator an explicit service-period gate so a negotiated TWT
     setup/accept exchange can be tied to observable PS behavior.

     Build proof:

     ```text
     /tmp/hwsim-build-twtsp2-ap.log:
     ./tools/firmware/sim/build-ap.sh -j8 -> rc=0
     ../build/nuttx-sim-ap -> size 23151928

     /tmp/hwsim-build-twtsp2-sta1.log:
     ./tools/firmware/sim/build-sta1.sh -j8 -> rc=0
     ../build/nuttx-sim-sta1 -> size 27437360
     ```

     Runtime proof:

     ```text
     /tmp/hwsim-twtsp2-sta1.log:
     wifi_generation=6
     wpa_state=COMPLETED
     3 packets transmitted, 3 received, 0% packet loss
     wlan0: TWT: STA parsed setup response from 02:00:00:00:00:01 dialog=1 control=0x10 req_type=0x2878 setup_cmd=4 requestor=0 trigger=1 implicit=1 flow_type=1 flow_id=0 exponent=10 protection=0 twt=0 min_twt=255 mantissa=8192 channel=0

     hwsim_tm set_ps 1
     hwsim_tm: ps=1
     hwsim_tm: twt_sp=0

     /tmp/hwsim-twtsp2-ap.log:
     TWT: AP accept setup from 02:00:00:00:00:02 dialog=1 req_type=0x2878 len=44
     fc=0x1148
     ps-proof: sta sleep sta=02:00:00:00:00:02 aid=1 num_sta_ps=1 txq_tids=0x0
     ps-proof: buffer unicast sta=02:00:00:00:00:02 aid=1 ac=2 queue=1 total=1
     ps-proof: buffer unicast sta=02:00:00:00:00:02 aid=1 ac=2 queue=2 total=2
     ps-proof: buffer unicast sta=02:00:00:00:00:02 aid=1 ac=2 queue=3 total=3
     3 packets transmitted, 0 received, 100% packet loss

     /tmp/hwsim-twtsp2-sta1.log:
     twt-proof: hwsim service period open
     fc=0x0148
     hwsim_tm: twt_sp=1

     /tmp/hwsim-twtsp2-ap.log:
     fc=0x0148
     ps-proof: sta wake sta=02:00:00:00:00:02 aid=1 driver_ps=0
     ps-proof: wake deliver sta=02:00:00:00:00:02 aid=1 filtered=0 buffered=3 total=3 num_sta_ps=0
     3 packets transmitted, 3 received, 0% packet loss
     0.00-   6.02 sec     901120 Bytes    1.20 Mbits/sec
     iperf exit

     /tmp/hwsim-twtsp2-sta1.log:
     twt-proof: hwsim service period closed
     hwsim_tm: twt_sp=0
     hwsim_tm: ps=0
     3 packets transmitted, 3 received, 0% packet loss

     /tmp/hwsim-twtsp2-ap.log:
     TWT: AP teardown from 02:00:00:00:00:02 flags=0x01
     ```

     Problem found and fixed:
     `set_twt_sp 1` initially did not wake the AP because the hwsim TX path
     forced the PM bit on every frame while `ps != PS_DISABLED`. That rewrote
     the intended PM=0 NullFunc into another PM=1 sleep indication. The fix is
     to skip that PM-bit forcing while `twt_sp_open` is true, so opening the
     service period emits `fc=0x0148` and the AP releases buffered traffic.

     Follow-up automatic-scheduler smoke proof now accepts the observed TWT
     action category 23 in addition to the imported `WLAN_CATEGORY_S1G` value
     22, parses the AP Accept frame on STA RX, and installs a periodic hwsim
     service-period timer from the negotiated parameters. With `dialog=1
     exponent=1 mantissa=50000 min_twt=255`, AP logs `TWT: AP accept setup ...
     req_type=0x0478`, STA logs `TWT: STA parsed setup response ...
     req_type=0x0478`, and installs the periodic scheduler. The original
     negotiated timing produced `interval_jiffies=10 wake_jiffies=6`, which was
     too tight for the current NuttX sim plus hostfs-backed hwsim medium; the
     simulator now clamps automatic TWT proof timing to at least a 1 s interval
     and 500 ms wake window, producing `interval_jiffies=100 wake_jiffies=50`.
     The automatic service-period RX gate also accepts broadcast/multicast
     frames while open, fixing AP-to-STA ARP under TWT.

     Current result is now `TESTMODE SERVICE PERIOD PASS /
     AUTO-SCHEDULER TCP PARTIAL PASS`. The negotiated setup/accept/teardown
     exchange, off-SP AP buffering, open-SP wake/deliver, AP-to-STA ping,
     AP-to-STA iperf, post-teardown STA-to-AP ping, automatic scheduler
     installation/periodic toggling, and automatic-scheduler data-plane ping are
     proven. Latest automatic run: AP accepts setup, STA installs
     `interval_jiffies=100 wake_jiffies=50`, STA-to-AP ping passes 2/2 after
     one initial ARP retry, AP-to-STA ping passes 3/3 after ARP/path warm-up,
     STA-to-AP iperf reports 294912 Bytes over 6.02 s at ~0.39 Mbits/sec,
     AP-to-STA iperf reports 327680 Bytes over 6.02 s at ~0.44 Mbits/sec,
     `twt_teardown flags=1` returns `OK`, the STA disables the local scheduler
     with `auto disabled reason=tx-teardown restore_ps=1`, and the AP receives
     teardown. Remaining gaps: the first AP-originated ARP
     before STA traffic can still miss the service-period window, AP-to-STA TCP
     can fill the 512-frame sleep buffer, and the hostfs hwsim medium can still
     hit the configured bounded-medium limit under this debug-heavy long flow,
     so tighter service-period timing, queue release, and medium/performance
     tuning remain pending. Evidence: `/tmp/hwsim-twt-iperf1-ap.log` and
     `/tmp/hwsim-twt-iperf1-sta1.log`.

16. Open AP + two STA ping/iperf retest

   A clean three-process open-auth run was repeated with AP, STA1, and STA2:

   ```text
   AP:
   wlan0: AP-ENABLED
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:03

   STA1 -> STA2:
   ping -c 3 192.168.201.3
   3 packets transmitted, 3 received, 0% packet loss
   iperf -c 192.168.201.3 -t 5
   0.00-   6.02 sec    6750208 Bytes    8.97 Mbits/sec

   STA2 -> STA1:
   ping -c 3 192.168.201.2
   3 packets transmitted, 3 received, 0% packet loss
   iperf -c 192.168.201.2 -t 5
   0.00-   6.02 sec    6750208 Bytes    8.97 Mbits/sec
   ```

   Evidence logs: `/tmp/hwsim-sta2-ap.log`, `/tmp/hwsim-sta2-sta1.log`, and
   `/tmp/hwsim-sta2-sta2.log`. This confirms sequential bidirectional STA
   traffic through the AP data path. Simultaneous bidirectional stress remains a
   separate hardening item.

   2026-06-06 quick retest after the EAP-TLS work used the same open-auth AP +
   STA1 + STA2 setup. Both STAs associated with the AP:

   ```text
   AP:
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:03

   STA1 -> STA2:
   ping -c 3 192.168.201.3
   3 packets transmitted, 3 received, 0% packet loss
   iperf -c 192.168.201.3 -B 192.168.201.2 -p 5071 -t 5
   0.00-   6.02 sec     606208 Bytes    0.81 Mbits/sec

   STA2 -> STA1:
   ping -c 3 192.168.201.2
   3 packets transmitted, 3 received, 0% packet loss
   iperf -c 192.168.201.2 -B 192.168.201.3 -p 5072 -t 5
   0.00-   6.02 sec     475136 Bytes    0.63 Mbits/sec
   ```

   This confirms that current open-auth STA-to-STA forwarding still works.
   Throughput is lower than the earlier sequential run, so throughput tuning
   remains open.

   2026-06-06 current-session retest split the result by runtime path:

   - Full `hostapd` + `wpa_supplicant` path: PASS. AP reported both stations
     connected, STA1 (`192.168.201.2`) and STA2 (`192.168.201.3`) both reported
     `CTRL-EVENT-CONNECTED`, STA1->STA2 ping returned 3/3 packets, STA2->STA1
     ping returned 3/3 packets, STA1->STA2 iperf reached about 0.83 Mbits/sec,
     and STA2->STA1 iperf reached about 0.96 Mbits/sec. Evidence:
     `/tmp/hwsim-open2sta-ap.log`, `/tmp/hwsim-open2sta-sta1.log`, and
     `/tmp/hwsim-open2sta-sta2.log`.
   - Minimal `wifi_ap_demo` + `wifi_sta_demo` path: FAIL for STA-to-STA data.
     Both STA-side pings failed at ARP resolution and both iperf clients failed
     with `error code: 101`. Evidence: `/tmp/hwsim-sta2sta-ap.log`,
     `/tmp/hwsim-sta2sta-sta1.log`, and `/tmp/hwsim-sta2sta-sta2.log`.
     This should be tracked separately from the imported mac80211/nl80211 +
     hostapd/wpa_supplicant path, which is the path used for the full Wi-Fi
     validation matrix.

   2026-06-06 AP + STA1 + STA2 + STA3 open-auth run extended the independent
   role proof to four separate simulator processes:

   ```text
   AP:
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:04

   STA3 -> AP:
   ping -c 3 192.168.201.1
   3 packets transmitted, 3 received, 0% packet loss

   STA1 -> STA3:
   ping -c 3 192.168.201.4
   3 packets transmitted, 3 received, 0% packet loss
   iperf -c 192.168.201.4 -p 5301 -t 10
   0.00-  12.04 sec    1081344 Bytes    0.72 Mbits/sec

   STA2 -> STA3:
   ping -c 3 192.168.201.4
   3 packets transmitted, 3 received, 0% packet loss

   STA3 -> STA1:
   ping -c 3 192.168.201.2
   3 packets transmitted, 3 received, 0% packet loss
   iperf -c 192.168.201.2 -p 5302 -t 10
   0.00-  12.04 sec    2293760 Bytes    1.52 Mbits/sec
   ```

   Evidence logs: `/tmp/hwsim-open-3sta-ap.log`,
   `/tmp/hwsim-open-3sta-sta1.log`, `/tmp/hwsim-open-3sta-sta2.log`, and
   `/tmp/hwsim-open-3sta-sta3.log`. STA1/STA2 first probes to STA3 each logged
   one or two ARP warm-up misses, but the final ICMP summaries were 3/3 with
   0% packet loss and both iperf clients printed `iperf exit`.

   2026-06-06 follow-up after wiring virtual hwsim WEXT ioctls into the
   imported cfg80211 WEXT handlers and accepting `SIOCSIWCOMMIT` in the NuttX
   netdev upper half narrowed the minimal-demo failure:

   ```text
   wifi_ap_demo
   wifi_ap_demo: role=ap if=wlan0 ssid=nuttx-hwsim ip=192.168.201.1 mask=255.255.255.0
   wifi_ap_demo: expected sta peers: 192.168.201.2 192.168.201.3
   wifi_ap_demo: SIOCSIWMODE(wlan0) failed: -22
   ```

   Evidence: `/tmp/hwsim-demo-wext-ap.log`. This is now an AP control-plane
   limitation rather than an ambiguous STA-to-STA data-plane failure. The
   imported Linux `cfg80211_wext_siwmode()` accepts WEXT `IW_MODE_INFRA`,
   `IW_MODE_ADHOC`, and `IW_MODE_MONITOR`, but rejects `IW_MODE_MASTER` with
   `-EINVAL`. Starting a real AP therefore needs the full nl80211
   `START_AP`/hostapd path, or a dedicated simulator AP helper that builds
   cfg80211 AP settings. The minimal WEXT demo remains outside the current
   full-stack validation pass criteria.

17. Static PS control-path smoke

   A clean AP + STA1 open-auth run was used to validate the standard nl80211
   power-save control path. The STA used `/h/wpa-ps.conf` so `wpa_cli` could
   reach the supplicant over the UDP control interface.

   ```text
   /tmp/hwsim-ps-ap.log:
   wlan0: AP-ENABLED
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

   /tmp/hwsim-ps-sta1.log:
   wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
   wpa_cli -i wlan0 ping -> PONG

   wpa_cli -i wlan0 p2p_set ps 1
   nl80211: set_p2p_powersave (legacy_ps=1 opp_ps=-1 ctwindow=-1)
   genl_bridge: enter family=nl80211 cmd=61 flags=0x5 len=36 dump=0
   genl_bridge: complete family=nl80211 cmd=61 ret=0
   genl_bridge: ack type=19 cmd=61 seq=1514764863 error=0
   OK
   ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss

   wpa_cli -i wlan0 p2p_set ps 0
   nl80211: set_p2p_powersave (legacy_ps=0 opp_ps=-1 ctwindow=-1)
   genl_bridge: enter family=nl80211 cmd=61 flags=0x5 len=36 dump=0
   genl_bridge: complete family=nl80211 cmd=61 ret=0
   genl_bridge: ack type=19 cmd=61 seq=1514764864 error=0
   OK
   ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
   ```

   Current result:

   - Standard static PS control reaches `NL80211_CMD_SET_POWER_SAVE` and returns
     success through the imported cfg80211/mac80211 path.
   - Data traffic remains alive after toggling PS on and off.
   - Later hwsim-private validation now proves nullfunc and PS-Poll frame
     emission, buffered-frame delivery, and dynamic idle-timeout behavior; this
     standard nl80211 smoke alone still does not prove AP link PS semantics.

18. DCM and hwsim private testmode audit

   DCM is now a simulator runtime proof item, not only a source/control-path
   audit. This pass also keeps the existing build and runtime proof for the
   hwsim-private PS testmode latch and basic queue stop/wake behavior.

   DCM plumbing evidence:

   ```text
   drivers/wireless/virtual/mac80211_hwsim_linux.c:
     HWSIM_RATE_INFO_ATTR_HE_DCM is accepted by hwsim_rate_info_policy
     mac80211_hwsim_parse_rate_info() copies it into rate_info->he_dcm

   wireless/ieee80211/mac80211/status.c:
     radiotap HE DATA3_DATA_DCM can be filled from status_rate->rate_idx.he_dcm

   wireless/ieee80211/cfg80211/nl80211.c:
     NL80211_RATE_INFO_HE_DCM can be emitted when rate_info->he_dcm is set
   ```

   Previous gap:

   ```text
   Imported hwsim HE capability blocks left the remaining PHY bytes unset and
   documented DCM as unsupported, so hostapd/wpa_supplicant could exercise HE
   association but no runtime frame could prove he_dcm.
   ```

   Fix:

   ```text
   CONFIG_WL_NUTTX_HWSIM_DCM_PROOF=y

   drivers/wireless/virtual/mac80211_hwsim_linux.c:
     HWSIM_HE_DCM_PHY_CAPS advertises HE DCM BPSK Tx/Rx and max RU 484
     /h/hwsim-dcm-proof gates runtime RX status injection
     dcm-proof: rx status ... encoding=HE ... he_dcm=1
   ```

   The marker file is deliberate: DCM proof builds can advertise the capability,
   but non-DCM data tests are not reported as HE DCM traffic unless
   `/h/hwsim-dcm-proof` exists in each simulator's shared hostfs view.

   Runtime result from 2026-06-06 AP+STA1 11ax channel 6:

   ```text
   AP:
   mkdir /h
   mount -t hostfs -o fs=. /h
   hostapd /h/hostapd-ax.conf &
   wlan0: AP-ENABLED
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

   STA1:
   mkdir /h
   mount -t hostfs -o fs=. /h
   wpa_supplicant -i wlan0 -c /h/wpa-ax.conf &
   wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

   DCM proof logs:
   dcm-proof: rx status path=direct freq=2437 bw=0 encoding=HE mcs=0 nss=1 he_gi=0 he_dcm=1
   dcm-proof: rx status path=shared freq=2437 bw=0 encoding=HE mcs=0 nss=1 he_gi=0 he_dcm=1

   Data proof:
   ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
   STA -> AP iperf: 0.00-   6.02 sec   13647872 Bytes   18.14 Mbits/sec
   AP -> STA iperf: 0.00-   6.02 sec   13680640 Bytes   18.18 Mbits/sec
   ```

   Build/runtime problems and fixes found during this DCM pass:

   - Problem: AP and STA sim builds were first launched in parallel in the same
     NuttX source tree. The two configurations raced and left generated
     dependency files such as `apps/wireless/wifi/Make.dep` unusable, causing
     `Make.dep: multiple target patterns` during the next clean.
   - Fix: remove generated `Make.dep`/`.depend`/object dependency files after
     the interrupted build and rebuild AP/STA images serially. Use `-type f`
     when cleaning broad patterns, for example `find apps nuttx -type f \( -name
     Make.dep -o -name .depend -o -name '*.ddc' -o -name '*.d' -o -name '*.o'
     -o -name '*.a' -o -name '*.la' \) -delete`, so directories such as
     `init.d` are not matched. Keep hwsim image builds serial unless separate
     out-of-tree build directories are introduced.
   - Problem: the current generated `.config` can compile the IEEE80211 compat
     auto-netdev path without `CONFIG_NETDEV_WIRELESS_HANDLER`, while
     `struct netdev_lowerhalf_s.iw_ops` only exists under that option.
   - Fix: guard `g_auto_netdev_iw_ops` and `lower.iw_ops` assignment with
     `CONFIG_NETDEV_WIRELESS_HANDLER`. The nl80211/cfg80211 runtime path does
     not depend on the legacy wireless-extension handler.
   - Problem: `/h` is not mounted by default in these simulator sessions, so
     `hostapd /h/hostapd-ax.conf` fails to open its config.
   - Fix: run `mkdir /h` and `mount -t hostfs -o fs=. /h` in each AP/STA sim
     before starting hostapd or wpa_supplicant.

   Current hwsim private testmode evidence:

   ```text
   wireless/ieee80211/cfg80211/Kconfig:
     config NL80211_TESTMODE

   wireless/ieee80211/Kconfig:
     wrapper config NL80211_TESTMODE for the NuttX Linux-wireless port

   drivers/wireless/virtual/mac80211_hwsim_linux.c:
     HWSIM_TM_CMD_SET_PS
     HWSIM_TM_CMD_GET_PS
     HWSIM_TM_CMD_STOP_QUEUES
     HWSIM_TM_CMD_WAKE_QUEUES

   wireless/ieee80211/mac80211/cfg.c:
     ieee80211_testmode_cmd() forwards cfg80211 testmode to driver ops

   apps/examples/hwsim_testmode:
     hwsim_tm sends nested NL80211_ATTR_TESTDATA commands
   ```

   Build fix:

   - `CONFIG_NL80211_TESTMODE=y` is now enabled in `hwsim_ap`,
     `hwsim_sta1`, and `hwsim_sta2` defconfigs.
   - `CONFIG_EXAMPLES_HWSIM_TESTMODE=y` adds the NSH command `hwsim_tm` to the
     three hwsim simulator images.
   - AP, STA1, and STA2 builds all register `hwsim_tm`. Latest sequential
     build proof:

     ```text
     ./tools/firmware/sim/build-ap.sh -j8   -> PASS, Register: hwsim_tm
     ./tools/firmware/sim/build-sta1.sh -j8 -> PASS, Register: hwsim_tm
     ./tools/firmware/sim/build-sta2.sh -j8 -> PASS, Register: hwsim_tm
     ```

   PS-latch runtime proof on a standalone STA sim:

   ```text
   nsh> hwsim_tm get_ps
   hwsim_tm: ps=0
   hwsim_tm: OK

   nsh> hwsim_tm set_ps 1
   hwsim_tm: OK

   nsh> hwsim_tm get_ps
   hwsim_tm: ps=1
   hwsim_tm: OK

   nsh> hwsim_tm set_ps 0
   hwsim_tm: OK

   nsh> hwsim_tm get_ps
   hwsim_tm: ps=0
   hwsim_tm: OK
   ```

   Queue stop/wake source behavior:

   ```text
   drivers/wireless/virtual/mac80211_hwsim_linux.c:
     HWSIM_TM_CMD_STOP_QUEUES -> ieee80211_stop_queues(hw)
     HWSIM_TM_CMD_WAKE_QUEUES -> ieee80211_wake_queues(hw)

   wireless/ieee80211/mac80211/util.c:
     ieee80211_stop_queues()
     ieee80211_wake_queues()
     reason: IEEE80211_QUEUE_STOP_REASON_DRIVER

   wireless/ieee80211/mac80211/tx.c:
     TX checks local->queue_stop_reasons[q] before dequeuing new frames
   ```

   Queue stop/wake runtime proof on the full hostapd/wpa_supplicant path:

   ```text
   AP:
   mkdir /h
   mount -t hostfs -o fs=. /h
   hostapd -dd /h/hostapd-hwsim.conf &
   wlan0: AP-ENABLED

   STA:
   mkdir /h
   mount -t hostfs -o fs=. /h
   wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim.conf &
   wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
   ifconfig wlan0 192.168.201.3 netmask 255.255.255.0

   Baseline:
   ping -c 3 192.168.201.1
   3 packets transmitted, 3 received, 0% packet loss

   Stop queues:
   hwsim_tm stop_queues
   hwsim_tm: OK
   ping -c 3 192.168.201.1
   3 packets transmitted, 0 received, 100% packet loss

   Wake queues:
   hwsim_tm wake_queues
   hwsim_tm: OK
   ping -c 3 192.168.201.1
   WARNING: Ignoring ICMP reply with ID 2. Expected 3
   3 packets transmitted, 3 received, 0% packet loss
   ```

   Interpretation: `stop_queues` gates new STA TX strongly enough to make ICMP
   fail, and `wake_queues` restores traffic. The ignored old ICMP replies after
   wake are consistent with frames from the stopped ping draining after the
   queue is released. This is a basic ping-level behavior proof, not a complete
   iperf or long-flow stress proof.

   Problem found and fix:

   - Problem: running `build-ap.sh` and `build-sta2.sh` in parallel corrupted
     the shared `nuttx/.config`, leaving an incomplete `arch//...` state.
   - Fix: restore the two dummy Kconfig placeholders that were accidentally
     removed while cleaning the broken config state, then run hwsim board builds
     sequentially. These scripts all configure the same NuttX tree, so they must
     not be used concurrently unless separate build trees are introduced.
   - Problem: the lightweight `wifi_ap_demo` / `wifi_sta_demo` path did not
     establish a pingable AP/STA link for this queue test; the STA side timed
     out waiting for ARP and ping finished with 0/3 received.
   - Fix: use the already-proven hostapd/wpa_supplicant path for queue
     stop/wake behavior validation. A later WEXT-bridge retest showed the
     lightweight path now fails earlier and honestly at AP mode setup:
     `SIOCSIWMODE(IW_MODE_MASTER)` returns `-EINVAL` from Linux cfg80211 WEXT
     compat. The right next fix is not to fake WEXT success, but to drive AP
     setup through nl80211 `START_AP`/hostapd or a dedicated AP settings helper.
   - Problem: when launching from `tools/firmware/sim`, `/h` is not mounted by
     default, so `hostapd -dd /h/hostapd-hwsim.conf` fails to open its config.
   - Fix: run `mkdir /h` and `mount -t hostfs -o fs=. /h` in each simulator
     before launching hostapd or wpa_supplicant.

   Queue stop/wake iperf long-flow proof after the bounded hwsim medium fix:

   ```text
   AP:
   iperf -s -p 5061 &

   STA:
   iperf -c 192.168.201.1 -p 5061 -t 15 &
   sleep 3
   hwsim_tm stop_queues
   hwsim_tm: OK
   sleep 5
   hwsim_tm wake_queues
   hwsim_tm: OK

   STA iperf:
     0.00-   3.01 sec     360448 Bytes    0.96 Mbits/sec
     3.01-   6.02 sec          0 Bytes    0.00 Mbits/sec
     6.02-   9.03 sec      32768 Bytes    0.08 Mbits/sec
     9.03-  12.04 sec     360448 Bytes    0.96 Mbits/sec
    12.04-  15.05 sec     196608 Bytes    0.52 Mbits/sec
     0.00-  15.05 sec     950272 Bytes    0.51 Mbits/sec

   AP iperf:
     0.00-   3.01 sec     176488 Bytes    0.47 Mbits/sec
     3.01-   6.02 sec          0 Bytes    0.00 Mbits/sec
     6.02-   9.03 sec      21440 Bytes    0.05 Mbits/sec
     9.03-  12.04 sec     373056 Bytes    0.99 Mbits/sec
    12.04-  15.05 sec     326174 Bytes    0.87 Mbits/sec

   Post-stress:
   ping -c 3 192.168.201.1
   3 packets transmitted, 3 received, 0% packet loss
   ```

   Interpretation: `stop_queues` now has TCP long-flow proof, not only ICMP
   proof. The active STA->AP iperf stream drops to zero during the stopped
   queue window and recovers after `wake_queues`; AP and STA interval reports
   agree on the pause/recovery shape. This closes the previous private-testmode
   queue stop/wake iperf proof gap.

   Remaining private-testmode/PS work after the queue stop/wake proof:
   PS-Poll/nullfunc frame emission, buffered-frame delivery, dynamic PS, and
   AP-side sleeping-STA delivery semantics still needed dedicated proof at this
   point in the log. The next section closes the PS-Poll/nullfunc emission gap.

19. hwsim PS nullfunc and PS-Poll frame proof

   Tooling change: `apps/examples/hwsim_testmode` now exposes the upstream hwsim
   private PS modes needed for this proof. `hwsim_tm set_ps <0|1|2|4>` covers
   disabled, enabled, auto-poll latch, and dynamic idle-timeout states, while
   `hwsim_tm poll` sends the manual `PS_MANUAL_POLL` testmode command after
   the STA is already in `PS_ENABLED`.

   The first PS-Poll run showed a useful split: the `hwsim_tm poll` command
   returned `OK`, but the shared medium did not contain the expected PS-Poll
   frame-control bytes (`a4 10`). `set_ps 1` and `set_ps 0` did emit nullfunc
   frames (`48 11` and `48 01`), so the issue was below the testmode command
   path rather than in nl80211 dispatch.

   Root cause: `hwsim_send_ps_poll()` creates a standard 16-byte control frame,
   while `nuttx_hwsim_medium_publish_frame()` only accepted frames with length
   >= 24 because it assumed `struct ieee80211_hdr` address fields. As a result,
   PS-Poll frames were transmitted inside the local hwsim path but were not
   written to the hostfs-backed shared medium used by separate AP/STA simulator
   processes.

   Fix: `nuttx_hwsim_medium_publish_frame()` now accepts short PS-Poll control
   frames, fills the shared-medium record from `struct ieee80211_pspoll`
   (`ta` as source and `bssid` as destination/BSSID), and keeps the existing
   24-byte minimum for non-PS-Poll management/data frames.

   Rebuild proof:

   ```text
   ./tools/firmware/sim/build-ap.sh -j8
     -> rc=0
     -> build/nuttx-sim-ap
     -> size: 22213768 bytes

   ./tools/firmware/sim/build-sta1.sh -j8
     -> rc=0
     -> build/nuttx-sim-sta1
     -> size: 26159688 bytes
   ```

   Runtime proof used AP + STA1 with WPA2-PSK/CCMP, because this item validates
   PS control frames rather than encryption coverage:

   ```text
   /tmp/hwsim-pspoll-ap.log:
   wlan0: AP-ENABLED
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

   /tmp/hwsim-pspoll-sta1.log:
   wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
   wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

   baseline ping:
   3 packets transmitted, 3 received, 0% packet loss

   hwsim_tm get_ps -> ps=0, OK
   hwsim_tm set_ps 1 -> OK
   hwsim_tm get_ps -> ps=1, OK
   hwsim_tm poll -> OK
   hwsim_tm set_ps 0 -> OK
   hwsim_tm get_ps -> ps=0, OK
   post manual-poll/wake ping:
   3 packets transmitted, 3 received, 0% packet loss

   hwsim_tm set_ps 2 -> OK
   hwsim_tm get_ps -> ps=2, OK
   hwsim_tm set_ps 0 -> OK
   hwsim_tm get_ps -> ps=0, OK
   post auto-latch/wake ping:
   3 packets transmitted, 3 received, 0% packet loss
   ```

   Shared medium frame-control scan:

   ```text
   hwsim-frames.bin size: 542340 bytes
   hwsim-bss.bin size: 4120 bytes

   xxd -p hwsim-frames.bin | tr -d '\n' | rg -o '4811|4801|a410' | sort | uniq -c
         2 4801
         2 4811
         1 a410
   ```

   Interpretation: the current port now proves explicit hwsim PS latch control,
   PM=1 nullfunc emission, manual PS-Poll emission, PM=0 nullfunc wake emission,
   and data-plane recovery after wake. `PS_AUTO_POLL` is only proven as a mode
   latch plus wake/recovery path here. Full AP buffered-frame delivery, dynamic
   PS timeout behavior, and AP-side sleeping-STA semantics remain open.

20. Static PS buffered-frame and PS-Poll delivery proof

   This pass closes the AP-side sleeping-STA buffered-frame path for static PS.
   Two shared-medium gaps showed up during validation:

   - Short PS-Poll frames reached the shared medium publisher after the previous
     transmit fix, but the receiver-side shared-medium filter still required a
     24-byte 802.11 header. That rejected the standard 16-byte PS-Poll control
     frame before mac80211 could process it. The receive filter now accepts
     `sizeof(struct ieee80211_pspoll)` for PS-Poll while keeping the 24-byte
     minimum for normal management/data frames.
   - The hostfs-backed shared frame consumer injected buffered data frames
     directly into mac80211 without calling the hwsim PS receive gate. The
     buffered frame was delivered, but the STA-side testmode state stayed in
     `PS_MANUAL_POLL`. `nuttx_hwsim_medium_consume_frames()` now calls
     `hwsim_ps_rx_ok()` before injection, and the helper also matches the
     radio's wiphy address list when the active interface iterator does not
     report the own address in the sim port.

   Rebuild proof:

   ```text
   /tmp/hwsim-build-psproof-ap6.log:
   ap_rc=0
   file: ../build/nuttx-sim-ap
   size: 23146688 bytes

   /tmp/hwsim-build-psproof-sta1-6.log:
   sta1_rc=0
   file: ../build/nuttx-sim-sta1
   size: 27433136 bytes
   ```

   Runtime proof:

   ```text
   /tmp/hwsim-psbuffer5-ap.log:
   wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
   baseline AP->STA ping: 3 packets transmitted, 3 received, 0% packet loss
   ps-proof: sta sleep sta=02:00:00:00:00:02 aid=1 num_sta_ps=1
   ps-proof: buffer unicast sta=02:00:00:00:00:02 aid=1 ac=2 queue=1 total=1
   ps-proof: tim sta=02:00:00:00:00:02 aid=1 set=1 total=1
   sleep-period AP->STA ping: 1 packets transmitted, 0 received, 100% packet loss
   hwsim-debug: injected shared frame fc=0x10a4 ... len=16
   ps-proof: pspoll rx sta=02:00:00:00:00:02 aid=1 driver_ps=0
   ps-proof: deliver response ... requested=1 frames=1 ... more=0
   ps-proof: allow buffered ... tids=0x1 num=1 more=0
   ps-proof: tim sta=02:00:00:00:00:02 aid=1 set=0 total=0
   ps-proof: sta wake sta=02:00:00:00:00:02 aid=1 driver_ps=0
   ps-proof: wake deliver ... buffered=0 total=0 num_sta_ps=0

   /tmp/hwsim-psbuffer5-sta1.log:
   wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
   hwsim_tm: ps=1
   ps-proof: hwsim manual poll armed
   ps-proof: hwsim manual poll rx one
   hwsim_tm: ps=1
   hwsim_tm: ps=0
   post-wake STA->AP ping: 3 packets transmitted, 3 received, 0% packet loss
   ```

   Interpretation: static PS now proves not only explicit control and frame
   emission, but also AP-side unicast buffering, TIM set/clear, PS-Poll
   delivery of one buffered frame, STA return from manual poll to enabled PS,
   wake, and post-wake data recovery. Dynamic PS timeout/AP-link semantics are
   still separate future coverage.

### WPA2-Enterprise EAP-PSK smoke validation

Goal: enable one cert-free WPA2-Enterprise path first, so the port exercises
hostapd's integrated EAP server, wpa_supplicant EAP peer code, PF_PACKET
EAPOL delivery, RSN keying, and encrypted AP/STA data traffic.

Build enablement:

- Added `EAP_PSK` and `EAP_SERVER_PSK` to the local wpa/hostapd build flags.
- Added the missing EAP-PSK sources:
  `src/eap_peer/eap_psk.c`, `src/eap_common/eap_psk_common.c`, and
  `src/eap_server/eap_server_psk.c`.
- The first AP build then failed at link because EAP-PSK uses AES-EAX helpers:
  `aes_128_eax_encrypt`, `aes_128_eax_decrypt`, and
  `aes_128_encrypt_block`.
- Fixed the link by adding `src/crypto/aes-eax.c` and
  `src/crypto/aes-encblock.c`.

Runtime configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-eap-psk.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-psk.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-psk.conf
```

Runtime issues fixed:

- AP config initially omitted `ieee8021x=1`. Symptom:
  `IEEE 802.1X not enabled`, followed by `AUTH_GET_MSK: Key is null` and AP
  deauthentication before the 4-way handshake.
- STA config initially used `anonymous_identity="eap_psk_user"` while the AP
  user database only had `eap_psk_user@example.com`. Symptom:
  `ieee802_1x_get_eap_user: Failed to find user`. The final config removes
  the anonymous identity and keeps both the full and short identities in the
  AP user file.

Final AP + STA1 interactive proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 47 (PSK) selected
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
3 packets transmitted, 3 received, 0% packet loss
0.00-   6.02 sec   13975552 Bytes   18.57 Mbits/sec
```

Status: WPA2-Enterprise EAP-PSK AP/STA smoke is `PASS`.

### WPA2-Enterprise EAP-TLS validation

Goal: exercise certificate-based WPA2-Enterprise over the same NuttX hwsim
path, including hostapd's integrated EAP server, internal TLS server/client,
PF_PACKET EAPOL delivery, RSN keying, and encrypted AP/STA data traffic.

Runtime configs and generated certificate material:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-eap-tls.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-tls.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-tls.conf
tools/firmware/sim/eaptls.conf
tools/firmware/sim/eap-tls-certs/{ca,server,client}.pem
tools/firmware/sim/eap-tls-certs/{ca,server,client}.key
tools/firmware/sim/eap-tls-certs/dhparam.pem
```

Runtime issues fixed:

- AP failed immediately after the ClientHello with no TLS alert. Root cause:
  the internal TLS server selected a DHE-RSA cipher suite and needed DH
  parameters. Fix: generate `dhparam.pem` and add
  `dh_file=/h/eap-tls-certs/dhparam.pem`.
- AP and STA then hit `devif_send error: -90` while sending certificate-sized
  EAP-TLS messages. Root cause: default 1398-byte EAP-TLS fragments exceeded
  the current hwsim/EAPOL send limit. Fix: set `fragment_size=512` in both
  hostapd and wpa_supplicant EAP-TLS configs.
- AP then rejected the client certificate as not yet valid because NuttX sim
  time starts before the generated certificate `notBefore`. The imported
  internal TLS server ignored hostapd `tls_flags=[DISABLE-TIME-CHECKS]` for
  server-side client certificate validation. Fix: add internal TLS server
  flag storage, pass flags from `tls_connection_set_verify()`, and honor
  `TLS_CONN_DISABLE_TIME_CHECKS` in `tlsv1_server_read.c`. The sim AP config
  sets `tls_flags=[DISABLE-TIME-CHECKS]`.

Final AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 13 (TLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec     344064 Bytes    0.46 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     671744 Bytes    0.89 Mbits/sec
Post-iperf ping: 3 packets transmitted, 3 received, 0% packet loss
```

Status: WPA2-Enterprise EAP-TLS AP/STA smoke is `PASS`.

### WPA2-Enterprise PEAP/MSCHAPv2 validation

Goal: exercise a tunneled Enterprise method over the same NuttX hwsim path,
including hostapd's integrated EAP server, outer PEAP TLS, inner MSCHAPv2,
EAP-TLV result handling, PF_PACKET EAPOL delivery, RSN keying, and encrypted
AP/STA data traffic.

Runtime configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-eap-peap.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-peap.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-peap.conf
```

Runtime notes:

- The AP config reuses the EAP-TLS CA, server certificate, server key, and
  `dhparam.pem`.
- The AP and STA configs keep `fragment_size=512`, matching the EAP-TLS fix
  for the current hwsim/EAPOL send-size limit.
- The AP keeps `tls_flags=[DISABLE-TIME-CHECKS]`, and the STA uses
  `phase1="tls_disable_time_checks=1 peapver=0"` because the simulator clock
  is earlier than the generated test certificate validity window.
- The user database needs separate phase-1 and phase-2 entries:

  ```text
  "anonymous@example.com" PEAP
  "peap_user@example.com" PEAP
  "peap_user@example.com" MSCHAPV2 "peap_password" [2]
  ```

Final AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 25 (PEAP) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
EAP-MSCHAPV2: Authentication succeeded
EAP-TLV: TLV Result - Success - EAP-TLV/Phase2 Completed
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     458752 Bytes    0.61 Mbits/sec
```

Status: WPA2-Enterprise PEAP/MSCHAPv2 AP/STA smoke is `PASS`.

### WPA2-Enterprise TTLS/PAP validation

Goal: exercise EAP-TTLS over the same NuttX hwsim path, including hostapd's
integrated EAP server, outer TTLS TLS, tunneled PAP user/password validation,
PF_PACKET EAPOL delivery, RSN keying, and encrypted AP/STA data traffic.

Runtime configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-ttls.conf
```

Runtime notes:

- The AP config reuses the EAP-TLS CA, server certificate, server key, and
  `dhparam.pem`.
- The AP and STA configs keep `fragment_size=512`, matching the EAP-TLS fix
  for the current hwsim/EAPOL send-size limit.
- The AP keeps `tls_flags=[DISABLE-TIME-CHECKS]`, and the STA uses
  `phase1="tls_disable_time_checks=1"` because the simulator clock is earlier
  than the generated test certificate validity window.
- The user database needs a phase-2 TTLS non-EAP method entry:

  ```text
  "anonymous@example.com" TTLS
  "ttls_user@example.com" TTLS
  "ttls_user@example.com" TTLS-PAP "ttls_password" [2]
  ```

Final AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 21 (TTLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec    1048576 Bytes    1.39 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     540672 Bytes    0.72 Mbits/sec
```

Status: WPA2-Enterprise TTLS/PAP AP/STA smoke is `PASS`.

### WPA2-Enterprise TTLS/MSCHAPv2 validation

Goal: exercise EAP-TTLS with the MSCHAPv2 tunneled inner method over the same
NuttX hwsim path, including hostapd's integrated EAP server, outer TTLS TLS,
MSCHAPv2 challenge/response, PF_PACKET EAPOL delivery, RSN keying, and
encrypted AP/STA data traffic.

Runtime configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls-mschapv2.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls-mschapv2.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-ttls-mschapv2.conf
tools/firmware/sim/ttlsms.conf
```

Runtime notes:

- The AP config reuses the EAP-TLS CA, server certificate, server key, and
  `dhparam.pem`.
- The AP and STA configs keep `fragment_size=512`, matching the EAP-TLS fix
  for the current hwsim/EAPOL send-size limit.
- The AP keeps `tls_flags=[DISABLE-TIME-CHECKS]`, and the STA uses
  `phase1="tls_disable_time_checks=1"` because the simulator clock is earlier
  than the generated test certificate validity window.
- The user database needs a phase-2 TTLS non-EAP method entry:

  ```text
  "anonymous@example.com" TTLS
  "ttls_mschapv2_user@example.com" TTLS
  "ttls_mschapv2_user@example.com" TTLS-MSCHAPV2 "ttls_mschapv2_password" [2]
  ```
- The full STA config path made the NSH command long enough that one launch
  lost the trailing background `&`; the final proof used the shorter alias
  `ttlsms.conf`.

Final AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 21 (TTLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
EAP-TTLS: Phase 2 MSCHAPV2 authentication succeeded
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec    1048576 Bytes    1.39 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec    1327104 Bytes    1.76 Mbits/sec
```

Status: WPA2-Enterprise TTLS/MSCHAPv2 AP/STA smoke is `PASS`.

### WPA2-Enterprise TTLS/CHAP and TTLS/MSCHAP validation

Goal: complete the common EAP-TTLS non-EAP inner method matrix over the same
NuttX hwsim path by adding CHAP and MSCHAP after the already validated PAP and
MSCHAPv2 paths.

Runtime configs:

```text
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls-chap.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls-chap.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-ttls-chap.conf
tools/firmware/sim/ttlschap.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls-mschap.conf
tools/firmware/sim/hostapd-hwsim-wpa2-eap-ttls-mschap.users
tools/firmware/sim/wpa_supplicant-hwsim-wpa2-eap-ttls-mschap.conf
tools/firmware/sim/ttlsmschap.conf
```

Runtime notes:

- The AP configs reuse the EAP-TLS CA, server certificate, server key, and
  `dhparam.pem`.
- The AP and STA configs keep `fragment_size=512`, matching the EAP-TLS fix
  for the current hwsim/EAPOL send-size limit.
- The AP keeps `tls_flags=[DISABLE-TIME-CHECKS]`, and the STAs use
  `phase1="tls_disable_time_checks=1"` because the simulator clock is earlier
  than the generated test certificate validity window.
- The user databases constrain the phase-2 method:

  ```text
  "ttls_chap_user@example.com" TTLS-CHAP "ttls_chap_password" [2]
  "ttls_mschap_user@example.com" TTLS-MSCHAP "ttls_mschap_password" [2]
  ```

TTLS/CHAP AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 21 (TTLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec    1081344 Bytes    1.44 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     327680 Bytes    0.44 Mbits/sec
```

TTLS/MSCHAP AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 21 (TTLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec    1163264 Bytes    1.55 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     753664 Bytes    1.00 Mbits/sec
```

Status: WPA2-Enterprise TTLS/CHAP and TTLS/MSCHAP AP/STA smokes are `PASS`.
External RADIUS remains unproven.

### Suite-B/192-bit Enterprise runtime validation

Goal: start the highest WPA-Enterprise security profile in the simulator path
and identify whether the remaining work is userspace configuration, internal
TLS, RSN keying, or hwsim/control-port transport.

Build enablement:

- Added `CONFIG_SUITEB` and `CONFIG_SUITEB192` to the local
  hostapd/wpa_supplicant build flags.
- Added `CONFIG_SHA384` and `CONFIG_SHA512` to the same build flags. The
  SHA384/SHA512 source files were already compiled, but `wpa_pmk_to_ptk()`
  still returns failure for Suite-B AKMs unless the feature macros are also
  defined.
- AP and STA sim builds were rerun serially. Running `build-ap.sh` and
  `build-sta1.sh` at the same time corrupted the shared NuttX configure/Kconfig
  state and produced misleading Kconfig parse/config errors. The solution is to
  build AP/STA simulator variants one at a time.

Runtime configs and certificate material:

```text
tools/firmware/sim/hostapd-hwsim-suiteb192-eap-tls.conf
tools/firmware/sim/hostapd-hwsim-suiteb192-eap-tls.users
tools/firmware/sim/wpa_supplicant-hwsim-suiteb192-eap-tls.conf
tools/firmware/sim/suiteb-rsa3072-certs/{ca,server,client}.pem
tools/firmware/sim/suiteb-rsa3072-certs/{ca,server,client}.key
tools/firmware/sim/suiteb-rsa3072-certs/dhparam.pem
```

Runtime issues fixed:

- STA initially rejected the AP RSN IE because the AP required PMF and the STA
  network block did not set PMF. Fix: add `ieee80211w=2` to the Suite-B STA
  network block.
- The internal TLS server failed while building ServerKeyExchange with
  RSA3072:
  `PKCS #1: pkcs1_generate_encryption_block - Invalid buffer lengths
  (modlen=384 outlen=378 inlen=36)`. Root cause: the fixed non-certificate
  part of the server handshake allocation left less than the 384-byte RSA3072
  signature size once DHE parameters and CertificateRequest were included.
  Fix: increase the TLS server full-handshake buffer allowance from 1000 to
  2048 bytes in both imported internal TLS server source copies.
- After EAP success, the STA failed the first RSN 4-way attempt with
  `WPA: PTK derivation failed`, while the AP kept retrying message 1/4 and
  eventually disconnected. Root cause: the Suite-B path in
  `wpa_pmk_to_ptk()` requires `CONFIG_SHA384` for `wpa_key_mgmt_sha384()`;
  compiling the SHA384 sources alone was not sufficient. Fix: define
  `CONFIG_SHA384` and `CONFIG_SHA512` in
  `apps/wireless/wifi/wpa_hostapd_sources.mk`.

Current AP + STA1 proof:

```text
AP:
wlan0: AP-ENABLED
RSN: Suite selector 00:0f:ac:9
RSN: Suite selector 00:0f:ac:12
WPA: PMK from EAPOL state machine (MSK len=64 PMK len=48)
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
accept: 192.168.201.2:15301
0.00-   6.03 sec     237592 Bytes    0.32 Mbits/sec

STA:
wlan0: WPA: using KEY_MGMT 802.1X with Suite B (192-bit)
wlan0: WPA: using PTK GCMP-256
wlan0: WPA: using GTK GCMP-256
wlan0: WPA: using MGMT group cipher BIP-GMAC-256
wlan0: Associated with 02:00:00:00:00:01
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=GCMP-256 GTK=GCMP-256]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 3 192.168.201.1: 3 packets transmitted, 3 received, 0% packet loss
iperf -c 192.168.201.1 -t 5: 0.00-   6.02 sec     425984 Bytes    0.57 Mbits/sec
```

Current status: Suite-B/192-bit Enterprise is runtime `PASS` for the current
simulator profile. RSN capability, PMF-required association, RSA3072 EAP-TLS
fragmentation, EAP success, RSN 4-way handshake, `CTRL-EVENT-CONNECTED`, ping,
and STA-to-AP iperf are proven.

Important limitation: this is not a strict CNSA/Suite-B TLS profile proof. The
current imported internal TLS stack negotiates TLS 1.0 DHE-RSA/AES-CBC for this
test, with RSA3072 certificates and SHA384 signatures. A full WPA3-Enterprise
192-bit validation still needs an ECDHE/ECDSA SHA384/GCM-capable TLS path or a
documented equivalent policy decision.

Status:

P2P control-interface smoke, single-STA P2P commands, dual-STA peer discovery,
GO/client group formation, and sequential bidirectional GO/client data traffic
are now `PASS`. WPS PBC AP/STA provisioning and WPA2 data traffic are also
`PASS`. WNM BSS Transition Management request/response is now `PASS` for the
basic AP + STA1 no-candidate case. WNM Sleep Mode enter/exit is now `PASS` for
AP + STA1, including GTK group rekey, post-sleep ping/iperf, and non-empty TFS
Request/Response IE plumbing through the local nl80211 `.wnm_oper` path.
A-MPDU/ADDBA is now `PASS`: HT20 + WMM runtime shows BA session start,
ADDBA request/response, hwsim `ampdu_action` actions 0/2/6, `tx operational`,
ping 3/3, and bidirectional iperf. A-MSDU is now also `PASS` for the AP-side
software aggregation path after the queue-backlog proof produced
`amsdu-proof: aggregate` up to 8 subframes and preserved ping/iperf. TWT has
moved past the control command, AP responder, and STA receive/parse gaps: HE
AP/STA association passes with TWT responder config, `wpa_cli twt_setup` and
`twt_teardown` return `OK`, AP-side hostapd handles S1G setup/teardown, AP emits
a TWT Accept action frame, and STA-side supplicant receives and parses the
Accept fields (`setup_cmd=4`, `req_type=0x2878`, `dialog=1`). Manual TWT
service-period PS behavior and automatic service-period toggling are proven;
automatic TWT now also passes AP-to-STA and STA-to-AP ping smoke plus sequential
STA-to-AP and AP-to-STA iperf under sim-stretched 1 s / 500 ms service-period
timing, with sender-side teardown stop. Tighter timing, AP-originated ARP,
sleep-buffer release, hostfs medium truncation, and throughput tuning remain
pending. Static
PS control now reaches
standard nl80211 power-save set commands and preserves STA-to-AP ping. The
hwsim-private testmode PS latch,
queue stop/wake, PS nullfunc/PS-Poll frame emission, and AP-side buffered-frame
delivery paths are now proven:
`set_ps 1` emits PM=1 nullfunc (`48 11`), `poll` emits PS-Poll (`a4 10`), and
`set_ps 0` emits PM=0 nullfunc (`48 01`) in `hwsim-frames.bin`; queue stop makes
STA-to-AP traffic pause and wake restores it under ping and iperf; sleeping-STA
AP unicast buffering sets TIM, PS-Poll releases one buffered frame, TIM clears,
and wake restores normal ping. Dynamic PS is now covered by dedicated
MLO-enabled `hwsim_dynps_ap` / `hwsim_dynps_sta1` role images: idle-timeout
sleep/wake, auto-PS polling, AP-link-PS buffering/PS-Poll release, and
bidirectional ping/iperf data smoke are proven, while AP-to-STA auto-PS TCP
throughput still needs tuning. DCM is now covered by a simulator validation path:
hwsim HE capabilities advertise DCM under `CONFIG_WL_NUTTX_HWSIM_DCM_PROOF`,
`/h/hwsim-dcm-proof` gates runtime RX status injection, and AP+STA1 11ax data
traffic proves `he_dcm=1` with bidirectional iperf still healthy.
WPA2-Enterprise
EAP-PSK, EAP-TLS, PEAP/MSCHAPv2, and TTLS inner PAP/CHAP/MSCHAP/MSCHAPv2 are
now proven through EAP success, RSN 4-way handshake, ping, and iperf; Suite-B/
192-bit Enterprise is now proven through EAP success, GCMP-256 RSN 4-way
handshake, ping, and STA-to-AP iperf for the current simulator TLS profile;
external RADIUS remains unproven.
WPA3 SAE-PK is now enabled and proven through AP private-key
parse, STA `K_AP` fingerprint validation, KeyAuth signature verification,
SAE commit/confirm, RSN 4-way handshake, IGTK install,
`CTRL-EVENT-CONNECTED`, ping, and bidirectional iperf.

SAE-PK data-plane rerun notes:

```text
AP:
hostapd /h/hostapd-hwsim-wpa3-sae-pk.conf &
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

STA:
wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-wpa3-sae-pk.conf &
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 3 192.168.201.1
3 packets transmitted, 3 received, 0% packet loss

STA -> AP iperf:
0.00-   6.02 sec   15253504 Bytes   20.27 Mbits/sec

AP -> STA iperf:
0.00-   6.03 sec   15286272 Bytes   20.28 Mbits/sec
```

Problems and fixes found during the SAE-PK data-plane rerun:

- Problem: running `hostapd -B` or `wpa_supplicant -B` in the simulator can
  leave NSH without a usable prompt, so later `ifconfig`, `ping`, and `iperf`
  commands may not execute even after the RSN handshake succeeds.
- Fix: use NSH background jobs for the daemons during interactive validation:
  `hostapd /h/hostapd-hwsim-wpa3-sae-pk.conf &` and
  `wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-wpa3-sae-pk.conf &`.
- Problem: temporary `hwsim-debug` and `nl80211-debug` printf instrumentation
  made earlier interactive logs noisy enough to obscure prompt state.
- Fix: gate those temporary prints behind `CONFIG_WL_NUTTX_HWSIM_DEBUG` in the
  NuttX IEEE80211/hwsim/netlink path and in the local libnl/nl80211 userspace
  shims.

## A-MSDU Aggregation Proof

The A-MSDU item is now runtime `PASS` for the AP-side software aggregation path
over HT20/WMM.

Validation changes:

```text
CONFIG_WL_NUTTX_HWSIM_AMSDU_PROOF=y
mac80211_hwsim_linux.c:
  ieee80211_hw_set(hw, SUPPORTS_AMSDU_IN_AMPDU)
  ieee80211_hw_set(hw, TX_AMSDU)
tx.c:
  amsdu-proof: aggregate ...
  amsdu-proof: probe skip ...
```

First runtime proof with AP + STA1 HT20/WMM showed that negotiation was working
but the default hwsim wake path drained the queue too quickly:

```text
/tmp/hwsim-amsdu-probe-ap.log:
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
ampdu-proof: send ADDBA_RESP ... amsdu=1
ampdu-proof: rx ADDBA_RESP ... amsdu=1
ampdu-proof: tx operational ... amsdu=1
amsdu-proof: probe skip no-head tid=0 len=54 max=1600 tin_packets=0 tin_bytes=0 head=0

/tmp/hwsim-amsdu-probe-sta1.log:
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ampdu-proof: send ADDBA_RESP ... amsdu=1
ampdu-proof: rx ADDBA_RESP ... amsdu=1
ampdu-proof: tx operational ... amsdu=1
ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
iperf -c 192.168.201.1 -p 5003 -t 6 -> 0.00-   6.02 sec   13467648 Bytes   17.90 Mbits/sec
```

Final A-MSDU construction proof uses the existing hwsim private testmode command
to create a short AP-side TXQ backlog window:

```text
AP:
iperf -s -p 5004 &
hwsim_tm -i wlan0 stop_queues

STA:
iperf -c 192.168.201.1 -p 5004 -t 8 &

AP:
sleep 2
hwsim_tm -i wlan0 wake_queues
```

Latest proof:

```text
/tmp/hwsim-amsdu-backlog-ap.log:
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
ampdu-proof: send ADDBA_RESP ... amsdu=1
ampdu-proof: rx ADDBA_RESP ... amsdu=1
ampdu-proof: tx operational ... amsdu=1
hwsim_tm -i wlan0 stop_queues -> OK
amsdu-proof: aggregate tid=0 ... subframes=2 orig_head_len=78 added_len=68 total_len=160 max_len=1600 data_len=68
amsdu-proof: aggregate tid=0 ... subframes=8 orig_head_len=500 added_len=68 total_len=568 max_len=1600 data_len=476
hwsim_tm -i wlan0 wake_queues -> OK

/tmp/hwsim-amsdu-backlog-sta1.log:
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
iperf -c 192.168.201.1 -p 5004 -t 8 -> 0.00-   9.03 sec   19922944 Bytes   17.65 Mbits/sec
```

Problems and fixes found:

- Problem: imported Linux `mac80211_hwsim` does not enable software A-MSDU
  TX by default; it advertises HE A-MSDU-in-A-MPDU capability bits but not the
  hw flags needed for `ieee80211_amsdu_aggregate()`.
- Fix: add the validation-only `CONFIG_WL_NUTTX_HWSIM_AMSDU_PROOF` option and
  enable `SUPPORTS_AMSDU_IN_AMPDU` plus `TX_AMSDU` only under that option.
- Problem: the NuttX skb compatibility layer had a static `skb_shinfo()`,
  `skb_has_frag_list()` always returned false, and `skb_linearize()` was a
  no-op. That was not safe for software A-MSDU, which chains subframes through
  `frag_list`.
- Fix: make `sk_buff` own persistent `skb_shared_info`, copy from `frag_list`
  in `skb_copy_bits()`, free `frag_list` recursively, and linearize non-linear
  skbs before hwsim publish when the driver does not advertise `TX_FRAG_LIST`.
- Problem: even with A-MSDU capability and skb support in place, the first
  AP-side runtime probe reached `ieee80211_amsdu_aggregate()` but always found
  no previous frame in the flow queue: `skip no-head ... tin_packets=0`. The
  default hwsim wake/dequeue path drains each TXQ flow before a second compatible
  MSDU is available.
- Fix: use the existing hwsim private testmode queue control as the proof
  trigger. Pausing AP TX queues while STA sends TCP traffic creates the required
  TXQ backlog; waking queues then drains the A-MSDU frame successfully. This
  produced `amsdu-proof: aggregate` events up to 8 subframes and preserved
  `ping`/`iperf`.

## 20. HS20/interworking ANQP/GAS runtime proof

HS20/interworking was moved from compiled-only inventory to runtime proof on
the NuttX hwsim path.

Added simulator configs:

```text
tools/firmware/sim/hostapd-hwsim-hs20.conf
tools/firmware/sim/hostapd-hwsim-hs20.users
tools/firmware/sim/wpa_supplicant-hwsim-hs20.conf
```

The first AP attempt used open authentication with `hs20=1` and failed during
hostapd configuration parsing:

```text
HS 2.0: WPA2-Enterprise/CCMP configuration is required for Hotspot 2.0 functionality
```

Fix: layer HS20/interworking on the already validated WPA2-Enterprise EAP-PSK
path with `wpa=2`, `wpa_key_mgmt=WPA-EAP`, `rsn_pairwise=CCMP`,
`ieee8021x=1`, hostapd's internal EAP server, and an EAP-PSK user file.

Runtime proof commands:

```text
AP:
hostapd /h/hostapd-hwsim-hs20.conf &
ifconfig wlan0 192.168.201.1 netmask 255.255.255.0
iperf -s &

STA:
wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-hs20.conf &
ifconfig wlan0 192.168.201.2 netmask 255.255.255.0
ping -c 3 192.168.201.1
wpa_cli -i wlan0 anqp_get 02:00:00:00:00:01 257,258,262,263,268
wpa_cli -i wlan0 hs20_anqp_get 02:00:00:00:00:01 2,3,4,5
wpa_cli -i wlan0 bss 02:00:00:00:00:01
iperf -c 192.168.201.1 -t 5
```

Latest proof:

```text
/tmp/hwsim-hs20-ap.log:
wlan0: AP-ENABLED
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
iperf server accepted 192.168.201.2 and completed the TCP run

/tmp/hwsim-hs20-sta1.log:
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
GAS-QUERY-DONE ... dialog_token=255 ... result=SUCCESS
RX-ANQP ... ANQP Capability list
RX-ANQP ... Venue Name
RX-ANQP ... IP Address Type Availability information
RX-ANQP ... NAI Realm list
RX-ANQP ... Domain Name list
GAS-QUERY-DONE ... dialog_token=16 ... result=SUCCESS
RX-HS20-ANQP ... HS Capability List
RX-HS20-ANQP ... Operator Friendly Name
RX-HS20-ANQP ... WAN Metrics 01:8000:1000:10:20:300
RX-HS20-ANQP ... Connection Capability
flags=[WPA2-EAP-CCMP][ESS][HS20]
anqp_venue_name=...
anqp_nai_realm=...
anqp_domain_name=...
hs20_operator_friendly_name=...
hs20_wan_metrics=...
hs20_connection_capability=...
iperf -c 192.168.201.1 -t 5 -> 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec
```

Status: HS20/interworking ANQP/GAS is `PASS` for the AP + STA1 simulator path.
OSU/provider provisioning, icon fetch, and subscription remediation remain
separate follow-up coverage.

## 21. 802.11r FT-PSK initial association runtime proof

802.11r was moved from compiled-only inventory to an initial FT AKM runtime
proof on the NuttX hwsim path. This validates FT-PSK association and key
derivation on a single BSS. It does not yet prove FT roaming or reassociation
between multiple BSSes.

Added simulator configs:

```text
tools/firmware/sim/hostapd-hwsim-ft-psk.conf
tools/firmware/sim/wpa_supplicant-hwsim-ft-psk.conf
```

Runtime proof commands:

```text
AP:
hostapd /h/hostapd-hwsim-ft-psk.conf &
ifconfig wlan0 192.168.201.1 netmask 255.255.255.0
iperf -s &

STA:
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-ft-psk.conf &
ifconfig wlan0 192.168.201.2 netmask 255.255.255.0
wpa_cli -i wlan0 status
wpa_cli -i wlan0 bss 02:00:00:00:00:01
ping -c 3 192.168.201.1
iperf -c 192.168.201.1 -t 5
```

Latest proof:

```text
/tmp/hwsim-ft-psk-ap.log:
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
accept: 192.168.201.2:8208

/tmp/hwsim-ft-psk-sta1.log:
wlan0: WPA: using KEY_MGMT FT/PSK
FT: MDE - hexdump(len=3): a1 b2 01
FT: Mobility domain - hexdump(len=2): a1 b2
FT: Derive PMK-R0 using KDF-SHA256
FT: Derive PMK-R1 using KDF-SHA256
WPA: MDIE in EAPOL-Key - hexdump(len=5): 36 03 a1 b2 01
WPA: FTIE in EAPOL-Key - hexdump(len=112): ...
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
key_mgmt=FT-PSK
wpa_state=COMPLETED
flags=[WPA2-FT/PSK-CCMP][ESS]
ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
iperf -c 192.168.201.1 -t 5 -> 0.00-   6.02 sec    1572864 Bytes    2.09 Mbits/sec
```

Problem/fix chain:

- Problem: previous matrix state only proved that `CONFIG_IEEE80211R` and FT
  sources were compiled. It did not prove the FT AKM path at runtime.
- Fix: add dedicated FT-PSK AP/STA configs with `wpa_key_mgmt=FT-PSK`,
  `mobility_domain=a1b2`, `ft_over_ds=1`, and
  `ft_psk_generate_local=1`, then run hostapd/wpa_supplicant over the NuttX
  hwsim medium and capture FT-specific key derivation and data-plane proof.
- Remaining gap: full 802.11r still needs a multi-BSS roaming test that proves
  reassociation or FT-over-DS between two AP/BSS instances.

## 22. Multi-role simulator objects and FT multi-BSS roam status

Simulator role changes are now modeled as separate build/run objects instead
of reusing one binary for different logical roles. This keeps multi-terminal
tests explicit: every AP, STA, and P2P participant can have its own
`nuttx-sim-*` process and its own hwsim radio base.

Added role build scripts:

```text
tools/firmware/sim/build-hwsim-role.sh
tools/firmware/sim/build-sta3.sh
tools/firmware/sim/build-ap1.sh
tools/firmware/sim/build-ap2.sh
tools/firmware/sim/build-p2p1.sh
tools/firmware/sim/build-p2p2.sh
```

Added sim configs:

```text
nuttx/boards/sim/sim/sim/configs/hwsim_sta3/defconfig
nuttx/boards/sim/sim/sim/configs/hwsim_ap1/defconfig
nuttx/boards/sim/sim/sim/configs/hwsim_ap2/defconfig
nuttx/boards/sim/sim/sim/configs/hwsim_p2p1/defconfig
nuttx/boards/sim/sim/sim/configs/hwsim_p2p2/defconfig
```

Role assignments:

```text
ap        radio_base=1   IP 192.168.201.1
sta1      radio_base=2   IP 192.168.201.2
sta2      radio_base=3   IP 192.168.201.3
sta3      radio_base=4   IP 192.168.201.4
ap1       radio_base=10  IP 192.168.201.11
ap2       radio_base=11  IP 192.168.201.12
p2p1      radio_base=20  IP 192.168.202.1
p2p2      radio_base=21  IP 192.168.202.2
```

Problem/fix chain:

- Problem: future multi-AP, multi-STA, and P2P tests need more independent
  simulator processes than the original AP/STA1/STA2 trio.
- Fix: add a shared `build-hwsim-role.sh` helper and thin role wrappers so new
  roles build to separate artifacts such as `build/nuttx-sim-ap1` and
  `build/nuttx-sim-p2p2`.
- Problem: AP/STA sim images must still be built serially because the NuttX
  tree has a single active `.config` and generated dependency state.
- Fix: run the new role builds one after another. The 2026-06-06 build pass
  produced `nuttx-sim-sta3`, `nuttx-sim-ap1`, `nuttx-sim-ap2`,
  `nuttx-sim-p2p1`, and `nuttx-sim-p2p2`; all five accepted a `poweroff`
  startup smoke command with `rc=0`.

Current role-object build refresh:

```text
/tmp/hwsim-build-role-ap1-current.log:
  FeatherCore/tools/firmware/sim/build-ap1.sh -j8 -> ap1_rc=0
  build/nuttx-sim-ap1 size: 23120544 bytes

/tmp/hwsim-build-role-ap2-current.log:
  FeatherCore/tools/firmware/sim/build-ap2.sh -j8 -> ap2_rc=0
  build/nuttx-sim-ap2 size: 23120544 bytes

/tmp/hwsim-build-role-sta3-current.log:
  FeatherCore/tools/firmware/sim/build-sta3.sh -j8 -> sta3_rc=0
  build/nuttx-sim-sta3 size: 27410072 bytes
```

Current startup smoke:

```text
/tmp/hwsim-boot-role-ap1-current.log  -> poweroff reached NSH, ap1_rc=0
/tmp/hwsim-boot-role-ap2-current.log  -> poweroff reached NSH, ap2_rc=0
/tmp/hwsim-boot-role-sta3-current.log -> poweroff reached NSH, sta3_rc=0
```

2026-06-06 role-object refresh:

```text
/tmp/hwsim-build-role-ap1-refresh.log:
  FeatherCore/tools/firmware/sim/build-ap1.sh -j8 -> ap1_rc=0
  build/nuttx-sim-ap1 size: 23353584 bytes
  build/nuttx-sim-ap1.map generated

/tmp/hwsim-build-role-ap2-refresh.log:
  FeatherCore/tools/firmware/sim/build-ap2.sh -j8 -> ap2_rc=0
  build/nuttx-sim-ap2 size: 23353584 bytes
  build/nuttx-sim-ap2.map generated

/tmp/hwsim-build-role-sta3-refresh.log:
  FeatherCore/tools/firmware/sim/build-sta3.sh -j8 -> sta3_rc=0
  build/nuttx-sim-sta3 size: 27714792 bytes
  build/nuttx-sim-sta3.map generated

/tmp/hwsim-build-role-p2p1-refresh.log:
  FeatherCore/tools/firmware/sim/build-p2p1.sh -j8 -> p2p1_rc=0
  build/nuttx-sim-p2p1 size: 27714792 bytes
  build/nuttx-sim-p2p1.map generated

/tmp/hwsim-build-role-p2p2-refresh.log:
  FeatherCore/tools/firmware/sim/build-p2p2.sh -j8 -> p2p2_rc=0
  build/nuttx-sim-p2p2 size: 27714792 bytes
  build/nuttx-sim-p2p2.map generated

/tmp/hwsim-boot-role-ap1-refresh.log -> reached NuttShell, poweroff, ap1_boot_rc=0
/tmp/hwsim-boot-role-ap2-refresh.log -> reached NuttShell, poweroff, ap2_boot_rc=0
/tmp/hwsim-boot-role-sta3-refresh.log -> reached NuttShell, poweroff, sta3_boot_rc=0
/tmp/hwsim-boot-role-p2p1-refresh.log -> reached NuttShell, poweroff, p2p1_boot_rc=0
/tmp/hwsim-boot-role-p2p2-refresh.log -> reached NuttShell, poweroff, p2p2_boot_rc=0

/tmp/hwsim-build-dynps-ap-refresh.log:
  FeatherCore/tools/firmware/sim/build-dynps-ap.sh -j8 -> dynps_ap_rc=0
  build/nuttx-sim-dynps-ap size: 23353584 bytes
  build/nuttx-sim-dynps-ap.map generated

/tmp/hwsim-build-dynps-sta1-refresh.log:
  FeatherCore/tools/firmware/sim/build-dynps-sta1.sh -j8 -> dynps_sta1_rc=0
  build/nuttx-sim-dynps-sta1 size: 27714792 bytes
  build/nuttx-sim-dynps-sta1.map generated

/tmp/hwsim-boot-dynps-ap-refresh.log   -> reached NuttShell, poweroff, dynps_ap_boot_rc=0
/tmp/hwsim-boot-dynps-sta1-refresh.log -> reached NuttShell, poweroff, dynps_sta1_boot_rc=0
```

Repeatable manual recipe:

- Added `docs/WIFI_HWSIM_MANUAL_RECIPES.md` so multi-terminal validation keeps
  the individual NuttX commands visible instead of hiding them in another
  script.
- The recipe covers sequential role builds, stale hostfs hwsim medium cleanup,
  AP + STA1 + STA2 boot commands, STA-to-STA ping/iperf in both directions, and
  the dedicated P2P1/P2P2 GO/client flow with explicit `wpa_cli` commands.
- The recipe now includes a PHY / 802.11 version config map for 802.11b,
  802.11g, 802.11a, 802.11n HT20/HT40, 802.11ac VHT20/VHT80/VHT160/VHT80+80,
  802.11ax HE 2.4/5/6 GHz, and S1G / 802.11ah. This makes the existing PASS
  coverage directly reproducible from the same AP/STA terminal skeleton by
  swapping only the hostapd and wpa_supplicant config names.
- The recipe also includes a security/encryption config map for open auth,
  WPA2-PSK CCMP/TKIP/PMF, WPA3 SAE/H2E/SAE-PK, WPA2/WPA3 transition mode,
  OWE groups 19/20/21, WPA2-Enterprise EAP-PSK/EAP-TLS/PEAP/TTLS variants,
  Suite-B 192-bit, WPS, 802.11r FT, WNM, HS20, and the special DPP
  provisioning flow. This turns the existing security PASS matrix into a
  repeatable AP/STA command skeleton with explicit config names.
- The recipe now also covers the feature/testmode proofs for DCM, A-MPDU,
  A-MSDU, TWT, static PS, and dynamic PS/MLO. It documents the exact
  `hwsim_tm` commands currently supported (`get_ps`, `set_ps`, `poll`,
  `stop_queues`, `wake_queues`, `get_twt_sp`, `set_twt_sp`), the DCM marker
  file, the HT/WMM aggregation configs, TWT service-period gating, static
  PS-Poll/buffered-frame checks, and the dedicated `build-dynps-ap.sh` /
  `build-dynps-sta1.sh` objects.
- Problem: hand-driven validation was spread across logs, making later
  encryption/PHY/P2P/PS retests easy to run with slightly different terminal
  setup. Fix: document the exact command skeleton and call out common issues:
  serial builds, deleting `hwsim-frames.bin` / `hwsim-bss.bin`, ARP warm-up,
  P2P peer address discovery, no-bridge multi-BSS IP setup, 6 GHz secure-AKM
  requirements, wide-channel throughput caveats, S1G simulator-scope caveats,
  DPP long-argument `@file` handling, Suite-B TLS-profile limits, and the fact
  that DPP CSR/PKCS#7 certificate-mode still needs real crypto backend support.
  The feature recipe also preserves current caveats: DCM is simulator
  RX-status/capability proof rather than real PHY modulation, automatic TWT
  still has hostfs timing/throughput caveats, and dynamic PS AP-to-STA TCP
  throughput remains a tuning gap.

Independent p2p1/p2p2 P2P GO/client runtime proof:

- Problem: the first independent-object attempt used shorthand commands
  (`start`, `status`, `p2p_find`, `p2p_connect`) that were not installed NSH
  commands in the sim image. The logs showed `command not found`,
  `p2p-wlan0-0` was never created, and later ping/iperf failed with
  unreachable/bind errors.
- Fix: use explicit commands in each simulator: mount hostfs, run
  `wifi_sta_demo` to bring up and sync `wlan0`, start `wpa_supplicant`, and run
  P2P operations through `wpa_cli -i wlan0`.
- Problem: with p2p1 only in `p2p_listen`, it created a peer entry from
  p2p2's Probe Request but did not have a complete peer record with config
  methods. `p2p_peers` listed `42:00:00:00:00:15`, but `p2p_connect` failed
  with `Cannot connect to unknown P2P Device 42:00:00:00:00:15`.
- Fix: run `p2p_find` on both sides long enough for full Probe Response /
  Device Info parsing before GO negotiation.
- Working run evidence:

  ```text
  Logs:
  /tmp/hwsim-p2p1-go5.log
  /tmp/hwsim-p2p2-client5.log

  p2p1:
  P2P-GO-NEG-SUCCESS role=GO freq=2437
  P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-QH" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:15 p2p_dev_addr=42:00:00:00:00:15
  p2p-wlan0-0: EAPOL-4WAY-HS-COMPLETED 52:00:00:00:00:15
  p2p-wlan0-0: inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0

  p2p2:
  P2P-GO-NEG-SUCCESS role=client freq=2437
  p2p-wlan0-0: WPA: Key negotiation completed with 52:00:00:00:00:14 [PTK=CCMP GTK=CCMP]
  P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-QH" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0

  p2p1 GO -> p2p2 client:
  ping -c 5 192.168.77.2 -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 6
  -> 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec

  p2p2 client -> p2p1 GO:
  ping -c 5 192.168.77.1 -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 6
  -> 0.00-   6.02 sec     720896 Bytes    0.96 Mbits/sec
  ```

- Remaining caveat: the final p2p2 iperf client printed the summary but did
  not return to the prompt before cleanup. This matches the existing simulator
  iperf close-behavior issue. The printed transfer summary and bidirectional
  ping are valid data-plane evidence, but repeated long-run P2P stress and
  iperf close tuning remain follow-up work.

P2P 15-second independent-object stress rerun:

The follow-up run reused the dedicated `nuttx-sim-p2p1` and `nuttx-sim-p2p2`
objects and kept the validation manual: one simulator per terminal, hostfs
mounted in each, `wpa_supplicant` started on `wlan0`, P2P discovery on both
sides, and explicit `wpa_cli -i wlan0 p2p_connect` commands using the current
peer addresses.

```text
Logs:
  /tmp/hwsim-p2p-long-p2p1.log
  /tmp/hwsim-p2p-long-p2p2.log

p2p1 / GO:
  P2P-GO-NEG-SUCCESS role=GO freq=2437
  P2P-GROUP-FORMATION-SUCCESS
  P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-5t" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:15 p2p_dev_addr=42:00:00:00:00:15
  p2p-wlan0-0: EAPOL-4WAY-HS-COMPLETED 52:00:00:00:00:15

p2p2 / client:
  P2P-GO-NEG-SUCCESS role=client freq=2437
  P2P-GROUP-FORMATION-SUCCESS
  P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-5t" freq=2437 go_dev_addr=42:00:00:00:00:14

GO -> client:
  ping -c 5 192.168.77.2 -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 15
    -> 0.00-  15.05 sec    2211840 Bytes    1.18 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.2
    -> 3 packets transmitted, 3 received, 0% packet loss

client -> GO:
  ping -c 5 192.168.77.1 -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 15
    -> 0.00-  15.05 sec    2015232 Bytes    1.07 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.1
    -> 3 packets transmitted, 3 received, 0% packet loss
```

This rerun did not print the bounded hostfs medium truncate warning, and both
iperf clients returned to the prompt. The older close-behavior caveat is now
reduced to a historical issue for the shorter proof; remaining P2P work is
longer soak testing and throughput tuning.

P2P 30-second independent-object soak:

```text
Logs:
  /tmp/hwsim-p2p-soak30-p2p1.log
  /tmp/hwsim-p2p-soak30-p2p2.log

p2p1 / GO:
  P2P-GO-NEG-SUCCESS role=GO freq=2437
  P2P-GROUP-FORMATION-SUCCESS
  P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-50" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:15 p2p_dev_addr=42:00:00:00:00:15
  p2p-wlan0-0: EAPOL-4WAY-HS-COMPLETED 52:00:00:00:00:15
  p2p-wlan0-0: inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0

p2p2 / client:
  P2P-GO-NEG-SUCCESS role=client freq=2437
  P2P-GROUP-FORMATION-SUCCESS
  P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-50" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0

Baseline ping:
  GO -> client: 3 packets transmitted, 3 received, 0% packet loss
  client -> GO: 3 packets transmitted, 3 received, 0% packet loss

GO -> client:
  iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 30
    -> 0.00-  30.10 sec    4489216 Bytes    1.19 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.2
    -> 3 packets transmitted, 3 received, 0% packet loss

client -> GO:
  iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 30
    -> 0.00-  30.10 sec    2818048 Bytes    0.75 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.1
    -> 3 packets transmitted, 3 received, 0% packet loss
```

The GO side again showed a first ARP warm-up miss before the successful baseline
ping summary. Both 30-second iperf clients returned normally. The same P2P run
also logged ADDBA request/response and `tx operational` on both peers for TID 0,
so P2P group interfaces now have explicit A-MPDU control-path evidence too.

P2P 60-second independent-object soak:

```text
Logs:
  /tmp/hwsim-p2p-soak60-p2p1.log
  /tmp/hwsim-p2p-soak60-p2p2.log

p2p1 / GO:
  P2P-GO-NEG-SUCCESS role=GO freq=2437
  P2P-GROUP-FORMATION-SUCCESS
  P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-Nj" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:15 p2p_dev_addr=42:00:00:00:00:15
  p2p-wlan0-0: EAPOL-4WAY-HS-COMPLETED 52:00:00:00:00:15
  p2p-wlan0-0: inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0

p2p2 / client:
  P2P-GO-NEG-SUCCESS role=client freq=2437
  P2P-GROUP-FORMATION-SUCCESS
  p2p-wlan0-0: WPA: Key negotiation completed with 52:00:00:00:00:14 [PTK=CCMP GTK=CCMP]
  p2p-wlan0-0: CTRL-EVENT-CONNECTED - Connection to 52:00:00:00:00:14 completed
  P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-Nj" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0

GO -> client:
  ping -c 5 192.168.77.2
    -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 60
    -> 0.00-  60.20 sec   10092544 Bytes    1.34 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.2
    -> 3 packets transmitted, 3 received, 0% packet loss

client -> GO:
  ping -c 5 192.168.77.1
    -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 60
    -> 0.00-  60.20 sec    9945088 Bytes    1.32 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.1
    -> 3 packets transmitted, 3 received, 0% packet loss
```

The client-side server for the GO->client run also printed the full 60-second
receive summary and `closed by the peer`; the GO-side server did the same for
the client->GO run. Both peers logged ADDBA request/response and `tx
operational`, so the longer P2P soak keeps both the data path and aggregation
control path healthy. The historical WPS/EAP failure line still appears during
P2P group formation, but it is followed by successful CCMP key negotiation and
connected/group-started events.

2026-06-06 refreshed p2p1/p2p2 short runtime smoke:

After rebuilding `nuttx-sim-p2p1` and `nuttx-sim-p2p2`, the refreshed images
were started from `tools/firmware/sim` with hostfs mounted to the same
directory as `wpa_supplicant-hwsim-p2p.conf`. The run reused explicit P2P
bindings and short 6-second iperf windows to catch any regression introduced by
the rebuild.

```text
Logs:
  /tmp/hwsim-p2p-refresh-p2p1.log
  /tmp/hwsim-p2p-refresh-p2p2.log

p2p1 / GO:
  P2P-GO-NEG-SUCCESS role=GO freq=2437
  P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-TE" freq=2437 go_dev_addr=42:00:00:00:00:14
  p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:15 p2p_dev_addr=42:00:00:00:00:15
  p2p-wlan0-0: EAPOL-4WAY-HS-COMPLETED 52:00:00:00:00:15

p2p2 / client:
  P2P-GO-NEG-SUCCESS role=client freq=2437
  P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-TE" freq=2437 go_dev_addr=42:00:00:00:00:14

GO -> client:
  ping -c 5 192.168.77.2
    -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 6
    -> 0.00-   6.02 sec     475136 Bytes    0.63 Mbits/sec
    -> iperf exit
  post-iperf ping -c 3 192.168.77.2
    -> 3 packets transmitted, 3 received, 0% packet loss

client -> GO:
  ping -c 5 192.168.77.1
    -> 5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 6
    -> 0.00-   6.02 sec    3244032 Bytes    4.31 Mbits/sec
    -> iperf exit
  first post-iperf ping -c 3 192.168.77.1
    -> 3 packets transmitted, 2 received, 33% packet loss
  immediate second post-iperf ping -c 3 192.168.77.1
    -> 3 packets transmitted, 3 received, 0% packet loss

Aggregation side evidence:
  Both peers logged `ampdu-proof: tx operational` on p2p-wlan0-0 for TID 0.
```

The one lost packet in the first client-side post-iperf ping matches earlier
transient warm-up/path timing behavior; the immediate second post-iperf ping
returned to 0% loss. No simulator processes were left running after `poweroff`
on both roles.

802.11r FT multi-BSS status:

- Single-BSS FT-PSK remains `PASS` for association, key derivation, ping, and
  STA-to-AP iperf.
- Multi-BSS FT-PSK now reaches a stronger control-plane milestone. With
  `hostapd-hwsim-ft-psk-multibss.conf`, scan results contain both BSSes:
  `02:00:00:00:10:00` and `02:00:00:00:10:01`.
- `wpa_cli -i wlan0 roam 02:00:00:00:10:01` now completes FT authentication
  and reassociation. STA logs show `Associated with 02:00:00:00:10:01`,
  `CTRL-EVENT-CONNECTED`, `bssid=02:00:00:00:10:01`,
  `key_mgmt=FT-PSK`, and `wpa_state=COMPLETED`.
- AP logs show target-BSS control frames are now accepted instead of failing
  `NL80211_CMD_FRAME`: `send_mlme ... sa=02:00:00:00:10:01
  bssid=02:00:00:00:10:01` followed by `Frame TX command accepted`.
- Problem: after the first multi-BSS roam proof, data traffic was still
  unhealthy. `/tmp/hwsim-ft-roam2-sta1.log` showed roam completed, then
  `ping -c 3 192.168.201.1` returned `3 packets transmitted, 0 received`.
- Cause: this was not an FT key/control-plane failure. In the current NuttX sim
  path there is no AP bridge/DS connecting the primary `wlan0` BSS and the
  secondary `ap1` BSS at the IP layer. Configuring only `wlan0` leaves `ap1`
  able to transmit/receive 802.11 management frames, but data frames received
  on `ap1` do not reach the AP's NuttX IPv4 endpoint.
- Fix/test requirement for STA-originated post-roam traffic: configure the
  target BSS netdev in NuttX as well:

  ```text
  ifconfig wlan0 192.168.201.1 netmask 255.255.255.0
  ifconfig ap1 192.168.201.1 netmask 255.255.255.0
  ```

- Result: with `ap1` configured, FT multi-BSS data traffic passes after roam.
  `/tmp/hwsim-ft-roam-ap1ip-sta1.log` shows
  `bssid=02:00:00:00:10:01`, `key_mgmt=FT-PSK`, `wpa_state=COMPLETED`,
  post-roam ping `3 packets transmitted, 2 received`, and STA-to-AP iperf
  `0.00-   6.02 sec    1490944 Bytes    1.98 Mbits/sec`.
  `/tmp/hwsim-ft-roam-ap1ip-2ping-sta1.log` then proves the transient first
  post-roam ICMP miss is not persistent: the first 5-packet ping after roam is
  4/5, and the immediate second 5-packet ping is 5/5 with 0% packet loss.
- Stronger 64 MiB rerun: `/tmp/hwsim-ft-roam64-ap.log` and
  `/tmp/hwsim-ft-roam64-sta1.log` again prove FT over-the-air reassociation to
  `02:00:00:00:10:01`: STA logs `FT: Completed successfully`,
  `Associated with 02:00:00:00:10:01`, `CTRL-EVENT-CONNECTED`,
  `bssid=02:00:00:00:10:01`, `key_mgmt=FT-PSK`, and `wpa_state=COMPLETED`.
  The first post-roam 10-packet ping was 9/10 due to initial ARP/path warm-up;
  the immediate second 10-packet ping was 10/10, and later ping was 3/3.
  STA->AP iperf after roam passed:
  `0.00-  12.04 sec    8306688 Bytes    5.52 Mbits/sec`.
- Additional AP-originated traffic rule: if `wlan0` and `ap1` both keep
  `192.168.201.1/24`, AP-originated ping/iperf can choose the old BSS and fail
  even though the STA is connected to target BSS `ap1`. Move the old BSS out of
  the target subnet, then bind AP-originated iperf to the target-BSS source:

  ```text
  ifconfig wlan0 192.168.202.1 netmask 255.255.255.0
  ifconfig ap1 192.168.201.1 netmask 255.255.255.0
  iperf -c 192.168.201.2 -B 192.168.201.1 -p 5112 -t 10
  ```

  With this setup AP->STA ping passed 5/5, unbound AP->STA iperf failed with
  source `192.168.202.1`, and bound AP->STA iperf passed:
  `0.00-  12.04 sec    9469952 Bytes    6.29 Mbits/sec`.
- Remaining FT cleanup item: the old BSS can still later emit an inactivity
  deauth for its stale station entry after the STA has roamed to `ap1`. The STA
  remains connected on the target BSS and data stays healthy, but old-BSS
  station timer cleanup should be tightened.

Dynamic PS / MLO validation entry point:

- Problem: Linux `mac80211_hwsim` exposes `radios`, `channels`, `mlo`, and
  `multi_radio` through `module_param()`, but the NuttX compatibility macro is
  intentionally a no-op. As a result, the imported hwsim code had no NuttX-side
  way to enable the MLO path that sets `SUPPORTS_DYNAMIC_PS` and `AP_LINK_PS`.
- Fix: added NuttX Kconfig replacements in
  `nuttx/drivers/wireless/virtual/Kconfig`:
  `CONFIG_WL_NUTTX_HWSIM_RADIOS`,
  `CONFIG_WL_NUTTX_HWSIM_CHANNELS`,
  `CONFIG_WL_NUTTX_HWSIM_MLO_SUPPORT`, and
  `CONFIG_WL_NUTTX_HWSIM_MULTI_RADIO_SUPPORT`. `mac80211_hwsim_linux.c` now
  seeds the imported module parameters from those Kconfig values.
- Isolation: added separate MLO/dynamic-PS validation objects instead of
  changing the baseline AP/STA/P2P configs:
  `tools/firmware/sim/build-dynps-ap.sh`,
  `tools/firmware/sim/build-dynps-sta1.sh`,
  `nuttx/boards/sim/sim/sim/configs/hwsim_dynps_ap/defconfig`, and
  `nuttx/boards/sim/sim/sim/configs/hwsim_dynps_sta1/defconfig`.
- Build proof: both dedicated objects build:
  `FeatherCore/build/nuttx-sim-dynps-ap` and
  `FeatherCore/build/nuttx-sim-dynps-sta1`. Logs:
  `/tmp/hwsim-build-dynps-ap.log` and
  `/tmp/hwsim-build-dynps-sta1.log`.
- Smoke proof: both images boot to NSH and expose the expected AP/STA userspace
  tools plus `hwsim_tm`; logs:
  `/tmp/hwsim-smoke-dynps-ap.log` and
  `/tmp/hwsim-smoke-dynps-sta1.log`.
- Runtime data smoke: `hwsim_dynps_ap` and `hwsim_dynps_sta1` now pass a
  minimal open-auth AP/STA run with MLO support enabled. Evidence:
  `/tmp/hwsim-dynps-open3-ap.log` and `/tmp/hwsim-dynps-open3-sta1.log`.
  The AP side shows:

  ```text
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  ping -c 3 192.168.203.2 -> 3 packets transmitted, 3 received, 0% packet loss
  AP -> STA iperf: 0.00-   6.02 sec     720896 Bytes    0.96 Mbits/sec
  ```

  The STA side shows:

  ```text
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  ping -c 3 192.168.203.1 -> 3 packets transmitted, 3 received, 0% packet loss
  STA -> AP iperf: 0.00-   6.02 sec     737280 Bytes    0.98 Mbits/sec
  ```

- Problems found during this smoke:
  - Starting `ifconfig`/`iperf` immediately after launching hostapd races AP
    interface creation in the dynps AP object; adding a short NSH `sleep` after
    `hostapd ... &` lets hostapd finish `AP-ENABLED` before IP setup.
  - The first AP->STA iperf attempt was refused because the STA script had not
    yet started its iperf server. The passing run starts the STA server before
    AP->STA traffic.
- Dynamic/auto-PS boundary proof: the first dedicated `PS_AUTO_POLL` run showed
  that the imported Linux hwsim behavior still accepted too much traffic and
  did not emit PS-Poll. The NuttX hwsim port now arms a delayed auto-poll work
  item for `PS_AUTO_POLL`, sends periodic PS-Poll frames, and gates STA RX with
  a finite auto-poll RX budget. The one-frame prototype passed ICMP but made
  AP->STA TCP stall after the first burst; the current budgeted version gives
  each poll period a 64-frame receive budget. Rebuild proof:
  `/tmp/hwsim-build-dynps-ap-autops-budget.log` and
  `/tmp/hwsim-build-dynps-sta1-autops-budget.log`.
  A fresh open-auth run (`/tmp/hwsim-dynps-autops-budget-ap.log` and
  `/tmp/hwsim-dynps-autops-budget-sta1.log`) shows the dynps objects still
  associate and pass baseline plus auto-PS traffic:

  ```text
  /tmp/hwsim-dynps-autops-budget-ap.log:
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  AP -> STA while ps=2: 3 packets transmitted, 3 received, 0% packet loss
  AP -> STA while ps=2 iperf:
  0.00-   6.02 sec     294912 Bytes    0.39 Mbits/sec

  /tmp/hwsim-dynps-autops-budget-sta1.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  baseline STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
  hwsim_tm get_ps -> ps=0
  hwsim_tm set_ps 2 -> OK
  hwsim_tm get_ps -> ps=2
  ps-proof: hwsim auto poll tx budget=64
  ps-proof: hwsim auto poll rx budget=63 ... rx budget=0
  hwsim_tm set_ps 0 -> OK
  hwsim_tm get_ps -> ps=0
  post-wake STA->AP ping: 3 packets transmitted, 3 received, 0% packet loss
  ```

  The shared-medium frame scan for this run found one PM=1 nullfunc (`48 11`),
  one PM=0 nullfunc (`48 01`), and 311 PS-Poll frames (`a4 10`). This proves
  that `PS_AUTO_POLL` no longer relies on accepting all received frames and now
  exercises automatic PS-Poll emission plus gated receive-budget delivery.
  Remaining issue from the same run: AP->STA TCP under `ps=2` is functional but
  low-throughput, and the STA log still shows budget exhaustion followed by
  `auto poll reject ... budget=0` bursts before the next poll budget refresh.
  The TCP server can also finish with `EAGAIN` after the client exits. This is
  now a throughput/service-period tuning problem rather than a missing
  automatic PS-Poll proof.
- Dynamic idle-timeout proof: `hwsim_tm set_ps 4` now arms a NuttX delayed-work
  idle timer for `PS_DYNAMIC_IDLE`. The STA starts awake, sends a PM=0 NullFunc
  on entry, transitions to sleep after `NUTTX_HWSIM_DYNPS_IDLE_MS`, and wakes
  on outbound data before rearming the idle timer. Rebuild proof:
  `/tmp/hwsim-build-dynps-idle-sta1-rerun.log` and
  `/tmp/hwsim-build-dynps-idle-ap-rerun.log`. A single-STA smoke also proved
  that the rebuilt dynps STA image registers `wlan0` correctly after a previous
  stale/competing build artifact had produced `netlib_setmacaddr(wlan0)
  failed: -1`; evidence: `/tmp/hwsim-dynps-sta1-single.log`.
- Runtime proof: `/tmp/hwsim-dynps-idle-ap-rerun.log` and
  `/tmp/hwsim-dynps-idle-sta1-rerun.log` show AP/STA association, baseline
  STA->AP ping, dynamic idle sleep, sleep-period AP->STA rejection, STA TX wake,
  and post-disable AP->STA recovery:

2026-06-06 dynps role-object refresh:

```text
/tmp/hwsim-build-dynps-ap-refresh.log:
  FeatherCore/tools/firmware/sim/build-dynps-ap.sh -j8 -> dynps_ap_rc=0
  build/nuttx-sim-dynps-ap size: 23353584 bytes

/tmp/hwsim-build-dynps-sta1-refresh.log:
  FeatherCore/tools/firmware/sim/build-dynps-sta1.sh -j8 -> dynps_sta1_rc=0
  build/nuttx-sim-dynps-sta1 size: 27714792 bytes

/tmp/hwsim-boot-dynps-ap-refresh.log   -> reached NuttShell, poweroff, dynps_ap_boot_rc=0
/tmp/hwsim-boot-dynps-sta1-refresh.log -> reached NuttShell, poweroff, dynps_sta1_boot_rc=0
```

  ```text
  /tmp/hwsim-dynps-idle-sta1-rerun.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  baseline STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
  hwsim_tm set_ps 4 -> OK
  hwsim_tm get_ps -> ps=4
  ps-proof: hwsim dynps idle sleep timeout_ms=500
  ps-proof: hwsim dynps reject sleeping=1 own=1 len=116
  ps-proof: hwsim dynps tx wake len=116
  STA -> AP while dynps enabled: 3 packets transmitted, 3 received, 0% packet loss
  hwsim_tm set_ps 0 -> OK
  hwsim_tm get_ps -> ps=0

  /tmp/hwsim-dynps-idle-ap-rerun.log:
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  AP -> STA while STA is dynps sleeping:
  3 packets transmitted, 0 received, 100% packet loss
	  AP -> STA after set_ps 0:
	  3 packets transmitted, 3 received, 0% packet loss
	  ```

	  This closes the dynamic idle-timeout proof gap.
- AP link PS proof: MLO-enabled dynps objects now validate the
  `IEEE80211_HW_AP_LINK_PS` driver responsibility path. The hwsim AP receive
  helper reports STA PM-bit transitions with `ieee80211_sta_ps_transition_ni()`
  and reports PS-Poll frames with `ieee80211_sta_pspoll()`. The first attempt
  found that `ieee80211_find_sta_by_link_addrs()` did not match the open-auth
  AP/STA path, so the helper now falls back to `ieee80211_find_sta_by_ifaddr()`
  with the frame's STA and local AP addresses.
- Rebuild proof: `/tmp/hwsim-build-aplinkps-sta1-fallback.log` and
  `/tmp/hwsim-build-aplinkps-ap-fallback.log` both completed with `rc=0`.
  Runtime proof: `/tmp/hwsim-aplinkps2-ap.log` and
  `/tmp/hwsim-aplinkps2-sta1.log` show association, AP-link sleep transition,
  AP-side buffering, PS-Poll delivery, wake transition, and post-wake
  bidirectional ping:

  ```text
  /tmp/hwsim-aplinkps2-sta1.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  baseline STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
  hwsim_tm get_ps -> ps=0
  hwsim_tm set_ps 1 -> OK
  hwsim_tm get_ps -> ps=1
  hwsim_tm poll -> OK
  hwsim_tm get_ps -> ps=3
  ps-proof: hwsim manual poll rx one
  hwsim_tm set_ps 0 -> OK
  hwsim_tm get_ps -> ps=0
  post-wake STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss

  /tmp/hwsim-aplinkps2-ap.log:
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  ps-proof: sta sleep sta=02:00:00:00:00:1f aid=1
  ap-link-ps-proof: transition sta=02:00:00:00:00:1f link=0 start=1 ret=0
  ps-proof: buffer unicast sta=02:00:00:00:00:1f aid=1 total=1
  AP -> STA while STA sleeps: 1 packets transmitted, 0 received, 100% packet loss
  ap-link-ps-proof: pspoll sta=02:00:00:00:00:1f link=0
  ps-proof: pspoll rx sta=02:00:00:00:00:1f aid=1
  ps-proof: deliver response sta=02:00:00:00:00:1f requested=1 frames=1
  ps-proof: allow buffered sta=02:00:00:00:00:1f num=1 more=0
  ps-proof: sta wake sta=02:00:00:00:00:1f aid=1
  ps-proof: wake deliver sta=02:00:00:00:00:1f buffered=0 total=0
  ap-link-ps-proof: transition sta=02:00:00:00:00:1f link=0 start=0 ret=0
  post-wake AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
  ```

  Remaining dynps gap: AP->STA TCP under auto-PS still needs
  throughput/service-period tuning.
- Problem found while validating: AP and STA sim configs cannot be built in
  parallel from the same NuttX tree. A concurrent AP/STA attempt stomped the
  shared generated config area and failed with
  `mv: cannot stat 'include/nuttx/config.h.tmp'`. Fix/test rule: build hwsim
  role images sequentially, or use separate NuttX build trees if parallel
  builds are required.

## Dynamic Auto-PS Bulk Poll Retest

The latest dynamic-PS run narrows the remaining auto-PS issue to AP-to-STA TCP
service-period tuning. Automatic PS-Poll is now emitted and consumed by the AP,
and the AP can release buffered frames in bulk instead of one frame per poll.

- Fix: hwsim now marks simulator-generated auto PS-Poll frames separately from
  manual PS-Poll frames, so the AP receive helper can call
  `ieee80211_sta_pspoll_n(sta, 64)` for auto-poll service periods while keeping
  manual `hwsim_tm poll` behavior on the normal one-frame
  `ieee80211_sta_pspoll()` path.
- Fix: `ieee80211_sta_pspoll_n()` lives in `mac80211/sta_info.c` next to the
  internal buffered-frame release helper, and proof logs in hwsim, mac80211 PS
  delivery, and PS buffering were throttled so the test output no longer hides
  the real traffic behavior.
- Build proof:

  ```text
  FeatherCore/tools/firmware/sim/build-dynps-sta1.sh -j8
    -> rc=0
    -> build/nuttx-sim-dynps-sta1
    -> size: 27409896 bytes

  FeatherCore/tools/firmware/sim/build-dynps-ap.sh -j8
    -> rc=0
    -> build/nuttx-sim-dynps-ap
    -> size: 23120360 bytes
  ```

- Runtime proof from `/tmp/hwsim-autops-buffer-throttle-ap.log` and
  `/tmp/hwsim-autops-buffer-throttle-sta1.log`:

  ```text
  STA: wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  STA baseline ping -> AP: 3 packets transmitted, 3 received, 0% packet loss
  STA hwsim_tm get_ps -> ps=0
  STA hwsim_tm set_ps 2 -> OK
  STA hwsim_tm get_ps -> ps=2
  STA auto-PS ping -> AP: 3 packets transmitted, 3 received, 0% packet loss

  AP: wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  AP auto-PS ping -> STA: 3 packets transmitted, 3 received, 0% packet loss
  AP auto-PS iperf -> STA:
    0.00-   3.01 sec     196608 Bytes    0.52 Mbits/sec
    3.01-   6.02 sec          0 Bytes    0.00 Mbits/sec
    0.00-   6.02 sec     196608 Bytes    0.26 Mbits/sec
  ```

- Buffered-frame release proof: the AP log shows bulk auto-poll service periods
  such as `requested=64 frames=64`, followed by smaller non-empty releases
  (`frames=22`, `frames=9`, `frames=3`, `frames=2`, `frames=1`). The latest
  run captured 66 non-empty bulk releases.
- Remaining problem: AP-to-STA TCP under `ps=2` still fills the AP sleep queue
  (`queue=64` samples remain, 19 in the latest throttled log) and then stalls
  after the first burst. The STA iperf server reports repeated tiny receive
  intervals and exits with error 107 after the client closes. This means the
  auto-PS data path is functional, but the AP-to-STA TCP path still needs
  service-period timing, ACK flow, or sleep-buffer tuning.

Follow-up auto-poll interval tuning:

- Problem: a first attempt to reduce the auto-poll interval from 20 ms to 5 ms
  exposed a NuttX sim timing hazard. `msecs_to_jiffies(5)` can become 0 ticks,
  so the delayed work requeued immediately and produced a near busy-loop
  (`sample` count reached tens of thousands in one short run). That run was
  discarded as invalid evidence.
- Fix: the hwsim auto-poll work now schedules with
  `max_t(unsigned long, 1, msecs_to_jiffies(...))`, and the trial interval was
  set to 10 ms. The same change raised the simulator auto-poll release request
  to 256 frames and the STA RX budget to 4096 frames, while keeping manual
  PS-Poll on the one-frame path.
- Build proof after the min-1 scheduling fix:

  ```text
  FeatherCore/tools/firmware/sim/build-dynps-sta1.sh -j8
    -> rc=0
    -> build/nuttx-sim-dynps-sta1
    -> size: 27410064 bytes

  FeatherCore/tools/firmware/sim/build-dynps-ap.sh -j8
    -> rc=0
    -> build/nuttx-sim-dynps-ap
    -> size: 23120536 bytes
  ```

- Runtime proof from `/tmp/hwsim-autops-min1-ap.log` and
  `/tmp/hwsim-autops-min1-sta1.log`:

  ```text
  STA: wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  STA hwsim_tm get_ps -> ps=2
  STA auto-PS ping -> AP: 3 packets transmitted, 3 received, 0% packet loss

  AP: wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  AP auto-PS ping -> STA: 3 packets transmitted, 3 received, 0% packet loss
  AP auto-PS iperf -> STA:
    0.00-   3.01 sec     196608 Bytes    0.52 Mbits/sec
    3.01-   6.02 sec          0 Bytes    0.00 Mbits/sec
    0.00-   6.02 sec     196608 Bytes    0.26 Mbits/sec

  STA iperf server:
    0.00-   3.01 sec       4288 Bytes    0.01 Mbits/sec
    3.01-   6.02 sec       3752 Bytes    0.01 Mbits/sec
    6.02-   9.03 sec       4288 Bytes    0.01 Mbits/sec
    9.03-  12.04 sec       4056 Bytes    0.01 Mbits/sec
    tcp server recv error, error code: 107
  ```

- Result: the min-1 scheduling fix closes the auto-poll busy-loop hazard, but
  the AP-to-STA TCP bottleneck is unchanged. The AP still reports 65 non-empty
  bulk releases and 19 sampled `queue=64` events. This points away from the
  simple poll-interval/release-count hypothesis and toward TCP ACK/service
  period interaction or mac80211 sleeping-STA buffering semantics.

Follow-up sleeping-STA buffer sizing proof:

- Problem: the min-1 run showed repeated AP-side `queue=64` samples. Source
  audit confirmed the upstream mac80211 sleeping-STA path uses
  `STA_MAX_TX_BUFFER=64`; when that per-AC buffer is full,
  `ieee80211_tx_h_unicast_ps_buf()` drops the oldest queued frame. That means
  AP-to-STA TCP can lose data in mac80211 before the next auto-poll service
  period has a chance to release it.
- Fix/test change: for `CONFIG_ARCH_SIM`, raise `STA_MAX_TX_BUFFER` to 512 and
  `TOTAL_MAX_TX_BUFFER` to 2048, and match the hwsim auto-poll release request
  to 512 frames with an 8192-frame STA RX budget. This is a simulator validation
  knob to prove the bottleneck; manual PS-Poll remains on the one-frame path.
- Build proof after the sim buffer-size change:

  ```text
  FeatherCore/tools/firmware/sim/build-dynps-sta1.sh -j8
    -> rc=0
    -> build/nuttx-sim-dynps-sta1
    -> size: 27410072 bytes

  FeatherCore/tools/firmware/sim/build-dynps-ap.sh -j8
    -> rc=0
    -> build/nuttx-sim-dynps-ap
    -> size: 23120544 bytes
  ```

- Runtime proof from `/tmp/hwsim-autops-bigbuf-ap.log` and
  `/tmp/hwsim-autops-bigbuf-sta1.log`:

  ```text
  STA: wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  baseline STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
  hwsim_tm get_ps -> ps=0
  hwsim_tm set_ps 2 -> OK
  hwsim_tm get_ps -> ps=2
  auto-PS STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss

  AP: wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  auto-PS AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
  AP auto-PS iperf -> STA:
    0.00-   3.01 sec     409600 Bytes    1.09 Mbits/sec
    3.01-   6.02 sec     573440 Bytes    1.52 Mbits/sec
    0.00-   6.02 sec     983040 Bytes    1.31 Mbits/sec

  STA iperf server:
    0.00-   3.01 sec     218296 Bytes    0.58 Mbits/sec
    3.01-   6.02 sec     567624 Bytes    1.51 Mbits/sec
    6.02-   9.03 sec     135608 Bytes    0.36 Mbits/sec
    closed by the peer: 192.168.203.1:13430
  ```

- Result: this confirms the 64-frame sleeping-STA buffer was a real AP-to-STA
  auto-PS TCP bottleneck. The latest run has 79 non-empty bulk releases, no
  sampled `queue=512` or `queue=64` full-buffer event, and no STA-side
  `tcp server recv error`. AP-to-STA TCP is now functional across both client
  intervals, though throughput is still below the non-PS MLO data smoke.

Reverse auto-PS TCP proof:

- Runtime proof from `/tmp/hwsim-autops-bigbuf-sta2ap-ap.log` and
  `/tmp/hwsim-autops-bigbuf-sta2ap-sta1.log` reuses the same
  `CONFIG_ARCH_SIM` buffer sizing and auto-poll settings, but drives TCP from
  the sleeping STA toward the AP while `hwsim_tm set_ps 2` is active.

  ```text
  STA: wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:1e completed
  baseline STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
  hwsim_tm set_ps 2 -> OK
  auto-PS STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss

  AP: wlan0: AP-STA-CONNECTED 02:00:00:00:00:1f
  AP iperf server accepted: 192.168.203.2:26062

  STA auto-PS iperf -> AP:
    0.00-   3.01 sec    1556480 Bytes    4.14 Mbits/sec
    3.01-   6.02 sec    1114112 Bytes    2.96 Mbits/sec
    0.00-   6.02 sec    2670592 Bytes    3.55 Mbits/sec

  AP iperf server:
    0.00-   3.01 sec    1499086 Bytes    3.98 Mbits/sec
    3.01-   6.02 sec     981130 Bytes    2.61 Mbits/sec
    closed by the peer: 192.168.203.2:26062
  ```

- Result: auto-PS TCP now has bidirectional functional evidence. The reverse
  STA-to-AP run had 99 non-empty bulk releases, 66 sampled auto-poll TX budget
  logs, no sampled `queue=512` or `queue=64` full-buffer event, and no
  `tcp server recv error`. Throughput is still below non-PS runs, so this
  remains a performance/timing tuning item rather than a functional blocker.

## DPP / Easy Connect Runtime

DPP-PSK provisioning is now validated end to end with AP configurator and STA
enrollee simulator objects.

- Build/source changes:
  - `apps/wireless/wifi/wpa_hostapd_sources.mk` now enables `CONFIG_DPP`,
    `CONFIG_DPP2`, and `CONFIG_GAS_SERVER` for `CONFIG_ARCH_SIM`.
  - The DPP common/AP/supplicant sources and `src/common/gas_server.c` are in
    the active sim source list.
  - `apps/wireless/wifi/common/nuttx_wpa_openssl_ec.c` now provides the EC-key
    APIs needed by the DPP bootstrap/auth code paths, including key generation,
    public point export, ECPrivateKey export, public-key comparison, and raw
    `r || s` ECDSA sign/verify helpers.
  - DPP CSR/PKCS#7 helpers are present as explicit unsupported stubs. This
    keeps the non-certificate DPP paths buildable while marking CSR/CA flows as
    pending instead of pretending they work.
- Build proof:

  ```text
  FeatherCore/tools/firmware/sim/build-ap.sh -j8
    -> rc=0
    -> build/nuttx-sim-ap
    -> size: 23098584 bytes

  FeatherCore/tools/firmware/sim/build-sta1.sh -j8
    -> rc=0
    -> build/nuttx-sim-sta1
    -> size: 27380008 bytes
  ```

- Runtime setup:
  - Start `../../../build/nuttx-sim-sta1` and `../../../build/nuttx-sim-ap`
    from `tools/firmware/sim`.
  - Mount hostfs in each NSH before referencing `/h/...`:

    ```text
    mkdir /h
    mount -t hostfs -o fs=. /h
    ```

  - STA enrollee:

    ```text
    wifi_sta_demo wlan0 192.168.201.2 255.255.255.0 dppnet
    wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-dpp-enrollee.conf &
    wpa_cli -i wlan0 dpp_bootstrap_gen type=qrcode chan=81/1 mac=020000000002
    wpa_cli -i wlan0 dpp_bootstrap_get_uri 1 > /h/hwsim-dpp-uri.txt
    wpa_cli -i wlan0 dpp_listen 2412
    ```

  - AP configurator:

    ```text
    wifi_ap_demo wlan0 192.168.201.1 255.255.255.0 dppnet
    hostapd -dd /h/hostapd-hwsim-dpp-psk.conf &
    hostapd_cli -i wlan0 dpp_configurator_add
    hostapd_cli -i wlan0 dpp_qr_code @/h/hwsim-dpp-uri.txt
    hostapd_cli -i wlan0 dpp_auth_init @/h/hwsim-dpp-auth.txt
    ```

- DPP provisioning proof:

  ```text
  hostapd_cli -i wlan0 dpp_qr_code @/h/hwsim-dpp-uri.txt -> 1
  hostapd_cli -i wlan0 dpp_auth_init @/h/hwsim-dpp-auth.txt -> OK

  nl80211: CMD_FRAME freq=2412 wait=2000 ...
  nl80211: Frame TX command accepted; cookie 0x2
  wlan0: DPP-TX-STATUS dst=02:00:00:00:00:02 result=SUCCESS
  wlan0: DPP-AUTH-SUCCESS init=1 ...
  DPP: Configuration exchange completed (ok=1)
  wlan0: DPP-CONF-SENT conf_status=0

  wlan0: DPP-AUTH-SUCCESS init=0 ...
  DPP: Try to connect after completed configuration result
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  wpa_cli -i wlan0 status -> wpa_state=COMPLETED, key_mgmt=WPA2-PSK, pmf=1
  ```

- Data-plane proof after DPP connector-based association:

  ```text
  AP -> STA ping: 3 packets transmitted, 3 received, 0% packet loss
  STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
  STA -> AP iperf: 0.00-   6.02 sec    1245184 Bytes    1.65 Mbits/sec
  AP -> STA iperf: 0.00-   6.02 sec    1458176 Bytes    1.94 Mbits/sec
  ```

- Problems found and fixes:
  - Problem: after enabling `CONFIG_DPP`, the simulator link failed on missing
    `crypto_ec_key_*`, `crypto_csr_*`, `crypto_pkcs7_get_certificates`, and
    `gas_server_*` symbols. Fix: added the missing GAS server source and
    implemented the non-certificate EC-key helpers in the existing simulator
    crypto glue. CSR/PKCS#7 helpers are still pending real implementation.
  - Problem: the first AP `hostapd_cli` smoke hung on `ping`. This was not a
    DPP failure; the open `hostapd-hwsim.conf` file did not enable a control
    interface. Fix: added `tools/firmware/sim/hostapd-hwsim-dpp.conf` with
    `ctrl_interface=udp:9877`.
  - Problem: a long AP-side `dpp_bootstrap_gen ... mac=02:00:00:00:00:01`
    command was split by NSH input handling, leaving `:01` as a separate
    command. Workaround for the current smoke: use the shorter AP bootstrap
    command without `mac=`.
  - Problem: full DPP auth initially crashed with `wpabuf size=64 used=65
    overflow`. Cause: `crypto_ec_key_get_pubkey_point(prefix=0)` allocated only
    the stripped `X || Y` length while OpenSSL first wrote the uncompressed
    `0x04 || X || Y` point. Fix: allocate the full encoded EC point and then
    drop the prefix before returning the 64-byte DPP public point.
  - Problem: AP `dpp_qr_code` / `dpp_auth_init` arguments are too long for
    reliable NSH interactive input. Fix: `hostapd_cli` and `wpa_cli` support
    `@file` arguments for DPP commands, so the URI and auth parameters can be
    passed as `/h/hwsim-dpp-uri.txt` and `/h/hwsim-dpp-auth.txt`.
  - Problem: AP-side DPP Authentication Request failed with `CMD_FRAME
    wait=2000` returning `-EINVAL`. Cause: `nl80211_tx_mgmt()` rejects
    `NL80211_ATTR_DURATION` values above the wiphy advertised
    `max_remain_on_channel_duration`, while hwsim declared only 1000 ms. Fix:
    the NuttX hwsim sim driver now advertises 5000 ms, matching mac80211's
    fallback and allowing DPP public action exchanges.
- Remaining gap: DPP-PSK provisioning is proven. CSR/PKCS#7 certificate-mode
  DPP remains blocked on real crypto backend implementation, and repeated
  long-run/negative DPP cases still need coverage.

### FILS/OCV build enable, OCV runtime proof, and FILS-SHA256 proof

Progress:

- `apps/wireless/wifi/wpa_hostapd_sources.mk` now enables
  `CONFIG_FILS`, `CONFIG_OCV`, and `CONFIG_ERP`.
- The AP/STA source inventory includes `wpa_supplicant-2.11/src/common/ocv.c`
  and `wpa_supplicant-2.11/src/ap/fils_hlp.c`.
- Build proof:

  ```text
  tools/firmware/sim/build-ap.sh -j8
    -> rc=0
    -> build/nuttx-sim-ap
    -> size: 23354736 bytes

  tools/firmware/sim/build-sta1.sh -j8
    -> rc=0
    -> build/nuttx-sim-sta1
    -> size: 27705648 bytes
  ```

- Symbol inventory:

  ```text
  nm build/nuttx-sim-ap
    -> ocv_*, hostapd_eid_fils_indic, fils_hlp_finish_assoc,
       fils_rmsk_to_pmk, fils_pmk_to_ptk

  nm build/nuttx-sim-sta1
    -> ocv_*, fils_build_auth, fils_process_auth,
       fils_process_assoc_resp, fils_rmsk_to_pmk, fils_pmk_to_ptk
  ```

OCV runtime setup:

```text
# AP
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.201.1 netmask 255.255.255.0
hostapd -dd /h/hostapd-hwsim-wpa2-psk-ocv.conf &

# STA1
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.201.2 netmask 255.255.255.0
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-wpa2-psk-ocv.conf &
```

OCV association proof:

```text
hostapd:
  wlan0: AP-ENABLED
  Beacon/probe/assoc response IEs include OCV extended capabilities
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

wpa_supplicant:
  ieee80211w=2
  WPA: WPA IE for msg 2/4 ... dd 07 00 0f ac 0d 51 01 00
  WPA: OCI KDE in EAPOL-Key - hexdump(len=9): dd 07 00 0f ac 0d 51 01 00
  wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
```

OCV data-plane proof:

```text
STA1 -> AP ping:
  3 packets transmitted, 3 received, 0% packet loss

STA1 -> AP iperf:
  0.00-   6.02 sec     311296 Bytes    0.41 Mbits/sec

AP -> STA1 ping:
  3 packets transmitted, 3 received, 0% packet loss

AP -> STA1 iperf:
  0.00-   6.02 sec     360448 Bytes    0.48 Mbits/sec
```

Problems and follow-ups:

- Problem: older AP->STA1 OCV iperf runs hit the known
  `hwsim-frames.bin` 8 MiB bounded-medium truncate path. This did not break
  the functional proof, but it kept throughput/performance tuning separate.
  Fix: the shared medium limit is now configurable through
  `CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES`, defaulting to 64 MiB. Clean
  validation runs should still delete stale medium files, and longer
  debug-heavy soaks still need transport/throughput tuning.
- Problem: after earlier parallel role builds, generated dependency files under
  `apps`/`nuttx` could contain malformed absolute-path targets and fail with
  `Make.dep: multiple target patterns`. Fix/workaround for this run was to
  delete generated `Make.dep` and `*.ddc` files and rebuild AP then STA1
  serially.
- Problem: the first FILS AP run failed on `eap_server_erp` because ERP support
  was not compiled into the AP image. Fix: enable `CONFIG_ERP` in
  `wpa_hostapd_sources.mk` together with `CONFIG_FILS`.
- Problem: the cached FILS reconnect path initially failed before completing
  association because the NuttX compatibility crypto layer still treated
  `ctr(aes)` skcipher as a no-op. AP-side AES-SIV decrypt then saw an all-zero
  encrypted Association Request tail. Fix: `include/crypto/skcipher.h` now keeps
  the scatterlist request state and runs real AES-CTR over it with the local
  `rijndael` implementation.
- Problem: after the CTR fix, AP-side FILS still rejected the encrypted
  Association Request with an AES-SIV tag mismatch. Cause: `cmac(aes)` was also
  a zero-output stub, so S2V generated the wrong synthetic IV. Fix:
  `include/crypto/hash.h` now implements the streaming AES-CMAC subset required
  by FILS AES-SIV, again backed by local `rijndael`.
- Problem: enabling NuttX `CONFIG_CRYPTO` directly produced a local
  `libcrypto.a` that shadowed host OpenSSL during the sim link and caused
  unresolved OpenSSL EC/BN symbols. Fix: keep `CONFIG_CRYPTO` disabled for these
  role defconfigs and compile `nuttx/crypto/rijndael.c` privately into the
  mac80211/cfg80211 Wi-Fi port build.

FILS-SHA256 runtime setup:

```text
# AP
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.202.1 netmask 255.255.255.0
hostapd -dd /h/hostapd-hwsim-fils-sha256.conf &

# STA1
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.202.2 netmask 255.255.255.0
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-fils-sha256.conf &
```

FILS-SHA256 association proof:

```text
hostapd:
  wlan0: AP-ENABLED
  Beacon/probe response RSN IE advertises AKM 00-0f-ac:0e (FILS-SHA256)
  Beacon/probe response includes FILS Indication IE
  wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
  EAP: Stored ERP keys 16aa9b561a593459@example.com
  wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

wpa_supplicant:
  wlan0: WPA: using KEY_MGMT FILS-SHA256
  EAP: Stored ERP keys 16aa9b561a593459@example.com
  wlan0: PMKSA-CACHE-ADDED 02:00:00:00:00:01 0
  nl80211: Add PMKSA for cache id 0011 SSID nuttx-hwsim-fils
  wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
```

FILS-SHA256 data-plane proof:

```text
AP -> STA1 ping:
  3 packets transmitted, 3 received, 0% packet loss

STA1 -> AP ping:
  3 packets transmitted, 3 received, 0% packet loss

STA1 -> AP iperf:
  0.00-   6.02 sec    2064384 Bytes    2.74 Mbits/sec

AP -> STA1 iperf:
  0.00-   6.02 sec     770048 Bytes    1.02 Mbits/sec
```

FILS-SHA256 fast-reconnect and HLP proof:

The first association intentionally has no existing ERP/PMKSA material, so it
still uses EAP-PSK to seed ERP and PMKSA. A same-supplicant-context reconnect
now proves the cached FILS path explicitly after the AES-CTR/AES-CMAC fixes.

```text
Build:
  ./build-ap.sh -j8 > /tmp/hwsim-build-fils-cmac-ap.log 2>&1
    -> rc=0
    -> build/nuttx-sim-ap size: 23394624 bytes

  ./build-sta1.sh -j8 > /tmp/hwsim-build-fils-cmac-sta1.log 2>&1
    -> rc=0
    -> build/nuttx-sim-sta1 size: 27755832 bytes

STA1 log /tmp/hwsim-fils-cmac-sta1.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  EAP: Valid ERP key found 16aa9b561a593459@example.com (SEQ=0)
  FILS: Try to use FILS (erp=1 pmksa_cache=1)
  FILS: Auth+Assoc completed successfully
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed [id=0 id_str= FILS_HLP_SENT]

AP log /tmp/hwsim-fils-cmac-ap.log:
  FILS: Found matching PMKSA cache entry
  FILS: Decrypted Association Request elements - hexdump(len=159): ...
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

Post-fast-reconnect AP -> STA1 ping:
  3 packets transmitted, 3 received, 0% packet loss
```

FILS-SHA256 STA2-present fast-reconnect and STA-to-STA stress proof:

The follow-up run kept STA2 associated while STA1 disconnected and reconnected
through cached FILS. This proves the AP can keep another encrypted STA active
while processing the FILS reconnect path for STA1.

```text
Build:
  ./build-sta2.sh -j8 > /tmp/hwsim-build-fils-cmac-sta2.log 2>&1
    -> rc=0
    -> build/nuttx-sim-sta2 size: 27712000 bytes

AP log /tmp/hwsim-fils-sta2stress-ap.log:
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
  wlan0: AP-STA-DISCONNECTED 02:00:00:00:00:02
  FILS: Found matching PMKSA cache entry
  FILS: Decrypted Association Request elements - hexdump(len=159): ...
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1 log /tmp/hwsim-fils-sta2stress-sta1.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  wlan0: CTRL-EVENT-DISCONNECTED bssid=02:00:00:00:00:01 reason=3 locally_generated=1
  SME: Try to use FILS with PMKSA caching
  EAP: Valid ERP key found 16aa9b561a593459@example.com (SEQ=0)
  FILS: Try to use FILS (erp=1 pmksa_cache=1)
  FILS: Auth+Assoc completed successfully
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed [id=0 id_str= FILS_HLP_SENT]

STA2 log /tmp/hwsim-fils-sta2stress-sta2.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
```

Post-reconnect STA-to-STA data proof:

```text
STA1 -> STA2 ping:
  3 packets transmitted, 3 received, 0% packet loss

STA2 -> STA1 ping:
  3 packets transmitted, 3 received, 0% packet loss

STA1 -> STA2 iperf:
  0.00-   6.02 sec    1048576 Bytes    1.39 Mbits/sec

STA2 -> STA1 iperf:
  0.00-   6.02 sec     573440 Bytes    0.76 Mbits/sec

Post-iperf STA1 -> STA2 ping:
  3 packets transmitted, 3 received, 0% packet loss
```

Known caveat: this older run hit the bounded hostfs hwsim medium truncate path
at 8 MiB during iperf. This did not break the proof, but repeated long-run FILS
stress and throughput/medium tuning remain follow-up work. The shared medium
limit has since been raised to a configurable 64 MiB default. The FILS control
path, ERP/PMKSA cache use, AES-SIV decrypt, HLP-sent indication, STA2-present
reconnect, and bidirectional post-reconnect STA-to-STA traffic are now proven.

Additional FILS AP + STA1 + STA2 data-plane rerun:

The follow-up run kept the same FILS AP active with two encrypted STAs on
`192.168.203.0/24` and verified direct STA-to-STA traffic again without a
script wrapper. This was not another cached-reconnect loop; it was a focused
two-STA data-plane confirmation after the FILS AES-CTR/AES-CMAC fixes.

```text
AP log /tmp/hwsim-fils-reconn3-ap.log:
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:03

STA1 log /tmp/hwsim-fils-reconn3-sta1.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  STA1 -> STA2 ping:
    3 packets transmitted, 3 received, 0% packet loss
  STA1 -> STA2 iperf:
    0.00-   6.02 sec     458752 Bytes    0.61 Mbits/sec

STA2 log /tmp/hwsim-fils-reconn3-sta2.log:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  STA2 -> STA1 ping:
    3 packets transmitted, 3 received, 0% packet loss
  STA2 -> STA1 iperf:
    0.00-   6.02 sec     819200 Bytes    1.09 Mbits/sec
```

The AP and STA1 logs again showed the then-known bounded medium truncate
message at 8 MiB near the end of the iperf run. Functional association,
bidirectional ping, and bidirectional iperf had already completed; the truncate
remains a throughput/long-run tuning issue, not a FILS control-path failure.
Later runs use the configurable 64 MiB default medium limit.

FILS-SHA256 three-cycle cached reconnect loop with STA2 online:

The latest run kept AP, STA1, and STA2 as three independent simulator objects
on `192.168.204.0/24`. STA2 stayed associated while STA1 performed three
manual `wpa_cli -i wlan0 disconnect` / `reconnect` cycles in the same
supplicant process, proving ERP/PMKSA cache reuse across repeated FILS
reconnects rather than a single smoke reconnect.

```text
Logs:
  /tmp/hwsim-fils-loop-ap.log
  /tmp/hwsim-fils-loop-sta1.log
  /tmp/hwsim-fils-loop-sta2.log

AP:
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
  Three STA1 reconnect cycles:
    FILS: Found matching PMKSA cache entry
    FILS: Decrypted Association Request elements - hexdump(len=159): ...
    wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  Initial connect:
    wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  Reconnect cycle 1:
    EAP: Valid ERP key found ... (SEQ=0)
    FILS: Try to use FILS (erp=1 pmksa_cache=1)
    FILS: Auth+Assoc completed successfully
    wlan0: CTRL-EVENT-CONNECTED - ... FILS_HLP_SENT
  Reconnect cycle 2:
    EAP: Valid ERP key found ... (SEQ=1)
    FILS: Try to use FILS (erp=1 pmksa_cache=1)
    FILS: Auth+Assoc completed successfully
    wlan0: CTRL-EVENT-CONNECTED - ... FILS_HLP_SENT
  Reconnect cycle 3:
    EAP: Valid ERP key found ... (SEQ=2)
    FILS: Try to use FILS (erp=1 pmksa_cache=1)
    FILS: Auth+Assoc completed successfully
    wlan0: CTRL-EVENT-CONNECTED - ... FILS_HLP_SENT

Counters:
  STA1 CTRL-EVENT-CONNECTED: 4
  STA1 FILS_HLP_SENT: 3
  AP PMKSA cache hits: 3
  AP decrypted encrypted Association Requests: 3

Post-loop STA1 -> STA2 ping:
  3 packets transmitted, 3 received, 0% packet loss

Post-loop STA2 -> STA1 ping:
  3 packets transmitted, 3 received, 0% packet loss

Post-loop STA1 -> STA2 iperf:
  0.00-   6.02 sec     393216 Bytes    0.52 Mbits/sec

Post-loop STA2 -> STA1 iperf:
  0.00-   6.02 sec     327680 Bytes    0.44 Mbits/sec
```

Known caveat: the debug-heavy run hit the bounded hostfs hwsim medium truncate
path during iperf on AP/STA1. The three cached reconnect cycles and both
post-loop iperf clients completed; this remains a medium/throughput tuning
issue rather than a FILS control-path failure.

## Hostfs Medium Limit Retest

The hwsim hostfs shared frame medium is now bounded by
`CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES` instead of a hard-coded 8 MiB limit.
The Kconfig default is 64 MiB (`67108864`) with a range of 1 MiB to 256 MiB, so
debug-heavy validation can run longer while the host-side backing file remains
bounded.

Build proof:

```text
./build-ap.sh -j8   -> PASS, ../build/nuttx-sim-ap
./build-sta1.sh -j8 -> PASS, ../build/nuttx-sim-sta1
FeatherCore/nuttx/.config:
  CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES=67108864
```

Runtime proof on the full `hostapd` + `wpa_supplicant` path:

```text
Logs:
  /tmp/hwsim-medium64-ap.log
  /tmp/hwsim-medium64-sta1.log

AP:
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

AP -> STA1:
  ping -c 3 192.168.201.2
    3 packets transmitted, 3 received, 0% packet loss
  iperf -c 192.168.201.2 -p 5099 -t 20
    0.00-  21.07 sec    3260416 Bytes    1.24 Mbits/sec
    iperf exit

STA1 -> AP:
  ping -c 3 192.168.201.1
    3 packets transmitted, 3 received, 0% packet loss
  iperf -c 192.168.201.1 -p 5100 -t 20
    0.00-  21.07 sec    2392064 Bytes    0.91 Mbits/sec
    iperf exit
```

The old 8 MiB false limit is gone. A delayed server-side close from the earlier
AP->STA1 iperf still eventually hit the new configured bound:

```text
hwsim-debug: truncating shared frame medium size=67109400 limit=67108864 path=/h/hwsim-frames.bin
```

That confirms the new limit is active and also preserves the remaining caveat:
long debug-heavy runs can still reach the configured bound, so repeated soak
testing should either clean `hwsim-frames.bin`/`hwsim-bss.bin` between cases or
continue improving the shared-medium transport and queue cleanup.

## 802.11b Current-Image Refresh

Pending validation items for this pass:

- 802.11b open-auth AP + STA baseline on channel 1.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- Association proof plus bidirectional ping and iperf.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-11b-refresh-ap.log
  /tmp/hwsim-11b-refresh-sta1.log

AP:
  nl80211-debug: start_ap parse chandef freq=2412
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-b' freq=2412 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-b' freq=2412 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA1 -> AP:
  ping -c 5 192.168.206.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.206.1 -p 5221 -t 6
    0.00-   6.02 sec     589824 Bytes    0.78 Mbits/sec
  ping -c 3 192.168.206.1
    3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.206.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.206.2 -p 5222 -t 6
    0.00-   6.02 sec     671744 Bytes    0.89 Mbits/sec
  ping -c 3 192.168.206.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1. This is a compat-layer limitation for attaching a packet
  filter in the simulator; the supplicant continued and association/data
  traffic passed.
- `genl_bridge: pre_doit failed family=nl80211 cmd=6 ret=-22` and
  `cmd=54 ret=-19` appeared during supplicant bring-up. These are unsupported
  optional nl80211 operations in the current shim path, not blockers for this
  open-auth association; no code change was needed for the 11b data proof.
- The run intentionally waited for `CTRL-EVENT-CONNECTED` before assigning IP
  and starting traffic, avoiding the early-ARP failures seen in earlier tests.

## 802.11g Current-Image Refresh

Pending validation items for this pass:

- 802.11g open-auth AP + STA baseline on channel 1.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- Legacy 2.4 GHz OFDM AP bring-up without HT/VHT/HE capability blocks.
- Association proof plus bidirectional ping and iperf.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-11g-refresh-ap.log
  /tmp/hwsim-11g-refresh-sta1.log

AP:
  nl80211-debug: start_ap parse chandef freq=2412
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0 vht_cap=0
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-g' freq=2412 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-g' freq=2412 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA1 -> AP:
  ping -c 5 192.168.207.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.207.1 -p 5231 -t 6
    0.00-   6.02 sec     851968 Bytes    1.13 Mbits/sec
  ping -c 3 192.168.207.1
    3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.207.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.207.2 -p 5232 -t 6
    0.00-   6.02 sec    1032192 Bytes    1.37 Mbits/sec
  ping -c 3 192.168.207.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1, matching the 11b baseline. This is a simulator compat
  limitation for attaching a packet filter; the supplicant continued and data
  traffic passed.
- `genl_bridge: pre_doit failed family=nl80211 cmd=6 ret=-22` and
  `cmd=54 ret=-19` appeared during supplicant bring-up, again matching the 11b
  baseline. These are unsupported optional nl80211 operations in the shim path,
  not blockers for the 11g open-auth association.
- Unlike the older 11g run, this current-image refresh did not log
  `truncating shared frame medium`; the configured 64 MiB medium default was
  sufficient for the short bidirectional proof.
- The run waited for `CTRL-EVENT-CONNECTED` before assigning IP and starting
  traffic.

## 802.11a Current-Image Refresh

Pending validation items for this pass:

- 802.11a open-auth AP + STA baseline on 5 GHz channel 36.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- Legacy 5 GHz OFDM AP bring-up without HT/VHT/HE capability blocks.
- Association proof plus bidirectional ping and iperf.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-11a-refresh-ap.log
  /tmp/hwsim-11a-refresh-sta1.log

AP:
  nl80211-debug: start_ap parse chandef freq=5180
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0 vht_cap=0
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-a' freq=5180 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-a' freq=5180 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA1 -> AP:
  ping -c 5 192.168.208.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.208.1 -p 5241 -t 6
    0.00-   6.02 sec     491520 Bytes    0.65 Mbits/sec
  ping -c 3 192.168.208.1
    3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.208.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.208.2 -p 5242 -t 6
    0.00-   6.02 sec    1146880 Bytes    1.52 Mbits/sec
  ping -c 3 192.168.208.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- `rfkill: Cannot get wiphy information` appeared on AP and STA1. This remains
  a non-blocking simulator compat gap; AP startup, STA association, and data
  traffic all passed.
- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1, matching the 11b/11g baseline. The supplicant continued
  and data traffic passed.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block the open-auth 5 GHz path.
- The STA->AP iperf server side closed later than the client-side six-second
  interval, but the client completed cleanly and the post-iperf ping still
  passed 3/3.
- This current-image refresh did not log `truncating shared frame medium`.

## 802.11n HT20 Current-Image Refresh

Pending validation items for this pass:

- 802.11n HT20 open-auth AP + STA baseline on 2.4 GHz channel 6.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- HT AP bring-up with `ht_cap` present and HE/VHT capability blocks absent.
- Association proof plus bidirectional ping and iperf.
- A-MPDU/ADDBA and A-MSDU instrumentation evidence.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-11n-ht20-refresh-ap.log
  /tmp/hwsim-11n-ht20-refresh-sta1.log

AP:
  nl80211-debug: start_ap parse chandef freq=2437
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0x... vht_cap=0
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-n' freq=2437 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-n' freq=2437 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

A-MPDU / ADDBA:
  STA1 side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
  AP side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=1

A-MSDU:
  amsdu-proof: probe skip no-head tid=0 len=98 max=1600 tin_packets=0 tin_bytes=0 head=0
  amsdu-proof: probe skip no-head tid=0 len=54 max=1600 tin_packets=0 tin_bytes=0 head=0

STA1 -> AP:
  ping -c 5 192.168.209.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.209.1 -p 5251 -t 6
    0.00-   6.02 sec    3178496 Bytes    4.22 Mbits/sec
  immediate post-iperf ping -c 3 192.168.209.1
    3 packets transmitted, 2 received, 33% packet loss
  follow-up ping -c 3 192.168.209.1
    3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.209.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.209.2 -p 5252 -t 6
    0.00-   6.02 sec     344064 Bytes    0.46 Mbits/sec
  ping -c 3 192.168.209.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- `rfkill: Cannot get wiphy information` appeared on AP and STA1, matching the
  11a baseline. This remains a non-blocking simulator compat gap.
- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1; the supplicant continued and associated.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block the HT20 AP/STA path.
- The immediate STA1->AP post-iperf ping lost the first of three packets. A
  follow-up 3-packet ping after the reverse iperf returned 3/3, so this is
  recorded as a transient post-iperf warm-up miss rather than a persistent
  data-plane failure.
- A-MSDU instrumentation only reached the probe path in this short traffic
  pattern (`probe skip no-head`); stronger A-MSDU aggregation proof still needs
  a targeted traffic shape.
- This current-image refresh did not log `truncating shared frame medium`.

## 802.11n HT40 Current-Image Refresh

Pending validation items for this pass:

- 802.11n HT40 open-auth AP + STA baseline on 2.4 GHz channel 6.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- `hostapd-n-ht40.conf` `HT40+` configuration, HT AP state transition, and
  non-null `ht_cap` proof.
- Association proof plus bidirectional ping and iperf.
- A-MPDU/ADDBA and A-MSDU instrumentation evidence.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-11n-ht40-refresh-ap.log
  /tmp/hwsim-11n-ht40-refresh-sta1.log

Config:
  hostapd-n-ht40.conf:
    ieee80211n=1
    ht_capab=[HT40+][SHORT-GI-20][SHORT-GI-40]

AP:
  wlan0: interface state UNINITIALIZED->HT_SCAN
  nl80211-debug: start_ap parse chandef freq=2437
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0x... vht_cap=0
  wlan0: interface state HT_SCAN->ENABLED
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-n40' freq=2437 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-n40' freq=2437 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

A-MPDU / ADDBA:
  STA1 side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
  AP side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=1

A-MSDU:
  amsdu-proof: probe skip no-head tid=0 len=98 max=1600 tin_packets=0 tin_bytes=0 head=0
  amsdu-proof: probe skip no-head tid=0 len=54 max=1600 tin_packets=0 tin_bytes=0 head=0

STA1 -> AP:
  ping -c 5 192.168.210.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.210.1 -p 5261 -t 6
    0.00-   6.02 sec    2981888 Bytes    3.96 Mbits/sec
  ping -c 3 192.168.210.1
    3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.210.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.210.2 -p 5262 -t 6
    0.00-   6.02 sec     442368 Bytes    0.59 Mbits/sec
  ping -c 3 192.168.210.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- Current logs prove `HT40+` configuration, HT AP state transition
  `HT_SCAN->ENABLED`, and non-null `ht_cap`, but they do not emit an explicit
  runtime bandwidth or secondary-channel field. Keep the older HT40 width proof
  as reference, and add targeted chandef logging if runtime width evidence must
  be machine-checkable from the current image.
- `rfkill: Cannot get wiphy information` appeared on AP and STA1, matching the
  11a/HT20 baselines. This remains a non-blocking simulator compat gap.
- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1; the supplicant continued and associated.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block the HT40 AP/STA path.
- The server-side iperf close happened later than the six-second client window
  in both directions, but both clients returned clean summaries and post-iperf
  ping passed 3/3.
- A-MSDU instrumentation only reached the probe path in this short traffic
  pattern (`probe skip no-head`); stronger A-MSDU aggregation proof still needs
  a targeted traffic shape.
- This current-image refresh did not log `truncating shared frame medium`.

## 802.11ac VHT20 Current-Image Refresh

Pending validation items for this pass:

- 802.11ac VHT20 open-auth AP + STA baseline on 5 GHz channel 36.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- VHT AP bring-up with `ht_cap` and `vht_cap` present and HE capability blocks
  absent.
- Association proof plus bidirectional ping and iperf.
- A-MPDU/ADDBA and A-MSDU instrumentation evidence.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-vht20-refresh-ap.log
  /tmp/hwsim-vht20-refresh-sta1.log

Config:
  hostapd-ac.conf:
    hw_mode=a
    channel=36
    ieee80211n=1
    ieee80211ac=1
    vht_oper_chwidth=0

AP:
  nl80211-debug: start_ap parse chandef freq=5180
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0x... vht_cap=0x...
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-ac' freq=5180 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-ac' freq=5180 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

A-MPDU / ADDBA:
  STA1 side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
  AP side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=1

A-MSDU:
  amsdu-proof: probe skip no-head tid=0 len=98 max=1600 tin_packets=0 tin_bytes=0 head=0
  amsdu-proof: probe skip no-head tid=0 len=54 max=1600 tin_packets=0 tin_bytes=0 head=0

STA1 -> AP:
  ping -c 5 192.168.211.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.211.1 -p 5271 -t 6
    0.00-   6.02 sec    1818624 Bytes    2.42 Mbits/sec
  ping -c 3 192.168.211.1
    3 packets transmitted, 3 received, 0% packet loss

AP -> STA1:
  ping -c 5 192.168.211.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.211.2 -p 5272 -t 6
    0.00-   6.02 sec    2129920 Bytes    2.83 Mbits/sec
  ping -c 3 192.168.211.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- `rfkill: Cannot get wiphy information` appeared on AP and STA1, matching the
  earlier open-auth baselines. This remains a non-blocking simulator compat
  gap.
- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1; the supplicant continued and associated.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block the VHT20 AP/STA path.
- The AP-side iperf server for STA1->AP closed later than the six-second
  client window, but the client returned a clean summary and post-iperf ping
  passed 3/3.
- During AP->STA1 server-side accounting, the hostfs shared medium hit the
  configured 64 MiB guard:
  `truncating shared frame medium size=67109400 limit=67108864`. Both iperf
  clients completed and the AP->STA1 post-iperf ping still passed 3/3, so this
  is recorded as medium-bound/throughput tuning rather than a functional VHT20
  failure.
- A-MSDU instrumentation only reached the probe path in this short traffic
  pattern (`probe skip no-head`); stronger A-MSDU aggregation proof still needs
  a targeted traffic shape.

## 802.11ac VHT80 Current-Image Refresh

Pending validation items for this pass:

- 802.11ac VHT80 open-auth AP + STA baseline on 5 GHz channel 36.
- AP and STA independent simulator objects: `nuttx-sim-ap` and
  `nuttx-sim-sta1`.
- Config-level VHT80 proof through `vht_oper_chwidth=1` and
  `vht_oper_centr_freq_seg0_idx=42`.
- AP bring-up with `ht_cap` and `vht_cap` present and HE capability blocks
  absent.
- Association proof plus bidirectional ping and iperf.
- A-MPDU/ADDBA and A-MSDU instrumentation evidence.
- Problem/fix notes for warnings that appear during the run.

Runtime evidence:

```text
Logs:
  /tmp/hwsim-vht80-refresh-ap.log
  /tmp/hwsim-vht80-refresh-sta1.log

Config:
  hostapd-ac-vht80.conf:
    hw_mode=a
    channel=36
    ieee80211n=1
    ht_capab=[HT40+][SHORT-GI-20][SHORT-GI-40]
    ieee80211ac=1
    vht_oper_chwidth=1
    vht_oper_centr_freq_seg0_idx=42
    vht_capab=[SHORT-GI-80]

AP:
  wlan0: interface state COUNTRY_UPDATE->HT_SCAN
  nl80211-debug: start_ap parse chandef freq=5180
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0x... vht_cap=0x...
  wlan0: interface state HT_SCAN->ENABLED
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA1:
  wlan0: SME: Trying to authenticate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-ac80' freq=5180 MHz)
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-ac80' freq=5180 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

A-MPDU / ADDBA:
  STA1 side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
    ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=1
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
  AP side:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
    ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=1
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=1

A-MSDU:
  amsdu-proof: probe skip no-head tid=0 len=98 max=1600 tin_packets=0 tin_bytes=0 head=0
  amsdu-proof: probe skip no-head tid=0 len=54 max=1600 tin_packets=0 tin_bytes=0 head=0

STA1 -> AP:
  ping -c 5 192.168.212.1
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.212.1 -p 5281 -t 6
    0.00-   6.02 sec    3178496 Bytes    4.22 Mbits/sec
  ping -c 3 192.168.212.1
    3 packets transmitted, 2 received, 33% packet loss

AP -> STA1:
  ping -c 5 192.168.212.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.212.2 -p 5282 -t 6
    0.00-   6.02 sec     655360 Bytes    0.87 Mbits/sec
  ping -c 3 192.168.212.2
    3 packets transmitted, 3 received, 0% packet loss
```

Problems and resolution notes:

- Current logs prove the VHT80 configuration and non-null VHT capability block,
  but this run does not emit an explicit runtime channel-width line. Keep the
  older VHT80 IE proof as the stronger VHT Operation IE evidence.
- `rfkill: Cannot get wiphy information` appeared on AP and STA1, matching the
  earlier AP/STA baselines. This remains a non-blocking simulator compat gap.
- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on STA1; the supplicant continued and associated.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block the VHT80 AP/STA path.
- The immediate STA1->AP post-iperf ping lost the first of three packets after
  the client summary; AP->STA1 post-iperf ping then passed 3/3. Treat this as
  the same first-ARP/post-load warm-up behavior seen in earlier refresh runs.
- During the AP->STA1 reverse server-side accounting, the hostfs shared medium
  hit the configured 64 MiB guard:
  `truncating shared frame medium size=67109400 limit=67108864`. The AP->STA1
  client completed, the STA1 server closed normally, and post-iperf ping passed
  3/3, so this is recorded as medium-bound/throughput tuning rather than a
  functional VHT80 failure.
- A-MSDU instrumentation only reached the probe path in this short traffic
  pattern (`probe skip no-head`); stronger A-MSDU aggregation proof still needs
  a targeted traffic shape.
- This refresh covers AP<->STA1 only. A separate VHT80 AP+STA1+STA2 replay
  below covers STA-to-STA ping and iperf.

## 802.11ac VHT80 STA-to-STA Current-Image Replay

Pending validation items for this pass:

- AP + STA1 + STA2 independent simulator objects: `nuttx-sim-ap`,
  `nuttx-sim-sta1`, and refreshed `nuttx-sim-sta2`.
- VHT80 AP using `hostapd-ac-vht80.conf` and two STAs using
  `wpa-ac-vht80.conf`.
- Both STAs associated to the same AP.
- STA1->STA2 and STA2->STA1 ping and TCP iperf.
- A-MPDU/ADDBA and A-MSDU instrumentation evidence.
- Problem/fix notes for warnings and hostfs medium behavior.

Build evidence:

```text
Command:
  FeatherCore/tools/firmware/sim/build-sta2.sh -j8

Output:
  ../build/nuttx-sim-sta2
  size: 27714792 bytes
  log: /tmp/hwsim-build-sta2-vht80-replay.log
```

Runtime evidence:

```text
Logs:
  /tmp/hwsim-vht80-sta2sta-ap.log
  /tmp/hwsim-vht80-sta2sta-sta1.log
  /tmp/hwsim-vht80-sta2sta-sta2.log

AP:
  wlan0: interface state COUNTRY_UPDATE->HT_SCAN
  nl80211-debug: start_ap parse chandef freq=5180
  nl80211-debug: start_ap rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0x... vht_cap=0x...
  wlan0: interface state HT_SCAN->ENABLED
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:03

STA1:
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-ac80' freq=5180 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA2:
  wlan0: Trying to associate with 02:00:00:00:00:01 (SSID='nuttx-hwsim-ac80' freq=5180 MHz)
  wlan0: Associated with 02:00:00:00:00:01
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed

STA1 -> STA2:
  ping -c 5 192.168.213.3
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.213.3 -p 5291 -t 6
    0.00-   6.02 sec    1097728 Bytes    1.46 Mbits/sec
  STA2 server:
    accept: 192.168.213.2:26062
    closed by the peer: 192.168.213.2:26062
    iperf exit
  ping -c 3 192.168.213.3
    3 packets transmitted, 2 received, 33% packet loss

STA2 -> STA1:
  ping -c 5 192.168.213.2
    5 packets transmitted, 5 received, 0% packet loss
  iperf -c 192.168.213.2 -p 5292 -t 6
    0.00-   6.02 sec    2818048 Bytes    3.74 Mbits/sec
  STA1 server:
    accept: 192.168.213.3:26062
    closed by the peer: 192.168.213.3:26062
    iperf exit
  ping -c 3 192.168.213.2
    3 packets transmitted, 3 received, 0% packet loss

A-MPDU / ADDBA:
  STA1:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
    ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
    ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=1
  STA2:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=1
    ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
    ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=1
  AP:
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:02 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:02 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:02 tid=0 buf=64 amsdu=1
    ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:03 tid=0 token=1 ssn=1 buf=64 timeout=0
    ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:03 tid=0 token=1 status=0 raw_buf=64 amsdu=1 timeout=0
    ampdu-proof: tx operational sta=02:00:00:00:00:03 tid=0 buf=64 amsdu=1

A-MSDU:
  amsdu-proof: probe skip no-head tid=0 len=590 max=1 tin_packets=0 tin_bytes=0 head=0
```

Problems and resolution notes:

- STA1->STA2 first ping printed two ARP wait timeouts while the path warmed up,
  but the final five-packet summary still passed 5/5.
- STA1->STA2 post-iperf ping lost one of three packets immediately after the
  client summary. The reverse post-iperf ping passed 3/3.
- `l2_packet_linux: setsockopt(SO_ATTACH_FILTER) failed: Unknown error 92`
  appeared on both STAs; both supplicants continued and associated.
- Optional nl80211 operations returned `-22`, `-19`, `-67`, or `-95` during
  setup/scanning. These did not block AP bring-up, association, ping, or iperf.
- The AP-side shared-medium file hit the configured 64 MiB guard during the
  traffic run:
  `truncating shared frame medium size=67109400 limit=67108864`. Both iperf
  directions completed and the reverse post-iperf ping passed 3/3, so this
  remains a medium-bound/throughput tuning item rather than a functional
  STA-to-STA failure.
- A-MSDU instrumentation again only reached the probe path in this short
  traffic pattern; stronger A-MSDU aggregation proof still needs a targeted
  traffic shape.

## Next Steps

1. Extend the AP + STA1 + STA2 stress coverage from the passing 20-packet
   bidirectional ping plus 20-second sequential iperf smoke to repeated
   association, repeated ping, longer iperf, and scripted replay.
2. Continue tuning simultaneous bidirectional STA-to-STA throughput and iperf
   close behavior over the bounded hostfs-backed hwsim medium.
3. Extend P2P GO/client validation with repeated long runs and throughput
   tuning now that sequential and simultaneous bidirectional ping/iperf pass.
4. Extend DPP beyond the passing PSK provisioning path with repeated long-run
   tests, negative cases, and CSR/PKCS#7 certificate-mode support once the real
   crypto backend APIs exist.
5. Extend FILS beyond the passing first-association, STA2-present reconnect,
   and three-cycle cached reconnect proof with longer soak loops and throughput
   stress tuning.
6. Add dedicated tests or instrumentation for feature inventory items:
   tighter automatic TWT service-period timing/queue release and AP-originated
   ARP behavior,
   real WNM TFS classifier/filter policy behavior beyond the proven non-empty
   IE request/response plumbing,
   repeated/negative WPS cases, and multi-BSS 802.11r roaming.
7. Tune dynamic auto-PS AP-to-STA TCP service periods so the AP sleep queue no
   longer saturates at 64 frames after the first burst.
8. Replace high-risk compat stubs with real NuttX-backed implementations where
   runtime behavior depends on Linux semantics.

## 2026-06-10 Current-Tree hwsim AP/STA Revalidation

Scope:

- Rebuilt `nuttx-sim-ap` with `tools/firmware/sim/build-ap.sh -j8`.
- Rebuilt `nuttx-sim-sta1` with `tools/firmware/sim/build-sta1.sh -j8`.
- Revalidated full `hostapd` + `wpa_supplicant` path with open auth and
  WPA2-PSK/CCMP.
- Used non-PTY subprocess logs for runtime validation because debug-heavy hwsim
  output can fill a PTY pipe and make the AP process look protocol-stuck.

Root cause fixed in this pass:

- WPA2 previously associated but failed the 4-way handshake and disconnected
  with reason 15.
- AP logs showed EAPOL Ethernet frames with EtherType `0x888e`, but mac80211
  saw `skb->protocol=0x8e88` while `sdata->control_port_protocol` was
  `0x888e`.
- That mismatch came from NuttX compatibility code treating Linux `__be16`
  values as identity host-order integers on a little-endian host, plus the
  lower-to-Linux TX bridge storing a parsed Ethernet protocol directly into
  `skb->protocol`.
- Fix:
  - `cfg80211_compat.h` now maps `htons()`, `ntohs()`, `htonl()`, `ntohl()`,
    `cpu_to_be16()`, `be16_to_cpu()`, `cpu_to_be32()`, and `be32_to_cpu()` to
    real endian conversions.
  - `netdevice_compat.c` now stores parsed Ethernet protocol values in
    `skb->protocol` with `cpu_to_be16()`, matching Linux mac80211 expectations.
  - The wpa/hostapd Ethernet header writers use explicit big-endian writes for
    EAPOL/preauth/OUI EtherTypes.

Validation evidence:

```text
WPA2-PSK/CCMP:
  logs:
    /tmp/hwsim-full-current-wpa2-protofix-ap.log
    /tmp/hwsim-full-current-wpa2-protofix-sta.log

  STA:
    wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
    ping -c 5 192.168.90.1
      5 packets transmitted, 5 received, 0% packet loss
    STA -> AP iperf:
      0.00-   6.02 sec     704512 Bytes    0.94 Mbits/sec

  AP:
    ping -c 5 192.168.90.2
      5 packets transmitted, 5 received, 0% packet loss
    AP -> STA iperf:
      0.00-   6.02 sec    2801664 Bytes    3.72 Mbits/sec
```

```text
Open auth:
  logs:
    /tmp/hwsim-full-current-open-protofix-ap.log
    /tmp/hwsim-full-current-open-protofix-sta.log

  STA:
    wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
    ping -c 5 192.168.91.1
      5 packets transmitted, 5 received, 0% packet loss
    STA -> AP iperf:
      0.00-   6.02 sec    2015232 Bytes    2.68 Mbits/sec

  AP:
    ping -c 5 192.168.91.2
      5 packets transmitted, 5 received, 0% packet loss
    AP -> STA iperf:
      0.00-   6.02 sec    2785280 Bytes    3.70 Mbits/sec
```

Remaining follow-up:

- Re-run the broader security matrix after the endian fix, especially PMF,
  OCV, WPA3/SAE, OWE, FILS, FT, P2P, and Enterprise.
- Continue reducing non-functional compile warnings from the imported Linux
  wireless sources without changing protocol behavior.
- Keep validation commands short when using NSH, or mount short hostfs aliases,
  because long command lines can wrap/truncate in interactive terminal tests.
