use super::vec3::*;
use std::io::*;

pub type Color = Vec3;

pub fn write_color(out: &mut std::io::Stdout, pixel: Color, spp: i32) {
    let mut r = pixel.x();
    let mut g = pixel.y();
    let mut b = pixel.z();

    let scale = 1.0 / (spp as f32);

    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    out.write_fmt(format_args!(
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32,
    ))
    .expect("Couldn't write color to stdout.");
}
