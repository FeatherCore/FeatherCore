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
    MR8 `0x4b`, linear-burst read command `0x20`, linear-burst write command
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
  - user static `.data/.bss` uses the top internal app SRAM window.  The
    default protected window is `128 KiB` at
    `0x341e0000..0x341fffff`; the window is configurable with
    `CONFIG_STM32N6_PROTECTED_USRAM_BASE` and
    `CONFIG_STM32N6_PROTECTED_USRAM_SIZE`.
  - remaining internal SRAM below `0x341e0000` is kernel SRAM and runtime
    kernel heap
  - `up_allocate_heap()` returns an internal user SRAM bootstrap heap so
    `struct mm_heap_s`, allocator locks, and earliest allocations do not live
    in memory-mapped PSRAM during `nx_start()`.
  - XSPI1 PSRAM is added later by the board `arm_addregion()` override as
    the large secondary user heap.  LCD-enabled configurations keep the first
    `2 MiB` for the double-buffered framebuffer and use
    `0x90200000..0x91ffffff`; non-LCD protected builds add
    `0x90000000..0x91ffffff`.
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

0x341e0000  default protected user SRAM base, 128 KiB window
0x34200000  protected KNSh user SRAM end

0x90000000  XSPI1 PSRAM base
0x90000000  LCD framebuffer reserve, 2 MiB window for two RGB565 buffers
0x90000000  protected non-LCD secondary PSRAM user heap base, size 32 MiB
0x90200000  LCD-enabled app-side PSRAM heap base, default size 30 MiB
```

## Current Runtime Expectations

Normal NXboot and protected KNSh serial logs should include the external
memory bring-up checkpoints without temporary heap/scheduler breadcrumbs:

- `XSPI2 NOR JEDEC ID c2 81 3b`
- `XSPI2 NOR OPI/DTR config readback 02`
- `XSPI2 NOR mapped 0x70000000 ota0[0]=...`
- `XSPI1 PSRAM MR00000000 initial 08 write 30 readback 30`
- `XSPI1 PSRAM MR00000004 initial 40 write 20 readback 20`
- `XSPI1 PSRAM MR00000008 initial 05 write 4b readback 4b`
- `XSPI1 PSRAM self-test passed`
- `XSPI1 PSRAM mapped at 0x90000000 refresh=396`
- `XSPI1 PSRAM already memory-mapped refresh=396`
- `XSPI2 NOR already memory-mapped`
- `NuttShell (NSH)`

Normal logs should not include temporary `N6A*`, `N6N*`, `N6MM*`, `N6G*`,
`N6I*`, `N6P*`, `N6S*`, `N6T*`, or `N6U*` markers, and should not include
the verbose `stm32n6: mpuXX ...` region dump.  They also should not include
the former syscall bring-up traces such as `armctx:`, `semctx:`, `taskexit:`,
or `mallinfo_trace:`.  Heap placement messages are `finfo()` diagnostics and
are not expected unless debug info logging is enabled.

If no app image has been burned at `0x70100000`, NXboot should still end with
`Could not find bootable image`.  That is expected until a valid NXboot app
image is present.

When the flat NSH app boots with `CONFIG_STM32N6_PSRAM_HEAP=y`, early heap
setup calls `stm32n6570_xspi1_psram_initialize()` idempotently and then adds
the PSRAM heap region starting at `0x90200000`.  Protected KNSh first uses
`CONFIG_STM32N6_PROTECTED_UHEAP_SIZE` bytes of internal user SRAM as the
initial user heap, keeping allocator metadata and earliest allocations
internal during `nx_start()`.  Then `arm_addregion()` initializes/verifies
XSPI1 PSRAM idempotently, maps it for user access, and appends the configured
PSRAM window with `kumm_addregion()`.  For non-LCD KNSh this is the full
`0x90000000..0x91ffffff` range.  For LCD LVGL builds this is
`0x90200000..0x91ffffff`, leaving the first `2 MiB` for the double-buffered
framebuffer.  The remaining internal SRAM below
`CONFIG_STM32N6_PROTECTED_USRAM_BASE` is the protected kernel heap.

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
XSPI1 PSRAM MR00000008 initial 05 write 4b readback 4b
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
XSPI1 PSRAM mapped at 0x90000000 refresh=396
XSPI1 PSRAM already memory-mapped refresh=396
```

## Optional XSPI Bandwidth Test

The normal `stm32n6570-dk:lvgl` firmware must not run memory bandwidth tests
at boot.  The test is destructive to a reserved PSRAM scratch window, adds
seconds of boot log spam, and is only useful during board bring-up or
clock/timing regression checks.  Keep the benchmark as a temporary local patch
or throw-away branch, then remove it before committing normal firmware changes.

Build and burn the normal LVGL images:

```bash
cd /home/uan-wsl2/Feather
./tools/firmware/stm32n6570-dk/build-lvgl.sh -j 8
```

Program addresses:

```text
build/stm32n6570-dk-nxboot.bin  -> 0x70000000
build/stm32n6570-dk-lvgl.bin    -> 0x70100000
```

For a temporary benchmark, insert the probe after `stm32n6570_xspi1_psram_initialize()`
returns successfully in `stm32n6_extmem_initialize()`, while XSPI1 PSRAM and
XSPI2 NOR are both memory-mapped and before LVGL starts allocating from PSRAM.
Do not leave the hook in the default `lvgl` defconfig or board source.

Interpret only the `[BENCH][BEST-FULL] hpdma ...` lines as external bus
throughput.  CPU paths are useful diagnostics, but cacheable PSRAM/NOR access
means they also include CPU loop, cache, and line-fill behavior.  In
particular, `cpu flash-hot->sram` is a cache-effect probe and can exceed the
raw NOR bus peak.

Observed aligned STM32N6570-DK results at CPU 800 MHz, XSPI1 PSRAM 16-bit DDR
200 MHz, and XSPI2 NOR 8-bit DTR 200 MHz:

```text
hpdma sram->psram   369.30 MiB/s
hpdma psram->sram   400.86 MiB/s
hpdma psram->psram   99.46 MiB/s
hpdma flash->sram   380.70 MiB/s
```

The NOR result is effectively saturated: the theoretical 8-bit DTR 200 MHz
peak is `381.46 MiB/s`.  The PSRAM one-way DMA paths reach roughly half of
the theoretical `762.93 MiB/s` 16-bit DDR peak on the current controller/DMA
path.  `psram->psram` is much lower because reads and writes contend on the
same PSRAM interface.

The test method is:

1. Keep the benchmark scratch buffers outside the framebuffer and LVGL heap.
2. Run critical CPU copy/fill loops from SRAM when `CONFIG_ARCH_RAMFUNCS=y`.
3. Use the cycle counter/perf timer, not millisecond ticks.
4. Clean/invalidate cache outside the timed HPDMA window.
5. Sweep HPDMA transfer width, burst length, and source/destination ports.
6. Re-run the best HPDMA setting over the full test size and compare
   `[BEST-FULL]` against the raw bus peak.

Temporary hook sketch:

```c
/* Temporary bring-up code only.  Remove before committing normal firmware. */
static void stm32n6570_xspi_bandwidth_probe(void)
{
  uintptr_t psram_a = 0x90000000;
  uintptr_t psram_b = 0x90100000;
  uintptr_t xip = 0x70100000;

  bench_cpu_copy(psram_a, psram_b, 65520, 512);
  bench_cpu_copy((uintptr_t)g_sram_scratch, psram_a, 65520, 512);
  bench_hpdma_sweep("hpdma flash->sram", (uintptr_t)g_sram_scratch,
                    xip, 65520, 256);
}

int stm32n6_extmem_initialize(void)
{
  ...
  ret = stm32n6570_xspi1_psram_initialize();
  if (ret < 0)
    {
      return ret;
    }

  stm32n6570_xspi_bandwidth_probe();
  ...
}
```

Minimal CPU-path timing skeleton:

```c
static uint32_t bench_now(void)
{
  return (uint32_t)up_perf_gettime();
}

static uint64_t bench_rate_x100(uint64_t bytes, uint32_t cycles)
{
  uint64_t hz = up_perf_getfreq();
  return bytes * hz * 100ull / cycles / (1024ull * 1024ull);
}

static locate_code(".ramfunc") noinline_function void
bench_copy64(void *dst, const void *src, size_t nbytes)
{
  uint64_t *d = dst;
  const uint64_t *s = src;

  for (size_t i = 0; i < nbytes / sizeof(uint64_t); i++)
    {
      d[i] = s[i];
    }
}

static void bench_cpu_copy(uintptr_t dst, uintptr_t src,
                           size_t chunk, unsigned int loops)
{
  uint32_t start;
  uint32_t cycles;

  UP_DSB();
  UP_ISB();
  start = bench_now();
  for (unsigned int i = 0; i < loops; i++)
    {
      bench_copy64((void *)dst, (const void *)src, chunk);
    }

  UP_DSB();
  UP_ISB();
  cycles = bench_now() - start;
  syslog(LOG_INFO, "rate=%llu.%02llu MiB/s\n",
         bench_rate_x100((uint64_t)chunk * loops, cycles) / 100,
         bench_rate_x100((uint64_t)chunk * loops, cycles) % 100);
}
```

Minimal HPDMA-path timing pattern:

```c
up_clean_dcache(src, src + span);
up_clean_dcache(dst, dst + span);
up_invalidate_dcache(dst, dst + span);
UP_DSB();

start = bench_now();
for (unsigned int i = 0; i < loops; i++)
  {
    hpdma_program_width_burst_and_ports(best_config);
    hpdma_copy_block(dst + dst_off, src + src_off, chunk);
    wait_for_hpdma_idle_or_error();
  }

UP_DSB();
UP_ISB();
cycles = bench_now() - start;
up_invalidate_dcache(dst, dst + span);
```

The HPDMA setup itself is intentionally board-private test code until the
STM32N6 port has a proper reusable HPDMA driver.  Invalid sweep combinations
can report transfer errors; those are expected during probing and should not
be treated as the final bandwidth result.  Use the selected `[BEST-FULL]`
line.

## 2026-05-17 Protected KNSh PSRAM Heap/MPU Bug Fix

This stabilization is still required on STM32N6570-DK protected KNSh.  After
the later ARMv8-M protected-syscall fixes, a trial build moved the normal
non-LCD KNSh path back to the STM32H7S78-DK-style direct PSRAM initial user
heap.  Hardware then stalled immediately after the app-side PSRAM self-test,
before the app bring-up banner, so the internal bootstrap heap is again the
default protected KNSh layout.

Symptom:

- Directly using the full XSPI1 PSRAM window as the protected KNSh initial
  user heap, like the STM32H7S78-DK port, was not stable on STM32N6570-DK.
- Early failures included memory-management faults at `0x90000000` while the
  protected heap code touched PSRAM.  One observed crash had `CFSR=0x82`,
  `MMFAR=0x90000000`, `R0=0x90000000`, and happened before NSH started.
- After the first heap placement fix, the boot reached the board bring-up and
  task creation path, but temporary `N6*` breadcrumbs showed the port was
  still relying on debug prints to prove allocator progress.

Root cause:

- The H7RS and N6 build scripts are structurally similar, but the runtime heap
  path is not equivalent.  H7RS is able to use memory-mapped PSRAM directly as
  the initial protected user heap.  On N6, the ARMv8-M MPU/cache path, XIP
  execution from XSPI2, and XSPI1 memory-mapped PSRAM made that early heap
  placement fragile.
- Putting the initial heap in PSRAM also places `struct mm_heap_s`, allocator
  locks, early task-group allocations, and the first user stacks in the
  external-memory window before the protected runtime has fully settled.
- The MPU setup also created an overlapping PSRAM mapping: the privileged
  default external SRAM region already covered `0x90000000..0x91ffffff`, then
  the user heap path added another region for the same window.  On ARMv8-M,
  overlapping enabled regions are ordered by MPU region number, so this made
  the effective user access permissions dependent on allocation order.

Fix:

- Reserve an `8 KiB` protected user bootstrap heap in
  `boards/arm/stm32n6/stm32n6570-dk/scripts/user-space.ld`.
- Make `up_allocate_heap()` return that internal USRAM bootstrap region.
- Keep the protected kernel heap in the remaining internal SRAM below
  `CONFIG_STM32N6_PROTECTED_USRAM_BASE`.
- Add the full `32 MiB` PSRAM window with `kumm_addregion()` from the board
  `arm_addregion()` override after the initial user heap exists.
- Set `CONFIG_MM_REGIONS=2` for KNSh so the bootstrap heap and PSRAM extension
  are both visible to the user allocator.
- Change `stm32n6_mpu_uheap()` to find an existing matching MPU region and
  call `mpu_modify_region()` instead of blindly adding a duplicate PSRAM
  region.
- Use the idempotent `stm32n6570_xspi1_psram_initialize()` path when adding
  the protected PSRAM heap after the initial setup.  If NXboot already left
  PSRAM memory-mapped, the app-side path only verifies the mapping before
  appending it to the user heap.
- Remove temporary heap, scheduler, SVC, task, and MPU breadcrumbs after the
  protected KNSh boot reached NSH and basic commands.

Validated high-speed protected KNSh boot log:

```text
STM32N6570-DK bring-up skeleton
XSPI2 NOR JEDEC ID c2 81 3b
XSPI2 NOR OPI/DTR config readback 02
XSPI2 NOR mapped 0x70000000 ota0[0]=0x534f584e
XSPI1 PSRAM MR00000000 initial 08 write 30 readback 30
XSPI1 PSRAM MR00000004 initial 40 write 20 readback 20
XSPI1 PSRAM MR00000008 initial 05 write 4b readback 4b
XSPI1 PSRAM self-test passed
XSPI1 PSRAM mapped at 0x90000000 refresh=396
Found bootable image, boot from primary.
Boot vector msp=0x34003400 reset=0x70101b01 vtor=0x70100400
XSPI1 PSRAM already memory-mapped refresh=396
XSPI1 PSRAM self-test passed
STM32N6570-DK bring-up skeleton
XSPI2 NOR already memory-mapped
XSPI1 PSRAM already memory-mapped refresh=396
NuttShell (NSH)
nsh>
```

`free` after reaching NSH reported a small internal kernel heap and the
PSRAM-backed user heap:

```text
      total       used       free    maxused    maxfree  nused  nfree name
       8188       3052       5136       3432       5136     26      1 Kmem
   33562620       6340   33556280      21800   33542136     10      3 Umem
```

This STM32N6570-DK protected KNSh layout keeps early protected user heap
metadata in internal USRAM, then uses the full 32 MiB PSRAM as a large
secondary user heap.  It was first introduced while isolating MPU, cache, and
protected-syscall issues, and is again the normal non-LCD KNSh layout after
the direct-PSRAM initial heap trial stalled on hardware.

## 2026-05-21 N6 KNSH LVGL/RAMSpeed Investigation

Symptom:

- `stm32n6570-dk:knsh-lvgl` boots through NXboot, initializes XSPI2 NOR,
  XSPI1 PSRAM, LTDC, and GT911, and reaches NSH.
- `lvgldemo` runs, but the LVGL benchmark is much slower than both
  `stm32n6570-dk:nsh-lvgl` and the STM32H7S78-DK protected KNSh LVGL build.
- The slow N6 KNSH case shows normal flush time but very high render time.
  The observed all-scene average was about `74%` CPU, `8 FPS`, `154 ms`
  average time, `147 ms` render time, and `7 ms` flush time.
- N6 flat NSH on the same board was much closer to expected behavior:
  about `46%` CPU, `15 FPS`, `38 ms` average time, `31 ms` render time, and
  `7 ms` flush time.
- STM32H7S78-DK KNSh and NSH were both in the normal range, with all-scene
  render time around `17 ms` and `14 ms` respectively.

LVGL path analysis:

- The framebuffer driver is using direct double-buffer rendering:
  `apps/graphics/lvgl/lvgl/src/drivers/nuttx/lv_nuttx_fbdev.c` sets
  `LV_DISPLAY_RENDER_MODE_DIRECT`.
- In direct double-buffer mode, LVGL copies updated areas from the draw buffer
  into the second framebuffer before the flush completes.  That work is in
  the render path, not the flush path:
  - `apps/graphics/lvgl/lvgl/src/core/lv_refr.c`
  - `apps/graphics/lvgl/lvgl/src/draw/lv_draw_buf.c`
- The normal flush time is therefore not proof that CPU rendering is healthy.
  LTDC page flipping and the global-VBR IRQ synchronization keep the flush
  side cheap, while the PSRAM-to-PSRAM synchronization copy is charged to
  render time.

Flat NSH `ramspeed` baseline:

- `memset` to PSRAM at `0x90000000` reached about `640000 KB/s`.
- PSRAM-to-PSRAM `memcpy`, from `0x900bb800` to `0x90000000`, was much
  slower: about `51717 KB/s` for system `memcpy` and about `21245 KB/s` for
  the internal byte/word copy at the `512 KiB` test size.
- XIP NOR-to-PSRAM `memcpy`, from `0x70180400` to `0x90000000`, reached about
  `393846 KB/s`.
- Heap-allocated `-a` testing showed PSRAM-to-internal-SRAM copy around
  `128000 KB/s`, while internal-SRAM `memset` was much faster.
- This supports the LVGL hypothesis: the worst path is not LTDC flush, but
  CPU-side PSRAM-to-PSRAM copy contention.

KNSH `ramspeed` command anomaly:

- Enabling `CONFIG_BENCHMARK_RAMSPEED=y` in the N6 KNSH LVGL build exposed a
  separate problem: normal `ramspeed` commands appeared to hang instead of
  returning to the NSH prompt.
- Increasing `CONFIG_BENCHMARK_RAMSPEED_STACKSIZE` to `16384` did not fix it.
- Disabling ARMv8-M optimized libc memory routines did not fix it:
  - `CONFIG_LIBC_ARCH_MEMCPY` disabled
  - `CONFIG_LIBC_ARCH_MEMSET` disabled
  - `CONFIG_ARMV8M_MEMCPY` disabled
  - `CONFIG_ARMV8M_MEMSET` disabled
- A diagnostic build that changed the PSRAM MPU mapping to
  `SH_NO + WRITE_BACK` was not viable.  It stopped during the second-stage
  app external-memory init after:

```text
XSPI1 PSRAM already memory-mapped refresh=396
```

  before the PSRAM self-test completed.  That combination should not be used
  as the next fix direction.

STM32CubeN6 reference check:

