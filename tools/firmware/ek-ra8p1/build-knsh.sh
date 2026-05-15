#!/usr/bin/env bash
############################################################################
# tools/firmware/ek-ra8p1/build-knsh.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build EK-RA8P1 NXboot and protected KNSh firmware.\n\n'
  printf 'Outputs:\n'
  printf '  build/ek-ra8p1-nxboot.bin  raw NuttX NXboot, program at MRAM 0x02000000\n'
  printf '  build/ek-ra8p1-knsh.bin    [NXboot header][kernel][padding][user], program at OSPI0 CS1 0x90000000\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N          Parallel make jobs (default: 8)\n'
  printf '  -v, --version VERSION App semantic version (default: 0.1.0)\n'
  printf '      --header-size N   NXboot header size (default: app .config or 0x400)\n'
  printf '      --identifier ID   NXboot platform identifier (default: .config or 0x52413850)\n'
  printf '  -h, --help            Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
nximage_tool="../apps/boot/nxboot/tools/nximage.py"

loader_bin="${build_dir}/ek-ra8p1-nxboot.bin"
app_image_bin="${build_dir}/ek-ra8p1-knsh.bin"

jobs="${JOBS:-8}"
version="0.1.0"
header_size=""
identifier=""
userspace=""
tmp_files=()

cleanup()
{
  local tmp

  for tmp in "${tmp_files[@]}"; do
    rm -f "${tmp}"
  done
}

trap cleanup EXIT

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
  find "${build_dir}" -maxdepth 1 -type f -name 'ek-ra8p1-*' -delete
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

load_image_config()
{
  if [[ -z "${header_size}" ]]; then
    header_size="$(config_value CONFIG_NXBOOT_HEADER_SIZE)"
    header_size="${header_size:-0x400}"
  fi

  if [[ -z "${identifier}" ]]; then
    identifier="$(config_value CONFIG_NXBOOT_PLATFORM_IDENTIFIER)"
    identifier="${identifier:-0x52413850}"
  fi

  if [[ -z "${userspace}" ]]; then
    userspace="$(config_value CONFIG_NUTTX_USERSPACE)"
    userspace="${userspace:-0x90080400}"
  fi
}

create_protected_payload()
{
  local kernel_input="$1"
  local user_input="$2"
  local output="$3"

  load_image_config

  python3 - "${kernel_input}" "${user_input}" "${output}" \
    "${header_size}" "${userspace}" <<'PY'
import io
import os
import sys

kernel_path, user_path, output_path, header_size, userspace = sys.argv[1:6]
slot_base = 0x90000000
header_size = int(header_size, 0)
userspace = int(userspace, 0)
kernel_base = slot_base + header_size
kernel_window = userspace - kernel_base

if kernel_window <= 0:
    raise SystemExit("ERROR: CONFIG_NUTTX_USERSPACE must be above the kernel vector address")

kernel_size = os.stat(kernel_path).st_size
if kernel_size > kernel_window:
    raise SystemExit(
        f"ERROR: kernel binary is {kernel_size} bytes but the protected "
        f"kernel window is only {kernel_window} bytes"
    )

with open(output_path, "wb") as output:
    with open(kernel_path, "rb") as kernel:
        while True:
            data = kernel.read(io.DEFAULT_BUFFER_SIZE)
            if not data:
                break
            output.write(data)

    output.write(b"\xff" * (kernel_window - kernel_size))

    with open(user_path, "rb") as user:
        while True:
            data = user.read(io.DEFAULT_BUFFER_SIZE)
            if not data:
                break
            output.write(data)
PY
}

create_nxboot_image()
{
  local input="$1"
  local output="$2"

  load_image_config

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

printf '==> Cleaning EK-RA8P1 build outputs\n'
clean_build_dir

printf '==> Building EK-RA8P1 NXboot\n'
distclean_tree
configure_board ek-ra8p1:nxboot
make "-j${jobs}"
cp nuttx.bin "${loader_bin}"

printf '\n==> Building EK-RA8P1 protected KNSh app\n'
distclean_tree
configure_board ek-ra8p1:knsh
make "-j${jobs}"

tmp_payload="$(mktemp)"
tmp_files+=("${tmp_payload}")
create_protected_payload nuttx.bin nuttx_user.bin "${tmp_payload}"
create_nxboot_image "${tmp_payload}" "${app_image_bin}"

printf '\n==> Firmware outputs\n'
printf '  NXboot raw image:\n'
printf '    file:       %s\n' "${loader_bin}"
printf '    size:       %s bytes\n' "$(file_size "${loader_bin}")"
printf '    program at: internal MRAM 0x02000000\n\n'

printf '  KNSh NXboot app image:\n'
printf '    file:       %s\n' "${app_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${app_image_bin}")"
printf '    structure:  [NXboot header %s][kernel][padding to %s][user]\n' "${header_size}" "${userspace}"
printf '    identifier: %s\n' "${identifier}"
printf '    program at: OSPI0 CS1 NOR 0x90000000\n'
printf '    kernel vector: 0x90000000 + %s, normally 0x90000400\n\n' "${header_size}"
