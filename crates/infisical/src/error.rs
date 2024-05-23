//! Errors that can occur when using this SDK

use crate::api::{BadRequestError, Error as BaseError, UnauthorizedError};

use reqwest::{Response, StatusCode};
use std::fmt::Debug;
use thiserror::Error;

// These are hardcoded errors. When the response include these messages, we should create a custom error to give the user proper guidance on how to fix it.
const E2EE_ENABLED_MESSAGE: &str =
    "Failed workspace authorization due to end-to-end encryption not being disabled";

const BLIND_INDICES_DISABLED_MESSAGE: &str =
    "Failed workspace authorization due to blind indices not being enabled";

#[derive(Debug, Error)]
pub enum Error {
    #[error("Something unexpected went wrong.")]
    UnknownError,

    #[error("Something went wrong: {}", .message)]
    UnknownErrorWithMessage { message: String },

    #[error("Failed to get AWS credentials: {}", .message)]
    AwsCredentialsError { message: String },

    #[error("Failed to build AWS request signer: {}", .message)]
    AwsBuildRequestSignerError { message: String },

    #[error("Failed to sign AWS request: {}", .message)]
    AwsSignRequestError { message: String },

    #[error("Failed to get AWS region: {}", .message)]
    AwsGetRegionError { message: String },

    #[error("Failed to create symmetric key: {}", .message)]
    CreateSymmetricKeyError { message: String },

    #[error("Failed to authenticate due to missing parameters: {}", .message)]
    MissingParametersAuthError { message: String },

    #[error("Failed to obtain metadata from Google Cloud")]
    GoogleMetadataError,

    #[error("Failed to sign JWT from Google Cloud: {}", .message)]
    GoogleJwtError { message: String },

    #[error("Failed to get token from Google Cloud Platform: {}", .message)]
    GoogleTokenError { message: String },

    #[error("Authentication parsing failed: {}", .message)]
    AuthSanitizationError { message: String },

    #[error("No access token was obtained after authentication.")]
    NoAccessTokenObtained,

    #[error("Failed to encrypt symmetric key: {}", .message)]
    EncryptSymmetricKeyError { message: String },

    #[error("Failed to decrypt symmetric key: {}", .message)]
    DecryptSymmetricKeyError { message: String },

    #[error("Missing access token.")]
    MissingAccessToken,

    // Secret not found
    #[error("Secret with name '{}' not found.", .secret_name)]
    SecretNotFound { secret_name: String },

    // Secret bad request
    #[error("[Bad request]: {}", .message)]
    SecretBadRequest { message: String },

    // Access token 404 error
    #[error("Failed to authenticate, did you provide the correct site URL?")]
    NotFoundAccessTokenRequest,

    // Access token 401 error
    #[error("[Failed to authenticate]: Did you provide the correct client ID and secret?")]
    UnauthorizedAccessTokenRequest,

    // Blind indicies error
    #[error("Blind indicies are not enabled for this project. Read more here: https://infisical.com/docs/")]
    BlindIndicesDisabled,

    // E2EE enabled error
    #[error("End-to-end encryption is enabled for this project. Please disable it to use this SDK. Read more here: https://infisical.com/docs/api-reference/overview/examples/note")]
    E2EEEnabled,

    // Regular unauthorized error
    #[error("Failed to authenticate: {}", .message)]
    Unauthorized { message: String },

    // Generic "base" error. This is the last resort error really.
    #[error("Received error message from server: (status {}), {}", .status, .message)]
    ResponseContent { status: StatusCode, message: String },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub async fn api_error_handler(
    status: StatusCode,
    res: Response,
    secret_name: Option<String>,
    is_auth_request: bool,
) -> Result<Error> {
    if status == StatusCode::NOT_FOUND {
        if is_auth_request {
            return Err(Error::NotFoundAccessTokenRequest);
        }

        let s = match secret_name {
            Some(secret_name) => secret_name,
            None => "".to_string(),
        };

        return Err(Error::SecretNotFound { secret_name: s });
    }

    if status == StatusCode::BAD_REQUEST {
        let r = res.json::<BadRequestError>().await?;

        if r.message.contains(E2EE_ENABLED_MESSAGE) {
            return Ok(Error::E2EEEnabled);
        }

        return Ok(Error::SecretBadRequest { message: r.message });
    }

    if status == StatusCode::UNAUTHORIZED {
        let r = res.json::<UnauthorizedError>().await?;

        if r.message.contains(BLIND_INDICES_DISABLED_MESSAGE) {
            return Ok(Error::BlindIndicesDisabled);
        }

        return Ok(Error::Unauthorized { message: r.message });
    }

    let json_string = res.json::<serde_json::Value>().await;

    let err_message = match json_string {
        Ok(json) => json.to_string(),
        Err(_) => "Failed to decode error message".to_string(),
    };

    return Ok(Error::ResponseContent {
        status,
        message: err_message,
    });
}

macro_rules! impl_infisical_error {
    ($name:ident) => {
        impl<T> From<$name<T>> for Error {
            fn from(e: $name<T>) -> Self {
                match e {
                    $name::Reqwest(e) => Self::Reqwest(e),
                    $name::ResponseError(e) => Self::ResponseContent {
                        status: e.status,
                        message: e.content,
                    },
                    $name::Serde(e) => Self::Serde(e),
                    $name::Io(e) => Self::Io(e),
                }
            }
        }
    };
}

impl_infisical_error!(BaseError);

pub type Result<T, E = Error> = std::result::Result<T, E>;
