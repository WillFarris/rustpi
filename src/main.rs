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

use synchronization::SpinLock;
use utils::{get_el, get_core};

use crate::synchronization::interface::Mutex;

extern "C" {
    fn vectors();
    fn core_execute(core: u8, f: fn());
}

#[no_mangle]
static mut TEST_LOCK: SpinLock<usize> = SpinLock::new(0usize);

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();

    unsafe {
        crate::exception::init_vectors(vectors as *const () as u64);
        core::arch::asm!("msr daifset, 0b10");
        bsp::raspberrypi::QA7_REGS.init_core_timer(0, 1);
    }

    let el = get_el();
    let core = get_core();

    println!("\n\rRaspberry Pi 3\n\rIn EL{} on core {}\n\r", el, core);

    unsafe {
        let mut data = TEST_LOCK.lock().unwrap();
        *data = 1;
    }

    unsafe {
        for i in 1..=3 {
            core_execute(i, || {
                for _ in 0..1 {
                    let mut data = TEST_LOCK.lock().unwrap();
                    *data += 1;
                }
            });
        }
    }
    
    utils::spin_for_cycles(100000000);

    unsafe {
        let data = TEST_LOCK.lock().unwrap();
        println!("TEST_LOCK: {}", *data);
    }

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