use std::path::PathBuf;
use std::sync::Arc;

use tokio::runtime::Handle;
use tokio::sync::Mutex;

mod id_token;
mod postgres;
mod repository;
mod seed;
mod seed_context;

pub use id_token::BootstrapIdTokenIssuer;
mod environment;

use crate::bootstrap::seed_context::DemoSeedContext;
pub use environment::Environment;

pub use postgres::{create_postgres_repositories, PostgresRepositories};

use crate::bootstrap::postgres::{
    SharedPostgresAuthorizationCodeRepository, SharedPostgresClientRepository,
    SharedPostgresCredentialRepository, SharedPostgresIdentityRepository,
    SharedPostgresIdentityRoleRepository, SharedPostgresPasswordMaterialRepository,
    SharedPostgresRefreshTokenRepository, SharedPostgresSessionRepository,
    SharedPostgresTokenRepository,
};

use crate::bootstrap::seed_context::seed_demo_environment;

use localid_application::{
    authentication::{PasswordAuthenticationAdapter, TokenVerificationAdapter},
    ActivateOAuthClientUseCase, AuthorizationContextResolver, AuthorizationRepositoryAdapter,
    AuthorizeUseCase, ClientAuthenticationUseCase, ClientRepositoryAdapter,
    CreateOAuthClientUseCase, DeleteOAuthClientUseCase, DisableOAuthClientUseCase,
    GetClientUseCase, GetCurrentSessionUseCase, GetIdentityUseCase, GetOAuthClientUseCase,
    IdentityRepositoryAdapter, IdentityRoleAdapter, ListOAuthClientsUseCase, LoginUseCase,
    LogoutSessionUseCase, RefreshTokenAdapter, RefreshTokenUseCase, SessionAdapter,
    TokenExchangeRepositoryAdapter, TokenExchangeUseCase, VerifyTokenUseCase,
};

use localid_authentication::{
    DefaultPasswordAuthenticationService, DefaultRefreshTokenService, DefaultSessionFactory,
    DefaultSessionService, DefaultTokenIssuanceService, DefaultTokenVerificationService,
    PasswordAuthenticationDependencies,
};

use localid_client::ClientId;
use localid_config::{DatabaseConfig, ServerConfig};
use localid_credential::CredentialId;
use localid_database_postgres::PostgresOAuthClientRepository;
use localid_identity::IdentityId;

use localid_oauth_client::OAuthClientId;

use localid_crypto::{FileKeyStorage, KeyId, KeyPair};

use localid_password_argon2::Argon2PasswordHasher;
use localid_refresh_token_random::RandomRefreshTokenIssuer;
use localid_token_random::RandomTokenIssuer;

use crate::{
    middleware::{AuthMiddlewareState, AuthorizationMiddlewareState},
    AppState,
};

use crate::bootstrap::repository::SharedRepository;

type SharedIdentityRepository = SharedPostgresIdentityRepository;

type SharedCredentialRepository = SharedPostgresCredentialRepository;

type SharedPasswordMaterialRepository = SharedPostgresPasswordMaterialRepository;

type SharedIdentityRoleRepository = SharedPostgresIdentityRoleRepository;

type SharedSessionRepository = SharedPostgresSessionRepository;

type SharedTokenRepository = SharedPostgresTokenRepository;

type SharedRefreshTokenRepository = SharedPostgresRefreshTokenRepository;

/// Context containing initialized application dependencies.
pub struct BootstrapContext<L, R, V, S, C, O, REX, TEX, IR, ID, ITI, CA, OCM> {
    /// Shared application state.
    pub state: AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>,

    /// Authentication middleware state.
    pub auth_state: AuthMiddlewareState<V>,

    /// Authorization middleware state.
    pub authorization_state: AuthorizationMiddlewareState<AuthorizationContextResolver<IR>>,

    /// Seeded credential identifier.
    pub credential_id: CredentialId,

    /// Seeded identity identifier.
    pub identity_id: IdentityId,

    /// Seeded local client identifier.
    pub client_id: ClientId,

    /// Seeded secondary OAuth client public identifier.
    pub oauth_client_other_public_id: String,

    /// Seeded OAuth client internal identifier.
    pub oauth_client_id: OAuthClientId,

    /// Seeded OAuth client public identifier.
    pub oauth_client_public_id: String,

    /// Seeded OAuth client secret.
    pub oauth_client_secret: String,

    /// Shared OAuth client repository.
    pub oauth_client_repository: OCM,

    /// Development demo seed data.
    pub demo_seed: Option<DemoSeedContext>,
}

type SharedAuthorizationCodeRepository = SharedPostgresAuthorizationCodeRepository;

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
        SharedCredentialRepository,
        SharedPasswordMaterialRepository,
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

type BootstrapIdentityRoleAdapter = IdentityRoleAdapter<SharedIdentityRoleRepository>;

/// Identity repository adapter used by bootstrap dependencies.
pub type BootstrapIdentityRepositoryAdapter = IdentityRepositoryAdapter<SharedIdentityRepository>;

/// Identity lookup use case used by bootstrap dependencies.
pub type BootstrapIdentityUseCase = GetIdentityUseCase<BootstrapIdentityRepositoryAdapter>;

type BootstrapClientAuthenticationRepository = SharedPostgresOAuthClientRepository;

type BootstrapOAuthClientRepository = SharedPostgresOAuthClientRepository;

pub(super) type SharedPostgresOAuthClientRepository =
    SharedRepository<PostgresOAuthClientRepository>;

type BootstrapAuthorizationAdapter<OCR> =
    AuthorizationRepositoryAdapter<OCR, SharedAuthorizationCodeRepository>;

