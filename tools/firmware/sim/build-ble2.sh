#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-ble2.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=ble2 HWSIM_LABEL=BLE2 HWSIM_CONFIG=hwsim_ble2 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
