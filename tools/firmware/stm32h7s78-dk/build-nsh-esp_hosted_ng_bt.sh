#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng_bt.sh
#
# SPDX-License-Identifier: Apache-2.0
#
# Build STM32H7S78-DK NXboot plus an NSH application image configured for
# ESP-Hosted NG over SPI with the ESP32-C5 Bluetooth/BLE HCI path enabled.
#
# The ESP32-C5 companion firmware is an external prerequisite and is flashed
# manually. This script only builds the STM32H7S78-DK NuttX host firmware.
#
# and must be built for SPI host interface plus controller-only HCI over the
# ESP-Hosted NG SPI payload channel.
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32H7S78-DK NXboot and ESP-Hosted NG BT/BLE NSH app firmware.\n\n'
  printf 'Run from anywhere inside the FeatherCore_ESP checkout. Outputs are written to:\n\n'
  printf '  FeatherCore_ESP/build/stm32h7s78-dk-nxboot.bin\n'
  printf '      Raw NuttX NXboot image. Program at internal Flash 0x08000000.\n\n'
  printf '  FeatherCore_ESP/build/stm32h7s78-dk-nsh-esp_hosted_ng_bt.bin\n'
  printf '      Final NSH app image: [NXboot header][NuttX app raw binary].\n'
  printf '      Program at XSPI2 NOR 0x70000000. The app vector table starts at\n'
  printf '      0x70000000 + CONFIG_NXBOOT_HEADER_SIZE, normally 0x70000400.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N          Parallel make jobs (default: 8)\n'
  printf '  -v, --version VERSION App semantic version (default: 0.1.0)\n'
  printf '      --header-size N   NXboot header size (default: app .config or 0x400)\n'
  printf '      --identifier ID   NXboot platform identifier (default: .config or 0x48735378)\n'
  printf '  -h, --help            Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"
esp_hosted_ng_src="${feather_root}/../third/esp-hosted/esp_hosted_ng"

cd "${feather_root}/nuttx"

build_dir="../build"
nximage_tool="../apps/boot/nxboot/tools/nximage.py"

loader_bin="${build_dir}/stm32h7s78-dk-nxboot.bin"
app_image_bin="${build_dir}/stm32h7s78-dk-nsh-esp_hosted_ng_bt.bin"

jobs="${JOBS:-8}"
version="0.1.0"
header_size=""
identifier=""

config_value()
{
  local key="$1"
  local file=".config"

  if [[ -f "${file}" ]]; then
    sed -n "s/^${key}=//p" "${file}" | tail -n 1
  fi
}

file_size()
{
  wc -c < "$1" | tr -d '[:space:]'
}

clean_build_dir()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'stm32h7s78-dk-*' -delete
}

distclean_tree()
{
  if [[ -e Make.defs ]]; then
    make distclean
  else
    rm -f Make.defs .config .config.orig defconfig
  fi
}

configure_board()
{
  ./tools/configure.sh "$1"
  make clean
}

set_config()
{
  local key="$1"
  local value="$2"

  if grep -q -E "^(# )?${key}[= ]" .config; then
    sed -i -E "s|^(# )?${key}([= ].*)?|${key}=${value}|" .config
  else
    printf '%s=%s\n' "${key}" "${value}" >> .config
  fi
}

unset_config()
{
  local key="$1"

  if grep -q -E "^(# )?${key}[= ]" .config; then
    sed -i -E "s|^(# )?${key}([= ].*)?|# ${key} is not set|" .config
  else
    printf '# %s is not set\n' "${key}" >> .config
  fi
}

