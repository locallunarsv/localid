use chrono::Duration;
use localid_password::PasswordVerifier;
use localid_refresh_token_random::RefreshTokenIssuer;
use localid_repository::{
    CredentialRepository, IdentityRepository, PasswordMaterialRepository, RefreshTokenRepository,
    SessionRepository, TokenRepository,
};
use localid_token::TokenIssuer;

use super::{
    AuthenticatePasswordRequest, PasswordAuthenticationDependencies, PasswordAuthenticationService,
};
use crate::{AuthenticateResult, AuthenticationError, SessionFactory};

/// Default password authentication service.
///
/// The service coordinates Credential, Identity, password material,
/// password verification, Session creation, access token issuance,
/// refresh token issuance, and persistence through injected ports.
pub struct DefaultPasswordAuthenticationService<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI> {
    identity_repository: IR,
    credential_repository: CR,
    password_material_repository: PR,
    session_repository: SR,
    token_repository: TR,
    refresh_token_repository: RTR,
    password_verifier: V,
    session_factory: SF,
    token_issuer: TI,
    refresh_token_issuer: RTI,
}

impl<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI>
    DefaultPasswordAuthenticationService<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI>
{
    /// Creates a default password authentication service.
    ///
    /// The service receives repository contracts and token issuance
    /// strategies through dependency injection.
    #[must_use]
    pub fn new(
        dependencies: PasswordAuthenticationDependencies<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI>,
    ) -> Self {
        Self {
            identity_repository: dependencies.identity_repository,
            credential_repository: dependencies.credential_repository,
            password_material_repository: dependencies.password_material_repository,
            session_repository: dependencies.session_repository,
            token_repository: dependencies.token_repository,
            refresh_token_repository: dependencies.refresh_token_repository,
            password_verifier: dependencies.password_verifier,
            session_factory: dependencies.session_factory,
            token_issuer: dependencies.token_issuer,
            refresh_token_issuer: dependencies.refresh_token_issuer,
        }
    }
}

impl<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI> PasswordAuthenticationService
    for DefaultPasswordAuthenticationService<IR, CR, PR, SR, TR, RTR, V, SF, TI, RTI>
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    PR: PasswordMaterialRepository,
    SR: SessionRepository,
    TR: TokenRepository,
    RTR: RefreshTokenRepository,
    V: PasswordVerifier,
    SF: SessionFactory,
    RTI: RefreshTokenIssuer<Error = localid_refresh_token::RefreshTokenError>,
    TI: TokenIssuer<Error = localid_token::TokenError>,
{
    /// Authenticates a password credential.
    ///
    /// Authentication flow:
    ///
    /// 1. Validate Credential.
    /// 2. Validate Identity.
    /// 3. Verify password.
    /// 4. Create Session.
    /// 5. Issue access token.
    /// 6. Issue refresh token.
    /// 7. Persist authentication artifacts.
    /// 8. Return authentication result.
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

        self.token_repository
            .save(issued_token.token().clone())
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        let issued_refresh_token = self
            .refresh_token_issuer
            .issue(session.id(), session.created_at() + Duration::days(30))
            .map_err(|_| AuthenticationError::TokenCreationFailure)?;

        self.refresh_token_repository
            .save(issued_refresh_token.token().clone())
            .map_err(|_| AuthenticationError::TokenRepositoryFailure)?;

        Ok(AuthenticateResult::new(
            session,
            issued_token,
            issued_refresh_token,
        ))
    }
}
