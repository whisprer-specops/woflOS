The project woflOS is an operating system designed for stability and
performance, with a clean, open, and future-proof architecture based
on RISC-V. It employs a microkernel design featuring isolated services to
maximize stability. The system is implemented in Rust to ensure memory
safety and prevent segmentation faults. The development environment includes
WSL Ubuntu and QEMU for testing and emulation.


Architecture: RISC-V (clean, open, future-proof)
Kernel Type:  Microkernel (isolated services, max stability)
Language:     Rust (memory safety = no segfaults)
Tooling:      WSL Ubuntu + QEMU


Memory Architecture Plan:RISC-V Memory Layout (128MB):

┌─────────────────────────────────┐ 0x88000000 (Top of RAM)
│                                 |
│   Free Memory Pool              │ ← We allocate from here
│                                 │
├─────────────────────────────────┤ End of kernel
│   woflOS Kernel (.text/.data)   │
├─────────────────────────────────┤ 0x80200000
│   OpenSBI Firmware              │
└─────────────────────────────────┘ 0x80000000


The memory architecture for woflOS is allocated from a free memory pool
starting at address 0x88000000, just above the end of the kernel located
at 0x80200000. The OpenSBI firmware resides at 0x80000000.

The boot sequence for woflOS involves initializing various platform components
such as console devices and timers, followed by clearing and initializing memory.
Debug information confirms successful heap allocation and readiness for
input/output operations via UART, validating basic system functions.


The design roadmap highlights the following key phases:

Layer 0 (Trap Handling): This includes boot sequence, memory and heap allocation,
timer interrupts, exception handling, and dispatcher setup.

Layer 1 (Privilege Transitions): Implementation of context switching between
supervisor and user modes.

Layer 2 (Process Isolation): Configuration of Physical Memory Protection (PMP)
toisolate user processes and running of initial userspace programs.

Layer 3 (Scheduling): Enabling multitasking with a simple round-robin scheduler and
timer-based preemption.

Layer 4 (IPC Foundation): Establishing synchronous message passing and kernel message
buffers for inter-process communication.

Layer 5 (Capabilities): Development of a capability structure with Ed25519 cryptographic
signing, verification on syscalls, and capability passing through IPC to build the
security model.


The project emphasizes a radical security model rooted in capability-based security,
where cryptographically unforgeable tokens replace traditional permission models.
Capabilities are signed by the kernel’s private key and grant access rights to
resources such as memory regions or IPC endpoints. Processes acquire and delegate
capabilities to communicate and access resources securely.


Memory As Capabilities
Forget malloc(). Forget mmap(). Everything is capabilities.
rust// Request memory from kernel
let mem_cap = syscall::request_memory(4096)?;

// mem_cap is a signed token proving you own this 4KB region
// You can:
// - Read/write it (if rights allow)
// - Split it into smaller capabilities
// - Pass it to another process via IPC
// - Return it to the kernel


An advanced concept within woflOS is that the kernel has a single syscall for IPC,
simplifying the system interface. All resource requests, like memory allocation or
device access, are handled through IPC to userspace servers. This microkernel
architecture reduces the trusted computing base, isolating device drivers and
filesystems as untrusted components that can be restarted without compromising
system integrity.

The security design benefits from Rust’s memory safety and RISC-V’s privilege levels,
leveraging PMP registers for isolation enforcement. The cryptographic capability system
promises advantages such as expiration, delegation, and persistent capabilities across
kernel restarts.

Kernel TCB: ~2000 lines of Rust
├─ Capability verification (crypto)
├─ IPC delivery
├─ Context switching  
├─ PMP enforcement
└─ Timer interrupts

Everything else is UNTRUSTED:
├─ Device drivers (can crash, no problem)
├─ Filesystems (compromised? Isolated)
├─ Network stack (buggy? Who cares)
└─ Even the "init" process (it just has capabilities, not power)

The anticipated next steps involve enabling timer interrupts, implementing context
switching to user mode, configuring PMP, developing basic IPC mechanisms, and moving key
drivers to userspace to demonstrate capability-based security in practice.

This approach aims to build a secure, minimal trusted base OS with cryptographic
enforcement, capability-based IPC, and microkernel isolation on a cutting-edge RISC-V
platform programmed in Rust, setting the foundation for future experimental and
security-hardened operating system research and development.

could you please now draw me an ASCII art diagram to represent the way the security
feature involving cryptographic signing operates please fren?

Here's an ASCII art diagram representing the cryptographic signing security feature in woflOS:

                   +-----------------------+
                   |        Kernel         |
                   |  (holds Private Key)  |
                   +-----------+-----------+
                               |
            Signs Capability Token with Private Key
                               |
                               v
         +--------------------------------------------+
         |          Cryptographically Signed          |
         |             Capability Token               |
         | (unforgeable access permission granted)    |
         +--------------------------------------------+
                               |
                               |       Passed via IPC
                               v
                 +---------------------------+
                 |        User Process       |
                 | (holds Capability Token)  |
                 +-------------+-------------+
                               |
        Uses Capability Token to request resource access
                               |
                               v
         +--------------------------------------------+
         |         Kernel verifies Signature          |
         |        using Public Key counterpart        |
         +--------------------------------------------+
                               |
          Grants or denies access based on verification

This diagram illustrates how the kernel signs capabilities with its
private key to create unforgeable tokens that user processes hold and
use to request access. The kernel verifies these tokens cryptographically
to enforce access control securely.