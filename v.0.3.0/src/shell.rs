use crate::uart::Uart;
use crate::memory;

/// Shell state
pub struct Shell {
    uart: Uart,
    buffer: [u8; 256],
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            uart: Uart::new(0x1000_0000),
            buffer: [0; 256],
        }
    }
    
    /// Print the shell prompt
    fn print_prompt(&self) {
        self.uart.puts("woflOS> ");
    }
    
    /// Run the shell - main loop
    pub fn run(&mut self) -> ! {
        self.uart.puts("\n");
        self.uart.puts("╔═══════════════════════════════════════╗\n");
        self.uart.puts("║  Welcome to woflOS Interactive Shell  ║\n");
        self.uart.puts("╚═══════════════════════════════════════╝\n");
        self.uart.puts("\n");
        self.uart.puts("Type 'help' for available commands\n");
        self.uart.puts("\n");
        
        loop {
            self.print_prompt();
            
            // Read a line of input
            let len = self.uart.getline(&mut self.buffer);
            
            if len > 0 {
                // Parse and execute command
                self.execute_command(&self.buffer[..len]);
            }
        }
    }
    
    /// Execute a command
    fn execute_command(&self, input: &[u8]) {
        // Convert to string (ignore invalid UTF-8)
        let cmd_str = core::str::from_utf8(input).unwrap_or("");
        
        // Split into command and arguments
        let mut parts = cmd_str.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        
        match cmd {
            "help" => self.cmd_help(),
            "meminfo" => self.cmd_meminfo(),
            "clear" => self.cmd_clear(),
            "echo" => self.cmd_echo(parts),
            "about" => self.cmd_about(),
            "" => {}, // Empty command, do nothing
            _ => {
                self.uart.puts("Unknown command: ");
                self.uart.puts(cmd);
                self.uart.puts("\n");
                self.uart.puts("Type 'help' for available commands\n");
            }
        }
    }
    
    /// Help command
    fn cmd_help(&self) {
        self.uart.puts("\nAvailable commands:\n");
        self.uart.puts("  help     - Show this help message\n");
        self.uart.puts("  about    - About woflOS\n");
        self.uart.puts("  meminfo  - Display memory information\n");
        self.uart.puts("  echo     - Echo text back\n");
        self.uart.puts("  clear    - Clear the screen\n");
        self.uart.puts("\n");
    }
    
    /// Memory info command
    fn cmd_meminfo(&self) {
        self.uart.puts("\n");
        self.uart.puts("═══ Memory Information ═══\n");
        self.uart.puts("\n");
        
        let (used_frames, total_frames) = memory::frame::get_stats();
        let heap_used = memory::heap::heap_used();
        
        self.uart.puts("Physical Memory:\n");
        self.uart.puts("  Frames used:      ");
        self.print_number(used_frames);
        self.uart.puts("\n");
        
        self.uart.puts("  Frames total:     ");
        self.print_number(total_frames);
        self.uart.puts("\n");
        
        self.uart.puts("  Frame size:       4096 bytes\n");
        self.uart.puts("\n");
        
        self.uart.puts("Kernel Heap:\n");
        self.uart.puts("  Heap used:        ");
        self.print_number(heap_used);
        self.uart.puts(" bytes\n");
        
        self.uart.puts("  Heap total:       65536 bytes\n");
        self.uart.puts("\n");
    }
    
    /// Clear screen command
    fn cmd_clear(&self) {
        // ANSI escape sequence to clear screen
        self.uart.puts("\x1b[2J\x1b[H");
    }
    
    /// Echo command
    fn cmd_echo(&self, mut args: core::str::SplitWhitespace) {
        while let Some(word) = args.next() {
            self.uart.puts(word);
            if args.clone().next().is_some() {
                self.uart.puts(" ");
            }
        }
        self.uart.puts("\n");
    }
    
    /// About command
    fn cmd_about(&self) {
        self.uart.puts("\n");
        self.uart.puts("╔═══════════════════════════════════════╗\n");
        self.uart.puts("║            woflOS v0.3.0              ║\n");
        self.uart.puts("╠═══════════════════════════════════════╣\n");
        self.uart.puts("║ Architecture:  RISC-V 64-bit          ║\n");
        self.uart.puts("║ Type:          Microkernel            ║\n");
        self.uart.puts("║ Language:      Rust                   ║\n");
        self.uart.puts("║ Design:        Stability-First        ║\n");
        self.uart.puts("╠═══════════════════════════════════════╣\n");
        self.uart.puts("║ Features:                             ║\n");
        self.uart.puts("║  ✓ Memory Management                  ║\n");
        self.uart.puts("║  ✓ Dynamic Allocation                 ║\n");
        self.uart.puts("║  ✓ Interactive Shell                  ║\n");
        self.uart.puts("║  ✓ Serial Console I/O                 ║\n");
        self.uart.puts("╚═══════════════════════════════════════╝\n");
        self.uart.puts("\n");
        self.uart.puts("Built with 🐺 by wofl\n");
        self.uart.puts("\n");
    }
    
    /// Helper: Print a number as decimal
    fn print_number(&self, mut n: usize) {
        if n == 0 {
            self.uart.putc(b'0');
            return;
        }
        
        // Build number in reverse
        let mut digits = [0u8; 20];
        let mut i = 0;
        
        while n > 0 {
            digits[i] = (n % 10) as u8 + b'0';
            n /= 10;
            i += 1;
        }
        
        // Print in correct order
        while i > 0 {
            i -= 1;
            self.uart.putc(digits[i]);
        }
    }
}