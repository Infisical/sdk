use std::borrow::Borrow;
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

    debug!("Access key ID {}", credentials.borrow().access_key_id());
    debug!(
        "Secret access key {}",
        credentials.borrow().secret_access_key()
    );

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
        iam_request_body.len().to_string(),
    );

    // we do this so the signed headers will contain the date header
    headers.insert("X-Amz-Date".to_string(), "tmp".to_string());

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

    let signing_headers: Vec<_> = signing_instructions.headers().collect();
    let signing_params = signing_instructions.params();

    debug!("SIGNING PARAMS: {:?}", signing_params);
    debug!("SIGNING HEADERS: {:?}", signing_headers);

    // let url = url::Url::parse(&iam_request_url).unwrap();
    for (name, value) in signing_instructions.params() {
        // url.query_pairs_mut().append_pair(name, &value);

        debug!("HEADER() ---:---:--- {}: {}", name, value);
        headers.insert(name.to_string(), value.to_string());
    }

    // Authorization: AWS4-HMAC-SHA256 Credential=AKIAI44QH8DHBEXAMPLE/20160126/us-east-1/sts/aws4_request, SignedHeaders=host;user-agent;x-amz-date, Signature=1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
    // build the authorization header like above
    let auth_header = format!(
        "{} Credential={}, SignedHeaders={}, Signature={}",
        headers.get("X-Amz-Algorithm").unwrap(),
        headers.get("X-Amz-Credential").unwrap(),
        headers.get("X-Amz-SignedHeaders").unwrap(),
        headers.get("X-Amz-Signature").unwrap()
    );

    headers.insert("Authorization".to_string(), auth_header);

    // debug!("URL: {}", url);

    let iam_data = AwsIamRequestData {
        http_request_method: "POST".to_string(),
        iam_request_body: iam_request_body.to_string(),
        iam_request_headers: headers,
    };

    // this is where we send the request to infisical, just pretend this works as it should
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
