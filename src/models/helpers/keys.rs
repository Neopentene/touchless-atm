use super::common::timestamp;
use crate::{
    models::keys::KEY,
    utilities::crypto::{to_hex, Generator},
};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

impl KEY {
    pub fn update_keys(&mut self) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("keys.txt");

        let mut file = match file {
            Ok(file) => file,
            Err(_) => return Err("Failed to Update Keys".to_string()),
        };

        self.update();
        let seed = to_hex(&self.seed);
        let secret = to_hex(&self.secret);
        let bytes = to_hex(&self.bytes);
        let rate = self.rate;
        let last_changed = self.last_changed;

        let file_contents = format!("{seed}\n{secret}\n{bytes}\n{rate}\n{last_changed}");

        match file.write(file_contents.as_bytes()) {
            Ok(result) => {
                if result > 0 {
                    Ok(())
                } else {
                    Err("Failed to Update Keys".to_string())
                }
            }
            Err(_) => Err("Failed to Update Keys".to_string()),
        }
    }

    pub fn create_keys() -> Self {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open("keys.txt");

        let mut file = match file {
            Ok(result) => result,
            Err(_) => match File::create("keys.txt") {
                Ok(result) => result,
                Err(_) => panic!("Failed to create file"),
            },
        };

        let seed = to_hex(&Generator::generate_seed());
        let bytes = to_hex(&Generator::generate_random_bytes());
        let mut secret: [u8; 16] = [0u8; 16];
        Generator::generate_nonce(&mut secret);
        let secret = to_hex(&secret);
        let rate = Generator::generate_rate();
        let last_changed = timestamp();
        let file_contents = format!("{seed}\n{secret}\n{bytes}\n{rate}\n{last_changed}");

        file.write(file_contents.as_bytes())
            .expect("Failed to write new keys");

        Self::new(seed, secret, bytes, rate, last_changed)
    }

    pub fn retrive_keys() -> Self {
        let file = OpenOptions::new()
            .read(true)
            .truncate(true)
            .open("keys.txt");

        let file = match file {
            Ok(result) => result,
            Err(_) => return Self::create_keys(),
        };

        let mut reader = BufReader::new(file);
        let mut seed = String::new();
        reader.read_line(&mut seed).expect("Failed to get seed");

        let mut secret = String::new();
        reader.read_line(&mut secret).expect("Failed to get secret");

        let mut bytes = String::new();
        reader.read_line(&mut bytes).expect("Failed to get bytes");

        let mut rate = String::new();
        reader.read_line(&mut rate).expect("Failed to get rate");
        let rate: u8 = rate.parse().expect("Failed to parse rate");

        let mut last_changed = String::new();
        reader
            .read_line(&mut last_changed)
            .expect("Failed to get last_changed");
        let last_changed: i64 = last_changed.parse().expect("Failed to parse last_changed");

        Self::new(seed, secret, bytes, rate, last_changed)
    }
}
