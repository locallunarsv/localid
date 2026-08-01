use localid_credential::Credential;

use super::AuthenticationEvidence;

/// Verifies authentication evidence against a Credential.
///
/// A verifier is responsible only for evaluating whether the supplied evidence
/// is valid for a particular Credential. It does not load aggregates, create
/// Sessions, or modify Identity and Credential lifecycle state.
pub trait CredentialVerifier {
    /// Error produced by the concrete verification mechanism.
    type Error;

    /// Verifies whether the presented evidence matches the Credential.
    ///
    /// Returns `true` when the evidence is valid and `false` when the evidence
    /// does not prove possession of the Credential.
    ///
    /// # Errors
    ///
    /// Returns the concrete verifier error when verification cannot be
    /// completed.
    fn verify(
        &self,
        credential: &Credential,
        evidence: &AuthenticationEvidence,
    ) -> Result<bool, Self::Error>;
}
