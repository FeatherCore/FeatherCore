#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Report readiness for Bluetooth IP networking over the Linux-aligned BT hwsim path.

set -u

BT_ROOT="${BT_ROOT:-/home/uan/Feather-develop-BT}"
FC_ROOT="${FC_ROOT:-$BT_ROOT/FeatherCore}"
LINUX_BT="$BT_ROOT/third/linux-hwe-6.17-6.17.0/net/bluetooth"
BLUEZ="$BT_ROOT/third/bluez"
STATUS=0

say() { printf '%s\n' "$*"; }
pass() { say "PASS: $*"; }
info() { say "INFO: $*"; }
fail() { say "FAIL: $*"; STATUS=1; }
missing() { say "MISSING: $*"; STATUS=1; }

require_file() {
  local label="$1"
  local path="$2"
  if [ -e "$path" ]; then
    pass "$label: $path"
  else
    missing "$label: $path"
  fi
}

require_exec() {
  local label="$1"
  local path="$2"
  if [ -x "$path" ]; then
    pass "$label executable: $path"
  elif [ -e "$path" ]; then
    fail "$label exists but is not executable: $path"
  else
    missing "$label executable: $path"
  fi
}

symbol_in_tree() {
  local pattern="$1"
  shift
  if rg -q -g '*.[ch]' "$pattern" "$@" 2>/dev/null; then
    return 0
  fi
  return 1
}

config_has() {
  local cfg="$1"
  local symbol="$2"
  grep -qx "${symbol}=y" "$cfg"
}

say "Bluetooth networking readiness preflight"
say "FC_ROOT=$FC_ROOT"
say "LINUX_BT=$LINUX_BT"
say "BLUEZ=$BLUEZ"
say ""

say "Linux kernel source candidates"
require_file "Linux BNEP core" "$LINUX_BT/bnep/core.c"
require_file "Linux BNEP socket" "$LINUX_BT/bnep/sock.c"
require_file "Linux BNEP netdev" "$LINUX_BT/bnep/netdev.c"
require_file "Linux Bluetooth 6LoWPAN" "$LINUX_BT/6lowpan.c"
require_file "Linux L2CAP core" "$LINUX_BT/l2cap_core.c"
require_file "Linux L2CAP socket" "$LINUX_BT/l2cap_sock.c"
say ""

say "BlueZ userspace/profile candidates"
require_file "BlueZ network profile bnep.c" "$BLUEZ/profiles/network/bnep.c"
require_file "BlueZ network profile connection.c" "$BLUEZ/profiles/network/connection.c"
require_file "BlueZ network profile server.c" "$BLUEZ/profiles/network/server.c"
require_exec "BlueZ bneptest" "$BLUEZ/tools/bneptest"
require_exec "BlueZ bnep-tester" "$BLUEZ/tools/bnep-tester"
require_exec "BlueZ 6lowpan-tester" "$BLUEZ/tools/6lowpan-tester"
say ""

say "FeatherCore current Linux-BT networking port status"
if [ -e "$FC_ROOT/nuttx/wireless/linux_bluetooth/upstream/net_bluetooth/bnep/core.c" ] &&
   [ -e "$FC_ROOT/nuttx/wireless/linux_bluetooth/upstream/net_bluetooth/bnep/sock.c" ] &&
   [ -e "$FC_ROOT/nuttx/wireless/linux_bluetooth/upstream/net_bluetooth/bnep/netdev.c" ]; then
  pass "Linux Bluetooth BNEP source is imported into FeatherCore upstream tree"
else
  missing "Linux Bluetooth BNEP core/sock/netdev source is not yet imported into FeatherCore"
fi

if rg -q "NET_LINUX_BLUETOOTH_UPSTREAM_BNEP" "$FC_ROOT/nuttx/wireless/linux_bluetooth/Kconfig" "$FC_ROOT/nuttx/wireless/linux_bluetooth/Make.defs" 2>/dev/null; then
  pass "Linux Bluetooth BNEP Kconfig/Make.defs staging switch is wired"
else
  missing "Linux Bluetooth BNEP Kconfig/Make.defs staging switch is missing"
fi

if [ -e "$FC_ROOT/nuttx/wireless/linux_bluetooth/upstream/net_bluetooth/6lowpan.c" ]; then
  pass "Linux Bluetooth 6LoWPAN source is imported into FeatherCore upstream tree"
else
  missing "Linux Bluetooth 6LoWPAN source is not yet imported into FeatherCore"
fi

