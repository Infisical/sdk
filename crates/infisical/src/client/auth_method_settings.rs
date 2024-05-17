use log::debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::constants::{
    INFISICAL_GCP_AUTH_IDENTITY_ID_ENV_NAME, INFISICAL_UNIVERSAL_AUTH_CLIENT_ID_ENV_NAME,
    INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET_ENV_NAME,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UniversalAuthMethod {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GCPAuthMethod {
    pub identity_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct Authentication {
    pub access_token: Option<String>,
    pub universal_auth: Option<UniversalAuthMethod>,
    pub gcp_auth: Option<GCPAuthMethod>,
}

impl Default for Authentication {
    fn default() -> Self {
        Self {
            access_token: None,
            universal_auth: None,
            gcp_auth: None,
        }
    }
}

#[derive(Debug)]
pub enum AuthMethod {
    UniversalAuth,
    GcpIdToken,
}

// Custom validation to ensure that if universal_auth or gcp_auth are present, their fields are populated
impl Authentication {
    pub fn validate(&mut self) -> Result<AuthMethod, String> {
        // UNIVERSAL AUTH:
        if let Some(ref auth) = self.universal_auth {
            if !auth.client_id.is_empty() && !auth.client_secret.is_empty() {
                return Ok(AuthMethod::UniversalAuth);
            }

            return Err("universal_auth is present but client_id or client_secret is empty".into());
        }
        // GCP AUTH:
        if let Some(ref auth) = self.gcp_auth {
            if !auth.identity_id.is_empty() {
                return Ok(AuthMethod::GcpIdToken);
            }

            return Err("gcp_auth is present but identity_id is empty".into());
        } else {
            debug!("No authentication method is set. Checking environment variables.");

            let universal_auth_client_id_env =
                std::env::var(INFISICAL_UNIVERSAL_AUTH_CLIENT_ID_ENV_NAME).unwrap_or_default();
            let universal_auth_client_secret_env =
                std::env::var(INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET_ENV_NAME).unwrap_or_default();

            let gcp_auth_identity_id_env =
                std::env::var(INFISICAL_GCP_AUTH_IDENTITY_ID_ENV_NAME).unwrap_or_default();

            // universal auth env check
            if !universal_auth_client_id_env.is_empty()
                && !universal_auth_client_secret_env.is_empty()
            {
                self.universal_auth = Some(UniversalAuthMethod {
                    client_id: universal_auth_client_id_env,
                    client_secret: universal_auth_client_secret_env,
                });

                return Ok(AuthMethod::UniversalAuth);
            }
            // gcp auth env check
            if !gcp_auth_identity_id_env.is_empty() {
                self.gcp_auth = Some(GCPAuthMethod {
                    identity_id: gcp_auth_identity_id_env,
                });

                return Ok(AuthMethod::GcpIdToken);
            }

            return Err("No authentication method is set.".into());
        }
    }
}