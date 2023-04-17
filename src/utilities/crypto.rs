#![allow(dead_code)]
use bcrypt::{self, hash, verify};
use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::{random, thread_rng, Rng};
use ring::{
    aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM},
    digest::{Context, SHA256},
    error::Unspecified,
};
use std::{
    collections::HashSet,
    num::{ParseIntError, Wrapping},
};

// Generates 58_122_955_296_762_404_570_121_600_000 nonce values
// It will take 1_843_066_821_941 years to crack all the combinations
// Assuming that it takes 1 nano second to compute on one combination
// A secret header is also used therefore even longer time will be required for an encryption to be compromised
// Key must be changed regularly to avoid such attempts

/// Default Cost for hashing password, to be changed to increase security
pub const DEFAULT_COST: u32 = 12;

/// Default hash password method
pub fn hash_password_default(password: String) -> Result<String, String> {
    match hash(password, DEFAULT_COST) {
        Ok(hash) => Ok(hash),
        Err(_) => Err("Error while hashing".to_string()),
    }
}

/// Hash password with a cost provided
pub fn hash_password(password: String, cost: u32) -> Result<String, String> {
    match hash(password, cost) {
        Ok(hash) => Ok(hash),
        Err(_) => Err("Error while hashing".to_string()),
    }
}

/// Verify the hashed password with a raw password
pub fn verify_password(unhashed_password: &String, hash: &String) -> Result<bool, String> {
    match verify(unhashed_password, hash) {
        Ok(result) => Ok(result),
        Err(_) => Err("Error while verification".to_string()),
    }
}

/// Returns an array of bytes from a hex string
/// # Example
/// ```
/// let hex_string = String::from("786544228956123645747965");
/// let bytes = from_hex(hex_string); // [120, 101, 68, 66, 142, 150, 18, 54, 69, 116, 121, 101] output
/// ```
pub fn from_hex(value: String) -> Result<Vec<u8>, ParseIntError> {
    (0..value.len())
        .step_by(2)
        .map(|index| {
            u8::from_str_radix(
                match value.get(index..index + 2) {
                    Some(val) => val,
                    None => "~", // For improper bytes pad with 126 -> '~'
                },
                16,
            )
        })
        .collect()
}

/// Converts the given bytes array to an hex string
/// # Example
/// ```
/// let bytes: [u8] = [120, 101, 68, 66, 142, 150, 18, 54, 69, 116, 121, 101];
/// let hex_string = to_hex(&bytes);
/// ```
pub fn to_hex(value: &[u8]) -> String {
    let mut hex: String = String::new();
    for bytes in value {
        hex.push_str(&format!("{:02x}", bytes));
    }
    hex
}

/// Hashes the given string and returns a hashed string
pub fn hasher(value: String) -> String {
    let mut context = Context::new(&SHA256);
    context.update(value.as_bytes());
    let digest = context.finish();
    to_hex(digest.as_ref())
}

/// Encrypts the given value and returns a Result<String, String> where Err holds a string saying Failed to Encrypt
pub fn encrypt(
    bytes: [u8; 32],
    value: String,
    secret: &[u8],
    seed: &[u8],
    rate: u8,
) -> Result<String, String> {
    let sealing_key = Generator::generate_sealing_key(&bytes, seed, rate);
    let mut data = value.as_bytes().to_vec();

    match sealing_key {
        Ok(mut key) => {
            match key.seal_in_place_append_tag(Generator::generate_aad(secret), &mut data) {
                Ok(_) => Ok(to_hex(data.as_slice())),
                Err(_) => Err("Failed to Encrypt".to_string()),
            }
        }
        Err(_) => Err("Failed to Encrypt".to_string()),
    }
}

/// Decrypts the given value and returns a Result<String, String> where Err holds a string saying Failed to Decrypt or Invalid Hex Value
pub fn decrypt(
    bytes: [u8; 32],
    value: String,
    secret: &[u8],
    seed: &[u8],
    rate: u8,
) -> Result<String, String> {
    let opening_key = Generator::generate_opening_key(&bytes, seed, rate);
    let mut data = match from_hex(value) {
        Ok(result) => result,
        Err(_) => return Err("Invalid Hex Value".to_string()),
    };

    match opening_key {
        Ok(mut key) => {
            let result = key.open_in_place(Generator::generate_aad(secret), &mut data);
            match result {
                Ok(result) => Ok(to_hex(result)),
                Err(_) => Err("Failed to Decrypt".to_string()),
            }
        }
        Err(_) => Err("Failed to Decrypt".to_string()),
    }
}

/// Generator struct that is used to generate or modify varied amount of bytes
pub struct Generator;
impl Generator {
    pub fn build_sequence<U: Ord + Clone>(array: Vec<U>) -> Vec<usize> {
        let mut cloned = array.clone();
        cloned.sort();
        array
            .iter()
            .map(|value| cloned.binary_search(value).unwrap())
            .collect::<Vec<usize>>()
    }

