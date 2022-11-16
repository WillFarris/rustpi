#![no_main]
#![no_std]
#![feature(format_args_nl)]

mod bsp;
mod exception;
mod start;
mod print;
mod console;
mod synchronization;
mod utils;

use utils::{get_el, get_core};

extern "C" {
    fn vectors();
    fn core_execute(core: u8, f: fn());
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();
    println!("\n[core {}] Raspberry Pi 3 in EL{}", get_core(), get_el());

    unsafe {
        core::arch::asm!("msr daifclr, #2");
        bsp::raspberrypi::QA7_REGS.init_core_timer(0, 1);
    }
    println!("[core {}] Initialized core timer", get_core());

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