- `STM32CubeN6/Projects/STM32N6570-DK/Examples/XSPI/XSPI_PSRAM_MemoryMapped`
  confirms the same general XSPI1 PSRAM memory-mapped direction, but does not
  provide an MPU policy for the simple memory-mapped example.
- Other STM32CubeN6 applications mark benchmark PSRAM buffers as uncached.
  Their MPU setup uses a non-cacheable attribute for PSRAM benchmark regions,
  not `SH_NO + WRITE_BACK`.
- A temporary N6 KNSH LVGL autoprobe build with PSRAM changed to
  non-cacheable was therefore built as an A/B test.  The autoprobe still
  completed, so uncached PSRAM remains useful as a performance/coherency A/B,
  but it did not by itself explain the NSH prompt not returning after
  `ramspeed`.

Diagnostic images and results:

- `stm32n6570-dk-knsh-lvgl-ramspeed-stack16k.bin`
  enabled `ramspeed` with a larger stack.  The command still appeared to hang.
- `stm32n6570-dk-knsh-lvgl-ramspeed-noarchmem.bin`
  also disabled the ARMv8-M optimized memcpy/memset path.  The command still
  appeared to hang.
- `stm32n6570-dk-knsh-lvgl-ramspeed-noarchmem-shno.bin`
  changed PSRAM to `SH_NO + WRITE_BACK`; boot did not pass the second app-side
  PSRAM self-test path.
- `stm32n6570-dk-knsh-lvgl-ramspeed-entryonly-noarchmem.bin`
  proved that the benchmark application entrypoint is reached.
- `stm32n6570-dk-knsh-lvgl-ramspeed-autoprobe-noarchmem.bin`
  ignored command-line arguments and tested malloc/free, `clock_gettime`, XIP
  reads, framebuffer touches, manual PSRAM copy, libc `memcpy`, and libc
  `memset`.  It reached `autoprobe: complete`.
- `stm32n6570-dk-knsh-lvgl-ramspeed-autoprobe-noarchmem-psram-nc.bin`
  repeated the autoprobe with PSRAM non-cacheable.  It also reached the end of
  the probe.
- `stm32n6570-dk-knsh-lvgl-ramspeed-argprobe-noarchmem.bin`
  tested the user-space argument vector path.  It successfully read and wrote
  `argv[0]`, read and wrote `argv[1]`, completed `strcmp`, and printed
  `argprobe: complete`.

Current conclusion:

- The KNSH `ramspeed` anomaly is no longer pointing at `argv`, basic user
  app entry, XIP reads, PSRAM reads/writes, libc `memcpy`, libc `memset`, or
  the benchmark stack size.
- Later low-output progress builds showed the memory loops themselves
  completing, but console output could block part-way through a progress
  string.  The latest observed stop was while printing the next RAMSpeed
  progress marker after `memcpy` completed.  That moves the RAMSpeed anomaly
  toward the protected user stdout/write path, not toward the PSRAM memory
  loops themselves.
- The next useful split is to bypass libc `exit()` with a direct `_exit(0)`
  probe:
  - if `_exit(0)` returns control cleanly, investigate libc `exit()` cleanup,
    especially stdio flushing and atexit/TLS cleanup;
  - if `_exit(0)` still does not return to the prompt, instrument kernel-side
    task teardown around `_exit()`, `nxtask_exithook()`, `nxtask_exit()`,
    `nxsched_release_tcb()`, `group_leave()`, and the NSH wait/reap path.
- A 2026-05-22 partial-framebuffer validation build used an internal 32-row
  draw buffer.  Runtime confirmed the intended path:

```text
Use partial framebuffer mode, memory: 0x341e0e58, rows: 32, size: 51200
```

  The result was worse than direct mode: all-scene average `73%` CPU, `8 FPS`,
  `233 ms` average time, `232 ms` render time, and `1 ms` flush time.  This
  rules out the direct-mode PSRAM framebuffer synchronization copy as the
  primary LVGL performance root cause.  Partial mode should not be used as the
  N6 KNSh LVGL performance fix.
- The retained default layout restores direct framebuffer mode and keeps KNSh
  LVGL aligned with the normal protected KNSh memory split: internal user SRAM
  stays at `0x341e0000..0x341fffff` (`128 KiB`) with a `0x2000` bootstrap user
  heap, while PSRAM is added later as the secondary user heap.  The
  LVGL-specific difference is that the LCD configuration reserves the first
  `2 MiB` of PSRAM for the double-buffered RGB565 framebuffer, so the app-side
  PSRAM heap starts at `0x90200000`.
- Hardware validation of the earlier enlarged-user-SRAM experiment confirmed
  that moving hot LVGL allocations out of the slow PSRAM path recovers
  performance, but that was a control experiment, not the desired final
  solution.  The runtime log returned to direct double-buffer mode:

```text
fbdev_init_mem2: Use consecutive mem2 = 0x900bb800, yoffset = 480
```

  `free` reported `Kmem` total `1555628` and `Umem` total `31916028`, matching
  the enlarged validation split plus the LCD-safe PSRAM user heap after the
  first `2 MiB` framebuffer window.  The all-scene average improved
  to `47%` CPU, `15 FPS`, `38 ms` average time, `32 ms` render time, and
  `6 ms` flush time.  This is effectively the same as the earlier N6 flat NSH
  baseline (`46%`, `15 FPS`, `38 ms`, `31 ms`, `7 ms`).  This proves the
  render path can be fast when the CPU does not pay the slow PSRAM access
  pattern, but it does not satisfy the PSRAM-as-main-RAM goal.
- After validation, the temporary `ramspeed` progress/skip-output/direct-exit
  diagnostics and the LVGL fbdev partial-render Kconfig/test path were removed
  from the normal tree.  The normal N6 KNSh LVGL build keeps the direct
  framebuffer path, the small `0x2000` internal bootstrap user heap, and PSRAM
  for the `lvgldemo` task stack and most LVGL runtime allocations.
- The later ARM Cortex-M55 cache/MPU cross-check changed the performance
  interpretation.  With the old `SH_OUTER + WRITE_BACK` mapping, D-side
  Shareable PSRAM transactions are effectively treated as Non-cacheable by
  the M55 L1 data cache.  The enlarged-user-SRAM test avoided this bad PSRAM
  path; it did not prove that N6 must keep LVGL in SRAM.
- The normal STM32N6 PSRAM policy is therefore
  `Non-shareable + Write-back + read-allocate/no-write-allocate` for
  CPU-owned heap and stack memory.  This is the policy that must be validated
  for the uClinux-style "PSRAM is main RAM" direction.  If the LVGL benchmark
  is still slow under this policy, the next target is the XSPI/MPU/cache
  implementation itself, not increasing the internal SRAM allocation.

Open LVGL performance directions:

- Burn and benchmark the normal `stm32n6570-dk:knsh-lvgl` image with
  `CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE=y`,
  `CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK=y`, and
  `CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE=y`.
- Re-run PSRAM stack/syscall smoke tests and PSRAM-to-PSRAM `ramspeed` on the
  same image.  Good LVGL performance requires ordinary protected user stacks,
  heap allocations, and memcpy/fill paths to remain stable while backed by
  PSRAM.
- If performance is still close to the slow `8 FPS` result, inspect the
  runtime MPU/MAIR programming and XSPI1 memory-mapped timing before changing
  heap placement.  The framebuffer window shared with LTDC may still need a
  separate non-cacheable MPU subregion or explicit cache maintenance, but the
  CPU-owned PSRAM heap/stack should stay cacheable non-shareable.
- Compare the same `ramspeed` command-exit behavior on STM32H7S78-DK KNSh to
  separate a generic protected-build app-exit bug from an N6-specific exit
  path issue.

## Important Gaps

- Basic RCC/USART1/XSPI2/XSPI1 protected KNSh boot was hardware validated at
  both the conservative 50 MHz PSRAM setting and the CPU 800 MHz / PSRAM
  200 MHz setting.
- The CPU 800 MHz / PSRAM 200 MHz profile still needs sustained heap,
  stack, and long-running memory stress testing on hardware.
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
- `stm32n6570-dk:knsh` builds successfully as of 2026-05-17.
  - Kernel links at `0x70100400` and uses a `512 KiB` slot window before
    user space.
  - User blob links at `0x70180400`.
  - Latest observed kernel XIP usage: `92552 B`.
  - Latest observed kernel SRAM usage: `12272 B`.
  - Latest observed user image size: `64344 B`.
- `tools/firmware/stm32n6570-dk/build-knsh.sh -j 8` builds both burnable
  images successfully as of 2026-05-17 after the protected heap/MPU fix and
  temporary debug-print cleanup.
  - `build/stm32n6570-dk-nxboot.bin`: `61408 B`.
  - `build/stm32n6570-dk-knsh.bin`: `581408 B`.
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
4. Hardware-validate explicit NOR erase/write through the MTD path.
5. Add SD/LCD/touch/audio/camera/USBPD in
   the existing SoC/board layering.

## 2026-05-22 PSRAM Stack Placement Analysis

The issue under investigation is PSRAM-backed thread stacks, not a permanent
policy of keeping all stacks in internal SRAM.  The target is to make the
`0x90000000` XSPI1 PSRAM window behave like reliable Normal system RAM for
ordinary user stacks.  Internal SRAM is only the control sample until the PSRAM
MPU/cache policy is proven.

Reference checks:

- STM32CubeN6 does not show a clear RTOS-thread-stack-in-PSRAM pattern for
  STM32N6570-DK applications.  The ThreadX VENC RTSP example allocates its
  ThreadX stack from an internal RAM byte pool, while the linker exposes PSRAM
  as an explicit `.psram_bss` style area for large buffers.
- Renesas FSP RA8D1/RA8P1 M85 examples follow the same conservative pattern.
  ThreadX/FreeRTOS thread stacks are generated as normal `.stack.<thread>` or
  RTOS stack-size settings, while external SDRAM sections are used explicitly
  for framebuffer, VIN image buffers, FileX media storage, or AI buffers.
- STM32H7S78-DK was not, by itself, proof that all N6 user stacks should live
  in PSRAM: H7RS uses the ARMv7-M MPU/cache path, while N6 uses ARMv8-M MPU
  MAIR/RBAR/RLAR attributes, XIP user code from XSPI2, and the M55 exception
  stack/unstack path.  After the protected syscall fixes below, the normal
  non-LCD N6 KNSh build is intentionally aligned back to the H7S78 direct
  PSRAM initial-user-heap model.

NuttX stack allocation facts:

- ARM common `up_create_stack()` allocates user task and pthread stacks with
  `kumm_malloc()` or `kumm_memalign()` when `CONFIG_MM_KERNEL_HEAP=y`.
- There is no generic NuttX section selector for ordinary user task/pthread
  stacks.  In normal non-LCD KNSh those allocations come from the PSRAM user
  heap.  LCD/LVGL validation builds that enable an internal bootstrap user
  heap may still allocate from either region depending on allocator state and
  request size.
- HP/LP workqueue stacks are a special case; they have
  `CONFIG_SCHED_HPWORKSTACKSECTION` and `CONFIG_SCHED_LPWORKSTACKSECTION`, but
  that mechanism does not cover arbitrary pthread/task stacks.

Current working hypothesis:

- PSRAM bandwidth itself is not the only issue.  Large linear CPU fills/copies
  and DMA paths can work well enough when access patterns are simple.
- Stack placement is a worst-case access pattern: every SVC, PendSV, IRQ, and
  context switch may read or write PSP-backed frames, software-saved registers,
  TLS-at-stack metadata, and stack-color/overflow probes.  If that stack is in
  PSRAM, the ARMv8-M exception path touches the XSPI mapping at scheduler
  frequency.
- The known N6 protected-build failure mode was worse than raw performance:
  putting the initial user heap in PSRAM also put allocator metadata, early
  user stacks, and early task-group allocations in PSRAM before the system had
  proven that protected user access and the external-memory MPU attributes were
  stable.
- The N6 PSRAM MPU mapping is a prime suspect.  The bootable baseline is
  `SH_OUTER + WRITE_BACK`, which also matches the generic ARMv8-M external SRAM
  helper policy.  A `SH_NO + WRITE_BACK` app image still stalled during the
  second-stage PSRAM self-test immediately after
  `XSPI1 PSRAM already memory-mapped refresh=396`, so N6 cannot simply copy the
  RA8P1 M85 non-shareable SDRAM policy.
- The app runs after NXboot, so MPU attribute changes must not inherit dirty
  cache state from the first image.  The N6 app now disables I-cache and
  D-cache before resetting and rebuilding the MPU map, then enables them again
  after the selected PSRAM policy is installed.

Validation firmware support:

- `tools/firmware/stm32n6570-dk/build-psram-verify.sh` now enables a
  `psramstack` diagnostic app in addition to `ramtest`, `memstress`,
  `ostest`, stack coloration, stack monitor, and ARMv8-M hardware stack-limit
  checking.
- `psramstack` explicitly allocates one pthread stack from
  `0x34000000..0x341fffff` and one pthread stack from
  `0x90000000..0x91ffffff`, then uses `pthread_attr_setstack()` so the test
  stack placement is deterministic instead of depending on ordinary allocator
  spillover.
- The command reports each thread's stack pointer locality and prints average
  timings for pure stack-touch loops, `sched_yield()`-only loops, stack-touch
  plus `sched_yield()` loops, and semaphore ping-pong loops.  A useful first
  run is:

  ```text
  psramstack -l 100000 -s 8192
  ```
- Run `ramspeed -a -s 65536 -n 1000` as a separate control.  If raw heap
  memset/memcpy speed is acceptable but `psramstack` is much slower, the problem
  is concentrated in PSRAM-backed stack access during exception entry/exit and
  scheduler activity.
- The validation builder can now generate A/B firmware for the PSRAM MPU
  policy.  Its default now matches the normal STM32N6 PSRAM policy
  (`SH_NONE + WRITE_BACK + no-write-allocate`):

  ```text
  ./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
    --app-only --psram-mpu-policy no-wb-nwa -j 8
  ```

  Supported policy names are `outer-wb`, `no-wb`, `no-wb-nwa`, `no-wt`,
  `no-wt-nwa`, `outer-wt`, `no-nc`, and `outer-nc`.  `outer-wb` is retained
  as the historical Shareable write-back baseline.  `no-wb`/`no-wt` without
  no-write-allocate are negative data points from the earlier M55 cache-policy
  investigation; use `no-wb-nwa` for the current regression image.

Measured `outer-wb` baseline:

- Boot succeeded through second-stage PSRAM self-test and NSH.
- `ramspeed -a -s 65536 -n 1000` allocated both buffers in PSRAM
  (`0x90000010` and `0x90010018`).  The 64 KiB system `memcpy()` case reported
  about 71 MiB/s, while the internal byte-copy loop reported about 27 MiB/s.
  Cached `memset()` results reached about 711 MiB/s and should not be treated
  as raw PSRAM write bandwidth without an explicit cache flush.
- `psramstack -l 100000 -s 8192` confirmed deterministic stack placement:
  internal local stack addresses were in `0x341e....`, PSRAM local stack
  addresses were in `0x9000....`.
- Measured averages:
  - internal pure stack-touch test: 15.4 us/loop
  - PSRAM pure stack-touch test: 21.2-22.1 us/loop, about 1.4x internal
  - internal yield-only test: 1.1 us/loop
  - PSRAM yield-only test: 8.5 us/loop, about 7.7x internal
  - internal yield test: 16.5 us/loop
  - PSRAM yield test: 28.9-30.0 us/loop, about 1.8x internal
  - internal semaphore ping-pong: 9.9-10.2 us/round
  - PSRAM semaphore ping-pong: 31.8-33.6 us/round, about 3.1-3.4x internal
- Repeating the same test with `psramstack -l 200000 -s 8192` produced
  effectively identical averages: internal touch 15.45 us, PSRAM touch
  22.05 us, internal yield 16.5 us, PSRAM yield 28.95 us, internal ping-pong
  9.9 us, and PSRAM ping-pong 33.6 us.  The totals scale linearly with loop
  count, so the gap is a stable access-path characteristic rather than timer
  granularity or cache warm-up noise.
- The yield-only result is the key data point: no explicit stack-touch loop is
  running, but putting PSP in PSRAM still adds about 7.4 us per yield.  This
  points at ARMv8-M exception entry/exit and NuttX context save/restore traffic
  on the interrupted thread's PSP, not only ordinary C stack locals.
- Current ARMv8-M FPU setup also amplifies this path.  `arm_fpuconfig()` sets
  `CONTROL.FPCA`, disables lazy FP save, and therefore makes exception contexts
  use the extended FP stack frame whenever `CONFIG_ARCH_FPU=y`.  The exception
  entry code then also reserves/saves software FP registers.  This is correct
  for conservative FPU preservation, but it is expensive when the thread stack
  lives in XSPI PSRAM.
- This means PSRAM stacks are functional under `SH_OUTER + WRITE_BACK`, but
  exception-heavy paths amplify PSRAM random-access latency.

Measured no-FPU A/B image:

- Firmware: `build/stm32n6570-dk-psram-verify-nofpu-full.bin`, built with the
  same `outer-wb` PSRAM MPU policy and `CONFIG_ARCH_FPU` disabled.
- `psramstack -l 100000 -s 8192` measured:
  - internal pure stack-touch test: 15.4 us/loop
  - PSRAM pure stack-touch test: 21.5 us/loop, about 1.4x internal
  - internal yield-only test: 1.0 us/loop
  - PSRAM yield-only test: 6.9 us/loop, about 6.9x internal
  - internal yield test: 16.3 us/loop
  - PSRAM yield test: 27.6 us/loop, about 1.7x internal
  - internal semaphore ping-pong: 5.1 us/round
  - PSRAM semaphore ping-pong: 23.1 us/round, about 4.5x internal
- Compared with the FPU-enabled baseline, disabling eager FP context reduced
  `psram-yieldonly` by about 1.6 us and reduced PSRAM semaphore ping-pong by
  about 8.7 us.  So the FPU extended frame is a real amplifier.
- The no-FPU image still leaves `psram-yieldonly` at about 6.9x the internal
  stack case, with about 5.9 us extra absolute cost per yield.  Therefore the
  remaining root issue is still PSP-backed exception entry/return plus NuttX
  context save/restore traffic hitting XSPI PSRAM with small, latency-sensitive
  stack accesses.  This is not explained by ordinary bulk PSRAM bandwidth.
