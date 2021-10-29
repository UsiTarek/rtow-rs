pub mod camera;
pub mod hittable;
pub mod ray;
pub mod vec3;

use hittable::*;
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use ray::*;
use std::fs::File;
use std::io::stderr;
use vec3::*;

use crate::camera::Camera;

fn ray_color<'a>(r: &Ray, hittables: &[Box<dyn Hittable>]) -> Color {
    if let Some(hr) = hittables.hit(r, 0.0, f32::INFINITY) {
        return 0.5 * (hr.normal + [1.0, 1.0, 1.0].into());
    }

    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
}

fn write_ppm_header(file: &mut dyn std::io::Write, width: usize, height: usize) {
    writeln!(file, "P3\n{} {} \n255\n", width, height).unwrap();
}

fn write_ppm_color(file: &mut dyn std::io::Write, color: Color, spp: u32) {
    const CONV_TO_BYTE: f32 = 255.999;

    let scale = 1.0 / spp as f32;
    let (r, g, b) = (
        (color.r() * CONV_TO_BYTE * scale) as u8, // r
        (color.g() * CONV_TO_BYTE * scale) as u8, // g
        (color.b() * CONV_TO_BYTE * scale) as u8, // b
    );

    writeln!(file, "{} {} {}", r, g, b).unwrap();
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f32 / ASPECT_RATIO) as usize;
    let spp = 100;

    // World
    let world: &[Box<dyn Hittable>] = &[
        Box::new(Sphere {
            center: [0.0, 0.0, -1.0].into(),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: [0.0, -100.5, -1.0].into(),
            radius: 100.0,
        }),
    ];

    // Camera
    let camera = Camera::new();

    // Progress Bar
    let mut progress_bar = { ProgressBar::on(stderr(), img_height as u64) };

    // Render
    let mut file = File::create("img.ppm").unwrap();
    write_ppm_header(&mut file, img_width, img_height);
    for j in (0..img_height).rev() {
        for i in 0..img_width {
            let pixel_color = {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..spp {
                    let u = (i as f32 + thread_rng().gen::<f32>()) / (img_width - 1) as f32;
                    let v = (j as f32 + thread_rng().gen::<f32>()) / (img_height - 1) as f32;
                    pixel_color += ray_color(&camera.ray(u, v), world);
                }
                pixel_color
            };
            write_ppm_color(&mut file, pixel_color, spp);
        }
        progress_bar.inc(); // +1% Progress bar
    }
    progress_bar.finish(); // End of progress
}
