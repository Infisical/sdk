use crate::{
    api::universal_auth_login::universal_auth_login,
    client::auth_method_settings::AuthMethod,
    error::{Error, Result},
    manager::secrets::Secret,
    Client,
};
use log::debug;
use reqwest;
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
    } else {
        debug!("Auth validation passed");
    }

    let auth_method = validation_result.unwrap_or(AuthMethod::UniversalAuth);
    let access_token;

    match auth_method {
        AuthMethod::UniversalAuth => {
            debug!("Auth method is Universal Auth");
            let result = universal_auth_login(client).await?;
            access_token = result.access_token;
        }
        AuthMethod::GcpIdToken => {
            debug!("Auth method is GCP ID Token!!!");
            debug!(
                "GCP Identity ID: {}",
                client.auth.gcp_auth.as_ref().unwrap().identity_id
            );
            access_token = "NOT_IMPLEMENTED".to_string();
        }
    }

    debug!("New access token token: {}", access_token);

    if access_token.is_empty() {
        debug!("No access token obtained");
        return Err(Error::NoAccessTokenObtained);
    }

    debug!("Setting access token");
    client.set_access_token(access_token);
    Ok(())
}

pub fn ensure_unique_secrets_by_key(secrets: &mut Vec<Secret>) {
    let mut secret_map = std::collections::HashMap::new();

    // Iterate over the secrets and insert them into the map
    for secret in std::mem::take(secrets) {
        secret_map.insert(secret.secret_key.clone(), secret);
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

            url.push_str(key);
            url.push_str("=");
            url.push_str(val);
            url.push_str("&");
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
