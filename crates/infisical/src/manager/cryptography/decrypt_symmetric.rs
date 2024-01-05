use crate::error::{Error, Result};
use aes::cipher::generic_array::GenericArray;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use base64::engine::Engine;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize}; // Assuming you are using the 'aes-gcm' crate

macro_rules! b64_decode {
    ($key:expr) => {
        base64::engine::general_purpose::STANDARD.decode($key)
    };
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecryptSymmetricOptions {
    pub key: String,
    pub ciphertext: String,
    pub iv: String,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DecryptSymmetricResponse {
    pub decrypted: String,
}

pub fn decrypt_symmetric(input: &DecryptSymmetricOptions) -> Result<DecryptSymmetricResponse> {
    let decoded_tag = b64_decode!(&input.tag).map_err(|e| Error::DecryptSymmetricKeyError {
        message: e.to_string(),
    })?;
    let decoded_key = b64_decode!(&input.key).map_err(|e| Error::DecryptSymmetricKeyError {
        message: e.to_string(),
    })?;
    let iv = b64_decode!(&input.iv).map_err(|e| Error::DecryptSymmetricKeyError {
        message: e.to_string(),
    })?;
    let mut decoded_ciphertext =
        b64_decode!(&input.ciphertext).map_err(|e| Error::DecryptSymmetricKeyError {
            message: e.to_string(),
        })?;

    // We modify the ciphertext a little bit here to remove the pre-existing tag, and append the tag that was provided as a parameter.
    //decoded_ciphertext.truncate(decoded_ciphertext.len() - 16);
    decoded_ciphertext.extend_from_slice(&decoded_tag);

    let nonce = GenericArray::from_slice(&iv);

    let cipher =
        Aes256Gcm::new_from_slice(&decoded_key).map_err(|e| Error::DecryptSymmetricKeyError {
            message: e.to_string(),
        })?;

    let plaintext_bytes = cipher
        .decrypt(nonce, decoded_ciphertext.as_ref())
        .map_err(|e| Error::DecryptSymmetricKeyError {
            message: e.to_string(),
        })?;

    return Ok(DecryptSymmetricResponse {
        decrypted: String::from_utf8(plaintext_bytes)
            .map_err(|e| Error::DecryptSymmetricKeyError {
                message: e.to_string(),
            })
            .expect("Failed to convert bytes to string."),
    });
}
