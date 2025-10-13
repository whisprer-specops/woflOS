AS OF 2025-10-13:06:00 THIS IS NOW SPECIFICALLY GOING TO BE A SECUTIY ANGLED OS AND HENCE THERE IS SOME CHANGING OF DIRECTION. IT CURRENTLY BOOTS UP TO INTERUPT HANDLING BUT HAS HIT A SNAG THERE AND AWAITS FURTHER DEVELOPMENT.

# woflOS - A Stable, Resource-Efficient Operating System

**Architecture:** RISC-V 64-bit  
**Kernel Type:** Microkernel  
**Language:** Rust  
**Version:** 0.4.0 - Interactive Shell

## Project Goals

- **Stability First:** Microkernel architecture + Rust memory safety
- **Resource Efficiency:** Optimized for minimal memory/CPU usage
- **Modern Design:** Clean RISC-V ISA, no legacy baggage

## Memory System Architecture

woflOS implements a two-tier memory management system:

### Physical Frame Allocator
- **Type:** Bitmap-based allocator
- **Page Size:** 4KB (standard RISC-V)
- **Thread-Safe:** Uses atomic operations for SMP safety
- **Performance:** O(n) allocation in worst case, but with hints for O(1) average case
- **Capacity:** Up to 128MB (32,768 frames)

### Kernel Heap Allocator  
- **Type:** Bump allocator (simple and fast)
- **Size:** 1MB dedicated kernel heap
- **Features:** Enables `alloc` crate (Vec, String, Box, etc.)
- **Trade-off:** No deallocation support (fine for long-lived kernel objects)

Memory Layout:
```
0x88000000 ┌─────────────────────┐ Top of RAM
           │ Free Physical Pages │ ← Frame allocator manages this
           ├─────────────────────┤
           │ Kernel Heap (1MB)   │ ← Bump allocator
           ├─────────────────────┤
           │ Kernel .data/.bss   │
0x80200000 ├─────────────────────┤ Kernel starts here
           │ OpenSBI Firmware    │
0x80000000 └─────────────────────┘ RAM starts here
```

## Setup Instructions

### 1. Create Project Structure

In your WSL Ubuntu terminal:

```bash
mkdir -p woflOS/{src,.cargo}
cd woflOS
```

### 2. Create Files

Copy each artifact into its respective file:
- `Cargo.toml` → project root
- `.cargo/config.toml` → `.cargo/` directory
- `linker.ld` → project root
- `src/main.rs` → `src/` directory
- `src/uart.rs` → `src/` directory
- `build.sh` → project root

### 3. Make Build Script Executable

```bash
chmod +x build.sh
```

### 4. Build and Run!

```bash
./build.sh
```

You should see woflOS boot with ASCII art and status messages!

## QEMU Controls

- **Quit QEMU:** Press `Ctrl+A` then `X`
- **View QEMU monitor:** Press `Ctrl+A` then `C`

## Project Structure

```
woflOS/
├── Cargo.toml           # Rust project config
├── .cargo/
│   └── config.toml      # RISC-V build settings
├── linker.ld            # Memory layout
├── src/
│   ├── main.rs          # Kernel entry point
│   └── uart.rs          # Serial console driver
└── build.sh             # Build and run script
```

## What's Working (v0.1.0)

- [x] Boots on RISC-V in QEMU
- [x] Serial console output via UART
- [x] BSS initialization
- [x] Panic handler
- [x] Power-efficient idle loop (WFI instruction)

## Coming Next (v0.2.0)

- [ ] Memory management (page allocator)
- [ ] Process/thread scheduling
- [ ] Interrupt handling
- [ ] System calls
- [ ] IPC for microkernel services

## Troubleshooting

**Build fails with "can't find crate":**
```bash
rustup target add riscv64gc-unknown-none-elf
```

**QEMU not found:**
```bash
sudo apt install qemu-system-misc
```

**Nothing appears in QEMU:**
- Make sure you're using `-nographic` and `-serial mon:stdio`
- Check that the binary was created in `target/riscv64gc-unknown-none-elf/release/`

---

**Built with 🐺 by wofl and Claude**
