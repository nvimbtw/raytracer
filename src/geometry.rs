use crate::bodies::{Plane, Sphere};
use crate::vector::Vec3;
use raylib::prelude::Color;

#[derive(Copy, Clone)]
pub enum SceneObject {
    SphereObj(Sphere),
    PlaneObj(Plane),
}

fn add_room_planes(scene: &mut Vec<SceneObject>) {
    // INFO: Floor (Checkered)
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
        color: Color::WHITE,
        roughness: 0.1,
        is_checkered: true,
        emissive: false,
        emission_color: Color::BLANK,
    }));

    // INFO: Ceiling
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
        roughness: 0.8,
        is_checkered: false,
        emissive: false,
        emission_color: Color::BLANK,
    }));

    // INFO: Back wall
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
        roughness: 0.08,
        is_checkered: false,
        emissive: false,
        emission_color: Color::BLANK,
    }));

    // INFO: Left wall (Reddish)
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
        color: Color::new(200, 100, 100, 255),
        roughness: 0.08,
        is_checkered: false,
        emissive: false,
        emission_color: Color::BLANK,
    }));

    // INFO: Right wall (Greenish)
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
        color: Color::new(100, 200, 100, 255),
        roughness: 0.08,
        is_checkered: false,
        emissive: false,
        emission_color: Color::BLANK,
    }));
}

pub fn generate_scene() -> Vec<SceneObject> {
    let mut scene: Vec<SceneObject> = Vec::new();

    // INFO: Ceiling light
    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.7,
            z: -2.5,
        },
        radius: 0.4,
        color: Color::GHOSTWHITE,
        roughness: 1.0,
        emissive: true,
        emission_color: Color::GHOSTWHITE,
    }));

    // INFO: Small reflective sphere in the center
    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -0.675,
            z: -2.5,
        },
        radius: 0.3,
        color: Color::WHITE,
        roughness: 0.0, // Fully reflective
        emissive: false,
        emission_color: Color::BLANK,
    }));

    // INFO: Large matte blue ball
    scene.push(SceneObject::SphereObj(Sphere {
        center: Vec3 {
            x: 1.0,
            y: -0.0,
            z: -4.0,
        },
        radius: 1.2,
        color: Color::SKYBLUE,
        roughness: 0.75,
        emissive: false,
        emission_color: Color::BLANK,
    }));

    add_room_planes(&mut scene);

    scene
}
