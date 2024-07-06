use rand::Rng;

use crate::operations::{add_mod_u32, mul_mod_u32, sub_mod_u32};

#[derive(Debug)]
pub struct ReedSolomonFingerprint {
    modulo: u32,
    seed: u32,
    sum_fingerprint: u32,
    perm_fingerprint: u32,
}

impl ReedSolomonFingerprint {
    pub fn new(data: &[u8], modulo: u32) -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen::<u32>() % modulo;
        Self::new_with_seed(data, modulo, seed)
    }

    pub fn new_with_seed(data: &[u8], modulo: u32, seed: u32) -> Self {
        let sum_fingerprint = Self::compute_sum_fingerprint(data, modulo, seed);
        let perm_fingerprint = Self::compute_perm_fingerprint(data, modulo, seed);
        Self {
            seed,
            modulo,
            sum_fingerprint,
            perm_fingerprint,
        }
    }

    pub fn equal(&self, value: &[u8]) -> bool {
        Self::compute_sum_fingerprint(value, self.modulo, self.seed) == self.sum_fingerprint
    }

    pub fn perm(&self, value: &[u8]) -> bool {
        Self::compute_perm_fingerprint(value, self.modulo, self.seed) == self.perm_fingerprint
    }

    fn compute_sum_fingerprint(data: &[u8], modulo: u32, seed: u32) -> u32 {
        let mut power = 1;
        let mut fingerprint = 0;
        for val in data {
            fingerprint = add_mod_u32(
                fingerprint,
                mul_mod_u32((*val as u32) % modulo, power, modulo),
                modulo,
            );
            power = mul_mod_u32(power, seed, modulo);
        }
        fingerprint
    }

    fn compute_perm_fingerprint(data: &[u8], modulo: u32, seed: u32) -> u32 {
        let mut fingerprint = 1;
        for val in data {
            fingerprint = mul_mod_u32(
                fingerprint,
                sub_mod_u32(seed, (*val as u32) % modulo, modulo),
                modulo,
            );
        }
        fingerprint
    }
}
