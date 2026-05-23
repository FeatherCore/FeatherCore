#!/usr/bin/env bash
############################################################################
# tools/firmware/stm32n6570-dk/build-psram-verify.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

usage()
{
  printf 'Usage: %s [OPTIONS]\n\n' "$0"
  printf 'Build STM32N6570-DK NXboot and a protected KNSh PSRAM validation app.\n\n'
  printf 'The validation app starts from stm32n6570-dk:knsh, then enables\n'
  printf 'diagnostics for PSRAM heap and thread-stack validation: psramstack,\n'
  printf 'sysresume, ostest, ramtest, memstress, stack coloration, stack monitor, dumpstack,\n'
  printf 'backtrace, and ARMv8-M hardware stack-limit checking.\n\n'
  printf 'Outputs are written to:\n\n'
  printf '  build/stm32n6570-dk-nxboot.bin\n'
  printf '      Standard trusted NXboot image. Program at XSPI2 NOR 0x70000000.\n\n'
  printf '  build/stm32n6570-dk-psram-verify.bin\n'
  printf '      Final protected validation app image. Program at XSPI2 NOR 0x70100000.\n\n'
  printf '  build/stm32n6570-dk-psram-verify-full.bin\n'
  printf '      Combined XSPI2 NOR image. Program at 0x70000000.\n\n'
  printf 'Options:\n'
  printf '  -j, --jobs N              Parallel make jobs (default: 8)\n'
  printf '  -v, --version VERSION     App semantic version (default: 0.1.0)\n'
  printf '      --signing-tool PATH   STM32_SigningTool_CLI path\n'
  printf '      --app-only            Build only the validation app image\n'
  printf '      --psram-heap-offset X PSRAM heap offset from 0x90000000 (default: 0x0)\n'
  printf '      --psram-mpu-policy P  PSRAM MPU policy: outer-wb, no-wb, no-wb-nwa,\n'
  printf '                            no-wt, no-wt-nwa, outer-wt, no-nc, outer-nc\n'
  printf '                            (default: no-wb-nwa)\n'
  printf '      --bootstrap-uheap-size X\n'
  printf '                            Internal user bootstrap heap size (default: 0x10000)\n'
  printf '      --no-fpu              Build a no-FPU A/B image with -nofpu app names\n'
  printf '      --lazy-fpu            Build a lazy-FPU A/B image with -lazyfpu app names\n'
  printf '      --psram-selftest-debug\n'
  printf '                            Enable verbose XSPI1 PSRAM self-test logs\n'
  printf '      --psram-selftest-noflush\n'
  printf '                            Skip PSRAM self-test cache maintenance\n'
  printf '      --syscall-barrier     Enable ARMv8-M SVC return DSB/ISB diagnostic\n'
  printf '      --basepri-isb         Emit ISB after Armv8-M BASEPRI writes\n'
  printf '      --syscall-current-frame\n'
  printf '                            Preserve active SVC FP frame shape on syscall return\n'
  printf '      --syscall-user-basepri0\n'
  printf '                            Clear BASEPRI when returning from syscall to user\n'
  printf '      --syscall-dispatch-basepri0\n'
  printf '                            Clear BASEPRI when entering syscall dispatcher\n'
  printf '      --syscall-kstack      Run protected syscall dispatcher on kernel stack\n'
  printf '      --syscall-kstack-size X\n'
  printf '                            Protected syscall kernel stack size (default: 4096)\n'
  printf '      --syscall-kstack-basic\n'
  printf '                            Use a basic FP frame for syscall-kstack dispatch\n'
  printf '      --syscall-kstack-psp\n'
  printf '                            Run syscall-kstack dispatch through PSP\n'
  printf '      --usart1-txpoll      Drain USART1 TX output by polling\n'
  printf '      --usart1-highpri     Run USART1 IRQ at high priority\n'
  printf '      --ostest-nowait       Do not wait for the spawned ostest child task\n'
  printf '      --ostest-delay-usec X Override ostest short delay in microseconds\n'
  printf '      --ostest-startup-trace\n'
  printf '                            Trace user_main startup checkpoints\n'
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
legacy_psram_nxboot_image_bin="${build_dir}/stm32n6570-dk-psram-verify-nxboot.bin"
app_image_bin="${build_dir}/stm32n6570-dk-psram-verify.bin"
full_image_bin="${build_dir}/stm32n6570-dk-psram-verify-full.bin"

jobs="${JOBS:-8}"
version="0.1.0"
signing_tool="${STM32_SIGNING_TOOL:-${STM32_SIGNINGTOOL_CLI:-}}"
app_only=0
psram_heap_offset="0x0"

# Default to the policy validated for normal STM32N6 CPU-owned PSRAM heap and
# stacks.  The older outer-wb policy is still available as an A/B baseline, but
# M55 D-cache treats Shareable Normal data as non-cacheable.

psram_mpu_policy="no-wb-nwa"
bootstrap_uheap_size="0x10000"
no_fpu=0
lazy_fpu=0
psram_selftest_debug=0
psram_selftest_noflush=0
syscall_barrier=0
basepri_isb=0
syscall_current_frame=0
syscall_user_basepri0=0
syscall_dispatch_basepri0=0
syscall_kstack=0
syscall_kstack_size=4096
syscall_kstack_basic=0
syscall_kstack_psp=0
usart1_txpoll=0
usart1_highpri=0
ostest_nowait=0
ostest_delay_usec=""
ostest_startup_trace=0
header_size=""
identifier=""
userspace=""
full_image_created=0
tmp_files=()
app_name_suffix=""

cleanup()
{
  local tmp

  for tmp in "${tmp_files[@]}"; do
    rm -f "${tmp}"
  done
}

trap cleanup EXIT

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

config_state()
{
  local name="$1"

  if grep -q "^${name}=y" .config; then
    printf 'enabled'
  else
    printf 'disabled'
  fi
}

clean_build_dir()
{
  mkdir -p "${build_dir}"
  rm -f "${app_image_bin}" "${full_image_bin}" "${legacy_psram_nxboot_image_bin}"

  if [[ "${app_only}" -eq 0 ]]; then
    rm -f "${nxboot_image_bin}"
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

set_config()
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

enable_config()
{
  set_config "$1" y
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
  if [[ "${app_only}" -ne 0 ]]; then
    return
  fi

  if [[ -z "${signing_tool}" ]] &&
     command -v STM32_SigningTool_CLI >/dev/null 2>&1; then
    signing_tool="$(command -v STM32_SigningTool_CLI)"
  fi

  if [[ -z "${signing_tool}" ]]; then
    find_signing_tool || true
  fi

  if [[ -z "${signing_tool}" ]]; then
    echo "ERROR: STM32_SigningTool_CLI is required to build NXboot." >&2
    echo "ERROR: pass --signing-tool, set STM32_SIGNING_TOOL, or use --app-only." >&2
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

require_enabled()
{
  local name="$1"

  if ! grep -q "^${name}=y" .config; then
    printf 'ERROR: validation config did not enable %s\n' "${name}" >&2
    exit 1
  fi
}

apply_validation_config()
{
  printf '==> Enabling STM32N6570-DK PSRAM validation diagnostics\n'

  enable_config CONFIG_DEBUG_FEATURES
  enable_config CONFIG_ARCH_STACKDUMP
  enable_config CONFIG_DEBUG_HARDFAULT_ALERT
  enable_config CONFIG_DEBUG_MEMFAULT
  enable_config CONFIG_DEBUG_BUSFAULT
  enable_config CONFIG_DEBUG_USAGEFAULT

  disable_config CONFIG_ARMV8M_STACKCHECK_NONE
  disable_config CONFIG_ARMV8M_STACKCHECK
  enable_config CONFIG_ARMV8M_STACKCHECK_HARDWARE
  enable_config CONFIG_STACK_COLORATION
  set_config CONFIG_IDLETHREAD_STACKSIZE 4096
  enable_config CONFIG_SCHED_BACKTRACE
  enable_config CONFIG_SYSTEM_DUMPSTACK
  set_config CONFIG_SYSTEM_DUMPSTACK_STACKSIZE 4096
  set_config CONFIG_SYSTEM_DUMPSTACK_PRIORITY 100
  enable_config CONFIG_SYSTEM_STACKMONITOR
  set_config CONFIG_SYSTEM_STACKMONITOR_STACKSIZE 4096
  set_config CONFIG_SYSTEM_STACKMONITOR_PRIORITY 50
  set_config CONFIG_SYSTEM_STACKMONITOR_INTERVAL 1

  enable_config CONFIG_STM32N6_PSRAM_HEAP
  set_config CONFIG_STM32N6_PSRAM_HEAP_OFFSET "${psram_heap_offset}"
  set_config CONFIG_STM32N6_PROTECTED_UHEAP_SIZE "${bootstrap_uheap_size}"
  set_config CONFIG_MM_REGIONS 2

  if [[ "${psram_selftest_debug}" -ne 0 ]]; then
    enable_config CONFIG_STM32N6570_DK_PSRAM_SELFTEST_DEBUG
  else
    disable_config CONFIG_STM32N6570_DK_PSRAM_SELFTEST_DEBUG
  fi

  if [[ "${psram_selftest_noflush}" -ne 0 ]]; then
    enable_config CONFIG_STM32N6570_DK_PSRAM_SELFTEST_NOFLUSH
  else
    disable_config CONFIG_STM32N6570_DK_PSRAM_SELFTEST_NOFLUSH
  fi

  if [[ "${syscall_barrier}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_BARRIER
  else
    disable_config CONFIG_ARMV8M_SYSCALL_BARRIER
  fi

  if [[ "${basepri_isb}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_BASEPRI_ISB
  else
    disable_config CONFIG_ARMV8M_BASEPRI_ISB
  fi

  if [[ "${syscall_current_frame}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_RETURN_CURRENT_FRAME
  else
    disable_config CONFIG_ARMV8M_SYSCALL_RETURN_CURRENT_FRAME
  fi

  if [[ "${syscall_user_basepri0}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_RETURN_USER_BASEPRI0
  else
    disable_config CONFIG_ARMV8M_SYSCALL_RETURN_USER_BASEPRI0
  fi

  if [[ "${syscall_dispatch_basepri0}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_DISPATCH_BASEPRI0
  else
    disable_config CONFIG_ARMV8M_SYSCALL_DISPATCH_BASEPRI0
  fi

  if [[ "${syscall_kstack}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACK
    set_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACKSIZE "${syscall_kstack_size}"
  else
    disable_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACK
  fi

  if [[ "${syscall_kstack_basic}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME
  else
    disable_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME
  fi

  if [[ "${syscall_kstack_psp}" -ne 0 ]]; then
    enable_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP
  else
    disable_config CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP
  fi

  if [[ "${usart1_txpoll}" -ne 0 ]]; then
    enable_config CONFIG_STM32N6_USART1_TX_POLL_DRAIN
  else
    disable_config CONFIG_STM32N6_USART1_TX_POLL_DRAIN
  fi

  if [[ "${usart1_highpri}" -ne 0 ]]; then
    enable_config CONFIG_STM32N6_USART1_HIGH_PRIORITY
  else
    disable_config CONFIG_STM32N6_USART1_HIGH_PRIORITY
  fi

  if [[ "${no_fpu}" -ne 0 ]]; then
    disable_config CONFIG_ARCH_FPU
    disable_config CONFIG_ARMV8M_LAZYFPU
    enable_config CONFIG_TESTING_OSTEST_FPUTESTDISABLE
  elif [[ "${lazy_fpu}" -ne 0 ]]; then
    enable_config CONFIG_ARCH_FPU
    enable_config CONFIG_ARMV8M_LAZYFPU
    disable_config CONFIG_TESTING_OSTEST_FPUTESTDISABLE
  else
    disable_config CONFIG_ARMV8M_LAZYFPU
  fi

  case "${psram_mpu_policy}" in
    outer-wb)
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      enable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      disable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      disable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    no-wb)
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      enable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      disable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      disable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    no-wb-nwa)
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      enable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      disable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      enable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    no-wt)
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      enable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      disable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      disable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    no-wt-nwa)
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      enable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      disable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      enable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    outer-wt)
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      enable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      disable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      disable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    no-nc)
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      enable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      disable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    outer-nc)
      enable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_OUTER
      disable_config CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK
      disable_config CONFIG_STM32N6_PSRAM_MPU_WRITE_THROUGH
      enable_config CONFIG_STM32N6_PSRAM_MPU_NONCACHEABLE
      disable_config CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE
      ;;
    *)
      printf 'ERROR: unsupported PSRAM MPU policy: %s\n' \
        "${psram_mpu_policy}" >&2
      exit 2
      ;;
  esac

  enable_config CONFIG_TESTING_RAMTEST
  set_config CONFIG_TESTING_RAMTEST_STACKSIZE 4096
  enable_config CONFIG_TESTING_MEMORY_STRESS
  set_config CONFIG_TESTING_MEMORY_STRESS_STACKSIZE 8192
  enable_config CONFIG_TESTING_PSRAM_STACK
  set_config CONFIG_TESTING_PSRAM_STACK_STACKSIZE 8192
  enable_config CONFIG_TESTING_SYSCALL_RESUME
  set_config CONFIG_TESTING_SYSCALL_RESUME_STACKSIZE 8192
  set_config CONFIG_TESTING_SYSCALL_RESUME_DELAY_USEC 5000000

  enable_config CONFIG_TESTING_OSTEST
  set_config CONFIG_TESTING_OSTEST_LOOPS 1
  set_config CONFIG_TESTING_OSTEST_STACKSIZE 16384
  set_config CONFIG_TESTING_OSTEST_DELAY_USEC "${ostest_delay_usec:-500000}"
  set_config CONFIG_TESTING_OSTEST_NBARRIER_THREADS 8
  set_config CONFIG_TESTING_OSTEST_FPUSTACKSIZE 8192
  if [[ "${ostest_startup_trace}" -ne 0 ]]; then
    enable_config CONFIG_TESTING_OSTEST_STARTUP_TRACE
  else
    disable_config CONFIG_TESTING_OSTEST_STARTUP_TRACE
  fi

  if [[ "${ostest_nowait}" -ne 0 ]]; then
    disable_config CONFIG_TESTING_OSTEST_WAITRESULT
  else
    enable_config CONFIG_TESTING_OSTEST_WAITRESULT
  fi

  make olddefconfig

  require_enabled CONFIG_STM32N6_PSRAM_HEAP
  require_enabled CONFIG_ARMV8M_STACKCHECK_HARDWARE
  require_enabled CONFIG_STACK_COLORATION
  require_enabled CONFIG_SYSTEM_STACKMONITOR
  require_enabled CONFIG_SYSTEM_DUMPSTACK
  require_enabled CONFIG_TESTING_RAMTEST
  require_enabled CONFIG_TESTING_MEMORY_STRESS
  require_enabled CONFIG_TESTING_PSRAM_STACK
  require_enabled CONFIG_TESTING_SYSCALL_RESUME
  require_enabled CONFIG_TESTING_OSTEST
}

load_image_config()
{
  if [[ -z "${header_size}" ]]; then
    header_size="$(config_value CONFIG_NXBOOT_HEADER_SIZE)"
    header_size="${header_size:-0x400}"
  fi

  if [[ -z "${identifier}" ]]; then
    identifier="$(config_value CONFIG_NXBOOT_PLATFORM_IDENTIFIER)"
    identifier="${identifier:-0x4e363537}"
  fi

  if [[ -z "${userspace}" ]]; then
    userspace="$(config_value CONFIG_NUTTX_USERSPACE)"
    userspace="${userspace:-0x70180400}"
  fi
}

create_protected_payload()
{
  local kernel_input="$1"
  local user_input="$2"
  local output="$3"

  load_image_config

  if [[ ! -f "${kernel_input}" ]]; then
    echo "ERROR: kernel binary not found: ${kernel_input}" >&2
    exit 1
  fi

  if [[ ! -f "${user_input}" ]]; then
    echo "ERROR: user binary not found: ${user_input}" >&2
    exit 1
  fi

  python3 - "${kernel_input}" "${user_input}" "${output}" \
    "${header_size}" "${userspace}" <<'PY'
import io
import os
import sys

kernel_path, user_path, output_path, header_size, userspace = sys.argv[1:6]
slot_base = 0x70100000
header_size = int(header_size, 0)
userspace = int(userspace, 0)
kernel_base = slot_base + header_size
kernel_window = userspace - kernel_base

if kernel_window <= 0:
    raise SystemExit("ERROR: CONFIG_NUTTX_USERSPACE must be above the "
                     "kernel vector address")

kernel_size = os.stat(kernel_path).st_size
if kernel_size > kernel_window:
    raise SystemExit(
        f"ERROR: kernel binary is {kernel_size} bytes but the protected "
        f"kernel window is only {kernel_window} bytes"
    )

with open(output_path, "wb") as output:
    with open(kernel_path, "rb") as kernel:
        while True:
            data = kernel.read(io.DEFAULT_BUFFER_SIZE)
            if not data:
                break
            output.write(data)

    output.write(b"\xff" * (kernel_window - kernel_size))

    with open(user_path, "rb") as user:
        while True:
            data = user.read(io.DEFAULT_BUFFER_SIZE)
            if not data:
                break
            output.write(data)
PY
}

kernel_window_size()
{
  load_image_config

  python3 - "${header_size}" "${userspace}" <<'PY'
import sys

header_size, userspace = sys.argv[1:3]
slot_base = 0x70100000
print(int(userspace, 0) - (slot_base + int(header_size, 0)))
PY
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
    --app-only)
      app_only=1
      shift
      ;;
    --psram-heap-offset)
      psram_heap_offset="$2"
      shift 2
      ;;
    --psram-mpu-policy)
      psram_mpu_policy="$2"
      shift 2
      ;;
    --bootstrap-uheap-size)
      bootstrap_uheap_size="$2"
      shift 2
      ;;
    --no-fpu)
      no_fpu=1
      shift
      ;;
    --lazy-fpu)
      lazy_fpu=1
      shift
      ;;
    --psram-selftest-debug)
      psram_selftest_debug=1
      shift
      ;;
    --psram-selftest-noflush)
      psram_selftest_debug=1
      psram_selftest_noflush=1
      shift
      ;;
    --syscall-barrier)
      syscall_barrier=1
      shift
      ;;
    --basepri-isb)
      basepri_isb=1
      shift
      ;;
    --syscall-current-frame)
      syscall_current_frame=1
      shift
      ;;
    --syscall-user-basepri0)
      syscall_user_basepri0=1
      shift
      ;;
    --syscall-dispatch-basepri0)
      syscall_dispatch_basepri0=1
      shift
      ;;
    --syscall-kstack)
      syscall_kstack=1
      shift
      ;;
    --syscall-kstack-size)
      syscall_kstack=1
      syscall_kstack_size="$2"
      shift 2
      ;;
    --syscall-kstack-basic)
      syscall_kstack=1
      syscall_kstack_basic=1
      shift
      ;;
    --syscall-kstack-psp)
      syscall_kstack=1
      syscall_kstack_psp=1
      shift
      ;;
    --usart1-txpoll)
      usart1_txpoll=1
      shift
      ;;
    --usart1-highpri)
      usart1_highpri=1
      shift
      ;;
    --ostest-nowait)
      ostest_nowait=1
      shift
      ;;
    --ostest-delay-usec)
      ostest_delay_usec="$2"
      shift 2
      ;;
    --ostest-startup-trace)
      ostest_startup_trace=1
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

