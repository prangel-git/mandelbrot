use std::fs::File;

use image::gif::GifEncoder;

extern crate colorous;
extern crate image;

fn main() {
    let width = 600u32;
    let height = 400u32;

    let left_botton = (-2.5f64, -1f64);
    let right_up = (1f64, 1f64);

    let center = (1f64.exp() / 7f64, -1f64.exp() / 20f64);
    let speed = 0.1f64;

    let mut frames_generator =
        mandelbrot::FramesGenerator::new(left_botton, right_up, width, height, center, speed);

    let file_out = File::create("fractal.gif").unwrap();

    let mut encoder = GifEncoder::new(file_out);

    let max_iterations = 1000u16;

    for k in 0..80 {
        let frame = frames_generator.next().unwrap();

        let image_generator = mandelbrot::ImageGenerator::new(&frame, max_iterations);

        let img = image::ImageBuffer::from_fn(width, height, |x, y| {
            let index = image_generator.eval(x, y);
            let colorous::Color { r, g, b } = colorous::TURBO.eval_continuous(index);
            image::Rgba([r, g, b, 255u8])
        });

        let gif_frame = image::Frame::new(img);
        encoder.encode_frame(gif_frame).unwrap();

        // let path = format!("fractal{:03}.png", k);

        // img.save(path).unwrap();
        let x_size = frames_generator.right_up.0 - frames_generator.left_botton.0;
        let y_size = frames_generator.right_up.1 - frames_generator.left_botton.1;
        let ratio = x_size / y_size;

        println!(
            "Frame {} finished. Width {}, Height {}, Ratio {}",
            k, x_size, y_size, ratio
        );
    }
}
