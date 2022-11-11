#![no_main]
#![no_std]
#![feature(format_args_nl)]

mod bsp;
mod exception;
mod start;
pub mod print;
pub mod console;

extern "C" {
    fn get_el() -> u64;
    fn get_core() -> u64;
    //fn core_execute(core: u64, f: u64);
}

#[no_mangle]
pub fn kernel_main() -> ! {
    //bsp::memory::mmu::init();

    bsp::raspberrypi::uart_init();
    let (el, core) = unsafe {
        (get_el(), get_core())
    };

    println!("\n\rRaspberry Pi 3\n\rIn EL{} on core {}\n\r", el, core);

    loop {
        unsafe {
            let c = bsp::raspberrypi::MINI_UART_GLOBAL.read_char();
            bsp::raspberrypi::MINI_UART_GLOBAL.putc(c);
        }
    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    println!(" ~ UwU we panic now ~");
    loop {

    }
}