use log::debug;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::constants::{
    INFISICAL_AWS_IAM_AUTH_IDENTITY_ID_ENV_NAME, INFISICAL_AZURE_AUTH_IDENTITY_ID_ENV_NAME,
    INFISICAL_GCP_AUTH_IDENTITY_ID_ENV_NAME,
    INFISICAL_GCP_IAM_SERVICE_ACCOUNT_KEY_FILE_PATH_ENV_NAME,
    INFISICAL_KUBERNETES_IDENTITY_ID_ENV_NAME,
    INFISICAL_KUBERNETES_SERVICE_ACCOUNT_TOKEN_PATH_ENV_NAME,
    INFISICAL_UNIVERSAL_AUTH_CLIENT_ID_ENV_NAME, INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET_ENV_NAME,
};

fn default_kubernetes_service_account_token_path() -> Option<String> {
    Some("/var/run/secrets/kubernetes.io/serviceaccount/token".to_string())
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UniversalAuthMethod {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GCPIdTokenAuthMethod {
    pub identity_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GCPIamAuthMethod {
    #[schemars(
        description = "The path to the GCP Service Account key file.\n\n You can generate this key file by going to the GCP Console -> IAM & Admin -> Service Accounts -> *Select your service account* -> Keys tab -> Add key.\nNote: The key must be in JSON format."
    )]
    pub service_account_key_file_path: String,
    pub identity_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AWSIamAuthMethod {
    #[schemars(
        description = "The Infisical Identity ID that you want to authenticate to Infisical with."
    )]
    pub identity_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesAuthMethod {
    #[schemars(
        description = "The Infisical Identity ID that you want to authenticate to Infisical with."
    )]
    pub identity_id: String,

    #[schemars(
        description = "The path to the Kubernetes Service Account token file.\n\nIf no path is provided, it will default to /var/run/secrets/kubernetes.io/serviceaccount/token."
    )]
    #[serde(default = "default_kubernetes_service_account_token_path")]
    pub service_account_token_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AzureAuthMethod {
    #[schemars(
        description = "The Infisical Identity ID that you want to authenticate to Infisical with."
    )]
    pub identity_id: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(default, rename_all = "camelCase")]
pub struct AuthenticationOptions {
    pub access_token: Option<String>,
    pub universal_auth: Option<UniversalAuthMethod>,
    pub kubernetes: Option<KubernetesAuthMethod>,
    pub azure: Option<AzureAuthMethod>,
    pub gcp_id_token: Option<GCPIdTokenAuthMethod>,
    pub gcp_iam: Option<GCPIamAuthMethod>,
    pub aws_iam: Option<AWSIamAuthMethod>,
}

impl Default for AuthenticationOptions {
    fn default() -> Self {
        Self {
            access_token: None,
            universal_auth: None,
            gcp_id_token: None,
            gcp_iam: None,
            aws_iam: None,
            kubernetes: None,
            azure: None,
        }
    }
}

#[derive(Debug)]
pub enum AuthMethod {
    UniversalAuth,
    Kubernetes,
    Azure,
    GcpIdToken,
    GcpIam,
    AwsIam,
}

