use localid_application::{
    DeleteOAuthClientCommand, DeleteOAuthClientError, DeleteOAuthClientUseCase,
};

use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientRepository};

use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

use localid_client::ClientId;

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
fn should_delete_active_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let client = create_active_client();
    let client_id = client.id();

    repository.save(client).expect("client should be saved");

    let mut use_case = DeleteOAuthClientUseCase::new(repository);

    let result = use_case.execute(DeleteOAuthClientCommand::new(client_id));

    assert!(result.is_ok());
}

#[test]
fn should_reject_unknown_oauth_client() {
    let repository = MemoryOAuthClientRepository::new();

    let mut use_case = DeleteOAuthClientUseCase::new(repository);

    let result = use_case.execute(DeleteOAuthClientCommand::new(OAuthClientId::new()));

    assert!(matches!(result, Err(DeleteOAuthClientError::NotFound)));
}

#[test]
fn should_reject_deleted_oauth_client() {
    let mut repository = MemoryOAuthClientRepository::new();

    let mut client = create_active_client();

    let client_id = client.id();

    client.delete().expect("client should delete");

    repository.save(client).expect("client should be saved");

    let mut use_case = DeleteOAuthClientUseCase::new(repository);

    let result = use_case.execute(DeleteOAuthClientCommand::new(client_id));

    assert!(matches!(
        result,
        Err(DeleteOAuthClientError::AlreadyDeleted)
    ));
}
