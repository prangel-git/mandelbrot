use crate::F64Pair;

use super::frame::Frame;

pub struct FramesGenerator {
    left_botton: F64Pair,
    right_up: F64Pair,
    x_length: u32,
    y_length: u32,
    center: F64Pair,
    speed: f64,
}

impl FramesGenerator {
    pub fn new(
        left_botton: F64Pair,
        right_up: F64Pair,
        x_length: u32,
        y_length: u32,
        center: F64Pair,
        speed: f64,
    ) -> Self {
        FramesGenerator {
            left_botton,
            right_up,
            x_length,
            y_length,
            center,
            speed,
        }
    }
}

impl Iterator for FramesGenerator {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        let frame = Frame::new(
            self.left_botton,
            self.right_up,
            self.x_length,
            self.y_length,
        );
        self.left_botton = convex_combination(self.left_botton, self.center, self.speed);
        self.right_up = convex_combination(self.right_up, self.center, self.speed);
        return Some(frame);
    }
}

fn convex_combination(x: F64Pair, y: F64Pair, alpha: f64) -> F64Pair {
    (
        (1f64 - alpha) * x.0 + alpha * y.0,
        (1f64 - alpha) * x.1 + alpha * y.1,
    )
}
