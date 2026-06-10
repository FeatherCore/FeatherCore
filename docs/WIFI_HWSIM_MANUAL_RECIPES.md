# Wi-Fi HWSIM Manual Validation Recipes

This document keeps the repeatable manual flows visible. It intentionally does
not hide the NuttX commands in another automation script: each AP, STA, P2P GO,
or P2P client is a separate `nuttx-sim-*` process in a separate terminal.

## Common Rules

Build role images sequentially from the FeatherCore root. The NuttX tree has a
single active `.config` and generated include state, so parallel role builds can
race and corrupt generated files.

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore
tools/firmware/sim/build-ap.sh -j8
tools/firmware/sim/build-sta1.sh -j8
tools/firmware/sim/build-sta2.sh -j8
```

Run simulator terminals from `tools/firmware/sim` so
`mount -t hostfs -o fs=. /h` exposes all hostapd/wpa_supplicant config files
under `/h`.

Before a fresh multi-terminal run, remove stale shared hwsim media files:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
rm -f hwsim-frames.bin hwsim-bss.bin
```

For the standard AP + STA1 PASS matrix, the current repeatable runner is:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore_ESP
tools/firmware/sim/validate-hwsim-pass-matrix.py \
  --out-dir /tmp/hwsim-pass-matrix-$(date +%Y%m%d-%H%M%S) \
  --hostfs-dir /tmp/hwsim-pass-matrix-hostfs
```

Useful targeted reruns:

```sh
tools/firmware/sim/validate-hwsim-pass-matrix.py --case open --case wpa2-psk-ccmp
tools/firmware/sim/validate-hwsim-pass-matrix.py --case owe-g20 --case owe-g21
tools/firmware/sim/validate-hwsim-pass-matrix.py --case ft-psk
```

OWE groups 20 and 21 must use the dedicated STA configs
`wpa-owe-g20.conf` and `wpa-owe-g21.conf`; using the generic OWE STA config
will scan for the wrong SSID and produce a runner-only failure. The runner
records all AP/STA logs and writes a `summary.md` file under the selected
output directory.

Use the same pattern for additional roles:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore
tools/firmware/sim/build-sta3.sh -j8
tools/firmware/sim/build-ap1.sh -j8
tools/firmware/sim/build-ap2.sh -j8
tools/firmware/sim/build-p2p1.sh -j8
tools/firmware/sim/build-p2p2.sh -j8
```

## AP + STA1 + STA2 Data Plane

Terminal AP:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
../../../build/nuttx-sim-ap
```

Inside AP NSH:

```text
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.201.1 netmask 255.255.255.0
hostapd -dd /h/hostapd-hwsim.conf &
```

Wait for:

```text
wlan0: AP-ENABLED
```

Terminal STA1:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
../../../build/nuttx-sim-sta1
```

Inside STA1 NSH:

```text
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.201.2 netmask 255.255.255.0
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim.conf &
```

Terminal STA2:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
../../../build/nuttx-sim-sta2
```

Inside STA2 NSH:

```text
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.201.3 netmask 255.255.255.0
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim.conf &
```

Wait for both STAs:

```text
CTRL-EVENT-CONNECTED
```

STA1 to STA2 traffic:

```text
# STA2
iperf -s -p 5091 &

# STA1
ping -c 20 192.168.201.3
iperf -c 192.168.201.3 -p 5091 -t 20
ping -c 10 192.168.201.3
```

STA2 to STA1 traffic:

```text
# STA1
iperf -s -p 5092 &

# STA2
ping -c 20 192.168.201.2
iperf -c 192.168.201.2 -p 5092 -t 20
ping -c 10 192.168.201.2
```

Passing evidence should include both STAs connected, ping with 0% packet loss
after ARP warm-up, and iperf summaries on both client and server sides.

Optional STA3 extension:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
../../../build/nuttx-sim-sta3
```

Inside STA3 NSH:

```text
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.201.4 netmask 255.255.255.0
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim.conf &
```

Expected AP evidence is a third station:

```text
wlan0: AP-STA-CONNECTED 02:00:00:00:00:04
```

Useful four-object traffic checks:

