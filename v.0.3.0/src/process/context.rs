/// CPU context for RISC-V process switching
/// 
/// This structure holds ALL CPU state needed to pause/resume a process.
/// RISC-V has 32 general-purpose registers (x0-x31), but x0 is hardwired to zero,
/// so we only need to save x1-x31 (31 registers).

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    // General purpose registers (x1-x31)
    // x0 is always zero, so we don't save it
    pub ra: usize,      // x1  - return address
    pub sp: usize,      // x2  - stack pointer
    pub gp: usize,      // x3  - global pointer
    pub tp: usize,      // x4  - thread pointer
    pub t0: usize,      // x5  - temporary
    pub t1: usize,      // x6  - temporary
    pub t2: usize,      // x7  - temporary
    pub s0: usize,      // x8  - saved register / frame pointer
    pub s1: usize,      // x9  - saved register
    pub a0: usize,      // x10 - argument/return value
    pub a1: usize,      // x11 - argument/return value
    pub a2: usize,      // x12 - argument
    pub a3: usize,      // x13 - argument
    pub a4: usize,      // x14 - argument
    pub a5: usize,      // x15 - argument
    pub a6: usize,      // x16 - argument
    pub a7: usize,      // x17 - argument
    pub s2: usize,      // x18 - saved register
    pub s3: usize,      // x19 - saved register
    pub s4: usize,      // x20 - saved register
    pub s5: usize,      // x21 - saved register
    pub s6: usize,      // x22 - saved register
    pub s7: usize,      // x23 - saved register
    pub s8: usize,      // x24 - saved register
    pub s9: usize,      // x25 - saved register
    pub s10: usize,     // x26 - saved register
    pub s11: usize,     // x27 - saved register
    pub t3: usize,      // x28 - temporary
    pub t4: usize,      // x29 - temporary
    pub t5: usize,      // x30 - temporary
    pub t6: usize,      // x31 - temporary
    
    // Special registers
    pub pc: usize,      // Program counter (sepc for S-mode)
    pub sstatus: usize, // Status register (privilege level, interrupts, etc.)
}

impl Context {
    /// Create a new zeroed context
    pub const fn zero() -> Self {
        Context {
            ra: 0, sp: 0, gp: 0, tp: 0,
            t0: 0, t1: 0, t2: 0,
            s0: 0, s1: 0,
            a0: 0, a1: 0, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0,
            s2: 0, s3: 0, s4: 0, s5: 0, s6: 0, s7: 0, s8: 0, s9: 0, s10: 0, s11: 0,
            t3: 0, t4: 0, t5: 0, t6: 0,
            pc: 0,
            sstatus: 0,
        }
    }
    
    /// Create a new context for a user process
    /// 
    /// # Arguments
    /// * `entry_point` - Where the process should start executing
    /// * `user_stack` - Top of the user stack
    pub fn new_user(entry_point: usize, user_stack: usize) -> Self {
        let mut ctx = Self::zero();
        
        ctx.pc = entry_point;
        ctx.sp = user_stack;
        
        // Set up sstatus for user mode:
        // - SPP (bit 8) = 0 for user mode (will return to U-mode on sret)
        // - SPIE (bit 5) = 1 to enable interrupts when we sret
        // - SIE (bit 1) = 0 during trap handling
        ctx.sstatus = 1 << 5; // SPIE = 1
        
        ctx
    }
    
    /// Set return value in context (for syscalls)
    pub fn set_return_value(&mut self, value: usize) {
        self.a0 = value;
    }
    
    /// Get syscall number (from a7 register)
    pub fn syscall_number(&self) -> usize {
        self.a7
    }
    
    /// Get syscall arguments (a0-a5)
    pub fn syscall_args(&self) -> [usize; 6] {
        [self.a0, self.a1, self.a2, self.a3, self.a4, self.a5]
    }
}