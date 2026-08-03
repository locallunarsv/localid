use localid_authentication::AuthenticationError;

use crate::ApplicationError;

/// Maps authentication errors into application errors.
#[must_use]
pub fn map_authentication_error(error: AuthenticationError) -> ApplicationError {
    match error {
        AuthenticationError::InvalidPassword
        | AuthenticationError::CredentialUnavailable
        | AuthenticationError::CredentialNotFound
        | AuthenticationError::IdentityUnavailable
        | AuthenticationError::IdentityNotFound => ApplicationError::AuthenticationFailed,

        _ => ApplicationError::InternalFailure,
    }
}
