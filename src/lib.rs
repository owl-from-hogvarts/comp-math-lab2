//! Defines simple implementations of required language items that `libstd` normally defines.

#![no_std]

#![feature(lang_items)]

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() -> () {
}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}

