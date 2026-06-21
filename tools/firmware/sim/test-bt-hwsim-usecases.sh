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
  ble-ip-closeout-full
  bluez-ipsp-closeout-full
  bluez-daemon-ipsp-closeout-full
  bluez-net-current-complete-closeout
  bluez-net-upstream-convergence-closeout
  bluez-current-functional-closeout
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
  bluez-mgmt-daemon-bootstrap
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
  bluez-hci-mgmt-socket-closeout-full
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
  bluez-network-iperf-matrix
  bluez-network-frag-ping
  bluez-network-jumbo-ping
  bluez-network-mtu-ping
  bluez-network-mtu-soak
  bluez-network-mtu-reconnect-stress
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
  bluez-a2dp-sbc-codec-concurrent
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
  bluez-daemon-a2dp-dbus-client-full
  bluez-daemon-a2dp-dbus-client-busy
  bluez-daemon-a2dp-full-concurrent
  bluez-daemon-a2dp-full-concurrent-reconnect
  bluez-daemon-a2dp-full-concurrent-soak
  bluez-daemon-a2dp-integrated-profile
  bluez-daemon-a2dp-integrated-reconnect
  bluez-daemon-a2dp-session-ownership
  bluez-daemon-a2dp-error-policy
  bluez-daemon-a2dp-upstream-session
  bluez-daemon-a2dp-upstream-reconnect
  bluez-daemon-a2dp-upstream-transactions
  bluez-daemon-a2dp-media-transport-fd
  bluez-daemon-a2dp-codec-policy
  bluez-daemon-a2dp-closeout-full
  bluez-a2dp-current-complete-closeout
  bluez-a2dp-upstream-convergence-closeout
  bluez-basic-mgmt-flow
  bluez-basic-scan-connect-auth-flow
  bluez-basic-upstream-convergence-closeout
  bluez-hid-hogp-profile-closeout
  bluez-hfp-hsp-profile-closeout
  bluez-obex-pbap-opp-profile-closeout
  bluez-obex-map-mns-profile-closeout
  bluez-obex-ftp-sync-profile-closeout
  bluez-mesh-profile-closeout
  bluez-gatt-profile-closeout
  bluez-asha-profile-closeout
  bluez-obex-bip-profile-closeout
  bluez-print-profile-closeout
  bluez-iap-profile-closeout
  bluez-midi-profile-closeout
  bluez-ranging-profile-closeout
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
  bluez-le-audio-daemon-full-lifecycle
  bluez-le-audio-dbus-client-full
  bluez-le-audio-lc3-codec-transport
  bluez-le-audio-iso-dataplane-soak
  bluez-le-audio-controller-iso-dataplane-soak
  bluez-le-audio-daemon-profile-flow
  bluez-le-audio-controller-daemon-profile-flow
  bluez-le-audio-controller-daemon-profile-reconnect
  bluez-le-audio-controller-daemon-error-recovery
  bluez-le-audio-controller-bap-ascs-error-matrix
  bluez-le-audio-controller-daemon-dbus-ownership
  bluez-le-audio-bap-ascs-dbus-owner-recovery
  bluez-le-audio-controller-dbus-bap-ascs-reconnect
  bluez-le-audio-controller-daemon-full-stack
  bluez-le-audio-controller-daemon-full-stack-reconnect
  bluez-le-audio-controller-daemon-mainloop-cleanup
  bluez-le-audio-daemon-broadcast-profile-flow
  bluez-le-audio-controller-daemon-broadcast-profile-flow
  bluez-le-audio-controller-daemon-broadcast-reconnect
  bluez-le-audio-controller-lc3-bidir
  bluez-le-audio-coordinated-services
  bluez-le-audio-cap-csip-group
  bluez-le-audio-tmap-mcp-ccp-flow
  bluez-le-audio-broadcast-multibis
  bluez-le-audio-broadcast-multibis-reconnect
  bluez-le-audio-bass-scan-delegator
  bluez-le-audio-daemon-integrated-profile
  bluez-le-audio-daemon-integrated-profile-reconnect
  bluez-le-audio-bap-pacs-ascs-session
  bluez-le-audio-bap-pacs-ascs-reconnect-recovery
  bluez-le-audio-bap-pacs-ascs-metadata-reconfig
  bluez-le-audio-codec-qos-policy-matrix
  bluez-le-audio-role-soak
  bluez-le-audio-umbrella
  bluez-le-audio-controller-setup
  bluez-le-audio-controller-reconnect
  bluez-hid-upstream-convergence-closeout
  bluez-gatt-upstream-convergence-closeout
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
ble-ip-closeout-full
bluez-ipsp-closeout-full
bluez-daemon-ipsp-closeout-full
bluez-net-current-complete-closeout
bluez-net-upstream-convergence-closeout
bluez-hid-upstream-convergence-closeout
bluez-gatt-upstream-convergence-closeout
bluez-current-functional-closeout
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
bluez-mgmt-daemon-bootstrap
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
bluez-hci-mgmt-socket-closeout-full
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
bluez-network-closeout-full
bluez-network-error-path
bluez-network-iperf-tcp
bluez-network-iperf-tcp-reverse
bluez-network-iperf-udp
bluez-network-iperf-udp-reverse
bluez-network-iperf-tcp-soak
bluez-network-iperf-matrix
bluez-network-frag-ping
bluez-network-jumbo-ping
bluez-network-mtu-ping
bluez-network-mtu-soak
bluez-network-mtu-reconnect-stress
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
bluez-a2dp-sbc-codec-concurrent
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
bluez-daemon-a2dp-dbus-client-full
bluez-daemon-a2dp-dbus-client-busy
bluez-daemon-a2dp-full-concurrent
bluez-daemon-a2dp-full-concurrent-reconnect
bluez-daemon-a2dp-full-concurrent-soak
bluez-daemon-a2dp-integrated-profile
bluez-daemon-a2dp-integrated-reconnect
bluez-daemon-a2dp-session-ownership
bluez-daemon-a2dp-error-policy
bluez-daemon-a2dp-upstream-session
bluez-daemon-a2dp-upstream-reconnect
bluez-daemon-a2dp-upstream-transactions
bluez-daemon-a2dp-media-transport-fd
bluez-daemon-a2dp-codec-policy
bluez-daemon-a2dp-closeout-full
bluez-a2dp-current-complete-closeout
bluez-a2dp-upstream-convergence-closeout
bluez-basic-mgmt-flow
bluez-basic-scan-connect-auth-flow
bluez-basic-upstream-convergence-closeout
bluez-hid-hogp-profile-closeout
bluez-hfp-hsp-profile-closeout
bluez-obex-pbap-opp-profile-closeout
bluez-obex-map-mns-profile-closeout
bluez-obex-ftp-sync-profile-closeout
bluez-mesh-profile-closeout
bluez-gatt-profile-closeout
bluez-asha-profile-closeout
bluez-obex-bip-profile-closeout
bluez-print-profile-closeout
bluez-iap-profile-closeout
bluez-midi-profile-closeout
bluez-ranging-profile-closeout
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
bluez-le-audio-daemon-full-lifecycle
bluez-le-audio-dbus-client-full
bluez-le-audio-lc3-codec-transport
bluez-le-audio-iso-dataplane-soak
bluez-le-audio-controller-iso-dataplane-soak
bluez-le-audio-daemon-profile-flow
bluez-le-audio-controller-daemon-profile-flow
bluez-le-audio-controller-daemon-profile-reconnect
bluez-le-audio-controller-daemon-error-recovery
bluez-le-audio-controller-bap-ascs-error-matrix
bluez-le-audio-controller-daemon-dbus-ownership
bluez-le-audio-bap-ascs-dbus-owner-recovery
bluez-le-audio-controller-dbus-bap-ascs-reconnect
bluez-le-audio-controller-daemon-full-stack
bluez-le-audio-controller-daemon-full-stack-reconnect
bluez-le-audio-controller-daemon-mainloop-cleanup
bluez-le-audio-daemon-broadcast-profile-flow
bluez-le-audio-controller-daemon-broadcast-profile-flow
bluez-le-audio-controller-daemon-broadcast-reconnect
bluez-le-audio-controller-lc3-bidir
bluez-le-audio-coordinated-services
bluez-le-audio-cap-csip-group
bluez-le-audio-tmap-mcp-ccp-flow
bluez-le-audio-broadcast-multibis
bluez-le-audio-broadcast-multibis-reconnect
bluez-le-audio-bass-scan-delegator
bluez-le-audio-daemon-integrated-profile
bluez-le-audio-daemon-integrated-profile-reconnect
bluez-le-audio-bap-pacs-ascs-session
bluez-le-audio-bap-pacs-ascs-reconnect-recovery
bluez-le-audio-bap-pacs-ascs-metadata-reconfig
bluez-le-audio-codec-qos-policy-matrix
bluez-le-audio-role-soak
bluez-le-audio-umbrella
bluez-le-audio-controller-setup
bluez-le-audio-controller-reconnect
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
btctl state
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
btctl state
btctl scan le
btctl connect 4
btctl pair 4
btctl gatt-read 4 0x0001
btctl gatt-write 4 0x0001 ble-basic-write
btctl state
btctl events
EOF
}

case_bluez_hid_upstream_convergence_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hid-upstream-convergence-closeout.bt1.nsh" <<'EOF'
echo BLUEZHIDUPSTREAM_BEGIN_BT1
bluezhid closeout classic-host 2
btctl upstream ordinary-hidp-socket 0x00a5
echo BLUEZHIDUPSTREAM_DONE_BT1
EOF

  cat >"${out_dir}/bluez-hid-upstream-convergence-closeout.bt2.nsh" <<'EOF'
echo BLUEZHIDUPSTREAM_BEGIN_BT2
bluezhid closeout classic-device 1
btctl upstream ordinary-hidp-socket 0x00a6
echo BLUEZHIDUPSTREAM_DONE_BT2
EOF

  cat >"${out_dir}/bluez-hid-upstream-convergence-closeout.ble1.nsh" <<'EOF'
echo BLUEZHIDUPSTREAM_BEGIN_BLE1
bluezhid closeout hogp-host 4
echo BLUEZHIDUPSTREAM_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-hid-upstream-convergence-closeout.ble2.nsh" <<'EOF'
echo BLUEZHIDUPSTREAM_BEGIN_BLE2
bluezhid closeout hogp-device 3
echo BLUEZHIDUPSTREAM_DONE_BLE2
EOF
}

case_bluez_gatt_upstream_convergence_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-gatt-upstream-convergence-closeout.ble1.nsh" <<'EOF'
echo BLUEZGATTUPSTREAM_BEGIN_BLE1
bluezaudio le-att-bearer open source 0 1
bluezaudio le-att-bearer mtu-exchange source 0 1
bluezaudio le-att-bearer security source 0 1
bluezaudio le-att-bearer enable-ccc source 0 1
bluezaudio le-att-bearer prepare-write source 0 1
bluezaudio le-att-bearer execute-write source 0 1
bluezaudio le-att-bearer indicate source 0 1
bluezaudio le-att-io attach source 0 1
bluezaudio le-att-io watch-rx source 0 1
bluezaudio le-att-io watch-tx source 0 1
bluezaudio le-att-io rx-pdu source 0 1
bluezaudio le-att-io tx-pdu source 0 1
bluezaudio le-att-io fragment-write source 0 1
bluezaudio le-att-io reassemble source 0 1
bluezaudio le-att-io persist-ccc source 0 1
bluezaudio le-att-queue alloc-req source 0 1
bluezaudio le-att-queue enqueue source 0 1
bluezaudio le-att-queue socket-read source 0 1
bluezaudio le-att-queue socket-write source 0 1
bluezaudio le-att-queue timeout source 0 1
bluezaudio le-att-queue cancel source 0 1
bluezaudio le-att-queue error-rsp source 0 1
bluezaudio le-att-queue complete source 0 1
bluezaudio le-att-queue free-req source 0 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-pac source 0 1
bluezaudio le-gatt-db read-location source 0 1
bluezaudio le-gatt-db read-context source 0 1
bluezaudio le-gatt-db update-context source 0 1
bluezaudio le-gatt-db notify-pac source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-gatt-db write-ascs-cp source 0 1
bluezaudio le-gatt-db notify-ase source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-att-io flush source 0 1
bluezaudio le-att-io detach source 0 1
bluezaudio le-att-bearer close source 0 1
bluezaudio le-gatt-upstream closeout source 0 1
bluezgatt closeout source 0 1
echo BLUEZGATTUPSTREAM_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-gatt-upstream-convergence-closeout.ble2.nsh" <<'EOF'
echo BLUEZGATTUPSTREAM_BEGIN_BLE2
bluezaudio le-att-bearer open sink 0 2
bluezaudio le-att-bearer mtu-exchange sink 0 2
bluezaudio le-att-bearer security sink 0 2
bluezaudio le-att-bearer enable-ccc sink 0 2
bluezaudio le-att-bearer prepare-write sink 0 2
bluezaudio le-att-bearer execute-write sink 0 2
bluezaudio le-att-bearer indicate sink 0 2
bluezaudio le-att-io attach sink 0 2
bluezaudio le-att-io watch-rx sink 0 2
bluezaudio le-att-io watch-tx sink 0 2
bluezaudio le-att-io rx-pdu sink 0 2
bluezaudio le-att-io tx-pdu sink 0 2
bluezaudio le-att-io fragment-write sink 0 2
bluezaudio le-att-io reassemble sink 0 2
bluezaudio le-att-io persist-ccc sink 0 2
bluezaudio le-att-queue alloc-req sink 0 2
bluezaudio le-att-queue enqueue sink 0 2
bluezaudio le-att-queue socket-read sink 0 2
bluezaudio le-att-queue socket-write sink 0 2
bluezaudio le-att-queue timeout sink 0 2
bluezaudio le-att-queue cancel sink 0 2
bluezaudio le-att-queue error-rsp sink 0 2
bluezaudio le-att-queue complete sink 0 2
bluezaudio le-att-queue free-req sink 0 2
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-pac sink 0 2
bluezaudio le-gatt-db read-location sink 0 2
bluezaudio le-gatt-db read-context sink 0 2
bluezaudio le-gatt-db update-context sink 0 2
bluezaudio le-gatt-db notify-pac sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-gatt-db write-ascs-cp sink 0 2
bluezaudio le-gatt-db notify-ase sink 0 2
bluezaudio le-gatt-db release sink 0 2
bluezaudio le-att-io flush sink 0 2
bluezaudio le-att-io detach sink 0 2
bluezaudio le-att-bearer close sink 0 2
bluezaudio le-gatt-upstream closeout sink 0 2
bluezgatt closeout sink 0 2
echo BLUEZGATTUPSTREAM_DONE_BLE2
EOF
}

case_ble_ip_ping() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-ping.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl upstream hci-pump 9000 &
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
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
sleep 1
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

case_bluez_basic_mgmt_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-basic-mgmt-flow.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-mgmt-control disconnect source 0 1
bluezaudio le-mgmt-control error source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-basic-mgmt-flow.ble2.nsh" <<'EOF'
sleep 2
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-mgmt-control disconnect sink 0 1
bluezaudio le-mgmt-control error sink 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream hci-status
btctl state
EOF
}

case_bluez_basic_scan_connect_auth_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-basic-scan-connect-auth-flow.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl state
sleep 8
btctl advertise start
btctl state
sleep 8
EOF

  cat >"${out_dir}/bluez-basic-scan-connect-auth-flow.ble1.nsh" <<'EOF'
sleep 4
bluezdaemon discovery-peer
btctl upstream status
bluezmgmt control
btctl upstream status
bluezmgmt lifecycle
btctl upstream status
bluezmgmt pair-noio
btctl upstream status
bluezdaemon pairing-matrix
btctl upstream status
bluezmgmt security-closeout
btctl upstream status
bluezdaemon reconnect-stress 3
btctl upstream status
bluezdaemon device-policy
btctl upstream status
bluezmgmt error-path
btctl upstream status
btctl state
EOF
}

case_bluez_basic_upstream_convergence_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-basic-upstream-convergence-closeout.bt1.nsh" <<'EOF'
echo BLUEZBASICUPSTREAM_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl scan bredr
btctl connect 2
btctl pair 2
btctl l2cap-connect 2 0x1001
btctl l2cap-echo 2 bt-basic-upstream-echo
btctl events
btctl state
btctl upstream status
bluezdaemon basic-closeout bt
echo BLUEZBASICUPSTREAM_DONE_BT1
EOF

  cat >"${out_dir}/bluez-basic-upstream-convergence-closeout.bt2.nsh" <<'EOF'
echo BLUEZBASICUPSTREAM_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
btctl poll ctrl
btctl poll acl
btctl events
btctl state
btctl upstream status
bluezdaemon basic-closeout bt
echo BLUEZBASICUPSTREAM_DONE_BT2
EOF

  cat >"${out_dir}/bluez-basic-upstream-convergence-closeout.ble2.nsh" <<'EOF'
echo BLUEZBASICUPSTREAM_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl state
sleep 4
btctl advertise start
btctl state
sleep 8
btctl advertise start
btctl state
sleep 12
btctl upstream status
bluezdaemon basic-closeout ble
echo BLUEZBASICUPSTREAM_DONE_BLE2
EOF

  cat >"${out_dir}/bluez-basic-upstream-convergence-closeout.ble1.nsh" <<'EOF'
echo BLUEZBASICUPSTREAM_BEGIN_BLE1
sleep 8
bluezdaemon discovery-peer
btctl upstream status
bluezmgmt control
btctl upstream status
bluezmgmt lifecycle
btctl upstream status
bluezmgmt pair-noio
btctl upstream status
bluezdaemon pairing-matrix
btctl upstream status
bluezmgmt security-closeout
btctl upstream status
bluezdaemon reconnect-stress 3
btctl upstream status
bluezdaemon device-policy
btctl upstream status
bluezmgmt error-path
btctl upstream status
bluezdaemon basic-closeout ble
btctl state
echo BLUEZBASICUPSTREAM_DONE_BLE1
EOF
}

case_bluez_hid_hogp_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hid-hogp-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZHIDHOGP_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
sleep 2
btctl scan bredr
btctl connect 2
btctl l2cap-connect 2 0x0011
btctl l2cap-connect 2 0x0013
bluezdaemon profile-hid-closeout classic-host 2 &
sleep 4
bluezprofile closeout classic-hid-host 2
btctl events
btctl state
echo BLUEZHIDHOGP_DONE_BT1
EOF

  cat >"${out_dir}/bluez-hid-hogp-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZHIDHOGP_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl poll ctrl
bluezdaemon profile-hid-closeout classic-device 1 &
sleep 14
bluezprofile closeout classic-hid-device 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZHIDHOGP_DONE_BT2
EOF

  cat >"${out_dir}/bluez-hid-hogp-profile-closeout.ble2.nsh" <<'EOF'
echo BLUEZHIDHOGP_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
btctl state
sleep 1
bluezdaemon profile-hid-closeout hogp-device 3 &
sleep 16
bluezprofile closeout hogp-device 3
echo BLUEZHIDHOGP_DONE_BLE2
EOF

  cat >"${out_dir}/bluez-hid-hogp-profile-closeout.ble1.nsh" <<'EOF'
echo BLUEZHIDHOGP_BEGIN_BLE1
sleep 3
btctl info
btctl mgmt power on
btctl mgmt le on
btctl state
bluezdaemon profile-hid-closeout hogp-host 4
bluezprofile closeout hogp-host 4
btctl events
btctl state
echo BLUEZHIDHOGP_DONE_BLE1
EOF
}

case_bluez_hfp_hsp_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hfp-hsp-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZHFPHSP_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-hfp-closeout hfp-hf 2
bluezdaemon profile-hfp-closeout hsp-hs 2
bluezhfp closeout hfp-hf 2
bluezhfp closeout hsp-hs 2
bluezprofile closeout hfp-hf 2
bluezprofile closeout hsp-hs 2
btctl events
btctl state
echo BLUEZHFPHSP_DONE_BT1
EOF

  cat >"${out_dir}/bluez-hfp-hsp-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZHFPHSP_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-hfp-closeout hfp-ag 1
bluezdaemon profile-hfp-closeout hsp-ag 1
bluezhfp closeout hfp-ag 1
bluezhfp closeout hsp-ag 1
bluezprofile closeout hfp-ag 1
bluezprofile closeout hsp-ag 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZHFPHSP_DONE_BT2
EOF
}

case_bluez_obex_pbap_opp_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-obex-pbap-opp-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZOBEXPBO_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-obex-closeout pbap-client 2
bluezobex closeout pbap-client 2
bluezdaemon profile-obex-closeout opp-client 2
bluezobex closeout opp-client 2
btctl events
btctl state
echo BLUEZOBEXPBO_DONE_BT1
EOF

  cat >"${out_dir}/bluez-obex-pbap-opp-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZOBEXPBO_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-obex-closeout pbap-server 1
bluezobex closeout pbap-server 1
bluezdaemon profile-obex-closeout opp-server 1
bluezobex closeout opp-server 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZOBEXPBO_DONE_BT2
EOF
}

case_bluez_obex_map_mns_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-obex-map-mns-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZOBEXMAP_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-map-closeout map-client 2
bluezobex closeout map-client 2
bluezdaemon profile-map-closeout mns-client 2
bluezobex closeout mns-client 2
btctl events
btctl state
echo BLUEZOBEXMAP_DONE_BT1
EOF

  cat >"${out_dir}/bluez-obex-map-mns-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZOBEXMAP_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-map-closeout map-server 1
bluezobex closeout map-server 1
bluezdaemon profile-map-closeout mns-server 1
bluezobex closeout mns-server 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZOBEXMAP_DONE_BT2
EOF
}

case_bluez_obex_ftp_sync_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-obex-ftp-sync-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZOBEXFTPSYNC_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-sync-closeout ftp-client 2
bluezobex closeout ftp-client 2
bluezdaemon profile-sync-closeout sync-client 2
bluezobex closeout sync-client 2
btctl events
btctl state
echo BLUEZOBEXFTPSYNC_DONE_BT1
EOF

  cat >"${out_dir}/bluez-obex-ftp-sync-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZOBEXFTPSYNC_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-sync-closeout ftp-server 1
