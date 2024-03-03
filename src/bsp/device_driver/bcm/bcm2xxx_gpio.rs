// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

//! GPIO Driver.

use tock_registers::{
    interfaces::{Writeable, Readable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::bsp::device_driver::common::MMIODerefWrapper;
use crate::utils::spin_for_cycles;

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

// GPIO registers.
//
// Descriptions taken from
// - https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
// - https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf
register_bitfields! {
    u32,

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100,  // PL011 UART RX
            AltFunc1 = 0b101,
            AltFunc2 = 0b110,
            AltFunc5 = 0b010,
        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100,  // PL011 UART TX
            AltFunc1 = 0b101,
            AltFunc2 = 0b110,
            AltFunc5 = 0b010,
        ]
    ],

    /// GPIO Pull-up/down Register
    ///
    /// BCM2837 only.
    GPPUD [
        /// Controls the actuation of the internal pull-up/down control line to ALL the GPIO pins.
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    ///
    /// BCM2837 only.
    GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],

    /// GPIO Pull-up / Pull-down Register 0
    ///
    /// BCM2711 only.
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => _reserved3),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),
        (0xE8 => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

#[allow(clippy::upper_case_acronyms)]
pub struct GPIO {
    registers: Registers,
}

impl GPIO {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr)
        }
    }

    fn set_func(&self, pin: u32, func: u32) {
        let bit_start = (pin * 3) % 30;

        let mut selector = self.registers.GPFSEL1.get();
        selector &= !(7 << bit_start);
        selector |= func << bit_start;


        self.registers.GPFSEL1.set(selector);
    }

    pub fn enable_pin(&self, pin: usize) {
        self.registers.GPPUD.set(0);
        spin_for_cycles(2000);
        self.registers.GPPUDCLK0.set(1 << (pin % 32));
        spin_for_cycles(2000);
        self.registers.GPPUD.set(0);
        self.registers.GPPUDCLK0.set(0);
    }

    pub fn init_mini_uart_pins(&self) {
        self.set_func(14, 2);
        self.set_func(15, 2);

        self.enable_pin(14);
        self.enable_pin(15);
    }
}