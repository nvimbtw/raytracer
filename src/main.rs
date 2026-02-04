use rand::Rng;
use raylib::prelude::*;
use rayon::prelude::*;

#[derive(Copy, Clone)]
enum SceneObject {
    SphereObj(Sphere),
    PlaneObj(Plane),
}

#[derive(Copy, Clone)]
struct Plane {
    point: Vec3,
    normal: Vec3,
    color: Color,
    emissive: bool,
    emission_color: Color,
}

#[derive(Copy, Clone)]
struct Sphere {
    center: Vec3,
    radius: f32,
    color: Color,
    emissive: bool,
    emission_color: Color,
}

#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Vec3 {
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

fn add_vec(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

fn subtract_vec(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

fn multiply_vec_scalar(vec: Vec3, scalar: f32) -> Vec3 {
    Vec3 {
        x: vec.x * scalar,
        y: vec.y * scalar,
        z: vec.z * scalar,
    }
}

fn multiply_vecs(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
}

fn scale_vec(a: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: a.x * t,
        y: a.y * t,
        z: a.z * t,
    }
}

fn generate_scene() -> Vec<SceneObject> {
    let mut scene: Vec<SceneObject> = Vec::new();

    // Main white light source (top center)
    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.5,
            z: -3.5,
        },
        radius: 0.4,
        color: Color::WHITE,
        emissive: true,
        emission_color: Color::WHITE,
    }));

    // Warm accent light (left side)
    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: -1.2,
            y: 0.3,
            z: -2.8,
        },
        radius: 0.25,
        color: Color::new(150, 150, 100, 255),
        emissive: true,
        emission_color: Color::new(150, 150, 100, 255),
    }));

    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: -0.7,
            y: -0.3,
            z: -2.5,
        },
        radius: 0.3,
        color: Color::new(255, 255, 255, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 0.8,
            y: -0.4,
            z: -2.7,
        },
        radius: 0.65,
        color: Color::new(50, 100, 220, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.1,
            z: -2.2,
        },
        radius: 0.35,
        color: Color::new(255, 100, 255, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    // Green sphere (back right) - adds depth
    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 1.3,
            y: 0.2,
            z: -4.0,
        },
        radius: 0.5,
        color: Color::new(50, 200, 80, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::PlaneObj(Plane {
        point: Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        color: Color::new(200, 200, 200, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::PlaneObj(Plane {
        point: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        color: Color::new(220, 220, 220, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::PlaneObj(Plane {
        point: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        color: Color::new(180, 180, 200, 255),
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::PlaneObj(Plane {
        point: Vec3 {
            x: -2.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        color: Color::new(200, 100, 100, 255), // Reddish
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene.push(SceneObject::PlaneObj(Plane {
        point: Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        color: Color::new(100, 200, 100, 255), // Greenish
        emissive: false,
        emission_color: Color::BLANK,
    }));

    scene
}

fn scr_to_ndc(x: i32, y: i32, win_w: i32, win_h: i32) -> (f32, f32) {
    let aspect_ratio = win_w as f32 / win_h as f32;

    let ndc_x = (2.0 * x as f32 / win_w as f32) - 1.0;
    let ndc_x = ndc_x * aspect_ratio;
    let ndc_y = 1.0 - (2.0 * y as f32 / win_h as f32);

    (ndc_x, ndc_y)
}

fn square(x: f32) -> f32 {
    x * x
}

fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3 {
        x: a.x * (1.0 - t) + b.x * t,
        y: a.y * (1.0 - t) + b.y * t,
        z: a.z * (1.0 - t) + b.z * t,
    }
}

fn perfect_reflection(direction: Vec3, normal: Vec3) -> Vec3 {
    let scalar_component = 2.0 * dot(direction, normal);
    let scaled_normal = multiply_vec_scalar(normal, scalar_component);
    let reflection = subtract_vec(direction, scaled_normal);
    reflection.normalize()
}

fn normalize(v: Vec3) -> Vec3 {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    Vec3 {
        x: v.x / length,
        y: v.y / length,
        z: v.z / length,
    }
}

fn ray_plane_intersect(
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

fn ray_sphere_intersect(
    origin: Vec3,
    direction: Vec3,
    sphere: Sphere,
) -> Option<f32> {
    let l = subtract_vec(origin, sphere.center);
    let a = dot(direction, direction);
    let b = 2.0 * dot(direction, l);
    let c = dot(l, l) - square(sphere.radius);

    let discriminant = square(b) - 4.0 * a * c;

    if discriminant >= 0.0 {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);

        return Some(t);
    }

    None
}

fn random_cosine_direction(normal: Vec3) -> Vec3 {
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

fn create_coordinate_system(normal: Vec3) -> (Vec3, Vec3) {
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

fn trace_ray(
    x: i32,
    y: i32,
    win_w: i32,
    win_h: i32,
    scene: &Vec<SceneObject>,
    max_bounces: i32,
) -> Vec3 {
    let (ndc_x, ndc_y) = scr_to_ndc(x, y, win_w, win_h);
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

fn cast_ray(
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
    let mut hit_object: Option<SceneObject> = None;

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
                        hit_object = Some(*obj);
                    }
                }
            }
            SceneObject::PlaneObj(plane) => {
                if let Some(t) =
                    ray_plane_intersect(origin, direction, *plane)
                {
                    if t > 0.0 && t < closest_t {
                        closest_t = t;
                        hit_normal = plane.normal;
                        hit_color = plane.color;
                        hit_emissive = plane.emissive;
                        hit_emission_color = plane.emission_color;
                        hit_object = Some(*obj);
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

        let roughness = match hit_object.unwrap() {
            SceneObject::PlaneObj(_) => 0.90,
            SceneObject::SphereObj(_) => 0.90,
        };

        let reflection_dir =
            lerp_vec3(perfect_refl, random_dir, roughness).normalize();

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

fn vec3_to_color(vec: Vec3) -> Color {
    Color {
        r: (vec.x * 255.0).min(255.0) as u8,
        g: (vec.y * 255.0).min(255.0) as u8,
        b: (vec.z * 255.0).min(255.0) as u8,
        a: 255,
    }
}

fn color_to_vec3(color: Color) -> Vec3 {
    Vec3 {
        x: color.r as f32 / 255.0,
        y: color.g as f32 / 255.0,
        z: color.b as f32 / 255.0,
    }
}

fn main() {
    let win_w = 1200;
    let win_h = 700;

    let scene = generate_scene();

    let (mut raylib, thread) = raylib::init()
        .size(win_w, win_h)
        .title("Hello, Raytracer")
        .build();

    let mut accumulation_buffer = vec![
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        };
        (win_w * win_h) as usize
    ];
    let mut frame_count = 0.0;

    while !raylib.window_should_close() {
        frame_count += 1.0;

        let row_results: Vec<Vec<Vec3>> = (0..win_h)
            .into_par_iter()
            .map(|y| {
                (0..win_w)
                    .map(|x| trace_ray(x, y, win_w, win_h, &scene, 5))
                    .collect()
            })
            .collect();

        let mut screen = raylib.begin_drawing(&thread);

        for y in 0..win_h {
            for x in 0..win_w {
                let index = (y * win_w + x) as usize;

                accumulation_buffer[index] = add_vec(
                    accumulation_buffer[index],
                    row_results[y as usize][x as usize],
                );

                let avg_color_vec = scale_vec(
                    accumulation_buffer[index],
                    1.0 / frame_count,
                );
                let avg_color = vec3_to_color(avg_color_vec);

                screen.draw_pixel(x, y, avg_color);
            }
        }
    }
}
