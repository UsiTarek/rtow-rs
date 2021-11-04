pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod vec3;

use hittable::*;
use material::*;
use pbr::ProgressBar;
use rand::{thread_rng, Rng};
use ray::*;
use std::{fs::File, io::stderr, rc::Rc};
use vec3::*;

use crate::camera::Camera;

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (uv * -1.0).dot(&n).min(1.0);
    let r_out_perop = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (r_out_perop.length_squared() - 1.0).abs().sqrt() * -1.0 * n;
    r_out_perop + r_out_parallel
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

fn random_float() -> f32 {
    thread_rng().gen_range(0.0..1.0)
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::rand_range(-1.0..1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

fn random_in_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

// NOTE(Tarik) : Optional in section 8.6
//fn random_in_hemisphere(normal: &Vec3) -> Vec3
//{
//    let r_unit_sphere = random_in_unit_sphere();
//    if r_unit_sphere.dot(&normal) > 0.0 {
//        r_unit_sphere
//    }else {
//        -1.0 * r_unit_sphere
//    }
//}

// NOTE(Tarik) : Disabled until future use
//fn random_float_range(range: Range<f32>) -> f32 {
//    thread_rng().gen_range(range)
//}

fn ray_color(r: &Ray, hittables: &[Box<dyn Hittable>], depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hr) = hittables.hit(r, 0.001, f32::INFINITY) {
        return {
            if let Some(scatter) = hr.mat.scatter(r, &hr) {
                scatter.attenuation * ray_color(&scatter.scattered, hittables, depth - 1)
            } else {
                [0.0, 0.0, 0.0].into()
            }
        };
    }

    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
}

fn write_ppm_header(file: &mut dyn std::io::Write, width: usize, height: usize) {
    writeln!(file, "P3\n{} {} \n255\n", width, height).unwrap();
}

fn write_ppm_color(file: &mut dyn std::io::Write, color: Color, spp: u32) {
    const CONV_TO_BYTE: f32 = 255.999;

    let scale = 1.0 / spp as f32;
    let (r, g, b) = (color[0], color[1], color[2]);
    let (r, g, b) = ((r * scale).sqrt(), (g * scale).sqrt(), (b * scale).sqrt());

    let (r, g, b) = (
        (r.clamp(0.0, 0.999) * CONV_TO_BYTE) as u8, // r
        (g.clamp(0.0, 0.999) * CONV_TO_BYTE) as u8, // g
        (b.clamp(0.0, 0.999) * CONV_TO_BYTE) as u8, // b
    );

    writeln!(file, "{} {} {}", r, g, b).unwrap();
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f32 / ASPECT_RATIO) as usize;
    let spp = 100;
    let max_depth = 50;

    // World
    let world: &[Box<dyn Hittable>] = &[
        // Ground
        Box::new(Sphere {
            center: [0.0, -100.5, -1.0].into(),
            radius: 100.0,
            mat: Rc::new(Lambertian {
                albedo: [0.8, 0.8, 0.0].into(),
            }),
        }),
        // Center
        Box::new(Sphere {
            center: [0.0, 0.0, -1.0].into(),
            radius: 0.5,
            mat: Rc::new(Lambertian{ albedo: [0.1, 0.2, 0.5].into() }),
        }),
        // Left
        Box::new(Sphere {
            center: [-1.0, 0.0, -1.0].into(),
            radius: 0.5,
            mat: Rc::new(Dieletric { ir: 1.5 }),
        }),
        Box::new(Sphere {
            center: [-1.0, 0.0, -1.0].into(),
            radius: -0.4,
            mat: Rc::new(Dieletric { ir: 1.5 }),
        }),
        // Right
        Box::new(Sphere {
            center: [1.0, 0.0, -1.0].into(),
            radius: 0.5,
            mat: Rc::new(Metal {
                albedo: [0.8, 0.6, -0.2].into(),
                fuzz: 0.0,
            }),
        }),
    ];

    // Camera
    let camera = Camera::default();

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
                    let u = (i as f32 + random_float()) / (img_width - 1) as f32;
                    let v = (j as f32 + random_float()) / (img_height - 1) as f32;
                    pixel_color += ray_color(&camera.ray(u, v), world, max_depth);
                }
                pixel_color
            };
            write_ppm_color(&mut file, pixel_color, spp);
        }
        progress_bar.inc(); // +1% Progress bar
    }
    progress_bar.finish(); // End of progress
}
