use localid_application::{
    ClientAuthenticationCommand, ClientAuthenticationError, ClientAuthenticationUseCase,
};

use localid_crypto::hash_secret;
use localid_oauth_client::{OAuthClient, OAuthClientId};
use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

use localid_client::ClientId;

use localid_application::ClientAuthenticationPort;

use localid_oauth_client::OAuthClientRepository;

fn create_active_client() -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        ClientId::new(),
        "demo-client",
        "Demo Client",
        hash_secret("demo-secret"),
        vec!["http://localhost:3000/callback".to_string()],
    )
}

struct TestClientAuthenticationRepository {
    repository: MemoryOAuthClientRepository,
}

impl ClientAuthenticationPort for TestClientAuthenticationRepository {
    type Error = ();

    fn find_client(
        &self,
        client_id: &str,
    ) -> Result<Option<localid_oauth_client::OAuthClient>, Self::Error> {
        self.repository.find_by_client_id(client_id)
    }
}

#[test]
fn should_authenticate_active_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    repository
        .save(create_active_client())
        .expect("save should succeed");

    let use_case =
        ClientAuthenticationUseCase::new(TestClientAuthenticationRepository { repository });

    let command = ClientAuthenticationCommand::new("demo-client", "demo-secret");

    let result = use_case
        .execute(command)
        .expect("authentication should succeed");

    assert_eq!(result.client_id(), "demo-client");
}

#[test]
fn should_reject_invalid_secret() {
    let mut repository = MemoryOAuthClientRepository::new();

    repository
        .save(create_active_client())
        .expect("save should succeed");

    let use_case =
        ClientAuthenticationUseCase::new(TestClientAuthenticationRepository { repository });

    let command = ClientAuthenticationCommand::new("demo-client", "wrong-secret");

    let result = use_case.execute(command);

    assert!(matches!(
        result,
        Err(ClientAuthenticationError::InvalidSecret)
    ));
}

#[test]
fn should_reject_disabled_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let mut client = create_active_client();

    client.disable().expect("disable should succeed");

    repository.save(client).expect("save should succeed");

    let use_case =
        ClientAuthenticationUseCase::new(TestClientAuthenticationRepository { repository });

    let command = ClientAuthenticationCommand::new("demo-client", "demo-secret");

    let result = use_case.execute(command);

    assert!(matches!(
        result,
        Err(ClientAuthenticationError::ClientInactive)
    ));
}

#[test]
fn should_reject_deleted_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let mut client = create_active_client();

    client.delete().expect("delete should succeed");

    repository.save(client).expect("save should succeed");

    let use_case =
        ClientAuthenticationUseCase::new(TestClientAuthenticationRepository { repository });

    let command = ClientAuthenticationCommand::new("demo-client", "demo-secret");

    let result = use_case.execute(command);

    assert!(matches!(
        result,
        Err(ClientAuthenticationError::ClientInactive)
    ));
}
