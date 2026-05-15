# STM32N6570-DK NuttX Porting Notes

## Current Direction

- Branch: `vendor/stm32n6-bringup`
- SoC family: `arch/arm/src/stm32n6`
- Board: `boards/arm/stm32n6/stm32n6570-dk`
- Reference: `STM32CubeN6/Projects/STM32N6570-DK/Templates/Template_FSBL_XIP_Custom`

The STM32N6 has no internal Flash.  The selected boot model is
**NXboot with an ST trusted FSBL wrapper**:

1. STM32N6 BootROM loads an ST trusted FSBL image from XSPI2 NOR
   `0x70000000`.
2. The trusted payload is NuttX/NXboot, linked at `0x34180400`.
3. NXboot reads an app image at `0x70100000`.
4. The app image uses an NXboot header of `0x400`; its vector table starts
   at `0x70100400`.

The ST BootROM header and the NXboot app header are separate layers.  The
FSBL image is signed/packed by STM32 SigningTool; the application is packed
by `apps/boot/nxboot/tools/nximage.py`.

## Implemented In This Checkpoint

- Added `ARCH_CHIP_STM32N6` and `ARCH_CHIP_STM32N657X0`.
- Added STM32N6 IRQ/chip headers based on Cube CMSIS IRQ numbers.
- Added minimal STM32N6 startup, IRQ, SysTick, USART1 lowputc, serial init,
  RCC/PWR bring-up, and XSPI init interfaces.
- Added app-side USART1 serial lower-half:
  - USART1 TX `PE5`, RX `PE6`, 115200 8N1.
  - `nsh` registers `/dev/console` and `/dev/ttyS0`.
  - `nxboot` keeps the smaller early/syslog-only path.
- Added `stm32n6570-dk:nxboot`, `stm32n6570-dk:nsh`, and
  `stm32n6570-dk:knsh`.
- Added linker scripts:
  - NXboot: RAM payload at `0x34180400`, length `511 KiB`.
  - App: XIP at `0x70100400`, RAM at `0x34000000`.
  - Protected KNSh app: NXboot header at `0x70100000`, kernel vector at
    `0x70100400`, user blob at `0x70180400`.
- Implemented the Cube 800 MHz first-stage clock profile in pure NuttX
  register code:
  - DK PF4 selects external SMPS overdrive before VOS is raised.
  - VOS scale 0.
  - HSI -> PLL1, `M=8/N=100/P1=1/P2=1`.
  - CPU clock `800 MHz`.
  - SYS/AXI class clock `400 MHz`.
  - HCLK/PCLK class clock `200 MHz`.
- Added stable post-lowsetup boot logging for the first-stage path:
  - PWR VDDIO2/VDDIO3 ready status and `SVMCR3`.
  - RCC status and `SR/CFGR1/CFGR2` snapshots.
  - CPU/SYS/HCLK/PCLK frequency profile.
- Added PWR/BSEC/SYSCFG/RCC minimum register definitions for this first-stage
  boot path.
- Added read-only OTP124 HSLV detection:
  - VDDIO3 HSLV controls whether XSPI2 NOR may use the 200 MHz path.
  - VDDIO2 HSLV controls whether XSPI1 PSRAM may use the 200 MHz path.
  - The port does not burn OTP; missing fuses are reported and the affected
    XSPI bus stays on the conservative 50 MHz path.
- Added XSPI startup/optional clock diagnostics:
  - XSPI1/XSPI2 source clock and XSPIM common setup state.
  - Startup prescaler/effective clock for NOR and PSRAM.
  - Optional memory-mapped prescaler/effective clock, including OTP-based
    high-speed/fallback diagnostics.
- Implemented XSPI2 NOR `MX66UW1G45G` boot-read path:
  - GPION AF9 pin setup.
  - 1S startup reset and JEDEC ID read.
  - Cube-style CFG2 DOPI/OPI register setup.
  - 8D-8D-8D OPI/DTR memory-mapped read mode at `0x70000000`.
  - Diagnostic print for JEDEC ID and the primary NXboot slot header word at
    `0x70100000`.
- Implemented XSPI1 PSRAM `APS256XX` boot path:
  - GPIOO/GPIOP AF9 pin setup.
  - 8S-8D-8D startup register mode.
  - MR0/MR4/MR8 configuration and readback.
  - STM32CubeN6-aligned stable settings: MR0 `0x30`, MR4 `0x20`,
    MR8 `0x40`, linear-burst read command `0x20`, linear-burst write command
    `0xa0`, FIFO threshold 8 bytes, and refresh enabled.
  - 8S-8D-16D memory-mapped mode at `0x90000000`.
  - PSRAM startup register access and self-test stay at 50 MHz.
  - The final memory-mapped target is 200 MHz when VDDIO2 HSLV is available;
    otherwise PSRAM remains mapped at 50 MHz as a defensive fallback.
  - H7S78-style startup self-test during PSRAM initialization.
