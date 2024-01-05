use crate::{error::Result, Client};

// DELETE SECRET
use super::cryptography::{
    decrypt_symmetric::{decrypt_symmetric, DecryptSymmetricOptions},
    encrypt_symmetric::{encrypt_symmetric, EncryptSymmetricOptions, EncryptSymmetricResponse},
    CreateSymmetricKeyResponse, DecryptSymmetricResponse,
};
pub use crate::manager::cryptography::create_symmetric_key::create_symmetric_key;

#[allow(dead_code)]
pub struct ClientCryptography<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientCryptography<'a> {
    pub fn create_symmetric_key(&'a mut self) -> Result<CreateSymmetricKeyResponse> {
        create_symmetric_key()
    }

    pub fn encrypt_symmetric(
        &'a mut self,
        input: &EncryptSymmetricOptions,
    ) -> Result<EncryptSymmetricResponse> {
        encrypt_symmetric(input)
    }

    pub fn decrypt_symmetric(
        &'a mut self,
        input: &DecryptSymmetricOptions,
    ) -> Result<DecryptSymmetricResponse> {
        decrypt_symmetric(input)
    }
}

impl<'a> Client {
    pub fn cryptography(&'a mut self) -> ClientCryptography<'a> {
        ClientCryptography { client: self }
    }
}
