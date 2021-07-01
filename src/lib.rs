extern crate colorous;
extern crate image;
extern crate rayon;

type F64Pair = (f64, f64);

mod frame;
pub use frame::Frame;

mod frames_generator;
pub use frames_generator::FramesGenerator;

mod escape_time;
pub use escape_time::iterations_before_escape;
pub use escape_time::iterations_before_escape_optimize;

mod image_generator;
pub use image_generator::ImageGenerator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
