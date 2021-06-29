extern crate colorous;
extern crate image;

type F64Pair = (f64, f64);

pub mod frame;
pub mod frames_generator;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
