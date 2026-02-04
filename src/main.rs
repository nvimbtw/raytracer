mod bodies;
mod geometry;
mod ray_tracing;
mod vector;

use geometry::generate_scene;
use rand::Rng;
use ray_tracing::trace_ray;
use raylib::prelude::*;
use rayon::prelude::*;
use std::env;
use vector::{Vec3, add_vec, scale_vec, vec3_to_color};

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_bounces = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or(5)
    } else {
        5
    };

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

        let samples_per_frame = 4;
        let row_results: Vec<Vec<Vec3>> = (0..win_h)
            .into_par_iter()
            .map(|y| {
                let mut rng = rand::rng();
                (0..win_w)
                    .map(|x| {
                        let mut color = Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        };
                        for _ in 0..samples_per_frame {
                            let jitter_x = rng.random::<f32>();
                            let jitter_y = rng.random::<f32>();
                            color = add_vec(
                                color,
                                trace_ray(
                                    x,
                                    y,
                                    jitter_x,
                                    jitter_y,
                                    win_w,
                                    win_h,
                                    &scene,
                                    max_bounces,
                                ),
                            );
                        }
                        scale_vec(color, 1.0 / samples_per_frame as f32)
                    })
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
