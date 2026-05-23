# STM32N6570-DK Firmware Images

This directory contains helper scripts for building and packaging
STM32N6570-DK firmware images for the NuttX NXboot flow.

## Build Details

Run the helpers from anywhere inside the Feather checkout.  The examples below
use the local STM32CubeProgrammer install path that is present in this
workspace:

```sh
cd /home/uan-wsl2/Feather
export STM32CUBE_PROGRAMMER_PATH=/home/uan-wsl2/third/stm32cubeprogrammer
```

Build the normal NSH image:

```sh
tools/firmware/stm32n6570-dk/build-nsh.sh -j 8
```

Build the protected KNSh image:

```sh
tools/firmware/stm32n6570-dk/build-knsh.sh -j 8
```

Build the LVGL framebuffer variants:

```sh
tools/firmware/stm32n6570-dk/build-nsh-lvgl.sh -j 8
tools/firmware/stm32n6570-dk/build-knsh-lvgl.sh -j 8
```

Build the protected KNSh PSRAM validation image:

```sh
tools/firmware/stm32n6570-dk/build-psram-verify.sh -j 8
```

All build paths produce a standard NXboot image with the same name:

- `build/stm32n6570-dk-nxboot.bin`
  - Structure: `[ST BootROM FSBL header][NuttX NXboot payload]`
  - Program at XSPI2 NOR `0x70000000`

Only the app image name changes:

- `build/stm32n6570-dk-nsh.bin`, `build/stm32n6570-dk-knsh.bin`,
  `build/stm32n6570-dk-nsh-lvgl.bin`, or
  `build/stm32n6570-dk-knsh-lvgl.bin`
  - Structure: `[NXboot header][NuttX app payload]`
  - Program at XSPI2 NOR `0x70100000`
- `build/stm32n6570-dk-psram-verify.bin`
  - Structure: `[NXboot header][protected KNSh validation payload]`
  - Program at XSPI2 NOR `0x70100000`

Each build also emits a combined full-flash image:

- `build/stm32n6570-dk-<app>-full.bin`
  - Structure: `[NXboot at +0x0][0xff padding][app at +0x100000]`
  - Program at XSPI2 NOR `0x70000000`
  - The app portion starts at `0x70000000 + 0x100000 = 0x70100000`

The build scripts package the ST FSBL header with `STM32_SigningTool_CLI`.
Set `STM32_SIGNING_TOOL`, `STM32_SIGNINGTOOL_CLI`, or
`STM32CUBE_PROGRAMMER_PATH` if the tool is not on `PATH`.

The normal and protected helpers build in two phases:

1. Configure and build `stm32n6570-dk:nxboot`.
2. Wrap `nuttx/nuttx.bin` with `pack-stm32-fsbl-nxboot.sh` to create
   `build/stm32n6570-dk-nxboot.bin`.
3. Configure and build the selected app.
4. Wrap the app payload with `pack-nxboot-header-app.sh` to create the app
   image at `0x70100000`.
5. Merge the standard NXboot image and app image with
   `pack-full-flash-image.sh` to create the optional single-download full
   image.

The protected KNSh helpers combine the kernel and user blobs before adding the
NXboot app header.  The PSRAM validation helper starts from the protected KNSh
configuration, then enables diagnostic applications and stack checking.

Useful PSRAM validation build options:

```sh
tools/firmware/stm32n6570-dk/build-psram-verify.sh --app-only

tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --bootstrap-uheap-size 0x4000

tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --psram-heap-offset 0x00200000
```

`--app-only` rebuilds only `build/stm32n6570-dk-psram-verify.bin` and leaves
the standard `build/stm32n6570-dk-nxboot.bin` untouched.  If that NXboot file
already exists in `build/`, the helper also regenerates
`build/stm32n6570-dk-psram-verify-full.bin`.  Use `--app-only` when the
NXboot image already programmed on the board matches the current clock and
external-memory setup.

## Current KNSh Layout

The protected KNSh app image uses this runtime layout:

- `0x70100400`: kernel vector table and kernel XIP text
- `0x70180400`: protected user-space image
- `0x34000000`: internal SRAM
- `0x341e0000`: protected user SRAM window
- `0x90000000`: XSPI1 PSRAM, added as user heap

The kernel heap stays in internal SRAM.  User space starts from a small
bootstrap heap in protected user SRAM, then adds the 32 MiB PSRAM region as a
second user heap region.

## Current External Memory Settings

The current N6 high-speed target uses:

- CPU 800 MHz, SYS/AXI 400 MHz, HCLK/PCLK 200 MHz.
- XSPI2 NOR at `0x70000000`, JEDEC ID `c2 81 3b`, OPI/DTR memory-mapped mode.
- XSPI1 PSRAM at `0x90000000`.
- APS256 MR0 `0x30`, MR4 `0x20`, MR8 `0x40`.
- Linear-burst PSRAM read command `0x20` and write command `0xa0`.
- PSRAM refresh enabled, with `refresh=96` during the 50 MHz startup phase
  and `refresh=396` at the 200 MHz memory-mapped target.
- PSRAM final memory-mapped clock targets 200 MHz when the VDDIO2 HSLV fuse is
  set.  If HSLV is missing, NXboot keeps the conservative 50 MHz mapping and
  logs the fallback reason.

Program the matching full image, or both generated images, after this clock
change.  The XIP app expects NXboot to establish the 800 MHz PLL and XSPI
memory-mapped modes before the handoff.

## Download / Flashing Details

The scripts do not flash automatically.  Program the two generated images to:

```text
0x70000000  build/stm32n6570-dk-nxboot.bin
0x70100000  build/stm32n6570-dk-knsh.bin
```

