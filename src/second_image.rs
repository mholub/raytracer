use crate::types::*;
use crate::intersections::{Sphere, Hittable};
use crate::ppm::*;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::fs::{OpenOptions};
use std::process::Command;

fn ray_color(ray: &Ray) -> Color {
    let sphere1 = Sphere {
        center : Point3::new(0.0, 0.0, -1.0),
        radius : 0.5
    };

    let sphere2 = Sphere {
        center : Point3::new(3.0, 1.0, -3.5),
        radius : 1.0
    };

    let sphere3 = Sphere {
        center : Point3::new(-3.0, 1.0, -3.5),
        radius : 1.3
    };

    let spheres = vec![sphere1, sphere2, sphere3];

    if let Some(hit) = spheres.hit(&ray, 0.0, 1000.0) {
        0.5 * Color::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0)
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

pub fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let path = Path::new("target/image.ppm");
    let mut file = BufWriter::new(OpenOptions::new().create(true).write(true).open(&path).unwrap());

    write_header(&mut file, image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = (i as f32) / (image_width - 1) as f32;
            let v = (j as f32) / (image_height - 1) as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let color = ray_color(&r);
            color.write_ppm(&mut file);
        }
    }

    Command::new("open").arg(path).spawn().unwrap();
}