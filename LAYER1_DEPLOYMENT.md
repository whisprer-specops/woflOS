\# woflOS Layer 1 - Privilege Transitions



\*\*Status:\*\* Ready to deploy! 🐺⚡  

\*\*Version:\*\* v0.4.0  

\*\*Achievement Unlocked:\*\* User mode execution + Syscalls



---



\## What's New in Layer 1



Layer 1 introduces \*\*privilege transitions\*\* - the ability to switch between kernel mode (S-mode) and user mode (U-mode) securely.



\### New Components



```

src/

├── process/

│   ├── mod.rs           # Process management

│   └── context.rs       # CPU context structure (31 regs + PC + sstatus)

├── syscall/

│   └── mod.rs           # Syscall interface (putc, exit, getpid, yield)

├── user/

│   ├── mod.rs           # User module wrapper

│   └── init.rs          # First userspace program

└── interrupts/

&nbsp;   └── trap.rs          # Updated: Now handles syscalls + interrupts

```



\### Architecture



```

┌─────────────────────────────────────────┐

│         Kernel (S-mode)                 │

│  - Full hardware access                 │

│  - Handles interrupts \& syscalls        │

│  - Manages processes                    │

└──────────┬──────────────────────────────┘

&nbsp;          │ sret (enter U-mode)

&nbsp;          │ ecall (syscall to S-mode)

&nbsp;          v

┌─────────────────────────────────────────┐

│       User Process (U-mode)             │

│  - Restricted privileges                │

│  - Cannot access hardware directly      │

│  - Uses syscalls for I/O                │

└─────────────────────────────────────────┘

```



---



\## File Updates



\### New Files to Create



1\. \*\*src/process/mod.rs\*\*

&nbsp;  - Process structure (PID, context, state)

&nbsp;  - Global process table

&nbsp;  - PID allocator



2\. \*\*src/process/context.rs\*\*

&nbsp;  - CPU context (all 31 registers + PC + sstatus)

&nbsp;  - Context creation for user processes

&nbsp;  - Syscall argument extraction



3\. \*\*src/syscall/mod.rs\*\*

&nbsp;  - Syscall numbers (SYS\_PUTC, SYS\_EXIT, SYS\_GETPID, SYS\_YIELD)

&nbsp;  - Syscall dispatcher

&nbsp;  - Individual syscall implementations



4\. \*\*src/user/mod.rs\*\*

&nbsp;  - User module wrapper



5\. \*\*src/user/init.rs\*\*

&nbsp;  - First userspace program (`user\_main`)

&nbsp;  - Process launcher (`launch\_init\_process`)



\### Files to Update



1\. \*\*src/main.rs\*\*

&nbsp;  - Add new module declarations

&nbsp;  - Initialize process subsystem

&nbsp;  - Launch first user process



2\. \*\*src/interrupts/trap.rs\*\*

&nbsp;  - Add syscall detection (scause == 8)

&nbsp;  - Dispatch to syscall handler

&nbsp;  - Update PC after syscall



---



\## Syscall Interface



woflOS Layer 1 provides 4 syscalls:



| Number | Name       | Args         | Description              |

|--------|------------|--------------|--------------------------|

| 1      | SYS\_PUTC   | a0: char     | Write character to UART  |

| 2      | SYS\_EXIT   | a0: code     | Exit process             |

| 3      | SYS\_GETPID | -            | Get process ID           |

| 4      | SYS\_YIELD  | -            | Yield CPU (nop for now)  |



\### Usage from User Code



```rust

// Print a character

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, 1",     // SYS\_PUTC

&nbsp;       "ecall",

&nbsp;       in("a0") 'A' as usize,

&nbsp;   );

}



// Get PID

let pid: usize;

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, 3",     // SYS\_GETPID

&nbsp;       "ecall",

&nbsp;       lateout("a0") pid,

&nbsp;   );

}

```



---



\## How It Works



\### 1. Process Creation



