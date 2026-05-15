# EK-RA8P1 NuttX Porting Notes

## Current Direction

- SoC family: new scaffold under `nuttx/arch/arm/src/ra8p`
- Board: `nuttx/boards/arm/ra8p/ek-ra8p1`
- Reference project:
  `/home/uan-wsl2/third/ra-fsp-examples/example_projects/ek_ra8p1/_quickstart/quickstart_ek_ra8p1_ep`
- Vendor BSP reference:
  `/home/uan-wsl2/third/fsp/ra/board/ra8p1_ek`

The selected boot model is **NXboot in internal MRAM + XIP app in external
OSPI NOR**:

1. RA8P1 boots NuttX/NXboot from internal MRAM at `0x02000000`.
2. NXboot initializes external OSPI0 CS1 NOR.
3. NXboot reads an app image at `0x90000000`.
4. The app image uses an NXboot header of `0x400`; its vector table starts at
   `0x90000400`.

This matches the board resources better than placing NXboot in external flash:
internal MRAM is non-volatile and bootable, while the external Macronix NOR is
reserved for updateable XIP application slots.

## Board Facts From FSP Quickstart

```text
MCU:          R7KA8P1KFLCAC, Cortex-M85
Internal MRAM 0x02000000 / 1 MiB
Internal SRAM 0x22000000 / 0x001d4000 bytes
ITCM          0x00000000 / 128 KiB
DTCM          0x20000000 / 128 KiB
SDRAM         0x68000000 / 64 MiB populated
OSPI0 CS0     0x80000000 / 256 MiB aperture
OSPI0 CS1     0x90000000 / 256 MiB aperture
OSPI1 CS0     0x70000000 / 128 MiB aperture
OSPI1 CS1     0x78000000 / 128 MiB aperture
OSPI NOR      Macronix MX25LW51245G, 512 Mbit / 64 MiB on OSPI0 CS1
```

FSP quickstart clock reference:

```text
XTAL 24 MHz
PLL1P 1 GHz
CPUCLK /1
ICLK /4
PCLKA /8
BCLK /8
SCICLK 96 MHz
```

## Implemented In This Checkpoint

- Added `ARCH_CHIP_RA8P` and `ARCH_CHIP_R7KA8P1KFLCAC`.
- Added minimal Cortex-M85 RA8P arch scaffold:
  - Kconfig and build glue.
  - chip and IRQ headers.
  - reset entry, BSS/data setup, FPU setup, heap allocation, and generic NVIC
    interrupt plumbing.
  - SysTick timer plumbing against the FSP quickstart nominal clock tree.
  - MPU capability selection for protected builds.
- Added `ek-ra8p1` board scaffold:
  - board Kconfig options for NXboot image format, OSPI NOR OTA slots, SDRAM
    heap, and protected user SRAM window.
  - `nxboot`, `nsh`, and `knsh` defconfigs.
  - linker scripts for MRAM loader, OSPI NXboot app, and protected KNSh
    kernel/user layout.
  - board bring-up path and an initial external memory registration path.
- Aligned the board source layout with the STM32H7RS/STM32N6 external-memory
  boards:
  - `ra8p1_extmem.c` is the board-level external-memory coordinator and MTD
    registration point.
  - `ra8p1_ospi.c` owns the OSPI0 CS1 NOR bring-up hook.
  - `ra8p1_sdram.c` owns the SDRAM-controller bring-up hook.
  - Future register-level work should go into those OSPI/SDRAM files rather
    than expanding the extmem coordinator.
- Ported the EK-RA8P1 external-memory pinmux into board-owned NuttX code:
  - `ra8p1_pins.c` programs PFS directly for OSPI0 CS1 and SDRAM pins;
  - `ra8p_boardinitialize()` applies the board pinmux before external-memory
    bring-up;
  - no FSP pin APIs or generated FSP data are used at runtime.
- Added direct-register OSPI_B support for the Macronix MX25LW51245G on
  OSPI0 CS1:
  - releases OSPI0 from module stop;
  - programs the CS1 command map initially for 1S-1S-1S SPI;
  - resets the flash through the OSPI LIO reset line;
  - reads and validates the 1S JEDEC ID `0x3a86c2`;
  - writes the required CFG2 registers for DQS, dummy cycles, and DTR mode;
  - switches the CS1 command map to 8D-8D-8D xSPI profile 1 and leaves the
    flash memory-mapped at `0x90000000`;
  - when the app is already executing from OSPI, it detects the XIP context and
    skips destructive reconfiguration.
- Added protected KNSh heap split:
  - internal SRAM from the idle stack top to the protected user SRAM window is
    the kernel heap;
  - the populated SDRAM at `0x68000000..0x6bffffff` is the user
    heap when `CONFIG_EK_RA8P1_SDRAM_HEAP=y`.
