# rustpi
Programming Raspberry Pi 3 in Rust on bare metal! This is a Rust re-write of my C project [rpios](https://github.com/WillFarris/rpios).

# Build and Run
Requires the [Rust toolchain](https://rustup.rs) and Make. 

# What's Working
So far, barely anything. There is a mini UART driver but that's about it. The following are planned and just need to be ported from my previous project:
* Timer interrupts
* SMP scheduler
* Identity-mapped MMU
