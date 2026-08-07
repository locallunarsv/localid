use localid_password::PasswordVerifier;
use localid_repository::{CredentialRepository, IdentityRepository, PasswordMaterialRepository};

use crate::TokenIssuanceService;

use super::{
    AuthenticatePasswordRequest, PasswordAuthenticationDependencies, PasswordAuthenticationService,
};

use crate::{AuthenticateResult, AuthenticationError};

/// Default password authentication service.
///
/// The service coordinates Credential, Identity, password material,
/// password verification, and delegates session/token creation
/// to the token issuance service.
pub struct DefaultPasswordAuthenticationService<IR, CR, PR, V, TIS> {
    identity_repository: IR,
    credential_repository: CR,
    password_material_repository: PR,
    password_verifier: V,
    token_issuance_service: TIS,
}

impl<IR, CR, PR, V, TIS> DefaultPasswordAuthenticationService<IR, CR, PR, V, TIS> {
    /// Creates a default password authentication service.
    #[must_use]
    pub fn new(dependencies: PasswordAuthenticationDependencies<IR, CR, PR, V, TIS>) -> Self {
        Self {
            identity_repository: dependencies.identity_repository,
            credential_repository: dependencies.credential_repository,
            password_material_repository: dependencies.password_material_repository,
            password_verifier: dependencies.password_verifier,
            token_issuance_service: dependencies.token_issuance_service,
        }
    }
}

impl<IR, CR, PR, V, TIS> PasswordAuthenticationService
    for DefaultPasswordAuthenticationService<IR, CR, PR, V, TIS>
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    V: PasswordVerifier,
    TIS: TokenIssuanceService<Error = AuthenticationError>,
{
    /// Authenticates a password credential.
    ///
    /// Authentication flow:
    ///
    /// 1. Validate Credential.
    /// 2. Validate Identity.
    /// 3. Verify password.
    /// 4. Delegate session and token issuance.
    /// 5. Return authentication result.
    fn authenticate_password(
        &mut self,
        request: AuthenticatePasswordRequest,
    ) -> Result<AuthenticateResult, AuthenticationError> {
        let credential = self
            .credential_repository
            .find_by_id(request.credential_id())
            .map_err(|_| AuthenticationError::CredentialRepositoryFailure)?
            .ok_or(AuthenticationError::CredentialNotFound)?;

        if !credential.kind().is_password() {
            return Err(AuthenticationError::InvalidCredentialKind);
        }

        if !credential.is_active() {
            return Err(AuthenticationError::CredentialUnavailable);
        }

        let identity = self
            .identity_repository
            .find_by_id(credential.identity_id())
            .map_err(|_| AuthenticationError::IdentityRepositoryFailure)?
            .ok_or(AuthenticationError::IdentityNotFound)?;

        if !identity.is_active() {
            return Err(AuthenticationError::IdentityUnavailable);
        }

        let password_material = self
            .password_material_repository
            .find_by_credential_id(credential.id())
            .map_err(|_| AuthenticationError::PasswordMaterialRepositoryFailure)?
            .ok_or(AuthenticationError::PasswordMaterialNotFound)?;

        let password_is_valid = self
            .password_verifier
            .verify(&password_material, request.password())
            .map_err(|_| AuthenticationError::PasswordVerificationFailure)?;

        if !password_is_valid {
            return Err(AuthenticationError::InvalidPassword);
        }

        self.token_issuance_service
            .issue(identity.id(), request.client_id())
    }
}
