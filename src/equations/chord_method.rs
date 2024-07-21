use protocol::point::Point;

use crate::equations::MethodError;

use super::Abs;
use super::{NonLinearEquation, Solver, SolverInput, MAX_ITERATIONS};

pub struct ChordSolver;

impl Solver<NonLinearEquation> for ChordSolver {
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
            if Abs::abs(y) <= epsilon {
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