bluezobex closeout ftp-server 1
bluezdaemon profile-sync-closeout sync-server 1
bluezobex closeout sync-server 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZOBEXFTPSYNC_DONE_BT2
EOF
}

case_bluez_mesh_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mesh-profile-closeout.ble1.nsh" <<'EOF'
echo BLUEZMESH_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl advertise start
btctl state
sleep 2
bluezdaemon profile-mesh-closeout provisioner 4
bluezprofile closeout mesh-provisioner 4
btctl events
btctl state
echo BLUEZMESH_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-mesh-profile-closeout.ble2.nsh" <<'EOF'
echo BLUEZMESH_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl advertise start
btctl state
bluezdaemon profile-mesh-closeout node 3 &
sleep 10
bluezprofile closeout mesh-node 3
btctl events
btctl state
echo BLUEZMESH_DONE_BLE2
EOF
}

case_bluez_gatt_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-gatt-profile-closeout.ble1.nsh" <<'EOF'
echo BLUEZGATTPROFILE_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl state
sleep 2
bluezdaemon profile-gatt-closeout client 4
bluezprofile closeout gatt-client 4
btctl events
btctl state
echo BLUEZGATTPROFILE_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-gatt-profile-closeout.ble2.nsh" <<'EOF'
echo BLUEZGATTPROFILE_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl advertise start
btctl state
bluezdaemon profile-gatt-closeout server 3 &
sleep 10
bluezprofile closeout gatt-server 3
btctl events
btctl state
echo BLUEZGATTPROFILE_DONE_BLE2
EOF
}

case_bluez_asha_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-asha-profile-closeout.ble1.nsh" <<'EOF'
echo BLUEZASHA_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl state
sleep 2
bluezdaemon profile-asha-closeout central 4
bluezprofile closeout asha-central 4
btctl events
btctl state
echo BLUEZASHA_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-asha-profile-closeout.ble2.nsh" <<'EOF'
echo BLUEZASHA_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl advertise start
btctl state
bluezdaemon profile-asha-closeout hearing-aid 3 &
sleep 10
bluezprofile closeout asha-hearing-aid 3
btctl events
btctl state
echo BLUEZASHA_DONE_BLE2
EOF
}

case_bluez_obex_bip_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-obex-bip-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZOBEXBIP_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
sleep 2
bluezdaemon profile-bip-closeout client 2
bluezobex closeout bip-client 2
btctl events
btctl state
echo BLUEZOBEXBIP_DONE_BT1
EOF

  cat >"${out_dir}/bluez-obex-bip-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZOBEXBIP_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-bip-closeout server 1
bluezobex closeout bip-server 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZOBEXBIP_DONE_BT2
EOF
}

case_bluez_print_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-print-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZPRINT_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
sleep 2
bluezdaemon profile-print-closeout client 2
bluezprofile closeout print-client 2
btctl events
btctl state
echo BLUEZPRINT_DONE_BT1
EOF

  cat >"${out_dir}/bluez-print-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZPRINT_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-print-closeout printer 1
bluezprofile closeout print-printer 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZPRINT_DONE_BT2
EOF
}

case_bluez_iap_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-iap-profile-closeout.bt1.nsh" <<'EOF'
echo BLUEZIAP_BEGIN_BT1
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-iap-closeout controller 2
bluezprofile closeout iap-controller 2
btctl events
btctl state
echo BLUEZIAP_DONE_BT1
EOF

  cat >"${out_dir}/bluez-iap-profile-closeout.bt2.nsh" <<'EOF'
echo BLUEZIAP_BEGIN_BT2
btctl info
btctl mgmt power on
btctl mgmt bredr on
btctl mgmt connectable on
btctl mgmt discoverable on
btctl state
bluezdaemon profile-iap-closeout accessory 1
bluezprofile closeout iap-accessory 1
btctl poll ctrl
btctl poll acl
btctl events
btctl state
echo BLUEZIAP_DONE_BT2
EOF
}

case_bluez_midi_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-midi-profile-closeout.ble1.nsh" <<'EOF'
echo BLUEZMIDI_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl state
sleep 2
bluezdaemon profile-midi-closeout controller 4
bluezprofile closeout midi-controller 4
btctl events
btctl state
echo BLUEZMIDI_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-midi-profile-closeout.ble2.nsh" <<'EOF'
echo BLUEZMIDI_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl advertise start
btctl state
bluezdaemon profile-midi-closeout peripheral 3 &
sleep 10
bluezprofile closeout midi-peripheral 3
btctl events
btctl state
echo BLUEZMIDI_DONE_BLE2
EOF
}

case_bluez_ranging_profile_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-ranging-profile-closeout.ble1.nsh" <<'EOF'
echo BLUEZRANGING_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl state
sleep 2
bluezdaemon profile-ranging-closeout initiator 4
bluezprofile closeout ranging-initiator 4
btctl events
btctl state
echo BLUEZRANGING_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-ranging-profile-closeout.ble2.nsh" <<'EOF'
echo BLUEZRANGING_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
btctl mgmt connectable on
btctl advertise start
btctl state
bluezdaemon profile-ranging-closeout reflector 3 &
sleep 10
bluezprofile closeout ranging-reflector 3
btctl events
btctl state
echo BLUEZRANGING_DONE_BLE2
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
sleep 1
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
btctl upstream 6lowpan-status
EOF

  cat >"${out_dir}/ble-ip-iperf-tcp.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
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
btctl upstream 6lowpan-status
EOF
}

case_ble_ip_iperf_tcp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-iperf-tcp-reverse.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
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
btctl upstream 6lowpan-status
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
btctl upstream 6lowpan-status
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
btctl upstream 6lowpan-status
EOF

  cat >"${out_dir}/ble-ip-iperf-udp.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
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
btctl upstream 6lowpan-status
EOF
}

case_ble_ip_iperf_udp_reverse() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-iperf-udp-reverse.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
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
btctl upstream 6lowpan-status
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
btctl upstream 6lowpan-status
EOF
}

case_ble_ip_closeout_full() {
  local out_dir="$1"

  cat >"${out_dir}/ble-ip-closeout-full.ble1.nsh" <<'EOF'
echo BLEIP_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 56000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 8
ping6 -c 2 -W 5000 fc00::2
iperf -V -s -B fc00::1 -i 1 -t 6 &
sleep 8
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::1 -i 1 -t 6 &
sleep 8
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
echo BLEIP_DONE_BLE1
EOF

  cat >"${out_dir}/ble-ip-closeout-full.ble2.nsh" <<'EOF'
echo BLEIP_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
btctl connect 3
btctl upstream hci-pump 56000 &
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 10
ping6 -c 2 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -i 1 -t 4
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
sleep 1
btctl upstream 6lowpan-up bt%d
btctl upstream 6lowpan-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -c fc00::1 -B fc00::2 -u -i 1 -t 4
btctl upstream 6lowpan-status
btctl upstream 6lowpan-down
btctl upstream 6lowpan-status
echo BLEIP_DONE_BLE2
EOF
}

case_hci_le_lifecycle() {
  local out_dir="$1"

  cat >"${out_dir}/hci-le-lifecycle.ble1.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream status
btctl upstream hci-status
btctl upstream hci-connect-le 2
btctl upstream status
btctl upstream hci-status
btctl upstream hci-disconnect-le 2
btctl upstream status
btctl upstream hci-status
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
btctl poll ctrl
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
sleep 4
btctl poll ctrl
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
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
sleep 1
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
btctl upstream hci-status
btctl upstream hci-connect-le 4
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
sleep 1
btctl upstream hci-disconnect-le 4
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
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
btctl poll ctrl
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
sleep 4
btctl poll ctrl
btctl events
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
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
btctl upstream hci-status
btctl upstream mgmt-send 0x0018 0 3
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 4
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
sleep 1
btctl upstream hci-disconnect-le 4
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
btctl upstream mgmt-close
EOF

  cat >"${out_dir}/hci-le-pairing.ble2.nsh" <<'EOF'
btctl info
btctl mgmt power on
btctl mgmt le on
btctl upstream mgmt-listen
sleep 1
btctl poll ctrl
btctl events
btctl upstream mgmt-send 0x0018 0 3
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 3
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream mgmt-read
btctl upstream status
btctl upstream hci-status
sleep 5
btctl poll ctrl
btctl events
btctl upstream hci-status
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

case_bluez_mgmt_daemon_bootstrap() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-mgmt-daemon-bootstrap.ble1.nsh" <<'EOF'
bluezmgmt daemon-bootstrap
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
btctl upstream hci-status
bluezdaemon smoke
btctl upstream status
btctl upstream hci-status
bluezdaemon device-policy
btctl upstream status
btctl upstream hci-status
bluezdaemon pairing-matrix
btctl upstream status
btctl upstream hci-status
bluezdaemon reconnect-stress 3
btctl upstream status
btctl upstream hci-status
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

case_bluez_hci_mgmt_socket_closeout_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-hci-mgmt-socket-closeout-full.ble1.nsh" <<'EOF'
echo BLUEZ_HCI_MGMT_SOCKET_BEGIN_BLE1
bluezhciraw user-advertise-enable
btctl upstream status
sleep 3
bluezhciraw user-command
bluezhciraw user-command-monitor
bluezhciraw user-command-sequence-monitor
bluezhciraw user-command-error-monitor
bluezhciraw user-command-init-sequence-monitor
bluezhciraw user-iso-setup-monitor
bluezhciraw socket-abi-closeout
btctl upstream socket cmtp
btctl upstream socket-cmtp 0x00c5
btctl upstream ordinary-cmtp-socket 0x00c6
btctl upstream status
bluezhciioctl basic
btctl upstream status
bluezhciraw command
bluezmgmt control
bluezmgmt pair-noio
bluezmgmt user-confirm
bluezmgmt user-confirm-neg
bluezmgmt passkey
bluezmgmt passkey-neg
bluezmgmt cancel-pair
bluezmgmt cancel-pair-pending
bluezmgmt pair-unpair
bluezmgmt lifecycle
bluezmgmt reconnect-stress
bluezmgmt error-path
btctl upstream status
bluezbtmon control
btctl upstream status
btctl upstream hci-status
btctl state
echo BLUEZ_HCI_MGMT_SOCKET_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-hci-mgmt-socket-closeout-full.ble2.nsh" <<'EOF'
echo BLUEZ_HCI_MGMT_SOCKET_BEGIN_BLE2
sleep 1
bluezhciraw user-scan-report
btctl upstream status
btctl upstream hci-status
btctl state
echo BLUEZ_HCI_MGMT_SOCKET_DONE_BLE2
EOF
}

case_bluez_le_audio_controller_setup() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-setup.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-monitor
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-reconnect.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-reconnect-monitor
btctl upstream status
btctl upstream hci-status
btctl state
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
bluezbneptest native-closeout
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
ping -c 2 -W 5000 10.77.0.2
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
ping -c 2 -W 5000 10.77.0.2
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
ping -c 2 -W 5000 10.77.0.1
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
ping -c 2 -W 5000 10.77.0.1
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
ping -c 2 -W 5000 10.77.0.2
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
sleep 1
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
ping -s 1400 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
sleep 1
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
ping -c 2 -W 5000 10.77.0.2
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

case_bluez_network_closeout_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-closeout-full.bt1.nsh" <<'EOF'
blueznetwork closeout-full begin
blueznetwork daemon-profile register
btctl upstream hci-pump 52000 &
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 2
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
sleep 1
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
sleep 1
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path gn
btctl upstream status
blueznetwork daemon-profile unregister
blueznetwork closeout-full end
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-closeout-full.bt2.nsh" <<'EOF'
blueznetwork closeout-full begin
blueznetwork daemon-profile register
btctl upstream hci-pump 52000 &
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 2
ping -s 1400 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
sleep 1
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
sleep 1
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path panu
btctl upstream status
blueznetwork daemon-profile unregister
blueznetwork closeout-full end
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

case_bluez_network_iperf_matrix() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-iperf-matrix.bt1.nsh" <<'EOF'
btctl upstream hci-pump 60000 &
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
iperf -s -i 1 -t 3 &
sleep 4
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
iperf -c 10.77.0.2 -i 1 -t 2
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
iperf -c 10.77.0.2 -u -i 1 -t 4
btctl upstream status
blueznetwork disconnect
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-iperf-matrix.bt2.nsh" <<'EOF'
btctl upstream hci-pump 60000 &
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
iperf -c 10.77.0.1 -i 1 -t 2
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
iperf -s -i 1 -t 3 &
sleep 4
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
iperf -c 10.77.0.1 -u -i 1 -t 4
btctl upstream status
blueznetwork disconnect
sleep 1
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
iperf -s -u -i 1 -t 6 &
sleep 7
btctl upstream status
blueznetwork disconnect
btctl upstream status
EOF
}

case_bluez_network_mtu_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-mtu-ping.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 16000 &
sleep 1
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-mtu-ping.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 16000 &
sleep 1
ping -s 1400 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_jumbo_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-jumbo-ping.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifconfig btn0 mtu 2500
ifup btn0
ifconfig btn0
btctl upstream hci-pump 22000 &
sleep 1
ping -s 2000 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-jumbo-ping.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifconfig btn0 mtu 2500
ifup btn0
ifconfig btn0
btctl upstream hci-pump 22000 &
sleep 1
ping -s 2000 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_frag_ping() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-frag-ping.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 22000 &
sleep 1
ping -s 2000 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-frag-ping.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 22000 &
sleep 1
ping -s 2000 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_mtu_soak() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-mtu-soak.bt1.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 36000 &
sleep 1
ping -s 1400 -c 12 -i 1000 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-mtu-soak.bt2.nsh" <<'EOF'
blueznetwork connect panu
blueznetwork status
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
ifconfig btn0
btctl upstream hci-pump 36000 &
sleep 1
ping -s 1400 -c 12 -i 1000 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF
}

case_bluez_network_mtu_reconnect_stress() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-network-mtu-reconnect-stress.bt1.nsh" <<'EOF'
btctl upstream hci-pump 42000 &
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 1 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
blueznetwork connect panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
EOF

  cat >"${out_dir}/bluez-network-mtu-reconnect-stress.bt2.nsh" <<'EOF'
btctl upstream hci-pump 42000 &
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 1 -W 5000 10.77.0.1
btctl upstream status
blueznetwork disconnect
sleep 1
btctl upstream status
blueznetwork connect panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 2 -W 5000 10.77.0.1
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
iperf -s -i 1 -t 6 &
sleep 7
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
bluezbneptest status
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
bluezbneptest status
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
iperf -s -i 1 -t 6 &
sleep 7
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
bluezbneptest status
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
bluezbneptest status
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
bluezbneptest status
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
bluezbneptest status
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
bluezbneptest status
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
bluezbneptest status
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
btctl upstream avdtp-getcap 2
btctl upstream avdtp-setconfig 2
btctl upstream avdtp-open 2
btctl upstream avdtp-start 2
btctl upstream avdtp-suspend 2
btctl upstream avdtp-close-stream 2
btctl upstream l2cap-close
sleep 1
btctl upstream hci-disconnect-br 2
btctl state
EOF

  cat >"${out_dir}/a2dp.bt2.nsh" <<'EOF'
btctl upstream avdtp-listen 0x0052
btctl upstream avdtp-auto-rsp-loop 1 7 &
sleep 12
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
bluezaudio a2dp-signal getallcap 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal getconfig 2
bluezaudio a2dp-signal reconfigure 2
bluezaudio a2dp-signal delay-report 2
bluezaudio a2dp-signal security-control 2
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
bluezaudio a2dp-signal getallcap 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal getconfig 2
bluezaudio a2dp-signal reconfigure 2
bluezaudio a2dp-signal delay-report 2
bluezaudio a2dp-signal security-control 2
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
bluezaudio a2dp-signal auto-rsp-loop 1 10
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
bluezaudio a2dp-signal getallcap 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal getconfig 2
bluezaudio a2dp-signal reconfigure 2
bluezaudio a2dp-signal delay-report 2
bluezaudio a2dp-signal security-control 2
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
bluezaudio a2dp-signal auto-rsp-loop 1 10
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
bluezaudio a2dp-signal auto-rsp-loop 1 10
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

  cat >"${out_dir}/bluez-a2dp-sbc-codec-reconnect.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-endpoint clear sink 1
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

case_bluez_a2dp_sbc_codec_concurrent() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-sbc-codec-concurrent.bt1.nsh" <<'EOF'
sleep 1
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
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-a2dp-sbc-codec-concurrent.bt2.nsh" <<'EOF'
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
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
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_transport_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-transport-reconnect.bt1.nsh" <<'EOF'
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
sleep 1
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
sleep 2
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
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-read-release 1
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 2
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
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
bluezaudio a2dp-signal getallcap 2
bluezaudio a2dp-signal getcap 2
bluezaudio a2dp-signal setconfig 2
bluezaudio a2dp-signal getconfig 2
bluezaudio a2dp-signal reconfigure 2
bluezaudio a2dp-signal delay-report 2
bluezaudio a2dp-signal security-control 2
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
bluezaudio a2dp-signal auto-rsp-loop 1 10
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

case_bluez_daemon_a2dp_dbus_client_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-dbus-client-full.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-reconnect source 2 2
btctl upstream hci-disconnect-br 2
sleep 1
bluezaudio a2dp-endpoint lifecycle source 2
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
bluezaudio media-transport a2dp-source-acquire-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-dbus-client-full.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-reconnect sink 1 2
btctl upstream hci-disconnect-br 1
sleep 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-read-release 1
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_dbus_client_busy() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-dbus-client-busy.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-owner source 2
btctl upstream hci-disconnect-br 2
sleep 1
bluezaudio a2dp-endpoint lifecycle source 2
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
btctl upstream status
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-dbus-client-busy.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-owner sink 1
btctl upstream hci-disconnect-br 1
sleep 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
btctl upstream status
bluezaudio a2dp-signal listen 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_full_concurrent() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-full-concurrent.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-owner source 2
btctl upstream hci-disconnect-br 2
sleep 1
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-full-concurrent.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-owner sink 1
btctl upstream hci-disconnect-br 1
sleep 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_full_concurrent_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-full-concurrent-reconnect.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-reconnect source 2 2
btctl upstream hci-disconnect-br 2
sleep 1
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-full-concurrent-reconnect.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-reconnect sink 1 2
btctl upstream hci-disconnect-br 1
sleep 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_full_concurrent_soak() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-full-concurrent-soak.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-reconnect source 2 3
btctl upstream hci-disconnect-br 2
sleep 1
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
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
bluezaudio media-transport a2dp-source-acquire-busy-write-release 2
bluezaudio a2dp-codec source-sbc-encode-write-release 2
bluezaudio a2dp-signal suspend 2
bluezaudio a2dp-signal close-stream 2
bluezaudio a2dp-signal source-session-close 2
bluezaudio a2dp-endpoint clear source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-full-concurrent-soak.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-reconnect sink 1 3
btctl upstream hci-disconnect-br 1
sleep 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
bluezaudio a2dp-endpoint lifecycle sink 1
bluezaudio a2dp-signal listen-native 0x0052
bluezaudio a2dp-signal auto-rsp-loop 1 10
bluezaudio media-transport a2dp-sink-acquire-busy-read-release 1
bluezaudio a2dp-codec sink-sbc-recv-decode-release 1
bluezaudio a2dp-signal auto-rsp-loop 1 2 streaming
bluezaudio a2dp-signal close
bluezaudio a2dp-endpoint clear sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_integrated_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-integrated-profile.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-integrated-flow source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-integrated-profile.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-integrated-flow sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_integrated_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-integrated-reconnect.bt1.nsh" <<'EOF'
btctl upstream hci-pump 64000 &
sleep 1
bluezdaemon audio-a2dp-integrated-reconnect source 2 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-integrated-reconnect.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-integrated-reconnect sink 1 2
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_session_ownership() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-session-ownership.bt1.nsh" <<'EOF'
btctl upstream hci-pump 64000 &
sleep 1
bluezdaemon audio-a2dp-session-ownership source 2 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-session-ownership.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-session-ownership sink 1 2
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_error_policy() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-error-policy.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-error-policy source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-error-policy.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-error-policy sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_upstream_session() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-upstream-session.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-upstream-session source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-upstream-session.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-upstream-session sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_upstream_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-upstream-reconnect.bt1.nsh" <<'EOF'
btctl upstream hci-pump 64000 &
sleep 1
bluezdaemon audio-a2dp-upstream-reconnect source 2 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-upstream-reconnect.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-upstream-reconnect sink 1 2
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_upstream_transactions() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-upstream-transactions.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-upstream-transactions source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-upstream-transactions.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-upstream-transactions sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_media_transport_fd() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-media-transport-fd.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-media-transport-fd source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-media-transport-fd.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-media-transport-fd sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_codec_policy() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-codec-policy.bt1.nsh" <<'EOF'
btctl upstream hci-pump 32000 &
sleep 1
bluezdaemon audio-a2dp-codec-policy source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-codec-policy.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-codec-policy sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_daemon_a2dp_closeout_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-a2dp-closeout-full.bt1.nsh" <<'EOF'
btctl upstream hci-pump 64000 &
sleep 1
bluezdaemon audio-a2dp-closeout-full source 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-daemon-a2dp-closeout-full.bt2.nsh" <<'EOF'
bluezdaemon audio-a2dp-closeout-full sink 1
btctl upstream status
btctl state
EOF
}

case_bluez_a2dp_current_complete_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-current-complete-closeout.bt1.nsh" <<'EOF'
echo BLUEZA2DPCOMPLETE_BEGIN_BT1
btctl upstream hci-pump 64000 &
sleep 1
bluezdaemon audio-a2dp-closeout-full source 2
btctl upstream status
btctl state
echo BLUEZA2DPCOMPLETE_DONE_BT1
EOF

  cat >"${out_dir}/bluez-a2dp-current-complete-closeout.bt2.nsh" <<'EOF'