```text
# STA3
ping -c 3 192.168.201.1
ping -c 3 192.168.201.2
iperf -s -p 5301 &

# STA1
ping -c 3 192.168.201.4
iperf -c 192.168.201.4 -p 5301 -t 10
iperf -s -p 5302 &

# STA2
ping -c 3 192.168.201.4

# STA3
iperf -c 192.168.201.2 -p 5302 -t 10
```

The 2026-06-06 AP + STA1 + STA2 + STA3 run passed all pings with 0% packet
loss after ARP warm-up. STA1->STA3 iperf reached about 0.72 Mbits/sec and
STA3->STA1 reached about 1.52 Mbits/sec.

To reuse this flow for WPA2/WPA3/OWE/Enterprise/PHY validation, change only the
config file names in the AP and STA commands, for example:

```text
hostapd -dd /h/hostapd-hwsim-wpa3-sae.conf &
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-wpa3-sae.conf &
```

## PHY / 802.11 Version Config Map

Use the AP + STA flow above and replace the AP/STA config files with the pair
for the version under test. Keep the same ping and iperf evidence pattern after
association.

| Version | AP config | STA config | Expected proof |
| --- | --- | --- | --- |
| 802.11b | `hostapd-b.conf` | `wpa-b.conf` | AP reports `Mode: IEEE 802.11b` on channel 1 / 2412 MHz; STA associates; ping and iperf pass. |
| 802.11g | `hostapd-g.conf` | `wpa-g.conf` | AP reports `Mode: IEEE 802.11g`; chandef has HT/VHT/HE disabled; ping and iperf pass. |
| 802.11a | `hostapd-a.conf` | `wpa-a.conf` | AP starts on channel 36 / 5180 MHz; STA associates on 5180 MHz; ping and iperf pass. |
| 802.11n HT20 | `hostapd-n.conf` | `wpa-n.conf` | AP sets `ht_enabled=1` with 20 MHz bandwidth; STA HT capabilities appear; ping and iperf pass. |
| 802.11n HT40 | `hostapd-n-ht40.conf` | `wpa-n-ht40.conf` | AP sets `bandwidth=40 MHz`, control channel 6, secondary channel 10; ping and iperf pass. |
| 802.11ac VHT20 | `hostapd-ac.conf` | `wpa-ac.conf` | AP sets `vht_enabled=1`, 20 MHz on 5180 MHz; ping and iperf pass. |
| 802.11ac VHT80 | `hostapd-ac-vht80.conf` | `wpa-ac-vht80.conf` | AP sets `bandwidth=80 MHz`, center frequency 5210 MHz, and VHT IEs appear in management frames. |
| 802.11ac VHT160 | `hostapd-ac-vht160.conf` | `wpa-ac-vht160.conf` | AP sets `channel_width=5`, center frequency 5250 MHz; bidirectional ping and iperf pass. |
| 802.11ac VHT80+80 | `hostapd-ac-vht80p80.conf` | `wpa-ac-vht80p80.conf` | AP sets `channel_width=4`, center frequencies 5210/5775 MHz; bidirectional ping and iperf pass. |
| 802.11ax HE 2.4 GHz | `hostapd-ax.conf` | `wpa-ax.conf` | AP starts on 2437 MHz with `he_enabled=1`; STA HE capabilities appear; ping and iperf pass. |
| 802.11ax HE 5 GHz | `hostapd-ax-a.conf` | `wpa-ax-a.conf` | AP enables HT, VHT, and HE together on 5180 MHz; STA VHT and HE capabilities appear; ping and iperf pass. |
| 802.11ax HE 6 GHz SAE/H2E | `hostapd-ax6-sae.conf` | `wpa-ax6-sae.conf` | AP starts on 5955 MHz with secure SAE/H2E; STA scans 6 GHz, completes SAE, and traffic passes. |
| 802.11ax HE 6 GHz OWE | `hostapd-ax6-owe.conf` | `wpa-ax6-owe.conf` | AP starts on 5955 MHz with OWE and PMF; OWE PMKID matches and traffic passes. |
| S1G / 802.11ah | `hostapd-hwsim-s1g.conf` | `wpa_supplicant-hwsim-s1g.conf` | AP starts on 912 MHz; STA1/STA2 associate; STA-to-AP and STA-to-STA ping/iperf pass. |

Important PHY caveats:

