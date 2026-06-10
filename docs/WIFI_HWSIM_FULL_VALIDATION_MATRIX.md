# Wi-Fi hwsim full validation matrix

## Goal

Validate the FeatherCore hwsim path against the Linux hwsim-style stack:

```text
hostapd-2.11 / wpa_supplicant-2.11
  -> libnl
  -> nl80211
  -> cfg80211
  -> mac80211
  -> mac80211_hwsim
```

The current validated baseline is:

- open authentication with AP + STA1 + STA2 in three separate `nuttx-sim-*`
  processes, including STA-to-AP and STA-to-STA ping/iperf traffic.
- WPA2-Personal PSK/CCMP with AP + STA1 + STA2, including 4-way handshake,
  PTK/GTK install, STA-to-AP ping/iperf, and sequential STA-to-STA ping/iperf
  in both directions.
- WPA2-Personal TKIP and mixed TKIP/CCMP with AP + STA1, including cipher
  negotiation, 4-way handshake, PTK/GTK install, ping, and iperf.
- WPA2 PMF optional and PMF required with AES-128-CMAC management group cipher,
  IGTK install, ping, and iperf traffic.
- FILS-SHA256 AP + STA1 first association and same-context fast reconnect,
  including ERP/PMKSA cache seeding, cached FILS Auth+Assoc, AP-side AES-SIV
  decrypt of the encrypted Association Request, `FILS_HLP_SENT`, and
  post-reconnect ping.
- WPA3-Personal SAE AP + STA1 + STA2 runtime, including SAE commit/confirm,
  4-way handshake, PTK/GTK/IGTK install, STA-to-AP traffic, and sequential
  STA-to-STA ping/iperf in both directions.
- WPA3-SAE H2E with AP + STA1, including PT derivation, RSNXE propagation,
  4-way handshake, IGTK install, ping, and iperf.
- WPA3 OWE with AP + STA1, including OWE Diffie-Hellman key generation,
  PMKID agreement, 4-way handshake, PMF-required association, bidirectional
  ping, and bidirectional iperf.
- WPA2/WPA3 transition mode with the same AP accepting both an SAE STA and a
  legacy WPA-PSK STA, including handshake, key install, ping, and iperf.
- P2P discovery and P2P GO/client group formation, including `p2p-wlan0-0`
  static IP setup, bidirectional ping, and bidirectional iperf over the P2P
  group interface.
- WPS PBC over WPA2-PSK with AP + STA1, including registrar/enrollee exchange,
  credential provisioning, WPA2 4-way handshake, ping, and bidirectional iperf.
- WNM BSS Transition Management over WPA2-PSK with AP + STA1, including
  AP-originated BSS TM Request, STA BSS TM Response, and bidirectional data
  traffic after the WNM action exchange.
- WNM Sleep Mode over WPA2-PSK with AP + STA1, including enter/exit action
  frames, AP responses, GTK group rekey after exit, post-sleep ping, and
  bidirectional iperf.
- WPA2-Enterprise EAP-TLS, PEAP/MSCHAPv2, and TTLS with PAP/CHAP/MSCHAP/
  MSCHAPv2 inner methods using AP + STA1 and hostapd's integrated EAP server,
  including certificate exchange, tunneled inner authentication, EAP success,
  RSN 4-way handshake, bidirectional ping, and bidirectional iperf.
- HS20/interworking ANQP/GAS over WPA2-Enterprise, including HS20 indication,
  standard ANQP venue/IP/NAI/domain queries, HS20 operator/WAN metrics/
  connection capability queries, EAP success, RSN 4-way handshake, ping, and
  STA-to-AP iperf.
- 802.11r FT-PSK over WPA2-PSK/CCMP, including single-BSS FT association and
  multi-BSS FT roaming. The STA completes FT authentication/reassociation to
  the target BSS and data traffic passes after the target BSS interface is
  configured in the NuttX sim IP stack. AP-originated post-roam traffic must
  use the target-BSS source address in the current no-bridge simulator path.

## 2026-06-10 Current-Tree PASS Rerun

All previously recorded `PASS` hwsim/mac80211 items were replayed against the
current `FeatherCore_ESP` tree on 2026-06-10.

Standard AP + STA1 cases were replayed with
`tools/firmware/sim/validate-hwsim-pass-matrix.py`. The primary run produced
39/41 PASS and exposed two runner mapping errors for OWE groups 20 and 21. The
OWE runner mapping was corrected to use the existing `wpa-owe-g20.conf` and
`wpa-owe-g21.conf` STA configs, and both OWE groups then passed. The standalone
FT-PSK association case was also added to the runner and passed.

Effective current result for previously recorded `PASS` items: no remaining
current `FAIL`. The only failed rows in the first replay were OWE group 20/21
runner-config mistakes, and both have clean PASS reruns.

Current evidence:

- Standard AP + STA1 matrix: `/tmp/hwsim-pass-matrix-20260610-r2/summary.md`
- OWE group 20/21 rerun: `/tmp/hwsim-pass-owe-rerun/summary.md`
- FT-PSK single-BSS rerun: `/tmp/hwsim-pass-ft-rerun/summary.md`
- Runner smoke after iperf result matching fix:
  `/tmp/hwsim-pass-smoke-r2/summary.md`
- WPA3 SAE smoke after aligning NuttX `AF_PACKET` `SOCK_DGRAM`/`SOCK_RAW`
  behavior with Linux:
  `/tmp/hwsim-wpa3-sae-basic-20260610-072141/summary.md`. This run passed
  AP enable, SAE commit/confirm, RSN key negotiation, STA-to-AP ping,
  AP-to-STA ping, and bidirectional iperf.

Special multi-role and control-path flows were replayed separately because they
need more than one STA, P2P roles, multiple BSSs, or explicit action-frame
steps. Current PASS evidence:

| Flow | Result | Evidence |
| --- | --- | --- |
| Open AP + STA1 + STA2 + STA3 | PASS | `/tmp/hwsim-current-open-3sta/` |
| P2P GO/client 60-second bidirectional soak | PASS | `/tmp/hwsim-current-p2p-soak60-r2/` |
| WPS PBC provisioning | PASS | `/tmp/hwsim-current-wps-pbc/` |
| DPP-PSK provisioning | PASS | `/tmp/hwsim-current-dpp-psk/` |
| FT multi-BSS roam | PASS | `/tmp/hwsim-current-ft-roam/` |
| FILS cached reconnect with STA2 online | PASS | `/tmp/hwsim-current-fils-reconnect/` |
| A-MPDU ADDBA + A-MSDU aggregation | PASS | `/tmp/hwsim-current-aggregation/` |
| TWT automatic service-period data smoke | PASS | `/tmp/hwsim-current-twt-auto/` |
| Static power-save | PASS | `/tmp/hwsim-current-static-ps/` |
| Dynamic PS / MLO-style PS hooks | PASS | `/tmp/hwsim-current-dynps/` |
| WNM BSS Transition + Sleep/TFS | PASS | `/tmp/hwsim-current-wnm/` |
| HE DCM RX-status proof | PASS | `/tmp/hwsim-current-dcm/` |

Notes from the rerun:

- The matrix runner no longer treats a missing literal `iperf exit` string as
  failure. It now keys the data-plane proof on ping success and iperf summary
  lines, with a short grace period for server-side shutdown logs.
- The first P2P automation attempt issued `p2p_connect` before peer discovery
  completed; the corrected replay waits for `P2P-DEVICE-FOUND` first.
- The aggregation one-off checker had a strict string match for
  `rx ADDBA_RESP status=0`; the actual log includes token and dialog fields
  around `status=0`. The raw logs prove ADDBA request/response, TX operational
  state, A-MSDU aggregation, ping, and iperf.
- Manual TWT service-period gating still has an AP-originated ARP timing
  caveat if traffic is started inside a non-warmed manual service period. The
  automatic TWT setup/period/teardown flow passes bidirectional ping and iperf.

## Capability Inventory

Current source and build facts as of 2026-06-10:

- `apps/wireless/wifi/wpa_hostapd_sources.mk` enables:
  - `CONFIG_DRIVER_NL80211`
  - `CONFIG_LIBNL3_ROUTE`
  - `CONFIG_CRYPTO_INTERNAL`
  - `CONFIG_TLS_INTERNAL`
  - `CONFIG_IEEE80211R`
  - `CONFIG_IEEE80211W`
  - `CONFIG_IEEE80211AC`
  - `CONFIG_SAE` under `CONFIG_ARCH_SIM`
  - `CONFIG_OWE` under `CONFIG_ARCH_SIM`
  - `CONFIG_DPP`, `CONFIG_DPP2`, and `CONFIG_GAS_SERVER` under
    `CONFIG_ARCH_SIM`
  - `CONFIG_WNM`
  - `CONFIG_WNM_AP`
  - `CONFIG_P2P`
  - `CONFIG_SUITEB`
  - `CONFIG_SUITEB192`
  - `CONFIG_AP`
  - `CONFIG_WPS`
  - `CONFIG_HS20`
  - `CONFIG_INTERWORKING`
  - `CONFIG_SME`
  - many EAP peer/server methods
- Current generated build flags now also enable:
  - `CONFIG_OCV`
  - `CONFIG_FILS`
- The simulator build includes a simulator-only OpenSSL-backed
  `crypto_bignum_*` / `crypto_ec_*` glue layer for SAE/Dragonfly while the rest
  of the WPA build continues to use the internal crypto/TLS source list.
- Upstream defconfigs contain more security options. The active generated
  NuttX source list now includes OWE, Suite-B/Suite-B-192, DPP, OCV, and FILS.
  OWE, OCV, and FILS now all have runtime association plus data-plane proof;
  FILS also has same-context cached reconnect and STA2-present reconnect/data
  stress coverage.
- `mac80211_hwsim_linux.c` contains 2.4 GHz, 5 GHz, 6 GHz, and S1G channel
  tables.
- In NuttX sim, `mac80211_hwsim_linux.c` now defaults `regtest` to
  `HWSIM_REGTEST_CUSTOM_WORLD` under `CONFIG_ARCH_SIM`, so the imported hwsim
  path can start AP validation on non-DFS 5 GHz channels without relying on
  Linux module parameters or a userspace regulatory database path.
- `mac80211_hwsim_linux.c` advertises HT/VHT/HE capability blocks and P2P
  interface types in source.
- `mac80211_hwsim_linux.c` has AMPDU action support and sets
  `IEEE80211_HW_AMPDU_AGGREGATION`.
- S1G capability advertises TWT request/respond bits. HE TWT behavior now has
  command-level/runtime validation for setup, AP accept response, STA RX/parse,
  teardown, manual service-period gating, and automatic service-period data
  smoke.
- The Linux wireless compatibility endian helpers now preserve Linux `__be16`
  semantics on little-endian NuttX hosts. `htons()`, `cpu_to_be16()`, and
  related macros map through `htobe16()`/`be16toh()` instead of identity
  conversions, and the NuttX lower-to-Linux TX bridge stores parsed Ethernet
  protocol values in `skb->protocol` with `cpu_to_be16()`. This is required for
  control-port/EAPOL comparisons such as `ETH_P_PAE` to match the imported
  Linux mac80211 behavior.

Important distinction:

Source-level capability structures are not enough. A feature is marked
`PASS` only after hostapd/wpa_supplicant complete the relevant negotiation and
data traffic still passes. If the feature is merely compiled or advertised, it
is marked `INVENTORY` or `TODO`.

Status key:

- `PASS`: runtime negotiation and data traffic evidence exists.
- `PARTIAL`: the feature reaches a meaningful runtime milestone, but traffic,
  stress, or a required sub-mode is not reliable enough yet.
- `INVENTORY`: source/capability evidence exists only.
- `TODO`: not yet exercised.
- `BLOCKED`: a missing build option, userspace feature, or kernel/driver path
  prevents a valid runtime test.

## Goal Completion Audit

This audit keeps the requested full-scope goal separate from the subset that is
already proven. A row is complete only when the current tree has role objects,
repeatable recipes, runtime logs, and data-plane evidence for the relevant
scope.

| Objective area | Current proof | Remaining gap |
| --- | --- | --- |
| Role objects for changed simulated roles | Dedicated build wrappers and defconfigs exist for `ap`, `sta1`, `sta2`, `sta3`, `ap1`, `ap2`, `p2p1`, `p2p2`, `dynps_ap`, and `dynps_sta1`. The additional wrapper scripts pass shell syntax checks, and the manual recipe requires one `nuttx-sim-*` process per role. Latest open-auth AP + STA1 + STA2 + STA3 proof uses four independent `nuttx-sim-*` processes, logs three AP `AP-STA-CONNECTED` events, and passes STA3/AP plus STA1/STA2/STA3 data-plane ping and STA1<->STA3 iperf. The 2026-06-06 AP1/AP2 refresh rebuilt `nuttx-sim-ap1` and `nuttx-sim-ap2` with matching 23353584-byte images, generated both map files, and boot-smoked both images to `NuttShell (NSH)` with `poweroff` returning 0. The same refresh rebuilt and boot-smoked `nuttx-sim-sta3`, `nuttx-sim-p2p1`, and `nuttx-sim-p2p2` with matching 27714792-byte images and generated all three map files. The MLO/dynamic-PS-specific `nuttx-sim-dynps-ap` and `nuttx-sim-dynps-sta1` images are also rebuilt and boot-smoked. | Builds must remain sequential because all roles share one generated NuttX `.config` and include state. More scripted replay would reduce manual drift, but the current policy is intentionally terminal-driven. Evidence: `/tmp/hwsim-open-3sta-ap.log`, `/tmp/hwsim-open-3sta-sta1.log`, `/tmp/hwsim-open-3sta-sta2.log`, `/tmp/hwsim-open-3sta-sta3.log`, `/tmp/hwsim-build-role-ap1-refresh.log`, `/tmp/hwsim-build-role-ap2-refresh.log`, `/tmp/hwsim-build-role-sta3-refresh.log`, `/tmp/hwsim-build-role-p2p1-refresh.log`, `/tmp/hwsim-build-role-p2p2-refresh.log`, `/tmp/hwsim-boot-role-ap1-refresh.log`, `/tmp/hwsim-boot-role-ap2-refresh.log`, `/tmp/hwsim-boot-role-sta3-refresh.log`, `/tmp/hwsim-boot-role-p2p1-refresh.log`, `/tmp/hwsim-boot-role-p2p2-refresh.log`, `/tmp/hwsim-build-dynps-ap-refresh.log`, `/tmp/hwsim-build-dynps-sta1-refresh.log`, `/tmp/hwsim-boot-dynps-ap-refresh.log`, and `/tmp/hwsim-boot-dynps-sta1-refresh.log`. |
| Encryption through WPA3 | Runtime PASS exists for open auth, WPA2 PSK variants, PMF optional/required, WPA2 PMF + OCV, FILS-SHA256 first association plus same-context fast reconnect/HLP-sent proof, STA2-present FILS reconnect, WPA3 SAE, SAE H2E, SAE-PK, WPA2/WPA3 transition, OWE groups 19/20/21, WPS provisioning, WPA2 Enterprise EAP methods, Suite-B/192-bit AKM/ciphers, HS20, WNM, FT, and DPP-PSK provisioning. | DPP CSR/PKCS#7 certificate mode is blocked on real crypto backend helpers. External RADIUS is not yet covered. Suite-B/192-bit is proven for Wi-Fi AKM/ciphers and the current simulator TLS profile, but not for a strict CNSA ECDHE/ECDSA SHA384/GCM TLS policy. FILS now has a three-cycle cached reconnect proof with STA2 online; longer soak and throughput tuning remain follow-up stress work. |
| 802.11 versions and bands | Runtime PASS exists for 802.11b, 11g, 11a, 11n HT20/HT40, 11ac VHT20/VHT80/VHT160/VHT80+80, 11ax HE on 2.4 GHz, 5 GHz, and secured 6 GHz, plus S1G/802.11ah simulator control/data path. | VHT160 and VHT80+80 throughput remain low under debug-heavy hostfs hwsim runs. S1G proves simulator compatibility and data path, not real S1G PHY modulation. |
| P2P capability | Dedicated `p2p1` and `p2p2` images form a GO/client group on `p2p-wlan0-0`, configure static IPs, and pass bidirectional ping/iperf. Latest 60-second independent-object P2P soak: GO/client negotiation succeeds on 2437 MHz, GO sees `AP-STA-CONNECTED` and `EAPOL-4WAY-HS-COMPLETED`, client completes `PTK=CCMP GTK=CCMP`, baseline ping is 5/5 in both directions, GO->client iperf runs 60.20 s at ~1.34 Mbits/sec with normal `iperf exit`, client->GO iperf runs 60.20 s at ~1.32 Mbits/sec with normal `iperf exit`, and post-iperf ping remains 3/3 both ways. Baseline STA1/STA2 P2P discovery and simultaneous P2P TCP stress also have evidence. After the 2026-06-06 `p2p1`/`p2p2` rebuild refresh, a short runtime smoke again forms GO/client, passes GO->client ping 5/5, GO->client iperf ~0.63 Mbits/sec, client->GO ping 5/5, client->GO iperf ~4.31 Mbits/sec, and an immediate second post-iperf client->GO ping returns to 3/3 after one transient first post-ping loss. | Longer repeated soak and throughput tuning remain follow-up work. Evidence: `/tmp/hwsim-p2p-soak60-p2p1.log`, `/tmp/hwsim-p2p-soak60-p2p2.log`, `/tmp/hwsim-p2p-refresh-p2p1.log`, and `/tmp/hwsim-p2p-refresh-p2p2.log`. |
| DCM, aggregation, TWT, and PS modes | DCM has sim runtime RX-status proof. A-MPDU ADDBA and A-MSDU software aggregation are runtime PASS; the 60-second P2P soak also proves ADDBA request/response plus `tx operational` on both peers over `p2p-wlan0-0`. Static PS, queue stop/wake, NullFunc/PS-Poll, AP buffered delivery, Dynamic PS idle timeout, auto-PS polling, AP-link-PS, and MLO/dynps data smoke are proven. TWT setup/teardown, manual service-period gating, and automatic service-period smoke with ping/iperf are proven. The hwsim shared medium limit is now configurable through `CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES`, defaulting to 64 MiB; AP<->STA 20-second iperf smoke passed after this change. | DCM is a simulator RX-status/capability proof, not real modulation. TWT automatic timing, AP-originated ARP windowing, sleep-buffer release, configured hostfs medium bound under very long debug-heavy runs, and throughput need tuning. Dynamic auto-PS TCP is functional but below non-PS throughput. Real WNM TFS classifier/filter enforcement is not implemented beyond non-empty IE request/response plumbing. |
| Minimal demo path | Full `hostapd` + `wpa_supplicant` path passes AP/STA and STA-to-STA data. The lightweight WEXT bridge now forwards virtual hwsim wireless ioctls into the imported cfg80211 WEXT handlers, and `SIOCSIWCOMMIT` no longer dies in the NuttX upper-half dispatcher. | The minimal `wifi_ap_demo` + `wifi_sta_demo` path is still not a valid AP/STA proof. A 2026-06-06 rebuild-and-rerun reached the real AP setup failure earlier: `wifi_ap_demo` returns `SIOCSIWMODE(wlan0) failed: -22` because Linux cfg80211 WEXT compat rejects `IW_MODE_MASTER`; AP creation needs nl80211 `START_AP`/hostapd or a simulator-specific AP helper that builds cfg80211 AP settings. Evidence: `/tmp/hwsim-demo-wext-ap.log`. |

