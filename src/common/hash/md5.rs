use crate::common::hash::hex;
use md5::{Digest, Md5};
use std::error::Error;
use std::io::Read;

pub fn compare<R: Read>(
    mut reader: R,
    expected_hash: &str,
) -> Result<bool, Box<dyn Error>> {
    let mut hasher = Md5::new();
    let mut buffer = [0; 1024];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = hasher.finalize();
    let calculated_hash_str = hex::encode(&hash);
    Ok(calculated_hash_str == expected_hash)
}
