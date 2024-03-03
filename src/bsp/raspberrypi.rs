use super::device_driver;

pub mod memory;

pub const PBASE_START: usize = 0x3F00_0000;
pub const PBASE_END: usize = 0x4000_FFFF;
pub const GPIO_ADDR: usize = PBASE_START + 0x0020_0000;
pub const AUX_REGS_ADDR: usize = PBASE_START + 0x0021_5000;
const _PL011_UART_ADDR: usize = PBASE_START + 0x0020_1000;
const SYS_TIMER_ADDR: usize = PBASE_START + 0x0000_3000;
const QA7_REGS_ADDR: usize = 0x4000_0000;

pub static GPIO: device_driver::GPIO = unsafe { device_driver::GPIO::new(GPIO_ADDR) };
pub static MINI_UART: device_driver::MiniUart = unsafe { device_driver::MiniUart::new(AUX_REGS_ADDR) };
pub static QA7_REGS: device_driver::QA7Registers = unsafe { device_driver::QA7Registers::new(QA7_REGS_ADDR) };
pub static SYSTEM_TIMER: device_driver::SystemTimer = unsafe { device_driver::SystemTimer::new(SYS_TIMER_ADDR) };

pub mod driver {
    use crate::bsp::raspberrypi::{GPIO, MINI_UART};

    pub fn init() {
        GPIO.init_mini_uart_pins();
        MINI_UART.init();

        crate::console::register_console(&MINI_UART);
    }
}

pub fn system_timer() -> &'static device_driver::SystemTimer {
    &SYSTEM_TIMER
}
