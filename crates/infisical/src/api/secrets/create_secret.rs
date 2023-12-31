use crate::cache::remove_from_cache;
use crate::error::api_error_handler;
use crate::helper::build_base_request;
use crate::manager::secrets::{CreateSecretOptions, CreateSecretResponse};
use crate::{error::Result, Client};
use log::debug;
use reqwest::StatusCode;

pub async fn create_secret_request(
    client: &mut Client,
    input: &CreateSecretOptions,
) -> Result<CreateSecretResponse> {
    let base_url = format!(
        "{}/api/v3/secrets/raw/{}",
        client.site_url.clone(),
        input.secret_name
    );

    let json = &serde_json::json!({
        "environment": input.environment,
        "workspaceId": input.project_id,
        "secretValue": input.secret_value,

        // conditionally add path, and if its not there add type shared
        "type": input.r#type.as_ref().unwrap_or(&"shared".to_string()), // default is shared
        "secretPath": input.path.as_ref().unwrap_or(&"/".to_string()), // default is "/"
        "secretComment": input.secret_comment.as_ref().unwrap_or(&"".to_string()), // default is "/"
        "skipMultilineEncoding": input.skip_multiline_encoding.as_ref().unwrap_or(&false), // default is "/"

    });

    let base_request = build_base_request(client, &base_url, reqwest::Method::POST);

    let request = match base_request {
        Ok(request) => request,
        Err(e) => return Err(e),
    };

    let token = match client.auth.access_token {
        Some(ref token) => format!("Bearer {}", token),
        None => "".to_string(),
    };

    debug!("Creating secret with token: {}", token);

    debug!("Creating secret with JSON body: {:?}", json);
    debug!("Creating secret with url: {}", base_url);

    let response = request.json(json).send().await?;
    let status = response.status();

    if status == StatusCode::OK {
        let response = response.json::<CreateSecretResponse>().await?;

        // Just to be sure, remove secret from cache since we're since we just created it
        remove_from_cache(
            client,
            &response.secret.secret_key,
            &response.secret.r#type,
            &response.secret.environment,
        );

        Ok(response)
    } else {
        let err =
            api_error_handler(status, response, Some(input.secret_name.clone()), false).await?;
        Err(err)
    }
}
