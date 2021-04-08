use std::rc::Rc;

use super::material::*;
use super::ray::*;
use super::vec3::*;

#[derive(Clone)]
pub struct HitResult {
    pub location: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitResult {
    pub fn new(
        location: Point3,
        normal: Vec3,
        t: f32,
        front_face: bool,
        material: Rc<dyn Material>,
    ) -> HitResult {
        Self {
            location,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

pub trait HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

impl HittableList for &[Box<dyn Hittable>] {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut closest: Option<HitResult> = None;
        let mut closest_so_far = t_max;

        self.iter().for_each(|hittable| {
            if let Some(hitresult) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hitresult.t;
                closest = Some(hitresult);
            }
        });

        closest
    }
}
