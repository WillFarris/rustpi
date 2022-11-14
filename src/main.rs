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
use crate::synchronization::interface::Mutex;

extern "C" {
    fn vectors();
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::memory::mmu::init();

    bsp::raspberrypi::uart_init();

    unsafe {
        crate::exception::init_vectors(vectors as *const () as u64);

        core::arch::asm!("msr daifclr, 0b10");
    }

    unsafe {
        //let qa7 = &mut bsp::raspberrypi::QA7_REGS;
        //qa7.init_core_timer(0, 1000);

        bsp::raspberrypi::QA7_REGS.init_core_timer(0, 1);
    }

    let el = get_el();
    let core = get_core();

    println!("\n\rRaspberry Pi 3\n\rIn EL{} on core {}\n\r", el, core);

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