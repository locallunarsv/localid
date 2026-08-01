/// Lifecycle state of a Credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CredentialLifecycleState {
    /// Credential is available for verification.
    Active,

    /// Credential is temporarily unavailable.
    Disabled,

    /// Credential is permanently unavailable.
    Revoked,
}
