use core::fmt;

use uart;

pub const WRITER: Writer = Writer{};

pub struct Writer {

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          uart::uart_send_byte(byte)
        }
        Ok(())
    }
}

// pub fn write_str(s: &str) {
//     for byte in s.bytes() {
//       uart::uart_send_byte(byte)
//     }
// } 