## Security Validation Matrix

| Area | Test | Current status | Evidence / next action |
| --- | --- | --- | --- |
| Open auth | `wpa=0`, `key_mgmt=NONE` | PASS | AP + STA1 + STA2 association, STA-to-AP ping, and sequential STA1<->STA2 ping/iperf verified on the full `hostapd` + `wpa_supplicant` path. Latest 2026-06-10 current-tree AP+STA1 rerun after the Linux `__be16` protocol fix: AP reached `AP-ENABLED`, STA reached `CTRL-EVENT-CONNECTED`, STA->AP ping 5/5 with 0% loss, AP->STA ping 5/5 with 0% loss, STA->AP iperf completed over 6.02 s at ~2.68 Mbits/sec, and AP->STA iperf completed over 6.02 s at ~3.70 Mbits/sec. Evidence: `/tmp/hwsim-full-current-open-protofix-ap.log` and `/tmp/hwsim-full-current-open-protofix-sta.log`. Earlier current-session AP+STA1+STA2 rerun with `nuttx-sim-ap`, `nuttx-sim-sta1`, and `nuttx-sim-sta2`: AP reached `AP-ENABLED`, AP logged both `AP-STA-CONNECTED` events, both STAs reached `CTRL-EVENT-CONNECTED`, STA1->STA2 ping 5/5 with 0% loss, STA1->STA2 iperf 12.04 s at ~0.58 Mbits/sec, STA2->STA1 ping 5/5 with 0% loss, and STA2->STA1 iperf 12.04 s at ~1.18 Mbits/sec. Evidence: `/tmp/hwsim-sta2sta-ap.log`, `/tmp/hwsim-sta2sta-sta1.log`, and `/tmp/hwsim-sta2sta-sta2.log`. Earlier longer sequential open two-STA proofs reached 20/20 ping and 20-second iperf in both directions, and older no-debug sequential proof reached ~8.97 Mbits/sec. After bounding the hwsim medium backlog, simultaneous bidirectional STA-to-STA iperf is functional: STA1->STA2 ~0.67 Mbits/sec, STA2->STA1 ~1.00 Mbits/sec, and post-stress ping remains 3/3 in both directions. A later AP+STA1 rerun after raising the configurable shared medium default to 64 MiB passed AP->STA1 ping 3/3 and 20-second iperf at ~1.24 Mbits/sec plus STA1->AP ping 3/3 and 20-second iperf at ~0.91 Mbits/sec; logs are `/tmp/hwsim-medium64-ap.log` and `/tmp/hwsim-medium64-sta1.log`. The minimal `wifi_ap_demo` + `wifi_sta_demo` path is tracked separately; after the WEXT bridge fix it now fails at AP mode setup (`SIOCSIWMODE(IW_MODE_MASTER)` -> `-EINVAL`), which confirms the full-stack STA-to-STA result should not be judged through that WEXT demo. Throughput tuning, configured hostfs medium bounds under longer runs, first-ARP warm-up misses, and occasional iperf server close behavior remain open. |
| Open auth multi-STA | AP + STA1 + STA2 + STA3, four independent `nuttx-sim-*` processes | PASS | AP logs `AP-STA-CONNECTED` for STA1/STA2/STA3 (`02:00:00:00:00:02/03/04`), all three STAs reach `CTRL-EVENT-CONNECTED`, STA3->AP ping passes 3/3, STA1->STA3 ping passes 3/3, STA2->STA3 ping passes 3/3, and STA3->STA1 ping passes 3/3. STA1->STA3 iperf runs 12.04 s at ~0.72 Mbits/sec, and STA3->STA1 iperf runs 12.04 s at ~1.52 Mbits/sec. Evidence: `/tmp/hwsim-open-3sta-ap.log`, `/tmp/hwsim-open-3sta-sta1.log`, `/tmp/hwsim-open-3sta-sta2.log`, and `/tmp/hwsim-open-3sta-sta3.log`. First ARP warm-up misses can appear before the 0%-loss ping summaries. |
| WPA2-Personal CCMP | `WPA-PSK`, `rsn_pairwise=CCMP` | PASS | Latest 2026-06-10 current-tree AP+STA1 rerun passes after restoring Linux `__be16` protocol semantics in the NuttX compatibility/TX bridge path. AP reached `AP-ENABLED`, STA reached `CTRL-EVENT-CONNECTED`, STA->AP ping 5/5 with 0% loss, AP->STA ping 5/5 with 0% loss, STA->AP iperf completed over 6.02 s at ~0.94 Mbits/sec, and AP->STA iperf completed over 6.02 s at ~3.72 Mbits/sec. Evidence: `/tmp/hwsim-full-current-wpa2-protofix-ap.log` and `/tmp/hwsim-full-current-wpa2-protofix-sta.log`. Root cause of the preceding failure was an endian mismatch: AP EAPOL frames carried EtherType `0x888e`, but the lower-to-mac80211 bridge stored `skb->protocol` as `0x8e88`, so mac80211 control-port checks rejected the frame and STA disconnected with reason 15. Older AP + STA1 + STA2 proof also completes EAPOL 4-way handshake, installs `PTK=CCMP GTK=CCMP`, pings AP with 0% loss, runs STA-to-AP iperf at ~21.21 Mbits/sec, and passes sequential STA1<->STA2 ping/iperf. After bounding the hwsim medium backlog, simultaneous bidirectional STA-to-STA stress is functional; throughput tuning remains open. |
| WPA2-Personal TKIP | `WPA-PSK`, `rsn_pairwise=TKIP` | PASS | AP + STA1 select `PTK=TKIP GTK=TKIP`, complete the 4-way handshake, ping with 0% loss, and run iperf at ~21.02 Mbits/sec. |
| WPA2 mixed TKIP/CCMP | `wpa_pairwise=TKIP CCMP`, `rsn_pairwise=CCMP` | PASS | AP + STA1 negotiate RSN with `PTK=CCMP GTK=TKIP`, complete the 4-way handshake, ping with 0% loss, and run iperf at ~17.75 Mbits/sec. |
| WPA2 PMF optional | `ieee80211w=1` | PASS | AP + STA1 complete PMF-optional association, select MFP, install IGTK, ping with 0% loss, and run iperf at ~18.08 Mbits/sec. |
| WPA2 PMF required | `ieee80211w=2`, SHA256 AKM | PASS | AP + STA1 complete PMF-required association with `WPA-PSK-SHA256`, AES-128-CMAC MGMT group cipher, IGTK install, ping, and iperf. |
| WPA2 PMF + OCV | `ocv=1`, `ieee80211w=2`, SHA256 AKM | PASS | `CONFIG_OCV` is enabled and `src/common/ocv.c` is linked into AP/STA images. AP `hostapd-hwsim-wpa2-psk-ocv.conf` starts on 2412 MHz with `WPA-PSK-SHA256`, PMF required, and OCV extended capabilities in beacon/probe/assoc response IEs. STA `wpa_supplicant-hwsim-wpa2-psk-ocv.conf` completes the RSN 4-way handshake with `WPA IE for msg 2/4` carrying OCI KDE and message 3/4 carrying `OCI KDE in EAPOL-Key`, then reaches `CTRL-EVENT-CONNECTED` with `PTK=CCMP GTK=CCMP` and IGTK installed. Data proof: STA->AP ping 3/3 and iperf ~0.41 Mbits/sec over 6.02 s; AP->STA ping 3/3 and iperf ~0.48 Mbits/sec over 6.02 s. During this older AP->STA iperf proof the hostfs shared medium hit the then-known 8 MiB truncate path; newer builds use the configurable 64 MiB default, and throughput tuning remains separate from the functional OCV pass. |
| FILS | `WPA-EAP`, `wpa_key_mgmt=FILS-SHA256`, `eap_server_erp=1` | PASS | `CONFIG_FILS` and `CONFIG_ERP` are enabled and `src/ap/fils_hlp.c` is linked. AP config `hostapd-hwsim-fils-sha256.conf` starts with a FILS indication IE and reaches `AP-ENABLED`; STA config `wpa_supplicant-hwsim-fils-sha256.conf` selects `KEY_MGMT FILS-SHA256`, completes EAP-PSK, stores ERP keys, adds PMKSA cache entry for FILS cache id `0011`, completes RSN 4-way handshake with `PTK=CCMP GTK=CCMP` plus IGTK, and reaches `CTRL-EVENT-CONNECTED`. Data proof on `192.168.202.0/24`: AP->STA ping 3/3 and STA->AP ping 3/3, STA->AP iperf ~2.74 Mbits/sec over 6.02 s, and AP->STA iperf ~1.02 Mbits/sec over 6.02 s. Same-context reconnect proves the cached FILS path: STA finds a valid ERP key, logs `FILS: Try to use FILS (erp=1 pmksa_cache=1)`, completes `FILS: Auth+Assoc completed successfully`, reports `FILS_HLP_SENT`, AP finds the matching PMKSA cache entry, decrypts the encrypted Association Request elements with AES-SIV, and post-reconnect AP->STA ping is 3/3. STA2-present runs keep STA2 associated while STA1 reconnects or while both encrypted STAs exchange traffic. The latest reconnect-loop proof on `192.168.204.0/24` keeps STA2 online while STA1 performs three cached FILS reconnects: STA1 reaches four total `CTRL-EVENT-CONNECTED` events, logs three `FILS_HLP_SENT` reconnects, ERP key sequence advances through `SEQ=0`, `SEQ=1`, and `SEQ=2`, and the AP logs three PMKSA cache hits plus three decrypted encrypted Association Requests. Post-loop data proof passes STA1->STA2 ping 3/3, STA2->STA1 ping 3/3, STA1->STA2 iperf ~0.52 Mbits/sec, and STA2->STA1 iperf ~0.44 Mbits/sec. Earlier AP+STA1+STA2 proof on `192.168.203.0/24` showed AP connections for both STAs, both STAs at `CTRL-EVENT-CONNECTED`, STA1->STA2 ping 3/3 and iperf ~0.61 Mbits/sec, plus STA2->STA1 ping 3/3 and iperf ~1.09 Mbits/sec. Evidence: `/tmp/hwsim-fils-cmac-ap.log`, `/tmp/hwsim-fils-cmac-sta1.log`, `/tmp/hwsim-build-fils-cmac-ap.log`, `/tmp/hwsim-build-fils-cmac-sta1.log`, `/tmp/hwsim-build-fils-cmac-sta2.log`, `/tmp/hwsim-fils-sta2stress-ap.log`, `/tmp/hwsim-fils-sta2stress-sta1.log`, `/tmp/hwsim-fils-sta2stress-sta2.log`, `/tmp/hwsim-fils-reconn3-ap.log`, `/tmp/hwsim-fils-reconn3-sta1.log`, `/tmp/hwsim-fils-reconn3-sta2.log`, `/tmp/hwsim-fils-loop-ap.log`, `/tmp/hwsim-fils-loop-sta1.log`, and `/tmp/hwsim-fils-loop-sta2.log`. Longer soak loops and throughput/medium tuning remain follow-up stress coverage. |
| WPA3-Personal SAE | `wpa_key_mgmt=SAE`, `ieee80211w=2` | PASS | AP + STA1 + STA2 complete SAE group 19 commit/confirm, 4-way handshake with AES-CMAC MIC, install `PTK=CCMP GTK=CCMP` plus IGTK, STA-to-AP ping/iperf pass, and sequential STA1<->STA2 ping/iperf pass. After bounding the hwsim medium backlog, simultaneous bidirectional STA-to-STA stress is now functional: two runs kept post-stress ping at 3/3 in both directions, with low full-duplex iperf throughput around ~0.67/~0.96 and ~1.07/~1.28 Mbits/sec. A 2026-06-06 AP+STA1 refresh with the current images again proves `AP-STA-CONNECTED`, `EAPOL-4WAY-HS-COMPLETED`, `PTK=CCMP GTK=CCMP`, STA1->AP ping 5/5, STA1->AP iperf ~5.44 Mbits/sec, AP->STA1 ping 5/5, AP->STA1 iperf ~6.10 Mbits/sec, and post-iperf ping 3/3 both ways. During that refresh, one `wpa_cli status` call timed out and iperf hit the configured 64 MiB medium truncate bound, but data traffic remained healthy. Evidence: `/tmp/hwsim-wpa3-sae-refresh-ap.log` and `/tmp/hwsim-wpa3-sae-refresh-sta1.log`. Throughput/control-interface timing tuning remains open. |
| WPA2/WPA3 transition | `WPA-PSK SAE`, PMF optional | PASS | Transition AP accepts both paths. SAE STA uses `KEY_MGMT SAE`, AES-128-CMAC management group cipher, installs `PTK=CCMP GTK=CCMP`, pings with 0% loss, and runs iperf at ~17.73 Mbits/sec. A PSK-only STA uses `KEY_MGMT WPA-PSK`, installs `PTK=CCMP GTK=CCMP`, pings with 0% loss, and runs iperf at ~18.19 Mbits/sec. |
| WPA3 SAE H2E | `sae_pwe=1` | PASS | AP + STA1 derive PT, use `H2E=1`, exchange RSNXE, install `PTK=CCMP GTK=CCMP` plus IGTK, ping with 0% loss, and run iperf at ~18.32 Mbits/sec. A 2026-06-06 current-image refresh with `nuttx-sim-ap` and `nuttx-sim-sta1` again proves `AP-STA-CONNECTED`, `EAPOL-4WAY-HS-COMPLETED`, `PMKSA-CACHE-ADDED`, `PTK=CCMP GTK=CCMP`, and `CTRL-EVENT-CONNECTED`. Data proof: STA->AP ping 5/5, STA->AP iperf ~0.52 Mbits/sec, AP->STA ping 5/5, AP->STA iperf ~9.99 Mbits/sec, and post-iperf ping 3/3 both ways. Evidence: `/tmp/hwsim-wpa3-h2e-refresh2-ap.log` and `/tmp/hwsim-wpa3-h2e-refresh2-sta1.log`. One earlier refresh attempt started traffic before `CTRL-EVENT-CONNECTED` and produced ARP `Not reachable` errors; the successful run gates traffic on association/key completion. The 64 MiB shared medium truncate guard appeared during iperf but did not break data traffic. |
| WPA3 SAE-PK | SAE-PK variant | PASS | AP + STA1 complete the SAE-PK path: AP parses the SAE-PK private key, STA validates `K_AP` fingerprint, verifies KeyAuth, completes SAE commit/confirm, installs `PTK=CCMP GTK=CCMP` plus IGTK, reaches `CTRL-EVENT-CONNECTED`, pings with 0% loss, and passes bidirectional iperf. Earlier proof after gating temporary debug output reached STA->AP ping 3/3, STA->AP iperf ~20.27 Mbits/sec, and AP->STA iperf ~20.28 Mbits/sec. A 2026-06-06 current-image refresh with `nuttx-sim-ap` and `nuttx-sim-sta1` again proves AP `AP-ENABLED`, `AP-STA-CONNECTED`, `EAPOL-4WAY-HS-COMPLETED`, STA `Associated with`, `PTK=CCMP GTK=CCMP`, and `CTRL-EVENT-CONNECTED`. Current data proof: STA->AP ping 5/5, STA->AP iperf ~2.18 Mbits/sec, AP->STA ping 5/5, AP->STA iperf ~3.27 Mbits/sec, and AP->STA post-iperf ping 3/3. STA->AP post-iperf ping was 2/3 after one warm-up miss, and AP->STA iperf hit the configured 64 MiB shared-medium truncate guard, but both iperf directions completed and servers closed normally. Evidence: `/tmp/hwsim-sae-pk-data-ap.log` and `/tmp/hwsim-sae-pk-data-sta1.log`. |
| OWE | `wpa_key_mgmt=OWE`, PMF required | PASS | OWE groups 19, 20, and 21 now pass AP + STA1 runtime validation. Group 19 passes 2.4 GHz and 6 GHz HE association, matching PMKID, 4-way handshake, bidirectional ping, and bidirectional iperf. Group 21 passes after fixing the NuttX/libnl default netlink message size: STA->AP iperf ~17.75 Mbits/sec and AP->STA ~18.66 Mbits/sec. Group 20 passes after routing secp384r1 ECDH through a NuttX P-384 wrapper instead of the simulator OpenSSL EC keygen/scalar path: matching PMKID, OWE HMAC-SHA384 4-way handshake, `CTRL-EVENT-CONNECTED`, at least one 0%-loss ping in each direction, STA->AP iperf ~18.58 Mbits/sec, and AP->STA iperf ~18.77 Mbits/sec. |
| DPP / Easy Connect | DPP bootstrap/auth/config | PASS | AP configurator + STA enrollee DPP-PSK provisioning now passes end to end on the full `hostapd` + `wpa_supplicant` path. Latest build proof after the remain-on-channel fix: `build-ap.sh -j8` -> `rc=0`, `build/nuttx-sim-ap` size `23098584`; `build-sta1.sh -j8` -> `rc=0`, `build/nuttx-sim-sta1` size `27380008`. Runtime proof: STA generates a QR bootstrap URI, AP imports it with `hostapd_cli dpp_qr_code @/h/hwsim-dpp-uri.txt`, `dpp_auth_init @/h/hwsim-dpp-auth.txt` reaches `DPP-AUTH-SUCCESS`, AP reports `DPP-CONF-SENT conf_status=0`, STA processes the connector, scans `dppnet`, completes WPA2-PSK/PMF association with `PTK=CCMP GTK=CCMP`, reaches `CTRL-EVENT-CONNECTED`, and `wpa_cli status` reports `wpa_state=COMPLETED`. Traffic proof: AP->STA ping 3/3, STA->AP ping 3/3, STA->AP iperf ~1.65 Mbits/sec, and AP->STA iperf ~1.94 Mbits/sec. Fixes needed for this proof: avoid NSH long-line truncation with `@file` support in `hostapd_cli`/`wpa_cli`, allocate a full EC point buffer before dropping the uncompressed prefix in the DPP EC helper, and raise NuttX hwsim `max_remain_on_channel_duration` to 5000 ms so DPP Action TX `wait=2000` is accepted. CSR/PKCS#7 certificate-mode helpers are still explicit unsupported stubs, so DPP CSR/CA flows remain blocked until those crypto APIs are implemented. |
| Enterprise WPA2 | WPA-EAP with EAP-PSK, EAP-TLS, PEAP/MSCHAPv2, and TTLS inner PAP/CHAP/MSCHAP/MSCHAPv2 | PASS | EAP-PSK: AP + STA1 pass the internal hostapd EAP server, complete RSN 4-way handshake with `PTK=CCMP GTK=CCMP`, ping with 0% loss, and run STA-to-AP iperf at ~18.57 Mbits/sec. EAP-TLS: AP + STA1 complete certificate-based TLS authentication, `CTRL-EVENT-EAP-SUCCESS`, RSN 4-way handshake with `PTK=CCMP GTK=CCMP`, STA->AP ping 3/3, AP->STA ping 3/3, STA->AP iperf ~0.46 Mbits/sec, AP->STA iperf ~0.89 Mbits/sec, and post-iperf ping 3/3. PEAP/MSCHAPv2: AP + STA1 complete outer PEAP method 25, server certificate validation, tunneled `EAP-MSCHAPV2: Authentication succeeded`, EAP-TLV success, RSN 4-way with `PTK=CCMP GTK=CCMP`, bidirectional ping 3/3, STA->AP iperf ~0.67 Mbits/sec, and AP->STA iperf ~0.61 Mbits/sec. TTLS/PAP, TTLS/CHAP, TTLS/MSCHAP, and TTLS/MSCHAPv2 all complete outer TTLS method 21, server certificate validation, EAP success, RSN 4-way with `PTK=CCMP GTK=CCMP`, bidirectional ping 3/3, and bidirectional iperf. Latest TTLS results: PAP ~1.39/~0.72 Mbits/sec, CHAP ~1.44/~0.44 Mbits/sec, MSCHAP ~1.55/~1.00 Mbits/sec, MSCHAPv2 ~1.39/~1.76 Mbits/sec. External RADIUS remains separate pending coverage. |
| Suite-B / 192-bit | `WPA-EAP-SUITE-B-192`, `GCMP-256`, `BIP-GMAC-256` | PASS | `CONFIG_SUITEB`, `CONFIG_SUITEB192`, `CONFIG_SHA384`, and `CONFIG_SHA512` are enabled. AP + STA1 advertise and select `WPA-EAP-SUITE-B-192`, `PTK=GCMP-256`, `GTK=GCMP-256`, and `BIP-GMAC-256`; PMF-required association succeeds; RSA3072 EAP-TLS certificate exchange completes with `CTRL-EVENT-EAP-SUCCESS`; RSN 4-way completes with `PTK=GCMP-256 GTK=GCMP-256`; `CTRL-EVENT-CONNECTED`, ping 3/3, and STA-to-AP iperf pass. Latest proof: STA->AP ping 0% loss and iperf ~0.57 Mbits/sec. Caveat: this is simulator runtime coverage for the current internal TLS profile, not a strict CNSA/Suite-B TLS proof, because the internal TLS stack negotiates TLS 1.0 DHE-RSA/AES-CBC rather than an ECDHE/ECDSA SHA384/GCM profile. |

