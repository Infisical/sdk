use crate::error::{Error, Result};
use aes::cipher::generic_array::GenericArray;
use aes_gcm::Nonce;
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
    pub iv: String,
    pub tag: String,
    pub ciphertext: String,
}

pub fn decrypt_symmetric(input: &DecryptSymmetricOptions) -> Result<String> {
    let key = &input.key;
    let iv = &input.iv;
    let tag = &input.tag;
    let ciphertext = &input.ciphertext;

    let decoded_key = &b64_decode!(key);
    let decoded_iv = b64_decode!(iv);
    let decoded_tag = b64_decode!(tag);
    let decoded_ciphertext = b64_decode!(ciphertext);

    println!("1");

    if decoded_key.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_key.as_ref().unwrap_err().to_string(),
        });
    }

    println!("2");

    if decoded_key.as_ref().unwrap().len() != 32 {
        return Err(Error::DecryptSymmetricKeyError {
            message: "Key must be 32 bytes in length.".to_string(),
        });
    }

    println!("3");

    if decoded_iv.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_iv.unwrap_err().to_string(),
        });
    }

    println!("4");

    if decoded_tag.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_tag.unwrap_err().to_string(),
        });
    }

    println!("5");

    if decoded_ciphertext.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: decoded_ciphertext.unwrap_err().to_string(),
        });
    }

    println!("decoded_key: {:?}", decoded_key);
    println!("decoded_iv: {:?}", decoded_iv);
    println!("decoded_tag: {:?}", decoded_tag);
    println!("decoded_ciphertext: {:?}", decoded_ciphertext);

    println!("6");

    let k = GenericArray::clone_from_slice(decoded_key.as_ref().unwrap());
    let key_final = KeyInit::new(&k.as_ref().clone());

    let cipher = Aes256Gcm::new(key_final);

    println!("7");

    // Concatenate the tag to the end of the ciphertext as required by the decrypt method
    let mut combined_ct = decoded_ciphertext.unwrap();
    combined_ct.extend_from_slice(&decoded_tag.unwrap());

    println!("8");

    let plaintext = cipher.decrypt(
        Nonce::from_slice(&decoded_iv.unwrap()),
        combined_ct.as_ref(),
    );

    println!("9");

    println!("plaintext: {:?}", plaintext);

    if plaintext.is_err() {
        return Err(Error::DecryptSymmetricKeyError {
            message: plaintext.clone().unwrap_err().to_string(),
        });
    }

    println!("10");

    let plaintext = String::from_utf8(plaintext.unwrap()).unwrap();

    Ok(plaintext)
}