echo BLUEZA2DPCOMPLETE_BEGIN_BT2
bluezdaemon audio-a2dp-closeout-full sink 1
btctl upstream status
btctl state
echo BLUEZA2DPCOMPLETE_DONE_BT2
EOF
}

case_bluez_a2dp_upstream_convergence_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-a2dp-upstream-convergence-closeout.bt1.nsh" <<'EOF'
echo BLUEZA2DPUPSTREAM_BEGIN_BT1
btctl upstream hci-pump 64000 &
sleep 1
bluezaudio upstream-object-probe source
bluezdaemon audio-a2dp-closeout-full source 2
blueza2dp closeout source 2
btctl upstream status
btctl state
echo BLUEZA2DPUPSTREAM_DONE_BT1
EOF

  cat >"${out_dir}/bluez-a2dp-upstream-convergence-closeout.bt2.nsh" <<'EOF'
echo BLUEZA2DPUPSTREAM_BEGIN_BT2
bluezaudio upstream-object-probe sink
bluezdaemon audio-a2dp-closeout-full sink 1
blueza2dp closeout sink 1
btctl upstream status
btctl state
echo BLUEZA2DPUPSTREAM_DONE_BT2
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
sleep 3
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

case_bluez_le_audio_daemon_full_lifecycle() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-daemon-full-lifecycle.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-cap-control coordinator-register
bluezaudio le-vcp-control register source 0 1
bluezaudio le-vcp-control discover source 0 1
bluezaudio le-vcp-control read-state source 0 1
bluezaudio le-vcp-control set-volume source 0 1
bluezaudio le-vcp-control notify-state source 0 1
bluezaudio le-vcp-control flags source 0 1
bluezaudio le-vcp-control error source 0 1
bluezaudio le-micp-control register source 0 1
bluezaudio le-micp-control discover source 0 1
bluezaudio le-micp-control read-state source 0 1
bluezaudio le-micp-control mute source 0 1
bluezaudio le-micp-control notify-state source 0 1
bluezaudio le-micp-control flags source 0 1
bluezaudio le-micp-control error source 0 1
bluezaudio le-csip-control register source 0 1
bluezaudio le-csip-control discover source 0 1
bluezaudio le-csip-control read-sirk source 0 1
bluezaudio le-csip-control read-size source 0 1
bluezaudio le-csip-control read-rank source 0 1
bluezaudio le-csip-control lock source 0 1
bluezaudio le-csip-control unlock source 0 1
bluezaudio le-csip-control notify source 0 1
bluezaudio le-csip-control error source 0 1
bluezaudio le-mcp-control register source 0 1
bluezaudio le-mcp-control discover source 0 1
bluezaudio le-mcp-control read-player source 0 1
bluezaudio le-mcp-control read-track source 0 1
bluezaudio le-mcp-control play source 0 1
bluezaudio le-mcp-control pause source 0 1
bluezaudio le-mcp-control next source 0 1
bluezaudio le-mcp-control notify-state source 0 1
bluezaudio le-mcp-control search source 0 1
bluezaudio le-mcp-control error source 0 1
bluezaudio le-tmap-control register source 0 1
bluezaudio le-tmap-control discover source 0 1
bluezaudio le-tmap-control read-role source 0 1
bluezaudio le-tmap-control update-role source 0 1
bluezaudio le-tmap-control notify-role source 0 1
bluezaudio le-tmap-control error source 0 1
bluezaudio le-ccp-control register source 0 1
bluezaudio le-ccp-control discover source 0 1
bluezaudio le-ccp-control read-bearer source 0 1
bluezaudio le-ccp-control read-call-state source 0 1
bluezaudio le-ccp-control originate source 0 1
bluezaudio le-ccp-control accept source 0 1
bluezaudio le-ccp-control terminate source 0 1
bluezaudio le-ccp-control notify-call-state source 0 1
bluezaudio le-ccp-control termination-reason source 0 1
bluezaudio le-ccp-control error source 0 1
bluezaudio le-gmap-control register source 0 1
bluezaudio le-gmap-control discover source 0 1
bluezaudio le-gmap-control read-role source 0 1
bluezaudio le-gmap-control update-role source 0 1
bluezaudio le-gmap-control notify-role source 0 1
bluezaudio le-gmap-control error source 0 1
bluezaudio le-bap-control source-announce 0 1
bluezaudio le-broadcast-iso adv-start source 0 1
bluezaudio le-broadcast-iso base-config source 0 1
bluezaudio le-broadcast-security set-code source 0 1
bluezaudio le-broadcast-iso big-create source 0 1
bluezaudio le-broadcast-security encrypt-big source 0 1
bluezaudio le-broadcast-iso bis-setup source 0 1
bluezaudio le-broadcast-iso bis-bind source 0 1
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-iso bis-credit source 0 1
bluezaudio le-broadcast-source start 0 1
bluezaudio le-broadcast-iso bis-complete source 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-bap-control source-stop 0 1
bluezaudio le-broadcast-iso big-terminate source 0 1
bluezaudio le-broadcast-security clear-code source 0 1
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
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-daemon-full-lifecycle.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
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
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_dbus_client_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-dbus-client-full.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-cap-control group-config 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-cap-control group-enable 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-cap-control group-release 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client transport-busy sink 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-mcp-control release source 0 1
bluezaudio le-csip-control release source 0 1
bluezaudio le-micp-control release source 0 1
bluezaudio le-vcp-control release source 0 1
bluezaudio le-cap-control coordinator-release
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-dbus-client-full.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client transport-busy source 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_lc3_codec_transport() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-lc3-codec-transport.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket timestamp source 0 1
bluezaudio le-iso-socket error-eagain source 0 1
bluezaudio le-iso-qos credit-complete source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket timestamp sink 0 2
bluezaudio le-iso-socket error-eagain sink 0 2
bluezaudio le-iso-qos credit-complete sink 0 2
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-lc3-codec-transport.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket timestamp sink 0 1
bluezaudio le-iso-socket error-eagain sink 0 1
bluezaudio le-iso-qos credit-complete sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket timestamp source 0 2
bluezaudio le-iso-socket error-eagain source 0 2
bluezaudio le-iso-qos credit-complete source 0 2
bluezaudio le-audio-codec source-lc3-encode-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_iso_dataplane_soak() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-iso-dataplane-soak.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-att-bearer open source 0 1
bluezaudio le-att-io attach source 0 1
bluezaudio le-att-io watch-rx source 0 1
bluezaudio le-att-io watch-tx source 0 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-pac source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-ascs-cp update-metadata source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-iso-socket open source 0 1
bluezaudio le-iso-socket bind-cis source 0 1
bluezaudio le-iso-qos configure source 0 1
bluezaudio le-iso-qos select-phy source 0 1
bluezaudio le-iso-qos setup-cig source 0 1
bluezaudio le-iso-qos setup-cis source 0 1
bluezaudio le-iso-qos apply-qos source 0 1
bluezaudio le-iso-qos controller-timing source 0 1
bluezaudio le-iso-qos credit-grant source 0 1
bluezaudio le-iso-socket connect source 0 1
bluezaudio le-iso-socket pollout source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket timestamp source 0 1
bluezaudio le-iso-socket error-eagain source 0 1
bluezaudio le-iso-qos credit-complete source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
btctl upstream status
bluezaudio le-iso-qos teardown source 0 1
bluezaudio le-iso-socket shutdown source 0 1
bluezaudio le-iso-socket close source 0 1
bluezaudio le-bap-policy suspend-stream source 0 1
bluezaudio le-ascs-cp disable source 0 1
bluezaudio le-ascs-cp receiver-stop-ready source 0 1
bluezaudio le-bap-policy stop-stream source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-att-io detach source 0 1
bluezaudio le-att-bearer close source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-att-bearer open sink 0 2
bluezaudio le-att-io attach sink 0 2
bluezaudio le-att-io watch-rx sink 0 2
bluezaudio le-att-io watch-tx sink 0 2
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-pac sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-ascs-cp update-metadata sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-iso-socket open sink 0 2
bluezaudio le-iso-socket bind-cis sink 0 2
bluezaudio le-iso-qos configure sink 0 2
bluezaudio le-iso-qos select-phy sink 0 2
bluezaudio le-iso-qos setup-cig sink 0 2
bluezaudio le-iso-qos setup-cis sink 0 2
bluezaudio le-iso-qos apply-qos sink 0 2
bluezaudio le-iso-qos controller-timing sink 0 2
bluezaudio le-iso-qos credit-grant sink 0 2
bluezaudio le-iso-socket listen sink 0 2
bluezaudio le-iso-socket accept sink 0 2
bluezaudio le-iso-socket pollin sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client transport-busy sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket timestamp sink 0 2
bluezaudio le-iso-socket error-eagain sink 0 2
bluezaudio le-iso-qos credit-complete sink 0 2
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2
btctl upstream status
bluezaudio le-iso-qos teardown sink 0 2
bluezaudio le-iso-socket shutdown sink 0 2
bluezaudio le-iso-socket close sink 0 2
bluezaudio le-bap-policy suspend-stream sink 0 2
bluezaudio le-ascs-cp disable sink 0 2
bluezaudio le-ascs-cp receiver-stop-ready sink 0 2
bluezaudio le-bap-policy stop-stream sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-gatt-db release sink 0 2
bluezaudio le-att-io detach sink 0 2
bluezaudio le-att-bearer close sink 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-qos-update 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-iso-socket open source 0 1
bluezaudio le-iso-socket bind-cis source 0 1
bluezaudio le-iso-qos configure source 0 1
bluezaudio le-iso-qos select-phy source 0 1
bluezaudio le-iso-qos setup-cig source 0 1
bluezaudio le-iso-qos setup-cis source 0 1
bluezaudio le-iso-qos apply-qos source 0 1
bluezaudio le-iso-qos credit-grant source 0 1
bluezaudio le-iso-socket connect source 0 1
bluezaudio le-iso-socket pollout source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket timestamp source 0 1
bluezaudio le-iso-qos credit-complete source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
btctl upstream status
bluezaudio le-iso-qos teardown source 0 1
bluezaudio le-iso-socket shutdown source 0 1
bluezaudio le-iso-socket close source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-qos-update 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-iso-socket open sink 0 2
bluezaudio le-iso-socket bind-cis sink 0 2
bluezaudio le-iso-qos configure sink 0 2
bluezaudio le-iso-qos select-phy sink 0 2
bluezaudio le-iso-qos setup-cig sink 0 2
bluezaudio le-iso-qos setup-cis sink 0 2
bluezaudio le-iso-qos apply-qos sink 0 2
bluezaudio le-iso-qos credit-grant sink 0 2
bluezaudio le-iso-socket listen sink 0 2
bluezaudio le-iso-socket accept sink 0 2
bluezaudio le-iso-socket pollin sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket timestamp sink 0 2
bluezaudio le-iso-qos credit-complete sink 0 2
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2
btctl upstream status
bluezaudio le-iso-qos teardown sink 0 2
bluezaudio le-iso-socket shutdown sink 0 2
bluezaudio le-iso-socket close sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-iso-dataplane-soak.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-att-bearer open sink 0 1
bluezaudio le-att-io attach sink 0 1
bluezaudio le-att-io watch-rx sink 0 1
bluezaudio le-att-io watch-tx sink 0 1
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-pac sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-ascs-cp update-metadata sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-iso-socket open sink 0 1
bluezaudio le-iso-socket bind-cis sink 0 1
bluezaudio le-iso-qos configure sink 0 1
bluezaudio le-iso-qos select-phy sink 0 1
bluezaudio le-iso-qos setup-cig sink 0 1
bluezaudio le-iso-qos setup-cis sink 0 1
bluezaudio le-iso-qos apply-qos sink 0 1
bluezaudio le-iso-qos controller-timing sink 0 1
bluezaudio le-iso-qos credit-grant sink 0 1
bluezaudio le-iso-socket listen sink 0 1
bluezaudio le-iso-socket accept sink 0 1
bluezaudio le-iso-socket pollin sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket timestamp sink 0 1
bluezaudio le-iso-socket error-eagain sink 0 1
bluezaudio le-iso-qos credit-complete sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
btctl upstream status
bluezaudio le-iso-qos teardown sink 0 1
bluezaudio le-iso-socket shutdown sink 0 1
bluezaudio le-iso-socket close sink 0 1
bluezaudio le-bap-policy suspend-stream sink 0 1
bluezaudio le-ascs-cp disable sink 0 1
bluezaudio le-ascs-cp receiver-stop-ready sink 0 1
bluezaudio le-bap-policy stop-stream sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-att-io detach sink 0 1
bluezaudio le-att-bearer close sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-att-bearer open source 0 2
bluezaudio le-att-io attach source 0 2
bluezaudio le-att-io watch-rx source 0 2
bluezaudio le-att-io watch-tx source 0 2
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-pac source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-ascs-cp update-metadata source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-iso-socket open source 0 2
bluezaudio le-iso-socket bind-cis source 0 2
bluezaudio le-iso-qos configure source 0 2
bluezaudio le-iso-qos select-phy source 0 2
bluezaudio le-iso-qos setup-cig source 0 2
bluezaudio le-iso-qos setup-cis source 0 2
bluezaudio le-iso-qos apply-qos source 0 2
bluezaudio le-iso-qos controller-timing source 0 2
bluezaudio le-iso-qos credit-grant source 0 2
bluezaudio le-iso-socket connect source 0 2
bluezaudio le-iso-socket pollout source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client transport-busy source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket timestamp source 0 2
bluezaudio le-iso-socket error-eagain source 0 2
bluezaudio le-iso-qos credit-complete source 0 2
bluezaudio le-audio-codec source-lc3-encode-write-release 0 2
btctl upstream status
bluezaudio le-iso-qos teardown source 0 2
bluezaudio le-iso-socket shutdown source 0 2
bluezaudio le-iso-socket close source 0 2
bluezaudio le-bap-policy suspend-stream source 0 2
bluezaudio le-ascs-cp disable source 0 2
bluezaudio le-ascs-cp receiver-stop-ready source 0 2
bluezaudio le-bap-policy stop-stream source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-gatt-db release source 0 2
bluezaudio le-att-io detach source 0 2
bluezaudio le-att-bearer close source 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-qos-update 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-iso-socket open sink 0 1
bluezaudio le-iso-socket bind-cis sink 0 1
bluezaudio le-iso-qos configure sink 0 1
bluezaudio le-iso-qos select-phy sink 0 1
bluezaudio le-iso-qos setup-cig sink 0 1
bluezaudio le-iso-qos setup-cis sink 0 1
bluezaudio le-iso-qos apply-qos sink 0 1
bluezaudio le-iso-qos credit-grant sink 0 1
bluezaudio le-iso-socket listen sink 0 1
bluezaudio le-iso-socket accept sink 0 1
bluezaudio le-iso-socket pollin sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket timestamp sink 0 1
bluezaudio le-iso-qos credit-complete sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
btctl upstream status
bluezaudio le-iso-qos teardown sink 0 1
bluezaudio le-iso-socket shutdown sink 0 1
bluezaudio le-iso-socket close sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-qos-update 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-iso-socket open source 0 2
bluezaudio le-iso-socket bind-cis source 0 2
bluezaudio le-iso-qos configure source 0 2
bluezaudio le-iso-qos select-phy source 0 2
bluezaudio le-iso-qos setup-cig source 0 2
bluezaudio le-iso-qos setup-cis source 0 2
bluezaudio le-iso-qos apply-qos source 0 2
bluezaudio le-iso-qos credit-grant source 0 2
bluezaudio le-iso-socket connect source 0 2
bluezaudio le-iso-socket pollout source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket timestamp source 0 2
bluezaudio le-iso-qos credit-complete source 0 2
bluezaudio le-audio-codec source-lc3-encode-write-release 0 2
btctl upstream status
bluezaudio le-iso-qos teardown source 0 2
bluezaudio le-iso-socket shutdown source 0 2
bluezaudio le-iso-socket close source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_iso_dataplane_soak() {
  local out_dir="$1"
  local base_ble1="${out_dir}/bluez-le-audio-iso-dataplane-soak.ble1.nsh"
  local base_ble2="${out_dir}/bluez-le-audio-iso-dataplane-soak.ble2.nsh"

  case_bluez_le_audio_iso_dataplane_soak "${out_dir}"

  {
    cat <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
EOF
    cat "${base_ble1}"
  } >"${out_dir}/bluez-le-audio-controller-iso-dataplane-soak.ble1.nsh"

  {
    cat <<'EOF'
sleep 6
EOF
    cat "${base_ble2}"
  } >"${out_dir}/bluez-le-audio-controller-iso-dataplane-soak.ble2.nsh"
}

