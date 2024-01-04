use crate::error::Result;
use base64::engine::Engine;
use rand::Rng;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! b64_encode {
    ($key:expr) => {
        base64::engine::general_purpose::STANDARD.encode($key)
    };
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSymmetricKeyResponse {
    pub key: String,
}

pub fn create_symmetric_key() -> Result<CreateSymmetricKeyResponse> {
    // Generate a 256-bit key (32 bytes * 8 bits/byte)
    let key: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>().to_vec();

    let encoded_key = b64_encode!(key);

    return Ok(CreateSymmetricKeyResponse { key: encoded_key });
}
