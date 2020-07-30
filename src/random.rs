use rand::Rng;
use crate::types::Vec3;

pub fn rand() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range(low: f32, high: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high)
}

pub trait Vector {
    fn rand() -> Vec3;
    fn rand_range(min: f32, max: f32) -> Vec3;
    fn rand_unit() -> Vec3;
    fn rand_in_hemisphere(normal: &Vec3) -> Vec3;
    fn rand_in_unit_sphere() -> Vec3;
    fn rand_in_unit_disk() -> Vec3;
}

impl Vector for Vec3 {
    fn rand() -> Vec3 {
        Self::new(rand(), rand(), rand())
    }

    fn rand_range(min: f32, max: f32) -> Vec3 {
        Self::new(rand_range(min, max), rand_range(min, max), rand_range(min, max))
    }

    fn rand_unit() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        while p.magnitude_squared() >= 1.0 {
            p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        }
        p.normalize()
    }

    fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
        let p = Self::rand_in_unit_sphere();
        if p.dot(&normal) > 0.0 {
            p
        } else {
            -p
        }
    }

    fn rand_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        while p.magnitude_squared() >= 1.0 {
            p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        }
        p
    }

    fn rand_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        while p.magnitude_squared() >= 1.0 {
            p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0)
        }
        p
    }
}