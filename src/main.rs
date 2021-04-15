use pbr::ProgressBar;
use std::io::{stderr, Stderr};

const IMAGE_EXTENT: usize = 256;

fn main() {
    // Render
    println!("P3\n{} {} \n255\n", IMAGE_EXTENT, IMAGE_EXTENT);

    // Progress bar (stderr)
    let mut pb: ProgressBar<Stderr> = {
        let handle = stderr();
        ProgressBar::on(handle, IMAGE_EXTENT as u64)
    };

    for y in (0..IMAGE_EXTENT).rev() {
        pb.inc(); // +1% Progress bar
        for x in 0..IMAGE_EXTENT {
            let (r, g, b) = {
                const CONV_TO_BYTE: f32 = 255.999;

                (
                    ((x as f32 / (IMAGE_EXTENT - 1) as f32) * CONV_TO_BYTE) as u32, // r
                    ((y as f32 / (IMAGE_EXTENT - 1) as f32) * CONV_TO_BYTE) as u32, // g
                    (0.25 * CONV_TO_BYTE) as u32,                                   // b
                )
            };

            println!("{} {} {}", r, g, b); // Print .ppm colors (stdout)
        }
    }

    pb.finish(); // End of progress
}
