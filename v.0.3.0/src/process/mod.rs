pub mod context;

use context::Context;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Process ID type
pub type Pid = usize;

/// Process states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,      // Ready to run
    Running,    // Currently executing
    Blocked,    // Waiting for something
    Dead,       // Finished execution
}

/// Process Control Block (PCB)
/// 
/// This is the kernel's view of a process. It contains everything needed
/// to schedule, switch, and manage the process.
#[derive(Debug)]
pub struct Process {
    pub pid: Pid,
    pub context: Context,
    pub state: ProcessState,
    pub name: &'static str,
}

impl Process {
    /// Create a new process
    pub fn new(pid: Pid, name: &'static str, entry_point: usize, stack: usize) -> Self {
        Process {
            pid,
            context: Context::new_user(entry_point, stack),
            state: ProcessState::Ready,
            name,
        }
    }
    
    /// Mark process as running
    pub fn set_running(&mut self) {
        self.state = ProcessState::Running;
    }
    
    /// Mark process as ready
    pub fn set_ready(&mut self) {
        self.state = ProcessState::Ready;
    }
}

/// Global PID counter
static NEXT_PID: AtomicUsize = AtomicUsize::new(1);

/// Allocate a new process ID
pub fn alloc_pid() -> Pid {
    NEXT_PID.fetch_add(1, Ordering::Relaxed)
}

/// Simple process table (for now, just one process)
static mut CURRENT_PROCESS: Option<Process> = None;

/// Get the current running process
pub fn current_process() -> Option<&'static mut Process> {
    unsafe { CURRENT_PROCESS.as_mut() }
}

/// Set the current process
pub unsafe fn set_current_process(process: Process) {
    CURRENT_PROCESS = Some(process);
}

/// Initialize process subsystem
pub fn init() {
    // For now, nothing to initialize
    // Later: process table, scheduler, etc.
}