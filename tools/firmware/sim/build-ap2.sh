#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-ap2.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=ap2 HWSIM_LABEL=AP2 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
