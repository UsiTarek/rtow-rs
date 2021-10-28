pub mod ray;
pub mod vec3;

use pbr::ProgressBar;
use ray::Ray;
use std::fs::File;
use std::io::stderr;
use vec3::{Color, Point3, Vec3};

fn hit_sphere(r: &Ray, center: &Vec3, radius: f32) -> f32 {
    let oc = r.origin() - *center;
    let a = r.direction().dot(&r.direction());
    let b = oc.dot(&r.direction()) * 2.0;
    let c = oc.dot(&oc) - radius.powi(2);
    let discr = b.powi(2) - (4.0 * a * c);
    if discr < 0.0 {
        -1.0
    } else {
        (-b - discr.sqrt()) / (a * 2.0)
    }
}

fn ray_color(r: &Ray) -> Color {
    let c = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(r, &c, 0.5);
    if t > 0.0 {
        let n = (r.at(t) - c).unit();
        return (n + [1.0, 1.0, 1.0].into()) * 0.5;
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
                ray_color(&Ray::new(
                    origin,
                    lower_left_corner + (horizontal * u) + (vertical * v) - origin,
                )),
            );
        }
        progress_bar.inc(); // +1% Progress bar
    }
    progress_bar.finish(); // End of progress
}
