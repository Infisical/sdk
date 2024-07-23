use crate::{
    error::{api_error_handler, Result},
    helper::build_base_request,
    Client,
};
use log::debug;

use std::collections::HashMap;

use super::AccessTokenSuccessResponse;

pub async fn universal_auth_login(
    client: &mut Client,
    client_id: String,
    client_secret: String,
) -> Result<AccessTokenSuccessResponse> {
    let mut body = HashMap::new();
    body.insert("clientId", Some(client_id));
    body.insert("clientSecret", Some(client_secret));
    let request_body = serde_json::to_string(&body).unwrap();

    let url = format!(
        "{}/api/v1/auth/universal-auth/login",
        client.site_url.clone()
    );

    let request_client = build_base_request(client, &url, reqwest::Method::POST).await?;

    let request = request_client
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