case_bluez_le_audio_daemon_profile_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-daemon-profile-flow.ble1.nsh" <<'EOF'
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-daemon-profile-flow.ble2.nsh" <<'EOF'
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_profile_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-profile-flow.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-profile-flow.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_profile_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-profile-reconnect.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
sleep 1
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-profile-reconnect.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
sleep 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_error_recovery() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-error-recovery.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-qos-reject 0 1
bluezaudio le-unicast-control source-qos-cancel 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket timestamp source 0 1
bluezaudio le-iso-socket error-eagain source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-error-recovery.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-ascs-cp config-qos-reject sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-qos-reject 0 1
bluezaudio le-unicast-control sink-qos-cancel 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket timestamp sink 0 1
bluezaudio le-iso-socket error-eagain sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_bap_ascs_error_matrix() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-bap-ascs-error-matrix.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-unicast-control source-qos-reject 0 1
bluezaudio le-unicast-control source-qos-cancel 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-ascs-cp update-metadata source 0 1
bluezaudio le-bap-policy suspend-stream source 0 1
bluezaudio le-ascs-cp disable source 0 1
bluezaudio le-ascs-cp receiver-stop-ready source 0 1
bluezaudio le-bap-policy stop-stream source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-mgmt-control disconnect source 0 1
bluezaudio le-mgmt-control error source 0 1
bluezaudio le-mgmt-control power-on sink 0 2
bluezaudio le-mgmt-control scan-start sink 0 2
bluezaudio le-mgmt-control connect sink 0 2
bluezaudio le-mgmt-control security sink 0 2
bluezaudio le-mgmt-control cis-request sink 0 2
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos-reject sink 0 2
bluezaudio le-unicast-control sink-qos-reject 0 2
bluezaudio le-unicast-control sink-qos-cancel 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-ascs-cp update-metadata sink 0 2
bluezaudio le-bap-policy suspend-stream sink 0 2
bluezaudio le-ascs-cp disable sink 0 2
bluezaudio le-ascs-cp receiver-stop-ready sink 0 2
bluezaudio le-bap-policy stop-stream sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
bluezaudio le-mgmt-control disconnect sink 0 2
bluezaudio le-mgmt-control error sink 0 2
btctl upstream hci-status
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-bap-ascs-error-matrix.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos-reject sink 0 1
bluezaudio le-unicast-control sink-qos-reject 0 1
bluezaudio le-unicast-control sink-qos-cancel 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-ascs-cp update-metadata sink 0 1
bluezaudio le-bap-policy suspend-stream sink 0 1
bluezaudio le-ascs-cp disable sink 0 1
bluezaudio le-ascs-cp receiver-stop-ready sink 0 1
bluezaudio le-bap-policy stop-stream sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-mgmt-control disconnect sink 0 1
bluezaudio le-mgmt-control error sink 0 1
bluezaudio le-mgmt-control power-on source 0 2
bluezaudio le-mgmt-control scan-start source 0 2
bluezaudio le-mgmt-control connect source 0 2
bluezaudio le-mgmt-control security source 0 2
bluezaudio le-mgmt-control cis-request source 0 2
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos-reject source 0 2
bluezaudio le-unicast-control source-qos-reject 0 2
bluezaudio le-unicast-control source-qos-cancel 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-ascs-cp update-metadata source 0 2
bluezaudio le-bap-policy suspend-stream source 0 2
bluezaudio le-ascs-cp disable source 0 2
bluezaudio le-ascs-cp receiver-stop-ready source 0 2
bluezaudio le-bap-policy stop-stream source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
bluezaudio le-mgmt-control disconnect source 0 2
bluezaudio le-mgmt-control error source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_dbus_ownership() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-dbus-ownership.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client transport-busy sink 0 2
bluezaudio le-dbus-client owner-lost sink 0 2
bluezaudio le-dbus-client owner-reacquire sink 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-dbus-ownership.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client transport-busy source 0 2
bluezaudio le-dbus-client owner-lost source 0 2
bluezaudio le-dbus-client owner-reacquire source 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_bap_ascs_dbus_owner_recovery() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-bap-ascs-dbus-owner-recovery.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-gatt-db write-ascs-cp source 0 1
bluezaudio le-gatt-db notify-ase source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client owner-lost sink 0 2
bluezaudio le-dbus-client owner-reacquire sink 0 2
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-gatt-db write-ascs-cp sink 0 2
bluezaudio le-gatt-db notify-ase sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-bap-ascs-dbus-owner-recovery.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-gatt-db write-ascs-cp sink 0 1
bluezaudio le-gatt-db notify-ase sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client owner-lost source 0 2
bluezaudio le-dbus-client owner-reacquire source 0 2
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-gatt-db write-ascs-cp source 0 2
bluezaudio le-gatt-db notify-ase source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_dbus_bap_ascs_reconnect() {
  local out_dir="$1"

  case_bluez_le_audio_bap_ascs_dbus_owner_recovery "${out_dir}"
  cp "${out_dir}/bluez-le-audio-bap-ascs-dbus-owner-recovery.ble1.nsh" \
     "${out_dir}/bluez-le-audio-controller-dbus-bap-ascs-reconnect.ble1.nsh"
  cp "${out_dir}/bluez-le-audio-bap-ascs-dbus-owner-recovery.ble2.nsh" \
     "${out_dir}/bluez-le-audio-controller-dbus-bap-ascs-reconnect.ble2.nsh"

  cat >>"${out_dir}/bluez-le-audio-controller-dbus-bap-ascs-reconnect.ble1.nsh" <<'EOF'
sleep 1
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-mgmt-control disconnect source 0 1
bluezaudio le-mgmt-control error source 0 1
bluezaudio le-mgmt-control power-on sink 0 2
bluezaudio le-mgmt-control scan-start sink 0 2
bluezaudio le-mgmt-control connect sink 0 2
bluezaudio le-mgmt-control security sink 0 2
bluezaudio le-mgmt-control cis-request sink 0 2
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client owner-lost sink 0 2
bluezaudio le-dbus-client owner-reacquire sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-mgmt-control disconnect sink 0 2
bluezaudio le-mgmt-control error sink 0 2
btctl upstream hci-status
btctl upstream status
btctl state
EOF

  cat >>"${out_dir}/bluez-le-audio-controller-dbus-bap-ascs-reconnect.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-mgmt-control disconnect sink 0 1
bluezaudio le-mgmt-control error sink 0 1
bluezaudio le-mgmt-control power-on source 0 2
bluezaudio le-mgmt-control scan-start source 0 2
bluezaudio le-mgmt-control connect source 0 2
bluezaudio le-mgmt-control security source 0 2
bluezaudio le-mgmt-control cis-request source 0 2
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client owner-lost source 0 2
bluezaudio le-dbus-client owner-reacquire source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-mgmt-control disconnect source 0 2
bluezaudio le-mgmt-control error source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_full_stack() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-full-stack.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-unicast-control source-qos-reject 0 1
bluezaudio le-unicast-control source-qos-cancel 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-full-stack.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-ascs-cp config-qos-reject sink 0 1
bluezaudio le-unicast-control sink-qos-reject 0 1
bluezaudio le-unicast-control sink-qos-cancel 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_full_stack_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-full-stack-reconnect.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-unicast-control source-qos-reject 0 1
bluezaudio le-unicast-control source-qos-cancel 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
sleep 1
bluezaudio le-daemon unicast-profile-flow sink 0 2
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-full-stack-reconnect.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-ascs-cp config-qos-reject sink 0 1
bluezaudio le-unicast-control sink-qos-reject 0 1
bluezaudio le-unicast-control sink-qos-cancel 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
sleep 1
bluezaudio le-daemon unicast-profile-flow source 0 2
btctl upstream status
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_mainloop_cleanup() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-mainloop-cleanup.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
bluezaudio le-daemon unicast-profile-flow source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-mainloop-cleanup.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
bluezaudio le-daemon unicast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_daemon_broadcast_profile_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-daemon-broadcast-profile-flow.ble1.nsh" <<'EOF'
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-daemon-broadcast-profile-flow.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_broadcast_profile_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-broadcast-profile-flow.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-monitor
btctl upstream hci-status
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-broadcast-profile-flow.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_daemon_broadcast_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-daemon-broadcast-reconnect.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-monitor
btctl upstream hci-status
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
sleep 1
bluezaudio le-daemon broadcast-profile-flow source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-daemon-broadcast-reconnect.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
sleep 1
bluezaudio le-daemon broadcast-profile-flow sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_controller_lc3_bidir() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-controller-lc3-bidir.ble1.nsh" <<'EOF'
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream status
btctl upstream hci-status
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket timestamp source 0 1
bluezaudio le-iso-socket error-eagain source 0 1
bluezaudio le-iso-qos credit-complete source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
btctl upstream status
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket timestamp sink 0 2
bluezaudio le-iso-socket error-eagain sink 0 2
bluezaudio le-iso-qos credit-complete sink 0 2
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-controller-lc3-bidir.ble2.nsh" <<'EOF'
sleep 6
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket timestamp sink 0 1
bluezaudio le-iso-socket error-eagain sink 0 1
bluezaudio le-iso-qos credit-complete sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
btctl upstream status
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket timestamp source 0 2
bluezaudio le-iso-socket error-eagain source 0 2
bluezaudio le-iso-qos credit-complete source 0 2
bluezaudio le-audio-codec source-lc3-encode-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_coordinated_services() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-coordinated-services.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-vcp-control register source 0 1
bluezaudio le-vcp-control discover source 0 1
bluezaudio le-vcp-control read-state source 0 1
bluezaudio le-vcp-control set-volume source 0 1
bluezaudio le-vcp-control notify-state source 0 1
bluezaudio le-vcp-control flags source 0 1
bluezaudio le-vcp-control error source 0 1
bluezaudio le-micp-control register source 0 1
bluezaudio le-micp-control discover source 0 1
bluezaudio le-micp-control read-state source 0 1
bluezaudio le-micp-control mute source 0 1
bluezaudio le-micp-control notify-state source 0 1
bluezaudio le-micp-control flags source 0 1
bluezaudio le-micp-control error source 0 1
bluezaudio le-csip-control register source 0 1
bluezaudio le-csip-control discover source 0 1
bluezaudio le-csip-control read-sirk source 0 1
bluezaudio le-csip-control read-size source 0 1
bluezaudio le-csip-control read-rank source 0 1
bluezaudio le-csip-control lock source 0 1
bluezaudio le-csip-control unlock source 0 1
bluezaudio le-csip-control notify source 0 1
bluezaudio le-csip-control error source 0 1
bluezaudio le-mcp-control register source 0 1
bluezaudio le-mcp-control discover source 0 1
bluezaudio le-mcp-control read-player source 0 1
bluezaudio le-mcp-control read-track source 0 1
bluezaudio le-mcp-control play source 0 1
bluezaudio le-mcp-control pause source 0 1
bluezaudio le-mcp-control next source 0 1
bluezaudio le-mcp-control notify-state source 0 1
bluezaudio le-mcp-control search source 0 1
bluezaudio le-mcp-control error source 0 1
bluezaudio le-tmap-control register source 0 1
bluezaudio le-tmap-control discover source 0 1
bluezaudio le-tmap-control read-role source 0 1
bluezaudio le-tmap-control update-role source 0 1
bluezaudio le-tmap-control notify-role source 0 1
bluezaudio le-tmap-control error source 0 1
bluezaudio le-ccp-control register source 0 1
bluezaudio le-ccp-control discover source 0 1
bluezaudio le-ccp-control read-bearer source 0 1
bluezaudio le-ccp-control read-call-state source 0 1
bluezaudio le-ccp-control originate source 0 1
bluezaudio le-ccp-control accept source 0 1
bluezaudio le-ccp-control terminate source 0 1
bluezaudio le-ccp-control notify-call-state source 0 1
bluezaudio le-ccp-control termination-reason source 0 1
bluezaudio le-ccp-control error source 0 1
bluezaudio le-gmap-control register source 0 1
bluezaudio le-gmap-control discover source 0 1
bluezaudio le-gmap-control read-role source 0 1
bluezaudio le-gmap-control update-role source 0 1
bluezaudio le-gmap-control notify-role source 0 1
bluezaudio le-gmap-control error source 0 1
bluezaudio le-gmap-control release source 0 1
bluezaudio le-ccp-control release source 0 1
bluezaudio le-tmap-control release source 0 1
bluezaudio le-mcp-control release source 0 1
bluezaudio le-csip-control release source 0 1
bluezaudio le-micp-control release source 0 1
bluezaudio le-vcp-control release source 0 1
bluezaudio le-vcp-control register sink 0 2
bluezaudio le-vcp-control discover sink 0 2
bluezaudio le-vcp-control read-state sink 0 2
bluezaudio le-vcp-control set-volume sink 0 2
bluezaudio le-vcp-control notify-state sink 0 2
bluezaudio le-vcp-control flags sink 0 2
bluezaudio le-micp-control register sink 0 2
bluezaudio le-micp-control discover sink 0 2
bluezaudio le-micp-control read-state sink 0 2
bluezaudio le-micp-control mute sink 0 2
bluezaudio le-micp-control notify-state sink 0 2
bluezaudio le-csip-control register sink 0 2
bluezaudio le-csip-control discover sink 0 2
bluezaudio le-csip-control read-sirk sink 0 2
bluezaudio le-csip-control lock sink 0 2
bluezaudio le-csip-control unlock sink 0 2
bluezaudio le-mcp-control register sink 0 2
bluezaudio le-mcp-control discover sink 0 2
bluezaudio le-mcp-control read-player sink 0 2
bluezaudio le-mcp-control play sink 0 2
bluezaudio le-mcp-control pause sink 0 2
bluezaudio le-tmap-control register sink 0 2
bluezaudio le-tmap-control read-role sink 0 2
bluezaudio le-ccp-control register sink 0 2
bluezaudio le-ccp-control read-bearer sink 0 2
bluezaudio le-gmap-control register sink 0 2
bluezaudio le-gmap-control read-role sink 0 2
bluezaudio le-gmap-control release sink 0 2
bluezaudio le-ccp-control release sink 0 2
bluezaudio le-tmap-control release sink 0 2
bluezaudio le-mcp-control release sink 0 2
bluezaudio le-csip-control release sink 0 2
bluezaudio le-micp-control release sink 0 2
bluezaudio le-vcp-control release sink 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-coordinated-services.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-vcp-control register sink 0 1
bluezaudio le-vcp-control discover sink 0 1
bluezaudio le-vcp-control read-state sink 0 1
bluezaudio le-vcp-control set-volume sink 0 1
bluezaudio le-vcp-control notify-state sink 0 1
bluezaudio le-vcp-control flags sink 0 1
bluezaudio le-vcp-control error sink 0 1
bluezaudio le-micp-control register sink 0 1
bluezaudio le-micp-control discover sink 0 1
bluezaudio le-micp-control read-state sink 0 1
bluezaudio le-micp-control mute sink 0 1
bluezaudio le-micp-control notify-state sink 0 1
bluezaudio le-micp-control flags sink 0 1
bluezaudio le-micp-control error sink 0 1
bluezaudio le-csip-control register sink 0 1
bluezaudio le-csip-control discover sink 0 1
bluezaudio le-csip-control read-sirk sink 0 1
bluezaudio le-csip-control read-size sink 0 1
bluezaudio le-csip-control read-rank sink 0 1
bluezaudio le-csip-control lock sink 0 1
bluezaudio le-csip-control unlock sink 0 1
bluezaudio le-csip-control notify sink 0 1
bluezaudio le-csip-control error sink 0 1
bluezaudio le-mcp-control register sink 0 1
bluezaudio le-mcp-control discover sink 0 1
bluezaudio le-mcp-control read-player sink 0 1
bluezaudio le-mcp-control read-track sink 0 1
bluezaudio le-mcp-control play sink 0 1
bluezaudio le-mcp-control pause sink 0 1
bluezaudio le-mcp-control next sink 0 1
bluezaudio le-mcp-control notify-state sink 0 1
bluezaudio le-mcp-control search sink 0 1
bluezaudio le-mcp-control error sink 0 1
bluezaudio le-tmap-control register sink 0 1
bluezaudio le-tmap-control discover sink 0 1
bluezaudio le-tmap-control read-role sink 0 1
bluezaudio le-tmap-control update-role sink 0 1
bluezaudio le-tmap-control notify-role sink 0 1
bluezaudio le-tmap-control error sink 0 1
bluezaudio le-ccp-control register sink 0 1
bluezaudio le-ccp-control discover sink 0 1
bluezaudio le-ccp-control read-bearer sink 0 1
bluezaudio le-ccp-control read-call-state sink 0 1
bluezaudio le-ccp-control originate sink 0 1
bluezaudio le-ccp-control accept sink 0 1
bluezaudio le-ccp-control terminate sink 0 1
bluezaudio le-ccp-control notify-call-state sink 0 1
bluezaudio le-ccp-control termination-reason sink 0 1
bluezaudio le-ccp-control error sink 0 1
bluezaudio le-gmap-control register sink 0 1
bluezaudio le-gmap-control discover sink 0 1
bluezaudio le-gmap-control read-role sink 0 1
bluezaudio le-gmap-control update-role sink 0 1
bluezaudio le-gmap-control notify-role sink 0 1
bluezaudio le-gmap-control error sink 0 1
bluezaudio le-gmap-control release sink 0 1
bluezaudio le-ccp-control release sink 0 1
bluezaudio le-tmap-control release sink 0 1
bluezaudio le-mcp-control release sink 0 1
bluezaudio le-csip-control release sink 0 1
bluezaudio le-micp-control release sink 0 1
bluezaudio le-vcp-control release sink 0 1
bluezaudio le-vcp-control register source 0 2
bluezaudio le-vcp-control discover source 0 2
bluezaudio le-vcp-control read-state source 0 2
bluezaudio le-vcp-control set-volume source 0 2
bluezaudio le-vcp-control notify-state source 0 2
bluezaudio le-vcp-control flags source 0 2
bluezaudio le-micp-control register source 0 2
bluezaudio le-micp-control discover source 0 2
bluezaudio le-micp-control read-state source 0 2
bluezaudio le-micp-control mute source 0 2
bluezaudio le-micp-control notify-state source 0 2
bluezaudio le-csip-control register source 0 2
bluezaudio le-csip-control discover source 0 2
bluezaudio le-csip-control read-sirk source 0 2
bluezaudio le-csip-control lock source 0 2
bluezaudio le-csip-control unlock source 0 2
bluezaudio le-mcp-control register source 0 2
bluezaudio le-mcp-control discover source 0 2
bluezaudio le-mcp-control read-player source 0 2
bluezaudio le-mcp-control play source 0 2
bluezaudio le-mcp-control pause source 0 2
bluezaudio le-tmap-control register source 0 2
bluezaudio le-tmap-control read-role source 0 2
bluezaudio le-ccp-control register source 0 2
bluezaudio le-ccp-control read-bearer source 0 2
bluezaudio le-gmap-control register source 0 2
bluezaudio le-gmap-control read-role source 0 2
bluezaudio le-gmap-control release source 0 2
bluezaudio le-ccp-control release source 0 2
bluezaudio le-tmap-control release source 0 2
bluezaudio le-mcp-control release source 0 2
bluezaudio le-csip-control release source 0 2
bluezaudio le-micp-control release source 0 2
bluezaudio le-vcp-control release source 0 2
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_cap_csip_group() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-cap-csip-group.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-cap-control coordinator-register
bluezaudio le-csip-control register source 0 1
bluezaudio le-csip-control discover source 0 1
bluezaudio le-csip-control read-sirk source 0 1
bluezaudio le-csip-control read-size source 0 1
bluezaudio le-csip-control read-rank source 0 1
bluezaudio le-csip-control lock source 0 1
bluezaudio le-csip-control register sink 0 2
bluezaudio le-csip-control discover sink 0 2
bluezaudio le-csip-control read-sirk sink 0 2
bluezaudio le-csip-control read-size sink 0 2
bluezaudio le-csip-control read-rank sink 0 2
bluezaudio le-csip-control lock sink 0 2
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-cap-control group-config-bidir 0 1 2
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-cap-control group-enable-bidir 0 1 2
bluezaudio le-dbus-client transport source 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
bluezaudio le-dbus-client transport sink 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-cap-control group-release-bidir 0 1 2
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-csip-control unlock source 0 1
bluezaudio le-csip-control release source 0 1
bluezaudio le-csip-control unlock sink 0 2
bluezaudio le-csip-control release sink 0 2
bluezaudio le-cap-control coordinator-release
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-cap-csip-group.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-cap-control coordinator-register
bluezaudio le-csip-control register sink 0 1
bluezaudio le-csip-control discover sink 0 1
bluezaudio le-csip-control read-sirk sink 0 1
bluezaudio le-csip-control read-size sink 0 1
bluezaudio le-csip-control read-rank sink 0 1
bluezaudio le-csip-control lock sink 0 1
bluezaudio le-csip-control register source 0 2
bluezaudio le-csip-control discover source 0 2
bluezaudio le-csip-control read-sirk source 0 2
bluezaudio le-csip-control read-size source 0 2
bluezaudio le-csip-control read-rank source 0 2
bluezaudio le-csip-control lock source 0 2
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-cap-control group-config-bidir 0 2 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-cap-control group-enable-bidir 0 2 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
bluezaudio le-dbus-client transport source 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-cap-control group-release-bidir 0 2 1
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client release source 0 2
bluezaudio le-csip-control unlock sink 0 1
bluezaudio le-csip-control release sink 0 1
bluezaudio le-csip-control unlock source 0 2
bluezaudio le-csip-control release source 0 2
bluezaudio le-cap-control coordinator-release
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_tmap_mcp_ccp_flow() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-tmap-mcp-ccp-flow.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-tmap-control register source 0 1
bluezaudio le-tmap-control discover source 0 1
bluezaudio le-tmap-control read-role source 0 1
bluezaudio le-tmap-control update-role source 0 1
bluezaudio le-tmap-control notify-role source 0 1
bluezaudio le-mcp-control register source 0 1
bluezaudio le-mcp-control discover source 0 1
bluezaudio le-mcp-control read-player source 0 1
bluezaudio le-mcp-control read-track source 0 1
bluezaudio le-mcp-control play source 0 1
bluezaudio le-mcp-control notify-state source 0 1
bluezaudio le-mcp-control search source 0 1
bluezaudio le-ccp-control register source 0 1
bluezaudio le-ccp-control discover source 0 1
bluezaudio le-ccp-control read-bearer source 0 1
bluezaudio le-ccp-control read-call-state source 0 1
bluezaudio le-ccp-control originate source 0 1
bluezaudio le-ccp-control accept source 0 1
bluezaudio le-ccp-control notify-call-state source 0 1
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio media-transport unicast-source-acquire-write-release 0 1
btctl upstream status
bluezaudio le-mcp-control pause source 0 1
bluezaudio le-ccp-control terminate source 0 1
bluezaudio le-ccp-control termination-reason source 0 1
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-tmap-control register sink 0 2
bluezaudio le-tmap-control read-role sink 0 2
bluezaudio le-mcp-control register sink 0 2
bluezaudio le-mcp-control read-player sink 0 2
bluezaudio le-ccp-control register sink 0 2
bluezaudio le-ccp-control read-bearer sink 0 2
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio media-transport unicast-sink-acquire-read-release 0 2
btctl upstream status
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-ccp-control release sink 0 2
bluezaudio le-mcp-control release sink 0 2
bluezaudio le-tmap-control release sink 0 2
bluezaudio le-ccp-control release source 0 1
bluezaudio le-mcp-control release source 0 1
bluezaudio le-tmap-control release source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-tmap-mcp-ccp-flow.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-tmap-control register sink 0 1
bluezaudio le-tmap-control discover sink 0 1
bluezaudio le-tmap-control read-role sink 0 1
bluezaudio le-tmap-control update-role sink 0 1
bluezaudio le-tmap-control notify-role sink 0 1
bluezaudio le-mcp-control register sink 0 1
bluezaudio le-mcp-control discover sink 0 1
bluezaudio le-mcp-control read-player sink 0 1
bluezaudio le-mcp-control read-track sink 0 1
bluezaudio le-mcp-control play sink 0 1
bluezaudio le-mcp-control notify-state sink 0 1
bluezaudio le-mcp-control search sink 0 1
bluezaudio le-ccp-control register sink 0 1
bluezaudio le-ccp-control discover sink 0 1
bluezaudio le-ccp-control read-bearer sink 0 1
bluezaudio le-ccp-control read-call-state sink 0 1
bluezaudio le-ccp-control originate sink 0 1
bluezaudio le-ccp-control accept sink 0 1
bluezaudio le-ccp-control notify-call-state sink 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio media-transport unicast-sink-acquire-read-release 0 1
btctl upstream status
bluezaudio le-mcp-control pause sink 0 1
bluezaudio le-ccp-control terminate sink 0 1
bluezaudio le-ccp-control termination-reason sink 0 1
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-tmap-control register source 0 2
bluezaudio le-tmap-control read-role source 0 2
bluezaudio le-mcp-control register source 0 2
bluezaudio le-mcp-control read-player source 0 2
bluezaudio le-ccp-control register source 0 2
bluezaudio le-ccp-control read-bearer source 0 2
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio media-transport unicast-source-acquire-write-release 0 2
btctl upstream status
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-ccp-control release source 0 2
bluezaudio le-mcp-control release source 0 2
bluezaudio le-tmap-control release source 0 2
bluezaudio le-ccp-control release sink 0 1
bluezaudio le-mcp-control release sink 0 1
bluezaudio le-tmap-control release sink 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_broadcast_multibis() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-broadcast-multibis.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-bap-control source-announce 0 1
bluezaudio le-broadcast-iso adv-start source 0 1
bluezaudio le-broadcast-iso base-config source 0 1
bluezaudio le-broadcast-security set-code source 0 1
bluezaudio le-broadcast-iso big-create source 0 1
bluezaudio le-broadcast-security encrypt-big source 0 1
bluezaudio le-broadcast-iso bis-setup source 0 1
bluezaudio le-broadcast-iso bis-bind source 0 1
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-iso bis-credit source 0 1
bluezaudio le-broadcast-source start 0 1
bluezaudio le-broadcast-iso bis-complete source 0 1
btctl upstream status
bluezaudio le-bap-control source-announce 0 2
bluezaudio le-broadcast-iso base-config source 0 2
bluezaudio le-broadcast-iso bis-setup source 0 2
bluezaudio le-broadcast-iso bis-bind source 0 2
bluezaudio le-bap-control source-start 0 2
bluezaudio le-broadcast-iso bis-credit source 0 2
bluezaudio le-broadcast-source start 0 2
bluezaudio le-broadcast-iso bis-complete source 0 2
btctl upstream status
bluezaudio le-bap-control source-stop 0 2
bluezaudio le-broadcast-iso big-terminate source 0 2
bluezaudio le-bap-control source-stop 0 1
bluezaudio le-broadcast-iso big-terminate source 0 1
bluezaudio le-broadcast-security clear-code source 0 1
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-broadcast-multibis.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-bass-control assistant-register
bluezaudio le-bass-control add-source 0 1
bluezaudio le-broadcast-iso pa-sync sink 0 1
bluezaudio le-broadcast-security bad-code sink 0 1
bluezaudio le-broadcast-security decrypt-setup sink 0 1
bluezaudio le-bap-control sink-discover 0 1
bluezaudio le-bap-control sink-config 0 1
bluezaudio le-broadcast-sink sync 0 1
bluezaudio le-bap-control sink-sync 0 1
bluezaudio le-broadcast-iso big-sync sink 0 1
bluezaudio le-broadcast-iso receive-state sink 0 1
bluezaudio le-broadcast-security receive-state-encrypted sink 0 1
bluezaudio le-broadcast-iso bis-credit sink 0 1
bluezaudio le-bass-control modify-source 0 1
bluezaudio le-broadcast-sink start 0 1
bluezaudio le-broadcast-iso bis-complete sink 0 1
btctl upstream status
bluezaudio le-bass-control add-source 0 2
bluezaudio le-broadcast-iso pa-sync sink 0 2
bluezaudio le-broadcast-security decrypt-setup sink 0 2
bluezaudio le-bap-control sink-discover 0 2
bluezaudio le-bap-control sink-config 0 2
bluezaudio le-broadcast-sink sync 0 2
bluezaudio le-bap-control sink-sync 0 2
bluezaudio le-broadcast-iso big-sync sink 0 2
bluezaudio le-broadcast-iso receive-state sink 0 2
bluezaudio le-broadcast-security receive-state-encrypted sink 0 2
bluezaudio le-broadcast-iso bis-credit sink 0 2
bluezaudio le-bass-control modify-source 0 2
bluezaudio le-broadcast-sink start 0 2
bluezaudio le-broadcast-iso bis-complete sink 0 2
btctl upstream status
bluezaudio le-broadcast-sink stop
bluezaudio le-broadcast-iso big-terminate sink 0 2
bluezaudio le-bass-control remove-source 0 2
bluezaudio le-broadcast-sink stop
bluezaudio le-broadcast-iso big-terminate sink 0 1
bluezaudio le-broadcast-security clear-code sink 0 1
bluezaudio le-bass-control remove-source 0 1
bluezaudio le-bass-control assistant-release
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_broadcast_multibis_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-broadcast-multibis-reconnect.ble1.nsh" <<'EOF'
bluezaudio le-daemon broadcast-profile-flow source 0 1
bluezaudio le-bap-control source-announce 0 2
bluezaudio le-broadcast-iso base-config source 0 2
bluezaudio le-broadcast-iso bis-setup source 0 2
bluezaudio le-broadcast-iso bis-bind source 0 2
bluezaudio le-bap-control source-start 0 2
bluezaudio le-broadcast-iso bis-credit source 0 2
bluezaudio le-broadcast-source start 0 2
bluezaudio le-broadcast-iso bis-complete source 0 2
btctl upstream status
bluezaudio le-bap-control source-stop 0 2
bluezaudio le-broadcast-iso big-terminate source 0 2
sleep 1
bluezaudio le-daemon broadcast-profile-flow source 0 1
bluezaudio le-bap-control source-announce 0 2
bluezaudio le-broadcast-iso base-config source 0 2
bluezaudio le-broadcast-iso bis-setup source 0 2
bluezaudio le-broadcast-iso bis-bind source 0 2
bluezaudio le-bap-control source-start 0 2
bluezaudio le-broadcast-iso bis-credit source 0 2
bluezaudio le-broadcast-source start 0 2
bluezaudio le-broadcast-iso bis-complete source 0 2
btctl upstream status
bluezaudio le-bap-control source-stop 0 2
bluezaudio le-broadcast-iso big-terminate source 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-broadcast-multibis-reconnect.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-daemon broadcast-profile-flow sink 0 1
bluezaudio le-bass-control assistant-register
bluezaudio le-bass-control add-source 0 2
bluezaudio le-broadcast-iso pa-sync sink 0 2
bluezaudio le-broadcast-security decrypt-setup sink 0 2
bluezaudio le-bap-control sink-discover 0 2
bluezaudio le-bap-control sink-config 0 2
bluezaudio le-broadcast-sink sync 0 2
bluezaudio le-bap-control sink-sync 0 2
bluezaudio le-broadcast-iso big-sync sink 0 2
bluezaudio le-broadcast-iso receive-state sink 0 2
bluezaudio le-broadcast-security receive-state-encrypted sink 0 2
bluezaudio le-broadcast-iso bis-credit sink 0 2
bluezaudio le-bass-control modify-source 0 2
bluezaudio le-broadcast-sink start 0 2
bluezaudio le-broadcast-iso bis-complete sink 0 2
btctl upstream status
bluezaudio le-broadcast-sink stop
bluezaudio le-broadcast-iso big-terminate sink 0 2
bluezaudio le-bass-control remove-source 0 2
sleep 1
bluezaudio le-daemon broadcast-profile-flow sink 0 1
bluezaudio le-bass-control assistant-register
bluezaudio le-bass-control add-source 0 2
bluezaudio le-broadcast-iso pa-sync sink 0 2
bluezaudio le-broadcast-security decrypt-setup sink 0 2
bluezaudio le-bap-control sink-discover 0 2
bluezaudio le-bap-control sink-config 0 2
bluezaudio le-broadcast-sink sync 0 2
bluezaudio le-bap-control sink-sync 0 2
bluezaudio le-broadcast-iso big-sync sink 0 2
bluezaudio le-broadcast-iso receive-state sink 0 2
bluezaudio le-broadcast-security receive-state-encrypted sink 0 2
bluezaudio le-broadcast-iso bis-credit sink 0 2
bluezaudio le-bass-control modify-source 0 2
bluezaudio le-broadcast-sink start 0 2
bluezaudio le-broadcast-iso bis-complete sink 0 2
btctl upstream status
bluezaudio le-broadcast-sink stop
bluezaudio le-broadcast-iso big-terminate sink 0 2
bluezaudio le-bass-control remove-source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_bass_scan_delegator() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-bass-scan-delegator.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-bass-control assistant-register
bluezaudio le-bass-control add-source 0 1
bluezaudio le-bass-control modify-source 0 1
bluezaudio le-bass-control assistant-release
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-bass-scan-delegator.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-bass-control assistant-register
bluezaudio le-bass-control add-source 0 1
bluezaudio le-bass-control remove-source 0 1
bluezaudio le-bass-control assistant-release
bluezaudio le-bass-control scan-delegator-register
bluezaudio le-bass-control scan-delegator-receive-state 0 1
bluezaudio le-bass-control scan-delegator-notify 0 1
bluezaudio le-bass-control scan-delegator-receive-state 0 2
bluezaudio le-bass-control scan-delegator-notify 0 2
bluezaudio le-bass-control scan-delegator-release
bluezaudio le-daemon profile-release
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_daemon_integrated_profile() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-daemon-integrated-profile.ble1.nsh" <<'EOF'
bluezaudio le-daemon integrated-profile-flow source 0 1 2
btctl upstream status
bluezaudio le-daemon integrated-profile-flow sink 0 2 1
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-daemon-integrated-profile.ble2.nsh" <<'EOF'
bluezaudio le-daemon integrated-profile-flow sink 0 1 2
btctl upstream status
bluezaudio le-daemon integrated-profile-flow source 0 2 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_daemon_integrated_profile_reconnect() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-daemon-integrated-profile-reconnect.ble1.nsh" <<'EOF'
bluezaudio le-daemon integrated-profile-flow source 0 1 2
btctl upstream status
bluezaudio le-daemon integrated-profile-flow sink 0 2 1
btctl upstream status
sleep 1
bluezaudio le-daemon integrated-profile-flow source 0 1 2
btctl upstream status
bluezaudio le-daemon integrated-profile-flow sink 0 2 1
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-daemon-integrated-profile-reconnect.ble2.nsh" <<'EOF'
bluezaudio le-daemon integrated-profile-flow sink 0 1 2
btctl upstream status
bluezaudio le-daemon integrated-profile-flow source 0 2 1
btctl upstream status
sleep 1
bluezaudio le-daemon integrated-profile-flow sink 0 1 2
btctl upstream status
bluezaudio le-daemon integrated-profile-flow source 0 2 1
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_bap_pacs_ascs_session() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-bap-pacs-ascs-session.ble1.nsh" <<'EOF'
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-pac source 0 1
bluezaudio le-gatt-db read-location source 0 1
bluezaudio le-gatt-db read-context source 0 1
bluezaudio le-gatt-db update-context source 0 1
bluezaudio le-gatt-db notify-pac source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-gatt-db write-ascs-cp source 0 1
bluezaudio le-gatt-db notify-ase source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-ascs-cp update-metadata source 0 1
bluezaudio le-bap-policy suspend-stream source 0 1
bluezaudio le-ascs-cp disable source 0 1
bluezaudio le-ascs-cp receiver-stop-ready source 0 1
bluezaudio le-bap-policy stop-stream source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-iso-socket sockapi-closeout source 0 1
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-pac sink 0 2
bluezaudio le-gatt-db read-location sink 0 2
bluezaudio le-gatt-db read-context sink 0 2
bluezaudio le-gatt-db update-context sink 0 2
bluezaudio le-gatt-db notify-pac sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-gatt-db write-ascs-cp sink 0 2
bluezaudio le-gatt-db notify-ase sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-ascs-cp update-metadata sink 0 2
bluezaudio le-bap-policy suspend-stream sink 0 2
bluezaudio le-ascs-cp disable sink 0 2
bluezaudio le-ascs-cp receiver-stop-ready sink 0 2
bluezaudio le-bap-policy stop-stream sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
btctl upstream status
EOF

  cat >"${out_dir}/bluez-le-audio-bap-pacs-ascs-session.ble2.nsh" <<'EOF'
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-pac sink 0 1
bluezaudio le-gatt-db read-location sink 0 1
bluezaudio le-gatt-db read-context sink 0 1
bluezaudio le-gatt-db update-context sink 0 1
bluezaudio le-gatt-db notify-pac sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-gatt-db write-ascs-cp sink 0 1
bluezaudio le-gatt-db notify-ase sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-ascs-cp update-metadata sink 0 1
bluezaudio le-bap-policy suspend-stream sink 0 1
bluezaudio le-ascs-cp disable sink 0 1
bluezaudio le-ascs-cp receiver-stop-ready sink 0 1
bluezaudio le-bap-policy stop-stream sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-iso-socket sockapi-closeout sink 0 1
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-pac source 0 2
bluezaudio le-gatt-db read-location source 0 2
bluezaudio le-gatt-db read-context source 0 2
bluezaudio le-gatt-db update-context source 0 2
bluezaudio le-gatt-db notify-pac source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-gatt-db write-ascs-cp source 0 2
bluezaudio le-gatt-db notify-ase source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-ascs-cp update-metadata source 0 2
bluezaudio le-bap-policy suspend-stream source 0 2
bluezaudio le-ascs-cp disable source 0 2
bluezaudio le-ascs-cp receiver-stop-ready source 0 2
bluezaudio le-bap-policy stop-stream source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
btctl upstream status
EOF
}

