use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// Errors that may occur during authentication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationError {
    /// The requested Credential could not be found.
    CredentialNotFound,

    /// The Credential cannot currently be used.
    CredentialUnavailable,

    /// The owning Identity could not be found.
    IdentityNotFound,

    /// The Identity cannot currently be used.
    IdentityUnavailable,

    /// The Credential kind is incompatible with password authentication.
    InvalidCredentialKind,

    /// Password material associated with the Credential could not be found.
    PasswordMaterialNotFound,

    /// The Password Material repository could not complete its operation.
    PasswordMaterialRepositoryFailure,

    /// The supplied password did not match the stored password material.
    InvalidPassword,

    /// Password verification could not be completed.
    PasswordVerificationFailure,

    /// The Credential repository could not complete its operation.
    CredentialRepositoryFailure,

    /// The Identity repository could not complete its operation.
    IdentityRepositoryFailure,

    /// The Session repository could not complete its operation.
    SessionRepositoryFailure,

    /// A Session could not be created.
    SessionCreationFailure,

    /// A Token could not be created.
    TokenCreationFailure,

    /// The requested Token could not be found.
    TokenNotFound,

    /// The Token cannot currently be used.
    TokenUnavailable,

    /// The Token repository could not complete its operation.
    TokenRepositoryFailure,

    /// The requested Session could not be found.
    SessionNotFound,

    /// The Session cannot currently be used.
    SessionUnavailable,
}

impl Display for AuthenticationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::CredentialNotFound => "credential not found",

            Self::CredentialUnavailable => "credential is unavailable",

            Self::IdentityNotFound => "identity not found",

            Self::IdentityUnavailable => "identity is unavailable",

            Self::InvalidCredentialKind => {
                "credential kind is incompatible with password authentication"
            }

            Self::PasswordMaterialNotFound => "password material not found",

            Self::PasswordMaterialRepositoryFailure => {
                "password material repository operation failed"
            }

            Self::InvalidPassword => "invalid password",

            Self::PasswordVerificationFailure => "password verification failed",

            Self::CredentialRepositoryFailure => "credential repository operation failed",

            Self::IdentityRepositoryFailure => "identity repository operation failed",

            Self::SessionRepositoryFailure => "session repository operation failed",

            Self::SessionCreationFailure => "session could not be created",

            Self::TokenCreationFailure => "token could not be created",

            Self::TokenNotFound => "token not found",

            Self::TokenUnavailable => "token is unavailable",

            Self::TokenRepositoryFailure => "token repository operation failed",

            Self::SessionNotFound => "session not found",

            Self::SessionUnavailable => "session is unavailable",
        };

        formatter.write_str(message)
    }
}

impl Error for AuthenticationError {}
