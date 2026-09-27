use rand::{Rng, rngs::SmallRng};

#[derive(Clone, Copy)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pz: f32,
}

pub const Z_NEAR: f32 = 0.06;
pub const Z_FAR: f32 = 1.0;
pub const Y_SCALE: f32 = 0.5;

impl Star {
    #[inline]
    pub fn spawn(rng: &mut SmallRng, half_w: f32, half_h: f32, random_depth: bool) -> Self {
        if random_depth {
            let z = rng.random_range(Z_NEAR..=Z_FAR);
            let sx = rng.random_range(-half_w..=half_w);
            let sy = rng.random_range(-half_h..=half_h);
            Self {
                x: sx * z,
                y: sy * z * 2.0,
                z,
                pz: z,
            }
        } else {
            let x = rng.random_range(-half_w..=half_w);
            let y = rng.random_range(-half_h * 2.0..=half_h * 2.0);
            let z = rng.random_range(0.92..=Z_FAR);
            Self { x, y, z, pz: z }
        }
    }

    #[inline]
    pub fn respawn(&mut self, rng: &mut SmallRng, half_w: f32, half_h: f32) {
        self.x = rng.random_range(-half_w..=half_w);
        self.y = rng.random_range(-half_h * 2.0..=half_h * 2.0);
        self.z = rng.random_range(0.92..=Z_FAR);
        self.pz = self.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::SmallRng};
    #[test]
    fn spawn_in_range() {
        let mut rng = SmallRng::seed_from_u64(42);
        for _ in 0..1000 {
            let s = Star::spawn(&mut rng, 100.0, 30.0, true);
            assert!(s.x >= -100.0 && s.x <= 100.0);
            assert!(s.y >= -60.0 && s.y <= 60.0);
            assert!(s.z >= Z_NEAR && s.z <= Z_FAR);
        }
    }
}
