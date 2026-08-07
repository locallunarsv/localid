use localid_application::{ AuthorizationPort, AuthorizationRepositoryAdapter };

use localid_oauth_authorization::{ AuthorizationCode, AuthorizationCodeId };

use localid_oauth_authorization_repository_memory::{ MemoryAuthorizationCodeRepository };

use localid_client::ClientId;
use localid_oauth_client::{ OAuthClient, OAuthClientId };

use localid_oauth_client_repository_memory::{ MemoryOAuthClientRepository };

fn create_client() -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        ClientId::new(),
        "demo-client",
        "Demo OAuth Client",
        "secret-hash",
        vec!["http://localhost/callback".to_owned()]
    )
}

#[test]
fn adapter_should_find_oauth_client() {
    let mut client_repository = MemoryOAuthClientRepository::new();

    let client = create_client();

    let client_id = client.client_id().to_owned();

    client_repository.save(client).expect("client should save");

    let code_repository = MemoryAuthorizationCodeRepository::new();

    let adapter = AuthorizationRepositoryAdapter::new(client_repository, code_repository);

    let result = adapter.find_client(&client_id).expect("lookup should succeed");

    assert!(result.is_some());
}

#[test]
fn adapter_should_save_authorization_code() {
    let client_repository = MemoryOAuthClientRepository::new();

    let code_repository = MemoryAuthorizationCodeRepository::new();

    let mut adapter = AuthorizationRepositoryAdapter::new(client_repository, code_repository);

    let code = AuthorizationCode::new(
        AuthorizationCodeId::new(),
        OAuthClientId::new(),
        localid_identity::IdentityId::new(),
        "code-hash",
        "http://localhost/callback",
        chrono::Utc::now(),
        chrono::Utc::now() + chrono::Duration::minutes(10)
    ).expect("code should be valid");

    let result = adapter.save_code(code);

    assert!(result.is_ok());
}
