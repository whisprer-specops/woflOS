#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::asm;
use core::panic::PanicInfo;

mod uart;
mod memory;
mod interrupts;

use uart::Uart;

// Boot code - this runs first!
#[link_section = ".text.boot"]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Get UART working FIRST for debugging
    let uart = Uart::new(0x1000_0000);
    uart.puts("[DEBUG] _start() entered\n");
    
    // Clear BSS section (uninitialized data)
    extern "C" {
        static mut __bss_start: u8;
        static mut __bss_end: u8;
        static __kernel_start: u8;
        static __kernel_end: u8;
    }
    
    uart.puts("[DEBUG] Clearing BSS...\n");
    let bss_start = &raw mut __bss_start as *mut u8;
    let bss_end = &raw mut __bss_end as *mut u8;
    let bss_len = bss_end as usize - bss_start as usize;
    
    core::ptr::write_bytes(bss_start, 0, bss_len);
    uart.puts("[DEBUG] BSS cleared\n");
    
    // Initialize memory system
    uart.puts("[DEBUG] Initializing memory...\n");
    let kernel_end = &raw const __kernel_end as *const u8 as usize;
    let memory_end = 0x8800_0000; // 128MB total RAM
    
    memory::init(kernel_end, memory_end);
    uart.puts("[DEBUG] Memory initialized\n");
    
    // Jump to kernel main
    uart.puts("[DEBUG] Jumping to kernel_main\n");
    kernel_main();
}

fn kernel_main() -> ! {
    let uart = Uart::new(0x1000_0000);
    
    uart.puts("\n");
    uart.puts("============================================\n");
    uart.puts(" __      __ ___  ___  _     ___   ___ \n");
    uart.puts(" \\ \\    / // _ \\| __|| |   / _ \\ / __|\n");
    uart.puts("  \\ \\/\\/ /| (_) | _| | |__| (_) |\\__ \\\n");
    uart.puts("   \\_/\\_/  \\___/|_|  |____|\\___/ |___/\n");
    uart.puts("                                        \n");
    uart.puts("   Rust + RISC-V + Microkernel = Stability\n");
    uart.puts("============================================\n");
    uart.puts("\n");
    uart.puts("[OK] woflOS v0.4.0 booting...\n");
    uart.puts("[OK] UART initialized\n");
    uart.puts("[OK] BSS cleared\n");
    uart.puts("[OK] Memory manager initialized\n");
    uart.puts("[OK] Kernel main entered\n");
    uart.puts("\n");
    
    // NEW: Initialize interrupts!
    interrupts::trap::init();
    uart.puts("\n");
    
    uart.puts("[DEBUG] Interrupts initialized, about to enter idle loop\n");
    uart.puts("[DEBUG] Current time: ");
    unsafe {
        let time: u64;
        core::arch::asm!("rdtime {}", out(reg) time);
        // Can't print numbers easily, but at least we're here
        uart.puts("...\n");
    }
    uart.puts("[DEBUG] Entering wfi loop now!\n");
    
    // Test memory allocation
    uart.puts("[TEST] Testing heap allocator directly...\n");
    
    use core::alloc::Layout;
    unsafe {
        let layout = Layout::from_size_align_unchecked(32, 8);
        uart.puts("[DEBUG] Trying to allocate 32 bytes...\n");
        
        let ptr = alloc::alloc::alloc(layout);
        if ptr.is_null() {
            uart.puts("[ERROR] Direct allocation returned null!\n");
        } else {
            uart.puts("[OK] Direct allocation succeeded!\n");
            *ptr = 0x42;
            uart.puts("[OK] Successfully wrote to allocated memory!\n");
            alloc::alloc::dealloc(ptr, layout);
        }
    }
    
    uart.puts("\n");
    uart.puts("woflOS is alive, fren! 🐺\n");
    uart.puts("Memory management: OPERATIONAL ✓\n");
    uart.puts("Interrupts: ENABLED ✓\n");
    uart.puts("\n");
    uart.puts("[INFO] Entering idle loop. Timer ticks should appear!\n");
    uart.puts("\n");
    
    // Idle loop - interrupts will fire!
    loop {
        // Just spin - don't use wfi, it might cause exceptions
        for _ in 0..1000 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let uart = Uart::new(0x1000_0000);
    uart.puts("\n[PANIC] Kernel panic!\n");
    
    if let Some(location) = info.location() {
        uart.puts("Location: ");
        uart.puts(location.file());
        uart.puts(":");
        uart.puts("\n");
    }
    
    uart.puts("[PANIC] System halted.\n");
    
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    let uart = Uart::new(0x1000_0000);
    uart.puts("\n[PANIC] Allocation error!\n");
    uart.puts("Failed to allocate memory\n");
    
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}