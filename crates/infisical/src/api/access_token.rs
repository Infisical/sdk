use crate::{
    error::{api_error_handler, Result},
    Client,
};
use log::debug;
use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenSuccessResponse {
    pub access_token: String,
}

pub async fn access_token_request(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let req_client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert(
        "clientId",
        client.auth.client_id.clone().to_string().to_owned(),
    );
    body.insert(
        "clientSecret",
        client.auth.client_secret.clone().to_string().to_owned(),
    );

    let object = serde_json::to_string(&body).unwrap();

    let url = format!(
        "{}/api/v1/auth/universal-auth/login",
        client.site_url.clone()
    );

    let request = req_client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    let response = request.body(object).send().await?;

    debug!("access_token_request status: {}", response.status());

    let status = response.status();

    if status == StatusCode::OK {
        let response = response.json::<AccessTokenSuccessResponse>().await?;

        Ok(response)
    } else {
        let err = api_error_handler(status, response, None, true).await?;
        Err(err)
    }
}
