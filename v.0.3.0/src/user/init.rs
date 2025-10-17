/// First userspace program for woflOS - PROPER IMPLEMENTATION
/// 
/// This contains a hand-crafted RISC-V assembly program that:
/// 1. Runs in U-mode (user mode) with NO privileged instructions
/// 2. Makes syscalls via ecall
/// 3. Prints "Hello from userspace!\n"
/// 4. Gets and prints its PID
/// 5. Exits cleanly

use crate::uart::Uart;

/// Pure assembly user program encoded as RISC-V machine code
/// 
/// Assembly source:
/// ```asm
/// .section .text
/// .global _start
/// _start:
///     # Print "Hello from userspace!\n"
///     la a1, message          # Load message address
///     li a2, 23               # Message length
/// 
/// print_loop:
///     beqz a2, get_pid        # If length == 0, get PID
///     lb a0, 0(a1)            # Load byte
///     li a7, 1                # SYS_PUTC
///     ecall
///     addi a1, a1, 1          # Next char
///     addi a2, a2, -1         # Decrement length
///     j print_loop
/// 
/// get_pid:
///     li a7, 3                # SYS_GETPID
///     ecall                   # Returns PID in a0
///     mv s0, a0               # Save PID
///     
///     # Print "PID: 0x"
///     li a0, 'P'
///     li a7, 1
///     ecall
///     li a0, 'I'
///     li a7, 1
///     ecall
///     li a0, 'D'
///     li a7, 1
///     ecall
///     li a0, ':'
///     li a7, 1
///     ecall
///     li a0, ' '
///     li a7, 1
///     ecall
///     li a0, '0'
///     li a7, 1
///     ecall
///     li a0, 'x'
///     li a7, 1
///     ecall
///     
///     # Print PID as hex (16 nibbles)
///     li t0, 15               # Counter (15 down to 0)
/// hex_loop:
///     slli t1, t0, 2          # t1 = counter * 4
///     srl t2, s0, t1          # Shift PID right by (counter*4) bits
///     andi t2, t2, 0xF        # Mask to get nibble
///     li t3, 10
///     blt t2, t3, hex_digit   # If < 10, it's 0-9
///     addi a0, t2, 87         # 'a'-10 = 87, so 10->a, 11->b, etc.
///     j hex_print
/// hex_digit:
///     addi a0, t2, 48         # '0' = 48
/// hex_print:
///     li a7, 1                # SYS_PUTC
///     ecall
///     addi t0, t0, -1         # Decrement counter
///     bgez t0, hex_loop       # Loop if counter >= 0
///     
///     # Print newline
///     li a0, 10               # '\n'
///     li a7, 1
///     ecall
///     
/// exit:
///     li a0, 0                # Exit code 0
///     li a7, 2                # SYS_EXIT
///     ecall
///     
/// loop_forever:
///     j loop_forever          # Should never reach here
/// 
/// message:
///     .ascii "Hello from userspace!\n"
/// ```
static USER_PROGRAM: &[u32] = &[
    // _start: Print "Hello from userspace!\n"
    0x00000597,  // auipc a1, 0          # Get PC
    0x0d058593,  // addi a1, a1, 208     # a1 = &message (adjust offset)
    0x01700613,  // li a2, 23            # Length of message
    
    // print_loop:
    0x06060663,  // beqz a2, get_pid     # If done, jump to get_pid
    0x0005c503,  // lbu a0, 0(a1)        # Load byte from message
    0x00100893,  // li a7, 1             # SYS_PUTC
    0x00000073,  // ecall
    0x00158593,  // addi a1, a1, 1       # Next char
    0xfff60613,  // addi a2, a2, -1      # Decrement length
    0xfe9ff06f,  // j print_loop
    
    // get_pid:
    0x00300893,  // li a7, 3             # SYS_GETPID
    0x00000073,  // ecall                # Returns PID in a0
    0x00050413,  // mv s0, a0            # Save PID in s0
    
    // Print "PID: 0x"
    0x05000513,  // li a0, 'P'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0x04900513,  // li a0, 'I'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0x04400513,  // li a0, 'D'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0x03a00513,  // li a0, ':'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0x02000513,  // li a0, ' '
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0x03000513,  // li a0, '0'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0x07800513,  // li a0, 'x'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    
    // Print PID as hex
    0x00f00293,  // li t0, 15            # Counter
    
    // hex_loop:
    0x00229313,  // slli t1, t0, 2       # t1 = counter * 4
    0x006453b3,  // srl t2, s0, t1       # Shift PID right
    0x00f3f393,  // andi t2, t2, 0xF     # Mask nibble
    0x00a00e13,  // li t3, 10
    0x01c3c463,  // blt t2, t3, hex_digit
    0x05738513,  // addi a0, t2, 87      # Convert to 'a'-'f'
    0x0080006f,  // j hex_print
    // hex_digit:
    0x03038513,  // addi a0, t2, 48      # Convert to '0'-'9'
    // hex_print:
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    0xfff28293,  // addi t0, t0, -1
    0xfc02dce3,  // bgez t0, hex_loop
    
    // Print newline
    0x00a00513,  // li a0, 10            # '\n'
    0x00100893,  // li a7, 1
    0x00000073,  // ecall
    
    // exit:
    0x00000513,  // li a0, 0             # Exit code 0
    0x00200893,  // li a7, 2             # SYS_EXIT
    0x00000073,  // ecall
    
    // loop_forever: (should never reach)
    0x0000006f,  // j loop_forever
    
    // message: "Hello from userspace!\n" (encoded as u32s)
    0x6c6c6548,  // "Hell"
    0x7266206f,  // "o fr"
    0x75206d6f,  // "om u"
    0x73726573,  // "sers"
    0x65636170,  // "pace"
    0x000a2121,  // "!!\n\0"
];

