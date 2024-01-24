use itertools::Itertools;
use std::{fs, io};

const IMAGE_WIDTH: u32 = 2;
const IMAGE_HEIGHT: u32 = 3;
const MAX_PIXEL_VALUE: u8 = 255;

fn main() -> io::Result<()> {
        
    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .map(|(y, x)| {
            let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            format!(
                "{} {} {}",
                r * MAX_PIXEL_VALUE as f64,
                g * MAX_PIXEL_VALUE as f64,
                b * MAX_PIXEL_VALUE as f64
                )
            })
        .chunks(IMAGE_WIDTH as usize)
            .into_iter()
            .map(|chunk| chunk.into_iter().join(" "))
            .join("\n");
        println!("{}", pixels);

        fs::write(
            "image.ppm",
            format!(
                "P3
                {IMAGE_WIDTH} {IMAGE_HEIGHT}
                {MAX_PIXEL_VALUE}
                {pixels}
                "

                ),
                )?;


    Ok(())
}
