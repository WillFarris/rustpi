use core::fmt::{self, Write};
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::{console, synchronization::{SpinLock, FakeLock}};
use super::common::MMIODerefWrapper;

register_structs! {
    #[allow(non_snake_case)]
    pub SysTimerRegisters {
        (0x00 => CS: ReadWrite<u32>),
        (0x04 => CLO: ReadWrite<u32>),
        (0x08 => CHI: ReadWrite<u32>),
        (0x0C => C0: ReadWrite<u32>),
        (0x10 => C1: ReadWrite<u32>),
        (0x14 => C2: ReadWrite<u32>),
        (0x18 => C3: ReadWrite<u32>),
        (0x1C => @END),
    }
}

type Registers = MMIODerefWrapper<SysTimerRegisters>;

pub struct SystemTimer {
    registers: Registers,
}

impl SystemTimer {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
        }
    }

    fn get_ticks(&self) -> usize {
        let clo = self.registers.CLO.get() as usize;
        let chi = self.registers.CHI.get() as usize;
        clo | (chi << 32)
    }

    pub fn wait_for_ms(&self, ms: usize) {
        let start = self.get_ticks();
        while self.get_ticks() < start + ms*1000 {}        
    }
}