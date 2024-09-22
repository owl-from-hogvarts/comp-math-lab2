//! Routines for managing interrupts.

use core::arch::asm;
use core::prelude::v1::*;

use crate::Register;

/// Helper struct that automatically restores interrupts
/// on drop.
enum InterruptsStatus {
    Enabled,
    Disabled,
}

impl From<bool> for InterruptsStatus {
    fn from(value: bool) -> Self {
        match value {
            true => InterruptsStatus::Enabled,
            false => InterruptsStatus::Disabled,
        }
    }
}

/// Executes a closure, disabling interrupts until its completion.
///
/// Restores interrupts after the closure has completed
/// execution.
#[inline(always)]
pub fn without_interrupts<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let _disabled = InterruptsStatus::disable_safe();
    f()
}

impl InterruptsStatus {
    #[inline(always)]
    pub fn disable_safe() -> InterruptsStatus {
        use crate::cores::atmega328::SREG;
        let status: InterruptsStatus = SREG::is_set(SREG::I).into();
        unsafe { asm!("CLI") };
        status
    }
}

impl Drop for InterruptsStatus {
    #[inline(always)]
    fn drop(&mut self) {
        if let InterruptsStatus::Enabled = self {
            // blink(10);
            unsafe { asm!("SEI") };
        }
    }
}

use crate::pin::Pin;

fn blink(amount: u8) {
    const DURATION_MS: u64 = 100;
    crate::cores::atmega328::port::B5::set_output();
    crate::cores::atmega328::port::B5::set_low();

    for _ in 0..amount {
        crate::cores::atmega328::port::B5::set_high();

        avr_delay::delay_ms(DURATION_MS);

        crate::cores::atmega328::port::B5::set_low();

        avr_delay::delay_ms(DURATION_MS);
    }
}
