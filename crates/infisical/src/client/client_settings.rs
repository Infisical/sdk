use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::auth_method_settings::AuthenticationOptions;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct ClientSettings {
    // These are optional because the access token can be set directly as well
    #[deprecated]
    #[schemars(
        description = "**DEPRECATED**: The client secret field is deprecated. Please use the new auth object field instead."
    )]
    pub client_secret: Option<String>,

    #[deprecated]
    #[schemars(
        description = "**DEPRECATED**: The client secret field is deprecated. Please use the new auth object field instead."
    )]
    pub client_id: Option<String>,

    #[deprecated]
    #[schemars(
        description = "**DEPRECATED**: The access token field is deprecated. Please use the new auth object field instead."
    )]
    pub access_token: Option<String>,

    #[schemars(
        description = "The URL of the site to connect to. Defaults to \"https://app.infisical.com\"."
    )]
    pub site_url: Option<String>,

    #[schemars(
        description = "cacheTTL controls how often the cache should refresh, default is 300 seconds. Set to 0 to disable the cache."
    )]
    pub cache_ttl: Option<u64>,
    pub user_agent: Option<String>, // We use this to identity which SDK/language was used to make a request.

    #[schemars(
        description = "The SSL certificate path is an optional field that allows you to specify a custom SSL certificate to use for requests made to Infisical.
        This option can be substituted with the `INFISICAL_SSL_CERTIFICATE` environment variable, which should contain the certificate as a string, not the path."
    )]
    pub ssl_certificate_path: Option<String>, // Path to the SSL certificate file.

    #[schemars(
        description = "Configure the authentication method to use.\n\nMake sure to only set one one method at a time to avoid conflicts and unexpected behavior."
    )]
    pub auth: AuthenticationOptions,
}

#[allow(deprecated)]
impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            ssl_certificate_path: None,
            client_secret: None,
            client_id: None,
            access_token: None,
            site_url: None,
            cache_ttl: None,
            auth: AuthenticationOptions::default(),
            user_agent: Some("infisical-unknown-sdk".to_string()),
        }
    }
}
