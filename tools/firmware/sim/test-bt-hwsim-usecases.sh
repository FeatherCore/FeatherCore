#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/test-bt-hwsim-usecases.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../../.." && pwd)"
default_out="${repo_root}/build/bt-hwsim-usecases"

usage() {
  cat <<'EOF'
usage: test-bt-hwsim-usecases.sh list
       test-bt-hwsim-usecases.sh write [out-dir]
       test-bt-hwsim-usecases.sh show <case>

This helper writes per-terminal nsh command files for the BT/BLE hwsim
validation matrix.  It does not start the sim binaries by itself; the current
model still expects one sim instance per terminal.

cases:
  bt-basic
  ble-basic
  ble-ip-ping
  ble-ip-reconnect-stress
  ble-ip-iperf-tcp
  ble-ip-iperf-tcp-reverse
  ble-ip-iperf-udp
  ble-ip-iperf-udp-reverse
  hci-bredr-medium
  l2cap-native-basic
  hci-le-lifecycle
  hci-le-reconnect-stress
  hci-le-medium
  hci-le-pairing
  mgmt-control
  bluez-mgmt-control
  bluez-mgmt-pair-noio
  bluez-mgmt-user-confirm
  bluez-mgmt-user-confirm-neg
  bluez-mgmt-passkey
  bluez-mgmt-passkey-neg
  bluez-mgmt-cancel-pair
  bluez-mgmt-cancel-pair-pending
  bluez-mgmt-pair-unpair
  bluez-mgmt-lifecycle
  bluez-mgmt-reconnect-stress
  bluez-mgmt-error-path
  bluez-daemon-smoke
  bluez-daemon-reconnect-stress
  bluez-daemon-device-policy
  bluez-daemon-discovery-peer
  bluez-daemon-pairing-matrix
  bluez-daemon-mgmt-full-lifecycle
  bluez-btmon-monitor
  bluez-hciioctl-basic
  bluez-hciraw-command
  bluez-hciuser-command
  bluez-hciuser-monitor
  bluez-hciuser-sequence-monitor
  bluez-hciuser-error-monitor
  bluez-hciuser-init-sequence-monitor
  bluez-hciuser-full-abi
  bluez-hciuser-adv-scan-medium
  mgmt-noio
  mgmt-confirm
  mgmt-passkey
  bnep-session
  bneptest-fd-probe
  bluez-bneptest-fd-handoff
  bluez-bneptest-ping
  bluez-network-ping
  bluez-network-error-path
  bluez-network-iperf-tcp
  bluez-network-iperf-tcp-reverse
  bluez-network-iperf-udp
  bluez-network-iperf-udp-reverse
  bluez-network-iperf-tcp-soak
  bluez-network-reconnect
  bluez-network-reconnect-stress
  bluez-bneptest-iperf-tcp
  bluez-bneptest-iperf-tcp-reverse
  bluez-bneptest-iperf-udp
  bluez-bneptest-iperf-udp-reverse
  bluez-bneptest-reconnect
  bluez-bneptest-reconnect-stress
  bneptest-ping
  bneptest-reconnect
  bneptest-reconnect-stress
  bnep-ping
  bnep-iperf-tcp
  bnep-iperf-tcp-reverse
  bnep-iperf-udp
  bnep-iperf-udp-reverse
  a2dp
  a2dp-extended
  bluez-a2dp-signaling
  bluez-a2dp-signaling-native
  bluez-a2dp-transaction
  bluez-a2dp-transaction-native
  bluez-a2dp-transaction-reconnect-native
  bluez-a2dp-extended
  a2dp-media
  bluez-a2dp-media
  bluez-a2dp-profile
  bluez-a2dp-profile-reconnect
  bluez-a2dp-extended-profile
  bluez-a2dp-transport
  bluez-a2dp-endpoint-transport
  bluez-a2dp-sbc-codec-transport
  bluez-a2dp-sbc-codec-extended
  bluez-a2dp-sbc-codec-abort
  bluez-a2dp-sbc-codec-reconnect
  bluez-a2dp-transport-reconnect
  bluez-a2dp-transport-bidir
  bluez-a2dp-transport-bidir-teardown
  bluez-a2dp-avrcp-control
  bluez-a2dp-avrcp-browsing
  bluez-a2dp-avrcp-notification
  bluez-a2dp-avrcp-absolute-volume
  bluez-a2dp-avrcp-metadata
  bluez-a2dp-avrcp-player-settings-list
  bluez-a2dp-avrcp-player-settings-values
  bluez-a2dp-avrcp-player-settings-value-text
  bluez-a2dp-avrcp-player-settings-notification
  bluez-a2dp-avrcp-player-settings-error
  bluez-a2dp-avrcp-addressed-player
  bluez-a2dp-avrcp-player-settings
  bluez-a2dp-avrcp-player-settings-set
  bluez-daemon-a2dp-full
  bluez-daemon-a2dp-reconnect-full
  le-audio
  bluez-le-audio
  bluez-le-audio-profile
  bluez-le-audio-broadcast-restart
  bluez-le-audio-unicast-profile
  bluez-le-audio-transport
  bluez-le-audio-transport-reconnect
  bluez-le-audio-transport-bidir
  bluez-le-audio-transport-bidir-disable
  bluez-le-audio-transport-bidir-qos-update
  bluez-le-audio-transport-bidir-qos-reject
  bluez-le-audio-transport-bidir-qos-cancel
  bluez-le-audio-transport-bidir-release-reconfig
  bluez-le-audio-transport-bidir-reconnect
  bluez-le-audio-full-lifecycle
EOF
}

case_names() {
  cat <<'EOF'
bt-basic
ble-basic
ble-ip-ping
ble-ip-reconnect-stress
ble-ip-iperf-tcp
ble-ip-iperf-tcp-reverse
ble-ip-iperf-udp
ble-ip-iperf-udp-reverse
hci-bredr-medium
l2cap-native-basic
hci-le-lifecycle
hci-le-reconnect-stress
hci-le-medium
hci-le-pairing
mgmt-control
bluez-mgmt-control
bluez-mgmt-pair-noio
bluez-mgmt-user-confirm
bluez-mgmt-user-confirm-neg
bluez-mgmt-passkey
bluez-mgmt-passkey-neg
bluez-mgmt-cancel-pair
bluez-mgmt-cancel-pair-pending
bluez-mgmt-pair-unpair
bluez-mgmt-lifecycle
bluez-mgmt-reconnect-stress
bluez-mgmt-error-path
bluez-daemon-smoke
bluez-daemon-reconnect-stress
bluez-daemon-device-policy
bluez-daemon-discovery-peer
bluez-daemon-pairing-matrix
bluez-daemon-mgmt-full-lifecycle
bluez-btmon-monitor
bluez-hciioctl-basic
bluez-hciraw-command
bluez-hciuser-command
bluez-hciuser-monitor
bluez-hciuser-sequence-monitor
bluez-hciuser-error-monitor
bluez-hciuser-init-sequence-monitor
bluez-hciuser-full-abi
bluez-hciuser-adv-scan-medium
mgmt-noio
mgmt-confirm
mgmt-passkey
bnep-session
bneptest-fd-probe
bluez-bneptest-fd-handoff
bluez-bneptest-ping
bluez-network-ping
bluez-network-daemon-profile
bluez-network-daemon-role-matrix
bluez-network-daemon-full-lifecycle
bluez-network-error-path
bluez-network-iperf-tcp
bluez-network-iperf-tcp-reverse
bluez-network-iperf-udp
bluez-network-iperf-udp-reverse
bluez-network-iperf-tcp-soak
bluez-network-reconnect
bluez-network-reconnect-stress
bluez-bneptest-iperf-tcp
bluez-bneptest-iperf-tcp-reverse
bluez-bneptest-iperf-udp
bluez-bneptest-iperf-udp-reverse
bluez-bneptest-reconnect
bluez-bneptest-reconnect-stress
bneptest-ping
bneptest-reconnect
bneptest-reconnect-stress
bnep-ping
bnep-iperf-tcp
bnep-iperf-tcp-reverse
bnep-iperf-udp
bnep-iperf-udp-reverse
a2dp
a2dp-extended
bluez-a2dp-signaling
bluez-a2dp-signaling-native
bluez-a2dp-transaction
bluez-a2dp-transaction-native
bluez-a2dp-transaction-reconnect-native
bluez-a2dp-extended
a2dp-media
bluez-a2dp-media
bluez-a2dp-profile
bluez-a2dp-profile-reconnect
bluez-a2dp-extended-profile
bluez-a2dp-transport
bluez-a2dp-endpoint-transport
bluez-a2dp-sbc-codec-transport
bluez-a2dp-sbc-codec-extended
bluez-a2dp-sbc-codec-abort
bluez-a2dp-sbc-codec-reconnect
bluez-a2dp-transport-reconnect
bluez-a2dp-transport-bidir
bluez-a2dp-transport-bidir-teardown
bluez-a2dp-avrcp-control
bluez-a2dp-avrcp-browsing
bluez-a2dp-avrcp-notification
bluez-a2dp-avrcp-absolute-volume
bluez-a2dp-avrcp-metadata
bluez-a2dp-avrcp-player-settings-list
bluez-a2dp-avrcp-player-settings-values
bluez-a2dp-avrcp-player-settings-value-text
bluez-a2dp-avrcp-player-settings-notification
bluez-a2dp-avrcp-player-settings-error
bluez-a2dp-avrcp-addressed-player
bluez-a2dp-avrcp-player-settings
bluez-a2dp-avrcp-player-settings-set
bluez-daemon-a2dp-full
bluez-daemon-a2dp-reconnect-full
le-audio
bluez-le-audio
bluez-le-audio-profile
bluez-le-audio-broadcast-restart
bluez-le-audio-unicast-profile
bluez-le-audio-transport
bluez-le-audio-transport-reconnect
bluez-le-audio-transport-bidir
bluez-le-audio-transport-bidir-disable
bluez-le-audio-transport-bidir-qos-update
bluez-le-audio-transport-bidir-qos-reject
bluez-le-audio-transport-bidir-qos-cancel
bluez-le-audio-transport-bidir-release-reconfig
bluez-le-audio-transport-bidir-reconnect
bluez-le-audio-full-lifecycle
EOF
}

