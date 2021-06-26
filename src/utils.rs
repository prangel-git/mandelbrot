use super::F64Pair;

pub fn quadratic_map(z: &F64Pair, c: &F64Pair) -> F64Pair {
    (z.0 * z.0 - z.1 * z.1 + c.0, 2f64 * z.0 * z.1 + c.1)
}

pub fn square_distance(z: &F64Pair) -> f64 {
    z.0 * z.0 + z.1 * z.1
}

pub fn iterations_before_escape(c: &F64Pair, max_iterations: &u8) -> u8 {
    let mut z = (0f64, 0f64);
    let mut iterations = 0u8;

    while (square_distance(&z) < 4f64) && (iterations < *max_iterations) {
        z = quadratic_map(&z, c);
        iterations += 1;
    }

    return iterations;
}