    pub fn next_permutation<U: Ord + Clone>(array: Vec<U>) -> Vec<U> {
        let (mut array, length, mut index) = (array.clone(), array.len() - 1, array.len() - 1);

        while index > 0 {
            match array[index] > array[index - 1] {
                true => {
                    let mut splitter = length;
                    while splitter >= index {
                        match array[splitter] > array[index - 1] {
                            true => {
                                array.swap(splitter, index - 1);
                                break;
                            }
                            false => splitter -= 1,
                        }
                    }

                    let (left, right) = array.split_at_mut(index);
                    right.reverse();
                    array = [left, right].concat().into();

                    break;
                }
                false => {
                    if index == 1 {
                        array.reverse();
                        break;
                    } else {
                        index -= 1;
                    }
                }
            }
        }
        array
    }

    pub fn generate_number_i32(start: i32, end: i32) -> i32 {
        thread_rng().gen_range(start..=end)
    }

    pub fn generate_number_i16(start: i16, end: i16) -> i16 {
        thread_rng().gen_range(start..=end)
    }

    pub fn generate_rate() -> u8 {
        thread_rng().gen_range(0..=255u8)
    }

    pub fn generate_nonce(reference: &mut [u8]) {
        for index in 0..reference.len() {
            reference[index] = Self::generate_rate();
        }
    }

    pub fn generate_seed() -> [u8; 12] {
        let mut set: HashSet<u8> = HashSet::new();
        let mut seed = [0u8; 12];
        loop {
            set.insert(Self::generate_rate());
            if set.len() == 12 {
                seed.copy_from_slice(&set.iter().copied().collect::<Vec<u8>>());
                break;
            }
        }
        seed
    }

    pub fn generate_token_encoding_key(secret: &[u8]) -> EncodingKey {
        EncodingKey::from_secret(&secret)
    }

    pub fn generate_token_decoding_key(secret: &[u8]) -> DecodingKey {
        DecodingKey::from_secret(&secret)
    }

    pub fn generate_random_bytes() -> [u8; 32] {
        random()
    }

    pub fn generate_sealing_key(
        bytes: &[u8; 32],
        seed: &[u8],
        rate: u8,
    ) -> Result<SealingKey<NonceGenerator>, String> {
        let mut generator_seed: [u8; 12] = [0u8; 12];

        for index in 0usize..12usize {
            if seed.len() - 1 >= index {
                generator_seed[index] = seed[index].to_owned();
            }
        }

        let unbound_key = match UnboundKey::new(&AES_256_GCM, bytes) {
            Ok(key) => key,
            Err(_) => return Err("Couldn't Generate Key".to_string()),
        };

        let generator = NonceGenerator::new(generator_seed, rate);
        Ok(SealingKey::new(unbound_key, generator))
    }

    pub fn generate_opening_key(
        bytes: &[u8; 32],
        seed: &[u8],
        rate: u8,
    ) -> Result<OpeningKey<NonceGenerator>, String> {
        let mut generator_seed: [u8; 12] = [0u8; 12];

        for index in 0usize..12usize {
            if seed.len() - 1 >= index {
                generator_seed[index] = seed[index].to_owned();
            }
        }

        let unbound_key = match UnboundKey::new(&AES_256_GCM, bytes) {
            Ok(key) => key,
            Err(_) => return Err("Couldn't Generate Key".to_string()),
        };

        let generator = NonceGenerator::new(generator_seed, rate);
        Ok(OpeningKey::new(unbound_key, generator))
    }

    pub fn generate_empty_aad() -> Aad<[u8; 0]> {
        Aad::empty()
    }

    pub fn generate_aad(secret: &[u8]) -> Aad<&[u8]> {
        Aad::from(secret)
    }
}

pub struct NonceGenerator {
    pub current: Wrapping<u128>,
    pub start: u128,
    seed: [u8; 12],
    rate: u8,
}

impl NonceGenerator {
    pub const SIZE: usize = 12;
    pub fn new(seed: [u8; 12], rate: u8) -> Self {
        let mut array = [0u8; 16];
        array[..12].copy_from_slice(&seed);

        let start = u128::from_le_bytes(array);
        Self {
            current: Wrapping(start),
            start,
            seed,
            rate,
        }
    }

    /// A clever yet totally useless function that implements Fisher Yale Shuffle and uses next permutations
    pub fn shuffle(&mut self) {
        for index in 0..usize::from(self.rate) {
            let target = index % Self::SIZE;
            self.seed.swap(target, Self::SIZE - 1 - target);
        }

        let mut array = [0u8; 16];
        array[..12].copy_from_slice(&self.seed);

        let current = u128::from_le_bytes(array);
        self.current = Wrapping(current);

        let seed = self.seed.clone();

        self.seed
            .copy_from_slice(&Generator::next_permutation::<u8>(seed.into()));
    }
}

impl NonceSequence for NonceGenerator {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let current = self.current.0;
        self.shuffle();

        if current == self.current.0 {
            return Err(Unspecified);
        }

        Ok(Nonce::assume_unique_for_key(
            current.to_le_bytes()[..12].try_into().unwrap(),
        ))
    }
}
