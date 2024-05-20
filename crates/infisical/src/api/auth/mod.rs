use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::Result, Client};

pub mod aws_iam_login;
pub mod aws_iam_login_new;
pub mod gcp_iam_login;
pub mod gcp_id_token_login;
pub mod universal_auth_login;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenSuccessResponse {
    pub access_token: String,
}

// Universal method for getting infisical token through google auth methods
pub(self) async fn auth_infisical_google(
    client: &mut Client,
    identity_id: Option<String>,
    jwt: Option<String>,
) -> Result<reqwest::Response> {
    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()
        .unwrap();

    let request = request_client
        .post(format!(
            "{}/api/v1/auth/gcp-auth/login",
            client.site_url.clone()
        ))
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    let mut body = HashMap::new();
    body.insert("identityId", identity_id);
    body.insert("jwt", jwt);

    let response = request.form(&body).send().await?;

    return Ok(response);
}