- The next diagnostic A/B is lazy-FPU: keep `CONFIG_ARCH_FPU=y`, keep CP10/CP11
  enabled, but keep integer-only threads on the basic exception frame until they
  execute FP instructions.

Lazy-FPU bring-up note:

- The first lazy-FPU diagnostic image booted through the second-stage PSRAM
  self-test, then faulted in `nsh_main` with `CFSR=00000001`, `PC=00000000`,
  `CONTROL=0000000d`, and `EXC_RETURN=ffffffed`.  This is not a PSRAM data
  corruption signature.  It shows that changing only `EXC_RETURN_THREAD` to a
  basic frame is insufficient: ARMv8-M can switch between basic and extended FP
  hardware frames dynamically, while the existing NuttX ARMv8-M path had several
  assumptions tied to a single compile-time FP frame shape.
- The current diagnostic patch therefore also clears `CONTROL.FPCA/SFPA` at FPU
  init, synchronizes CONTROL FP state from `EXC_RETURN_STD_CONTEXT`, and
  computes the interrupted SP from the actual basic or extended hardware frame
  size in `arm_exception.S`.  `EXC_RETURN_STD_CONTEXT=1` means the hardware
  saved only the 8-word integer frame; `EXC_RETURN_STD_CONTEXT=0` means the
  hardware frame also includes volatile FP state.  This is still an A/B
  diagnostic path, not yet a final upstream-quality lazy-FPU implementation.
- A second lazy-FPU image with those frame-size fixes still faulted before the
  NSH prompt with the same signature: `CFSR=00000001`, `PC=00000000`,
  `CONTROL=0000000d`, and `EXC_RETURN=ffffffed`.  The remaining suspect is the
  protected syscall path: normal syscalls save the caller's `EXC_RETURN`, switch
  to `arm_dispatch_syscall`, then `SYS_syscall_return` restores the saved
  `EXC_RETURN` into the current SVC frame.  In lazy-FPU mode that is only valid
  if both frames have the same basic-vs-extended FP shape.
- The latest diagnostic patch keeps `arm_dispatch_syscall` on a frame shape
  compatible with the saved user frame under `CONFIG_ARMV8M_LAZYFPU`.  If this
  image reaches NSH, rerun `psramstack -l 100000 -s 8192` and compare the
  `yieldonly` and `pingpong` numbers against the eager-FPU and no-FPU baselines.
- The corrected lazy-FPU image reaches NSH.  With `outer-wb` PSRAM MPU policy,
  `psramstack -l 100000 -s 8192` measured:
  - internal pure stack-touch test: 15.4 us/loop
  - PSRAM pure stack-touch test: 20.6 us/loop, about 1.3x internal
  - internal yield-only test: 1.0 us/loop
  - PSRAM yield-only test: 7.2 us/loop, about 7.2x internal
  - internal yield test: 16.4 us/loop
  - PSRAM yield test: 28.0 us/loop, about 1.7x internal
  - internal semaphore ping-pong: 5.8 us/round
  - PSRAM semaphore ping-pong: 25.3 us/round, about 4.4x internal
- Compared with eager-FPU, lazy-FPU reduces PSRAM yield-only by about 1.3 us
  and PSRAM semaphore ping-pong by roughly 6.5-8.3 us.  Compared with the
  no-FPU A/B, lazy-FPU is close: PSRAM yield-only is about 0.3 us slower and
  PSRAM ping-pong is about 2.2 us slower.  This confirms that eager FP context
  was a removable amplifier, while the residual gap is still PSP-backed
  exception/syscall/context-save traffic hitting XSPI PSRAM.
- The lazy-FPU MPU attribute A/B did not show a meaningful performance
  difference between `SH_OUTER + WRITE_BACK`, `SH_OUTER + NONCACHEABLE`, and
  `SH_NO + NONCACHEABLE`.  Both non-cacheable images reached NSH and measured
  about:
  - internal stack-touch: 15.4-15.5 us/loop
  - PSRAM stack-touch: 20.9 us/loop
  - internal yield-only: 1.0 us/loop
  - PSRAM yield-only: 7.2 us/loop
  - internal yield: 16.4 us/loop
  - PSRAM yield: 28.1-28.2 us/loop
  - internal semaphore ping-pong: 5.7 us/round
  - PSRAM semaphore ping-pong: 25.2-25.3 us/round
  At first glance this appeared to rule out MPU cacheability/shareability as a
  useful lever.  The Cortex-M55 TRM explains why the result is so flat:
  shareable Normal data regions are treated as inner Non-cacheable by the M55
  L1 D-cache, regardless of the programmed inner cacheability attribute.  In
  other words, the reliable `SH_OUTER + WRITE_BACK` baseline is not giving
  normal L1 D-cache behavior for stack data.  It is effectively a shareable,
  non-cacheable D-side PSRAM path for the small stack accesses that matter here.
- The lighter `syscallonly` protected SVC round-trip test, using
  `sched_getcpu()`, measured `internal-syscallonly=1.0 us/loop` and
  `psram-syscallonly=6.9 us/loop`.  The `timeonly` test measured
  `internal-timeonly=1.1 us/loop` and `psram-timeonly=9.9 us/loop`.  These
  tests do not intentionally yield the CPU, so the PSRAM stack penalty is
  already visible on syscall/exception entry and return alone.  This confirms
  that the residual cost is not bulk PSRAM bandwidth, malloc behavior, or
  application work; it is the small, latency-sensitive hardware and software
  stack traffic while PSP points into the XSPI1 PSRAM window.

ARMv8-M/M55 MPU/cache interpretation:

- The M55 TRM states that D-side shareable Normal memory is treated as inner
  Non-cacheable, regardless of the programmed inner cacheability.  The same TRM
  says only non-shareable Normal memory uses the programmed inner cacheability
  for L1 D-cache allocation.  The ARMv8-M ARM also warns that changing memory
  attributes or using mismatched cacheability/shareability requires proper cache
  maintenance and barriers to preserve coherency.
- Therefore a real L1-cached PSRAM stack candidate must use `SH_NO` plus a
  cacheable MAIR attribute.  Our current `SH_OUTER + WRITE_BACK` baseline is a
  reliable system-RAM mapping, but it should not be expected to make stack
  push/pop, exception frame, or SVC traffic behave like internal SRAM.
- `SH_NO + WRITE_BACK` is not currently reliable on STM32N6570-DK PSRAM.  With
  `CONFIG_STM32N6570_DK_PSRAM_SELFTEST_DEBUG=y`, it reached the first pattern
  write at `0x90000000`, completed `up_flush_dcache()` and
  `up_invalidate_dcache()` for that cache line, printed `pattern
  offset=00000000 flushed`, and then stopped before `pattern
  offset=00000000 verified`.  This places the failure on the first readback
  linefill after a non-shareable write-back PSRAM cache-line writeback.
- `SH_NO + WRITE_THROUGH` is also not currently reliable.  With debug enabled,
  the normal image printed `pattern offset=00000000 written` and then stopped
  before the explicit `dcache flush start` log.  A follow-up `NOFLUSH` image
  removed the explicit PSRAM self-test flush/invalidate calls and still stopped
  during the first pattern flow around `pattern offset=00000000`.  This rules
  out the self-test cache-maintenance calls as the primary trigger and points
  at the non-shareable cacheable store/read path to XSPI1 PSRAM itself.
- The practical conclusion is that the current N6 port already has PSRAM working
  as reliable Normal system RAM for heap, framebuffer, `ramtest`, `memstress`,
  and bulk `ramspeed`, but not as internal-SRAM-like cached stack memory.  Making
  PSRAM stacks close to internal SRAM is not a NuttX ARMv8-M thread-model change
  alone; it requires a stable SoC-supported non-shareable cacheable PSRAM
  mapping, or a hybrid policy that keeps exception-heavy stacks in internal SRAM
  and uses PSRAM for colder stacks/data.
- `SH_OUTER + WRITE_THROUGH` was validated with the verbose PSRAM self-test.
  It completed pattern checks across `0x90000000`, `0x90000040`,
  `0x90001000`, `0x90001040`, `0x90100000`, `0x90ffffc0`,
  `0x91ffffc0`, and `0x91ffffe0`, then reached NSH.  The matching
  `psramstack -l 100000 -s 8192` result stayed in the same band as the other
  shareable/non-cacheable baselines:
  - internal stack-touch: 15.4 us/loop
  - PSRAM stack-touch: 20.7 us/loop
  - internal syscall-only: 1.0 us/loop
  - PSRAM syscall-only: 7.0 us/loop
  - internal time-only: 1.1 us/loop
  - PSRAM time-only: 9.9 us/loop
  - internal yield-only: 1.0 us/loop
  - PSRAM yield-only: 7.3 us/loop
  - internal yield: 16.4 us/loop
  - PSRAM yield: 28.0 us/loop
  - internal semaphore ping-pong: 5.7 us/round
  - PSRAM semaphore ping-pong: 25.5 us/round
  This closes the shareable `WRITE_BACK` versus shareable `WRITE_THROUGH`
  branch: changing the MAIR cache policy while keeping PSRAM shareable does not
  make PSRAM-backed stacks behave like internal SRAM on Cortex-M55.

No-write-allocate validation result:

- `SH_NO + WRITE_THROUGH + no-write-allocate` passed the verbose PSRAM
  self-test and reached NSH.  `psramstack -l 100000 -s 8192` measured:
  - internal stack-touch: 15.4 us/loop
  - PSRAM stack-touch: 15.8 us/loop
  - internal syscall-only: 1.0 us/loop
  - PSRAM syscall-only: 1.3 us/loop
  - internal time-only: 1.1 us/loop
  - PSRAM time-only: 1.9 us/loop
  - internal yield-only: 1.0 us/loop
  - PSRAM yield-only: 1.5 us/loop
  - internal yield: 16.4 us/loop
  - PSRAM yield: 17.3 us/loop
  - internal semaphore ping-pong: 5.7 us/round
  - PSRAM semaphore ping-pong: 7.7 us/round
- `SH_NO + WRITE_BACK + no-write-allocate` also passed the verbose PSRAM
  self-test and produced the best stack result:
  - internal stack-touch: 15.4 us/loop
  - PSRAM stack-touch: 15.4 us/loop
  - internal syscall-only: 1.0 us/loop
  - PSRAM syscall-only: 0.9 us/loop
  - internal time-only: 1.1 us/loop
  - PSRAM time-only: 1.2 us/loop
  - internal yield-only: 1.0 us/loop
  - PSRAM yield-only: 1.0 us/loop
  - internal yield: 16.4 us/loop
  - PSRAM yield: 16.4 us/loop
  - internal semaphore ping-pong: 5.7 us/round
  - PSRAM semaphore ping-pong: 5.8 us/round
- This reverses the earlier negative conclusion for non-shareable cacheable
  PSRAM.  The failure is not all cacheable PSRAM access; it is specifically the
  write-allocate path.  On M55, a cacheable write-allocate store miss can start
  a linefill before the store data is merged.  Disabling write allocation avoids
  that path and makes PSP-backed exception/SVC/context-switch stack traffic
  behave close to internal SRAM.
- The current best candidate for CPU-owned PSRAM stack and heap is therefore
  `SH_NO + WRITE_BACK + read-allocate + no-write-allocate`.  This still needs
  a separate coherency policy for non-CPU bus masters.  PSRAM regions used by
  LTDC, DMA, or other peripherals must either be mapped non-cacheable in a
  more specific MPU region or have explicit cache maintenance at ownership
  handoff.

Build the current best validation image with:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu -j 8
```

Ordinary task-stack validation:

- The default validation image still leaves enough bootstrap user heap in
  internal SRAM that early ordinary stacks may stay internal.  Rebuild with a
  smaller bootstrap user heap to force normal task and pthread stacks into
  PSRAM:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu \
  --bootstrap-uheap-size 0x4000 -j 8
```

- With `--bootstrap-uheap-size 0x4000`, `ps` and `/proc/<pid>/stack` confirmed
  `nsh_main`, `memstress`, its pthreads, and `ostest` were allocated in the
  `0x90000000` PSRAM window.
- That image still hit an `ostest` fault at the first `write()` from
  `ostest_main`: the user syscall wrapper returned with `LR=0x7018a9bb`, but
  then attempted to execute at `PC=0`.  The fault frame shows `R0/R2/R3=0x17`
  for the 23-byte write and the PSRAM user stack at `0x90052028`.  This points
  to a remaining syscall/exception-return ordering issue with fresh protected
  task stacks in `WRITE_BACK + no-write-allocate` PSRAM, not to the basic PSRAM
  bandwidth path measured by `psramstack`.
- Two follow-up images isolate the remaining issue:
  - `no-wt-nwa-uheap0x4000`: conservative write-through/no-write-allocate
    ordinary PSRAM stacks.
  - `no-wb-nwa-uheap0x4000-svcbar`: write-back/no-write-allocate plus DSB/ISB
    before returning from the ARMv8-M SVC handler.
- Both follow-up images still faulted in the same place: `ostest` printed the
  first `s`, entered the protected `write()` syscall wrapper, and returned with
  `PC=0`.  The fault frame consistently shows the correct saved return frame
  0x40 bytes above the PSP used by exception return.  `0x40` is exactly the
  software FP save area for `s16-s31` on this ARMv8-M path, so this is no
  longer a PSRAM cache-policy or missing-barrier symptom.  It is a remaining
  FPU/lazy-FPU exception-frame accounting issue in the protected syscall return
  path when ordinary user task stacks live in PSRAM.
- The direct diagnostic image preserves the active `SYS_syscall_return` SVC
  frame shape instead of copying the original user SVC frame bit blindly:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu \
  --bootstrap-uheap-size 0x4000 --syscall-current-frame -j 8
```

- If this image passes `ostest`, the fix direction is to make the protected
  syscall return path preserve or recompute the active ARMv8-M FP frame shape.
  If it still fails but the no-FPU or eager-FPU image passes, the remaining
  bug is still in FPU context accounting rather than in the PSRAM MPU policy.
- On hardware, the current-frame image passed the first `write()` failure point
  and reached `ostest_main: Started user_main at PID=10`, then stopped before
  `user_main` printed its first line.  `user_main` starts with a 500 ms
  `usleep()` and then `mallinfo()`, so the next diagnostic is to inspect the
  child task state rather than keep `ostest` in the foreground.  Build the same
  image with `--ostest-nowait` to disable `CONFIG_TESTING_OSTEST_WAITRESULT`
  and keep the NSH prompt available while the child task runs:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu \
  --bootstrap-uheap-size 0x4000 --syscall-current-frame \
  --ostest-nowait -j 8
```

- The `--ostest-nowait` image confirms that the first protected `write()`
  return no longer faults: `ostest_main` prints `Started user_main` and returns
  `Exiting with status 0` to NSH.  On hardware, the console then became
  non-interactive before `user_main` printed its first line.  Since `user_main`
  begins with `usleep(HALF_SECOND_USEC)`, the next image stretches that first
  sleep to 30 seconds so the child task can be inspected while it is blocked:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu \
  --bootstrap-uheap-size 0x4000 --syscall-current-frame \
  --ostest-nowait --ostest-delay-usec 30000000 -j 8
```

- The 30-second-delay full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-ostnowait-ostdelay30000000-full.bin`.
  SHA-256:
  `80c1a329fdc41ae6ef8c82c03f778f95a6219a692c98eb9b055face3cbe2ee30`.
  After running `ostest`, do not start `memstress` for this pass.  While
  `user_main` is sleeping, run `ps` and `cat /proc/<user_main-pid>/stack`
  using the PID printed by `Started user_main at PID=...`.  If NSH remains
  responsive until the 30-second sleep expires and then freezes, the remaining
  bug is in the child task's first sleep return / following `mallinfo()` path.
  If NSH freezes immediately despite the 30-second delay, the remaining issue
  is earlier: task start, parent exit, or protected syscall return from the
  child's initial blocking call.
- A narrower startup trace image adds short `write()` checkpoints around
  `user_main` entry, the first `usleep()`, and the initial `mallinfo()` call:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu \
  --bootstrap-uheap-size 0x4000 --syscall-current-frame \
  --ostest-nowait --ostest-delay-usec 5000000 \
  --ostest-startup-trace -j 8
```

- The 5-second startup-trace full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `91499c65e87ae5a829084556d9438ad9a60ced92335b639a84e419aa216a9757`.
  The expected trace order is `entry`, `before initial usleep`, then after
  about five seconds `after initial usleep`, `before initial mallinfo`,
  `after initial mallinfo`, followed by the normal
  `user_main: Begin argument test`.  The last line printed before a freeze
  identifies the next failing path.
- On hardware, the 5-second startup-trace image printed `user_main: trace
  entry` and `user_main: trace before initial usleep`, returned to NSH, then
  after the sleep interval emitted only `us` before the console became
  non-interactive.  This is the first two characters of the next trace line
  (`user_main: trace after initial usleep`), so the blocked `usleep()` syscall
  did wake and return to user code.  The hang is now after a resumed user task
  enters the next `write()` path, not in the PSRAM stack allocation, the first
  task start, or `mallinfo()`.
- The next diagnostic tests whether a blocking protected syscall resumes a
  PSRAM-stacked user task with `BASEPRI` still masked from a kernel critical
  section.  The build option below clears the saved `BASEPRI` field in the
  active `SYS_syscall_return` frame only when returning to unprivileged thread
  mode:

```text
./tools/firmware/stm32n6570-dk/build-psram-verify.sh \
  --app-only --psram-mpu-policy no-wb-nwa --lazy-fpu \
  --bootstrap-uheap-size 0x4000 --syscall-current-frame \
  --syscall-user-basepri0 --ostest-nowait \
  --ostest-delay-usec 5000000 --ostest-startup-trace -j 8
