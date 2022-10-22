use super::drivers;

const PBASE: usize = 0x3F00_0000;
const AUX_REGS_ADDR: usize = PBASE + 0x0021_5000;
const PL011_UART_ADDR: usize = PBASE + 0x0020_1000;


pub static PL011_UART: drivers::bcm2xxx_pl011_uart::PL011Uart = unsafe { drivers::bcm2xxx_pl011_uart::PL011Uart::new(PL011_UART_ADDR) };
pub static GPIO_GLOBAL: drivers::bcm2xxx_gpio::GPIO = unsafe { drivers::bcm2xxx_gpio::GPIO::new(PBASE) };