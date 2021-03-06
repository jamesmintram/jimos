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

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::kwriter::WRITER.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

// pub fn write_str(s: &str) {
//     for byte in s.bytes() {
//       uart::uart_send_byte(byte)
//     }
// } 