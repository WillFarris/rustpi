#![no_main]
#![no_std]
#![feature(format_args_nl)]
#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]

mod bsp;
mod exception;
mod start;
mod print;
mod console;
mod synchronization;
mod utils;
mod scheduler;

extern crate alloc;

use utils::{get_el, get_core};

use crate::bsp::raspberrypi::SYSTEM_TIMER;

extern "C" {
    fn core_execute(core: u8, f: fn());
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();
    println!("\n[core {}] Raspberry Pi 3 in EL{}", get_core(), get_el());

    
    loop {
        let c = console::console().read_char();
        console::console().write_char(c);
    }
}

#[panic_handler]
pub unsafe fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    println!(" ~ UwU we panic now ~\n{:?}", panic_info);
    loop {

    }
}