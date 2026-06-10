#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-sta3.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=sta3 HWSIM_LABEL=STA3 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