## WPA2-Enterprise EAP-PSK Runtime Proof

Additional Enterprise smoke-test configs are kept under `tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-eap-psk.conf
hostapd-hwsim-wpa2-eap-psk.users
wpa_supplicant-hwsim-wpa2-eap-psk.conf
```

Current interactive AP + STA1 proof:

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

Issues fixed during bring-up:

- AP config must enable `ieee8021x=1`; without it, hostapd associated the STA
  but failed with `IEEE 802.1X not enabled` and could not derive the PMK.
- The STA must use the same EAP identity that appears in the hostapd user
  file. An earlier `anonymous_identity=eap_psk_user` made the AP fail with
  `ieee802_1x_get_eap_user: Failed to find user`.

## WPA2-Enterprise EAP-TLS Runtime Proof

Additional EAP-TLS configs and certificate material are kept under
`tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-eap-tls.conf
hostapd-hwsim-wpa2-eap-tls.users
wpa_supplicant-hwsim-wpa2-eap-tls.conf
eaptls.conf
eap-tls-certs/ca.pem
eap-tls-certs/server.pem
eap-tls-certs/server.key
eap-tls-certs/client.pem
eap-tls-certs/client.key
eap-tls-certs/dhparam.pem
```

Current AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 13 (TLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Traffic proof:

```text
STA -> AP ping:
3 packets transmitted, 3 received, 0% packet loss

AP -> STA ping:
3 packets transmitted, 3 received, 0% packet loss

STA -> AP iperf:
0.00-   6.02 sec     344064 Bytes    0.46 Mbits/sec

AP -> STA iperf:
0.00-   6.02 sec     671744 Bytes    0.89 Mbits/sec

Post-iperf STA -> AP ping:
3 packets transmitted, 3 received, 0% packet loss
```

Issues fixed during EAP-TLS bring-up:

- Problem: the internal TLS server selected a DHE-RSA cipher suite, but the AP
  config had no DH parameters. The first failure stopped after the ClientHello
  with no TLS alert.
- Fix: generate `eap-tls-certs/dhparam.pem` and set
  `dh_file=/h/eap-tls-certs/dhparam.pem` in the hostapd config.
- Problem: the server certificate flight and later the client certificate
  response exceeded the current NuttX hwsim/EAPOL send size and failed with
  `devif_send error: -90`.
- Fix: set `fragment_size=512` in both hostapd and wpa_supplicant EAP-TLS
  configs.
- Problem: NuttX sim time starts before the generated certificates'
  `notBefore`, and the imported internal TLS server ignored hostapd
  `tls_flags=[DISABLE-TIME-CHECKS]` while validating client certificates.
- Fix: add internal TLS server flag storage, pass flags from
  `tls_connection_set_verify()`, and honor `TLS_CONN_DISABLE_TIME_CHECKS` in
  server-side client certificate validation. The sim hostapd config now sets
  `tls_flags=[DISABLE-TIME-CHECKS]`. Real targets should prefer correct RTC
  time or certificates valid for the target clock.

## WPA2-Enterprise PEAP/MSCHAPv2 Runtime Proof

Additional PEAP configs are kept under `tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-eap-peap.conf
hostapd-hwsim-wpa2-eap-peap.users
wpa_supplicant-hwsim-wpa2-eap-peap.conf
```

The AP config reuses the EAP-TLS server certificate, CA, DH parameter, TLS
time-check disable flag, and 512-byte EAP fragment size. The user database
allows an anonymous PEAP outer identity and validates the tunneled inner user
with MSCHAPv2:

```text
"anonymous@example.com" PEAP
"peap_user@example.com" PEAP
"peap_user@example.com" MSCHAPV2 "peap_password" [2]
```

Current AP + STA1 proof:

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
```

Traffic proof:

```text
STA -> AP ping:
3 packets transmitted, 3 received, 0% packet loss

AP -> STA ping:
3 packets transmitted, 3 received, 0% packet loss

STA -> AP iperf:
0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec

AP -> STA iperf:
0.00-   6.02 sec     458752 Bytes    0.61 Mbits/sec
```

Issues and notes:

- PEAP uses the same TLS server path as EAP-TLS, so the same simulator fixes
  remain required: `dh_file`, `fragment_size=512`, and
  `tls_flags=[DISABLE-TIME-CHECKS]`.
- The PEAP user file needs distinct phase-1 and phase-2 entries. The tunneled
  MSCHAPv2 identity is marked with `[2]`; without that, hostapd would not use
  the password entry for the inner method.
- Throughput is low like the EAP-TLS run, so Enterprise throughput tuning
  remains open even though authentication and data traffic are now functional.

## WPA2-Enterprise TTLS/PAP Runtime Proof

Additional TTLS configs are kept under `tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-eap-ttls.conf
hostapd-hwsim-wpa2-eap-ttls.users
wpa_supplicant-hwsim-wpa2-eap-ttls.conf
```

The AP config reuses the EAP-TLS server certificate, CA, DH parameter, TLS
time-check disable flag, and 512-byte EAP fragment size. The user database
allows an anonymous TTLS outer identity and validates the tunneled inner user
with TTLS-PAP:

```text
"anonymous@example.com" TTLS
"ttls_user@example.com" TTLS
"ttls_user@example.com" TTLS-PAP "ttls_password" [2]
```

Current AP + STA1 proof:

```text
wlan0: CTRL-EVENT-EAP-METHOD EAP vendor 0 method 21 (TTLS) selected
wlan0: CTRL-EVENT-EAP-PEER-CERT depth=0 subject='CN=nuttx-hwsim-eap-server'
wlan0: CTRL-EVENT-EAP-SUCCESS EAP authentication completed successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: CTRL-EVENT-EAP-SUCCESS 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Traffic proof:

```text
STA -> AP ping:
3 packets transmitted, 3 received, 0% packet loss

AP -> STA ping:
3 packets transmitted, 3 received, 0% packet loss

STA -> AP iperf:
0.00-   6.02 sec    1048576 Bytes    1.39 Mbits/sec

AP -> STA iperf:
0.00-   6.02 sec     540672 Bytes    0.72 Mbits/sec
```

Issues and notes:

- TTLS uses the same TLS server path as EAP-TLS/PEAP, so the same simulator
  parameters remain required: `dh_file`, `fragment_size=512`, and
  `tls_flags=[DISABLE-TIME-CHECKS]`.
- The TTLS user file needs a phase-2 `[2]` entry using the non-EAP method name
  `TTLS-PAP`. The STA side pairs that with `phase2="auth=PAP"`.
- This validates TTLS/PAP. TTLS with CHAP/MSCHAP/MSCHAPV2 remains separate
  optional coverage if needed for a broader Enterprise method matrix.

## WPA2-Enterprise TTLS/MSCHAPv2 Runtime Proof

Additional TTLS/MSCHAPv2 configs are kept under `tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-eap-ttls-mschapv2.conf
hostapd-hwsim-wpa2-eap-ttls-mschapv2.users
wpa_supplicant-hwsim-wpa2-eap-ttls-mschapv2.conf
ttlsms.conf
```

The AP config reuses the EAP-TLS server certificate, CA, DH parameter, TLS
time-check disable flag, and 512-byte EAP fragment size. The user database
allows an anonymous TTLS outer identity and validates the tunneled inner user
with TTLS-MSCHAPV2:

```text
"anonymous@example.com" TTLS
"ttls_mschapv2_user@example.com" TTLS
"ttls_mschapv2_user@example.com" TTLS-MSCHAPV2 "ttls_mschapv2_password" [2]
```

Current AP + STA1 proof:

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
```

Traffic proof:

```text
STA -> AP ping:
3 packets transmitted, 3 received, 0% packet loss

AP -> STA ping:
3 packets transmitted, 3 received, 0% packet loss

STA -> AP iperf:
0.00-   6.02 sec    1048576 Bytes    1.39 Mbits/sec

AP -> STA iperf:
0.00-   6.02 sec    1327104 Bytes    1.76 Mbits/sec
```

Issues and notes:

- TTLS/MSCHAPv2 uses the same TLS server path as EAP-TLS/PEAP/TTLS-PAP, so
  the same simulator parameters remain required: `dh_file`,
  `fragment_size=512`, and `tls_flags=[DISABLE-TIME-CHECKS]`.
- The TTLS user file needs a phase-2 `[2]` entry using the non-EAP method name
  `TTLS-MSCHAPV2`. The STA side pairs that with `phase2="auth=MSCHAPV2"`.
- The full STA config path was long enough that one NSH launch lost the
  trailing background `&`, leaving `wpa_supplicant` in the foreground. A short
  alias config, `ttlsms.conf`, avoids the command-line-length issue and was
  used for the final runtime proof.

## WPA2-Enterprise TTLS/CHAP And TTLS/MSCHAP Runtime Proof

Additional TTLS inner-method configs are kept under `tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-eap-ttls-chap.conf
hostapd-hwsim-wpa2-eap-ttls-chap.users
wpa_supplicant-hwsim-wpa2-eap-ttls-chap.conf
ttlschap.conf
hostapd-hwsim-wpa2-eap-ttls-mschap.conf
hostapd-hwsim-wpa2-eap-ttls-mschap.users
wpa_supplicant-hwsim-wpa2-eap-ttls-mschap.conf
ttlsmschap.conf
```

Both tests reuse the EAP-TLS server certificate, CA, DH parameter, TLS
time-check disable flag, and 512-byte EAP fragment size. The user databases
allow an anonymous TTLS outer identity and then constrain the tunneled inner
method:

```text
"ttls_chap_user@example.com" TTLS-CHAP "ttls_chap_password" [2]
"ttls_mschap_user@example.com" TTLS-MSCHAP "ttls_mschap_password" [2]
```

Current TTLS/CHAP AP + STA1 proof:

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
STA -> AP iperf: 0.00-   6.02 sec    1081344 Bytes    1.44 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     327680 Bytes    0.44 Mbits/sec
```

Current TTLS/MSCHAP AP + STA1 proof:

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
STA -> AP iperf: 0.00-   6.02 sec    1163264 Bytes    1.55 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec     753664 Bytes    1.00 Mbits/sec
```

Issues and notes:

- TTLS/CHAP and TTLS/MSCHAP use the same TLS server path as the other
  Enterprise tunneled methods, so `dh_file`, `fragment_size=512`, and
  `tls_flags=[DISABLE-TIME-CHECKS]` remain required in the simulator.
- The imported EAP-TTLS peer prints an explicit phase-2 success line for
  MSCHAPv2, but not for CHAP or MSCHAP. These runs used user databases that
  only allow the requested inner method, so EAP success plus RSN keying proves
  the configured phase-2 method completed.

## WPA2 TKIP, Mixed Cipher, And Transition Runtime Proof

Additional security configs are kept under `tools/firmware/sim`:

```text
hostapd-hwsim-wpa2-psk-tkip.conf
wpa_supplicant-hwsim-wpa2-psk-tkip.conf
hostapd-hwsim-wpa-mixed.conf
wpa_supplicant-hwsim-wpa-mixed.conf
hostapd-hwsim-wpa-transition.conf
wpa_supplicant-hwsim-wpa-transition.conf
wpa_supplicant-hwsim-wpa-transition-psk.conf
hostapd-hwsim-wpa3-sae-h2e.conf
wpa_supplicant-hwsim-wpa3-sae-h2e.conf
```

TKIP proof from `/tmp/hwsim-tkip-ap.log` and
`/tmp/hwsim-tkip-sta1.log`:

```text
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=TKIP GTK=TKIP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   13271040 Bytes   21.02 Mbits/sec
```

Mixed TKIP/CCMP proof from `/tmp/hwsim-mixed-ap.log` and
`/tmp/hwsim-mixed-sta1.log`:

```text
wlan0: WPA: Selected cipher suites: group 8 pairwise 16 key_mgmt 2 proto 2
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=TKIP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
```

WPA2/WPA3 transition SAE proof from `/tmp/hwsim-transition-ap.log` and
`/tmp/hwsim-transition-sta1.log`:

```text
wlan0: RSN: using KEY_MGMT SAE
SAE: State Confirmed -> Accepted for peer 02:00:00:00:00:02 (Accept Confirm)
wlan0: WPA: using MGMT group cipher AES-128-CMAC
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11190272 Bytes   17.73 Mbits/sec
```

WPA2/WPA3 transition legacy WPA-PSK proof from
`/tmp/hwsim-transition-psk2-sta1.log` and
`/tmp/hwsim-transition-psk3-sta1.log`:

```text
wlan0: WPA: using KEY_MGMT WPA-PSK
wlan0: WPA: not using MGMT group cipher
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11485184 Bytes   18.19 Mbits/sec
```

WPA3-SAE H2E proof from `/tmp/hwsim-sae-h2e-ap.log` and
`/tmp/hwsim-sae-h2e-sta1.log`:

```text
SAE: Derive PT - group 19
SAE: Derive PWE from PT
SAE: Derive keys - H2E=1 AKMP=0x400 = 000fac08 (SAE)
WPA: RSNXE in EAPOL-Key - hexdump(len=3): f4 01 20
wlan0: WPA: IGTK keyid 4 pn 000000000000
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   11567104 Bytes   18.32 Mbits/sec
```

2.4 GHz OWE proof from `/tmp/hwsim-owe-bidir-ap.log` and
`/tmp/hwsim-owe-bidir-sta1.log`:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
OWE: PMKID - hexdump(len=16): [matching on AP and STA]
STA -> AP: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP: 0.00-   5.05 sec   12435456 Bytes   19.70 Mbits/sec
AP -> STA: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA: 0.00-   5.05 sec   11649024 Bytes   18.45 Mbits/sec
```

6 GHz OWE proof from `/tmp/hwsim-ax6-owe-probe-ap.log`,
`/tmp/hwsim-ax6-owe-probe-sta1.log`, `/tmp/hwsim-ax6-owe-data-ap.log`, and
`/tmp/hwsim-ax6-owe-data-sta1.log`:

```text
nl80211: Set freq 5955 (... he_enabled=1 ... bandwidth=20 MHz ...)
wlan0: BSS: Add new id 0 BSSID 02:00:00:00:00:01 SSID 'nuttx-hwsim-ax6-owe' freq 5955
OWE: PMKID - hexdump(len=16): b6 04 68 46 f2 c2 b6 a2 1e 0f 7e be 48 c8 da 32
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA -> AP: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP: 0.00-   5.06 sec   12533760 Bytes   19.82 Mbits/sec
AP -> STA: 3 packets transmitted, 3 received, 0% packet loss
AP -> STA: 0.00-   5.05 sec   12255232 Bytes   19.41 Mbits/sec
```

OWE group 21 proof after fixing the NuttX/libnl default netlink message size
from `/tmp/hwsim-owe-g21-libnl-ap.log`,
`/tmp/hwsim-owe-g21-libnl-sta1.log`,
`/tmp/hwsim-owe-g21-sta2ap-sta1.log`, and
`/tmp/hwsim-owe-g21-ap2sta-ap.log`:

```text
OWE: PMKID - hexdump(len=16): [matching on AP and STA]
nl80211: Association request send successfully
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA -> AP: 5 packets transmitted, 5 received, 0% packet loss
STA -> AP: 0.00-   5.05 sec   11206656 Bytes   17.75 Mbits/sec
AP -> STA: 5 packets transmitted, 5 received, 0% packet loss
AP -> STA: 0.00-   5.05 sec   11780096 Bytes   18.66 Mbits/sec
```

OWE group 20 proof after adding the simulator-only NuttX P-384 ECDH wrapper in
`apps/wireless/wifi/common/nuttx_wpa_p384_ecdh.c` and routing group 20 through
it from `nuttx_wpa_openssl_ec.c`:

```text
/tmp/hwsim-owe-g20-p384-clean-ap.log
/tmp/hwsim-owe-g20-p384-clean-sta1.log

AP OWE PMKID:  4f f1 17 cd 38 9a 64 ba 4e 72 ea 1c 8e 44 7e f5
STA OWE PMKID: 4f f1 17 cd 38 9a 64 ba 4e 72 ea 1c 8e 44 7e f5
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

STA -> AP ping: 5 packets transmitted, 5 received, 0% packet loss
AP -> STA ping: 5 packets transmitted, 5 received, 0% packet loss
STA -> AP iperf: 0.00-   5.05 sec   11730944 Bytes   18.58 Mbits/sec
AP -> STA iperf: 0.00-   5.05 sec   11845632 Bytes   18.77 Mbits/sec
```

## WPA3 / SAE Runtime Enablement

SAE is enabled for the simulator build with:

```text
CONFIG_SAE
wpa_supplicant-2.11/src/common/sae.c
wpa_supplicant-2.11/src/common/dragonfly.c
apps/wireless/wifi/common/nuttx_wpa_openssl_ec.c
```

The first enablement attempt reached link time but failed because the internal
crypto build did not provide the SAE/Dragonfly bignum and elliptic-curve API.
That gap is now covered in simulator builds by `nuttx_wpa_openssl_ec.c`, linked
with `-lcrypto` from the sim board link rules.

Two simulator-specific fixes were needed:

- `crypto_bignum_rand()` uses the WPA/NuttX `random_get_bytes()` path instead
  of OpenSSL `BN_rand_range()` to avoid OpenSSL's own RNG path inside NuttX sim.
- Multiple NuttX sim instances can start with the same `/dev/urandom` state.
  `crypto_bignum_rand()` now mixes process-local time/address/counter data into
  the random buffer so SAE peers do not generate identical reflected commits.
- early libc `getenv()` / `get_environ_ptr()` now tolerate calls before a NuttX
  task TCB exists, because OpenSSL static initialization queries environment
  variables before NSH starts.

Build proof:

```text
./FeatherCore/tools/firmware/sim/build-ap.sh -j8    -> ap_rc=0
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8  -> sta1_rc=0
```

Runtime proof from `/tmp/hwsim-sae-ap-randfix.log` and
`/tmp/hwsim-sae-sta1-randfix.log`:

```text
SAE: State Confirmed -> Accepted for peer 02:00:00:00:00:02 (Accept Confirm)
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
10 packets transmitted, 10 received, 0% packet loss, time 10100 ms
0.00-   5.05 sec   12632064 Bytes   20.01 Mbits/sec
```

Two-STA WPA3-SAE proof from `/tmp/hwsim-sae-2sta-ap.log`,
`/tmp/hwsim-sae-2sta-sta1.log`, and `/tmp/hwsim-sae-2sta-sta2.log`:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: AP-STA-CONNECTED 02:00:00:00:00:03
STA1 -> STA2: 10 packets transmitted, 10 received, 0% packet loss
STA1 -> STA2: 0.00-   5.05 sec    5799936 Bytes    9.19 Mbits/sec
STA2 -> STA1: 10 packets transmitted, 10 received, 0% packet loss
STA2 -> STA1: 0.00-   5.05 sec    5718016 Bytes    9.06 Mbits/sec
```

WPA3-SAE simultaneous bidirectional STA-to-STA stress was then rerun with AP +
STA1 + STA2 after bounding the file-backed hwsim medium. Both STAs remained
associated, full-duplex TCP iperf completed in both directions twice, and
post-stress ping still passed in both directions:

```text
Baseline:
STA1 -> STA2 ping: 3 packets transmitted, 3 received, 0% packet loss
STA2 -> STA1 ping: 3 packets transmitted, 3 received, 0% packet loss

Full-duplex run 1:
STA1 -> STA2 iperf: 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec
STA2 -> STA1 iperf: 0.00-   6.02 sec     720896 Bytes    0.96 Mbits/sec
Post-stress ping: both directions 3/3, 0% packet loss

Full-duplex run 2:
STA1 -> STA2 iperf: 0.00-   6.02 sec     802816 Bytes    1.07 Mbits/sec
STA2 -> STA1 iperf: 0.00-   6.02 sec     966656 Bytes    1.28 Mbits/sec
Post-stress ping: both directions 3/3, 0% packet loss
```

This upgrades simultaneous WPA3-SAE STA-to-STA stress from link-breaking to
functional. Throughput is still much lower than sequential STA-to-STA iperf,
so longer stress and medium scheduling/performance tuning remain open.

SAE-PK AP + STA1 proof after gating temporary debug output and running
hostapd/wpa_supplicant through NSH background tasks:

```text
AP:
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

STA:
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
ping -c 3 192.168.201.1: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec   15253504 Bytes   20.27 Mbits/sec
AP -> STA iperf: 0.00-   6.03 sec   15286272 Bytes   20.28 Mbits/sec
```

Issue/fix captured during this rerun:

- Problem: `hostapd -B` and `wpa_supplicant -B` can leave the simulator shell
  without a usable prompt in this NuttX port, so follow-up `ifconfig`, `ping`,
  and `iperf` commands may not run even after association succeeds.
- Fix: run both daemons as explicit NSH background jobs, for example
  `hostapd /h/hostapd-hwsim-wpa3-sae-pk.conf &` and
  `wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-wpa3-sae-pk.conf &`.
- Problem: temporary `hwsim-debug`/`nl80211-debug` printf instrumentation made
  interactive data-plane captures hard to read and previously hid prompt state.
- Fix: gate the temporary debug prints behind `CONFIG_WL_NUTTX_HWSIM_DEBUG` in
  the NuttX IEEE80211/hwsim/netlink path and the local libnl/nl80211 driver
  shims.

Remaining SAE work: longer repeated full-duplex stress and throughput tuning.

## PHY / Standard Validation Matrix

| Standard | Representative config | Current status | Evidence / next action |
| --- | --- | --- | --- |
| 802.11b | `hw_mode=b`, channel 1 | PASS | AP starts in `Mode: IEEE 802.11b` on 2412 MHz, STA associates, ping passes with 0% loss, and iperf reaches ~17.68 Mbits/sec. A 2026-06-06 current-image refresh with `nuttx-sim-ap` and `nuttx-sim-sta1` again proves `start_ap parse chandef freq=2412`, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. Data proof: STA->AP ping 5/5, STA->AP iperf ~0.78 Mbits/sec, post ping 3/3, AP->STA ping 5/5, AP->STA iperf ~0.89 Mbits/sec, and post ping 3/3. Evidence: `/tmp/hwsim-11b-refresh-ap.log` and `/tmp/hwsim-11b-refresh-sta1.log`. Non-blocking compatibility warnings seen in the run: `SO_ATTACH_FILTER` unsupported, one unsupported nl80211 command returning `-22`, and one `cmd=54` returning `-19`; association and data traffic still passed. |
| 802.11g | `hw_mode=g`, channel 1 | PASS | Dedicated open-auth 11g AP/STA configs now exist: `hostapd-g.conf` and `wpa-g.conf`. Runtime proof starts AP on channel 1 / 2412 MHz with legacy chandef only (`ht_cap=0`, `vht_cap=0`, `he_cap=0`, `he_oper=0`), STA selects `nuttx-hwsim-g`, associates on 2412 MHz, and data traffic passes. A 2026-06-06 current-image refresh with `nuttx-sim-ap` and `nuttx-sim-sta1` proves `start_ap parse chandef freq=2412`, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. Data proof: STA->AP ping 5/5, STA->AP iperf ~1.13 Mbits/sec, post ping 3/3, AP->STA ping 5/5, AP->STA iperf ~1.37 Mbits/sec, and post ping 3/3. Evidence: `/tmp/hwsim-11g-refresh-ap.log` and `/tmp/hwsim-11g-refresh-sta1.log`. Non-blocking compatibility warnings seen in the run match the 11b baseline: `SO_ATTACH_FILTER` unsupported plus optional nl80211 `-22`/`-19`; association and data traffic still passed, and no medium truncate occurred. Older reference logs: `/tmp/hwsim-11g-ap.log`, `/tmp/hwsim-11g-sta1.log`. |
| 802.11a | `hw_mode=a`, 5 GHz channel 36 | PASS | AP starts on 5180 MHz, STA associates on 5180 MHz, ping passes with 0% loss, and earlier iperf reaches ~17.75 Mbits/sec. A 2026-06-06 current-image refresh with `nuttx-sim-ap` and `nuttx-sim-sta1` again proves `start_ap parse chandef freq=5180`, legacy-only `rdev_start_ap ret=0 he_cap=0 he_oper=0 ht_cap=0 vht_cap=0`, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. Data proof: STA->AP ping 5/5, STA->AP iperf ~0.65 Mbits/sec, post ping 3/3, AP->STA ping 5/5, AP->STA iperf ~1.52 Mbits/sec, and post ping 3/3. Evidence: `/tmp/hwsim-11a-refresh-ap.log` and `/tmp/hwsim-11a-refresh-sta1.log`. Non-blocking compatibility warnings seen in the run: rfkill info query unsupported, `SO_ATTACH_FILTER` unsupported, and optional nl80211 commands returning `-22`, `-19`, `-67`, or `-95`; association and data traffic still passed, and no medium truncate occurred. |
| 802.11n / HT | `ieee80211n=1`, 2.4 GHz HT20/HT40 | PASS | HT20 and HT40 AP/STA runtime pass. HT20 reaches ~17.93 Mbits/sec in older reference runs. HT40 starts with `bandwidth=40 MHz`, secondary channel 10, ping passes with 0% loss, and iperf reaches ~17.75 Mbits/sec. A 2026-06-06 current-image HT20 refresh with `nuttx-sim-ap` and `nuttx-sim-sta1` proves `start_ap parse chandef freq=2437`, `rdev_start_ap ret=0` with non-null `ht_cap` and null HE/VHT caps, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. Data proof: STA->AP ping 5/5, STA->AP iperf ~4.22 Mbits/sec, AP->STA ping 5/5, AP->STA iperf ~0.46 Mbits/sec, AP->STA post ping 3/3, and a follow-up STA->AP post ping 3/3 after one immediate post-iperf warm-up miss. A 2026-06-06 current-image HT40 refresh uses `hostapd-n-ht40.conf` with `ht_capab=[HT40+][SHORT-GI-20][SHORT-GI-40]`, proves AP `HT_SCAN->ENABLED`, `start_ap parse chandef freq=2437`, non-null `ht_cap`, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. Current HT40 logs do not emit an explicit runtime bandwidth field, so this refresh records config-level `HT40+` plus HT AP state/capability proof. HT40 data proof: STA->AP ping 5/5, STA->AP iperf ~3.96 Mbits/sec, post ping 3/3, AP->STA ping 5/5, AP->STA iperf ~0.59 Mbits/sec, and post ping 3/3. A-MPDU/ADDBA was re-triggered in both directions with `tx operational`; A-MSDU probe instrumentation ran but did not form an A-MSDU in these short flows (`probe skip no-head`). Evidence: `/tmp/hwsim-11n-ht20-refresh-ap.log`, `/tmp/hwsim-11n-ht20-refresh-sta1.log`, `/tmp/hwsim-11n-ht40-refresh-ap.log`, and `/tmp/hwsim-11n-ht40-refresh-sta1.log`. Non-blocking compatibility warnings match the 11a baseline: rfkill info query unsupported, `SO_ATTACH_FILTER` unsupported, and optional nl80211 commands returning `-22`, `-19`, `-67`, or `-95`; no medium truncate occurred. |
| 802.11ac / VHT | `ieee80211ac=1`, 5 GHz VHT20/VHT80/VHT160/VHT80+80 | PASS | VHT20, VHT80, VHT160, and VHT80+80 AP/STA runtime pass. A 2026-06-06 current-image VHT20 refresh with `hostapd-ac.conf` and `wpa-ac.conf` proves `start_ap parse chandef freq=5180`, `rdev_start_ap ret=0` with non-null `ht_cap` and `vht_cap`, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. VHT20 data proof: STA->AP ping 5/5, STA->AP iperf ~2.42 Mbits/sec, post ping 3/3, AP->STA ping 5/5, AP->STA iperf ~2.83 Mbits/sec, and post ping 3/3. The same run re-triggered ADDBA/A-MPDU `tx operational` on both peers; A-MSDU instrumentation stayed probe-only (`probe skip no-head`). The run did hit the configured 64 MiB hostfs shared-medium truncate guard during AP->STA server-side accounting, but both iperf clients completed and post-iperf ping passed. A same-day current-image VHT80 refresh with `hostapd-ac-vht80.conf` (`vht_oper_chwidth=1`, `vht_oper_centr_freq_seg0_idx=42`) and `wpa-ac-vht80.conf` proves AP `HT_SCAN->ENABLED`, `start_ap parse chandef freq=5180`, `rdev_start_ap ret=0` with non-null `ht_cap` and `vht_cap`, AP `AP-STA-CONNECTED`, STA `Associated with`, and STA `CTRL-EVENT-CONNECTED`. Current VHT80 data proof: STA->AP ping 5/5, STA->AP iperf ~4.22 Mbits/sec, immediate post ping 2/3 after one warm-up miss, AP->STA ping 5/5, AP->STA iperf ~0.87 Mbits/sec, and post ping 3/3. A follow-up current-image AP+STA1+STA2 VHT80 replay proves two STAs associate to the same AP (`AP-STA-CONNECTED` for `02:00:00:00:00:02` and `02:00:00:00:00:03`), STA1->STA2 ping 5/5 and TCP iperf ~1.46 Mbits/sec, and STA2->STA1 ping 5/5 and TCP iperf ~3.74 Mbits/sec; post-load pings were STA1->STA2 2/3 after one warm-up miss and STA2->STA1 3/3. ADDBA/A-MPDU reaches `tx operational` on AP and both STAs; A-MSDU remains probe-only. Evidence: `/tmp/hwsim-vht20-refresh-ap.log`, `/tmp/hwsim-vht20-refresh-sta1.log`, `/tmp/hwsim-vht80-refresh-ap.log`, `/tmp/hwsim-vht80-refresh-sta1.log`, `/tmp/hwsim-vht80-sta2sta-ap.log`, `/tmp/hwsim-vht80-sta2sta-sta1.log`, and `/tmp/hwsim-vht80-sta2sta-sta2.log`. Earlier VHT80 proof starts with `bandwidth=80 MHz`, center frequency 5210 MHz, STA associates on 5180 MHz, ping passes with 0% loss, and iperf reaches ~17.68 Mbits/sec. Latest VHT80 IE proof shows Beacon, Probe Response, and Association Response all carry VHT Capabilities IE `bf 0c 20 00 00 00 aa aa 00 00 aa aa 00 00` and VHT Operation IE `c0 05 01 2a 00 fc ff`; post-association ping is 3/3. VHT160 proof starts with `bandwidth=160 MHz`, `channel_width=5`, center frequency 5250 MHz, STA associates on 5180 MHz, ping passes with 0% loss, and bidirectional iperf reaches ~1.22/~1.17 Mbits/sec; existing logs also show VHT Capabilities plus VHT Operation `c0 05 01 2a 32 fc ff` in Beacon/Probe Response/Association Response. VHT80+80 proof starts with `channel_width=4`, center frequencies 5210/5775 MHz, ping passes with 0% loss, and bidirectional iperf reaches ~1.30/~0.31 Mbits/sec; existing logs also show VHT Capabilities plus VHT Operation `c0 05 01 2a 9b fc ff` in Beacon/Probe Response/Association Response. Wide-channel throughput and medium-bound tuning remain open. Evidence: `/tmp/hwsim-vht80-ie-ap.log`, `/tmp/hwsim-vht80-ie-sta1.log`, `/tmp/hwsim-vht160-ap2.log`, `/tmp/hwsim-vht160-sta1-2.log`, `/tmp/hwsim-vht80p80-ap.log`, and `/tmp/hwsim-vht80p80-sta1.log`. |
| 802.11ax / HE 2.4 GHz | `ieee80211ax=1`, 2.4 GHz channel 6 | PASS | `hostapd-ax.conf` starts AP on 2437 MHz with `he_enabled=1`; AP reaches `AP-ENABLED`, receives STA `he_capab`, STA associates, ping passes with 0% loss, and iperf reaches ~17.75 Mbits/sec. |
| 802.11ax / HE 5 GHz | `ieee80211ac=1`, `ieee80211ax=1`, 5 GHz channel 36 | PASS | `hostapd-ax-a.conf` starts AP on 5180 MHz with `vht_enabled=1` and `he_enabled=1`; AP reaches `AP-ENABLED`, receives STA `vht_capabilities` and `he_capab`, STA associates on 5180 MHz, ping passes with 0% loss, and iperf reaches ~17.75 Mbits/sec. A 2026-06-06 current-image refresh again proves `start_ap parse chandef freq=5180`, `rdev_start_ap ret=0` with non-null `he_cap`, `he_oper`, `ht_cap`, and `vht_cap`, AP `AP-ENABLED`, AP `AP-STA-CONNECTED`, STA `CTRL-EVENT-CONNECTED`, STA->AP ping 5/5, STA->AP iperf ~1.52 Mbits/sec, AP->STA ping 5/5, AP->STA iperf ~2.40 Mbits/sec, and post-iperf ping 3/3 both ways. The same refresh also re-triggered ADDBA/A-MPDU `tx operational` on both peers. Evidence: `/tmp/hwsim-11ax5-refresh-ap.log` and `/tmp/hwsim-11ax5-refresh-sta1.log`. One duplicate AP `iperf -s` command returned `tcp_selectport failed: -98` because the original server was already listening, and the first STA ARP probe had a warm-up timeout before the 5/5 ping summary. |
| 802.11ax / HE 6 GHz | `op_class=131`, 6 GHz channel 1, SAE/H2E and OWE | PASS | Open/pre-RSNA AP mode is correctly rejected on 6 GHz, so validation uses secure AKMs. SAE/H2E AP starts on 5955 MHz with `he_enabled=1`, STA scans the 6 GHz BSS, SAE/H2E completes, AP reports `AP-STA-CONNECTED`, ping is 10/10 both directions, and STA->AP iperf reaches ~20.35 Mbits/sec. OWE also passes on 5955 MHz with PMF required, 0% loss ping both directions, STA->AP ~19.82 Mbits/sec, and AP->STA ~19.41 Mbits/sec. |
| S1G / 802.11ah | S1G channel 20 / 912 MHz, 1 MHz width | PASS | Open-auth S1G AP + STA1 + STA2 now passes on the full `hostapd` + `wpa_supplicant` path. AP starts on 912 MHz, both STAs associate, STA1->AP ping is 10/10, STA1->STA2 ping is 10/10, STA2->STA1 ping is 10/10, STA1->STA2 TCP iperf reaches ~0.88 Mbits/sec client-side, and STA2->STA1 TCP iperf reaches ~1.35 Mbits/sec client-side. Fixes needed for this proof: map hwsim S1G RX status away from legacy-rate validation so mac80211 does not drop frames as `bad_legacy_rate`, and skip supported-rates IE enforcement for 802.11ah/S1G association in hostapd. Caveat: this validates the simulator S1G control/data path, not real S1G PHY modulation; the hostfs-backed shared medium can still hit its configured bound during long debug-heavy iperf and still needs cleanup/tuning. Evidence: `/tmp/hwsim-s1g-assoc-ap.log`, `/tmp/hwsim-s1g-assoc-sta1.log`, `/tmp/hwsim-s1g-assoc-sta2.log`. |

### 802.11ax / HE 6 GHz proof

Latest runtime evidence from `/tmp/hwsim-ax6-sae-entropy-ap.log` and
`/tmp/hwsim-ax6-sae-entropy-sta1.log`:

```text
Mode: IEEE 802.11a  Channel: 1  Frequency: 5955 MHz
nl80211: Set freq 5955 (... he_enabled=1 ... bandwidth=20 MHz ...)
wlan0: AP-ENABLED
wlan0: BSS: Add new id 0 BSSID 02:00:00:00:00:01 SSID 'nuttx-hwsim-ax6-sae' freq 5955
SAE: own commit-scalar ... e1 13 ...
SAE: Peer commit-scalar ... 84 a7 ...
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
AP -> STA: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP: 0.00-   5.05 sec   12845056 Bytes   20.35 Mbits/sec
```

Problem/fix chain:

- First valid 6 GHz SAE attempt still exposed only 2.4/5 GHz inventory because
  NuttX nl80211 forced unsplit wiphy dumps; 6 GHz bands are only reported in
  split dumps. `nl80211_dump_wiphy_parse()` now honors
  `NL80211_ATTR_SPLIT_WIPHY_DUMP`, and `genetlink_bridge.c` iterates dump
  callbacks until all split skb responses have been emitted.
- After 6 GHz became visible, SAE failed as a reflection attack because AP and
  STA generated identical commit scalars from synchronized simulator RNG state.
  The simulator OpenSSL bignum glue now mixes process-local data into
  `crypto_bignum_rand()` output after `random_get_bytes()`.

## P2P / Feature Matrix

| Feature | Current status | Evidence / next action |
| --- | --- | --- |
| P2P device discovery | PASS | Sim builds use UDP control interface; one-shot `wpa_cli ping/status` passes, `p2p-dev-wlan0` initializes, single-STA `p2p_find 5` / `p2p_stop_find` returns `OK`, and dual-STA discovery now passes. STA1 `p2p_peers` reports `42:00:00:00:00:03`; STA2 receives `P2P-DEVICE-FOUND 42:00:00:00:00:02` and `p2p_peers` reports `42:00:00:00:00:02`. Evidence: `/tmp/hwsim-p2p-listen-sta1-dwell.log`, `/tmp/hwsim-p2p-find-sta2-dwell.log`. |
| P2P GO/client | PASS | STA1 can become GO and STA2 can join as client. Both sides report `P2P-GROUP-STARTED`, STA2 completes `PTK=CCMP GTK=CCMP`, `p2p-wlan0-0` gets `192.168.77.1` / `192.168.77.2`, and sequential bidirectional traffic passes. Sequential proof: STA1->STA2 ping 10/10 and iperf ~20.81 Mbits/sec; STA2->STA1 ping 10/10 and iperf ~20.95 Mbits/sec. After the bounded hwsim medium fix, simultaneous bidirectional P2P iperf is also functional with explicit `-B` P2P bindings: GO->client ~1.07 Mbits/sec, client->GO ~0.41 Mbits/sec, and post-stress P2P ping remains 3/3 in both directions. Independent `nuttx-sim-p2p1`/`nuttx-sim-p2p2` objects are also proven: p2p1 becomes GO, p2p2 joins as client, both configure `p2p-wlan0-0`, and both directions ping. The latest independent-object soak extends the iperf window to 60 seconds in each direction and both clients return normally: GO->client ~1.34 Mbits/sec, client->GO ~1.32 Mbits/sec, and post-iperf ping is 3/3 both ways. Throughput tuning and repeated soak stress remain open. Evidence: `/tmp/hwsim-p2p-bind-sta1.log`, `/tmp/hwsim-p2p-bind-sta2.log`, `/tmp/hwsim-p2p1-go5.log`, `/tmp/hwsim-p2p2-client5.log`, `/tmp/hwsim-p2p-long-p2p1.log`, `/tmp/hwsim-p2p-long-p2p2.log`, `/tmp/hwsim-p2p-soak60-p2p1.log`, `/tmp/hwsim-p2p-soak60-p2p2.log`, and 2026-06-06 P2P simultaneous stress rerun. |
| WPS | PASS | AP + STA1 WPS PBC completes registrar/enrollee negotiation, provisions WPA2-PSK credentials, completes `PTK=CCMP GTK=CCMP`, connects, pings with 0% loss, and passes bidirectional iperf. A 2026-06-06 refresh also covers a negative short window: STA-side `wps_pbc` before AP PBC logs only `WPS-PBC-ACTIVE` and does not connect until AP PBC is started. The same refresh proves repeatability with a second fresh AP/STA PBC run, plus post-rerun ping 3/3. Evidence: `/tmp/hwsim-wps-ap.log`, `/tmp/hwsim-wps-sta1.log`, `/tmp/hwsim-wps-repeat1-ap.log`, `/tmp/hwsim-wps-repeat1-sta1.log`, `/tmp/hwsim-wps-repeat2-ap.log`, and `/tmp/hwsim-wps-repeat2-sta1.log`. |
| 802.11r FT | PASS WITH NUTTX NO-BRIDGE CAVEAT | AP + STA1 over WPA2-PSK/CCMP validates the FT AKM path with `wpa_key_mgmt=FT-PSK`, `mobility_domain=a1b2`, and `ft_psk_generate_local=1`. STA selects `KEY_MGMT FT/PSK`, parses MDIE/FTIE, derives PMK-R0 and PMK-R1 with KDF-SHA256, includes MDIE/FTIE in EAPOL-Key, reaches `key_mgmt=FT-PSK` / `wpa_state=COMPLETED`, sees BSS flags `[WPA2-FT/PSK-CCMP][ESS]`, pings AP 3/3, and runs STA-to-AP iperf at ~2.09 Mbits/sec. Multi-BSS FT-PSK advertises two BSSes (`02:00:00:00:10:00` and `02:00:00:00:10:01`), accepts `wpa_cli roam 02:00:00:00:10:01`, completes FT authentication and reassociation to the target BSS, and reports `CTRL-EVENT-CONNECTED` plus `bssid=02:00:00:00:10:01` / `key_mgmt=FT-PSK` / `wpa_state=COMPLETED`. Because the current NuttX sim path has no AP bridge/DS, the target BSS netdev must be configured too: `ifconfig ap1 192.168.201.1 netmask 255.255.255.0`. Without that, roam control passes but post-roam ping is 0/3. With `ap1` configured, post-roam data passes: first 5-packet ping after roam is 4/5 while ARP warms, the next ping is 5/5, and STA-to-AP iperf passes at ~1.98 Mbits/sec. Evidence: `/tmp/hwsim-ft-psk-ap.log`, `/tmp/hwsim-ft-psk-sta1.log`, `/tmp/hwsim-ft-roam2-ap.log`, `/tmp/hwsim-ft-roam2-sta1.log`, `/tmp/hwsim-ft-roam-ap1ip-ap.log`, `/tmp/hwsim-ft-roam-ap1ip-sta1.log`, `/tmp/hwsim-ft-roam-ap1ip-2ping-ap.log`, `/tmp/hwsim-ft-roam-ap1ip-2ping-sta1.log`, `tools/firmware/sim/hostapd-hwsim-ft-psk.conf`, `tools/firmware/sim/hostapd-hwsim-ft-psk-multibss.conf`, and `tools/firmware/sim/wpa_supplicant-hwsim-ft-psk-roam.conf`. |
| WNM BSS Transition Management | PASS | AP + STA1 over WPA2-PSK exchange BSS Transition Management Request/Response action frames. With no candidate BSS included, STA replies `status_code=1` and remains associated; post-WNM bidirectional ping/iperf still passes. Evidence: `/tmp/hwsim-wnm-ap.log`, `/tmp/hwsim-wnm-sta1.log`. |
| WNM Sleep Mode | PASS | AP + STA1 over WPA2-PSK exchange WNM-Sleep enter and exit action frames. The original no-TFS smoke proved `action=0 intval=5`, `action=1 intval=0`, GTK group rekey after exit, post-sleep ping 10/10, and bidirectional iperf. The 2026-06-06 non-empty TFS rerun adds local nl80211 `.wnm_oper` coverage: STA sends `tfs_req=5b020100`, AP logs `TFS Req IE(s) found`, stores the 4-byte TFS Request IE, generates a 4-byte TFS Response IE `5c 02 01 00`, STA logs `TFS Resp IE(s) found` and accepts that response, WNM Sleep exit succeeds, and post-exit ping remains 3/3 in both directions. This proves the management-frame/non-empty-IE plumbing; real TFS classifier/filter policy enforcement is still not implemented. Evidence: `/tmp/hwsim-wnm-sleep-ap.log`, `/tmp/hwsim-wnm-sleep-sta1.log`, `/tmp/hwsim-wnm-tfs-ap-fix.log`, and `/tmp/hwsim-wnm-tfs-sta1-fix.log`. |
| HS20/interworking | PASS | AP + STA1 over WPA2-Enterprise EAP-PSK/CCMP validates HS20/interworking. Open HS20 was rejected by hostapd because Hotspot 2.0 requires WPA2-Enterprise/CCMP; the passing config layers HS20 on the existing internal EAP-PSK path. STA status reports `hs20=3`, BSS flags include `[HS20]`, `anqp_get` returns `GAS-QUERY-DONE ... result=SUCCESS` and `RX-ANQP` for capability list, venue name, IP address availability, NAI realm, and domain name. `hs20_anqp_get` returns `RX-HS20-ANQP` for HS capability list, operator friendly name, WAN metrics, and connection capability. Cached BSS output includes `anqp_venue_name`, `anqp_nai_realm`, `anqp_domain_name`, `hs20_operator_friendly_name`, `hs20_wan_metrics`, and `hs20_connection_capability`. Data path remains up: ping 3/3 and STA->AP iperf ~0.67 Mbits/sec. Evidence: `/tmp/hwsim-hs20-ap.log`, `/tmp/hwsim-hs20-sta1.log`. OSU/provider provisioning remains separate future coverage. |
| DCM | SIM RUNTIME PASS | `CONFIG_WL_NUTTX_HWSIM_DCM_PROOF` now advertises HE DCM BPSK Tx/Rx and max RU 484 in the hwsim HE capability blocks. Runtime RX status injection is gated by `/h/hwsim-dcm-proof` so non-DCM tests are not mislabeled. AP+STA1 11ax channel 6 runtime proof shows `dcm-proof: rx status ... encoding=HE ... he_dcm=1` on data frames, STA->AP ping 3/3, STA->AP iperf ~18.14 Mbits/sec, and AP->STA iperf ~18.18 Mbits/sec. This proves the simulator capability/RX-status path; real PHY DCM modulation is outside hwsim scope. |
| AMPDU | PASS | Source audit confirms `mac80211_hwsim_linux.c` sets `IEEE80211_HW_AMPDU_AGGREGATION`, exposes `mac80211_hwsim_ampdu_action()`, and mac80211 includes ADDBA/DELBA TX/RX paths plus `ieee80211_start_tx_ba_session()`. Dedicated HT20 + WMM AP/STA runtime now proves both directions: `aggr_check start`, `start_tx_ba_session request/queued`, `send ADDBA_REQ`, `rx ADDBA_REQ`, `send ADDBA_RESP`, `rx ADDBA_RESP status=0`, `tx operational`, and hwsim `ampdu_action` actions 0/2/6 are captured in `/tmp/hwsim-ampdu-proof-ap.log` and `/tmp/hwsim-ampdu-proof-sta1.log`. Data traffic also passes: ping 3/3, STA->AP iperf ~17.77 Mbits/sec, AP->STA iperf ~17.94 Mbits/sec. |
| AMSDU | PASS | `CONFIG_WL_NUTTX_HWSIM_AMSDU_PROOF` enables hwsim `SUPPORTS_AMSDU_IN_AMPDU` and `TX_AMSDU` for validation, and AP+STA1 HT/WMM runtime negotiates BA sessions with `amsdu=1` in ADDBA response and TX operational state. Actual software A-MSDU construction is now proven by pausing AP TX queues with `hwsim_tm -i wlan0 stop_queues`, running STA->AP TCP iperf, then waking AP queues. AP logs show `amsdu-proof: aggregate` growing from subframes 2 through 8, with `total_len=568` and `data_len=476`; post-wake data traffic remains healthy: STA->AP ping 3/3 and iperf ~17.65 Mbits/sec. Evidence: `/tmp/hwsim-amsdu-backlog-ap.log`, `/tmp/hwsim-amsdu-backlog-sta1.log`. |
| TWT | TESTMODE SERVICE PERIOD PASS / AUTO-SCHEDULER DATA SMOKE PASS / TCP TIMING TUNING GAP | S1G capability has TWT request/respond bits and hwsim extended capabilities include TWT responder support. HE AP/STA with `he_twt_responder=1` associates successfully. The sim Wi-Fi build enables upstream `CONFIG_TESTING_OPTIONS`, so `wpa_cli twt_setup` and `wpa_cli twt_teardown` return `OK`. AP-side and non-AP nl80211 now register S1G Action frames under sim testing options, and hostapd has a minimal sim-only TWT responder. Runtime proof shows AP RX of setup action 6, AP TWT Accept response TX with ACK, STA RX_MGMT delivery of the AP action frame, STA-side parsing of the Accept response (`setup_cmd=4`, `req_type=0x2878`, `dialog=1`), and teardown action 7 RX. Testmode-controlled service-period proof adds `hwsim_tm get_twt_sp` / `set_twt_sp <0|1>`: with `ps=1` and `twt_sp=0`, AP->STA ping is 0/3 and AP logs `buffer unicast`; opening the SP emits PM=0 NullFunc `fc=0x0148`, AP logs `sta wake` and `wake deliver`, AP->STA ping passes 3/3, AP->STA iperf passes at ~1.20 Mbits/sec, closing the SP reports `twt_sp=0`, waking reports `ps=0`, TWT teardown reaches AP, and final STA->AP ping passes 3/3. Automatic scheduler proof accepts the observed TWT action category 23 in addition to the imported `WLAN_CATEGORY_S1G` value 22, parses the negotiated Accept frame on STA RX, and installs a periodic hwsim service-period timer. The negotiated `dialog=1 exponent=1 mantissa=50000 min_twt=255` timing was too tight for the NuttX sim/hostfs medium at `interval_jiffies=10 wake_jiffies=6`, so the simulator now clamps TWT proof timing to at least 1 s interval / 500 ms wake (`interval_jiffies=100 wake_jiffies=50`). The automatic open-SP RX gate also now accepts broadcast/multicast frames, fixing AP->STA ARP under TWT. Latest AP+STA1 run proves setup accept, automatic period start/end cycles, STA->AP ping 2/2 after one initial ARP retry, AP->STA ping 3/3 after ARP/path warm-up, STA->AP iperf `294912 Bytes` over 6.02 s at ~0.39 Mbits/sec, AP->STA iperf `327680 Bytes` over 6.02 s at ~0.44 Mbits/sec, TX teardown local scheduler stop (`auto disabled reason=tx-teardown restore_ps=1`), and AP teardown RX. Caveats: the first AP-originated ARP before STA traffic can still miss the service-period window, AP->STA TCP can fill the 512-frame sleep buffer, and the hostfs hwsim medium can hit the configured bounded-medium limit under this debug-heavy long flow, so tighter timing, queue release, and medium/performance tuning remain follow-up work. Evidence: `/tmp/hwsim-build-twtsp2-ap.log`, `/tmp/hwsim-build-twtsp2-sta1.log`, `/tmp/hwsim-twtsp2-ap.log`, `/tmp/hwsim-twtsp2-sta1.log`, `/tmp/hwsim-build-twt-auto4-ap.log`, `/tmp/hwsim-build-twt-auto4-sta1.log`, `/tmp/hwsim-build-twt-auto5-sta1.log`, `/tmp/hwsim-build-twt-auto7-ap.log`, `/tmp/hwsim-build-twt-auto8-sta1.log`, `/tmp/hwsim-twt-auto8-ap.log`, `/tmp/hwsim-twt-auto8-sta1.log`, `/tmp/hwsim-twt-iperf1-ap.log`, and `/tmp/hwsim-twt-iperf1-sta1.log`. |
| TWT current status | TESTMODE SERVICE PERIOD PASS / AUTO-SCHEDULER DATA SMOKE PASS / TCP TIMING TUNING GAP | Current evidence covers HE AP/STA association with TWT responder enabled, `wpa_cli twt_setup` / `twt_teardown`, AP-side setup action RX, AP Accept response TX, STA RX_MGMT delivery and Accept parsing, teardown, manual SP closed/open behavior, automatic SP open/close timer installation, bidirectional ping, and bidirectional iperf smoke. Latest automatic run passes AP->STA ping 3/3 after ARP warm-up, STA->AP ping, STA->AP iperf at ~0.39 Mbits/sec, AP->STA iperf at ~0.44 Mbits/sec, TX-side scheduler teardown, and AP teardown RX. Remaining work is timing/performance tuning under longer TCP windows and debug-heavy hostfs medium limits, not missing TWT control-path coverage. Evidence: `/tmp/hwsim-twt-iperf1-ap.log`, `/tmp/hwsim-twt-iperf1-sta1.log`, `/tmp/hwsim-twt-auto8-ap.log`, and `/tmp/hwsim-twt-auto8-sta1.log`. |
| Static PS mode | PASS | Standard userspace control now reaches `NL80211_CMD_SET_POWER_SAVE`: `wpa_cli -i wlan0 p2p_set ps 1` and `ps 0` both return `OK`, nl80211 cmd 61 completes with `ret=0`, and post-toggle STA-to-AP ping passes 3/3 in both PS-on and PS-off states. hwsim private testmode is now build- and runtime-proven for the PS latch: AP/STA1/STA2 hwsim defconfigs enable `CONFIG_NL80211_TESTMODE` and `CONFIG_EXAMPLES_HWSIM_TESTMODE`, `hwsim_tm` registers in the AP/STA builds, and a STA sim run proved `get_ps -> 0`, `set_ps 1`, `get_ps -> 1`, `set_ps 0`, `get_ps -> 0` through `NL80211_CMD_TESTMODE`. hwsim private queue stop/wake is proven for ICMP and TCP long-flow gating over the full hostapd/wpa_supplicant path: baseline ping passes 3/3, `hwsim_tm stop_queues` makes ICMP drop to 0/3 and makes an active STA->AP iperf interval fall to 0.00 Mbits/sec, then `hwsim_tm wake_queues` restores ping to 3/3 and TCP throughput to ~0.96 Mbits/sec. Nullfunc and PS-Poll frame emission is now proven over the shared hwsim medium: `hwsim_tm set_ps 1` emits PM=1 nullfunc (`48 11`), `hwsim_tm poll` emits PS-Poll (`a4 10`), and `hwsim_tm set_ps 0` emits PM=0 nullfunc (`48 01`); pings before PS, after manual poll/wake, and after auto-PS latch/wake all pass 3/3. AP-side sleeping-STA buffered-frame delivery is now proven: AP logs `sta sleep`, `buffer unicast`, TIM set, PS-Poll `fc=0x10a4`, `deliver response`, `allow buffered`, and TIM clear; STA logs `hwsim manual poll rx one`, `get_ps -> ps=1` after the poll, wake to `ps=0`, and post-wake ping 3/3. Dynamic idle-timeout and AP-link-PS semantics are covered separately in the Dynamic PS row. Evidence: `/tmp/hwsim-ps-ap.log`, `/tmp/hwsim-ps-sta1.log`, interactive `hwsim_tm` smoke on `build/nuttx-sim-sta2`, AP/STA hostapd/wpa_supplicant queue stop/wake smoke on 2026-06-05, iperf long-flow queue stop/wake proof on 2026-06-06, and `/tmp/hwsim-pspoll-ap.log` / `/tmp/hwsim-pspoll-sta1.log` plus `hwsim-frames.bin` frame-control scan on 2026-06-06; AP-side buffered-frame/PS-Poll delivery proof in `/tmp/hwsim-psbuffer5-ap.log` / `/tmp/hwsim-psbuffer5-sta1.log`; rebuild logs `/tmp/hwsim-build-psproof-ap6.log` and `/tmp/hwsim-build-psproof-sta1-6.log`. |
| Dynamic PS mode | MLO DATA SMOKE PASS / AUTO-PS BIDIRECTIONAL TCP FUNCTIONAL PASS / DYNPS IDLE-TIMEOUT PASS / AP-LINK PS PASS / AUTO-PS THROUGHPUT TUNING GAP | hwsim can set `SUPPORTS_DYNAMIC_PS`, `AP_LINK_PS`, and related PS flags when MLO is enabled. Dedicated `hwsim_dynps_ap` and `hwsim_dynps_sta1` configs enable MLO without changing the baseline AP/STA/P2P objects; both build, boot to NSH, associate over open auth, and pass bidirectional ping/iperf. Latest MLO-enabled data smoke: AP sees `AP-STA-CONNECTED`, AP->STA ping 3/3 and iperf ~0.96 Mbits/sec, STA->AP ping 3/3 and iperf ~0.98 Mbits/sec. `PS_AUTO_POLL` now has a NuttX hwsim delayed-work implementation that sends periodic PS-Poll and gates STA RX with a finite receive budget instead of accepting all frames; runtime proof for `hwsim_tm set_ps 2` emits PM=1/PM=0 NullFunc frames, automatic PS-Poll frames, AP->STA ping 3/3, and STA->AP ping 3/3. Auto-poll service periods use bulk buffered-frame release. A failed 5 ms tuning attempt exposed that `msecs_to_jiffies(5)` can become 0 ticks in NuttX sim, causing an auto-poll busy loop; the scheduler now clamps auto-poll delayed work to at least one tick. A later min-1 run with release=256/RX budget=4096 still stalled at `0.26 Mbits/sec` and sampled `queue=64`, proving the 64-frame sleeping-STA buffer was a bottleneck. The current sim validation raises `STA_MAX_TX_BUFFER` to 512 and `TOTAL_MAX_TX_BUFFER` to 2048 under `CONFIG_ARCH_SIM`, uses release=512/RX budget=8192, and improves AP->STA auto-PS TCP to a sustained two-interval run: `0.00-   3.01 sec     409600 Bytes    1.09 Mbits/sec`, `3.01-   6.02 sec     573440 Bytes    1.52 Mbits/sec`, total `983040 Bytes    1.31 Mbits/sec`. The reverse STA->AP proof reaches `2670592 Bytes` over 6.02 s at ~3.55 Mbits/sec. Dynamic idle-timeout is proven with `hwsim_tm set_ps 4`: idle sleep occurs after 500 ms, AP->STA ping is rejected while sleeping, STA TX wakes the link, and `set_ps 0` restores AP->STA ping to 3/3. AP-link-PS is now proven on the MLO/dynps objects: the AP reports `ap-link-ps-proof: transition ... start=1 ret=0`, buffers AP->STA unicast while the STA sleeps, reports `ap-link-ps-proof: pspoll`, delivers the buffered response, then reports `transition ... start=0 ret=0` and post-wake AP->STA/STA->AP ping both pass 3/3. Build/run caveat: hwsim role images must be built sequentially in one NuttX tree because concurrent role builds can race generated `.config/include` files (`config.h.tmp`). Remaining gap: auto-PS TCP is functional but still below non-PS throughput, so service-period/sleep-buffer/TCP ACK tuning is not finished. Evidence: `/tmp/hwsim-autops-bigbuf-ap.log`, `/tmp/hwsim-autops-bigbuf-sta1.log`, `/tmp/hwsim-autops-bigbuf-sta2ap-ap.log`, `/tmp/hwsim-autops-bigbuf-sta2ap-sta1.log`, `/tmp/hwsim-dynps-idle-ap-rerun.log`, `/tmp/hwsim-dynps-idle-sta1-rerun.log`, `/tmp/hwsim-aplinkps2-ap.log`, and `/tmp/hwsim-aplinkps2-sta1.log`. |
| Dynamic PS auto-PS TCP current status | BIDIRECTIONAL FUNCTIONAL PASS / THROUGHPUT TUNING GAP | Latest evidence upgrades auto-PS TCP from one-direction partial coverage to bidirectional functional coverage. AP->STA auto-PS iperf reaches `983040 Bytes` over 6.02 s at ~1.31 Mbits/sec with normal peer close. STA->AP auto-PS iperf reaches `2670592 Bytes` over 6.02 s at ~3.55 Mbits/sec. Both directions keep ping healthy while `hwsim_tm set_ps 2` is active, and the big-buffer runs show no sampled `queue=512` or `queue=64` full-buffer event and no `tcp server recv error`. Throughput remains lower than non-PS runs, so the remaining item is performance/timing tuning rather than missing functional coverage. Evidence: `/tmp/hwsim-autops-bigbuf-ap.log`, `/tmp/hwsim-autops-bigbuf-sta1.log`, `/tmp/hwsim-autops-bigbuf-sta2ap-ap.log`, and `/tmp/hwsim-autops-bigbuf-sta2ap-sta1.log`. |

Dynamic PS auto-PS reverse TCP addendum:

- Latest reverse proof uses `/tmp/hwsim-autops-bigbuf-sta2ap-ap.log` and
  `/tmp/hwsim-autops-bigbuf-sta2ap-sta1.log` with `hwsim_tm set_ps 2`,
  release=512, and RX budget=8192. The STA remains in auto-PS, baseline and
  auto-PS STA->AP ping both pass 3/3, then STA->AP iperf reports
  `0.00-   6.02 sec    2670592 Bytes    3.55 Mbits/sec`. The AP iperf server
  records `0.00-   3.01 sec    1499086 Bytes    3.98 Mbits/sec` and
  `3.01-   6.02 sec     981130 Bytes    2.61 Mbits/sec`, then closes normally
  by peer `192.168.203.2:26062`.
- Counters from the same logs: 99 non-empty bulk releases, 66 sampled auto-poll
  TX budget logs, no sampled `queue=512` or `queue=64` full-buffer event, and
  no `tcp server recv error`. This upgrades auto-PS TCP evidence to
  bidirectional functional coverage while keeping throughput tuning open.

### Aggregation inventory audit

Current source evidence:

```text
drivers/wireless/virtual/mac80211_hwsim_linux.c:
  ieee80211_hw_set(hw, AMPDU_AGGREGATION)
  .ampdu_action = mac80211_hwsim_ampdu_action

wireless/ieee80211/mac80211/agg-tx.c:
  ieee80211_start_tx_ba_session()
  WLAN_ACTION_ADDBA_REQ

wireless/ieee80211/mac80211/agg-rx.c:
  WLAN_ACTION_ADDBA_RESP
```

Latest runtime audit:

- HT20 AP+STA test with WMM enabled exposes station HT capabilities and passes
  bidirectional iperf.
- Earlier `/tmp/hwsim-ampdu-ap.log` and `/tmp/hwsim-ampdu-sta1.log` traffic
  proved HT/WMM data flow but contained no ADDBA/DELBA/BlockAck/session-start
  text. That was a logging/proof gap, not enough evidence for `PASS`.
- The sim hwsim/mac80211 path now has a gated
  `CONFIG_WL_NUTTX_HWSIM_AMPDU_PROOF` trace for low-volume BA proof. NuttX
  `printf` does not support Linux `%pM`, so the proof trace uses an explicit
  `%02x:%02x:%02x:%02x:%02x:%02x` MAC formatter.
- Latest evidence from `/tmp/hwsim-ampdu-proof-ap.log` and
  `/tmp/hwsim-ampdu-proof-sta1.log`:

  ```text
  wlan0: AP-ENABLED
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  ampdu-proof: aggr_check start tid=0 sta=02:00:00:00:00:01 queue=2 priority=0
  ampdu-proof: start_tx_ba_session request sta=02:00:00:00:00:01 tid=0 timeout=0 vif_type=2
  ampdu-proof: start_tx_ba_session queued sta=02:00:00:00:00:01 tid=0 token=1
  ampdu-proof: hwsim ampdu_action action=2 tid=0 sta=02:00:00:00:00:01 buf=0 amsdu=0 ssn=1 timeout=0
  ampdu-proof: send ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 buf=64 timeout=0
  ampdu-proof: rx ADDBA_REQ sta=02:00:00:00:00:01 tid=0 token=1 ssn=1 policy=1 buf=64 timeout=0
  ampdu-proof: hwsim ampdu_action action=0 tid=0 sta=02:00:00:00:00:01 buf=64 amsdu=0 ssn=1 timeout=0
  ampdu-proof: send ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 policy=1 buf=64 timeout=0 amsdu=0
  ampdu-proof: rx ADDBA_RESP sta=02:00:00:00:00:01 tid=0 token=1 status=0 raw_buf=64 amsdu=0 timeout=0
  ampdu-proof: tx operational sta=02:00:00:00:00:01 tid=0 buf=64 amsdu=0
  ampdu-proof: hwsim ampdu_action action=6 tid=0 sta=02:00:00:00:00:01 buf=64 amsdu=0 ssn=0 timeout=0
  ping -c 3 192.168.201.1: 3 packets transmitted, 3 received, 0% packet loss
  STA -> AP iperf: 0.00-   6.02 sec   13369344 Bytes   17.77 Mbits/sec
  AP -> STA iperf: 0.00-   6.02 sec   13500416 Bytes   17.94 Mbits/sec
  ```

- AP-side proof also shows the same BA state machine toward STA
  `02:00:00:00:00:02` with `vif_type=3`, proving aggregation setup in both
  traffic directions. Current status is `PASS` for A-MPDU/ADDBA.
- A-MSDU was then moved from pure inventory to runtime `PASS`. The validation
  build advertises `SUPPORTS_AMSDU_IN_AMPDU` and `TX_AMSDU`, and
  `/tmp/hwsim-amsdu-backlog-ap.log` plus `/tmp/hwsim-amsdu-backlog-sta1.log`
  show ADDBA response and TX operational state with `amsdu=1`. To create the
  required TXQ backlog window, AP queues were paused with
  `hwsim_tm -i wlan0 stop_queues` while STA sent TCP iperf, then resumed with
  `hwsim_tm -i wlan0 wake_queues`. The AP proof logs contain
  `amsdu-proof: aggregate` events growing from subframes 2 through 8
  (`total_len=568`, `data_len=476`), and STA->AP data remains healthy:
  `ping` 3/3 and iperf ~17.65 Mbits/sec.

### DCM capability and RX-status proof

Validation source/config evidence:

```text
CONFIG_WL_NUTTX_HWSIM_DCM_PROOF=y

drivers/wireless/virtual/mac80211_hwsim_linux.c:
  HWSIM_HE_DCM_PHY_CAPS advertises HE DCM BPSK Tx/Rx and max RU 484
  /h/hwsim-dcm-proof gates runtime RX status injection
  dcm-proof: rx status ... encoding=HE ... he_dcm=1

wireless/ieee80211/cfg80211/nl80211.c:
  NL80211_RATE_INFO_HE_DCM can be emitted when rate_info->he_dcm is set
```

Runtime proof from the 2026-06-06 AP+STA1 11ax channel 6 run:

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

dcm-proof: rx status path=direct freq=2437 bw=0 encoding=HE mcs=0 nss=1 he_gi=0 he_dcm=1
dcm-proof: rx status path=shared freq=2437 bw=0 encoding=HE mcs=0 nss=1 he_gi=0 he_dcm=1

STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec   13647872 Bytes   18.14 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec   13680640 Bytes   18.18 Mbits/sec
```

### TWT control/action audit

Source/config evidence:

```text
tools/firmware/sim/hostapd-ax.conf:
  he_twt_responder=1

apps/wireless/wifi/wpa_hostapd_sources.mk:
  CSRCS += wpa_supplicant-2.11/wpa_supplicant/twt.c
  CONFIG_ARCH_SIM -> WIFI_WPA_HOSTAPD_CFLAGS += -DCONFIG_TESTING_OPTIONS

wpa_supplicant-2.11/wpa_supplicant/twt.c:
  wpas_twt_send_setup()
  wpas_twt_send_teardown()
  #ifdef CONFIG_TESTING_OPTIONS

wpa_supplicant-2.11/src/drivers/driver_nl80211.c:
  CONFIG_TESTING_OPTIONS -> nl80211_action_subscribe_ap() registers
  S1G Action category 0x17 for the sim TWT responder path
  CONFIG_TESTING_OPTIONS -> nl80211_mgmt_subscribe_non_ap() registers
  S1G Action category 0x17 so the STA receives AP Accept responses

wpa_supplicant-2.11/src/ap/ieee802_11.c:
  CONFIG_TESTING_OPTIONS -> handle_twt_action() handles S1G TWT setup
  and teardown in AP mode

wpa_supplicant-2.11/wpa_supplicant/twt.c:
  CONFIG_TESTING_OPTIONS -> wpas_twt_rx_action() parses STA-side S1G TWT
  setup responses and teardown frames for sim validation logs
```

Runtime audit:

- `wpa-ax-twt.conf` adds `ctrl_interface=udp:9877` and
  `disable_scan_offload=1` so `wpa_cli` can reach the STA control interface in
  the AX/TWT run.
- First HE AP/STA run proved the old control-command gap:

  ```text
  /tmp/hwsim-twt2-ap.log:
  nl80211: Set freq 2437 (... he_enabled=1 ...)
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
  * he_capab - hexdump(len=21): 01 78 c8 1a 40 00 02 bf ce ...

  /tmp/hwsim-twt2-sta1.log:
  ctrl_interface='udp:9877'
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  wpa_cli -i wlan0 ping -> PONG
  wpa_cli -i wlan0 twt_setup -> UNKNOWN COMMAND
  wpa_cli -i wlan0 twt_teardown -> UNKNOWN COMMAND
  ```

- After enabling sim-only `CONFIG_TESTING_OPTIONS`, the second run reached
  control command handling and S1G Action frame transmission:

  ```text
  /tmp/hwsim-twt3-sta1.log:
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

- After registering S1G Action frames on the AP path and adding the minimal
  sim-only AP responder, the next run reached setup request, AP Accept response,
  teardown, and post-TWT ping:

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
  nl80211: Frame TX status: cookie=0x1 (match) (ack=1)
  wpa_cli -i wlan0 twt_teardown -> OK
  nl80211: Frame TX status: cookie=0x2 (match) (ack=1)
  ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
  ```

- After adding non-AP S1G Action registration and STA-side TWT response parsing,
  the latest run proves supplicant-level delivery and field parsing of the AP
  Accept response:

  ```text
  /tmp/hwsim-twt-rx2-ap.log:
  nl80211: Register frame type=0xd0 (WLAN_FC_STYPE_ACTION) ... match=17
  TWT: AP accept setup from 02:00:00:00:00:02 dialog=1 req_type=0x2878 len=44
  TWT: AP teardown from 02:00:00:00:00:02 flags=0x01

  /tmp/hwsim-twt-rx2-sta1.log:
  nl80211: Register frame type=0xd0 (WLAN_FC_STYPE_ACTION) ... match=17
  wlan0: Event RX_MGMT (18) received
  wlan0: Received Action frame: SA=02:00:00:00:00:01 Category=23 DataLen=19 freq=0 MHz
  wlan0: TWT: STA parsed setup response from 02:00:00:00:00:01 dialog=1 control=0x10 req_type=0x2878 setup_cmd=4 requestor=0 trigger=1 implicit=1 flow_type=1 flow_id=0 exponent=10 protection=0 twt=0 min_twt=255 mantissa=8192 channel=0
  ping -c 3 192.168.201.1 -> 3 packets transmitted, 3 received, 0% packet loss
  ```

- After adding hwsim testmode TWT service-period control, the latest run proves
  manual SP gating tied to the negotiated setup/accept path:

  ```text
  /tmp/hwsim-build-twtsp2-ap.log:
  ./tools/firmware/sim/build-ap.sh -j8 -> rc=0
  ../build/nuttx-sim-ap -> size 23151928

  /tmp/hwsim-build-twtsp2-sta1.log:
  ./tools/firmware/sim/build-sta1.sh -j8 -> rc=0
  ../build/nuttx-sim-sta1 -> size 27437360

  /tmp/hwsim-twtsp2-sta1.log:
  wifi_generation=6
  wpa_state=COMPLETED
  wlan0: TWT: STA parsed setup response from 02:00:00:00:00:01 dialog=1 control=0x10 req_type=0x2878 setup_cmd=4 requestor=0 trigger=1 implicit=1 flow_type=1 flow_id=0 exponent=10 protection=0 twt=0 min_twt=255 mantissa=8192 channel=0
  hwsim_tm set_ps 1 -> hwsim_tm: ps=1
  hwsim_tm get_twt_sp -> hwsim_tm: twt_sp=0

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
  hwsim_tm get_twt_sp -> hwsim_tm: twt_sp=1

  /tmp/hwsim-twtsp2-ap.log:
  fc=0x0148
  ps-proof: sta wake sta=02:00:00:00:00:02 aid=1 driver_ps=0
  ps-proof: wake deliver sta=02:00:00:00:00:02 aid=1 filtered=0 buffered=3 total=3 num_sta_ps=0
  3 packets transmitted, 3 received, 0% packet loss
  0.00-   6.02 sec     901120 Bytes    1.20 Mbits/sec
  iperf exit

  /tmp/hwsim-twtsp2-sta1.log:
  twt-proof: hwsim service period closed
  hwsim_tm get_twt_sp -> hwsim_tm: twt_sp=0
  hwsim_tm get_ps -> hwsim_tm: ps=0
  3 packets transmitted, 3 received, 0% packet loss

  /tmp/hwsim-twtsp2-ap.log:
  TWT: AP teardown from 02:00:00:00:00:02 flags=0x01
  ```

- Problem fixed during service-period bring-up: `set_twt_sp 1` originally
  emitted a no-PS NullFunc, but the hwsim TX path later forced the PM bit on
  because `ps != PS_DISABLED`, producing another `fc=0x1148` sleep indication.
  The fix skips that PM-bit forcing while `twt_sp_open` is true, so SP open
  emits `fc=0x0148`, the AP wakes the STA state, and buffered frames are
  delivered.

- Follow-up automatic-scheduler smoke proof now parses the AP Accept frame in
  the STA hwsim RX path and accepts the observed TWT action category 23 as well
  as the imported `WLAN_CATEGORY_S1G` value 22. With `wpa_cli -i wlan0
  twt_setup dialog=1 exponent=1 mantissa=50000 min_twt=255`, AP logs `TWT: AP
  accept setup ... req_type=0x0478`, STA logs `TWT: STA parsed setup response
  ... setup_cmd=4 ... req_type=0x0478`, and installs the automatic scheduler.
  The initial negotiated `interval_jiffies=10 wake_jiffies=6` was too tight for
  NuttX sim/hostfs polling, so the simulator now clamps proof timing to at
  least a 1 s interval and 500 ms wake window. Latest run installs
  `interval_jiffies=100 wake_jiffies=50` and repeatedly logs service-period
  open/close transitions.

- Current status is `TESTMODE SERVICE PERIOD PASS / AUTO-SCHEDULER DATA SMOKE
  PASS`. This proves the sim control path, AP and STA S1G Action registration,
  setup request, AP Accept response frame, STA RX_MGMT delivery, STA-side Accept
  field parsing, teardown reception, manual service-period off-SP buffering,
  open-SP wake/deliver, AP->STA ping/iperf, post-teardown data plane, automatic
  service-period timer installation, and automatic service-period ping data.
  Latest run passes AP->STA `ping -c 5` 5/5 and STA->AP `ping -c 5` 5/5; TX
  teardown disables the sender-side scheduler (`auto disabled
  reason=tx-teardown restore_ps=1`), AP receives teardown, and post-teardown
  STA->AP `ping -c 3` passes 3/3. Automatic TWT long-flow iperf and tighter
  service-period timing/performance tuning remain pending.

### P2P GO/client proof

Latest runtime evidence from `/tmp/hwsim-p2p-bind-sta1.log` and
`/tmp/hwsim-p2p-bind-sta2.log`:

```text
P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-YG" freq=2437 go_dev_addr=42:00:00:00:00:02
P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-YG" freq=2437 go_dev_addr=42:00:00:00:00:02
p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:03 p2p_dev_addr=42:00:00:00:00:03
p2p-wlan0-0: WPA: Key negotiation completed with 52:00:00:00:00:02 [PTK=CCMP GTK=CCMP]
p2p-wlan0-0: CTRL-EVENT-CONNECTED - Connection to 52:00:00:00:00:02 completed
inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0
inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0
STA1 -> STA2: 10 packets transmitted, 10 received, 0% packet loss
STA1 -> STA2: 0.00-   6.04 sec   15712256 Bytes   20.81 Mbits/sec
STA2 -> STA1: 10 packets transmitted, 10 received, 0% packet loss
STA2 -> STA1: 0.00-   6.02 sec   15761408 Bytes   20.95 Mbits/sec
```

Independent role-object evidence from `/tmp/hwsim-p2p1-go5.log` and
`/tmp/hwsim-p2p2-client5.log`:

```text
P2P-GO-NEG-SUCCESS role=GO freq=2437 peer_dev=42:00:00:00:00:15
P2P-GO-NEG-SUCCESS role=client freq=2437 peer_dev=42:00:00:00:00:14
P2P-GROUP-STARTED p2p-wlan0-0 GO ssid="DIRECT-QH" freq=2437 go_dev_addr=42:00:00:00:00:14
P2P-GROUP-STARTED p2p-wlan0-0 client ssid="DIRECT-QH" freq=2437 go_dev_addr=42:00:00:00:00:14
p2p-wlan0-0: AP-STA-CONNECTED 52:00:00:00:00:15 p2p_dev_addr=42:00:00:00:00:15
p2p-wlan0-0: WPA: Key negotiation completed with 52:00:00:00:00:14 [PTK=CCMP GTK=CCMP]
GO p2p-wlan0-0: inet addr:192.168.77.1 DRaddr:192.168.77.1 Mask:255.255.255.0
client p2p-wlan0-0: inet addr:192.168.77.2 DRaddr:192.168.77.1 Mask:255.255.255.0
GO -> client: 5 packets transmitted, 5 received, 0% packet loss
GO -> client: 0.00-   6.02 sec     507904 Bytes    0.67 Mbits/sec
client -> GO: 5 packets transmitted, 5 received, 0% packet loss
client -> GO: 0.00-   6.02 sec     720896 Bytes    0.96 Mbits/sec
```

Problem fixed during GO/client bring-up:

- Symptom: GO-side EAPOL TX previously failed with `Unknown error 19`
  (`ENODEV`) after the P2P group interface was created.
- Root cause: wpa_supplicant sent PF_PACKET EAPOL frames with the Linux compat
  ifindex, while the NuttX packet socket path expects the native lower-netdev
  ifindex.
- Fix: `ieee80211_linux_if_indextonative()` translates compat ifindex to the
  native ifindex before `sendto()` in the NuttX nl80211 driver path.
- Symptom: after group formation, the first P2P traffic run selected
  `192.168.201.x` as the NuttX iperf client source even though the P2P peer was
  on `192.168.77.0/24`, and TCP connect failed with error 111.
- Root cause: `iperf -B <ip>` set `cfg.sip`, but the TCP/UDP client sockets did
  not actually bind to that local address. In a multi-interface STA process,
  client traffic could therefore use the base `wlan0` source instead of
  `p2p-wlan0-0`.
- Fix: `apps/netutils/iperf/iperf.c` now binds TCP and UDP client sockets to
  `cfg.sip` when provided. The client bind uses port 0 so a local iperf server
  can still listen on port 5001 in the same NuttX instance during bidirectional
  test setup.
- Current result: EAPOL TX logs show `ifindex linux=4 native=3`, WPS/EAPOL can
  progress, the group forms, and P2P GO/client data traffic passes when iperf is
  run with explicit P2P bindings, for example `iperf -c 192.168.77.2 -B
  192.168.77.1 -t 6`.
- Independent role-object problem: the first `nuttx-sim-p2p1`/`nuttx-sim-p2p2`
  run used shorthand NSH commands such as `start`, `status`, and `p2p_connect`
  that were never installed as commands in the sim image, so the run only
  proved command invocation failure. The working run uses explicit
  `wifi_sta_demo`, `wpa_supplicant`, and `wpa_cli -i wlan0 ...` commands after
  mounting hostfs.
- Independent role-object problem: p2p1 in pure `p2p_listen` learned p2p2 only
  from a Probe Request and later failed `p2p_connect` with `Cannot connect to
  unknown P2P Device`. The working sequence runs `p2p_find` on both p2p1 and
  p2p2 long enough for full P2P Device Info and config methods to be learned
  before starting GO negotiation.
- Historical caveat: one earlier p2p2 iperf client printed the final 6-second
  summary but did not return to NSH before cleanup. The later 15-second
  independent-object rerun returned both iperf clients normally, so the active
  remaining P2P work is longer soak testing and throughput tuning.

### WPS PBC proof

Config files:

```text
hostapd-hwsim-wps.conf
wpa_supplicant-hwsim-wps.conf
```

Latest runtime evidence from `/tmp/hwsim-wps-ap.log` and
`/tmp/hwsim-wps-sta1.log`:

```text
hostapd_cli -i wlan0 ping -> PONG
wpa_cli -i wlan0 ping -> PONG
wlan0: WPS-PBC-ACTIVE
WPS: Probe Request for PBC received from 02:00:00:00:00:02
WPS: Negotiation completed successfully
wlan0: WPS-REG-SUCCESS 02:00:00:00:00:02 ed0bfab2-2afe-50f2-9fcb-5dc0a86a824e
wlan0: WPS-SUCCESS
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA -> AP: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP: 0.00-   6.02 sec    1097728 Bytes    1.46 Mbits/sec
AP -> STA: 10 packets transmitted, 10 received, 0% packet loss
AP -> STA: 0.00-   6.02 sec    1245184 Bytes    1.65 Mbits/sec
```

The WPS run used `-dd` debug logging and hwsim instrumentation, so this is a
functional PASS rather than a throughput target.

2026-06-06 repeat/negative refresh:

```text
Negative window:
STA starts wps_pbc before AP PBC.
/tmp/hwsim-wps-repeat1-sta1.log shows WPS-PBC-ACTIVE only during the short
observation window, with no WPS-SUCCESS, no WPS-REG-SUCCESS, no
CTRL-EVENT-CONNECTED, and no AP-STA-CONNECTED.

First positive run:
/tmp/hwsim-wps-repeat1-ap.log:
PONG
wlan0: WPS-PBC-ACTIVE
wlan0: WPS-REG-SUCCESS 02:00:00:00:00:02 ed0bfab2-2afe-50f2-9fcb-5dc0a86a824e
wlan0: WPS-SUCCESS
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02
AP -> STA ping 5/5 and post-iperf ping 3/3
AP -> STA iperf: 0.00-   6.02 sec    1179648 Bytes    1.57 Mbits/sec

/tmp/hwsim-wps-repeat1-sta1.log:
PONG
wlan0: WPS-PBC-ACTIVE
wlan0: WPS-SUCCESS
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA -> AP ping 5/5 and post-iperf ping 3/3
STA -> AP iperf: 0.00-   6.02 sec    7192576 Bytes    9.56 Mbits/sec

Second fresh AP/STA run:
/tmp/hwsim-wps-repeat2-ap.log:
PONG
wlan0: WPS-PBC-ACTIVE
wlan0: WPS-REG-SUCCESS 02:00:00:00:00:02 ed0bfab2-2afe-50f2-9fcb-5dc0a86a824e
wlan0: WPS-SUCCESS
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: EAPOL-4WAY-HS-COMPLETED 02:00:00:00:00:02

/tmp/hwsim-wps-repeat2-sta1.log:
PONG
wlan0: WPS-PBC-ACTIVE
wlan0: WPS-SUCCESS
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
STA -> AP ping: 3 packets transmitted, 3 received, 0% packet loss
```

Known non-blocking WPS observations: after WPS success the logs still include
historical `CTRL-EVENT-EAP-FAILURE` / IEEE 802.1X failure lines, but the run
continues into CCMP keying, connected state, AP-side four-way-handshake
completion, and working data traffic. The first ARP can time out and retry
successfully. The debug-heavy iperf flow can hit the 64 MiB shared-medium
truncation guard; post-iperf ping remains healthy.

Problem fixed during WPS bring-up:

- Symptom: running `hostapd_cli -i wlan0 ping` inside the AP simulator process
  returned through global cleanup, disabled the AP, and could crash the
  background `hostapd` instance.
- Root cause: NuttX builtins share one flat address space. The one-shot
  `hostapd_cli` command initialized and destroyed global `eloop` state while
  background `hostapd` was still using it.
- Fix: `hostapd_cli.c` now resets NuttX builtin static/getopt state on entry
  and skips private `eloop_init()` / `eloop_destroy()` for one-shot commands
  that do not need an interactive loop, action file, daemon mode, or reconnect
  mode.
- Current result: `hostapd_cli -i wlan0 ping` returns `PONG`, AP remains
  enabled, `hostapd_cli -i wlan0 wps_pbc` starts PBC, and the STA completes WPS
  and WPA2 association.

### WNM BSS Transition Management proof

Config files:

```text
hostapd-hwsim-wnm.conf
wpa_supplicant-hwsim-wnm.conf
```

Build proof:

```text
./FeatherCore/tools/firmware/sim/build-ap.sh -j8 -> ap_rc=0
./FeatherCore/tools/firmware/sim/build-sta1.sh -j8 -> sta1_rc=0
/tmp/hwsim-build-ap-wnm.log: CC: wpa_supplicant-2.11/src/ap/wnm_ap.c
/tmp/hwsim-build-sta1-wnm.log: CC: wpa_supplicant-2.11/src/ap/wnm_ap.c
```

Latest runtime evidence from `/tmp/hwsim-wnm-ap.log` and
`/tmp/hwsim-wnm-sta1.log`:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: STA 02:00:00:00:00:02 WPA: pairwise key handshake completed (RSN)
hostapd_cli -i wlan0 ping -> PONG
WNM: Send BSS Transition Management Request to 02:00:00:00:00:02 req_mode=0x0 disassoc_timer=0 valid_int=0x1 dialog_token=1
WNM: BSS Transition Management Request: dialog_token=1 request_mode=0x0 disassoc_timer=0 validity_interval=1
wlan0: WNM: BSS Transition Management Request did not include candidates
WNM: Send BSS Transition Management Response to 02:00:00:00:00:01 dialog_token=1 status=1 reason=0 delay=0
wlan0: BSS-TM-RESP 02:00:00:00:00:02 status_code=1 bss_termination_delay=0
STA -> AP: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP: 0.00-   6.02 sec   14680064 Bytes   19.51 Mbits/sec
AP -> STA: 10 packets transmitted, 10 received, 0% packet loss
AP -> STA: 0.00-   6.02 sec   14548992 Bytes   19.33 Mbits/sec
```

Problem fixed during WNM bring-up:

- Symptom: `hostapd` rejected `bss_transition=1` and `wnm_sleep_mode=1` as
  unknown configuration items.
- Root cause: the local wpa/hostapd source list enabled STA-side `CONFIG_WNM`
  and `wnm_sta.c`, but did not enable AP-side `CONFIG_WNM_AP` or compile
  `src/ap/wnm_ap.c`.
- Fix: `apps/wireless/wifi/wpa_hostapd_sources.mk` now adds
  `-DCONFIG_WNM_AP` and includes `wpa_supplicant-2.11/src/ap/wnm_ap.c`.
- Current result: AP parses WNM config, advertises extended capabilities with
  BSS transition bits, sends BSS TM Request through `hostapd_cli`, receives STA
  BSS TM Response, and bidirectional data traffic remains healthy.

### WNM Sleep Mode proof

Config files:

```text
hostapd-hwsim-wnm.conf
wpa_supplicant-hwsim-wnm.conf
```

Latest no-TFS runtime evidence from `/tmp/hwsim-wnm-sleep-ap.log` and
`/tmp/hwsim-wnm-sleep-sta1.log`:

```text
wlan0: AP-ENABLED
wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]
wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
wpa_cli -i wlan0 ping -> PONG
wpa_cli -i wlan0 wnm_sleep enter interval=5 -> OK
WNM: Request to send WNM-Sleep Mode Request action=enter to 02:00:00:00:00:01
Successfully send WNM-Sleep Response frame
Successfully recv WNM-Sleep Response frame (action=0, intval=5)
wpa_cli -i wlan0 wnm_sleep exit -> OK
WNM: Request to send WNM-Sleep Mode Request action=exit to 02:00:00:00:00:01
Successfully send WNM-Sleep Response frame
Successfully recv WNM-Sleep Response frame (action=1, intval=0)
wlan0: STA 02:00:00:00:00:02 WPA: group key handshake completed (RSN)
wlan0: WPA: Not reinstalling already in-use GTK to the driver (keyidx=1 tx=0 len=16)
STA -> AP ping: 10 packets transmitted, 10 received, 0% packet loss
STA -> AP iperf: 0.00-   6.02 sec   16220160 Bytes   21.56 Mbits/sec
AP -> STA iperf: 0.00-   6.02 sec   14499840 Bytes   19.27 Mbits/sec
```

Notes:

- The smoke test used WNM Sleep enter interval 5 followed by exit on the same
  association. AP responded to both action frames and initiated a group-key
  handshake after exit.
- A later non-empty TFS rerun used
  `wpa_cli -i wlan0 wnm_sleep enter interval=5 tfs_req=5b020100` and proved the
  local nl80211 `.wnm_oper` path:

  ```text
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

