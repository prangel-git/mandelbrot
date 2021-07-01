use super::F64Pair;

pub fn quadratic_map(z: &F64Pair, c: &F64Pair) -> F64Pair {
    (z.0 * z.0 - z.1 * z.1 + c.0, 2f64 * z.0 * z.1 + c.1)
}

pub fn square_distance(z: &F64Pair) -> f64 {
    z.0 * z.0 + z.1 * z.1
}

pub fn iterations_before_escape_optimize(c: &F64Pair, max_iterations: &u16) -> u16 {
    let mut x_sq = 0f64;
    let mut y_sq = 0f64;

    let mut x = 0f64;
    let mut y = 0f64;

    let (x0, y0) = *c;

    let mut iterations = 0u16;

    while (x_sq + y_sq < 4f64) && (iterations < *max_iterations) {
        y = (x + x) * y + y0;
        x = x_sq - y_sq + x0;
        x_sq = x * x;
        y_sq = y * y;
        iterations += 1;
    }

    return iterations;
}

pub fn iterations_before_escape(c: &F64Pair, max_iterations: &u16) -> u16 {
    let mut z = (0f64, 0f64);
    let mut iterations = 0u16;

    while (square_distance(&z) < 4f64) && (iterations < *max_iterations) {
        z = quadratic_map(&z, c);
        iterations += 1;
    }

    return iterations;
}