case_bluez_le_audio_bap_pacs_ascs_reconnect_recovery() {
  local out_dir="$1"

  case_bluez_le_audio_bap_pacs_ascs_session "${out_dir}"
  cp "${out_dir}/bluez-le-audio-bap-pacs-ascs-session.ble1.nsh" \
     "${out_dir}/bluez-le-audio-bap-pacs-ascs-reconnect-recovery.ble1.nsh"
  cp "${out_dir}/bluez-le-audio-bap-pacs-ascs-session.ble2.nsh" \
     "${out_dir}/bluez-le-audio-bap-pacs-ascs-reconnect-recovery.ble2.nsh"

  cat >>"${out_dir}/bluez-le-audio-bap-pacs-ascs-reconnect-recovery.ble1.nsh" <<'EOF'
sleep 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-gatt-db write-ascs-cp source 0 1
bluezaudio le-gatt-db notify-ase source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos-reject sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-gatt-db write-ascs-cp sink 0 2
bluezaudio le-gatt-db notify-ase sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
btctl upstream status
EOF

  cat >>"${out_dir}/bluez-le-audio-bap-pacs-ascs-reconnect-recovery.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos-reject sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-gatt-db write-ascs-cp sink 0 1
bluezaudio le-gatt-db notify-ase sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos-reject source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-gatt-db write-ascs-cp source 0 2
bluezaudio le-gatt-db notify-ase source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
btctl upstream status
EOF
}

case_bluez_le_audio_bap_pacs_ascs_metadata_reconfig() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-bap-pacs-ascs-metadata-reconfig.ble1.nsh" <<'EOF'
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-context source 0 1
bluezaudio le-gatt-db update-context source 0 1
bluezaudio le-gatt-db notify-pac source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-ascs-cp update-metadata source 0 1
bluezaudio le-bap-policy suspend-stream source 0 1
bluezaudio le-ascs-cp disable source 0 1
bluezaudio le-ascs-cp receiver-stop-ready source 0 1
bluezaudio le-bap-policy stop-stream source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-context source 0 1
bluezaudio le-gatt-db update-context source 0 1
bluezaudio le-gatt-db notify-pac source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-gatt-db write-ascs-cp source 0 1
bluezaudio le-gatt-db notify-ase source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-context sink 0 2
bluezaudio le-gatt-db update-context sink 0 2
bluezaudio le-gatt-db notify-pac sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-ascs-cp update-metadata sink 0 2
bluezaudio le-bap-policy suspend-stream sink 0 2
bluezaudio le-ascs-cp disable sink 0 2
bluezaudio le-ascs-cp receiver-stop-ready sink 0 2
bluezaudio le-bap-policy stop-stream sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-context sink 0 2
bluezaudio le-gatt-db update-context sink 0 2
bluezaudio le-gatt-db notify-pac sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-gatt-db write-ascs-cp sink 0 2
bluezaudio le-gatt-db notify-ase sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
btctl upstream status
EOF

  cat >"${out_dir}/bluez-le-audio-bap-pacs-ascs-metadata-reconfig.ble2.nsh" <<'EOF'
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-context sink 0 1
bluezaudio le-gatt-db update-context sink 0 1
bluezaudio le-gatt-db notify-pac sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-ascs-cp update-metadata sink 0 1
bluezaudio le-bap-policy suspend-stream sink 0 1
bluezaudio le-ascs-cp disable sink 0 1
bluezaudio le-ascs-cp receiver-stop-ready sink 0 1
bluezaudio le-bap-policy stop-stream sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-context sink 0 1
bluezaudio le-gatt-db update-context sink 0 1
bluezaudio le-gatt-db notify-pac sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-gatt-db write-ascs-cp sink 0 1
bluezaudio le-gatt-db notify-ase sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-context source 0 2
bluezaudio le-gatt-db update-context source 0 2
bluezaudio le-gatt-db notify-pac source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-ascs-cp update-metadata source 0 2
bluezaudio le-bap-policy suspend-stream source 0 2
bluezaudio le-ascs-cp disable source 0 2
bluezaudio le-ascs-cp receiver-stop-ready source 0 2
bluezaudio le-bap-policy stop-stream source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-context source 0 2
bluezaudio le-gatt-db update-context source 0 2
bluezaudio le-gatt-db notify-pac source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-gatt-db write-ascs-cp source 0 2
bluezaudio le-gatt-db notify-ase source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
btctl upstream status
EOF
}

case_bluez_le_audio_codec_qos_policy_matrix() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-codec-qos-policy-matrix.ble1.nsh" <<'EOF'
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-pac source 0 1
bluezaudio le-gatt-db read-context source 0 1
bluezaudio le-gatt-db update-context source 0 1
bluezaudio le-gatt-db notify-pac source 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-ascs-cp update-metadata source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-pac sink 0 2
bluezaudio le-gatt-db read-context sink 0 2
bluezaudio le-gatt-db update-context sink 0 2
bluezaudio le-gatt-db notify-pac sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-ascs-cp config-qos-reject sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-ascs-cp update-metadata sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-gatt-db release sink 0 2
btctl upstream status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-codec-qos-policy-matrix.ble2.nsh" <<'EOF'
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-pac sink 0 1
bluezaudio le-gatt-db read-context sink 0 1
bluezaudio le-gatt-db update-context sink 0 1
bluezaudio le-gatt-db notify-pac sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-ascs-cp config-qos-reject sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-ascs-cp update-metadata sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-pac source 0 2
bluezaudio le-gatt-db read-context source 0 2
bluezaudio le-gatt-db update-context source 0 2
bluezaudio le-gatt-db notify-pac source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-ascs-cp config-qos-reject source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-ascs-cp update-metadata source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-audio-codec source-lc3-encode-write-release 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-gatt-db release source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_role_soak() {
  local out_dir="$1"

  case_bluez_le_audio_codec_qos_policy_matrix "${out_dir}"
  cp "${out_dir}/bluez-le-audio-codec-qos-policy-matrix.ble1.nsh" \
     "${out_dir}/bluez-le-audio-role-soak.ble1.nsh"
  cp "${out_dir}/bluez-le-audio-codec-qos-policy-matrix.ble2.nsh" \
     "${out_dir}/bluez-le-audio-role-soak.ble2.nsh"

  cat >>"${out_dir}/bluez-le-audio-role-soak.ble1.nsh" <<'EOF'
sleep 1
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-daemon unicast-profile-flow source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-mgmt-control disconnect source 0 1
bluezaudio le-mgmt-control error source 0 1
bluezaudio le-mgmt-control power-on sink 0 2
bluezaudio le-mgmt-control scan-start sink 0 2
bluezaudio le-mgmt-control connect sink 0 2
bluezaudio le-mgmt-control security sink 0 2
bluezaudio le-mgmt-control cis-request sink 0 2
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client owner-lost sink 0 2
bluezaudio le-dbus-client owner-reacquire sink 0 2
bluezaudio le-daemon unicast-profile-flow sink 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-mgmt-control disconnect sink 0 2
bluezaudio le-mgmt-control error sink 0 2
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-iso-socket open source 0 1
bluezaudio le-iso-socket bind-cis source 0 1
bluezaudio le-iso-socket connect source 0 1
bluezaudio le-iso-socket pollout source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket shutdown source 0 1
bluezaudio le-iso-socket close source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-iso-socket open sink 0 2
bluezaudio le-iso-socket bind-cis sink 0 2
bluezaudio le-iso-socket listen sink 0 2
bluezaudio le-iso-socket accept sink 0 2
bluezaudio le-iso-socket pollin sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket shutdown sink 0 2
bluezaudio le-iso-socket close sink 0 2
bluezaudio le-dbus-client release sink 0 2
btctl upstream status
btctl state
EOF

  cat >>"${out_dir}/bluez-le-audio-role-soak.ble2.nsh" <<'EOF'
sleep 1
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-daemon unicast-profile-flow sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-mgmt-control disconnect sink 0 1
bluezaudio le-mgmt-control error sink 0 1
bluezaudio le-mgmt-control power-on source 0 2
bluezaudio le-mgmt-control scan-start source 0 2
bluezaudio le-mgmt-control connect source 0 2
bluezaudio le-mgmt-control security source 0 2
bluezaudio le-mgmt-control cis-request source 0 2
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client owner-lost source 0 2
bluezaudio le-dbus-client owner-reacquire source 0 2
bluezaudio le-daemon unicast-profile-flow source 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-mgmt-control disconnect source 0 2
bluezaudio le-mgmt-control error source 0 2
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-iso-socket open sink 0 1
bluezaudio le-iso-socket bind-cis sink 0 1
bluezaudio le-iso-socket listen sink 0 1
bluezaudio le-iso-socket accept sink 0 1
bluezaudio le-iso-socket pollin sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket shutdown sink 0 1
bluezaudio le-iso-socket close sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-iso-socket open source 0 2
bluezaudio le-iso-socket bind-cis source 0 2
bluezaudio le-iso-socket connect source 0 2
bluezaudio le-iso-socket pollout source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket shutdown source 0 2
bluezaudio le-iso-socket close source 0 2
bluezaudio le-dbus-client release source 0 2
btctl upstream status
btctl state
EOF
}

