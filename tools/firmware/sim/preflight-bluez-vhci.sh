#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
# Check host-side prerequisites for BlueZ btproxy <-> /dev/vhci <-> NuttX H4 TCP smoke.

set -u

BT_ROOT="${BT_ROOT:-/home/uan/Feather-develop-BT}"
BLUEZ_DIR="${BLUEZ_DIR:-$BT_ROOT/third/bluez}"
BTPROXY="${BTPROXY:-$BLUEZ_DIR/tools/btproxy}"
BTVIRT="${BTVIRT:-$BLUEZ_DIR/emulator/btvirt}"
STATUS=0

say() {
  printf '%s\n' "$*"
}

check_file_exec() {
  local name="$1"
  local path="$2"

  if [ -x "$path" ]; then
    say "PASS: $name executable: $path"
  elif [ -e "$path" ]; then
    say "FAIL: $name exists but is not executable: $path"
    STATUS=1
  else
    say "FAIL: $name missing: $path"
    STATUS=1
  fi
}

say "BlueZ/VHCI host preflight"
say "BT_ROOT=$BT_ROOT"
say "BLUEZ_DIR=$BLUEZ_DIR"

check_file_exec "btproxy" "$BTPROXY"
check_file_exec "btvirt" "$BTVIRT"

if [ -e /dev/vhci ]; then
  if [ -r /dev/vhci ] && [ -w /dev/vhci ]; then
    say "PASS: /dev/vhci exists and is readable/writable by current user"
  else
    say "FAIL: /dev/vhci exists but current user lacks read/write access"
    ls -l /dev/vhci 2>/dev/null || true
    STATUS=2
  fi
else
  say "FAIL: /dev/vhci is missing"
  STATUS=2
fi

if command -v lsmod >/dev/null 2>&1; then
  if lsmod | awk '{print $1}' | grep -qx 'vhci_hci'; then
    say "PASS: kernel module vhci_hci is loaded"
  else
    say "INFO: kernel module vhci_hci is not currently loaded"
  fi
else
  say "INFO: lsmod not available; cannot inspect vhci_hci module state"
fi

if command -v modinfo >/dev/null 2>&1; then
  if modinfo vhci_hci >/dev/null 2>&1; then
    say "PASS: kernel module vhci_hci is available to modprobe"
  else
    say "INFO: modinfo vhci_hci failed; module may be unavailable for this host kernel"
  fi
else
  say "INFO: modinfo not available; cannot inspect vhci_hci module availability"
fi

if [ "$STATUS" -eq 2 ]; then
  say ""
  say "Host /dev/vhci is required for BlueZ btproxy connect mode."
  say "Typical host setup, if allowed by the environment:"
  say "  sudo modprobe vhci_hci"
  say "  sudo chmod a+rw /dev/vhci"
  say ""
  say "If /dev/vhci cannot be enabled, use the Python controller smoke or add a BlueZ btdev/emulator direct TCP mode that does not require host VHCI."
fi

exit "$STATUS"
