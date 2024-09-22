//! Contains simple implementations of required language items that `libstd` normally defines on
//! other targets.
//!
//! You should always have `extern crate avr_std_stubs` defined when using this crate.

#![no_std]

#![feature(lang_items)]

// Disable the stubs for non-AVR targets because #[no_std] is ignored
// when using the Rust test harness, and we cannot detect if a downstream
// crate dependent on this crate is being compiled in test mode or not.
#[cfg(target_arch = "avr")]
mod stubs;