case_bt_basic() {
  local out_dir="$1"

  cat >"${out_dir}/bt-basic.bt1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl scan bredr
btctl connect 2
btctl pair 2
btctl l2cap-connect 2 0x1001
btctl l2cap-echo 2 bt-basic-echo
btctl state
btctl events
EOF

  cat >"${out_dir}/bt-basic.bt2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl poll ctrl
btctl poll acl
btctl events
btctl state
EOF
}

case_ble_basic() {
  local out_dir="$1"

  cat >"${out_dir}/ble-basic.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl state
btctl events
btctl poll acl
btctl events
EOF

  cat >"${out_dir}/ble-basic.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 4
btctl pair 4
btctl gatt-read 4 0x0001
btctl gatt-write 4 0x0001 ble-basic-write
btctl state
btctl events
EOF
}

case_ble_ip_ping() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-ping.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
btctl upstream hci-pump 9000 &
sleep 1
ping6 -c 2 -W 5000 fc00::2
btctl upstream 6lowpan-status
sleep 5
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
EOF

  cat >"${out_dir}/ble-ip-ping.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 3
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
btctl upstream hci-pump 9000 &
sleep 1
ping6 -c 2 -W 5000 fc00::1
btctl upstream 6lowpan-status
sleep 5
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
EOF
}

case_ble_ip_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-reconnect-stress.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl upstream hci-pump 24000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
sleep 2
ping6 -c 1 -W 5000 fc00::2
sleep 1
ping6 -c 2 -W 5000 fc00::2
btctl upstream 6lowpan-status
sleep 8
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
sleep 2
ping6 -c 1 -W 5000 fc00::2
sleep 1
ping6 -c 2 -W 5000 fc00::2
btctl upstream 6lowpan-status
sleep 8
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
sleep 2
ping6 -c 1 -W 5000 fc00::2
sleep 1
ping6 -c 2 -W 5000 fc00::2
btctl upstream 6lowpan-status
sleep 8
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
EOF

  cat >"${out_dir}/ble-ip-reconnect-stress.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 3
btctl upstream hci-pump 24000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
sleep 8
ping6 -c 1 -W 5000 fc00::1
sleep 1
ping6 -c 2 -W 5000 fc00::1
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
sleep 8
ping6 -c 1 -W 5000 fc00::1
sleep 1
ping6 -c 2 -W 5000 fc00::1
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
sleep 8
ping6 -c 1 -W 5000 fc00::1
sleep 1
ping6 -c 2 -W 5000 fc00::1
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
EOF
}

case_ble_ip_iperf_tcp() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-iperf-tcp.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -B fc00::1 -i 1 -t 8 &
sleep 10
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF

  cat >"${out_dir}/ble-ip-iperf-tcp.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 3
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
sleep 2
ping6 -c 1 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -i 1 -t 4
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF
}

case_ble_ip_iperf_tcp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-iperf-tcp-reverse.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 3
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -B fc00::2 -i 1 -t 8 &
sleep 10
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF

  cat >"${out_dir}/ble-ip-iperf-tcp-reverse.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
sleep 2
ping6 -c 1 -W 5000 fc00::2
iperf -V -c fc00::2 -B fc00::1 -i 1 -t 4
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF
}

case_ble_ip_iperf_udp() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-iperf-udp.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::1 -i 1 -t 8 &
sleep 9
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF

  cat >"${out_dir}/ble-ip-iperf-udp.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 3
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
sleep 2
ping6 -c 1 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -u -i 1 -t 4
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF
}

case_ble_ip_iperf_udp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-iperf-udp-reverse.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl scan le
btctl connect 3
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::2 -i 1 -t 8 &
sleep 9
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF

  cat >"${out_dir}/ble-ip-iperf-udp-reverse.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 22000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
sleep 2
ping6 -c 1 -W 5000 fc00::2
iperf -V -c fc00::2 -B fc00::1 -u -i 1 -t 4
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
EOF
}

case_hci_le_lifecycle() {
  local out_dir="$1"

  cat >"${out_dir}/hci-le-lifecycle.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream status
btctl upstream hci-connect-le 2
btctl upstream status
btctl upstream hci-disconnect-le 2
btctl upstream status
EOF
}

case_hci_bredr_medium() {
  local out_dir="$1"

  cat >"${out_dir}/hci-bredr-medium.bt1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl upstream mgmt-listen
btctl upstream status
btctl upstream hci-connect-br 2
btctl upstream mgmt-read
btctl upstream status
sleep 1
btctl upstream hci-disconnect-br 2
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
sleep 1
EOF

  cat >"${out_dir}/hci-bredr-medium.bt2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl upstream mgmt-listen
btctl state
sleep 1
btctl scan bredr
btctl events
btctl upstream mgmt-read
btctl upstream status
sleep 1
btctl scan bredr
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
btctl state
EOF
}

case_l2cap_native_basic() {
  local out_dir="$1"

  cat >"${out_dir}/l2cap-native-basic.bt1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl upstream mgmt-listen
btctl upstream hci-connect-br 2
btctl upstream mgmt-read
btctl upstream l2cap-bind 0x1001 0x0040 0x0052
btctl upstream l2cap-connect 0x1001 0x0040
sleep 1
btctl upstream l2cap-write 01 02 03 04 05 06 07 08
sleep 1
btctl upstream status
btctl upstream l2cap-close
btctl upstream hci-disconnect-br 2
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
EOF

  cat >"${out_dir}/l2cap-native-basic.bt2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl upstream mgmt-listen
btctl upstream l2cap-bind 0x1001 0x0040 0x0052
btctl upstream l2cap-listen 1
sleep 1
btctl scan bredr
btctl events
btctl upstream mgmt-read
btctl upstream l2cap-recv 32
btctl upstream status
btctl upstream l2cap-close
btctl scan bredr
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
EOF
}

case_hci_le_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/hci-le-reconnect-stress.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream status
btctl upstream hci-connect-le 2
btctl upstream status
btctl upstream hci-disconnect-le 2
btctl upstream status
btctl upstream hci-connect-le 2
btctl upstream status
btctl upstream hci-disconnect-le 2
btctl upstream status
btctl upstream hci-connect-le 2
btctl upstream status
btctl upstream hci-disconnect-le 2
btctl upstream status
EOF
}

case_hci_le_medium() {
  local out_dir="$1"

  cat >"${out_dir}/hci-le-medium.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream mgmt-listen
btctl upstream status
btctl upstream hci-connect-le 4
btctl upstream mgmt-read
btctl upstream status
sleep 1
btctl upstream hci-disconnect-le 4
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
sleep 1
EOF

  cat >"${out_dir}/hci-le-medium.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream mgmt-listen
btctl state
sleep 1
btctl scan le
btctl events
btctl upstream mgmt-read
btctl upstream status
sleep 1
btctl scan le
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
btctl state
EOF
}

case_hci_le_pairing() {
  local out_dir="$1"

  cat >"${out_dir}/hci-le-pairing.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream mgmt-listen
btctl upstream hci-connect-le 4
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0018 0 3
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 4
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream status
sleep 1
btctl upstream hci-disconnect-le 4
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
EOF

  cat >"${out_dir}/hci-le-pairing.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream mgmt-listen
sleep 1
btctl scan le
btctl events
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0018 0 3
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 3
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream status
sleep 1
btctl scan le
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
EOF
}

case_mgmt_noio() {
  local out_dir="$1"

  cat >"${out_dir}/mgmt-noio.ble1.nsh" <<'EOF'
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 3
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream mgmt-close
EOF
}

case_mgmt_control() {
  local out_dir="$1"

  cat >"${out_dir}/mgmt-control.ble1.nsh" <<'EOF'
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0001 0xffff
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0002 0xffff
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0003 0xffff
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0004 0
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0005 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0007 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0006 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0009 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x000d 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0029 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x002a 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0023 0 1
btctl upstream mgmt-read
btctl upstream mgmt-poll-discovery 8
btctl upstream mgmt-send 0x0024 0 1
btctl upstream mgmt-read
btctl upstream status
btctl upstream mgmt-close
EOF
}

case_bluez_mgmt_control() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-control.ble1.nsh" <<'EOF'
bluezmgmt control
btctl upstream status
EOF
}

case_bluez_mgmt_pair_noio() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-pair-noio.ble1.nsh" <<'EOF'
bluezmgmt pair-noio
btctl upstream status
EOF
}

case_bluez_mgmt_user_confirm() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-user-confirm.ble1.nsh" <<'EOF'
bluezmgmt user-confirm
btctl upstream status
EOF
}

case_bluez_mgmt_user_confirm_neg() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-user-confirm-neg.ble1.nsh" <<'EOF'
bluezmgmt user-confirm-neg
btctl upstream status
EOF
}

case_bluez_mgmt_passkey() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-passkey.ble1.nsh" <<'EOF'
bluezmgmt passkey
btctl upstream status
EOF
}

case_bluez_mgmt_passkey_neg() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-passkey-neg.ble1.nsh" <<'EOF'
bluezmgmt passkey-neg
btctl upstream status
EOF
}

case_bluez_mgmt_cancel_pair() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-cancel-pair.ble1.nsh" <<'EOF'
bluezmgmt cancel-pair
btctl upstream status
EOF
}

case_bluez_mgmt_cancel_pair_pending() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-cancel-pair-pending.ble1.nsh" <<'EOF'
bluezmgmt cancel-pair-pending
btctl upstream status
EOF
}

case_bluez_mgmt_pair_unpair() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-pair-unpair.ble1.nsh" <<'EOF'
bluezmgmt pair-unpair
btctl upstream status
EOF
}

case_bluez_mgmt_lifecycle() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-lifecycle.ble1.nsh" <<'EOF'
bluezmgmt lifecycle
btctl upstream status
EOF
}

case_bluez_mgmt_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-reconnect-stress.ble1.nsh" <<'EOF'
bluezmgmt reconnect-stress 3
btctl upstream status
EOF
}

