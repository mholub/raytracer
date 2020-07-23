use crate::types::*;
use crate::intersections::{Sphere, Hittable, World};
use crate::ppm::*;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::fs::{OpenOptions};
use std::process::Command;
use crate::camera::Camera;
use crate::random::*;
use crate::material::{Lambertian, Metal, Scatter, Material};
use rayon::prelude::*;

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

fn make_world() -> World {
    let material_ground = Material::Lambertian(Lambertian(Color::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian(Color::new(0.7, 0.3, 0.3)));
    let material_left = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = World::new();

    world.objects.push(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground.clone(),
    });

    world.objects.push(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center.clone(),
    });

    world.objects.push(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left.clone(),
    });

    world.objects.push(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right.clone(),
    });

    world
}

pub fn main() {
    let cam = Camera::new();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let world = make_world();

    let path = Path::new("target/image.ppm");
    let mut file = BufWriter::new(OpenOptions::new().create(true).write(true).open(&path).unwrap());

    write_header(&mut file, image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color: Color = (0..samples_per_pixel).into_par_iter().map(|_i| {
                let u = (i as f32 + rand()) / (image_width - 1) as f32;
                let v = (j as f32 + rand()) / (image_height - 1) as f32;

                let r = cam.get_ray(u, v);
                ray_color(&r, &world, max_depth)
            }).sum();

            pixel_color /= samples_per_pixel as f32;
            pixel_color = Color::new(pixel_color.x.sqrt(), pixel_color.y.sqrt(), pixel_color.z.sqrt());
            pixel_color.write_ppm(&mut file);
        }
    }

    Command::new("open").arg(path).spawn().unwrap();
}