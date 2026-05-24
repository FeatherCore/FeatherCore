#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32u5x9j-dk/build-nsh-lvgl.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32U5x9J-DK NSH LVGL framebuffer firmware for internal Flash.\n\n'
  printf 'Run from anywhere inside the Feather checkout. Outputs are written to:\n\n'
  printf '  build/stm32u5x9j-dk-nsh-lvgl.bin\n'
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

cd "${feather_root}/nuttx"

build_dir="../build"
image_prefix="${build_dir}/stm32u5x9j-dk-nsh-lvgl"

jobs="${JOBS:-8}"
flash_start="0x08000000"
flash_size="4096 KiB"
lvgl_zip="${LVGL_ZIP:-}"
lvgl_source="${LVGL_SOURCE:-}"
lvgl_url="${LVGL_URL:-}"
download_timeout="${DOWNLOAD_TIMEOUT:-30}"

config_value()
{
  local key="$1"

  if [[ -f .config ]]; then
    sed -n "s/^${key}=//p" .config | tail -n 1 | tr -d '"'
  fi
}

file_size()
{
  wc -c < "$1" | tr -d '[:space:]'
}

clean_build_dir()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'stm32u5x9j-dk-nsh-lvgl*' \
    -delete
}

clean_lvgl_objects()
{
  local lvgl_app_dir="${feather_root}/apps/graphics/lvgl"

  if [[ -d "${lvgl_app_dir}" ]]; then
    find "${lvgl_app_dir}" -type f \
      \( -name '*.o' -o -name '*.d' -o -name 'Make.dep' \
         -o -name '.built' -o -name '.depend' \) \
      -delete
  fi
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

set_config_int()
{
  local name="$1"
  local value="$2"

  if grep -q "^${name}=" .config; then
    sed -i "s/^${name}=.*/${name}=${value}/" .config
  elif grep -q "^# ${name} is not set" .config; then
    sed -i "s/^# ${name} is not set/${name}=${value}/" .config
  else
    printf '%s=%s\n' "${name}" "${value}" >> .config
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

lcd_format_name()
{
  if grep -q "^CONFIG_STM32U5X9J_DK_LCD_RGB565=y" .config; then
    printf 'RGB565'
  else
    printf 'XRGB8888'
  fi
}

lcd_fb_map_name()
{
  if grep -q "^CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM=y" .config; then
    printf 'direct PSRAM'
  elif grep -q "^CONFIG_STM32U5X9J_DK_LCD_FB_SRAM=y" .config; then
    printf 'GFXMMU internal SRAM'
  else
    printf 'unknown framebuffer backing'
  fi
}

enable_lvgl_config()
{
  printf '==> Enabling STM32U5x9J-DK LVGL framebuffer options\n'

  enable_config CONFIG_DRIVERS_VIDEO
  enable_config CONFIG_VIDEO_FB
  enable_config CONFIG_FB_UPDATE
  enable_config CONFIG_FB_SYNC
  enable_config CONFIG_INPUT
  enable_config CONFIG_INPUT_TOUCHSCREEN
  enable_config CONFIG_I2C
  enable_config CONFIG_I2C_DRIVER
  enable_config CONFIG_STM32U5X9J_DK_I2C_BUSES
  enable_config CONFIG_STM32U5X9J_DK_HSPI_RAM
  enable_config CONFIG_STM32U5X9J_DK_HSPI_HEAP
  enable_config CONFIG_STM32U5X9J_DK_LCD
  enable_config CONFIG_STM32U5X9J_DK_TOUCH
  enable_config CONFIG_STM32U5_LTDC_FB_DOUBLE_BUFFER
  enable_config CONFIG_STM32U5X9J_DK_LCD_RGB565
  disable_config CONFIG_STM32U5X9J_DK_LCD_XRGB8888
  enable_config CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM
  disable_config CONFIG_STM32U5X9J_DK_LCD_FB_SRAM
  enable_config CONFIG_STM32U5X9J_DK_LCD_COLORBAR
  disable_config CONFIG_STM32U5X9J_DK_LCD_PATTERN

  # LVGL benchmark results are render-bound on the Cortex-M33.  The base NSH
  # defconfig enables debug symbols, which otherwise selects DEBUG_NOOPT.
  disable_config CONFIG_DEBUG_NOOPT
  disable_config CONFIG_DEBUG_CUSTOMOPT
  enable_config CONFIG_DEBUG_FULLOPT
  enable_config CONFIG_ARMV8M_MEMCPY
  enable_config CONFIG_ARMV8M_MEMSET

  # Keep instruction fetches and PSRAM-backed LVGL heap traffic cached in the
  # flat NSH build too.  Framebuffer ownership is handled by explicit cache
  # clean before LTDC scanout, not by disabling caches globally.
  enable_config CONFIG_STM32U5_ICACHE
  enable_config CONFIG_STM32U5_ICACHE_DIRECT
  enable_config CONFIG_STM32U5_DCACHE1

  enable_config CONFIG_GRAPHICS_LVGL
  enable_config CONFIG_EXAMPLES_LVGLDEMO
  enable_config CONFIG_LV_USE_NUTTX
  enable_config CONFIG_LV_USE_NUTTX_TOUCHSCREEN
  disable_config CONFIG_LV_USE_BUILTIN_MALLOC
  enable_config CONFIG_LV_USE_CLIB_MALLOC
  disable_config CONFIG_LV_USE_BUILTIN_STRING
  enable_config CONFIG_LV_USE_CLIB_STRING
  disable_config CONFIG_LV_USE_BUILTIN_SPRINTF
  enable_config CONFIG_LV_USE_CLIB_SPRINTF
  enable_config CONFIG_LV_USE_LOG
  disable_config CONFIG_LV_LOG_LEVEL_TRACE
  disable_config CONFIG_LV_LOG_LEVEL_INFO
  enable_config CONFIG_LV_LOG_LEVEL_WARN
  disable_config CONFIG_LV_LOG_LEVEL_ERROR
  disable_config CONFIG_LV_LOG_LEVEL_USER
  disable_config CONFIG_LV_LOG_LEVEL_NONE
  enable_config CONFIG_LV_USE_SYSMON
  enable_config CONFIG_LV_USE_PERF_MONITOR
  disable_config CONFIG_LV_USE_PERF_MONITOR_LOG_MODE
  disable_config CONFIG_LV_BUILD_EXAMPLES
  enable_config CONFIG_LV_COLOR_DEPTH_16
  disable_config CONFIG_LV_COLOR_DEPTH_32
  set_config_int CONFIG_LV_COLOR_DEPTH 16
  enable_config CONFIG_LV_FONT_MONTSERRAT_20
  enable_config CONFIG_LV_FONT_MONTSERRAT_24
  enable_config CONFIG_LV_USE_DEMO_WIDGETS
  enable_config CONFIG_LV_USE_DEMO_BENCHMARK

  set_config_int CONFIG_EXAMPLES_LVGLDEMO_STACKSIZE 32768
  set_config_string CONFIG_EXAMPLES_LVGLDEMO_INPUT_DEVPATH /dev/input0

  make olddefconfig
}

verify_lvgl_config()
{
  local missing=0
  local name

  for name in \
    CONFIG_VIDEO_FB \
    CONFIG_FB_SYNC \
    CONFIG_INPUT_TOUCHSCREEN \
    CONFIG_STM32U5X9J_DK_HSPI_RAM \
    CONFIG_STM32U5X9J_DK_HSPI_HEAP \
    CONFIG_STM32U5X9J_DK_LCD \
    CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM \
    CONFIG_STM32U5X9J_DK_TOUCH \
    CONFIG_STM32U5_LTDC_FB_DOUBLE_BUFFER \
    CONFIG_STM32U5_ICACHE \
    CONFIG_STM32U5_ICACHE_DIRECT \
    CONFIG_STM32U5_DCACHE1 \
    CONFIG_DEBUG_FULLOPT \
    CONFIG_ARMV8M_MEMCPY \
    CONFIG_ARMV8M_MEMSET \
    CONFIG_GRAPHICS_LVGL \
    CONFIG_EXAMPLES_LVGLDEMO \
    CONFIG_LV_USE_NUTTX \
    CONFIG_LV_USE_NUTTX_TOUCHSCREEN \
    CONFIG_LV_USE_CLIB_MALLOC \
    CONFIG_LV_USE_CLIB_STRING \
    CONFIG_LV_USE_CLIB_SPRINTF \
    CONFIG_LV_USE_LOG \
    CONFIG_LV_LOG_LEVEL_WARN \
    CONFIG_LV_USE_SYSMON \
    CONFIG_LV_USE_PERF_MONITOR \
    CONFIG_LV_FONT_MONTSERRAT_20 \
    CONFIG_LV_FONT_MONTSERRAT_24 \
    CONFIG_LV_USE_DEMO_WIDGETS \
    CONFIG_LV_USE_DEMO_BENCHMARK
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled for stm32u5x9j-dk:nsh LVGL\n' \
        "${name}" >&2
      missing=1
    fi
  done

  if grep -q "^CONFIG_STM32U5X9J_DK_LCD_XRGB8888=y" .config; then
    if ! grep -q "^CONFIG_LV_COLOR_DEPTH_32=y" .config; then
      printf 'ERROR: 32-bit framebuffer requires CONFIG_LV_COLOR_DEPTH_32\n' >&2
      missing=1
    fi
  elif grep -q "^CONFIG_STM32U5X9J_DK_LCD_RGB565=y" .config; then
    if ! grep -q "^CONFIG_LV_COLOR_DEPTH_16=y" .config; then
      printf 'ERROR: RGB565 framebuffer requires CONFIG_LV_COLOR_DEPTH_16\n' >&2
      missing=1
    fi
  else
    printf 'ERROR: no STM32U5x9J-DK LCD pixel format is selected\n' >&2
    missing=1
  fi

  if [[ "${missing}" -ne 0 ]]; then
    exit 1
  fi
}

verify_zip()
{
  local zip_path="$1"

  if command -v unzip >/dev/null 2>&1; then
    if ! unzip -tq "${zip_path}" >/dev/null; then
      printf 'ERROR: LVGL archive is not a valid zip: %s\n' "${zip_path}" >&2
      exit 1
    fi
  fi
}

lvgl_source_has_tag()
{
  local source_dir="$1"
  local version="$2"

  [[ -d "${source_dir}/.git" ]] &&
    git -C "${source_dir}" rev-parse -q --verify "v${version}^{commit}" \
      >/dev/null
}

install_lvgl_source_from_git()
{
  local source_dir="$1"
  local version="$2"
  local target_dir="$3"

  if ! command -v git >/dev/null 2>&1; then
    echo "ERROR: git is required to use a local LVGL source checkout" >&2
    exit 1
  fi

  if ! lvgl_source_has_tag "${source_dir}" "${version}"; then
    printf 'ERROR: local LVGL source does not contain tag v%s: %s\n' \
      "${version}" "${source_dir}" >&2
    exit 1
  fi

  printf '==> Installing LVGL %s from local git source\n' "${version}"
  rm -rf "${target_dir}"
  git clone --shared --branch "v${version}" "${source_dir}" "${target_dir}"
}

find_local_lvgl_source()
{
  local version="$1"
  local candidate
  local candidates=(
    "${feather_root}/../lvgl"
    "${HOME:-}/FeatherCore/lvgl"
    "${HOME:-}/codes/lvgl"
    "${HOME:-}/zephyrproject/modules/lib/gui/lvgl"
  )

  for candidate in "${candidates[@]}"; do
    if lvgl_source_has_tag "${candidate}" "${version}"; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done

  return 1
}

prepare_lvgl_source()
{
  local lvgl_app_dir="${feather_root}/apps/graphics/lvgl"
  local lvgl_source_dir="${lvgl_app_dir}/lvgl"
  local major minor patch version tarball url local_source
  local -a urls=()

  major="$(config_value CONFIG_LVGL_VERSION_MAJOR)"
  minor="$(config_value CONFIG_LVGL_VERSION_MINOR)"
  patch="$(config_value CONFIG_LVGL_VERSION_PATCH)"

  if [[ -z "${major}" || -z "${minor}" || -z "${patch}" ]]; then
    echo "ERROR: LVGL version is missing from .config" >&2
    exit 1
  fi

  version="${major}.${minor}.${patch}"
  tarball="${lvgl_app_dir}/v${version}.zip"

  if [[ -f "${lvgl_source_dir}/lvgl.mk" ]]; then
    printf '==> Reusing LVGL source tree: %s\n' "${lvgl_source_dir}"
    return
  fi

  if [[ -n "${lvgl_source}" ]]; then
    install_lvgl_source_from_git "${lvgl_source}" "${version}" "${lvgl_source_dir}"
    return
  fi

  if [[ -n "${lvgl_zip}" ]]; then
    if [[ ! -f "${lvgl_zip}" ]]; then
      printf 'ERROR: local LVGL archive does not exist: %s\n' "${lvgl_zip}" >&2
      exit 1
    fi

    printf '==> Using local LVGL archive: %s\n' "${lvgl_zip}"
    verify_zip "${lvgl_zip}"
    cp "${lvgl_zip}" "${tarball}"
    return
  fi

  if local_source="$(find_local_lvgl_source "${version}")"; then
    install_lvgl_source_from_git "${local_source}" "${version}" "${lvgl_source_dir}"
    return
  fi

  if [[ -f "${tarball}" ]]; then
    printf '==> Reusing LVGL archive: %s\n' "${tarball}"
    verify_zip "${tarball}"
    return
  fi

  if ! command -v curl >/dev/null 2>&1; then
    cat >&2 <<EOF
ERROR: curl is required to download LVGL ${version}, or provide:
  $0 --lvgl-zip /path/to/v${version}.zip
  $0 --lvgl-source /path/to/lvgl
EOF
    exit 1
  fi

  if [[ -n "${lvgl_url}" ]]; then
    urls+=("${lvgl_url}")
  fi

  if [[ -n "${LVGL_URLS:-}" ]]; then
    # shellcheck disable=SC2206
    urls+=(${LVGL_URLS})
  fi

  urls+=(
    "https://codeload.github.com/lvgl/lvgl/zip/refs/tags/v${version}"
    "https://github.com/lvgl/lvgl/archive/refs/tags/v${version}.zip"
  )

  printf '==> Downloading LVGL %s source\n' "${version}"
  for url in "${urls[@]}"; do
    printf '    trying %s\n' "${url}"
    if curl -fL --connect-timeout "${download_timeout}" \
        -o "${tarball}.tmp" "${url}"; then
      mv "${tarball}.tmp" "${tarball}"
      verify_zip "${tarball}"
      return
    fi
    rm -f "${tarball}.tmp"
  done

  printf 'ERROR: failed to download LVGL %s\n' "${version}" >&2
  exit 1
}

copy_output()
{
  local input="$1"
  local output="$2"

  if [[ -f "${input}" ]]; then
    cp -f "${input}" "${output}"
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
    --lvgl-zip)
      lvgl_zip="$2"
      shift 2
      ;;
    --lvgl-source)
      lvgl_source="$2"
      shift 2
      ;;
    --lvgl-url)
      lvgl_url="$2"
      shift 2
      ;;
    --download-timeout)
      download_timeout="$2"
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