case_bluez_le_audio_umbrella() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-le-audio-umbrella.ble1.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-cap-control coordinator-register
bluezaudio le-vcp-control register source 0 1
bluezaudio le-vcp-control discover source 0 1
bluezaudio le-vcp-control read-state source 0 1
bluezaudio le-vcp-control set-volume source 0 1
bluezaudio le-vcp-control notify-state source 0 1
bluezaudio le-vcp-control flags source 0 1
bluezaudio le-vcp-control error source 0 1
bluezaudio le-micp-control register source 0 1
bluezaudio le-micp-control discover source 0 1
bluezaudio le-micp-control read-state source 0 1
bluezaudio le-micp-control mute source 0 1
bluezaudio le-micp-control notify-state source 0 1
bluezaudio le-micp-control flags source 0 1
bluezaudio le-micp-control error source 0 1
bluezaudio le-csip-control register source 0 1
bluezaudio le-csip-control discover source 0 1
bluezaudio le-csip-control read-sirk source 0 1
bluezaudio le-csip-control read-size source 0 1
bluezaudio le-csip-control read-rank source 0 1
bluezaudio le-csip-control lock source 0 1
bluezaudio le-csip-control unlock source 0 1
bluezaudio le-csip-control notify source 0 1
bluezaudio le-csip-control error source 0 1
bluezaudio le-mcp-control register source 0 1
bluezaudio le-mcp-control discover source 0 1
bluezaudio le-mcp-control read-player source 0 1
bluezaudio le-mcp-control read-track source 0 1
bluezaudio le-mcp-control play source 0 1
bluezaudio le-mcp-control pause source 0 1
bluezaudio le-mcp-control next source 0 1
bluezaudio le-mcp-control notify-state source 0 1
bluezaudio le-mcp-control search source 0 1
bluezaudio le-mcp-control error source 0 1
bluezaudio le-tmap-control register source 0 1
bluezaudio le-tmap-control discover source 0 1
bluezaudio le-tmap-control read-role source 0 1
bluezaudio le-tmap-control update-role source 0 1
bluezaudio le-tmap-control notify-role source 0 1
bluezaudio le-tmap-control error source 0 1
bluezaudio le-ccp-control register source 0 1
bluezaudio le-ccp-control discover source 0 1
bluezaudio le-ccp-control read-bearer source 0 1
bluezaudio le-ccp-control read-call-state source 0 1
bluezaudio le-ccp-control originate source 0 1
bluezaudio le-ccp-control accept source 0 1
bluezaudio le-ccp-control terminate source 0 1
bluezaudio le-ccp-control notify-call-state source 0 1
bluezaudio le-ccp-control termination-reason source 0 1
bluezaudio le-ccp-control error source 0 1
bluezaudio le-gmap-control register source 0 1
bluezaudio le-gmap-control discover source 0 1
bluezaudio le-gmap-control read-role source 0 1
bluezaudio le-gmap-control update-role source 0 1
bluezaudio le-gmap-control notify-role source 0 1
bluezaudio le-gmap-control error source 0 1
bluezaudio le-bap-control source-announce 0 1
bluezaudio le-broadcast-iso adv-start source 0 1
bluezaudio le-broadcast-iso base-config source 0 1
bluezaudio le-broadcast-security set-code source 0 1
bluezaudio le-broadcast-iso big-create source 0 1
bluezaudio le-broadcast-security encrypt-big source 0 1
bluezaudio le-broadcast-iso bis-setup source 0 1
bluezaudio le-broadcast-iso bis-bind source 0 1
btctl upstream iso-bind 0 0x0101
btctl upstream iso-connect 0
bluezaudio le-bap-control source-start 0 1
bluezaudio le-broadcast-iso bis-credit source 0 1
bluezaudio le-broadcast-source start 0 1
bluezaudio le-broadcast-iso bis-complete source 0 1
btctl upstream pump
btctl upstream status
bluezaudio le-bap-control source-stop 0 1
bluezaudio le-broadcast-iso big-terminate source 0 1
bluezaudio le-broadcast-security clear-code source 0 1
btctl upstream iso-close
bluezhciraw user-iso-setup-bidir-monitor
btctl upstream status
btctl upstream hci-status
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-att-bearer open source 0 1
bluezaudio le-att-io attach source 0 1
bluezaudio le-att-io watch-rx source 0 1
bluezaudio le-att-io watch-tx source 0 1
bluezaudio le-att-queue alloc-req source 0 1
bluezaudio le-att-queue enqueue source 0 1
bluezaudio le-att-queue socket-write source 0 1
bluezaudio le-att-bearer mtu-exchange source 0 1
bluezaudio le-att-bearer security source 0 1
bluezaudio le-gatt-db register source 0 1
bluezaudio le-gatt-db discover-pacs source 0 1
bluezaudio le-gatt-db discover-ascs source 0 1
bluezaudio le-gatt-db read-pac source 0 1
bluezaudio le-gatt-db read-location source 0 1
bluezaudio le-gatt-db read-context source 0 1
bluezaudio le-gatt-db update-context source 0 1
bluezaudio le-gatt-db notify-pac source 0 1
bluezaudio le-gatt-db read-ase source 0 1
bluezaudio le-att-bearer enable-ccc source 0 1
bluezaudio le-cap-control group-config 0 1
bluezaudio le-bap-policy scheduler-register source 0 1
bluezaudio le-bap-policy select-codec source 0 1
bluezaudio le-ascs-cp config-codec source 0 1
bluezaudio le-ascs-cp config-qos-reject source 0 1
bluezaudio le-att-queue error-rsp source 0 1
bluezaudio le-att-queue socket-read source 0 1
bluezaudio le-att-io rx-pdu source 0 1
bluezaudio le-att-io fragment-write source 0 1
bluezaudio le-att-io reassemble source 0 1
bluezaudio le-att-bearer prepare-write source 0 1
bluezaudio le-att-bearer execute-write source 0 1
bluezaudio le-att-queue complete source 0 1
bluezaudio le-gatt-db write-ascs-cp source 0 1
bluezaudio le-bap-policy select-qos source 0 1
bluezaudio le-ascs-cp config-qos source 0 1
bluezaudio le-bap-policy select-cis source 0 1
bluezaudio le-unicast-control source-config 0 1
bluezaudio le-cap-control group-enable 0 1
bluezaudio le-ascs-cp enable source 0 1
bluezaudio le-unicast-control source-enable 0 1
bluezaudio le-ascs-cp receiver-start-ready source 0 1
bluezaudio le-ascs-cp update-metadata source 0 1
bluezaudio le-gatt-db notify-ase source 0 1
bluezaudio le-att-bearer indicate source 0 1
bluezaudio le-att-io tx-pdu source 0 1
bluezaudio le-att-io persist-ccc source 0 1
bluezaudio le-bap-policy bind-transport source 0 1
bluezaudio le-iso-socket open source 0 1
bluezaudio le-iso-socket bind-cis source 0 1
bluezaudio le-iso-qos configure source 0 1
bluezaudio le-iso-qos select-phy source 0 1
bluezaudio le-iso-qos setup-cig source 0 1
bluezaudio le-iso-qos setup-cis source 0 1
bluezaudio le-iso-qos apply-qos source 0 1
bluezaudio le-iso-qos controller-timing source 0 1
bluezaudio le-iso-qos credit-grant source 0 1
bluezaudio le-iso-socket connect source 0 1
bluezaudio le-iso-socket pollout source 0 1
bluezaudio le-bap-policy start-stream source 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-iso-socket timestamp source 0 1
bluezaudio le-iso-socket error-eagain source 0 1
bluezaudio le-iso-qos credit-complete source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
btctl upstream status
bluezaudio le-iso-qos teardown source 0 1
bluezaudio le-iso-socket shutdown source 0 1
bluezaudio le-iso-socket close source 0 1
bluezaudio le-bap-policy suspend-stream source 0 1
bluezaudio le-ascs-cp disable source 0 1
bluezaudio le-ascs-cp receiver-stop-ready source 0 1
bluezaudio le-bap-policy stop-stream source 0 1
bluezaudio le-ascs-cp release source 0 1
bluezaudio le-bap-policy scheduler-release source 0 1
bluezaudio le-unicast-control source-release 0 1
bluezaudio le-cap-control group-release 0 1
bluezaudio le-gatt-db release source 0 1
bluezaudio le-att-queue timeout source 0 1
bluezaudio le-att-queue cancel source 0 1
bluezaudio le-att-queue free-req source 0 1
bluezaudio le-att-io flush source 0 1
bluezaudio le-att-io detach source 0 1
bluezaudio le-att-bearer close source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-dbus-client register sink 0 2
bluezaudio le-dbus-client configure sink 0 2
bluezaudio le-att-bearer open sink 0 2
bluezaudio le-att-io attach sink 0 2
bluezaudio le-att-io watch-rx sink 0 2
bluezaudio le-att-io watch-tx sink 0 2
bluezaudio le-att-queue alloc-req sink 0 2
bluezaudio le-att-queue enqueue sink 0 2
bluezaudio le-att-queue socket-write sink 0 2
bluezaudio le-att-bearer mtu-exchange sink 0 2
bluezaudio le-att-bearer security sink 0 2
bluezaudio le-gatt-db register sink 0 2
bluezaudio le-gatt-db discover-pacs sink 0 2
bluezaudio le-gatt-db discover-ascs sink 0 2
bluezaudio le-gatt-db read-pac sink 0 2
bluezaudio le-gatt-db read-location sink 0 2
bluezaudio le-gatt-db read-context sink 0 2
bluezaudio le-gatt-db update-context sink 0 2
bluezaudio le-gatt-db notify-pac sink 0 2
bluezaudio le-gatt-db read-ase sink 0 2
bluezaudio le-att-bearer enable-ccc sink 0 2
bluezaudio le-bap-policy scheduler-register sink 0 2
bluezaudio le-bap-policy select-codec sink 0 2
bluezaudio le-ascs-cp config-codec sink 0 2
bluezaudio le-att-queue socket-read sink 0 2
bluezaudio le-att-io rx-pdu sink 0 2
bluezaudio le-att-io fragment-write sink 0 2
bluezaudio le-att-io reassemble sink 0 2
bluezaudio le-att-bearer prepare-write sink 0 2
bluezaudio le-att-bearer execute-write sink 0 2
bluezaudio le-att-queue complete sink 0 2
bluezaudio le-gatt-db write-ascs-cp sink 0 2
bluezaudio le-bap-policy select-qos sink 0 2
bluezaudio le-ascs-cp config-qos sink 0 2
bluezaudio le-bap-policy select-cis sink 0 2
bluezaudio le-unicast-control sink-discover 0 2
bluezaudio le-unicast-control sink-config 0 2
bluezaudio le-ascs-cp enable sink 0 2
bluezaudio le-unicast-control sink-enable 0 2
bluezaudio le-ascs-cp receiver-start-ready sink 0 2
bluezaudio le-ascs-cp update-metadata sink 0 2
bluezaudio le-gatt-db notify-ase sink 0 2
bluezaudio le-att-bearer indicate sink 0 2
bluezaudio le-att-io tx-pdu sink 0 2
bluezaudio le-att-io persist-ccc sink 0 2
bluezaudio le-bap-policy bind-transport sink 0 2
bluezaudio le-iso-socket open sink 0 2
bluezaudio le-iso-socket bind-cis sink 0 2
bluezaudio le-iso-qos configure sink 0 2
bluezaudio le-iso-qos select-phy sink 0 2
bluezaudio le-iso-qos setup-cig sink 0 2
bluezaudio le-iso-qos setup-cis sink 0 2
bluezaudio le-iso-qos apply-qos sink 0 2
bluezaudio le-iso-qos controller-timing sink 0 2
bluezaudio le-iso-qos credit-grant sink 0 2
bluezaudio le-iso-socket listen sink 0 2
bluezaudio le-iso-socket accept sink 0 2
bluezaudio le-iso-socket pollin sink 0 2
bluezaudio le-bap-policy start-stream sink 0 2
bluezaudio le-dbus-client transport sink 0 2
bluezaudio le-dbus-client transport-busy sink 0 2
bluezaudio le-dbus-client owner-lost sink 0 2
bluezaudio le-dbus-client owner-reacquire sink 0 2
bluezaudio le-iso-socket recvmsg sink 0 2
bluezaudio le-iso-socket timestamp sink 0 2
bluezaudio le-iso-socket error-eagain sink 0 2
bluezaudio le-iso-qos credit-complete sink 0 2
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2
btctl upstream status
bluezaudio le-iso-qos teardown sink 0 2
bluezaudio le-iso-socket shutdown sink 0 2
bluezaudio le-iso-socket close sink 0 2
bluezaudio le-bap-policy suspend-stream sink 0 2
bluezaudio le-ascs-cp disable sink 0 2
bluezaudio le-ascs-cp receiver-stop-ready sink 0 2
bluezaudio le-bap-policy stop-stream sink 0 2
bluezaudio le-ascs-cp release sink 0 2
bluezaudio le-bap-policy scheduler-release sink 0 2
bluezaudio le-unicast-control sink-release 0 2
bluezaudio le-gatt-db release sink 0 2
bluezaudio le-att-queue timeout sink 0 2
bluezaudio le-att-queue cancel sink 0 2
bluezaudio le-att-queue free-req sink 0 2
bluezaudio le-att-io flush sink 0 2
bluezaudio le-att-io detach sink 0 2
bluezaudio le-att-bearer close sink 0 2
bluezaudio le-dbus-client release sink 0 2
bluezaudio le-tmap-control release source 0 1
bluezaudio le-ccp-control release source 0 1
bluezaudio le-gmap-control release source 0 1
bluezaudio le-mcp-control release source 0 1
bluezaudio le-csip-control release source 0 1
bluezaudio le-micp-control release source 0 1
bluezaudio le-vcp-control release source 0 1
bluezaudio le-cap-control coordinator-release
bluezaudio le-daemon profile-release
bluezaudio le-mgmt-control disconnect source 0 1
bluezaudio le-mgmt-control error source 0 1
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezleaudio closeout source 0 1
btctl upstream status
btctl upstream hci-status
btctl state
EOF

  cat >"${out_dir}/bluez-le-audio-umbrella.ble2.nsh" <<'EOF'
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-bass-control assistant-register
bluezaudio le-vcp-control register sink 0 1
bluezaudio le-vcp-control discover sink 0 1
bluezaudio le-vcp-control read-state sink 0 1
bluezaudio le-vcp-control set-volume sink 0 1
bluezaudio le-vcp-control notify-state sink 0 1
bluezaudio le-vcp-control flags sink 0 1
bluezaudio le-vcp-control error sink 0 1
bluezaudio le-micp-control register sink 0 1
bluezaudio le-micp-control discover sink 0 1
bluezaudio le-micp-control read-state sink 0 1
bluezaudio le-micp-control mute sink 0 1
bluezaudio le-micp-control notify-state sink 0 1
bluezaudio le-micp-control flags sink 0 1
bluezaudio le-micp-control error sink 0 1
bluezaudio le-csip-control register sink 0 1
bluezaudio le-csip-control discover sink 0 1
bluezaudio le-csip-control read-sirk sink 0 1
bluezaudio le-csip-control read-size sink 0 1
bluezaudio le-csip-control read-rank sink 0 1
bluezaudio le-csip-control lock sink 0 1
bluezaudio le-csip-control unlock sink 0 1
bluezaudio le-csip-control notify sink 0 1
bluezaudio le-csip-control error sink 0 1
bluezaudio le-mcp-control register sink 0 1
bluezaudio le-mcp-control discover sink 0 1
bluezaudio le-mcp-control read-player sink 0 1
bluezaudio le-mcp-control read-track sink 0 1
bluezaudio le-mcp-control play sink 0 1
bluezaudio le-mcp-control pause sink 0 1
bluezaudio le-mcp-control next sink 0 1
bluezaudio le-mcp-control notify-state sink 0 1
bluezaudio le-mcp-control search sink 0 1
bluezaudio le-mcp-control error sink 0 1
bluezaudio le-tmap-control register sink 0 1
bluezaudio le-tmap-control discover sink 0 1
bluezaudio le-tmap-control read-role sink 0 1
bluezaudio le-tmap-control update-role sink 0 1
bluezaudio le-tmap-control notify-role sink 0 1
bluezaudio le-tmap-control error sink 0 1
bluezaudio le-ccp-control register sink 0 1
bluezaudio le-ccp-control discover sink 0 1
bluezaudio le-ccp-control read-bearer sink 0 1
bluezaudio le-ccp-control read-call-state sink 0 1
bluezaudio le-ccp-control originate sink 0 1
bluezaudio le-ccp-control accept sink 0 1
bluezaudio le-ccp-control terminate sink 0 1
bluezaudio le-ccp-control notify-call-state sink 0 1
bluezaudio le-ccp-control termination-reason sink 0 1
bluezaudio le-ccp-control error sink 0 1
bluezaudio le-gmap-control register sink 0 1
bluezaudio le-gmap-control discover sink 0 1
bluezaudio le-gmap-control read-role sink 0 1
bluezaudio le-gmap-control update-role sink 0 1
bluezaudio le-gmap-control notify-role sink 0 1
bluezaudio le-gmap-control error sink 0 1
bluezaudio le-bap-control sink-discover 0 1
bluezaudio le-bass-control add-source 0 1
bluezaudio le-broadcast-iso pa-sync sink 0 1
bluezaudio le-broadcast-security bad-code sink 0 1
bluezaudio le-broadcast-security decrypt-setup sink 0 1
bluezaudio le-bap-control sink-config 0 1
bluezaudio le-broadcast-sink sync 0 1
bluezaudio le-broadcast-iso big-sync sink 0 1
bluezaudio le-broadcast-iso receive-state sink 0 1
bluezaudio le-broadcast-security receive-state-encrypted sink 0 1
bluezaudio le-broadcast-iso bis-credit sink 0 1
bluezaudio le-bass-control modify-source 0 1
bluezaudio le-bap-control sink-sync 0 1
btctl upstream pump
sleep 4
bluezaudio le-broadcast-sink start 0 1
bluezaudio le-broadcast-iso bis-complete sink 0 1
btctl upstream status
bluezaudio le-broadcast-sink stop
bluezaudio le-broadcast-iso big-terminate sink 0 1
bluezaudio le-broadcast-security clear-code sink 0 1
bluezaudio le-bass-control remove-source 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-att-bearer open sink 0 1
bluezaudio le-att-io attach sink 0 1
bluezaudio le-att-io watch-rx sink 0 1
bluezaudio le-att-io watch-tx sink 0 1
bluezaudio le-att-queue alloc-req sink 0 1
bluezaudio le-att-queue enqueue sink 0 1
bluezaudio le-att-queue socket-write sink 0 1
bluezaudio le-att-bearer mtu-exchange sink 0 1
bluezaudio le-att-bearer security sink 0 1
bluezaudio le-gatt-db register sink 0 1
bluezaudio le-gatt-db discover-pacs sink 0 1
bluezaudio le-gatt-db discover-ascs sink 0 1
bluezaudio le-gatt-db read-pac sink 0 1
bluezaudio le-gatt-db read-location sink 0 1
bluezaudio le-gatt-db read-context sink 0 1
bluezaudio le-gatt-db update-context sink 0 1
bluezaudio le-gatt-db notify-pac sink 0 1
bluezaudio le-gatt-db read-ase sink 0 1
bluezaudio le-att-bearer enable-ccc sink 0 1
bluezaudio le-bap-policy scheduler-register sink 0 1
bluezaudio le-bap-policy select-codec sink 0 1
bluezaudio le-ascs-cp config-codec sink 0 1
bluezaudio le-att-queue socket-read sink 0 1
bluezaudio le-att-io rx-pdu sink 0 1
bluezaudio le-att-io fragment-write sink 0 1
bluezaudio le-att-io reassemble sink 0 1
bluezaudio le-att-bearer prepare-write sink 0 1
bluezaudio le-att-bearer execute-write sink 0 1
bluezaudio le-att-queue complete sink 0 1
bluezaudio le-gatt-db write-ascs-cp sink 0 1
bluezaudio le-bap-policy select-qos sink 0 1
bluezaudio le-ascs-cp config-qos sink 0 1
bluezaudio le-bap-policy select-cis sink 0 1
bluezaudio le-unicast-control sink-discover 0 1
bluezaudio le-unicast-control sink-config 0 1
bluezaudio le-ascs-cp enable sink 0 1
bluezaudio le-unicast-control sink-enable 0 1
bluezaudio le-ascs-cp receiver-start-ready sink 0 1
bluezaudio le-ascs-cp update-metadata sink 0 1
bluezaudio le-gatt-db notify-ase sink 0 1
bluezaudio le-att-bearer indicate sink 0 1
bluezaudio le-att-io tx-pdu sink 0 1
bluezaudio le-att-io persist-ccc sink 0 1
bluezaudio le-bap-policy bind-transport sink 0 1
bluezaudio le-iso-socket open sink 0 1
bluezaudio le-iso-socket bind-cis sink 0 1
bluezaudio le-iso-qos configure sink 0 1
bluezaudio le-iso-qos select-phy sink 0 1
bluezaudio le-iso-qos setup-cig sink 0 1
bluezaudio le-iso-qos setup-cis sink 0 1
bluezaudio le-iso-qos apply-qos sink 0 1
bluezaudio le-iso-qos controller-timing sink 0 1
bluezaudio le-iso-qos credit-grant sink 0 1
bluezaudio le-iso-socket listen sink 0 1
bluezaudio le-iso-socket accept sink 0 1
bluezaudio le-iso-socket pollin sink 0 1
bluezaudio le-bap-policy start-stream sink 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-iso-socket timestamp sink 0 1
bluezaudio le-iso-socket error-eagain sink 0 1
bluezaudio le-iso-qos credit-complete sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
btctl upstream status
bluezaudio le-iso-qos teardown sink 0 1
bluezaudio le-iso-socket shutdown sink 0 1
bluezaudio le-iso-socket close sink 0 1
bluezaudio le-bap-policy suspend-stream sink 0 1
bluezaudio le-ascs-cp disable sink 0 1
bluezaudio le-ascs-cp receiver-stop-ready sink 0 1
bluezaudio le-bap-policy stop-stream sink 0 1
bluezaudio le-ascs-cp release sink 0 1
bluezaudio le-bap-policy scheduler-release sink 0 1
bluezaudio le-unicast-control sink-release 0 1
bluezaudio le-gatt-db release sink 0 1
bluezaudio le-att-queue timeout sink 0 1
bluezaudio le-att-queue cancel sink 0 1
bluezaudio le-att-queue free-req sink 0 1
bluezaudio le-att-io flush sink 0 1
bluezaudio le-att-io detach sink 0 1
bluezaudio le-att-bearer close sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-dbus-client register source 0 2
bluezaudio le-dbus-client configure source 0 2
bluezaudio le-att-bearer open source 0 2
bluezaudio le-att-io attach source 0 2
bluezaudio le-att-io watch-rx source 0 2
bluezaudio le-att-io watch-tx source 0 2
bluezaudio le-att-queue alloc-req source 0 2
bluezaudio le-att-queue enqueue source 0 2
bluezaudio le-att-queue socket-write source 0 2
bluezaudio le-att-bearer mtu-exchange source 0 2
bluezaudio le-att-bearer security source 0 2
bluezaudio le-gatt-db register source 0 2
bluezaudio le-gatt-db discover-pacs source 0 2
bluezaudio le-gatt-db discover-ascs source 0 2
bluezaudio le-gatt-db read-pac source 0 2
bluezaudio le-gatt-db read-location source 0 2
bluezaudio le-gatt-db read-context source 0 2
bluezaudio le-gatt-db update-context source 0 2
bluezaudio le-gatt-db notify-pac source 0 2
bluezaudio le-gatt-db read-ase source 0 2
bluezaudio le-att-bearer enable-ccc source 0 2
bluezaudio le-bap-policy scheduler-register source 0 2
bluezaudio le-bap-policy select-codec source 0 2
bluezaudio le-ascs-cp config-codec source 0 2
bluezaudio le-att-queue socket-read source 0 2
bluezaudio le-att-io rx-pdu source 0 2
bluezaudio le-att-io fragment-write source 0 2
bluezaudio le-att-io reassemble source 0 2
bluezaudio le-att-bearer prepare-write source 0 2
bluezaudio le-att-bearer execute-write source 0 2
bluezaudio le-att-queue complete source 0 2
bluezaudio le-gatt-db write-ascs-cp source 0 2
bluezaudio le-bap-policy select-qos source 0 2
bluezaudio le-ascs-cp config-qos source 0 2
bluezaudio le-bap-policy select-cis source 0 2
bluezaudio le-unicast-control source-config 0 2
bluezaudio le-ascs-cp enable source 0 2
bluezaudio le-unicast-control source-enable 0 2
bluezaudio le-ascs-cp receiver-start-ready source 0 2
bluezaudio le-ascs-cp update-metadata source 0 2
bluezaudio le-gatt-db notify-ase source 0 2
bluezaudio le-att-bearer indicate source 0 2
bluezaudio le-att-io tx-pdu source 0 2
bluezaudio le-att-io persist-ccc source 0 2
bluezaudio le-bap-policy bind-transport source 0 2
bluezaudio le-iso-socket open source 0 2
bluezaudio le-iso-socket bind-cis source 0 2
bluezaudio le-iso-qos configure source 0 2
bluezaudio le-iso-qos select-phy source 0 2
bluezaudio le-iso-qos setup-cig source 0 2
bluezaudio le-iso-qos setup-cis source 0 2
bluezaudio le-iso-qos apply-qos source 0 2
bluezaudio le-iso-qos controller-timing source 0 2
bluezaudio le-iso-qos credit-grant source 0 2
bluezaudio le-iso-socket connect source 0 2
bluezaudio le-iso-socket pollout source 0 2
bluezaudio le-bap-policy start-stream source 0 2
bluezaudio le-dbus-client transport source 0 2
bluezaudio le-dbus-client transport-busy source 0 2
bluezaudio le-dbus-client owner-lost source 0 2
bluezaudio le-dbus-client owner-reacquire source 0 2
bluezaudio le-iso-socket sendmsg source 0 2
bluezaudio le-iso-socket timestamp source 0 2
bluezaudio le-iso-socket error-eagain source 0 2
bluezaudio le-iso-qos credit-complete source 0 2
bluezaudio le-audio-codec source-lc3-encode-write-release 0 2
btctl upstream status
bluezaudio le-iso-qos teardown source 0 2
bluezaudio le-iso-socket shutdown source 0 2
bluezaudio le-iso-socket close source 0 2
bluezaudio le-bap-policy suspend-stream source 0 2
bluezaudio le-ascs-cp disable source 0 2
bluezaudio le-ascs-cp receiver-stop-ready source 0 2
bluezaudio le-bap-policy stop-stream source 0 2
bluezaudio le-ascs-cp release source 0 2
bluezaudio le-bap-policy scheduler-release source 0 2
bluezaudio le-unicast-control source-release 0 2
bluezaudio le-gatt-db release source 0 2
bluezaudio le-att-queue timeout source 0 2
bluezaudio le-att-queue cancel source 0 2
bluezaudio le-att-queue free-req source 0 2
bluezaudio le-att-io flush source 0 2
bluezaudio le-att-io detach source 0 2
bluezaudio le-att-bearer close source 0 2
bluezaudio le-dbus-client release source 0 2
bluezaudio le-tmap-control release sink 0 1
bluezaudio le-ccp-control release sink 0 1
bluezaudio le-gmap-control release sink 0 1
bluezaudio le-mcp-control release sink 0 1
bluezaudio le-csip-control release sink 0 1
bluezaudio le-micp-control release sink 0 1
bluezaudio le-vcp-control release sink 0 1
bluezaudio le-bass-control assistant-release
bluezaudio le-daemon profile-release
bluezaudio le-mgmt-control disconnect sink 0 1
bluezaudio le-mgmt-control error sink 0 1
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezleaudio closeout sink 0 1
btctl upstream status
btctl state
EOF
}