```

- The `BASEPRI=0` startup-trace full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `1bf9013fc675322a114e55ac20fc3c1ca2c424f9f0fd2766bf4d3c20d750cded`.
  If this image continues past `user_main: trace after initial usleep`, the
  remaining bug is interrupt-mask restoration in the protected syscall return
  path.  If it still stops at `us`, the next target is the saved PSP/CONTROL/
  EXC_RETURN state after a blocking syscall wake-up.
- On hardware, the `BASEPRI=0` image still stopped after printing only `us`.
  Clearing saved `BASEPRI` on user-mode syscall return is therefore not the
  root cause.  The performance problem has a good MPU/cache answer
  (`SH_NO + WRITE_BACK + no-write-allocate`), but ordinary protected user task
  stacks allocated from PSRAM are not yet fully reliable across the broader
  syscall/sleep/write path.  Further work should move away from `ostest`
  tracing and toward a smaller protected-mode syscall resume reproducer or a
  kernel-side ring-buffer trace of PSP/CONTROL/EXC_RETURN around blocking
  syscall wake-up.
- Added a smaller `sysresume` diagnostic app so the failing path can be tested
  without the rest of `ostest`.  It prints its local stack address and then runs
  a direct `write() -> usleep() -> write()` sequence.  The `-T` option spawns a
  separate task and returns to NSH, matching the parent/child shape used by the
  `ostest` startup test.

```text
sysresume
sysresume -T
```

- The first `sysresume` full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-full.bin`.
  SHA-256:
  `951ba5a67a99eb90575302fe108e544a7ee1fd349cb3437d830da5f4648c6ba3`.
  Test `sysresume` first.  If it passes on a PSRAM stack but `sysresume -T`
  stops, the issue is tied to a newly created task or parent/child lifetime.  If
  direct `sysresume` stops after `marker before sleep`, the bug is in the
  blocking syscall resume path itself.
- Hardware test result: both direct `sysresume` and `sysresume -T` passed with
  locals on PSRAM stacks.  This proves that ordinary protected-mode PSRAM stacks
  are functional for the minimal `write() -> usleep() -> write()` resume path,
  including a task created with `task_create()`.
- The next minimal check is the `mallinfo()` path, because `ostest user_main`
  calls `mallinfo()` immediately after the first startup sleep.  `sysresume`
  now has `-m` for this:

```text
sysresume -m
sysresume -T -m
```

- The current `sysresume -m` full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-full.bin`.
  SHA-256:
  `33eba1ea47d253b8854a3335edf646e46906787a732f40cd1e22d3954635a994`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe.bin`.
  SHA-256:
  `f7617f39da0d015cd1ae3d5812e7eddf0aca45eb89c65b962817d72a9107f009`.
  If `sysresume -m` stops after `before mallinfo`, the remaining reliability
  problem is in user-heap introspection/traversal with the PSRAM user heap.  If
  it passes, the remaining `ostest` startup hang is outside the generic
  PSRAM-stack syscall resume path and the `mallinfo()` path; likely suspects
  become `ostest`'s single-write startup trace, stdio buffering, argument
  validation, or later test-specific code.
- `ostest` startup tracing now writes each trace line with a short-write-safe
  loop instead of a single `write()`.  The current trace image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `01c052b5843e53fca72cdd1a3f4cd2324dffedfa4912375d1c20b09d42a5eefe`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `991b6c471946cb67a7b2b882557d0c5bc56e37c4d0c8af0e994740d6f66616c5`.
  Use this image only after the `sysresume -m` checks.  A full trace sequence
  should include `trace after initial usleep`, `trace before initial mallinfo`,
  and `trace after initial mallinfo`.
- Hardware result with the short-write-safe trace image: `sysresume -m` stopped
  after printing only `sy` from the post-sleep line, before reaching
  `before mallinfo`.  This points away from `mallinfo()` and toward a blocked
  console `write()` immediately after a blocking syscall resume.  The likely
  failure mode is that a stale `BASEPRI` mask is restored into either
  unprivileged user mode or the next privileged syscall dispatcher entry, so
  serial write can put a few bytes into the TX FIFO and then sleep forever
  waiting for an interrupt that remains masked.
- Added `CONFIG_ARMV8M_SYSCALL_DISPATCH_BASEPRI0` and the script option
  `--syscall-dispatch-basepri0`.  This clears the saved `BASEPRI` field before
  returning from SVC into `arm_dispatch_syscall`, complementing
  `--syscall-user-basepri0`, which clears it before returning to user mode.
  The current candidate image with both clear points enabled is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `2d4facc8d0f539b5a69bd87d4fa4c9ae70dc6c51bb7fdbe44f5f38fd3b64e624`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `43fd4840d3e41daebf242c60caad2d9216d36440a0bd21c560c79ff32b908251`.
  Re-test `sysresume -m` first.  If this passes, follow with
  `sysresume -T -m`, then `ostest`.
- Hardware result with both `BASEPRI=0` clear points enabled: `sysresume -m`
  still stopped immediately after the post-sleep write began, printing only
  `sys`.  This rules out a simple stale interrupt-mask restore as the root
  cause.
- The next candidate changes the protected ARMv8-M syscall model: the outer
  user SVC frame is copied from the PSRAM PSP frame to a per-thread internal
  kernel stack before returning to `arm_dispatch_syscall`.  The syscall return
  path restores the original PSP frame before returning to unprivileged user
  mode.  This keeps ordinary task/pthread stacks in PSRAM, but prevents
  privileged blocking syscall paths from running on the same external user
  stack.
- The syscall-kstack full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `7a5afab3444a1bf39701a6bb6b9af99aa623a2e99b2cd20b8b90a85ce9baa137`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `58ac1a52a994b20c7746f112ee3c9bddf3c1227f9c0f7da9f06d351e4c85218b`.
  Re-test in this order: `sysresume -m`, `sysresume -T -m`,
  `memstress -m 4096 -n 64 -x 4 -t 1000 &`, `ps`, `cat /proc/<pid>/stack`,
  and finally `ostest`.  The `/proc/<pid>/stack` check should still show user
  stacks at `0x9000...`; the internal kernel stack is intentionally separate
  and not reported as the user stack.
- Hardware result with the syscall-kstack image: it did not reach the intended
  `sysresume -m` test.  The image booted through NXboot and second-stage
  external-memory init, allocated `nsh_main` on a PSRAM user stack, then faulted
  during early NSH startup with `UsageFault Invalid state`, `PC=0`,
  `LR=0x0000000c`, `CONTROL=0x0000000d`, and `EXC_RETURN=0xffffffe1`.
  The fault `SP=0x34005830` is outside both the IRQ stack and the reported user
  stack, consistent with the new internal syscall kernel stack.  The user stack
  still contains the `nsh_main -> nsh_session -> write` path.  This narrows the
  failure from a blocking `sysresume` resume bug to the syscall-kstack
  dispatcher entry/return frame itself.
- The strongest current hypothesis is an ARMv8-M lazy-FPU exception-frame
  shape mismatch on the copied dispatcher frame.  The copied internal kstack
  frame can preserve an extended FP frame shape from the user PSP while the
  privileged syscall dispatcher path expects a basic integer frame.  In that
  case exception return decodes the wrong slots and can turn the syscall number
  (`0x0000000c`, `sched_getparam`) into `LR` while taking `PC` from a zero
  slot, matching the observed `LR=0x0c, PC=0` signature.
- Added `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME` and the build-script
  option `--syscall-kstack-basic`.  With this diagnostic enabled, the internal
  syscall kernel stack uses a basic integer-only frame for the privileged
  dispatcher.  The original user PSP frame is still kept and restored by
  `SYS_syscall_return` before returning to unprivileged user mode.
- The syscall-kstack basic-frame full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `b0a889670a70e9a2eb52731239214560606f6fe87c883868a07a3aa6d78fa73c`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `1a10b0dfb71803d52c4b42b2994ac3f8f75409891bb1b990ad172e0027188804`.
  Test this image first for reaching the NSH prompt.  If it reaches NSH,
  re-run `sysresume -m`, `sysresume -T -m`,
  `memstress -m 4096 -n 64 -x 4 -t 1000 &`, `ps`, `cat /proc/<pid>/stack`,
  and then `ostest`.  If it still faults before NSH with `PC=0` and
  `LR=0x0c`, the next step should be a small kernel-side ring trace of the
  copied SVC frame (`REG_R0`, `REG_PC`, `REG_LR`, `REG_EXC_RETURN`, MSP, PSP)
  instead of adding more `ostest` tracing.
- Hardware result with the syscall-kstack basic-frame image: the image now
  reaches the NSH prompt, proving that the earlier pre-NSH `PC=0/LR=0x0c`
  fault was in the copied dispatcher frame shape.  However, pressing Enter at
  the idle prompt immediately faults in `Idle_Task` with `CFSR=0x00100000`
  (`STKOF`), `CONTROL=0x0000000c`, `EXC_RETURN=0xffffffed`, and a saved
  `SP=0x34004950`, outside both the IRQ stack and the idle task stack.  This
  points at the syscall-kstack dispatcher leaking an internal stack pointer or
  stack-limit state into the normal idle/interrupt path after returning to
  user mode.
- The follow-up fix keeps the dispatcher on the internal syscall stack but uses
  PSP for that privileged thread-mode dispatcher frame instead of MSP.  Handler
  mode therefore continues to use the normal interrupt MSP, while the original
  user PSP frame is still restored by `SYS_syscall_return`.  This is controlled
  by `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP` and the build-script option
  `--syscall-kstack-psp`.
- The syscall-kstack basic-frame PSP-dispatch full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `7799312a5107aa8902603ddd1fb8acc351120df9698b91443f16e7d9243b752c`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `89b37f5275c969c112302ba6aea3d9f4aaff0dbec902691172a6a16188af28da`.
  First test that plain Enter at the NSH prompt no longer faults.  If the
  prompt remains responsive, continue with `sysresume -m`,
  `sysresume -T -m`, `memstress -m 4096 -n 64 -x 4 -t 1000 &`, `ps`,
  `cat /proc/<pid>/stack`, and `ostest`.
- Hardware result with the PSP-dispatch image: plain Enter at the NSH prompt
  no longer faults, so the MSP/stack-limit leak is fixed.  `sysresume -m`
  starts on a PSRAM stack, reaches `marker before sleep`, wakes from
  `usleep()`, and then stops after printing only `sys` from the next
  `sysresume: main loop=0 after-usleep ...` line.  Because that line is
  printed before `before mallinfo`, this rules out `mallinfo()` as the current
  stop point.  The remaining failure is now a post-blocking-syscall console
  write/TX-progress problem.
- Extended `CONFIG_ARMV8M_SYSCALL_BARRIER` so the diagnostic also emits
  DSB/ISB in `arm_exception.S` after restoring `BASEPRI` and `CONTROL`, just
  before exception return.  This tests whether the remaining TX stall is an
  exception-return synchronization issue that the earlier C-side SVC barrier
  could not cover.  The updated barrier full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-svcbar-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `8573e58746e8e7c352120d8bfbb7703768d48cfcdb60ff96ce9654cf2faca9b6`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-svcbar-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `2692eaf12cbd50e81bd106bb6d3e4a475c9a9952145103ac8cf98d3a79a94fcb`.
  Test this image first with Enter, then `sysresume -m`.
- Added `CONFIG_STM32N6_USART1_TX_POLL_DRAIN` and the build-script option
  `--usart1-txpoll`.  This diagnostic drains the USART1 software TX buffer by
  polling when TX output is kicked, instead of depending on later TXE/TXFNF
  interrupts.  The current TX-poll full image, rebuilt with boot/app build
  timestamp logging and `memstress` worker pthread stacks set to
  `CONFIG_TESTING_MEMORY_STRESS_STACKSIZE`, is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `430c7634800944278b2d5e8e88d302a863fddc5fe0c07b6df2185cb4bb4dc350`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `f2d101e30e58abded71db675c6e885b88aac64c5158db3d908eeef6b06f730c1`.
  If the barrier image still stops at `sys`, test this TX-poll image next.  If
  TX-poll passes, the root cause is specifically interrupt-driven console TX
  progress after a protected blocking syscall.  If TX-poll also hangs, move the
  next trace point earlier, around syscall return/scheduler state rather than
  the USART TX interrupt path.
- Copied the ARM official references into `docs/armv8-m/`:
  `Arm® Cortex™-M55 Processor Technical Reference Manual.pdf`,
  `Arm® Cortex™-M55 Devices Generic User Guide.pdf`, and
  `Arm® v8-M Architecture Reference Manual.pdf`.
- Relevant ARMv8-M/M55 points, rechecked directly from the PDFs, used for the
  current fix:
  `EXC_RETURN.SPSEL` selects which stack contains the exception frame, and
  exception return updates `CONTROL.SPSEL` automatically.  Handler mode always
  uses MSP.  ARM recommends OS threads use PSP while kernel and exception
  handlers use MSP.  When software changes `CONTROL.SPSEL` with `MSR`, an ISB
  is required immediately after the write.  Exception return itself performs an
  instruction synchronization barrier, but `PRIMASK` and `BASEPRI` are unchanged
  by exception exit.  ISB is the architectural context-synchronization operation
  that makes earlier context-changing effects visible to later fetched
  instructions.
- The remaining `sysresume -m` stop after printing only `sys` is consistent
  with the serial write path filling USART1 only until the hardware FIFO stops
  accepting bytes, then depending on the TXE/TXFNF interrupt for the rest.  If
  `BASEPRI` was just lowered by `leave_critical_section()` after the blocking
  `usleep()` path, but the unmask is not synchronized before the next syscall
  starts UART TX, the first few bytes can be emitted synchronously and the rest
  may never progress.  The PDFs do not state a blanket "MSR BASEPRI requires
  ISB" rule like they do for `CONTROL.SPSEL`, so the next build keeps this as a
  controlled validation point.
- Added `CONFIG_ARMV8M_BASEPRI_ISB` and the build-script option
  `--basepri-isb`.  This emits `isb sy` immediately after the Armv8-M
  `msr basepri, <value>` used by `up_irq_save()`/`up_irq_restore()`.  The
  first validation image keeps `CONFIG_STM32N6_USART1_TX_POLL_DRAIN` disabled,
  so it tests interrupt-driven TX progress rather than hiding the issue with
  polling.
- The BASEPRI-ISB full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `5cc6139c1eb799758cb72a7ba5667c359d3e50efa71ca69bf48e06e3c3193c38`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `36a5899f6f24fbd15bd186909d78e2e9fc914990fc4ef5bc8e791980f9964fc6`.
  This image has `CONFIG_ARMV8M_BASEPRI_ISB=y`,
  `CONFIG_ARMV8M_SYSCALL_BARRIER` disabled, and
  `CONFIG_STM32N6_USART1_TX_POLL_DRAIN` disabled.  Test plain Enter first,
  then `sysresume -m`.  If `sysresume -m` now prints the full
  `after-usleep` line and returns to `nsh>`, continue with `sysresume -T -m`
  and `ostest`.  If it still stops at `sys`, use the TX-poll image above to
  separate a UART IRQ-progress problem from a broader post-wakeup syscall
  resume problem.
- Hardware result with the BASEPRI-ISB image: the board still reached NSH but
  `sysresume -m` stopped immediately after printing only `sy`.  This rejects
  the "BASEPRI write needs only an added ISB" hypothesis for the current hang.
  The next split is therefore explicit: make USART1 a high-priority external
  interrupt without enabling TX polling.  If this image progresses past the
  partial `sys`/`sy` output, the failure is still consistent with normal-priority
  USART1 TXE/TXFNF IRQs being blocked by the post-wakeup priority mask.  If it
  also stops, keep suspicion on USART/NVIC pending state or the syscall/scheduler
  return path rather than only on the BASEPRI synchronization point.
- Added `CONFIG_STM32N6_USART1_HIGH_PRIORITY` and build-script option
  `--usart1-highpri`.  This selects `CONFIG_ARCH_IRQPRIO` and calls
  `up_prioritize_irq(STM32_IRQ_USART1, NVIC_SYSH_HIGH_PRIORITY)` after USART1
  IRQ attachment and before enabling the IRQ.  This is diagnostic bring-up code;
  the default USART1 priority remains unchanged unless the option is enabled.
- The USART1-high-priority full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-uarthipri-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `d2975c0c7448759d9782bcd6321801698a19522d23fd68bc7a470039f333a1cf`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-uarthipri-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `790d33fe0a96075a31bcdbee0f26cedfb243e20f1787811597af5d7a6a1a4a2f`.
  This image has `CONFIG_STM32N6_USART1_HIGH_PRIORITY=y`,
  `CONFIG_ARCH_IRQPRIO=y`, `CONFIG_STM32N6_USART1_TX_POLL_DRAIN` disabled,
  `CONFIG_ARMV8M_BASEPRI_ISB` disabled, and
  `CONFIG_ARMV8M_SYSCALL_BARRIER` disabled.  Test plain Enter first, then
  `sysresume -m`.
- Hardware result with the USART1-high-priority image: plain Enter and a simple
  `s` input still returned to the NSH prompt, but `sysresume -m` again stopped
  immediately after the post-sleep write began, printing only `sy`.  This
  rejects the "normal-priority USART1 TXE/TXFNF IRQ is merely blocked by
  BASEPRI" split.  The next discriminator is the TX-poll image above: if it
  passes, the remaining bug is in interrupt-driven USART TX progress/pending
  state after a protected blocking syscall; if it also hangs, trace earlier in
  the post-wakeup syscall/scheduler return path.
- Build identity logging is now enabled in the normal board bring-up path.
  NuttX already includes `CONFIG_VERSION_BUILD`, `__DATE__`, and `__TIME__` in
  `uname().version` when `CONFIG_LIBC_UNAME_DISABLE_TIMESTAMP` is not set.  The
  STM32N6570-DK bring-up now prints that string with an explicit image role, so
  the serial log should show one `image=nxboot` line from the boot firmware and
  one `image=app` line from the app firmware:

```text
stm32n6570-dk: image=nxboot version=NuttX 0.0.0 2e840a00b1-dirty May 23 2026 15:04:53 arm
stm32n6570-dk: image=app version=NuttX 0.0.0 2e840a00b1-dirty May 23 2026 15:05:19 arm
```

  The timestamp is compile-host local time from the binary itself, so it is
  useful for confirming that both the NXboot image at `0x70000000` and the app
  image at `0x70100000` are the intended rebuild.
- Hardware result with the timestamped TX-poll image: the boot log showed the
  expected distinct `image=nxboot` and `image=app` build timestamps, then
  `sysresume -m` completed:

```text
sysresume: main loop=0 after-usleep ret=0 errno=0
sysresume: before mallinfo
sysresume: after mallinfo arena=02003ffc uordblks=000038cc fordblks=02000730
sysresume: marker after sleep
sysresume: main done local=53595245
nsh>
```

  This proves the reduced path now survives a protected task stack in PSRAM
  across `write() -> usleep() -> write() -> mallinfo()`.  Because the passing
  image uses `CONFIG_STM32N6_USART1_TX_POLL_DRAIN=y`, it does not yet prove the
  normal interrupt-driven USART TX path is fixed.  It does, however, move the
  current reliability focus away from PSRAM user-stack exception-frame restore
  and toward USART TX interrupt/pending progress after a protected blocking
  syscall.  Continue validation with `sysresume -T -m`, a small background
  `memstress`, and then `ostest` on the same image.
