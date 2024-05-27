use std::collections::HashMap;

use base64::Engine;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    Client,
};

pub mod aws_iam_login;
pub mod azure_login;
pub mod gcp_iam_login;
pub mod gcp_id_token_login;
pub mod kubernetes_login;
pub mod universal_auth_login;

fn base64_encode(plain: String) -> String {
    return base64::engine::general_purpose::STANDARD.encode(plain);
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]

pub struct AccessTokenSuccessResponse {
    pub access_token: String,
}

#[derive(Serialize)]
pub(self) struct AwsIamRequestData {
    http_request_method: String,
    // base64 encoded body
    iam_request_body: String,
    // json stringified headers
    iam_request_headers: HashMap<String, String>,
}

// Universal method for getting infisical token through google auth methods
pub(self) async fn auth_infisical_google(
    client: &mut Client,
    identity_id: Option<String>,
    jwt: Option<String>,
) -> Result<reqwest::Response> {
    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

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

pub(self) async fn auth_infisical_azure(
    client: &mut Client,
    identity_id: Option<String>,
    jwt: Option<String>,
) -> Result<reqwest::Response> {
    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

    let request = request_client
        .post(format!(
            "{}/api/v1/auth/azure-auth/login",
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

pub(self) async fn auth_infisical_kubernetes(
    client: &mut Client,
    identity_id: Option<String>,
    jwt: Option<String>,
) -> Result<reqwest::Response> {
    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

    let request = request_client
        .post(format!(
            "{}/api/v1/auth/kubernetes-auth/login",
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

pub(self) async fn auth_infisical_aws(
    client: &mut Client,
    identity_id: Option<String>,
    iam_data: AwsIamRequestData,
) -> Result<reqwest::Response> {
    let header_json = serde_json::to_string(&iam_data.iam_request_headers).map_err(|e| {
        Error::UnknownErrorWithMessage {
            message: e.to_string(),
        }
    })?;

    let iam_headers = base64_encode(header_json);
    let request_body = base64_encode(iam_data.iam_request_body.clone());

    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

    let mut form_data = HashMap::new();

    form_data.insert("identityId", identity_id);
    form_data.insert("iamHttpRequestMethod", Some(iam_data.http_request_method));
    form_data.insert("iamRequestBody", Some(request_body));
    form_data.insert("iamRequestHeaders", Some(iam_headers));

    let request = request_client
        .post(format!(
            "{}/api/v1/auth/aws-auth/login",
            client.site_url.clone()
        ))
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    let response = request.form(&form_data).send().await?;

    return Ok(response);
}