- Added board-level NXboot slot MTD registration skeleton:
  - `/dev/mtd0`
  - `/dev/ota0`
  - `/dev/ota1`
  - `/dev/ota2`
- XSPI2 NOR is treated as raw XIP/OTA image storage only:
  - no `/mnt/flash`
  - no automount
  - no default littlefs/FTL/format path
  - future data storage must use a separate tail partition and is not part of
    this checkpoint
- Extended XSPI2 NOR MTD support:
  - `MX66UW1G45G` 4 KiB subsector erase, 64 KiB block erase, and 256 B page
    program commands.
  - `CONFIG_STM32N6_EXTNOR_WRITE=y` in `nsh`.
  - `nxboot` does not enable write/erase and never performs automatic
    destructive tests.
  - NOR write/erase helpers are linked into `.ramfunc` and copied to SRAM
    before use, because the app itself executes XIP from the same NOR.
  - Each write/erase exits memory-mapped mode, performs indirect OPI/DTR
    commands, then restores XIP read mode.
- External memory bring-up now registers MTD/OTA nodes only after NOR and
  PSRAM initialization both succeed, and logs each registration result.
- Added `BOARDIOC_BOOT_IMAGE` handoff for NXboot app vectors.
- Added app-side optional PSRAM heap support:
  - `CONFIG_STM32N6_PSRAM_HEAP=y`
  - default heap region `0x90200000..0x91ffffff`
  - first `2 MiB` left free for future framebuffer or early bring-up use.
- Added protected KNSh heap split:
  - user static `.data/.bss` uses the top `128 KiB` of the app SRAM window:
    `0x341e0000..0x341fffff`
  - remaining internal SRAM below `0x341e0000` is kernel SRAM and runtime
    kernel heap
  - XSPI1 PSRAM `0x90000000..0x91ffffff` is mapped as the user heap
- Added Feather firmware helper scripts:
  - `tools/firmware/stm32n6570-dk/pack-stm32-fsbl-nxboot.sh`
  - `tools/firmware/stm32n6570-dk/pack-nxboot-header-app.sh`
  - `tools/firmware/stm32n6570-dk/build-knsh.sh`
- Aligned the board structure with `stm32h7s78-dk` where applicable:
  - `stm32n6570-dk:nxboot` / `stm32n6570-dk:nsh` config naming.
  - board-level CMake source and linker-script selection.
  - board private header for bring-up helper prototypes.
  - optional procfs mount in late initialization when `CONFIG_FS_PROCFS=y`.
- Split STM32N6 XSPI layering to match the H7RS style:
  - `arch/arm/src/stm32n6/stm32n6_xspi.c` now only provides controller-level
    XSPI helpers, clock/HSLV setup and memory-mapped mode programming.
  - STM32N6570-DK-specific MX66UW1G45G NOR and APS256 PSRAM commands,
    timing, erase/write/self-test logic live in board source.
  - NXboot/OTA/external NOR/PSRAM Kconfig options live in the board Kconfig,
    not the STM32N6 SoC Kconfig.
  - PSRAM heap extension is implemented by the board overriding the weak
    arch `arm_addregion()` default.

## External Memory Layout

```text
0x70000000  ST BootROM header + signed NXboot payload
0x70100000  NXboot primary app image header
0x70100400  app vector table / app text
0x70180400  protected KNSh user blob base
0x72100000  reserved secondary app slot
0x74100000  reserved tertiary app slot
0x76100000  tail reserve for future data/scratch use, not used as a default
            filesystem partition in this checkpoint

0x90000000  XSPI1 PSRAM base
0x90000000  protected KNSh user heap base, size 32 MiB
0x90200000  flat NSH app-side PSRAM heap base, default size 30 MiB
```

## Current Runtime Expectations

NXboot serial log should include:

