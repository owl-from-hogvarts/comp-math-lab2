use protocol::point::Point;

use crate::equations::{Abs, MethodError, NonLinearEquation, Solver, SolverInput, MAX_ITERATIONS};

pub type SystemOfEquations = [NonLinearEquation; 2];

pub struct SimpleIteratorSolverForSystems;

impl Solver<SystemOfEquations> for SimpleIteratorSolverForSystems {
    fn solve(
        &self,
        system: &SystemOfEquations,
        parameters: &SolverInput,
    ) -> Result<Point, MethodError> {
        // TODO: divergence check
        let mut x = [parameters.start, parameters.end];
        for _ in 0..MAX_ITERATIONS {
            let new_x = [(system[0].function)(x[0]), (system[1].function)(x[1])];

            if f64::max((new_x[0] - x[0]).abs(), (new_x[1] - x[1]).abs()) < parameters.epsilon {
                return Ok(Point {
                    x: new_x[0],
                    y: new_x[1],
                });
            }

            x = new_x;
        }

        Err(MethodError::Diverges)
    }
}
