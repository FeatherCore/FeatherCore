#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 3 ]; then
  echo "usage: $0 <nxboot.bin> <app.bin> <output-full.bin> [app-offset]" >&2
  exit 64
fi

nxboot_input=$1
app_input=$2
output=$3
app_offset=${4:-0x100000}

if [ ! -f "$nxboot_input" ]; then
  echo "error: NXboot image not found: $nxboot_input" >&2
  exit 66
fi

if [ ! -f "$app_input" ]; then
  echo "error: app image not found: $app_input" >&2
  exit 66
fi

output_dir=$(dirname "$output")
if [ ! -d "$output_dir" ]; then
  echo "error: output directory not found: $output_dir" >&2
  exit 73
fi

python3 - "$nxboot_input" "$app_input" "$output" "$app_offset" <<'PY'
import io
import os
import sys

nxboot_path, app_path, output_path, app_offset = sys.argv[1:5]
app_offset = int(app_offset, 0)

nxboot_size = os.stat(nxboot_path).st_size
app_size = os.stat(app_path).st_size

if nxboot_size > app_offset:
    raise SystemExit(
        f"error: NXboot image is {nxboot_size} bytes, larger than app "
        f"offset {app_offset} bytes"
    )

with open(output_path, "wb") as output:
    with open(nxboot_path, "rb") as nxboot:
        while True:
            data = nxboot.read(io.DEFAULT_BUFFER_SIZE)
            if not data:
                break
            output.write(data)

    output.write(b"\xff" * (app_offset - nxboot_size))

    with open(app_path, "rb") as app:
        while True:
            data = app.read(io.DEFAULT_BUFFER_SIZE)
            if not data:
                break
            output.write(data)

print(f"Full flash image: {output_path}")
print(f"  base address:       0x70000000")
print(f"  NXboot offset:      0x00000000")
print(f"  NXboot size:        {nxboot_size} bytes")
print(f"  app offset:         0x{app_offset:08x}")
print(f"  app load address:   0x{0x70000000 + app_offset:08x}")
print(f"  app size:           {app_size} bytes")
print(f"  output size:        {app_offset + app_size} bytes")
PY
