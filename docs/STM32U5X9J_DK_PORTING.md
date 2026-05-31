# STM32U5x9J-DK NuttX Porting Notes

This document tracks the STM32U5x9J-DK port in the Feather super-project.
The current hardware-porting workspace is:

```text
/home/uan/Feather-develop-HW
```

Original development branch context:

```text
Feather: /home/uan-wsl2/Feather       -> main
nuttx:   /home/uan-wsl2/Feather/nuttx -> develop
apps:    /home/uan-wsl2/Feather/apps  -> develop
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
LCD:        480 x 480 DSI panel, LTDC/direct-PSRAM or LTDC/GFXMMU/DMA2D path
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
/dev/fb0        480x480 RGB565 or 32-bit XRGB8888 framebuffer
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
PSRAM FB mode:   framebuffer starts at 0xa0000000, PSRAM heap follows reserve
SRAM FB mode:    framebuffer is reserved below protected user SRAM
```

The framebuffer storage location is selected by
`CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM` or `CONFIG_STM32U5X9J_DK_LCD_FB_SRAM`.
In PSRAM mode the reserved window is 1 MiB for RGB565 and 2 MiB for XRGB8888;
the remaining PSRAM is added to the heap when
`CONFIG_STM32U5X9J_DK_HSPI_HEAP` is enabled.  In protected SRAM mode the
linker script places the compact GFXMMU-backed framebuffer window below
`CONFIG_STM32U5X9J_PROTECTED_USRAM_BASE`, and the board heap code ends the
kernel heap at the framebuffer base.

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

The board registers a framebuffer backed by a board-selected display window.
The framebuffer pixel format and backing memory are board Kconfig choices:

```text
Node:         /dev/fb0
Format:       CONFIG_STM32U5X9J_DK_LCD_RGB565 or
              CONFIG_STM32U5X9J_DK_LCD_XRGB8888
Backing:      CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM or
              CONFIG_STM32U5X9J_DK_LCD_FB_SRAM
Resolution:   480 x 480
Framebuffer:  direct PSRAM window or GFXMMU virtual window
Buffers:      two framebuffers when CONFIG_STM32U5_LTDC_FB_DOUBLE_BUFFER=y
Pattern:      optional startup color bars for bring-up validation
```

The board pixel-format choice drives the framebuffer format exported by
`/dev/fb0`, the LTDC layer format, LVGL's default color depth, and the GFXMMU
bytes-per-pixel calculation when GFXMMU mode is enabled.  The DSI video output
and the HX8379C panel are kept at RGB888 (`COLMOD=0x77`) for both framebuffer
formats, matching the working Cube BSP path and avoiding panel-side RGB565
ambiguity.  The board default is RGB565, which halves the scanout memory
traffic and framebuffer footprint compared with 32-bit XRGB8888.

The board backing-memory choice drives the physical framebuffer reserve, heap
layout, and GFXMMU policy.  PSRAM-backed LVGL uses direct linear scanout and
does not select `CONFIG_STM32U5_GFXMMU`.  Internal SRAM-backed LVGL selects
`CONFIG_STM32U5_GFXMMU` and reserves compact circular framebuffer storage below
the protected user SRAM window.  The board default is
`CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM=y`.

The 32-bit choice is named `CONFIG_STM32U5X9J_DK_LCD_XRGB8888` because that is
the fbdev format exported to LVGL (`FB_FMT_RGB32`/XRGB8888), matching LVGL's
`LV_COLOR_DEPTH_32` mode.  The LTDC hardware layer still uses the STM32
ARGB8888 pixel-format packing and is configured with constant alpha, so the
final framebuffer is displayed as fully opaque instead of consuming per-pixel
alpha left by LVGL rendering.

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
- `CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM` uses direct linear PSRAM framebuffers
- `CONFIG_STM32U5X9J_DK_LCD_FB_SRAM` uses GFXMMU-backed internal SRAM storage
- LTDC layer 1 is configured as RGB565 or 32-bit XRGB8888 and exposed through NuttX fb
- DSI host runs video mode with RGB888 output toward the panel
- HX8379C initialization lives in the common LCD driver
- DMA2D has a simple register-to-memory fill helper used by colorbar setup
- Board LCD init performs panel power/reset, vendor init, sleep-out,
  display-on, framebuffer fill, DSI/LTDC enable, and /dev/fb0 registration
