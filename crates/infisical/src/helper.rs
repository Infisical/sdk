use crate::{
    api::access_token::access_token_request,
    error::{Error, Result},
    manager::secrets::Secret,
    Client,
};
use log::debug;
use reqwest;
pub async fn handle_authentication(client: &mut Client) -> Result<()> {
    if client.auth.access_token == None {
        let res = access_token_request(client).await?;

        debug!("New MI token: {}", res.access_token);

        if res.access_token.len() > 0 {
            client.set_access_token(res.access_token);
        }
    }
    Ok(())
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

    let base_request = reqwest::Client::new()
        .request(method, url)
        // Setting JSON as the content type is OK since we only work with JSON.
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header("Authorization", token)
        .header(reqwest::header::USER_AGENT, client.user_agent.clone());

    // we need to be able to do .json() on this request
    // .json(json)
    // .send()
    // .await?;

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
