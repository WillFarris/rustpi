#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]

mod start;

#[no_mangle]
pub unsafe fn kernel_main() {
    let mut x = 0;
    loop {
        x += 1;
    }
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {

    }
}