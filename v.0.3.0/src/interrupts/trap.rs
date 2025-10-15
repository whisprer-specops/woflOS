use core::arch::{asm, naked_asm};
use crate::uart::Uart;

static mut TICK_COUNT: u64 = 0;

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn trap_entry() {
    naked_asm!(
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
        
        "call {handler}",
        
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
        "sret",
        
        handler = sym trap_handler,
    );
}

#[no_mangle]
extern "C" fn trap_handler() {
    let uart = Uart::new(0x1000_0000);
    
    unsafe {
        let scause: usize;
        asm!("csrr {}, scause", out(reg) scause);
        
        if (scause & (1 << 63)) != 0 && (scause & 0xFF) == 5 {
            TICK_COUNT += 1;
            
            uart.puts("[TICK] ");
            uart.print_hex(TICK_COUNT);
            uart.puts("\n");
            
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
        }
    }
}

pub fn init() {
    unsafe {
        let trap_addr = trap_entry as usize;
        asm!("csrw stvec, {}", in(reg) trap_addr);
        
        let uart = Uart::new(0x1000_0000);
        uart.puts("[OK] Timer interrupt handler installed\n");
        
        asm!("csrs sie, {}", in(reg) 0x20usize);
        
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
        
        asm!("csrsi sstatus, 0x2");
        uart.puts("[OK] Interrupts enabled!\n");
    }
}
