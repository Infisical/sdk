use std::borrow::Cow;

use crate::{
    api::auth::{
        aws_iam_login::aws_iam_login, azure_login::azure_login, gcp_iam_login::gcp_iam_login,
        gcp_id_token_login::gcp_id_token_login, kubernetes_login::kubernetes_login,
        universal_auth_login::universal_auth_login,
    },
    client::auth_method_settings::AuthMethod,
    constants::{AWS_EC2_INSTANCE_IDENTITY_DOCUMENT_URL, AWS_EC2_METADATA_TOKEN_URL},
    error::{Error, Result},
    manager::secrets::Secret,
    Client,
};
use log::debug;
use reqwest::{self, header::HeaderValue};
use serde::{Deserialize, Serialize};
pub async fn handle_authentication(client: &mut Client) -> Result<()> {
    if client.auth.access_token.is_some() {
        return Ok(());
    }

    let validation_result = client.auth.validate();

    if validation_result.is_err() {
        let err = Error::AuthSanitizationError {
            message: validation_result
                .err()
                .unwrap_or("Unknown error in auth validation".to_string())
                .to_string(),
        };

        return Err(err);
    };
    debug!("Auth validation passed");

    let auth_method = validation_result.unwrap_or(AuthMethod::UniversalAuth);

    let result;

    match auth_method {
        AuthMethod::UniversalAuth => {
            debug!("Auth method is Universal Auth");
            result = universal_auth_login(client).await?;
        }
        AuthMethod::GcpIdToken => {
            debug!("Auth method is GCP ID Token");
            result = gcp_id_token_login(client).await?;
        }
        AuthMethod::GcpIam => {
            debug!("Auth method is GCP IAM");
            result = gcp_iam_login(client).await?;
        }

        AuthMethod::AwsIam => {
            debug!("Auth method is AWS IAM");
            result = aws_iam_login(client).await?;
        }

        AuthMethod::Kubernetes => {
            debug!("Auth method is Kubernetes");
            result = kubernetes_login(client).await?;
        }

        AuthMethod::Azure => {
            debug!("Auth method is Azure");
            result = azure_login(client).await?;
        }
    }

    if result.access_token.is_empty() {
        debug!("No access token obtained");
        return Err(Error::NoAccessTokenObtained);
    }

    debug!("Setting access token");
    client.set_access_token(result.access_token);
    Ok(())
}

pub fn ensure_unique_secrets_by_key(secrets: &mut Vec<Secret>) {
    let mut secret_map = std::collections::HashMap::new();

    // Use the entry API to check and insert if the key does not exist
    // This will result in the "deepest" secret being kept (e.g like we do in the CLI)
    for secret in std::mem::take(secrets) {
        secret_map
            .entry(secret.secret_key.clone())
            .or_insert(secret);
    }

    // Clear the original vector and extend it with the unique secrets
    secrets.clear();
    secrets.extend(secret_map.into_iter().map(|(_, v)| v));
}
pub fn set_env_vars(should_attach_envs: bool, secrets: &Vec<Secret>) {
    if !should_attach_envs {
        return;
    }

    for secret in secrets {
        // check if a env variable with the same name already exists, if it does, skip
        if std::env::var(&secret.secret_key).is_ok() {
            continue;
        }

        std::env::set_var(&secret.secret_key, &secret.secret_value);
    }
}

pub fn build_base_request(
    client: &mut Client,
    url: &str,
    method: reqwest::Method,
) -> Result<reqwest::RequestBuilder> {
    let token = match client.auth.access_token {
        Some(ref token) => format!("Bearer {}", token),
        None => Err(Error::MissingAccessToken)?,
    };

    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build();

    if request_client.is_err() {
        return Err(Error::Reqwest(request_client.err().unwrap()))?;
    }

    let base_request = request_client?
        .request(method, url)
        // Setting JSON as the content type is OK since we only work with JSON.
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header("Authorization", token)
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    Ok(base_request)
}

// It takes in a URL string, and a hashmap of query parameters.
pub fn build_url(url: String, query_params: &serde_json::Value) -> String {
    let mut url = url.to_string();

    if query_params.is_null() {
        return url;
    }

    let query_params = query_params.as_object().unwrap();

    if query_params.len() > 0 {
        url.push_str("?");

        for (key, value) in query_params {
            // The value might be an option, so we need to make sure its not

            let val = match value.as_str() {
                Some(val) => val,
                None => "",
            };

            if val.len() == 0 {
                continue;
            }

            url.push_str(&format!("{}={}&", key, val));
        }

        // Remove the last "&"
        url.pop();
    }

    return url.to_string();
}

pub fn get_fallback_env_secret(key: &str) -> Option<Secret> {
    let fallback = std::env::var(key);

    let default_secret = Secret {
        is_fallback: true,
        version: 0,
        workspace: "".to_string(),
        secret_comment: "".to_string(),
        r#type: "".to_string(),
        environment: "".to_string(),

        secret_key: key.to_string(),
        secret_value: "".to_string(),
    };

    match fallback {
        Ok(val) => Some(Secret {
            secret_value: val,
            ..default_secret
        }),
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct IdentityDocumentResponse {
    region: String,
}

async fn get_aws_ec2_identity_document(timeout: u64) -> Result<IdentityDocumentResponse> {
    let mut token_headers = reqwest::header::HeaderMap::new();
    token_headers.insert(
        "X-aws-ec2-metadata-token-ttl-seconds",
        HeaderValue::from_str("21600").unwrap(),
    );

    let request_client = reqwest::Client::builder()
        .use_preconfigured_tls(rustls_platform_verifier::tls_config())
        .build()?;

    // Get the token from the metadata service. This is required to fetch the identity document.
    let token_response = request_client
        .get(AWS_EC2_METADATA_TOKEN_URL)
        .headers(token_headers)
        .timeout(std::time::Duration::from_millis(timeout))
        .send()
        .await?;

    let token = token_response.text().await?;

    // Get the identity document from the metadata service, which will contain the region if it's an EC2 instance.
    let mut identity_doc_headers = reqwest::header::HeaderMap::new();
    identity_doc_headers.insert(
        "X-aws-ec2-metadata-token",
        HeaderValue::from_str(&token).unwrap(),
    );

    let identity_doc_response = request_client
        .get(AWS_EC2_INSTANCE_IDENTITY_DOCUMENT_URL)
        .headers(identity_doc_headers)
        .timeout(std::time::Duration::from_millis(timeout))
        .send()
        .await?;

    let identity_doc = identity_doc_response
        .json::<IdentityDocumentResponse>()
        .await?;

    if identity_doc.region.is_empty() {
        return Err(Error::UnknownErrorWithMessage {
            message: "Failed to get region from EC2 instance metadata".to_string(),
        });
    }

    return Ok(identity_doc);
}

pub async fn get_aws_region<'a>() -> Result<Cow<'a, str>> {
    // in Lambda, the region is available in the environment variable AWS_REGION, but it might not be available in other environments. we should check for it and use it if it's available
    if let Ok(region) = std::env::var("AWS_REGION") {
        if !region.is_empty() {
            return Ok(Cow::Owned(region));
        }
    }

    // in EC2 instances, the region is available in the identity document
    let identity_doc = get_aws_ec2_identity_document(5000).await;

    if let Ok(identity_doc) = identity_doc {
        return Ok(Cow::Owned(identity_doc.region));
    }

    return Err(Error::UnknownErrorWithMessage {
        message: "Failed to find AWS region.".to_string(),
    });
}
