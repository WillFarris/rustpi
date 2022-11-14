use aarch64_cpu::registers::{CNTFRQ_EL0, CNTP_TVAL_EL0, CNTP_CTL_EL0};
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
    interfaces::{Readable, Writeable},
};

use super::common::MMIODerefWrapper;
use crate::synchronization::{SpinLock, interface::Mutex};

register_bitfields! {
    u32,

    ControlRegister [
        ClockSource OFFSET(8) NUMBITS(1) [
            APB = 1,
            Crystal = 0
        ],

        ClockIncrement OFFSET(9) NUMBITS(1) [
            One = 0,
            Two = 1
        ],
    ],

    GPUInterruptRouting [
        GPUIRQRouting OFFSET(0) NUMBITS(2) [
            Core0 = 0b00,
            Core1 = 0b01,
            Core2 = 0b10,
            Core3 = 0b11
        ],
        GPUFIQRouting OFFSET(2) NUMBITS(2) [
            Core0 = 0b00,
            Core1 = 0b01,
            Core2 = 0b10,
            Core3 = 0b11
        ],
    ],

    PMUInterruptRouting [
        nPMUIRQ0 OFFSET(0) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        nPMUIRQ1 OFFSET(1) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        nPMUIRQ2 OFFSET(2) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        nPMUIRQ3 OFFSET(3) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],

        nPMUFIQ0 OFFSET(4) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        nPMUFIQ1 OFFSET(5) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        nPMUFIQ2 OFFSET(6) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        nPMUFIQ3 OFFSET(7) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],

    ],

    CoreTimerInterruptControl [
        nCNTPSIRQ OFFSET(0) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        nCNTPNSIRQ OFFSET(1) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        nCNTHPIRQ OFFSET(2) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        nCNTVIRQ OFFSET(3) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],

        nCNTPSFIQ OFFSET(4) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        nCNTPNSFIQ OFFSET(5) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        nCNTHPFIQ OFFSET(6) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        nCNTVFIQ OFFSET(7) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],

    ],

    CoreIRQSource [
        CNTPSIRQ OFFSET(0) NUMBITS(1) [],
        CNTPNSIRQ OFFSET(1) NUMBITS(1) [],
        CNTHPIRQ OFFSET(2) NUMBITS(1) [],
        CNTVIRQ OFFSET(3) NUMBITS(1) [],
        Mailbox0 OFFSET(4) NUMBITS(1) [],
        Mailbox1 OFFSET(5) NUMBITS(1) [],
        Mailbox2 OFFSET(6) NUMBITS(1) [],
        Mailbox3 OFFSET(7) NUMBITS(1) [],
        GPUInterrupt OFFSET(8) NUMBITS(1) [],
        PMUInterrupt OFFSET(9) NUMBITS(1) [],
        AXIInterrupt OFFSET(10) NUMBITS(1) [],
        LocalTimerInterrupt OFFSET(11) NUMBITS(1) [],
    ],

    CoreMailboxInterruptControl [
        Mailbox0IRQ OFFSET(0) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        Mailbox1IRQ OFFSET(1) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        Mailbox2IRQ OFFSET(2) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],
        Mailbox3IRQ OFFSET(3) NUMBITS(1) [
            IRQEnabled = 1,
            IRQDisabled = 0
        ],

        Mailbox0FIQ OFFSET(4) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        Mailbox1FIQ OFFSET(5) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        Mailbox2FIQ OFFSET(6) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],
        Mailbox3FIQ OFFSET(7) NUMBITS(1) [
            FIQEnabled = 1,
            FIQDisabled = 0
        ],

    ],

    AxiOutstandingCounters [
        OutstandingReads OFFSET(0) NUMBITS(10),
        OutstandingWrites OFFSET(15) NUMBITS(2),
    ],

    AxiOutstandingInterrupt [
        TimeoutMS24Bits OFFSET(0) NUMBITS(20) [],
        Enable OFFSET(20) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0,
        ],
    ],

    LocalTimerControlStatus [
        ReloadValue OFFSET(0) NUMBITS(28) [],
        TimerEnabled OFFSET(28) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0,
        ],
        InterruptEnabled OFFSET(29) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0,
        ],
        InterruptFlag OFFSET(31) NUMBITS(1) [],
    ],

    LocalTimerIRQClearReload [
        Reloaded OFFSET(30) NUMBITS(1) [],
        InterruptFlagClear OFFSET(31) NUMBITS(1) [],
    ],



}


