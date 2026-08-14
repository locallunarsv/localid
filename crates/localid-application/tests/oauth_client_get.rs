use localid_application::{GetOAuthClientError, GetOAuthClientQuery, GetOAuthClientUseCase};

use localid_client::ClientId;

use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

fn create_client() -> OAuthClient {
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
fn should_get_existing_oauth_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let client = create_client();

    let client_id = client.id();

    repository.save(client).expect("save should succeed");

    let use_case = GetOAuthClientUseCase::new(repository);

    let query = GetOAuthClientQuery::new(client_id);

    let result = use_case.execute(query).expect("client should exist");

    assert_eq!(result.client().client_id(), "demo-client");
}

#[test]
fn should_reject_unknown_oauth_client() {
    let repository = MemoryOAuthClientRepository::new();

    let use_case = GetOAuthClientUseCase::new(repository);

    let query = GetOAuthClientQuery::new(OAuthClientId::new());

    let result = use_case.execute(query);

    assert!(matches!(result, Err(GetOAuthClientError::NotFound)));
}
