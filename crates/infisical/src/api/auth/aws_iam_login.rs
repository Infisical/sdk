use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::api::auth::{auth_infisical_aws, AccessTokenSuccessResponse, AwsIamRequestData};

use crate::error::{api_error_handler, Error, Result};
use crate::Client;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_credential_types::provider::ProvideCredentials;
use aws_sigv4::{
    http_request::{sign, SignableBody, SignableRequest, SigningSettings},
    sign::v4,
};

use crate::helper::get_aws_region;
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

    let aws_region = get_aws_region().await?.into_owned();
    let aws_region_str: &'static str = Box::leak(aws_region.into_boxed_str());

    let credentials = DefaultCredentialsChain::builder()
        .region(aws_region_str) // Convert Cow<str> to &str
        .build()
        .await
        .provide_credentials()
        .await
        .map_err(|e| Error::AwsCredentialsError {
            message: e.to_string(),
        })?;

    let identity = credentials.into();

    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(Duration::from_secs(900));

    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(aws_region_str) // Use a reference to the owned String
        .name("sts")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()
        .map_err(|e| Error::AwsBuildRequestSignerError {
            message: e.to_string(),
        })?;

    let iam_request_url = format!("https://sts.{}.amazonaws.com/", aws_region_str);
    let iam_request_body = "Action=GetCallerIdentity&Version=2011-06-15";

    let headers: HashMap<String, String> = [
        (
            "Host".to_string(),
            format!("sts.{}.amazonaws.com", aws_region_str),
        ),
        // These are here so the SignedHeaders are correct
        ("X-Amz-Date".to_string(), "tmp".to_string()),
        ("X-Amz-Security-Token".to_string(), "tmp".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let signable_request = SignableRequest::new(
        "POST",
        &iam_request_url,
        headers.iter().map(|(k, v)| (k.as_str(), v.as_str())),
        SignableBody::Bytes(iam_request_body.as_bytes()),
    )
    .map_err(|e| Error::AwsSignRequestError {
        message: e.to_string(),
    })?;

    let (signing_instructions, _signature) = sign(signable_request, &signing_params.into())
        .unwrap()
        .into_parts();

    let mut signed_request: http::Request<String> =
        http::Request::new(iam_request_body.to_string());
    signing_instructions.apply_to_request_http1x(&mut signed_request);

    let iam_data = AwsIamRequestData {
        http_request_method: "POST".to_string(),
        iam_request_body: iam_request_body.to_string(),
        iam_request_headers: signed_request
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
            .collect(),
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
