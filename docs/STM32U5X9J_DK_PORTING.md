# STM32U5x9J-DK NuttX Porting Notes

This document tracks the STM32U5x9J-DK port in:

```text
/home/uan-wsl2/nuttx-work
```

Current branch:

```text
nuttx: /home/uan-wsl2/nuttx-work/nuttx -> vendor/stm32u5x9-bringup
apps:  /home/uan-wsl2/nuttx-work/apps  -> vendor/stm32u5x9-bringup
```

## Scope

STM32U5 is an existing NuttX SoC family, so STM32U5x9J-DK is implemented as a
new board target under the existing architecture:

```text
nuttx/arch/arm/src/stm32u5
nuttx/boards/arm/stm32u5/stm32u5x9j-dk
```

Cube references used for pin, clock, register, and initialization order:

```text
/home/uan-wsl2/third/STM32CubeU5/Projects/STM32U5x9J-DK
/home/uan-wsl2/third/STM32CubeU5/Projects/STM32U5x9J-DK/Examples/BSP
/home/uan-wsl2/third/STM32CubeU5/Drivers/BSP/STM32U5x9J-DK
```

The port is pure NuttX style. Cube HAL/LL/BSP `.c` files are not copied into
the NuttX build.

## Board Facts

```text
Board: STM32U5x9J-DK
MCU:   STM32U5A9NJHxQ
Flash: 0x08000000 / 4096 KiB
SRAM1: 0x20000000 / 768 KiB
Clock: MSIS 4 MHz -> PLL1 -> SYSCLK/HCLK 160 MHz
COM1:  USART1, TX PA9 AF7, RX PA10 AF7, 115200 8N1
LED3:  PE0, green, active high
LED4:  PE1, red, active high
USER:  PC13
```

Cube BSP I2C mapping:

```text
I2C2: PF1 SCL, PF0 SDA, AF4
I2C3: PH7 SCL, PH8 SDA, AF4, STTS22H + VL53L5CX
I2C4: PB6 SCL, PB7 SDA, AF5
I2C5: PH5 SCL, PH4 SDA, AF2, Sitronix touch
```

External resources:

```text
OSPI1 NOR:  MX25UM51245G, 64 MiB, XIP window 0x90000000
HSPI1 RAM:  APS512xx, 64 MiB, memory window 0xa0000000
SDMMC1:     eMMC, 8-bit bus
LCD:        480 x 480 DSI panel, LTDC/GFXMMU/DMA2D path
```

## Current Implementation

Configuration:

```text
stm32u5x9j-dk:nsh
ARCH_CHIP_STM32U5A9NJHXQ
ARCH_BOARD_STM32U5X9J_DK
```

Core bring-up:

```text
- USART1 serial console on PA9/PA10
- 160 MHz STM32U5 clock path
- NSH shell
- procfs mount in board late initialization
- SRAM2/SRAM3/SRAM5 heap regions
- HSPI PSRAM heap region added from arm_addregion()
```

NuttX device nodes and board resources:

```text
/dev/userleds   LED3/LED4
/dev/buttons    USER button
/dev/i2c2       I2C2
/dev/i2c3       I2C3
/dev/i2c4       I2C4
/dev/i2c5       I2C5
/dev/mtd0       OSPI1 NOR MTD, XIP read + erase/page-program path
/dev/ospi0      OSPI1 NOR read-only diagnostic + opt-in scratch command
/dev/hspiram0   HSPI1 PSRAM diagnostic and manual 32-byte self-test
/dev/mmcsd0     SDMMC1 eMMC, 8-bit MMCSD block device
/dev/emmcinfo0  eMMC read-only geometry/block0 diagnostic
/dev/fb0        480x480 RGB32 framebuffer backed by PSRAM
/dev/input0     Sitronix touchscreen via NuttX touchscreen upper-half
/dev/temp0      STTS22H one-shot temperature diagnostic
/dev/range0     VL53L5CX identity/health diagnostic
```

## Memory Layout

```text
Internal flash:  0x08000000..0x083fffff
Internal SRAM1:  0x20000000..0x200bffff
OSPI1 NOR XIP:   0x90000000..0x93ffffff
HSPI1 PSRAM:     0xa0000000..0xa3ffffff
Framebuffer:     0xa0000000..0xa01fffff reserved window
PSRAM heap:      0xa0200000..0xa3ffffff
```

