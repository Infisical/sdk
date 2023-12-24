use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct ClientSettings {
    // These are optional because the access token can be set directly as well
    pub client_secret: Option<String>,
    pub client_id: Option<String>,
    pub access_token: Option<String>,

    // Access token is optional because the user can also provide a machine token.
    pub site_url: Option<String>,

    pub cache_ttl: Option<u64>, // This controls how often the cache should refresh, default is 300 seconds
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            client_secret: None,
            client_id: None,
            access_token: None,
            site_url: None,
            cache_ttl: None,
        }
    }
}
