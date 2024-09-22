use protocol::response::MethodError;
use protocol::{point::Point, TNumber};

use crate::system_of_equations::SystemOfEquations;

pub(crate) const MAX_ITERATIONS: usize = 1000;

pub const LEFT_BORDER: TNumber = -10.;
pub const RIGHT_BORDER: TNumber = 10.;
pub const POINT_INTERVAL_LENGTH: TNumber = (RIGHT_BORDER - LEFT_BORDER) / POINT_AMOUNT as TNumber;
pub use protocol::POINT_AMOUNT;

pub struct Range {
    left: TNumber,
    right: TNumber,
}

mod chord_method;
mod secant_method;
mod simple_iteration_method;

pub use chord_method::ChordSolver;
pub use secant_method::SecantSolver;
pub use simple_iteration_method::SimpleIterationSolver;

pub(crate) struct SolverInput {
    pub start: TNumber,
    pub end: TNumber,
    pub epsilon: TNumber,
}

pub fn check_roots_in_range(
    equation: &NonLinearEquation,
    parameters: &SolverInput,
) -> Result<(), MethodError> {
    let different_signs = (equation.function)(parameters.start).is_sign_negative()
        != (equation.function)(parameters.end).is_sign_negative();

    const SAMPLES_AMOUNT: usize = 100;

    let mut x = parameters.start;
    let mut derevative_different_signs = false;
    let is_first_negative = (equation.first_derivative)(x).is_sign_negative();
    let step = (parameters.end - parameters.start) / SAMPLES_AMOUNT as f32;
    for _ in 0..SAMPLES_AMOUNT {
        x += step;
        let y = (equation.first_derivative)(x);
        if y.is_sign_negative() != is_first_negative {
            derevative_different_signs = true;
            break;
        }
    }

    // using heurisctics to guess amount of roots withing range
    match (different_signs, derevative_different_signs) {
        (true, false | true) => Ok(()),
        (false, true) => Err(MethodError::MoreThanOneRootInRange),
        (false, false) => Err(MethodError::NoRootInRange),
    }
}

pub type SingleArgFunction = fn(x: TNumber) -> TNumber;

#[derive(Clone)]
pub struct NonLinearEquation {
    pub function: SingleArgFunction,
    pub first_derivative: SingleArgFunction,
}

pub struct Equations {
    pub single: &'static [NonLinearEquation],
    pub systems: &'static [SystemOfEquations],
}

pub(crate) trait Solver<T> {
    fn solve(&self, equation: &T, parameters: &SolverInput) -> Result<Point, MethodError>;
}

pub trait Abs {
    fn abs(self) -> Self;
}

pub trait Pow {
    fn pow(self, power: Self) -> Self;
}

impl Pow for TNumber {
    fn pow(self, power: TNumber) -> Self {
        // migrate to intrinsics
        // libc supports only f32
        unsafe { avr_libc::powf(self, power) as TNumber }
    }
}

impl Abs for TNumber {
    fn abs(self) -> TNumber {
        if self.is_sign_positive() {
            return self;
        }

        return -self;
    }
}

pub trait Trigonometry {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
}

impl Trigonometry for TNumber {
    fn sin(self) -> Self {
        unsafe { avr_libc::sinf(self as f32) as TNumber }
    }

    fn cos(self) -> Self {
        // (1. - self.sin().pow(2.)).pow(0.5)
        unsafe { avr_libc::cosf(self as f32) as TNumber }
    }
}

pub trait Logarithm {
    fn ln(self) -> Self;
}

impl Logarithm for TNumber {
    fn ln(self) -> Self {
        unsafe { avr_libc::logf(self) }
    }
}
