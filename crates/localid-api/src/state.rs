use std::sync::Arc;

use tokio::sync::Mutex;

use localid_application::{
    ActivateOAuthClientUseCase, AuthorizeUseCase, ClientAuthenticationUseCase,
    CreateOAuthClientUseCase, DeleteOAuthClientUseCase, DisableOAuthClientUseCase,
    GetCurrentSessionUseCase, GetOAuthClientUseCase, ListOAuthClientsUseCase, LoginUseCase,
    LogoutSessionUseCase, RefreshTokenUseCase, TokenExchangeUseCase, VerifyTokenUseCase,
};

use localid_config::ServerConfig;
use localid_crypto::KeyPair;

/// Shared application state.
pub struct AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM> {
    /// Server configuration.
    pub config: Arc<ServerConfig>,

    /// Login authentication use case.
    pub login_use_case: Arc<Mutex<LoginUseCase<L, C>>>,

    /// Refresh token use case.
    pub refresh_use_case: Arc<Mutex<RefreshTokenUseCase<R>>>,

    /// Token verification use case.
    pub verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,

    /// Current session use case.
    pub current_session_use_case: Arc<Mutex<GetCurrentSessionUseCase<S>>>,

    /// Logout session use case.
    pub logout_use_case: Arc<Mutex<LogoutSessionUseCase<S>>>,

    /// OAuth authorization use case.
    pub authorize_use_case: Arc<Mutex<AuthorizeUseCase<O>>>,

    /// OAuth token exchange use case.
    pub token_exchange_use_case: Arc<Mutex<TokenExchangeUseCase<REX, TEX, ITI>>>,

    /// OAuth client authentication use case.
    pub client_authentication_use_case: Arc<Mutex<ClientAuthenticationUseCase<CA>>>,

    /// OAuth client creation use case.
    pub create_oauth_client_use_case: Arc<Mutex<CreateOAuthClientUseCase<OCM>>>,

    /// OAuth client lookup use case.
    pub get_oauth_client_use_case: Arc<Mutex<GetOAuthClientUseCase<OCM>>>,

    /// OAuth client listing use case.
    pub list_oauth_clients_use_case: Arc<Mutex<ListOAuthClientsUseCase<OCM>>>,

    /// OAuth client disable use case.
    pub disable_oauth_client_use_case: Arc<Mutex<DisableOAuthClientUseCase<OCM>>>,

    /// OAuth client activation use case.
    pub activate_oauth_client_use_case: Arc<Mutex<ActivateOAuthClientUseCase<OCM>>>,

    /// OAuth client deletion use case.
    pub delete_oauth_client_use_case: Arc<Mutex<DeleteOAuthClientUseCase<OCM>>>,

    /// Identity lookup use case.
    pub identity_use_case: Arc<Mutex<ID>>,

    /// Active OIDC signing key.
    pub signing_key: Arc<KeyPair>,
}

impl<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM> Clone
    for AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>
{
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),

            login_use_case: Arc::clone(&self.login_use_case),
            refresh_use_case: Arc::clone(&self.refresh_use_case),
            verify_token_use_case: Arc::clone(&self.verify_token_use_case),

            current_session_use_case: Arc::clone(&self.current_session_use_case),
            logout_use_case: Arc::clone(&self.logout_use_case),

            authorize_use_case: Arc::clone(&self.authorize_use_case),
            token_exchange_use_case: Arc::clone(&self.token_exchange_use_case),

            client_authentication_use_case: Arc::clone(&self.client_authentication_use_case),

            create_oauth_client_use_case: Arc::clone(&self.create_oauth_client_use_case),

            get_oauth_client_use_case: Arc::clone(&self.get_oauth_client_use_case),

            list_oauth_clients_use_case: Arc::clone(&self.list_oauth_clients_use_case),

            disable_oauth_client_use_case: Arc::clone(&self.disable_oauth_client_use_case),

            activate_oauth_client_use_case: Arc::clone(&self.activate_oauth_client_use_case),

            delete_oauth_client_use_case: Arc::clone(&self.delete_oauth_client_use_case),

            identity_use_case: Arc::clone(&self.identity_use_case),

            signing_key: Arc::clone(&self.signing_key),
        }
    }
}

impl<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>
    AppState<L, R, V, S, C, O, REX, TEX, ID, ITI, CA, OCM>
{
    /// Creates shared application state.
    #[must_use]
    pub fn new(
        config: ServerConfig,
        login_use_case: LoginUseCase<L, C>,
        refresh_use_case: RefreshTokenUseCase<R>,
        verify_token_use_case: Arc<Mutex<VerifyTokenUseCase<V>>>,
        current_session_use_case: GetCurrentSessionUseCase<S>,
        logout_use_case: LogoutSessionUseCase<S>,
        authorize_use_case: AuthorizeUseCase<O>,
        token_exchange_use_case: TokenExchangeUseCase<REX, TEX, ITI>,
        client_authentication_use_case: ClientAuthenticationUseCase<CA>,
        create_oauth_client_use_case: CreateOAuthClientUseCase<OCM>,
        get_oauth_client_use_case: GetOAuthClientUseCase<OCM>,
        list_oauth_clients_use_case: ListOAuthClientsUseCase<OCM>,
        disable_oauth_client_use_case: DisableOAuthClientUseCase<OCM>,
        activate_oauth_client_use_case: ActivateOAuthClientUseCase<OCM>,
        delete_oauth_client_use_case: DeleteOAuthClientUseCase<OCM>,
        identity_use_case: ID,
        signing_key: Arc<KeyPair>,
    ) -> Self {
        Self {
            config: Arc::new(config),

            login_use_case: Arc::new(Mutex::new(login_use_case)),
            refresh_use_case: Arc::new(Mutex::new(refresh_use_case)),
            verify_token_use_case,

            current_session_use_case: Arc::new(Mutex::new(current_session_use_case)),
            logout_use_case: Arc::new(Mutex::new(logout_use_case)),

            authorize_use_case: Arc::new(Mutex::new(authorize_use_case)),
            token_exchange_use_case: Arc::new(Mutex::new(token_exchange_use_case)),

            client_authentication_use_case: Arc::new(Mutex::new(client_authentication_use_case)),

            create_oauth_client_use_case: Arc::new(Mutex::new(create_oauth_client_use_case)),
            get_oauth_client_use_case: Arc::new(Mutex::new(get_oauth_client_use_case)),
            list_oauth_clients_use_case: Arc::new(Mutex::new(list_oauth_clients_use_case)),
            disable_oauth_client_use_case: Arc::new(Mutex::new(disable_oauth_client_use_case)),
            activate_oauth_client_use_case: Arc::new(Mutex::new(activate_oauth_client_use_case)),
            delete_oauth_client_use_case: Arc::new(Mutex::new(delete_oauth_client_use_case)),

            identity_use_case: Arc::new(Mutex::new(identity_use_case)),

            signing_key,
        }
    }
}
