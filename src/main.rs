#![no_main]
#![no_std]

mod start;

//use cortex_a::{asm, registers::*};
//use tock_registers::interfaces::{Readable, Writeable};

#[no_mangle]
pub unsafe fn kernel_main() {

    
    loop {

    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {

    }
}