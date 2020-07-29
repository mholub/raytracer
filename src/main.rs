mod camera;
mod intersections;
mod material;
mod perlin;
mod ppm;
mod random;
mod scenes;
mod texture;
mod types;

use crate::camera::*;
use crate::intersections::*;
use crate::material::*;
use crate::ppm::*;
use crate::random::*;
use crate::types::*;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;

fn ray_color(ray: &Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(&ray, 0.001, f32::INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
            attenuation.component_mul(&ray_color(&scattered, &world, depth - 1))
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

pub fn main() {
    //let scene = scenes::random_spheres::scene();
    //let scene = scenes::two_spheres::scene();
    let scene = scenes::two_perlin_spheres::scene();

    let aspect_ratio = 16.0 / 9.0;

    let focus_dist = 10.0;

    let cam = Camera::new(
        &scene.lookfrom,
        &scene.lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        scene.vfov,
        aspect_ratio,
        scene.aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let path = Path::new("target/image.ppm");
    let mut file = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .unwrap(),
    );

    write_header(&mut file, image_width, image_height);

    let pb = ProgressBar::new((image_height * image_width) as u64);
    pb.set_draw_target(ProgressDrawTarget::stdout());
    pb.set_draw_delta(pb.length() / 100);
    pb.set_style(ProgressStyle::default_bar().template(
        "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
    ));

    let pixels = (0..image_height)
        .rev()
        .flat_map(|j| (0..image_width).map(move |i| (j, i)))
        .collect::<Vec<_>>()
        .into_par_iter()
        .chunks(1024)
        .map(|chunk| {
            chunk
                .iter()
                .map(|(j, i)| {
                    let mut pixel_color: Color = (0..samples_per_pixel)
                        .into_iter()
                        .map(|_i| {
                            let u = (*i as f32 + rand()) / (image_width - 1) as f32;
                            let v = (*j as f32 + rand()) / (image_height - 1) as f32;

                            let r = cam.get_ray(u, v);
                            ray_color(&r, &scene.world, max_depth)
                        })
                        .sum();

                    pixel_color /= samples_per_pixel as f32;
                    pixel_color = Color::new(
                        pixel_color.x.sqrt(),
                        pixel_color.y.sqrt(),
                        pixel_color.z.sqrt(),
                    );
                    pb.inc(1);
                    pixel_color
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    pb.finish();

    for pixel_color in pixels {
        pixel_color.write_ppm(&mut file);
    }

    Command::new("open").arg(path).spawn().unwrap();
}
