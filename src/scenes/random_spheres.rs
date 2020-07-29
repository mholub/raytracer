use crate::intersections::*;
use crate::types::*;
use crate::material::*;
use crate::random::*;
use crate::scenes::Scene;

pub fn scene() -> Scene {
    Scene {
        world: make_world(),
        vfov: 20.0,
        aperture: 0.1,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0)
    }
}

fn make_world() -> World {
    let mut world = World::new();

    let material_ground = Material::from(
        Lambertian::from_colors(Color::new(0.2, 0.3, 0.1),
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
                            material: Material::from(
                                Lambertian::from_color(Color::new(rand(), rand(), rand()).component_mul(&Color::new(rand(), rand(), rand())))),
                        });
                    }
                    x if x < 0.95 => {
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            material: Material::from(Metal::new(
                                Color::new(rand_range(0.5, 1.0), rand_range(0.5, 1.0), rand_range(0.5, 1.0)),
                                rand_range(0.0, 0.5))),
                        });
                    }
                    _ => {
                        world.add(Sphere {
                            center: center,
                            radius: 0.2,
                            material: Material::from(Dielectric(1.5)),
                        });
                    }
                }
            }
        }
    }

    let material1 = Material::from(Dielectric(1.5));

    world.add(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1.clone(),
    });

    let material2 = Material::from(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));

    world.add(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2.clone(),
    });

    let material3 = Material::from(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3.clone(),
    });

    world.build_bvh();
    world
}