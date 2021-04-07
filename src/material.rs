

use super::hittable::HitResult;
use super::vec3::{dot, Vec3};
use super::Color;
use super::Ray;

pub fn reflect(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    *lhs - rhs.scale(dot(&lhs, &rhs) * 2.0)
}

pub fn refract(uv: &Vec3, n: &Vec3, factor: f32) -> Vec3 {
    let cos_theta = dot(&uv.scale(-1.0), n).min(1.0);
    let r_out_perp = (*uv + n.scale(cos_theta)).scale(factor);
    let r_out_parallel = n.scale(-1.0 * ((1.0 - r_out_perp.length_squared()).abs()).sqrt());
    r_out_perp + r_out_parallel
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_result: &HitResult) -> ScatterResult {
        let scatter_direction = {
            let direction = hit_result.normal + super::random_unit_vector();
            if direction.near_zero() {
                hit_result.normal
            } else {
                direction
            }
        };
        let ray = Ray::new(hit_result.location, scatter_direction);
        let attenuation = self.albedo;
        ScatterResult { ray, attenuation }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> ScatterResult {
        let scatter_direction = reflect(&ray.direction().unit(), &hit_result.normal);
        let ray = Ray::new(
            hit_result.location,
            scatter_direction + super::random_in_unit_sphere().scale(self.fuzz),
        );
        let attenuation = self.albedo;
        ScatterResult { ray, attenuation }
    }
}

pub struct Dieletric {
    ir: f32,
}

impl Dieletric {
    pub fn new(ir: f32) -> Dieletric {
        Self { ir }
    }

    fn reflectance(cos: f32, refraction_idx: f32) -> f32 {
        let r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
        let r0 = r0.powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos).powf(2.0)
    }
}

impl Material for Dieletric {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> ScatterResult {
        let refraction_ratio = if hit_result.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = dot(&unit_direction.scale(-1.0), &hit_result.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();
        let direction = if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > super::random_float()
        {
            reflect(&unit_direction, &hit_result.normal)
        } else {
            refract(&unit_direction, &hit_result.normal, refraction_ratio)
        };

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ray = Ray::new(hit_result.location, direction);

        ScatterResult { attenuation, ray }
    }
}
