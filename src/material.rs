use crate::types::{Ray, Color, Vec3};
use crate::intersections::HitRecord;
use crate::random::*;

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal)
}

impl Scatter for Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray_in, hit),
            Material::Metal(ref inner) => inner.scatter(ray_in, hit)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Lambertian(pub Color);

impl Scatter for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit.normal + Vec3::rand_unit();
        Some((Ray::new(hit.point, scatter_direction), self.0))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal{
            albedo, fuzz: if fuzz < 1.0 { fuzz} else { 1.0 }
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n)*n
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray_in.direction().normalize(), &hit.normal);

        let scattered = Ray::new(hit.point, reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}