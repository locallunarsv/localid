use localid_password::PasswordVerifier;
use localid_repository::{
    CredentialRepository, IdentityRepository, PasswordMaterialRepository, SessionRepository,
};
use localid_token::TokenIssuer;

use super::{AuthenticatePasswordRequest, PasswordAuthenticationService};
use crate::{AuthenticateResult, AuthenticationError, SessionFactory};

/// Default password authentication service.
///
/// The service coordinates Credential, Identity, password material, password
/// verification, Session creation, Token issuance, and persistence through
/// injected ports.
pub struct DefaultPasswordAuthenticationService<IR, CR, PR, SR, V, SF, TI> {
    identity_repository: IR,
    credential_repository: CR,
    password_material_repository: PR,
    session_repository: SR,
    password_verifier: V,
    session_factory: SF,
    token_issuer: TI,
}

impl<IR, CR, PR, SR, V, SF, TI> DefaultPasswordAuthenticationService<IR, CR, PR, SR, V, SF, TI> {
    /// Creates a default password authentication service.
    #[must_use]
    pub const fn new(
        identity_repository: IR,
        credential_repository: CR,
        password_material_repository: PR,
        session_repository: SR,
        password_verifier: V,
        session_factory: SF,
        token_issuer: TI,
    ) -> Self {
        Self {
            identity_repository,
            credential_repository,
            password_material_repository,
            session_repository,
            password_verifier,
            session_factory,
            token_issuer,
        }
    }
}

impl<IR, CR, PR, SR, V, SF, TI> PasswordAuthenticationService
    for DefaultPasswordAuthenticationService<IR, CR, PR, SR, V, SF, TI>
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    SR: SessionRepository,
    V: PasswordVerifier,
    SF: SessionFactory,
    TI: TokenIssuer<Error = localid_token::TokenError>,
{
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

        let session = self
            .session_factory
            .create_session(identity.id())
            .map_err(|_| AuthenticationError::SessionCreationFailure)?;

        self.session_repository
            .save(session.clone())
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)?;

        let issued_token = self
            .token_issuer
            .issue(session.id(), session.expires_at())
            .map_err(|_| AuthenticationError::TokenCreationFailure)?;

        Ok(AuthenticateResult::new(session, issued_token))
    }
}
