use crate::cache::{add_to_cache, create_cache_key, get_secret_from_cache};
use crate::error::api_error_handler;
use crate::helper::{build_base_request, build_url};
use crate::manager::secrets::{GetSecretOptions, GetSecretResponse};
use crate::{error::Result, Client};
use log::debug;
use reqwest::StatusCode;

pub async fn get_secret_request(
    client: &mut Client,
    input: &GetSecretOptions,
) -> Result<GetSecretResponse> {
    let base_url = format!(
        "{}/api/v3/secrets/raw/{}",
        client.site_url.clone(),
        &input.secret_name
    );

    let json: &serde_json::Value = &serde_json::json!({
        "workspaceId": input.project_id,
        "environment": input.environment,
        "secretPath": input.path.as_ref().unwrap_or(&"/".to_string()), // default is "/"
        "type": input.r#type.as_ref().unwrap_or(&"shared".to_string()), // default is shared
        "include_imports": input.include_imports.as_ref().unwrap_or(&false), // default is false
    });

    let secret_type = match input.r#type.as_ref() {
        Some(r#type) => r#type,
        None => "shared",
    };
    let cached_secret = get_secret_from_cache(
        client,
        &create_cache_key(&input.secret_name, secret_type, &input.environment),
    );

    if cached_secret.is_some() {
        return Ok(GetSecretResponse {
            secret: cached_secret.unwrap(),
        });
    }

    let url = build_url(base_url, json);

    let base_request = build_base_request(client, &url, reqwest::Method::GET);

    let token = match client.auth.access_token {
        Some(ref token) => format!("Bearer {}", token),
        None => "".to_string(),
    };

    debug!("Getting secret with token: {}", token);

    debug!("Getting secret with body: {:?}", input);
    debug!("Getting secret with url: {}", url);

    let request = match base_request {
        Ok(request) => request,
        Err(e) => return Err(e),
    };

    let response = request.send().await?;

    let status = response.status();

    if status == StatusCode::OK {
        let response = response.json::<GetSecretResponse>().await?;

        add_to_cache(client, &response.secret);

        Ok(response)
    } else {
        let err =
            api_error_handler(status, response, Some(input.secret_name.clone()), false).await?;
        Err(err)
    }
}