printf '==> Cleaning STM32U5x9J-DK NSH LVGL build outputs\n'
clean_build_dir
clean_lvgl_objects

printf '==> Configuring STM32U5x9J-DK NSH LVGL for internal Flash\n'
distclean_tree
configure_board stm32u5x9j-dk:nsh
enable_lvgl_config
verify_lvgl_config
prepare_lvgl_source

printf '==> Building STM32U5x9J-DK NSH LVGL for internal Flash\n'
make "-j${jobs}"

if [[ ! -f nuttx.bin ]]; then
  echo "ERROR: nuttx.bin was not produced" >&2
  exit 1
fi

copy_output nuttx.bin "${image_prefix}.bin"

validate_vector "${image_prefix}.bin"

printf '\n==> Firmware outputs\n'
printf '  NSH LVGL internal-Flash image:\n'
printf '    bin:        %s\n' "${image_prefix}.bin"
printf '    bin size:   %s bytes\n' "$(file_size "${image_prefix}.bin")"
printf '    structure:  raw NuttX image, no NXboot header\n'
printf '    program at: internal Flash %s\n' "${flash_start}"
printf '    flash size: %s\n' "${flash_size}"
printf '    display:    /dev/fb0 with two %s 480x480 %s framebuffers\n' \
  "$(lcd_format_name)" "$(lcd_fb_map_name)"
printf '    input:      /dev/input0 touchscreen\n\n'