case_bluez_ipsp_closeout_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-ipsp-closeout-full.ble1.nsh" <<'EOF'
echo BLUEZIPSP_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 56000 &
bluezipsp connect bt%d
bluezipsp status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
ping6 -c 2 -W 5000 fc00::2
iperf -V -s -B fc00::1 -i 1 -t 6 &
sleep 8
bluezipsp status
bluezipsp disconnect
bluezipsp status
sleep 1
bluezipsp connect bt%d
bluezipsp status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::1 -i 1 -t 6 &
sleep 8
bluezipsp status
bluezipsp disconnect
bluezipsp status
echo BLUEZIPSP_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-ipsp-closeout-full.ble2.nsh" <<'EOF'
echo BLUEZIPSP_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
btctl connect 3
btctl upstream hci-pump 56000 &
bluezipsp connect bt%d
bluezipsp status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 4
ping6 -c 2 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -i 1 -t 4
bluezipsp status
bluezipsp disconnect
bluezipsp status
sleep 1
bluezipsp connect bt%d
bluezipsp status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -c fc00::1 -B fc00::2 -u -i 1 -t 4
bluezipsp status
bluezipsp disconnect
bluezipsp status
echo BLUEZIPSP_DONE_BLE2
EOF
}

case_bluez_daemon_ipsp_closeout_full() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-daemon-ipsp-closeout-full.ble1.nsh" <<'EOF'
echo BLUEZDAEMONIPSP_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 56000 &
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
ping6 -c 2 -W 5000 fc00::2
iperf -V -s -B fc00::1 -i 1 -t 6 &
sleep 8
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
sleep 1
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::1 -i 1 -t 6 &
sleep 8
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
echo BLUEZDAEMONIPSP_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-daemon-ipsp-closeout-full.ble2.nsh" <<'EOF'
echo BLUEZDAEMONIPSP_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
btctl connect 3
btctl upstream hci-pump 56000 &
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 4
ping6 -c 2 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -i 1 -t 4
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
sleep 1
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -c fc00::1 -B fc00::2 -u -i 1 -t 4
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
echo BLUEZDAEMONIPSP_DONE_BLE2
EOF
}

case_bluez_net_current_complete_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-net-current-complete-closeout.bt1.nsh" <<'EOF'
echo BLUEZNETCOMPLETE_BEGIN_BT1
blueznetwork closeout-full begin
blueznetwork daemon-profile register
btctl upstream hci-pump 52000 &
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
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path gn
btctl upstream status
blueznetwork daemon-profile unregister
blueznetwork closeout-full end
btctl upstream status
echo BLUEZNETCOMPLETE_DONE_BT1
EOF

  cat >"${out_dir}/bluez-net-current-complete-closeout.bt2.nsh" <<'EOF'
echo BLUEZNETCOMPLETE_BEGIN_BT2
blueznetwork closeout-full begin
blueznetwork daemon-profile register
btctl upstream hci-pump 52000 &
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
ping -s 1400 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path panu
btctl upstream status
blueznetwork daemon-profile unregister
blueznetwork closeout-full end
btctl upstream status
echo BLUEZNETCOMPLETE_DONE_BT2
EOF

  cat >"${out_dir}/bluez-net-current-complete-closeout.ble1.nsh" <<'EOF'
echo BLUEZNETCOMPLETE_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 56000 &
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
ping6 -c 2 -W 5000 fc00::2
iperf -V -s -B fc00::1 -i 1 -t 6 &
sleep 8
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
sleep 1
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::1 -i 1 -t 6 &
sleep 8
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
echo BLUEZNETCOMPLETE_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-net-current-complete-closeout.ble2.nsh" <<'EOF'
echo BLUEZNETCOMPLETE_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
btctl connect 3
btctl upstream hci-pump 56000 &
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 4
ping6 -c 2 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -i 1 -t 4
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
sleep 1
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -c fc00::1 -B fc00::2 -u -i 1 -t 4
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
echo BLUEZNETCOMPLETE_DONE_BLE2
EOF
}

case_bluez_net_upstream_convergence_closeout() {
  local out_dir="$1"
  local role
  local src
  local dst

  case_bluez_net_current_complete_closeout "${out_dir}"
  for role in bt1 bt2 ble1 ble2; do
    src="${out_dir}/bluez-net-current-complete-closeout.${role}.nsh"
    dst="${out_dir}/bluez-net-upstream-convergence-closeout.${role}.nsh"
    cp "${src}" "${dst}"
    sed -i 's/BLUEZNETCOMPLETE/BLUEZNETUPSTREAM/g' "${dst}"
    case "${role}" in
      bt1|bt2)
        sed -i '/echo BLUEZNETUPSTREAM_DONE_BT[12]/i bluezbneptest native-closeout' "${dst}"
        ;;
    esac
  done
}

