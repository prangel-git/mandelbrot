use super::F64Pair;

pub struct Frame {
    width: u32,
    height: u32,
    x0: f64,
    y0: f64,
    x_delta: f64,
    y_delta: f64,
}

impl Frame {
    pub fn new(left_botton: F64Pair, right_up: F64Pair, width: u32, height: u32) -> Self {
        let x0 = left_botton.0;
        let y0 = left_botton.1;

        let x_delta = (right_up.0 - left_botton.0) / (width as f64);
        let y_delta = (right_up.1 - left_botton.1) / (height as f64);

        Frame {
            width,
            height,
            x0,
            y0,
            x_delta,
            y_delta,
        }
    }

    pub fn coordinates_to_value(&self, x_coordinate: u32, y_coordinate: u32) -> F64Pair {
        (
            self.x0 + (x_coordinate as f64) * self.x_delta,
            self.y0 + (y_coordinate as f64) * self.y_delta,
        )
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