/// Launch the first userspace process
/// 
/// This function:
/// 1. Allocates memory for user code and stack
/// 2. Copies the pure assembly program to user memory
/// 3. Creates a process structure
/// 4. Switches to U-mode and starts executing
pub unsafe fn launch_init_process() {
    let uart = Uart::new(0x1000_0000);
    uart.puts("\n");
    uart.puts("═════════════════════════════════════════\n");
    uart.puts("   LAUNCHING FIRST USERSPACE PROCESS\n");
    uart.puts("═════════════════════════════════════════\n");
    uart.puts("\n");
    
    uart.puts("[DEBUG] Step 1: Setting up memory addresses\n");
    
    // User memory layout
    let user_base = 0x8700_0000;        // User code starts here
    let user_stack_top = 0x8701_0000;   // 64KB stack
    
    uart.puts("[PROCESS] User memory base: ");
    uart.print_hex(user_base as u64);
    uart.puts("\n");
    
    uart.puts("[PROCESS] User stack top:   ");
    uart.print_hex(user_stack_top as u64);
    uart.puts("\n");
    
    uart.puts("[DEBUG] Step 2: Getting program size\n");
    
    // Copy user program to user memory
    let prog_size = USER_PROGRAM.len() * 4; // Size in bytes
    uart.puts("[PROCESS] Program size:     ");
    uart.print_hex(prog_size as u64);
    uart.puts(" bytes\n");
    
    uart.puts("[DEBUG] Step 3: Copying program to user memory\n");
    
    core::ptr::copy_nonoverlapping(
        USER_PROGRAM.as_ptr() as *const u8,
        user_base as *mut u8,
        prog_size,
    );
    
    uart.puts("[PROCESS] Program copied successfully\n");
    
    uart.puts("[DEBUG] Step 4: Creating process structure\n");
    
    // Create process structure
    use crate::process::{Process, alloc_pid, set_current_process};
    
    let pid = alloc_pid();
    uart.puts("[PROCESS] Allocated PID:    ");
    uart.print_hex(pid as u64);
    uart.puts("\n");
    
    uart.puts("[DEBUG] Step 5: Calling Process::new()\n");
    
    let process = Process::new(pid, "init", user_base, user_stack_top);
    
    uart.puts("[DEBUG] Step 6: Setting current process\n");
    
    set_current_process(process);
    
    uart.puts("[PROCESS] Process structure created\n");
    uart.puts("\n");
    
    uart.puts("[DEBUG] Step 7: About to switch to U-mode\n");
    uart.puts("═════════════════════════════════════════\n");
    uart.puts("   SWITCHING TO USER MODE (U-MODE)\n");
    uart.puts("   All output below is from userspace!\n");
    uart.puts("═════════════════════════════════════════\n");
    uart.puts("\n");
    
    uart.puts("[DEBUG] Step 8: Executing sret instruction...\n");
    
    // Switch to user mode!
    use core::arch::asm;
    
    asm!(
        // Set sepc (where we'll jump to)
        "csrw sepc, {entry}",
        
        // Set sstatus for U-mode
        "csrr t0, sstatus",
        "li t1, 0x100",         // SPP bit mask
        "not t1, t1",
        "and t0, t0, t1",       // Clear SPP (= U-mode)
        "ori t0, t0, 0x20",     // Set SPIE (enable interrupts)
        "csrw sstatus, t0",
        
        // Set up user stack
        "mv sp, {stack}",
        
        // Clear ALL registers for security
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