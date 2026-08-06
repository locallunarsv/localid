use std::sync::Arc;

use tokio::sync::Mutex;

mod repository;
mod seed;

pub use seed::{seed_demo_client, seed_demo_identity};

use localid_application::{ClientRepositoryAdapter, GetClientUseCase};
use localid_client::ClientId;
use localid_credential::CredentialId;

use crate::{
    middleware::{AuthMiddlewareState, AuthorizationMiddlewareState},
    AppState,
};

use localid_application::{
    authentication::{PasswordAuthenticationAdapter, TokenVerificationAdapter},
    AuthorizationContextResolver, GetCurrentSessionUseCase, IdentityRoleAdapter, LoginUseCase,
    LogoutSessionUseCase, RefreshTokenAdapter, RefreshTokenUseCase, SessionAdapter,
    VerifyTokenUseCase,
};

use localid_authentication::{
    DefaultPasswordAuthenticationService, DefaultRefreshTokenService, DefaultSessionFactory,
    DefaultSessionService, DefaultTokenVerificationService, PasswordAuthenticationDependencies,
};

use localid_password_argon2::Argon2PasswordHasher;
use localid_refresh_token_random::RandomRefreshTokenIssuer;

use localid_repository_memory::{
    MemoryClientRepository, MemoryCredentialRepository, MemoryIdentityRepository,
    MemoryIdentityRoleRepository, MemoryPasswordMaterialRepository, MemoryRefreshTokenRepository,
    MemorySessionRepository, MemoryTokenRepository,
};

use localid_token_random::RandomTokenIssuer;

use crate::bootstrap::repository::SharedRepository;

/// Result of application bootstrap initialization.
pub struct BootstrapContext<L, R, V, S, A, C> {
    /// Ready-to-use application state.
    pub state: AppState<L, R, V, S, C>,

    /// Authentication middleware state.
    pub auth_state: AuthMiddlewareState<V>,

    /// Authorization middleware state.
    pub authorization_state: AuthorizationMiddlewareState<A>,

    /// Credential identifier created during development seed.
    pub credential_id: CredentialId,

    /// Client identifier created during development seed.
    pub client_id: ClientId,
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

type BootstrapVerificationService = TokenVerificationAdapter<
    DefaultTokenVerificationService<SharedTokenRepository, SharedSessionRepository>,
>;

type BootstrapSessionService = SessionAdapter<DefaultSessionService<SharedSessionRepository>>;

type BootstrapAuthorizationResolver =
    AuthorizationContextResolver<IdentityRoleAdapter<MemoryIdentityRoleRepository>>;

/// Creates application state with in-memory authentication dependencies.
pub fn create_state() -> BootstrapContext<
    BootstrapAuthenticationService,
    BootstrapRefreshService,
    BootstrapVerificationService,
    BootstrapSessionService,
    BootstrapAuthorizationResolver,
    ClientRepositoryAdapter<MemoryClientRepository>,
> {
    let mut identity_repository = MemoryIdentityRepository::new();
    let mut credential_repository = MemoryCredentialRepository::new();
    let mut password_material_repository = MemoryPasswordMaterialRepository::new();
    let mut identity_role_repository = MemoryIdentityRoleRepository::new();

    let credential_id = seed_demo_identity(
        &mut identity_repository,
        &mut credential_repository,
        &mut password_material_repository,
        &mut identity_role_repository,
    );

    let mut client_repository = MemoryClientRepository::new();

    let client_id = seed_demo_client(&mut client_repository);

    let client_adapter = ClientRepositoryAdapter::new(client_repository);

    let client_use_case = GetClientUseCase::new(client_adapter);

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
        token_repository.clone(),
        session_repository.clone(),
        RandomRefreshTokenIssuer::new(),
        RandomTokenIssuer::new(),
    );

    let refresh_adapter = RefreshTokenAdapter::new(refresh_service);

    let refresh_use_case = RefreshTokenUseCase::new(refresh_adapter);

    let verification_service =
        DefaultTokenVerificationService::new(token_repository.clone(), session_repository.clone());

    let verification_adapter = TokenVerificationAdapter::new(verification_service);

    let verify_token_use_case = Arc::new(Mutex::new(VerifyTokenUseCase::new(verification_adapter)));

    let auth_state = AuthMiddlewareState::new(verify_token_use_case.clone());

    let current_session_service = DefaultSessionService::new(session_repository.clone());

    let current_session_adapter = SessionAdapter::new(current_session_service);

    let current_session_use_case = GetCurrentSessionUseCase::new(current_session_adapter);

    let logout_session_service = DefaultSessionService::new(session_repository);

    let logout_session_adapter = SessionAdapter::new(logout_session_service);

    let logout_session_use_case = LogoutSessionUseCase::new(logout_session_adapter);

    let identity_role_adapter = IdentityRoleAdapter::new(identity_role_repository);

    let authorization_resolver = AuthorizationContextResolver::new(identity_role_adapter);

    let authorization_state = AuthorizationMiddlewareState::new(authorization_resolver);

    let adapter = PasswordAuthenticationAdapter::new(authentication_service);

    let login_use_case = LoginUseCase::new(adapter);

    BootstrapContext {
        state: AppState::new(
            login_use_case,
            refresh_use_case,
            verify_token_use_case,
            current_session_use_case,
            logout_session_use_case,
            client_use_case,
        ),
        auth_state,
        authorization_state,
        credential_id,
        client_id,
    }
}
