mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::{write_color, Color};
use crossterm::{cursor, QueueableCommand};
use hittable::{Hittable, HittableList};
use material::{Dieletric, Lambertian, Material, Metal};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::{
    io::{prelude::*, stderr},
    rc::Rc,
};
use vec3::*;

pub fn random_float() -> f32 {
    let mut thread_rng = rand::thread_rng();
    thread_rng.gen_range(0.0f32..1.0f32)
}

pub fn random_float_mm(min: f32, max: f32) -> f32 {
    let mut thread_rng = rand::thread_rng();
    thread_rng.gen_range(min..max)
}

pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let p = Vec3::new_random_mm(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

fn write_progress(progress: i32) {
    stderr().queue(cursor::SavePosition).unwrap();
    stderr()
        .write_fmt(format_args!("Remaining scanlines : {}", progress))
        .unwrap();
    stderr().queue(cursor::RestorePosition).unwrap();
    stderr().flush().unwrap();
}

fn ray_color(ray: &Ray, hittables: &[&dyn Hittable], depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_result) = hittables.hit(ray, 0.001, f32::MAX) {
        let scatter = hit_result.material.scatter(ray, &hit_result);
        ray_color(&scatter.ray, hittables, depth - 1) * scatter.attenuation
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0).scale(1.0 - t) + Color::new(0.5, 0.7, 1.0).scale(t)
    }
}

const MAX_DEPTH: i32 = 50;
const SPP: i32 = 100;

fn main() {
    // Camera
    let camera = Camera::new();

    // Render
    let materials: [Rc<dyn Material>; 4] = [
        Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
        Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
        Rc::new(Dieletric::new(1.5)),
        Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)),
    ];

    let hittables: [&dyn Hittable; 5] = [
        &Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, materials[0].clone()),
        &Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, materials[1].clone()),
        &Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, materials[2].clone()),
        &Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, materials[2].clone()),
        &Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, materials[3].clone()),
    ];

    let screen = camera.screen();
    print!("P3\n{} {}\n255\n", screen.width, screen.height);

    let mut stdout = std::io::stdout();
    for j in (0..screen.height).rev() {
        write_progress(j);
        for i in 0..screen.width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SPP {
                let u = (i as f32 + random_float()) / (screen.width - 1) as f32;
                let v = (j as f32 + random_float()) / (screen.height - 1) as f32;
                pixel_color += ray_color(&camera.get_ray(u, v), &hittables, MAX_DEPTH);
            }
            write_color(&mut stdout, pixel_color, SPP);
        }
    }
    stderr().write("\nDone.".as_bytes()).unwrap();
}
