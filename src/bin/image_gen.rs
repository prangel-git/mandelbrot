use std::fs::File;

use image::gif::GifEncoder;

extern crate colorous;
extern crate image;

fn main() {
    let x_length = 600u32;
    let y_length = 400u32;

    let left_botton = (-2.5f64, -1f64);
    let right_up = (1f64, 1f64);

    

    let center = (-1f64.exp() / 7f64, -1f64.exp()/20f64);
    let speed = 0.1f64;

    let mut frames_generator = mandelbrot::frames_generator::FramesGenerator::new(
        left_botton,
        right_up,
        x_length,
        y_length,
        center,
        speed,
    );

    let file_out = File::create("fractal.gif").unwrap();

    let mut encoder = GifEncoder::new(file_out);

    for k in 0..70 {
        let frame = frames_generator.next().unwrap();

        let img = image::ImageBuffer::from_fn(x_length, y_length, |x, y| {
            let c = frame.coordinates_to_value(x, y);
            let value = mandelbrot::utils::iterations_before_escape(&c, &1000u16);
            let index = (value as f64 / 100f64).powf(0.3f64);
            let colorous::Color { r, g, b } = colorous::TURBO.eval_continuous(index);
            image::Rgba([r, g, b, 0u8])
        });

        let gif_frame = image::Frame::new(img);
        encoder.encode_frame(gif_frame).unwrap();

        // let path = format!("fractal{:03}.png", k);

        // img.save(path).unwrap();

        println!("Frame {} finished.", k);
    }
}
