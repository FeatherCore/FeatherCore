#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-p2p2.sh
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=p2p2 HWSIM_LABEL=P2P2 \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
