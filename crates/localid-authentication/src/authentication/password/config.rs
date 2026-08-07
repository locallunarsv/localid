/// Dependencies required by password authentication service.
///
/// This structure contains all external dependencies needed by
/// [`DefaultPasswordAuthenticationService`], including repository ports,
/// password verification strategy, session creation strategy, and token
/// issuance strategies.
pub struct PasswordAuthenticationDependencies<IR, CR, PR, V, TIS> {
    /// Identity repository.
    pub identity_repository: IR,

    /// Credential repository.
    pub credential_repository: CR,

    /// Password material repository.
    pub password_material_repository: PR,

    /// Password verifier.
    pub password_verifier: V,

    /// Token issuance service.
    pub token_issuance_service: TIS,
}
