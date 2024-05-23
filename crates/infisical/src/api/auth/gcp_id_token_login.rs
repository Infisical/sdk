use crate::{
    api::auth::auth_infisical_google,
    error::{api_error_handler, Error, Result},
    Client,
};

use super::AccessTokenSuccessResponse;

pub async fn gcp_id_token_login(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let identity_id;

    if let Some(gcp_id_token_auth) = &client.auth.gcp_id_token {
        identity_id = gcp_id_token_auth.identity_id.clone();
    } else {
        return Err(Error::MissingParametersAuthError {
            message: "Attempt to authenticate with GCP ID Token failed. Identity ID is missing."
                .to_string(),
        });
    }

    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()
        .unwrap();

    let metadata_request = request_client
        .get(format!(
          "http://metadata/computeMetadata/v1/instance/service-accounts/default/identity?audience={}&format=full",
          identity_id
      ))
        .header("Metadata-Flavor", "Google");

    let metadata_response = metadata_request.send().await?;

    let status = metadata_response.status();

    if !status.is_success() {
        return Err(Error::GoogleMetadataError);
    }

    let id_token = metadata_response.text().await?;
    let response = auth_infisical_google(client, Some(identity_id), Some(id_token)).await;

    if let Err(e) = response {
        return Err(Error::UnknownErrorWithMessage {
            message: e.to_string(),
        });
    }

    let response = response.unwrap();
    let status = response.status();

    if status.is_success() {
        let response = response.json::<AccessTokenSuccessResponse>().await?;
        Ok(response)
    } else {
        let err = api_error_handler(status, response, None, true).await?;
        Err(err)
    }
}
