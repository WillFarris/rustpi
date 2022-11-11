use super::drivers;

pub const PBASE_START: usize = 0x3F00_0000;
pub const PBASE_END: usize = 0x4000_FFFF;
pub const GPIO_ADDR: usize = PBASE_START + 0x0020_0000;
pub const AUX_REGS_ADDR: usize = PBASE_START + 0x0021_5000;
const _PL011_UART_ADDR: usize = PBASE_START + 0x0020_1000;


pub static mut MINI_UART_GLOBAL: drivers::bcm2837_mini_uart::MiniUart = unsafe { drivers::bcm2837_mini_uart::MiniUart::new(AUX_REGS_ADDR) };
pub static mut GPIO_GLOBAL: drivers::bcm2xxx_gpio::GPIO = unsafe { drivers::bcm2xxx_gpio::GPIO::new(GPIO_ADDR) };

pub fn uart_init() {
    unsafe {
        GPIO_GLOBAL.init_mini_uart_pins();
        MINI_UART_GLOBAL.init();
    }
}