- 6 GHz open/pre-RSNA AP mode is expected to be rejected; use SAE/H2E or OWE
  for valid 6 GHz HE runtime validation.
- VHT160 and VHT80+80 are functional in the simulator, but throughput is much
  lower than VHT20/VHT80 under debug-heavy hostfs hwsim runs. Treat throughput
  tuning as separate from functional pass/fail.
- S1G validation proves the simulator control/data path and compatibility
  handling, not real S1G PHY modulation.

## Security / Encryption Config Map

Use the AP + STA flow above and replace the AP/STA config files with the pair
for the security mode under test. For two-STA coverage, boot STA2 with the same
supplicant config as STA1 unless the row explicitly calls out a mixed-client
test.

| Mode | AP config | STA config | Expected proof |
| --- | --- | --- | --- |
| Open auth | `hostapd-hwsim.conf` | `wpa_supplicant-hwsim.conf` | Association, STA-to-AP traffic, and STA-to-STA traffic pass. |
| WPA2-PSK CCMP | `hostapd-hwsim-wpa2-psk-ccmp.conf` | `wpa_supplicant-hwsim-wpa2-psk-ccmp.conf` | RSN 4-way handshake installs `PTK=CCMP GTK=CCMP`; ping and iperf pass. |
| WPA2-PSK hex | `hostapd-hwsim-wpa2-psk-hex.conf` | `wpa_supplicant-hwsim-wpa2-psk-hex.conf` | Same WPA2-PSK CCMP proof using a raw PSK. |
| WPA2-PSK TKIP | `hostapd-hwsim-wpa2-psk-tkip.conf` | `wpa_supplicant-hwsim-wpa2-psk-tkip.conf` | RSN 4-way handshake selects TKIP; ping and iperf pass. |
| WPA mixed TKIP/CCMP | `hostapd-hwsim-wpa-mixed.conf` | `wpa_supplicant-hwsim-wpa-mixed.conf` | Mixed WPA/WPA2 advertisement selects CCMP pairwise with TKIP group where configured. |
| WPA2 PMF optional | `hostapd-hwsim-wpa2-psk-pmf-optional.conf` | `wpa_supplicant-hwsim-wpa2-psk-pmf-optional.conf` | STA selects MFP when available; IGTK install and traffic pass. |
| WPA2 PMF required | `hostapd-hwsim-wpa2-psk-pmf.conf` | `wpa_supplicant-hwsim-wpa2-psk-pmf.conf` | `WPA-PSK-SHA256`, PMF required, IGTK install, ping, and iperf pass. |
| WPA2 PMF + OCV | `hostapd-hwsim-wpa2-psk-ocv.conf` | `wpa_supplicant-hwsim-wpa2-psk-ocv.conf` | `ocv=1`, PMF required, OCI KDE in EAPOL message 2/4 and 3/4, `CTRL-EVENT-CONNECTED`, bidirectional ping, and bidirectional iperf pass. |
| FILS-SHA256 | `hostapd-hwsim-fils-sha256.conf` | `wpa_supplicant-hwsim-fils-sha256.conf` | First association uses internal EAP-PSK, stores ERP keys, adds PMKSA cache id `0011`, installs keys, and passes traffic. Same-context reconnect then uses cached FILS with ERP+PMKSA, AP decrypts the encrypted Association Request with AES-SIV, STA reports `FILS_HLP_SENT`, and post-reconnect ping passes. |
| WPA3 SAE | `hostapd-hwsim-wpa3-sae.conf` | `wpa_supplicant-hwsim-wpa3-sae.conf` | SAE commit/confirm, PMF required, 4-way handshake, ping, and iperf pass. |
| WPA3 SAE H2E | `hostapd-hwsim-wpa3-sae-h2e.conf` | `wpa_supplicant-hwsim-wpa3-sae-h2e.conf` | H2E/PT derivation and RSNXE exchange are visible; ping and iperf pass. |
| WPA3 SAE-PK | `hostapd-hwsim-wpa3-sae-pk.conf` | `wpa_supplicant-hwsim-wpa3-sae-pk.conf` | STA validates SAE-PK fingerprint/KeyAuth; ping and bidirectional iperf pass. |
| WPA2/WPA3 transition, SAE STA | `hostapd-hwsim-wpa-transition.conf` | `wpa_supplicant-hwsim-wpa-transition.conf` | SAE client connects to transition AP with PMF optional; traffic passes. |
| WPA2/WPA3 transition, PSK STA | `hostapd-hwsim-wpa-transition.conf` | `wpa_supplicant-hwsim-wpa-transition-psk.conf` | Legacy WPA-PSK client also connects to the same transition AP; traffic passes. |
| OWE group 19 | `hostapd-hwsim-owe.conf` | `wpa_supplicant-hwsim-owe.conf` | OWE Diffie-Hellman, matching PMKID, PMF required, and traffic pass. |
| OWE group 20 | `hostapd-hwsim-owe-g20.conf` | `wpa_supplicant-hwsim-owe.conf` | P-384 OWE path passes after the NuttX P-384 wrapper fix. |
| OWE group 21 | `hostapd-hwsim-owe-g21.conf` | `wpa_supplicant-hwsim-owe.conf` | P-521 OWE path passes after increasing the libnl netlink message size. |
| WPA2-Enterprise EAP-PSK | `hostapd-hwsim-wpa2-eap-psk.conf` | `wpa_supplicant-hwsim-wpa2-eap-psk.conf` | Internal EAP server authenticates EAP-PSK; RSN 4-way and traffic pass. |
| WPA2-Enterprise EAP-TLS | `hostapd-hwsim-wpa2-eap-tls.conf` | `wpa_supplicant-hwsim-wpa2-eap-tls.conf` | Certificate-based TLS authentication, EAP success, RSN 4-way, and traffic pass. |
| WPA2-Enterprise PEAP/MSCHAPv2 | `hostapd-hwsim-wpa2-eap-peap.conf` | `wpa_supplicant-hwsim-wpa2-eap-peap.conf` | PEAP tunnel and MSCHAPv2 inner auth succeed; traffic passes. |
| WPA2-Enterprise TTLS/PAP | `hostapd-hwsim-wpa2-eap-ttls.conf` | `wpa_supplicant-hwsim-wpa2-eap-ttls.conf` | TTLS outer tunnel and PAP inner auth succeed; traffic passes. |
| WPA2-Enterprise TTLS/CHAP | `hostapd-hwsim-wpa2-eap-ttls-chap.conf` | `wpa_supplicant-hwsim-wpa2-eap-ttls-chap.conf` | TTLS/CHAP succeeds; traffic passes. |
| WPA2-Enterprise TTLS/MSCHAP | `hostapd-hwsim-wpa2-eap-ttls-mschap.conf` | `wpa_supplicant-hwsim-wpa2-eap-ttls-mschap.conf` | TTLS/MSCHAP succeeds; traffic passes. |
| WPA2-Enterprise TTLS/MSCHAPv2 | `hostapd-hwsim-wpa2-eap-ttls-mschapv2.conf` | `wpa_supplicant-hwsim-wpa2-eap-ttls-mschapv2.conf` | TTLS/MSCHAPv2 succeeds; traffic passes. |
| Suite-B / 192-bit | `hostapd-hwsim-suiteb192-eap-tls.conf` | `wpa_supplicant-hwsim-suiteb192-eap-tls.conf` | `WPA-EAP-SUITE-B-192`, GCMP-256, BIP-GMAC-256, PMF required, EAP success, and traffic pass. |
| WPS PBC | `hostapd-hwsim-wps.conf` | `wpa_supplicant-hwsim-wps.conf` | Run WPS PBC through control interfaces, provision WPA2-PSK credentials, then verify traffic. |
| 802.11r FT-PSK | `hostapd-hwsim-ft-psk.conf` | `wpa_supplicant-hwsim-ft-psk.conf` | FT-PSK AKM, MDIE/FTIE parsing, and data traffic pass. |
| 802.11r FT-PSK roam | `hostapd-hwsim-ft-psk-multibss.conf` | `wpa_supplicant-hwsim-ft-psk-roam.conf` | `wpa_cli roam` completes FT reassociation; keep the target BSS on the test subnet and bind AP-originated traffic to the target-BSS source address. |
| WNM Sleep/BSS-TM | `hostapd-hwsim-wnm.conf` | `wpa_supplicant-hwsim-wnm.conf` | BSS Transition and WNM Sleep action frames pass; non-empty TFS IE plumbing is proven separately. |
| HS20 / interworking | `hostapd-hwsim-hs20.conf` | `wpa_supplicant-hwsim-hs20.conf` | WPA2-Enterprise EAP-PSK association plus ANQP/HS20 queries pass. |