case_bluez_mgmt_error_path() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-error-path.ble1.nsh" <<'EOF'
bluezmgmt error-path
btctl upstream status
EOF
}

case_bluez_daemon_smoke() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-smoke.ble1.nsh" <<'EOF'
bluezdaemon smoke
btctl upstream status
EOF
}

case_bluez_daemon_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-reconnect-stress.ble1.nsh" <<'EOF'
bluezdaemon reconnect-stress 3
btctl upstream status
EOF
}

case_bluez_daemon_device_policy() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-device-policy.ble1.nsh" <<'EOF'
bluezdaemon device-policy
btctl upstream status
EOF
}

case_bluez_daemon_discovery_peer() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-discovery-peer.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl state
sleep 2
btctl advertise start
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-discovery-peer.ble1.nsh" <<'EOF'
sleep 4
bluezdaemon discovery-peer
btctl upstream status
EOF
}

case_bluez_daemon_pairing_matrix() {
    local out_dir="$1"

    cat >"${out_dir}/bluez-daemon-pairing-matrix.ble1.nsh" <<'EOF'
bluezdaemon pairing-matrix
btctl upstream status
EOF
}

case_bluez_daemon_mgmt_full_lifecycle() {
    local out_dir="$1"

    cat >"${out_dir}/bluez-daemon-mgmt-full-lifecycle.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl state
sleep 10
btctl advertise start
btctl state
sleep 10
EOF

    cat >"${out_dir}/bluez-daemon-mgmt-full-lifecycle.ble1.nsh" <<'EOF'
sleep 4
bluezdaemon discovery-peer
btctl upstream status
bluezdaemon smoke
btctl upstream status
bluezdaemon device-policy
btctl upstream status
bluezdaemon pairing-matrix
btctl upstream status
bluezdaemon reconnect-stress 3
btctl upstream status
btctl state
EOF
}

case_bluez_btmon_monitor() {
    local out_dir="$1"

    cat >"${out_dir}/bluez-btmon-monitor.ble1.nsh" <<'EOF'
bluezbtmon control
btctl upstream status
EOF
}

case_bluez_hciioctl_basic() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciioctl-basic.ble1.nsh" <<'EOF'
bluezhciioctl basic
btctl upstream status
EOF
}

case_bluez_hciraw_command() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciraw-command.ble1.nsh" <<'EOF'
bluezhciraw command
btctl upstream status
EOF
}

case_bluez_hciuser_command() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-command.ble1.nsh" <<'EOF'
bluezhciraw user-command
btctl upstream status
EOF
}

case_bluez_hciuser_monitor() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-monitor.ble1.nsh" <<'EOF'
bluezhciraw user-command-monitor
btctl upstream status
EOF
}

case_bluez_hciuser_sequence_monitor() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-sequence-monitor.ble1.nsh" <<'EOF'
bluezhciraw user-command-sequence-monitor
btctl upstream status
EOF
}

case_bluez_hciuser_error_monitor() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-error-monitor.ble1.nsh" <<'EOF'
bluezhciraw user-command-error-monitor
btctl upstream status
EOF
}

case_bluez_hciuser_init_sequence_monitor() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-init-sequence-monitor.ble1.nsh" <<'EOF'
bluezhciraw user-command-init-sequence-monitor
btctl upstream status
EOF
}

case_bluez_hciuser_full_abi() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-full-abi.ble1.nsh" <<'EOF'
bluezhciraw user-command-sequence-monitor
btctl upstream status
bluezhciraw user-command-error-monitor
btctl upstream status
bluezhciraw user-command-init-sequence-monitor
btctl upstream status
btctl state
EOF
}

case_bluez_hciuser_adv_scan_medium() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hciuser-adv-scan-medium.ble1.nsh" <<'EOF'
bluezhciraw user-advertise-enable
btctl upstream status
sleep 1
EOF

  cat >"${out_dir}/bluez-hciuser-adv-scan-medium.ble2.nsh" <<'EOF'
sleep 1
bluezhciraw user-scan-report
btctl upstream status
EOF
}

case_mgmt_confirm() {
  local out_dir="$1"

  cat >"${out_dir}/mgmt-confirm.ble1.nsh" <<'EOF'
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x001c 0 1
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream mgmt-close
EOF
}

case_mgmt_passkey() {
  local out_dir="$1"

  cat >"${out_dir}/mgmt-passkey.ble1.nsh" <<'EOF'
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 2
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x001e 0 1
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream mgmt-close
EOF
}

case_bnep_session() {
  local out_dir="$1"

  cat >"${out_dir}/bnep-session.bt1.nsh" <<'EOF'
btctl upstream l2cap-bind 0x0019 0x0041 0x0040
btctl upstream l2cap-connect 0x0019 0x0041
btctl upstream bnep-ioctl suppfeat
btctl upstream bnep-ioctl connadd l2cap
btctl upstream bnep-ioctl connlist
btctl upstream bnep-ioctl conninfo l2cap
btctl upstream status
btctl upstream bnep-ioctl conndel l2cap
btctl upstream bnep-ioctl connlist
btctl upstream l2cap-close
btctl upstream status
EOF
}

case_bneptest_fd_probe() {
  local out_dir="$1"

  cat >"${out_dir}/bneptest-fd-probe.bt1.nsh" <<'EOF'
btbneptest fd-probe
sleep 1
btctl upstream status
EOF
}

case_bluez_bneptest_fd_handoff() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-fd-handoff.bt1.nsh" <<'EOF'
bluezbneptest fd-handoff
sleep 1
btctl upstream status
EOF
}

case_bluez_bneptest_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-ping.bt1.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
sleep 4
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-bneptest-ping.bt2.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
sleep 4
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
EOF
}

case_bluez_network_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-ping.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
sleep 4
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-ping.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
sleep 4
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_error_path() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-error-path.bt1.nsh" <<'EOF'
blueznetwork error-path
btctl upstream status
EOF
}

case_bluez_network_daemon_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-daemon-profile.bt1.nsh" <<'EOF'
blueznetwork daemon-profile register
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
sleep 4
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path panu
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-daemon-profile.bt2.nsh" <<'EOF'
blueznetwork daemon-profile register
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
sleep 4
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
EOF
}

case_bluez_network_daemon_role_matrix() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-daemon-role-matrix.bt1.nsh" <<'EOF'
blueznetwork daemon-profile register
btctl upstream hci-pump 30000 &
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-daemon-role-matrix.bt2.nsh" <<'EOF'
blueznetwork daemon-profile register
btctl upstream hci-pump 30000 &
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
EOF
}

case_bluez_network_daemon_full_lifecycle() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-daemon-full-lifecycle.bt1.nsh" <<'EOF'
blueznetwork daemon-profile register
btctl upstream hci-pump 45000 &
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path gn
btctl upstream status
blueznetwork daemon-profile unregister
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-daemon-full-lifecycle.bt2.nsh" <<'EOF'
blueznetwork daemon-profile register
btctl upstream hci-pump 45000 &
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path panu
btctl upstream status
blueznetwork daemon-profile unregister
btctl upstream status
EOF
}

case_bluez_network_iperf_tcp() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-iperf-tcp.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -i 1 -t 6 &
sleep 7
btctl upstream status
blueznetwork disconnect
EOF

  cat >"${out_dir}/bluez-network-iperf-tcp.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.1 -i 1 -t 4
btctl upstream status
blueznetwork disconnect
EOF
}

case_bluez_network_iperf_tcp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-iperf-tcp-reverse.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -i 1 -t 6 &
sleep 7
btctl upstream status
blueznetwork disconnect
EOF

  cat >"${out_dir}/bluez-network-iperf-tcp-reverse.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.2 -i 1 -t 4
btctl upstream status
blueznetwork disconnect
EOF
}

case_bluez_network_iperf_udp() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-iperf-udp.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
blueznetwork disconnect
EOF

  cat >"${out_dir}/bluez-network-iperf-udp.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.1 -u -i 1 -t 4
btctl upstream status
blueznetwork disconnect
EOF
}

case_bluez_network_iperf_udp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-iperf-udp-reverse.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
blueznetwork disconnect
EOF

  cat >"${out_dir}/bluez-network-iperf-udp-reverse.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.2 -u -i 1 -t 4
btctl upstream status
blueznetwork disconnect
EOF
}

case_bluez_network_iperf_tcp_soak() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-iperf-tcp-soak.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 22000 &
iperf -s -i 2 -t 18 &
sleep 20
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-iperf-tcp-soak.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 22000 &
sleep 1
iperf -c 10.77.0.1 -i 2 -t 16
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-reconnect.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-reconnect.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-reconnect-stress.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
blueznetwork disconnect
sleep 1
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
blueznetwork disconnect
sleep 1
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-reconnect-stress.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
blueznetwork disconnect
sleep 1
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
blueznetwork disconnect
sleep 1
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bneptest_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bneptest-ping.bt1.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.2
sleep 4
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bneptest-ping.bt2.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 6000 &
sleep 1
ping -c 1 -W 5000 10.77.0.1
sleep 4
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
EOF
}

case_bneptest_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bneptest-reconnect.bt1.nsh" <<'EOF'
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 9000 &
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
sleep 1
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bneptest-reconnect.bt2.nsh" <<'EOF'
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 9000 &
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
sleep 1
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
EOF
}

case_bneptest_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/bneptest-reconnect-stress.bt1.nsh" <<'EOF'
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 20000 &
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
sleep 1
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
sleep 1
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bneptest-reconnect-stress.bt2.nsh" <<'EOF'
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 20000 &
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
sleep 1
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
sleep 1
btbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
btbneptest pan-down
sleep 1
btctl upstream status
EOF
}

case_bnep_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bnep-ping.bt1.nsh" <<'EOF'
btctl upstream l2cap-bind 0x0019 0x0041 0x0040
btctl upstream l2cap-connect 0x0019 0x0041
btctl upstream bnep-ioctl connadd l2cap
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
sleep 1
ping -c 2 10.77.0.2
btctl upstream bnep-ioctl conndel l2cap
btctl upstream l2cap-close
EOF

  cat >"${out_dir}/bnep-ping.bt2.nsh" <<'EOF'
