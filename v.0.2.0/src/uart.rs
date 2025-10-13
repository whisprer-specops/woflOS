/// UART driver for 16550-compatible serial console
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
}