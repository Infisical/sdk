use crate::{error::Result, Client};

// GET SECRET
pub use crate::{
    manager::secrets::get::get_secret, manager::secrets::get::GetSecretOptions,
    manager::secrets::get::GetSecretResponse,
};

// LIST SECRET
pub use crate::{
    manager::secrets::list::list_secrets, manager::secrets::list::ListSecretsOptions,
    manager::secrets::list::ListSecretsResponse,
};

// CREATE SECRET
pub use crate::{
    manager::secrets::create::create_secret, manager::secrets::create::CreateSecretOptions,
    manager::secrets::create::CreateSecretResponse,
};

// UPDATE SECRET
pub use crate::{
    manager::secrets::update::update_secret, manager::secrets::update::UpdateSecretOptions,
    manager::secrets::update::UpdateSecretResponse,
};
// DELETE SECRET
pub use crate::{
    manager::secrets::delete::delete_secret, manager::secrets::delete::DeleteSecretOptions,
    manager::secrets::delete::DeleteSecretResponse,
};

pub struct ClientSecrets<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientSecrets<'a> {
    pub async fn get(&mut self, input: &GetSecretOptions) -> Result<GetSecretResponse> {
        get_secret(self.client, input).await
    }

    pub async fn list(&mut self, input: &ListSecretsOptions) -> Result<ListSecretsResponse> {
        list_secrets(self.client, input).await
    }

    pub async fn create(&mut self, input: &CreateSecretOptions) -> Result<CreateSecretResponse> {
        create_secret(self.client, input).await
    }

    pub async fn update(&mut self, input: &UpdateSecretOptions) -> Result<UpdateSecretResponse> {
        update_secret(self.client, input).await
    }

    pub async fn delete(&mut self, input: &DeleteSecretOptions) -> Result<DeleteSecretResponse> {
        delete_secret(self.client, input).await
    }
}

impl<'a> Client {
    pub fn secrets(&'a mut self) -> ClientSecrets<'a> {
        ClientSecrets { client: self }
    }
}