- Problem/fix: adding `.wnm_oper` alone was not enough because hostapd passes
  the TFS response length as an output parameter; treating the incoming
  `*buf_len` as a valid capacity suppressed the response IE. The local nl80211
  helper now stores the Request IE and writes the Response IE length itself.
  This validates non-empty TFS IE request/response plumbing. Real TFS
  classifier/filter policy enforcement remains a separate follow-up.

## PHY Validation Notes

### 802.11a / 5 GHz

Config files:

```text
hostapd-a.conf
wpa-a.conf
```

Initial result: BLOCKED before AP startup.

Initial evidence from `/tmp/hwsim-11a-ap.log`:

```text
nl80211: Mode IEEE 802.11a: 5180[NO_IR] ...
Frequency 5180 (primary) not allowed for AP mode, flags: 0x853 NO-IR
wlan0: IEEE 802.11 Configured channel (36) or frequency (5180) ...
wlan0: IEEE 802.11 Hardware does not support configured channel
Could not select hw_mode and channel. (-3)
wlan0: AP-DISABLED
```

STA scan evidence from `/tmp/hwsim-11a-sta1.log` confirms that 5 GHz channels
are visible to the scan path:

```text
Scan included frequencies: ... 5180 5200 5220 5240 ... 5825
```

Conclusion:

