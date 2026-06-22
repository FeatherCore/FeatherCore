#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Validate a captured STM32H7S78-DK NSH console log for the ESP-Hosted NG
# BT/BLE board-level smoke flow.
#
# This script intentionally consumes evidence produced by real hardware. It
# does not start advertising, control a phone, or synthesize connection events.

set -euo pipefail

usage()
{
  cat <<'EOF'
Usage:
  validate-nsh-esp_hosted_ng_bt-board-log.sh <board-console.log>

The log should capture this NSH/phone flow:

  nsh> bt bnep0 info
  nsh> ble_periph check
  nsh> ble_periph start
  phone/nRF Connect: scan Feather-ESP-BLE, connect, discover service
  phone/nRF Connect: read characteristic
  phone/nRF Connect: write characteristic
  nsh> ble_periph status

Required NSH evidence:
  ble_periph: host path ok ...
  ble_periph: buffers ...
  ble_periph: advertising started ...
  ble_periph: read handle=0x0202 ...
  ble_periph: write handle=0x0202 ...

The GATT read/write lines are treated as the observable proof that a central
connected and delivered ATT/GATT traffic to the STM32H7S host.
EOF
}

if [[ $# -ne 1 ]]; then
  usage >&2
  exit 2
fi

log_file="$1"
failures=0

pass()
{
  printf 'PASS: %s\n' "$*"
}

fail()
{
  printf 'FAIL: %s\n' "$*" >&2
  failures=$((failures + 1))
}

require_log()
{
  local pattern="$1"
  local label="$2"

  if grep -Eq "${pattern}" "${log_file}"; then
    pass "${label}"
  else
    fail "${label}"
  fi
}

if [[ ! -s "${log_file}" ]]; then
  printf 'FAIL: missing or empty log: %s\n' "${log_file}" >&2
  exit 1
fi

printf '==> STM32H7S78-DK ESP-Hosted NG BT/BLE board log validation\n'
printf 'log: %s\n' "${log_file}"

require_log 'ble_periph: host path ok ifname=' \
  'Bluetooth socket/ioctl host path reached a controller'
require_log 'ble_periph: buffers cmd=.*acl=.*mtu_acl=' \
  'Controller buffer/MTU information was readable'
require_log 'ble_periph: advertising started on .*Feather-ESP-BLE' \
  'Connectable BLE advertising was started with the expected name'
require_log 'ble_periph: GATT service=6df18a2c-7d8d-4b5d-9f9e-465354484552' \
  'Combined BLE peripheral GATT service was registered/reported'
require_log 'ble_periph: read handle=0x0202 .*returned=.*count=' \
  'Central-triggered GATT read reached NSH-visible host code'
require_log 'ble_periph: write handle=0x0202 .*count=.*data=' \
  'Central-triggered GATT write reached NSH-visible host code'

if grep -Eq 'ioctl\(SIOCGBTINFO\) failed|ioctl\(SIOCBTADVSTART\) failed|socket failed' \
     "${log_file}"; then
  fail 'No Bluetooth socket/ioctl startup failure is present'
else
  pass 'No Bluetooth socket/ioctl startup failure is present'
fi

printf '\n'
if ((failures == 0)); then
  printf 'Board log validation passed.\n'
  printf 'This proves the captured run reached advertising plus GATT read/write over the real board path.\n'
  exit 0
fi

printf 'Board log validation failed: %d issue(s).\n' "${failures}" >&2
printf 'Capture a full NSH session that includes ble_periph check/start/status and phone read/write.\n' >&2
exit 1
