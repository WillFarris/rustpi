use crate::synchronization::{FakeLock, interface::Mutex};
use crate::console::null_console::NULL_CONSOLE;

mod null_console;

pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }

    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }
        fn clear_rx(&self);
    }

    pub trait ReadWrite: Write + Read {}
}

static CUR_CONSOLE: FakeLock<&'static (dyn interface::ReadWrite + Sync)> = FakeLock::new(&NULL_CONSOLE);

pub fn register_console(new_console: &'static (dyn interface::ReadWrite + Sync)) {
    let mut lock = CUR_CONSOLE.lock().unwrap();
    *lock = new_console;
}

pub fn console() -> &'static dyn interface::ReadWrite {
    *CUR_CONSOLE.lock().unwrap()
}