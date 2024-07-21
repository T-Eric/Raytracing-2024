// perlin noise by hashing

use super::vec3::{dot, Point3, Vec3};
use rand::Rng;

#[derive(Clone)]
pub struct Perlin {
    randvec: Vec<Vec3>, // random unit vec, the real spectacular thing
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut randvec = Vec::new();
        for _ in 0..Self::POINT_COUNT {
            randvec.push(Vec3::random_in(-1.0, 1.0));
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Perlin {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    // pub fn noise(&self, p: &Point3) -> f64 {
    // for "pixel-like" perlin
    // let i = (4.0 * p.x()) as i32 & 255;
    // let j = (4.0 * p.y()) as i32 & 255;
    // let k = (4.0 * p.z()) as i32 & 255;
    // let pos = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];
    // self.randfloat[pos as usize]
    // }

    pub fn noise(&self, p: &Point3) -> f64 {
        // <0 part
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        // >0 part
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for (di, layer) in c.iter_mut().enumerate().take(2) {
            for (dj, row) in layer.iter_mut().enumerate().take(2) {
                for (dk, cell) in row.iter_mut().enumerate().take(2) {
                    *cell = self.randvec[(self.perm_x[(i as usize + di) & 255]
                        ^ self.perm_y[(j as usize + dj) & 255]
                        ^ self.perm_z[(k as usize + dk) & 255])
                        as usize]
                }
            }
        }

        trilinear_interpolate(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }
        accum.abs()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::new();
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        permute(&mut p, Self::POINT_COUNT);
        p
    }
}

fn permute(p: &mut [i32], n: i32) {
    let mut rng = rand::thread_rng();
    for i in (1..n).rev() {
        let target = rng.gen_range(0..=i);
        p.swap(i as usize, target as usize);
    }
}

fn trilinear_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // hermitian smoothing: cubic round off (here now)
    let u = u * u * (3.0 - 2.0 * u);
    let v = v * v * (3.0 - 2.0 * v);
    let w = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for (i, layer) in c.iter().enumerate().take(2) {
        for (j, row) in layer.iter().enumerate().take(2) {
            for (k, cell) in row.iter().enumerate().take(2) {
                let weight_vec = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * dot(cell, &weight_vec)
            }
        }
    }
    accum
}

impl Perlin {
    const POINT_COUNT: i32 = 256;
}