// Custom validation to ensure that if universal_auth or gcp_auth are present, their fields are populated
impl AuthenticationOptions {
    pub fn validate(&mut self) -> Result<AuthMethod, String> {
        // UNIVERSAL AUTH:
        if let Some(ref auth) = self.universal_auth {
            if !auth.client_id.is_empty() && !auth.client_secret.is_empty() {
                return Ok(AuthMethod::UniversalAuth);
            }

            return Err("universal_auth is present but client_id or client_secret is empty".into());
        }
        // GCP ID TOKEN AUTH:
        else if let Some(ref auth) = self.gcp_id_token {
            if !auth.identity_id.is_empty() {
                return Ok(AuthMethod::GcpIdToken);
            }
            return Err("gcp_auth is present but identity_id is empty".into());
        }
        // GCP IAM AUTH:
        else if let Some(ref auth) = self.gcp_iam {
            if !auth.service_account_key_file_path.is_empty() && !auth.identity_id.is_empty() {
                return Ok(AuthMethod::GcpIam);
            }
            return Err("gcp_auth is present but service_account_key_file_path is empty".into());
        }
        // AWS IAM AUTH:
        else if let Some(ref auth) = self.aws_iam {
            if !auth.identity_id.is_empty() {
                return Ok(AuthMethod::AwsIam);
            }
            return Err("aws_iam is present but identity_id is empty".into());
        }
        // KUBERNETES AUTH:
        else if let Some(ref auth) = self.kubernetes {
            if !auth.identity_id.is_empty() {
                return Ok(AuthMethod::Kubernetes);
            }
            return Err("kubernetes auth is present but identity_id is empty".into());

        // AZURE AUTH:
        } else if let Some(ref auth) = self.azure {
            if !auth.identity_id.is_empty() {
                return Ok(AuthMethod::Azure);
            }
            return Err("azure auth is present but identity_id is empty".into());
        } else {
            debug!("No authentication method is set. Checking environment variables.");

            // universal auth env's
            let universal_auth_client_id_env =
                std::env::var(INFISICAL_UNIVERSAL_AUTH_CLIENT_ID_ENV_NAME).unwrap_or_default();
            let universal_auth_client_secret_env =
                std::env::var(INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET_ENV_NAME).unwrap_or_default();

            // gcp auth env's
            let gcp_auth_identity_id_env =
                std::env::var(INFISICAL_GCP_AUTH_IDENTITY_ID_ENV_NAME).unwrap_or_default();
            let gcp_iam_service_account_key_file_path_env =
                std::env::var(INFISICAL_GCP_IAM_SERVICE_ACCOUNT_KEY_FILE_PATH_ENV_NAME)
                    .unwrap_or_default();

            // aws iam auth env's
            let aws_iam_identity_id_env =
                std::env::var(INFISICAL_AWS_IAM_AUTH_IDENTITY_ID_ENV_NAME).unwrap_or_default();

            // kubernetes auth env's
            let kubernetes_identity_id_env =
                std::env::var(INFISICAL_KUBERNETES_IDENTITY_ID_ENV_NAME).unwrap_or_default();
            let kubernetes_service_account_token_path_env =
                std::env::var(INFISICAL_KUBERNETES_SERVICE_ACCOUNT_TOKEN_PATH_ENV_NAME)
                    .unwrap_or_default();

            // azure auth env's
            let azure_auth_identity_id_env =
                std::env::var(INFISICAL_AZURE_AUTH_IDENTITY_ID_ENV_NAME).unwrap_or_default();

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

            // aws iam auth env check
            if !aws_iam_identity_id_env.is_empty() {
                self.aws_iam = Some(AWSIamAuthMethod {
                    identity_id: aws_iam_identity_id_env,
                });

                return Ok(AuthMethod::AwsIam);
            }

            // gcp iam auth env check
            if !gcp_iam_service_account_key_file_path_env.is_empty()
                && !gcp_auth_identity_id_env.is_empty()
            {
                self.gcp_iam = Some(GCPIamAuthMethod {
                    service_account_key_file_path: gcp_iam_service_account_key_file_path_env,
                    identity_id: gcp_auth_identity_id_env,
                });
                return Ok(AuthMethod::GcpIam);
            }

            // gcp id token auth env check
            if !gcp_auth_identity_id_env.is_empty() {
                self.gcp_id_token = Some(GCPIdTokenAuthMethod {
                    identity_id: gcp_auth_identity_id_env,
                });

                return Ok(AuthMethod::GcpIdToken);
            }

            // kubernetes auth env check
            if !kubernetes_identity_id_env.is_empty() {
                self.kubernetes = Some(KubernetesAuthMethod {
                    identity_id: kubernetes_identity_id_env,
                    service_account_token_path: Some(kubernetes_service_account_token_path_env)
                        .or(default_kubernetes_service_account_token_path()),
                });

                return Ok(AuthMethod::Kubernetes);
            }

            // azure auth env check
            if !azure_auth_identity_id_env.is_empty() {
                self.azure = Some(AzureAuthMethod {
                    identity_id: azure_auth_identity_id_env,
                });

                return Ok(AuthMethod::Azure);
            }

            return Err("No authentication method is set.".into());
        }
    }
}
