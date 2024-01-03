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

pub fn decrypt_symmetric(input: &DecryptSymmetricOptions) -> Result<String> {
    let decoded_tag = b64_decode!(&input.tag);
    let decoded_key = b64_decode!(&input.key);
    let iv = b64_decode!(&input.iv);
    let decoded_ciphertext = b64_decode!(&input.ciphertext);

    if decoded_tag.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_tag.unwrap_err().to_string(),
        });
    }

    if decoded_key.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_key.unwrap_err().to_string(),
        });
    }

    if iv.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: iv.unwrap_err().to_string(),
        });
    }

    if decoded_ciphertext.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_ciphertext.unwrap_err().to_string(),
        });
    }

    let decoded_tag = decoded_tag.unwrap();
    let decoded_key = decoded_key.unwrap();
    let iv = iv.unwrap();
    let mut decoded_ciphertext = decoded_ciphertext.unwrap();

    // We modify the ciphertext a little bit here to remove the pre-existing tag, and append the tag that was provided as a parameter.
    decoded_ciphertext.truncate(decoded_ciphertext.len() - 16);
    decoded_ciphertext.extend_from_slice(&decoded_tag);

    let nonce = GenericArray::from_slice(&iv);

    let cipher = Aes256Gcm::new_from_slice(&decoded_key);

    if cipher.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: "Failed to create cipher.".to_string(),
        });
    }

    let plaintext_bytes = cipher
        .unwrap()
        .decrypt(nonce, decoded_ciphertext.as_ref())
        .map_err(|e| Error::DecryptSymmetricKeyError {
            message: e.to_string(),
        })?;

    return Ok(String::from_utf8(plaintext_bytes)
        .map_err(|e| Error::DecryptSymmetricKeyError {
            message: e.to_string(),
        })
        .expect("Failed to convert bytes to string."));
}
