use crate::error::{Error, Result};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use base64::engine::Engine;
use rand::{rngs::OsRng, RngCore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! b64_encode {
    ($key:expr) => {
        base64::engine::general_purpose::STANDARD.encode($key)
    };
}

macro_rules! b64_decode {
    ($key:expr) => {
        base64::engine::general_purpose::STANDARD.decode($key)
    };
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EncryptSymmetricOptions {
    pub key: String,
    pub plaintext: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EncryptSymmetricResponse {
    pub ciphertext: String,
    pub iv: String,
    pub tag: String,
}

pub fn encrypt_symmetric(input: &EncryptSymmetricOptions) -> Result<EncryptSymmetricResponse> {
    let key = &input.key;
    let plaintext = &input.plaintext;

    let decoded_key = b64_decode!(key);

    // Generate a random IV
    let mut iv = [0u8; 12];
    OsRng.fill_bytes(&mut iv);

    if decoded_key.is_err() {
        return Err(Error::EncryptSymmetricKeyError {
            message: decoded_key.unwrap_err().to_string(),
        });
    }

    // Create a new AES256-GCM instance
    let cipher = Aes256Gcm::new_from_slice(decoded_key.unwrap().as_slice());

    if cipher.is_err() {
        return Err(Error::EncryptSymmetricKeyError {
            message: "Failed to create cipher.".to_string(),
        });
    }

    let ciphertext = &cipher.unwrap().encrypt(&iv.into(), plaintext.as_bytes());

    if ciphertext.is_err() {
        return Err(Error::EncryptSymmetricKeyError {
            message: ciphertext.clone().unwrap_err().to_string(),
        });
    }

    let encryption_tag = &ciphertext.clone().unwrap()[&ciphertext.clone().unwrap().len() - 16..];

    let encoded_ciphertext = b64_encode!(&ciphertext.clone().unwrap());
    let encoded_iv = b64_encode!(iv);
    let encoded_tag = b64_encode!(encryption_tag);

    return Ok(EncryptSymmetricResponse {
        ciphertext: encoded_ciphertext,
        iv: encoded_iv,
        tag: encoded_tag,
    });
}