### FILS Fast Reconnect

Use the FILS row above for AP and STA startup. After the first
`CTRL-EVENT-CONNECTED`, keep the same `wpa_supplicant` process alive so its
ERP and PMKSA caches remain populated, then force a same-BSS reconnect:

```text
# STA1
wpa_cli -i wlan0 disconnect
wpa_cli -i wlan0 reconnect
```

Expected proof:

```text
STA1:
  EAP: Valid ERP key found 16aa9b561a593459@example.com (SEQ=0)
  FILS: Try to use FILS (erp=1 pmksa_cache=1)
  FILS: Auth+Assoc completed successfully
  wlan0: CTRL-EVENT-CONNECTED - ... FILS_HLP_SENT

AP:
  FILS: Found matching PMKSA cache entry
  FILS: Decrypted Association Request elements - hexdump(len=159): ...
  wlan0: AP-STA-CONNECTED 02:00:00:00:00:02
```

Then repeat ping and iperf in both directions. The 2026-06-06 fast-reconnect
smoke proves post-reconnect AP-to-STA ping with 0% loss. The follow-up
STA2-present run keeps STA2 associated while STA1 reconnects through cached
FILS, then proves STA1->STA2 ping 3/3, STA2->STA1 ping 3/3, STA1->STA2 iperf
at about 1.39 Mbits/sec, STA2->STA1 iperf at about 0.76 Mbits/sec, and
post-iperf STA1->STA2 ping 3/3. A later focused two-STA FILS data-plane rerun
on `192.168.203.0/24` again showed AP connections for both STA MACs, both STAs
at `CTRL-EVENT-CONNECTED`, STA1->STA2 ping 3/3 and iperf about 0.61 Mbits/sec,
plus STA2->STA1 ping 3/3 and iperf about 1.09 Mbits/sec. Repeated long-run
reconnect can be exercised by running the disconnect/reconnect pair multiple
times before the final traffic check:

```text
wpa_cli -i wlan0 disconnect
sleep 1
wpa_cli -i wlan0 reconnect
```

The 2026-06-06 AP+STA1+STA2 loop run on `192.168.204.0/24` kept STA2 online
while STA1 completed three cached FILS reconnect cycles. Expected proof lines
are three STA1 `FILS_HLP_SENT` events, ERP key sequence advancing through
`SEQ=0`, `SEQ=1`, and `SEQ=2`, and three AP-side `Decrypted Association
Request elements` lines. After the loop, STA1->STA2 ping 3/3 and iperf about
0.52 Mbits/sec passed, and STA2->STA1 ping 3/3 and iperf about
0.44 Mbits/sec passed. Longer soak loops and throughput/medium tuning remain
useful extensions.

DPP / Easy Connect uses a control-flow recipe rather than only a config-file
swap:

```text
# AP configurator bootstrap/control config
hostapd -dd /h/hostapd-hwsim-dpp.conf &

# STA enrollee config
wpa_supplicant -dd -i wlan0 -c /h/wpa_supplicant-hwsim-dpp-enrollee.conf &

# After DPP provisioning, AP serves the provisioned PSK network:
hostapd -dd /h/hostapd-hwsim-dpp-psk.conf &
```

For DPP, use `wpa_cli` / `hostapd_cli` with `@file` arguments for long QR URI
and auth-init parameters. The passing PSK provisioning proof expects
`DPP-AUTH-SUCCESS`, `DPP-CONF-SENT conf_status=0`, STA `wpa_state=COMPLETED`,
and post-provisioning ping/iperf. CSR/PKCS#7 certificate-mode flows still need
real crypto backend support.

Important security caveats:

- Suite-B / 192-bit validates the Wi-Fi AKM/cipher selection in the simulator,
  but the imported internal TLS stack still negotiates a simulator TLS profile
  rather than a strict CNSA ECDHE/ECDSA SHA384/GCM profile.
- Enterprise tests use the internal hostapd EAP server and local certificate
  material in `tools/firmware/sim`; external RADIUS is separate coverage.
