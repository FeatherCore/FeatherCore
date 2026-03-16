# FeatherCore Architecture Support Library

## Overview

The FeatherCore Architecture Support Library (`feathercore-arch`) provides low-level architecture-specific implementations for the FeatherCore OS. It supports multiple ARM and RISC-V architectures with both MMU and no-MMU configurations.

## Supported Architectures

### ARM Architectures

#### M-Profile (Microcontroller)

| Architecture | Feature Flag | Cores | MMU Support | Typical Use |
|-------------|-------------|-------|-------------|-------------|
| ARMv6-M | `armv6-m` | Cortex-M0, Cortex-M0+ | No MMU | Ultra-low-power MCUs |
| ARMv7-M | `armv7-m` | Cortex-M3, Cortex-M4 | Optional | General-purpose MCUs |
| ARMv7-EM | `armv7-em` | Cortex-M7, Cortex-M4F | Optional | High-performance MCUs with FPU |
| ARMv8-M Baseline | `armv8-m-base` | Cortex-M23 | Optional | Secure enclave MCUs |
| ARMv8-M Mainline | `armv8-m-main` | Cortex-M33, Cortex-M55 | Optional | Secure MCUs with TrustZone |

#### A-Profile (Application Processor)

| Architecture | Feature Flag | Cores | MMU Support | Typical Use |
|-------------|-------------|-------|-------------|-------------|
| ARMv7-A | `armv7-a` | Cortex-A5, A7, A8, A15 | With MMU | Application processors |
| ARMv8-A | `armv8-a` | Cortex-A53, A72 | With MMU | 64-bit application processors |
| ARMv9-A | `armv9-a` | Latest cores | With MMU | Next-gen application processors |

### RISC-V Architectures

| Architecture | Feature Flag | Extensions | MMU Support | Typical Use |
|-------------|-------------|------------|-------------|-------------|
| RV32IMAC | `riscv32imac` | I, M, A, C | Optional | Embedded MCUs |
| RV64GC | `riscv64gc` | G (I,M,A,F,D), C | With MMU | Application processors |

## Usage

### Cargo.toml Configuration

#### For ARMv7-M (Cortex-M3/M4) without MMU:

```toml
[dependencies]
feathercore-arch = { path = "../common/arch", features = ["armv7-m", "no_mmu"] }
```

#### For ARMv7-A (Cortex-A7) with MMU:

```toml
[dependencies]
feathercore-arch = { path = "../common/arch", features = ["armv7-a", "with_mmu"] }
```

#### For ARMv8-M (Cortex-M33) with MMU:

```toml
[dependencies]
feathercore-arch = { path = "../common/arch", features = ["armv8-m-main", "with_mmu"] }
```

#### For RISC-V RV32IMAC without MMU:

```toml
[dependencies]
feathercore-arch = { path = "../common/arch", features = ["riscv32imac", "no_mmu"] }
```

#### For RISC-V RV64GC with MMU:

```toml
[dependencies]
feathercore-arch = { path = "../common/arch", features = ["riscv64gc", "with_mmu"] }
```

## API Reference

### Core Functions

#### `init()`

Initializes architecture-specific components.

```rust
use feathercore_arch::init;

#[no_mangle]
pub extern "C" fn arch_init() {
    init();
}
```

#### `TaskContext`

Task context structure for context switching.

```rust
use feathercore_arch::TaskContext;

let mut context = TaskContext::new();
context.init_user_stack(stack_top, entry_point, arg);
```

#### `switch_context()`

Switches execution from one task to another.

```rust
use feathercore_arch::{TaskContext, switch_context};

unsafe {
    switch_context(&from_task, &to_task);
}
```

#### `start_first_task()`

Starts execution of the first task in the system.

```rust
use feathercore_arch::{TaskContext, start_first_task};

unsafe {
    start_first_task(&first_task_context);
}
```

#### `jump_to_kernel()`

Transfers control to the kernel entry point (used by bootloaders).

```rust
use feathercore_arch::jump_to_kernel;

unsafe {
    jump_to_kernel(vector_table_addr);
}
```

### Architecture-Specific Modules

#### ARM Module

```rust
#[cfg(feature = "arm")]
use feathercore_arch::arm;

// ARM-specific initialization
arm::init();

// ARM task context
let arm_context = arm::ArmTaskContext::new();
```

#### RISC-V Module

```rust
#[cfg(feature = "riscv")]
use feathercore_arch::riscv;

// RISC-V-specific initialization
riscv::init();

// RISC-V task context
let riscv_context = riscv::RiscvTaskContext::new();
```

## Implementation Details

### Context Switching

#### ARM M-Profile

For ARMv7-M and ARMv8-M, context switching uses the PendSV exception:

1. Trigger PendSV by setting PENDSVSET bit in NVIC_ICSR
2. PendSV handler saves current context
3. PendSV handler restores target context
4. Exception return to target task

For ARMv6-M, a simpler software interrupt approach is used.

#### ARM A-Profile

Context switching on A-profile is more complex:

