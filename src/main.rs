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
use core::cell::UnsafeCell;
use core::panic::PanicInfo;

use buttons::DEBOUNCED_BUTTONS_CONTEXT;
use equations::{
    check_roots_in_range, ChordSolver, Equations, Logarithm, NonLinearEquation, Pow, SecantSolver,
    SimpleIterationSolver, Solver, SolverInput, Trigonometry, LEFT_BORDER, POINT_AMOUNT,
    POINT_INTERVAL_LENGTH, RIGHT_BORDER,
};
use lazy::Lazy;
use protocol::point::{Point, PointCoordinate};
use protocol::request::compute_method::Method;
use protocol::request::payloads::ComputeRootPayload;
use protocol::request::SingleEquation;
use protocol::response::InitialApproximationsResponse;
use protocol::TNumber;
use protocol_handler::Connection;
use ruduino::cores::current::port;
use ruduino::interrupt::without_interrupts;
use ruduino::Pin;
use system_of_equations::{EquationWithPhi, SimpleIteratorSolverForSystems, SystemOfEquations};

mod buttons;
mod equations;
mod lazy;
mod protocol_handler;
mod ring_buffer;
mod system_of_equations;
mod usart;

const SINGLE: [NonLinearEquation; 2] = [
    NonLinearEquation {
        function: |x: TNumber| x.pow(2.) + x + Trigonometry::sin(x),
        first_derivative: |x: TNumber| 2. * x + 1. + Trigonometry::cos(x),
    },
    NonLinearEquation {
        function: |x: TNumber| Logarithm::ln(x + 15.) as TNumber,
        first_derivative: |x: TNumber| 1. / (x + 15.),
    },
    // NonLinearEquation {
    //     function: todo!(),
    //     first_derivative: todo!(),
    // },
];

const SYSTEMS: [SystemOfEquations; 1] = [SystemOfEquations {
    first: EquationWithPhi {
        function: |x| (1. - Trigonometry::sin(x) / 2., PointCoordinate::Y),
        phi: |(_x, y)| 0.7 - Trigonometry::cos(y - 1.),
    },
    second: EquationWithPhi {
        function: |y| (0.7 - Trigonometry::cos(y - 1.), PointCoordinate::X),
        phi: |(x, _y)| 1. - Trigonometry::sin(x) / 2.,
    },
}];

static INITIAL_APPROXIMATIONS: Lazy<UnsafeCell<InitialApproximationsResponse>> = Lazy::new(|| {
    UnsafeCell::new(InitialApproximationsResponse {
        left: -1.,
        right: 1.,
    })
});

enum InitialApproximationsEvent {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

fn update_initial_approximations(event: InitialApproximationsEvent) {
    const STEP: f32 = 0.25;

    let approximations = unsafe { &mut *INITIAL_APPROXIMATIONS.get() };
    match event {
        InitialApproximationsEvent::LeftUp => {
            let new = approximations.left + STEP;
            if (LEFT_BORDER..RIGHT_BORDER).contains(&new) {
                approximations.left = new
            }
        }
        InitialApproximationsEvent::LeftDown => {
            let new = approximations.left - STEP;
            if (LEFT_BORDER..RIGHT_BORDER).contains(&new) {
                approximations.left = new
            }
        }
        InitialApproximationsEvent::RightUp => {
            let new = approximations.right + STEP;
            if (LEFT_BORDER..RIGHT_BORDER).contains(&new) {
                approximations.right = new
            }
        }
        InitialApproximationsEvent::RightDown => {
            let new = approximations.right - STEP;
            if (LEFT_BORDER..RIGHT_BORDER).contains(&new) {
                approximations.right = new
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe { asm!("SEI") }

    DEBOUNCED_BUTTONS_CONTEXT.set_callback(
        || update_initial_approximations(InitialApproximationsEvent::LeftDown),
        0,
    );
    DEBOUNCED_BUTTONS_CONTEXT.set_callback(
        || update_initial_approximations(InitialApproximationsEvent::LeftUp),
        1,
    );
    DEBOUNCED_BUTTONS_CONTEXT.set_callback(
        || update_initial_approximations(InitialApproximationsEvent::RightDown),
        2,
    );
    DEBOUNCED_BUTTONS_CONTEXT.set_callback(
        || update_initial_approximations(InitialApproximationsEvent::RightUp),
        3,
    );

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

    let mut compute_root_handler = |payload: ComputeRootPayload| {
        let approximations =
            without_interrupts(|| unsafe { *INITIAL_APPROXIMATIONS.get().clone() });

        let parameters = SolverInput {
            start: approximations.left,
            end: approximations.right,
            epsilon: payload.epsilon,
        };

        match payload.mode {
            protocol::request::EquationMode::Single(SingleEquation {
                method,
                equation_number,
            }) => {
                let equation = &SINGLE[equation_number as usize];

                check_roots_in_range(equation, &parameters)?;
                match method {
                    Method::Chord => ChordSolver::solve(&ChordSolver, equation, &parameters),
                    Method::Secant => SecantSolver::solve(&SecantSolver, equation, &parameters),
                    Method::SimpleIterationSingle => {
                        SimpleIterationSolver::solve(&SimpleIterationSolver, equation, &parameters)
                    }
                }
            }
            protocol::request::EquationMode::SystemOfEquations { system_number } => {
                let system = &SYSTEMS[system_number as usize];
                SimpleIteratorSolverForSystems::solve(
                    &SimpleIteratorSolverForSystems,
                    system,
                    &parameters,
                )
            }
        }
    };

    let mut initial_approximations_handler =
        || without_interrupts(|| unsafe { *INITIAL_APPROXIMATIONS.get().clone() });
    connection.set_points_handler(&mut points_handler);
    connection.set_initial_approximation(&mut initial_approximations_handler);
    connection.set_compute_root(&mut compute_root_handler);

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