- LTDC framebuffer vtable exposes getvideoinfo, getplaneinfo, open, close,
  getpower, setpower, update, pan-display, and wait-for-vsync semantics used by
  LVGL's NuttX fbdev backend
```

### 2026-05-24 LCD/LVGL Hardware Findings

The LCD path was hardware-validated with both the startup framebuffer color bar
and `lvgldemo` on STM32U5x9J-DK.  An early GFXMMU-backed RGB565 bring-up log
looked like:

```text
stm32u5x9j: LCD fb-format=RGB565 fb-bpp=16 dsi-format=RGB888 dsi-bpp=32 panel-colmod=0x77 phys-stride=960 layer-stride=3072 fb-count=2
stm32u5: DSI video bpp=32 colorcoding=RGB888 bytespp=3
stm32u5x9j: LCD framebuffer colorbar done
stm32u5x9j: /dev/fb0 framebuffer virt=0x24000000 virt1=0x24400000 phys=0xa0000000 size=921600 count=2
```

The GFXMMU is programmed in `192BM` mode, so one GFXMMU virtual line is always:

```text
192 blocks * 16 bytes = 3072 bytes
```

This value is the LTDC layer stride through the GFXMMU virtual address space.
It is not the physical PSRAM framebuffer stride.  Physical framebuffer storage
is still tightly sized by the selected framebuffer format:

```text
RGB565 physical stride:    480 * 2 = 960 bytes
XRGB8888 physical stride:  480 * 4 = 1920 bytes
GFXMMU/LTDC layer stride:  3072 bytes for both formats
```

The visible layer width is still 480 pixels.  The remaining virtual-line bytes
are part of the GFXMMU address layout and must be preserved.  For RGB565 this
means the virtual line contains `3072 / 2 = 1536` addressable pixels; for
XRGB8888 it contains `3072 / 4 = 768` addressable pixels.  The board code must
index CPU test fills and color bars with `layer_pixels`, not with the physical
display width, whenever it writes through the GFXMMU virtual framebuffer.

Issue history and fixes:

```text
Symptom:
  LVGL logs showed FBIO_UPDATE/FBIOPAN_DISPLAY activity, but the panel stayed
  on the old startup color bars.
Fix:
  Complete the DSI video-mode setup, keep the panel in RGB888, and avoid
  command-mode refresh assumptions.  In video mode LTDC/DSI continuously scans
  the selected framebuffer; LVGL only draws and requests the pan/update.

Symptom:
  FBIO_WAITFORVSYNC returned ETIMEDOUT while the panel was otherwise updating.
Fix:
  Treat the current DSI video path as not providing the LTDC refresh interrupt
  used by the generic wait path.  The fbdev wait path is allowed to complete
  once the LTDC shadow reload has been issued.

Symptom:
  RGB565 startup color bars and LVGL output showed horizontal striping or
  corrupted circular content.
Root cause:
  The RGB565 layer stride was reduced to 1536 bytes.  That is only
  `768 * 2`, but the GFXMMU hardware virtual line is still 3072 bytes in
  192BM mode.  LTDC therefore fetched subsequent lines from the wrong virtual
  addresses.
Fix:
  Keep `layer-stride=3072` for RGB565 and the 32-bit format.  Only the physical
  framebuffer stride and byte count change with the selected format.

Symptom:
  32-bit protected LVGL could show unexpected color/alpha artifacts.
Root cause:
  LVGL's NuttX 32-bit fbdev path is XRGB8888, while the LTDC layer was using
  pixel-alpha blending as if the final framebuffer alpha were meaningful.
Fix:
  Export the 32-bit framebuffer as `FB_FMT_RGB32` and configure LTDC layer
  blending with constant alpha so the final display surface is fully opaque.

Symptom:
  32-bit protected LVGL no longer crashed, but the benchmark screen still had
  horizontal corruption and dark bands similar to earlier GFXMMU failures.
Root cause:
  Double buffering was modeled as a 960-line framebuffer inside GFXMMU virtual
  buffer 0 (`0x24000000` plus one virtual frame).  The U5 GFXMMU instead has
  separate 4 MiB virtual buffers (`0x24000000`, `0x24400000`, ...), each using
  the same 480-line LUT and its own physical buffer base register.  Treating the
  second framebuffer as extra LUT lines made the LTDC scanout depend on an
  address layout that does not match the hardware model or the Cube BSP LUT.