```rust

let process = Process::new(

&nbsp;   pid,              // Process ID

&nbsp;   "init",           // Process name

&nbsp;   entry\_point,      // Where to start executing

&nbsp;   stack\_pointer,    // Top of user stack

);

```



\### 2. Entering User Mode



The kernel uses `sret` to transition to U-mode:



```rust

asm!(

&nbsp;   "csrw sepc, {entry}",      // Set return address

&nbsp;   "csrw sstatus, {status}",  // SPP=0 for U-mode

&nbsp;   "mv sp, {stack}",          // Set user stack

&nbsp;   "sret",                    // Jump to userspace!

&nbsp;   entry = in(reg) user\_base,

&nbsp;   status = in(reg) 0x20,     // SPIE=1

&nbsp;   stack = in(reg) user\_stack\_top,

);

```



\### 3. Syscalls (U-mode → S-mode)



User code executes `ecall`:



```

User: ecall           # Trap to S-mode

&nbsp;     ↓

Trap: scause = 8      # U-mode ecall

&nbsp;     ↓

Handler: dispatch     # Call syscall handler

&nbsp;     ↓

Syscall: handle       # Execute syscall

&nbsp;     ↓

Return: sret          # Back to U-mode

```



\### 4. Context Switching



The trap handler saves/restores ALL CPU state:



\- 31 general-purpose registers (x1-x31)

\- Program counter (sepc)

\- Status register (sstatus)



This allows processes to be paused and resumed seamlessly.



---



\## Memory Layout



```

0x88000000 ┌─────────────────────────┐ Top of RAM

&nbsp;          │ Free Memory             │

&nbsp;          ├─────────────────────────┤

0x87010000 │ User Stack (64KB)       │

&nbsp;          ├─────────────────────────┤

0x87000000 │ User Code \& Data        │ ← First user process

&nbsp;          ├─────────────────────────┤

&nbsp;          │ Kernel Heap             │

&nbsp;          ├─────────────────────────┤

&nbsp;          │ Kernel .data/.bss       │

0x80200000 ├─────────────────────────┤ Kernel starts

&nbsp;          │ OpenSBI Firmware        │

0x80000000 └─────────────────────────┘ RAM starts

```



---



\## Building \& Running



\### In WSL Ubuntu:



```bash

cd v.0.4.0

chmod +x build.sh

./build.sh

```



\### Expected Output



```

============================================

&nbsp;\_\_      \_\_ \_\_\_  \_\_\_  \_     \_\_\_   \_\_\_ 

&nbsp;\\ \\    / // \_ \\| \_\_|| |   / \_ \\ / \_\_|

&nbsp; \\ \\/\\/ /| (\_) | \_| | |\_\_| (\_) |\\\_\_ \\

&nbsp;  \\\_/\\\_/  \\\_\_\_/|\_|  |\_\_\_\_|\\\_\_\_/ |\_\_\_/

&nbsp;                                       

&nbsp;  Microkernel + Capabilities + Crypto

============================================



\[OK] woflOS v0.4.0 - Layer 1 (Privilege Transitions)

\[OK] UART initialized

\[OK] BSS cleared

\[OK] Memory manager initialized



\[OK] Trap handler installed at 0x...

\[OK] Timer interrupts enabled

\[OK] Global interrupts enabled



\[OK] Initializing process subsystem...

\[OK] Process subsystem ready



\[TEST] Testing heap allocator...

\[OK] Heap allocator working!



═══════════════════════════════════════════

&nbsp;  Layer 0: Complete ✓

&nbsp;    - Memory management

&nbsp;    - Timer interrupts

&nbsp;    - Exception handling



&nbsp;  Layer 1: Activating...

&nbsp;    - Process structure

&nbsp;    - Syscall interface

&nbsp;    - User mode transition

═══════════════════════════════════════════



\[PROCESS] Launching init process...

\[PROCESS] User code at: 0x...

\[PROCESS] User memory at: 0x87000000

\[PROCESS] User stack at: 0x87010000

\[PROCESS] Code copied!

\[PROCESS] Allocated PID: 0x...

\[PROCESS] Process created!

\[PROCESS] Switching to user mode...



═════════════════════════════════════════

&nbsp; Entering userspace! (U-mode)

═════════════════════════════════════════



Hello from userspace!

My PID: 0x0000000000000001

\[SYSCALL] Process exit with code 0x0000000000000000000000000000000000000000000000000000000000000000

```



