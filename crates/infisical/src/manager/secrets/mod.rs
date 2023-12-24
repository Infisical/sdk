pub(crate) mod create;
pub(crate) mod delete;
pub(crate) mod get;
pub(crate) mod list;
pub(crate) mod update;

pub use create::{CreateSecretOptions, CreateSecretResponse};
pub use delete::{DeleteSecretOptions, DeleteSecretResponse};
pub use get::{get_secret, GetSecretOptions, GetSecretResponse};
pub use list::{ListSecretsOptions, ListSecretsResponse};
pub use update::{UpdateSecretOptions, UpdateSecretResponse};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    pub version: i32,
    pub workspace: String,
    pub r#type: String,
    pub environment: String,
    pub secret_key: String,
    pub secret_value: String,
    pub secret_comment: String,
}
