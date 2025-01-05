use tock_registers::{interfaces::{Readable, Writeable}, register_structs, registers::{ReadOnly, ReadWrite}};

use crate::bsp::device_driver::common::MMIODerefWrapper;


register_structs! {
    #[allow(non_snake_case)]
    pub VideoCoreMailbox {
        (0x00 => Mbox_Read: ReadOnly<u32>),
        (0x04 => _reserved1),
        (0x14 => Mbox_Poll),
        (0x18 => Mbox_Sender),
        (0x1C => Mbox_Status: ReadOnly<u32>),
        (0x20 => Mbox_Config),
        (0x24 => Mbox_Write: ReadWrite<u32>),
        (0x28 => @END),
    }
}

type Registers = MMIODerefWrapper<VideoCoreMailbox>;

pub struct MailboxInterface {
    registers: Registers,
}

impl MailboxInterface {
    pub const fn new(mmio_start_address: usize) -> Self {
        unsafe {
            Self {
                registers: Registers::new(mmio_start_address),
            }
        }
    }

    pub fn call(&self, channel: u8, mailbox: &[u32]) -> u32 {

        let address = &mailbox[0] as *const u32 as u32 | channel as u32;

        while self.registers.Mbox_Status.get() == 0x80000000 {}

        self.registers.Mbox_Write.set(address);

        while self.registers.Mbox_Status.get() == 0x40000000 {}

        let mailbox_response = self.registers.Mbox_Read.get();
        mailbox_response
    }
}