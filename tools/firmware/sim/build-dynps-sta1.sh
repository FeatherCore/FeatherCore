#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-dynps-sta1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=dynps-sta1 HWSIM_LABEL=DYNPS-STA1 HWSIM_CONFIG=hwsim_dynps_sta1 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
