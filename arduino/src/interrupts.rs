//! Routines for managing interrupts.

use core::arch::asm;
use core::prelude::v1::*;

use ruduino::Register;

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
        use ruduino::cores::atmega328::SREG;
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
