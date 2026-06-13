#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
BUILD_DIR="${REPO_ROOT}/build/tools"
BIN="${BUILD_DIR}/x11-input"
SRC="${SCRIPT_DIR}/x11-input.c"

mkdir -p "${BUILD_DIR}"

if [[ ! -x "${BIN}" || "${SRC}" -nt "${BIN}" ]]; then
  if [[ -f /usr/include/X11/extensions/XTest.h ]] &&
     ldconfig -p 2>/dev/null | grep -q 'libXtst\.so'; then
    cc -Wall -Wextra -std=c99 -DHAVE_XTEST "${SRC}" \
      -o "${BIN}" -lX11 -lXtst
  else
    cc -Wall -Wextra -std=c99 "${SRC}" -o "${BIN}" -lX11
  fi
fi

exec "${BIN}" "$@"
