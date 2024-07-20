use protocol::point::Point;

use super::Abs;
use super::MethodError;
use super::NonLinearEquation;
use super::Solver;
use super::SolverInput;
use super::MAX_ITERATIONS;

pub struct SimpleIterationSolver;

type Type = MethodError;

impl Solver<NonLinearEquation> for SimpleIterationSolver {
    fn solve(&self, equation: &NonLinearEquation, parameters: &SolverInput) -> Result<Point, Type> {
        let &SolverInput {
            start,
            end,
            epsilon,
        } = parameters;
        let lambda = calculate_lambda(equation, parameters);
        let phi = |x| x + lambda * (equation.function)(x);
        let phi_derevative = |x| 1. + lambda * (equation.first_derevative)(x);

        let q = f64::max(phi_derevative(start), phi_derevative(end));
        if q >= 1. {
            return Err(MethodError::Diverges);
        }

        let mut x = (start + end) / 2.;

        for _ in 0..MAX_ITERATIONS {
            let next_x = phi(x);
            if is_precise(x, next_x, q, epsilon) {
                return Ok(Point::new(next_x, (equation.function)(next_x)));
            }

            x = next_x;
        }

        return Err(MethodError::Diverges);
    }
}

fn calculate_lambda(
    NonLinearEquation {
        first_derevative, ..
    }: &NonLinearEquation,
    &SolverInput { start, end, .. }: &SolverInput,
) -> f64 {
    1. / f64::max(first_derevative(start), first_derevative(end))
}

fn is_precise(x: f64, next_x: f64, q: f64, epsilon: f64) -> bool {
    let difference = (x - next_x).abs();
    if q <= 0.5 {
        return difference < epsilon;
    }

    return difference < ((1. - q) / q * epsilon);
}
