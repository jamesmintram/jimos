
// Public mailbox
//--------------------------------------------------------------------
use gpio;

pub const MBOX_REQUEST:     u32 = 0;

#[inline(always)]
fn nop() {
    unsafe { asm!("nop" :::: "volatile"); }
}

#[repr(align(16))]
pub struct MboxMessage {
    pub data: [u32; 64]
}


/* channels */
#[derive(Copy, Clone)]
pub enum MboxChannel {
    POWER = 0,
    FB    = 1,
    VUART = 2,
    VCHIQ = 3,
    LEDS  = 4,
    BTNS  = 5,
    TOUCH = 6,
    COUNT = 7,
    PROP  = 8,
}

/* tags */

pub const MBOX_TAG_GETSERIAL: u32  = 0x10004;
pub const MBOX_TAG_SETCLKRATE: u32 = 0x38002;
pub const MBOX_TAG_LAST: u32       = 0;


// Private mailbox
//--------------------------------------------------------------------

const VIDEOCORE_MBOX: u32 = gpio::MMIO_BASE+0x0000B880;
const MBOX_READ:   *mut u32 = (VIDEOCORE_MBOX + 0x00) as *mut u32;
const MBOX_POLL:   *mut u32 = (VIDEOCORE_MBOX + 0x10) as *mut u32;
const MBOX_SENDER: *mut u32 = (VIDEOCORE_MBOX + 0x14) as *mut u32;
const MBOX_STATUS: *mut u32 = (VIDEOCORE_MBOX + 0x18) as *mut u32;
const MBOX_CONFIG: *mut u32 = (VIDEOCORE_MBOX + 0x1C) as *mut u32;
const MBOX_WRITE:  *mut u32 = (VIDEOCORE_MBOX + 0x20) as *mut u32;

const MBOX_RESPONSE:  u32 = 0x80000000;
const MBOX_FULL:      u32 = 0x80000000;
const MBOX_EMPTY:     u32 = 0x40000000;

pub fn mbox_call(channel: MboxChannel, message: &mut MboxMessage) -> bool {
    
    //TODO: Possible to set something as volatile?
    let chars: *mut u32 = &mut message.data[0] as *mut u32;
    //TODO: Assert that (chars & !0xF) == chars (ie - data is correctly aligned)

    unsafe {

        while MBOX_STATUS.read_volatile() & MBOX_FULL != 0 {
            nop()
        }

        //Write here
        let write_data = (chars as u32) | (channel as u32);
        MBOX_WRITE.write_volatile(write_data);

        
        loop {
            // Wait for a response
            while MBOX_STATUS.read_volatile() & MBOX_EMPTY != 0 {
                nop()
            }

            let r = MBOX_READ.read_volatile();

            let read_chan = (r & 0xF) as u8;
            let read_data = (r & !0xF) as *const u32;

            //TODO: Stuff these into a register and loop - we can then break

            if read_chan == (channel as u8)  {
                return false;
            }

            // Is it a response to our message?
            if read_chan == (channel as u8) && read_data == chars {
                return *chars.offset(1) == MBOX_RESPONSE;
            }
        }
    }
}