Fix:
  Program one 480-line circular LUT, set GFXMMU B0/B1 physical bases to the two
  PSRAM framebuffers, expose fb0 at `0x24000000` and fb1 at `0x24400000`, and
  keep fbdev pan offsets logical (`0` and `480`).  LVGL's NuttX fbdev backend
  now uses the second plane's reported `yoffset` for non-consecutive memory
  instead of deriving yoffset from the raw address gap.

Symptom:
  DCache improved LVGL benchmark performance, but framebuffer output could
  become stale if cache maintenance was missing.
Fix:
  Clean the framebuffer/GFXMMU ranges after board startup test fills and clean
  updated flush ranges before LTDC scans them.
```

The practical rule is:

```text
Kconfig choice:
  CONFIG_STM32U5X9J_DK_LCD_RGB565
  CONFIG_STM32U5X9J_DK_LCD_XRGB8888

Changes with the choice:
  /dev/fb0 format, LVGL color depth, LTDC pixel format, GFXMMU bpp,
  physical framebuffer stride, physical framebuffer size.

Does not change with the choice:
  DSI output format, panel COLMOD, GFXMMU 192BM virtual-line width,
  LTDC layer stride through the GFXMMU window.
```

Memory-layout note:

```text
layer-stride = 3072 bytes
  GFXMMU virtual address stride consumed by LTDC.  This may look wasteful in
  RGB565, but it is not physical PSRAM framebuffer storage.  It must remain
  aligned with the GFXMMU 192BM virtual-line layout.

direct PSRAM physical framebuffer storage
  RGB565:   480 * 480 * 2 * 2 = 921600 bytes for double buffering
  XRGB8888: 480 * 480 * 4 * 2 = 1843200 bytes for double buffering
```

Double buffering uses separate GFXMMU virtual buffers rather than a single
contiguous virtual framebuffer:

```text
GFXMMU buffer 0: 0x24000000 -> physical frame 0
GFXMMU buffer 1: 0x24400000 -> physical frame 1
fbdev logical yoffsets: 0 and 480
```

Both GFXMMU virtual buffers must be mapped into the protected user address
space, because LVGL writes directly to both draw buffers.

The current direct-PSRAM framebuffer reserve remains rectangular because PSRAM
capacity is plentiful and the stable path avoids GFXMMU entirely.  The
internal-SRAM framebuffer option uses the compact circular GFXMMU storage
instead, because SRAM capacity is scarce and SRAM has enough latency margin for
GFXMMU scanout.

Recommended cleanup direction:

```text
Short term:
  Keep the PSRAM path direct and rectangular, and name the 3072-byte value as
  GFXMMU virtual-line bytes rather than as a framebuffer stride.

Longer term:
  Move the 192BM details into stm32_gfxmmu.  The board LCD code should provide
  width, height, bpp, frame count, and circular=true; the GFXMMU helper should
  report virt_stride, virt_frame_size, and the physical frame size/reserve that
  the board must expose to LTDC and fbdev.

Internal-SRAM path:
  Pack only the circular visible blocks through GFXMMU.  Do not use this
  optimization for PSRAM-backed LVGL scanout unless the LTDC FIFO underrun
  margin has been revalidated.
