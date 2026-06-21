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
  printf 'Build the NuttX simulator with FRender and the standalone WING GUI demo enabled.\n\n'
  printf 'The resulting simulator boots NSH, where frender_demo and wing_gui_demo are available as builtin commands.\n\n'
  printf 'Outputs:\n'
  printf '  build/sim-wing      host executable for NuttX sim:<config> + frender_demo + wing_gui_demo\n'
  printf '  build/sim-wing.map  linker map, when produced by the build\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N        Parallel make jobs (default: 8)\n'
  printf '      --config NAME   Simulator config to start from (default: nsh)\n'
  printf '      --nsh           Shortcut for --config nsh\n'
  printf '      --width N       WING GUI demo software surface width (default: 320)\n'
  printf '      --height N      WING GUI demo software surface height (default: 240)\n'
  printf '      --no-clean      Reuse current NuttX configuration instead of distclean/configure\n'
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
demo_width=320
demo_height=240

set_config_bool()
{
  local name="$1"
  local value="$2"

  if [[ "${value}" == "y" ]]; then
    if grep -q "^# ${name} is not set" .config; then
      sed -i "s/^# ${name} is not set/${name}=y/" .config
    elif grep -q "^${name}=" .config; then
      sed -i "s/^${name}=.*/${name}=y/" .config
    else
      printf '%s=y\n' "${name}" >> .config
    fi
  else
    if grep -q "^${name}=" .config; then
      sed -i "s/^${name}=.*/# ${name} is not set/" .config
    elif ! grep -q "^# ${name} is not set" .config; then
      printf '# %s is not set\n' "${name}" >> .config
    fi
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

clean_outputs()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'sim-wing*' -delete
}

