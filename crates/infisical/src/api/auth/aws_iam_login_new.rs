use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::api::auth::{auth_infisical_aws, AccessTokenSuccessResponse, AwsIamRequestData};
use crate::error::{api_error_handler, Error, Result};
use crate::Client;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::BehaviorVersion;
use aws_credential_types::provider::ProvideCredentials;
use aws_credential_types::Credentials;
use aws_sigv4::{
    http_request::{sign, SignableBody, SignableRequest, SigningSettings},
    sign::v4,
};
use bytecount;

use log::debug;

pub async fn aws_iam_login(client: &mut Client) -> Result<AccessTokenSuccessResponse> {
    let identity_id;

    if let Some(aws_iam_auth) = &client.auth.aws_iam {
        identity_id = aws_iam_auth.identity_id.clone();
    } else {
        return Err(Error::MissingParametersAuthError {
            message: "Attempt to authenticate with AWS IAM failed. Identity ID is missing."
                .to_string(),
        });
    }

    let region = "us-east-1";

    let credentials = DefaultCredentialsChain::builder()
        .region(region)
        .build()
        .await
        .provide_credentials()
        .await
        .expect("Failed to get credentials");

    let identity = credentials.into();

    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(Duration::from_secs(900));
    signing_settings.signature_location = aws_sigv4::http_request::SignatureLocation::QueryParams;

    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name("sts")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build();

    if let Err(e) = signing_params {
        return Err(Error::UnknownErrorWithMessage {
            message: e.to_string(),
        });
    }
    let signing_params = signing_params.unwrap();

    let iam_request_url = format!("https://sts.{}.amazonaws.com/", region);
    let iam_request_body = "Action=GetCallerIdentity&Version=2011-06-15";

    let mut headers = HashMap::<String, String>::new();

    headers.insert(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded; charset=utf-8".to_string(),
    );
    headers.insert("Host".to_string(), format!("sts.{}.amazonaws.com", region));
    headers.insert(
        "Content-Length".to_string(),
        bytecount::num_chars(iam_request_body.as_bytes()).to_string(),
    );

    // ! maybe need to insert the date header here

    let signable_request = SignableRequest::new(
        "POST",
        &iam_request_url,
        headers.iter().map(|(k, v)| (k.as_str(), v.as_str())),
        SignableBody::Bytes(iam_request_body.as_bytes()),
    );

    if let Err(e) = signable_request {
        return Err(Error::UnknownErrorWithMessage {
            message: e.to_string(),
        });
    }
    let signable_request = signable_request.unwrap();

    let (signing_instructions, _signature) = sign(signable_request, &signing_params.into())
        .unwrap()
        .into_parts();

    let mut url = url::Url::parse(&iam_request_url).unwrap();
    for (name, value) in signing_instructions.params() {
        url.query_pairs_mut().append_pair(name, &value);
    }

    debug!("URL: {}", url);
    debug!("URL: {}", url);
    debug!("URL: {}", url);
    debug!("URL: {}", url);
    debug!("URL: {}", url);
    debug!("URL: {}", url);

    let iam_data = AwsIamRequestData {
        http_request_method: "POST".to_string(),
        iam_request_url: url.to_string(),
        iam_request_body: iam_request_body.to_string(),
        iam_request_headers: headers,
    };

    let response = auth_infisical_aws(client, Some(identity_id), iam_data).await?;
    let status = response.status();

    debug!("aws_iam_login status: {}", status);

    let status = response.status();

    if status.is_success() {
        let json_response = response.json::<AccessTokenSuccessResponse>().await?;
        return Ok(json_response);
    } else {
        let err = api_error_handler(status, response, None, true).await?;
        return Err(err);
    }
}
