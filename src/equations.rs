use libm;

use core::intrinsics;
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

#[derive(Debug)]
pub(crate) enum MethodError {
    Diverges,
}

pub(crate) struct SolverInput {
    pub start: TNumber,
    pub end: TNumber,
    pub epsilon: TNumber,
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

impl Pow for f64 {
    fn pow(self, power: f64) -> Self {
        // migrate to intrinsics
        // libc supports only f32
        unsafe { intrinsics::powf32(self as f32, power as f32) as f64 }
    }
}

impl Abs for f64 {
    fn abs(self) -> f64 {
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

impl Trigonometry for f64 {
    fn sin(self) -> Self {
        unsafe { intrinsics::sinf32(self as f32) as f64 }
    }

    fn cos(self) -> Self {
        // (1. - self.sin().pow(2.)).pow(0.5)
        unsafe { intrinsics::cosf32(self as f32) as f64 }
    }
}

pub trait Logarithm {
    fn ln(self) -> Self;
}

impl Logarithm for f64 {
    fn ln(self) -> Self {
        libm::log(self)
    }
}