- 6 GHz HE security is covered in the PHY map with SAE/H2E and OWE configs.

## Feature / Testmode Recipes

The normal AP/STA hwsim builds include the validation-only hwsim feature knobs:

```text
CONFIG_WL_NUTTX_HWSIM_AMPDU_PROOF=y
CONFIG_WL_NUTTX_HWSIM_AMSDU_PROOF=y
CONFIG_WL_NUTTX_HWSIM_DCM_PROOF=y
CONFIG_WL_NUTTX_HWSIM_PS_PROOF=y
CONFIG_NL80211_TESTMODE=y
CONFIG_EXAMPLES_HWSIM_TESTMODE=y
CONFIG_EXAMPLES_HWSIM_TESTMODE_PROGNAME="hwsim_tm"
```

The `hwsim_tm` command syntax used by the current proofs is:

```text
hwsim_tm [-i wlan0] get_ps
hwsim_tm [-i wlan0] set_ps 0
hwsim_tm [-i wlan0] set_ps 1
hwsim_tm [-i wlan0] set_ps 2
hwsim_tm [-i wlan0] set_ps 4
hwsim_tm [-i wlan0] poll
hwsim_tm [-i wlan0] stop_queues
hwsim_tm [-i wlan0] wake_queues
hwsim_tm [-i wlan0] get_twt_sp
hwsim_tm [-i wlan0] set_twt_sp 0
hwsim_tm [-i wlan0] set_twt_sp 1
```

### DCM

Use the HE 2.4 GHz AP/STA pair and create the marker file before starting data
traffic:

```text
# AP
hostapd -dd /h/hostapd-ax.conf &

# STA
wpa_supplicant -dd -i wlan0 -c /h/wpa-ax.conf &

# AP or STA hostfs, before ping/iperf
touch /h/hwsim-dcm-proof
```

Expected proof:

```text
dcm-proof: rx status ... encoding=HE ... he_dcm=1
```

Then run bidirectional ping and iperf. Remove `/h/hwsim-dcm-proof` after the
run so non-DCM tests are not mislabeled.

### A-MPDU / ADDBA

Use the dedicated HT20/WMM configs:

```text
# AP
hostapd -dd /h/hostapd-ampdu.conf &

# STA
wpa_supplicant -dd -i wlan0 -c /h/wpa-ampdu.conf &
```

Run bidirectional ping and iperf. Expected AP/STA log evidence:

```text
aggr_check start
start_tx_ba_session request/queued
send ADDBA_REQ
rx ADDBA_REQ
send ADDBA_RESP
rx ADDBA_RESP status=0
tx operational
ampdu_action
```

### A-MSDU

Use the same `hostapd-ampdu.conf` / `wpa-ampdu.conf` association, then force AP
queue backlog while STA sends TCP:

```text
# AP, after association
hwsim_tm -i wlan0 stop_queues

# STA
iperf -c 192.168.201.1 -p 5004 -t 8 &

# AP, after backlog has formed
hwsim_tm -i wlan0 wake_queues
```

Expected AP log evidence:

```text
amsdu-proof: aggregate
```

The current passing proof saw aggregation up to eight subframes and kept
post-wake ping/iperf healthy.

### TWT

Use HE AP/STA with TWT responder support:

```text
# AP
hostapd -dd /h/hostapd-ax.conf &

# STA
wpa_supplicant -dd -i wlan0 -c /h/wpa-ax-twt.conf &
```

Control/action smoke:

```text
# STA
wpa_cli -i wlan0 twt_setup
wpa_cli -i wlan0 twt_teardown
```

Service-period gate proof:

```text
# STA
hwsim_tm -i wlan0 set_ps 1
hwsim_tm -i wlan0 get_twt_sp

# AP, AP->STA traffic should be buffered while twt_sp=0
ping -c 3 192.168.201.2

# STA, open service period
hwsim_tm -i wlan0 set_twt_sp 1
hwsim_tm -i wlan0 get_twt_sp

# AP, traffic should recover
ping -c 3 192.168.201.2
iperf -c 192.168.201.2 -p 5005 -t 6

# STA, close service period and wake normally
hwsim_tm -i wlan0 set_twt_sp 0
hwsim_tm -i wlan0 set_ps 0
wpa_cli -i wlan0 twt_teardown
```

