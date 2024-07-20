#![feature(panic_info_message)]
#![allow(dead_code)]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]
#![no_std]
#![no_main]

// extern crate avr_libc;

use core::arch::asm;
use core::panic::PanicInfo;

use equations::{NonLinearEquation, Pow, LEFT_BORDER, POINT_INTERVAL_LENGTH};
use protocol::point::Point;
use protocol::request::payloads::FunctionPointsPayload;
use protocol::response::InitialApproximationsResponse;
use protocol_handler::Connection;
use ring_buffer::RingBuffer;
use ruduino::cores::current::port;
use ruduino::Pin;

mod equations;
mod lazy;
mod protocol_handler;
mod ring_buffer;
mod system_of_equations;
mod usart;

use crate::equations::Trigonometry;

pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe { asm!("SEI") }

    let equations: [NonLinearEquation; 1] = [
        NonLinearEquation {
            function: |x: f64| x.pow(2_f64) + x + x.sin(),
            first_derevative: |x: f64| 2_f64 * x + 1_f64 + x.cos(),
        },
        // NonLinearEquation {
        //     function: todo!(),
        //     first_derevative: todo!(),
        // },
        // NonLinearEquation {
        //     function: todo!(),
        //     first_derevative: todo!(),
        // },
    ];

    let mut connection = Connection::new(&*usart::USART);
    let mut points_handler = |config: &FunctionPointsPayload, index| {
        let equation = &equations[config.equation_number as usize];
        let x = LEFT_BORDER + POINT_INTERVAL_LENGTH * index as f64;
        Point::new(x, (equation.function)(x))
    };

    let mut initial_approximations_handler = || InitialApproximationsResponse {
        left: 1.,
        right: 5.,
    };
    connection.set_points_handler(&mut points_handler);
    connection.set_initial_approximation(&mut initial_approximations_handler);

    loop {
        connection.handle_request();
    }
}

#[panic_handler]
fn panic_handler(_data: &PanicInfo) -> ! {
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
extern "avr-interrupt" fn __vector_16() {}
#[no_mangle]
extern "avr-interrupt" fn __vector_17() {}
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