btctl upstream l2cap-bind 0x0019 0x0041 0x0040
btctl upstream l2cap-connect 0x0019 0x0041
btctl upstream bnep-ioctl connadd l2cap
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
sleep 1
ping -c 2 10.77.0.1
btctl upstream bnep-ioctl conndel l2cap
btctl upstream l2cap-close
EOF
}

case_bnep_iperf_tcp() {
  local out_dir="$1"

  cat >"${out_dir}/bnep-iperf-tcp.bt1.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -i 1 -t 6
btctl upstream status
btbneptest pan-down
EOF

  cat >"${out_dir}/bnep-iperf-tcp.bt2.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.1 -i 1 -t 4
btctl upstream status
btbneptest pan-down
EOF
}

case_bluez_bneptest_iperf_tcp() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-iperf-tcp.bt1.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -i 1 -t 6
btctl upstream status
bluezbneptest pan-down
EOF

  cat >"${out_dir}/bluez-bneptest-iperf-tcp.bt2.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.1 -i 1 -t 4
btctl upstream status
bluezbneptest pan-down
EOF
}

case_bnep_iperf_tcp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/bnep-iperf-tcp-reverse.bt2.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -i 1 -t 6
btctl upstream status
btbneptest pan-down
EOF

  cat >"${out_dir}/bnep-iperf-tcp-reverse.bt1.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.2 -i 1 -t 4
btctl upstream status
btbneptest pan-down
EOF
}

case_bluez_bneptest_iperf_tcp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-iperf-tcp-reverse.bt2.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -i 1 -t 6
btctl upstream status
bluezbneptest pan-down
EOF

  cat >"${out_dir}/bluez-bneptest-iperf-tcp-reverse.bt1.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.2 -i 1 -t 4
btctl upstream status
bluezbneptest pan-down
EOF
}

case_bnep_iperf_udp() {
  local out_dir="$1"

  cat >"${out_dir}/bnep-iperf-udp.bt1.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
btbneptest pan-down
EOF

  cat >"${out_dir}/bnep-iperf-udp.bt2.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.1 -u -i 1 -t 4
btctl upstream status
btbneptest pan-down
EOF
}

case_bluez_bneptest_iperf_udp() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-iperf-udp.bt1.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
bluezbneptest pan-down
EOF

  cat >"${out_dir}/bluez-bneptest-iperf-udp.bt2.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.1 -u -i 1 -t 4
btctl upstream status
bluezbneptest pan-down
EOF
}

case_bnep_iperf_udp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/bnep-iperf-udp-reverse.bt2.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
btbneptest pan-down
EOF

  cat >"${out_dir}/bnep-iperf-udp-reverse.bt1.nsh" <<'EOF'
btbneptest pan-up
btbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.2 -u -i 1 -t 4
btctl upstream status
btbneptest pan-down
EOF
}

case_bluez_bneptest_iperf_udp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-iperf-udp-reverse.bt2.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
bluezbneptest pan-down
EOF

  cat >"${out_dir}/bluez-bneptest-iperf-udp-reverse.bt1.nsh" <<'EOF'
bluezbneptest pan-up
bluezbneptest status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 9000 &
sleep 1
iperf -c 10.77.0.2 -u -i 1 -t 4
btctl upstream status
bluezbneptest pan-down
EOF
}

case_bluez_bneptest_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-reconnect.bt1.nsh" <<'EOF'
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 9000 &
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
sleep 1
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-bneptest-reconnect.bt2.nsh" <<'EOF'
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 9000 &
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
sleep 1
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
EOF
}

case_bluez_bneptest_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-bneptest-reconnect-stress.bt1.nsh" <<'EOF'
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 20000 &
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
sleep 1
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
sleep 1
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-bneptest-reconnect-stress.bt2.nsh" <<'EOF'
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
btctl upstream hci-pump 20000 &
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
sleep 1
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
sleep 1
bluezbneptest pan-up
btctl upstream status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
bluezbneptest pan-down
sleep 1
btctl upstream status
EOF
}

case_a2dp() {
  local out_dir="$1"

  cat >"${out_dir}/a2dp.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
btctl upstream avdtp-discover 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-getcap 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-setconfig 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-open 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-start 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-suspend 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-close-stream 2
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/a2dp.bt2.nsh" <<'EOF'
btctl upstream avdtp-listen 0x0052
btctl upstream avdtp-auto-rsp-loop 1 7 &
sleep 30
btctl upstream avdtp-close
btctl state
EOF
}

case_a2dp_extended() {
  local out_dir="$1"

  cat >"${out_dir}/a2dp-extended.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-discover 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-getallcap 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-getcap 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-setconfig 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-getconfig 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-open 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-reconfigure 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-delay-report 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-security-control 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-start 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-suspend 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
btctl upstream avdtp-abort 2
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/a2dp-extended.bt2.nsh" <<'EOF'
btctl upstream avdtp-listen 0x0052
btctl upstream avdtp-auto-rsp-loop 1 12 &
sleep 45
btctl upstream avdtp-close
btctl state
EOF
}

case_bluez_a2dp_signaling() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-signaling.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal discover 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal getcap 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal setconfig 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal open 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal start 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal suspend 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal close-stream 2
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-signaling.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7 &
sleep 30
bluezaudio a2dp-signal close
btctl state
EOF
}

case_bluez_a2dp_signaling_native() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-signaling-native.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
btctl upstream status
sleep 5
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-signaling-native.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
btctl upstream status
bluezaudio a2dp-signal close
btctl state
EOF
}

case_bluez_a2dp_transaction() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transaction.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal source-transaction 2
btctl upstream status
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transaction.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
btctl upstream status
bluezaudio a2dp-signal close
btctl state
EOF
}

case_bluez_a2dp_transaction_native() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transaction-native.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal source-transaction 2
btctl upstream status
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transaction-native.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
btctl upstream status
bluezaudio a2dp-signal close
btctl state
EOF
}

case_bluez_a2dp_transaction_reconnect_native() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transaction-reconnect-native.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal source-transaction 2
btctl upstream status
btctl upstream l2cap-close
sleep 8
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal source-transaction 2
btctl upstream status
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transaction-reconnect-native.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
btctl upstream status
bluezaudio a2dp-signal close
sleep 8
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
btctl upstream status
bluezaudio a2dp-signal close
btctl state
EOF
}

case_bluez_a2dp_extended() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-extended.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal discover 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal getallcap 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal getcap 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal setconfig 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal getconfig 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal open 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal reconfigure 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal delay-report 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal security-control 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal start 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal suspend 2
btctl upstream l2cap-close
sleep 1
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
bluezaudio a2dp-signal abort 2
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-extended.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 12 &
sleep 45
bluezaudio a2dp-signal close
btctl state
EOF
}

case_a2dp_media() {
  local out_dir="$1"

  cat >"${out_dir}/a2dp-media.bt1.nsh" <<'EOF'
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0041 0x0052
btctl upstream l2cap-connect 0x0019 0x0041
btaudio upstream-a2dp-source start 2
btctl upstream pump
btctl upstream status
btctl upstream l2cap-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/a2dp-media.bt2.nsh" <<'EOF'
btaudio upstream-a2dp-sink start
btctl upstream pump
btaudio upstream-a2dp-sink read
btctl upstream status
btaudio upstream-a2dp-sink stop
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF
}

case_bluez_a2dp_media() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-media.bt1.nsh" <<'EOF'
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0041 0x0052
btctl upstream l2cap-connect 0x0019 0x0041
bluezaudio a2dp-source start 2
btctl upstream pump
btctl upstream status
btctl upstream l2cap-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-media.bt2.nsh" <<'EOF'
bluezaudio a2dp-sink start 1
btctl upstream pump
bluezaudio a2dp-sink read
btctl upstream status
bluezaudio a2dp-sink stop
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF
}

case_bluez_a2dp_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-profile.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal discover 2
sleep 1
bluezaudio a2dp-signal getcap 2
sleep 1
bluezaudio a2dp-signal setconfig 2
sleep 1
bluezaudio a2dp-signal open 2
sleep 1
bluezaudio a2dp-signal start 2
sleep 1
bluezaudio a2dp-signal suspend 2
sleep 1
bluezaudio a2dp-signal close-stream 2
btctl upstream l2cap-close
sleep 5
btctl upstream l2cap-bind 0x0019 0x0041 0x0052
btctl upstream l2cap-connect 0x0019 0x0041
bluezaudio a2dp-source start 2
btctl upstream pump
btctl upstream status
btctl upstream l2cap-close
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-profile.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
bluezaudio a2dp-signal close
bluezaudio a2dp-sink start 1
btctl upstream pump
bluezaudio a2dp-sink read
btctl upstream status
bluezaudio a2dp-sink stop
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF
}

case_bluez_a2dp_profile_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-profile-reconnect.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal source-transaction 2
btctl upstream l2cap-close
sleep 2
btctl upstream l2cap-bind 0x0019 0x0041 0x0052
btctl upstream l2cap-connect 0x0019 0x0041
bluezaudio a2dp-source start 2
btctl upstream pump
btctl upstream status
btctl upstream l2cap-close
sleep 5
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal source-transaction 2
btctl upstream l2cap-close
sleep 5
btctl upstream l2cap-bind 0x0019 0x0041 0x0052
btctl upstream l2cap-connect 0x0019 0x0041
bluezaudio a2dp-source start 2
btctl upstream pump
btctl upstream status
btctl upstream l2cap-close
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-profile-reconnect.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
bluezaudio a2dp-signal close
bluezaudio a2dp-sink start 1
btctl upstream pump
bluezaudio a2dp-sink read
btctl upstream status
bluezaudio a2dp-sink stop
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 7
bluezaudio a2dp-signal close
bluezaudio a2dp-sink start 1
btctl upstream pump
bluezaudio a2dp-sink read
btctl upstream status
bluezaudio a2dp-sink stop
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF
}

