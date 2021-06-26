extern crate image;

fn main() {
    let x_length = 1200u32;
    let y_length = 800u32;

    let left_botton = (-2.5f64, -1f64);
    let right_up = (1f64, 1f64);

    let frame = mandelbrot::frame::Frame::new(left_botton, right_up, x_length, y_length);

    let img = image::ImageBuffer::from_fn(x_length, y_length, |x, y| {
        let c = frame.coordinates_to_value(x, y);
        let value = mandelbrot::utils::iterations_before_escape(&c, &1023u16);
        let r = value as u8;
        let g = (value >> 1) as u8;
        let b = (value >> 2) as u8;
        image::Rgb([r, g, b])
    });

    img.save("fractal.png").unwrap();
}
