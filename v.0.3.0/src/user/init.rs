/// First userspace program for woflOS
/// 
/// This is a simple test program that demonstrates:
/// 1. Running in U-mode (user mode)
/// 2. Making syscalls via ecall
/// 3. Process isolation

use crate::uart::Uart;
use core::arch::asm;

/// User program code as raw bytes
/// 
/// This is a hand-written RISC-V program that:
/// 1. Prints "Hello from userspace!\n" using SYS_PUTC
/// 2. Exits with code 0
/// 
/// Assembly source:
/// ```asm
/// .global _user_start
/// _user_start:
///     # Load message address
///     la a1, message
///     li a2, message_end - message  # length
/// 
/// loop:
///     beqz a2, exit           # if length == 0, exit
///     lb a0, 0(a1)            # load byte from message
///     li a7, 1                # SYS_PUTC
///     ecall                   # syscall
///     addi a1, a1, 1          # next char
///     addi a2, a2, -1         # length--
///     j loop
/// 
/// exit:
///     li a0, 0                # exit code 0
///     li a7, 2                # SYS_EXIT
///     ecall
/// 
/// message:
///     .ascii "Hello from userspace!\n"
/// message_end:
/// ```
#[allow(dead_code)]
static USER_PROGRAM: &[u8] = &[
    // This will be filled in with actual machine code
    // For now, we'll generate it dynamically
];

/// Generate user program dynamically
/// 
/// This creates a simple program that makes syscalls.
/// We generate it at runtime because encoding raw RISC-V is tedious.
#[allow(dead_code)]
pub fn generate_user_program() -> &'static [u32] {
    static mut PROGRAM: [u32; 128] = [0; 128];
    
    unsafe {
        // For simplicity, we'll write a Rust function that uses inline asm
        // and copy its code. But that's complex...
        // 
        // Instead, let's just return a pointer to a Rust function!
        // We'll compile it with the kernel but it only uses syscalls.
        
        &PROGRAM[0..0]
    }
}

/// User program entry point (written in Rust!)
/// 
/// This function will be copied to user memory and executed in U-mode.
/// It can ONLY interact with the kernel via syscalls.
#[no_mangle]
pub extern "C" fn user_main() -> ! {
    // Print message using syscalls
    let message = b"Hello from userspace!\n";
    
    for &ch in message {
        unsafe {
            // SYS_PUTC syscall
            asm!(
                "li a7, 1",     // SYS_PUTC
                "ecall",
                in("a0") ch as usize,
            );
        }
    }
    
    // Get our PID
    let pid: usize;
    unsafe {
        asm!(
            "li a7, 3",         // SYS_GETPID
            "ecall",
            lateout("a0") pid,
        );
    }
    
    // Print it (in hex, no division!)
    let msg = b"My PID: 0x";
    for &ch in msg {
        unsafe {
            asm!(
                "li a7, 1",
                "ecall",
                in("a0") ch as usize,
            );
        }
    }
    
    // Print PID as hex
    for i in (0..16).rev() {
        let nibble = ((pid >> (i * 4)) & 0xF) as u8;
        let ch = if nibble < 10 { b'0' + nibble } else { b'a' + (nibble - 10) };
        unsafe {
            asm!(
                "li a7, 1",
                "ecall",
                in("a0") ch as usize,
            );
        }
    }
    
    // Newline
    unsafe {
        asm!(
            "li a7, 1",
            "ecall",
            in("a0") b'\n' as usize,
        );
    }
    
    // Exit
    unsafe {
        asm!(
            "li a7, 2",         // SYS_EXIT
            "li a0, 0",         // exit code 0
            "ecall",
        );
    }
    
    // Should never reach here
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

/// Launch the first userspace process
/// 
/// This function:
/// 1. Allocates memory for the user program and stack
/// 2. Copies user_main code to user memory
/// 3. Creates a process structure
/// 4. Switches to U-mode and starts executing
pub unsafe fn launch_init_process() {
    let uart = Uart::new(0x1000_0000);
    uart.puts("\n[PROCESS] Launching init process (first userspace program)...\n");
    
    // Get the address and size of user_main function
    let user_code_start = user_main as usize;
    let user_code_size = 4096; // Assume < 4KB for now
    
    uart.puts("[PROCESS] User code at: ");
    uart.print_hex(user_code_start as u64);
    uart.puts("\n");
    
    // Allocate memory for user process
    // In a real OS, we'd use the frame allocator and set up page tables
    // For now, just use a fixed address in high memory
    let user_base = 0x8700_0000; // User memory region
    let user_stack_top = 0x8701_0000; // 64KB stack
    
    uart.puts("[PROCESS] User memory at: ");
    uart.print_hex(user_base as u64);
    uart.puts("\n");
    uart.puts("[PROCESS] User stack at: ");
    uart.print_hex(user_stack_top as u64);
    uart.puts("\n");
    
    // Copy user code to user memory
    uart.puts("[PROCESS] Copying user code...\n");
    core::ptr::copy_nonoverlapping(
        user_code_start as *const u8,
        user_base as *mut u8,
        user_code_size,
    );
    uart.puts("[PROCESS] Code copied!\n");
    
    // Create process structure
    use crate::process::{Process, alloc_pid, set_current_process};
    
    let pid = alloc_pid();
    uart.puts("[PROCESS] Allocated PID: ");
    uart.print_hex(pid as u64);
    uart.puts("\n");
    
    let process = Process::new(pid, "init", user_base, user_stack_top);
    set_current_process(process);
    
    uart.puts("[PROCESS] Process created!\n");
    uart.puts("[PROCESS] Switching to user mode...\n");
    uart.puts("\n");
    uart.puts("═════════════════════════════════════════\n");
    uart.puts("  Entering userspace! (U-mode)\n");
    uart.puts("═════════════════════════════════════════\n");
    uart.puts("\n");
    
    // Switch to user mode!
    // We need to:
    // 1. Set sepc to user_base (where to return to)
    // 2. Set sstatus.SPP = 0 (return to U-mode)
    // 3. Set sstatus.SPIE = 1 (enable interrupts in U-mode)
    // 4. Set sp to user_stack_top
    // 5. Execute sret
    
    asm!(
        // Set up sepc (where we'll jump to on sret)
        "csrw sepc, {entry}",
        
        // Set up sstatus
        "csrr t0, sstatus",
        "li t1, 0x100",         // SPP bit (bit 8)
        "not t1, t1",           // Invert mask
        "and t0, t0, t1",       // Clear SPP (= U-mode)
        "ori t0, t0, 0x20",     // Set SPIE (enable interrupts)
        "csrw sstatus, t0",
        
        // Set up user stack
        "mv sp, {stack}",
        
        // Clear all registers (security: don't leak kernel data)
        "li ra, 0",
        "li gp, 0",
        "li tp, 0",
        "li t0, 0",
        "li t1, 0",
        "li t2, 0",
        "li s0, 0",
        "li s1, 0",
        "li a0, 0",
        "li a1, 0",
        "li a2, 0",
        "li a3, 0",
        "li a4, 0",
        "li a5, 0",
        "li a6, 0",
        "li a7, 0",
        "li s2, 0",
        "li s3, 0",
        "li s4, 0",
        "li s5, 0",
        "li s6, 0",
        "li s7, 0",
        "li s8, 0",
        "li s9, 0",
        "li s10, 0",
        "li s11, 0",
        "li t3, 0",
        "li t4, 0",
        "li t5, 0",
        "li t6, 0",
        
        // Jump to userspace!
        "sret",
        
        entry = in(reg) user_base,
        stack = in(reg) user_stack_top,
        options(noreturn)
    );
}