case_bluez_a2dp_extended_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-extended-profile.bt1.nsh" <<'EOF'
sleep 1
btctl upstream hci-connect-br 2
btctl upstream l2cap-bind 0x0019 0x0040 0x0052
btctl upstream l2cap-connect 0x0019 0x0040
sleep 1
bluezaudio a2dp-signal discover 2
sleep 1
bluezaudio a2dp-signal getallcap 2
sleep 1
bluezaudio a2dp-signal getcap 2
sleep 1
bluezaudio a2dp-signal setconfig 2
sleep 1
bluezaudio a2dp-signal getconfig 2
sleep 1
bluezaudio a2dp-signal open 2
sleep 1
bluezaudio a2dp-signal reconfigure 2
sleep 1
bluezaudio a2dp-signal delay-report 2
sleep 1
bluezaudio a2dp-signal security-control 2
sleep 1
bluezaudio a2dp-signal start 2
sleep 1
bluezaudio a2dp-signal suspend 2
sleep 1
bluezaudio a2dp-signal abort 2
btctl upstream l2cap-close
sleep 5
btctl upstream l2cap-bind 0x0019 0x0041 0x0052
btctl upstream l2cap-connect 0x0019 0x0041
bluezaudio a2dp-source start 2
btctl upstream pump
btctl upstream status
btctl upstream l2cap-close
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-extended-profile.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 12
bluezaudio a2dp-signal close
bluezaudio a2dp-sink start
btctl upstream pump
bluezaudio a2dp-sink read
btctl upstream status
bluezaudio a2dp-sink stop
btctl upstream status
btctl upstream hci-disconnect-br 2
btctl state
EOF
}

case_bluez_a2dp_transport() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transport.bt1.nsh" <<'EOF'
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 1
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transport.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio media-transport a2dp-sink-acquire-read-release 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_endpoint_transport() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-endpoint-transport.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 1
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-endpoint-transport.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio media-transport a2dp-sink-acquire-read-release 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_sbc_codec_transport() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-sbc-codec-transport.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
bluezaudio a2dp-signal source-session-open 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
btctl upstream status
sleep 1
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-sbc-codec-transport.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_sbc_codec_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-sbc-codec-reconnect.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
bluezaudio a2dp-signal source-session-open 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
sleep 1
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
bluezaudio a2dp-endpoint lifecycle source 2
bluezaudio a2dp-signal source-session-open 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
sleep 1
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-sbc-codec-reconnect.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-endpoint clear sink 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_sbc_codec_extended() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-sbc-codec-extended.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
bluezaudio a2dp-signal source-session-open 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getallcap 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal getconfig 2
bluezaudio a2dp-signal reconfigure 2
bluezaudio a2dp-signal delay-report 2
bluezaudio a2dp-signal security-control 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
sleep 1
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-sbc-codec-extended.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_sbc_codec_abort() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-sbc-codec-abort.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
bluezaudio a2dp-signal source-session-open 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal abort 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-sbc-codec-abort.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_transport_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transport-reconnect.bt1.nsh" <<'EOF'
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 1
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
sleep 2
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
sleep 1
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transport-reconnect.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio media-transport a2dp-sink-acquire-read-release 1
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 2
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio media-transport a2dp-sink-acquire-read-release 1
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 2
btctl state
EOF
}

case_bluez_a2dp_transport_bidir() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transport-bidir.bt1.nsh" <<'EOF'
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 2 5
bluezaudio media-transport a2dp-sink-acquire-read-release 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transport-bidir.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio media-transport a2dp-sink-acquire-read-release 1
btctl upstream status
bluezaudio a2dp-signal discover 1
bluezaudio a2dp-signal getcap 1
bluezaudio a2dp-signal setconfig 1
bluezaudio a2dp-signal open 1
bluezaudio a2dp-signal start 1
bluezaudio media-transport a2dp-source-acquire-write-release 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_transport_bidir_teardown() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transport-bidir-teardown.bt1.nsh" <<'EOF'
bluezaudio a2dp-signal discover 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal open 2
bluezaudio a2dp-signal start 2
bluezaudio media-transport a2dp-source-acquire-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal close
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 2 5
bluezaudio media-transport a2dp-sink-acquire-read-release 2
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 2 2 streaming
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-transport-bidir-teardown.bt2.nsh" <<'EOF'
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 5
bluezaudio media-transport a2dp-sink-acquire-read-release 1
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
btctl upstream status
bluezaudio a2dp-signal discover 1
bluezaudio a2dp-signal getcap 1
bluezaudio a2dp-signal setconfig 1
bluezaudio a2dp-signal open 1
bluezaudio a2dp-signal start 1
bluezaudio media-transport a2dp-source-acquire-write-release 1
bluezaudio a2dp-signal suspend 1
bluezaudio a2dp-signal close-stream 1
bluezaudio a2dp-signal close
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_control() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-control.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-play 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-control.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_browsing() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-browsing.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-browse 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-browsing.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-browse-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_notification() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-notification.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-notify 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-notification.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-notify-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_absolute_volume() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-absolute-volume.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-volume 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-absolute-volume.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-volume-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_metadata() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-metadata.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-metadata 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-metadata.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-metadata-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings_list() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-list.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings-list 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-list.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-list-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings_values() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-values.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings-values 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-values.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-values-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings_value_text() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-value-text.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings-value-text 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-value-text.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-value-text-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings_notification() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-notification.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings-notify 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-notification.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-notify-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings_error() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-error.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings-error 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-error.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-error-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_addressed_player() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-addressed-player.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-addressed-player 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-addressed-player.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-addressed-player-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_avrcp_player_settings_set() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-set.bt1.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle source 2
sleep 1
bluezaudio avrcp-control controller-player-settings-set 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-avrcp-player-settings-set.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio avrcp-control target-player-settings-set-respond 1
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-full.bt1.nsh" <<'EOF'
sleep 1
bluezdaemon audio-a2dp-owner source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-full.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-owner sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_reconnect_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-reconnect-full.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-reconnect source 2 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-reconnect-full.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-reconnect sink 1 2
btctl upstream status
btctl state
EOF
}

case_le_audio() {
  local out_dir="$1"

  cat >"${out_dir}/le-audio.ble1.nsh" <<'EOF'
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
btaudio upstream-le-broadcast-source start 0 1
btctl upstream pump
btctl upstream status
btctl upstream iso-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/le-audio.ble2.nsh" <<'EOF'
btaudio upstream-le-broadcast-sink sync 0 1
btctl upstream pump
btaudio upstream-le-broadcast-sink start 0 1
btctl upstream status
btaudio upstream-le-broadcast-sink stop
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio.ble1.nsh" <<'EOF'
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-broadcast-source start 0 1
btctl upstream pump
btctl upstream status
btctl upstream iso-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio.ble2.nsh" <<'EOF'
bluezaudio le-broadcast-sink sync 0 1
btctl upstream pump
bluezaudio le-broadcast-sink start 0 1
btctl upstream status
bluezaudio le-broadcast-sink stop
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-profile.ble1.nsh" <<'EOF'
bluezaudio le-bap-control source-announce 0 1
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-source start 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-bap-control source-stop 0 1
btctl upstream iso-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-profile.ble2.nsh" <<'EOF'
bluezaudio le-bap-control sink-discover 0 1
bluezaudio le-bap-control sink-config 0 1
bluezaudio le-broadcast-sink sync 0 1
bluezaudio le-bap-control sink-sync 0 1
btctl upstream pump
bluezaudio le-broadcast-sink start 0 1
btctl upstream status
bluezaudio le-broadcast-sink stop
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_broadcast_restart() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-broadcast-restart.ble1.nsh" <<'EOF'
bluezaudio le-bap-control source-announce 0 1
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-source start 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-bap-control source-stop 0 1
btctl upstream iso-close
btctl upstream status
bluezaudio le-bap-control source-announce 0 1
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-source start 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-bap-control source-stop 0 1
btctl upstream iso-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-broadcast-restart.ble2.nsh" <<'EOF'
bluezaudio le-bap-control sink-discover 0 1
bluezaudio le-bap-control sink-config 0 1
bluezaudio le-broadcast-sink sync 0 1
bluezaudio le-bap-control sink-sync 0 1
btctl upstream pump
bluezaudio le-broadcast-sink start 0 1
btctl upstream status
bluezaudio le-broadcast-sink stop
btctl upstream status
bluezaudio le-bap-control sink-discover 0 1
bluezaudio le-bap-control sink-config 0 1
bluezaudio le-broadcast-sink sync 0 1
bluezaudio le-bap-control sink-sync 0 1
btctl upstream pump
bluezaudio le-broadcast-sink start 0 1
btctl upstream status
bluezaudio le-broadcast-sink stop
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_unicast_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-unicast-profile.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
btctl upstream iso-bind 1 0x0201
btctl upstream iso-connect 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-unicast-source start 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream iso-close
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-unicast-profile.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-sink sync 0 1
bluezaudio le-unicast-control sink-enable 0 1
btctl upstream pump
bluezaudio le-unicast-sink start 0 1
btctl upstream status
bluezaudio le-unicast-sink stop
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-reconnect.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-reconnect.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir_disable() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir-disable.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir-disable.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir_qos_update() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir-qos-update.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-qos-update 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
bluezaudio le-unicast-control sink-qos-update 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir-qos-update.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
bluezaudio le-unicast-control sink-qos-update 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-qos-update 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir_qos_reject() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir-qos-reject.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-qos-reject 0 1
bluezaudio le-unicast-control source-qos-update 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
bluezaudio le-unicast-control sink-qos-reject 0 2
bluezaudio le-unicast-control sink-qos-update 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir-qos-reject.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
bluezaudio le-unicast-control sink-qos-reject 0 1
bluezaudio le-unicast-control sink-qos-update 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-qos-reject 0 2
bluezaudio le-unicast-control source-qos-update 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir_qos_cancel() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir-qos-cancel.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-qos-cancel 0 1
bluezaudio le-unicast-control source-qos-update 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
bluezaudio le-unicast-control sink-qos-cancel 0 2
bluezaudio le-unicast-control sink-qos-update 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir-qos-cancel.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
bluezaudio le-unicast-control sink-qos-cancel 0 1
bluezaudio le-unicast-control sink-qos-update 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-qos-cancel 0 2
bluezaudio le-unicast-control source-qos-update 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir_release_reconfig() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir-release-reconfig.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-disable 0 1
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
bluezaudio le-unicast-control sink-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 2
bluezaudio le-unicast-control sink-release 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir-release-reconfig.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
bluezaudio le-unicast-control sink-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-disable 0 1
bluezaudio le-unicast-control sink-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-disable 0 2
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_transport_bidir_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-transport-bidir-reconnect.ble1.nsh" <<'EOF'
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-transport-bidir-reconnect.ble2.nsh" <<'EOF'
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_full_lifecycle() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-full-lifecycle.ble1.nsh" <<'EOF'
bluezaudio le-bap-control source-announce 0 1
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-source start 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-bap-control source-stop 0 1
btctl upstream iso-close
btctl upstream status
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-release 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-full-lifecycle.ble2.nsh" <<'EOF'
bluezaudio le-bap-control sink-discover 0 1
bluezaudio le-bap-control sink-config 0 1
bluezaudio le-broadcast-sink sync 0 1
bluezaudio le-bap-control sink-sync 0 1
btctl upstream pump
bluezaudio le-broadcast-sink start 0 1
btctl upstream status
bluezaudio le-broadcast-sink stop
btctl upstream status
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
btctl upstream status
btctl state
EOF
}

