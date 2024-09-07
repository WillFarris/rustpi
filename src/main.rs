#![no_main]
#![no_std]
#![allow(unstable_features)]
#![feature(format_args_nl)]

mod bsp;
mod console;
mod exception;
mod memory;
mod print;
mod start;
mod synchronization;
mod tasks;
mod time;
mod utils;

use time::time_manager;
use utils::{get_core, get_el};

#[no_mangle]
pub fn kernel_main() -> ! {

    crate::memory::mmu::map_translation_table();
    crate::memory::mmu::enable_mmu_and_caching();

    bsp::driver::init();

    info!("Booting Raspberry Pi 3 in EL{}", get_el());
    info!("Timer resolution: {}ns", time_manager().resolution().as_nanos());

    bsp::raspberrypi::QA7_REGS.init_core_timer();

    bsp::memory::virt_mem_layout().print_layout_info();

    tasks::shell::shell();

    loop {}
}

#[panic_handler]
pub unsafe fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    println!(" ~ UwU we panic now ~\n{:?}", panic_info);
    loop {}
}
