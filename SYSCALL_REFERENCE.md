\# woflOS Syscall Reference Card



Quick reference for user programs calling into the kernel.



---



\## Making Syscalls



All syscalls use the `ecall` instruction with:

\- \*\*a7\*\*: Syscall number

\- \*\*a0-a5\*\*: Arguments (up to 6)

\- \*\*a0\*\*: Return value



\### Assembly Template



```asm

li a7, <syscall\_number>

li a0, <arg0>

li a1, <arg1>

...

ecall

\# Return value now in a0

```



\### Rust Template



```rust

let result: usize;

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, {syscall}",

&nbsp;       "ecall",

&nbsp;       in("a0") arg0,

&nbsp;       in("a1") arg1,

&nbsp;       lateout("a0") result,

&nbsp;       syscall = const SYSCALL\_NUMBER,

&nbsp;   );

}

```



---



\## Available Syscalls



\### 1. SYS\_PUTC (1) - Write Character



Writes a single character to the serial console.



\*\*Arguments:\*\*

\- a0: Character to write (as usize, low byte used)



\*\*Returns:\*\*

\- 0 on success



\*\*Example:\*\*

```rust

// Print 'A'

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, 1",

&nbsp;       "li a0, 65",  // ASCII 'A'

&nbsp;       "ecall",

&nbsp;   );

}

```



\*\*Use Cases:\*\*

\- Print debugging output

\- Simple text output

\- Status messages



---



\### 2. SYS\_EXIT (2) - Exit Process



Terminates the current process.



\*\*Arguments:\*\*

\- a0: Exit code (0 = success, non-zero = error)



\*\*Returns:\*\*

\- Never returns



\*\*Example:\*\*

```rust

// Exit with code 0

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, 2",

&nbsp;       "li a0, 0",

&nbsp;       "ecall",

&nbsp;   );

}

```



\*\*Use Cases:\*\*

\- Clean process shutdown

\- Error exit with code

\- Return from main()



---



\### 3. SYS\_GETPID (3) - Get Process ID



Returns the current process ID.



\*\*Arguments:\*\*

\- None



\*\*Returns:\*\*

\- a0: Process ID (usize)



\*\*Example:\*\*

```rust

let pid: usize;

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, 3",

&nbsp;       "ecall",

&nbsp;       lateout("a0") pid,

&nbsp;   );

}

```



\*\*Use Cases:\*\*

\- Process identification

\- Debugging

\- IPC addressing (future)



---



\### 4. SYS\_YIELD (4) - Yield CPU



Voluntarily yields the CPU to the scheduler.



\*\*Arguments:\*\*

\- None



\*\*Returns:\*\*

\- 0 on success



\*\*Example:\*\*

```rust

unsafe {

&nbsp;   asm!(

&nbsp;       "li a7, 4",

&nbsp;       "ecall",

&nbsp;   );

}

```



\*\*Use Cases:\*\*

\- Cooperative multitasking

\- Busy-wait loops

\- Background tasks



\*\*Note:\*\* Currently a no-op (no scheduler yet).



---



\## Error Handling



\- \*\*Unknown syscall\*\*: Returns `usize::MAX` (0xFFFFFFFFFFFFFFFF)

\- \*\*Invalid arguments\*\*: Behavior depends on syscall

\- \*\*Kernel errors\*\*: Printed to console



Check return values!



---



\## Helper Functions (Rust)



\### Print String



```rust

fn print(s: \&str) {

&nbsp;   for byte in s.bytes() {

&nbsp;       unsafe {

&nbsp;           asm!(

&nbsp;               "li a7, 1",

&nbsp;               "ecall",

&nbsp;               in("a0") byte as usize,

&nbsp;           );

&nbsp;       }

&nbsp;   }

}

```



\### Print Hex Number



```rust

fn print\_hex(mut n: usize) {

&nbsp;   let digits = b"0123456789abcdef";

&nbsp;   for i in (0..16).rev() {

&nbsp;       let nibble = ((n >> (i \* 4)) \& 0xF) as usize;

&nbsp;       unsafe {

&nbsp;           asm!(

&nbsp;               "li a7, 1",

&nbsp;               "ecall",

&nbsp;               in("a0") digits\[nibble] as usize,

&nbsp;           );

&nbsp;       }

&nbsp;   }

}

```



\### Print Decimal Number



