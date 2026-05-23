#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-wing.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build NuttX simulator Wing widget demo using sim:nsh by default.\n\n'
  printf 'Outputs:\n'
  printf '  build/sim-wing      host executable for NuttX sim:nsh + wingdemo\n'
  printf '  build/sim-wing.map  linker map, when produced by the build\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N        Parallel make jobs (default: 8)\n'
  printf '      --config NAME   Build a custom sim config, for example fb\n'
  printf '      --nsh           Build sim:nsh and enable the wingdemo command\n'
  printf '      --frames N      Print a wingdemo command that renders N frames\n'
  printf '      --no-clean      Reuse the current NuttX tree configuration\n'
  printf '  -h, --help          Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
sim_exe="${build_dir}/sim-wing"
sim_map="${build_dir}/sim-wing.map"

jobs="${JOBS:-8}"
clean_first=1
sim_config="nsh"
frames=""

clean_build_dir()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'sim-wing*' -delete
}

clean_wing_objects()
{
  local dir

  for dir in \
    "../apps/graphics/wing" \
    "../apps/examples/wingdemo"
  do
    if [[ -d "${dir}" ]]; then
      find "${dir}" -maxdepth 1 -type f \( \
        -name '*.o' -o \
        -name '*.d' -o \
        -name '*.gcda' -o \
        -name '*.gcno' -o \
        -name '.depend' -o \
        -name 'Make.dep' -o \
        -name '.built' \
      \) -delete
    fi
  done
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

enable_wing_fb_demo()
{
  printf '==> Enabling Wing widget demo as an NSH command\n'

  disable_config CONFIG_GRAPHICS_LVGL
  disable_config CONFIG_EXAMPLES_LVGLDEMO

  enable_config CONFIG_DRIVERS_VIDEO
  enable_config CONFIG_VIDEO_FB
  enable_config CONFIG_FB_UPDATE
  enable_config CONFIG_SIM_X11FB
  enable_config CONFIG_SIM_FRAMEBUFFER
  set_config_int CONFIG_SIM_FBWIDTH 640
  set_config_int CONFIG_SIM_FBHEIGHT 480
  set_config_int CONFIG_SIM_FBBPP 32
  disable_config CONFIG_BOARD_LATE_INITIALIZE
  enable_config CONFIG_BOARDCTL
  enable_config CONFIG_BOARDCTL_POWEROFF

  enable_config CONFIG_GRAPHICS_WING
  enable_config CONFIG_EXAMPLES_WINGDEMO
  enable_config CONFIG_BUILTIN
  enable_config CONFIG_SYSTEM_NSH
  enable_config CONFIG_NSH_BUILTIN_APPS
  enable_config CONFIG_NSH_READLINE
  set_config_int CONFIG_INIT_STACKSIZE 131072
  set_config_int CONFIG_SYSTEM_NSH_STACKSIZE 131072
  set_config_int CONFIG_EXAMPLES_WINGDEMO_STACKSIZE 131072
  set_config_string CONFIG_INIT_ENTRYPOINT nsh_main
  set_config_string CONFIG_INIT_ARGS ""

  set_config_int CONFIG_EXAMPLES_WINGDEMO_FRAMES 0

  make olddefconfig
  verify_wing_fb_demo_config
}

verify_wing_fb_demo_config()
{
  local missing=0
  local name

  for name in \
    CONFIG_DRIVERS_VIDEO \
    CONFIG_VIDEO_FB \
    CONFIG_SIM_X11FB \
    CONFIG_SIM_FRAMEBUFFER \
    CONFIG_GRAPHICS_WING \
    CONFIG_EXAMPLES_WINGDEMO \
    CONFIG_BUILTIN \
    CONFIG_SYSTEM_NSH \
    CONFIG_NSH_BUILTIN_APPS
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled by sim:%s\n' \
        "${name}" "${sim_config}" >&2
      missing=1
    fi
  done

  if grep -q '^CONFIG_GRAPHICS_LVGL=y' .config; then
    printf 'ERROR: CONFIG_GRAPHICS_LVGL must be disabled for Wing\n' >&2
    missing=1
  fi

  if ! grep -q '^CONFIG_INIT_ENTRYPOINT="nsh_main"$' .config; then
    printf 'ERROR: CONFIG_INIT_ENTRYPOINT is not nsh_main\n' >&2
    missing=1
  fi

  if grep -q '^CONFIG_BOARD_LATE_INITIALIZE=y' .config; then
    printf 'ERROR: CONFIG_BOARD_LATE_INITIALIZE must be disabled so wingdemo opens the X11 framebuffer lazily\n' >&2
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
    --frames)
      frames="$2"
      shift 2
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

printf '==> Cleaning simulator Wing build outputs\n'
clean_build_dir
clean_wing_objects

if [[ "${clean_first}" -ne 0 ]]; then
  printf '==> Configuring sim:%s\n' "${sim_config}"
  distclean_tree
  configure_board "sim:${sim_config}"
else
  printf '==> Reusing current NuttX configuration\n'
fi

prepare_host_optional_romfs
enable_wing_fb_demo

printf '==> Building simulator Wing image\n'
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

printf '\n==> Simulator Wing output\n'
printf '  Wing NSH widget demo executable:\n'
printf '    config:     sim:%s\n' "${sim_config}"
printf '    file:       %s\n' "${sim_exe}"
printf '    size:       %s bytes\n' "$(wc -c < "${sim_exe}" | tr -d '[:space:]')"
if [[ -n "${frames}" ]]; then
  printf '    run:        printf "wingdemo --frames %s\\\\n" | %s\n' \
    "${frames}" "${sim_exe}"
else
  printf '    run:        printf "wingdemo\\\\n" | %s\n' "${sim_exe}"
fi

if [[ -f "${sim_map}" ]]; then
  printf '\n  Linker map:\n'
  printf '    file:       %s\n' "${sim_map}"
fi
