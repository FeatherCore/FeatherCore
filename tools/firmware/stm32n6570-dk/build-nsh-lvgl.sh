#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32n6570-dk/build-nsh-lvgl.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32N6570-DK NXboot and NSH LVGL framebuffer firmware.\n\n'
  printf 'Outputs are written to:\n\n'
  printf '  build/stm32n6570-dk-nxboot.bin\n'
  printf '      Program at XSPI2 NOR 0x70000000.\n\n'
  printf '  build/stm32n6570-dk-nsh-lvgl.bin\n'
  printf '      Program at XSPI2 NOR 0x70100000.\n\n'
  printf '  build/stm32n6570-dk-nsh-lvgl-full.bin\n'
  printf '      Combined XSPI2 NOR image. Program at 0x70000000.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N              Parallel make jobs (default: 8)\n'
  printf '  -v, --version VERSION     App semantic version (default: 0.1.0)\n'
  printf '      --signing-tool PATH   STM32_SigningTool_CLI path\n'
  printf '      --lvgl-zip PATH       Use a local LVGL vX.Y.Z.zip archive\n'
  printf '      --lvgl-source PATH    Use a local LVGL git checkout with vX.Y.Z tag\n'
  printf '      --lvgl-url URL        Preferred LVGL archive download URL\n'
  printf '      --arm2d-source PATH   Use a local Arm-2D checkout if enabled in .config\n'
  printf '      --arm2d-ref REF       Arm-2D git ref to fetch (default: v1.2.6)\n'
  printf '      --arm2d-url URL       Arm-2D git repository URL\n'
  printf '      --download-timeout N  Curl connect timeout (default: 30)\n'
  printf '  -h, --help                Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
pack_stm32_fsbl_nxboot_tool="../tools/firmware/stm32n6570-dk/pack-stm32-fsbl-nxboot.sh"
pack_nxboot_header_app_tool="../tools/firmware/stm32n6570-dk/pack-nxboot-header-app.sh"
pack_full_flash_image_tool="../tools/firmware/stm32n6570-dk/pack-full-flash-image.sh"

nxboot_image_bin="${build_dir}/stm32n6570-dk-nxboot.bin"
app_image_bin="${build_dir}/stm32n6570-dk-nsh-lvgl.bin"
full_image_bin="${build_dir}/stm32n6570-dk-nsh-lvgl-full.bin"

jobs="${JOBS:-8}"
version="0.1.0"
signing_tool="${STM32_SIGNING_TOOL:-${STM32_SIGNINGTOOL_CLI:-}}"
lvgl_zip="${LVGL_ZIP:-}"
lvgl_source="${LVGL_SOURCE:-}"
lvgl_url="${LVGL_URL:-}"
arm2d_source="${ARM2D_SOURCE:-${ARM_2D_SOURCE:-}}"
arm2d_ref="${ARM2D_REF:-${ARM_2D_REF:-v1.2.6}}"
arm2d_url="${ARM2D_URL:-${ARM_2D_URL:-https://github.com/ARM-software/Arm-2D.git}}"
download_timeout="${DOWNLOAD_TIMEOUT:-30}"
header_size=""
identifier=""

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
  find "${build_dir}" -maxdepth 1 -type f -name 'stm32n6570-dk-*' -delete
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

