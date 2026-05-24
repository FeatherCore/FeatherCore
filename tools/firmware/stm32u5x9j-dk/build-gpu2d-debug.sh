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
  printf 'Build STM32U5x9J-DK NSH LVGL firmware with GPU2D debug options.\n\n'
  printf 'This script reuses build-nsh-lvgl.sh for LVGL source preparation and\n'
  printf 'base board configuration, then enables GPU2D plus debug-friendly NuttX\n'
  printf 'options and rebuilds a separately named image.\n\n'
  printf 'Outputs are written to:\n\n'
  printf '  build/stm32u5x9j-dk-gpu2d-debug.bin\n'
  printf '      Raw NuttX NSH LVGL binary. Program at internal Flash 0x08000000.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N             Parallel make jobs (default: 8)\n'
  printf '      --lvgl-zip PATH      Use a local LVGL vX.Y.Z.zip archive\n'
  printf '      --lvgl-source PATH   Use a local LVGL git checkout with vX.Y.Z tag\n'
  printf '      --lvgl-url URL       Preferred LVGL archive download URL\n'
  printf '      --download-timeout N Curl connect timeout (default: 30)\n'
  printf '  -h, --help               Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"
base_script="${script_dir}/build-nsh-lvgl.sh"

build_dir="${feather_root}/build"
image_prefix="${build_dir}/stm32u5x9j-dk-gpu2d-debug"
jobs="${JOBS:-8}"
base_args=()

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

enable_config()
{
  local name="$1"

  if grep -q "^# ${name} is not set" .config; then
    sed -i "s/^# ${name} is not set/${name}=y/" .config
  elif grep -q "^${name}=" .config; then
    sed -i "s/^${name}=.*/${name}=y/" .config
  else
    printf '%s=y\n' "${name}" >> .config
  fi
}

disable_config()
{
  local name="$1"

  if grep -q "^${name}=" .config; then
    sed -i "s/^${name}=.*/# ${name} is not set/" .config
  elif ! grep -q "^# ${name} is not set" .config; then
    printf '# %s is not set\n' "${name}" >> .config
  fi
}

set_config_string()
{
  local name="$1"
  local value="$2"

  if grep -q "^${name}=" .config; then
    sed -i "s|^${name}=.*|${name}=\"${value}\"|" .config
  elif grep -q "^# ${name} is not set" .config; then
    sed -i "s|^# ${name} is not set|${name}=\"${value}\"|" .config
  else
    printf '%s="%s"\n' "${name}" "${value}" >> .config
  fi
}

copy_output()
{
  local input="$1"
  local output="$2"

  if [[ -f "${input}" ]]; then
    cp -f "${input}" "${output}"
  fi
}

