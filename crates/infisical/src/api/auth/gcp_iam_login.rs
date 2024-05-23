use google_iamcredentials1::oauth2::{read_service_account_key, ServiceAccountAuthenticator};
use google_iamcredentials1::{hyper, hyper_rustls, IAMCredentials};
use serde::{Deserialize, Serialize};

use crate::api::auth::auth_infisical_google;
use crate::{
    error::{api_error_handler, Error, Result},
    Client,
};
use log::debug;

#[derive(Serialize, Deserialize)]
struct JwtPayload {
    sub: String,
    aud: String,
}

use super::AccessTokenSuccessResponse;

pub async fn gcp_iam_login(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let service_account_key_path;
    let identity_id;

    if let Some(gcp_iam_auth) = &client.auth.gcp_iam {
        service_account_key_path = gcp_iam_auth.service_account_key_file_path.clone();
        identity_id = gcp_iam_auth.identity_id.clone();
    } else {
        return Err(Error::MissingParametersAuthError {
          message: "Attempt to authenticate with GCP IAM failed. Identity ID or service account key path is missing.".to_string(),
      });
    }

    let service_account_key = &read_service_account_key(service_account_key_path).await?;

    // Create an authenticator
    let auth = ServiceAccountAuthenticator::builder(service_account_key.clone())
        .build()
        .await?;

    // We do this to make sure the token is valid
    let token = auth
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await;

    if let Err(e) = token {
        return Err(Error::GoogleTokenError {
            message: e.to_string(),
        });
    }

    // Get the client email from the credentials
    let client_email = service_account_key.client_email.clone();

    // Create the JWT payload
    let jwt_payload = JwtPayload {
        sub: client_email.clone(),
        aud: identity_id.to_string(),
    };

    // Convert the payload to a JSON string
    let payload = serde_json::to_string(&jwt_payload)?;

    debug!("Payload: {}", payload);

    // Create the IAM credentials hub
    let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    let hyper_client = hyper::Client::builder().build(https_connector);
    // let boxed_auth = Box::new(auth) as Box<dyn GetToken>;

    // Create the IAM credentials hub
    let iam_credentials_hub = IAMCredentials::new(hyper_client, auth);

    // Call the IAM service to sign the JWT
    let request = google_iamcredentials1::api::SignJwtRequest {
        payload: Some(payload),
        delegates: None,
    };

    let response = iam_credentials_hub
        .projects()
        .service_accounts_sign_jwt(
            request,
            &format!("projects/-/serviceAccounts/{}", client_email),
        )
        .doit()
        .await;

    if let Err(e) = response {
        return Err(Error::GoogleJwtError {
            message: format!(
                "Are you sure you have enabled the IAM Service Account Credentials API?\n{}",
                e.to_string()
            ),
        });
    }

    let signed_jwt = &response.unwrap().1.signed_jwt;

    debug!("Signed JWT {:?}", signed_jwt.clone());

    // authenticate with infisical!

    let response = auth_infisical_google(client, Some(identity_id), signed_jwt.clone()).await;

    if let Err(e) = response {
        return Err(Error::UnknownErrorWithMessage {
            message: e.to_string(),
        });
    }

    let response = response.unwrap();
    let status = response.status();

    debug!("gcp_id_token_auth_status status: {}", status);

    let status = response.status();

    if status.is_success() {
        let json_response = response.json::<AccessTokenSuccessResponse>().await?;
        return Ok(json_response);
    } else {
        let err = api_error_handler(status, response, None, true).await?;
        return Err(err);
    }
}
