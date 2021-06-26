extern crate image;

fn main() {
    let x_length = 800u32;
    let y_length = 800u32;

    let left_botton = (-2.5f64, -1f64);
    let right_up = (1f64, 1f64);

    let frame = mandelbrot::frame::Frame::new(left_botton, right_up, x_length, y_length);

    let img = image::ImageBuffer::from_fn(x_length, x_length, |x, y| {
        let c = frame.coordinates_to_value(x, y);
        let value = mandelbrot::utils::iterations_before_escape(&c, &255u8);
        // println!("At coordinates ({}. {}), the current number of iterations to escape is {}", c.0, c.1, value);
        image::Luma([value])
    });

    img.save("fractal.png").unwrap();
}
