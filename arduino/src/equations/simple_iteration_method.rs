use protocol::point::Point;
use protocol::TNumber;

use super::Abs;
use super::MethodError;
use super::NonLinearEquation;
use super::Solver;
use super::SolverInput;
use super::MAX_ITERATIONS;

pub struct SimpleIterationSolver;

impl Solver<NonLinearEquation> for SimpleIterationSolver {
    fn solve(
        &self,
        equation: &NonLinearEquation,
        parameters: &SolverInput,
    ) -> Result<Point, MethodError> {
        let &SolverInput {
            start,
            end,
            epsilon,
        } = parameters;
        let lambda = calculate_lambda(equation, parameters);
        let phi = |x| x + lambda * (equation.function)(x);
        let phi_derivative = |x| 1. + lambda * (equation.first_derivative)(x);

        let q = TNumber::max(phi_derivative(start), phi_derivative(end));
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
        first_derivative, ..
    }: &NonLinearEquation,
    &SolverInput { start, end, .. }: &SolverInput,
) -> TNumber {
    let sign = if first_derivative(start).is_sign_negative() {
        1.
    } else {
        -1.
    };

    sign * 1.
        / TNumber::max(
            Abs::abs(first_derivative(start)),
            Abs::abs(first_derivative(end)),
        )
}

fn is_precise(x: TNumber, next_x: TNumber, q: TNumber, epsilon: TNumber) -> bool {
    let difference = Abs::abs(x - next_x);
    if q <= 0.5 {
        return difference < epsilon;
    }

    return difference < ((1. - q) / q * epsilon);
}
