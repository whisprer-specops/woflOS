// src/user_test.rs - First user mode program

use core::arch::asm;
use crate::syscall::*;

/// A simple user mode function that proves context switching works
#[no_mangle]
pub extern "C" fn user_main() -> ! {
    // We're now in user mode! Let's do something observable.
    
    // Do 3 simple operations to prove we're executing
    let mut counter = 0usize;
    counter += 1; // Operation 1
    counter += 1; // Operation 2
    counter += 1; // Operation 3
    
    // The counter should now equal 3
    // We'll pass it back to kernel via syscall
    
    // Make a test syscall to prove we can trap back to kernel
    // Syscall convention: number in a7, args in a0-a5, result in a0
    let result: usize;
    unsafe {
        asm!(
            "li a7, {syscall}",  // Load syscall number
            "ecall",              // Trap to kernel
            "mv {result}, a0",    // Get return value
            syscall = const SYS_TEST,
            result = out(reg) result,
            out("a7") _,
        );
    }
    
    // If we get here, syscall succeeded and returned a value
    // The test syscall should return 42
    
    // Now exit with the counter value as exit code
    unsafe {
        asm!(
            "li a7, {syscall}",   // SYS_EXIT
            "mv a0, {code}",       // Exit code = counter value
            "ecall",
            syscall = const SYS_EXIT,
            code = in(reg) counter,
            options(noreturn)
        );
    }
}

/// Allocate a user stack (just a static buffer for now)
/// Layer 2 will allocate this properly with PMP protection
#[no_mangle]
static mut USER_STACK: [u8; 4096] = [0; 4096];

pub fn get_user_stack_top() -> usize {
    unsafe {
        USER_STACK.as_ptr() as usize + USER_STACK.len()
    }
}