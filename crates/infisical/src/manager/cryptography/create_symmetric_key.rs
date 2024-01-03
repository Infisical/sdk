use crate::error::Result;
use base64::engine::Engine;
use rand::Rng;

macro_rules! b64_encode {
    ($key:expr) => {
        base64::engine::general_purpose::STANDARD.encode($key)
    };
}

pub fn create_symmetric_key() -> Result<String> {
    // Generate a 256-bit key (32 bytes * 8 bits/byte)
    let key: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>().to_vec();

    let encoded_key = b64_encode!(key);

    return Ok(encoded_key);
}