- Hardware result with the previous 14:50 TX-poll image under concurrent
  stress: running `sysresume -T -m` and then starting
  `memstress -m 4096 -n 64 -x 4 -t 1000 &` before the child woke caused the
  child to stop after:

```text
sysresume: child loop=0 after-usleep ret=0 errno=0
sysresume: before mallinfo
```

  This is a different failure from the earlier post-sleep UART stall.  The
  child has already resumed from `usleep()` and printed after wake-up, so the
  new stop point is inside `mallinfo()` while the user heap is under concurrent
  malloc/free/realloc pressure.  One plausible local cause is that `memstress`
  created worker pthreads with the global default pthread stack
  (`CONFIG_PTHREAD_STACK_DEFAULT=2048`) even though the `memstress` command
  itself has `CONFIG_TESTING_MEMORY_STRESS_STACKSIZE=8192`.  The rebuilt image
  above makes each `memstress` worker use the testing stack size via
  `pthread_attr_setstacksize()`.
- Re-test the rebuilt 15:05 image in two steps:

```text
sysresume -T -w -m
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
```

  The first command waits for the child and separates a plain child
  `mallinfo()` failure from the concurrent `memstress` case.  The second and
  third commands intentionally reproduce the previous overlap.  If it still
  stops at `before mallinfo`, keep the board alive and capture `ps`,
  `dumpstack 6 12`, and `/proc/<pid>/stack` for the `sysresume-child`,
  `memstress`, and its worker pthreads.
- Hardware result with the 15:05 TX-poll image and `sysresume -T -w -m`: the
  child task completed the whole test body, including the post-sleep
  `mallinfo()` and final `child done`, but the parent did not print the
  `waitpid` result and the NSH prompt did not return:

```text
sysresume: child loop=0 after-usleep ret=0 errno=0
sysresume: before mallinfo
sysresume: after mallinfo arena=02003ffc uordblks=00005adc fordblks=01ffe520
sysresume: marker after sleep
sysresume: child done local=53595245
```

  This is no longer evidence for a PSRAM user-stack resume failure.  The child
  has already resumed from `usleep()`, used a PSRAM stack, touched the heap via
  `mallinfo()`, and returned from the diagnostic body.  The remaining stop is
  in the parent wait/child-exit notification path, or in child task teardown
  before `waitpid()` can complete.
- `sysresume -T -w` now uses a diagnostic completion semaphore instead of a
  blocking `waitpid()` as the primary success condition.  The child posts the
  semaphore after `sysresume_run()` returns and then yields once, allowing the
  parent to print the child-body result before the child enters normal task
  exit cleanup.  The parent also samples `waitpid(pid, ..., WNOHANG)` as a
  non-blocking clue:

```text
sysresume: child notify status=0
sysresume: child wait ret=0 status=0 errno=0
sysresume: waitpid WNOHANG ret=0 status=0 errno=0
```

  If `waitpid WNOHANG` reports `ret=0`, the diagnostic body passed but the task
  had not yet been reaped at the sample point; run `ps` to see whether
  `sysresume-child` remains.  If it reports `ret=<child-pid>`, both the body and
  the task exit notification path completed.
- The rebuilt TX-poll/semaphore-wait full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `12c57c384de6a6c50b1691327f4ad6fa433237f25ca33574879038d51b21e540`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `dea8e247fa07ca28cd0f4ce03731e7aa9704f9064385da015986094b8b5dad7f`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `1259c746060dfcac717b085382e34018e89c523a6497f459d9fa0288887fea14`.
  Expected embedded build strings are approximately
  `May 23 2026 15:16:38` for NXboot and `May 23 2026 15:17:07` for the app
  kernel image.
- Re-test this image in this order:

```text
sysresume -T -w -m
ps
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
ps
```

  The first `ps` is important if `waitpid WNOHANG ret=0`; it tells us whether
  child task teardown is merely delayed or stuck.  The second sequence then
  revisits the original overlap with `memstress` workers using the enlarged
  `CONFIG_TESTING_MEMORY_STRESS_STACKSIZE` stack.
- Hardware result with the 15:16 TX-poll/semaphore-wait image: the child
  reached the explicit completion notification, then the board stopped before
  the parent printed `child wait`:

```text
sysresume: child done local=53595245
sysresume: child notify status=0
```

  This narrows the remaining stop to the post-child-body path.  The child had
  already resumed from `usleep()`, used a PSRAM stack, called `mallinfo()`, and
  reached the notification point.  The new suspect is not the PSRAM user stack
  itself but the protected syscall kernel-stack lifetime around child exit and
  scheduler handoff.
- Fixed one concrete lifetime bug in the ARMv8-M protected syscall stack
  prototype: `up_release_stack()` was freeing `xcp.kstack` immediately.  A task
  can exit while still executing `_exit()` on that per-task syscall stack, so
  immediate `kmm_free()` can release the current live stack before
  `SYS_restore_context` switches to the next task.  The stack is now detached
  from the TCB and put on a small deferred-free list; the list is drained on a
  later protected SVC entry while running from the exception stack.
- `sysresume -T -w` no longer inserts a default `sched_yield()` after
  `sem_post()`.  It now traces `sem_post` directly:

```text
sysresume: child notify status=0
sysresume: child sem_post begin
sysresume: child sem_post ret=0 errno=0
sysresume: child return
```

  Use `sysresume -T -w -y -m` only as a deliberate follow-up test for the
  `sched_yield()` syscall/context-switch path after the no-yield wait case is
  known to pass.
- The rebuilt TX-poll/deferred-kstack-free full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `37878e963c5e1055c3d3e5cbe216278bbdc4e3af916382eb3479fbe072c0a438`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `718c6b6ca59becb651e5f445bfe1b9a9b3d81c5111b7d544101a56f9090de154`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `8f32dd627ab8bdbbefe07cc4d3bb31171d065441071e6b42d68610940635761b`.
  Embedded build strings are `May 23 2026 15:33:51` for NXboot and
  `May 23 2026 15:34:17` for the app.
- Re-test this image in this order:

```text
sysresume -T -w -m
ps
sysresume -T -w -y -m
ps
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
ps
```

  Passing `sysresume -T -w -m` means the child body, child exit, and parent
  semaphore wait path have all survived PSRAM task stacks with deferred syscall
  kstack release.  If only `-y` still stops, the remaining issue is specifically
  the explicit yield/same-priority context-switch path rather than child exit.
- Hardware result with the 15:34 TX-poll/deferred-kstack-free image: the board
  stopped inside the child notification `sem_post()` path, before the child
  could print the return value:

```text
sysresume: child done local=53595245
sysresume: child notify status=0
sysresume: child sem_post begin
```

  This means the PSRAM-backed child stack survived `usleep()`, `mallinfo()`,
  and the diagnostic body.  The active suspect is now the semaphore wake path
  under protected syscall dispatch, not the plain PSRAM user-stack resume path.
- `sysresume -T -w` now creates the diagnostic child one priority level above
  `CONFIG_TESTING_SYSCALL_RESUME_PRIORITY` unless `-P <priority>` is supplied.
  The new start line includes the chosen priority:

```text
sysresume: started child pid=<pid> wait=1 prio=101
```

  The purpose is to keep the child running through `sem_post()` instead of
  immediately switching to the waiting parent from inside the `nxsem_post_slow`
  syscall.  If this image prints `child sem_post ret=0` and then `child wait
  ret=0`, the remaining kernel bug is specifically the syscall-internal
  semaphore wake/context-switch path.  If it still stops at `child sem_post
  begin`, instrument `nxsem_post_slow()` itself.
- The rebuilt TX-poll/deferred-kstack-free/child-priority-split full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `595d80579110cd55158049e5c0033a7586b68e4a704f38d0bce7ae844fceccfc`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `5bb3b669a628fd88971c7c88f82512835f1e7d393f630cf10a3dd3ac87722877`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `d066bd0fac17b91a30a37e448bae512d7b2f6b3f32985a5bd42c96c76862d5ea`.
  Embedded build strings are `May 23 2026 16:06:28` for NXboot and about
  `May 23 2026 16:06:54` for the app.
- Re-test this image in this order:

```text
sysresume -T -w -m
ps
sysresume -T -w -m -P 100
ps
sysresume -T -w -y -m
ps
```

  The default `-w` run tests the no-immediate-switch split.  The explicit
  `-P 100` run recreates the old same-priority case after the split is known.
- Hardware result with the 16:06 child-priority-split image:

```text
sysresume -T -w -m
sysresume: started child pid=5 wait=1 prio=101
...
sysresume: child sem_post ret=0 errno=0
sysresume: child return
sysresume: child wait ret=0 status=0 errno=0
sysresume: waitpid WNOHANG ret=-1 status=0 errno=10

sysresume -T -w -m -P 100
sysresume: started child pid=7 wait=1 prio=100
...
sysresume: child notify status=0
sysresume: child sem_post begin
```

  This confirms that the PSRAM child stack and normal syscall return path are
  now surviving.  The failing condition is the same-priority case.  Since
  `nxsched_add_prioritized()` inserts an equal-priority waiter behind the
  currently-running child, the likely trigger is the next round-robin tick after
  `sem_post()` makes the parent runnable: the tick rotates to the waiting
  parent while the child is still inside the protected syscall dispatcher on
  its syscall kstack.
- Added a scheduler-side guard for the ARMv8-M syscall-kstack prototype:
  `nxsched_process_roundrobin()` now defers equal-priority RR rotation while
  the current TCB has `TCB_FLAG_SYSCALL` and
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK=y`.  This lets `sem_post()` finish and
  return to user mode before same-priority RR scheduling can rotate to the
  parent.
- The rebuilt TX-poll/deferred-kstack-free/RR-syscall-defer full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `5b6d172ff97064be87c7a21f147ed29b0d915ce13f876617b03e5d6e83664b5c`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `e35f6172598f894e6fe2fccd8eceee2aa331c1ce04a350f027b7a52e158dd6c9`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `efd62c983a4cc1ac8c349df36b97653f9e06559bec74d44fc6f836aa4187ba33`.
  Embedded build strings are `May 23 2026 16:22:28` for NXboot and about
  `May 23 2026 16:22:54` for the app.
- Re-test this image in this order:

```text
sysresume -T -w -m -P 100
ps
sysresume -T -w -y -m -P 100
ps
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
ps
```

  Passing the first command closes the old same-priority `sem_post()` stop.
  The `-y` case then deliberately reintroduces an explicit `sched_yield()` after
  `sem_post()` so that yield/context-switch behavior is tested separately.
- Hardware result with the 16:22 RR-syscall-defer image:

```text
sysresume -T -w -m -P 100
sysresume: started child pid=5 wait=1 prio=100
...
sysresume: child sem_post ret=0 errno=0
sysresume: child return
sysresume: child wait ret=0 status=0 errno=0
sysresume: waitpid WNOHANG ret=-1 status=0 errno=10

ps
...
3     3     0 100 RR       Task      - Running ... nsh_main

sysresume -T -w -y -m -P 100
sysresume: started child pid=7 wait=1 prio=100
...
sysresume: child notify status=0
sysresume: child sem_post begin
```

  The first command passing is a real fix for the old same-priority
  `sem_post()` stop: the child completes the notification, returns, the parent
  wakes, and `ps` shows no leftover `sysresume-child`.  The
  `waitpid WNOHANG ret=-1 errno=10` sample is not treated as a failure in this
  build because `CONFIG_SCHED_HAVE_PARENT` is disabled, so the diagnostic
  semaphore is the primary completion check.

  The remaining `-y` run is intended to exercise a different path:
  `sysresume` calls `sched_yield()` after `sem_post()` returns.  The last
  visible line is still `child sem_post begin`, so the exact stop point could
  be a timing-sensitive recurrence in the same-priority `sem_post()` return
  path or the first write immediately after `sem_post()`.  The next candidate
  nevertheless targets the extra path that `-y` adds: in NuttX,
  `sched_yield()` is implemented as a same-priority
  `nxsched_set_priority()`/ready-list reorder, not as the timer round-robin
  path fixed above.
- Added a second scheduler-side guard for the ARMv8-M syscall-kstack
  prototype: `nxsched_running_setpriority()` now defers same-priority
  `sched_yield()` while both the current TCB and the next same-priority TCB
  have `TCB_FLAG_SYSCALL` and `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK=y`.  This is
  deliberately narrow: it only affects the protected syscall-kstack prototype
  and only the "yield from one unfinished syscall into another unfinished
  syscall" case.
- The rebuilt TX-poll/deferred-kstack-free/RR-plus-yield-syscall-defer full
  image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `549ae244a17c4365006de61874bc49dadeea83929fd66322e61a746e69bb4d30`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `36db00d0964e83b64cb32133e548fbbe698408845be7fe45096c5c57b4519d91`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `5c9beba46bf20c92c011451b2c04e8a9784de7452e7abd36100473ebb70714be`.
  Embedded build strings are `May 23 2026 16:29:11` for NXboot and about
  `May 23 2026 16:29:37` for the app.
- Re-test this image in this order:

```text
sysresume -T -w -y -m -P 100
ps
sysresume -T -w -m -P 100
ps
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
ps
```

  If the first command now reaches `child sched_yield ret=0`, `child return`,
  and `child wait ret=0`, the explicit yield path is closed too.  At that point
  the remaining validation should move back to concurrent `memstress` and then
  the original protected KNSh workload.
- Hardware result with the 16:29 RR-plus-yield-syscall-defer image:

```text
sysresume -T -w -y -m -P 100
sysresume: started child pid=5 wait=1 prio=100
...
sysresume: child sem_post begin
sysresume: child sem_post ret=0 errno=0
sysresume: child sched_yield begin
```

  This proves the previous guard was still too narrow.  The notification
  `sem_post()` now returns cleanly, but the child stops when entering the
  explicit `sched_yield()` syscall.  The likely problem is not whether the
  next ready task also carries `TCB_FLAG_SYSCALL`; it is the same-priority
  ready-list reorder while the current task itself is still running on the
  protected syscall kernel stack.
- Tightened the yield guard: `nxsched_running_setpriority()` now treats a
  same-priority setpriority as deferred whenever the current TCB has
  `TCB_FLAG_SYSCALL` under `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK`.  This makes
  `sched_yield()` return from the protected syscall before any same-priority
  ready-list rotation is attempted by a later normal scheduling point.
- The rebuilt TX-poll/deferred-kstack-free/RR-plus-current-syscall-yield-defer
  full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `ad81cb17b6f9212eddf6b3ec4c82d765b1aae39e4dcb47de05763b4a926ee6e2`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-curframe-basepri0-dispbasepri0-svckstack4096-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `89d9e4c20507b3626559306d9467f5ec9ebc80159e761428366aa399bf32cbca`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `cad6615e5bd9a1108feb2ff6a7ed3f63d1e61114979d68a307a7cb6d01466138`.
  Embedded build strings are `May 23 2026 16:38:00` for NXboot and about
  `May 23 2026 16:38:26` for the app.
- Re-test this image in this order:

```text
sysresume -T -w -y -m -P 100
ps
sysresume -T -w -m -P 100
ps
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
ps
```

  Passing the first command should print `child sched_yield ret=0 errno=0`,
  `child return`, and `child wait ret=0 status=0`.
- Hardware result with the 16:38 current-syscall-yield-defer image:

```text
sysresume -T -w -y -m -P 100
sysresume: started child pid=5 wait=1 prio=100
...
sysresume: child done local=53595245
sysresume: child notify status=0
sysresume: child sem_post begin
```

  The same-priority/yield guard is therefore still not the whole fix.  The
  command can now reach the child notification path with a clean PSRAM user
  stack, but it can still stop inside the `sem_post()` syscall before returning
  to user code.  This is not evidence that ordinary PSRAM task stacks are too
  slow or immediately corrupt; the direct `sysresume -m` path and the child
  `usleep()`/`mallinfo()` path have already returned from PSRAM-backed stacks.
- Rechecked the ARM PDFs in `docs/armv8-m`, using the PDFs rather than the
  derived text files:
  - `Arm® Cortex™-M55 Devices Generic User Guide.pdf` says Handler mode always
    uses MSP, while Thread mode selects MSP or PSP through `CONTROL.SPSEL`; an
    exception return can also select the Thread stack through `EXC_RETURN`.
  - `Arm® v8-M Architecture Reference Manual.pdf` shows exception entry first
    uses `PushStack()`, which allocates the exception frame on the current
    stack selected by `LookUpSP()`.  On exception return, `EXC_RETURN.SPSEL` is
    written back to `CONTROL.SPSEL`.
  - The practical consequence for this prototype is subtle: `--syscall-kstack-psp`
    returns from SVC to privileged Thread mode with PSP pointing at the
    per-task syscall kernel stack.  Later interrupts still execute their
    handlers on MSP, but their hardware entry frame, and the NuttX software
    context save in `arm_exception.S`, are based on the interrupted Thread
    context.  When that Thread context is the privileged syscall dispatcher,
    those frames use the syscall PSP stack.
- Current hypothesis after the 16:38 stop: the remaining hang is the
  re-enable-interrupts window at the end of a non-blocking syscall while the
  current task still has `TCB_FLAG_SYSCALL` and another same-priority task has
  been made ready from its own blocked syscall.  This makes the problem a
  protected syscall-kstack/exception nesting issue, not a generic PSRAM stack
  performance issue.
- The next validation image keeps the 16:38 scheduler guards, enables
  `CONFIG_ARMV8M_BASEPRI_ISB`, and raises
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACKSIZE` from 4096 to 8192.  This tests the
  two ARM-doc-driven risks at once: immediate dependence on BASEPRI mask
  changes, and extra PSP consumption by exception entry while running the
  privileged syscall dispatcher.
- The BASEPRI-ISB/8192-syscall-kstack full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-full.bin`.
  SHA-256:
  `83a9ced37faefc6d21b8e878c3a87b731c2997cdc03f42151a09efe4fb1157f3`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace.bin`.
  SHA-256:
  `2b635899671eefc2f471c47fd1a4818f1b86d3b8664005aee351e93923464057`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `daa3edf7ea1aa85c210eae0c970ae1e605145d2b9b3c8a81a7cdf8ec96306e0d`.
  Embedded build strings are `May 23 2026 16:53:18` for NXboot and about
  `May 23 2026 16:53:44` for the app.
