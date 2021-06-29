extern crate colorous;
extern crate image;

fn main() {
    let x_length = 2400u32;
    let y_length = 1600u32;

    let left_botton = (-2.5f64, -1f64);
    let right_up = (1f64, 1f64);

    let frame = mandelbrot::frame::Frame::new(left_botton, right_up, x_length, y_length);

    let img = image::ImageBuffer::from_fn(x_length, y_length, |x, y| {
        let c = frame.coordinates_to_value(x, y);
        let value = mandelbrot::utils::iterations_before_escape(&c, &100u16);
        let index = (value as f64 / 100f64).powf(0.3f64);
        let colorous::Color { r, g, b } = colorous::TURBO.eval_continuous(index);
        image::Rgb([r, g, b])
    });

    img.save("fractal.png").unwrap();
}
