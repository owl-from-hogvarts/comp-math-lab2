use newton_method::{solve, Input, Point};

mod newton_method;

fn main() {
    let solution = solve(Input {
        epsilon: 0.01,
        first_approximation: Point { x: 1., y: 1. },
    });

    println!("Solution: {solution}");
}
