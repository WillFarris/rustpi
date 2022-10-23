use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use super::common::MMIODerefWrapper;

register_bitfields! {
    u32,
    AUXIRQ [
        MiniUART OFFSET(0) NUMBITS(1) [
            NoInterruptPending = 0,
            InterruptPending = 1,
        ],
        SPI1 OFFSET(1) NUMBITS(1) [
            NoInterruptPending = 0,
            InterruptPending = 1,
        ],
        SPI2 OFFSET(2) NUMBITS(1) [
            NoInterruptPending = 0,
            InterruptPending = 1,
        ],
    ],

    AUXENB [
        MiniUART OFFSET(0) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0,
        ],
        SPI1 OFFSET(1) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0,
        ],
        SPI2 OFFSET(2) NUMBITS(1) [
            Enabled = 1,
            Disabled = 0,
        ]
    ],

    MU_IO [
        BYTE OFFSET(0) NUMBITS(8),
    ],

    MU_IER [
        ENABLET OFFSET(0) NUMBITS(1),
        ENABLER OFFSET(1) NUMBITS(1),
    ],

    MU_IIR [
        INTPENDING OFFSET(0) NUMBITS(1),
        INTID_FIFOCLR OFFSET(1) NUMBITS(2),
    ],
    MU_LCR [
        DATASIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11,
        ],
        BREAK OFFSET(6) NUMBITS(1) [
            TxPulledLowCont = 1
        ],
        DLAB OFFSET(7) NUMBITS(1) []
    ],
    MU_MCR [
        RTS OFFSET(1) NUMBITS(1),
    ],
    MU_LSR [
            DATAREADY OFFSET(0) NUMBITS(1),
            RXOVERRUN OFFSET(1) NUMBITS(1),
            TXEMPTY OFFSET(5) NUMBITS(1),
            TXIDLE OFFSET(6) NUMBITS(1),
    ],
    /*MU_MSR [
        
    ],
    MU_SCRATCH [
        
    ],*/
    MU_CNTL [
        RXEN OFFSET(0) NUMBITS(1) [],
        TXEN OFFSET(1) NUMBITS(1) [],
        RXAUTOEN OFFSET(2) NUMBITS(1) [],
        TXAUTOEN OFFSET(3) NUMBITS(1) [],
        RTSAUTOLVL OFFSET(4) NUMBITS(2) [
            ThreeEmpty = 0b00,
            TwoEmpty = 0b01,
            OneEmpty = 0b10,
            FourEmpty = 0b11,
        ],
        RTSASSERTLVL OFFSET(6) NUMBITS(1) [],
        CTSASSERTLVL OFFSET(7) NUMBITS(1) [],
    ],
    MU_STAT [
        SYMAVAIL OFFSET(0) NUMBITS(1) [],
        SPACEAVAIL OFFSET(1) NUMBITS(1) [],
        RXIDLE OFFSET(2) NUMBITS(1) [],
        TXIDLE OFFSET(3) NUMBITS(1) [],
        RXOVRUN OFFSET(4) NUMBITS(1) [],
        TXFIFOFULL OFFSET(5) NUMBITS(1) [],
        RTSSTAT OFFSET(6) NUMBITS(1) [],
        CTSLINE OFFSET(7) NUMBITS(1) [],
        TXFIFOEMPTY OFFSET(8) NUMBITS(1) [],
        TXDONE OFFSET(9) NUMBITS(1) [],
        RXFIFOLVL OFFSET(16 ) NUMBITS(4) [],
        TXFIFOLVL OFFSET(24) NUMBITS(4) [],
    ],
    AUX_MU_BAUD [
        BAUDRATE OFFSET(0) NUMBITS(16) [],
    ],
}
        

register_structs! {
    #[allow(non_snake_case)]
    pub AuxRegisters {
        (0x00 => AUXIRQ: ReadOnly<u32, AUXIRQ::Register>),
        (0x04 => AUXENB: ReadWrite<u32, AUXENB::Register>),
        (0x08 => _reserved1),
        (0x40 => MU_IO: ReadWrite<u32, MU_IO::Register>),
        (0x44 => MU_IER: ReadWrite<u32>),
        (0x48 => MU_IIR: ReadWrite<u32>),
        (0x4c => MU_LCR: ReadWrite<u32>),
        (0x50 => MU_MCR: ReadWrite<u32>),
        (0x54 => MU_LSR: ReadWrite<u32, MU_LSR::Register>),
        (0x58 => MU_MSR: ReadWrite<u32>),
        (0x5c => MU_SCRATCH: ReadWrite<u32>),
        (0x60 => MU_CNTL: ReadWrite<u32>),
        (0x64=> MU_STAT: ReadWrite<u32>),
        (0x68 => MU_BAUD: ReadWrite<u32>),
        (0x6c => @END),
    }
}

type Registers = MMIODerefWrapper<AuxRegisters>;

pub struct MiniUart {
    registers: Registers
}

impl MiniUart {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
        }
    }

    #[no_mangle]
    pub fn init(&mut self) {
        self.registers.AUXENB.set(AUXENB::MiniUART::Enabled.value);
        self.registers.MU_CNTL.set(0);
        self.registers.MU_IER.set(0);
        self.registers.MU_LCR.set(MU_LCR::DATASIZE::EightBit.value);
        self.registers.MU_MCR.set(0);
        self.registers.MU_BAUD.set(270);
        self.registers.MU_CNTL.set((MU_CNTL::RXEN::SET + MU_CNTL::TXEN::SET + MU_CNTL::RXAUTOEN::SET + MU_CNTL::RXAUTOEN::SET).value);
    }

    pub fn write_char(&self, c: char) {
        if c == '\r' {
            self.write_char('\n');
        }

        while !self.registers.MU_LSR.matches_all(MU_LSR::TXEMPTY::SET) {
            cortex_a::asm::nop();
        }
        self.registers.MU_IO.set(c as u32);
    }
    
    pub fn read_char(&self) -> char {
        while self.registers.MU_LSR.matches_all(MU_LSR::DATAREADY::CLEAR) {
            cortex_a::asm::nop();
        }
        char::from_u32(self.registers.MU_IO.read(MU_IO::BYTE)).unwrap()
    }
    
    pub fn write_str(&self, string: &str) {
         for c in string.chars() {
             self.write_char(c);
         }
     }
}