- `N6`
- `stm32n6: PWR VDDIO2/3 status=... SVMCR3=...`
- `stm32n6: clock status=... SR=... CFGR1=... CFGR2=...`
- `stm32n6: clock CPU=800000000 SYS=400000000 HCLK=200000000 PCLK1=200000000 PCLK2=200000000`
- `stm32n6: HSLV OTP124=... VDDIO2=... VDDIO3=...`
- `stm32n6: XSPI1 source=...Hz XSPI2 source=...Hz XSPIM_CR=...`
- `XSPI2 NOR startup prescaler=... effective=...Hz`
- `XSPI2 NOR JEDEC ID c2 81 3b`
- `XSPI2 NOR OPI/DTR config readback 02`
- `XSPI2 NOR optional prescaler=... effective=...Hz`
- `XSPI2 NOR mapped 0x70000000 ota0[0]=...`
- `XSPI1 PSRAM startup prescaler=3 effective=50000000Hz refresh=96`
- `XSPI1 PSRAM MR00000000 initial 08 write 30 readback 30`
- `XSPI1 PSRAM MR00000004 initial 40 write 20 readback 20`
- `XSPI1 PSRAM MR00000008 initial 05 write 45 readback 45`
- `XSPI1 PSRAM optional prescaler=0 effective=200000000Hz refresh=396`
- `XSPI1 PSRAM self-test passed`
- `XSPI1 PSRAM mapped at 0x90000000`
- `stm32n6: registered /dev/mtd0`
- `stm32n6: registered /dev/ota0`
- `stm32n6: registered /dev/ota1`
- `stm32n6: registered /dev/ota2`

If no app image has been burned at `0x70100000`, NXboot should still end with
`Could not find bootable image`.  That is expected until a valid NXboot app
image is present.

When the flat NSH app boots with `CONFIG_STM32N6_PSRAM_HEAP=y`, early heap
setup calls `stm32n6570_xspi1_psram_initialize()` idempotently and then adds
the PSRAM heap region starting at `0x90200000`.  Protected KNSh uses the same
PSRAM initialization path but gives the whole PSRAM window to the user heap
and keeps the remaining internal SRAM for the kernel heap.

App-side device expectations after a valid jump:

- `/dev/console`
- `/dev/ttyS0`
- `/dev/mtd0`
- `/dev/ota0`
- `/dev/ota1`
- `/dev/ota2`

Useful non-automatic checks:

```bash
ls /dev
```

External NOR is not mounted as a filesystem by default.  `/dev/mtd0` and
`/dev/ota0..2` are raw image-storage interfaces for XIP and NXboot updates.
Startup never erases, formats, or writes the external NOR automatically.

## Hardware Validation Checkpoint

On 2026-05-14, the protected KNSh path reached the NSH prompt on
STM32N6570-DK hardware with the 50 MHz PSRAM mapping:

```text
XSPI2 NOR JEDEC ID c2 81 3b
XSPI2 NOR OPI/DTR config readback 02
XSPI2 NOR mapped 0x70000000 ota0[0]=0x534f584e
XSPI1 PSRAM MR00000000 initial 08 write 30 readback 30
XSPI1 PSRAM MR00000004 initial 40 write 20 readback 20
XSPI1 PSRAM MR00000008 initial 05 write 45 readback 45
XSPI1 PSRAM optional prescaler=3 effective=50000000Hz refresh=96
XSPI1 PSRAM self-test passed
Found bootable image, boot from primary.
Boot vector msp=0x34003400 reset=0x70101861 vtor=0x70100400
stm32n6: user SRAM bootstrap heap base=0x341e0290 size=0x00002000 psram=0x90000000 psram-size=0x02000000
stm32n6: kernel SRAM heap base=0x34003400 size=0x001dcc00 end=0x341e0000
stm32n6: added PSRAM heap base=0x90000000 size=0x02000000
NuttShell (NSH)
nsh>
```

This confirms the BootROM FSBL wrapper, NuttX/NXboot handoff, XSPI2 XIP NOR,
protected kernel/user image layout, USART1 console, and PSRAM-backed user heap
all worked at the conservative PSRAM clock.

The current high-speed target is CPU 800 MHz with PSRAM memory-mapped at
200 MHz:

```text
stm32n6: clock CPU=800000000 SYS=400000000 HCLK=200000000 PCLK1=200000000 PCLK2=200000000
XSPI1 PSRAM startup prescaler=3 effective=50000000Hz refresh=96
XSPI1 PSRAM optional prescaler=0 effective=200000000Hz refresh=396
```

## Important Gaps

- Basic RCC/USART1/XSPI2/XSPI1 protected KNSh boot was hardware validated at
  the 50 MHz PSRAM setting.
- The CPU 800 MHz / PSRAM 200 MHz profile still needs sustained heap and stack
  stress testing on hardware.
- Temporary heap/scheduler bring-up logs should be removed after the protected
  heap split is stable.
- NOR write/erase is implemented for explicit MTD use only.  Startup code
  still never erases, formats, or writes NOR automatically.
- LCD/touch/SD/audio/camera/USBPD are planned but not implemented in this
  checkpoint.

## Build Commands

```bash
cd /home/uan-wsl2/Feather

./tools/firmware/stm32n6570-dk/build-nsh.sh \
  --signing-tool /path/to/STM32_SigningTool_CLI \
  -j 8

./tools/firmware/stm32n6570-dk/build-knsh.sh \
  --signing-tool /path/to/STM32_SigningTool_CLI \
  -j 8
```

