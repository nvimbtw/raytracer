use raylib::prelude::Color;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
}

pub fn add_vec(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

pub fn subtract_vec(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

pub fn multiply_vec_scalar(vec: Vec3, scalar: f32) -> Vec3 {
    Vec3 {
        x: vec.x * scalar,
        y: vec.y * scalar,
        z: vec.z * scalar,
    }
}

pub fn multiply_vecs(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
}

pub fn scale_vec(a: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: a.x * t,
        y: a.y * t,
        z: a.z * t,
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: a.x * (1.0 - t) + b.x * t,
        y: a.y * (1.0 - t) + b.y * t,
        z: a.z * (1.0 - t) + b.z * t,
    }
}

pub fn normalize(v: Vec3) -> Vec3 {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    Vec3 {
        x: v.x / length,
        y: v.y / length,
        z: v.z / length,
    }
}

pub fn vec3_to_color(vec: Vec3) -> Color {
    Color {
        r: (vec.x * 255.0).min(255.0) as u8,
        g: (vec.y * 255.0).min(255.0) as u8,
        b: (vec.z * 255.0).min(255.0) as u8,
        a: 255,
    }
}

pub fn color_to_vec3(color: Color) -> Vec3 {
    Vec3 {
        x: color.r as f32 / 255.0,
        y: color.g as f32 / 255.0,
        z: color.b as f32 / 255.0,
    }
}