lvgl_config_enabled()
{
  local key="$1"

  grep -q "^${key}=y" .config
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

find_signing_tool()
{
  local candidate
  local cube_path="${STM32CUBE_PROGRAMMER_PATH:-}"
  local candidates=()

  if [[ -n "${cube_path}" ]]; then
    candidates+=("${cube_path}/bin/STM32_SigningTool_CLI")
    candidates+=("${cube_path}/bin/STM32_SigningTool_CLI.exe")
  fi

  candidates+=(
    "${feather_root}/tools/vendor/stmicro/stm32cubeprogrammer/bin/STM32_SigningTool_CLI"
    "${feather_root}/../third/stm32cubeprogrammer/bin/STM32_SigningTool_CLI"
    "/opt/st/stm32cubeprogrammer/bin/STM32_SigningTool_CLI"
    "/opt/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_SigningTool_CLI"
    "/mnt/c/Program Files/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_SigningTool_CLI.exe"
    "/mnt/c/Program Files/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_SigningTool_CLI"
  )

  for candidate in "${candidates[@]}"; do
    if [[ -x "${candidate}" ]]; then
      signing_tool="${candidate}"
      return 0
    fi
  done

  return 1
}

resolve_signing_tool()
{
  if [[ -z "${signing_tool}" ]] &&
     command -v STM32_SigningTool_CLI >/dev/null 2>&1; then
    signing_tool="$(command -v STM32_SigningTool_CLI)"
  fi

  if [[ -z "${signing_tool}" ]]; then
    find_signing_tool || true
  fi

  if [[ -z "${signing_tool}" ]]; then
    echo "ERROR: STM32_SigningTool_CLI is required to create the ST FSBL header." >&2
    echo "ERROR: pass --signing-tool /path/to/STM32_SigningTool_CLI or set STM32_SIGNING_TOOL." >&2
    exit 1
  fi

  if [[ ! -x "${signing_tool}" ]]; then
    printf 'ERROR: signing tool is not executable: %s\n' "${signing_tool}" >&2
    exit 1
  fi
}

require_helper()
{
  local path="$1"

  if [[ ! -x "${path}" ]]; then
    printf 'ERROR: helper script is not executable: %s\n' "${path}" >&2
    exit 1
  fi
}

verify_lvgl_config()
{
  local missing=0
  local name

  for name in \
    CONFIG_VIDEO_FB \
    CONFIG_FB_UPDATE \
    CONFIG_FB_SYNC \
    CONFIG_INPUT_TOUCHSCREEN \
    CONFIG_STM32N6_PSRAM_HEAP \
    CONFIG_STM32N6_DISPLAY \
    CONFIG_STM32N6_I2C2 \
    CONFIG_STM32N6_LTDC_FB_DOUBLE_BUFFER \
    CONFIG_STM32N6570_DK_LCD \
    CONFIG_STM32N6570_DK_GT911 \
    CONFIG_GRAPHICS_LVGL \
    CONFIG_EXAMPLES_LVGLDEMO \
    CONFIG_LV_USE_NUTTX \
    CONFIG_LV_USE_NUTTX_TOUCHSCREEN
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled by stm32n6570-dk:nsh-lvgl\n' \
        "${name}" >&2
      missing=1
    fi
  done

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
  local source_version="$2"

  [[ -d "${source_dir}/.git" ]] &&
    git -C "${source_dir}" rev-parse -q --verify \
      "v${source_version}^{commit}" >/dev/null
}

install_lvgl_source_from_git()
{
  local source_dir="$1"
  local source_version="$2"
  local target_dir="$3"

  if ! command -v git >/dev/null 2>&1; then
    echo "ERROR: git is required to use a local LVGL source checkout" >&2
    exit 1
  fi

  if ! lvgl_source_has_tag "${source_dir}" "${source_version}"; then
    printf 'ERROR: local LVGL source does not contain tag v%s: %s\n' \
      "${source_version}" "${source_dir}" >&2
    exit 1
  fi

  printf '==> Installing LVGL %s from local git source\n' "${source_version}"
  rm -rf "${target_dir}"
  git clone --shared --branch "v${source_version}" \
    "${source_dir}" "${target_dir}"
}

prepare_lvgl_source()
{
  local lvgl_app_dir="${feather_root}/apps/graphics/lvgl"
  local lvgl_source_dir="${lvgl_app_dir}/lvgl"
  local major minor patch source_version tarball url
  local -a urls=()

  major="$(config_value CONFIG_LVGL_VERSION_MAJOR)"
  minor="$(config_value CONFIG_LVGL_VERSION_MINOR)"
  patch="$(config_value CONFIG_LVGL_VERSION_PATCH)"

  if [[ -z "${major}" || -z "${minor}" || -z "${patch}" ]]; then
    echo "ERROR: LVGL version is missing from .config" >&2
    exit 1
  fi

  source_version="${major}.${minor}.${patch}"
  tarball="${lvgl_app_dir}/v${source_version}.zip"

  if [[ -f "${lvgl_source_dir}/lvgl.mk" ]]; then
    printf '==> Reusing LVGL source tree: %s\n' "${lvgl_source_dir}"
    return
  fi

  if [[ -n "${lvgl_source}" ]]; then
    install_lvgl_source_from_git "${lvgl_source}" \
      "${source_version}" "${lvgl_source_dir}"
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

  if [[ -f "${tarball}" ]]; then
    printf '==> Reusing LVGL archive: %s\n' "${tarball}"
    verify_zip "${tarball}"
    return
  fi

  if ! command -v curl >/dev/null 2>&1; then
    cat >&2 <<EOF
ERROR: curl is required to download LVGL ${source_version}, or provide:
  $0 --lvgl-zip /path/to/v${source_version}.zip
EOF
    exit 1
  fi

  if [[ -n "${lvgl_url}" ]]; then
    urls+=("${lvgl_url}")
  fi

  urls+=(
    "https://codeload.github.com/lvgl/lvgl/zip/refs/tags/v${source_version}"
    "https://github.com/lvgl/lvgl/archive/refs/tags/v${source_version}.zip"
  )

  printf '==> Downloading LVGL %s source\n' "${source_version}"
  for url in "${urls[@]}"; do
    printf '    url: %s\n' "${url}"
    if curl -fL --connect-timeout "${download_timeout}" --retry 1 \
        -o "${tarball}" "${url}"; then
      verify_zip "${tarball}"
      return
    fi

    rm -f "${tarball}"
  done

  printf 'ERROR: failed to download LVGL %s\n' "${source_version}" >&2
  exit 1
}

arm2d_source_is_valid()
{
  local source_dir="$1"

  [[ -f "${source_dir}/Library/Include/arm_2d.h" ]] &&
    [[ -d "${source_dir}/Library/Source" ]] &&
    [[ -d "${source_dir}/examples/[template][pc][vscode]/platform/math" ]]
}

install_arm2d_subset()
{
  local source_dir="$1"
  local target_dir="$2"
  local math_source="${source_dir}/examples/[template][pc][vscode]/platform/math"
  local math_target="${target_dir}/examples/[template][pc][vscode]/platform/math"

  printf '==> Installing Arm-2D from local source: %s\n' "${source_dir}"
  rm -rf "${target_dir}"
  mkdir -p "${target_dir}/examples/[template][pc][vscode]/platform"
  cp -a "${source_dir}/Library" "${target_dir}/Library"
  cp -a "${math_source}" "${math_target}"

  if [[ -f "${source_dir}/LICENSE" ]]; then
    cp -a "${source_dir}/LICENSE" "${target_dir}/LICENSE"
  fi
}

prepare_arm2d_source()
{
  local lvgl_app_dir="${feather_root}/apps/graphics/lvgl"
  local arm2d_target_dir="${lvgl_app_dir}/arm-2d"

  if ! lvgl_config_enabled CONFIG_LV_USE_DRAW_ARM2D_SYNC; then
    return
  fi

  if arm2d_source_is_valid "${arm2d_target_dir}"; then
    printf '==> Reusing Arm-2D source tree: %s\n' "${arm2d_target_dir}"
    return
  fi

  if [[ -n "${arm2d_source}" ]]; then
    if ! arm2d_source_is_valid "${arm2d_source}"; then
      printf 'ERROR: local Arm-2D source is missing Library or platform/math: %s\n' \
        "${arm2d_source}" >&2
      exit 1
    fi

    install_arm2d_subset "${arm2d_source}" "${arm2d_target_dir}"
    return
  fi

  if ! command -v git >/dev/null 2>&1; then
    cat >&2 <<EOF
ERROR: git is required to fetch Arm-2D ${arm2d_ref}, or provide:
  $0 --arm2d-source /path/to/Arm-2D
EOF
    exit 1
  fi

  printf '==> Downloading Arm-2D %s source\n' "${arm2d_ref}"
  rm -rf "${arm2d_target_dir}"
  git clone --depth 1 --branch "${arm2d_ref}" --filter=blob:none --sparse \
    "${arm2d_url}" "${arm2d_target_dir}"
  git -C "${arm2d_target_dir}" sparse-checkout set \
    LICENSE \
    Library/Include \
    Library/Source \
    'examples/[template][pc][vscode]/platform/math'

  if ! arm2d_source_is_valid "${arm2d_target_dir}"; then
    printf 'ERROR: downloaded Arm-2D source is incomplete: %s\n' \
      "${arm2d_target_dir}" >&2
    exit 1
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
    --signing-tool)
      signing_tool="$2"
      shift 2
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
    --arm2d-source)
      arm2d_source="$2"
      shift 2
      ;;
    --arm2d-ref)
      arm2d_ref="$2"
      shift 2
      ;;
    --arm2d-url)
      arm2d_url="$2"
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

