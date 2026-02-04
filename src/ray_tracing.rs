use crate::bodies::{ray_plane_intersect, ray_sphere_intersect};
use crate::geometry::SceneObject;
use crate::vector::*;
use rand::Rng;
use raylib::prelude::Color;

pub fn scr_to_ndc_f32(
    x: f32,
    y: f32,
    win_w: i32,
    win_h: i32,
) -> (f32, f32) {
    let aspect_ratio = win_w as f32 / win_h as f32;
    let ndc_x = ((2.0 * x) / win_w as f32 - 1.0) * aspect_ratio;
    let ndc_y = 1.0 - (2.0 * y) / win_h as f32;
    (ndc_x, ndc_y)
}

pub fn perfect_reflection(direction: Vec3, normal: Vec3) -> Vec3 {
    let scalar_component = 2.0 * dot(direction, normal);
    let scaled_normal = multiply_vec_scalar(normal, scalar_component);
    let reflection = subtract_vec(direction, scaled_normal);
    reflection.normalize()
}

pub fn random_cosine_direction(normal: Vec3) -> Vec3 {
    let mut rng = rand::rng();

    let r1 = rng.random::<f32>();
    let r2 = rng.random::<f32>();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let cos_theta = (1.0 - r2).sqrt();
    let sin_theta = r2.sqrt();

    let x = phi.cos() * sin_theta;
    let y = phi.sin() * sin_theta;
    let z = cos_theta;

    let (tangent, bitangent) = create_coordinate_system(normal);

    Vec3 {
        x: tangent.x * x + bitangent.x * y + normal.x * z,
        y: tangent.y * x + bitangent.y * y + normal.y * z,
        z: tangent.z * x + bitangent.z * y + normal.z * z,
    }
}

pub fn create_coordinate_system(normal: Vec3) -> (Vec3, Vec3) {
    let tangent = if normal.x.abs() > 0.9 {
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    } else {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    };

    let bitangent = Vec3 {
        x: normal.y * tangent.z - normal.z * tangent.y,
        y: normal.z * tangent.x - normal.x * tangent.z,
        z: normal.x * tangent.y - normal.y * tangent.x,
    }
    .normalize();

    let tangent = Vec3 {
        x: bitangent.y * normal.z - bitangent.z * normal.y,
        y: bitangent.z * normal.x - bitangent.x * normal.z,
        z: bitangent.x * normal.y - bitangent.y * normal.x,
    }
    .normalize();

    (tangent, bitangent)
}

pub fn trace_ray(
    x: i32,
    y: i32,
    jitter_x: f32,
    jitter_y: f32,
    win_w: i32,
    win_h: i32,
    scene: &Vec<SceneObject>,
    max_bounces: i32,
) -> Vec3 {
    let pixel_x = x as f32 + jitter_x;
    let pixel_y = y as f32 + jitter_y;

    let (ndc_x, ndc_y) = scr_to_ndc_f32(pixel_x, pixel_y, win_w, win_h);
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let direction = normalize(Vec3 {
        x: ndc_x,
        y: ndc_y,
        z: -1.0,
    });
    cast_ray(origin, direction, scene, max_bounces)
}

pub fn cast_ray(
    origin: Vec3,
    direction: Vec3,
    scene: &Vec<SceneObject>,
    bounces: i32,
) -> Vec3 {
    if bounces <= 0 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    let mut closest_t = f32::MAX;
    let mut hit_normal = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut hit_color = Color::BLACK;
    let mut hit_emissive = false;
    let mut hit_emission_color = Color::BLANK;
    let mut hit_roughness = 0.0;

    for obj in scene {
        match obj {
            SceneObject::SphereObj(sphere) => {
                if let Some(t) =
                    ray_sphere_intersect(origin, direction, *sphere)
                {
                    if t > 0.0 && t < closest_t {
                        closest_t = t;
                        let hit_point =
                            add_vec(origin, scale_vec(direction, t));
                        hit_normal = normalize(subtract_vec(
                            hit_point,
                            sphere.center,
                        ));
                        hit_color = sphere.color;
                        hit_emissive = sphere.emissive;
                        hit_emission_color = sphere.emission_color;
                        hit_roughness = sphere.roughness;
                    }
                }
            }
            SceneObject::PlaneObj(plane) => {
                if let Some(t) =
                    ray_plane_intersect(origin, direction, *plane)
                {
                    if t > 0.0 && t < closest_t {
                        closest_t = t;
                        let hit_point =
                            add_vec(origin, scale_vec(direction, t));
                        hit_normal = plane.normal;
                        hit_color = plane.get_color_at(hit_point);
                        hit_emissive = plane.emissive;
                        hit_emission_color = plane.emission_color;
                        hit_roughness = plane.roughness;
                    }
                }
            }
        }
    }

    if closest_t < f32::MAX {
        let hit_point = add_vec(origin, scale_vec(direction, closest_t));

        let emitted_light = if hit_emissive {
            let emission_strength = 7.5;
            scale_vec(color_to_vec3(hit_emission_color), emission_strength)
        } else {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        };

        let perfect_refl = perfect_reflection(direction, hit_normal);
        let random_dir = random_cosine_direction(hit_normal);

        let reflection_dir =
            lerp_vec3(perfect_refl, random_dir, hit_roughness).normalize();

        let new_origin = add_vec(hit_point, scale_vec(hit_normal, 0.0001));
        let bounced_light =
            cast_ray(new_origin, reflection_dir, scene, bounces - 1);

        let material_albedo = color_to_vec3(hit_color);
        let reflected_light =
            multiply_vecs(material_albedo, bounced_light);

        return add_vec(emitted_light, reflected_light);
    }

    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}
