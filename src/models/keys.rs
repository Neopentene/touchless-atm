use crate::models::helpers::common::{copy_from_slice, timestamp};
use crate::utilities::crypto::{from_hex, Generator};

#[derive(Debug, Clone, Copy)]
pub struct KEY {
    pub seed: [u8; 12],
    pub secret: [u8; 16],
    pub bytes: [u8; 32],
    pub rate: u8,
    pub last_changed: i64,
}

impl KEY {
    pub fn new(seed: String, secret: String, bytes: String, rate: u8, last_changed: i64) -> Self {
        let mut _seed: [u8; 12] = [0u8; 12];
        let seed = from_hex(seed).unwrap();
        copy_from_slice(&mut _seed, &seed);

        let mut _secret: [u8; 16] = [0u8; 16];
        let secret = from_hex(secret).unwrap();
        copy_from_slice(&mut _secret, &secret);

        let mut _bytes: [u8; 32] = [0u8; 32];
        let bytes = from_hex(bytes).unwrap();
        copy_from_slice(&mut _bytes, &bytes);

        Self {
            seed: _seed,
            secret: _secret,
            bytes: _bytes,
            rate,
            last_changed,
        }
    }

    pub fn update(&mut self) {
        let seed = Generator::generate_seed();
        let bytes = Generator::generate_random_bytes();
        let mut secret: [u8; 16] = [0u8; 16];
        Generator::generate_nonce(&mut secret);
        let rate = Generator::generate_rate();

        self.seed = seed;
        self.secret = secret;
        self.bytes = bytes;
        self.rate = rate;
        self.last_changed = timestamp();
    }
}
