use crate::error::api_error_handler;
use crate::helper::{build_base_request, build_url};
use crate::manager::secrets::{ListSecretsOptions, ListSecretsResponse, Secret};
use crate::{error::Result, Client};
use log::debug;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListSecretsResponseImports {
    secret_path: String,
    folder_id: String,
    environment: String,
    secrets: Vec<Secret>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ImportResponse {
    pub(crate) imports: Vec<ListSecretsResponseImports>,
    pub(crate) secrets: Vec<Secret>,
}

pub async fn list_secrets_request(
    client: &mut Client,
    input: &ListSecretsOptions,
) -> Result<ListSecretsResponse> {
    let base_url = format!("{}/api/v3/secrets/raw", client.site_url.clone(),);

    let json = &serde_json::json!({
        "environment": input.environment,
        "workspaceId": input.project_id,

        "secretPath": input.path.as_ref().unwrap_or(&"/".to_string()), // default is "/"
        "include_imports": input.include_imports.unwrap_or(false).to_string(),

    });

    let url = build_url(base_url, json);

    let base_request = build_base_request(client, &url, reqwest::Method::GET);

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
    debug!("Creating secret with url: {}", url);

    let response = request.json(json).send().await?;
    let status = response.status();

    if status == StatusCode::OK {
        if input.include_imports.unwrap_or(false) == true {
            let response = response.json::<ImportResponse>().await?;

            let mut secrets = response.secrets.clone();

            for import in response.imports {
                secrets.extend(import.secrets);
            }

            if input.attach_to_process_env.unwrap_or(false) == true {
                for secret in secrets.clone() {
                    std::env::set_var(secret.secret_key, secret.secret_value);
                }
            }

            return Ok(ListSecretsResponse { secrets });
        }

        let response = response.json::<ListSecretsResponse>().await?;

        if input.attach_to_process_env.unwrap_or(false) == true {
            let secrets = response.secrets.clone();

            for secret in secrets {
                std::env::set_var(secret.secret_key, secret.secret_value);
            }
        }

        Ok(response)
    } else {
        let err = api_error_handler(status, response, None, false).await?;
        Err(err)
    }
}