- Re-test this image in this order:

```text
sysresume -T -w -y -m -P 100
ps
sysresume -T -w -m -P 100
ps
sysresume -T -m
memstress -m 4096 -n 64 -x 4 -t 1000 &
ps
```

  If this image passes, split the fix with A/B builds: first keep 8192 and
  remove `--basepri-isb`, then keep `--basepri-isb` and return the syscall
  kstack to 4096.  That will tell whether the real fix is synchronization,
  stack depth, or the combination.
- Hardware result with the 16:53 BASEPRI-ISB/8192-syscall-kstack image:

```text
sysresume -T -w -y -m -P 100
sysresume: started child pid=5 wait=1 prio=100
sysresume: child pid=5 local=0x90009f30 region=psram loops=1 delay=5000000
sysresume: child loop=0 before-write
sysresume: marker before sleep
sysresume: child loop=0 after-usleep ret=0 errno=0
sysresume: before mallinfo
```

  This changes the best hypothesis.  The child task has already slept,
  resumed, and written through a PSRAM-backed user stack; it stops only after
  entering the `mallinfo()` check.  In this tree, user `mallinfo()` calls
  `mm_mallinfo(USR_HEAP)`, and `mm_mallinfo()` normally calls `mm_foreach()`,
  which locks the user heap and walks each heap node by adding the encoded
  `MM_SIZEOF_NODE(node)` value.  A corrupted PSRAM heap header can therefore
  make the heap walk loop, jump backwards, jump outside the region, or block
  while taking the heap lock.
- ARM-doc interpretation remains important but is no longer the whole story:
  the M55/ARMv8-M documents make PSRAM Thread stacks architecturally legal as
  long as PSP/MSP selection and exception return state are coherent.  The
  direct `sysresume -m` pass and the child `after-usleep` line show that
  ordinary PSRAM stack access is not the immediate performance blocker.  The
  remaining failure is more likely one of these:
  - `mallinfo()` syscall entry/return still using the protected syscall PSP
    path incorrectly.
  - A user-heap delayed-free or lock interaction after the child resumes.
  - Heap metadata corruption near `0x90000000`, exposed by `mallinfo()`'s
    node traversal rather than by the local stack write itself.
- Added a targeted diagnostic option,
  `CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_TRACE`, enabled from
  `build-psram-verify.sh --sysresume-mallinfo-trace`.  With this option,
  `mm_mallinfo()` prints:
  - `mallinfo_trace: enter` before delayed-free processing.
  - `region=... start=... end=...` before each heap lock.
  - `lock ret=...` after taking the heap lock.
  - Up to 64 heap nodes, including raw size, decoded size, preceding size,
    allocation state, and computed next address.
  - `INVALID` for too-small, unaligned, non-progressing, or out-of-region
    next nodes.
  - `PREV-MISMATCH` for inconsistent previous-free metadata.

  The next test should therefore localize the stop point:
  - No `mallinfo_trace: enter`: syscall entry/dispatch path.
  - `enter` but no `region`: delayed-free path.
  - `region` but no `lock ret`: user heap mutex path.
  - Node output followed by `INVALID`/`PREV-MISMATCH`: user heap metadata
    corruption.
- The BASEPRI-ISB/8192-syscall-kstack/mallinfo-trace full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-full.bin`.
  SHA-256:
  `1d50fb5dcf6e5081f28268d59c417f7f7e04cfdfd7380385a37878963d6631c1`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace.bin`.
  SHA-256:
  `527c30dd40b860eab2f822d4aa5789b9ea1c94d0beabe9e6ca41b3978ba3b6d2`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `a701ba312425baac76ec219ad757fbe5e9c462ae1bd5a4781d92d86f0f3f3110`.
  Embedded build strings are `May 23 2026 17:11:26` for NXboot,
  `May 23 2026 17:11:52` for the app kernel, and
  `May 23 2026 17:11:55` for the user image.
- Re-test this image with the exact failing command:

```text
sysresume -T -w -y -m -P 100
```

  Capture the serial log from `sysresume: before mallinfo` through the first
  `mallinfo_trace:` stop point.  If it reaches `mallinfo_trace: done` and then
  `sysresume: after mallinfo`, the heap walk itself is clean and the next
  target returns to syscall exit/scheduler timing.
- Hardware result with the 17:11 mallinfo-trace image:

```text
sysresume -T -w -y -m -P 100
...
sysresume: before mallinfo
mallinfo_trace_collect: mallinfo_trace: enter heap=0x341e09e0 heapsize=33570432 cur=23260 max=39644 nregions=2
mallinfo_trace_collect: mallinfo_trace: region=0 start=0x341e0b60 end=0x341e49d8
mallinfo_trace_collect: mallinfo_trace: region=0 lock ret=0
mallinfo_trace_collect: mallinfo_trace: r=0 i=0 node=0x341e0b60 raw=00000009 size=00000008 prev=00000000 state=A next=0x341e0b68
mallinfo_trace_collect: mallinfo_trace: INVALID r=0 i=0 node=0x341e0b60 raw=00000009 size=00000008 prev=00000000 next=0x341e0b68 heapend=0x341e49d8
...
sysresume: child wait ret=0 status=0 errno=0
nsh>
```

  This is progress, but not the final localization.  The `INVALID` line was a
  diagnostic false positive: `mm_addregion()` deliberately creates allocated
  guard nodes of size `MM_SIZEOF_ALLOCNODE`, and `mm_foreach()` only requires
  `nodesize >= MM_SIZEOF_ALLOCNODE` for allocated nodes.  The first node above
  is therefore a valid region-0 guard, not heap corruption.  The command
  completed because the trace returned early at that false positive, so the
  original full `mallinfo()` heap walk was bypassed.
- Corrected the trace logic to match the allocator invariants:
  - allocated nodes require `MM_SIZEOF_ALLOCNODE`;
  - free nodes require `MM_MIN_CHUNK`;
  - free nodes now print `flink` and `blink` without first dereferencing them;
  - region end guards are counted and printed separately.
- The corrected BASEPRI-ISB/8192-syscall-kstack/mallinfo-trace full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-full.bin`.
  SHA-256:
  `eb64da1e996853f3b76768d99f8106517a1613ab7df1e9335be7c6586e3cf600`.
  The app-only image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace.bin`.
  SHA-256:
  `de27dac75d6e172f6e282669a1d479b2cceb78bbad80ec034eee576cda265d1d`.
  The NXboot image is:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `eae6380b5b74c991ca009b94746ba00558282d89b966a96660066cb65343f82d`.
  Embedded build strings are `May 23 2026 17:18:08` for NXboot,
  `May 23 2026 17:18:34` for the app kernel, and
  `May 23 2026 17:18:37` for the user image.
- Re-test the corrected image with the same command:

```text
sysresume -T -w -y -m -P 100
```

  The important evidence is now whether the trace reaches `region=1` at
  `0x90000000` and whether a real `INVALID`/`PREV-MISMATCH` appears there.
- Hardware result with the corrected 17:18 mallinfo-trace image:

```text
sysresume -T -w -y -m -P 100
...
mallinfo_trace: region=1 done node=0x91fffff8 heapend=0x91fffff8 count=15 endraw=0000000b endsize=00000008 endprev=01ff5ff8
mallinfo_trace: done total=20 aord=00000011 ord=00000005 uord=00005960 ford=01ffe520
arm_memfault: PANIC!!! Memory Management Fault:
arm_memfault:   CFSR: 00000001 MMFAR: 1be38a8c
arm_memfault: Memory Management Fault Reason:
arm_memfault:   Instruction access violation
up_dump_register: SP: 90009f20 LR: 70191ce1  PC: 9000a000
dump_task:       5     5 100 RR       Task    -   Running ... 0x90008030      8144       660     8.1%    sysresume-child
```

  This moves the failure out of the heap walker.  The trace walked both
  regions monotonically and reached the user-heap end guard; there was no real
  `INVALID` or `PREV-MISMATCH`.  The fault is an instruction access violation
  at `0x9000a000`, which is also:
  - the start of the large free user-heap node printed by the trace; and
  - the top/end of the child stack (`0x90008030 + 8144 = 0x9000a000`).

  The most likely remaining bug is therefore not PSRAM stack throughput and
  not a corrupted heap list.  It is a control-flow restore problem near the
  user stack top: a return slot or exception-return frame is being restored as
  `0x9000a000`, causing the CPU to fetch instructions from a non-executable
  PSRAM heap/stack boundary.
- ARM PDF cross-check:
  - The Cortex-M55 Generic User Guide describes exception entry as stacking a
    frame on the current stack and says that the frame includes the return
    address, restored to PC on exception return.
  - The same guide defines `EXC_RETURN.SPSEL` as the bit selecting whether the
    exception frame is on MSP or PSP, and `EXC_RETURN.FType` as the basic vs FP
    frame shape.
  - The Armv8-M Architecture Reference Manual states that `CONTROL.SPSEL` is
    automatically updated on exception entry and exception return; in Thread
    mode it selects PSP when set and MSP when clear.  It also states that on
    exception return the state context is popped from the selected stack and PC
    is set from the stacked return address.
  - The Armv8-M MMFSR definition labels bit 0 as `IACCVIOL`, an instruction
    access violation.  The latest `CFSR=00000001` is exactly this case.

  These rules match the observed failure: the processor is faithfully trying
  to execute the restored PC.  The restored PC is wrong.
- Built an A/B image that keeps the internal syscall kstack but disables PSP
  dispatch, so the privileged syscall dispatcher runs with MSP selected.  This
  leaves the PSRAM user stacks, PSRAM MPU policy, lazy FPU, basic dispatcher
  frame, BASEPRI/ISB, and mallinfo trace unchanged; only
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP` changes from enabled to disabled.

  Full image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-full.bin`.
  SHA-256:
  `c0f7b5701c7040a21078884c9fdb800b88e13268a9e0f83b8260c35e39442343`.

  App-only image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace.bin`.
  SHA-256:
  `ec041c0d4ef55458e985a441ddcf6e360b40d072b697707212a58d1322cff728`.

  NXboot image:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `584cce817f720bee6e75d0f76ee4c43bd4f9f87a25203afb3e8afe8c7a196270`.

  Embedded build strings are `May 23 2026 17:30:48` for NXboot,
  `May 23 2026 17:31:14` for the app kernel, and
  `May 23 2026 17:31:16` for the user image.  The build confirms:
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP` is not set.
- Hardware result with the 17:30 MSP-dispatch A/B image:

```text
<press Enter at the NSH prompt>
arm_usagefault: PANIC!!! Usage Fault:
arm_usagefault:         IRQ: 3 regs: 0x34006794
arm_usagefault:         CFSR: 00100000 ... CONTROL: 0000000c
arm_usagefault: Usage Fault Reason:
arm_usagefault:         Stack Overflow
up_dump_register: SP: 34006870 LR: f77f6359  PC: 3284e841
dump_stackinfo: User Stack:
dump_stackinfo:   base: 0x34003078
dump_stackinfo:   size: 00001000
dump_task:       0     0   0 FIFO     Kthread -   Running ... Idle_Task
```

  This A/B does not fix the `0x9000a000` return-PC failure.  It makes the
  system less stable: an empty line at the NSH prompt is enough to fault in
  `Idle_Task`, with SP far outside the idle stack range
  (`0x34003078..0x34003478`).  Do not promote MSP-dispatch as the fix for this
  port.  Keep PSP-dispatch enabled while narrowing the remaining bug.
- Added a second diagnostic that avoids the user-facing `mallinfo()` by-value
  return wrapper.  `mm_mallinfo_update(heap, info)` now fills a caller-supplied
  `struct mallinfo`, and the `sysresume` test can call `mallinfo_update(&info)`
  with `CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_INTO=y`.  This tests whether
  the `PC=0x9000a000` failure is specific to returning `struct mallinfo` by
  value across the protected syscall/user wrapper.
- The PSP-dispatch + mallinfo-trace + pointer-output diagnostic full image is:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `198ffa306f0df8644a2e1d008e62b1f063b2b1c3e6d56fe64431f05c56d2da69`.

  App-only image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckbasic-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
  SHA-256:
  `10daedccbc478b893ff0810cd6c673cc8f32c2361cd9ff7bcd5a84bc8ae074eb`.

  NXboot image:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `897e5c386ed69a4e6acc917dfa7fcfdba1ee49654751032b82d14ec1177f9515`.

  Embedded build strings are `May 23 2026 17:41:11` for NXboot,
  `May 23 2026 17:41:37` for the app kernel, and
  `May 23 2026 17:41:40` for the user image.  The build confirms:
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP=y`,
  `CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_TRACE=y`, and
  `CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_INTO=y`.
- Re-test the PSP-dispatch pointer-output image with:

```text
<press Enter a few times>
ps
sysresume -T -w -y -m -P 100
ps
```

  Expected split:
  - If plain Enter and `ps` survive, the MSP-dispatch regression is gone and
    the console is back to the earlier PSP-dispatch baseline.
  - If `sysresume` passes with `sysresume: after mallinfo ret=0 ...`, the
    remaining root is very likely the `mallinfo()` by-value ABI/wrapper path,
    specifically the saved LR/return slot near the top of the PSRAM child
    stack.
  - If it still faults with `PC=0x9000a000`, the corruption is below the
    user-facing `mallinfo()` wrapper, in the PSP-dispatch syscall return path
    or in the stacked return frame restoration itself.
- Hardware result with the 17:41 PSP-dispatch + mallinfo-trace +
  pointer-output image:

```text
<press Enter a few times>
ps
sysresume -T -w -y -m -P 100
...
mallinfo_trace: region=1 done node=0x91fffff8 heapend=0x91fffff8 count=15 endraw=0000000b endsize=00000008 endprev=01ff5ff8
mallinfo_trace: done total=20 aord=00000011 ord=00000005 uord=00005960 ford=01ffe520
arm_memfault: PANIC!!! Memory Management Fault:
arm_memfault:   CFSR: 00000001 MMFAR: 1be38a8c
arm_memfault: Memory Management Fault Reason:
arm_memfault:   Instruction access violation
up_dump_register: SP: 90009f20 LR: 70191cf5  PC: 9000a000
dump_task:       5     5 100 RR       Task    -   Running ... 0x90008030      8144       660     8.1%    sysresume-child
```

  Plain Enter and `ps` survived, so the PSP-dispatch baseline is still better
  than the MSP-dispatch A/B.  The pointer-output diagnostic did not fix the
  failure: the heap trace completed cleanly, but the task faulted before
  `sysresume` printed `after mallinfo ret=0 ...`.  `addr2line` and the user
  image disassembly place `LR=0x70191cf5` immediately after the final trace
  `syslog` inside user-side `mm_mallinfo_update()`.  Therefore the failure is
  below the user-facing `mallinfo()` by-value wrapper.  The same bad target
  remains: `PC=0x9000a000`, the child stack top and the start of the next large
  PSRAM heap free node.

  The user image also contains MVE code in hot libc paths, for example
  `memset` uses vector instructions.  Combining this with the ARMv8-M/M55
  exception rules is important: with lazy FPU/MVE enabled, an exception return
  may restore an extended FP/MVE frame from PSP.  The current
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME=y` diagnostic forces the
  syscall-kstack path toward a basic frame and `arm_control_set_mode()` then
  clears FPCA for that return shape.  That makes `svckbasic` a strong suspect
  for corrupting the frame shape/accounting when user code has touched MVE on
  a PSRAM-backed PSP.
- Built the next A/B image by removing only
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME` while keeping:
  `CONFIG_ARMV8M_LAZYFPU=y`, `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK=y`,
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACKSIZE=8192`,
  `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP=y`,
  `CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_TRACE=y`, and
  `CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_INTO=y`.

  Full image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `23cfd7ca720b29918799b9138fa292e4cb344c43385c974f678d478a6e26784a`.

  App-only image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
  SHA-256:
  `4163d706cde3ec206c761bed56292109a2fda133a69329a170f620c5e4b00dec`.

  NXboot image:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `26e2152cd23e2622a64e25de15086100b2c4a841e9def2629dbb36703f12b798`.

  Embedded build strings are `May 23 2026 17:55:14` for NXboot,
  `May 23 2026 17:55:40` for the app kernel, and
  `May 23 2026 17:55:43` for the user image.  The build confirms:
  `# CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME is not set`.
- Re-test the no-`svckbasic` A/B image with:

```text
<press Enter a few times>
ps
sysresume -T -w -y -m -P 100
ps
```

  Expected split:
  - If it passes and prints `sysresume: after mallinfo ret=0 ...`, the root
    cause is the `svckbasic`/lazy-FPU frame-shape interaction with an
    MVE-active user PSP, not PSRAM stack throughput.
  - If it still faults at `PC=0x9000a000`, the next target is the syscall
    dispatcher's callee-saved register preservation or a no-lazy-FPU A/B.
- Hardware result with the 17:55 no-`svckbasic` A/B image:

```text
sysresume -T -w -y -m -P 100
...
mallinfo_trace: done total=20 aord=00000011 ord=00000005 uord=00005960 ford=01ffe520
sysresume: after mallinfo ret=0 arena=02003ffc uordblks=00005adc fordblks=01ffe520
sysresume: marker after sleep
sysresume: child done local=53595245
sysresume: child notify status=0
sysresume: child sem_post begin
```

  This proves that removing `svckbasic` fixes the previous
  `PC=0x9000a000` instruction-access fault after the full heap trace.  The
  remaining stop is later, at the child completion notification path.  In this
  configuration `CONFIG_PRIORITY_INHERITANCE` and `CONFIG_PRIORITY_PROTECT`
  are both disabled, so the `sem_post()` slow path is not exercising the
  priority-inheritance holder logic.  With parent and child both at priority
  `100`, `nxsched_add_readytorun()` should not preempt the child when waking
  the parent.  Therefore `sem_post()` should normally return and print
  `child sem_post ret=0`.

  The next code-level suspect is the protected syscall dispatcher itself:
  `arm_dispatch_syscall.S` documents that `R4-R11` must be preserved for the
  user-space callee, but it only saved `R4-R6`.  Earlier faults already showed
  `FP=deadbeef`, and `sem_post()` returning through the user stub is exactly a
  path where clobbering callee-saved registers can break the caller before the
  next diagnostic print.
