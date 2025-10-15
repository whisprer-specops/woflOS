/// Syscall numbers for woflOS
/// 
/// These are the interfaces between user processes and the kernel.
/// User processes use the `ecall` instruction with these numbers in a7.

use crate::uart::Uart;
use crate::process::context::Context;

/// Syscall numbers
pub const SYS_PUTC: usize = 1;      // Write a single character
pub const SYS_EXIT: usize = 2;      // Exit process
pub const SYS_GETPID: usize = 3;    // Get process ID
pub const SYS_YIELD: usize = 4;     // Yield CPU to scheduler

/// Handle a syscall from user mode
/// 
/// This is called by the trap handler when it detects an ecall from U-mode.
/// The context contains the syscall number (in a7) and arguments (in a0-a5).
/// 
/// # Arguments
/// * `ctx` - Mutable reference to the process context
/// 
/// # Returns
/// The return value is placed in ctx.a0
pub fn handle_syscall(ctx: &mut Context) {
    let syscall_num = ctx.syscall_number();
    let args = ctx.syscall_args();
    
    let result = match syscall_num {
        SYS_PUTC => sys_putc(args[0]),
        SYS_EXIT => sys_exit(args[0]),
        SYS_GETPID => sys_getpid(),
        SYS_YIELD => sys_yield(),
        _ => {
            // Unknown syscall
            let uart = Uart::new(0x1000_0000);
            uart.puts("[SYSCALL] Unknown syscall: ");
            uart.print_hex(syscall_num as u64);
            uart.puts("\n");
            usize::MAX // Return error
        }
    };
    
    // Set return value in context
    ctx.set_return_value(result);
    
    // Advance PC past the ecall instruction (4 bytes)
    ctx.pc += 4;
}

/// Syscall: Write a single character to console
/// 
/// Arguments:
/// - a0: character to write (as usize, only low byte used)
/// 
/// Returns: 0 on success
fn sys_putc(ch: usize) -> usize {
    let uart = Uart::new(0x1000_0000);
    uart.putc(ch as u8);
    0
}

/// Syscall: Exit the current process
/// 
/// Arguments:
/// - a0: exit code
/// 
/// Returns: never (but we return 0 for now since we can't actually kill processes yet)
fn sys_exit(code: usize) -> usize {
    let uart = Uart::new(0x1000_0000);
    uart.puts("[SYSCALL] Process exit with code ");
    uart.print_hex(code as u64);
    uart.puts("\n");
    
    // TODO: Actually kill the process and return to kernel
    // For now, just halt
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

/// Syscall: Get process ID
/// 
/// Returns: current process ID
fn sys_getpid() -> usize {
    use crate::process;
    
    if let Some(proc) = process::current_process() {
        proc.pid
    } else {
        0 // No current process?
    }
}

/// Syscall: Yield CPU to scheduler
/// 
/// Returns: 0 on success
fn sys_yield() -> usize {
    // For now, just return - we don't have a real scheduler yet
    0
}