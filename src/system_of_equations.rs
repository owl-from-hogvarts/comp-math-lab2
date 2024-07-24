use protocol::{
    point::{Point, PointCoordinate},
    TNumber,
};

use crate::equations::{Abs, MethodError, Solver, SolverInput, MAX_ITERATIONS};

#[derive(Clone)]
pub struct EquationWithPhi {
    /// To generate points for function graph.
    /// Use point coordinate to distinguish between
    /// *f(x) = y* and *f(y) = x*
    /// `PointCoordinate` designates, which coordinate is dependent.
    ///
    /// Example:
    /// `(3., PointCoordinate::x)` is returned for f(y) = x
    pub function: fn(TNumber) -> (TNumber, PointCoordinate),
    /// Calculate next step in simple iteration algorithm
    pub phi: fn((TNumber, TNumber)) -> TNumber,
}

#[derive(Clone)]
pub struct SystemOfEquations {
    pub first: EquationWithPhi,
    pub second: EquationWithPhi,
}

pub struct SimpleIteratorSolverForSystems;

impl Solver<SystemOfEquations> for SimpleIteratorSolverForSystems {
    fn solve(
        &self,
        system: &SystemOfEquations,
        parameters: &SolverInput,
    ) -> Result<Point, MethodError> {
        // TODO: divergence check
        let mut x = (parameters.start, parameters.end);
        for _ in 0..MAX_ITERATIONS {
            let new_x = ((system.first.phi)(x), (system.second.phi)(x));

            if TNumber::max(Abs::abs(new_x.0 - x.0), Abs::abs(new_x.1 - x.1)) < parameters.epsilon {
                return Ok(Point {
                    x: new_x.0,
                    y: new_x.1,
                });
            }

            x = new_x;
        }

        Err(MethodError::Diverges)
    }
}
