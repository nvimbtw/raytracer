use crate::vector::{subtract_vec, dot, Vec3};
use raylib::prelude::Color;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
    pub roughness: f32,
    pub emissive: bool,
    pub emission_color: Color,
}

pub fn ray_sphere_intersect(
    origin: Vec3,
    direction: Vec3,
    sphere: Sphere,
) -> Option<f32> {
    let l = subtract_vec(origin, sphere.center);
    let a = dot(direction, direction);
    let b = 2.0 * dot(direction, l);
    let c = dot(l, l) - (sphere.radius * sphere.radius);

    let discriminant = (b * b) - 4.0 * a * c;

    if discriminant >= 0.0 {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        return Some(t);
    }

    None
}