if [[ "${no_fpu}" -ne 0 && "${lazy_fpu}" -ne 0 ]]; then
  echo "ERROR: --no-fpu and --lazy-fpu are mutually exclusive" >&2
  exit 2
fi

if [[ "${no_fpu}" -ne 0 ]]; then
  app_name_suffix="-nofpu"
elif [[ "${lazy_fpu}" -ne 0 ]]; then
  app_name_suffix="-lazyfpu"
fi

if [[ "${psram_mpu_policy}" != "no-wb-nwa" ]]; then
  app_name_suffix="${app_name_suffix}-${psram_mpu_policy}"
fi

if [[ "${bootstrap_uheap_size}" != "0x10000" ]]; then
  sanitized="${bootstrap_uheap_size//[^[:alnum:]]/}"
  app_name_suffix="${app_name_suffix}-uheap${sanitized}"
fi

if [[ "${psram_heap_offset}" != "0x0" ]]; then
  sanitized="${psram_heap_offset//[^[:alnum:]]/}"
  app_name_suffix="${app_name_suffix}-psramoff${sanitized}"
fi

if [[ "${psram_selftest_debug}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-selftestdbg"
fi

if [[ "${psram_selftest_noflush}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-noflush"
fi

if [[ "${syscall_barrier}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-svcbar"
fi

if [[ "${basepri_isb}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-basepriisb"
fi

if [[ "${syscall_current_frame}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-curframe"
fi

if [[ "${syscall_user_basepri0}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-basepri0"
fi

if [[ "${syscall_dispatch_basepri0}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-dispbasepri0"
fi

if [[ "${syscall_kstack}" -ne 0 ]]; then
  sanitized="${syscall_kstack_size//[^[:alnum:]]/}"
  app_name_suffix="${app_name_suffix}-svckstack${sanitized}"
fi

if [[ "${syscall_kstack_basic}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-svckbasic"
fi

if [[ "${syscall_kstack_psp}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-svckpsp"
fi

if [[ "${usart1_txpoll}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-txpoll"
fi

if [[ "${usart1_highpri}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-uarthipri"
fi

if [[ "${ostest_nowait}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-ostnowait"
fi

if [[ -n "${ostest_delay_usec}" && "${ostest_delay_usec}" != "500000" ]]; then
  sanitized="${ostest_delay_usec//[^[:alnum:]]/}"
  app_name_suffix="${app_name_suffix}-ostdelay${sanitized}"
fi

if [[ "${ostest_startup_trace}" -ne 0 ]]; then
  app_name_suffix="${app_name_suffix}-osttrace"
fi

if [[ -n "${app_name_suffix}" ]]; then
  app_image_bin="${build_dir}/stm32n6570-dk-psram-verify${app_name_suffix}.bin"
  full_image_bin="${build_dir}/stm32n6570-dk-psram-verify${app_name_suffix}-full.bin"
fi

resolve_signing_tool
require_helper "${pack_nxboot_header_app_tool}"
require_helper "${pack_full_flash_image_tool}"
if [[ "${app_only}" -eq 0 ]]; then
  require_helper "${pack_stm32_fsbl_nxboot_tool}"
fi

printf '==> Cleaning STM32N6570-DK PSRAM validation outputs\n'
clean_build_dir

if [[ "${app_only}" -eq 0 ]]; then
  printf '==> Building STM32N6570-DK NXboot for PSRAM validation\n'
  distclean_tree
  configure_board stm32n6570-dk:nxboot
  make "-j${jobs}"
  nxboot_payload_size="$(file_size nuttx.bin)"
  "${pack_stm32_fsbl_nxboot_tool}" "${signing_tool}" nuttx.bin \
    "${nxboot_image_bin}"
fi

printf '\n==> Building STM32N6570-DK protected PSRAM validation app\n'
distclean_tree
configure_board stm32n6570-dk:knsh
apply_validation_config
make "-j${jobs}"
kernel_payload_size="$(file_size nuttx.bin)"
user_payload_size="$(file_size nuttx_user.bin)"
app_raw_bin="$(mktemp "${TMPDIR:-/tmp}/stm32n6570-dk-psram-verify-raw.XXXXXX")"
tmp_files+=("${app_raw_bin}")
create_protected_payload nuttx.bin nuttx_user.bin "${app_raw_bin}"
"${pack_nxboot_header_app_tool}" "${app_raw_bin}" "${app_image_bin}" \
  "${version}"

if [[ -f "${nxboot_image_bin}" ]]; then
  "${pack_full_flash_image_tool}" "${nxboot_image_bin}" "${app_image_bin}" \
    "${full_image_bin}"
  full_image_created=1
elif [[ "${app_only}" -eq 0 ]]; then
  printf 'ERROR: expected NXboot image is missing: %s\n' \
    "${nxboot_image_bin}" >&2
  exit 1
fi

printf '\n==> Firmware outputs\n'
if [[ "${app_only}" -eq 0 ]]; then
  printf '  NXboot trusted image:\n'
  printf '    file:       %s\n' "${nxboot_image_bin}"
  printf '    size:       %s bytes\n' "$(file_size "${nxboot_image_bin}")"
  printf '    payload:    %s bytes raw NuttX NXboot\n' "${nxboot_payload_size}"
  printf '    structure:  [ST BootROM FSBL header][NuttX NXboot payload]\n'
  printf '    program at: XSPI2 NOR 0x70000000\n\n'
else
  printf '  NXboot trusted image: skipped by --app-only\n\n'
fi

printf '  PSRAM validation app payload:\n'
printf '    size:       %s bytes\n' "$(file_size "${app_raw_bin}")"
printf '    kernel:     %s bytes at 0x70100000 + %s, normally 0x70100400\n' \
  "${kernel_payload_size}" "${header_size}"
printf '    user:       %s bytes at %s\n' "${user_payload_size}" "${userspace}"
printf '    structure:  [kernel blob][0xff padding to %s bytes][user blob]\n' \
  "$(kernel_window_size)"
printf '    storage:    temporary build input, not kept in build/\n\n'

printf '  PSRAM validation NXboot app image:\n'
printf '    file:       %s\n' "${app_image_bin}"
printf '    size:       %s bytes\n' "$(file_size "${app_image_bin}")"
printf '    structure:  [NXboot header %s][protected app payload]\n' \
  "${header_size}"
printf '    identifier: %s\n' "${identifier}"
printf '    program at: XSPI2 NOR 0x70100000\n'
printf '    kernel vector: 0x70100000 + %s, normally 0x70100400\n' \
  "${header_size}"
printf '    PSRAM heap: 0x90000000 + %s\n\n' "${psram_heap_offset}"
printf '    PSRAM MPU policy: %s\n\n' "${psram_mpu_policy}"
printf '    PSRAM no-write-allocate MAIR: %s\n\n' \
  "$(config_state CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE)"
printf '    PSRAM self-test debug: %s\n\n' \
  "$(config_state CONFIG_STM32N6570_DK_PSRAM_SELFTEST_DEBUG)"
printf '    PSRAM self-test noflush: %s\n\n' \
  "$(config_state CONFIG_STM32N6570_DK_PSRAM_SELFTEST_NOFLUSH)"
printf '    SVC return barrier: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_BARRIER)"
printf '    BASEPRI ISB: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_BASEPRI_ISB)"
printf '    SVC return current-frame: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_RETURN_CURRENT_FRAME)"
printf '    SVC return user BASEPRI=0: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_RETURN_USER_BASEPRI0)"
printf '    SVC dispatch BASEPRI=0: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_DISPATCH_BASEPRI0)"
printf '    SVC kernel stack: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_KERNEL_STACK)"
printf '    SVC kernel stack size: %s bytes\n\n' \
  "$(config_value CONFIG_ARMV8M_SYSCALL_KERNEL_STACKSIZE)"
printf '    SVC kernel stack basic frame: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME)"
printf '    SVC kernel stack PSP dispatch: %s\n\n' \
  "$(config_state CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP)"
printf '    USART1 TX poll drain: %s\n\n' \
  "$(config_state CONFIG_STM32N6_USART1_TX_POLL_DRAIN)"
printf '    USART1 high priority: %s\n\n' \
  "$(config_state CONFIG_STM32N6_USART1_HIGH_PRIORITY)"
printf '    ostest wait result: %s\n\n' \
  "$(config_state CONFIG_TESTING_OSTEST_WAITRESULT)"
printf '    ostest short delay: %s usec\n\n' \
  "$(config_value CONFIG_TESTING_OSTEST_DELAY_USEC)"
printf '    ostest startup trace: %s\n\n' \
  "$(config_state CONFIG_TESTING_OSTEST_STARTUP_TRACE)"
printf '    FPU context: %s\n\n' "$(config_state CONFIG_ARCH_FPU)"
printf '    Lazy FPU: %s\n\n' "$(config_state CONFIG_ARMV8M_LAZYFPU)"
printf '    idle stack: %s bytes\n\n' \
  "$(config_value CONFIG_IDLETHREAD_STACKSIZE)"
printf '    bootstrap user heap: %s in internal SRAM\n\n' \
  "$(config_value CONFIG_STM32N6_PROTECTED_UHEAP_SIZE)"

if [[ "${full_image_created}" -ne 0 ]]; then
  printf '  Full XSPI2 NOR image:\n'
  printf '    file:       %s\n' "${full_image_bin}"
  printf '    size:       %s bytes\n' "$(file_size "${full_image_bin}")"
  printf '    structure:  [NXboot at +0x0][0xff padding][app at +0x100000]\n'
  printf '    program at: XSPI2 NOR 0x70000000\n\n'
else
  printf '  Full XSPI2 NOR image: skipped by --app-only because %s is missing\n\n' \
    "${nxboot_image_bin}"
fi

printf '==> Suggested serial checks after boot\n'
printf '  help\n'
printf '  free\n'
printf '  ps\n'
printf '  stackmonitor_start\n'
printf '  stackmonitor_stop\n'
printf '  ramtest -w -s 1024\n'
printf '  ramtest -w -s 65536\n'
printf '  ramtest -w -s 1048576\n'
printf '  ramtest -w -s 16777216\n'
printf '  ramspeed -a -s 65536 -n 1000\n'
printf '  memstress -m 4096 -n 64 -x 1 -t 1000 &\n'
printf '  ps\n'
printf '  cat /proc/4/stack      # replace 4 with the memstress task PID from ps\n'
printf '  cat /proc/5/stack      # replace 5 with one memstress pthread TID from ps\n'
printf '  psramstack -l 100000 -s 8192\n'
printf '  sysresume\n'
printf '  sysresume -T\n'
printf '  sysresume -m\n'
printf '  sysresume -T -m\n'
printf '  ostest\n'
printf '  dumpstack\n'
printf '\n'
printf '  Keep stackmonitor stopped while typing stress commands; its periodic\n'
printf '  output can interleave with NSH input on the serial console.\n'
printf '\n'
printf '  Do not raw-write 0x90000000 with -a when PSRAM heap offset is 0x0.\n'
printf '  For a direct base-window ramtest, rebuild with --psram-heap-offset 0x00200000\n'
printf '  and only test an address range below that offset.\n'
printf '\n'
printf '  To make ordinary task/pthread stacks more likely to land in PSRAM,\n'
printf '  rebuild with --bootstrap-uheap-size 0x4000 or smaller and recheck\n'
printf '  /proc/<pid>/stack for StackAlloc/StackBase in the 0x90000000 window.\n'
printf '\n'
printf '  The NSH help output must list ramtest, memstress, psramstack, sysresume,\n'
printf '  ostest, stackmonitor_start, stackmonitor_stop, and dumpstack. If it lists\n'
printf '  lvgldemo but not these commands, the board is running an LVGL image\n'
printf '  instead of the PSRAM validation image.\n'