- Fixed `arm_dispatch_syscall.S` to save and restore `R4-R11` plus `LR`
  around the kernel syscall stub call, keeping the dispatcher stack 8-byte
  aligned.  `objdump` confirms the generated dispatcher now stores and loads
  `r4`, `r5`, `r6`, `r7`, `r8`, `r9`, `sl`, `fp`, and `lr` before issuing
  `SYS_syscall_return`.
- Rebuilt the same no-`svckbasic` diagnostic image with the `R4-R11`
  dispatcher fix.

  Full image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `9726b26c5e9cca1ae917987dc33b94832dca923ed6ba97eb0359ea0be7720a88`.

  App-only image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
  SHA-256:
  `a56fccc8e2942ea7f555ac8f8ce8125304634861cb6e28d1a6941c1f2e1a1ba8`.

  NXboot image:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `b35b7834dcfeedf7ce35fe1eabc70010d6d3bce621d4f62f1df486a4c8e27f07`.

  Embedded build strings are `May 23 2026 18:05:05` for NXboot,
  `May 23 2026 18:05:31` for the app kernel, and
  `May 23 2026 18:05:34` for the user image.
- Re-test the dispatcher-save fix with:

```text
<press Enter a few times>
ps
sysresume -T -w -y -m -P 100
ps
```

  Expected split:
  - If it prints `child sem_post ret=0`, `child sched_yield ret=0`,
    `child return`, and `child wait ret=0`, then the remaining completion hang
    was the dispatcher clobbering callee-saved registers.
  - If it still stops at `child sem_post begin`, add a kernel-side
    `nxsem_post_slow()` trace around the atomic count update, wait-list
    removal, `nxsched_add_readytorun()`, and `leave_critical_section()`.
- Hardware result with the 18:05 dispatcher-save image:

```text
sysresume -T -w -y -m -P 100
...
sysresume: after mallinfo ret=0 arena=02003ffc uordblks=00005adc fordblks=01ffe520
sysresume: marker after sleep
sysresume: child done local=53595245
sysresume: child notify status=0
sysresume: child sem_post begin
sysresume: child sem_post ret=0 errno=0
sysresume: child sched_yield begin
sysresume: child sched_yield ret=0 errno=0
sysresume: child return
```

  The `R4-R11` dispatcher fix moved the failure boundary again: the child now
  survives `mallinfo_update()`, `sem_post()`, and `sched_yield()`, then stops
  only after returning from `sysresume_child_main()`.

  The ARM PDFs under `docs/armv8-m` explain why the earlier
  `PC=0x9000a000` fault was an exception-frame problem: exception return only
  runs from Handler mode, uses EXC_RETURN to choose Thread/Handler mode and
  MSP/PSP, restores CONTROL.SPSEL from EXC_RETURN, and restores PC/xPSR from
  the selected stacked frame.  With FP/MVE present, CONTROL.FPCA and
  EXC_RETURN.FType decide whether the return frame includes floating-point
  state or lazy-preserved space.  Therefore the old fault, where PC became the
  child stack top / PSRAM heap free-node address, was consistent with a bad
  stacked return frame, not with a corrupt `mallinfo` heap walk.  Disabling
  forced syscall-kstack basic frames and preserving `R4-R11` are consistent
  with these architectural rules.

  The new stop point is later.  A user task return goes back through
  `nxtask_startup()`, which calls `exit(entrypt(...))`.  `exit()` performs
  libc cleanup including TLS/atexit and `fflush(NULL)`, then calls `_exit()`.
  `_exit()` enters the kernel and `up_exit()` finally switches to the next
  task with `SYS_restore_context`.  Also note that the local
  `sched_yield()` path deliberately defers same-priority yielding while
  `TCB_FLAG_SYSCALL` is set on the ARMv8-M syscall-kstack prototype, so the
  parent is not expected to run merely because the child printed
  `sched_yield ret=0`.

  Current diagnosis: PSRAM heap contents and ordinary syscall return are now
  clean.  The remaining hang is in the post-return task-exit path, either in
  libc `exit()` cleanup or in kernel `_exit()/up_exit()/SYS_restore_context`
  when restoring the parent that is blocked in `sem_wait()`.
- Added two `sysresume` diagnostic switches to split the task-exit path:
  - `-B`: after successful `sem_post()` and optional `sched_yield()`, the
    child sleeps for `delay_usec` before returning.  This forces a blocking
    point before child exit.  If the parent prints `child wait ret=0` during
    this sleep, the semaphore wake and parent syscall-resume path are good,
    and the remaining issue is specifically after child return.
  - `-X`: after notification, the child calls `_exit(ret)` directly instead
    of returning through libc `exit()`.  If `-X` passes, libc exit cleanup is
    the culprit.  If it hangs after `child _exit begin`, the kernel
    `_exit()/up_exit()/SYS_restore_context` path is the culprit.

  Rebuilt the same image with only these additional `sysresume` diagnostics.

  Full image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `c4fead09227073c927ff4c3f41418dd9b07023dd1816438314bf296dd426dba6`.

  App-only image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
  SHA-256:
  `9bb181dc246c85ae401cb9dc1d6112f39cf072c500a4b1042c605ede0ce855d7`.

  NXboot image:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `586d78f59e20e3938dd43f660dcb07b1658600cbb7900f709b783cf2fecb4aee`.

  Embedded build strings are `May 23 2026 18:16:38` for NXboot,
  `May 23 2026 18:17:03` for the app kernel, and
  `May 23 2026 18:17:06` for the user image.
- Re-test the 18:17 diagnostic image in this order:

```text
<press Enter a few times>
ps
sysresume -T -w -y -B -m -P 100
ps
```

  Expected split:
  - If the parent prints `child wait ret=0 ...` while the child is in
    `child post-notify sleep begin`, then the blocked parent can resume and
    the remaining hang is after child return / child exit.
  - If it still never reaches `child wait ret=0`, inspect restore of a parent
    that is blocked inside a syscall-kstack `sem_wait()` frame.

  Then reboot and test direct exit:

```text
sysresume -T -w -y -X -m -P 100
```

  Expected split:
  - If it reaches `child wait ret=0`, focus on libc `exit()` cleanup,
    especially `fflush(NULL)` and task/TLS destructors.
  - If it stops after `child _exit begin`, instrument kernel
    `_exit()`, `nxtask_exit()`, `up_exit()`, and the `SYS_restore_context`
    branch in `arm_svcall()`.
- Hardware result with the 18:17 diagnostic image:

```text
sysresume -T -w -y -B -m -P 100
...
sysresume: child sem_post ret=0 errno=0
sysresume: child sched_yield begin
sysresume: child sched_yield ret=0 errno=0
sysresume: child post-notify sleep begin
<board reset>

sysresume -T -w -y -X -m -P 100
...
sysresume: child notify status=0
sysresume: child sem_post begin
<stopped before sem_post ret>
```

  This split makes libc less likely as the primary root.  The `-B` case
  does not return through `exit()`: the child blocks in a second `usleep()`
  after a successful `sem_post()` and `sched_yield()`.  That blocking point
  should let the parent run from its blocked `sem_wait()` syscall frame.
  Because the parent never prints `child wait ret=...`, the next suspect is
  restoring the parent context that was saved while it slept inside
  `nxsem_wait_slow()` on the protected syscall kernel stack.

  The ARM PDFs in `docs/armv8-m` line up with this suspicion:
  `CONTROL.SPSEL` selects MSP/PSP in Thread mode, exception entry moves
  execution to Handler mode using MSP, and exception return uses the
  `EXC_RETURN` payload plus the selected stack frame to restore Thread mode.
  The M55 documents also define separate MSP/PSP stack limit registers.  A
  blocked syscall-kstack task therefore has three pieces that must agree when
  it is restored: `EXC_RETURN`, `CONTROL.SPSEL`, and the saved PSPLIM/MSPLIM
  value.  A mismatch can restore through the wrong stack or trigger a stack
  limit/return integrity failure even when PSRAM contents are good.

  There is also an independent stack-size issue: repeated `ps` output shows
  `Idle_Task` with a 1024-byte stack at `96.8%!`, and earlier "press Enter"
  failures reported `Usage Fault: Stack Overflow` in `Idle_Task`.  The
  validation build now forces `CONFIG_IDLETHREAD_STACKSIZE=4096` so this
  known overflow does not mask the syscall-kstack bug.
- Added `CONFIG_ARMV8M_SYSCALL_CONTEXT_TRACE` for bring-up only.  It logs:
  - `armctx:` lines for `SYS_restore_context`, `SYS_switch_context`, and
    `SYS_syscall_return`, including TCB, saved regs, `ustkptr`, `kstack`,
    `EXC_RETURN`, `CONTROL`, `BASEPRI`, and SPLIM.
  - `semctx:` lines around `nxsem_wait_slow()` block/resume and
    `nxsem_post_slow()` wakeup/switch decisions.

  Rebuilt the validation image with context trace and a larger idle stack.

  Full image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `ec7a2c04c2e71d0c73d4a2cf6a2ddd427db457064639324d500d18ae2ee7a7a4`.

  App-only image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
  SHA-256:
  `fcbe8e604f9c6d083197a3b2cb335f7af927c735442cd811596afa96505757d7`.

  NXboot image:
  `build/stm32n6570-dk-nxboot.bin`.
  SHA-256:
  `ae63d5faf0572d870aeb91014769eaa7f8545db5ce8832146e6bc350021cf3ed`.

  Build output confirmed:
  - `CONFIG_IDLETHREAD_STACKSIZE=4096`
  - `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK=y`
  - `CONFIG_ARMV8M_SYSCALL_KERNEL_STACKSIZE=8192`
  - `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_PSP=y`
  - `CONFIG_ARMV8M_SYSCALL_KERNEL_STACK_BASIC_FRAME` disabled
  - `CONFIG_ARMV8M_SYSCALL_CONTEXT_TRACE=y`

  Re-test order for the trace image:

```text
<press Enter several times>
ps
sysresume -T -w -y -B -m -P 100
```

  If it still fails, keep the last log window starting at the first
  `semctx: wait-block` line through the reset/fault.  The most important
  values are the parent's `semctx: wait-block`, the child's `semctx:
  post-wake`, and the next `armctx: entry/selected/return` for
  `cmd=2` (`SYS_switch_context`) or `cmd=3` (`SYS_syscall_return`).
- Hardware result with the 18:40 trace image reached the prompt and then
  stopped after:

```text
semctx: wait-block sem=0x34000800 tcb=0x34004a40 pid=3 ...
armctx: return pc=70104c12 ... bp=00000080 ...
```

  This particular stop is earlier than the `sysresume` failure: `pid=3`
  (`nsh_main`) is blocked in `nxsem_wait_slow()` waiting for console input,
  and `pc=0x70104c12` resolves to `sched_unlock()` immediately after
  `up_switch_context()`.  Per the Armv8-M/M55 PDFs, `BASEPRI` is a priority
  mask and exception return restores the processor state from the selected
  frame; a non-zero saved `BASEPRI` can therefore mask normal USART/SysTick
  interrupts until the resumed C path lowers it.  To distinguish a normal
  input wait from a real `BASEPRI` leak, the context trace now also prints
  `lockcnt`, optional `irqcnt`, and `r4..r7`.  In this path `r5` is the
  `sched_unlock()` saved interrupt state that will be written back to
  `BASEPRI` a few instructions after `pc=0x70104c12`.

  Updated trace image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `a5c3aa8fc413223883eec4319d663105babdad1551ea9739cd868d09a018e1f3`.

  App-only image SHA-256:
  `ba2412180e23960fddd7604cd4889bf4a8d612f42482ff2bb643dd053726cc5f`.

  NXboot image SHA-256:
  `822d9a3acbd8b27ae82088ba79de9f894316a145c1bf5a28d9873cdd65851d84`.

  Re-test split:

```text
<boot>
<at the final semctx/armctx wait-block lines, type one character or press Enter>
ps
sysresume -T -w -y -B -m -P 100
```

  Interpret the new fields as:
  - If the selected Idle return frame at `pc=70104c12` has `r5=00000000`,
    that prompt-side stop is just the normal NSH read wait; the next input
    should produce a serial wakeup and more trace.
  - If the same frame has `r5=00000080` or never produces a `semctx:
    post-wake` after input, then a critical-section `BASEPRI` mask is leaking
    into the idle/input wait path.
  - For the original `sysresume` failure, capture the first `semctx:
    post-wake` from the child's notify semaphore and the following `armctx:
    selected/return`; that will show whether the parent is selected but
    restored with an inconsistent `EXC_RETURN`/`CONTROL`/SPLIM/`BASEPRI`
    frame.
- Hardware result with the 18:52 trace image: the prompt-side case is not a
  new hang.  The trace shows `nsh_main` blocking on the console semaphore
  (`sem=0x34000800`), input causing `semctx: post-wake`, then
  `semctx: wait-resume`, followed by `SYS_syscall_return` back to the PSRAM
  user stack.  The characters such as the leading `s`/`y` before
  `arm_context_trace` are just echoed input interleaved with verbose trace
  output.

  The key ARMv8-M/M55 interpretation is:
  - The M55 Devices Generic User Guide says Handler mode always uses MSP,
    while Thread mode stack selection is controlled by `CONTROL.SPSEL`.
  - The same guide says exception return updates `CONTROL` from
    `EXC_RETURN`, and the Armv8-M ARM says exception return writes the
    `EXC_RETURN.SPSEL` value to `CONTROL.SPSEL` before restoring the stacked
    state.
  - `BASEPRI=0` disables the base-priority mask.  The latest trace reaches an
    idle return frame with `bp=00000000`; the earlier `pc=70104c46` frame had
    `r5=00000000`, meaning `sched_unlock()` would also restore `BASEPRI` to
    zero a few instructions later.

  Therefore the current log proves the console wait/wakeup path is healthy:
  USART input can wake `nsh_main`, the syscall return path can restore a
  PSRAM-backed user frame, and the final stop at `pc=701042da`/`701042de` is
  the idle loop (`nx_start` calling `up_idle`) waiting for more work.  In the
  longer capture, the final selected idle frame has `bp=00000000`, so normal
  maskable interrupts are not being held off.  It does not yet exercise the
  original `sysresume -T -w -y -B -m -P 100` child/parent semaphore handoff.
  The next useful capture should start at the
  `sysresume: child sem_post begin` line and include the first
  `semctx: post-wake` for the child completion semaphore plus the following
  `armctx: selected/return`.
- Added `CONFIG_ARMV8M_SYSCALL_CONTEXT_TRACE_SYSRESUME_ONLY` and
  `--syscall-context-trace-sysresume` so the next image can focus on that
  handoff.  Build the focused app with:

```console
./tools/firmware/stm32n6570-dk/build-psram-verify.sh --app-only \
  --lazy-fpu --psram-mpu-policy no-wb-nwa \
  --bootstrap-uheap-size 0x4000 \
  --basepri-isb --syscall-current-frame \
  --syscall-user-basepri0 --syscall-dispatch-basepri0 \
  --syscall-kstack --syscall-kstack-size 8192 \
  --syscall-kstack-psp --syscall-context-trace \
  --syscall-context-trace-sysresume --usart1-txpoll \
  --ostest-nowait --ostest-delay-usec 5000000 \
  --ostest-startup-trace --sysresume-mallinfo-trace \
  --sysresume-mallinfo-into
```

  Then run `sysresume -T -w -y -B -m -P 100`.  A healthy prompt before
  `sysresume` should now be mostly quiet; repeated idle frames without
  `sysresume` traffic only mean the filter is doing its job.

  Built app-only focused trace image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
  SHA-256:
  `f225ae1c77c8185914a2c31f46ba84b552d65a747a0eca498adc5832fa6dc82c`.

  Full image:
  `build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
  SHA-256:
  `51961928575c3403020a018b70eb31620b291af9c512d36bbbbcd52a6930f2a6`.

### sysresume focused trace: parent handoff crossed, exit path next

Latest `sysresume -T -w -y -B -m -P 100` trace shows the original child to
parent semaphore handoff is no longer the blocking point:

- Child pid 5 posts `g_child_done`:
  `semctx: post-wake sem=0x341e0304 switch=0 ... stcb=0x34004c10 spid=4`.
  `switch=0` is expected with equal priority 100; the child continues until
  its explicit `sched_yield()`.
- Parent pid 4 then resumes:
  `semctx: wait-resume sem=0x341e0304 tcb=0x34004c10 pid=4`, followed by
  `sysresume: child wait ret=0 status=0 errno=0` and
  `sysresume: waitpid WNOHANG ret=0 status=0 errno=0`.
- The `nsh>` prompt appears before the child finishes because `-B` asks the
  child to sleep after notifying the parent.  Later child lines after the
  prompt are expected, not evidence that the parent handoff is still stuck.

The remaining interesting boundary is the child task return/exit path after:

```text
sysresume: child post-notify sleep ret=0 errno=0
sysresume: child return
```

Added `taskexit:` trace in `sched/task/task_exit.c`, gated by
`CONFIG_ARMV8M_SYSCALL_CONTEXT_TRACE` and respecting
`CONFIG_ARMV8M_SYSCALL_CONTEXT_TRACE_SYSRESUME_ONLY`, so the next image can
show `nxtask_exit()` selecting the task that will run after a `sysresume*`
task exits.

Rebuilt app-only focused trace image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
SHA-256:
`2751f869854d7951a654501bebbe6e9486428f5fadcfd83a86219ca688227950`.

Full image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
SHA-256:
`629066b88d2de3978cfed6a5b34591cbfd005ee3d0c84495f9926b172b31ae5e`.

### sysresume focused trace: no-B boundary is before notify

Latest `sysresume -T -w -y -m -P 100` trace, without the post-notify
`-B` sleep, stops earlier than the previous `sem_post()`/task-exit
investigation.  The last visible user line is:

```text
sysresume: child done local=53595245
```

The following context trace is the final `write()` syscall return for that
line, returning to the user wrapper at `write+0xe` with LR pointing back into
`sysresume_write_all()`.  The child never prints
`sysresume: child notify status=0`, so the current boundary is after the final
`sysresume_run()` print and before the child notification branch.  This rules
out `sem_post()` and `nxtask_exit()` for this particular no-`-B` stop.

Added focused diagnostics:

- `sysresume: child after-run` immediately after `sysresume_run()` returns.
- `sysresume: child notify branch`, `child status store begin`, and
  `child status store done` around the `g_child_notify`/`g_child_status`
  path.
