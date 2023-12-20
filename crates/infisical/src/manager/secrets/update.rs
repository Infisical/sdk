use crate::api::secrets::update_secret::update_secret_request;
use crate::error::Result;
use crate::helper::handle_authentication;
use crate::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Secret;
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSecretOptions {
    pub secret_name: String,  // secretName (PASSED AS PARAMETER IN REQUEST)
    pub environment: String,  // environment
    pub path: Option<String>, // secretPath
    pub secret_value: String, // secretValue
    pub skip_multiline_encoding: Option<bool>, // skipMultilineEncoding
    pub r#type: Option<String>, // shared / personal
    pub project_id: String,   // workspaceId
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSecretResponse {
    pub secret: Secret,
}

pub async fn update_secret(
    client: &mut Client,
    input: &UpdateSecretOptions,
) -> Result<UpdateSecretResponse> {
    handle_authentication(client).await?;

    let secret = update_secret_request(client, input).await;

    match secret {
        Ok(secret) => Ok(secret),
        Err(e) => Err(e),
    }
}
