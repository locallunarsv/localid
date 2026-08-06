use std::sync::Arc;

use tokio::sync::Mutex;

mod repository;
mod seed;

pub use seed::{seed_demo_client, seed_demo_identity};

use localid_application::{
    authentication::{PasswordAuthenticationAdapter, TokenVerificationAdapter},
    AuthorizationContextResolver, AuthorizationRepositoryAdapter, AuthorizeUseCase,
    ClientRepositoryAdapter, GetClientUseCase, GetCurrentSessionUseCase, IdentityRoleAdapter,
    LoginUseCase, LogoutSessionUseCase, RefreshTokenAdapter, RefreshTokenUseCase, SessionAdapter,
    VerifyTokenUseCase,
};

use localid_client::ClientId;
use localid_credential::CredentialId;

use crate::{
    middleware::{AuthMiddlewareState, AuthorizationMiddlewareState},
    AppState,
};

use localid_authentication::{
    DefaultPasswordAuthenticationService, DefaultRefreshTokenService, DefaultSessionFactory,
    DefaultSessionService, DefaultTokenVerificationService, PasswordAuthenticationDependencies,
};

use localid_oauth_authorization_repository_memory::MemoryAuthorizationCodeRepository;
use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};
use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

use localid_password_argon2::Argon2PasswordHasher;
use localid_refresh_token_random::RandomRefreshTokenIssuer;

use localid_repository_memory::{
    MemoryClientRepository, MemoryCredentialRepository, MemoryIdentityRepository,
    MemoryIdentityRoleRepository, MemoryPasswordMaterialRepository, MemoryRefreshTokenRepository,
    MemorySessionRepository, MemoryTokenRepository,
};

use localid_token_random::RandomTokenIssuer;

use crate::bootstrap::repository::SharedRepository;

/// Context containing initialized application dependencies.
pub struct BootstrapContext<L, R, V, S, A, C, O> {
    /// Shared application state.
    pub state: AppState<L, R, V, S, C, O>,

    /// Authentication middleware state.
    pub auth_state: AuthMiddlewareState<V>,

    /// Authorization middleware state.
    pub authorization_state: AuthorizationMiddlewareState<A>,

    /// Seeded credential identifier.
    pub credential_id: CredentialId,

    /// Seeded client identifier.
    pub client_id: ClientId,

    /// Seeded OAuth client identifier.
    pub oauth_client_id: OAuthClientId,
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

type BootstrapAuthorizationAdapter =
    AuthorizationRepositoryAdapter<MemoryOAuthClientRepository, MemoryAuthorizationCodeRepository>;

fn seed_demo_oauth_client(repository: &mut MemoryOAuthClientRepository) -> OAuthClientId {
    let client = OAuthClient::new(
        OAuthClientId::new(),
        "demo-client",
        "LocalID Demo Client",
        "demo-secret-hash",
        vec!["http://localhost:3000/callback".to_string()],
    );

    let id = client.id();

    repository
        .save(client)
        .expect("failed to seed oauth client");

    id
}
/// Creates application state with in-memory dependencies.
pub fn create_state() -> BootstrapContext<
    BootstrapAuthenticationService,
    BootstrapRefreshService,
    BootstrapVerificationService,
    BootstrapSessionService,
    BootstrapAuthorizationResolver,
    ClientRepositoryAdapter<MemoryClientRepository>,
    BootstrapAuthorizationAdapter,
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

    let mut oauth_client_repository = MemoryOAuthClientRepository::new();

    let oauth_client_id = seed_demo_oauth_client(&mut oauth_client_repository);

    let authorization_code_repository = MemoryAuthorizationCodeRepository::new();

    let authorization_adapter =
        AuthorizationRepositoryAdapter::new(oauth_client_repository, authorization_code_repository);

    let authorize_use_case = AuthorizeUseCase::new(authorization_adapter);

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

    let refresh_use_case = RefreshTokenUseCase::new(RefreshTokenAdapter::new(refresh_service));

    let verification_service =
        DefaultTokenVerificationService::new(token_repository.clone(), session_repository.clone());

    let verify_token_use_case = Arc::new(Mutex::new(VerifyTokenUseCase::new(
        TokenVerificationAdapter::new(verification_service),
    )));

    let auth_state = AuthMiddlewareState::new(verify_token_use_case.clone());

    let current_session_use_case = GetCurrentSessionUseCase::new(SessionAdapter::new(
        DefaultSessionService::new(session_repository.clone()),
    ));

    let logout_session_use_case = LogoutSessionUseCase::new(SessionAdapter::new(
        DefaultSessionService::new(session_repository),
    ));

    let authorization_state = AuthorizationMiddlewareState::new(AuthorizationContextResolver::new(
        IdentityRoleAdapter::new(identity_role_repository),
    ));

    let login_use_case = LoginUseCase::new(
        PasswordAuthenticationAdapter::new(authentication_service),
        client_use_case,
    );

    BootstrapContext {
        state: AppState::new(
            login_use_case,
            refresh_use_case,
            verify_token_use_case,
            current_session_use_case,
            logout_session_use_case,
            authorize_use_case,
        ),
        auth_state,
        authorization_state,
        credential_id,
        client_id,
        oauth_client_id,
    }
}