The first 2 MiB of PSRAM is reserved for display/framebuffer bring-up. The
remaining 62 MiB is added to the NuttX heap when `CONFIG_STM32U5X9J_DK_HSPI_HEAP`
is enabled. The default NSH config sets `CONFIG_MM_REGIONS=5`.

PSRAM diagnostic:

```sh
cat /dev/hspiram0
echo selftest > /dev/hspiram0
cat /dev/hspiram0
```

The self-test is a 32-byte save/write/read/restore check at the PSRAM base. It
does not claim the full PSRAM has passed a memory test; it only verifies that
the mapped data path is alive enough for bring-up.

The SDMMC1 IDMA bounce buffer is placed in the framebuffer-reserved PSRAM
slack area:

```text
CONFIG_STM32U5_SDMMC_IDMA=y
CONFIG_STM32U5_SDMMC_IDMA_BUFFER_ADDR=0xa01f0000
CONFIG_STM32U5_SDMMC_IDMA_BUFFER_SIZE=4096
```

## External Storage Policy

OSPI1 NOR currently registers as `/dev/mtd0` after JEDEC-ID probe and
memory-mapped read setup. Register-level XSPI/OCTOSPI access now lives in the
STM32U5 SoC layer:

```text
- arch/arm/src/stm32u5/hardware/stm32_xspi.h
- arch/arm/src/stm32u5/stm32_xspi.c
- arch/arm/src/stm32u5/stm32_xspi.h
```

Board code only supplies MX25UM51245G commands, geometry, and registration
policy:

```text
- read and block-read work through the XIP memory window
- BIOC_XIPBASE reports 0x90000000
- erase uses 64 KiB sector erase
- 4 KiB subsector erase command is wrapped for explicit scratch diagnostics
- bwrite/write use 256 byte page program with WIP status polling
- erase/program suspend and resume commands are wrapped for manual diagnostics
- memory-mapped mode is restored after erase/write operations
```

The board also registers a safe diagnostic node:

```sh
cat /dev/ospi0
```

The output reports the MX25UM51245G static geometry, cached JEDEC ID, XIP
base/first word, mount-only filesystem policy, scratch-test enable state, and
last scratch result. By default, scratch erase/program testing is disabled. It
is only compiled in when `CONFIG_STM32U5X9J_DK_OSPI_SCRATCH_TEST=y`; then this
explicit command runs a destructive 4 KiB scratch test at the configured
offset:

```sh
echo scratch > /dev/ospi0
```

The default `nsh` defconfig enables littlefs and flash automount. Board
bring-up creates `/mnt/flash` and attempts to mount an existing littlefs volume:

```text
mount /dev/mtd0 /mnt/flash littlefs
```

No automatic format or erase is performed. If the external NOR is blank or
contains another filesystem, the mount failure is expected and only logged.

The verified local build uses littlefs v2.10.2 with NuttX-provided defines:

```text
CONFIG_FS_LITTLEFS=y
CONFIG_FS_LITTLEFS_VERSION="v2.10.2"
CONFIG_FS_LITTLEFS_HAS_LFS_DEFINES=y
CONFIG_STM32U5X9J_DK_FLASH_AUTOMOUNT=y
```

This workspace could not download the upstream littlefs tarball over the
restricted network, so `nuttx/fs/littlefs/littlefs` is supplied as an ignored
local source copy from `/home/uan-wsl2/zephyrproject/modules/fs/littlefs`.

eMMC is exposed through `/dev/mmcsd0`. SDMMC1 clock/source setup and the MMCSD
lower-half live in the STM32U5 SoC layer:

```text
arch/arm/src/stm32u5/stm32_sdmmc.c
arch/arm/src/stm32u5/stm32_sdmmc.h
```

The board only configures SDMMC1 pins, calls the SoC helper, registers MMCSD
slot 0, reports media present, and attempts to mount an existing FAT volume:

```text
mount /dev/mmcsd0 /mnt/emmc vfat
```

No automatic `mkfatfs`, partitioning, or destructive block write is performed.
The default config enables STM32U5 SDMMC IDMA. Buffers that are unaligned or
located in SRAM regions not reachable by SDMMC1 IDMA are bounced through the
board-provided PSRAM buffer before the transfer. This removes the earlier
non-DMA overrun warning while keeping filesystem mounts non-destructive.

