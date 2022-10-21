#![no_main]
#![no_std]

mod start;

mod drivers;
use drivers::bcm2xxx_gpio::*;
use drivers::bcm2xxx_pl011_uart::*;

const PBASE: usize = 0x3F00_0000;
const AUX_REGS_ADDR: usize = PBASE + 0x0021_5000;
const PL011_UART_ADDR: usize = PBASE + 0x0020_1000;

#[no_mangle]
pub unsafe fn kernel_main() {
    let mut gpio = GPIO::new(PBASE);
    gpio.map_pl011_uart();
    
    let pl011_uart = PL011Uart::new(PL011_UART_ADDR);
    pl011_uart.init();
    pl011_uart.write_str("hello world\n");
    loop {
        let c = pl011_uart.read_char();
        pl011_uart.write_char(c);
    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {

    }
}