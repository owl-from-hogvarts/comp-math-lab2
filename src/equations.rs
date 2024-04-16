use protocol::{point::Point, POINT_AMOUNT};

const MAX_ITERATIONS: usize = 1000;

pub const LEFT_BORDER: f64 = -10.;
pub const RIGHT_BORDER: f64 = 10.;
pub const POINT_INTERVAL_LENGTH: f64 = (RIGHT_BORDER - LEFT_BORDER) / POINT_AMOUNT as f64;

pub struct Range {
    left: f64,
    right: f64,
}

mod chord_method;
mod secant_method;
mod simple_iteration_method;

#[derive(Debug)]
enum MethodError {
    Diverges,
}

struct SolverInput {
    start: f64,
    end: f64,
    epsilon: f64,
}

pub struct NonLinearEquation {
    pub function: fn(x: f64) -> f64,
    pub first_derevative: fn(x: f64) -> f64,
}

trait Solver {
    fn solve(
        &self,
        equation: &NonLinearEquation,
        parameters: &SolverInput,
    ) -> Result<Point, MethodError>;
}

impl NonLinearEquation {}

trait Abs {
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
        unsafe { avr_libc::fabs(self) }
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
