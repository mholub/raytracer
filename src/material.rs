#![allow(dead_code)]

use crate::types::{Ray, Color, Vec3};
use crate::intersections::HitRecord;
use crate::random::{Vector, rand};
use crate::texture::{Checker, GetColor, SolidColor, Texture};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Material)]
pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

#[enum_dispatch]
#[derive(Clone)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
}

#[derive(Clone)]
pub struct Lambertian(pub Texture);

impl Lambertian {
    pub fn from_color(color: Color) -> Self {
        Lambertian(Texture::from(SolidColor(color)))
    }

    pub fn from_colors(color1: Color, color2: Color) -> Self {
        Lambertian(Texture::from(Checker::from_colors(color1, color2)))
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = hit.normal + Vec3::rand_unit();
        Some((Ray::new(hit.point, scatter_direction, ray_in.time), self.0.value(hit.u, hit.v, hit.point)))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&ray_in.direction().normalize(), &hit.normal);

        let scattered = Ray::new(hit.point, reflected + self.fuzz * Vec3::rand_in_unit_sphere(), ray_in.time);
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Clone, Copy, Debug)]
pub struct Dielectric(pub f32);

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit.front_face { 1.0 / self.0 } else { self.0 };
        let unit_direction = ray_in.direction().normalize();

        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &hit.normal);
            Some((Ray::new(hit.point, reflected, ray_in.time), attenuation))
        } else {
            let reflect_probability = schlick(cos_theta, etai_over_etat);
            if rand() < reflect_probability {
                let reflected = reflect(&unit_direction, &hit.normal);
                Some((Ray::new(hit.point, reflected, ray_in.time), attenuation))
            } else {
                let refracted = refract(&unit_direction, &hit.normal, etai_over_etat);
                Some((Ray::new(hit.point, refracted, ray_in.time), attenuation))
            }
        }
    }
}