```

### 2026-05-25 PSRAM Framebuffer And GFXMMU Policy

The final KNSh/LVGL fix for PSRAM-backed framebuffer scanout is to select the
PSRAM framebuffer location and expose the reserved PSRAM display window
directly to `/dev/fb0`:

```text
CONFIG_STM32U5X9J_DK_LCD_RGB565=y
CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM=y
# CONFIG_STM32U5X9J_DK_LCD_FB_SRAM is not set
```

This is the default board LVGL policy.  The scripts also keep
`CONFIG_STM32U5_ICACHE=y` and `CONFIG_STM32U5_DCACHE1=y`; in protected KNSh the
PSRAM user heap/stack MPU policy is write-back, non-shareable,
no-write-allocate.  Disabling cache makes PSRAM-backed user stacks and LVGL
object traffic slower and increases pressure on the LTDC scanout path.

The expected RGB565 direct-framebuffer boot log is:

```text
stm32u5x9j: LCD fb-map=direct fb-format=RGB565 fb-bpp=16 dsi-format=RGB888 dsi-bpp=32 panel-colmod=0x77 phys-stride=960 layer-stride=960 fb-count=2
stm32u5: DSI video bpp=32 colorcoding=RGB888 bytespp=3 pclk=15625000Hz lane-byte=62500000Hz hscale=4 refresh=59.34Hz
stm32u5x9j: LCD framebuffer colorbar done
stm32u5x9j: /dev/fb0 framebuffer virt=0xa0000000 virt1=0xa0070800 phys=0xa0000000 size=921600 count=2
```

LVGL then sees the two framebuffer planes as:

```text
plane 0: mem=0xa0000000 fblen=460800 stride=960 bpp=16 yoffset=0
plane 1: mem=0xa0070800 fblen=460800 stride=960 bpp=16 yoffset=480
```

This removes the earlier stride warning caused by the GFXMMU virtual buffer gap
and fixes the dynamic-refresh corruption.  The symptom before this change was
that static color bars were stable, but `lvgldemo`, widgets, containers, or even
interactive shell redraws could produce transient corruption.  When instrumented,
LTDC reported repeated FIFO underruns:

```text
stm32u5: LTDC irq error=00000002 ... count=...
```

`0x00000002` is `FUIF`, the LTDC FIFO underrun flag.  Temporary double-buffer
ownership instrumentation showed no LVGL/LTDC same-buffer conflict, so the
failure was not LVGL drawing into the same framebuffer that LTDC was scanning.
It was the scanout fetch path itself missing its real-time latency budget.

In DSI video mode, DSI continuously consumes pixels from LTDC.  LTDC therefore
must fetch one full 480x480 RGB888 panel stream every frame, even when the
application is idle.  With the GFXMMU path enabled for PSRAM, LTDC fetched from
the virtual windows at `0x24000000` and `0x24400000`, through a 3072-byte
GFXMMU virtual stride, into compact PSRAM storage with a 960-byte RGB565
physical stride.  That saves physical framebuffer bytes, but it adds GFXMMU
translation and less linear burst behavior in front of already-latency-sensitive
PSRAM.  During LVGL rendering, CPU PSRAM writes and cache-clean traffic were
enough to starve LTDC occasionally, causing the visible corruption.

Disabling cache is not the fix.  It made the protected LVGL case slower and
more fragile because CPU-owned user heap, stack, and LVGL object traffic then
hit PSRAM directly instead of being absorbed by DCACHE1.  The current policy is
to keep CPU-owned PSRAM cacheable, clean framebuffer ranges before LTDC scanout,
and remove GFXMMU from the PSRAM scanout path.

Framebuffer placement policy:

```text
Internal SRAM framebuffer:
  CONFIG_STM32U5X9J_DK_LCD_FB_SRAM=y
  Selects CONFIG_STM32U5_GFXMMU.  The linker script reserves compact circular
  framebuffer storage below CONFIG_STM32U5X9J_PROTECTED_USRAM_BASE, and the
  board heap code ends the kernel heap at the framebuffer base.  GFXMMU is
  useful here because it reduces real framebuffer storage and internal SRAM has
  enough deterministic bandwidth/latency for LTDC scanout.

External PSRAM framebuffer:
  CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM=y
  Use direct linear PSRAM framebuffer scanout.  PSRAM capacity is plentiful,
  but the LTDC/GFXMMU/PSRAM path has too little latency margin for dynamic LVGL
  scenes.  This mode does not select CONFIG_STM32U5_GFXMMU.
