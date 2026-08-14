use localid_application::{
    DisableOAuthClientCommand, DisableOAuthClientError, DisableOAuthClientUseCase,
};

use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

use localid_client::ClientId;
use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

fn create_active_client() -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        ClientId::new(),
        "demo-client",
        "Demo Client",
        "secret-hash",
        vec!["http://localhost:3000/callback".to_string()],
    )
}

#[test]
fn should_disable_active_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let client = create_active_client();
    let client_id = client.id();

    repository.save(client).expect("client should save");

    let mut use_case = DisableOAuthClientUseCase::new(repository);

    let result = use_case.execute(DisableOAuthClientCommand::new(client_id));

    assert!(result.is_ok());
}

#[test]
fn should_reject_unknown_oauth_client() {
    let repository = MemoryOAuthClientRepository::new();

    let mut use_case = DisableOAuthClientUseCase::new(repository);

    let result = use_case.execute(DisableOAuthClientCommand::new(OAuthClientId::new()));

    assert!(matches!(result, Err(DisableOAuthClientError::NotFound)));
}

#[test]
fn should_reject_deleted_oauth_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let mut client = create_active_client();

    let client_id = client.id();

    client.delete().expect("delete should succeed");

    repository.save(client).expect("client should save");

    let mut use_case = DisableOAuthClientUseCase::new(repository);

    let result = use_case.execute(DisableOAuthClientCommand::new(client_id));

    assert!(matches!(
        result,
        Err(DisableOAuthClientError::AlreadyDeleted)
    ));
}
