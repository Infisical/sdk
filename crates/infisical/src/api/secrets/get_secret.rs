use crate::cache::{add_to_cache, create_cache_key, get_secret_from_cache};
use crate::error::api_error_handler;
use crate::helper::{build_base_request, build_url, get_fallback_env_secret};
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
        "secretPath": input.path.clone().unwrap_or("/".to_string()), // default is "/"
        "expandSecretReferences": input.expand_secret_references.unwrap_or(true).to_string(),
        "type": input.r#type.clone().unwrap_or("shared".to_string()), // default is shared
        "include_imports": input.include_imports.unwrap_or(false).to_string(),
    });

    let secret_type = match input.r#type.as_ref() {
        Some(r#type) => r#type,
        None => "shared",
    };

    let secret_path = match input.path.as_ref() {
        Some(path) => path,
        None => "/",
    };

    let cached_secret = get_secret_from_cache(
        client,
        &create_cache_key(
            &input.secret_name,
            secret_type,
            &input.environment,
            secret_path,
        ),
    );

    if cached_secret.is_some() {
        return Ok(GetSecretResponse {
            secret: cached_secret.unwrap(),
        });
    }

    let url = build_url(base_url, json);

    let base_request = build_base_request(client, &url, reqwest::Method::GET).await?;

    let token = match client.auth.access_token {
        Some(ref token) => format!("Bearer {}", token),
        None => "".to_string(),
    };

    debug!("Getting secret with token: {}", token);

    debug!("Getting secret with body: {:?}", input);
    debug!("Getting secret with url: {}", url);

    let response = base_request.send().await?;

    let status = response.status();

    if status == StatusCode::OK {
        let response = response.json::<GetSecretResponse>().await?;

        add_to_cache(
            client,
            &response.secret,
            input.path.as_ref().unwrap_or(&"/".to_string()),
        );

        Ok(response)
    } else {
        let fallback_secret = get_fallback_env_secret(&input.secret_name);

        if fallback_secret.is_some() {
            return Ok(GetSecretResponse {
                secret: fallback_secret.unwrap(),
            });
        }

        let err =
            api_error_handler(status, response, Some(input.secret_name.clone()), false).await?;
        Err(err)
    }
}