enable_esp_hosted_ng_bt_config()
{
  printf '==> Enabling ESP-Hosted NG SPI BT/BLE overlay\n'

  if [[ ! -d "${esp_hosted_ng_src}" ]]; then
    echo "WARNING: ESP-Hosted NG source not found: ${esp_hosted_ng_src}" >&2
    echo "WARNING: host image will still build, but the ESP32-C5 firmware must be supplied separately." >&2
  fi

  set_config CONFIG_NET y
  set_config CONFIG_NET_NETLINK y
  set_config CONFIG_NET_PKT y
  set_config CONFIG_NETDEV_IFINDEX y
  set_config CONFIG_NETDEV_LATEINIT y
  set_config CONFIG_NETDEV_STATISTICS y
  set_config CONFIG_NETDEV_WIRELESS_IOCTL y

  set_config CONFIG_ALLOW_BSD_COMPONENTS y

  set_config CONFIG_WIRELESS y
  set_config CONFIG_DRIVERS_WIRELESS y
  set_config CONFIG_DRIVERS_IEEE80211 y
  set_config CONFIG_WIRELESS_IEEE80211_CFG80211_LINUX y
  unset_config CONFIG_WIRELESS_IEEE80211_NL80211_METADATA_ONLY

  set_config CONFIG_WIRELESS_ESP_HOSTED_NG y
  set_config CONFIG_WIRELESS_ESP_HOSTED_NG_SPI y
  set_config CONFIG_WL_ESP_HOSTED_NG y
  set_config CONFIG_WL_ESP_HOSTED_NG_LINUX_CFG80211 y
  set_config CONFIG_WL_ESP_HOSTED_NG_SPI_DEV 4
  set_config CONFIG_WL_ESP_HOSTED_NG_SPI_FREQUENCY 10000000
  set_config CONFIG_WL_ESP_HOSTED_NG_SPI_MODE 2
  set_config CONFIG_WL_ESP_HOSTED_NG_HANDSHAKE_PIN 2
  set_config CONFIG_WL_ESP_HOSTED_NG_DATA_READY_PIN 4
  set_config CONFIG_WL_ESP_HOSTED_NG_RESET_PIN 7
  set_config CONFIG_WL_ESP_HOSTED_NG_IFNAME '"wlan0"'

  set_config CONFIG_SPI y
  set_config CONFIG_SPI_EXCHANGE y
  set_config CONFIG_STM32H7RS_SPI y
  set_config CONFIG_STM32H7RS_SPI4 y

  set_config CONFIG_DRIVERS_BLUETOOTH y
  set_config CONFIG_WIRELESS_BLUETOOTH y
  set_config CONFIG_WIRELESS_BLUETOOTH_HOST y
  set_config CONFIG_NET_BLUETOOTH y
  set_config CONFIG_NET_BLUETOOTH_PREALLOC_CONNS 4
  set_config CONFIG_NET_BLUETOOTH_BACKLOG 8
  set_config CONFIG_BLUETOOTH_MAX_CONN 4
  set_config CONFIG_BLUETOOTH_MAX_PAIRED 8
  set_config CONFIG_BLUETOOTH_MAXSCANDATA 31
  set_config CONFIG_BLUETOOTH_MAXSCANRESULT 32
  set_config CONFIG_BLUETOOTH_BUFFER_PREALLOC 16
  set_config CONFIG_BTSAK y
  set_config CONFIG_BTSAK_PROGNAME '"bt"'
  set_config CONFIG_BLE_ADV_DEMO y
  set_config CONFIG_BLE_ADV_DEMO_PROGNAME '"ble_adv"'
  set_config CONFIG_BLE_ADV_DEMO_IFNAME '"bnep0"'
  set_config CONFIG_BLE_ADV_DEMO_NAME '"Feather-ESP-BLE"'
  set_config CONFIG_BLE_GATT_DEMO y
  set_config CONFIG_BLE_GATT_DEMO_PROGNAME '"ble_gatt"'
  set_config CONFIG_BLE_GATT_DEMO_DEFAULT_VALUE '"Feather GATT ready"'
  set_config CONFIG_BLE_PERIPH_DEMO y
  set_config CONFIG_BLE_PERIPH_DEMO_PROGNAME '"ble_periph"'
  set_config CONFIG_BLE_PERIPH_DEMO_IFNAME '"bnep0"'
  set_config CONFIG_BLE_PERIPH_DEMO_NAME '"Feather-ESP-BLE"'
  set_config CONFIG_BLE_PERIPH_DEMO_DEFAULT_VALUE '"Feather peripheral ready"'

  make olddefconfig
  make clean
}

