//! Contains simple implementations of required language items that `libstd` normally defines on
//! other targets.

#![no_std]

#![feature(lang_items)]


#[cfg(not(test))] // Disable the stubs for test because #[no_std] is ignored there because libtest depends on it.
mod stubs;
