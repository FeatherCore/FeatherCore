#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-ble1-hcisock-probe.sh
#
# Build an isolated BLE1 simulator image with upstream Linux HCI socket
# support enabled.  This remains separate from hwsim_ble1 until hci_sock.c
# compiles and its observable behavior can replace the staging HCI socket
# facade without regressing the normal bt1/bt2/ble1/ble2 loop.
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=ble1-hcisock-probe \
HWSIM_LABEL=BLE1-HCI-SOCK-PROBE \
HWSIM_CONFIG=hwsim_ble1_hcisock_probe \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
