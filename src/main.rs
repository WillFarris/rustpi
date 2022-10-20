#![no_main]
#![no_std]

mod start;

mod drivers;
use drivers::bcm2xxx_gpio::*;

const PBASE: u64 = 0x3F000000;
const AUX_REGS_ADDR: u64 = PBASE + 0x00215000;

#[no_mangle]
pub unsafe fn kernel_main() {
    
    let s = "hello world\n";
    for c in s.chars() {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }
    }

    
    loop {

    }
}

#[panic_handler]
pub unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {

    }
}