if rg -q "NET_LINUX_BLUETOOTH_UPSTREAM_6LOWPAN" "$FC_ROOT/nuttx/wireless/linux_bluetooth/Kconfig" "$FC_ROOT/nuttx/wireless/linux_bluetooth/Make.defs" 2>/dev/null; then
  pass "Linux Bluetooth 6LoWPAN Kconfig/Make.defs staging switch is wired"
else
  missing "Linux Bluetooth 6LoWPAN Kconfig/Make.defs staging switch is missing"
fi

if [ -e "$FC_ROOT/nuttx/drivers/bluetooth/linux_bt_6lowpan.c" ]; then
  info "NuttX-facing Bluetooth 6LoWPAN bridge exists"
else
  missing "NuttX-facing Bluetooth 6LoWPAN netdev/CoC bridge is not yet implemented"
fi

if symbol_in_tree "BTPROTO_BNEP" "$FC_ROOT/nuttx/wireless/linux_bluetooth"; then
  info "BTPROTO_BNEP constants exist, but protocol implementation is still required"
fi

if symbol_in_tree "bnep_sock_init|linux_bt_upstream_bnep_ioctl_probe|BNEPGETSUPPFEAT" \
   "$FC_ROOT/nuttx/wireless/linux_bluetooth"; then
  pass "BNEP socket/ioctl staging boundary exists"
else
  missing "BNEP socket/ioctl staging boundary is missing"
fi

if symbol_in_tree "linux_bt_bnep_netdev|netdev_register|struct net_driver_s" \
   "$FC_ROOT/nuttx/wireless/linux_bluetooth" "$FC_ROOT/nuttx/drivers/bluetooth"; then
  pass "Linux-BT area has some NuttX netdev integration hooks"
else
  missing "No NuttX netdev binding found in Linux-BT driver/wireless area"
fi

BT_APPS="$FC_ROOT/apps/wireless/linux_bluetooth"
if find "$BT_APPS" -maxdepth 2 -type f \
   \( -name '*bnep*' -o -name '*network*' -o -name '*6lowpan*' \) \
   ! -name 'README.md' ! -name '*.o' ! -name '.depend' ! -name '.built' | grep -q .; then
  pass "BlueZ networking app/source files exist under FeatherCore Linux-BT apps"
else
  missing "BlueZ networking userspace is not wired into FeatherCore apps"
fi
say ""

say "Four-role defconfig IP test readiness"
for role in hwsim_bt1 hwsim_bt2 hwsim_ble1 hwsim_ble2; do
  cfg="$FC_ROOT/nuttx/boards/sim/sim/sim/configs/$role/defconfig"
  if [ ! -e "$cfg" ]; then
    missing "$role defconfig: $cfg"
    continue
  fi

  say "[$role]"
  for sym in CONFIG_NET CONFIG_NET_IPv4 CONFIG_NET_TCP CONFIG_NET_UDP CONFIG_NET_ICMP_SOCKET CONFIG_NETUTILS_IPERF CONFIG_NET_TUN CONFIG_SIM_NETDEV; do
    if config_has "$cfg" "$sym"; then
      pass "$role $sym"
    else
      info "$role lacks $sym"
    fi
  done

  if config_has "$cfg" CONFIG_SYSTEM_PING ||
     config_has "$cfg" CONFIG_NETUTILS_PING; then
    pass "$role ping command support"
  else
    info "$role lacks CONFIG_SYSTEM_PING/CONFIG_NETUTILS_PING"
  fi

  case "$role" in
    hwsim_ble1|hwsim_ble2)
      for sym in CONFIG_NET_IPv6 CONFIG_NET_ICMPv6 CONFIG_NET_ICMPv6_SOCKET; do
        if config_has "$cfg" "$sym"; then
          pass "$role $sym"
        else
          missing "$role lacks BLE IP prerequisite $sym"
        fi
      done

      if config_has "$cfg" CONFIG_SYSTEM_PING6 ||
         config_has "$cfg" CONFIG_NETUTILS_PING6; then
        pass "$role ping6 command support"
      else
        missing "$role lacks CONFIG_SYSTEM_PING6/CONFIG_NETUTILS_PING6"
      fi
      ;;
  esac

done
say ""

say "Interpretation"
say "- BR/EDR ping/iperf path needs Linux BNEP socket/session enablement, NuttX netdev TX/RX data handoff, and BlueZ network profile/apps wiring."
say "- BLE IP path needs Linux Bluetooth 6LoWPAN or LE L2CAP CoC network binding plus IPv6/ICMPv6/iperf-style test config."
say "- Current H4/VHCI + BlueZ btvirt direct smoke is a controller boundary prerequisite, not yet an IP networking implementation."

exit "$STATUS"
