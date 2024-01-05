//! Errors that can occur when using this SDK

use crate::api::{BadRequestError, BaseApiError, Error as BaseError, UnauthorizedError};

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

    #[error("Failed to create symmetric key: {}", .message)]
    CreateSymmetricKeyError { message: String },

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
    #[error("Blind indicies are not enabled for this project. Read more here: https://infisical.com/docs/api-reference/overview/blind-indices")]
    BlindIndicesDisabled,

    // E2EE enabled error
    #[error("End-to-end encryption is enabled for this project. Please disable it to use this SDK. Read more here: https://github.com/infisical/sdk#end-to-end-encryption")]
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

    // We need to try and parse the text to a BaseApiError
    let r = res.json::<BaseApiError>().await?;
    return Ok(Error::ResponseContent {
        status: StatusCode::from_u16(r.status_code as u16).unwrap(),
        message: r.message,
    });

    // We gotta do this or the function doesn't have an OK return, which doesn't work with the result flow.
    #[allow(unreachable_code)]
    Err(Error::UnknownError)
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
