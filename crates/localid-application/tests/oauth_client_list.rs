use localid_application::ListOAuthClientsUseCase;

use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

use localid_client::ClientId;
use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

fn create_client(name: &str) -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        ClientId::new(),
        name,
        name,
        "secret-hash",
        vec!["http://localhost:3000/callback".to_string()],
    )
}

#[test]
fn should_list_oauth_clients() {
    let mut repository = MemoryOAuthClientRepository::new();

    repository
        .save(create_client("client-a"))
        .expect("save should succeed");

    repository
        .save(create_client("client-b"))
        .expect("save should succeed");

    let use_case = ListOAuthClientsUseCase::new(repository);

    let result = use_case.execute().expect("list should succeed");

    assert_eq!(result.clients().len(), 2);
}

#[test]
fn should_return_empty_list_when_no_client_exists() {
    let repository = MemoryOAuthClientRepository::new();

    let use_case = ListOAuthClientsUseCase::new(repository);

    let result = use_case.execute().expect("list should succeed");

    assert!(result.clients().is_empty());
}
