/// Evidence presented to prove possession of a Credential.
///
/// This type intentionally does not expose password, passkey, or API-key
/// representations. Credential-specific evidence models will be introduced
/// after their verification requirements are defined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticationEvidence;
