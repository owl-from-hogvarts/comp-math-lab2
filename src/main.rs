#![feature(panic_info_message)]
#![allow(dead_code)]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

// extern crate avr_libc;

use core::arch::asm;
use core::panic::PanicInfo;

use ruduino::cores::current::{port, SREG, USART0};
use ruduino::interrupt::without_interrupts;
use ruduino::modules::HardwareUsart;
use ruduino::{Pin, Register};

mod lazy;
mod ring_buffer;
mod usart;

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

#[no_mangle]
pub extern "C" fn main() {
    // let a: f64 = 1.;
    // let b: f64 = 2.;

    // let result = a + b;
    // let bytes = result.to_le_bytes();
    unsafe { asm!("SEI") }
    // let bytes: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];
    // for byte in bytes {
    // if let WriteStatus::Blocked = usart::USART.write_byte_blocking(byte) {
    // break;
    // }
    // }
    let hello_world = "Hello world";
    for byte in hello_world.bytes() {
        usart::USART.write_byte(byte);
    }
}

#[panic_handler]
fn panic_handler(data: &PanicInfo) -> ! {
    loop {
        blink(5, 50);
    }
}

fn blink(amount: u8, duration: u64) {
    port::B5::set_output();
    port::B5::set_low();

    for _ in 0..amount {
        port::B5::set_high();

        ruduino::delay::delay_ms(duration);

        port::B5::set_low();

        ruduino::delay::delay_ms(duration);
    }
}

#[no_mangle]
extern "avr-interrupt" fn __vector_0() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_1() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_2() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_3() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_4() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_5() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_6() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_7() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_8() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_9() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_10() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_11() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_12() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_13() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_14() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_15() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_16() {
    // blink(10, 50);
}
#[no_mangle]
extern "avr-interrupt" fn __vector_17() {
    // blink(10, 50);
}
#[no_mangle]
extern "avr-interrupt" fn __vector_18() {
    // blink(10, 50);
}
#[no_mangle]
extern "avr-interrupt" fn __vector_20() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_21() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_22() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_23() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_24() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_25() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_26() {}
