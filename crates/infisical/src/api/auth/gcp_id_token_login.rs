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
    // let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ.eyJhdWQiOiI3MWQxYjI3OS0yZmUzLTQ4MjgtODVmNS0yZjRhMmZkYjAyYTEiLCJhenAiOiIxMDY0NDg1NjQzNzYwMzg5MDIwNjYiLCJlbWFpbCI6InRlc3QtdXNlckBpbmZpc2ljYWwtZGV2LmlhbS5nc2VydmljZWFjY291bnQuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImV4cCI6MTcxNTk2NjM4NywiZ29vZ2xlIjp7ImNvbXB1dGVfZW5naW5lIjp7Imluc3RhbmNlX2NyZWF0aW9uX3RpbWVzdGFtcCI6MTcxNTk2MjcwMSwiaW5zdGFuY2VfaWQiOiI4NDE4MTYwNzE1Mjg4NDI3OTQxIiwiaW5zdGFuY2VfbmFtZSI6InRlc3QtaW5zdGFuY2UiLCJwcm9qZWN0X2lkIjoiaW5maXNpY2FsLWRldiIsInByb2plY3RfbnVtYmVyIjo3NDExNzYyOTQ2MTIsInpvbmUiOiJ1cy1jZW50cmFsMS1hIn19LCJpYXQiOjE3MTU5NjI3ODcsImlzcyI6Imh0dHBzOi8vYWNjb3VudHMuZ29vZ2xlLmNvbSIsInN1YiI6IjEwNjQ0ODU2NDM3NjAzODkwMjA2NiJ9.pYn9RCSaGzciPtmzwknii3CDI-zy5H1IZfRWwCNiP8KmvAhNrLgPL6upw330_7Yq9tZ5EPMvPXdzX2NCjqgSlFYXpwbq2ooVGuw8G4VRCiHMZGULvUdyO2qQh3bJh0CTdbeJLjVcgNgJL-qdAvP-T6MYXNwrr0jmHfZo3GjteW02I7yi2vSwfFxHl5ObgiRCvBa7JGKMjatGFW_ROdaz_iNzwbLGbhf34vwp9mDceeHMFTf36bw5RwFkfA-0bLmuTEXpMVzl16SYnzBbUhNNfBUm9_dfoGjMuyQyS3iem-cJIXZsh67mpkNnN3ZbH-E4HffqlNjiVkyTXDP2kI-agw".to_string();

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
