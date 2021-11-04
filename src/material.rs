use crate::{reflect, refract};

use super::{
    hittable::HitRecord, random_in_unit_sphere, random_in_unit_vector, ray::Ray, vec3::Color,
};

/* NOTE(Tarik) How to represent a material in Rust:
 *
 * In C++, a popular choice for a material type would be an abstract class.
 * Here, we can choose between enums (which can contain custom data) or
 * traits which are the rust equivalent of abstract classes in C++.
 *
 *
 * On a personal implementation, I would probably go for enums for data locality.
 * You can declare an enum like this :
 * ``
 *    enum Material {
 *        Lambertian(LambertianData),
 *        Metal(MetalData),
 *    }
 * ``
 * You can then implement 'scatter()' for the enum using pattern matching.
 *
 * To conform with Ray Tracing In One Weekend, I chose to use traits.
 */

// Here is the implementation with a trait:

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<ScatterResult> {
        let direction = {
            let direction = hr.normal + random_in_unit_vector();

            if direction.nearly_zero() {
                hr.normal
            } else {
                direction
            }
        };

        Some(ScatterResult {
            scattered: Ray::new(hr.hit_point, direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<ScatterResult> {
        let reflected = super::reflect(r.direction().unit(), hr.normal);
        let scattered = Ray::new(
            hr.hit_point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        if scattered.direction().dot(&hr.normal) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dieletric {
    pub ir: f32,
}

impl Dieletric{
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx); 
        let r0 = r0*r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = {
            if hr.front_face {
                1.0 / self.ir
            } else {
                self.ir
            }
        };

        let unit_direction = r.direction().unit();
        let cos_theta = (unit_direction * -1.0).dot(&hr.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction = {
            if (refraction_ratio * sin_theta > 1.0) || Self::reflectance(cos_theta, refraction_ratio) > super::random_float() {
                reflect(unit_direction, hr.normal)
            }
            else {
                refract(unit_direction, hr.normal, refraction_ratio)
            }
        };

        Some(ScatterResult {
            attenuation,
            scattered: Ray::new(hr.hit_point, direction),
        })
    }
}
