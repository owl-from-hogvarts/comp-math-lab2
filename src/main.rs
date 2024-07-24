#![allow(dead_code)]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]
// avr is tier 3 support.
// we need math intrinsics, because avr-libc supports only f32 computations
// and we can't ship two copies of `compiler_builtin`s -- linker fails to
// resolve them
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate avr_libc;

use core::arch::asm;
use core::panic::PanicInfo;

use equations::{
    Equations, Logarithm, NonLinearEquation, Pow, Trigonometry, LEFT_BORDER, POINT_AMOUNT,
    POINT_INTERVAL_LENGTH,
};
use protocol::point::{Point, PointCoordinate};
use protocol::response::InitialApproximationsResponse;
use protocol::TNumber;
use protocol_handler::Connection;
use ruduino::cores::current::port;
use ruduino::Pin;
use system_of_equations::{EquationWithPhi, SystemOfEquations};

mod equations;
mod lazy;
mod protocol_handler;
mod ring_buffer;
mod system_of_equations;
mod usart;

const SINGLE: [NonLinearEquation; 2] = [
    NonLinearEquation {
        function: |x: TNumber| x.pow(2_f64) + x + Trigonometry::sin(x),
        first_derivative: |x: TNumber| 2_f64 * x + 1_f64 + Trigonometry::cos(x),
    },
    NonLinearEquation {
        function: |x: TNumber| Logarithm::ln(x + 15.) as TNumber + x.sin(),
        first_derivative: |x: TNumber| {
            (3. / 10.) * x.pow(x) + (3. * x.pow(x) * Logarithm::ln(x)) / 10.
        },
    },
    // NonLinearEquation {
    //     function: todo!(),
    //     first_derivative: todo!(),
    // },
];

const SYSTEMS: [SystemOfEquations; 0] = [/* SystemOfEquations {
    first: EquationWithPhi {
        function: |x| (1. - Trigonometry::sin(x) / 2., PointCoordinate::Y),
        phi: |(_x, y)| 0.7 - Trigonometry::cos(y - 1.),
    },
    second: EquationWithPhi {
        function: |y| (0.7 - Trigonometry::cos(y - 1.), PointCoordinate::X),
        phi: |(x, _y)| 1. - Trigonometry::sin(x) / 2.,
    },
} */];

#[no_mangle]
pub extern "C" fn main() {
    unsafe { asm!("SEI") }

    let mut connection = Connection::new(
        &*usart::USART,
        Equations {
            single: &SINGLE,
            systems: &SYSTEMS,
        },
    );
    let mut points_handler =
        |equation: &mut dyn FnMut(TNumber) -> (TNumber, PointCoordinate),
         write_back: &mut dyn FnMut(Point) -> ()| {
            for index in 0..POINT_AMOUNT {
                let variable = LEFT_BORDER + POINT_INTERVAL_LENGTH * index as TNumber;
                let (dependent, coord) = (equation)(variable);
                let point = match coord {
                    PointCoordinate::X => Point::new(dependent, variable),
                    PointCoordinate::Y => Point::new(variable, dependent),
                };

                write_back(point);
            }
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
