#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-ap1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=ap1 HWSIM_LABEL=AP1 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