- `armctx` now also prints `r8`, `r9`, `r10`, `r11`, and `xpsr`, so the next
  capture can check the high-register restore path used by
  `sysresume_run()`'s `ldmia.w sp!, {r4-r11,pc}` epilogue.

Interpret the next `sysresume -T -w -y -m -P 100` capture as:

- No `child after-run`: stuck in the final `write()` return or
  `sysresume_run()` epilogue while unwinding the PSRAM-backed user stack.
- `child after-run` but no `child notify branch`: stuck on the
  `g_child_notify` load/branch.
- `child notify branch` but no `child status store done`: stuck storing
  `g_child_status` in internal user SRAM.
- `child status store done` but no `child notify status=0`: back to a
  user-side `sysresume_printf()`/`write()` boundary after the notify store.

Rebuilt app-only focused trace image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto.bin`.
SHA-256:
`181415c6ca38f7b3fa8027982ae067c5a2c1c14d5e3573dd3eb65cae7cd1fcec`.

Full image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-mitrace-miinto-full.bin`.
SHA-256:
`653efac55ffe686c4f51ff488b3c6e1ea8d4f71d718fad9339be709896d71ae5`.

### sysresume focused trace: mallinfo trace is now the noisy boundary

The `May 23 2026 19:33:37` app image with high-register `armctx` output moved
the no-`-B` stop earlier.  It no longer reaches
`sysresume: child done local=53595245`.  Instead, the last visible kernel trace
is the final `mallinfo_trace_collect()` summary:

```text
mallinfo_trace: done total=20 aord=00000011 ord=00000005 uord=00005960 ford=01ffe520
```

The following SVC context returns to `pc=7019027c`, which resolves to
`nx_vsyslog`, with `lr=70187909` in `vsyslog`.  The high-register fields look
coherent through the preceding `sysresume_run()` and `mallinfo_trace_collect()`
syscalls (`r8` carries loop/node indexes, `r9` sizes, `r10` mallinfo/format
pointers, and `r11` the region id), so this capture does not show an obvious
`r8-r11` restore corruption.  The stronger signal is that the verbose
per-node mallinfo `_alert()` stream is now perturbing the very return path we
are trying to measure.

Added one lower-noise breadcrumb in `sysresume_run()`:

```text
sysresume: after mallinfo_update return
```

This prints immediately after `mallinfo_update(&info)` returns and before the
formatted `sysresume: after mallinfo ret=...` line.  Interpret the next
no-mallinfo-trace capture as:

- No `after mallinfo_update return`: still stuck inside
  `mallinfo_update()` or its SVC return.
- `after mallinfo_update return` but no `after mallinfo ret=...`: stuck after
  returning to user code, in the simple printf/write path.
- `after mallinfo ret=...` and `marker after sleep`: the previous stop was
  introduced by the verbose `mallinfo_trace` `_alert()` traffic.

Rebuilt app-only focused trace image without `--sysresume-mallinfo-trace`,
but still with `--sysresume-mallinfo-into`:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto.bin`.
SHA-256:
`0602d524c5d2679084aa5f2a7c925befe456f49ca858f85a6a16cc28540b29b3`.

Full image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto-full.bin`.
SHA-256:
`f7da345d2ae8515a2266000e6297f8a26363c5961a4086c7ce8d1ffe98b296e8`.

### sysresume focused trace: sem_post trace is the next perturbation

The `May 23 2026 19:40:32` app image without mallinfo trace crossed the
previous boundary.  It printed:

```text
sysresume: after mallinfo_update return
sysresume: after mallinfo ret=0 arena=02003ffc uordblks=00005adc fordblks=01ffe520
sysresume: marker after sleep
sysresume: child done local=53595245
sysresume: child after-run
sysresume: child notify branch
sysresume: child status store begin
sysresume: child status store done
sysresume: child notify status=0
sysresume: child sem_post begin
nxsem_post_trace: se
```

This rules out `mallinfo_update()`, the `sysresume_run()` epilogue, and the
child status store.  The new stop is inside the semaphore diagnostic print
itself, while `sem_post()` is waking the waiting parent.  Split the semaphore
wait/post trace out of `ARMV8M_SYSCALL_CONTEXT_TRACE` into a separate
`ARMV8M_SYSCALL_SEMAPHORE_TRACE` switch and leave it disabled by default.
The SVC context trace stays enabled for the sysresume tasks.

Interpret the next capture as:

- `sysresume: child sem_post ret=0 errno=0`: the previous stop was introduced
  by `nxsem_post_trace()` logging from the semaphore wake path.
- Parent `sysresume: child wait ret=0 status=0 errno=0`: the child-to-parent
  handoff is complete.
- No `child sem_post ret`: continue in `nxsem_post_slow()`/ready-to-run
  handoff with semaphore tracing still disabled.

Rebuilt app-only focused trace image with semaphore trace disabled:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto.bin`.
SHA-256:
`950086ed7c4bb78cc849804ae05cee44e997b819108408340852c35ee471c943`.

Full image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto-full.bin`.
SHA-256:
`815e6d6e4b12d62e250ed7498a7cbb5c0918c401c0353b56f0e53fa9623ccff8`.

### sysresume focused trace: mallinfo returned, formatted print is next

The `May 23 2026 19:46:41` app image confirms the semaphore trace split took
effect: no `semctx:`/`nxsem_*_trace` lines appear.  The run now stops after:

```text
sysresume: before mallinfo
sysresume: after mallinfo_update return
```

The SVC trace around that line shows the `write()` for
`after mallinfo_update return` returning to user mode.  There is no following
syscall entry, so the current boundary is user-side code before the next
`write()`, most likely the `vsnprintf()` path in:

```text
sysresume: after mallinfo ret=%d arena=%08x uordblks=%08x fordblks=%08x
```

Replaced this one mallinfo result line with fixed `write()` calls and a tiny
manual hex printer, avoiding `vsnprintf()` while still reading the returned
`struct mallinfo` fields.  The next image prints:

```text
sysresume: after mallinfo values begin
sysresume: after mallinfo ret=00000000 arena=... uordblks=... fordblks=...
sysresume: after mallinfo values done
```

Interpret the next capture as:

- No `after mallinfo values begin`: stuck immediately after the previous
  fixed `write()` return, before reading the mallinfo fields.
- `after mallinfo values begin` but only a partial value line: stuck while
  reading the returned `struct mallinfo` fields or during one of the fixed
  `write()` calls.
- `after mallinfo values done` and `marker after sleep`: the previous stop was
  the diagnostic `vsnprintf()` path, not `mallinfo_update()`.

Rebuilt app-only focused trace image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto.bin`.
SHA-256:
`47ac69437047c054cd2b2314b483f5e425e4269f6270f7c24418db78b6c47e78`.

Full image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto-full.bin`.
SHA-256:
`ebae7253d92c6f56fad4af3bf3fa1f2ae66323403a084acfcce860754f1eb138`.

### sysresume focused trace: mallinfo_update entry is the live boundary

The `May 23 2026 19:57:39` app image did not reach the new
`after mallinfo values begin` marker.  It stops after:

```text
sysresume: before mallinfo
```

The following SVC context trace is the return from that fixed `write()`;
there is no `sysresume: after mallinfo_update return`, so the live boundary
has moved back into the user-side `mallinfo_update(&info)` call path rather
than the mallinfo value formatting path.

The rebuilt user image places these relevant symbols at:

```text
7018a638 sysresume_run
7018a6ea bl mallinfo_update
70191d30 mallinfo_update
70191e04 mm_mallinfo_update
70191f1c mm_foreach
```

Added low-noise fixed `write()` probes, compiled only for the user side of
`CONFIG_TESTING_SYSCALL_RESUME_MALLINFO_INTO`:

```text
mallinfo_update enter
mm_mallinfo_update enter
mm_mallinfo_update after memset
mm_mallinfo_update before foreach
mm_foreach enter
mm_foreach after delaylist
mm_foreach region begin
mm_foreach before lock
mm_foreach after lock
mm_foreach before unlock
mm_foreach after unlock
mm_foreach leave
mm_mallinfo_update after foreach
mm_mallinfo_update after arena
mallinfo_update leave
```

Interpret the next `sysresume -T -w -y -m -P 100` capture as:

- No `mallinfo_update enter`: stuck between returning from the previous
  `write()` and entering/calling the user-side mallinfo wrapper.
- `mallinfo_update enter` but no `mm_mallinfo_update enter`: stuck while
  loading `USR_HEAP` or branching into the common heap helper.
- `mm_mallinfo_update enter` but no `after memset`: stuck clearing the
  caller-provided `struct mallinfo` on the PSRAM-backed user stack.
- `before foreach` but no `mm_foreach enter`: stuck at the common helper call
  boundary.
- `mm_foreach before lock` but no `after lock`: stuck in `mm_lock()` for the
  user heap.
- `mm_foreach after lock` but no `before unlock`: stuck walking heap nodes or
  in the mallinfo node handler.
- `mallinfo_update leave` and then `after mallinfo_update return`: the
  current stop was caused by the previous probe placement, and the next
  boundary is again after mallinfo returns.

Rebuilt app-only focused trace image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto.bin`.
SHA-256:
`f3661e2e2ab1b40a51deb87adbbe32b0b104a7f9f40bd29a54fa42a3e2249cbf`.

Full image:
`build/stm32n6570-dk-psram-verify-lazyfpu-no-wb-nwa-uheap0x4000-basepriisb-curframe-basepri0-dispbasepri0-svckstack8192-svckpsp-svctrace-sysresume-txpoll-ostnowait-ostdelay5000000-osttrace-miinto-full.bin`.
SHA-256:
`182250986ae5aaeddad19e263c2cc21cfc0aa7d01a4c953de4a45ddd6c0bbe5a`.

### 2026-05-23 PSRAM-stack syscall resume fix confirmed

The protected KNSh PSRAM-stack failure is fixed in the cleaned build.  The
successful hardware smoke tests were:

```text
sysresume -T -w -P 100
sysresume -T -w -y -P 100
```

Both commands created a child whose local variable was on a PSRAM-backed user
stack (`0x90009f30 region=psram`).  The child resumed after `usleep()`, posted
the completion semaphore, returned through task exit, and the parent observed:

```text
sysresume: child wait ret=0 status=0 errno=0
sysresume: waitpid WNOHANG ret=-1 status=0 errno=10
```

The final `ECHILD` (`errno=10`) is expected because the blocking wait path had
already consumed the child.  The `-y` variant also completed
`sched_yield()` after `sem_post()`, covering the equal-priority yield corner
that previously exposed the remaining context-return issue.

The fixes that matter are:

- Lazy-FPU frame handling now keeps integer-only threads on basic exception
  frames and preserves the active frame shape when returning from protected
  syscalls.  This avoids restoring PSP with the wrong standard/extended-frame
  offset after a syscall entered from a PSRAM stack.
- Protected syscall dispatch can run from a per-thread internal kernel stack.
  The original user PSP frame is saved and restored on `SYS_syscall_return`,
  while blocking kernel work such as `write()`, `usleep()`, `sem_wait()`, and
  `sem_post()` no longer runs directly on the external PSRAM user stack.
- The syscall-kstack path uses PSP for the dispatcher frame and frees the
  per-task syscall stack through a deferred release path, so a task can exit
  while its last syscall/exit path is still unwinding safely.
- BASEPRI is cleared at the protected syscall dispatch and user-return
  boundaries, with an optional ISB after BASEPRI writes.  This prevents a
  stale syscall/critical-section interrupt mask from being inherited by resumed
  user code or serial/semaphore kernel paths.
- ARMv8-M exception stack accounting now accounts for extended FP frames when
  computing the pre-exception SP, preventing hardware stack-limit state from
  being derived from the wrong frame size.
- Same-priority round-robin/yield decisions are deferred while a task is still
  inside the protected syscall path.  That lets the syscall return to a normal
  user frame before equal-priority scheduling rotates the task.
- The syscall dispatcher assembly now preserves `r4-r11` across the kernel
  stub call, preventing callee-saved high-register corruption in user code
  resumed after a protected syscall.

Temporary bring-up diagnostics have been removed from the cleaned code path:
the `armctx:`, `semctx:`, `taskexit:`, `mallinfo_trace:`, and extra
`sysresume` breadcrumb prints are no longer part of normal validation images.
The remaining `sysresume` app is kept as a focused regression test for
protected syscall resume on PSRAM-backed user stacks.

### 2026-05-23 ARMv8-M manual cross-check cleanup

After re-reading the local ARMv8-M/M55 PDFs under `docs/armv8-m`, the cleaned
syscall-resume patch was tightened in three places:

- With `FPCCR.ASPEN` enabled, ARMv8-M creates an FP context when software
  executes an FP/MVE instruction while `CONTROL.FPCA` is clear.  The common
  exception path and direct IRQ path therefore must not touch `FPSCR` for a
  basic integer-only lazy-FPU frame.  The exception handler now updates
  `FPSCR.LTPSIZE` only when the interrupted context already has an extended FP
  frame; `exception_direct()` skips the `vmsr fpscr` helper under lazy FPU.
- The syscall-kstack frame copy no longer uses the generic `memcpy()` path.
  Current STM32N6 images enable the ARMv8-M optimized libc memory routines,
  which can use MVE instructions; the SVC frame copy now uses an explicit
  word loop so it does not accidentally create FP/MVE state in the lazy-FPU
  exception path.
- FPU setup now follows the architecture barrier requirement after changing
  context-affecting registers: `CONTROL` writes are followed by `ISB`, and
  enabling CP10/CP11 in `CPACR` is followed by `DSB; ISB`.
- `ARMV8M_SYSCALL_KERNEL_STACK` now depends on a real interrupt stack.  The
  syscall-kstack release path deliberately defers freeing per-task syscall
  stacks until a later SVC entry, so the SVC handler must be running from the
  interrupt stack rather than from the syscall stack being retired.

The Kconfig help text was also changed from "diagnostic" language to the
actual invariants that these options enforce: active exception-frame shape,
BASEPRI handoff boundaries, and internal protected-syscall stack lifetime.

### 2026-05-23 KNSh heap alignment with STM32H7S78-DK

After the protected-syscall resume path was fixed, the normal non-LCD
`stm32n6570-dk:knsh` heap setup was temporarily aligned with
`stm32h7s78-dk:knsh` by making `up_allocate_heap()` return the configured
PSRAM window directly.  The image built and NXboot jumped into it, but the app
stalled after:

```text
XSPI1 PSRAM already memory-mapped refresh=396
XSPI1 PSRAM self-test passed
```

That placed the user allocator control structure, heap mutex, first free-node
headers, and early user allocations in PSRAM while `nx_start()` was still
constructing the protected runtime.  Raw PSRAM access was valid, but this was
too early for using memory-mapped PSRAM as the primary protected user heap on
STM32N6570-DK.

The corrected KNSh layout is:

- `up_allocate_heap()` returns an `8 KiB` internal user SRAM bootstrap heap.
- `up_allocate_kheap()` returns internal SRAM below
  `CONFIG_STM32N6_PROTECTED_USRAM_BASE`, so the kernel heap stays internal.
- `arm_addregion()` initializes/verifies XSPI1 PSRAM idempotently, updates the
  MPU mapping for user access, and appends PSRAM as the secondary user heap.
- `CONFIG_MM_REGIONS=2` and `CONFIG_STM32N6_PROTECTED_UHEAP_SIZE=0x2000` are
  required for normal protected KNSh.

This keeps the H7S78-style firmware packaging flow, but not the direct-PSRAM
initial user heap placement.  On N6570-DK, PSRAM is the large user heap
extension, while the allocator bootstrap remains in internal user SRAM.

### 2026-05-23 PSRAM cache policy for performance

The STM32N6570-DK performance target is not to hide LVGL or future Linux-like
workloads in internal SRAM.  Internal SRAM is only a bootstrap/control region;
ordinary user stacks and large allocations must be able to live in XSPI1
PSRAM with usable CPU performance.

The key ARM Cortex-M55 rule is that D-side Shareable Normal-memory
transactions are treated as Non-cacheable by the L1 data cache.  Therefore the
old-looking-safe `SH_OUTER + WRITE_BACK` PSRAM mapping did not actually give
CPU heap/stack traffic the intended write-back D-cache behavior.  STM32H7S78
does not hit this exact rule because it uses the Cortex-M7 MPU/cache model.

The normal STM32N6 PSRAM mapping now defaults to:

```text
CONFIG_STM32N6_PSRAM_MPU_SHARE_NONE=y
CONFIG_STM32N6_PSRAM_MPU_WRITE_BACK=y
CONFIG_STM32N6_PSRAM_MPU_NO_WRITE_ALLOCATE=y
```

This maps CPU-owned PSRAM as Non-shareable write-back, read-allocate,
no-write-allocate memory.  The no-write-allocate part avoids the M55
write-allocate linefill path that previously made cacheable PSRAM unreliable
or slow for PSP-backed exception/SVC/context-switch traffic.

`tools/firmware/stm32n6570-dk/build-psram-verify.sh` uses the same
`no-wb-nwa` policy by default, while still accepting the older policy names
for A/B regression builds.

This policy is for CPU-owned heap and stacks.  PSRAM ranges shared with LTDC,
DMA, or other bus masters still need explicit ownership handoff cache
maintenance, or a more specific non-cacheable MPU region for the shared
window.  On the LVGL build, the first `2 MiB` PSRAM window is reserved for the
double-buffered framebuffer; the app user heap starts at `0x90200000`.

Validated `stm32n6570-dk:knsh-lvgl` after the cleaned rebuild:

```text
NXboot version: May 23 2026 22:07:44
app version:    May 23 2026 22:08:09
full image:     build/stm32n6570-dk-knsh-lvgl-full.bin
sha256:         97991c89c026576fd04408a92dc8e50237f66d8661bb464a286c5b24f073913b
```

The hardware LVGL benchmark now shows the PSRAM policy is effective:

```text
[LVGL] All scenes avg.,35%, 18, 20, 13, 7
```

The previous failing PSRAM-backed KNSh LVGL build was around `75%` CPU,
`8 FPS`, `261 ms` average time, and `254 ms` render time.  The cleaned
`SH_NONE + WRITE_BACK + NO_WRITE_ALLOCATE` build reduces average render time
to `13 ms` while flush time remains `7 ms`, confirming the major bottleneck
was CPU access to cache-ineffective PSRAM rather than LTDC page flipping.