Read-only eMMC diagnostic:

```sh
cat /dev/emmcinfo0
```

The diagnostic reports `/dev/mmcsd0` open status, `BIOC_GEOMETRY` data, a
block0 read checksum and first 16 bytes when the block device can be read
through VFS. It never writes, partitions, erases, or formats eMMC.

## Display And Touch

The touch path is now a real NuttX touchscreen upper-half:

```text
I2C5 address: 0x70
Node:         /dev/input0
Mode:         EXTI wake + LPWORK polling fallback, 1 point
Coordinates: 480x480 portrait, press/move/release events
```

The LCD path is now split along NuttX SoC/board boundaries. The STM32U5 SoC
layer owns DSI/LTDC/GFXMMU/DMA2D register access, while the board layer owns
panel reset, timing values, framebuffer address, and `/dev/fb0` registration.

The board registers a framebuffer and fills the PSRAM framebuffer with a
static bring-up pattern:

```text
Node:         /dev/fb0
Format:       RGB32
Resolution:   480 x 480
Framebuffer:  0xa0000000
Pattern:      black clear, vertical color bars, basic RGB rectangles
```

SoC display files:

```text
arch/arm/src/stm32u5/hardware/stm32_dsi.h
arch/arm/src/stm32u5/hardware/stm32_ltdc.h
arch/arm/src/stm32u5/hardware/stm32_gfxmmu.h
arch/arm/src/stm32u5/hardware/stm32_dma2d.h
arch/arm/src/stm32u5/stm32_dsi.c
arch/arm/src/stm32u5/stm32_dsi.h
arch/arm/src/stm32u5/stm32_ltdc.c
arch/arm/src/stm32u5/stm32_ltdc.h
arch/arm/src/stm32u5/stm32_gfxmmu.c
arch/arm/src/stm32u5/stm32_gfxmmu.h
arch/arm/src/stm32u5/stm32_dma2d.c
arch/arm/src/stm32u5/stm32_dma2d.h
```

Current LCD status:

```text
- GFXMMU circular LUT maps GFXMMU virtual buffer 0 to PSRAM framebuffer
- LTDC layer 1 is configured as ARGB8888 and exposed through NuttX fb
- DSI host has a video-mode setup and a board panel command table
- DMA2D has a simple register-to-memory fill helper used by colorbar setup
- Board LCD init performs panel reset, vendor init, sleep-out, display-on,
  LTDC enable, and static pattern fill
- LTDC framebuffer vtable exposes getvideoinfo, getplaneinfo, open, close,
  getpower, setpower, and unsupported ioctl returns -ENOTSUP
```

Important limitation: this is still awaiting hardware validation. The panel
command sequence is now present as NuttX board data, but DSI/LTDC/GFXMMU timing
may still need tuning on the actual display if color bars are not visible.

## Sensor Status

```text
STTS22H:
  Node: /dev/temp0
  Bus:  I2C3, address 0x7f
  Read: WHOAMI + BDU/auto-increment setup + one-shot raw temperature
  Unit: text output includes raw and milli-Celsius estimate

VL53L5CX:
  Node: /dev/range0
  Bus:  I2C3, address 0x29
  Read: LP wakeup + banked device/revision ID + health/capability text
  Ranging: returns blocked/-ENOTSUP status; no fake distance samples
```

VL53L5CX full ranging is intentionally not faked. The full ranging path needs
the firmware/config table dependency to be isolated or cleanly replaced before
valid distance samples can be reported.

## STM32U5 Architecture Changes

The current branch also extends the STM32U5 common layer for this board:

```text
- STM32U5A9 GPIOI/GPIOJ support
- STM32U5A9 I2C5 memory map, IRQs, pinmap, RCC bits
- I2C3 APB3 clock/reset path fixes
- SDMMC1 clock enable symbol fix
- STM32U5 SDMMC IDMA Kconfig, preflight, and bounce-buffer handling
- SoC-level XSPI/HSPI controller helper used by OSPI NOR and HSPI PSRAM
- OSPI1/OCTOSPIM/HSPI1 memory map and RCC definitions
- SoC-level SDMMC1 clock/source helper and MMCSD lower-half for 8-bit eMMC
- DSI/LTDC/GFXMMU/DMA2D base definitions and dedicated SoC source files
- DSI IRQ added to STM32U5 IRQ map
- SRAM5 heap address fix in stm32_allocateheap.c
```

