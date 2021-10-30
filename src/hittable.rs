use super::{Material, Point3, Ray, Vec3};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub hit_point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discr = (half_b * half_b) - (a * c);

        if discr < 0.0 {
            None
        } else {
            let discr_sqrt = discr.sqrt();

            /* In Rust, we use block expressions to calculate the final value of t.
             * Here we only have mutable 't' within the expression block.
             * We change t depending on conditions, then we return it to an immutable 't'
             * which will be used in the parent block.
             * (the parent block is also an expression that returns the whole function's value)
             * (https://doc.rust-lang.org/reference/expressions/block-expr.html)
             */
            let t = {
                let mut t = (-half_b - discr_sqrt) / a;
                if t < t_min || t_max < t {
                    t = (-half_b + discr_sqrt) / a;

                    if t < t_min || t_max < t {
                        return None;
                    };
                }

                Some(t)
            };

            match t {
                Some(t) => {
                    let hit_point = r.at(t);
                    let outward_normal = (hit_point - self.center) / self.radius;
                    let front_face = r.direction().dot(&outward_normal) <= 0.0;
                    let normal = {
                        if front_face {
                            outward_normal
                        } else {
                            outward_normal * -1.0
                        }
                    };
                    let mat = self.mat.clone();

                    Some(HitRecord {
                        t,
                        hit_point,
                        normal,
                        front_face,
                        mat,
                    })
                }
                None => None,
            }
        }
    }
}

// Here we implement the 'Hittable' trait on a slice of hittable objects that also implement the
// same trait.
// This enables the possibility of running hit() for an array of different types of geometry.
// You can also run hit() on multi-dimensional slices (arrays of array) because of this impl.
impl Hittable for &[Box<dyn Hittable>] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        self.iter().for_each(|hittable| {
            if let Some(hr) = hittable.hit(r, t_min, t_max) {
                if let Some(ch) = closest_hit.clone() {
                    if hr.t < ch.t {
                        closest_hit = Some(hr)
                    }
                } else {
                    closest_hit = Some(hr);
                }
            }
        });

        closest_hit
    }
}
