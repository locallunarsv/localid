use std::sync::Arc;

use tokio::sync::Mutex;

mod repository;
mod seed;

pub use seed::{seed_demo_client, seed_demo_identity, seed_demo_oauth_client, seed_oauth_client};

use localid_application::{
    authentication::{PasswordAuthenticationAdapter, TokenVerificationAdapter},
    AuthorizationContextResolver, AuthorizationRepositoryAdapter, AuthorizeUseCase,
    ClientRepositoryAdapter, GetClientUseCase, GetCurrentSessionUseCase, GetIdentityUseCase,
    IdentityRepositoryAdapter, IdentityRoleAdapter, LoginUseCase, LogoutSessionUseCase,
    RefreshTokenAdapter, RefreshTokenUseCase, SessionAdapter, TokenExchangeRepositoryAdapter,
    TokenExchangeUseCase, VerifyTokenUseCase,
};

use localid_authentication::{
    DefaultPasswordAuthenticationService, DefaultRefreshTokenService, DefaultSessionFactory,
    DefaultSessionService, DefaultTokenIssuanceService, DefaultTokenVerificationService,
    PasswordAuthenticationDependencies,
};

use localid_config::ServerConfig;

use localid_client::ClientId;
use localid_credential::CredentialId;
use localid_identity::IdentityId;

use localid_oauth_authorization_repository_memory::MemoryAuthorizationCodeRepository;
use localid_oauth_client::OAuthClientId;
use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

use localid_password_argon2::Argon2PasswordHasher;
use localid_refresh_token_random::RandomRefreshTokenIssuer;

use localid_repository_memory::{
    MemoryClientRepository, MemoryCredentialRepository, MemoryIdentityRepository,
    MemoryIdentityRoleRepository, MemoryPasswordMaterialRepository, MemoryRefreshTokenRepository,
    MemorySessionRepository, MemoryTokenRepository,
};

use localid_token_random::RandomTokenIssuer;

use crate::{
    middleware::{AuthMiddlewareState, AuthorizationMiddlewareState},
    AppState,
};

use crate::bootstrap::repository::SharedRepository;

type SharedIdentityRepository = SharedRepository<MemoryIdentityRepository>;

/// Context containing initialized application dependencies.
pub struct BootstrapContext<L, R, V, S, C, O, REX, TEX, IR, I> {
    /// Shared application state.
    pub state: AppState<L, R, V, S, C, O, REX, TEX, I>,

    /// Authentication middleware state.
    pub auth_state: AuthMiddlewareState<V>,

    /// Authorization middleware state.
    pub authorization_state: AuthorizationMiddlewareState<AuthorizationContextResolver<IR>>,

    /// Seeded credential identifier.
    pub credential_id: CredentialId,

    /// Seeded identity identifier.
    pub identity_id: IdentityId,

    /// Seeded client identifier.
    pub client_id: ClientId,

    /// Seeded second OAuth public identifier.
    pub oauth_client_other_public_id: String,

    /// Seeded OAuth internal identifier.
    pub oauth_client_id: OAuthClientId,

    /// Seeded OAuth public identifier.
    pub oauth_client_public_id: String,
}

type SharedSessionRepository = SharedRepository<MemorySessionRepository>;

type SharedTokenRepository = SharedRepository<MemoryTokenRepository>;

type SharedRefreshTokenRepository = SharedRepository<MemoryRefreshTokenRepository>;

type SharedOAuthClientRepository = SharedRepository<MemoryOAuthClientRepository>;

type SharedAuthorizationCodeRepository = SharedRepository<MemoryAuthorizationCodeRepository>;

type BootstrapTokenIssuanceService = DefaultTokenIssuanceService<
    SharedSessionRepository,
    SharedTokenRepository,
    SharedRefreshTokenRepository,
    DefaultSessionFactory,
    RandomTokenIssuer,
    RandomRefreshTokenIssuer,
>;

type BootstrapAuthenticationService = PasswordAuthenticationAdapter<
    DefaultPasswordAuthenticationService<
        SharedIdentityRepository,
        MemoryCredentialRepository,
        MemoryPasswordMaterialRepository,
        Argon2PasswordHasher,
        BootstrapTokenIssuanceService,
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

type BootstrapIdentityRoleAdapter = IdentityRoleAdapter<MemoryIdentityRoleRepository>;

/// Identity repository adapter used by bootstrap dependencies.
pub type BootstrapIdentityRepositoryAdapter = IdentityRepositoryAdapter<SharedIdentityRepository>;

/// Identity lookup use case used by bootstrap dependencies.
pub type BootstrapIdentityUseCase = GetIdentityUseCase<BootstrapIdentityRepositoryAdapter>;