type BootstrapTokenExchangeAdapter<OCR> =
    TokenExchangeRepositoryAdapter<OCR, SharedAuthorizationCodeRepository>;

/// Creates application state with in-memory dependencies.
pub async fn create_state(
    _database: DatabaseConfig,
    environment: Environment,
) -> BootstrapContext<
    BootstrapAuthenticationService,
    BootstrapRefreshService,
    BootstrapVerificationService,
    BootstrapSessionService,
    ClientRepositoryAdapter<SharedPostgresClientRepository>,
    BootstrapAuthorizationAdapter<SharedPostgresOAuthClientRepository>,
    BootstrapTokenExchangeAdapter<SharedPostgresOAuthClientRepository>,
    BootstrapTokenIssuanceService,
    BootstrapIdentityRoleAdapter,
    BootstrapIdentityUseCase,
    BootstrapIdTokenIssuer,
    BootstrapClientAuthenticationRepository,
    BootstrapOAuthClientRepository,
> {
    let should_seed = environment.should_seed();

    let repositories = create_postgres_repositories(&_database, Handle::current())
        .await
        .expect("postgres repositories should initialize");

    let mut identity_repository = repositories.identity.clone();

    let mut credential_repository = repositories.credential.clone();

    let mut password_material_repository = repositories.password_material.clone();

    let mut identity_role_repository = repositories.identity_role.clone();

    let identity_repository_adapter = IdentityRepositoryAdapter::new(identity_repository.clone());

    let identity_use_case = GetIdentityUseCase::new(identity_repository_adapter);

    let mut client_repository = repositories.client.clone();

    let mut oauth_client_repository = repositories.oauth_client.clone();

    let demo_seed = if should_seed {
        Some(seed_demo_environment(
            &mut identity_repository,
            &mut credential_repository,
            &mut password_material_repository,
            &mut identity_role_repository,
            &mut client_repository,
            &mut oauth_client_repository,
        ))
    } else {
        None
    };

    let client_adapter = ClientRepositoryAdapter::new(client_repository);

    let client_use_case = GetClientUseCase::new(client_adapter);

    let create_oauth_client_use_case =
        CreateOAuthClientUseCase::new(oauth_client_repository.clone());

    let get_oauth_client_use_case = GetOAuthClientUseCase::new(oauth_client_repository.clone());

    let list_oauth_clients_use_case = ListOAuthClientsUseCase::new(oauth_client_repository.clone());

    let disable_oauth_client_use_case =
        DisableOAuthClientUseCase::new(oauth_client_repository.clone());

    let activate_oauth_client_use_case =
        ActivateOAuthClientUseCase::new(oauth_client_repository.clone());

    let delete_oauth_client_use_case =
        DeleteOAuthClientUseCase::new(oauth_client_repository.clone());

    let authorization_code_repository = repositories.authorization_code.clone();

    let authorization_adapter = AuthorizationRepositoryAdapter::new(
        oauth_client_repository.clone(),
        authorization_code_repository.clone(),
    );

    let authorize_use_case = AuthorizeUseCase::new(authorization_adapter);

    let session_repository = repositories.session.clone();

    let token_repository = repositories.token.clone();

    let refresh_token_repository = repositories.refresh_token.clone();

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

    let config = ServerConfig::new("http://localhost:8080");

    let key_storage = FileKeyStorage::new();

    let key_path = PathBuf::from(config.signing_key_path.clone());

    let key_pair = Arc::new(
        KeyPair::load_or_generate(&key_storage, &key_path, KeyId::new("localid-key-1"))
            .expect("signing key loading should succeed"),
    );

    let id_token_issuer = BootstrapIdTokenIssuer::new(Arc::clone(&key_pair));

    let token_exchange_repository = TokenExchangeRepositoryAdapter::new(
        oauth_client_repository.clone(),
        authorization_code_repository.clone(),
    );

    let token_exchange_use_case = TokenExchangeUseCase::new(
        token_exchange_repository,
        token_exchange_issuance_service,
        id_token_issuer,
    );

    let client_authentication_use_case =
        ClientAuthenticationUseCase::new(oauth_client_repository.clone());

    let login_use_case = LoginUseCase::new(
        PasswordAuthenticationAdapter::new(authentication_service),
        client_use_case,
    );

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
            client_authentication_use_case,
            create_oauth_client_use_case,
            get_oauth_client_use_case,
            list_oauth_clients_use_case,
            disable_oauth_client_use_case,
            activate_oauth_client_use_case,
            delete_oauth_client_use_case,
            identity_use_case,
            key_pair,
        ),

        auth_state,
        authorization_state,

        credential_id: demo_seed
            .as_ref()
            .expect("demo seed required")
            .credential_id
            .clone(),

        identity_id: demo_seed
            .as_ref()
            .expect("demo seed required")
            .identity_id
            .clone(),

        client_id: demo_seed
            .as_ref()
            .expect("demo seed required")
            .client_id
            .clone(),

        oauth_client_id: demo_seed
            .as_ref()
            .expect("demo seed required")
            .oauth_client_id
            .clone(),

        oauth_client_public_id: demo_seed
            .as_ref()
            .expect("demo seed required")
            .oauth_client_public_id
            .clone(),

        oauth_client_secret: demo_seed
            .as_ref()
            .expect("demo seed required")
            .oauth_client_secret
            .clone(),

        oauth_client_other_public_id: demo_seed
            .as_ref()
            .expect("demo seed required")
            .oauth_client_other_public_id
            .clone(),

        oauth_client_repository,

        demo_seed,
    }
}
