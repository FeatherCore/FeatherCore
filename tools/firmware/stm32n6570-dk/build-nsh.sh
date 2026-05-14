#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32n6570-dk/build-nsh.sh
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
  printf 'Build STM32N6570-DK NXboot and NSH NXboot app firmware.\n\n'
  printf 'Run from anywhere inside the Feather checkout. Outputs are written to:\n\n'
  printf '  build/stm32n6570-dk-nxboot.bin\n'
  printf '      Final trusted NXboot image: [ST BootROM FSBL header][NuttX NXboot payload].\n'
  printf '      Program at XSPI2 NOR 0x70000000.\n\n'
  printf '  build/stm32n6570-dk-nsh.bin\n'
  printf '      Final NSH app image: [NXboot header][NuttX app raw binary].\n'
  printf '      Program at XSPI2 NOR 0x70100000. The app vector table starts at\n'
  printf '      0x70100000 + CONFIG_NXBOOT_HEADER_SIZE, normally 0x70100400.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N              Parallel make jobs (default: 8)\n'
  printf '  -v, --version VERSION     App semantic version (default: 0.1.0)\n'
  printf '      --signing-tool PATH   STM32_SigningTool_CLI path. May also be set via\n'
  printf '                            STM32_SIGNING_TOOL, STM32_SIGNINGTOOL_CLI, or\n'
  printf '                            STM32CUBE_PROGRAMMER_PATH/bin. If omitted, the\n'
  printf '                            script also checks tools/vendor/stmicro/\n'
  printf '                            stm32cubeprogrammer/bin.\n'
  printf '  -h, --help                Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
pack_stm32_fsbl_nxboot_tool="../tools/firmware/stm32n6570-dk/pack-stm32-fsbl-nxboot.sh"
pack_nxboot_header_app_tool="../tools/firmware/stm32n6570-dk/pack-nxboot-header-app.sh"

nxboot_image_bin="${build_dir}/stm32n6570-dk-nxboot.bin"
app_image_bin="${build_dir}/stm32n6570-dk-nsh.bin"

jobs="${JOBS:-8}"
version="0.1.0"
signing_tool="${STM32_SIGNING_TOOL:-${STM32_SIGNINGTOOL_CLI:-}}"

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
  find "${build_dir}" -maxdepth 1 -type f -name 'stm32n6570-dk-*' -delete
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
    echo "ERROR: install STM32CubeProgrammer/SigningTool and pass its path, for example:" >&2
    echo "ERROR:   $0 --signing-tool /opt/st/stm32cubeprogrammer/bin/STM32_SigningTool_CLI" >&2
    echo "ERROR: or set:" >&2
    echo "ERROR:   export STM32_SIGNING_TOOL=/path/to/STM32_SigningTool_CLI" >&2
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

printf '==> Cleaning STM32N6570-DK build outputs\n'
clean_build_dir

printf '==> Building STM32N6570-DK NXboot\n'
distclean_tree
configure_board stm32n6570-dk:nxboot
make "-j${jobs}"
nxboot_payload_size="$(file_size nuttx.bin)"
"${pack_stm32_fsbl_nxboot_tool}" "${signing_tool}" nuttx.bin "${nxboot_image_bin}"

printf '\n==> Building STM32N6570-DK NSH NXboot app\n'
distclean_tree
configure_board stm32n6570-dk:nsh
make "-j${jobs}"
app_payload_size="$(file_size nuttx.bin)"
"${pack_nxboot_header_app_tool}" nuttx.bin "${app_image_bin}" "${version}"

header_size="$(config_value CONFIG_NXBOOT_HEADER_SIZE)"
header_size="${header_size:-0x400}"
identifier="$(config_value CONFIG_NXBOOT_PLATFORM_IDENTIFIER)"
identifier="${identifier:-0x4e363537}"

printf '\n==> Firmware outputs\n'
printf '  NXboot trusted image:\n'
printf '    file:       %s\n' "${nxboot_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${nxboot_image_bin}")"
printf '    payload:    %s bytes raw NuttX NXboot\n' "${nxboot_payload_size}"
printf '    structure:  [ST BootROM FSBL header][NuttX NXboot payload]\n'
printf '    program at: XSPI2 NOR 0x70000000\n\n'

printf '  NSH NXboot app image:\n'
printf '    file:       %s\n' "${app_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${app_image_bin}")"
printf '    payload:    %s bytes raw NuttX NSH app\n' "${app_payload_size}"
printf '    structure:  [NXboot header %s][NuttX app raw binary]\n' "${header_size}"
printf '    identifier: %s\n' "${identifier}"
printf '    program at: XSPI2 NOR 0x70100000\n'
printf '    app vector: 0x70100000 + %s, normally 0x70100400\n\n' "${header_size}"
