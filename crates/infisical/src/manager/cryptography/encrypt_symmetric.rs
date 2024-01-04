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

    // Generate a random IV
    let mut iv = [0u8; 12];
    OsRng.fill_bytes(&mut iv);

    let decoded_key = b64_decode!(key).map_err(|e| Error::EncryptSymmetricKeyError {
        message: e.to_string(),
    })?;

    // Create a new AES256-GCM instance
    let cipher = Aes256Gcm::new_from_slice(decoded_key.as_slice()).map_err(|e| {
        Error::EncryptSymmetricKeyError {
            message: e.to_string(),
        }
    })?;

    let ciphertext_r = &cipher
        .encrypt(&iv.into(), plaintext.as_bytes())
        .map_err(|e| Error::EncryptSymmetricKeyError {
            message: e.to_string(),
        });

    let mut ciphertext = ciphertext_r.as_ref().unwrap().clone();

    // we need to take the last 16 bytes of the ciphertext and remove it from the ciphertext, and append it to the tag.

    let encryption_tag = &ciphertext.clone()[&ciphertext.clone().len() - 16..]; // This line is a bit confusing, but it basically takes the last 16 bytes of the ciphertext and stores it in a variable.
    ciphertext.truncate(ciphertext.len() - 16); // This line removes the last 16 bytes of the ciphertext.

    let encoded_ciphertext = b64_encode!(&ciphertext.clone());
    let encoded_iv = b64_encode!(iv);
    let encoded_tag = b64_encode!(encryption_tag);

    return Ok(EncryptSymmetricResponse {
        ciphertext: encoded_ciphertext,
        iv: encoded_iv,
        tag: encoded_tag,
    });
}