The imported hwsim source has 5 GHz channel inventory, but the current
regulatory state prevents AP/GO initiation on 5 GHz. The NuttX hwsim path needs
one of these before 802.11a/ac/ax-5GHz runtime validation can pass:

- apply a custom hwsim regulatory domain that allows 5150-5240 MHz AP mode, or
- implement enough nl80211/cfg80211 country regulatory handling for
  `country_code=US` / `ieee80211d=1` to clear `NO_IR` where allowed.

Fix:

`nuttx/drivers/wireless/virtual/mac80211_hwsim_linux.c` now defaults
`regtest` to `HWSIM_REGTEST_CUSTOM_WORLD` for `CONFIG_ARCH_SIM`. This applies
hwsim's custom world regulatory domain during radio creation and clears the
`NO_IR` block for the non-DFS 5 GHz hwsim validation channels.

Retest result: PASS on channel 36 / 5180 MHz.

Evidence from `/tmp/hwsim-11a-ap.log` and `/tmp/hwsim-11a-sta1.log`:

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

### 802.11n / HT20 and HT40

Config files:

```text
hostapd-n.conf
wpa-n.conf
```

Result: PASS for basic 2.4 GHz HT20 AP/STA runtime.

Evidence from `/tmp/hwsim-11n-ap.log` and `/tmp/hwsim-11n-sta1.log`:

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

