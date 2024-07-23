use crate::{
    api::auth::auth_infisical_google,
    error::{api_error_handler, Error, Result},
    helper::build_minimal_base_request,
    Client,
};

use super::AccessTokenSuccessResponse;

pub async fn gcp_id_token_login(
    client: &mut Client,
    identity_id: String,
) -> Result<AccessTokenSuccessResponse> {
    let request_client = build_minimal_base_request()?;

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