For the normal NSH image, replace the second file with
`build/stm32n6570-dk-nsh.bin`.

For the PSRAM validation image, use:

```text
0x70000000  build/stm32n6570-dk-nxboot.bin
0x70100000  build/stm32n6570-dk-psram-verify.bin
```

When using a full image, program only that one file at the base of XSPI2 NOR:

```text
0x70000000  build/stm32n6570-dk-psram-verify-full.bin
```

The STM32N6570-DK external NOR loader from STM32CubeProgrammer is required when
downloading to XSPI2 NOR.  In this workspace it is available at:

```text
/home/uan-wsl2/third/stm32cubeprogrammer/bin/ExternalLoader/MX66UW1G45G_STM32N6570-DK.stldr
```

CLI download example for the PSRAM validation full image:

```sh
cd /home/uan-wsl2/Feather

export STM32CUBE_PROGRAMMER_PATH=/home/uan-wsl2/third/stm32cubeprogrammer
export STM32_PROGRAMMER_CLI="${STM32CUBE_PROGRAMMER_PATH}/bin/STM32_Programmer_CLI"
export STM32_EXT_LOADER="${STM32CUBE_PROGRAMMER_PATH}/bin/ExternalLoader/MX66UW1G45G_STM32N6570-DK.stldr"

"${STM32_PROGRAMMER_CLI}" \
  -c port=SWD mode=HOTPLUG \
  -el "${STM32_EXT_LOADER}" \
  -d build/stm32n6570-dk-psram-verify-full.bin 0x70000000 \
  -v \
  -rst
```

For other apps, replace the full image with
`build/stm32n6570-dk-nsh-full.bin`, `build/stm32n6570-dk-knsh-full.bin`,
`build/stm32n6570-dk-nsh-lvgl-full.bin`, or
`build/stm32n6570-dk-knsh-lvgl-full.bin`.

If NXboot is already programmed and only the validation app changed, download
only the app image:

```sh
"${STM32_PROGRAMMER_CLI}" \
  -c port=SWD mode=HOTPLUG \
  -el "${STM32_EXT_LOADER}" \
  -d build/stm32n6570-dk-psram-verify.bin 0x70100000 \
  -v \
  -rst
```

For the normal and protected app images, keep the same NXboot image and replace
only the second `-d` file with `build/stm32n6570-dk-nsh.bin` or
`build/stm32n6570-dk-knsh.bin`.

The equivalent GUI flow is to open STM32CubeProgrammer, connect over SWD with
the `MX66UW1G45G_STM32N6570-DK` external loader enabled, download the selected
`*-full.bin` image at `0x70000000`, then reset the board.  To update only the
app, download the selected app image at `0x70100000`.

The validation build keeps the protected KNSh boot model and enables PSRAM
heap/stack diagnostics including `ostest`, `ramtest`, `memstress`, stack
coloration, stack monitor, `dumpstack`, backtrace, and ARMv8-M hardware stack
limit checking.  It also defaults the PSRAM heap offset to `0x0` so user
threads are more likely to allocate stacks from XSPI1 PSRAM.

## After Flashing the PSRAM Validation Image

The first boot checkpoint is the serial log.  A good boot should show NXboot
mapping XSPI1 PSRAM, the app inheriting that memory-mapped state, and NSH
starting:

```text
XSPI1 PSRAM mapped at 0x90000000 refresh=396
...
XSPI1 PSRAM already memory-mapped refresh=396
XSPI1 PSRAM self-test passed
...
NuttShell (NSH)
nsh>
```

Then run the validation commands from NSH:

```sh
help
free
ps
stackmonitor_start
stackmonitor_stop
ramtest -w -s 1024
ramtest -w -s 65536
ramtest -w -s 1048576
ramtest -w -s 16777216
memstress -m 4096 -n 64 -x 1 -t 1000 &
ps
cat /proc/<memstress-pid>/stack
cat /proc/<memstress-thread-tid>/stack
ostest
dumpstack
```

The `help` output must list `ramtest`, `memstress`, `ostest`,
`stackmonitor_start`, `stackmonitor_stop`, and `dumpstack`.  If `help` shows
`lvgldemo` but not these commands, the board is still running an LVGL image,
not the PSRAM validation image.

Keep `stackmonitor` stopped while typing stress commands.  Its periodic output
can interleave with NSH input on the serial console and corrupt a pasted
command.

Use `ps` to pick user-task PIDs and inspect their stack placement:

```sh
cat /proc/<pid>/stack
```

For PSRAM-backed user stacks, `StackAlloc` or `StackBase` should fall in the
`0x90000000..0x91ffffff` XSPI1 PSRAM window.  Internal boot/kernel stacks may
still appear in internal SRAM; for example, the reset handoff MSP in the boot
log is expected to be internal SRAM.

The validation build keeps a small internal user SRAM bootstrap heap before
adding PSRAM as a second user heap region.  Small allocations and small stacks
can still come from that internal heap.  To make ordinary task/pthread stacks
more likely to land in PSRAM, rebuild with a smaller bootstrap heap:

```sh
tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --bootstrap-uheap-size 0x4000
```

Use the heap-allocation form of `ramtest` by default.  With the validation
image's default `--psram-heap-offset 0x0`, PSRAM starts as NuttX heap at
`0x90000000`, so a raw `ramtest -a 0x90000000` would overwrite allocator state.
If a direct base-window test is needed, rebuild with a reserved prefix, for
example:

```sh
tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --psram-heap-offset 0x00200000
```

Then only raw-test the reserved window below the heap start:

```sh
ramtest -w -a 0x90000000 -s 1048576
```