```

The compact internal-SRAM GFXMMU reserve for the current 480x480 circular panel
is:

```text
RGB565:   0x59d80 bytes per frame, 0x0b3b00 bytes for double buffering
XRGB8888: 0xb20c0 bytes per frame, 0x164180 bytes for double buffering
```

DSI command mode is not a primary fix for this failure.  It can avoid a
continuous video scanout only if the application uses small partial updates, but
full-screen LVGL benchmark scenes still need roughly the same pixel throughput
and move the pressure into command/pixel-push transfers.  The direct PSRAM video
mode path is the verified stable path for this board.

Protected builds need one MPU detail for the direct path: framebuffer memory may
now be an external PSRAM address, not only an internal/GFXMMU virtual address.
`stm32_mpu_ufbmem()` therefore maps external framebuffer ranges with the same
selectable STM32U5 PSRAM user attributes used by the external user heap.

### 2026-05-24 Protected KNSh LVGL Notes

The protected `knsh-lvgl` image has one extra stack constraint that the flat
`nsh-lvgl` image does not expose.  Protected builds default to
`CONFIG_TLS_ALIGNED=y`, and the default `CONFIG_TLS_LOG2_MAXSTACK=13` limits
task stacks to 8192 bytes.  `lvgldemo` uses a 32768-byte stack, so leaving the
default TLS limit in place causes this assert as soon as the command creates
the demo task:

```text
Assertion failed : common/arm_createstack.c:93
```

The `tools/firmware/stm32u5x9j-dk/build-knsh-lvgl.sh` script therefore pins the
protected LVGL stack-related settings:

```text
CONFIG_EXAMPLES_LVGLDEMO_STACKSIZE=32768
CONFIG_TLS_LOG2_MAXSTACK=15
CONFIG_MM_KERNEL_HEAPSIZE=32768
CONFIG_ARMV8M_LAZYFPU=y
CONFIG_ARMV8M_BASEPRI_ISB=y
CONFIG_ARMV8M_SYSCALL_RETURN_CURRENT_FRAME=y
CONFIG_ARMV8M_SYSCALL_RETURN_USER_BASEPRI0=y
CONFIG_ARMV8M_SYSCALL_DISPATCH_BASEPRI0=y
CONFIG_ARMV8M_SYSCALL_KERNEL_STACK=y
CONFIG_ARMV8M_SYSCALL_KERNEL_STACKSIZE=8192
CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP=y
CONFIG_STM32U5_ICACHE=y
CONFIG_STM32U5_DCACHE1=y
CONFIG_STM32U5_PSRAM_MPU_SHARE_NONE=y
CONFIG_STM32U5_PSRAM_MPU_WRITE_BACK=y
CONFIG_STM32U5_PSRAM_MPU_NO_WRITE_ALLOCATE=y
CONFIG_STM32U5X9J_DK_LCD_RGB565=y
CONFIG_STM32U5X9J_DK_LCD_FB_PSRAM=y
# CONFIG_STM32U5X9J_DK_LCD_FB_SRAM is not set
```

This matches the same protected-build direction used by the STM32N6570-DK LVGL
image: the user heap is bootstrapped from internal user SRAM and then extended
with PSRAM, while protected syscall handling gets an internal kernel stack so
blocking kernel paths do not run entirely on PSRAM-backed user stacks.

Seeing a user task stack address in the `0xa0......` PSRAM range is expected
once `CONFIG_STM32U5X9J_DK_HSPI_HEAP=y` has appended PSRAM to the user heap.
That placement is a performance and cache-policy validation point, but it was
not the immediate cause of the `arm_createstack.c:93` failure.  The immediate
cause was the mismatch between a 32768-byte LVGL task stack and an 8192-byte
TLS maximum.

Startup color bars are controlled separately by
`CONFIG_STM32U5X9J_DK_LCD_COLORBAR`.  The LVGL build scripts enable the
colorbar as a cheap scanout sanity check; validation should still use the
`/dev/fb0` registration log and `lvgldemo` output for dynamic refresh.

If 32-bit LVGL shows unexpected transparency, dark patches, or color shifts,
check that `/dev/fb0` reports `FB_FMT_RGB32` and that LTDC blending uses
constant alpha (`CA`/`1-CA`) rather than pixel-alpha blending (`PAXCA`).  LVGL's
32-bit NuttX fbdev path is XRGB8888, while the DSI stream and panel remain
RGB888.

Protected LVGL also writes the mapped framebuffer directly.  In the verified
PSRAM path, `/dev/fb0` exposes direct PSRAM buffers at `0xa0000000` and
`0xa0070800`.  In optional GFXMMU mode, `/dev/fb0` exposes GFXMMU virtual
buffers at `0x24000000` and `0x24400000`.  Board LCD bring-up maps the selected
framebuffer span as user read/write before registering `/dev/fb0`.  Without
that MPU region, `lvgldemo` can open fbdev but faults in the first clear/draw
path with a MemFault in the framebuffer address range.

The protected `knsh-lvgl` build now also mirrors the STM32N6570-DK PSRAM heap
policy.  The initial protected user heap stays in a small internal user SRAM
window (`CONFIG_STM32U5X9J_PROTECTED_USRAM_BASE=0x20250000`,
`CONFIG_STM32U5X9J_PROTECTED_USRAM_SIZE=0x20000`,
`CONFIG_STM32U5X9J_PROTECTED_UHEAP_SIZE=0x2000`) so `nx_start()` and the early
allocator metadata do not depend on external memory.  The remaining contiguous
internal SRAM below `0x20250000` is left to the kernel heap in PSRAM
framebuffer mode.  In internal-SRAM framebuffer mode, the compact framebuffer
reserve is carved out immediately below `0x20250000` and the kernel heap ends
at that framebuffer base.  `arm_addregion()` then appends HSPI1 PSRAM as a
secondary user heap, requiring
`CONFIG_MM_REGIONS=2` and `CONFIG_STM32U5X9J_DK_HSPI_HEAP=y`.

The important fix is that CPU-owned PSRAM user heap/stack memory is not mapped
with the same non-cacheable attributes used for shared framebuffer windows.
`stm32_mpu_uheap()` now keeps internal SRAM heaps on the internal-SRAM policy,
but maps external user heaps with the selectable STM32U5 PSRAM MPU policy:

```text
CONFIG_STM32U5_PSRAM_MPU_SHARE_NONE=y
CONFIG_STM32U5_PSRAM_MPU_WRITE_BACK=y
CONFIG_STM32U5_PSRAM_MPU_NO_WRITE_ALLOCATE=y
```

This is for CPU-owned heap and stack traffic.  PSRAM ranges shared with LTDC
still need explicit cache maintenance before scanout.  The observed failure
mode before this split was that static framebuffer content could look correct,
while dynamic LVGL refresh or even interactive `nsh` input caused transient
corruption and much worse benchmark numbers.  That pointed at cache-ineffective
PSRAM heap/stack traffic in the protected user path.  The later `FUIF`
investigation separated the remaining scanout problem and fixed it by bypassing
GFXMMU for PSRAM framebuffers.

### 2026-05-31 External AMOLED and LVGL Regression Notes

Additional display validation was done in the hardware-porting workspace:

```text
/home/uan/Feather-develop-HW
```

Two external 1-lane AMOLED panels are now available as STM32U5x9J-DK
validation targets in addition to the official HX8379C panel:

```text
CO5300  1.73 inch, 466x466, RGB565, 1-lane MIPI DSI
ST7801  2.13 inch, 410x502, RGB565, 1-lane MIPI DSI
```

New panel drivers and board configs:

```text
drivers/lcd/co5300.c
drivers/lcd/st7801.c
include/nuttx/lcd/co5300.h
include/nuttx/lcd/st7801.h
boards/arm/stm32u5/stm32u5x9j-dk/configs/nsh-co5300
boards/arm/stm32u5/stm32u5x9j-dk/configs/nsh-st7801
tools/firmware/stm32u5x9j-dk/build-nsh-co5300.sh
tools/firmware/stm32u5x9j-dk/build-nsh-st7801.sh
```

Effective external-AMOLED baselines:

```text
CO5300:
  resolution: 466x466
  fb/dsi format: RGB565
  lanes: 1
  pclk: 16 MHz
  lane-byte clock: 50 MHz, current stable 400 Mbps lane baseline
  hscale: 2
  DSI PHY swap: clock lane only, phy-swap=01
  verified: framebuffer colorbar

