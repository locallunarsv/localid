/// Mechanism represented by a Credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialKind {
    /// Password-based Credential.
    Password,

    /// Passkey-based Credential.
    Passkey,

    /// API-key-based Credential.
    ApiKey,

    /// OAuth-backed Credential.
    OAuth,
}
