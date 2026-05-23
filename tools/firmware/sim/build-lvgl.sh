#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-lvgl.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build NuttX simulator LVGL framebuffer demo, using sim:lvgl_fb by default.\n\n'
  printf 'Outputs:\n'
  printf '  build/sim-lvgl      host executable for NuttX sim:lvgl_fb/lvgl_lcd\n'
  printf '  build/sim-lvgl.map  linker map, when produced by the build\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N        Parallel make jobs (default: 8)\n'
  printf '      --fb            Build sim:lvgl_fb (default)\n'
  printf '      --lcd           Build sim:lvgl_lcd\n'
  printf '      --config NAME   Build a custom sim config, for example lvgl_fb\n'
  printf '      --lvgl-zip PATH Use a local LVGL vX.Y.Z.zip archive\n'
  printf '      --lvgl-source PATH\n'
  printf '                      Use a local LVGL git checkout containing vX.Y.Z tag\n'
  printf '      --lvgl-url URL  Add a preferred LVGL archive download URL\n'
  printf '      --download-timeout N\n'
  printf '                      Curl connect timeout for LVGL source download (default: 30)\n'
  printf '      --no-clean      Reuse the current NuttX tree configuration\n'
  printf '  -h, --help          Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"

cd "${feather_root}/nuttx"

build_dir="../build"
sim_exe="${build_dir}/sim-lvgl"
sim_map="${build_dir}/sim-lvgl.map"

jobs="${JOBS:-8}"
clean_first=1
sim_config="lvgl_fb"
lvgl_zip="${LVGL_ZIP:-}"
lvgl_source="${LVGL_SOURCE:-}"
lvgl_url="${LVGL_URL:-}"
download_timeout="${DOWNLOAD_TIMEOUT:-30}"

clean_build_dir()
{
  mkdir -p "${build_dir}"
  find "${build_dir}" -maxdepth 1 -type f -name 'sim-lvgl*' -delete
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

verify_lvgl_config()
{
  local missing=0

  for name in \
    CONFIG_GRAPHICS_LVGL \
    CONFIG_EXAMPLES_LVGLDEMO \
    CONFIG_LV_USE_NUTTX
  do
    if ! grep -q "^${name}=y" .config; then
      printf 'ERROR: %s is not enabled by sim:%s\n' "${name}" "${sim_config}" >&2
      missing=1
    fi
  done

  if [[ "${missing}" -ne 0 ]]; then
    exit 1
  fi
}

config_value()
{
  local name="$1"

  sed -n "s/^${name}=//p" .config | tail -n 1 | tr -d '"'
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
  printf '    source: %s\n' "${source_dir}"
  printf '    target: %s\n' "${target_dir}"

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
ERROR: curl is required to download LVGL ${version}, or provide a local archive:
  LVGL_ZIP=/path/to/v${version}.zip $0
  $0 --lvgl-zip /path/to/v${version}.zip
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
  printf '    file: %s\n' "${tarball}"

  for url in "${urls[@]}"; do
    printf '    url:  %s\n' "${url}"
    if curl -fL --connect-timeout "${download_timeout}" --retry 1 \
        -o "${tarball}" "${url}"; then
      verify_zip "${tarball}"
      return
    fi
    rm -f "${tarball}"
  done

  cat >&2 <<EOF
ERROR: failed to download LVGL ${version}.

The NuttX LVGL app requires v${version}.zip before build. If this host cannot
reach GitHub/codeload, download the archive elsewhere and rerun with:
  $0 --lvgl-zip /path/to/v${version}.zip

Expected archive URL:
  https://codeload.github.com/lvgl/lvgl/zip/refs/tags/v${version}
EOF
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -j|--jobs)
      jobs="$2"
      shift 2
      ;;
    --fb)
      sim_config="lvgl_fb"
      shift
      ;;
    --lcd)
      sim_config="lvgl_lcd"
      shift
      ;;
    --config)
      sim_config="$2"
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
    --download-timeout)
      download_timeout="$2"
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

printf '==> Cleaning simulator LVGL build outputs\n'
clean_build_dir

if [[ "${clean_first}" -ne 0 ]]; then
  printf '==> Configuring sim:%s\n' "${sim_config}"
  distclean_tree
  configure_board "sim:${sim_config}"
else
  printf '==> Reusing current NuttX configuration\n'
fi

prepare_host_optional_romfs
verify_lvgl_config
prepare_lvgl_source

printf '==> Building simulator LVGL image\n'
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

printf '\n==> Simulator LVGL output\n'
printf '  LVGL executable:\n'
printf '    config:     sim:%s\n' "${sim_config}"
printf '    file:       %s\n' "${sim_exe}"
printf '    size:       %s bytes\n' "$(wc -c < "${sim_exe}" | tr -d '[:space:]')"
printf '    run:        %s\n' "${sim_exe}"

if [[ -f "${sim_map}" ]]; then
  printf '\n  Linker map:\n'
  printf '    file:       %s\n' "${sim_map}"
fi