ST7801:
  resolution: 410x502
  fb/dsi format: RGB565
  lanes: 1
  pclk: 15 MHz
  lane-byte clock: 52.5 MHz, current stable 420 Mbps lane baseline
  hscale: 3.500, represented as hscale_num=7 / hscale_den=2
  DSI PHY swap: clock lane only, phy-swap=01
  verified: framebuffer colorbar
```

Important implementation notes:

```text
- DCS read/BTA support is kept for panel ID/status checkpoints.
- Clock-lane P/N swap is required for the current flying-wire/adapter setup.
- ST7801 must keep fractional horizontal timing scale 3.5; integer hscale=2
  caused visible artifacts.
- CO5300 must keep its validated integer hscale=2 path.
- DSI wrapper pattern-generator diagnostics, solid-fill diagnostics, register
  dump helpers, and verbose per-packet debug logs have been removed from the
  formal build paths.
```

Latest build checks after cleanup and NuttX-style formatting:

```text
./FeatherCore/tools/firmware/stm32u5x9j-dk/build-nsh-co5300.sh -j8
  -> FeatherCore/build/stm32u5x9j-dk-nsh-co5300.bin
  -> 126980 bytes

./FeatherCore/tools/firmware/stm32u5x9j-dk/build-nsh-st7801.sh -j8
  -> FeatherCore/build/stm32u5x9j-dk-nsh-st7801.bin
  -> 126988 bytes

