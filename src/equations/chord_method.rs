use protocol::point::Point;

use crate::equations::MethodError;

use super::{NonLinearEquation, Solver, SolverInput};

const MAX_ITERATIONS: usize = 1000;

pub struct ChordSolver;

impl Solver for ChordSolver {
    fn solve(
        &self,
        equation: &NonLinearEquation,
        parameters: &SolverInput,
    ) -> Result<Point, MethodError> {
        let SolverInput {
            mut start,
            mut end,
            epsilon,
        } = *parameters;

        let mut count: usize = 0;
        while count < MAX_ITERATIONS {
            // println!("{:-^80}", count + 1);
            let x = start
                - ((equation.function)(start)
                    / ((equation.function)(start) - (equation.function)(end)))
                    * (start - end);
            // println!("x: {x:.5}");

            let y = (equation.function)(x);
            // println!("y: {y:.5}");
            if y.abs() <= epsilon {
                return Ok(Point::new(x, y));
            }

            if !(start <= x && x <= end) {
                return Err(MethodError::Diverges);
            }

            if y > 0_f64 {
                //     println!("y > 0");
                //     println!("b = x");
                end = x;
            } else {
                //     println!("y < 0");
                //     println!("a = x");
                start = x;
            }

            count += 1;
        }

        return Err(MethodError::Diverges);
    }
}

trait Abs<T> {
    fn abs(self) -> T;
}

impl Abs<f64> for f64 {
    fn abs(self) -> f64 {
        unsafe { avr_libc::fabs(self) }
    }
}
