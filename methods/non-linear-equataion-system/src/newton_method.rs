use core::panic;
use std::fmt::Display;

const MAX_ITERATIONS: usize = 10;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point { x, y } = self;
        write!(f, "Point({x:.5}, {y:.5})")
    }
}

pub struct Input {
    pub first_approximation: Point,
    pub epsilon: f64,
}

pub fn solve(
    Input {
        epsilon,
        first_approximation,
    }: Input,
) -> Point {
    let mut point = first_approximation;
    for count in 1..MAX_ITERATIONS {
        println!("{:-^80}", count);
        println!("{point}");
        let delta_x = compute_delta_x(point);
        let delta_y = compute_delta_y(point, delta_x);
        println!("deltas: {delta_x:.5}, {delta_y:.5}");

        let new_point = Point {
            x: point.x + delta_x,
            y: point.y + delta_y,
        };

        println!("new point: {new_point}");

        if (new_point.x - point.x).abs() < epsilon && (new_point.y - point.y).abs() < epsilon {
            return new_point;
        }

        point = new_point;
    }

    panic!("divirges")
}

fn compute_delta_x(Point { x, y }: Point) -> f64 {
    let numerator =
        2. * y * (1.4 * x - f64::sin(x + y)) - (f64::cos(x + y) * (1. - x.powi(2) - y.powi(2)));
    let denominator = -2.8 * y + 2. * f64::cos(x + y) * (y - x);
    numerator / denominator
}

fn compute_delta_y(Point { x, y }: Point, delta_x: f64) -> f64 {
    let numerator = 1. - x.powi(2) - y.powi(2) - (2. * x * delta_x);
    numerator / (2. * y)
}
