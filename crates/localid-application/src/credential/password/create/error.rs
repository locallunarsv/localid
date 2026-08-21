/// Errors returned when creating a password credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatePasswordCredentialError {
    /// Owning Identity was not found.
    IdentityNotFound,

    /// Owning Identity is not active.
    IdentityNotActive,

    /// Identity already has an active or disabled password credential.
    PasswordCredentialAlreadyExists,

    /// Password hashing failed.
    PasswordHashingFailure,

    /// Identity repository operation failed.
    IdentityRepositoryFailure,

    /// Credential repository operation failed.
    CredentialRepositoryFailure,

    /// Password material repository operation failed.
    PasswordMaterialRepositoryFailure,
}
