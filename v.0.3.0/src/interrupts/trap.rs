use core::arch::{asm, naked_asm};
use crate::uart::Uart;
use crate::process::context::Context;
use crate::syscall;

static mut TICK_COUNT: u64 = 0;

/// Assembly trap entry point
/// 
/// This naked function is the first code that runs when ANY interrupt or exception occurs.
/// It saves ALL CPU state onto the stack, calls trap_handler, then restores state and returns.
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn trap_entry() {
    naked_asm!(
        // Save all registers to stack
        "addi sp, sp, -256",
        "sd ra, 0(sp)",
        "sd gp, 16(sp)",
        "sd tp, 24(sp)",
        "sd t0, 32(sp)",
        "sd t1, 40(sp)",
        "sd t2, 48(sp)",
        "sd s0, 56(sp)",
        "sd s1, 64(sp)",
        "sd a0, 72(sp)",
        "sd a1, 80(sp)",
        "sd a2, 88(sp)",
        "sd a3, 96(sp)",
        "sd a4, 104(sp)",
        "sd a5, 112(sp)",
        "sd a6, 120(sp)",
        "sd a7, 128(sp)",
        "sd s2, 136(sp)",
        "sd s3, 144(sp)",
        "sd s4, 152(sp)",
        "sd s5, 160(sp)",
        "sd s6, 168(sp)",
        "sd s7, 176(sp)",
        "sd s8, 184(sp)",
        "sd s9, 192(sp)",
        "sd s10, 200(sp)",
        "sd s11, 208(sp)",
        "sd t3, 216(sp)",
        "sd t4, 224(sp)",
        "sd t5, 232(sp)",
        "sd t6, 240(sp)",
        
        // Call Rust trap handler (sp = pointer to saved context)
        "mv a0, sp",        // Pass stack pointer as argument
        "call {handler}",
        
        // Restore all registers from stack
        "ld ra, 0(sp)",
        "ld gp, 16(sp)",
        "ld tp, 24(sp)",
        "ld t0, 32(sp)",
        "ld t1, 40(sp)",
        "ld t2, 48(sp)",
        "ld s0, 56(sp)",
        "ld s1, 64(sp)",
        "ld a0, 72(sp)",
        "ld a1, 80(sp)",
        "ld a2, 88(sp)",
        "ld a3, 96(sp)",
        "ld a4, 104(sp)",
        "ld a5, 112(sp)",
        "ld a6, 120(sp)",
        "ld a7, 128(sp)",
        "ld s2, 136(sp)",
        "ld s3, 144(sp)",
        "ld s4, 152(sp)",
        "ld s5, 160(sp)",
        "ld s6, 168(sp)",
        "ld s7, 176(sp)",
        "ld s8, 184(sp)",
        "ld s9, 192(sp)",
        "ld s10, 200(sp)",
        "ld s11, 208(sp)",
        "ld t3, 216(sp)",
        "ld t4, 224(sp)",
        "ld t5, 232(sp)",
        "ld t6, 240(sp)",
        
        "addi sp, sp, 256",
        "sret",             // Return from trap
        
        handler = sym trap_handler,
    );
}

