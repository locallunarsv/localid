use localid_repository::{CredentialRepository, IdentityRepository, SessionRepository};

use super::{
    AuthenticateRequest, AuthenticateResult, AuthenticationError, AuthenticationService,
    CredentialVerifier, SessionFactory,
};

/// Default authentication service using repository and verification ports.
pub struct DefaultAuthenticationService<IR, CR, SR, V, SF> {
    identity_repository: IR,
    credential_repository: CR,
    session_repository: SR,
    verifier: V,
    session_factory: SF,
}

impl<IR, CR, SR, V, SF> DefaultAuthenticationService<IR, CR, SR, V, SF> {
    /// Creates a default authentication service.
    #[must_use]
    pub const fn new(
        identity_repository: IR,
        credential_repository: CR,
        session_repository: SR,
        verifier: V,
        session_factory: SF,
    ) -> Self {
        Self {
            identity_repository,
            credential_repository,
            session_repository,
            verifier,
            session_factory,
        }
    }

    /// Returns a reference to the Identity repository.
    #[must_use]
    pub const fn identity_repository(&self) -> &IR {
        &self.identity_repository
    }

    /// Returns a reference to the Credential repository.
    #[must_use]
    pub const fn credential_repository(&self) -> &CR {
        &self.credential_repository
    }

    /// Returns a reference to the Session repository.
    #[must_use]
    pub const fn session_repository(&self) -> &SR {
        &self.session_repository
    }
}

impl<IR, CR, SR, V, SF> AuthenticationService for DefaultAuthenticationService<IR, CR, SR, V, SF>
where
    IR: IdentityRepository,
    CR: CredentialRepository,
    SR: SessionRepository,
    V: CredentialVerifier,
    SF: SessionFactory,
{
    fn authenticate(
        &mut self,
        request: AuthenticateRequest,
    ) -> Result<AuthenticateResult, AuthenticationError> {
        let credential = self
            .credential_repository
            .find_by_id(request.credential_id())
            .map_err(|_| AuthenticationError::CredentialRepositoryFailure)?
            .ok_or(AuthenticationError::CredentialNotFound)?;

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

        let evidence_is_valid = self
            .verifier
            .verify(&credential, request.evidence())
            .map_err(|_| AuthenticationError::VerificationFailure)?;

        if !evidence_is_valid {
            return Err(AuthenticationError::InvalidEvidence);
        }

        let session = self
            .session_factory
            .create_session(identity.id())
            .map_err(|_| AuthenticationError::SessionCreationFailure)?;

        self.session_repository
            .save(session.clone())
            .map_err(|_| AuthenticationError::SessionRepositoryFailure)?;

        Ok(AuthenticateResult::new(session))
    }
}
