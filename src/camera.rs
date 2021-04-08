use super::ray::*;
use super::vec3::*;

#[derive(Clone, Copy)]
pub struct Recti {
    pub height: i32,
    pub width: i32,
}

pub struct Rectf {
    pub width: f32,
    pub height: f32,
}

pub struct Camera {
    screen: Recti,
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    wuv: (Vec3, Vec3, Vec3),
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        up: &Vec3,
        fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let screen_width = 640;
        let screen = Recti {
            width: screen_width,
            height: (screen_width as f32 / aspect_ratio) as i32,
        };
        let viewport_height = 2.0 * h;
        let viewport = Rectf {
            height: viewport_height,
            width: aspect_ratio * viewport_height,
        };

        let w = (*look_from - *look_at).unit();
        let u = cross(up, &w).unit();
        let v = cross(&w, &u);

        let origin = *look_from;
        let horizontal = u.scale(viewport.width * focus_distance);
        let vertical = v.scale(viewport.height * focus_distance);
        let lower_left_corner =
            origin - horizontal.scale(0.5) - vertical.scale(0.5) - w.scale(focus_distance);

        let lens_radius = aperture / 2.0;

        Self {
            screen,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            wuv: (w, u, v),
        }
    }

    pub fn screen(&self) -> Recti {
        self.screen
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = super::random_in_unit_disk().scale(self.lens_radius);
        let offset = self.wuv.1.scale(rd.x()) + self.wuv.2.scale(rd.y());
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal.scale(s) + self.vertical.scale(t)
                - self.origin
                - offset,
        )
    }
}
