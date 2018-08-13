use gpio;
use mailbox;

#[inline(always)]
fn nop() {
    unsafe { asm!("nop" :::: "volatile"); }
}

pub fn uart_init() {
    unsafe {
        gpio::UART0_CR.write_volatile(0);

        //TODO: Constructor?
        let mut message = mailbox::MboxMessage {
            data: [0;64],
        };

        //TODO: Build mailbox message
        message.data[0] = 8 * 4;
        message.data[1] = mailbox::MBOX_REQUEST;
        message.data[2] = mailbox::MBOX_TAG_SETCLKRATE;
        message.data[3] = 12;
        message.data[4] = 8;
        message.data[5] = 2;
        message.data[6] = 4000000;
        message.data[7] = mailbox::MBOX_TAG_LAST;

        mailbox::mbox_call(mailbox::MboxChannel::PROP, &mut message);

        //Map UART0 to GPIO
        let mut r = gpio::GPFSEL1.read();

        r &= !((7 << 12) | (7 << 15));
        r |=  ((4 << 12) | (4 << 15));

        gpio::GPFSEL1.write_volatile(r);

        gpio::GPPUD.write_volatile(0);

        for _ in 1..150 { nop(); }
        gpio::GPPUDCLK0.write_volatile((1 << 14) | (1 << 15));
        for _ in 1..150 { nop(); }
        gpio::GPPUDCLK0.write_volatile(0);

        //Enable UART0
        gpio::UART0_ICR.write_volatile(0x7FF);    // clear interrupts
        gpio::UART0_IBRD.write_volatile(2);       // 115200 baud
        gpio::UART0_FBRD.write_volatile(0xB);
        gpio::UART0_LCRH.write_volatile(0x3 << 5); // 8n1
        gpio::UART0_CR.write_volatile(0x301);     // enable Tx, Rx, FIFO
    }
}

pub fn uart_send(c: u32) {
    unsafe {
        while gpio::UART0_FR.read_volatile() & 0x20 != 0 { 
            nop();
        }

        gpio::UART0_DR.write_volatile(c);
    }
}

pub fn uart_send_byte(c: u8) {
    uart_send(c as u32);
}


// Private UART
//--------------------------------------------------------------------

