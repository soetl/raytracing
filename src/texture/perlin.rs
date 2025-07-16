use rand::{seq::SliceRandom, Rng};

use crate::{
    color::{Color, Linear},
    math::{Point3, Vec3, VecExt},
    texture::Texture,
};

#[derive(Debug)]
pub struct PerlinNoise {
    randvec: [Vec3; Self::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
    scale: f32,
}

impl PerlinNoise {
    const POINT_COUNT: usize = 256;

    pub fn new(scale: f32) -> Self {
        let mut rng = rand::rng();

        let randvec: [Vec3; Self::POINT_COUNT] = std::array::from_fn(|_| Vec3::random_unit());

        let perm_x = Self::generate_perm(&mut rng);
        let perm_y = Self::generate_perm(&mut rng);
        let perm_z = Self::generate_perm(&mut rng);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
            scale,
        }
    }

    fn generate_perm(rng: &mut impl Rng) -> [usize; Self::POINT_COUNT] {
        let mut perm = [0; Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            perm[i] = i;
        }
        perm.shuffle(rng);
        perm
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let hash_index = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.randvec[hash_index];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                        * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                        * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }

    fn turb(&self, p: &Point3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Texture for PerlinNoise {
    fn color(&self, _u: f32, _v: f32, p: &Point3) -> Color<Linear> {
        let noise_val = 1.0 + (self.scale * p.z + 10.0 * self.turb(p, 7)).sin();
        Color::from(Vec3::splat(0.5) * noise_val)
    }
}
