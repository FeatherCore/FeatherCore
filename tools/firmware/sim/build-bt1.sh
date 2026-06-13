#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-bt1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=bt1 HWSIM_LABEL=BT1 HWSIM_CONFIG=hwsim_bt1 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
