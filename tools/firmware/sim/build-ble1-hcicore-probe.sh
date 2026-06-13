#!/usr/bin/env bash
############################################################################
# tools/firmware/sim/build-ble1-hcicore-probe.sh
#
# Build an isolated BLE1 simulator image with upstream Linux HCI core enabled.
# This probe config is intentionally separate from hwsim_ble1 so the stable
# bt1/bt2/ble1/ble2 hwsim validation loop is not broken by partial HCI_CORE
# bring-up work.
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

HWSIM_ROLE=ble1-hcicore-probe \
HWSIM_LABEL=BLE1-HCI-CORE-PROBE \
HWSIM_CONFIG=hwsim_ble1_hcicore_probe \
  exec "${script_dir}/build-hwsim-role.sh" "$@"
