use core::fmt;

use uart;

pub const WRITER: Writer = Writer{};

pub struct Writer {

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            
            uart::uart_send_byte(byte);
            if byte == '\n' as u8 {
                uart::uart_send_byte('\r' as u8)
            }
        }
        Ok(())
    }
}

// pub fn write_str(s: &str) {
//     for byte in s.bytes() {
//       uart::uart_send_byte(byte)
//     }
// } 