## Cube BSP Coverage Table

```text
Cube BSP area        NuttX status
-------------------  ---------------------------------------------------------
LED/Button           Implemented through /dev/userleds and /dev/buttons.
I2C2/I2C3/I2C4/I2C5  Implemented and registered as /dev/i2c2..5.
OSPI NOR             Implemented as /dev/mtd0 plus /dev/ospi0 diagnostics.
HSPI PSRAM           Implemented, mapped, self-tested, /dev/hspiram0, heap add.
eMMC                 Implemented as /dev/mmcsd0 plus read-only /dev/emmcinfo0.
LCD                  Static panel command table, /dev/fb0, color/pattern fill.
Touch                Sitronix path uses NuttX touchscreen upper-half.
STTS22H              WHOAMI, BDU/auto-increment, one-shot /dev/temp0 read.
VL53L5CX             LP wakeup + ID/health/capability only; ranging blocked.
Cube demo UI/images  Not implemented; not needed for NuttX resource bring-up.
Semihost/menu flow   Not implemented; NSH and device nodes are the interface.
```

Static-source completion status:

```text
- Implemented and build-checkable: nodes, registration, non-destructive
  diagnostics, mount-only filesystem policy, framebuffer vtable semantics.
- Requires hardware validation: OSPI JEDEC/read/write timing, PSRAM stability,
  eMMC transfer reliability, LCD timing/panel visibility, touch coordinates,
  sensor electrical presence and real sample quality.
- Intentionally not implemented: automatic NOR/eMMC formatting or destructive
  storage tests at boot, Cube BSP demo menu, fake VL53L5CX range samples.
```

## Build Verification

Verified on 2026-05-07:

```bash
cd /home/uan-wsl2/nuttx-work/nuttx
make distclean
./tools/configure.sh stm32u5x9j-dk:nsh
make -j8
```

Result:

```text
nuttx generated
nuttx.bin generated
nuttx.hex generated
Memory report:
  flash 317692 B / 4 MiB
  sram   11536 B / 768 KiB
```

The first attempt could not download the default littlefs `v2.5.1` tarball from
GitHub. The verified build uses the local ignored littlefs v2.10.2 source copy
with `CONFIG_FS_LITTLEFS_HAS_LFS_DEFINES=y`.

The build no longer emits the STM32U5 SDMMC non-DMA overrun warning because the
default NSH config now enables `CONFIG_STM32U5_SDMMC_IDMA`.

Build artifacts are generated locally and should not be committed.

## Smoke Check

After flashing `nuttx.bin` to internal flash at `0x08000000`:

```text
USART1 VCP: 115200 8N1
```

Expected NSH checks:

```text
ls /dev
free
mount
ls /mnt/flash
ls /mnt/emmc
cat /dev/ospi0
cat /dev/hspiram0
cat /dev/emmcinfo0
cat /dev/temp0
cat /dev/range0
cat /dev/input0
```

Expected boot log highlights when hardware init succeeds:

```text
OSPI1 NOR JEDEC ID ...
OSPI diagnostic at /dev/ospi0
HSPI PSRAM self-test passed
HSPI PSRAM diagnostic at /dev/hspiram0
SDMMC1 eMMC registered as /dev/mmcsd0
eMMC read-only diagnostic at /dev/emmcinfo0
LCD static pattern done
Sitronix id=...
STTS22H id=...
VL53L5CX id=...
```

## Remaining Work

```text
1. Validate OSPI1 NOR erase/page-program on hardware and confirm existing
   littlefs automount at `/mnt/flash`.
2. If destructive NOR validation is needed, enable
   `CONFIG_STM32U5X9J_DK_OSPI_SCRATCH_TEST` only after choosing a safe
   `CONFIG_STM32U5X9J_DK_OSPI_SCRATCH_OFFSET`.
3. Validate SDMMC1/eMMC 8-bit block reads on hardware and confirm existing
   vfat automount at `/mnt/emmc`.
4. Hardware-validate the STM32U5x9J-DK DSI panel command sequence and tune
   DSI/LTDC/GFXMMU timing if /dev/fb0 color bars are not visible.
5. Replace VL53L5CX identity/health node with real ranging after the
   firmware/config table dependency is cleanly handled.
6. Run board smoke tests and tune cache/MPU only after the uncached bring-up is
   stable.
```
