mod repository;
mod seed;

pub use seed::seed_demo_identity;

use localid_credential::CredentialId;

use crate::bootstrap::repository::SharedRepository;
use crate::AppState;

use localid_application::{
    authentication::PasswordAuthenticationAdapter, LoginUseCase, RefreshTokenAdapter,
    RefreshTokenUseCase,
};

use localid_authentication::{
    DefaultPasswordAuthenticationService, DefaultRefreshTokenService, DefaultSessionFactory,
    PasswordAuthenticationDependencies,
};

use localid_password_argon2::Argon2PasswordHasher;

use localid_refresh_token_random::RandomRefreshTokenIssuer;

use localid_repository_memory::{
    MemoryCredentialRepository, MemoryIdentityRepository, MemoryPasswordMaterialRepository,
    MemoryRefreshTokenRepository, MemorySessionRepository, MemoryTokenRepository,
};

use localid_token_random::RandomTokenIssuer;

/// Result of application bootstrap initialization.
pub struct BootstrapContext<L, R> {
    /// Ready-to-use application state.
    pub state: AppState<L, R>,

    /// Credential identifier created during development seed.
    pub credential_id: CredentialId,
}

type SharedSessionRepository = SharedRepository<MemorySessionRepository>;
type SharedTokenRepository = SharedRepository<MemoryTokenRepository>;
type SharedRefreshTokenRepository = SharedRepository<MemoryRefreshTokenRepository>;

type BootstrapAuthenticationService = PasswordAuthenticationAdapter<
    DefaultPasswordAuthenticationService<
        MemoryIdentityRepository,
        MemoryCredentialRepository,
        MemoryPasswordMaterialRepository,
        SharedSessionRepository,
        SharedTokenRepository,
        SharedRefreshTokenRepository,
        Argon2PasswordHasher,
        DefaultSessionFactory,
        RandomTokenIssuer,
        RandomRefreshTokenIssuer,
    >,
>;

type BootstrapRefreshService = RefreshTokenAdapter<
    DefaultRefreshTokenService<
        SharedRefreshTokenRepository,
        SharedTokenRepository,
        SharedSessionRepository,
        RandomRefreshTokenIssuer,
        RandomTokenIssuer,
    >,
>;

/// Creates application state with in-memory authentication dependencies.
pub fn create_state() -> BootstrapContext<BootstrapAuthenticationService, BootstrapRefreshService> {
    let mut identity_repository = MemoryIdentityRepository::new();
    let mut credential_repository = MemoryCredentialRepository::new();
    let mut password_material_repository = MemoryPasswordMaterialRepository::new();

    let credential_id = seed_demo_identity(
        &mut identity_repository,
        &mut credential_repository,
        &mut password_material_repository,
    );

    let session_repository = SharedRepository::new(MemorySessionRepository::new());

    let token_repository = SharedRepository::new(MemoryTokenRepository::new());

    let refresh_token_repository = SharedRepository::new(MemoryRefreshTokenRepository::new());

    let authentication_service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository,
            credential_repository,
            password_material_repository,
            session_repository: session_repository.clone(),
            token_repository: token_repository.clone(),
            refresh_token_repository: refresh_token_repository.clone(),
            password_verifier: Argon2PasswordHasher::new(),
            session_factory: DefaultSessionFactory::new(),
            token_issuer: RandomTokenIssuer::new(),
            refresh_token_issuer: RandomRefreshTokenIssuer::new(),
        });

    let refresh_service = DefaultRefreshTokenService::new(
        refresh_token_repository,
        token_repository,
        session_repository,
        RandomRefreshTokenIssuer::new(),
        RandomTokenIssuer::new(),
    );

    let refresh_adapter = RefreshTokenAdapter::new(refresh_service);

    let refresh_use_case = RefreshTokenUseCase::new(refresh_adapter);

    let adapter = PasswordAuthenticationAdapter::new(authentication_service);

    let login_use_case = LoginUseCase::new(adapter);

    BootstrapContext {
        state: AppState::new(login_use_case, refresh_use_case),
        credential_id,
    }
}
