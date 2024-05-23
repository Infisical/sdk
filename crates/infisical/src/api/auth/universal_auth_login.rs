use crate::{
    error::{api_error_handler, Error, Result},
    Client,
};
use log::debug;

use std::collections::HashMap;

use super::AccessTokenSuccessResponse;

pub async fn universal_auth_login(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let client_id;
    let client_secret;

    if let Some(universal_auth) = &client.auth.universal_auth {
        client_id = universal_auth.client_id.clone();
        client_secret = universal_auth.client_secret.clone();
    } else {
        return Err(Error::MissingParametersAuthError {
            message: "Attempt to authenticate with Universal Auth failed. Universal auth credentials are missing.".to_string(),
        });
    }

    let mut body = HashMap::new();
    body.insert("clientId", Some(client_id));
    body.insert("clientSecret", Some(client_secret));
    let request_body = serde_json::to_string(&body).unwrap();

    let url = format!(
        "{}/api/v1/auth/universal-auth/login",
        client.site_url.clone()
    );

    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

    let request = request_client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    let response = request.body(request_body).send().await?;

    debug!("universal_auth_login status: {}", response.status());

    let status = response.status();

    if status.is_success() {
        let response = response.json::<AccessTokenSuccessResponse>().await?;
        Ok(response)
    } else {
        let err = api_error_handler(status, response, None, true).await?;
        Err(err)
    }
}
