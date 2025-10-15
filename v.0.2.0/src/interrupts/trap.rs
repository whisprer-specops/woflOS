// src/interrupts/trap.rs - KISS: Keep It Simple!
use core::arch::{asm, naked_asm};
use crate::uart::Uart;

static mut TICK_COUNT: u64 = 0;

fn uart_print(msg: &str) {
    let uart = Uart::new(0x1000_0000);
    uart.puts(msg);
}

pub fn init() {
    unsafe {
        let trap_addr = trap_entry as usize;
        asm!("csrw stvec, {}", in(reg) trap_addr);
        
        uart_print("[OK] Trap handler installed\n");
        
        // Enable timer interrupts
        let sie_val: usize = 0x20;
        asm!("csrs sie, {}", in(reg) sie_val);
        
        // Set first timer
        set_timer(100_000);
        
        // Enable interrupts globally
        asm!("csrsi sstatus, 0x2");
        
        uart_print("[OK] Interrupts enabled\n");
    }
}

// The trap entry - save everything, call handler, restore everything
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn trap_entry() {
    naked_asm!(
        // Make room on stack - LOTS of room for Rust function
        // Align sp to 16 bytes FIRST
        "andi sp, sp, -16",     // Mask off bottom 4 bits
        "addi sp, sp, -512",    // Now allocate
        "addi sp, sp, -512",  // 512 bytes should be plenty!
        
        // Save ALL callee and caller-saved registers
        "sd ra, 0(sp)",      // x1
        "sd sp, 8(sp)",      // x2 (save sp itself for debugging)
        "sd gp, 16(sp)",     // x3
        "sd tp, 24(sp)",     // x4
        "sd t0, 32(sp)",     // x5
        "sd t1, 40(sp)",     // x6
        "sd t2, 48(sp)",     // x7
        "sd s0, 56(sp)",     // x8/fp
        "sd s1, 64(sp)",     // x9
        "sd a0, 72(sp)",     // x10
        "sd a1, 80(sp)",     // x11
        "sd a2, 88(sp)",     // x12
        "sd a3, 96(sp)",     // x13
        "sd a4, 104(sp)",    // x14
        "sd a5, 112(sp)",    // x15
        "sd a6, 120(sp)",    // x16
        "sd a7, 128(sp)",    // x17
        "sd s2, 136(sp)",    // x18
        "sd s3, 144(sp)",    // x19
        "sd s4, 152(sp)",    // x20
        "sd s5, 160(sp)",    // x21
        "sd s6, 168(sp)",    // x22
        "sd s7, 176(sp)",    // x23
        "sd s8, 184(sp)",    // x24
        "sd s9, 192(sp)",    // x25
        "sd s10, 200(sp)",   // x26
        "sd s11, 208(sp)",   // x27
        "sd t3, 216(sp)",    // x28
        "sd t4, 224(sp)",    // x29
        "sd t5, 232(sp)",    // x30
        "sd t6, 240(sp)",    // x31
        
        // Call handler (sepc/sstatus are CSRs, handler reads them directly)
        "call {trap_handler}",
        
        // Restore everything
        "ld ra, 0(sp)",
        // Skip sp restore for now
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
        
        // Restore sp
        "addi sp, sp, 512",  // Match the allocation!
        
        "sret",
        trap_handler = sym trap_handler
    );
}

#[no_mangle]
extern "C" fn trap_handler() {
    // FIRST thing - print that we got here!
    let uart = Uart::new(0x1000_0000);
    uart.puts("[TRAP!]\n");
    
    let scause: usize;
    unsafe {
        asm!("csrr {}, scause", out(reg) scause);
    }
    
    if scause & (1 << 63) != 0 {
        uart.puts("[INT]\n");
        let cause = scause & 0x7FFF_FFFF_FFFF_FFFF;
        if cause == 5 {
            handle_timer_interrupt();
        }
} else {
    uart.puts("[EXC]\n");

    let sepc: usize;
    let scause: usize;
    let stval: usize;
    let sstatus: usize;
    
    unsafe {
        asm!("csrr {}, scause", out(reg) scause);
        asm!("csrr {}, sepc", out(reg) sepc);
        asm!("csrr {}, stval", out(reg) stval);
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }
    
        uart.puts("cause:   "); print_hex(scause); uart.puts("\n");
        uart.puts("sepc:    "); print_hex(sepc); uart.puts("\n");
        uart.puts("stval:   "); print_hex(stval); uart.puts("\n");
        uart.puts("sstatus: "); print_hex(sstatus); uart.puts("\n");
    
        loop {}
    }
}

fn handle_timer_interrupt() {
    unsafe {
        TICK_COUNT += 1;
        
        if TICK_COUNT % 100 == 0 {
            uart_print("[TICK] ...\n");
        }
        
        set_timer(100_000);
    }
}

fn read_time() -> u64 {
    let time: u64;
    unsafe {
        asm!("rdtime {}", out(reg) time);
    }
    time
}

fn set_timer(cycles_from_now: u64) {
    let current_time = read_time();
    let next_time = current_time + cycles_from_now;
    
    unsafe {
        sbi_set_timer(next_time);
    }
}

unsafe fn sbi_set_timer(stime_value: u64) {
    asm!(
        "mv a0, {0}",
        "li a6, 0",
        "li a7, 0x54494D45",
        "ecall",
        in(reg) stime_value,
        out("a0") _,
        out("a6") _,
        out("a7") _,
        // Add these! OpenSBI can clobber temporaries:
        out("a1") _,
        out("t0") _,
        out("t1") _,
        out("t2") _,
        out("t3") _,
        out("t4") _,
        out("t5") _,
        out("t6") _,
    );
}

fn print_hex(n: usize) {
    let uart = Uart::new(0x1000_0000);
    let hex_chars = b"0123456789abcdef";
    
    let sstatus: usize;
    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }
    uart.puts("sstatus: ");
    print_hex(sstatus);

    for i in (0..16).rev() {
        let digit = ((n >> (i * 4)) & 0xF) as usize;
        uart.putc(hex_chars[digit]);
    }
}