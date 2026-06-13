#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/validate-wing-phase1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Validate the WING GUI Phase 1 runtime closure on the NuttX simulator.\n\n'
  printf 'The validation builds sim-wing by default, starts NSH, runs wing_gui_demo,\n'
  printf 'checks the runtime/dirty/render/object-space logs, sends an X11 close\n'
  printf 'request, and verifies that the demo exits back to NSH.\n\n'
  printf 'Options:\n'
  printf '      --no-build      Reuse existing FeatherCore/build/sim-wing\n'
  printf '      --keep-log      Keep the temporary validation log\n'
  printf '  -h, --help          Show this help\n'
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
feather_root="$(cd "${script_dir}/../../.." && pwd)"
sim_exe="${feather_root}/build/sim-wing"
build_script="${script_dir}/build-wing.sh"
x11_input="${script_dir}/x11-input.sh"

build_first=1
keep_log=0
sim_pid=""
tmpdir=""
log_file=""
fifo=""

cleanup()
{
  if [[ -n "${sim_pid}" ]] && kill -0 "${sim_pid}" 2>/dev/null; then
    kill "${sim_pid}" 2>/dev/null || true
    wait "${sim_pid}" 2>/dev/null || true
  fi

  if [[ -n "${tmpdir}" && -d "${tmpdir}" && "${keep_log}" -eq 0 ]]; then
    rm -rf "${tmpdir}"
  elif [[ -n "${log_file}" && -f "${log_file}" ]]; then
    printf '==> WING Phase 1 validation log kept at %s\n' "${log_file}"
  fi
}

fail()
{
  printf 'ERROR: %s\n' "$*" >&2
  if [[ -n "${log_file}" && -f "${log_file}" ]]; then
    printf '==> Last 120 log lines:\n' >&2
    tail -n 120 "${log_file}" >&2 || true
  fi
  exit 1
}

wait_for_log()
{
  local pattern="$1"
  local timeout_s="$2"
  local elapsed=0

  while [[ "${elapsed}" -lt "${timeout_s}" ]]; do
    if grep -Fq "${pattern}" "${log_file}" 2>/dev/null; then
      printf '==> observed: %s\n' "${pattern}"
      return 0
    fi

    if [[ -n "${sim_pid}" ]] && ! kill -0 "${sim_pid}" 2>/dev/null; then
      fail "sim-wing exited before observing: ${pattern}"
    fi

    sleep 1
    elapsed=$((elapsed + 1))
  done

  fail "timed out waiting for: ${pattern}"
}

log_size()
{
  wc -c <"${log_file}" | tr -d ' '
}

wait_for_new_log()
{
  local pattern="$1"
  local timeout_s="$2"
  local start_offset="$3"
  local elapsed=0

  while [[ "${elapsed}" -lt "${timeout_s}" ]]; do
    if tail -c +"$((start_offset + 1))" "${log_file}" 2>/dev/null |
       grep -Fq "${pattern}"; then
      printf '==> observed after input: %s\n' "${pattern}"
      return 0
    fi

    if [[ -n "${sim_pid}" ]] && ! kill -0 "${sim_pid}" 2>/dev/null; then
      fail "sim-wing exited before observing after input: ${pattern}"
    fi

    sleep 1
    elapsed=$((elapsed + 1))
  done

  fail "timed out waiting after input for: ${pattern}"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --no-build)
      build_first=0
      shift
      ;;
    --keep-log)
      keep_log=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      fail "unknown option: $1"
      ;;
  esac
done

trap cleanup EXIT

tmpdir="$(mktemp -d)"
log_file="${tmpdir}/wing-phase1.log"
fifo="${tmpdir}/sim-wing.stdin"
mkfifo "${fifo}"
: > "${log_file}"

if [[ "${build_first}" -ne 0 ]]; then
  printf '==> Building sim-wing for WING Phase 1 validation\n'
  "${build_script}" --no-clean
fi

if [[ ! -x "${sim_exe}" ]]; then
  fail "sim executable not found: ${sim_exe}"
fi

printf '==> Starting sim-wing\n'
"${sim_exe}" <"${fifo}" > >(tee "${log_file}") 2>&1 &
sim_pid=$!
exec 3>"${fifo}"

wait_for_log 'NuttShell (NSH)' 20

printf '==> Running wing_gui_demo from NSH\n'
printf 'wing_gui_demo\n' >&3

wait_for_log 'wing_gui_demo: === runtime capability summary ===' 30
wait_for_log 'wing_gui_demo: framebuffer ' 30
wait_for_log 'wing_gui_demo: input provider registered events=1 source=x11-window+demo-script' 30
wait_for_log 'wing_gui_demo: dynamic space demo layout uses explicit control z-layers' 30
wait_for_log 'wing_gui_demo: core space treats 2D widgets as identity space transform objects by default' 30
wait_for_log 'wing_gui_demo: triangle primitive object uses runtime camera + core space projection -> WING render node/material seed -> FRender fill_triangle command seed' 30
wait_for_log 'wing_gui_demo: progress/slider/scrollbar use explicit z-layers' 30
wait_for_log 'wing_gui_demo: continuous pulse animation scheduled property=line_secondary_width' 30
wait_for_log 'wing_gui_demo: dirty before handler' 30
wait_for_log 'wing_gui_demo: redraw chunks this frame count=' 30
wait_for_log 'wing_gui_demo: present rect list count=' 30
wait_for_log 'wing_gui_demo: continuous pulse animation segment=1 completed' 60

printf '==> Sending X11 click input\n'
"${x11_input}" NuttX click 50 86 1
wait_for_log 'wing_gui_demo: x11 input provider emitted pending type=pointer_down source=mouse' 20

printf '==> Sending X11 slider drag while animation is running\n'
drag_log_start="$(log_size)"
"${x11_input}" NuttX drag 44 196 520 196 1 10
wait_for_new_log 'type=pointer_move pressed=yes point=520,196' 20 "${drag_log_start}"
wait_for_new_log 'wing_gui_demo: wing_slider value changed' 20 "${drag_log_start}"

printf '==> Sending X11 scrollbar drag while animation is running\n'
drag_log_start="$(log_size)"
"${x11_input}" NuttX drag 40 226 520 226 1 10
wait_for_new_log 'type=pointer_move pressed=yes point=520,226' 20 "${drag_log_start}"
wait_for_new_log 'wing_gui_demo: wing_scrollbar value changed' 20 "${drag_log_start}"

printf '==> Sending X11 close request\n'
"${x11_input}" NuttX close

wait_for_log 'wing_gui_demo: framebuffer window closed' 20
wait_for_log 'wing_gui_demo: root received close request through WING input/event queue' 20
wait_for_log 'wing_gui_demo: app task exit' 20

printf '==> Stopping sim-wing\n'
printf 'poweroff\n' >&3
wait "${sim_pid}" || true
sim_pid=""

printf '==> WING GUI Phase 1 runtime closure validation passed\n'
