#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-ble1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=ble1 HWSIM_LABEL=BLE1 HWSIM_CONFIG=hwsim_ble1 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
