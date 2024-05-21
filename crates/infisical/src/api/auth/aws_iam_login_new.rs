use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::error::{Error, Result};
use crate::Client;
use aws_config::BehaviorVersion;
use aws_credential_types::provider::ProvideCredentials;
use aws_sigv4::{
    http_request::{sign, SignableBody, SignableRequest, SigningSettings},
    sign::v4,
};
use log::debug;

pub async fn aws_iam_login(client: &mut Client) -> Result<()> {
    let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

    let credentials = config
        .credentials_provider()
        .expect("no credentials provider found")
        .provide_credentials()
        .await
        .expect("unable to load credentials");

    let identity = credentials.into();
    let region = config.region().unwrap().to_string();

    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(Duration::from_secs(900));
    signing_settings.signature_location = aws_sigv4::http_request::SignatureLocation::QueryParams;

    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(&region)
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
    headers.insert("Host".to_string(), format!("sts.${}.amazonaws.com", region));

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

    return Ok(());
}