case_bluez_current_functional_closeout() {
  local out_dir="$1"

  cat >"${out_dir}/bluez-current-functional-closeout.bt1.nsh" <<'EOF'
echo BLUEZCURRENT_BEGIN_BT1
btctl upstream hci-pump 96000 &
sleep 4
bluezdaemon audio-a2dp-closeout-full source 2
btctl upstream status
sleep 1
blueznetwork closeout-full begin
blueznetwork daemon-profile register
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
ifconfig btn0 10.77.0.1 dr 10.77.0.2 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 2 -W 5000 10.77.0.2
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path gn
btctl upstream status
blueznetwork daemon-profile unregister
blueznetwork closeout-full end
btctl upstream status
bluezdaemon basic-closeout bt
bluezdaemon profile-hid-closeout classic-host 2
bluezhid closeout classic-host 2
bluezdaemon profile-hfp-closeout hfp-hf 2
bluezdaemon profile-hfp-closeout hsp-hs 2
bluezhfp closeout hfp-hf 2
bluezhfp closeout hsp-hs 2
bluezdaemon profile-obex-closeout pbap-client 2
bluezobex closeout pbap-client 2
bluezdaemon profile-obex-closeout opp-client 2
bluezobex closeout opp-client 2
bluezdaemon profile-map-closeout map-client 2
bluezobex closeout map-client 2
bluezdaemon profile-map-closeout mns-client 2
bluezobex closeout mns-client 2
bluezdaemon profile-sync-closeout ftp-client 2
bluezobex closeout ftp-client 2
bluezdaemon profile-sync-closeout sync-client 2
bluezobex closeout sync-client 2
bluezdaemon profile-bip-closeout client 2
bluezobex closeout bip-client 2
bluezdaemon profile-print-closeout client 2
bluezprofile closeout print-client 2
bluezdaemon profile-iap-closeout controller 2
bluezprofile closeout iap-controller 2
btctl state
echo BLUEZCURRENT_DONE_BT1
EOF

  cat >"${out_dir}/bluez-current-functional-closeout.bt2.nsh" <<'EOF'
echo BLUEZCURRENT_BEGIN_BT2
bluezdaemon audio-a2dp-closeout-full sink 1
btctl upstream status
btctl upstream hci-pump 96000 &
sleep 1
blueznetwork closeout-full begin
blueznetwork daemon-profile register
blueznetwork daemon-profile connect nap
blueznetwork daemon-profile status nap
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
ping -s 1400 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect nap
sleep 1
btctl upstream status
blueznetwork daemon-profile connect gn
blueznetwork daemon-profile status gn
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect gn
sleep 1
btctl upstream status
blueznetwork daemon-profile connect panu
blueznetwork daemon-profile status panu
ifconfig btn0 10.77.0.2 dr 10.77.0.1 netmask 255.255.255.0
ifup btn0
sleep 1
ping -s 1400 -c 2 -W 5000 10.77.0.1
btctl upstream status
blueznetwork daemon-profile disconnect panu
sleep 1
btctl upstream status
blueznetwork daemon-profile error-path panu
btctl upstream status
blueznetwork daemon-profile unregister
blueznetwork closeout-full end
btctl upstream status
bluezdaemon basic-closeout bt
bluezdaemon profile-hid-closeout classic-device 1
bluezhid closeout classic-device 1
bluezdaemon profile-hfp-closeout hfp-ag 1
bluezdaemon profile-hfp-closeout hsp-ag 1
bluezhfp closeout hfp-ag 1
bluezhfp closeout hsp-ag 1
bluezdaemon profile-obex-closeout pbap-server 1
bluezobex closeout pbap-server 1
bluezdaemon profile-obex-closeout opp-server 1
bluezobex closeout opp-server 1
bluezdaemon profile-map-closeout map-server 1
bluezobex closeout map-server 1
bluezdaemon profile-map-closeout mns-server 1
bluezobex closeout mns-server 1
bluezdaemon profile-sync-closeout ftp-server 1
bluezobex closeout ftp-server 1
bluezdaemon profile-sync-closeout sync-server 1
bluezobex closeout sync-server 1
bluezdaemon profile-bip-closeout server 1
bluezobex closeout bip-server 1
bluezdaemon profile-print-closeout printer 1
bluezprofile closeout print-printer 1
bluezdaemon profile-iap-closeout accessory 1
bluezprofile closeout iap-accessory 1
btctl state
echo BLUEZCURRENT_DONE_BT2
EOF

  cat >"${out_dir}/bluez-current-functional-closeout.ble1.nsh" <<'EOF'
echo BLUEZCURRENT_BEGIN_BLE1
btctl info
btctl mgmt power on
btctl mgmt le on
btctl advertise start
sleep 1
btctl upstream hci-pump 96000 &
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
ping6 -c 2 -W 5000 fc00::2
iperf -V -s -B fc00::1 -i 1 -t 6 &
sleep 8
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
sleep 1
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::1 prefixlen 64
ifup bt0
ifconfig bt0
sleep 2
iperf -V -s -u -B fc00::1 -i 1 -t 6 &
sleep 8
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
bluezdaemon basic-closeout ble
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on source 0 1
bluezaudio le-mgmt-control scan-start source 0 1
bluezaudio le-mgmt-control connect source 0 1
bluezaudio le-mgmt-control security source 0 1
bluezaudio le-mgmt-control cis-request source 0 1
bluezaudio le-dbus-client register source 0 1
bluezaudio le-dbus-client configure source 0 1
bluezaudio le-iso-socket open source 0 1
bluezaudio le-iso-socket bind-cis source 0 1
bluezaudio le-iso-socket connect source 0 1
bluezaudio le-iso-socket pollout source 0 1
bluezaudio le-iso-socket sendmsg source 0 1
bluezaudio le-audio-codec source-lc3-encode-write-release 0 1
bluezaudio le-dbus-client transport source 0 1
bluezaudio le-dbus-client transport-busy source 0 1
bluezaudio le-dbus-client owner-lost source 0 1
bluezaudio le-dbus-client owner-reacquire source 0 1
bluezaudio le-dbus-client release source 0 1
bluezaudio le-gatt-upstream closeout source 0 1
bluezaudio le-daemon integrated-profile-flow source 0 1 2
bluezaudio le-daemon profile-release
bluezaudio le-mgmt-control disconnect source 0 1
bluezaudio le-mgmt-control error source 0 1
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezdaemon profile-hid-closeout hogp-host 4
bluezhid closeout hogp-host 4
bluezdaemon profile-gatt-closeout client 4
bluezgatt closeout source 0 4
bluezdaemon profile-mesh-closeout provisioner 4
bluezprofile closeout mesh-provisioner 4
bluezdaemon profile-asha-closeout central 4
bluezprofile closeout asha-central 4
bluezdaemon profile-midi-closeout controller 4
bluezprofile closeout midi-controller 4
bluezdaemon profile-ranging-closeout initiator 4
bluezprofile closeout ranging-initiator 4
echo BLUEZCURRENT_DONE_BLE1
EOF

  cat >"${out_dir}/bluez-current-functional-closeout.ble2.nsh" <<'EOF'
echo BLUEZCURRENT_BEGIN_BLE2
btctl info
btctl mgmt power on
btctl mgmt le on
sleep 1
btctl connect 3
btctl upstream hci-pump 96000 &
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 4
ping6 -c 2 -W 5000 fc00::1
iperf -V -c fc00::1 -B fc00::2 -i 1 -t 4
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
sleep 1
bluezdaemon ipsp-connect bt%d
bluezdaemon ipsp-status
ifconfig bt0 inet6 fc00::2 prefixlen 64
ifup bt0
ifconfig bt0
sleep 4
iperf -V -u -c fc00::1 -B fc00::2 -i 1 -t 4
bluezdaemon ipsp-status
bluezdaemon ipsp-disconnect
bluezdaemon ipsp-status
bluezdaemon basic-closeout ble
bluezaudio le-daemon plugin-init
bluezaudio le-daemon adapter-powered
bluezaudio le-daemon register
bluezaudio le-daemon mainloop-dispatch
bluezaudio le-daemon profile-accept
bluezaudio le-mgmt-control power-on sink 0 1
bluezaudio le-mgmt-control scan-start sink 0 1
bluezaudio le-mgmt-control connect sink 0 1
bluezaudio le-mgmt-control security sink 0 1
bluezaudio le-mgmt-control cis-request sink 0 1
bluezaudio le-dbus-client register sink 0 1
bluezaudio le-dbus-client configure sink 0 1
bluezaudio le-iso-socket open sink 0 1
bluezaudio le-iso-socket bind-cis sink 0 1
bluezaudio le-iso-socket listen sink 0 1
bluezaudio le-iso-socket accept sink 0 1
bluezaudio le-iso-socket pollin sink 0 1
bluezaudio le-iso-socket recvmsg sink 0 1
bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1
bluezaudio le-dbus-client transport sink 0 1
bluezaudio le-dbus-client transport-busy sink 0 1
bluezaudio le-dbus-client owner-lost sink 0 1
bluezaudio le-dbus-client owner-reacquire sink 0 1
bluezaudio le-dbus-client release sink 0 1
bluezaudio le-gatt-upstream closeout sink 0 1
bluezaudio le-daemon integrated-profile-flow sink 0 1 2
bluezaudio le-daemon profile-release
bluezaudio le-mgmt-control disconnect sink 0 1
bluezaudio le-mgmt-control error sink 0 1
bluezaudio le-daemon release
bluezaudio le-daemon plugin-exit
bluezdaemon profile-hid-closeout hogp-device 3
bluezhid closeout hogp-device 3
bluezdaemon profile-gatt-closeout server 3
bluezgatt closeout sink 0 3
bluezdaemon profile-mesh-closeout node 3
bluezprofile closeout mesh-node 3
bluezdaemon profile-asha-closeout hearing-aid 3
bluezprofile closeout asha-hearing-aid 3
bluezdaemon profile-midi-closeout peripheral 3
bluezprofile closeout midi-peripheral 3
bluezdaemon profile-ranging-closeout reflector 3
bluezprofile closeout ranging-reflector 3
btctl state
echo BLUEZCURRENT_DONE_BLE2
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
    ble-ip-closeout-full) case_ble_ip_closeout_full "${out_dir}" ;;
    bluez-ipsp-closeout-full) case_bluez_ipsp_closeout_full "${out_dir}" ;;
    bluez-daemon-ipsp-closeout-full) case_bluez_daemon_ipsp_closeout_full "${out_dir}" ;;
    bluez-net-current-complete-closeout) case_bluez_net_current_complete_closeout "${out_dir}" ;;
    bluez-net-upstream-convergence-closeout) case_bluez_net_upstream_convergence_closeout "${out_dir}" ;;
    bluez-current-functional-closeout) case_bluez_current_functional_closeout "${out_dir}" ;;
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
    bluez-mgmt-daemon-bootstrap) case_bluez_mgmt_daemon_bootstrap "${out_dir}" ;;
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
    bluez-hci-mgmt-socket-closeout-full) case_bluez_hci_mgmt_socket_closeout_full "${out_dir}" ;;
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
    bluez-network-closeout-full) case_bluez_network_closeout_full "${out_dir}" ;;
    bluez-network-error-path) case_bluez_network_error_path "${out_dir}" ;;
    bluez-network-iperf-tcp) case_bluez_network_iperf_tcp "${out_dir}" ;;
    bluez-network-iperf-tcp-reverse) case_bluez_network_iperf_tcp_reverse "${out_dir}" ;;
    bluez-network-iperf-udp) case_bluez_network_iperf_udp "${out_dir}" ;;
    bluez-network-iperf-udp-reverse) case_bluez_network_iperf_udp_reverse "${out_dir}" ;;
    bluez-network-iperf-tcp-soak) case_bluez_network_iperf_tcp_soak "${out_dir}" ;;
    bluez-network-iperf-matrix) case_bluez_network_iperf_matrix "${out_dir}" ;;
    bluez-network-frag-ping) case_bluez_network_frag_ping "${out_dir}" ;;
    bluez-network-jumbo-ping) case_bluez_network_jumbo_ping "${out_dir}" ;;
    bluez-network-mtu-ping) case_bluez_network_mtu_ping "${out_dir}" ;;
    bluez-network-mtu-soak) case_bluez_network_mtu_soak "${out_dir}" ;;
    bluez-network-mtu-reconnect-stress) case_bluez_network_mtu_reconnect_stress "${out_dir}" ;;
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
    bluez-a2dp-sbc-codec-concurrent) case_bluez_a2dp_sbc_codec_concurrent "${out_dir}" ;;
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
    bluez-daemon-a2dp-dbus-client-full) case_bluez_daemon_a2dp_dbus_client_full "${out_dir}" ;;
    bluez-daemon-a2dp-dbus-client-busy) case_bluez_daemon_a2dp_dbus_client_busy "${out_dir}" ;;
    bluez-daemon-a2dp-full-concurrent) case_bluez_daemon_a2dp_full_concurrent "${out_dir}" ;;
    bluez-daemon-a2dp-full-concurrent-reconnect) case_bluez_daemon_a2dp_full_concurrent_reconnect "${out_dir}" ;;
    bluez-daemon-a2dp-full-concurrent-soak) case_bluez_daemon_a2dp_full_concurrent_soak "${out_dir}" ;;
    bluez-daemon-a2dp-integrated-profile) case_bluez_daemon_a2dp_integrated_profile "${out_dir}" ;;
    bluez-daemon-a2dp-integrated-reconnect) case_bluez_daemon_a2dp_integrated_reconnect "${out_dir}" ;;
    bluez-daemon-a2dp-session-ownership) case_bluez_daemon_a2dp_session_ownership "${out_dir}" ;;
    bluez-daemon-a2dp-error-policy) case_bluez_daemon_a2dp_error_policy "${out_dir}" ;;
    bluez-daemon-a2dp-upstream-session) case_bluez_daemon_a2dp_upstream_session "${out_dir}" ;;
    bluez-daemon-a2dp-upstream-reconnect) case_bluez_daemon_a2dp_upstream_reconnect "${out_dir}" ;;
    bluez-daemon-a2dp-upstream-transactions) case_bluez_daemon_a2dp_upstream_transactions "${out_dir}" ;;
    bluez-daemon-a2dp-media-transport-fd) case_bluez_daemon_a2dp_media_transport_fd "${out_dir}" ;;
    bluez-daemon-a2dp-codec-policy) case_bluez_daemon_a2dp_codec_policy "${out_dir}" ;;
    bluez-daemon-a2dp-closeout-full) case_bluez_daemon_a2dp_closeout_full "${out_dir}" ;;
    bluez-a2dp-current-complete-closeout) case_bluez_a2dp_current_complete_closeout "${out_dir}" ;;
    bluez-a2dp-upstream-convergence-closeout) case_bluez_a2dp_upstream_convergence_closeout "${out_dir}" ;;
    bluez-basic-mgmt-flow) case_bluez_basic_mgmt_flow "${out_dir}" ;;
    bluez-basic-scan-connect-auth-flow) case_bluez_basic_scan_connect_auth_flow "${out_dir}" ;;
    bluez-basic-upstream-convergence-closeout) case_bluez_basic_upstream_convergence_closeout "${out_dir}" ;;
    bluez-hid-hogp-profile-closeout) case_bluez_hid_hogp_profile_closeout "${out_dir}" ;;
    bluez-hfp-hsp-profile-closeout) case_bluez_hfp_hsp_profile_closeout "${out_dir}" ;;
    bluez-obex-pbap-opp-profile-closeout) case_bluez_obex_pbap_opp_profile_closeout "${out_dir}" ;;
    bluez-obex-map-mns-profile-closeout) case_bluez_obex_map_mns_profile_closeout "${out_dir}" ;;
    bluez-obex-ftp-sync-profile-closeout) case_bluez_obex_ftp_sync_profile_closeout "${out_dir}" ;;
    bluez-mesh-profile-closeout) case_bluez_mesh_profile_closeout "${out_dir}" ;;
    bluez-gatt-profile-closeout) case_bluez_gatt_profile_closeout "${out_dir}" ;;
    bluez-asha-profile-closeout) case_bluez_asha_profile_closeout "${out_dir}" ;;
    bluez-obex-bip-profile-closeout) case_bluez_obex_bip_profile_closeout "${out_dir}" ;;
    bluez-print-profile-closeout) case_bluez_print_profile_closeout "${out_dir}" ;;
    bluez-iap-profile-closeout) case_bluez_iap_profile_closeout "${out_dir}" ;;
    bluez-midi-profile-closeout) case_bluez_midi_profile_closeout "${out_dir}" ;;
    bluez-ranging-profile-closeout) case_bluez_ranging_profile_closeout "${out_dir}" ;;
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
    bluez-le-audio-daemon-full-lifecycle) case_bluez_le_audio_daemon_full_lifecycle "${out_dir}" ;;
    bluez-le-audio-dbus-client-full) case_bluez_le_audio_dbus_client_full "${out_dir}" ;;
    bluez-le-audio-lc3-codec-transport) case_bluez_le_audio_lc3_codec_transport "${out_dir}" ;;
    bluez-le-audio-iso-dataplane-soak) case_bluez_le_audio_iso_dataplane_soak "${out_dir}" ;;
    bluez-le-audio-controller-iso-dataplane-soak) case_bluez_le_audio_controller_iso_dataplane_soak "${out_dir}" ;;
    bluez-le-audio-daemon-profile-flow) case_bluez_le_audio_daemon_profile_flow "${out_dir}" ;;
    bluez-le-audio-controller-daemon-profile-flow) case_bluez_le_audio_controller_daemon_profile_flow "${out_dir}" ;;
    bluez-le-audio-controller-daemon-profile-reconnect) case_bluez_le_audio_controller_daemon_profile_reconnect "${out_dir}" ;;
    bluez-le-audio-controller-daemon-error-recovery) case_bluez_le_audio_controller_daemon_error_recovery "${out_dir}" ;;
    bluez-le-audio-controller-bap-ascs-error-matrix) case_bluez_le_audio_controller_bap_ascs_error_matrix "${out_dir}" ;;
    bluez-le-audio-controller-daemon-dbus-ownership) case_bluez_le_audio_controller_daemon_dbus_ownership "${out_dir}" ;;
    bluez-le-audio-bap-ascs-dbus-owner-recovery) case_bluez_le_audio_bap_ascs_dbus_owner_recovery "${out_dir}" ;;
    bluez-le-audio-controller-dbus-bap-ascs-reconnect) case_bluez_le_audio_controller_dbus_bap_ascs_reconnect "${out_dir}" ;;
    bluez-le-audio-controller-daemon-full-stack) case_bluez_le_audio_controller_daemon_full_stack "${out_dir}" ;;
    bluez-le-audio-controller-daemon-full-stack-reconnect) case_bluez_le_audio_controller_daemon_full_stack_reconnect "${out_dir}" ;;
    bluez-le-audio-controller-daemon-mainloop-cleanup) case_bluez_le_audio_controller_daemon_mainloop_cleanup "${out_dir}" ;;
    bluez-le-audio-daemon-broadcast-profile-flow) case_bluez_le_audio_daemon_broadcast_profile_flow "${out_dir}" ;;
    bluez-le-audio-controller-daemon-broadcast-profile-flow) case_bluez_le_audio_controller_daemon_broadcast_profile_flow "${out_dir}" ;;
    bluez-le-audio-controller-daemon-broadcast-reconnect) case_bluez_le_audio_controller_daemon_broadcast_reconnect "${out_dir}" ;;
    bluez-le-audio-controller-lc3-bidir) case_bluez_le_audio_controller_lc3_bidir "${out_dir}" ;;
    bluez-le-audio-coordinated-services) case_bluez_le_audio_coordinated_services "${out_dir}" ;;
    bluez-le-audio-cap-csip-group) case_bluez_le_audio_cap_csip_group "${out_dir}" ;;
    bluez-le-audio-tmap-mcp-ccp-flow) case_bluez_le_audio_tmap_mcp_ccp_flow "${out_dir}" ;;
    bluez-le-audio-broadcast-multibis) case_bluez_le_audio_broadcast_multibis "${out_dir}" ;;
    bluez-le-audio-broadcast-multibis-reconnect) case_bluez_le_audio_broadcast_multibis_reconnect "${out_dir}" ;;
    bluez-le-audio-bass-scan-delegator) case_bluez_le_audio_bass_scan_delegator "${out_dir}" ;;
    bluez-le-audio-daemon-integrated-profile) case_bluez_le_audio_daemon_integrated_profile "${out_dir}" ;;
    bluez-le-audio-daemon-integrated-profile-reconnect) case_bluez_le_audio_daemon_integrated_profile_reconnect "${out_dir}" ;;
    bluez-le-audio-bap-pacs-ascs-session) case_bluez_le_audio_bap_pacs_ascs_session "${out_dir}" ;;
    bluez-le-audio-bap-pacs-ascs-reconnect-recovery) case_bluez_le_audio_bap_pacs_ascs_reconnect_recovery "${out_dir}" ;;
    bluez-le-audio-bap-pacs-ascs-metadata-reconfig) case_bluez_le_audio_bap_pacs_ascs_metadata_reconfig "${out_dir}" ;;
    bluez-le-audio-codec-qos-policy-matrix) case_bluez_le_audio_codec_qos_policy_matrix "${out_dir}" ;;
    bluez-le-audio-role-soak) case_bluez_le_audio_role_soak "${out_dir}" ;;
    bluez-le-audio-umbrella) case_bluez_le_audio_umbrella "${out_dir}" ;;
    bluez-le-audio-controller-setup) case_bluez_le_audio_controller_setup "${out_dir}" ;;
    bluez-le-audio-controller-reconnect) case_bluez_le_audio_controller_reconnect "${out_dir}" ;;
    bluez-hid-upstream-convergence-closeout) case_bluez_hid_upstream_convergence_closeout "${out_dir}" ;;
    bluez-gatt-upstream-convergence-closeout) case_bluez_gatt_upstream_convergence_closeout "${out_dir}" ;;
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
bluez-ipsp-closeout-full: BlueZ-facing LE IPSP profile adapter drives BLE 6LoWPAN ping/iperf closeout
bluez-daemon-ipsp-closeout-full: bluetoothd-style LE IPSP profile/mainloop/D-Bus closeout over BLE 6LoWPAN
bluez-net-current-complete-closeout: Current BT/BLE NET completion gate for BlueZ Network/BNEP and bluetoothd-style IPSP/6LoWPAN
bluez-net-upstream-convergence-closeout: BT/BLE NET upstream convergence gate requiring BlueZ/Linux source coverage maps and cleanup evidence
bluez-current-functional-closeout: Combined all-current-function closeout for BT/BLE basic, A2DP, LE Audio, BT Network/BNEP, BLE Network/IPSP, and implemented BlueZ profiles across all four sim roles
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
bluez-mgmt-daemon-bootstrap: BlueZ shared/mgmt-style controller bring-up, discovery, pair, disconnect, unpair, and error lifecycle
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
bluez-hci-mgmt-socket-closeout-full: BlueZ mgmt, btmon, HCI ioctl/raw/user/monitor/init/ISO, and ADV/SCAN socket ABI closeout
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
bluez-network-iperf-matrix: BlueZ Network Profile-shaped TCP/UDP forward/reverse BNEP iperf matrix in one lifecycle
bluez-network-frag-ping: BlueZ Network Profile-shaped default-MTU IPv4 fragmentation ping
bluez-network-jumbo-ping: BlueZ Network Profile-shaped BNEP 2000-byte jumbo payload ping
bluez-network-mtu-ping: BlueZ Network Profile-shaped BNEP 1400-byte ICMP payload stress
bluez-network-mtu-soak: BlueZ Network Profile-shaped BNEP 1400-byte ICMP payload soak
bluez-network-mtu-reconnect-stress: BlueZ Network Profile-shaped repeated 1400-byte ICMP BNEP lifecycle
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
bluez-a2dp-sbc-codec-concurrent: BlueZ A2DP SBC media while AVDTP signaling session remains open
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
bluez-daemon-a2dp-dbus-client-full: BlueZ daemon-owned A2DP plus external MediaEndpoint/MediaTransport client lifecycle
bluez-daemon-a2dp-dbus-client-busy: BlueZ daemon-owned A2DP external MediaTransport duplicate Acquire error lifecycle
bluez-daemon-a2dp-full-concurrent: BlueZ daemon-owned A2DP with external endpoint, busy Acquire, concurrent signaling/media, SBC, and teardown
bluez-daemon-a2dp-full-concurrent-reconnect: Two-round BlueZ daemon-owned A2DP full concurrent umbrella lifecycle
bluez-daemon-a2dp-full-concurrent-soak: Three-round BlueZ daemon-owned A2DP full concurrent umbrella lifecycle
bluez-daemon-a2dp-integrated-profile: Single-command BlueZ daemon-owned A2DP/AVRCP integrated profile flow and cleanup
bluez-daemon-a2dp-integrated-reconnect: Two-round single-command BlueZ daemon-owned A2DP/AVRCP integrated reconnect lifecycle
bluez-daemon-a2dp-session-ownership: BlueZ daemon-owned A2DP/AVRCP session fd/ref/watch ownership cleanup lifecycle
bluez-daemon-a2dp-error-policy: BlueZ daemon-owned A2DP AVDTP/L2CAP bad-state, duplicate-open, media-before-start, and abort cleanup policy
bluez-daemon-a2dp-upstream-session: BlueZ upstream-shaped A2DP profile/device/session/media/AVRCP object callback lifecycle
bluez-daemon-a2dp-upstream-reconnect: BlueZ upstream-shaped A2DP persistent profile reconnect lifecycle with per-round cleanup
bluez-daemon-a2dp-upstream-transactions: BlueZ upstream-shaped AVDTP transaction owner lifecycle with timeout, retry, cancel, and cleanup
bluez-daemon-a2dp-media-transport-fd: BlueZ MediaTransport1 D-Bus fd Acquire/TryAcquire/Release owner lifecycle
bluez-daemon-a2dp-codec-policy: BlueZ A2DP SBC codec capability, reject, reconfigure, and cleanup policy lifecycle
bluez-daemon-a2dp-closeout-full: Current A2DP closeout umbrella covering profile, AVDTP, MediaTransport, codec, error, reconnect cleanup
bluez-a2dp-current-complete-closeout: Named current A2DP completion gate backed by audio-a2dp-closeout-full source/sink closeout
bluez-a2dp-upstream-convergence-closeout: A2DP upstream convergence gate requiring explicit BlueZ/Linux source coverage map and cleanup evidence
bluez-daemon-device-policy: BlueZ daemon-shaped add/remove/block/unblock/device-flags/unpair lifecycle
bluez-basic-mgmt-flow: BlueZ-shaped basic LE mgmt/HCI power, scan, connect, security, CIS, disconnect, and error lifecycle
bluez-basic-scan-connect-auth-flow: BlueZ-shaped standalone scan, connect, pairing/auth, reconnect, policy, and error lifecycle
bluez-basic-upstream-convergence-closeout: BT/BLE basic upstream convergence gate for BR/EDR basic link and BLE BlueZ scan/connect/auth lifecycle
bluez-hid-hogp-profile-closeout: Classic HID and BLE HOGP profile closeout gate for SDP/Profile1, HIDP, ATT/GATT, report, notification, and cleanup lifecycle
bluez-hfp-hsp-profile-closeout: Classic HFP/HSP profile closeout gate for SDP/Profile1, RFCOMM, AT command state machine, SCO audio bearer, call/volume, and cleanup lifecycle
bluez-obex-pbap-opp-profile-closeout: Classic OBEX PBAP/OPP closeout gate for SDP, RFCOMM transport, OBEX session, phonebook/vCard transfer, object push, abort/error, and cleanup lifecycle
bluez-obex-map-mns-profile-closeout: Classic OBEX MAP/MNS closeout gate for SDP, RFCOMM transport, OBEX session, message listing/get/status/push, notification events, abort/error, and cleanup lifecycle
bluez-obex-ftp-sync-profile-closeout: Classic OBEX FTP/Sync closeout gate for SDP, RFCOMM transport, folder/file operations, phonebook/calendar/notes sync, abort/error, and cleanup lifecycle
bluez-mesh-profile-closeout: BLE Mesh closeout gate for provisioning, config, model messages, proxy, relay/friend, beacons, replay/error policy, and cleanup lifecycle
bluez-gatt-profile-closeout: Generic BLE GATT application services closeout gate for app registration, GAP/BAS/DIS/SCPP/custom services, read/write/notify/indicate, errors, and cleanup lifecycle
bluez-asha-profile-closeout: BLE ASHA/Hearing Aid closeout gate for GATT discovery/control/status, paired hearing aids, audio payload/control, battery, errors, and cleanup lifecycle
bluez-obex-bip-profile-closeout: Classic OBEX BIP closeout gate for imaging SDP, RFCOMM transport, capabilities, image put/get/thumbnail, abort/error, and cleanup lifecycle
bluez-print-profile-closeout: Classic CUPS/HCRP/SPP printing closeout gate for printer SDP, RFCOMM/HCRP transport, CUPS backend, print job, status/cancel/error, and cleanup lifecycle
bluez-iap-profile-closeout: Classic iAP accessory closeout gate for SDP, RFCOMM transport, identify, external accessory session, control payload, errors, and cleanup lifecycle
bluez-midi-profile-closeout: BLE MIDI closeout gate for MIDI GATT service, timestamped MIDI packets, notify/write, jitter/error policy, and cleanup lifecycle
bluez-ranging-profile-closeout: BLE Ranging/RAP closeout gate for ranging capability, security, procedure config/start/result, events, error policy, and cleanup lifecycle
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
bluez-le-audio-daemon-full-lifecycle: BlueZ daemon-owned LE Audio D-Bus/PACS/ASCS/BAP/MediaTransport plus BIS/CIS lifecycle
bluez-le-audio-dbus-client-full: BlueZ external LE Audio MediaEndpoint D-Bus client ownership plus bidirectional CIS lifecycle
bluez-le-audio-lc3-codec-transport: BlueZ LE Audio LC3 codec capability/QoS metadata plus ISO MediaTransport frame lifecycle
bluez-le-audio-iso-dataplane-soak: BlueZ LE Audio bidirectional two-round CIS ISO data-plane soak with ASCS/BAP/QoS/credit/LC3 lifecycle
bluez-le-audio-controller-iso-dataplane-soak: BlueZ HCI USER CIS controller setup followed by bidirectional two-round LC3 ISO data-plane soak
bluez-le-audio-daemon-profile-flow: BlueZ daemon-owned LE Audio unicast profile flow that internally drives D-Bus/ASCS/BAP/ISO/LC3 lifecycle
bluez-le-audio-controller-daemon-profile-flow: BlueZ HCI USER CIS controller setup followed by daemon-owned LE Audio unicast profile flow
bluez-le-audio-controller-daemon-profile-reconnect: BlueZ HCI USER CIS controller setup followed by two-round daemon-owned LE Audio unicast reconnect lifecycle
bluez-le-audio-controller-daemon-error-recovery: BlueZ HCI USER CIS setup plus LE Audio QoS/EAGAIN/BASS bad-code error recovery lifecycle
bluez-le-audio-controller-bap-ascs-error-matrix: BlueZ HCI USER and mgmt-driven BAP/ASCS QoS cancel, metadata, disconnect, and command-status matrix
bluez-le-audio-controller-daemon-dbus-ownership: BlueZ HCI USER CIS setup plus LE Audio MediaEndpoint/MediaTransport D-Bus owner loss/reacquire lifecycle
bluez-le-audio-bap-ascs-dbus-owner-recovery: BlueZ LE Audio D-Bus owner loss/reacquire followed by BAP/ASCS ASE session and daemon unicast payload
bluez-le-audio-controller-dbus-bap-ascs-reconnect: BlueZ LE Audio controller/mgmt reconnect after D-Bus owner recovery with repeated BAP/ASCS unicast payload
bluez-le-audio-controller-daemon-full-stack: BlueZ HCI USER, mgmt/security, D-Bus ownership, unicast, broadcast, and recovery LE Audio stack lifecycle
bluez-le-audio-controller-daemon-full-stack-reconnect: BlueZ full-stack LE Audio lifecycle followed by second-round CIS/BIS reconnect payload
bluez-le-audio-controller-daemon-mainloop-cleanup: BlueZ LE Audio daemon mainloop/fd cleanup followed by unicast profile restart
bluez-le-audio-daemon-broadcast-profile-flow: BlueZ daemon-owned LE Audio broadcast source/sink profile flow with BIS/BASS/BASE/ISO payload lifecycle
bluez-le-audio-controller-daemon-broadcast-profile-flow: BlueZ HCI USER BIG/BIS controller setup followed by daemon-owned LE Audio broadcast profile flow
bluez-le-audio-controller-daemon-broadcast-reconnect: BlueZ HCI USER BIG/BIS setup followed by two-round daemon-owned LE Audio broadcast reconnect lifecycle
bluez-le-audio-controller-lc3-bidir: BlueZ HCI USER two-CIS setup plus bidirectional LC3 MediaTransport lifecycle
bluez-le-audio-coordinated-services: BlueZ LE Audio VCP/MICP/CSIP/MCP/TMAP/CCP/GMAP service matrix for source and sink roles
bluez-le-audio-cap-csip-group: BlueZ LE Audio CAP coordinator plus CSIP lock/unlock over a bidirectional coordinated ASE group
bluez-le-audio-tmap-mcp-ccp-flow: BlueZ LE Audio TMAP role plus MCP media and CCP call control around unicast MediaTransport
bluez-le-audio-broadcast-multibis: BlueZ LE Audio broadcast BIG with BIS1/BIS2 source, BASS sync, payload, and teardown
bluez-le-audio-broadcast-multibis-reconnect: BlueZ LE Audio broadcast multi-BIS two-round reconnect/restart lifecycle
bluez-le-audio-bass-scan-delegator: BlueZ LE Audio BASS Broadcast Assistant plus Scan Delegator receive-state lifecycle
bluez-le-audio-daemon-integrated-profile: BlueZ LE Audio daemon-owned integrated CAP/CSIP/TMAP/MCP/CCP/BAP/LC3 unicast profile flow
bluez-le-audio-daemon-integrated-profile-reconnect: BlueZ LE Audio daemon-owned integrated profile flow repeated lifecycle
bluez-le-audio-bap-pacs-ascs-session: BlueZ LE Audio BAP/PACS/ASCS ASE session with source/sink role swap and unicast profile payload
bluez-le-audio-bap-pacs-ascs-reconnect-recovery: BlueZ LE Audio BAP/PACS/ASCS repeated ASE lifecycle with QoS reject and retry recovery
bluez-le-audio-bap-pacs-ascs-metadata-reconfig: BlueZ LE Audio BAP/PACS/ASCS metadata/context update followed by release and reconfiguration
bluez-le-audio-codec-qos-policy-matrix: BlueZ LE Audio LC3 codec plus BAP QoS reject/retry and metadata policy matrix
bluez-le-audio-role-soak: BlueZ LE Audio repeated source/sink role soak across codec/QoS, D-Bus ownership, mgmt reconnect, and unicast payload
bluez-le-audio-umbrella: BlueZ daemon/D-Bus/controller/LC3/BIS+CIS LE Audio umbrella lifecycle
bluez-le-audio-controller-setup: BlueZ HCI USER LE Audio ISO controller setup for CIG/CIS and BIG/BIS
bluez-le-audio-controller-reconnect: BlueZ HCI USER LE Audio ISO controller setup repeated lifecycle
bluez-hid-upstream-convergence-closeout: BlueZ HID/HOGP upstream convergence gate for Classic HID control/interrupt and HOGP ATT/GATT reports
bluez-gatt-upstream-convergence-closeout: BLE GATT/ATT upstream convergence gate for ATT bearer, IO, request queue, PACS and ASCS cleanup
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
