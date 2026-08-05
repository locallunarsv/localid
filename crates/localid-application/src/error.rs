use localid_authentication::AuthenticationError;

/// Application layer errors.
#[derive(Debug)]
pub enum ApplicationError {
    /// Authentication failed.
    AuthenticationFailed,

    /// Session could not be found.
    SessionNotFound,

    /// Internal application failure.
    InternalFailure,
}

impl From<AuthenticationError> for ApplicationError {
    fn from(error: AuthenticationError) -> Self {
        match error {
            AuthenticationError::SessionNotFound => Self::SessionNotFound,

            AuthenticationError::SessionUnavailable
            | AuthenticationError::SessionRepositoryFailure
            | AuthenticationError::SessionCreationFailure => Self::InternalFailure,

            _ => Self::AuthenticationFailed,
        }
    }
}
