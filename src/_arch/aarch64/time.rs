use core::time::Duration;

use aarch64_cpu::{asm, registers::{CNTFRQ_EL0, CNTPCT_EL0}};
use tock_registers::interfaces::Readable;

const NANOSEC_PER_SEC: u64 = 1_000_000_000;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct GenericTimerCounterValue(u64);


// Ignoring some safety things in the tutorial for brevity
impl From<GenericTimerCounterValue> for Duration {
    fn from(counter_value: GenericTimerCounterValue) -> Self {
        if counter_value.0 == 0 {
            return Duration::ZERO;
        }

        let frequency = arch_timer_counter_frequency();

        let secs = counter_value.0 / frequency;

        // The subsequent division ensures the result fits into u32, since the max result is smaller
        // than NANOSEC_PER_SEC. Therefore, just cast it to u32 using `as`.
        let sub_second_counter_value = counter_value.0 % frequency;
        let nanos =  (sub_second_counter_value * NANOSEC_PER_SEC) / frequency;

        Duration::new(secs, nanos as u32)
    }
}

fn arch_timer_counter_frequency() -> u64 {
    CNTFRQ_EL0.get().try_into().unwrap()
}


pub fn resolution() -> Duration {
    Duration::from(GenericTimerCounterValue(1))
}

pub fn uptime() -> Duration {
    asm::barrier::isb(asm::barrier::SY);
    let cnt = CNTPCT_EL0.get();
    GenericTimerCounterValue(cnt).into()
}

pub fn spin_for(_duration: Duration) {
    todo!()
}