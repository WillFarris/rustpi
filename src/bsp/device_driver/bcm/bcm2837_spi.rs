use core::fmt::{self, Write};
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::{console, synchronization::FakeLock};
use crate::bsp::device_driver::common::MMIODerefWrapper;

register_bitfields! {
    u32,
    AUXSPI_CNTL0 [
        ShiftLength OFFSET(0) NUMBITS(6) [],
        ShiftMSFirst OFFSET(6) NUMBITS(1) [
            MS = 1,
            LS = 0,
        ],
        InvertSPIClk OFFSET(7) NUMBITS(1) [],
        OutRising OFFSET(8) NUMBITS(1) [
            Rising = 1,
            Falling = 0,
        ],
        ClearFIFOs OFFSET(9) NUMBITS(1) [],
        InRising OFFSET(10) NUMBITS(1) [
            Rising = 1,
            Falling = 0,
        ],
        Enable OFFSET(11) NUMBITS(1) [],
        DOUTHoldTime OFFSET(12) NUMBITS(2) [
            NoExtra = 0b00,
            OneExtra = 0b01,
            FourExtra = 0b10,
            SevenExtra = 0b11,
        ],
        VariableWidth OFFSET(14) NUMBITS(1) [
            SLFromFIFO = 1,
            SLFromRegs = 0,
        ],
        VariableCS OFFSET(15) NUMBITS(1) [
            CSFromFIFO = 1,
            CSFromRegs = 0,
        ],
        PostInputMode OFFSET(16) NUMBITS(1) [],
        ChipSelects OFFSET(17) NUMBITS(3) [],
        Speed OFFSET(20) NUMBITS(12) [],
    ],

    AUXSPI_CNTL1 [
        KeepInput OFFSET(0) NUMBITS(1) [
            ClearedBeforeTxn = 0,
            ConcatWithOldData = 1,
        ],
        ShiftMSFirst OFFSET(1) NUMBITS(1) [
            MS = 1,
            LS = 0,
        ],
        //RES OFFSET(2) NUMBITS(4) [],
        DoneIRQ OFFSET(6) NUMBITS(1) [],
        TxEmptyIRQ OFFSET(7) NUMBITS(1) [],
        CSHighTime OFFSET(8) NUMBITS(3) [],
    ],

    /*AUXSPI_STAT [
        BitCount OFFSET(0) NUMBITS(6) [],
        Busy OFFSET(6) NUMBITS() [],
        RxEmpty OFFSET() NUMBITS(2) [],
        TxEmpty OFFSET() NUMBITS() [],
        TxFull OFFSET() NUMBITS() [],
        RxFIFOLevel OFFSET() NUMBITS() [],
        TxFIFOLevel OFFSET() NUMBITS() [],
    ],*/
    // AUXSPI_PEEK
    // AUXSPI_IO
    // AUXSPI_TXHOLD

}

register_structs! {
    #[allow(non_snake_case)]
    pub SpiRegisters {
        (0x00 => AUXSPI_CNTL0: ReadWrite<u32, AUXSPI_CNTL0::Register>),
        (0x04 => @END),
    }
}

type Registers = MMIODerefWrapper<SpiRegisters>;