1. Save banked registers
2. Handle MMU/TLB flushes
3. Manage cache coherency
4. Perform exception return or direct jump

#### RISC-V RV32IMAC

Context switching via:

1. Software interrupt (MSI)
2. Timer interrupt
3. Direct function call (cooperative)

#### RISC-V RV64GC

Full context switching including:

1. Callee-saved registers (ra, sp, gp, tp, s0-s11)
2. FPU registers (f0-f31) if enabled
3. MMU management (SATP register)
4. Privilege mode transitions (M/S/U)

### Memory Management

#### No-MMU Configuration

- Flat memory model
- Direct physical addressing
- No memory protection
- Suitable for embedded systems

#### With-MMU Configuration

- Virtual memory support
- Page table management
- Memory protection
- Process isolation
- Suitable for application processors

### Exception Handling

#### ARM Exception Handlers

- Reset handler
- NMI handler
- HardFault handler
- MemManage handler
- BusFault handler
- UsageFault handler
- SVCall handler
- DebugMonitor handler
- PendSV handler (context switching)
- SysTick handler

#### RISC-V Trap Handlers

- Machine mode traps (mtrapvec)
- Supervisor mode traps (stvec)
- Interrupts (timer, external, software)
- Exceptions (illegal instruction, load/store faults)

## Examples

### Minimal ARMv7-M Application

```rust
#![no_std]
#![no_main]

use feathercore_arch::{init, TaskContext, switch_context, start_first_task};

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialize architecture
    init();
    
    // Setup first task
    let mut task1 = TaskContext::new();
    task1.init_user_stack(0x20001000, task1_entry as usize, 0);
    
    // Start first task
    unsafe {
        start_first_task(&task1);
    }
}

fn task1_entry(_arg: usize) {
    // Task implementation
    loop {
        feathercore_arch::wfi!();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

### Minimal RISC-V RV32IMAC Application

```rust
#![no_std]
#![no_main]

use feathercore_arch::{init, TaskContext, start_first_task};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Initialize architecture
    init();
    
    // Setup exception handlers
    feathercore_arch::setup_exception_handlers();
    
    // Setup first task
    let mut task1 = TaskContext::new();
    task1.init_user_stack(0x80001000, task1_entry as usize, 0);
    
    // Start first task
    unsafe {
        start_first_task(&task1);
    }
}

fn task1_entry(_arg: usize) {
    // Task implementation
    loop {
        feathercore_arch::wfi!();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

## Build Configuration

### Cargo Features

| Feature | Description |
|---------|-------------|
| `arm` | Enable ARM architecture support |
| `riscv` | Enable RISC-V architecture support |
| `armv6-m` | ARMv6-M profile |
| `armv7-m` | ARMv7-M profile |
| `armv7-em` | ARMv7-EM profile |
| `armv8-m-base` | ARMv8-M Baseline |
| `armv8-m-main` | ARMv8-M Mainline |
| `armv7-a` | ARMv7-A profile |
| `armv8-a` | ARMv8-A profile |
| `armv9-a` | ARMv9-A profile |
| `riscv32imac` | RV32IMAC |
| `riscv64gc` | RV64GC |
| `no_mmu` | No MMU support |
| `with_mmu` | With MMU support |

### Build Profiles

```toml
[profile.dev]
opt-level = 1
debug = true

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
debug = false
```

## Platform Integration

The architecture library is designed to work with platform-specific crates that provide:

1. **Memory map definitions**
2. **Peripheral register definitions**
3. **Interrupt numbers**
4. **Vector table placement**
5. **Linker scripts**

Example platform crate structure:

```
platform/
├── stm32f4/       # STM32F4 series (ARMv7-M)
├── stm32h7/       # STM32H7 series (ARMv7-EM)
├── raspberrypi/   # Raspberry Pi (ARMv7-A/ARMv8-A)
├── gd32vf103/     # GigaDevice GD32V (RV32IMAC)
└── sifive/        # SiFive U74 (RV64GC)
```

## Testing

### Unit Tests

```bash
# Build for ARMv7-M
cargo build --features armv7-m,no_mmu --target thumbv7m-none-eabi

# Build for RISC-V RV32IMAC
cargo build --features riscv32imac,no_mmu --target riscv32imac-none-elf

# Build for ARMv7-A
cargo build --features armv7-a,with_mmu --target armv7-unknown-none-gnueabihf

# Build for RISC-V RV64GC
cargo build --features riscv64gc,with_mmu --target riscv64gc-unknown-none-elf
```

## Contributing

When contributing architecture-specific code:

1. Follow the existing module structure
2. Document all public APIs
3. Provide examples for each architecture
4. Test on real hardware when possible
5. Consider both MMU and no-MMU configurations

## License

This project is licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## References

- [ARM Architecture Reference Manual](https://developer.arm.com/documentation)
- [RISC-V Specifications](https://riscv.org/specifications/)
- [Cortex-M Programming Guide](https://developer.arm.com/documentation/dui0553/)
- [RISC-V Privileged Architecture](https://riscv.org/specifications/privileged-isa/)
