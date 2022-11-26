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

extern "C" {
    fn core_execute(core: u8, f: fn());
}

#[no_mangle]
pub fn kernel_main() -> ! {
    bsp::raspberrypi::uart_init();
    println!("\n[core {}] Raspberry Pi 3 in EL{}", get_core(), get_el());

    bsp::raspberrypi::QA7_REGS.init_core_timer();
    exception::irq_enable();

    scheduler::PTABLE.init_core();

    unsafe {
        for i in 0..3 {
            core_execute(i+1, || {
                bsp::memory::mmu::init();
                scheduler::PTABLE.init_core();
                bsp::raspberrypi::SYSTEM_TIMER.wait_for_ms(get_core() as usize * 100);
                //bsp::raspberrypi::QA7_REGS.init_core_timer();
                //exception::irq_enable();
            });
        }
    }

    bsp::raspberrypi::SYSTEM_TIMER.wait_for_ms(100);
    
    scheduler::PTABLE.new_process("test", || {
      println!("\n[core {}] PTABLE while inside a process:", get_core());
      scheduler::PTABLE.print();
      scheduler::PTABLE.schedule();
      loop {}
    });
    
    println!("\nPTABLE before task runs:");
    scheduler::PTABLE.print();

    scheduler::PTABLE.schedule();
    
    println!("\nPTABLE after task runs:");
    scheduler::PTABLE.print();
    
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