The build helper produces both burnable images:

```text
build/stm32n6570-dk-nxboot.bin
  [ST BootROM FSBL header][NuttX NXboot payload], burn at 0x70000000

build/stm32n6570-dk-nsh.bin
  [NXboot header][NuttX NSH app raw binary], burn at 0x70100000

build/stm32n6570-dk-knsh.bin
  [NXboot header][kernel blob][0xff padding][user blob], burn at 0x70100000
```

`--signing-tool` can be omitted when the local STM32CubeProgrammer CLI cache is
populated under `tools/vendor/stmicro/stm32cubeprogrammer`.

## Build Status

Static build verification:

- `stm32n6570-dk:nxboot` builds successfully as of 2026-05-14.
  - Latest observed RAM usage: `68384 B` in the `0x34180400` RAM region.
- `stm32n6570-dk:nsh` builds successfully as of 2026-05-14.
  - App links at `0x70100400`.
  - Internal SRAM data/BSS links at `0x34000000`.
  - `.ramfunc` section links at `0x34000000` and is loaded from NOR, so NOR
    write/erase routines can run while XSPI2 memory-mapped mode is disabled.
  - Latest observed external NOR usage: `91692 B`.
  - Latest observed internal SRAM usage: `12712 B`.
  - PSRAM linker region is present at `0x90000000`; runtime heap extension is
    controlled by `CONFIG_STM32N6_PSRAM_HEAP`.
- `stm32n6570-dk:knsh` builds successfully as of 2026-05-14.
  - Kernel links at `0x70100400` and uses a `512 KiB` slot window before
    user space.
  - User blob links at `0x70180400`.
  - Latest observed kernel XIP usage: `94204 B`.
  - Latest observed kernel SRAM usage: `12288 B`.
  - Latest observed user image size: `47208 B`.
- `tools/firmware/stm32n6570-dk/build-knsh.sh -j 8` builds both burnable
  images successfully as of 2026-05-14.
  - `build/stm32n6570-dk-nxboot.bin`: `62272 B`.
  - `build/stm32n6570-dk-knsh.bin`: `572520 B`.
- `git diff --check` passes.
- New STM32N6 code does not directly call Cube `HAL_`, `LL_`, or `BSP_`
  functions.

## Packaging

`build-nsh.sh` is the flat NSH entrypoint.  `build-knsh.sh` is the protected
KNSh entrypoint.  Both first build `stm32n6570-dk:nxboot` and wrap that
payload with the ST trusted FSBL header.  `build-nsh.sh` then wraps the flat
NSH app with the NXboot header; `build-knsh.sh` combines the kernel and user
blobs into one protected payload, then wraps that payload with the NXboot
header.

NXboot payload:

```bash
cd /home/uan-wsl2/Feather

./tools/firmware/stm32n6570-dk/pack-stm32-fsbl-nxboot.sh \
  /path/to/STM32_SigningTool_CLI \
  nuttx/nuttx.bin \
  build/stm32n6570-dk-nxboot-trusted.bin
```

Application image:

```bash
cd /home/uan-wsl2/Feather

./tools/firmware/stm32n6570-dk/pack-nxboot-header-app.sh \
  nuttx/nuttx.bin \
  build/stm32n6570-dk-nsh.bin
```

Burn addresses:

- Trusted NXboot image: `0x70000000`
- NXboot app image: `0x70100000`

Both helper scripts validate input files and output directories before
running, then print the expected burn address.  They do not change the image
format: the NXboot payload still receives only the ST BootROM trusted-image
wrapper, and the app still receives only the NXboot image header.

Additional script checks in this checkpoint:

- `pack-stm32-fsbl-nxboot.sh` rejects FSBL payloads larger than the
  `0x7fc00` RAM window.
- `pack-nxboot-header-app.sh` passes the STM32N657 platform identifier
  `0x4e363537`, then validates the generated NXboot header, MSP and reset
  vector.  The reset vector must point inside the primary XIP slot.

## Next Bring-up Steps

1. Reflash both NXboot and the KNSh app so the 800 MHz clock profile is set
   before the XIP app starts.
2. Run basic NSH commands and user-heap stress tests with PSRAM at 200 MHz.
3. Keep MR/linear-burst settings aligned with STM32CubeN6 while checking for
   stack/context corruption.
4. Remove temporary heap and scheduler debug prints once the protected KNSh
   boot is repeatable.
5. Hardware-validate explicit NOR erase/write through the MTD path.
6. Add SD/LCD/touch/audio/camera/USBPD in
   the existing SoC/board layering.
