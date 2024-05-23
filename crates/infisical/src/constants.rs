// Universal auth:
pub const INFISICAL_UNIVERSAL_AUTH_CLIENT_ID_ENV_NAME: &str = "INFISICAL_UNIVERSAL_AUTH_CLIENT_ID";
pub const INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET_ENV_NAME: &str =
    "INFISICAL_UNIVERSAL_AUTH_CLIENT_SECRET";

// GCP Auth:
pub const INFISICAL_GCP_AUTH_IDENTITY_ID_ENV_NAME: &str = "INFISICAL_GCP_AUTH_IDENTITY_ID";
pub const INFISICAL_GCP_IAM_SERVICE_ACCOUNT_KEY_FILE_PATH_ENV_NAME: &str =
    "INFISICAL_GCP_IAM_SERVICE_ACCOUNT_KEY_FILE_PATH";

// AWS IAM Auth:
pub const INFISICAL_AWS_IAM_IDENTITY_ID_ENV_NAME: &str = "INFISICAL_AWS_IAM_AUTH_IDENTITY_ID";

// Kubernetes Auth:
pub const INFISICAL_KUBERNETES_SERVICE_ACCOUNT_TOKEN_PATH_ENV_NAME: &str =
    //  /var/run/secrets/kubernetes.io/serviceaccount/token
    "INFISICAL_KUBERNETES_SERVICE_ACCOUNT_TOKEN_PATH";

pub const INFISICAL_KUBERNETES_IDENTITY_ID_ENV_NAME: &str = "INFISICAL_KUBERNETES_IDENTITY_ID";

// AWS EC2 Metadata Service:
pub const AWS_EC2_METADATA_TOKEN_URL: &str = "http://169.254.169.254/latest/api/token";
pub const AWS_EC2_INSTANCE_IDENTITY_DOCUMENT_URL: &str =
    "http://169.254.169.254/latest/dynamic/instance-identity/document";
