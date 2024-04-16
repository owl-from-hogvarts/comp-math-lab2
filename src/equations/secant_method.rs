use protocol::point::Point;

use super::Abs;
use super::{MethodError, NonLinearEquation, Solver, SolverInput, MAX_ITERATIONS};

pub struct SecantSolver;

impl Solver for SecantSolver {
    fn solve(
        &self,
        equation: &NonLinearEquation,
        parameters: &SolverInput,
    ) -> Result<Point, MethodError> {
        let length = parameters.end - parameters.start;
        let mut x_previous = parameters.start + length / 4.;
        let mut x = parameters.end - length / 4.;
        for _ in 0..MAX_ITERATIONS {
            let x_next = x
                - ((x - x_previous) / ((equation.function)(x) - (equation.function)(x_previous)))
                    * (equation.function)(x);

            x_previous = x;
            x = x_next;

            if (x - x_previous).abs() <= parameters.epsilon
                || ((equation.function)(x)).abs() <= parameters.epsilon
            {
                return Ok(Point::new(x, (equation.function)(x)));
            }
        }

        Err(MethodError::Diverges)
    }
}