Remaining HT work:

- A-MPDU / ADDBA proof is now captured separately in the aggregation audit
  section with HT20 + WMM and `CONFIG_WL_NUTTX_HWSIM_AMPDU_PROOF`.
- Run longer traffic to check whether the recurring initial ARP wait timeout
  affects sustained HT traffic.

HT40 config files:

```text
hostapd-n-ht40.conf
wpa-n-ht40.conf
```

Result: PASS for 2.4 GHz HT40 AP/STA runtime.

Evidence from `/tmp/hwsim-11n40-ap.log` and `/tmp/hwsim-11n40-sta1.log`:

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

### 802.11ac / VHT20

Config files:

```text
hostapd-ac.conf
wpa-ac.conf
```

Initial result: BLOCKED in userspace config parsing.

Initial evidence from `/tmp/hwsim-11ac-ap.log`:

```text
Line 9: unknown configuration item 'ieee80211ac'
Line 10: unknown configuration item 'vht_oper_chwidth'
Failed to set up interface with /h/hostapd-ac.conf
Failed to initialize interface
```

Fix:

`apps/wireless/wifi/wpa_hostapd_sources.mk` now enables
`CONFIG_IEEE80211AC` and includes
`wpa_supplicant-2.11/src/ap/ieee802_11_vht.c`, matching the upstream hostapd
and wpa_supplicant Makefile rules for VHT support.

