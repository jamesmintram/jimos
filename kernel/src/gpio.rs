
pub const MMIO_BASE: u32 = 0x3F000000;

// GPIO
//--------------------------------------------------------------------

pub const GPIO_BASE: u32 = MMIO_BASE + 0x200000;

pub const GPFSEL0:     *mut u32 = (GPIO_BASE+0x00) as *mut u32;
pub const GPFSEL1:     *mut u32 = (GPIO_BASE+0x04) as *mut u32;
pub const GPFSEL2:     *mut u32 = (GPIO_BASE+0x08) as *mut u32;
pub const GPFSEL3:     *mut u32 = (GPIO_BASE+0x0C) as *mut u32;
pub const GPFSEL4:     *mut u32 = (GPIO_BASE+0x10) as *mut u32;
pub const GPFSEL5:     *mut u32 = (GPIO_BASE+0x14) as *mut u32;
pub const GPSET0:      *mut u32 = (GPIO_BASE+0x1C) as *mut u32;
pub const GPSET1:      *mut u32 = (GPIO_BASE+0x20) as *mut u32;
pub const GPCLR0:      *mut u32 = (GPIO_BASE+0x28) as *mut u32;
pub const GPLEV0:      *mut u32 = (GPIO_BASE+0x34) as *mut u32;
pub const GPLEV1:      *mut u32 = (GPIO_BASE+0x38) as *mut u32;
pub const GPEDS0:      *mut u32 = (GPIO_BASE+0x40) as *mut u32;
pub const GPEDS1:      *mut u32 = (GPIO_BASE+0x44) as *mut u32;
pub const GPHEN0:      *mut u32 = (GPIO_BASE+0x64) as *mut u32;
pub const GPHEN1:      *mut u32 = (GPIO_BASE+0x68) as *mut u32;
pub const GPPUD:       *mut u32 = (GPIO_BASE+0x94) as *mut u32;
pub const GPPUDCLK0:   *mut u32 = (GPIO_BASE+0x98) as *mut u32;
pub const GPPUDCLK1:   *mut u32 = (GPIO_BASE+0x9C) as *mut u32;


pub const UART0_DR:     *mut u32 = (MMIO_BASE + 0x00201000) as *mut u32;
pub const UART0_FR:     *mut u32 = (MMIO_BASE + 0x00201018) as *mut u32;
pub const UART0_IBRD:   *mut u32 = (MMIO_BASE + 0x00201024) as *mut u32;
pub const UART0_FBRD :  *mut u32 = (MMIO_BASE + 0x00201028) as *mut u32;
pub const UART0_LCRH:   *mut u32 = (MMIO_BASE + 0x0020102C) as *mut u32;
pub const UART0_CR:     *mut u32 = (MMIO_BASE + 0x00201030) as *mut u32;
pub const UART0_IMSC:   *mut u32 = (MMIO_BASE + 0x00201038) as *mut u32;
pub const UART0_ICR:    *mut u32 = (MMIO_BASE + 0x00201044) as *mut u32;