resolve_signing_tool
require_helper "${pack_stm32_fsbl_nxboot_tool}"
require_helper "${pack_nxboot_header_app_tool}"
require_helper "${pack_full_flash_image_tool}"

printf '==> Cleaning STM32N6570-DK build outputs\n'
clean_build_dir

printf '==> Building STM32N6570-DK NXboot\n'
distclean_tree
configure_board stm32n6570-dk:nxboot
make "-j${jobs}"
nxboot_payload_size="$(file_size nuttx.bin)"
"${pack_stm32_fsbl_nxboot_tool}" "${signing_tool}" nuttx.bin "${nxboot_image_bin}"

printf '\n==> Building STM32N6570-DK NSH LVGL framebuffer app\n'
distclean_tree
configure_board stm32n6570-dk:nsh-lvgl
verify_lvgl_config
prepare_lvgl_source
prepare_arm2d_source
clean_lvgl_objects
make "-j${jobs}"
app_payload_size="$(file_size nuttx.bin)"
"${pack_nxboot_header_app_tool}" nuttx.bin "${app_image_bin}" "${version}"
"${pack_full_flash_image_tool}" "${nxboot_image_bin}" "${app_image_bin}" \
  "${full_image_bin}"

header_size="$(config_value CONFIG_NXBOOT_HEADER_SIZE)"
header_size="${header_size:-0x400}"
identifier="$(config_value CONFIG_NXBOOT_PLATFORM_IDENTIFIER)"
identifier="${identifier:-0x4e363537}"