/// Main trap handler - called from assembly trap_entry
/// 
/// This function examines scause to determine what caused the trap:
/// - Interrupt (bit 63 set): Timer, software, external
/// - Exception (bit 63 clear): Syscall, page fault, illegal instruction, etc.
#[no_mangle]
extern "C" fn trap_handler(stack_ptr: *mut usize) {
    let uart = Uart::new(0x1000_0000);
    
    unsafe {
        // Read trap cause
        let scause: usize;
        asm!("csrr {}, scause", out(reg) scause);
        
        let is_interrupt = (scause & (1 << 63)) != 0;
        let cause_code = scause & 0xFF;
        
        if is_interrupt {
            // INTERRUPT handling
            match cause_code {
                5 => handle_timer_interrupt(),
                _ => {
                    uart.puts("[TRAP] Unknown interrupt: ");
                    uart.print_hex(cause_code as u64);
                    uart.puts("\n");
                }
            }
        } else {
            // EXCEPTION handling
            match cause_code {
                8 => {
                    // Syscall from U-mode (Environment call from U-mode)
                    let ctx = stack_to_context(stack_ptr);
                    syscall::handle_syscall(ctx);
                    context_to_stack(ctx, stack_ptr);
                }
                9 => {
                    // Syscall from S-mode (shouldn't happen normally)
                    uart.puts("[TRAP] Unexpected ecall from S-mode!\n");
                }
                _ => {
                    // Other exception
                    uart.puts("[TRAP] Exception: ");
                    uart.print_hex(cause_code as u64);
                    uart.puts("\n");
                    
                    // Print sepc (where exception occurred)
                    let sepc: usize;
                    asm!("csrr {}, sepc", out(reg) sepc);
                    uart.puts("[TRAP] sepc: ");
                    uart.print_hex(sepc as u64);
                    uart.puts("\n");
                    
                    // Print stval (exception-specific value)
                    let stval: usize;
                    asm!("csrr {}, stval", out(reg) stval);
                    uart.puts("[TRAP] stval: ");
                    uart.print_hex(stval as u64);
                    uart.puts("\n");
                }
            }
        }
    }
}

/// Handle timer interrupt
fn handle_timer_interrupt() {
    let uart = Uart::new(0x1000_0000);
    
    unsafe {
        TICK_COUNT += 1;
        
        uart.puts("[TICK] ");
        uart.print_hex(TICK_COUNT);
        uart.puts("\n");
        
        // Schedule next timer interrupt
        let time: u64;
        asm!("rdtime {}", out(reg) time);
        let next = time + 10_000_000; // ~1Hz at 10MHz
        
        // SBI timer call
        asm!(
            "li a7, 0x54494D45",    // SBI timer extension
            "li a6, 0",             // SetTimer function
            "ecall",
            inout("a0") next => _,
            lateout("a1") _,
        );
    }
}

/// Convert stack pointer to Context structure
unsafe fn stack_to_context(stack_ptr: *mut usize) -> &'static mut Context {
    let ctx = &mut *(stack_ptr as *mut Context);
    
    // Also need to read sepc and sstatus
    asm!("csrr {}, sepc", out(reg) ctx.pc);
    asm!("csrr {}, sstatus", out(reg) ctx.sstatus);
    
    ctx
}

/// Write Context structure back to stack
unsafe fn context_to_stack(ctx: &Context, stack_ptr: *mut usize) {
    let stack_ctx = &mut *(stack_ptr as *mut Context);
    *stack_ctx = *ctx;
    
    // Also write back sepc (might have changed for syscalls)
    asm!("csrw sepc, {}", in(reg) ctx.pc);
}

/// Initialize trap handling
pub fn init() {
    unsafe {
        // Install trap handler
        let trap_addr = trap_entry as usize;
        asm!("csrw stvec, {}", in(reg) trap_addr);
        
        let uart = Uart::new(0x1000_0000);
        uart.puts("[OK] Trap handler installed at ");
        uart.print_hex(trap_addr as u64);
        uart.puts("\n");
        
        // Enable supervisor timer interrupts
        asm!("csrs sie, {}", in(reg) 0x20usize);
        uart.puts("[OK] Timer interrupts enabled (SIE.STIE = 1)\n");
        
        // Schedule first timer interrupt
        let time: u64;
        asm!("rdtime {}", out(reg) time);
        let next = time + 10_000_000;
        
        asm!(
            "li a7, 0x54494D45",
            "li a6, 0",
            "ecall",
            inout("a0") next => _,
            lateout("a1") _,
        );
        
        // Enable interrupts globally
        asm!("csrsi sstatus, 0x2");
        uart.puts("[OK] Global interrupts enabled (sstatus.SIE = 1)\n");
    }
}