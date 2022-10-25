use core::fmt::Write;

use super::drivers;

const PBASE: usize = 0x3F00_0000;
const GPIO_ADDR: usize = PBASE + 0x0020_0000;
const AUX_REGS_ADDR: usize = PBASE + 0x0021_5000;
const _PL011_UART_ADDR: usize = PBASE + 0x0020_1000;


pub static mut MINI_UART_GLOBAL: drivers::bcm2837_mini_uart::MiniUart = unsafe { drivers::bcm2837_mini_uart::MiniUart::new(AUX_REGS_ADDR) };
pub static mut GPIO_GLOBAL: drivers::bcm2xxx_gpio::GPIO = unsafe { drivers::bcm2xxx_gpio::GPIO::new(GPIO_ADDR) };

pub fn uart_init() {
    unsafe {
        GPIO_GLOBAL.init_mini_uart_pins();
        MINI_UART_GLOBAL.init();
    }
}