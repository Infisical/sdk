use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct ClientSettings {
    // These are optional because the access token can be set directly as well
    pub client_secret: Option<String>,
    pub client_id: Option<String>,

    // Access token is optional because the user can also provide maci
    pub access_token: Option<String>,
    pub site_url: Option<String>,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            client_secret: None,
            client_id: None,
            access_token: None,
            site_url: None,
        }
    }
}
