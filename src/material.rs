use crate::types::{Ray, Color, Vec3};
use crate::intersections::HitRecord;
use crate::random::*;
use crate::texture::*;

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatter for Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray_in, hit),
            Material::Metal(ref inner) => inner.scatter(ray_in, hit),
            Material::Dielectric(ref inner) => inner.scatter(ray_in, hit)
        }
    }
}

#[derive(Clone)]
pub struct Lambertian(pub Texture);

impl Lambertian {
    pub fn from_color(color: Color) -> Self {
        Lambertian(Texture::SolidColor(SolidColor(color)))
    }

    pub fn from_colors(color1: Color, color2: Color) -> Self {
        Lambertian(Texture::CheckerTexture(CheckerTexture::from_colors(color1, color2)))
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

// double schlick(double cosine, double ref_idx) {
// auto r0 = (1-ref_idx) / (1+ref_idx);
// r0 = r0*r0;
// return r0 + (1-r0)*pow((1 - cosine),5);
// }

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Clone, Copy, Debug)]
pub struct Dielectric(pub f32);

impl Scatter for Dielectric {
    /*
     virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
        ) const override {
            attenuation = color(1.0, 1.0, 1.0);
            double etai_over_etat = rec.front_face ? (1.0 / ref_idx) : ref_idx;

            vec3 unit_direction = unit_vector(r_in.direction());
            double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
            double sin_theta = sqrt(1.0 - cos_theta*cos_theta);
            if (etai_over_etat * sin_theta > 1.0 ) {
                vec3 reflected = reflect(unit_direction, rec.normal);
                scattered = ray(rec.p, reflected);
                return true;
            }
            double reflect_prob = schlick(cos_theta, etai_over_etat);
            if (random_double() < reflect_prob)
            {
                vec3 reflected = reflect(unit_direction, rec.normal);
                scattered = ray(rec.p, reflected);
                return true;
            }
            vec3 refracted = refract(unit_direction, rec.normal, etai_over_etat);
            scattered = ray(rec.p, refracted);
            return true;
        }
     */

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