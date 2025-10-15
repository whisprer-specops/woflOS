#![no_std]
#![no_main]
#![feature(alloc_error_handler)] 

extern crate alloc;

use core::arch::asm;
use core::panic::PanicInfo;

mod uart;
mod memory;
mod interrupts;
mod process;
mod syscall;
mod user;

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
    uart.puts("   Microkernel + Capabilities + Crypto\n");
    uart.puts("============================================\n");
    uart.puts("\n");
    uart.puts("[OK] woflOS v0.4.0 - Layer 1 (Privilege Transitions)\n");
    uart.puts("[OK] UART initialized\n");
    uart.puts("[OK] BSS cleared\n");
    uart.puts("[OK] Memory manager initialized\n");
    uart.puts("\n");
    
    // Initialize interrupts (Layer 0)
    interrupts::trap::init();
    uart.puts("\n");
    
    // Initialize process subsystem (Layer 1)
    uart.puts("[OK] Initializing process subsystem...\n");
    process::init();
    uart.puts("[OK] Process subsystem ready\n");
    uart.puts("\n");
    
    // Test memory allocation
    uart.puts("[TEST] Testing heap allocator...\n");
    use core::alloc::Layout;
    unsafe {
        let layout = Layout::from_size_align_unchecked(32, 8);
        let ptr = alloc::alloc::alloc(layout);
        if ptr.is_null() {
            uart.puts("[ERROR] Allocation failed!\n");
        } else {
            uart.puts("[OK] Heap allocator working!\n");
            *ptr = 0x42;
            alloc::alloc::dealloc(ptr, layout);
        }
    }
    uart.puts("\n");
    
    uart.puts("═══════════════════════════════════════════\n");
    uart.puts("   Layer 0: Complete ✓\n");
    uart.puts("     - Memory management\n");
    uart.puts("     - Timer interrupts\n");
    uart.puts("     - Exception handling\n");
    uart.puts("\n");
    uart.puts("   Layer 1: Activating...\n");
    uart.puts("     - Process structure\n");
    uart.puts("     - Syscall interface\n");
    uart.puts("     - User mode transition\n");
    uart.puts("═══════════════════════════════════════════\n");
    uart.puts("\n");
    
    // Launch first userspace process!
    unsafe {
        user::launch_init_process();
    }
    
    // Should never reach here (we jumped to userspace)
    uart.puts("[ERROR] Returned from userspace?!\n");
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let uart = Uart::new(0x1000_0000);
    uart.puts("\n");
    uart.puts("╔═══════════════════════════════════════╗\n");
    uart.puts("║        KERNEL PANIC! ðŸº               ║\n");
    uart.puts("╚═══════════════════════════════════════╝\n");
    uart.puts("\n");
    
    if let Some(location) = info.location() {
        uart.puts("Location: ");
        uart.puts(location.file());
        uart.puts(":");
        // Can't print line number easily
        uart.puts("\n");
    }
    
    uart.puts("Message: ");
    uart.puts(info.message().as_str().unwrap_or("<no message>"));
    uart.puts("\n");
    
    uart.puts("\n[PANIC] System halted.\n");
    
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