#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32h7s78-dk/build-nsh-lsm6dsox.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32H7S78-DK NXboot and NSH LSM6DSOX firmware.\n\n'
  printf 'Outputs are written to:\n\n'
  printf '  build/stm32h7s78-dk-nxboot.bin\n'
  printf '      Raw NuttX NXboot image. Program at internal Flash 0x08000000.\n\n'
  printf '  build/stm32h7s78-dk-nsh-lsm6dsox.bin\n'
  printf '      Final NSH LSM6DSOX app image: [NXboot header][NuttX app raw binary].\n'
  printf '      Program at XSPI2 NOR 0x70000000. The app vector table starts at\n'
  printf '      0x70000000 + CONFIG_NXBOOT_HEADER_SIZE, normally 0x70000400.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N          Parallel make jobs (default: 8)\n'
  printf '  -v, --version VERSION App semantic version (default: 0.1.0)\n'
  printf '      --header-size N   NXboot header size (default: app .config or 0x400)\n'
  printf '      --identifier ID   NXboot platform identifier (default: .config or 0x48735378)\n'
  printf '  -h, --help            Show this help\n'
}

script_dir="$(dirname "${BASH_SOURCE[0]}")"
feather_root="${script_dir}/../../.."

cd "${feather_root}/nuttx"

build_dir="../build"
nximage_tool="../apps/boot/nxboot/tools/nximage.py"

loader_bin="${build_dir}/stm32h7s78-dk-nxboot.bin"
app_image_bin="${build_dir}/stm32h7s78-dk-nsh-lsm6dsox.bin"

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
    echo "WARNING: Python module 'semantic_version' is not installed." >&2
    echo "WARNING: using built-in NXboot header fallback." >&2

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

printf '\n==> Building STM32H7S78-DK NSH LSM6DSOX NXboot app\n'
distclean_tree
configure_board stm32h7s78-dk:nsh-lsm6dsox
make "-j${jobs}"
create_nxboot_image nuttx.bin "${app_image_bin}"

printf '\n==> Firmware outputs\n'
printf '  NXboot raw image:\n'
printf '    file:       %s\n' "${loader_bin}"
printf '    size:       %s bytes\n' "$(file_size "${loader_bin}")"
printf '    structure:  raw NuttX NXboot binary\n'
printf '    program at: internal Flash 0x08000000\n\n'

printf '  NSH LSM6DSOX NXboot app image:\n'
printf '    file:       %s\n' "${app_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${app_image_bin}")"
printf '    structure:  [NXboot header %s][NuttX app raw binary]\n' "${header_size}"
printf '    program at: XSPI2 NOR 0x70000000\n'
printf '    app vector: 0x70000000 + %s, normally 0x70000400\n\n' "${header_size}"
