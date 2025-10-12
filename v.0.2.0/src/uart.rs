/// UART driver for 16550-compatible serial console
/// Used by QEMU's virt machine
pub struct Uart {
    base_address: usize,
}

impl Uart {
    /// Create a new UART instance at the given base address
    /// For QEMU virt: 0x1000_0000
    pub fn new(base_address: usize) -> Self {
        let uart = Uart { base_address };
        uart.init();
        uart
    }
    
    /// Initialize the UART hardware
    fn init(&self) {
        // For now, QEMU's UART works without explicit initialization
        // In a real implementation, we'd set baud rate, data bits, etc.
    }
    
    /// Write a single byte to the UART
    pub fn putc(&self, c: u8) {
        unsafe {
            let ptr = self.base_address as *mut u8;
            ptr.write_volatile(c);
        }
    }
    
    /// Write a string to the UART
    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            self.putc(byte);
        }
    }
}