use crate::types::{Point3, Ray, Vec3};
use crate::material::{Material};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = ray.direction().dot(&self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct World {
    pub objects: Vec<Sphere>
}

impl World {
    pub fn new() -> World {
        World { objects: vec![] }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_hit = None;
        let mut closest_t = t_max;

        for o in self.objects.iter() {
            if let Some(hit) = o.hit(&ray, t_min, closest_t) {
                closest_t = hit.t;
                temp_hit = Some(hit);
            }
        }
        temp_hit
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let d = half_b * half_b - a * c;

        if d > 0.0 {
            let root = d.sqrt();
            let temp = (-half_b - root) / a;
            if temp > t_min && temp < t_max {
                let p = ray.at(temp);
                let mut result = HitRecord {
                    point: ray.at(temp),
                    normal: (p - self.center) / self.radius,
                    t: temp,
                    front_face: true,
                    material: self.material
                };
                result.set_face_normal(ray);
                return Some(result);
            }

            let temp = (-half_b + root) / a;
            if temp > t_min && temp < t_max {
                let p = ray.at(temp);
                let mut result = HitRecord {
                    point: ray.at(temp),
                    normal: (p - self.center) / self.radius,
                    t: temp,
                    front_face: true,
                    material: self.material.clone()
                };
                result.set_face_normal(ray);
                return Some(result);
            }
        }
        None
    }
}