```rust

fn print\_dec(mut n: usize) {

&nbsp;   if n == 0 {

&nbsp;       unsafe {

&nbsp;           asm!("li a7, 1", "li a0, 48", "ecall"); // '0'

&nbsp;       }

&nbsp;       return;

&nbsp;   }

&nbsp;   

&nbsp;   let mut buf = \[0u8; 20];

&nbsp;   let mut i = 0;

&nbsp;   

&nbsp;   while n > 0 {

&nbsp;       buf\[i] = (n % 10) as u8 + b'0';

&nbsp;       n /= 10;

&nbsp;       i += 1;

&nbsp;   }

&nbsp;   

&nbsp;   while i > 0 {

&nbsp;       i -= 1;

&nbsp;       unsafe {

&nbsp;           asm!(

&nbsp;               "li a7, 1",

&nbsp;               "ecall",

&nbsp;               in("a0") buf\[i] as usize,

&nbsp;           );

&nbsp;       }

&nbsp;   }

}

```



---



\## Syscall Numbers (Quick Ref)



| Number | Name       | Args      | Description           |

|--------|------------|-----------|-----------------------|

| 1      | PUTC       | char      | Write character       |

| 2      | EXIT       | code      | Exit process          |

| 3      | GETPID     | -         | Get process ID        |

| 4      | YIELD      | -         | Yield CPU             |



---



\## Future Syscalls (Planned)



These will be added in later layers:



| Number | Name       | Layer | Description              |

|--------|------------|-------|--------------------------|

| 5      | SEND       | 4     | Send IPC message         |

| 6      | RECV       | 4     | Receive IPC message      |

| 7      | MAP        | 2     | Map memory region        |

| 8      | UNMAP      | 2     | Unmap memory region      |

| 9      | GRANT      | 5     | Grant capability         |

| 10     | VERIFY     | 5     | Verify capability        |



---



\## Calling Conventions



\### Register Usage



\*\*Preserved across syscalls:\*\*

\- sp (stack pointer)

\- s0-s11 (saved registers)

\- gp (global pointer)

\- tp (thread pointer)



\*\*Clobbered by syscalls:\*\*

\- a0-a7 (argument registers)

\- t0-t6 (temporary registers)

\- ra (return address - not used in syscalls)



\*\*Special:\*\*

\- PC automatically advanced by 4 after syscall



---



\## Example Programs



\### Hello World



```rust

\#\[no\_mangle]

pub extern "C" fn \_start() -> ! {

&nbsp;   let msg = b"Hello, woflOS!\\n";

&nbsp;   for \&ch in msg {

&nbsp;       unsafe {

&nbsp;           asm!(

&nbsp;               "li a7, 1",

&nbsp;               "ecall",

&nbsp;               in("a0") ch as usize,

&nbsp;           );

&nbsp;       }

&nbsp;   }

&nbsp;   

&nbsp;   unsafe {

&nbsp;       asm!(

&nbsp;           "li a7, 2",

&nbsp;           "li a0, 0",

&nbsp;           "ecall",

&nbsp;       );

&nbsp;   }

&nbsp;   

&nbsp;   loop {}

}

```



\### Echo Program (Future)



```rust

\#\[no\_mangle]

pub extern "C" fn \_start() -> ! {

&nbsp;   loop {

&nbsp;       // Read character (SYS\_GETC - not implemented yet)

&nbsp;       // Echo it back with SYS\_PUTC

&nbsp;       unsafe {

&nbsp;           asm!("li a7, 4", "ecall"); // Yield

&nbsp;       }

&nbsp;   }

}

```



---



\## Debugging



\### Check Syscall Execution



Add debug output in `src/syscall/mod.rs`:



```rust

pub fn handle\_syscall(ctx: \&mut Context) {

&nbsp;   let uart = Uart::new(0x1000\_0000);

&nbsp;   uart.puts("\[SYSCALL] num=");

&nbsp;   uart.print\_hex(ctx.syscall\_number() as u64);

&nbsp;   uart.puts("\\n");

&nbsp;   

&nbsp;   // ... rest of handler

}

```



\### Inspect Context



In trap handler:



```rust

uart.puts("Context dump:\\n");

uart.puts("  a0="); uart.print\_hex(ctx.a0 as u64); uart.puts("\\n");

uart.puts("  a7="); uart.print\_hex(ctx.a7 as u64); uart.puts("\\n");

uart.puts("  pc="); uart.print\_hex(ctx.pc as u64); uart.puts("\\n");

```



---



\*\*🐺 Built for wofl's OS journey\*\*  

\*Remember: With great syscalls comes great responsibility!\*

