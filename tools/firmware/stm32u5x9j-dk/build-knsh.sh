#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32u5x9j-dk/build-knsh.sh
#
# SPDX-License-Identifier: Apache-2.0
#
# Licensed to the Apache Software Foundation (ASF) under one or more
# contributor license agreements.  See the NOTICE file distributed with
# this work for additional information regarding copyright ownership.  The
# ASF licenses this file to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance with the
# License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
# WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  See the
# License for the specific language governing permissions and limitations
# under the License.
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32U5x9J-DK protected KNSh firmware for internal Flash.\n\n'
  printf 'Run from anywhere inside the Feather checkout. Outputs are written to:\n\n'
  printf '  build/stm32u5x9j-dk-knsh.bin\n'
  printf '      Final protected KNSh image: [kernel blob][padding to user-space]\n'
  printf '      [user blob]. Program at internal Flash 0x08000000.\n\n'
  printf '  build/stm32u5x9j-dk-knsh-kernel.bin\n'
  printf '      Kernel-only raw binary, starting at internal Flash 0x08000000.\n\n'
  printf '  build/stm32u5x9j-dk-knsh-user.bin\n'
  printf '      User-space raw binary, starting at CONFIG_NUTTX_USERSPACE.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N          Parallel make jobs (default: 8)\n'
  printf '  -h, --help            Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
image_prefix="${build_dir}/stm32u5x9j-dk-knsh"

jobs="${JOBS:-8}"
flash_start="0x08000000"
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
  find "${build_dir}" -maxdepth 1 -type f -name 'stm32u5x9j-dk-*' -delete
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

copy_output()
{
  local input="$1"
  local output="$2"

  if [[ -f "${input}" ]]; then
    cp -f "${input}" "${output}"
  fi
}

create_protected_image()
{
  local kernel_input="$1"
  local user_input="$2"
  local output="$3"
  local userspace="$4"

  python3 - "${kernel_input}" "${user_input}" "${output}" \
    "${flash_start}" "${userspace}" <<'PY'
import io
import os
import sys

kernel_path, user_path, output_path, flash_start, userspace = sys.argv[1:6]
flash_start = int(flash_start, 0)
userspace = int(userspace, 0)
kernel_window = userspace - flash_start

if kernel_window <= 0:
    raise SystemExit("ERROR: CONFIG_NUTTX_USERSPACE must be above Flash start")

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

validate_vector()
{
  local image="$1"
  local userspace="$2"

  python3 - "${image}" "${userspace}" <<'PY'
import struct
import sys

path, userspace = sys.argv[1:3]
userspace = int(userspace, 0)

with open(path, "rb") as image:
    data = image.read(8)

if len(data) < 8:
    raise SystemExit(f"ERROR: image too small: {path}")

msp, reset = struct.unpack("<II", data)
reset_addr = reset & ~1

if not (0x20000000 <= msp < 0x20270000):
    raise SystemExit(f"ERROR: MSP outside STM32U5x9J internal SRAM: 0x{msp:08x}")

if not (0x08000000 <= reset_addr < userspace):
    raise SystemExit(
        f"ERROR: reset vector outside protected kernel window: 0x{reset:08x}"
    )

print(f"Vector: msp=0x{msp:08x} reset=0x{reset:08x}")
PY
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -j|--jobs)
      jobs="$2"
      shift 2
      ;;
    -j*)
      jobs="${1#-j}"
      shift
      ;;
    --jobs=*)
      jobs="${1#--jobs=}"
      shift
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

printf '==> Cleaning STM32U5x9J-DK build outputs\n'
clean_build_dir

printf '==> Building STM32U5x9J-DK protected KNSh for internal Flash\n'
distclean_tree
configure_board stm32u5x9j-dk:knsh
make "-j${jobs}"

if [[ ! -f nuttx.bin ]]; then
  echo "ERROR: kernel nuttx.bin was not produced" >&2
  exit 1
fi

if [[ ! -f nuttx_user.bin ]]; then
  echo "ERROR: user nuttx_user.bin was not produced" >&2
  exit 1
fi

userspace="$(config_value CONFIG_NUTTX_USERSPACE)"
userspace="${userspace:-0x08080000}"

copy_output nuttx "${image_prefix}-kernel.elf"
copy_output nuttx.bin "${image_prefix}-kernel.bin"
copy_output nuttx.hex "${image_prefix}-kernel.hex"
copy_output nuttx_user.elf "${image_prefix}-user.elf"
copy_output nuttx_user.bin "${image_prefix}-user.bin"
copy_output nuttx_user.hex "${image_prefix}-user.hex"
copy_output System.map "${image_prefix}-kernel.map"
copy_output User.map "${image_prefix}-user.map"

create_protected_image nuttx.bin nuttx_user.bin "${image_prefix}.bin" \
  "${userspace}"
validate_vector "${image_prefix}.bin" "${userspace}"

kernel_window_size="$((userspace - flash_start))"

printf '\n==> Firmware outputs\n'
printf '  KNSh protected internal-Flash image:\n'
printf '    bin:        %s\n' "${image_prefix}.bin"
printf '    bin size:   %s bytes\n' "$(file_size "${image_prefix}.bin")"
printf '    structure:  [kernel blob][0xff padding to %d bytes][user blob]\n' \
  "${kernel_window_size}"
printf '    program at: internal Flash %s\n' "${flash_start}"
printf '    userspace:  %s\n' "${userspace}"
printf '    nxboot:     not used\n\n'

printf '  Components:\n'
printf '    kernel bin: %s (%s bytes)\n' "${image_prefix}-kernel.bin" \
  "$(file_size "${image_prefix}-kernel.bin")"
printf '    user bin:   %s (%s bytes)\n' "${image_prefix}-user.bin" \
  "$(file_size "${image_prefix}-user.bin")"
if [[ -f "${image_prefix}-kernel.elf" ]]; then
  printf '    kernel elf: %s\n' "${image_prefix}-kernel.elf"
fi
if [[ -f "${image_prefix}-user.elf" ]]; then
  printf '    user elf:   %s\n' "${image_prefix}-user.elf"
fi
printf '    kernel heap: internal SRAM 0x20000000..0x20270000 after kernel bss/idle stack\n'
printf '    user heap:   HSPI1 PSRAM after user bss, within 0xa0000000..0xa4000000\n\n'
