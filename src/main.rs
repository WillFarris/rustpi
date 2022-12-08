#![no_main]
#![no_std]
#![feature(format_args_nl)]
#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]

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

use crate::bsp::SYSTEM_TIMER;

extern "C" {
    fn core_execute(core: u8, f: fn());
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::driver::init();

    
    println!("\nBooting Raspberry Pi 3 in EL{}\n", get_el());

    bsp::raspberrypi::QA7_REGS.init_core_timer();

    scheduler::PTABLE.init_core();

    unsafe {
        for i in 0..3 {
            core_execute(i+1, || {
                memory::mmu::init();
                scheduler::PTABLE.init_core();
                bsp::raspberrypi::QA7_REGS.init_core_timer();
                exception::irq_enable();
            });
        }
    }

    tasks::register_cmd("ptable", || {
        scheduler::PTABLE.print();
    });
    
    tasks::register_cmd("test_loop", || {
        for i in 0..10 {
            SYSTEM_TIMER.wait_for_ms(1000);
            println!("loop {}", i+1);
        }
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