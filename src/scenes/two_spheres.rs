#![allow(dead_code)]

use crate::intersections::{Sphere, World};
use crate::material::{Lambertian, Material};
use crate::scenes::Scene;
use crate::types::{Color, Point3};

pub fn scene() -> Scene {
    Scene {
        world: make_world(),
        vfov: 20.0,
        aperture: 0.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
    }
}

fn make_world() -> World {
    let mut world = World::new();

    let material_ground = Material::from(Lambertian::from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Sphere {
        center: Point3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: material_ground.clone(),
    });

    world.add(Sphere {
        center: Point3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: material_ground,
    });

    world.build_bvh();
    world
}