You'll also see timer ticks interspersed throughout!



---



\## Security Properties



\### What Layer 1 Achieves



✅ \*\*Privilege Separation\*\*: User code runs in U-mode with restricted access  

✅ \*\*Controlled Kernel Entry\*\*: Only via `ecall` (syscalls)  

✅ \*\*Context Isolation\*\*: All registers cleared on mode switch  

✅ \*\*Trap Handling\*\*: Interrupts work in both S-mode and U-mode  



\### What's Still Missing (Future Layers)



❌ \*\*Memory Isolation\*\*: No PMP yet - user can access kernel memory  

❌ \*\*Process Scheduling\*\*: Only one process, no preemption  

❌ \*\*IPC\*\*: No inter-process communication  

❌ \*\*Capabilities\*\*: No cryptographic security model  



---



\## Debugging Tips



\### If user process doesn't print anything:



1\. Check that `launch\_init\_process()` is called

2\. Verify `sepc` is set correctly (use QEMU monitor: `info registers`)

3\. Check `sstatus.SPP` is 0 (U-mode)



\### If you get illegal instruction exception:



\- User code might be trying to execute privileged instructions

\- Check that code was copied correctly to user memory



\### If syscalls don't work:



1\. Verify trap handler detects `scause == 8`

2\. Check `syscall::handle\_syscall()` is called

3\. Ensure PC is advanced by 4 after ecall



\### QEMU Commands



```

Ctrl+A, C          # Enter QEMU monitor

info registers     # Show all CPU registers

info mem           # Show memory mappings

q                  # Quit QEMU

```



---



\## Next Steps: Layer 2 (Process Isolation)



Once Layer 1 is working, we'll add:



1\. \*\*PMP Configuration\*\*: Use Physical Memory Protection to isolate user memory

2\. \*\*Multiple Processes\*\*: Support more than one user process

3\. \*\*Simple Scheduler\*\*: Round-robin between processes

4\. \*\*Better Memory Management\*\*: Proper user memory allocation



---



\## Technical Details



\### RISC-V Privilege Modes



\- \*\*M-mode (Machine)\*\*: OpenSBI firmware - highest privilege

\- \*\*S-mode (Supervisor)\*\*: woflOS kernel - manages processes

\- \*\*U-mode (User)\*\*: User processes - restricted access



\### Key CSRs (Control and Status Registers)



\- `stvec`: Trap handler address

\- `sepc`: Return address after trap

\- `scause`: Trap cause (interrupt or exception number)

\- `sstatus`: Status register (privilege level, interrupts)

\- `stval`: Trap-specific value (e.g., faulting address)



\### Trap Flow



```

User: ecall

&nbsp; ↓

CPU: Save PC → sepc

&nbsp;    Set scause = 8 (U-mode ecall)

&nbsp;    Set sstatus.SPP = previous mode (U)

&nbsp;    Jump to stvec (trap handler)

&nbsp; ↓

Kernel: trap\_entry (save all regs)

&nbsp;       trap\_handler (dispatch)

&nbsp;       syscall::handle\_syscall

&nbsp;       (set return value in a0)

&nbsp;       (advance PC by 4)

&nbsp;       trap\_entry (restore all regs)

&nbsp;       sret

&nbsp; ↓

User: Continue execution

```



---



\*\*Built with 🐺 by wofl\*\*  

\*\*Guided by Claude Sonnet 4.5\*\*



\*"From bare metal to user mode - the journey continues!"\*

