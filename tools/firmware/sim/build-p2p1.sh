#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-p2p1.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=p2p1 HWSIM_LABEL=P2P1 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
