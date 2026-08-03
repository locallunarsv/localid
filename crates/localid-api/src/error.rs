use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

use localid_application::ApplicationError;

/// API error response body.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: &'static str,
    message: &'static str,
}

/// API layer error.
#[derive(Debug)]
pub enum ApiError {
    /// Authentication failed.
    AuthenticationFailed,

    /// Unexpected internal failure.
    InternalFailure,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
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

            ApplicationError::InternalFailure => Self::InternalFailure,
        }
    }
}