- Added the RA8P protected-build support needed by that layout:
  - user `.bss` clear and `.data` copy from the protected userspace descriptor;
  - ARMv8-M MPU regions for internal SRAM, internal MRAM, OSPI XIP flash, SDRAM,
    protected user text/data, and the SDRAM user heap;
  - cache/MPU setup after kernel `.bss/.data` initialization.
- Added an interim OSPI0 CS1 XIP/read-only MTD:
  - exposes the 64 MiB MX25LW51245G aperture at `0x90000000` as `/dev/mtd0`;
  - reports 512-byte MTD read blocks and 4 KiB erase blocks;
  - registers two 32 MiB NXboot OTA partitions as `/dev/ota0` and
    `/dev/ota1`;
  - supports `BIOC_XIPBASE` for direct XIP address discovery.
- Added Feather firmware helper scripts:
  - `tools/firmware/ek-ra8p1/build-nsh.sh`
  - `tools/firmware/ek-ra8p1/build-knsh.sh`

## Build Outputs

Both helper scripts currently build successfully.

```text
tools/firmware/ek-ra8p1/build-nsh.sh
  build/ek-ra8p1-nxboot.bin  -> program to internal MRAM 0x02000000
  build/ek-ra8p1-nsh.bin     -> program to OSPI0 CS1 0x90000000

tools/firmware/ek-ra8p1/build-knsh.sh
  build/ek-ra8p1-nxboot.bin  -> program to internal MRAM 0x02000000
  build/ek-ra8p1-knsh.bin    -> program to OSPI0 CS1 0x90000000
```

`build-nsh.sh` produces:

```text
[NXboot header 0x400][flat NuttX app]
```

`build-knsh.sh` produces:

```text
[NXboot header 0x400][protected kernel][padding to 0x90080400][user blob]
```

The scripts prefer `apps/boot/nxboot/tools/nximage.py`.  If the Python
`semantic_version` module is not installed, they fall back to a local NXboot
header writer so the build can still complete.

## Planned Image Layout

```text
0x02000000  raw NuttX/NXboot image in internal MRAM

0x90000000  primary NXboot app image header
0x90000400  flat NSH app vector table / text
0x90000400  protected KNSh kernel vector table / text
0x90080400  protected KNSh user blob base

0x92000000  secondary NXboot app image header
0x92000400  secondary app vector table / text
```

The first checkpoint uses two 32 MiB slots because the EK-RA8P1 OSPI NOR part
is 64 MiB.  There is no third OTA slot by default.

Protected KNSh reserves the top 128 KiB of internal SRAM for user-space static
data:

```text
0x22000000..idle_stack  kernel image/data/idle stack
idle_stack..0x221b3fff  kernel heap
0x221b4000..0x221d3fff  protected user SRAM .data/.bss window
0x68000000..0x6bffffff  SDRAM user heap, 64 MiB
```

This matches the STM32N6/H7RS protected layout intent: scarce internal SRAM is
kept for privileged kernel allocation, while large external memory backs user
allocations.

## Open Porting Work

- Translate the FSP RA8P1 clock setup into NuttX register code.
- Implement SCI console and select the board VCP pins.
- Extend OSPI_B support for MX25LW51245G:
  - add more configuration readback diagnostics around the 1S-to-8D
    transition;
  - replace the current read-only XIP MTD shim with controller-backed
    erase/program support.
- Hardware-validate the SDRAM user heap and pin configuration on EK-RA8P1
  silicon.
- Add flashing instructions once the RA flash/MRAM programming path is
  verified with the local probe/tooling.

## Current Status

This is still build-validated locally and needs hardware validation on
EK-RA8P1 silicon.

Validated locally:

```text
tools/firmware/ek-ra8p1/build-nsh.sh  passes
tools/firmware/ek-ra8p1/build-knsh.sh passes
```

Important limitations:

- `/dev/mtd0`, `/dev/ota0`, and `/dev/ota1` are still registered through a
  read-only XIP MTD shim. NXboot can validate and jump from OSPI XIP, but
  in-system erase/program support is not implemented yet.
- OSPI_B has a NuttX-owned direct-register bring-up path now, including a
  1S JEDEC ID check, but it still needs hardware validation and additional
  configuration readback diagnostics before it should be treated as
  production-ready.
- SDRAM is wired as the protected user heap in KNSh and
  `ra8p1_sdram_initialize()` programs the RA8P SDRAM controller directly from
  the EK-RA8P1 quickstart timing values. The populated SDRAM size is tracked as
  64 MiB at `0x68000000..0x6bffffff`; the wider external bus aperture is not
  added to heap.
- SCI console is still a placeholder, so early serial output is not functional.
- The clock setup uses nominal board constants; the RA8P1 clock tree still
  needs register-level initialization ported from FSP.

The layout, Kconfig names, defconfigs, link scripts, boot-image jump path,
firmware packaging, external-memory pinmux, OSPI_B memory-map transition, and
SDRAM heap path are in place. The next work should focus on RA8P1 clocks, SCI,
hardware bring-up diagnostics, and OSPI erase/program MTD support.
