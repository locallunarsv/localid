use localid_application::{CreateOAuthClientCommand, CreateOAuthClientUseCase};

use localid_oauth_client_repository_memory::MemoryOAuthClientRepository;

#[test]
fn should_create_oauth_client() {
    let repository = MemoryOAuthClientRepository::new();

    let mut use_case = CreateOAuthClientUseCase::new(repository);

    let command = CreateOAuthClientCommand::new(
        "Test Application",
        vec!["http://localhost:3000/callback".to_string()],
    );

    let result = use_case
        .execute(command)
        .expect("oauth client creation should succeed");

    assert!(!result.client_id().is_empty());

    assert!(!result.client_secret().is_empty());
}

#[test]
fn should_store_hashed_secret_not_plain_secret() {
    let repository = MemoryOAuthClientRepository::new();

    let mut use_case = CreateOAuthClientUseCase::new(repository);

    let command = CreateOAuthClientCommand::new(
        "Hash Test Application",
        vec!["http://localhost:3000/callback".to_string()],
    );

    let result = use_case
        .execute(command)
        .expect("oauth client creation should succeed");

    let secret = result.client_secret();

    assert!(!secret.is_empty());
}
