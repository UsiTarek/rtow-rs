pub mod hittable;
pub mod ray;
pub mod vec3;

use hittable::*;
use pbr::ProgressBar;
use ray::*;
use std::fs::File;
use std::io::stderr;
use vec3::*;

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

fn write_ppm_color(file: &mut dyn std::io::Write, color: Color) {
    const CONV_TO_BYTE: f32 = 255.999;
    let (r, g, b) = (
        (color.r() * CONV_TO_BYTE) as u8, // r
        (color.g() * CONV_TO_BYTE) as u8, // g
        (color.b() * CONV_TO_BYTE) as u8, // b
    );
    writeln!(file, "{} {} {}", r, g, b).unwrap();
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f32 / ASPECT_RATIO) as usize;

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
    let viewport_height = 2.0f32;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0f32;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal * 0.5) - (vertical * 0.5) - [0.0, 0.0, focal_length].into();

    // Progress Bar
    let mut progress_bar = { ProgressBar::on(stderr(), img_height as u64) };

    // Render
    let mut file = File::create("img.ppm").unwrap();
    write_ppm_header(&mut file, img_width, img_height);
    for j in (0..img_height).rev() {
        for i in 0..img_width {
            let u = i as f32 / (img_width - 1) as f32;
            let v = j as f32 / (img_height - 1) as f32;
            write_ppm_color(
                &mut file,
                ray_color(
                    &Ray::new(
                        origin,
                        lower_left_corner + (horizontal * u) + (vertical * v) - origin,
                    ),
                    world,
                ),
            );
        }
        progress_bar.inc(); // +1% Progress bar
    }
    progress_bar.finish(); // End of progress
}
