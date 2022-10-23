#![no_main]
#![no_std]

use bsp::raspberrypi::MINI_UART_GLOBAL;

mod bsp;
mod exception;
mod start;

#[no_mangle]
pub unsafe fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();

    loop {
        let c = MINI_UART_GLOBAL.read_char();
        MINI_UART_GLOBAL.write_char(c);
    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    MINI_UART_GLOBAL.write_str("uWu panic!\n");
    loop {

    }
}