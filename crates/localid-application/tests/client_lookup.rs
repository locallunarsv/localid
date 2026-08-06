use localid_application::{ClientRepositoryAdapter, FindClientQuery, GetClientUseCase};

use localid_client::{Client, ClientId};

use localid_repository::ClientRepository;
use localid_repository_memory::MemoryClientRepository;

#[test]
fn get_client_should_resolve_existing_client() {
    let mut repository = MemoryClientRepository::new();

    let client = Client::new(ClientId::new(), "localid-demo", "LocalID Demo Application");

    repository.save(client).unwrap();

    let adapter = ClientRepositoryAdapter::new(repository);

    let use_case = GetClientUseCase::new(adapter);

    let result = use_case
        .execute(FindClientQuery::new("localid-demo"))
        .unwrap();

    assert_eq!(result.client_id(), "localid-demo");
}
