use std::io::{Read};
use std::error::Error;
use sha2::{Digest, Sha256};
use crate::common::hash::hex;

pub fn compare<R: Read>(mut reader: R, expected_hash: &str) -> Result<bool, Box<dyn Error>> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4 * 1024 * 1024];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let calculated_hash = hasher.finalize();
    let calculated_hash_str = hex::encode(&calculated_hash);

    Ok(calculated_hash_str == expected_hash)
}