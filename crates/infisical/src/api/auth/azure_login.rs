use serde::{Deserialize, Serialize};

use crate::{
    constants::AZURE_METADATA_SERVICE_URL,
    error::{api_error_handler, Error, Result},
    Client,
};

use super::{auth_infisical_azure, AccessTokenSuccessResponse};

#[derive(Serialize, Deserialize, Debug)]
struct AzureSuccessResponse {
    access_token: String,
}

pub async fn azure_login(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let identity_id;

    if let Some(azure_auth) = &client.auth.azure {
        identity_id = azure_auth.identity_id.clone();
    } else {
        return Err(Error::MissingParametersAuthError {
            message: "Attempt to authenticate with Azure. Identity ID is missing.".to_string(),
        });
    }

    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()
        .unwrap();

    let metadata_request = request_client
        .get(AZURE_METADATA_SERVICE_URL)
        .header("Metadata", "true")
        .header(reqwest::header::ACCEPT, "application/json");

    let azure_response = metadata_request.send().await?;

    if !azure_response.status().is_success() {
        let err = api_error_handler(azure_response.status(), azure_response, None, false).await?;
        return Err(err);
    }

    let azure_metadata = azure_response.json::<AzureSuccessResponse>().await?;

    let response =
        auth_infisical_azure(client, Some(identity_id), Some(azure_metadata.access_token)).await?;

    if !response.status().is_success() {
        let err = api_error_handler(response.status(), response, None, false).await?;
        return Err(err);
    }

    let response_json = response.json::<AccessTokenSuccessResponse>().await?;
    return Ok(response_json);
}
