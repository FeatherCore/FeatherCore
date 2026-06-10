#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-sta1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build NuttX simulator Wi-Fi hwsim STA1 image.\n\n'
  printf 'Outputs:\n'
  printf '  build/nuttx-sim-sta1      host executable for NuttX sim:hwsim_sta1\n'
  printf '  build/nuttx-sim-sta1.map  linker map, when produced by the build\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N       Parallel make jobs (default: 8)\n'
  printf '      --no-clean     Reuse the current NuttX tree configuration\n'
  printf '  -h, --help         Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
sim_config="hwsim_sta1"
sim_exe="${build_dir}/nuttx-sim-sta1"
sim_map="${build_dir}/nuttx-sim-sta1.map"

jobs="${JOBS:-8}"
clean_first=1

clean_build_dir()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'nuttx-sim-sta1*' -delete
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

disable_config()
{
  local name="$1"

  if grep -q "^${name}=" .config; then
    sed -i "s/^${name}=.*/# ${name} is not set/" .config
  elif ! grep -q "^# ${name} is not set" .config; then
    printf '# %s is not set\n' "${name}" >> .config
  fi
}

prepare_host_optional_romfs()
{
  if command -v genromfs >/dev/null 2>&1; then
    return
  fi

  if grep -q '^CONFIG_ETC_ROMFS=y' .config; then
    printf '==> genromfs not found; disabling CONFIG_ETC_ROMFS for this host build\n'
    disable_config CONFIG_ETC_ROMFS
    make olddefconfig
  fi
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
    --no-clean)
      clean_first=0
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

printf '==> Cleaning simulator STA1 build outputs\n'
clean_build_dir

if [[ "${clean_first}" -ne 0 ]]; then
  printf '==> Configuring sim:%s\n' "${sim_config}"
  distclean_tree
  configure_board "sim:${sim_config}"
else
  printf '==> Reusing current NuttX configuration\n'
fi

prepare_host_optional_romfs

printf '==> Building simulator STA1 image\n'
rm -f nuttx nuttx.map
make "-j${jobs}"

if [[ ! -x nuttx ]]; then
  echo "ERROR: simulator executable was not produced: nuttx" >&2
  exit 1
fi

cp nuttx "${sim_exe}"
chmod +x "${sim_exe}"

if [[ -f nuttx.map ]]; then
  cp nuttx.map "${sim_map}"
fi

printf '\n==> Simulator STA1 output\n'
printf '  config:     sim:%s\n' "${sim_config}"
printf '  file:       %s\n' "${sim_exe}"
printf '  size:       %s bytes\n' "$(wc -c < "${sim_exe}" | tr -d '[:space:]')"
printf '  run:        %s\n' "${sim_exe}"

if [[ -f "${sim_map}" ]]; then
  printf '\n  Linker map:\n'
  printf '    file:     %s\n' "${sim_map}"
fi
