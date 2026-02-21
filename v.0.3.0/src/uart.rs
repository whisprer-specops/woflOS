/// UART driver for 16550-compatible serial console
use core::fmt;

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        let uart = Uart { base_address };
        uart.init();
        uart
    }
    
    fn init(&self) {
        // QEMU's UART works without explicit initialization
    }
    
    pub fn putc(&self, c: u8) {
        unsafe {
            let ptr = self.base_address as *mut u8;
            ptr.write_volatile(c);
        }
    }
    
    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            self.putc(byte);
        }
    }
    
    #[allow(dead_code)]
    pub fn getc(&self) -> Option<u8> {
        unsafe {
            let lsr = ((self.base_address + 5) as *const u8).read_volatile();
            if (lsr & 0x01) != 0 {
                let ptr = self.base_address as *const u8;
                Some(ptr.read_volatile())
            } else {
                None
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn getline(&self, buffer: &mut [u8]) -> usize {
        let mut pos = 0;
        
        loop {
            if let Some(byte) = self.getc() {
                match byte {
                    b'\r' | b'\n' => {
                        self.putc(b'\n');
                        return pos;
                    }
                    0x7F | 0x08 => {
                        if pos > 0 {
                            pos -= 1;
                            self.putc(0x08);
                            self.putc(b' ');
                            self.putc(0x08);
                        }
                    }
                    byte if byte >= 0x20 && byte < 0x7F && pos < buffer.len() => {
                        buffer[pos] = byte;
                        pos += 1;
                        self.putc(byte);
                    }
                    _ => {}
                }
            }
            
            for _ in 0..1000 {
                core::hint::spin_loop();
            }
        }
    }
    
    // Print in hex (no division needed!)
    pub fn print_hex(&self, n: u64) {
        self.puts("0x");
        for i in (0..16).rev() {
            let nibble = ((n >> (i * 4)) & 0xF) as u8;
            let c = if nibble < 10 {
                b'0' + nibble
            } else {
                b'a' + (nibble - 10)
            };
            self.putc(c);
        }
    }
}

/// Simple global printing support for `no_std`.
///
/// This is intentionally tiny: it only needs to be good enough for
/// bring-up and debugging of Layers 0-1.
struct UartWriter;

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let uart = Uart::new(0x1000_0000);
        uart.puts(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    let mut w = UartWriter;
    let _ = w.write_fmt(args);
}

/// Kernel print (no newline)
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::uart::_print(core::format_args!($($arg)*)));
}

/// Kernel print (with newline)
#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($fmt:expr) => ($crate::kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::kprint!(concat!($fmt, "\n"), $($arg)*));
}
