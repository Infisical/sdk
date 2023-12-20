use crate::api::secrets::delete_secret::delete_secret_request;
use crate::error::Result;
use crate::helper::handle_authentication;
use crate::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Secret;
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSecretOptions {
    pub environment: String,    // environment
    pub path: Option<String>,   // secretPath
    pub r#type: Option<String>, // shared / personal
    pub secret_name: String,    // secretName (PASSED AS PARAMETER IN REQUEST)
    pub project_id: String,     // workspaceId
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSecretResponse {
    pub secret: Secret,
}

pub async fn delete_secret(
    client: &mut Client,
    input: &DeleteSecretOptions,
) -> Result<DeleteSecretResponse> {
    handle_authentication(client).await?;

    let secret = delete_secret_request(client, input).await;

    match secret {
        Ok(secret) => Ok(secret),
        Err(e) => Err(e),
    }
}
