pub mod two_perlin_spheres;
pub mod two_spheres;
pub mod random_spheres;

use crate::intersections::World;
use crate::types::Point3;

pub struct Scene {
    pub world: World,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vfov: f32,
    pub aperture: f32
}