register_structs! {
    #[allow(non_snake_case)]
    pub QA7RegisterBlock {
        (0x00 => ControlRegister: ReadWrite<u32, ControlRegister::Register>),
        (0x04 => _reserved1),
        (0x08 => CoreTimerPrescaler: ReadWrite<u32>),
        (0x0C => GPUInterruptRouting: ReadWrite<u32, GPUInterruptRouting::Register>),
        (0x10 => PMUInterruptRoutingWriteSet: WriteOnly<u32, PMUInterruptRouting::Register>),
        (0x14 => PMUInterruptRoutingWriteClear: WriteOnly<u32, PMUInterruptRouting::Register>),
        (0x18 => _reserved2),
        (0x1C => CoreTimerAccesLS: ReadWrite<u32>),
        (0x20 => CoreTimerAccessMS: ReadWrite<u32>),
        (0x24 => LocalInterrupt0Routing: ReadWrite<u32>),
        (0x28 => _reserved3),
        (0x2C => AxiOutstandingCounters: ReadWrite<u32, AxiOutstandingCounters::Register>),
        (0x30 => AxiOutstandingInterrupt: ReadWrite<u32, AxiOutstandingInterrupt::Register>),
        (0x34 => LocalTimerControlStatus: ReadWrite<u32, LocalTimerControlStatus::Register>),
        (0x38 => LocalTimerIRQClearReload: WriteOnly<u32, LocalTimerIRQClearReload::Register>),
        (0x3C => _reserved4),
        (0x40 => Core0TimerInterruptControl: ReadWrite<u32, CoreTimerInterruptControl::Register>),
        (0x44 => Core1TimerInterruptControl: ReadWrite<u32, CoreTimerInterruptControl::Register>),
        (0x48 => Core2TimerInterruptControl: ReadWrite<u32, CoreTimerInterruptControl::Register>),
        (0x4C => Core3TimerInterruptControl: ReadWrite<u32, CoreTimerInterruptControl::Register>),
        (0x50 => Core0MailboxInterruptControl: ReadWrite<u32, CoreMailboxInterruptControl::Register>),
        (0x54 => Core1MailboxInterruptControl: ReadWrite<u32, CoreMailboxInterruptControl::Register>),
        (0x58 => Core2MailboxInterruptControl: ReadWrite<u32, CoreMailboxInterruptControl::Register>),
        (0x5C => Core3MailboxInterruptControl: ReadWrite<u32, CoreMailboxInterruptControl::Register>),
        (0x60 => Core0IRQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x64 => Core1IRQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x68 => Core2IRQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x6C => Core3IRQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x70 => Core0FIQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x74 => Core1FIQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x78 => Core2FIQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x7C => Core3FIQSource: ReadOnly<u32, CoreIRQSource::Register>),
        (0x80 => Core0Mailbox0Set: WriteOnly<u32>),
        (0x84 => Core0Mailbox1Set: WriteOnly<u32>),
        (0x88 => Core0Mailbox2Set: WriteOnly<u32>),
        (0x8C => Core0Mailbox3Set: WriteOnly<u32>),
        (0x90 => Core1Mailbox0Set: WriteOnly<u32>),
        (0x94 => Core1Mailbox1Set: WriteOnly<u32>),
        (0x98 => Core1Mailbox2Set: WriteOnly<u32>),
        (0x9C => Core1Mailbox3Set: WriteOnly<u32>),
        (0xA0 => Core2Mailbox0Set: WriteOnly<u32>),
        (0xA4 => Core2Mailbox1Set: WriteOnly<u32>),
        (0xA8 => Core2Mailbox2Set: WriteOnly<u32>),
        (0xAC => Core2Mailbox3Set: WriteOnly<u32>),
        (0xB0 => Core3Mailbox0Set: WriteOnly<u32>),
        (0xB4 => Core3Mailbox1Set: WriteOnly<u32>),
        (0xB8 => Core3Mailbox2Set: WriteOnly<u32>),
        (0xBC => Core3Mailbox3Set: WriteOnly<u32>),
        (0xC0 => Core0Mailbox0RdClr: ReadOnly<u32>),
        (0xC4 => Core0Mailbox1RdClr: ReadOnly<u32>),
        (0xC8 => Core0Mailbox2RdClr: ReadOnly<u32>),
        (0xCC => Core0Mailbox3RdClr: ReadOnly<u32>),
        (0xD0 => Core1Mailbox0RdClr: ReadOnly<u32>),
        (0xD4 => Core1Mailbox1RdClr: ReadOnly<u32>),
        (0xD8 => Core1Mailbox2RdClr: ReadOnly<u32>),
        (0xDC => Core1Mailbox3RdClr: ReadOnly<u32>),
        (0xE0 => Core2Mailbox0RdClr: ReadOnly<u32>),
        (0xE4 => Core2Mailbox1RdClr: ReadOnly<u32>),
        (0xE8 => Core20Mailbox2RdClr: ReadOnly<u32>),
        (0xEC => Core2Mailbox3RdClr: ReadOnly<u32>),
        (0xF0 => Core3Mailbox0RdClr: ReadOnly<u32>),
        (0xF4 => Core3Mailbox1RdClr: ReadOnly<u32>),
        (0xF8 => Core3Mailbox2RdClr: ReadOnly<u32>),
        (0xFC => Core3Mailbox3RdClr: ReadOnly<u32>),
        (0x100 => @END),
    }
}

