#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32u5x9j-dk/build-nsh-co5300.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32U5x9J-DK NSH firmware for the 1.73 AMOLED CO5300 panel.\n\n'
  printf 'This is the display-validation entry point for the external CO5300\n'
  printf 'MIPI-DSI adapter wiring. It intentionally refuses to build if the\n'
  printf 'tree still only contains the stock HX8379C panel path.\n\n'
  printf 'Run from anywhere inside the Feather checkout. Outputs are written to:\n\n'
  printf '  build/stm32u5x9j-dk-nsh-co5300.bin\n'
  printf '      Raw NuttX CO5300 validation binary. Program at internal Flash 0x08000000.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N          Parallel make jobs (default: 8)\n'
  printf '      --config NAME     Board config (default: stm32u5x9j-dk:nsh-co5300)\n'
  printf '  -h, --help            Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

build_dir="${feather_root}/build"
image_prefix="${build_dir}/stm32u5x9j-dk-nsh-co5300"

jobs="${JOBS:-8}"
board_config="${BOARD_CONFIG:-stm32u5x9j-dk:nsh-co5300}"
flash_start="0x08000000"
flash_size="4096 KiB"

file_size()
{
  wc -c < "$1" | tr -d '[:space:]'
}

clean_co5300_outputs()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f \
    -name 'stm32u5x9j-dk-nsh-co5300*' -delete
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

co5300_source_present()
{
  grep -Rqs 'CO5300' \
    "${feather_root}/nuttx/boards/arm/stm32u5/stm32u5x9j-dk" \
    "${feather_root}/nuttx/drivers/lcd" \
    "${feather_root}/nuttx/include/nuttx/lcd"
}

co5300_id_read_present()
{
  grep -Rqs 'CO5300_DCS_READ_ID' \
    "${feather_root}/nuttx/drivers/lcd/co5300.c" &&
  grep -Rqs 'stm32_dsiread' \
    "${feather_root}/nuttx/arch/arm/src/stm32u5" \
    "${feather_root}/nuttx/boards/arm/stm32u5/stm32u5x9j-dk"
}

verify_co5300_config()
{
  local missing=0
  local name

  if ! co5300_source_present; then
    cat >&2 <<'EOF'
ERROR: CO5300 panel support is not present in this NuttX tree yet.

This script is reserved for the real CO5300 validation firmware. Add the
STM32U5x9J-DK CO5300 board config and replace the stock HX8379C panel init
path before using it for hardware verification.

Expected first-bring-up baseline:
  - MIPI DSI clock lane + D0 only
  - 1 data lane
  - 466x466 active area
  - RGB565 or an explicitly documented RGB888 fallback
  - CO5300 vendor init sequence from CO5300_1_73_DRIVER_FLOW.md
EOF
    exit 1
  fi

  if ! co5300_id_read_present; then
    cat >&2 <<'EOF'
ERROR: CO5300 ID-read diagnostic is not present in this tree.

This validation build must actively read DCS 0x04 before continuing panel
initialization, so the serial log proves whether the STM32U5 DSI command/BTA
return path can talk to the panel.
EOF
    exit 1
  fi

  for name in \
    CONFIG_STM32U5X9J_DK_LCD \
    CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM \
    CONFIG_STM32U5X9J_DK_LCD_RGB565 \
    CONFIG_STM32U5_DSIHOST \
    CONFIG_STM32U5_LTDC \
    CONFIG_STM32U5_LTDC_FB_DOUBLE_BUFFER \
    CONFIG_STM32U5_DMA2D \
    CONFIG_VIDEO_FB \
    CONFIG_FB_SYNC
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled for CO5300 validation build\n' \
        "${name}" >&2
      missing=1
    fi
  done

  if grep -q '^CONFIG_LCD_HX8379C=y' .config; then
    printf 'ERROR: CONFIG_LCD_HX8379C is still enabled; this is not a CO5300 firmware\n' >&2
    missing=1
  fi

  if ! grep -Eq '^CONFIG_LCD_CO5300=y|^CONFIG_STM32U5X9J_DK_LCD_CO5300=y' .config; then
    printf 'ERROR: no CO5300 panel config is enabled in .config\n' >&2
    missing=1
  fi

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
    --config)
      board_config="$2"
      shift 2
      ;;
    --config=*)
      board_config="${1#--config=}"
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

clean_co5300_outputs

cd "${feather_root}/nuttx"

printf '==> Configuring STM32U5x9J-DK CO5300 validation build\n'
printf '    config: %s\n' "${board_config}"
distclean_tree
configure_board "${board_config}"
verify_co5300_config

printf '==> Building STM32U5x9J-DK CO5300 validation firmware\n'
make "-j${jobs}"

if [[ ! -f nuttx.bin ]]; then
  echo "ERROR: nuttx.bin was not produced" >&2
  exit 1
fi

copy_output nuttx.bin "${image_prefix}.bin"

validate_vector "${image_prefix}.bin"

printf '\n==> Firmware outputs\n'
printf '  CO5300 NSH internal-Flash image:\n'
printf '    kind:       1.73 AMOLED CO5300 display-validation build\n'
printf '    bin:        %s\n' "${image_prefix}.bin"
printf '    bin size:   %s bytes\n' "$(file_size "${image_prefix}.bin")"
printf '    program at: internal Flash %s\n' "${flash_start}"
printf '    flash size: %s\n' "${flash_size}"
printf '    wiring:     MIPI DSI CLK + D0, +3V3/GND, optional LCD_RST\n'
printf '    reference:  /home/uan/Feather-develop-HW/CO5300_1_73_DRIVER_FLOW.md\n\n'