enable_gpu2d_debug_config()
{
  printf '==> Enabling STM32U5x9J-DK GPU2D debug options\n'

  enable_config CONFIG_STM32U5_GPU2D
  enable_config CONFIG_STM32U5_GFXMMU
  enable_config CONFIG_STM32U5X9J_DK_LCD
  enable_config CONFIG_STM32U5X9J_DK_TOUCH
  enable_config CONFIG_STM32U5X9J_DK_HSPI_RAM
  enable_config CONFIG_DRIVERS_VIDEO
  enable_config CONFIG_VIDEO_FB
  enable_config CONFIG_FB_UPDATE
  enable_config CONFIG_FB_SYNC
  enable_config CONFIG_INPUT_TOUCHSCREEN

  enable_config CONFIG_DEBUG_FEATURES
  enable_config CONFIG_DEBUG_ASSERTIONS
  enable_config CONFIG_DEBUG_ASSERTIONS_EXPRESSION
  enable_config CONFIG_DEBUG_ASSERTIONS_FILENAME
  enable_config CONFIG_DEBUG_ERROR
  enable_config CONFIG_DEBUG_WARN
  enable_config CONFIG_DEBUG_INFO
  enable_config CONFIG_DEBUG_GRAPHICS
  enable_config CONFIG_DEBUG_GRAPHICS_ERROR
  enable_config CONFIG_DEBUG_GRAPHICS_WARN
  enable_config CONFIG_DEBUG_GRAPHICS_INFO
  enable_config CONFIG_DEBUG_VIDEO
  enable_config CONFIG_DEBUG_VIDEO_ERROR
  enable_config CONFIG_DEBUG_VIDEO_WARN
  enable_config CONFIG_DEBUG_VIDEO_INFO
  enable_config CONFIG_DEBUG_LCD
  enable_config CONFIG_DEBUG_LCD_ERROR
  enable_config CONFIG_DEBUG_LCD_WARN
  enable_config CONFIG_DEBUG_LCD_INFO
  enable_config CONFIG_DEBUG_INPUT
  enable_config CONFIG_DEBUG_INPUT_ERROR
  enable_config CONFIG_DEBUG_INPUT_WARN
  enable_config CONFIG_DEBUG_INPUT_INFO
  enable_config CONFIG_DEBUG_IRQ
  enable_config CONFIG_DEBUG_IRQ_ERROR
  enable_config CONFIG_DEBUG_IRQ_WARN
  enable_config CONFIG_DEBUG_DMA
  enable_config CONFIG_DEBUG_DMA_ERROR
  enable_config CONFIG_DEBUG_DMA_WARN

  enable_config CONFIG_DEBUG_SYMBOLS
  set_config_string CONFIG_DEBUG_SYMBOLS_LEVEL "-g3"
  enable_config CONFIG_FRAME_POINTER
  enable_config CONFIG_STACK_COLORATION
  enable_config CONFIG_STM32U5_DISABLE_IDLE_SLEEP_DURING_DEBUG

  enable_config CONFIG_DEBUG_NOOPT
  disable_config CONFIG_DEBUG_FULLOPT
  disable_config CONFIG_NDEBUG

  make olddefconfig
}

verify_gpu2d_debug_config()
{
  local missing=0
  local name

  for name in \
    CONFIG_STM32U5_GPU2D \
    CONFIG_STM32U5_GFXMMU \
    CONFIG_STM32U5X9J_DK_LCD \
    CONFIG_STM32U5X9J_DK_TOUCH \
    CONFIG_VIDEO_FB \
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
      base_args+=("$1" "$2")
      shift 2
      ;;
    -j*)
      jobs="${1#-j}"
      base_args+=("$1")
      shift
      ;;
    --jobs=*)
      jobs="${1#--jobs=}"
      base_args+=("$1")
      shift
      ;;
    --lvgl-zip|--lvgl-source|--lvgl-url|--download-timeout)
      base_args+=("$1" "$2")
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

if [[ ! -x "${base_script}" ]]; then
  printf 'ERROR: missing executable base script: %s\n' "${base_script}" >&2
  exit 1
fi

clean_debug_outputs

printf '==> Preparing STM32U5x9J-DK NSH LVGL base build\n'
"${base_script}" "${base_args[@]}"

cd "${feather_root}/nuttx"
enable_gpu2d_debug_config
verify_gpu2d_debug_config

printf '==> Rebuilding STM32U5x9J-DK GPU2D debug firmware\n'
make clean
make "-j${jobs}"

if [[ ! -f nuttx.bin ]]; then
  echo "ERROR: nuttx.bin was not produced" >&2
  exit 1
fi

copy_output nuttx.bin "${image_prefix}.bin"

validate_vector "${image_prefix}.bin"

printf '\n==> Firmware outputs\n'
printf '  GPU2D debug NSH LVGL internal-Flash image:\n'
printf '    bin:        %s\n' "${image_prefix}.bin"
printf '    bin size:   %s bytes\n' "$(file_size "${image_prefix}.bin")"
printf '    program at: internal Flash 0x08000000\n'
printf '    config:     GPU2D/GFXMMU + LCD/touch/LVGL + debug symbols/assertions\n'
printf '    note:       debug build uses CONFIG_DEBUG_NOOPT and is larger/slower\n\n'
