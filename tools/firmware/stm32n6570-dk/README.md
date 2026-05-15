# STM32N6570-DK Firmware Images

This directory contains helper scripts for building and packaging
STM32N6570-DK firmware images for the NuttX NXboot flow.

## Build

Build the normal NSH image:

```sh
tools/firmware/stm32n6570-dk/build-nsh.sh
```

Build the protected KNSh image:

```sh
tools/firmware/stm32n6570-dk/build-knsh.sh
```

Both scripts build two flashable images under `build/`:

- `build/stm32n6570-dk-nxboot.bin`
  - Structure: `[ST BootROM FSBL header][NuttX NXboot payload]`
  - Program at XSPI2 NOR `0x70000000`
- `build/stm32n6570-dk-nsh.bin` or `build/stm32n6570-dk-knsh.bin`
  - Structure: `[NXboot header][NuttX app payload]`
  - Program at XSPI2 NOR `0x70100000`

The build scripts package the ST FSBL header with `STM32_SigningTool_CLI`.
Set `STM32_SIGNING_TOOL`, `STM32_SIGNINGTOOL_CLI`, or
`STM32CUBE_PROGRAMMER_PATH` if the tool is not on `PATH`.

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

Program both generated images after this clock change.  The XIP app expects
NXboot to establish the 800 MHz PLL and XSPI memory-mapped modes before the
handoff.

## Flashing Addresses

The scripts do not flash automatically.  Program the two generated images to:

```text
0x70000000  build/stm32n6570-dk-nxboot.bin
0x70100000  build/stm32n6570-dk-knsh.bin
```

For the normal NSH image, replace the second file with
`build/stm32n6570-dk-nsh.bin`.
