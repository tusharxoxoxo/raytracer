use itertools::Itertools;
use core::f64;
use std::{f64, fs, io};

const IMAGE_WIDTH: u32 = 3; 
const IMAGE_HEIGHT: u32 = 2;
const MAX_PIXEL_VALUE: u32 = 255;

fn main() -> io::Result<()> {
    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .map(|(y, x)| {
            let r = x as f64 (IMAGE_WIDTH - 1) as f64;
            let g = y as f64 (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;
            format!(
                "{} {} {}\n", 
                r * 255.0, 
                g * 255.0, 
                b * 255.0
                )
        })
    .chuncks(IMAGE_WIDTH as usize)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>().join(""))
        .join("\n");

    print!("{}", pixels);
    fs::write(
        "output.ppm",
        format!(
            "P3\n{} {}\n{}\n",
            IMAGE_WIDTH,
            IMAGE_HEIGHT,
            MAX_PIXEL_VALUE,
            pixels
        ),
}
