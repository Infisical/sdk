use crate::api::secrets::list_secrets::list_secrets_request;
use crate::error::Result;
use crate::helper::handle_authentication;
use crate::Client;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::Secret;
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListSecretsOptions {
    pub environment: String,
    pub project_id: String,
    pub path: Option<String>,

    pub attach_to_process_env: Option<bool>,
    pub include_imports: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListSecretsResponse {
    pub secrets: Vec<Secret>,
}

pub async fn list_secrets(
    client: &mut Client,
    input: &ListSecretsOptions,
) -> Result<ListSecretsResponse> {
    handle_authentication(client).await?;

    let secret = list_secrets_request(client, input).await;

    match secret {
        Ok(secret) => Ok(secret),
        Err(e) => Err(e),
    }
}
