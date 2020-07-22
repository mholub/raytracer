extern crate nalgebra_glm as glm;

pub type Color = glm::Vec3;
pub type Point3 = glm::Vec3;
pub type Vec3 = glm::Vec3;

pub struct Ray {
    origin : Point3,
    direction : Vec3
}

impl Ray {
    pub(crate) fn new(origin : Point3, direction : Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub(crate) fn origin(&self) -> Point3 {
        self.origin
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.direction
    }

    pub(crate) fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

