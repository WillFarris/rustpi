#![no_main]
#![no_std]
#![allow(unstable_features)]
#![feature(format_args_nl)]
#![feature(allocator_api)]

mod utils;
mod start;
mod memory;
mod bsp;
mod synchronization;
mod print;
mod console;
mod exception;
mod scheduler;
mod tasks;

extern crate alloc;

use utils::{get_el, get_core};

use bsp::system_timer;

extern "C" {
    fn _core_execute(core: u8, f: extern fn());
}

extern "C" fn _init_core() {
    memory::mmu::enable_mmu_and_caching();
    scheduler::PTABLE.init_core();
    //bsp::raspberrypi::QA7_REGS.init_core_timer();
    exception::irq_enable();
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::driver::init();

    crate::memory::mmu::enable_mmu_and_caching();

    println!("\nBooting Raspberry Pi 3 in EL{}\n", get_el());

    bsp::raspberrypi::QA7_REGS.init_core_timer();

    scheduler::PTABLE.init_core();

    /*unsafe {
        for i in 0..3 {
            core_execute(i+1, init_core);
        }
    }*/

    tasks::register_cmd("ptable", || {
        scheduler::PTABLE.print();
    });
    
    tasks::register_cmd("test_loop", || {
        for i in 0..10 {
            system_timer().wait_for_ms(1000);
            println!("loop {}", i+1);
        }
    });

    tasks::register_cmd("uptime", || {
        let raw_time = system_timer().get_ticks();
        let ms = raw_time / 1000;
        let s = ms / 1000;
        let m = s / 60;
        let h = m / 60;
        let d = h / 24;
        crate::println!("uptime: {}d {}h {}m {}s", d, h % 24, m % 60, s % 60);
    });

    scheduler::PTABLE.new_process("shell", tasks::shell::shell);
    
    loop {
        scheduler::PTABLE.schedule();
    }
}


#[panic_handler]
pub unsafe fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    println!(" ~ UwU we panic now ~\n{:?}", panic_info);
    loop {

    }
}