use crate::iterations_before_escape;
use crate::Frame;

use rayon::prelude::*;

struct HistogramColoring {
    iteration_counts: Vec<u16>,
    num_iterations_per_pixels: Vec<u16>,
    total_iterations: f64,
}

impl HistogramColoring {
    fn new(frame: &Frame, max_iterations: u16) -> Self {
        let width = frame.width();
        let height = frame.height();
        let size = (width * height) as usize;

        let mut iteration_counts = vec![0u16; size];
        let mut num_iterations_per_pixels = vec![0u16; (max_iterations + 1) as usize];
        let mut total_iterations = 0f64;

        
        for i in 0..width {
            for j in 0..height {
                let index = (i + width * j) as usize;
                let iterations = iterations_before_escape(&frame.coordinates_to_value(i, j), &max_iterations);
                iteration_counts[index] = iterations;
            }
        }

        for &idx in &iteration_counts {
            num_iterations_per_pixels[idx as usize] += 1;
        }

        for &idx in &num_iterations_per_pixels {
            total_iterations += idx as f64;
        }

        HistogramColoring {
            iteration_counts,
            num_iterations_per_pixels,
            total_iterations,
        }
    }

    fn calculate_hue(&self) -> Vec<f64> {
        self.iteration_counts
            .par_iter()
            .map(|&i| (self.num_iterations_per_pixels[i as usize] as f64) / self.total_iterations)
            .collect::<Vec<f64>>()
    }
}
pub struct ImageGenerator {
    width: u32,
    height: u32,
    hue: Vec<f64>,
}

impl ImageGenerator {
    pub fn new(frame: &Frame, max_iterations: u16) -> Self {
        let width = frame.width();
        let height = frame.height();

        let histogram_coloring = HistogramColoring::new(frame, max_iterations);
        let hue = histogram_coloring.calculate_hue();

        ImageGenerator { width, height, hue }
    }

    pub fn eval(&self, i: u32, j: u32) -> f64 {
        if i >= self.width && j >= self.height {
            panic!("Coordinates out of bound");
        }

        let idx = (i + j * self.width) as usize;

        self.hue[idx]
    }

    pub fn hue(&self) -> &Vec<f64> {
        &self.hue
    }
}
