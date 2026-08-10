use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

use localid_error::OAuthError;

use localid_application::ApplicationError;

/// API error response body.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Machine-readable error code.
    pub code: &'static str,

    /// Human-readable error message.
    pub message: &'static str,
}

/// API layer error.
#[derive(Debug)]
pub enum ApiError {
    /// OAuth authorization grant is invalid.
    InvalidGrant,
    /// Invalid client request.
    InvalidRequest,

    /// Authentication failed.
    AuthenticationFailed,

    /// Unexpected internal failure.
    InternalFailure,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::InvalidGrant => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: "invalid_grant",
                    message: "invalid grant",
                },
            ),
            Self::InvalidRequest => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: "invalid_request",
                    message: "invalid request",
                },
            ),

            Self::AuthenticationFailed => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: "authentication_failed",
                    message: "authentication failed",
                },
            ),

            Self::InternalFailure => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: "internal_failure",
                    message: "internal server error",
                },
            ),
        };

        (status, Json(body)).into_response()
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::AuthenticationFailed => Self::AuthenticationFailed,

            ApplicationError::SessionNotFound => Self::AuthenticationFailed,

            ApplicationError::InternalFailure => Self::InternalFailure,
        }
    }
}

impl From<OAuthError> for ApiError {
    fn from(error: OAuthError) -> Self {
        match error {
            OAuthError::InvalidGrant => Self::InvalidGrant,
            OAuthError::InvalidRequest => Self::InvalidRequest,

            OAuthError::InvalidClient | OAuthError::InvalidScope => Self::InvalidRequest,

            OAuthError::ServerError => Self::InternalFailure,
        }
    }
}
