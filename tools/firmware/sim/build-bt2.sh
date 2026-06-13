#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-bt2.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=bt2 HWSIM_LABEL=BT2 HWSIM_CONFIG=hwsim_bt2 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