type BootstrapAuthorizationAdapter =
    AuthorizationRepositoryAdapter<SharedOAuthClientRepository, SharedAuthorizationCodeRepository>;

type BootstrapTokenExchangeAdapter =
    TokenExchangeRepositoryAdapter<SharedOAuthClientRepository, SharedAuthorizationCodeRepository>;

/// Creates application state with in-memory dependencies.
pub fn create_state() -> BootstrapContext<
    BootstrapAuthenticationService,
    BootstrapRefreshService,
    BootstrapVerificationService,
    BootstrapSessionService,
    ClientRepositoryAdapter<MemoryClientRepository>,
    BootstrapAuthorizationAdapter,
    BootstrapTokenExchangeAdapter,
    BootstrapTokenIssuanceService,
    BootstrapIdentityRoleAdapter,
    BootstrapIdentityUseCase,
> {
    let mut identity_repository = SharedRepository::new(MemoryIdentityRepository::new());

    let mut credential_repository = MemoryCredentialRepository::new();

    let mut password_material_repository = MemoryPasswordMaterialRepository::new();

    let mut identity_role_repository = MemoryIdentityRoleRepository::new();

    let (credential_id, identity_id) = seed_demo_identity(
        &mut identity_repository,
        &mut credential_repository,
        &mut password_material_repository,
        &mut identity_role_repository,
    );

    let identity_repository_adapter = IdentityRepositoryAdapter::new(identity_repository.clone());

    let identity_use_case = GetIdentityUseCase::new(identity_repository_adapter);

    let mut client_repository = MemoryClientRepository::new();

    let client_id = seed_demo_client(&mut client_repository);

    let client_adapter = ClientRepositoryAdapter::new(client_repository);

    let client_use_case = GetClientUseCase::new(client_adapter);

    let mut oauth_client_repository = MemoryOAuthClientRepository::new();

    let (oauth_client_id, oauth_client_public_id) =
        seed_demo_oauth_client(&mut oauth_client_repository);

    let (_, oauth_client_other_public_id) =
        seed_oauth_client(&mut oauth_client_repository, "different-client".to_string());

    let oauth_client_repository = SharedRepository::new(oauth_client_repository);

    let authorization_code_repository =
        SharedRepository::new(MemoryAuthorizationCodeRepository::new());

    let authorization_adapter = AuthorizationRepositoryAdapter::new(
        oauth_client_repository.clone(),
        authorization_code_repository.clone(),
    );

    let authorize_use_case = AuthorizeUseCase::new(authorization_adapter);

    let session_repository = SharedRepository::new(MemorySessionRepository::new());

    let token_repository = SharedRepository::new(MemoryTokenRepository::new());

    let refresh_token_repository = SharedRepository::new(MemoryRefreshTokenRepository::new());

    let token_issuance_service = DefaultTokenIssuanceService::new(
        session_repository.clone(),
        token_repository.clone(),
        refresh_token_repository.clone(),
        DefaultSessionFactory::new(),
        RandomTokenIssuer::new(),
        RandomRefreshTokenIssuer::new(),
    );

    let token_exchange_issuance_service = DefaultTokenIssuanceService::new(
        session_repository.clone(),
        token_repository.clone(),
        refresh_token_repository.clone(),
        DefaultSessionFactory::new(),
        RandomTokenIssuer::new(),
        RandomRefreshTokenIssuer::new(),
    );

    let authentication_service =
        DefaultPasswordAuthenticationService::new(PasswordAuthenticationDependencies {
            identity_repository,
            credential_repository,
            password_material_repository,
            password_verifier: Argon2PasswordHasher::new(),
            token_issuance_service,
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

    let token_exchange_repository = TokenExchangeRepositoryAdapter::new(
        oauth_client_repository.clone(),
        authorization_code_repository.clone(),
    );

    let token_exchange_use_case =
        TokenExchangeUseCase::new(token_exchange_repository, token_exchange_issuance_service);

    let login_use_case = LoginUseCase::new(
        PasswordAuthenticationAdapter::new(authentication_service),
        client_use_case,
    );

    let config = ServerConfig::new("http://localhost:8080");

    BootstrapContext {
        state: AppState::new(
            config,
            login_use_case,
            refresh_use_case,
            verify_token_use_case,
            current_session_use_case,
            logout_session_use_case,
            authorize_use_case,
            token_exchange_use_case,
            identity_use_case,
        ),

        auth_state,
        authorization_state,

        credential_id,
        identity_id,
        client_id,

        oauth_client_id,
        oauth_client_public_id,
        oauth_client_other_public_id,
    }
}