write_case() {
  local name="$1"
  local out_dir="$2"

  case "${name}" in
    bt-basic) case_bt_basic "${out_dir}" ;;
    ble-basic) case_ble_basic "${out_dir}" ;;
    ble-ip-ping) case_ble_ip_ping "${out_dir}" ;;
    ble-ip-reconnect-stress) case_ble_ip_reconnect_stress "${out_dir}" ;;
    ble-ip-iperf-tcp) case_ble_ip_iperf_tcp "${out_dir}" ;;
    ble-ip-iperf-tcp-reverse) case_ble_ip_iperf_tcp_reverse "${out_dir}" ;;
    ble-ip-iperf-udp) case_ble_ip_iperf_udp "${out_dir}" ;;
    ble-ip-iperf-udp-reverse) case_ble_ip_iperf_udp_reverse "${out_dir}" ;;
    hci-bredr-medium) case_hci_bredr_medium "${out_dir}" ;;
    l2cap-native-basic) case_l2cap_native_basic "${out_dir}" ;;
    hci-le-lifecycle) case_hci_le_lifecycle "${out_dir}" ;;
    hci-le-reconnect-stress) case_hci_le_reconnect_stress "${out_dir}" ;;
    hci-le-medium) case_hci_le_medium "${out_dir}" ;;
    hci-le-pairing) case_hci_le_pairing "${out_dir}" ;;
    mgmt-control) case_mgmt_control "${out_dir}" ;;
    bluez-mgmt-control) case_bluez_mgmt_control "${out_dir}" ;;
    bluez-mgmt-pair-noio) case_bluez_mgmt_pair_noio "${out_dir}" ;;
    bluez-mgmt-user-confirm) case_bluez_mgmt_user_confirm "${out_dir}" ;;
    bluez-mgmt-user-confirm-neg) case_bluez_mgmt_user_confirm_neg "${out_dir}" ;;
    bluez-mgmt-passkey) case_bluez_mgmt_passkey "${out_dir}" ;;
    bluez-mgmt-passkey-neg) case_bluez_mgmt_passkey_neg "${out_dir}" ;;
    bluez-mgmt-cancel-pair) case_bluez_mgmt_cancel_pair "${out_dir}" ;;
    bluez-mgmt-cancel-pair-pending) case_bluez_mgmt_cancel_pair_pending "${out_dir}" ;;
    bluez-mgmt-pair-unpair) case_bluez_mgmt_pair_unpair "${out_dir}" ;;
    bluez-mgmt-lifecycle) case_bluez_mgmt_lifecycle "${out_dir}" ;;
    bluez-mgmt-reconnect-stress) case_bluez_mgmt_reconnect_stress "${out_dir}" ;;
    bluez-mgmt-error-path) case_bluez_mgmt_error_path "${out_dir}" ;;
    bluez-daemon-smoke) case_bluez_daemon_smoke "${out_dir}" ;;
    bluez-daemon-reconnect-stress) case_bluez_daemon_reconnect_stress "${out_dir}" ;;
    bluez-daemon-device-policy) case_bluez_daemon_device_policy "${out_dir}" ;;
    bluez-daemon-discovery-peer) case_bluez_daemon_discovery_peer "${out_dir}" ;;
    bluez-daemon-pairing-matrix) case_bluez_daemon_pairing_matrix "${out_dir}" ;;
    bluez-daemon-mgmt-full-lifecycle) case_bluez_daemon_mgmt_full_lifecycle "${out_dir}" ;;
    bluez-btmon-monitor) case_bluez_btmon_monitor "${out_dir}" ;;
    bluez-hciioctl-basic) case_bluez_hciioctl_basic "${out_dir}" ;;
    bluez-hciraw-command) case_bluez_hciraw_command "${out_dir}" ;;
    bluez-hciuser-command) case_bluez_hciuser_command "${out_dir}" ;;
    bluez-hciuser-monitor) case_bluez_hciuser_monitor "${out_dir}" ;;
    bluez-hciuser-sequence-monitor) case_bluez_hciuser_sequence_monitor "${out_dir}" ;;
    bluez-hciuser-error-monitor) case_bluez_hciuser_error_monitor "${out_dir}" ;;
    bluez-hciuser-init-sequence-monitor) case_bluez_hciuser_init_sequence_monitor "${out_dir}" ;;
    bluez-hciuser-full-abi) case_bluez_hciuser_full_abi "${out_dir}" ;;
    bluez-hciuser-adv-scan-medium) case_bluez_hciuser_adv_scan_medium "${out_dir}" ;;
    mgmt-noio) case_mgmt_noio "${out_dir}" ;;
    mgmt-confirm) case_mgmt_confirm "${out_dir}" ;;
    mgmt-passkey) case_mgmt_passkey "${out_dir}" ;;
    bnep-session) case_bnep_session "${out_dir}" ;;
    bneptest-fd-probe) case_bneptest_fd_probe "${out_dir}" ;;
    bluez-bneptest-fd-handoff) case_bluez_bneptest_fd_handoff "${out_dir}" ;;
    bluez-bneptest-ping) case_bluez_bneptest_ping "${out_dir}" ;;
    bluez-network-ping) case_bluez_network_ping "${out_dir}" ;;
    bluez-network-daemon-profile) case_bluez_network_daemon_profile "${out_dir}" ;;
    bluez-network-daemon-role-matrix) case_bluez_network_daemon_role_matrix "${out_dir}" ;;
    bluez-network-daemon-full-lifecycle) case_bluez_network_daemon_full_lifecycle "${out_dir}" ;;
    bluez-network-error-path) case_bluez_network_error_path "${out_dir}" ;;
    bluez-network-iperf-tcp) case_bluez_network_iperf_tcp "${out_dir}" ;;
    bluez-network-iperf-tcp-reverse) case_bluez_network_iperf_tcp_reverse "${out_dir}" ;;
    bluez-network-iperf-udp) case_bluez_network_iperf_udp "${out_dir}" ;;
    bluez-network-iperf-udp-reverse) case_bluez_network_iperf_udp_reverse "${out_dir}" ;;
    bluez-network-iperf-tcp-soak) case_bluez_network_iperf_tcp_soak "${out_dir}" ;;
    bluez-network-reconnect) case_bluez_network_reconnect "${out_dir}" ;;
    bluez-network-reconnect-stress) case_bluez_network_reconnect_stress "${out_dir}" ;;
    bluez-bneptest-iperf-tcp) case_bluez_bneptest_iperf_tcp "${out_dir}" ;;
    bluez-bneptest-iperf-tcp-reverse) case_bluez_bneptest_iperf_tcp_reverse "${out_dir}" ;;
    bluez-bneptest-iperf-udp) case_bluez_bneptest_iperf_udp "${out_dir}" ;;
    bluez-bneptest-iperf-udp-reverse) case_bluez_bneptest_iperf_udp_reverse "${out_dir}" ;;
    bluez-bneptest-reconnect) case_bluez_bneptest_reconnect "${out_dir}" ;;
    bluez-bneptest-reconnect-stress) case_bluez_bneptest_reconnect_stress "${out_dir}" ;;
    bneptest-ping) case_bneptest_ping "${out_dir}" ;;
    bneptest-reconnect) case_bneptest_reconnect "${out_dir}" ;;
    bneptest-reconnect-stress) case_bneptest_reconnect_stress "${out_dir}" ;;
    bnep-ping) case_bnep_ping "${out_dir}" ;;
    bnep-iperf-tcp) case_bnep_iperf_tcp "${out_dir}" ;;
    bnep-iperf-tcp-reverse) case_bnep_iperf_tcp_reverse "${out_dir}" ;;
    bnep-iperf-udp) case_bnep_iperf_udp "${out_dir}" ;;
    bnep-iperf-udp-reverse) case_bnep_iperf_udp_reverse "${out_dir}" ;;
    a2dp) case_a2dp "${out_dir}" ;;
    a2dp-extended) case_a2dp_extended "${out_dir}" ;;
    bluez-a2dp-signaling) case_bluez_a2dp_signaling "${out_dir}" ;;
    bluez-a2dp-signaling-native) case_bluez_a2dp_signaling_native "${out_dir}" ;;
    bluez-a2dp-transaction) case_bluez_a2dp_transaction "${out_dir}" ;;
    bluez-a2dp-transaction-native) case_bluez_a2dp_transaction_native "${out_dir}" ;;
    bluez-a2dp-transaction-reconnect-native) case_bluez_a2dp_transaction_reconnect_native "${out_dir}" ;;
    bluez-a2dp-extended) case_bluez_a2dp_extended "${out_dir}" ;;
    a2dp-media) case_a2dp_media "${out_dir}" ;;
    bluez-a2dp-media) case_bluez_a2dp_media "${out_dir}" ;;
    bluez-a2dp-profile) case_bluez_a2dp_profile "${out_dir}" ;;
    bluez-a2dp-profile-reconnect) case_bluez_a2dp_profile_reconnect "${out_dir}" ;;
    bluez-a2dp-extended-profile) case_bluez_a2dp_extended_profile "${out_dir}" ;;
    bluez-a2dp-transport) case_bluez_a2dp_transport "${out_dir}" ;;
    bluez-a2dp-endpoint-transport) case_bluez_a2dp_endpoint_transport "${out_dir}" ;;
    bluez-a2dp-sbc-codec-transport) case_bluez_a2dp_sbc_codec_transport "${out_dir}" ;;
    bluez-a2dp-sbc-codec-extended) case_bluez_a2dp_sbc_codec_extended "${out_dir}" ;;
    bluez-a2dp-sbc-codec-abort) case_bluez_a2dp_sbc_codec_abort "${out_dir}" ;;
    bluez-a2dp-sbc-codec-reconnect) case_bluez_a2dp_sbc_codec_reconnect "${out_dir}" ;;
    bluez-a2dp-transport-reconnect) case_bluez_a2dp_transport_reconnect "${out_dir}" ;;
    bluez-a2dp-transport-bidir) case_bluez_a2dp_transport_bidir "${out_dir}" ;;
    bluez-a2dp-transport-bidir-teardown) case_bluez_a2dp_transport_bidir_teardown "${out_dir}" ;;
    bluez-a2dp-avrcp-control) case_bluez_a2dp_avrcp_control "${out_dir}" ;;
    bluez-a2dp-avrcp-browsing) case_bluez_a2dp_avrcp_browsing "${out_dir}" ;;
    bluez-a2dp-avrcp-notification) case_bluez_a2dp_avrcp_notification "${out_dir}" ;;
    bluez-a2dp-avrcp-absolute-volume) case_bluez_a2dp_avrcp_absolute_volume "${out_dir}" ;;
    bluez-a2dp-avrcp-metadata) case_bluez_a2dp_avrcp_metadata "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings-list) case_bluez_a2dp_avrcp_player_settings_list "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings-values) case_bluez_a2dp_avrcp_player_settings_values "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings-value-text) case_bluez_a2dp_avrcp_player_settings_value_text "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings-notification) case_bluez_a2dp_avrcp_player_settings_notification "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings-error) case_bluez_a2dp_avrcp_player_settings_error "${out_dir}" ;;
    bluez-a2dp-avrcp-addressed-player) case_bluez_a2dp_avrcp_addressed_player "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings) case_bluez_a2dp_avrcp_player_settings "${out_dir}" ;;
    bluez-a2dp-avrcp-player-settings-set) case_bluez_a2dp_avrcp_player_settings_set "${out_dir}" ;;
    bluez-daemon-a2dp-full) case_bluez_daemon_a2dp_full "${out_dir}" ;;
    bluez-daemon-a2dp-reconnect-full) case_bluez_daemon_a2dp_reconnect_full "${out_dir}" ;;
    le-audio) case_le_audio "${out_dir}" ;;
    bluez-le-audio) case_bluez_le_audio "${out_dir}" ;;
    bluez-le-audio-profile) case_bluez_le_audio_profile "${out_dir}" ;;
    bluez-le-audio-broadcast-restart) case_bluez_le_audio_broadcast_restart "${out_dir}" ;;
    bluez-le-audio-unicast-profile) case_bluez_le_audio_unicast_profile "${out_dir}" ;;
    bluez-le-audio-transport) case_bluez_le_audio_transport "${out_dir}" ;;
    bluez-le-audio-transport-reconnect) case_bluez_le_audio_transport_reconnect "${out_dir}" ;;
    bluez-le-audio-transport-bidir) case_bluez_le_audio_transport_bidir "${out_dir}" ;;
    bluez-le-audio-transport-bidir-disable) case_bluez_le_audio_transport_bidir_disable "${out_dir}" ;;
    bluez-le-audio-transport-bidir-qos-update) case_bluez_le_audio_transport_bidir_qos_update "${out_dir}" ;;
    bluez-le-audio-transport-bidir-qos-reject) case_bluez_le_audio_transport_bidir_qos_reject "${out_dir}" ;;
    bluez-le-audio-transport-bidir-qos-cancel) case_bluez_le_audio_transport_bidir_qos_cancel "${out_dir}" ;;
    bluez-le-audio-transport-bidir-release-reconfig) case_bluez_le_audio_transport_bidir_release_reconfig "${out_dir}" ;;
    bluez-le-audio-transport-bidir-reconnect) case_bluez_le_audio_transport_bidir_reconnect "${out_dir}" ;;
    bluez-le-audio-full-lifecycle) case_bluez_le_audio_full_lifecycle "${out_dir}" ;;
    *) return 1 ;;
  esac
}

