#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-dynps-ap.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=dynps-ap HWSIM_LABEL=DYNPS-AP HWSIM_CONFIG=hwsim_dynps_ap \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
