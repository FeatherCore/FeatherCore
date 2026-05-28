#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32u5x9j-dk/build-gpu2d-debug.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32U5x9J-DK non-LVGL GPU2D validation firmware.\n\n'
  printf 'The firmware enables the LCD framebuffer and low-level GPU2D debug\n'
  printf 'support, then includes the nemap_demo NSH command for hardware bring-up.\n\n'
  printf 'Outputs are written to:\n\n'
  printf '  build/stm32u5x9j-dk-gpu2d-debug.bin\n'
  printf '      Raw NuttX NSH GPU2D validation binary. Program at internal Flash 0x08000000.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N          Parallel make jobs (default: 8)\n'
  printf '  -h, --help            Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

build_dir="${feather_root}/build"
image_prefix="${build_dir}/stm32u5x9j-dk-gpu2d-debug"
jobs="${JOBS:-8}"

file_size()
{
  wc -c < "$1" | tr -d '[:space:]'
}

clean_debug_outputs()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f \
    -name 'stm32u5x9j-dk-gpu2d-debug*' -delete
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

verify_gpu2d_debug_config()
{
  local missing=0
  local name

  for name in \
    CONFIG_STM32U5_GPU2D \
    CONFIG_STM32U5X9J_DK_LCD \
    CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM \
    CONFIG_DRIVERS_VIDEO \
    CONFIG_VIDEO_FB \
    CONFIG_FB_UPDATE \
    CONFIG_FB_SYNC \
    CONFIG_EXAMPLES_NEMAP_DEMO \
    CONFIG_EXAMPLES_NEMAP_DEMO_STM32U5 \
    CONFIG_DEBUG_FEATURES \
    CONFIG_DEBUG_ASSERTIONS \
    CONFIG_DEBUG_SYMBOLS \
    CONFIG_DEBUG_NOOPT
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled for GPU2D debug build\n' \
        "${name}" >&2
      missing=1
    fi
  done

  for name in \
    CONFIG_GRAPHICS_LVGL \
    CONFIG_EXAMPLES_LVGLDEMO
  do
    if grep -q "^${name}=y" .config; then
      printf 'ERROR: %s must remain disabled for GPU2D validation build\n' \
        "${name}" >&2
      missing=1
    fi
  done

  if [[ "${missing}" -ne 0 ]]; then
    exit 1
  fi
}

validate_vector()
{
  local image="$1"

  python3 - "${image}" <<'PY'
import struct
import sys

path = sys.argv[1]
with open(path, "rb") as image:
    data = image.read(8)

if len(data) < 8:
    raise SystemExit(f"ERROR: image too small: {path}")

msp, reset = struct.unpack("<II", data)
reset_addr = reset & ~1

if not (0x20000000 <= msp < 0x20270000):
    raise SystemExit(f"ERROR: MSP outside STM32U5x9J internal SRAM: 0x{msp:08x}")

if not (0x08000000 <= reset_addr < 0x08400000):
    raise SystemExit(
        f"ERROR: reset vector outside STM32U5x9J internal Flash: 0x{reset:08x}"
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

clean_debug_outputs

cd "${feather_root}/nuttx"

printf '==> Configuring STM32U5x9J-DK GPU2D validation build\n'
distclean_tree
configure_board stm32u5x9j-dk:gpu2d-debug
verify_gpu2d_debug_config

printf '==> Building STM32U5x9J-DK GPU2D validation firmware\n'
make "-j${jobs}"

if [[ ! -f nuttx.bin ]]; then
  echo "ERROR: nuttx.bin was not produced" >&2
  exit 1
fi

copy_output nuttx.bin "${image_prefix}.bin"

validate_vector "${image_prefix}.bin"

printf '\n==> Firmware outputs\n'
printf '  GPU2D debug NSH internal-Flash image:\n'
printf '    kind:       non-LVGL screen + nemap_demo validation build\n'
printf '    bin:        %s\n' "${image_prefix}.bin"
printf '    bin size:   %s bytes\n' "$(file_size "${image_prefix}.bin")"
printf '    program at: internal Flash 0x08000000\n'
printf '    config:     GPU2D + LCD framebuffer + NSH nemap_demo + debug symbols/assertions\n'
printf '    run:        nemap_demo\n'
printf '    note:       debug build uses CONFIG_DEBUG_NOOPT and is larger/slower\n\n'
