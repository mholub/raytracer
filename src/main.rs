mod texture;
mod types;
mod camera;
mod intersections;
mod material;
mod ppm;
mod random;

use std::io::{BufWriter};
use std::path::Path;
use std::fs::{OpenOptions};
use std::process::Command;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, ProgressDrawTarget};
use crate::intersections::*;
use crate::types::*;
use crate::material::*;
use crate::camera::*;
use crate::ppm::*;
use crate::random::*;

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

fn make_random_world() -> World {
    let mut world = World::new();

    let material_ground = Material::Lambertian(Lambertian::from_colors(Color::new(0.2, 0.3, 0.1),
                                                                       Color::new(0.9, 0.9, 0.9)));

    world.add(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: material_ground.clone(),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand();
            let center = Vec3::new(a as f32 + 0.9 * rand(), 0.2, b as f32 + rand());
            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                        world.add(MovingSphere {
                            center1: center,
                            center2: center2,
                            time1: 0.0,
                            time2: 1.0,
                            radius: 0.2,
                            material: Material::Lambertian(
                                Lambertian::from_color(Color::new(rand(), rand(), rand()).component_mul(&Color::new(rand(), rand(), rand()))))
                        });
                    }
                    x if x < 0.95 => {
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            material: Material::Metal(Metal::new(
                                Color::new(rand_range(0.5, 1.0), rand_range(0.5, 1.0), rand_range(0.5, 1.0)),
                                rand_range(0.0, 0.5)))
                        });
                    },
                    _ => {
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            material: Material::Dielectric(Dielectric(1.5))
                        });
                    }
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric(1.5));

    world.add(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1.clone(),
    });

    let material2 = Material::Lambertian(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));

    world.add(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2.clone(),
    });

    let material3 = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3.clone(),
    });

    world.build_bvh();
    world
}

pub fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(&lookfrom, &lookat, &Vec3::new(0.0, 1.0, 0.0), 20.0, aspect_ratio,
                          aperture, focus_dist, 0.0, 1.0);

    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;
    let world = make_random_world();

    let path = Path::new("target/image.ppm");
    let mut file = BufWriter::new(OpenOptions::new().create(true).write(true).open(&path).unwrap());

    write_header(&mut file, image_width, image_height);

    let pb = ProgressBar::new((image_height * image_width) as u64);
    pb.set_draw_target(ProgressDrawTarget::stdout());
    pb.set_draw_delta(pb.length() / 100);
    pb.set_style(ProgressStyle::default_bar().template("[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]"));

    let pixels = (0..image_height).rev().flat_map(|j| {
        (0..image_width).map(move |i| (j, i))
    }).collect::<Vec<_>>().into_par_iter().chunks(1024).map(|chunk| {
        chunk.iter().map(|(j, i)| {
            let mut pixel_color: Color = (0..samples_per_pixel).into_iter().map(|_i| {
                let u = (*i as f32 + rand()) / (image_width - 1) as f32;
                let v = (*j as f32 + rand()) / (image_height - 1) as f32;

                let r = cam.get_ray(u, v);
                ray_color(&r, &world, max_depth)
            }).sum();

            pixel_color /= samples_per_pixel as f32;
            pixel_color = Color::new(pixel_color.x.sqrt(), pixel_color.y.sqrt(), pixel_color.z.sqrt());
            pb.inc(1);
            pixel_color
        }).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();

    pb.finish();

    for pixel_color in pixels {
        pixel_color.write_ppm(&mut file);
    }

    Command::new("open").arg(path).spawn().unwrap();
}