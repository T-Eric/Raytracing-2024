// perlin noise by hashing

use crate::utils::vec3::Point3;
use rand::Rng;

pub struct Perlin {
    randfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut randfloat = Vec::new();
        for _ in 0..Self::POINT_COUNT {
            randfloat.push(rng.gen_range(0.0..1.0));
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Perlin {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;
        let pos = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];
        self.randfloat[pos as usize]
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

impl Perlin {
    const POINT_COUNT: i32 = 256;
}