create_nxboot_image()
{
  local input="$1"
  local output="$2"

  if [[ -z "${header_size}" ]]; then
    header_size="$(config_value CONFIG_NXBOOT_HEADER_SIZE)"
    header_size="${header_size:-0x400}"
  fi

  if [[ -z "${identifier}" ]]; then
    identifier="$(config_value CONFIG_NXBOOT_PLATFORM_IDENTIFIER)"
    identifier="${identifier:-0x48735378}"
  fi

  if [[ ! -f "${nximage_tool}" ]]; then
    echo "ERROR: NXboot image tool not found: ${nximage_tool}" >&2
    exit 1
  fi

  if python3 -c 'import semantic_version' >/dev/null 2>&1; then
    python3 "${nximage_tool}" \
      --version "${version}" \
      --header_size "${header_size}" \
      --identifier "${identifier}" \
      "${input}" \
      "${output}"
  else
    echo "INFO: Python module 'semantic_version' is not installed." >&2
    echo "INFO: using built-in NXboot header fallback." >&2

    python3 - "${input}" "${output}" "${version}" "${header_size}" \
      "${identifier}" <<'PY'
import io
import os
import re
import struct
import sys
import zlib

src_path, dst_path, version, header_size, identifier = sys.argv[1:6]
header_size = int(header_size, 0)
identifier = int(identifier, 0)

if header_size < 128:
    raise SystemExit("ERROR: NXboot header size must be at least 128 bytes")

match = re.fullmatch(r"([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([0-9A-Za-z.-]+))?",
                     version)
if not match:
    raise SystemExit("ERROR: version must be MAJOR.MINOR.PATCH[-prerelease]")

major, minor, patch = (int(match.group(i)) for i in range(1, 4))
prerelease = (match.group(4) or "").encode("utf-8")
if len(prerelease) > 94:
    raise SystemExit("ERROR: NXboot prerelease string is longer than 94 bytes")

size = os.stat(src_path).st_size
crc = 0

with open(src_path, "rb") as src, open(dst_path, "wb") as dst:
    dst.write(b"\x4e\x58\x4f\x53")
    dst.write(struct.pack("<H", 0))
    dst.write(struct.pack("<H", header_size))
    dst.write(struct.pack("<I", 0xffffffff))
    dst.write(struct.pack("<I", size))
    dst.write(struct.pack("<Q", identifier))
    dst.write(struct.pack("<I", 0))
    dst.write(struct.pack("<H", major))
    dst.write(struct.pack("<H", minor))
    dst.write(struct.pack("<H", patch))
    dst.write(struct.pack("@94s", prerelease))
    dst.write(bytearray(b"\xff") * (header_size - 128))

    while True:
        data = src.read(io.DEFAULT_BUFFER_SIZE)
        if not data:
            break
        dst.write(data)

with open(dst_path, "r+b") as image:
    image.seek(12)
    while True:
        data = image.read(io.DEFAULT_BUFFER_SIZE)
        if not data:
            break
        crc = zlib.crc32(data, crc)
    image.seek(8)
    image.write(struct.pack("<I", crc))
PY
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -j[0-9]*)
      jobs="${1#-j}"
      shift
      ;;
    -j|--jobs)
      jobs="$2"
      shift 2
      ;;
    -v|--version)
      version="$2"
      shift 2
      ;;
    --header-size)
      header_size="$2"
      shift 2
      ;;
    --identifier)
      identifier="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERROR: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

printf '==> Cleaning STM32H7S78-DK build outputs\n'
clean_build_dir

printf '==> Building STM32H7S78-DK NXboot\n'
distclean_tree
configure_board stm32h7s78-dk:nxboot
make "-j${jobs}"
cp nuttx.bin "${loader_bin}"

printf '\n==> Building STM32H7S78-DK ESP-Hosted NG BT/BLE NSH NXboot app\n'
distclean_tree
configure_board stm32h7s78-dk:nsh-esp_hosted_ng
enable_esp_hosted_ng_bt_config
make "-j${jobs}"
create_nxboot_image nuttx.bin "${app_image_bin}"

printf '\n==> Firmware outputs\n'
printf '  NXboot raw image:\n'
printf '    file:       %s\n' "${loader_bin}"
printf '    size:       %s bytes\n' "$(file_size "${loader_bin}")"
printf '    structure:  raw NuttX NXboot binary\n'
printf '    program at: internal Flash 0x08000000\n\n'

printf '  ESP-Hosted NG BT/BLE NSH NXboot app image:\n'
printf '    file:       %s\n' "${app_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${app_image_bin}")"
printf '    structure:  [NXboot header %s][NuttX app raw binary]\n' "${header_size}"
printf '    program at: XSPI2 NOR 0x70000000\n'
printf '    app vector: 0x70000000 + %s, normally 0x70000400\n\n' "${header_size}"

printf '  ESP32-C5 companion firmware:\n'
printf '    status:     external prerequisite, manually flashed by user\n'
printf '    transport:  SPI only, ESP-Hosted NG payload HCI interface\n'