./FeatherCore/tools/firmware/stm32u5x9j-dk/build-knsh-lvgl.sh -j8
  -> FeatherCore/build/stm32u5x9j-dk-knsh-lvgl.bin
  -> 1148576 bytes
```

The CO5300 and ST7801 NSH colorbar validation builds compile after cleanup.
The official HX8379C protected `knsh-lvgl` build also compiles, but the
runtime LVGL path currently has a regression that must be fixed before treating
the official LVGL path as healthy.

Observed `knsh-lvgl` runtime issue, 2026-05-31:

```text
LCD panel=HX8379C fb-map=direct fb-format=RGB565 fb-bpp=16
dsi-format=RGB888 dsi-bpp=32 dsi-lanes=2 panel-colmod=0x77
phys-stride=960 layer-stride=960 fb-count=2

/dev/fb0 framebuffer virt=0xa0000000 virt1=0xa0070800
phys=0xa0000000 size=921600 count=2

lvgldemo opens /dev/fb0 successfully:
  xres=480 yres=480 nplanes=1
  plane0 mem=0xa0000000 fblen=460800 stride=960 display=0 bpp=16
  plane1 mem=0xa0070800 fblen=460800 stride=960 display=1 bpp=16

lvgldemo opens /dev/input0 successfully.

Then every LVGL flush fails:
  stm32u5: LTDC pan busy pending=1 srcr=00000000 isr=00000000
  [LVGL] flush_cb: ioctl(FBIOPAN_DISPLAY) failed: 16
```

Interpretation:

```text
- Device registration and LVGL fbdev/touch opening are working.
- The failure is in the dynamic pan-display path, not in framebuffer open.
- errno 16 is EBUSY from the board/LTDC pan path.
- The LTDC state shows pending=1 while SRCR is 0 and ISR is 0, so the pending
  state is not being retired by the current reload/vblank completion logic.
- Static startup colorbar still completes, so this is a dynamic LVGL
  page-flip/pan regression to debug separately.
```

Next debug priority:

```text
1. Inspect the LTDC pan-display state machine and the condition that leaves
   pending=1 after the first LVGL flip.
2. Confirm whether the immediate reload/global vertical blank reload interrupt
   is expected in the current DSI video path.
3. Recheck FBIOPAN_DISPLAY and FBIO_WAITFORVSYNC semantics against the NuttX
   fbdev backend used by LVGL 9.2.2.
4. Keep CO5300/ST7801 colorbar baselines unchanged while fixing HX8379C/LVGL.
```

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
- DSI configurable pixel PLL, PHY lane rate, BTA, lane P/N swap, video mode,
  and fractional horizontal timing scale for external AMOLED validation
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
cd /home/uan-wsl2/Feather/nuttx
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
LCD framebuffer colorbar done
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
4. Fix the current `knsh-lvgl` runtime regression where LVGL opens `/dev/fb0`
   and `/dev/input0`, but `FBIOPAN_DISPLAY` repeatedly returns `EBUSY` with
   `LTDC pan busy pending=1`.
5. Replace VL53L5CX identity/health node with real ranging after the
   firmware/config table dependency is cleanly handled.
6. Keep RGB565 and 32-bit XRGB8888 LCD/LVGL builds in the hardware smoke-test set;
   the PSRAM-backed path should stay direct-linear, while any future SRAM-backed
   memory-saving path must revalidate the GFXMMU 192BM virtual stride rule.
7. Continue LVGL performance tuning only after framebuffer cache maintenance,
   direct PSRAM scanout, PSRAM heap placement, and pan-display completion remain
   stable across both LCD formats.
```
