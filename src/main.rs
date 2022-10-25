#![no_main]
#![no_std]
#![feature(format_args_nl)]

use core::fmt::Write;

use bsp::raspberrypi::MINI_UART_GLOBAL;

mod bsp;
mod exception;
mod start;
pub mod print;
pub mod console;

extern "C" {
    fn get_el() -> u64;
    fn get_core() -> u64;
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();
    let (el, core) = unsafe {
        (get_el(), get_core())
    };
    
    unsafe {
        MINI_UART_GLOBAL.write_fmt(format_args!("EL{} | core {}", el, core)).unwrap();
    }

    //println!("\n\rRaspberry Pi 3\n\rIn EL{} on core {}\n\r", el, core);

    loop {
        unsafe {
            let c = MINI_UART_GLOBAL.read_char();
            MINI_UART_GLOBAL.putc(c);
        }
    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    MINI_UART_GLOBAL.write_str(" ~ UwU we panic now ~").unwrap_or(());
    loop {

    }
}