write_readme() {
  local out_dir="$1"

  cat >"${out_dir}/README.md" <<EOF
# BT/BLE hwsim usecase command files

Generated by:

\`\`\`bash
${script_dir}/test-bt-hwsim-usecases.sh write ${out_dir}
\`\`\`

Run model:

\`\`\`text
terminal 1: build/nuttx-sim-bt1
terminal 2: build/nuttx-sim-bt2
terminal 3: build/nuttx-sim-ble1
terminal 4: build/nuttx-sim-ble2
\`\`\`

Paste the matching \`.nsh\` file into the role terminal named in the file
suffix.  For example, \`bt-basic.bt1.nsh\` belongs in the bt1 terminal.

Expected coverage:

\`\`\`text
bt-basic:     BR/EDR mgmt, scan/connect/pair, L2CAP echo/state/events
ble-basic:    LE mgmt, advertising/scan/connect/pair, GATT read/write
ble-ip-ping:  BLE hwsim 6LoWPAN/IPSP bt0 IPv6 ping6 over ACL public-file medium
ble-ip-reconnect-stress: BLE 6LoWPAN bt0 three up/ping/down lifecycle rounds
ble-ip-iperf-tcp: BLE 6LoWPAN bt0 IPv6 TCP iperf, ble1 server to ble2 client
ble-ip-iperf-tcp-reverse: BLE 6LoWPAN bt0 IPv6 TCP iperf, ble2 server to ble1 client
ble-ip-iperf-udp: BLE 6LoWPAN bt0 IPv6 UDP iperf, ble1 server to ble2 client
ble-ip-iperf-udp-reverse: BLE 6LoWPAN bt0 IPv6 UDP iperf, ble2 server to ble1 client
hci-bredr-medium: BT1 upstream HCI BR/EDR command path writes CTRL medium consumed by BT2
l2cap-native-basic: BR/EDR ACL plus persistent upstream L2CAP socket bind/connect/write/recv
hci-le-lifecycle: upstream hci_conn LE connect/status/disconnect lifecycle
hci-le-reconnect-stress: upstream HCI LE command/event connect/disconnect repeated lifecycle
hci-le-medium: BLE1 upstream HCI command path writes CTRL medium consumed by BLE2
hci-le-pairing: dual BLE HCI/mgmt lifecycle with NoInputNoOutput LTK events
mgmt-noio:    persistent mgmt socket, NoInputNoOutput pair, LTK event
bluez-mgmt-control: BlueZ-style AF_BLUETOOTH mgmt control socket smoke
bluez-mgmt-pair-noio: BlueZ-style AF_BLUETOOTH mgmt NoInputNoOutput pair smoke
bluez-mgmt-user-confirm: BlueZ-style AF_BLUETOOTH mgmt user-confirm pair smoke
bluez-mgmt-user-confirm-neg: BlueZ-style AF_BLUETOOTH mgmt user-confirm reject smoke
bluez-mgmt-passkey: BlueZ-style AF_BLUETOOTH mgmt passkey pair smoke
bluez-mgmt-passkey-neg: BlueZ-style AF_BLUETOOTH mgmt passkey reject smoke
bluez-mgmt-cancel-pair: BlueZ-style AF_BLUETOOTH mgmt cancel-pair no-pending error smoke
bluez-mgmt-cancel-pair-pending: BlueZ-style AF_BLUETOOTH mgmt cancel-pair pending smoke
bluez-mgmt-pair-unpair: BlueZ-style AF_BLUETOOTH mgmt pair then unpair smoke
bluez-mgmt-lifecycle: BlueZ-style AF_BLUETOOTH mgmt pair/get-info/disconnect smoke
bluez-mgmt-reconnect-stress: BlueZ-style AF_BLUETOOTH mgmt three-round reconnect lifecycle
bluez-mgmt-error-path: BlueZ-style AF_BLUETOOTH mgmt invalid parameter smoke
bluez-daemon-smoke: BlueZ bluetoothd-shaped long-lived mgmt control smoke
bluez-daemon-reconnect-stress: BlueZ bluetoothd-shaped three-round mgmt lifecycle
bluez-daemon-discovery-peer: BlueZ daemon-shaped discovery receives peer Device Found
bluez-daemon-pairing-matrix: BlueZ daemon-shaped confirm/passkey/cancel pairing matrix
bluez-daemon-mgmt-full-lifecycle: BlueZ daemon-shaped discovery, pairing, policy, reconnect, and cleanup umbrella
bluez-btmon-monitor: BlueZ-style AF_BLUETOOTH HCI monitor socket smoke
bluez-hciioctl-basic: BlueZ-style AF_BLUETOOTH HCI ioctl smoke
bluez-hciraw-command: BlueZ-style AF_BLUETOOTH HCI raw command socket smoke
mgmt-confirm: persistent mgmt socket, user-confirm pair, pending complete
mgmt-passkey: persistent mgmt socket, passkey pair, pending complete
bnep-session: BTPROTO_BNEP ioctl session lifecycle backed by kept L2CAP socket
bneptest-fd-probe: app-level AF_BLUETOOTH fd + BNEPCONNADD ABI diagnostic
bluez-bneptest-fd-handoff: BlueZ bneptest-compatible fd handoff diagnostic
bluez-bneptest-ping: BlueZ bneptest-compatible BNEPCONNADD fd handoff and ping
bluez-network-ping: BlueZ Network Profile-shaped BNEPCONNADD fd handoff and ping
bluez-network-daemon-profile: BlueZ bluetoothd Network plugin-shaped registration/connect/error lifecycle
bluez-network-daemon-role-matrix: BlueZ Network plugin-shaped PANU/NAP/GN role matrix lifecycle
bluez-network-daemon-full-lifecycle: BlueZ Network plugin-shaped D-Bus ownership, PANU/NAP/GN, error, and release lifecycle
bluez-network-error-path: BlueZ Network Profile-shaped BNEP missing/duplicate connection errors
bluez-network-iperf-tcp: BlueZ Network Profile-shaped BNEP TCP iperf
bluez-network-iperf-tcp-reverse: BlueZ Network Profile-shaped reverse BNEP TCP iperf
bluez-network-iperf-udp: BlueZ Network Profile-shaped BNEP UDP iperf
bluez-network-iperf-udp-reverse: BlueZ Network Profile-shaped reverse BNEP UDP iperf
bluez-network-iperf-tcp-soak: BlueZ Network Profile-shaped longer BNEP TCP iperf soak
bluez-network-reconnect: BlueZ Network Profile-shaped repeated BNEP lifecycle
bluez-network-reconnect-stress: BlueZ Network Profile-shaped three-round BNEP lifecycle
bluez-bneptest-iperf-tcp: BlueZ bneptest-compatible BNEP TCP iperf
bluez-bneptest-iperf-tcp-reverse: BlueZ bneptest-compatible BNEP TCP iperf reverse
bluez-bneptest-iperf-udp: BlueZ bneptest-compatible BNEP UDP iperf
bluez-bneptest-iperf-udp-reverse: BlueZ bneptest-compatible BNEP UDP iperf reverse
bluez-bneptest-reconnect: BlueZ bneptest-compatible two-round BNEP lifecycle
bluez-bneptest-reconnect-stress: BlueZ bneptest-compatible three-round BNEP lifecycle
bneptest-ping: BlueZ bneptest-like BNEPCONNADD kept-L2CAP fd handoff and ping
bneptest-reconnect: repeated BNEPCONNADD/BNEPCONNDEL lifecycle and ping
bneptest-reconnect-stress: three BNEPCONNADD/BNEPCONNDEL lifecycle rounds
bnep-ping:    BNEP hwsim PAN btn0 ping over public-file medium
bnep-iperf-tcp: BNEP hwsim PAN btn0 TCP iperf, bt1 server to bt2 client
bnep-iperf-tcp-reverse: BNEP hwsim PAN btn0 TCP iperf, bt2 server to bt1 client
bnep-iperf-udp: BNEP hwsim PAN btn0 UDP iperf, bt1 server to bt2 client
bnep-iperf-udp-reverse: BNEP hwsim PAN btn0 UDP iperf, bt2 server to bt1 client
a2dp:         upstream L2CAP socket media path for A2DP-like payload
bluez-a2dp-signaling: BlueZ audio adapter AVDTP signaling lifecycle
bluez-a2dp-signaling-native: BlueZ AVDTP signaling responder over native L2CAP receive
bluez-a2dp-transaction: BlueZ AVDTP source transaction over one signaling socket
bluez-a2dp-transaction-native: BlueZ AVDTP source transaction with native L2CAP receive
bluez-a2dp-transaction-reconnect-native: repeated BlueZ AVDTP source transaction with native L2CAP receive
bluez-a2dp-extended: BlueZ audio adapter extended AVDTP signaling lifecycle
bluez-a2dp-media: BlueZ audio adapter A2DP media payload over L2CAP socket path
bluez-a2dp-profile: BlueZ audio adapter AVDTP signaling plus A2DP media lifecycle
bluez-a2dp-profile-reconnect: BlueZ audio adapter repeated AVDTP plus A2DP media lifecycle
bluez-a2dp-extended-profile: BlueZ audio adapter extended AVDTP signaling plus A2DP media lifecycle
bluez-a2dp-transport: BlueZ MediaTransport-shaped acquire/read-write/release over A2DP L2CAP media
bluez-a2dp-endpoint-transport: BlueZ MediaEndpoint SBC config plus MediaTransport-shaped A2DP L2CAP media
bluez-a2dp-sbc-codec-transport: BlueZ A2DP SBC frame encode/decode-shaped media over L2CAP transport
bluez-a2dp-sbc-codec-extended: BlueZ extended AVDTP control plus source-built SBC native media
bluez-a2dp-sbc-codec-abort: BlueZ A2DP SBC endpoint native AVDTP abort lifecycle
bluez-a2dp-sbc-codec-reconnect: Two-round BlueZ A2DP SBC native signaling/media lifecycle reconnect
bluez-a2dp-transport-reconnect: BlueZ MediaTransport-shaped repeated acquire/read-write/release over A2DP L2CAP media
bluez-a2dp-transport-bidir: BlueZ MediaTransport-shaped bidirectional A2DP media over BR/EDR L2CAP
bluez-a2dp-transport-bidir-teardown: BlueZ MediaTransport-shaped bidirectional A2DP media plus suspend/close teardown
bluez-a2dp-avrcp-control: BlueZ MediaControl1-shaped AVRCP play over AVCTP L2CAP control
bluez-a2dp-avrcp-browsing: BlueZ AVRCP browsing GetFolderItems over AVCTP browsing L2CAP
bluez-a2dp-avrcp-notification: BlueZ AVRCP RegisterNotification playback-status over AVCTP L2CAP control
bluez-a2dp-avrcp-absolute-volume: BlueZ AVRCP SetAbsoluteVolume over AVCTP L2CAP control
bluez-a2dp-avrcp-metadata: BlueZ AVRCP GetElementAttributes metadata over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings-list: BlueZ AVRCP list player application settings over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings-values: BlueZ AVRCP list player application setting values over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings-value-text: BlueZ AVRCP player application setting value text over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings-notification: BlueZ AVRCP player application setting changed notification over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings-error: BlueZ AVRCP player application setting invalid attribute rejection over AVCTP L2CAP control
bluez-a2dp-avrcp-addressed-player: BlueZ AVRCP SetAddressedPlayer over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings: BlueZ AVRCP player application settings over AVCTP L2CAP control
bluez-a2dp-avrcp-player-settings-set: BlueZ AVRCP set player application settings over AVCTP L2CAP control
bluez-daemon-a2dp-full: BlueZ daemon-owned A2DP/AVRCP mainloop, D-Bus object ownership, AVDTP, media, codec, and policy path
bluez-daemon-a2dp-reconnect-full: BlueZ daemon-owned two-round A2DP/AVRCP persistent mainloop lifecycle
bluez-daemon-device-policy: BlueZ daemon-shaped add/remove/block/unblock/device-flags/unpair lifecycle
le-audio:     upstream ISO socket media path for LE Audio-like payload
bluez-le-audio: BlueZ audio adapter LE Audio-like payload over ISO socket path
bluez-le-audio-profile: BlueZ audio adapter BAP/PACS/ASCS-shaped control plus ISO payload
bluez-le-audio-broadcast-restart: BlueZ audio adapter repeated BAP broadcast source/sink restart over BIS ISO payload
bluez-le-audio-unicast-profile: BlueZ audio adapter BAP/ASCS-shaped CIS/unicast control plus ISO payload
bluez-le-audio-transport: BlueZ MediaTransport-shaped acquire/read-write/release over CIS ISO payload
bluez-le-audio-transport-reconnect: BlueZ MediaTransport-shaped repeated acquire/read-write/release over CIS ISO payload
bluez-le-audio-transport-bidir: BlueZ MediaTransport-shaped bidirectional CIS ISO payload over two CIS handles
bluez-le-audio-transport-bidir-disable: BlueZ MediaTransport-shaped bidirectional CIS ISO payload plus source/sink disable lifecycle
bluez-le-audio-transport-bidir-qos-update: BlueZ MediaTransport-shaped bidirectional CIS ISO payload across disable, QoS update, and re-enable lifecycle
bluez-le-audio-transport-bidir-qos-reject: BlueZ MediaTransport-shaped bidirectional CIS ISO payload recovery after rejected QoS update
bluez-le-audio-transport-bidir-qos-cancel: BlueZ MediaTransport-shaped bidirectional CIS ISO payload retry after cancelled QoS procedure
bluez-le-audio-transport-bidir-release-reconfig: BlueZ MediaTransport-shaped bidirectional CIS ISO payload after ASE release and reconfiguration
bluez-le-audio-transport-bidir-reconnect: BlueZ MediaTransport-shaped repeated bidirectional CIS ISO payload lifecycle
bluez-le-audio-full-lifecycle: BlueZ LE Audio broadcast BIS plus unicast bidirectional CIS full lifecycle
\`\`\`

Result recording recommendation:

\`\`\`text
Capture each terminal output into logs named <case>.<role>.log.
Record PASS only after the expected command-complete/event/payload lines are
visible in the matching role log.
\`\`\`
EOF
}

cmd="${1:-}"
case "${cmd}" in
  list)
    case_names
    ;;
  write)
    out_dir="${2:-${default_out}}"
    mkdir -p "${out_dir}"
    while IFS= read -r name; do
      write_case "${name}" "${out_dir}"
    done < <(case_names)
    write_readme "${out_dir}"
    printf 'wrote BT/BLE hwsim usecases to %s\n' "${out_dir}"
    ;;
  show)
    name="${2:-}"
    if [ -z "${name}" ]; then
      usage
      exit 2
    fi
    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "${tmp_dir}"' EXIT
    write_case "${name}" "${tmp_dir}"
    for file in "${tmp_dir}/${name}".*.nsh; do
      [ -e "${file}" ] || continue
      printf '%s\n' "--- $(basename "${file}") ---"
      cat "${file}"
    done
    ;;
  *)
    usage
    exit 2
    ;;
esac
