#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

// extern crate avr_libc;

use core::arch::asm;

use ruduino::cores::current::port;
use ruduino::modules::HardwareUsart;
use ruduino::Pin;
use ruduino::Register;

use ruduino::cores::atmega328::USART0;
use usart::WriteStatus;

mod lazy;
mod mutex;
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
    // unsafe { asm!("SEI") }
    // let bytes: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];
    // for byte in bytes {
    // if let WriteStatus::Blocked = usart::USART.write_byte_blocking(byte) {
    // break;
    // }
    // }
    let hello_world = "Hello world";
    for byte in hello_world.bytes() {
        usart::USART.write_byte_blocking(byte);
    }
}

fn blink() {
    port::B5::set_output();

    for _ in 0..5 {
        port::B5::set_high();

        ruduino::delay::delay_ms(1000);

        port::B5::set_low();

        ruduino::delay::delay_ms(1000);
    }
}
