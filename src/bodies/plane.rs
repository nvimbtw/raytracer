use crate::vector::{subtract_vec, dot, Vec3};
use raylib::prelude::Color;

#[derive(Copy, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Color,
    pub roughness: f32,
    pub is_checkered: bool,
    pub emissive: bool,
    pub emission_color: Color,
}

impl Plane {
    pub fn get_color_at(&self, hit_point: Vec3) -> Color {
        if !self.is_checkered {
            return self.color;
        }

        let check_size = 0.5;
        
        // Create a simple 2D mapping based on the plane's orientation
        let (u, v) = if self.normal.y.abs() > 0.9 {
            (hit_point.x, hit_point.z)
        } else if self.normal.x.abs() > 0.9 {
            (hit_point.y, hit_point.z)
        } else {
            (hit_point.x, hit_point.y)
        };

        let is_white = ((u / check_size).floor() as i32 + (v / check_size).floor() as i32) % 2 == 0;
        
        if is_white { 
            Color::WHITE 
        } else { 
            // A more realistic dark gray to catch light and reflections
            Color::new(60, 60, 60, 255) 
        }
    }
}

pub fn ray_plane_intersect(
    origin: Vec3,
    direction: Vec3,
    plane: Plane,
) -> Option<f32> {
    let denom = dot(direction, plane.normal);

    if denom.abs() < 0.0001 {
        return None;
    }

    let p_to_origin = subtract_vec(plane.point, origin);
    let t = dot(p_to_origin, plane.normal) / denom;

    if t > 0.0 { Some(t) } else { None }
}