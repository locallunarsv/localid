use localid_oauth_client::{OAuthClient, OAuthClientId, OAuthClientLifecycleState};

fn create_client() -> OAuthClient {
    OAuthClient::new(
        OAuthClientId::new(),
        "github-client",
        "GitHub Application",
        "secret-hash",
        vec!["http://localhost:3000/callback".to_owned()],
    )
}

#[test]
fn creates_active_oauth_client() {
    let client = create_client();

    assert_eq!(client.state(), OAuthClientLifecycleState::Active);

    assert!(client.state().is_active());

    assert_eq!(client.client_id(), "github-client");
}

#[test]
fn disables_oauth_client() {
    let mut client = create_client();

    client.disable().expect("client should be disabled");

    assert_eq!(client.state(), OAuthClientLifecycleState::Disabled);
}

#[test]
fn activates_disabled_oauth_client() {
    let mut client = create_client();

    client.disable().expect("client should be disabled");

    client.activate().expect("client should activate");

    assert_eq!(client.state(), OAuthClientLifecycleState::Active);
}

#[test]
fn deletes_oauth_client() {
    let mut client = create_client();

    client.delete().expect("client should be deleted");

    assert_eq!(client.state(), OAuthClientLifecycleState::Deleted);
}

#[test]
fn deleted_oauth_client_cannot_be_activated() {
    let mut client = create_client();

    client.delete().expect("client should be deleted");

    let result = client.activate();

    assert_eq!(
        result,
        Err(localid_oauth_client::OAuthClientError::AlreadyDeleted)
    );
}

#[test]
fn validates_registered_redirect_uri() {
    let client = create_client();

    assert!(client.allows_redirect_uri("http://localhost:3000/callback"));

    assert!(!client.allows_redirect_uri("http://localhost:3000/unknown"));
}