Retest result: PASS for 5 GHz VHT20 AP/STA runtime.

Evidence from `/tmp/hwsim-11ac-ap.log` and `/tmp/hwsim-11ac-sta1.log`:

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

VHT80 validation:

Initial VHT80 config attempt was blocked by hostapd channel setup:

```text
80/80+80 MHz: no second channel offset
Could not set channel for kernel driver
wlan0: AP-DISABLED
```

Fix:

`hostapd-ac-vht80.conf` now includes `ht_capab=[HT40+][SHORT-GI-20][SHORT-GI-40]`
so hostapd can derive the primary/secondary channel relationship for the 80 MHz
operation centered on channel 42.

The next attempt reached nl80211/cfg80211 with the correct chandef but failed
because the NuttX sim hwsim custom world regulatory domain allowed only 40 MHz
in the lower 5 GHz band:

```text
nl80211: Set freq 5180 (... vht_enabled=1 ... bandwidth=80 MHz, cf1=5210 MHz ...)
nl80211: Failed to set channel (freq=5180): -22
```

Fix:

`mac80211_hwsim_linux.c` now lets the NuttX sim custom world regulatory domain
use 80 MHz bandwidth in the non-DFS 5 GHz validation ranges while preserving the
existing 6 GHz rule.

During the rebuild, the sim config also unexpectedly selected
`CONFIG_SYSTEM_ARGTABLE3`, which forced an unrelated network download and blocked
the local build when GitHub returned no data. The three hwsim sim defconfigs now
explicitly keep `CONFIG_SYSTEM_ARGTABLE3` unset.