type Registers = MMIODerefWrapper<QA7RegisterBlock>;

pub struct QA7Registers {
    inner: SpinLock<QA7RegistersInner>,
}

impl QA7Registers {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: SpinLock::new(QA7RegistersInner::new(mmio_start_addr))
        }
    }

    pub fn init_core_timer(&mut self, core: u8, freq_divider: u64) {
        let mut qa7 = self.inner.lock().unwrap();
        qa7.init_core_timer(core, freq_divider);
    }
}

struct QA7RegistersInner {
    registers: Registers
}

impl QA7RegistersInner {
    const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
        }
    }

    fn init_core_timer(&mut self, core: u8, freq_divider: u64) {
        let freq = CNTFRQ_EL0.get();
        let timer = freq / freq_divider;
        CNTP_TVAL_EL0.set(timer);
        CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);
        match core {
            0 => {
                self.registers.Core0TimerInterruptControl.write(
                    CoreTimerInterruptControl::nCNTPSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTPNSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTHPIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTVIRQ::IRQEnabled
                );
            }
            1 => {
                self.registers.Core1TimerInterruptControl.write(
                    CoreTimerInterruptControl::nCNTPSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTPNSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTHPIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTVIRQ::IRQEnabled
                );
            }
            2 => {
                self.registers.Core2TimerInterruptControl.write(
                    CoreTimerInterruptControl::nCNTPSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTPNSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTHPIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTVIRQ::IRQEnabled
                );
            }
            3 => {
                self.registers.Core3TimerInterruptControl.write(
                    CoreTimerInterruptControl::nCNTPSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTPNSIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTHPIRQ::IRQEnabled +
                    CoreTimerInterruptControl::nCNTVIRQ::IRQEnabled
                );
            }
            _ => {
                panic!("Can't enable core timer on invalid core")
            }
        }
    }

}
