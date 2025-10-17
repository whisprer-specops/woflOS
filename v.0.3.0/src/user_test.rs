// src/user_test.rs - First user mode program

use core::arch::asm;

/// A simple user mode function that proves context switching works
#[no_mangle]
pub extern "C" fn user_main() -> ! {
    // We're now in user mode! Let's do something observable.
    
    // Do 3 simple operations
    let mut counter = 0usize;
    counter += 1; // Operation 1
    counter += 1; // Operation 2
    counter += 1; // Operation 3
    
    // Now make a syscall to return to kernel
    // Syscall convention: number in a7, args in a0-a5, result in a0
    unsafe {
        asm!(
            "li a7, 0",    // SYS_TEST syscall number
            "ecall",        // Trap to kernel
            out("a7") _,
        );
    }
    
    // If we get here, the syscall returned successfully
    // Make another syscall to exit
    unsafe {
        asm!(
            "li a7, 1",    // SYS_EXIT syscall number  
            "ecall",
            options(noreturn)
        );
    }
}

/// Allocate a user stack (just a static buffer for now)
#[no_mangle]
static mut USER_STACK: [u8; 4096] = [0; 4096];

pub fn get_user_stack_top() -> usize {
    unsafe {
        USER_STACK.as_ptr() as usize + USER_STACK.len()
    }
}