Retest result: PASS for 5 GHz VHT80 AP/STA runtime.

Evidence from `/tmp/hwsim-vht80-ap.log` and `/tmp/hwsim-vht80-sta1.log`:

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

VHT160 validation:

Config files:

```text
hostapd-ac-vht160.conf
wpa-ac-vht160.conf
```

Initial VHT160 AP startup reached cfg80211 with the intended 160 MHz chandef
but was rejected with `NL80211_CMD_SET_WIPHY ret=-22` because the lower 5 GHz
custom world regulatory rule did not allow a 160 MHz channel.

Fix:

`mac80211_hwsim_linux.c` now lets the NuttX sim custom world regulatory domain
use 160 MHz in the lower 5 GHz validation range. During the same audit,
`hwsim_world_regdom_custom_04.n_reg_rules` was corrected to match its seven
rule entries.

Retest result: PASS for 5 GHz VHT160 AP/STA runtime.

Evidence from `/tmp/hwsim-vht160-ap2.log` and
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

VHT80+80 validation:

Config files:

```text
hostapd-ac-vht80p80.conf
wpa-ac-vht80p80.conf
```

Retest result: PASS for 5 GHz VHT80+80 AP/STA runtime.

Evidence from `/tmp/hwsim-vht80p80-ap.log` and
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

Build/runtime notes from the VHT160/VHT80+80 pass:

- The AP and STA images must be rebuilt serially when sharing the same NuttX
  source tree. A prior interrupted/parallel build left generated dependency
  files corrupt and caused `Make.dep: multiple target patterns`; remove only
  generated files with `find apps nuttx -type f \( -name Make.dep -o -name
  .depend -o -name '*.ddc' -o -name '*.d' -o -name '*.o' -o -name '*.a' -o
  -name '*.la' \) -delete`, then rebuild roles one at a time.
- Throughput is much lower than the earlier VHT20/VHT80 runs in this
  debug-heavy configuration. The hostfs-backed shared hwsim medium can still
  hit the configured bounded-medium limit during iperf, so performance tuning
  remains separate from the functional pass.

Latest VHT IE inspection:

- Fresh VHT80 run captures Beacon, Probe Response, and Association Response
  VHT IEs and keeps the data path alive:

  ```text
  /tmp/hwsim-vht80-ie-ap.log:
  nl80211: Beacon tail ... bf 0c 20 00 00 00 aa aa 00 00 aa aa 00 00 c0 05 01 2a 00 fc ff
  CMD_FRAME - hexdump(len=239): ... bf 0c 20 00 00 00 aa aa 00 00 aa aa 00 00 c0 05 01 2a 00 fc ff ...
  * vht_capabilities - hexdump(len=12): 32 00 80 03 aa aa 00 00 aa aa 00 00
  CMD_FRAME - hexdump(len=154): ... bf 0c 20 00 00 00 aa aa 00 00 aa aa 00 00 c0 05 01 2a 00 fc ff ...
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02

  /tmp/hwsim-vht80-ie-sta1.log:
  resp_ies - hexdump(len=124): ... bf 0c 20 00 00 00 aa aa 00 00 aa aa 00 00 c0 05 01 2a 00 fc ff ...
  wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed
  3 packets transmitted, 3 received, 0% packet loss
  ```

- Existing VHT160 and VHT80+80 logs also contain VHT Capabilities and VHT
  Operation IEs in Beacon, Probe Response, and Association Response:

  ```text
  /tmp/hwsim-vht160-ap2.log and /tmp/hwsim-vht160-sta1-2.log:
  channel_width=5
  center_freq1=5250
  center_freq2=0
  Beacon/Probe Response/Association Response: ... bf 0c 60 ... c0 05 01 2a 32 fc ff ...

  /tmp/hwsim-vht80p80-ap.log and /tmp/hwsim-vht80p80-sta1.log:
  channel_width=4
  center_freq1=5210
  center_freq2=5775
  Beacon/Probe Response/Association Response: ... bf 0c 60 ... c0 05 01 2a 9b fc ff ...
  ```

Remaining VHT work:

- Wide-channel throughput tuning remains separate from functional/IE validation.

## First Validation Batch

1. WPA2-Personal CCMP:
   - `hostapd-hwsim-wpa2-psk-ccmp.conf`
   - `wpa_supplicant-hwsim-wpa2-psk-ccmp.conf`
   - Expected proof: `CTRL-EVENT-CONNECTED`, AP `AP-STA-CONNECTED`,
     successful key install, ping, iperf.
   - First result: FAIL.
   - First failure evidence:
     - AP prints `AP-ENABLED`.
     - STA repeatedly reaches auth/assoc attempt with the AP BSSID, then
       reports `CTRL-EVENT-DISCONNECTED ... reason=15` and finally
       `CTRL-EVENT-SSID-TEMP-DISABLED ... reason=CONN_FAILED`.
     - STA ping to AP fails with ARP timeout and `100% packet loss`.
     - AP prints `hwsim-debug: skip AP EAPOL RX PF_PACKET socket for NuttX hwsim`.
   - Root-cause chain and fixes:
     - AP-side EAPOL RX had been skipped in `driver_nl80211.c`; the skip was
       removed so hostapd opens its PF_PACKET control-port socket.
     - NuttX lower TX did not set `skb->protocol`, so EAPOL frames were not
       published by the hwsim medium; `netdevice_compat.c` now extracts the
       Ethernet ethertype from bytes 12/13.
     - mac80211 control-port RX compared EAPOL ethertype values with mixed
       host/network byte order; `mac80211/rx.c` now accepts the compat form.
     - `eth_type_trans()` depended on an incomplete Linux `struct ethhdr`;
       `cfg80211_compat.h` now reads the ethertype directly from packet bytes.
     - AF_PACKET `recvfrom()` did not return `sockaddr_ll.sll_addr`, so
       hostapd received M2 from `00:00:00:00:00:00` and retried M1 until
       timeout. `net/pkt/pkt_recvmsg.c` now fills `sockaddr_ll` source MAC for
       both active callback and readahead receive paths.
   - Final result: PASS.
   - Final proof:
     - AP log `/tmp/hwsim-wpa2ccmp-ap.log`:
       - `wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (2/4 Pairwise)`
       - `wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)`
       - `hwsim-debug: mac80211 sta_apply_auth_flags done ... authorized=1`
       - `wlan0: AP-STA-CONNECTED 02:00:00:00:00:02`
     - STA log `/tmp/hwsim-wpa2ccmp-sta1.log`:
       - `l2_packet_receive: src=02:00:00:00:00:01 len=99`
       - `wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]`
       - `wlan0: CTRL-EVENT-CONNECTED - Connection to 02:00:00:00:00:01 completed`
       - `3 packets transmitted, 3 received, 0% packet loss`
       - `0.00-   5.05 sec   13385728 Bytes   21.21 Mbits/sec`
   - AP + STA1 + STA2 sequential STA-to-STA proof:
     - STA1 -> STA2, logs
       `/tmp/hwsim-wpa2-sta1-to-sta2-sta1.log` and
       `/tmp/hwsim-wpa2-sta1-to-sta2-sta2.log`:
       - both STAs report `Key negotiation completed ... [PTK=CCMP GTK=CCMP]`
       - `5 packets transmitted, 5 received, 0% packet loss`
       - `0.00-   5.05 sec    5734400 Bytes    9.08 Mbits/sec`
     - STA2 -> STA1, logs
       `/tmp/hwsim-wpa2-sta2-to-sta1-sta2.log` and
       `/tmp/hwsim-wpa2-sta2-to-sta1-sta1.log`:
       - both STAs report `Key negotiation completed ... [PTK=CCMP GTK=CCMP]`
       - `5 packets transmitted, 5 received, 0% packet loss`
       - `0.00-   5.05 sec    5832704 Bytes    9.24 Mbits/sec`
   - Simultaneous full-duplex stress after the bounded hwsim medium fix:
     - AP + STA1 + STA2 all completed WPA2-PSK/CCMP association and RSN 4-way
       handshake.
     - Baseline STA1 -> STA2 ping: 3/3, 0% packet loss.
     - Baseline STA2 -> STA1 ping: 3/3, 0% packet loss.
     - Full-duplex STA1 -> STA2 iperf:
       `0.00-   6.02 sec     835584 Bytes    1.11 Mbits/sec`.
     - Full-duplex STA2 -> STA1 iperf:
       `0.00-   6.02 sec     196608 Bytes    0.26 Mbits/sec`.
     - `hwsim-frames.bin` hit the bounded-medium truncate path and dropped
       from 6.9 MiB to 8.1 KiB instead of growing to the previous 2.1G backlog.
     - Post-stress ping still passed both directions: 3/3, 0% packet loss.
     - Status: functional stress pass, but throughput and delayed iperf client
       exit on the weaker direction still need tuning.
2. WPA2-Personal PMF required:
   - `hostapd-hwsim-wpa2-psk-pmf.conf`
   - `wpa_supplicant-hwsim-wpa2-psk-pmf.conf`
   - Expected proof: PMF required association, ping, iperf.
   - First result: FAIL.
   - First failure evidence:
     - hostapd printed `Failed to set beacon parameters`.
     - No `NL80211_CMD_START_AP` was sent after the PMF beacon attributes were
       assembled.
     - Cleanup then tried a broadcast deauthentication frame and received
       `Frame command failed: ret=-16`, but that was a secondary cleanup-path
       symptom.
   - Root cause:
     - The NuttX/libnl default `nlmsg_alloc()` size was too small for longer AP
       setup messages. Plain WPA2 `START_AP` reached exactly 256 bytes, while
       the PMF SSID/RSN data exceeded that and made an `nla_put()` fail before
       userspace sent `START_AP`.
   - Fix:
     - `driver_nl80211.c` now builds the AP beacon/start message with
       `nlmsg_alloc_size(4096)` through `nl80211_ifindex_msg_build()`.
   - Final result: PASS.
   - Final proof:
     - AP log `/tmp/hwsim-wpa2pmf2-ap.log`:
       - `send type=19 cmd=15`
       - `complete family=nl80211 cmd=15 ret=0`
       - `IGTK - hexdump(len=16): [REMOVED]`
       - `wlan0: AP-ENABLED`
       - `wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)`
       - `Set STA flags ... total_flags=0x6d ... authorized=1`
       - `wlan0: AP-STA-CONNECTED 02:00:00:00:00:02`
     - STA log `/tmp/hwsim-wpa2pmf2-sta1.log`:
       - `wlan0: WPA: using MGMT group cipher AES-128-CMAC`
       - `wlan0: SME: Selected AP supports MFP: require MFP`
       - `WPA: IGTK in EAPOL-Key`
       - `wlan0: WPA: IGTK keyid 4 pn 000000000000`
       - `wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]`
       - `3 packets transmitted, 3 received, 0% packet loss`
       - `0.00-   5.05 sec   11468800 Bytes   18.17 Mbits/sec`
3. WPA2-Personal PMF optional:
   - `hostapd-pmf-opt.conf`
   - `wpa-pmf-opt.conf`
   - Expected proof: PMF optional association, MFP selection, IGTK install,
     ping, iperf.
   - First result: FAIL due to test harness command length, not protocol.
   - First failure evidence:
     - The NSH input line truncated
       `/h/wpa_supplicant-hwsim-wpa2-psk-pmf-optional.conf` to
       `/h/wpa_supplicant-hwsim-wpa2-psk-pmf-optional.co`.
     - The remaining `nf &` text was executed as a second NSH command.
     - `wpa_supplicant` printed `Failed to open config file ... optional.co`
       and `Failed to add interface wlan0`.
   - Fix:
     - Added short NSH-friendly config names:
       `hostapd-pmf-opt.conf` and `wpa-pmf-opt.conf`.
   - Final result: PASS.
   - Final proof:
     - AP log `/tmp/hwsim-pmfopt-ap.log`:
       - `complete family=nl80211 cmd=15 ret=0`
       - `IGTK - hexdump(len=16): [REMOVED]`
       - `wlan0: AP-ENABLED`
       - `wlan0: STA 02:00:00:00:00:02 WPA: received EAPOL-Key frame (4/4 Pairwise)`
       - `Set STA flags ... total_flags=0x6d ... authorized=1`
       - `wlan0: AP-STA-CONNECTED 02:00:00:00:00:02`
     - STA log `/tmp/hwsim-pmfopt-sta1.log`:
       - `wlan0: WPA: using MGMT group cipher AES-128-CMAC`
       - `wlan0: SME: Selected AP supports MFP: require MFP`
       - `WPA: IGTK in EAPOL-Key`
       - `wlan0: WPA: Key negotiation completed with 02:00:00:00:00:01 [PTK=CCMP GTK=CCMP]`
       - `3 packets transmitted, 3 received, 0% packet loss`
       - `0.00-   5.06 sec   11436032 Bytes   18.08 Mbits/sec`
4. If WPA2 fails, inspect:
   - nl80211 `NEW_KEY` / `SET_KEY` support
   - mac80211 key install path
   - PF_PACKET/EAPOL delivery
   - encrypted data frame handling in the hostfs hwsim medium

## Logging Rules

For every validation item, record:

- Config files used.
- AP/STA commands.
- PASS/FAIL/BLOCKED.
- Exact failure line if failed.
- Root cause once known.
- Fix commit/file once fixed.
- Final proof lines for association and traffic.
