use crate::api::secrets::get_secret::get_secret_request;
use crate::error::Result;
use crate::helper::handle_authentication;
use crate::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Secret;
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSecretOptions {
    pub secret_name: String,
    pub environment: String,
    pub project_id: String,
    pub path: Option<String>,

    pub r#type: Option<String>,

    pub include_imports: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetSecretResponse {
    pub secret: Secret,
}

pub async fn get_secret(
    client: &mut Client,
    input: &GetSecretOptions,
) -> Result<GetSecretResponse> {
    handle_authentication(client).await?;

    let secret = get_secret_request(client, input).await;

    match secret {
        Ok(secret) => Ok(secret),
        Err(e) => Err(e),
    }
}
