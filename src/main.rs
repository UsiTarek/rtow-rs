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

pub const PI: f32 = 3.14;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    (180.0 * degrees) / PI
}

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

pub fn random_in_unit_disk() -> Point3 {
    loop {
        let p = Vec3::new(random_float_mm(-1.0, 1.0), random_float_mm(-1.0, 1.0), 0.0);
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

fn ray_color(ray: &Ray, hittables: &[Box<dyn Hittable>], depth: i32) -> Color {
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

const MAX_DEPTH: i32 = 8;
const SPP: i32 = 100;

fn main() {
    // Camera
    let look_from = &Vec3::new(13.0, 2.0, 3.0);
    let look_at = &Vec3::new(0.0, 0.0, 0.0);
    let distance_to_focus = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        3.0 / 2.0,
        0.1,
        distance_to_focus,
    );

    // Render
    let mut hittables: Vec<Box<dyn Hittable>> = vec![];

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    hittables.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::new(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    //Diffuse :
                    let albedo = Color::new_random() * Color::new_random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Color::new_random_mm(0.5, 1.0);
                    let fuzz = random_float_mm(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Rc::new(Dieletric::new(1.5));
                }

                hittables.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dieletric::new(1.5));
    hittables.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    hittables.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    hittables.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

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
