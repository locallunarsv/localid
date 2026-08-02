/// Dependencies required by password authentication service.
///
/// This structure contains all external dependencies needed by
/// [`DefaultPasswordAuthenticationService`], including repository ports,
/// password verification strategy, session creation strategy, and token
/// issuance strategies.
pub struct PasswordAuthenticationDependencies<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI> {
    /// Repository used to retrieve and persist identities.
    pub identity_repository: IR,

    /// Repository used to retrieve and persist credentials.
    pub credential_repository: CR,

    /// Repository used to retrieve stored password material.
    pub password_material_repository: PR,

    /// Repository used to persist authentication sessions.
    pub session_repository: SR,

    /// Repository used to persist access tokens.
    pub token_repository: TR,

    /// Repository used to persist refresh tokens.
    pub refresh_token_repository: RTR,

    /// Component responsible for verifying password credentials.
    pub password_verifier: V,

    /// Factory responsible for creating authentication sessions.
    pub session_factory: SF,

    /// Component responsible for issuing access tokens.
    pub token_issuer: TI,

    /// Component responsible for issuing refresh tokens.
    pub refresh_token_issuer: RTI,
}
