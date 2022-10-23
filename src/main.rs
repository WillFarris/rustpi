#![no_main]
#![no_std]
#![feature(format_args_nl)]

use bsp::raspberrypi::MINI_UART_GLOBAL;

mod bsp;
mod exception;
mod start;
pub mod print;
pub mod console;

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();
    
    println!("\n\rRaspberry Pi 3");

    loop {
        unsafe {
            let c = MINI_UART_GLOBAL.read_char();
            MINI_UART_GLOBAL.putc(c);
        }
    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    println!(" ~ UwU we panic now ~");
    loop {

    }
}