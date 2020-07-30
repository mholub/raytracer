use crate::types::{Point3, Ray, Vec3};
use crate::material::{Material};
use std::sync::Arc;
use bvh::aabb::{AABB, Bounded};
use bvh::bvh::{BVH, BVHNode};
use bvh::nalgebra::{Point3 as BVHPoint3, Vector3 as BVHVector3};
use bvh::ray::Ray as BVHRay;
use bvh::bounding_hierarchy::BHShape;
use std::cell::RefCell;
use nalgebra_glm::RealField;

pub struct HitRecord {
    pub point: Point3,
    pub u: f32,
    pub v: f32,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
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

pub struct WorldObject {
    inner: Arc<dyn Hittable + Send + Sync>,
    aabb: AABB,
    node_index: usize,
}

impl BHShape for WorldObject {
    fn set_bh_node_index(&mut self, idx: usize) {
        self.node_index = idx;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Hittable for WorldObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.inner.hit(ray, t_min, t_max)
    }
}

impl Bounded for WorldObject {
    fn aabb(&self) -> AABB {
        self.aabb
    }
}

thread_local! {
    pub static CACHED_INDICES: RefCell<Vec<usize>> = RefCell::new(vec![]);
}

pub struct World {
    pub objects: Vec<WorldObject>,
    pub bvh: BVH,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            bvh: BVH { nodes: vec![] },
        }
    }

    pub fn build_bvh(&mut self) {
        let bvh = BVH::build(&mut self.objects);
        self.bvh = bvh;
    }

    pub fn add<T>(&mut self, obj: T) where T: Hittable + Bounded + Sync + Send + 'static {
        let aabb = obj.aabb();

        self.objects.push(WorldObject {
            inner: Arc::new(obj) as Arc<dyn Hittable + Send + Sync>,
            aabb,
            node_index: self.objects.len(),
        });
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_hit = None;
        let mut closest_t = t_max;

        let bvh_ray = BVHRay::new(BVHPoint3::new(ray.origin().x, ray.origin().y, ray.origin().z),
                                  BVHVector3::new(ray.direction().x, ray.direction().y, ray.direction().z));

        CACHED_INDICES.with(|ci| {
            let mut ci = ci.borrow_mut();
            ci.clear();
            BVHNode::traverse_recursive(&self.bvh.nodes, 0, &bvh_ray, &mut ci);

            for index in ci.iter() {
                if let Some(hit) = self.objects[*index].hit(&ray, t_min, closest_t) {
                    closest_t = hit.t;
                    temp_hit = Some(hit);
                }
            }
        });
        temp_hit
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    fn get_uv(local_p: Point3) -> (f32, f32) {
        let phi = local_p.z.atan2(local_p.x);
        let theta = local_p.y.asin();
        let u = 1.0 - (phi + f32::pi()) / (2.0 * f32::pi());
        let v = (theta + f32::pi()/2.0) / f32::pi();
        (u, v)
    }
}

impl Hittable for Sphere {
    #[allow(clippy::many_single_char_names)]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let d = half_b * half_b - a * c;

        if d > 0.0 {
            let root = d.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp > t_min && temp < t_max {
                let p = ray.at(temp);
                let local_p = (p - self.center)/self.radius;
                let (u, v) = Self::get_uv(local_p);

                let mut result = HitRecord {
                    point: ray.at(temp),
                    normal: (p - self.center) / self.radius,
                    t: temp,
                    front_face: true,
                    material: self.material.clone(),
                    u, v
                };
                result.set_face_normal(ray);
                return Some(result);
            }

            temp = (-half_b + root) / a;
            if temp > t_min && temp < t_max {
                let p = ray.at(temp);
                let local_p = (p - self.center)/self.radius;
                let (u, v) = Self::get_uv(local_p);
                let mut result = HitRecord {
                    point: ray.at(temp),
                    normal: (p - self.center) / self.radius,
                    t: temp,
                    front_face: true,
                    material: self.material.clone(),
                    u, v
                };
                result.set_face_normal(ray);
                return Some(result);
            }
        }
        None
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let half_size = BVHVector3::new(self.radius, self.radius, self.radius);

        let center = BVHPoint3::new(self.center.x, self.center.y, self.center.z);

        let min = center - half_size;
        let max = center + half_size;
        AABB::with_bounds(min, max)
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center1: Point3,
    pub time1: f32,
    pub center2: Point3,
    pub time2: f32,
    pub radius: f32,
    pub material: Material,
}

impl MovingSphere {
    pub fn center(&self, time: f32) -> Point3 {
        self.center1 + ((time - self.time1) / (self.time2 - self.time1)) * (self.center2 - self.center1)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let s = Sphere {
            center: self.center(ray.time),
            material: self.material.clone(),
            radius: self.radius
        };
        s.hit(ray, t_min, t_max)
    }
}

impl Bounded for MovingSphere {
    fn aabb(&self) -> AABB {
        let s1 = Sphere {
            center: self.center(self.time1),
            material: self.material.clone(),
            radius: self.radius
        };

        let s2 = Sphere {
            center: self.center(self.time2),
            material: self.material.clone(),
            radius: self.radius
        };

        s1.aabb().join(&s2.aabb())
    }
}