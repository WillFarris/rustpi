#![no_main]
#![no_std]

use bsp::raspberrypi::PL011_UART;


mod start;
mod exception;

mod bsp;



#[no_mangle]
pub unsafe fn kernel_main() {
    bsp::raspberrypi::GPIO_GLOBAL.map_pl011_uart();
    
    PL011_UART.init();
    
    PL011_UART.write_str("Raspberry Pi 3\n");

    loop {
        let c = PL011_UART.read_char();
        PL011_UART.write_char(c);
    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    PL011_UART.write_str("uWu panic!\n");
    loop {

    }
}