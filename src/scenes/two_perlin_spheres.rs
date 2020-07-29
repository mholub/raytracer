use crate::intersections::*;
use crate::material::*;
use crate::scenes::Scene;
use crate::types::*;
use crate::texture::*;

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

    let material_spheres = Material::from(Lambertian(Texture::from(NoiseTexture::new(4.0))));

    world.add(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: material_spheres.clone(),
    });

    world.add(Sphere {
        center: Point3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: material_spheres.clone(),
    });

    world.build_bvh();
    world
}