printf '\n==> Firmware outputs\n'
printf '  NXboot trusted image:\n'
printf '    file:       %s\n' "${nxboot_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${nxboot_image_bin}")"
printf '    payload:    %s bytes raw NuttX NXboot\n' "${nxboot_payload_size}"
printf '    program at: XSPI2 NOR 0x70000000\n\n'

printf '  NSH LVGL NXboot app image:\n'
printf '    file:       %s\n' "${app_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${app_image_bin}")"
printf '    payload:    %s bytes raw NuttX NSH LVGL app\n' \
  "${app_payload_size}"
printf '    structure:  [NXboot header %s][NuttX app raw binary]\n' "${header_size}"
printf '    identifier: %s\n' "${identifier}"
printf '    program at: XSPI2 NOR 0x70100000\n'
printf '    app vector: 0x70100000 + %s, normally 0x70100400\n' \
  "${header_size}"
printf '    framebuffer: /dev/fb0, RGB565 800x480\n'
printf '    touchscreen: /dev/input0\n'

printf '\n  Full XSPI2 NOR image:\n'
printf '    file:       %s\n' "${full_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${full_image_bin}")"
printf '    structure:  [NXboot at +0x0][0xff padding][app at +0x100000]\n'
printf '    program at: XSPI2 NOR 0x70000000\n'
