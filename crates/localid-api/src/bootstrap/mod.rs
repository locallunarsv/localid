mod seed;

pub use seed::seed_demo_identity;

use localid_credential::CredentialId;

use localid_application::{authentication::PasswordAuthenticationAdapter, LoginUseCase};

use localid_authentication::{
    DefaultPasswordAuthenticationService, DefaultSessionFactory, PasswordAuthenticationDependencies,
};

use localid_password_argon2::Argon2PasswordHasher;

use localid_refresh_token_random::RandomRefreshTokenIssuer;

use localid_repository_memory::{
    MemoryCredentialRepository, MemoryIdentityRepository, MemoryPasswordMaterialRepository,
    MemoryRefreshTokenRepository, MemorySessionRepository, MemoryTokenRepository,
};

use localid_token_random::RandomTokenIssuer;

use crate::AppState;

/// Result of application bootstrap initialization.
pub struct BootstrapContext<A> {
    /// Ready-to-use application state.
    pub state: AppState<A>,

    /// Credential identifier created during development seed.
    pub credential_id: CredentialId,
}

type BootstrapAuthenticationService = PasswordAuthenticationAdapter<
    DefaultPasswordAuthenticationService<
        MemoryIdentityRepository,
        MemoryCredentialRepository,
        MemoryPasswordMaterialRepository,
        MemorySessionRepository,
        MemoryTokenRepository,
        MemoryRefreshTokenRepository,
        Argon2PasswordHasher,
        DefaultSessionFactory,
        RandomTokenIssuer,
        RandomRefreshTokenIssuer,
    >,
>;

/// Creates application state with in-memory authentication dependencies.
pub fn create_state() -> BootstrapContext<BootstrapAuthenticationService> {
    let mut identity_repository = MemoryIdentityRepository::new();
    let mut credential_repository = MemoryCredentialRepository::new();
    let mut password_material_repository = MemoryPasswordMaterialRepository::new();

    let credential_id = seed_demo_identity(
        &mut identity_repository,
        &mut credential_repository,
        &mut password_material_repository,
    );

    let authentication_service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository,
            credential_repository,
            password_material_repository,
            session_repository: MemorySessionRepository::new(),
            token_repository: MemoryTokenRepository::new(),
            refresh_token_repository: MemoryRefreshTokenRepository::new(),
            password_verifier: Argon2PasswordHasher::new(),
            session_factory: DefaultSessionFactory::new(),
            token_issuer: RandomTokenIssuer::new(),
            refresh_token_issuer: RandomRefreshTokenIssuer::new(),
        });

    let adapter = PasswordAuthenticationAdapter::new(authentication_service);

    let use_case = LoginUseCase::new(adapter);

    BootstrapContext {
        state: AppState::new(use_case),
        credential_id,
    }
}
