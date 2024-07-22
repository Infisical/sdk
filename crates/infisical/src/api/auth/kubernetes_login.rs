use log::debug;

use crate::{
    error::{api_error_handler, Error, Result},
    Client,
};

use super::{auth_infisical_kubernetes, AccessTokenSuccessResponse};

pub async fn kubernetes_login(
    client: &mut Client,
    identity_id: String,
    service_account_token_path: String,
) -> Result<AccessTokenSuccessResponse> {
    debug!(
        "Reading service account token from path: {}",
        service_account_token_path
    );

    let account_token = String::from_utf8(tokio::fs::read(service_account_token_path).await?)
        .map_err(|e| Error::UnknownErrorWithMessage {
            message: e.to_string(),
        })?;

    debug!(
        "First 10 characters of the K8's account token: {:?}",
        &account_token[0..10]
    );

    let response =
        auth_infisical_kubernetes(client, Some(identity_id), Some(account_token)).await?;

    let status = response.status();

    if status.is_success() {
        let response = response.json::<AccessTokenSuccessResponse>().await?;
        Ok(response)
    } else {
        let err = api_error_handler(status, response, None, true).await?;
        Err(err)
    }
}
