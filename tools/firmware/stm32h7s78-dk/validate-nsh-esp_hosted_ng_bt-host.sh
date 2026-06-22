#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Host-side validation for the STM32H7S78-DK ESP-Hosted NG BT/BLE image.
# This script does not prove real RF behavior. It verifies that the host build
# artifacts, Kconfig selections, NSH demo commands, and HCI lower-half glue are
# present before moving to board-level BLE scan/connect/GATT validation.

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../../.." && pwd)"
nuttx_dir="${repo_root}/nuttx"
apps_dir="${repo_root}/apps"
build_dir="${repo_root}/build"
config_file="${nuttx_dir}/.config"

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

require_file()
{
  local path="$1"

  if [[ -s "${path}" ]]; then
    pass "file exists: ${path#${repo_root}/}"
  else
    fail "missing or empty file: ${path}"
  fi
}

require_text()
{
  local path="$1"
  local pattern="$2"
  local label="$3"

  if [[ -f "${path}" ]] && grep -Eq "${pattern}" "${path}"; then
    pass "${label}"
  else
    fail "${label}"
  fi
}

require_config()
{
  local symbol="$1"
  local value="$2"

  require_text "${config_file}" "^${symbol}=${value}$" \
    "config ${symbol}=${value}"
}

printf '==> STM32H7S78-DK ESP-Hosted NG BT/BLE host validation\n'
printf 'repo: %s\n' "${repo_root}"

require_file "${build_dir}/stm32h7s78-dk-nxboot.bin"
require_file "${build_dir}/stm32h7s78-dk-nsh-esp_hosted_ng_bt.bin"
require_file "${config_file}"

if [[ -f "${config_file}" ]]; then
  require_config CONFIG_STM32H7RS_SPI4 y
  require_config CONFIG_WIRELESS_ESP_HOSTED_NG y
  require_config CONFIG_WIRELESS_ESP_HOSTED_NG_SPI y
  require_config CONFIG_WL_ESP_HOSTED_NG y
  require_config CONFIG_WL_ESP_HOSTED_NG_SPI_DEV 4
  require_config CONFIG_WL_ESP_HOSTED_NG_SPI_FREQUENCY 10000000
  require_config CONFIG_WL_ESP_HOSTED_NG_SPI_MODE 2
  require_config CONFIG_WL_ESP_HOSTED_NG_HANDSHAKE_PIN 2
  require_config CONFIG_WL_ESP_HOSTED_NG_DATA_READY_PIN 4
  require_config CONFIG_WL_ESP_HOSTED_NG_RESET_PIN 7
  require_config CONFIG_DRIVERS_BLUETOOTH y
  require_config CONFIG_WIRELESS_BLUETOOTH y
  require_config CONFIG_WIRELESS_BLUETOOTH_HOST y
  require_config CONFIG_NET_BLUETOOTH y
  require_config CONFIG_BTSAK y
  require_config CONFIG_BLE_ADV_DEMO y
  require_config CONFIG_BLE_GATT_DEMO y
  require_config CONFIG_BLE_PERIPH_DEMO y
fi

require_file "${apps_dir}/wireless/bluetooth/ble_adv/ble_adv_main.c"
require_file "${apps_dir}/wireless/bluetooth/ble_gatt/ble_gatt_main.c"
require_file "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c"
require_file "${nuttx_dir}/drivers/wireless/esp_hosted_ng/esp_hosted_ng_bt_stub.c"
require_file "${repo_root}/tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-board-log.sh"

require_text "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c" \
  'SIOCGBTINFO' \
  'ble_periph check uses Bluetooth controller info ioctl'
require_text "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c" \
  'SIOCBTADVSTART' \
  'ble_periph start uses advertising ioctl'
require_text "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c" \
  'bt_gatt_register' \
  'ble_periph registers a GATT service'
require_text "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c" \
  'BT_GATT_PERM_READ[[:space:]]*\|[[:space:]]*BT_GATT_PERM_WRITE' \
  'ble_periph exposes readable and writable characteristic'
require_text "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c" \
  'ble_periph: read handle=' \
  'ble_periph prints NSH-visible GATT read evidence'
require_text "${apps_dir}/wireless/bluetooth/ble_periph/ble_periph_main.c" \
  'ble_periph: write handle=.*count=' \
  'ble_periph prints NSH-visible GATT write evidence'
require_text "${repo_root}/tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-board-log.sh" \
  'ble_periph: read handle=0x0202' \
  'board-log validator requires GATT read evidence'
require_text "${repo_root}/tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-board-log.sh" \
  'ble_periph: write handle=0x0202' \
  'board-log validator requires GATT write evidence'

require_text "${nuttx_dir}/drivers/wireless/esp_hosted_ng/esp_hosted_ng_bt_stub.c" \
  'bt_driver_register' \
  'ESP-Hosted NG HCI lower-half registers with NuttX Bluetooth host'
require_text "${nuttx_dir}/drivers/wireless/esp_hosted_ng/esp_hosted_ng_bt_stub.c" \
  'ESP_HOSTED_NG_HCI_IF' \
  'ESP-Hosted NG HCI packets use HCI interface type'
require_text "${nuttx_dir}/drivers/wireless/esp_hosted_ng/esp_hosted_ng_bt_stub.c" \
  'HCI_ISODATA_PKT|BT_ISO' \
  'ESP-Hosted NG HCI lower-half preserves ISO packet path'

printf '\n'
if ((failures == 0)); then
  printf 'Host-side validation passed.\n'
  printf 'Next required proof is board-level: ble_periph check/start, phone scan/connect, GATT read/write.\n'
  exit 0
fi

printf 'Host-side validation failed: %d issue(s).\n' "${failures}" >&2
printf 'Run build-nsh-esp_hosted_ng_bt.sh first if artifacts or .config are missing.\n' >&2
exit 1
