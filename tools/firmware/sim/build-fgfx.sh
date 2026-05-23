#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-fgfx.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build NuttX simulator FGFX framebuffer demo using sim:fb by default.\n\n'
  printf 'Outputs:\n'
  printf '  build/sim-fgfx      host executable for NuttX sim:fb + FGFX demo\n'
  printf '  build/sim-fgfx.map  linker map, when produced by the build\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N        Parallel make jobs (default: 8)\n'
  printf '      --config NAME   Build a custom sim config, for example fb\n'
  printf '      --nsh           Build sim:nsh and enable the same framebuffer demo\n'
  printf '      --no-clean      Reuse the current NuttX tree configuration\n'
  printf '  -h, --help          Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
sim_exe="${build_dir}/sim-fgfx"
sim_map="${build_dir}/sim-fgfx.map"

jobs="${JOBS:-8}"
clean_first=1
sim_config="fb"

clean_build_dir()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'sim-fgfx*' -delete
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

enable_fgfx_fb_demo()
{
  printf '==> Enabling FGFX framebuffer demo\n'
  enable_config CONFIG_DRIVERS_VIDEO
  enable_config CONFIG_VIDEO_FB
  enable_config CONFIG_SIM_X11FB
  enable_config CONFIG_SIM_FRAMEBUFFER
  set_config_int CONFIG_SIM_FBWIDTH 640
  set_config_int CONFIG_SIM_FBHEIGHT 480
  set_config_int CONFIG_SIM_FBBPP 32
  enable_config CONFIG_GRAPHICS_FGFX
  enable_config CONFIG_EXAMPLES_FGFX_DEMO
  set_config_string CONFIG_INIT_ENTRYPOINT fgfxdemo_main
  disable_config CONFIG_INIT_ARGS
  make olddefconfig

  verify_fgfx_fb_demo_config
}

verify_fgfx_fb_demo_config()
{
  local missing=0

  for name in \
    CONFIG_DRIVERS_VIDEO \
    CONFIG_VIDEO_FB \
    CONFIG_SIM_X11FB \
    CONFIG_SIM_FRAMEBUFFER \
    CONFIG_GRAPHICS_FGFX \
    CONFIG_EXAMPLES_FGFX_DEMO
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled by sim:%s\n' \
        "${name}" "${sim_config}" >&2
      missing=1
    fi
  done

  if ! grep -q '^CONFIG_INIT_ENTRYPOINT="fgfxdemo_main"$' .config; then
    printf 'ERROR: CONFIG_INIT_ENTRYPOINT is not fgfxdemo_main\n' >&2
    missing=1
  fi

  if [[ "${missing}" -ne 0 ]]; then
    exit 1
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -j|--jobs)
      jobs="$2"
      shift 2
      ;;
    --config)
      sim_config="$2"
      shift 2
      ;;
    --nsh)
      sim_config="nsh"
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

printf '==> Cleaning simulator FGFX build outputs\n'
clean_build_dir

if [[ "${clean_first}" -ne 0 ]]; then
  printf '==> Configuring sim:%s\n' "${sim_config}"
  distclean_tree
  configure_board "sim:${sim_config}"
else
  printf '==> Reusing current NuttX configuration\n'
fi

prepare_host_optional_romfs
enable_fgfx_fb_demo

printf '==> Building simulator FGFX image\n'
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

printf '\n==> Simulator FGFX output\n'
printf '  FGFX framebuffer demo executable:\n'
printf '    config:     sim:%s\n' "${sim_config}"
printf '    file:       %s\n' "${sim_exe}"
printf '    size:       %s bytes\n' "$(wc -c < "${sim_exe}" | tr -d '[:space:]')"
printf '    run:        %s\n' "${sim_exe}"

if [[ -f "${sim_map}" ]]; then
  printf '\n  Linker map:\n'
  printf '    file:       %s\n' "${sim_map}"
fi
