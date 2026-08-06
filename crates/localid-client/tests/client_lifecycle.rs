use localid_client::{Client, ClientId, ClientLifecycleState};

#[test]
fn creates_active_client() {
    let client = Client::new(ClientId::new(), "loomnotes", "Loomnotes Dashboard");

    assert_eq!(client.client_id(), "loomnotes");
    assert_eq!(client.name(), "Loomnotes Dashboard");
    assert_eq!(client.state(), ClientLifecycleState::Active);
}

#[test]
fn disables_client() {
    let mut client = Client::new(ClientId::new(), "loomnotes", "Loomnotes Dashboard");

    client.disable().unwrap();

    assert_eq!(client.state(), ClientLifecycleState::Disabled);
}

#[test]
fn activates_disabled_client() {
    let mut client = Client::new(ClientId::new(), "loomnotes", "Loomnotes Dashboard");

    client.disable().unwrap();
    client.activate().unwrap();

    assert_eq!(client.state(), ClientLifecycleState::Active);
}

#[test]
fn deletes_client() {
    let mut client = Client::new(ClientId::new(), "loomnotes", "Loomnotes Dashboard");

    client.delete().unwrap();

    assert_eq!(client.state(), ClientLifecycleState::Deleted);
}

#[test]
fn deleted_client_cannot_be_activated() {
    let mut client = Client::new(ClientId::new(), "loomnotes", "Loomnotes Dashboard");

    client.delete().unwrap();

    let result = client.activate();

    assert!(result.is_err());
}
