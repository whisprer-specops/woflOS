# woflOS Changelog

## v0.3.0 - Layer 1: Privilege Transitions (Planned)
**Goals:**
- [ ] User mode context switching
- [ ] Syscall interface (ecall/sret)
- [ ] Basic process structure
- [ ] Privilege level enforcement
- [ ] First userspace program

---

## v0.2.0 - Layer 0: Trap Handling (2025-10-15) ✅
**Achievements:**
- ✅ Timer interrupts working (1Hz, stable)
- ✅ Full trap handler with context switching
- ✅ All 31 registers saved/restored on interrupt
- ✅ Exception dispatcher (interrupt vs exception)
- ✅ Hex number printing in interrupt context
- ✅ SBI timer calls working correctly

**Bug Fixes:**
- Fixed trap handler register save/restore
- Fixed stack alignment issues
- Fixed SBI ecall clobber lists
- Resolved file sync issues between VS Code and WSL

---

## v0.1.0 - Memory Management (2025-10-13)
**Achievements:**
- ✅ Frame allocator (bitmap-based, 4KB pages)
- ✅ Kernel heap allocator (bump allocator, 64KB)
- ✅ Memory initialization on boot
- ✅ BSS section clearing
- ✅ Rust `alloc` crate support (Vec, Box, etc.)

---

## v0.0.1 - First Boot (2025-10-12)
**Achievements:**
- ✅ Bare metal boot on RISC-V
- ✅ UART driver (16550-compatible)
- ✅ Serial console output
- ✅ Basic panic handler
- ✅ Power-efficient idle loop (wfi)
- ✅ Linker script and memory layout
- ✅ QEMU virt machine support

---

**Architecture:** RISC-V 64-bit  
**Kernel Type:** Microkernel  
**Language:** Rust + Assembly  
**Platform:** QEMU virt machine (128MB RAM)
