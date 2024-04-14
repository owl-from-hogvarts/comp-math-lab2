use protocol::Method;

struct Solver {
    current_method: Method,
    left: f64,
    right: f64,
}

struct Equation {
    function: fn(x: f64) -> f64,
    first_derevative: fn(x: f64) -> f64,
}

impl Equation {}