clean_local_objects()
{
  local dir

  for dir in \
    "../apps/graphics/frender" \
    "../apps/graphics/wing" \
    "../apps/examples/frender_demo" \
    "../apps/examples/wing_gui_demo"
  do
    if [[ -d "${dir}" ]]; then
      find "${dir}" -maxdepth 3 -type f \( \
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

configure_sim()
{
  ./tools/configure.sh "sim:${sim_config}"
  make clean
}

prepare_host_options()
{
  if ! command -v genromfs >/dev/null 2>&1 && \
     grep -q '^CONFIG_ETC_ROMFS=y' .config; then
    printf '==> genromfs not found; disabling CONFIG_ETC_ROMFS for host sim build\n'
    set_config_bool CONFIG_ETC_ROMFS n
    make olddefconfig
  fi
}

enable_wing_gui_demo()
{
  printf '==> Enabling FRender + WING GUI demos for NSH\n'

  set_config_bool CONFIG_EXAMPLES_WINGDEMO n
  set_config_bool CONFIG_EXAMPLES_WING_DESKTOP_DEMO n

  set_config_bool CONFIG_DRIVERS_VIDEO y
  set_config_bool CONFIG_VIDEO_FB y
  set_config_bool CONFIG_FB_UPDATE y
  set_config_bool CONFIG_SIM_X11FB y
  set_config_bool CONFIG_SIM_FRAMEBUFFER y
  set_config_int CONFIG_SIM_FBWIDTH 640
  set_config_int CONFIG_SIM_FBHEIGHT 480
  set_config_int CONFIG_SIM_FBBPP 32

  set_config_bool CONFIG_GRAPHICS_FRENDER y
  set_config_bool CONFIG_GRAPHICS_FRENDER_FB_PRESENT y
  set_config_bool CONFIG_GRAPHICS_FRENDER_FB y
  set_config_bool CONFIG_EXAMPLES_FRENDER_DEMO y
  set_config_string CONFIG_EXAMPLES_FRENDER_DEMO_PROGNAME frender_demo
  set_config_int CONFIG_EXAMPLES_FRENDER_DEMO_PRIORITY 100
  set_config_int CONFIG_EXAMPLES_FRENDER_DEMO_STACKSIZE 4096
  set_config_int CONFIG_EXAMPLES_FRENDER_DEMO_WIDTH "${demo_width}"
  set_config_int CONFIG_EXAMPLES_FRENDER_DEMO_HEIGHT "${demo_height}"

  set_config_bool CONFIG_GRAPHICS_WING y
  set_config_int CONFIG_GRAPHICS_WING_INPUT_QUEUE_SIZE 64
  set_config_int CONFIG_GRAPHICS_WING_EVENT_QUEUE_SIZE 128
  set_config_int CONFIG_GRAPHICS_WING_TIMER_MAX 8
  set_config_int CONFIG_GRAPHICS_WING_ANIM_MAX 8
  set_config_bool CONFIG_EXAMPLES_WING_GUI_DEMO y
  set_config_string CONFIG_EXAMPLES_WING_GUI_DEMO_PROGNAME wing_gui_demo
  set_config_int CONFIG_EXAMPLES_WING_GUI_DEMO_PRIORITY 100
  set_config_int CONFIG_EXAMPLES_WING_GUI_DEMO_STACKSIZE 4096
  set_config_int CONFIG_EXAMPLES_WING_GUI_DEMO_WIDTH "${demo_width}"
  set_config_int CONFIG_EXAMPLES_WING_GUI_DEMO_HEIGHT "${demo_height}"

  set_config_bool CONFIG_BUILTIN y
  set_config_bool CONFIG_SYSTEM_NSH y
  set_config_bool CONFIG_NSH_BUILTIN_APPS y
  set_config_bool CONFIG_NSH_READLINE y
  set_config_string CONFIG_INIT_ENTRYPOINT nsh_main

  make olddefconfig
}

verify_config()
{
  local missing=0
  local name

  for name in \
    CONFIG_VIDEO_FB \
    CONFIG_SIM_FRAMEBUFFER \
    CONFIG_GRAPHICS_FRENDER \
    CONFIG_EXAMPLES_FRENDER_DEMO \
    CONFIG_GRAPHICS_WING \
    CONFIG_EXAMPLES_WING_GUI_DEMO \
    CONFIG_BUILTIN \
    CONFIG_SYSTEM_NSH \
    CONFIG_NSH_BUILTIN_APPS
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled for sim:%s\n' "${name}" "${sim_config}" >&2
      missing=1
    fi
  done

  if ! grep -q '^CONFIG_INIT_ENTRYPOINT="nsh_main"$' .config; then
    printf 'ERROR: CONFIG_INIT_ENTRYPOINT is not nsh_main\n' >&2
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
    --width)
      demo_width="$2"
      shift 2
      ;;
    --height)
      demo_height="$2"
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
      printf 'ERROR: unknown option: %s\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

printf '==> Cleaning simulator WING GUI build outputs\n'
clean_outputs
clean_local_objects

if [[ "${clean_first}" -ne 0 ]]; then
  printf '==> Configuring sim:%s\n' "${sim_config}"
  distclean_tree
  configure_sim
else
  printf '==> Reusing current NuttX configuration\n'
fi

prepare_host_options
enable_wing_gui_demo
verify_config

printf '==> Building simulator FRender/WING GUI image\n'
make "-j${jobs}"

if [[ ! -x nuttx ]]; then
  printf 'ERROR: simulator executable was not produced: nuttx\n' >&2
  exit 1
fi

cp nuttx "${sim_exe}"
chmod +x "${sim_exe}"

if [[ -f nuttx.map ]]; then
  cp nuttx.map "${sim_map}"
fi

printf '\n==> Simulator FRender/WING GUI output\n'
printf '  config:     sim:%s\n' "${sim_config}"
printf '  executable: %s\n' "${sim_exe}"
printf '  size:       %s bytes\n' "$(wc -c < "${sim_exe}" | tr -d '[:space:]')"
printf '  run:        printf "frender_demo\\nwing_gui_demo\\n" | %s\n' "${sim_exe}"
printf '  demos:      frender_demo, wing_gui_demo\n'
printf '  note:       frender_demo validates command list + software backend + optional framebuffer present\n'
printf '              wing_gui_demo validates WING GUI without WING Desktop\n'

if [[ -f "${sim_map}" ]]; then
  printf '  map:        %s\n' "${sim_map}"
fi