Expected proof includes AP TWT Accept, STA parsed setup response, traffic blocked
while `twt_sp=0`, traffic restored when `twt_sp=1`, and teardown RX on the AP.
The automatic TWT scheduler is functional but still has throughput/ARP timing
caveats under debug-heavy hostfs hwsim runs.

### Static PS

Standard userspace power-save path:

```text
# STA
wpa_cli -i wlan0 p2p_set ps 1
ping -c 3 192.168.201.1
wpa_cli -i wlan0 p2p_set ps 0
ping -c 3 192.168.201.1
```

Private hwsim PS latch and PS-Poll path:

```text
# STA
hwsim_tm -i wlan0 get_ps
hwsim_tm -i wlan0 set_ps 1
hwsim_tm -i wlan0 get_ps
hwsim_tm -i wlan0 poll
hwsim_tm -i wlan0 set_ps 0
hwsim_tm -i wlan0 get_ps
ping -c 3 192.168.201.1
```

Queue stop/wake proof:

```text
# STA
hwsim_tm -i wlan0 stop_queues
ping -c 3 192.168.201.1
hwsim_tm -i wlan0 wake_queues
ping -c 3 192.168.201.1
```

Expected proof includes PM=1/PM=0 nullfunc frames, PS-Poll frame emission,
AP-side buffered-frame delivery, and traffic recovery after wake.

### Dynamic PS / MLO

Dynamic PS uses dedicated role images so the MLO/dynamic-PS knobs do not change
the baseline AP/STA/P2P objects:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore
tools/firmware/sim/build-dynps-ap.sh -j8
tools/firmware/sim/build-dynps-sta1.sh -j8
```

Boot them from `tools/firmware/sim`:

```sh
../../../build/nuttx-sim-dynps-ap
../../../build/nuttx-sim-dynps-sta1
```

Inside AP NSH:

```text
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.203.1 netmask 255.255.255.0
hostapd -dd /h/hostapd-hwsim.conf &
```

Inside STA NSH:

```text
mount -t hostfs -o fs=. /h
ifconfig wlan0 192.168.203.2 netmask 255.255.255.0
wpa_supplicant -dd -i wlan0 -c /h/wpa-ps.conf &
```

Auto-poll proof:

```text
# STA
hwsim_tm -i wlan0 get_ps
hwsim_tm -i wlan0 set_ps 2
hwsim_tm -i wlan0 get_ps

# AP and STA
ping -c 3 192.168.203.2
ping -c 3 192.168.203.1
```

Dynamic idle-timeout proof:

```text
# STA
hwsim_tm -i wlan0 set_ps 4
hwsim_tm -i wlan0 get_ps

# AP, after idle timeout, AP-originated traffic should be rejected/buffered
ping -c 1 192.168.203.2

# STA, transmit to wake the link
ping -c 3 192.168.203.1
hwsim_tm -i wlan0 set_ps 0
hwsim_tm -i wlan0 get_ps
```

AP-link-PS proof uses the same dynps objects and checks for
`ap-link-ps-proof` plus PS-Poll/buffer delivery logs. Current status is
functional, but AP-to-STA TCP under auto-PS still needs throughput tuning.

## P2P1 + P2P2 GO/Client

Build and boot the dedicated P2P role images:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore
tools/firmware/sim/build-p2p1.sh -j8
tools/firmware/sim/build-p2p2.sh -j8
```

Terminal P2P1:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
../../../build/nuttx-sim-p2p1
```

Inside P2P1 NSH:

```text
mount -t hostfs -o fs=. /h
wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-p2p.conf &
wpa_cli -i wlan0 status
wpa_cli -i wlan0 p2p_find 45
wpa_cli -i wlan0 p2p_peers
```

Terminal P2P2:

```sh
cd /home/uan/Feather-develop-WIFI/FeatherCore/tools/firmware/sim
../../../build/nuttx-sim-p2p2
```

Inside P2P2 NSH:

```text
mount -t hostfs -o fs=. /h
wpa_supplicant -i wlan0 -c /h/wpa_supplicant-hwsim-p2p.conf &
wpa_cli -i wlan0 status
wpa_cli -i wlan0 p2p_find 45
wpa_cli -i wlan0 p2p_peers
```

Use `p2p_peers` output to get the peer device addresses. In the current role
mapping, p2p1 is typically `42:00:00:00:00:14` and p2p2 is typically
`42:00:00:00:00:15`.

Start GO negotiation:

```text
# P2P1
wpa_cli -i wlan0 p2p_connect 42:00:00:00:00:15 pbc go_intent=15 freq=2437

