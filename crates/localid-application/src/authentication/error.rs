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
        | AuthenticationError::IdentityNotFound
        | AuthenticationError::TokenNotFound
        | AuthenticationError::TokenUnavailable
        | AuthenticationError::SessionNotFound
        | AuthenticationError::SessionUnavailable => ApplicationError::AuthenticationFailed,

        AuthenticationError::InvalidCredentialKind
        | AuthenticationError::PasswordMaterialNotFound
        | AuthenticationError::PasswordMaterialRepositoryFailure
        | AuthenticationError::PasswordVerificationFailure
        | AuthenticationError::CredentialRepositoryFailure
        | AuthenticationError::IdentityRepositoryFailure
        | AuthenticationError::SessionRepositoryFailure
        | AuthenticationError::SessionCreationFailure
        | AuthenticationError::TokenCreationFailure
        | AuthenticationError::TokenRepositoryFailure => ApplicationError::InternalFailure,
    }
}
