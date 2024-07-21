use protocol::point::Point;

use crate::system_of_equations::SystemOfEquations;

pub(crate) const MAX_ITERATIONS: usize = 1000;

pub const LEFT_BORDER: f64 = -10.;
pub const RIGHT_BORDER: f64 = 10.;
pub const POINT_INTERVAL_LENGTH: f64 = (RIGHT_BORDER - LEFT_BORDER) / POINT_AMOUNT as f64;
pub use protocol::POINT_AMOUNT;

pub struct Range {
    left: f64,
    right: f64,
}

mod chord_method;
mod secant_method;
mod simple_iteration_method;

#[derive(Debug)]
pub(crate) enum MethodError {
    Diverges,
}

pub(crate) struct SolverInput {
    pub start: f64,
    pub end: f64,
    pub epsilon: f64,
}

pub type SingleArgFunction = fn(x: f64) -> f64;

#[derive(Clone)]
pub struct NonLinearEquation {
    pub function: SingleArgFunction,
    pub first_derevative: SingleArgFunction,
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
    fn pow(self, power: f64) -> Self;
}

impl Pow for f64 {
    fn pow(self, power: f64) -> Self {
        unsafe { avr_libc::pow(self, power) }
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
        unsafe { avr_libc::sin(self) }
    }

    fn cos(self) -> Self {
        unsafe { avr_libc::cos(self) }
    }
}

pub trait Logarithm {
    fn ln(self) -> Self;
}

impl Logarithm for f64 {
    fn ln(self) -> Self {
        unsafe { avr_libc::log(self) }
    }
}
