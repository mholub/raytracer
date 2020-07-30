#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::cast_possible_truncation)]

use crate::random::Vector;
use rand::Rng;
use crate::types::{Point3, Vec3};
use std::sync::Arc;

const N: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Arc<[Vec3; N]>,
    perm_x: Arc<[usize; N]>,
    perm_y: Arc<[usize; N]>,
    perm_z: Arc<[usize; N]>,
}

#[allow(clippy::needless_range_loop)]
fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u*u*(3.0-2.0*u);
    let vv = v*v*(3.0-2.0*v);
    let ww = w*w*(3.0-2.0*w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u-i as f32, v-j as f32, w-k as f32);
                accum += (i as f32 * uu + (1.0-i as f32)*(1.0-uu))
                    * (j as f32 * vv + (1.0-j as f32)*(1.0-vv))
                    * (k as f32 *ww + (1.0-k as f32)*(1.0-ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }

    accum
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = [Vec3::new(0.0, 0.0, 0.0); N];
        for x in ranfloat.iter_mut() {
            *x = Vec3::rand_range(-1.0, 1.0).normalize();
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Self {
            ranfloat: Arc::new(ranfloat),
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn turb(&self, p: Point3, depth: isize) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight*self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    pub fn noise(&self, p: Point3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u*u*(3.0-2.0*u);
        v = v*v*(3.0-2.0*v);
        w = w*w*(3.0-2.0*w);

        let i = p.x.floor();
        let j = p.y.floor();
        let k = p.z.floor();

        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ii = ((i + di as f32) as isize & 255) as usize;
                    let jj = ((j + dj as f32) as isize & 255) as usize;
                    let kk = ((k + dk as f32) as isize & 255) as usize;

                    c[di][dj][dk] = self.ranfloat[self.perm_x[ii] ^ self.perm_y[jj] ^ self.perm_z[kk]];
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    fn generate_perm() -> Arc<[usize; N]> {
        let mut result = [0; N];
        for (i, x) in result.iter_mut().enumerate() {
            *x = i;
        }

        Self::permute(&mut result);
        Arc::new(result)
    }

    fn permute(arr: &mut [usize; N]) {
        let mut rng = rand::thread_rng();
        for i in (1..arr.len()).rev() {
            let target = rng.gen_range(0, i);
            arr.swap(i, target);
        }
    }
}
