#![no_main]
#![no_std]
#![feature(format_args_nl)]

use bsp::raspberrypi::MINI_UART_GLOBAL;

use crate::bsp::drivers::bcm2xxx_gpio::spin_for_cycles;

mod bsp;
mod exception;
mod start;
pub mod print;
pub mod console;

extern "C" {
    fn get_el() -> u64;
    fn get_core() -> u64;
    fn core_execute(core: u64, f: u64);
}

const COUNT_ITER_PER_CORE: usize = 100;
static mut COUNT: u64 = 0;

#[no_mangle]
unsafe fn test_multicore_func() {
    println!("Hello from core {} in EL{}!\n", get_core(), get_el());
    for _ in 0..COUNT_ITER_PER_CORE {
        COUNT += 1;
    }
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();
    let (el, core) = unsafe {
        (get_el(), get_core())
    };

    println!("\n\rRaspberry Pi 3\n\rIn EL{} on core {}\n\r", el, core);

    unsafe {
        core_execute(1, test_multicore_func as u64);
        core_execute(2, test_multicore_func as u64);
        core_execute(3, test_multicore_func as u64);
    }

    spin_for_cycles(100000);
    unsafe {
        println!("COUNT should be {} and is {}", 3*COUNT_ITER_PER_CORE, COUNT);
    }

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