use super::ray::*;
use super::vec3::*;

#[derive(Clone, Copy)]
pub struct Recti {
    pub height: i32,
    pub width: i32,
}

pub struct Rectf {
    pub height: f32,
    pub width: f32,
}

pub struct Camera {
    screen: Recti,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let focal_length = 1.0;
        let screen_width = 400;
        let screen = Recti {
            height: (screen_width as f32 / aspect_ratio) as i32,
            width: screen_width,
        };
        let viewport = Rectf {
            height: 2.0,
            width: aspect_ratio * 2.0,
        };
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport.width as f32, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport.height as f32, 0.0);
        let lower_left_corner = origin
            - horizontal.scale(0.5)
            - vertical.scale(0.5)
            - Vec3::new(0.0, 0.0, focal_length);

        Self {
            screen,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn screen(&self) -> Recti {
        self.screen
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
                - self.origin,
        )
    }
}
