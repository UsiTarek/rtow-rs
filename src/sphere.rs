use std::rc::Rc;

use super::hittable::*;
use super::material::Material;
use super::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let origin_to_center = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&origin_to_center, &ray.direction());
        let c = origin_to_center.length_squared() - self.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - (a * c);
        let sqrtd = discriminant.sqrt();

        let t: Option<f32> = if discriminant >= 0.0 {
            let t_candidate = (-half_b - sqrtd) / a;
            if t_candidate > t_min && t_max > t_candidate {
                Some(t_candidate)
            } else {
                let t_candidate = (-half_b + sqrtd) / a;

                if t_candidate > t_min && t_max > t_candidate {
                    Some(t_candidate)
                } else {
                    None
                }
            }
        } else {
            None
        };

        if let Some(t) = t {
            let mut normal = (ray.at(t) - self.center).scale(1.0 / self.radius);
            let mut front_face = true;
            if dot(&ray.direction(), &normal) > 0.0 {
                normal = normal.scale(-1.0);
                front_face = false;
            }

            Some(HitResult::new(
                ray.at(t),
                normal,
                t,
                front_face,
                self.material.clone(),
            ))
        } else {
            None
        }
    }
}