# P2P2
wpa_cli -i wlan0 p2p_connect 42:00:00:00:00:14 pbc go_intent=0 freq=2437
```

Wait for:

```text
P2P-GO-NEG-SUCCESS
P2P-GROUP-STARTED p2p-wlan0-0
```

Configure the P2P group interface:

```text
# P2P1 / GO
ifconfig p2p-wlan0-0 192.168.77.1 netmask 255.255.255.0
wpa_cli -i p2p-wlan0-0 status

# P2P2 / client
ifconfig p2p-wlan0-0 192.168.77.2 netmask 255.255.255.0
wpa_cli -i p2p-wlan0-0 status
```

Use `ifconfig` as the authoritative source for the group interface IP. In some
builds `wpa_cli -i p2p-wlan0-0 status` can still report the parent interface
IP even though `p2p-wlan0-0` is correctly configured.

GO to client traffic:

```text
# P2P2
iperf -s -B 192.168.77.2 -p 5082 &

# P2P1
ping -c 5 192.168.77.2
iperf -c 192.168.77.2 -B 192.168.77.1 -p 5082 -t 60
ping -c 3 192.168.77.2
```

Client to GO traffic:

```text
# P2P1
iperf -s -B 192.168.77.1 -p 5081 &

# P2P2
ping -c 5 192.168.77.1
iperf -c 192.168.77.1 -B 192.168.77.2 -p 5081 -t 60
ping -c 3 192.168.77.1
```

The latest 2026-06-06 independent-object soak used these 60-second client
windows. GO->client completed at about 1.34 Mbits/sec and client->GO completed
at about 1.32 Mbits/sec; both clients printed `iperf exit`, and post-iperf
pings were 3/3 in both directions.

## Common Problems And Fixes

- Problem: parallel builds of AP/STA/P2P roles can race generated `.config` and
  include files.
  Fix: build one role at a time.
- Problem: stale `hwsim-frames.bin` / `hwsim-bss.bin` can carry old medium
  state or hit the configured bounded-medium truncate path during long
  debug-heavy iperf runs. The default limit is now 64 MiB through
  `CONFIG_WL_NUTTX_HWSIM_MEDIUM_MAX_BYTES`.
  Fix: delete both files before a clean validation run, and raise the Kconfig
  value only when a longer soak needs a larger bounded hostfs medium.
- Problem: first ping may lose time to ARP path warm-up.
  Fix: record both the first ping and a post-iperf/post-warm-up ping.
- Problem: the lightweight `wifi_ap_demo` WEXT path fails at
  `SIOCSIWMODE(IW_MODE_MASTER)` with `-EINVAL`.
  Fix: use the full `hostapd` + `wpa_supplicant` nl80211 path for AP/STA
  validation, or add a dedicated AP helper that issues nl80211 `START_AP` /
  builds cfg80211 AP settings. Do not treat `wifi_ap_demo` as the full-stack
  STA-to-STA proof.
- Problem: P2P peer addresses can differ if role radio bases change.
  Fix: use `wpa_cli -i wlan0 p2p_peers` output instead of hard-coding peer
  addresses in the result log.
- Problem: AP-originated multi-BSS/roam traffic can fail if the target AP BSS
  netdev has no IP in the current NuttX sim no-bridge path.
  Fix: configure the target BSS netdev IP before post-roam ping/iperf. If both
  `wlan0` and `ap1` share `192.168.201.1/24`, AP-originated traffic can still
  choose the old BSS. For AP-originated post-roam traffic, move the old BSS out
  of the target subnet and bind test clients to the target-BSS source:

  ```text
  ifconfig wlan0 192.168.202.1 netmask 255.255.255.0
  ifconfig ap1 192.168.201.1 netmask 255.255.255.0
  iperf -c 192.168.201.2 -B 192.168.201.1 -p 5112 -t 10
  ```
